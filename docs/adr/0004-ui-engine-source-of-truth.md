# ADR-0004: UI owns the design-time blueprint; the engine owns runtime state

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R7

## Context

Topology exists in two places: the Zustand `useTopologyStore` (for the canvas, inspector and undo/redo) and the Rust `SimulationEngine` (for simulation). Without a rule, the two drift apart: edits during a run, undo after a chaos fault, reload from a snapshot, and so on.

## Decision

1. **Design-time truth = `useTopologyStore`.** It holds the editable blueprint (node specs, placements, edges, config). Undo/redo operate only on this.
2. **Run-time truth = the engine.** It holds queues, in-flight packets, health, chaos effects, telemetry. The UI never writes runtime state. It only *reads* it through telemetry (ADR-0002).
3. **One-way sync with commands.** Every store mutation emits exactly one `UiEngineCommand` (`UpsertNode`, `RemoveEdge`, and so on) to the worker. The engine never pushes topology edits back to the store.
4. **Reset = reload.** `ResetSimulation` makes the engine discard runtime state and rebuild from the most recent blueprint the UI sent. Chaos faults are runtime state, so they don't modify the blueprint (an edge partitioned by chaos is still "present" in the design).
5. **Editing while running is allowed** and applied at the next event boundary. Structural edits to a node with in-flight requests drop those requests with a distinct status, so the effect shows up in telemetry.
6. **Export** reads from the store (design intent), not the engine. **Snapshots** (mid-run save) come from the engine and embed the blueprint they were built from.

## Consequences

- Undo/redo stays simple because it never has to roll back simulation state.
- The engine must handle any command sequence idempotently (upsert semantics).
- Two serialization paths exist on purpose: blueprint (store) and snapshot (engine).
