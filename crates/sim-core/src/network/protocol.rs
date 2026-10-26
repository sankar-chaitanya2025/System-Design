use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongestionControlAlgorithm {
    Cubic,
    Bbr,
    Reno,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportProtocol {
    Tcp {
        tls_enabled: bool,
        keep_alive: bool,
        congestion_control: CongestionControlAlgorithm,
        head_of_line_blocking: bool,
    },
    Http1_1 {
        max_pipelined_connections: u16,
    },
    Http2 {
        max_concurrent_streams: u32,
    },
    Http3Quic {
        zero_rtt_handshake: bool,
    },
    Grpc {
        multiplexed_streams: u32,
    },
    UdpUnreliable,
    GossipUdp {
        fanout: u8,
        interval_ms: u32,
    },
}
