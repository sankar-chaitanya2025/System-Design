# SysDesign-Sim: Distributed Systems Discrete-Event Simulator

A browser-native, production-grade **Discrete-Event Simulation (DES)** engine and interactive topology studio for distributed systems architecture, scaling from foundational load-balanced clusters to advanced *Designing Data-Intensive Applications (DDIA)* workloads.

## Architecture Highlights
- **Hybrid Micro-Packet & Macro-Fluid Engine (`crates/sim-core`)**: Seamlessly transitions from discrete individual request tracing (`<10k RPS`) to continuous fluid-density differential equations and queueing theory (`M/M/c/K`, Kingman's approximation) for `1M+ concurrent users`.
- **Zero-Copy Shared Telemetry (`crates/sim-wasm`)**: Exposes typed ring buffers (`Float64Array`, `Uint32Array`) directly over WebAssembly linear memory to feed 60 FPS canvas animations and real-time HDR latency histograms (`p50`, `p95`, `p99`, `p99.9`).
- **DDIA Primitives**: Consensus protocols (Raft, Multi-Paxos), tunable replication consistency (`W + R > N`), storage engines (LSM-Tree SSTable compaction vs. B+Tree WAL), vector clocks, and interactive Chaos Engineering injectors.
