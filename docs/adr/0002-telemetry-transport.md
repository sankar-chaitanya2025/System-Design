# ADR-0002: Telemetry crosses the worker boundary as per-frame transferable copies

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R5

## Context

The scaffold describes "zero-copy" telemetry: `SharedTelemetryBuffers` in `sim-wasm` exposes raw pointers, and `TelemetryMemoryReader` on the main thread wraps `Float64Array`/`Float32Array` views over Wasm linear memory.

But the engine runs inside a **Web Worker** (`sim.worker.ts`). The main thread can only view the worker's Wasm memory if that memory is a `SharedArrayBuffer`. That requires building the Wasm module with shared memory (`+atomics,+bulk-memory`, `-Z build-std`), which only works on **nightly** Rust. `rust-toolchain.toml` pins **stable**. The COOP/COEP headers in `vite.config.ts` are necessary for this but not sufficient.

## Decision

1. Inside the worker, the engine fills flat `f64`/`f32` slabs (`SharedTelemetryBuffers::sync_from_engine`) at the end of every tick. The `TelemetryMemoryReader` concept is kept, but it runs **inside the worker**, where the views really are zero-copy.
2. Each frame, the worker copies the slabs into a fresh `ArrayBuffer` and sends it with `postMessage(..., [buffer])` as a **transferable**, which costs no structured-clone copy.
3. The main thread reads node and edge metrics through a small reader with the same API as `TelemetryMemoryReader`, backed by the received buffer.
4. Low-frequency, structured data (`WasmFrameDigest`: global metrics, anomalies, sampled traces) still goes through `serde-wasm-bindgen` and `postMessage`. It is throttled to the telemetry window, not every frame.
5. Slab layout (stride, field order) is defined once in Rust and exported as constants through ts-rs (see ADR-0001).

## Consequences

- Works on stable Rust with no special build flags.
- Cost per frame is roughly `nodes × stride × 8` bytes. At 1,000 nodes × 16 fields that is about 128 KB/frame (about 7.7 MB/s at 60 FPS), which is negligible.
- The COOP/COEP headers aren't needed for this design. They are kept for now so a later move to shared memory doesn't change hosting requirements, but they block cross-origin assets (fonts, CDNs) unless those assets send CORP/CORS headers. Self-host fonts.
- **Revisit trigger:** if profiling shows the copy or transfer above 1 ms per frame, or node count goes above 10k, re-evaluate nightly shared memory.
