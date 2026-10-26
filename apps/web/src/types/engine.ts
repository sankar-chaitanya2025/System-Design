import { ChaosFaultId, ChaosSchedule } from './chaos';
import { AnyNodeSpec } from './nodes';
import { ArchitecturalAnomaly, SystemGlobalMetrics, TraceSpan } from './telemetry';
import { EdgeId, NetworkEdge, NodeId, NodePlacement } from './topology';

export type FidelityMode =
  | { mode: 'MicroDiscretePacket' }
  | { mode: 'MacroFluidDensity' }
  | { mode: 'HybridAdaptive'; packetToFluidThresholdRps: number };

export type EngineRunState = 'Idle' | 'Running' | 'Paused' | 'Stepping' | 'Completed' | 'FaultHalted';

export interface SimulationConfig {
  randomSeed: number;
  fidelityMode: FidelityMode;
  fluidIntegrationStepNs: number;
  telemetryWindowIntervalNs: number;
  maxEventsPerFrame: number;
  maxInFlightPackets: number;
  enableDistributedTracing: boolean;
  traceSamplingRate: number;
}

export interface StepExecutionSummary {
  startTimeNs: number;
  endTimeNs: number;
  discreteEventsProcessed: number;
  fluidStepsIntegrated: number;
  activePacketsInFlight: number;
  droppedRequestsDelta: number;
}

export interface WasmFrameDigest {
  stepSummary: StepExecutionSummary;
  globalMetrics: SystemGlobalMetrics;
  newlyDetectedAnomalies: ArchitecturalAnomaly[];
  sampledTraces: TraceSpan[];
}

export type UiEngineCommand =
  | { type: 'LoadBlueprintJson'; rawJson: string }
  | { type: 'UpsertNode'; placement: NodePlacement; kind: AnyNodeSpec }
  | { type: 'RemoveNode'; nodeId: NodeId }
  | { type: 'UpsertEdge'; edge: NetworkEdge }
  | { type: 'RemoveEdge'; edgeId: EdgeId }
  | { type: 'SetClientTrafficRps'; clientNode: NodeId; targetRps: number }
  | { type: 'SetFidelityMode'; mode: FidelityMode }
  | { type: 'ScheduleChaosFault'; schedule: ChaosSchedule }
  | { type: 'CancelChaosFault'; faultId: ChaosFaultId }
  | { type: 'ResetSimulation'; newSeed?: number };
