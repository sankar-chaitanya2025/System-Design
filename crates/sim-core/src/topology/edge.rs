use crate::math::LatencyDistributionSpec;
use crate::network::TransportProtocol;
use crate::topology::ids::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl Default for LinkCharacteristics {
    fn default() -> Self {
        Self {
            bandwidth_bps: 10_000_000_000, // 10 Gbps
            base_propagation_delay_ns: 500_000, // 0.5 ms
            jitter_distribution: LatencyDistributionSpec::Constant { value: 0.0 },
            packet_loss_rate: 0.0,
            packet_reorder_rate: 0.0,
            mtu_bytes: 1500,
            max_in_flight_bytes: 64 * 1024 * 1024,
            duplex: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        Self {
            id,
            source,
            target,
            protocol,
            characteristics,
            partitioned: false,
            current_utilization_bps: 0.0,
            in_flight_packet_count: 0,
        }
    }

    /// Computes physical wire serialization delay: `(packet_bytes * 8 * 10^9) / bandwidth_bps`.
    /// Uses `u128` intermediate math to prevent overflow on large packets.
    #[inline]
    pub fn serialization_delay_ns(&self, packet_bytes: usize) -> u64 {
        let bw = self.characteristics.bandwidth_bps;
        if bw == 0 || packet_bytes == 0 {
            return 0;
        }
        let bits_ns = (packet_bytes as u128)
            .saturating_mul(8)
            .saturating_mul(1_000_000_000);
        (bits_ns / (bw as u128)) as u64
    }

    /// Minimum physical one-way delay (propagation + 64-byte frame serialization).
    #[inline]
    pub fn min_one_way_latency_ns(&self) -> u64 {
        self.characteristics
            .base_propagation_delay_ns
            .saturating_add(self.serialization_delay_ns(64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_delay_math() {
        let mut chars = LinkCharacteristics::default();
        chars.bandwidth_bps = 1_000_000_000; // 1 Gbps = 1 bit per ns
        let edge = NetworkEdge::new(
            EdgeId(1),
            NodeId(1),
            NodeId(2),
            TransportProtocol::UdpUnreliable,
            chars,
        );

        // 1500 bytes = 12,000 bits -> 12,000 ns at 1 Gbps
        assert_eq!(edge.serialization_delay_ns(1500), 12_000);
        assert_eq!(edge.serialization_delay_ns(0), 0);
    }
}