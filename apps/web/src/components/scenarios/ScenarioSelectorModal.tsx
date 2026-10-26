import React from 'react';
import { ScenarioPreset } from '@/types';

export interface ScenarioSelectorModalProps {
  isOpen: boolean;
  presets: ScenarioPreset[];
  onSelectPreset: (presetId: string) => void;
  onClose: () => void;
}

export const ScenarioSelectorModal: React.FC<ScenarioSelectorModalProps> = () => {
  // Stub: Modal for picking progressive System Design challenges (4-Server LB -> 1M User Global Flash Sale)
  return null;
};
