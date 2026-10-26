# ADR-0006: Defer low-value scope until a scenario requires it

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R10

## Context

The scaffold declares a very large model surface: 13 node types, 7 transport protocols, 4 consensus protocols, 5 isolation levels, 3 replication modes, 10 chaos faults and 3 fidelity modes. For a solo developer working phase by phase, building all of these spreads effort thin and delays the first working simulator.

## Decision

These items **stay in the type definitions** (so blueprints and the UI can reference them) but return `SimError::InvalidConfiguration("<feature> not yet supported")` until a scenario needs them:

| Area | Implemented (in roadmap phase) | Deferred |
|---|---|---|
| Node types | ClientPool, LoadBalancer, AppServer (P2); CacheCluster, Database (P5); ApiGateway (P6); ConsensusCoordinator (P8) | DnsResolver, CdnEdge, MessageQueue, StreamProcessor, SearchIndex, ObjectStorage |
| Consensus | Raft (P8d) | MultiPaxos, Zab, ViewstampedReplication |
| Transport | Fixed per-protocol overhead model: handshake RTTs, TLS cost, header bytes (P5) | TCP congestion-control simulation (Cubic/BBR/Reno dynamics), head-of-line blocking |
| LB algorithms | RoundRobin, LeastConnections, PowerOfTwoChoices (P2); ConsistentHashingRing (P8c) | WeightedRoundRobin, LeastResponseTime, Maglev |
| Isolation | Analytical penalty model only (P8e) | Per-transaction lock/MVCC simulation |
| Clocks | Single global virtual clock | ClockSkew / HLC (revisit with the InjectClockSkew fault) |

**Rule for un-deferring:** an item moves into scope only when a planned scenario (Phase 10) or an explicit user request needs it. Each un-deferral gets a short ADR.

## Consequences

- Each phase stays narrow and testable.
- The palette must visually mark unsupported node types (disabled with a "coming later" badge) so blueprints don't fail silently.
