# ADR-0033: One SharpYUV conversion core for WebP and AVIF

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
- **Amends:** [0026](0026-codecs-in-one-repository-and-workspace.md), SharpYUV consolidation review
- **Related:** [0027](0027-tiered-validation-and-rust-test-orchestration.md), [extraction evidence](../research/2026-11-10-sharpyuv-extraction-evidence.md), issue [#75](https://github.com/sebastian-software/ferrixel/issues/75)

## Context

Both codec ports use SharpYUV 0.4.2 from the same pinned libwebp 1.6.0 source. The safe WebP translation and the generated AVIF C ABI translation had separate conversion loops. The prior cross-port evidence compared 2,880 ordinary cases and signed layouts against the pinned C oracle. Their public boundaries are different: WebP owns checked byte slices and runtime SIMD dispatch; AVIF exports the C ABI, independent CPU callback/init state, static matrix symbols, and seven observable `malloc` calls.

## Decision

Extract the safe WebP numeric conversion, gamma, color-matrix, and portable DSP into the small `libsharpyuv-core` workspace crate. The core forbids unsafe code and owns no C symbols or codec-wide state. It receives sample access, seven initialized scratch buffers, and DSP functions from its caller. Keep the safe WebP module paths and signatures as wrappers; they retain bounds-checked slice indexing, fallible Rust allocations, and existing SIMD dispatch. Like the original safe API, an undersized slice or invalid origin panics when indexed, possibly after scratch allocation; this extraction does not change that behavior. Keep the AVIF C ABI and generated source archive in its package. Its hand-audited adapter retains C init/callback and matrix symbols, uses seven C `malloc` calls and original cleanup order, and implements raw sample access. It initializes allocated scratch before constructing Rust slices. The production AVIF conversion calls the one shared numeric loop. The generated translation and manifest hashes remain immutable so it remains an independent mechanical reference.

The AVIF package retains generated C-facing gamma, color-matrix, and DSP helper symbols for ABI compatibility. Those routines are not the production conversion loop; its numeric work goes through the shared core. These helpers and C oracle remain independent checks of the extraction. The core is a workspace implementation detail, not an engine API or a dependency on the entire WebP codec.

## Alternatives

- Depend on the whole WebP port from AVIF: rejected because codec ownership and CPU callback state would couple unrelated ports.
- Use the generated AVIF loop as the common implementation: rejected because its raw pointers, C allocator, and generated arithmetic exceptions would spread into WebP's safe API.
- Remove the generated AVIF source: rejected because the immutable translation and its source hashes are the provenance and comparison baseline.
- Share C pointers directly with the core: rejected because a safe crate cannot assume pointer extents, alignment, or valid Rust references from the C ABI.

## Consequences

The numeric loop has one production owner. AVIF still has an audited unsafe boundary because the C ABI does not provide slice extents; C callers retain their original memory-validity contract. The adapter zero-initializes all seven allocations before making typed slices, potentially adding work and touching pages earlier than the former generated loop. Allocation count, failure result, and free order are kept. The core trait and scratch types are public only to permit adapters in separate crates; they are not a promise of a stable general-purpose library API. Per-codec wrappers preserve WebP's safe interface and AVIF's C symbols.

## Validation

Local macOS ARM64 evidence: cross-port tests pass 2,880 combinations plus signed layouts and defined rejection cases in `checked` and `release`; the pinned C oracle passes 1,271 API/corpus groups in debug and release; 23 ABI checks pass both profiles; injected failure at all seven allocation calls preserves cleanup; CPU callback and linkage checks pass; the 16 signed-layout C hashes match. WebP's focused SharpYUV conversion, matrix, gamma, CPU, DSP and SIMD checks pass. Native GNU Linux and complete original WebP consumer verification remain required before adoption. The [matched before/after timing and RSS evidence](../research/2026-11-11-sharpyuv-shared-core-evidence.md) shows about 16–21% slower AVIF conversion and about 11–16% faster WebP conversion in two local workloads after the safe core borrows its sRGB tables once per conversion. The AVIF performance cost is an explicit adoption limit, not a speed claim for other inputs.

## Review when

Review after native Linux and full original consumer gates, if either codec changes SharpYUV version or CPU dispatch policy, if the C ABI offers checked buffer extents, or if measured time/peak memory warrants a narrower scratch representation. Re-run the independent C oracle and immutable-source checks for every numeric change.

## Amendment log

- 2026-11-11 — Extracted the single safe production conversion core and kept format-specific adapters and the generated mechanical reference. Adoption remains conditional on the stated gates.
