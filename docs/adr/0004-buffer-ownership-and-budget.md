# ADR-0004: Make buffer ownership and memory budgets explicit

- **Status:** Experimental
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-11
- **Supersedes:** None
- **Related:** [0003](0003-demand-driven-regions.md), [0006](0006-cpu-first-gpu-capable-execution.md)

## Context

Avoiding full-frame intermediate images is primarily about memory movement and allocation, not literally using zero memory. Pipelines still need decoder state, filter halos, queues, output buffers and sometimes unavoidable materialization. Concurrency can multiply these costs. Rust makes ownership explicit but borrowing across asynchronous execution requires care.

## Decision

Investigate **owner-controlled allocations** and **borrowed plane/region views** for CPU-local work, with clearly defined strides, lifetimes and exclusive write access. Separate a logical image from a particular buffer or device allocation. Prefer transfer/reuse of ownership over copies, but do not promise zero-copy across incompatible layouts, device boundaries or external codecs.

The executor should operate under a declared **budget for engine-managed memory** and bound the number of in-flight jobs. Account for work buffers, caches, task queues and requested output. Report materialization points, copies and peak managed memory. Separately report estimated or observed opaque decoder/encoder allocations; a total-process hard cap is **not** guaranteed unless the adapter actually enforces one.

Avoid defining a global allocator, reference-counting strategy, or mandatory ringbuffer layout until a real vertical slice demonstrates the required ownership pattern.

## Consequences

+ Makes resource use visible and allows controlled degradation/fallback.
+ Reduces pressure to introduce `unsafe` for convenient borrowing tricks.
- Ownership, pinned work and cross-thread lifetimes can constrain scheduling designs.
- Opaque codec allocations remain outside strict engine control.

## Alternatives considered

- **Shared, mutable frame objects:** simple integration but encourages implicit copies and locks.
- **Single global memory pool:** may improve reuse but can obscure ownership and contention.
- **Hard promise of O(width) total memory:** invalid for some formats, filters, encoders and decoder internals.

## Validation

Instrument managed buffer allocations, copied bytes and peak live bytes. Run large-height images at fixed width to check expected intermediate-space scaling for streaming-compatible operations; test multi-image backpressure. Test decoder failures, cancellation and out-of-budget behavior without leaks or deadlocks.

## Review when

Allocator instrumentation reveals excessive borrowing/ownership overhead, or a targeted codec's memory requirements make the model misleading.

## Experiment: synchronous ownership and budget (2026-10-31)

The first executor holds at most its required input region and output region for a scalar transform. The crop operation changes the tile's logical coordinate origin **without copying bytes**. An output sink receives a borrowed row slice and cannot retain it through the callback's type. There is no forced destination image allocation inside the engine.

An `Arc`-shared, atomically reserved byte budget tracks the **engine's owned pixel buffers**, using RAII to return reservations on success, cancellation, or allocation failure. The counter enforces a limit across concurrent holders and records a reservation high-water mark. Graph metadata, planner vectors, callback-owned data, allocator overhead and external codecs are **not** included; this is not a total process/RSS bound. Until these other allocations are explicitly modeled, claims must say *managed pixel-buffer budget*, not *strict engine memory limit*.

**New tests:** allocation failure returns an error without leaked reservations, early sink cancellation releases buffers, and a shared budget prevents two simultaneous reservations from exceeding the limit.

## Amendment log

- 2026-10-31 — Initial proposal.
- 2026-10-31 — Added a bounded scalar vertical-slice experiment; unresolved contracts remain explicitly provisional.

## Amendment: resize-plan admission (2026-11-11)

The 2026-10-31 experiment excluded planner vectors. That left a reachable
allocation before the executor allocated a managed row: region planning built
one `f64` tap vector per output coordinate, and both executors retained tap
and scheduling tables. On the prior implementation a bounded 10,000×1 Area
downscale to 1×1 succeeded with a 64-byte managed budget although the one
10,000-entry tap vector required at least 160,000 bytes. The regression test
fails on that behavior and now rejects the request before decoder work.

Both working-linear resize executors now reserve a checked, conservative
planning allowance before region planning, retaining it through execution.
For each axis, the allowance bounds each output's source span by the input
size and the filter support at the rounded-up scale. Named, type-size-based
components charge temporary and reference `f64` taps, flattened `f32` tables,
the SIMD weight copy, row ranks and needed-row lists, reference cache
records, shared-row descriptors with bookkeeping slack, parallel vertical-job
references, and per-position vector and offset slots. The bound sums the f64
and f32 components even though the two executors do not run simultaneously;
flat tables and cache descriptors reserve bounded capacities up front, and the
SIMD copy reserves its checked exact span. Arc bookkeeping is an allowance,
not a promise about the standard library's private layout. Per bounded entry,
the charge sums two `Tap` sizes, two index/weight pairs plus one SIMD weight,
two `usize` and two `u32` rank slots, two cached-row descriptors for capacity
slack, two shared-row descriptor allowances, and a row slot plus a vertical-job
reference. Per output position or boundary slot, it adds one tap-vector header,
two `usize` offsets, and four `u32` offsets. The types' sizes are calculated on
the target platform. It intentionally overestimates and can reject a request
whose eventual allocation would fit. Pixel buffers
continue to have separate reservations. Tap and major planning-vector
allocations use fallible reservation; cancellation and errors release the
planning charge through RAII. Existing output, precision, and execution-order
contracts do not change.

This is a bounded admission charge for those modeled allocations, not a hard
process-memory ceiling or an exact allocator accounting. Graph nodes, allocator
overhead and rounding, `BTreeMap` and channel/runtime internals, worker stacks,
caller-owned memory, and opaque codec allocations remain outside this model. It also does
not limit execution time or establish a general thread-count limit. Reported
managed-memory peaks now include the planning allowance and are not directly
comparable to historical pixel-buffer-only peaks.

Validation includes the bounded pre-fix reproducer, extreme up/downscale
rejection before decoder calls, an exhaustive small-geometry check that the
tap-span bound covers both filters, shared-budget release, existing sink
cancellation, and f32/f64 plus sequential/parallel output comparisons. No
speed, memory-reduction, or total-RSS claim follows from this change. Revisit
when actual planner storage is tracked exactly, the supported filters or graph
shapes change, or the conservative bound rejects representative valid jobs.

- 2026-11-11 — Added conservative resize-plan admission before footprint
  construction; this narrows the planner-vector exclusion recorded in the
  2026-10-31 experiment without replacing its historical evidence.
