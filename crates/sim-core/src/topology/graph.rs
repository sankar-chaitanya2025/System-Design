use crate::error::SimResult;
use crate::nodes::NodeKind;
use crate::topology::edge::NetworkEdge;
use crate::topology::ids::{EdgeId, NodeId, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePlacement {
    pub id: NodeId,
    pub label: String,
    pub region: RegionId,
    pub availability_zone: String,
    pub canvas_x: f64,
    pub canvas_y: f64,
}

#[derive(Default)]
pub struct TopologyGraph {
    nodes: HashMap<NodeId, NodeKind>,
    placements: HashMap<NodeId, NodePlacement>,
    edges: HashMap<EdgeId, NetworkEdge>,
    outgoing_edges: HashMap<NodeId, Vec<EdgeId>>,
    incoming_edges: HashMap<NodeId, Vec<EdgeId>>,
}

impl TopologyGraph {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_node(&mut self, placement: NodePlacement, node: NodeKind) -> SimResult<NodeId> {
        todo!()
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> SimResult<NodeKind> {
        todo!()
    }

    pub fn add_edge(&mut self, edge: NetworkEdge) -> SimResult<EdgeId> {
        todo!()
    }

    pub fn remove_edge(&mut self, edge_id: EdgeId) -> SimResult<NetworkEdge> {
        todo!()
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&NodeKind> {
        todo!()
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut NodeKind> {
        todo!()
    }

    pub fn get_edge(&self, edge_id: EdgeId) -> Option<&NetworkEdge> {
        todo!()
    }

    pub fn get_edge_mut(&mut self, edge_id: EdgeId) -> Option<&mut NetworkEdge> {
        todo!()
    }

    pub fn outgoing_edges(&self, node_id: NodeId) -> &[EdgeId] {
        todo!()
    }

    pub fn incoming_edges(&self, node_id: NodeId) -> &[EdgeId] {
        todo!()
    }

    pub fn node_count(&self) -> usize {
        todo!()
    }

    pub fn edge_count(&self) -> usize {
        todo!()
    }
}
