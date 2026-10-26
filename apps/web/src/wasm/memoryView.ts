import { NodeTelemetrySnapshot } from '@/types';

export interface ZeroCopyLayoutDescriptor {
  nodeMetricsPtr: number;
  nodeMetricsLen: number;
  nodeStrideF64: number;
  edgeAnimationsPtr: number;
  edgeAnimationsLen: number;
  edgeStrideF32: number;
}

/**
 * Reads typed `Float64Array` and `Float32Array` slices directly from WebAssembly linear memory.
 */
export class TelemetryMemoryReader {
  constructor(
    protected readonly wasmMemory: WebAssembly.Memory,
    protected layout: ZeroCopyLayoutDescriptor
  ) {}

  public updateLayout(_newLayout: ZeroCopyLayoutDescriptor): void {
    throw new Error('Not implemented');
  }

  public readNodeMetrics(_nodeIndex: number): NodeTelemetrySnapshot {
    throw new Error('Not implemented');
  }

  public readEdgeFlowDensity(_edgeIndex: number): {
    utilizationRatio: number;
    packetVelocity: number;
    dropRateRatio: number;
  } {
    throw new Error('Not implemented');
  }
}
