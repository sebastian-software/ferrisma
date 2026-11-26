# ADR-0002: Separate the semantic graph from the execution planner

- **Status:** Accepted
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-26
- **Supersedes:** None
- **Related:** [0001](0001-project-thesis.md), [0003](0003-demand-driven-regions.md), [0005](0005-codec-capabilities-and-pixel-contracts.md)

## Context

A pipeline must describe what image is required independently of scheduling and codec-specific optimization. Eagerly allocating output for each operation hides cross-stage opportunities; embedding execution policy inside each operator makes resource control inconsistent.

## Decision

Investigate an **immutable semantic graph** with nodes that describe output shape, pixel/color semantics, and backward dependencies. A separate planner receives demanded outputs, source/codec capabilities, constraints and a resource budget, and produces an **explainable execution plan**. Executor-specific decisions (CPU threads, region shape, copy/fusion points, optional GPU work) are not part of graph semantics.

Keep the graph **minimal**. A graph node is not necessarily a public trait and a graph need not be user-serializable. Preserve the option of single-chain fast paths within the same semantics. No mandatory generic compiler IR or cost-based optimizer in the first version.

The planner may reorder or fuse operations **only if output semantics are preserved**, including orientation, edge behavior, resampling quality, color transfer and alpha.

## Consequences

+ Optimizations can evolve independently from the user-facing API.
+ A plan can explain why a resize decoded an entire image, retained a layout, or materialized an intermediate.
- Introduces metadata and planner complexity; overgeneralization is a real risk.

## Alternatives considered

- **Each node owns execution and buffers:** simple for one operator; difficult for cross-operation optimizations.
- **A universal compute graph/DSL:** powerful but speculative and unrelated to the first use case.
- **Pure scanline-only pipeline:** attractive for simple resizes, too restrictive as the sole semantic abstraction for arbitrary crops, halos and format-specific access.

## Validation

Run the same crop + resize graph with synthetic scanline and region sources. Verify identical semantics, different valid execution plans, and no forced full-frame materialization. The graph representation must remain smaller than the executor code needed to exercise it.

## Review when

Graph-node metadata grows faster than supported behavior, or repeated experiments show the planner cannot meaningfully improve a fixed pipeline.

## Experiment: scalar single-chain prototype (2026-10-31)

The experimental code uses an append-only, index-addressed graph with internal node types for synthetic and decoder sources, planar sources, color transforms, crop and resize (`NodeKind` in `src/graph.rs`). Parent references always point to existing nodes, so cycles cannot be introduced through the builder. The graph is semantic: it stores dimensions, crop coordinates and resize intent, **not** execution buffers. `plan_region` walks backward from a requested output rectangle and returns region requirements in execution order. The CPU executor operates on that plan with no codec-specific knowledge.

**Not implemented:** a publicly exposed graph representation (the graph is `pub(crate)`), branching or multiple inputs, and a persistent execution IR. Scheduling and row caching live in the executors; color and pixel-layout decisions are settled per format by ADR-0013 to 0016, not by a negotiation step. The planner's `Vec<Step>` is intentionally disposable, not a stable interface.

**New tests:** a downstream crop after two resizes matches an independent eager reference; out-of-bounds requests are rejected before allocation.

## Amendment log

- 2026-10-31 — Initial proposal.
- 2026-10-31 — Added a bounded scalar vertical-slice experiment; unresolved contracts remain explicitly provisional.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
