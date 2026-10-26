export type DifficultyTier =
  | 'BeginnerFoundations'
  | 'IntermediateScaling'
  | 'AdvancedDdiaConsistency'
  | 'PrincipalMillionUserChaos';

export interface SloDefinition {
  name: string;
  targetAvailabilityPct: number;
  maxP95LatencyMs: number;
  maxP99LatencyMs: number;
  maxStaleReadRatio: number;
}

export interface ScenarioPreset {
  id: string;
  title: string;
  tier: DifficultyTier;
  briefingMarkdown: string;
  blueprintJson: string;
  requiredSlos: SloDefinition[];
  maxMonthlyBudgetUsd?: number;
}
