//! `sim-core`: Deterministic Discrete-Event Simulation (DES) & Fluid-Density Engine
//! for Distributed Systems Architecture and DDIA Failure Modeling.

pub mod chaos;
pub mod config;
pub mod consistency;
pub mod engine;
pub mod error;
pub mod event;
pub mod math;
pub mod network;
pub mod nodes;
pub mod payloads;
pub mod scenarios;
pub mod serialization;
pub mod telemetry;
pub mod time;
pub mod topology;

pub use config::SimulationConfig;
pub use engine::SimulationEngine;
pub use error::{SimError, SimResult};
pub use event::{Event, EventId, EventPayload, EventPriority};
pub use time::{SimDuration, SimTime};
pub use topology::{EdgeId, NodeId, TopologyGraph};
