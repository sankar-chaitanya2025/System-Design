use crate::network::PartitionRule;
use crate::time::{ ClockSkewModel, SimDuration};
use crate::topology::{EdgeId, NodeId, RegionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChaosFaultId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosFaultSpec {
    KillNodeCrashStop {
        node: NodeId,
    },
    PauseNodeProcessFreeze {
        node: NodeId,
        duration: SimDuration,
    },
    NetworkPartitionCut {
        rule: PartitionRule,
    },
    DegradeLinkLatency {
        edge: EdgeId,
        added_latency_ms: u64,
        added_jitter_ms: u64,
    },
    InjectPacketLoss {
        edge: EdgeId,
        loss_ratio: f64,
    },
    InjectClockSkew {
        node: NodeId,
        skew: ClockSkewModel,
    },
    TriggerHotKeyCelebritySpike {
        hot_key_hash: u64,
        traffic_fraction: f64,
    },
    FlushCacheColdStart {
        cache_node: NodeId,
    },
    DiskSlowdownGreyFailure {
        node: NodeId,
        iops_degradation_factor: f64,
    },
    RegionDisasterOutage {
        region: RegionId,
    },
}
