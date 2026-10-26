use crate::error::SimResult;
use crate::topology::graph::TopologyGraph;
use crate::topology::ids::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePath {
    pub hops: Vec<(NodeId, EdgeId)>,
    pub estimated_min_rtt_ns: u64,
}

pub struct TopologyRouter;

impl TopologyRouter {
    /// Finds shortest latency path respecting active network partitions.
    pub fn shortest_latency_path(
        graph: &TopologyGraph,
        src: NodeId,
        dst: NodeId,
    ) -> SimResult<RoutePath> {
        todo!()
    }

    /// Checks whether two nodes can communicate or are separated by a partition cut.
    pub fn is_reachable(graph: &TopologyGraph, src: NodeId, dst: NodeId) -> bool {
        todo!()
    }
}
