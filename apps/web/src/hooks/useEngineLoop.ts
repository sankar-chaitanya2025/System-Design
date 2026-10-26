import { WasmFrameDigest } from '@/types';

export interface UseEngineLoopResult {
  fps: number;
  eventsPerSecond: number;
  lastDigest: WasmFrameDigest | null;
}

export function useEngineLoop(): UseEngineLoopResult {
  throw new Error('Not implemented');
}
