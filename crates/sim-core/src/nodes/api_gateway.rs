use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitAlgorithm {
    TokenBucket {
        refill_rate_rps: f64,
        bucket_burst: u64,
    },
    LeakyBucket {
        drain_rate_rps: f64,
        queue_size: u64,
    },
    FixedWindowCounter {
        window: SimDuration,
        max_requests: u64,
    },
    SlidingWindowLog {
        window: SimDuration,
        max_requests: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen { trial_requests_remaining: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerSpec {
    pub failure_rate_threshold: f64,
    pub minimum_request_volume: u32,
    pub open_state_cooldown: SimDuration,
    pub half_open_permitted_calls: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayConfig {
    pub hardware: HardwareResourceSpec,
    pub rate_limiter: Option<RateLimitAlgorithm>,
    pub circuit_breaker: Option<CircuitBreakerSpec>,
    pub jwt_verification_overhead_us: u64,
    pub request_hedging_delay: Option<SimDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGatewayNode {
    pub id: NodeId,
    pub config: ApiGatewayConfig,
    pub breaker_state: CircuitBreakerState,
    pub throttled_requests_total: u64,
}
