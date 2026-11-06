# ADR-0022: Parallel row pipeline and an engine for many images

- **Status:** Experimental
- **Date:** 2026-11-06
- **Last reviewed:** 2026-11-06
- **Supersedes:** None
- **Related:** [0003](0003-demand-driven-regions.md), [0004](0004-buffer-ownership-and-budget.md), [0006](0006-cpu-first-gpu-capable-execution.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0021](0021-working-precision-policy.md), [M2 milestone](../milestones/M2-performance-architecture.md) work package 4, [M2 baseline](../benchmarks/M2-baseline.md)

## Context

After work packages 1–3 the transform stage runs on one core. On the Apple M1 Ultra (20 cores) a 48 Mpx JPEG to 1600-wide AVIF spends 170 ms decoding, 191 ms transforming and 140 ms encoding with all encoder cores; CPU/wall is 3.8. The transform is the largest stage that does not yet use the machine. Its work splits into independent pieces (measured on 48 Mpx, `sample`): per **source row**, linearization (about 24 %) and the horizontal pass (about 57 %); per **output row**, the vertical pass (about 10 %) and quantization (about 9 %).

Verified constraints:

- Decoders deliver rows **sequentially** through a borrowing callback (`DecodeSource::read_region`, ADR-0012). JPEG and AVIF adapters decode the full frame first and then stream rows; JPEG XL renders a region. The callback cannot hand a borrowed row to another thread.
- The AVIF sink requires rows **in order** (`AvifSink::write_row`).
- The core crate forbids `unsafe` and has one unconditional dependency, the first-party kernel crate. `rayon` is already in the lockfile through `rav1e`, but not a dependency of the core.
- Every horizontal row and every vertical row is a pure function of its inputs and the plan. The `f32` kernels are deterministic per level; the order of taps within a row is fixed.
- `MemoryBudget` is a shared atomic counter, already safe across threads (ADR-0004).

The M2 plan sketched *bands with filter halos*: split the output into horizontal bands, give each band to a worker that decodes, filters and emits its rows. That shape fits a decoder with random row access; ours stream once from the top.

## Decision

1. **A row pipeline, not bands.** The thread that drives the decoder copies each needed source row into an owned, budgeted buffer and submits it as a *horizontal job*. Workers linearize the row and run the horizontal kernel into a new, budgeted horizontal row, shared read-only (`Arc`). As soon as every horizontal row an output row needs exists, the driver submits a *vertical job* with those rows and weights; a worker runs the vertical kernel and quantizes into a budgeted output row. The driver puts finished output rows into a reorder buffer and hands them to the sink strictly in order. No row is computed twice and no halo is duplicated.
2. **Bit-identical to the sequential executor.** Each job runs exactly the kernels and operation order of the sequential path; only the thread differs. A test compares both executors byte for byte across geometries, crops, alpha and levels.
3. **Bounded memory and backpressure.** Jobs go through a bounded queue (twice the worker count); the driver blocks when it is full, so the decoder cannot run arbitrarily far ahead. Horizontal rows are dropped as soon as no remaining output row references them (the same `future_min` rule as the ring of the sequential path). Every row buffer is charged to the shared budget; exceeding it fails the run with `ResourceExhausted`, as before.
4. **Errors and cancellation stop everything.** A worker error, a decoder error or a sink error (including `Cancelled`) closes the queue; workers finish their current job and exit; the scope joins them before the call returns. No thread outlives the call (`std::thread::scope`).
5. **Standard library only.** `std::thread::scope`, `std::sync::mpsc` and `Arc`. No `rayon` in the core: the pipeline needs ordered emission and a bounded queue more than work stealing, and adding a scheduler crate is a separate decision.
6. **Threads are an option, defaulting to all cores.** `TransformOptions::transform_threads` (0 = all available cores, 1 = the sequential path). Because the output is bit-identical, the default does not trade determinism for speed. Small images (fewer than 64 needed source rows) take the sequential path. The report names the threads used.
7. **An `Engine` for many images (second step).** A long-lived `Engine` owns a worker pool and a budget, and offers a batch call that schedules *images* across workers with one transform and one encoder thread each, which the encoder experiment showed to be the most byte-efficient setting. Its API is specified when implemented, in this ADR's amendment log.

## Consequences

+ The transform stage scales with cores without changing a single output byte.
+ The decoder contract, the sink contract and the budget semantics stay as they are.
+ Memory is bounded by the plan's row window plus the queue depth, both reported through the budget peak.
- One extra copy per needed source row (the decoder's borrowed row into an owned buffer): about 18 KB per 6000-pixel RGB row, small against the horizontal work on it.
- Buffer allocation per row instead of a fixed ring; a buffer pool can replace it if profiles show allocator cost.
- The driver thread is serial: for decoders that stream slowly, the decoder becomes the limit, which is the decode-side work of package 5.
- Thread creation per call (about 20 threads) costs fractions of a millisecond; the `Engine` amortizes it for batches.

## Alternatives considered

- **Bands with halos (the M2 sketch).** Rejected for now: it needs random row access or a full decoded frame shared across workers, duplicates halo rows at every band edge (about 23 rows per edge at a 3.75× Lanczos3 downscale), and makes in-order emission a cross-band handoff. Reconsider for decoders with region access (JPEG XL, decode-side scaling in package 5).
- **`rayon`.** A fine work-stealing pool, and already compiled for `rav1e`. Not needed for a pipeline of independent row jobs with ordered output; deferred until the `Engine` needs nested parallelism.
- **Parallelize only the horizontal pass.** Simpler, but leaves the vertical pass and quantization (about 19 %) serial on the driver.
- **Split the decoded frame into strips for JPEG and AVIF**, since those adapters hold the full frame anyway. Faster for those two, but it bypasses the decoder contract and does not apply to JPEG XL; the row pipeline covers all three.

## Validation

- Byte-identical output between the sequential and the parallel executor for every test input of the ADR-0021 differential suite, at every available kernel level and for 1, 2, 3 and 8 threads.
- Cancellation from the sink and budget exhaustion release all memory (`live == 0`) and return the same errors as the sequential path.
- Scaling of the transform stage from 1 to all cores on the dev machine and on the CI runners, recorded in the M2 baseline. The decision is falsified if the transform does not get at least 4× faster on 20 cores for the 24 Mpx case, or if peak memory grows by more than the queue depth times a row.

## Review when

Decode-side scaling (package 5) gives decoders region access; the `Engine` needs nested or prioritized parallelism; the allocator shows up in profiles; a GPU backend (ADR-0024) needs a different job granularity.

## Amendment log

- 2026-11-06 — Initial proposal: row pipeline with ordered emission, bit-identical to the sequential executor, standard library only; `Engine` for batches as the second step.
- 2026-11-06 — Implemented; status Experimental. `src/parallel_execute.rs` runs the row pipeline with `std::thread::scope`, a bounded job queue of twice the worker count and an unbounded result channel; `TransformOptions::transform_threads` (default 0 = all cores) and `ProcessingReport::transform_threads`; CLI `--transform-threads`. Tests: byte-identical output to the sequential path for four inputs (RGB, Display P3, RGBA, extended-range `f32`), six geometries including a crop, both filters, the best and the scalar kernel level and 2, 3 and 8 threads; rows reach the sink in order; a cancelling sink, an exhausted budget and a worker error (non-finite float) all return the sequential path's error with `live == 0`. Transform stage on the Apple M1 Ultra (best of three, one encoder thread): 24 Mpx 104 ms on one thread, 59 on 2, 33 on 4, 20 on 8, 19 on 20 (5.5×); 48 Mpx 194, 109, 57, 31, 29 ms (6.7×). The validation threshold (4× at 24 Mpx) holds. Scaling flattens beyond 8 threads because the driver thread, which copies each source row and dispatches jobs, is serial; peak RSS rose by at most 8 MiB.
- 2026-11-06 — `Engine` and `BatchItem` added (decision 7): `Engine::new(threads)` and `Engine::transcode_batch(&[BatchItem])` schedule whole images over scoped worker threads, each image with one transform and one encoder thread, results in input order, failures isolated per image. Every batched image is byte-identical to `transcode` with those two options at 1 (tested). Threads are scoped to the batch call rather than a long-lived pool; per-image budgets stay, so the engine's managed memory is bounded by workers times the largest image budget. 100 synthetic 640×427 JPEGs to 320 wide: 221.6 images/s through the engine against 15.6 sequentially in one process and 13.9 with one process per image.
- 2026-11-06 — Horizontal jobs apply a source conversion that the executor took over from the source (`RowConversion`, ADR-0023) before linearizing, on the worker. For ICC input the driver now copies the decoder's 8-bit row instead of a converted `f32` row, a quarter of the bytes, and the moxcms transform leaves the driver thread. The sequential path applies the same conversion in the same place, so the output stays byte-identical.
