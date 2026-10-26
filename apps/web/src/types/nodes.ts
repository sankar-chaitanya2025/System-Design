import { NodeId, RegionId, ShardId } from './topology';
import {
  ConsensusProtocolConfig,
  IsolationLevel,
  PartitioningSchemeConfig,
  ReplicationModeConfig,
  StorageEngineConfig,
  TunableQuorumConfig,
} from './ddia';

export type NodeCategory =
  | 'ClientPool'
  | 'DnsResolver'
  | 'CdnEdge'
  | 'LoadBalancer'
  | 'ApiGateway'
  | 'AppServer'
  | 'CacheCluster'
  | 'MessageQueue'
  | 'StreamProcessor'
  | 'Database'
  | 'SearchIndex'
  | 'ObjectStorage'
  | 'ConsensusCoordinator';

export type NodeHealthStatus =
  | { status: 'Healthy' }
  | { status: 'Degraded'; saturationPct: number }
  | { status: 'OverloadedCircuitOpen' }
  | { status: 'CrashLooping' }
  | { status: 'Terminated' };

export interface HardwareResourceSpec {
  vcpuCores: number;
  memoryMib: number;
  maxNicBandwidthMbps: number;
  diskIopsLimit: number;
  diskThroughputMbS: number;
  maxOpenSockets: number;
}

export type LoadBalancingAlgorithm =
  | 'RoundRobin'
  | 'WeightedRoundRobin'
  | 'LeastConnections'
  | 'LeastResponseTime'
  | 'PowerOfTwoChoices'
  | 'ConsistentHashingRing'
  | 'MaglevHashTable';

export interface LoadBalancerNodeSpec {
  category: 'LoadBalancer';
  id: NodeId;
  layer: 'Layer4TcpPassthrough' | 'Layer7HttpReverseProxy';
  algorithm: LoadBalancingAlgorithm;
  hardware: HardwareResourceSpec;
  backends: NodeId[];
}

export interface AppServerNodeSpec {
  category: 'AppServer';
  id: NodeId;
  hardware: HardwareResourceSpec;
  concurrencyModel: 'OSThreadPool' | 'AsyncEventLoop' | 'ActorMailbox';
  maxWorkers: number;
  autoScalingMin: number;
  autoScalingMax: number;
  gcEnabled: boolean;
}

export interface CacheNodeSpec {
  category: 'CacheCluster';
  id: NodeId;
  hardware: HardwareResourceSpec;
  maxMemoryGb: number;
  evictionPolicy: 'Lru' | 'Lfu' | 'Wtinylfu' | 'Fifo' | 'Random';
  writePattern: 'CacheAside' | 'ReadThrough' | 'WriteThrough' | 'WriteBehindAsync' | 'WriteAround';
  singleflightLockEnabled: boolean;
  backingDatastore?: NodeId;
}

export interface DatabaseNodeSpec {
  category: 'Database';
  id: NodeId;
  shardId: ShardId;
  isPrimaryLeader: boolean;
  replicaPeers: NodeId[];
  hardware: HardwareResourceSpec;
  storageEngine: StorageEngineConfig;
  replicationMode: ReplicationModeConfig;
  quorumConfig: TunableQuorumConfig;
  isolationLevel: IsolationLevel;
  consensus?: ConsensusProtocolConfig;
  partitioning: PartitioningSchemeConfig;
}

export interface ClientPoolNodeSpec {
  category: 'ClientPool';
  id: NodeId;
  region: RegionId;
  simulatedUserCount: number;
  targetRps: number;
  readWriteRatio: number;
  zipfSkewExponent: number;
}

export type AnyNodeSpec =
  | ClientPoolNodeSpec
  | LoadBalancerNodeSpec
  | AppServerNodeSpec
  | CacheNodeSpec
  | DatabaseNodeSpec
  | { category: Exclude<NodeCategory, 'ClientPool' | 'LoadBalancer' | 'AppServer' | 'CacheCluster' | 'Database'>; id: NodeId; config: Record<string, unknown> };
