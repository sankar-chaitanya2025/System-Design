export type NodeId = number;
export type EdgeId = number;
export type ClusterId = number;
export type RegionId = number;
export type ShardId = number;

export type TransportProtocolType =
  | { type: 'Tcp'; tlsEnabled: boolean; keepAlive: boolean; headOfLineBlocking: boolean }
  | { type: 'Http1_1'; maxPipelinedConnections: number }
  | { type: 'Http2'; maxConcurrentStreams: number }
  | { type: 'Http3Quic'; zeroRttHandshake: boolean }
  | { type: 'Grpc'; multiplexedStreams: number }
  | { type: 'UdpUnreliable' }
  | { type: 'GossipUdp'; fanout: number; intervalMs: number };

export interface LatencyDistributionSpec {
  kind: 'Constant' | 'Uniform' | 'Normal' | 'LogNormal' | 'Exponential' | 'Weibull' | 'Pareto' | 'Bimodal';
  params: Record<string, number>;
}

export interface LinkCharacteristics {
  bandwidthBps: number;
  basePropagationDelayNs: number;
  jitterDistribution: LatencyDistributionSpec;
  packetLossRate: number;
  packetReorderRate: number;
  mtuBytes: number;
  maxInFlightBytes: number;
  duplex: boolean;
}

export interface NodePlacement {
  id: NodeId;
  label: string;
  region: RegionId;
  availabilityZone: string;
  canvasX: number;
  canvasY: number;
}

export interface NetworkEdge {
  id: EdgeId;
  source: NodeId;
  target: NodeId;
  protocol: TransportProtocolType;
  characteristics: LinkCharacteristics;
  partitioned: boolean;
  currentUtilizationBps: number;
  inFlightPacketCount: number;
}
