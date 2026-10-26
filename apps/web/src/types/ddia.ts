import { NodeId } from './topology';

export type StorageEngineConfig =
  | {
      engine: 'LsmTree';
      memtableSizeMb: number;
      level0StallThreshold: number;
      bloomFilterBitsPerKey: number;
      compactionBandwidthMbS: number;
    }
  | {
      engine: 'BPlusTree';
      pageSizeKb: number;
      bufferPoolHitRatio: number;
      walFsyncOnCommit: boolean;
    }
  | {
      engine: 'InMemoryColumnar';
    };

export type ReplicationModeConfig =
  | {
      mode: 'SingleLeader';
      synchronousFollowers: number;
      asynchronousFollowers: number;
      semiSyncTimeoutMs: number;
    }
  | {
      mode: 'MultiLeader';
      leaderNodes: NodeId[];
      conflictResolution: 'LastWriteWinsTimestamp' | 'VectorClockSiblingMerge' | 'CrdtStateConvergent';
      interDcReplicationDelayMs: number;
    }
  | {
      mode: 'LeaderlessDynamo';
      sloppyQuorumEnabled: boolean;
      hintedHandoffEnabled: boolean;
      readRepairProbability: number;
    };

export interface TunableQuorumConfig {
  replicationFactorN: number;
  writeQuorumW: number;
  readQuorumR: number;
}

export type IsolationLevel =
  | 'ReadUncommitted'
  | 'ReadCommitted'
  | 'RepeatableReadSnapshotIsolation'
  | 'SerializableTwoPhaseLocking'
  | 'SerializableSnapshotIsolationSsi';

export type ConsensusProtocolConfig =
  | { protocol: 'Raft'; preVoteEnabled: boolean; leaderLeaseReadEnabled: boolean }
  | { protocol: 'MultiPaxos' }
  | { protocol: 'Zab' };

export type PartitioningSchemeConfig =
  | { scheme: 'UnpartitionedSingleShard' }
  | { scheme: 'KeyRangePartitioning'; dynamicRangeSplitting: boolean; hotspotAutoSplitRps: number }
  | { scheme: 'ConsistentHashPartitioning'; virtualVnodesPerNode: number }
  | { scheme: 'HashModulusFixed'; numBuckets: number };
