import React from 'react';
import { RegionId } from '@/types';

export interface RegionBoundaryBoxProps {
  regionId: RegionId;
  title: string;
  availabilityZones: string[];
}

export const RegionBoundaryBox: React.FC<RegionBoundaryBoxProps> = () => {
  // Stub: Visual bounding box grouping nodes by Cloud Region (us-east-1, eu-west-1) and AZs
  return null;
};
