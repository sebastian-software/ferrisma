# ADR-0004: Make buffer ownership and memory budgets explicit

- **Status:** Accepted
- **Date:** 2026-10-31
- **Last reviewed:** 2026-11-26
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

## Open questions

- Codec-internal memory is only partly counted, and the AVIF and WebP writers reserve their frames only: tracked in #48 (estimate and report codec-internal memory) and #199 (AVIF decode memory).

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

## Amendment: default budget of 1 GiB (2026-11-15, owner decision)

**Decision (owner, 2026-11-15; not a working decision).** The default
managed-memory budget, `TransformOptions::max_managed_bytes` and the CLI's
`--budget-mb`, rises from 256 MiB to 1 GiB.

**Why.** The budget is the engine's second, finer line, for operators who
plan memory per job. Protection against decompression bombs is already the
input pixel limit (`max_input_pixels`, default 100 MP), which is what
comparable libraries rely on (Pillow and sharp/libvips limit pixels). At
256 MiB the default made legitimate work fail: lossless JPEG XL above about
3.7 MP (3,655,036 pixels opaque, from the measured chunked-encode model of
2026-11-15), and the deeper output rows of the same day (ADR-0021
amendment: 8 B/px instead of 4 for every sink frame) would have tightened it
further. An operator who wants a strict bound lowers it.

**What the budget covers** is unchanged: engine buffers, the encoders'
reserved frames and the measured lossless JPEG XL peak are counted; memory
that codecs allocate internally is only partly counted (issues #48 and
#199). It is a bound on what the engine manages, not on the resident memory
of the process.

**Consequences.** Under the default, the largest lossless JPEG XL output
grows from 3,655,036 to 49,344,752 pixels (opaque) and from 2,842,806 to
38,377,881 (with alpha); `supported_matrix::default_budget_rejects_lossless_jpeg_xl_above_about_49_megapixels`
replaces the 3.7 MP test. A caller that relied on the old default as a
bound must set it explicitly.

- 2026-11-15 — Default budget 1 GiB (owner decision); see the amendment above.

## Amendment: JPEG XL reservations follow measured peaks (2026-11-16)

**Finding.** A reservation is the engine's promise that an admitted encode
fits the caller's budget. Measured as the JPEG XL sink holds it
([record](../benchmarks/v1.0-jxl-encoder-memory-2026-11-16.md)), the
lossless model of 2026-11-15 was up to about 10 % below real peaks
(8-bit RGBA at 12.2 MP; noise at 6 and 12 MP; one-shot noise frames), and
the lossy sink reserved a source-derived estimate at less than a tenth of
its peak. A caller who set a budget could be exceeded without an error,
against this ADR and ADR-0042 D3.

**Change.** The JPEG XL sink reserves a measured model for both profiles
(ADR-0039 amendment of 2026-11-16): a constant, a per-frame term, the first
DC group's working set and a term for the second DC group, whose resident
peak includes memory the allocator kept from the first; fitted 10 % above
every recorded peak, photographs and noise, and pinned by
`jxl_output_tests::model_covers_every_measured_peak_with_margin`.

**Consequences.** Under the default 1 GiB the largest JPEG XL output is
49,133,275 px lossless opaque (was 49,344,752), 30,670,848 with alpha (was
38,377,881), 24,292,010 and 13,939,892 above 8 bits (were 28,272,715 and
20,185,088), and lossy 40,544,938 opaque, 25,547,124 with alpha,
22,282,240 and 11,240,734 above 8 bits (about 144, 127, 74 and 65 MP from
the estimate before). Lossy requests between those sizes that passed before
now fail with `ResourceExhausted` before encoding. What the budget covers is
otherwise unchanged: the AVIF and WebP writers still reserve their frames
only; measured on the same day at 2.4 to 9 (AVIF) and 2.3 to 44 (WebP)
times those frames (indicative, not changed; #48, #199).

- 2026-11-16 — JPEG XL reservations follow measured peaks; see the amendment above.

## Amendment: default budget of 2 GiB (2026-11-16, owner decision)

**Decision (owner, 2026-11-16; not a working decision).** The default
managed-memory budget, `TransformOptions::max_managed_bytes` and the CLI's
`--budget-mb`, rises from 1 GiB to 2 GiB.

**Context** (the decision as relayed records no reason of its own; these
are the facts around it). The reasons of the 1 GiB amendment stand (the
budget is the finer line for operators who plan memory per job;
decompression bombs are bounded by the input pixel limit), and the measured
JPEG XL reservations of 2026-11-16 (the amendment above) lowered what 1 GiB
admits: lossy JPEG XL
above about 40.5 MP opaque, and 11.2 MP with alpha above 8 bits, failed
under the default where it had passed with the estimate. 2 GiB admits those
sizes again with the measured model.

**What the budget covers** is unchanged (the amendment above).

**Consequences.** Under the default the largest JPEG XL output is, by the
committed models (`LOSSLESS_PEAK`, `LOSSY_PEAK`, `PeakModel` in
`src/jxl_sink.rs`): lossless 125,829,120 px opaque (was 49,133,275 at
1 GiB), 97,779,712 with alpha (was 30,670,848), 69,031,253 and 45,520,534
above 8 bits (were 24,292,010 and 13,939,892); lossy 159,849,585 opaque (was
40,544,938), 123,160,017 with alpha (was 25,547,124), 89,391,104 and
54,190,407 above 8 bits (were 22,282,240 and 11,240,734). Every one of these
lies above the largest measured frame (24 MP) and extrapolates the model's
per-frame slope; three of the 1 GiB limits (13.9, 22.3 and 11.2 MP) were
inside the measured range. `supported_matrix::default_budget_rejects_lossless_jpeg_xl_above_about_126_megapixels`
(11300×11200) replaces the 49 MP test (7100×7000). A caller that relied on
the 1 GiB default as a bound must set it explicitly; on a machine where
2 GiB of engine buffers is too much for the jobs it runs in parallel
(`Engine` runs `threads` images, each with its own budget), the operator
lowers it.

- 2026-11-16 — Default budget 2 GiB (owner decision); see the amendment above.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
