import { UiEngineCommand } from '@/types';

export type WorkerInboundMessage =
  | { type: 'INIT_WASM'; blueprintJson?: string }
  | { type: 'START_LOOP'; speedMultiplier: number }
  | { type: 'PAUSE_LOOP' }
  | { type: 'STEP_SINGLE_EVENT' }
  | { type: 'DISPATCH_COMMAND'; command: UiEngineCommand }
  | { type: 'EXPORT_BLUEPRINT' };

export function handleWorkerMessage(_event: MessageEvent<WorkerInboundMessage>): void {
  // Stub: WebWorker message router invoking WasmSimulatorHandle methods off the UI thread
}
