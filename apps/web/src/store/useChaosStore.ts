import { ChaosFaultId, ChaosFaultSpec, ChaosSchedule } from '@/types';

export interface ChaosStoreState {
  schedules: Record<ChaosFaultId, ChaosSchedule>;
  activeFaultIds: ChaosFaultId[];

  injectImmediateFault: (label: string, fault: ChaosFaultSpec, durationMs?: number) => void;
  scheduleTimelineFault: (schedule: ChaosSchedule) => void;
  healFault: (faultId: ChaosFaultId) => void;
  clearAllFaults: () => void;
}

export function useChaosStore(): ChaosStoreState {
  throw new Error('Not implemented');
}
