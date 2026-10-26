import { EngineRunState, FidelityMode } from '@/types';

export interface SimulationControlState {
  runState: EngineRunState;
  virtualClockNs: number;
  playbackSpeedMultiplier: number;
  fidelityMode: FidelityMode;
  simulatedUsersCount: number;
  globalTargetRps: number;
  randomSeed: number;

  play: () => void;
  pause: () => void;
  stepForwardEvent: () => void;
  resetClock: (newSeed?: number) => void;
  setPlaybackSpeed: (multiplier: number) => void;
  setFidelityMode: (mode: FidelityMode) => void;
  setSimulatedUsersCount: (users: number) => void;
}

export function useSimulationStore(): SimulationControlState {
  throw new Error('Not implemented');
}
