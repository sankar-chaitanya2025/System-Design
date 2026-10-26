import React from 'react';
import { TraceSpan } from '@/types';

export interface DistributedTraceWaterfallProps {
  spans: TraceSpan[];
}

export const DistributedTraceWaterfall: React.FC<DistributedTraceWaterfallProps> = () => {
  // Stub: Jaeger/Zipkin-style flame/waterfall chart showing hop-by-hop network delay, queue wait, and disk I/O
  return null;
};
