import { NodeHealthStatus } from './nodes';
import { EdgeId, NodeId } from './topology';

export interface NodeTelemetrySnapshot {
  nodeId: NodeId;
  health: NodeHealthStatus;
  cpuUtilization: number;
  memoryUtilization: number;
  diskIoUtilization: number;
  activeConnections: number;
  queueDepth: number;
  ingressRps: number;
  egressRps: number;
  errorRateRps: number;
  p50LatencyUs: number;
  p95LatencyUs: number;
  p99LatencyUs: number;
  p999LatencyUs: number;
}

export interface SystemGlobalMetrics {
  timestampNs: number;
  totalIngressRps: number;
  totalSuccessRps: number;
  totalErrorRps: number;
  availabilityPercentage: number;
  endToEndP50Us: number;
  endToEndP95Us: number;
  endToEndP99Us: number;
  endToEndP999Us: number;
  staleReadPercentage: number;
  estimatedMonthlyCloudCostUsd: number;
}

export type AnomalyCategory =
  | 'SinglePointOfFailure'
  | 'CacheStampedeThunderingHerd'
  | 'RetryStormAmplification'
  | 'SplitBrainMultipleLeaders'
  | 'ReplicationLagSloBreach'
  | 'HotShardPartitionSkew'
  | 'ConnectionPoolExhaustion'
  | 'LsmCompactionWriteStall'
  | 'QueueConsumerBacklogExplosion';

export interface ArchitecturalAnomaly {
  detectedAtNs: number;
  category: AnomalyCategory;
  affectedNodes: NodeId[];
  affectedEdges: EdgeId[];
  severityScore: number;
  explanation: string;
  remediationHint: string;
}

export interface TraceSpan {
  traceId: string;
  spanId: string;
  parentSpanId?: string;
  nodeId: NodeId;
  operationName: string;
  startTimeNs: number;
  endTimeNs: number;
  queueWaitNs: number;
  serviceExecNs: number;
  errorTag?: string;
}
