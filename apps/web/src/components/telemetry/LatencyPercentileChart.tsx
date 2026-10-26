import React from 'react';
import { SystemGlobalMetrics } from '@/types';

export interface LatencyPercentileChartProps {
  series: SystemGlobalMetrics[];
}

export const LatencyPercentileChart: React.FC<LatencyPercentileChartProps> = () => {
  // Stub: Real-time streaming chart plotting p50, p95, p99, and p99.9 tail latency
  return null;
};
