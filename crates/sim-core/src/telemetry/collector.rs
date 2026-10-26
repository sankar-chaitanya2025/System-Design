use crate::nodes::traits::NodeTelemetrySnapshot;
use crate::payloads::{ResponseStatus, TraceSpan};
use crate::telemetry::anomalies::ArchitecturalAnomaly;
use crate::telemetry::histogram::LatencyHdrHistogram;
use crate::time::SimTime;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemGlobalMetrics {
    pub timestamp: SimTime,
    pub total_ingress_rps: f64,
    pub total_success_rps: f64,
    pub total_error_rps: f64,
    pub availability_percentage: f64,
    pub end_to_end_p50_us: u64,
    pub end_to_end_p95_us: u64,
    pub end_to_end_p99_us: u64,
    pub end_to_end_p999_us: u64,
    pub stale_read_percentage: f64,
    pub estimated_monthly_cloud_cost_usd: f64,
}

pub trait MetricsSink {
    fn record_request_latency(&mut self, node: NodeId, latency_ns: u64, status: ResponseStatus);
    fn record_end_to_end_completion(
        &mut self,
        latency_ns: u64,
        status: ResponseStatus,
        stale_read: bool,
    );
    fn record_trace_span(&mut self, span: TraceSpan);
}

pub struct TelemetryCollector {
    global_latency: LatencyHdrHistogram,
    per_node_latency: HashMap<NodeId, LatencyHdrHistogram>,
    node_snapshots: HashMap<NodeId, NodeTelemetrySnapshot>,
    active_anomalies: Vec<ArchitecturalAnomaly>,
    recent_traces: Vec<TraceSpan>,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        todo!()
    }

    pub fn flush_window(&mut self, timestamp: SimTime) -> SystemGlobalMetrics {
        todo!()
    }

    pub fn node_snapshot(&self, node: NodeId) -> Option<&NodeTelemetrySnapshot> {
        todo!()
    }

    pub fn write_flat_telemetry_buffer(&self, out_slice: &mut [f64]) -> usize {
        todo!()
    }
}

impl MetricsSink for TelemetryCollector {
    fn record_request_latency(&mut self, _node: NodeId, _latency_ns: u64, _status: ResponseStatus) {
        todo!()
    }

    fn record_end_to_end_completion(
        &mut self,
        _latency_ns: u64,
        _status: ResponseStatus,
        _stale_read: bool,
    ) {
        todo!()
    }

    fn record_trace_span(&mut self, _span: TraceSpan) {
        todo!()
    }
}
