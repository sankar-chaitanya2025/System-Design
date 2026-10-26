import {
  ArchitecturalAnomaly,
  NodeId,
  NodeTelemetrySnapshot,
  SystemGlobalMetrics,
  TraceSpan,
} from '@/types';

export interface TelemetryStoreState {
  latestGlobalMetrics: SystemGlobalMetrics | null;
  globalHistorySeries: SystemGlobalMetrics[];
  nodeSnapshots: Record<NodeId, NodeTelemetrySnapshot>;
  activeAnomalies: ArchitecturalAnomaly[];
  sampledTraces: TraceSpan[];

  ingestFrameDigest: (
    globalMetrics: SystemGlobalMetrics,
    anomalies: ArchitecturalAnomaly[],
    traces: TraceSpan[]
  ) => void;
  clearTelemetryHistory: () => void;
}

export function useTelemetryStore(): TelemetryStoreState {
  throw new Error('Not implemented');
}
