import { ScenarioPreset } from '@/types';

export interface ScenarioStoreState {
  activeScenario: ScenarioPreset | null;
  availablePresets: ScenarioPreset[];
  sloCompliancePassed: boolean;
  errorBudgetRemainingPct: number;

  loadScenarioPreset: (presetId: string) => void;
}

export function useScenarioStore(): ScenarioStoreState {
  throw new Error('Not implemented');
}
