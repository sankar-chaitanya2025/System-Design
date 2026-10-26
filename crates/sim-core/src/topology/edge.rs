use crate::math::LatencyDistributionSpec;
use crate::network::TransportProtocol;
use crate::topology::ids::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCharacteristics {
    pub bandwidth_bps: u64,
    pub base_propagation_delay_ns: u64,
    pub jitter_distribution: LatencyDistributionSpec,
    pub packet_loss_rate: f64,
    pub packet_reorder_rate: f64,
    pub mtu_bytes: u16,
    pub max_in_flight_bytes: usize,
    pub duplex: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEdge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub protocol: TransportProtocol,
    pub characteristics: LinkCharacteristics,
    pub partitioned: bool,
    pub current_utilization_bps: f64,
    pub in_flight_packet_count: u32,
}

impl NetworkEdge {
    pub fn new(
        id: EdgeId,
        source: NodeId,
        target: NodeId,
        protocol: TransportProtocol,
        characteristics: LinkCharacteristics,
    ) -> Self {
        todo!()
    }

    pub fn serialization_delay_ns(&self, packet_bytes: usize) -> u64 {
        todo!()
    }
}
