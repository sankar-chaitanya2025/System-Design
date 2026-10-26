# ADR-0003: Split topology structure from node runtime state inside the engine

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R6

## Context

In the scaffold, `TopologyGraph` owns both the graph structure (placements, edges, adjacency) **and** the node runtime state (`HashMap<NodeId, NodeKind>`). When the engine dispatches an event, a node handler needs, all at once:

- `&mut` access to **its own** state (queues, worker slots, counters),
- **read** access to its outgoing edges and neighbours (to route and pick backends),
- `&mut` access to the event queue (`EventScheduler`), the RNG and the metrics sink.

If the node is borrowed mutably out of `TopologyGraph`, the graph can't be read at the same time. That forces clone-heavy code or `RefCell` everywhere.

## Decision

1. `SimulationEngine` holds **separate fields**:
   - `topology: TopologyGraph`: structure only (placements, `NetworkEdge`s, adjacency lists, link runtime counters).
   - `nodes: NodeStore`: owns `NodeKind` runtime state keyed by `NodeId`.
   - `event_queue`, `rng`, `telemetry`, `fault_engine`, unchanged.
2. `EventContext` gains a **read-only topology view** (`&TopologyGraph`) next to the existing `scheduler`, `metrics` and `rng`. Because these are disjoint struct fields, Rust's split borrows allow `&mut nodes[id]` plus `&topology` plus `&mut event_queue` at the same time without `unsafe` or interior mutability.
3. Nodes never mutate other nodes directly. Everything between nodes is an **event**. This keeps execution deterministic and the event log replayable.
4. Edge runtime counters (utilization, in-flight count) are only mutated by the engine while processing link events, never by node handlers.
5. `TopologyGraph::add_node`/`remove_node` keep their public signatures for the UI command path. Internally the engine forwards node state to `NodeStore`.

## Consequences

- Clean borrow structure with no `Rc<RefCell<_>>`.
- `TopologyGraph`'s public API changes slightly (node lookups move to `NodeStore`). This lands in Phase 2 when both are first implemented.
- Blueprints still serialize nodes and placements together. The split is an in-memory concern only.
