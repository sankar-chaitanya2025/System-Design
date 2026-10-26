use crate::error::{SimError, SimResult};
use crate::topology::graph::TopologyGraph;
use crate::topology::ids::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePath {
    pub hops: Vec<(NodeId, EdgeId)>,
    pub estimated_min_rtt_ns: u64,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RouteState {
    cost: u64,
    node: NodeId,
}

impl Ord for RouteState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for RouteState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TopologyRouter;

impl TopologyRouter {
    pub fn shortest_latency_path(
        graph: &TopologyGraph,
        src: NodeId,
        dst: NodeId,
    ) -> SimResult<RoutePath> {
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(src, 0);
        heap.push(RouteState { cost: 0, node: src });

        while let Some(RouteState { cost, node }) = heap.pop() {
            if node == dst {
                break;
            }
            if cost > *dist.get(&node).unwrap_or(&u64::MAX) {
                continue;
            }

            for &edge_id in graph.outgoing_edges(node) {
                if let Some(edge) = graph.get_edge(edge_id) {
                    if edge.partitioned {
                        continue;
                    }
                    let next_node = edge.target;
                    let next_cost = cost + edge.characteristics.base_propagation_delay_ns;

                    if next_cost < *dist.get(&next_node).unwrap_or(&u64::MAX) {
                        dist.insert(next_node, next_cost);
                        prev.insert(next_node, (node, edge_id));
                        heap.push(RouteState { cost: next_cost, node: next_node });
                    }
                }
            }
        }

        if !prev.contains_key(&dst) {
            return Err(SimError::Unsupported("No route to host".into()));
        }

        let mut hops = Vec::new();
        let mut current = dst;
        while let Some(&(prev_node, edge_id)) = prev.get(&current) {
            hops.push((current, edge_id));
            current = prev_node;
        }
        hops.reverse();

        Ok(RoutePath {
            hops,
            estimated_min_rtt_ns: *dist.get(&dst).unwrap() * 2, // approximation of round trip
        })
    }

    pub fn is_reachable(graph: &TopologyGraph, src: NodeId, dst: NodeId) -> bool {
        if src == dst {
            return true;
        }
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        visited.insert(src);
        queue.push_back(src);

        while let Some(node) = queue.pop_front() {
            if node == dst {
                return true;
            }
            for &edge_id in graph.outgoing_edges(node) {
                if let Some(edge) = graph.get_edge(edge_id) {
                    if !edge.partitioned && visited.insert(edge.target) {
                        queue.push_back(edge.target);
                    }
                }
            }
        }
        false
    }
}