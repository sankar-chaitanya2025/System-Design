//! `sim-wasm`: WebAssembly FFI boundary and Zero-Copy Shared Ring Buffers
//! connecting `sim-core` to the React/TypeScript UI and WebWorker host.

pub mod api;
pub mod commands;
pub mod events_out;
pub mod panic_hook;
pub mod shared_buffer;

pub use api::WasmSimulatorHandle;
pub use shared_buffer::ZeroCopyTelemetryLayout;
