use crate::error::{SimError, SimResult};
use crate::nodes::NodeKind;
use crate::topology::edge::NetworkEdge;
use crate::topology::ids::{EdgeId, NodeId, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePlacement {
    pub id: NodeId,
    pub label: String,
    pub region: RegionId,
    pub availability_zone: String,
    pub canvas_x: f64,
    pub canvas_y: f64,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyGraph {
    nodes: HashMap<NodeId, NodeKind>,
    placements: HashMap<NodeId, NodePlacement>,
    edges: HashMap<EdgeId, NetworkEdge>,
    outgoing_edges: HashMap<NodeId, Vec<EdgeId>>,
    incoming_edges: HashMap<NodeId, Vec<EdgeId>>,
}

impl TopologyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, placement: NodePlacement, node: NodeKind) -> SimResult<NodeId> {
        let id = placement.id;
        if self.nodes.contains_key(&id) {
            return Err(SimError::Unsupported("Node already exists in topology".into()));
        }
        self.placements.insert(id, placement);
        self.nodes.insert(id, node);
        self.outgoing_edges.entry(id).or_default();
        self.incoming_edges.entry(id).or_default();
        Ok(id)
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> SimResult<NodeKind> {
        let out_edges = self.outgoing_edges.remove(&node_id).unwrap_or_default();
        let in_edges = self.incoming_edges.remove(&node_id).unwrap_or_default();

        for edge_id in out_edges.iter().chain(in_edges.iter()) {
            self.edges.remove(edge_id);
        }

        self.placements.remove(&node_id);
        self.nodes
            .remove(&node_id)
            .ok_or_else(|| SimError::Unsupported("Node not found".into()))
    }

    pub fn add_edge(&mut self, edge: NetworkEdge) -> SimResult<EdgeId> {
        let id = edge.id;
        if self.edges.contains_key(&id) {
            return Err(SimError::Unsupported("Edge already exists".into()));
        }
        if !self.nodes.contains_key(&edge.source) || !self.nodes.contains_key(&edge.target) {
            return Err(SimError::Unsupported("Edge references missing node".into()));
        }

        self.outgoing_edges.get_mut(&edge.source).unwrap().push(id);
        self.incoming_edges.get_mut(&edge.target).unwrap().push(id);
        self.edges.insert(id, edge);
        
        Ok(id)
    }

    pub fn remove_edge(&mut self, edge_id: EdgeId) -> SimResult<NetworkEdge> {
        let edge = self
            .edges
            .remove(&edge_id)
            .ok_or_else(|| SimError::Unsupported("Edge not found".into()))?;

        if let Some(out_edges) = self.outgoing_edges.get_mut(&edge.source) {
            out_edges.retain(|&id| id != edge_id);
        }
        if let Some(in_edges) = self.incoming_edges.get_mut(&edge.target) {
            in_edges.retain(|&id| id != edge_id);
        }

        Ok(edge)
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&NodeKind> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut NodeKind> {
        self.nodes.get_mut(&node_id)
    }

    pub fn get_edge(&self, edge_id: EdgeId) -> Option<&NetworkEdge> {
        self.edges.get(&edge_id)
    }

    pub fn get_edge_mut(&mut self, edge_id: EdgeId) -> Option<&mut NetworkEdge> {
        self.edges.get_mut(&edge_id)
    }

    pub fn outgoing_edges(&self, node_id: NodeId) -> &[EdgeId] {
        self.outgoing_edges
            .get(&node_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn incoming_edges(&self, node_id: NodeId) -> &[EdgeId] {
        self.incoming_edges
            .get(&node_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}