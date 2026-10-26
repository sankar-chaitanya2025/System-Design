import { AnyNodeSpec, EdgeId, NetworkEdge, NodeId, NodePlacement } from '@/types';

export interface TopologyState {
  placements: Record<NodeId, NodePlacement>;
  nodes: Record<NodeId, AnyNodeSpec>;
  edges: Record<EdgeId, NetworkEdge>;
  selectedNodeId: NodeId | null;
  selectedEdgeId: EdgeId | null;
  undoStack: string[];
  redoStack: string[];

  addNode: (placement: NodePlacement, spec: AnyNodeSpec) => void;
  updateNodeSpec: (nodeId: NodeId, spec: Partial<AnyNodeSpec>) => void;
  moveNodePosition: (nodeId: NodeId, x: number, y: number) => void;
  removeNode: (nodeId: NodeId) => void;
  connectNodes: (edge: NetworkEdge) => void;
  updateEdge: (edgeId: EdgeId, edge: Partial<NetworkEdge>) => void;
  removeEdge: (edgeId: EdgeId) => void;
  selectNode: (nodeId: NodeId | null) => void;
  selectEdge: (edgeId: EdgeId | null) => void;
  undo: () => void;
  redo: () => void;
}

export function useTopologyStore(): TopologyState {
  throw new Error('Not implemented');
}
