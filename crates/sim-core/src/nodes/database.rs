use crate::consistency::{
    ConsensusProtocol, IsolationLevel,PartitioningScheme, ReplicationMode, TunableQuorumConfig,
};
use crate::math::LatencyDistributionSpec;
use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::{NodeId, ShardId};
use serde::{Deserialize, Serialize};

/// DDIA Chapter 3 Storage Engine Architecture: LSM-Tree vs B+Tree performance trade-offs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageEngineType {
    /// Write-optimized Log-Structured Merge-Tree (RocksDB / Cassandra / Scylla style).
    LsmTree {
        memtable_size_mb: u32,
        level0_sstable_stall_threshold: u8,
        bloom_filter_bits_per_key: u8,
        compaction_bandwidth_mb_s: u32,
        read_amplification_factor: f64,
        write_amplification_factor: f64,
    },
    /// Read-optimized Page-Oriented B+Tree (PostgreSQL / InnoDB style).
    BPlusTree {
        page_size_kb: u8,
        buffer_pool_hit_ratio: f64,
        wal_fsync_on_commit: bool,
        checkpoint_interval: SimDuration,
        random_io_penalty_multiplier: f64,
    },
    /// Pure In-Memory Transaction Engine.
    InMemoryColumnar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseNodeConfig {
    pub hardware: HardwareResourceSpec,
    pub storage_engine: StorageEngineType,
    pub replication_mode: ReplicationMode,
    pub quorum_config: TunableQuorumConfig,
    pub isolation_level: IsolationLevel,
    pub consensus: Option<ConsensusProtocol>,
    pub partitioning: PartitioningScheme,
    pub max_connections: u32,
    pub lock_wait_timeout: SimDuration,
    pub base_read_latency: LatencyDistributionSpec,
    pub base_write_latency: LatencyDistributionSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseNode {
    pub id: NodeId,
    pub shard_id: ShardId,
    pub is_primary_leader: bool,
    pub replica_peers: Vec<NodeId>,
    pub config: DatabaseNodeConfig,
    pub active_transactions: u32,
    pub replication_lag_ns: u64,
    pub compaction_active: bool,
    pub wal_queue_depth: u32,
}
