import { EdgeId, NodeId, RegionId } from './topology';

export type ChaosFaultId = number;

export type ChaosFaultSpec =
  | { kind: 'KillNodeCrashStop'; node: NodeId }
  | { kind: 'PauseNodeProcessFreeze'; node: NodeId; durationMs: number }
  | { kind: 'BipartiteNetworkPartition'; sideA: NodeId[]; sideB: NodeId[] }
  | { kind: 'DegradeLinkLatency'; edge: EdgeId; addedLatencyMs: number; addedJitterMs: number }
  | { kind: 'InjectPacketLoss'; edge: EdgeId; lossRatio: number }
  | { kind: 'InjectClockSkew'; node: NodeId; baseOffsetMs: number; driftPpm: number }
  | { kind: 'TriggerHotKeyCelebritySpike'; hotKeyHash: number; trafficFraction: number }
  | { kind: 'FlushCacheColdStart'; cacheNode: NodeId }
  | { kind: 'DiskSlowdownGreyFailure'; node: NodeId; iopsDegradationFactor: number }
  | { kind: 'RegionDisasterOutage'; region: RegionId };

export interface ChaosSchedule {
  id: ChaosFaultId;
  label: string;
  fault: ChaosFaultSpec;
  startTimeNs: number;
  durationNs?: number;
  active: boolean;
}
