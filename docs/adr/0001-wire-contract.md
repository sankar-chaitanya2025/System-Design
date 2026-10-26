# ADR-0001: Rust is the canonical wire contract; TypeScript types are generated

- **Status:** Accepted
- **Date:** 2026-10-26
- **Resolves:** Review finding R3, R4

## Context

The scaffold hand-wrote parallel type definitions in Rust (`crates/sim-core`, `crates/sim-wasm`) and TypeScript (`apps/web/src/types`). They disagree:

- Rust serializes struct fields as `snake_case`; TS expects `camelCase`.
- Rust enums use serde's default *externally tagged* representation (`{"UpsertNode": {...}}`); TS expects *internally tagged* unions and uses three different discriminator keys (`type`, `mode`, `status`).
- `LatencyDistributionSpec` is a typed enum in Rust and a loose `{ kind, params: Record<string, number> }` in TS.
- `TransportProtocol::Tcp` has `congestion_control` in Rust but not in TS.
- Rust uses `u64` hashes and a `u128` `TraceId`. JS `number` silently loses precision above 2^53, and TS declares `TraceId` as `string`.

Left alone, every `UiEngineCommand` and `WasmFrameDigest` would fail to deserialize at the boundary.

## Decision

1. **Rust is the single source of truth** for every type that crosses the Wasm boundary or is persisted (blueprints, snapshots).
2. **TS types are generated with [`ts-rs`](https://github.com/Aleph-Alpha/ts-rs)** into `apps/web/src/types/generated/`. Hand-written files in `apps/web/src/types/` are replaced by re-exports of the generated types as each module is implemented. Generated files are committed so the frontend builds without a Rust toolchain. CI fails if regeneration produces a diff.
3. **Serde conventions** for all wire types:
   - Structs: `rename_all = "camelCase"`.
   - Enums with data: internally tagged, **always with the discriminator key `"type"`**, variant names kept in PascalCase.
   - Unit-only enums serialize as plain PascalCase strings.
4. **Integer encoding:**
   - Virtual time (`SimTime`, `SimDuration`) stays `u64` nanoseconds in Rust and is emitted as a JS `number`. The engine treats 2^53 ns (about 104 days of virtual time) as a hard ceiling and returns `SimError::InvalidConfiguration` beyond it.
   - Opaque identifiers and hashes that can exceed 2^53 (`routing_key_hash`, `TraceId`, `SpanId`) serialize as **decimal strings**.
   - Small ids (`NodeId`, `EdgeId`, `RegionId`, `ShardId`, `ChaosFaultId`) are `u32`/`u16` and serialize as `number`.
5. **Transport encoding:** `serde-wasm-bindgen` for commands and digests (no JSON string round-trip), `serde_json` for blueprint/snapshot files.

## Consequences

- One edit in Rust updates both sides, so the two languages can't drift apart.
- Frontend developers read generated types; changes to wire types require a Rust change.
- Adds `ts-rs` as a `sim-core`/`sim-wasm` dependency. Annotations are applied module by module as each phase implements it, not all at once.
- The scaffolded TS type files are now provisional and will be overwritten.
