import React from 'react';
import { NodeCategory } from '@/types';

export interface PaletteItemDescriptor {
  category: NodeCategory;
  displayName: string;
  shortDescription: string;
  ddiaConceptTag: string;
}

export const ComponentPaletteSidebar: React.FC = () => {
  // Stub: Draggable catalog of architectural building blocks (Client Pool, LB, Gateway, App, Cache, DB, Queue, Consensus)
  return null;
};
