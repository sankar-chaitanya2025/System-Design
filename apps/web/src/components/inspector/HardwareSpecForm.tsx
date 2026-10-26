import React from 'react';
import { HardwareResourceSpec } from '@/types';

export interface HardwareSpecFormProps {
  value: HardwareResourceSpec;
  onChange: (updated: HardwareResourceSpec) => void;
}

export const HardwareSpecForm: React.FC<HardwareSpecFormProps> = () => {
  // Stub: Sliders for vCPU cores, RAM MiB, NIC Gbps, and Disk IOPS
  return null;
};
