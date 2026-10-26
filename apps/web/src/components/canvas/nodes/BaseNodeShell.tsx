import React from 'react';
import { NodeHealthStatus, NodeId } from '@/types';

export interface BaseNodeShellProps {
  nodeId: NodeId;
  title: string;
  subtitle: string;
  health: NodeHealthStatus;
  cpuUtilization: number;
  p99LatencyUs: number;
  ingressRps: number;
  selected: boolean;
  children?: React.ReactNode;
}

export const BaseNodeShell: React.FC<BaseNodeShellProps> = () => {
  // Stub: Shared node card chrome with live CPU/Queue micro-meters, health badge, and input/output ports
  return null;
};
