use crate::error::SimResult;
use crate::event::EventContext;
use crate::payloads::{FluidDensityWave, RequestPacket, ResponsePacket};
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealthStatus {
    Healthy,
    Degraded { saturation_pct: u8 },
    OverloadedCircuitOpen,
    CrashLooping,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareResourceSpec {
    pub vcpu_cores: u16,
    pub memory_mib: u64,
    pub max_nic_bandwidth_mbps: u32,
    pub disk_iops_limit: u32,
    pub disk_throughput_mb_s: u32,
    pub max_open_sockets: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTelemetrySnapshot {
    pub node_id: NodeId,
    pub health: NodeHealthStatus,
    pub cpu_utilization: f32,
    pub memory_utilization: f32,
    pub disk_io_utilization: f32,
    pub active_connections: u32,
    pub queue_depth: u32,
    pub ingress_rps: f64,
    pub egress_rps: f64,
    pub error_rate_rps: f64,
    pub p50_latency_us: u64,
    pub p95_latency_us: u64,
    pub p99_latency_us: u64,
    pub p999_latency_us: u64,
}

pub trait SimulatedNode {
    fn node_id(&self) -> NodeId;
    fn health_status(&self) -> NodeHealthStatus;

    fn on_request_arrival(
        &mut self,
        packet: RequestPacket,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()>;

    fn on_response_arrival(
        &mut self,
        response: ResponsePacket,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()>;

    fn on_fluid_wave(
        &mut self,
        wave: FluidDensityWave,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()>;

    fn collect_telemetry(&self) -> NodeTelemetrySnapshot;
}
