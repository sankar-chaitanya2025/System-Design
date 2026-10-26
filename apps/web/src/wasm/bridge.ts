import { UiEngineCommand, WasmFrameDigest } from '@/types';
import { TelemetryMemoryReader } from './memoryView';

export interface ISimulationWorkerBridge {
  initialize(blueprintJson?: string): Promise<void>;
  startLoop(speedMultiplier: number): void;
  pauseLoop(): void;
  stepSingleEvent(): Promise<WasmFrameDigest>;
  dispatchCommand(command: UiEngineCommand): Promise<void>;
  exportBlueprint(): Promise<string>;
  getTelemetryReader(): TelemetryMemoryReader | null;
  onFrameDigest(callback: (digest: WasmFrameDigest) => void): () => void;
}

export class SimulationWorkerBridge implements ISimulationWorkerBridge {
  protected worker: Worker | null = null;
  protected telemetryReader: TelemetryMemoryReader | null = null;

  public async initialize(_blueprintJson?: string): Promise<void> {
    throw new Error('Not implemented');
  }

  public startLoop(_speedMultiplier: number): void {
    throw new Error('Not implemented');
  }

  public pauseLoop(): void {
    throw new Error('Not implemented');
  }

  public async stepSingleEvent(): Promise<WasmFrameDigest> {
    throw new Error('Not implemented');
  }

  public async dispatchCommand(_command: UiEngineCommand): Promise<void> {
    throw new Error('Not implemented');
  }

  public async exportBlueprint(): Promise<string> {
    throw new Error('Not implemented');
  }

  public getTelemetryReader(): TelemetryMemoryReader | null {
    throw new Error('Not implemented');
  }

  public onFrameDigest(_callback: (digest: WasmFrameDigest) => void): () => void {
    throw new Error('Not implemented');
  }
}
