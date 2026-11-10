# Migration contract

## Authority and delivery

The requested outcome is a mechanical Rust port in a new local repository,
with English documentation, recorded decisions, unchanged upstream tests, and
separate Conventional Commits. The repository is public under
`sebastian-software/libavif-rs`. Execution is
sequential because the translation and test-linkage rules must be proven before
scaling the conversion.

## Equivalence boundary

Preserve public and internal C ABI contracts, AVIF bytes, error results and
messages, metadata, image planes, encoder/decoder behavior, I/O callbacks,
cleanup, and threading behavior. Preserve the complete upstream test tree and
its feature gates. SharpYUV, the portable dav1d decoder, and portable libaom
are delivered by the Rust workspace crates `crates/sharpyuv`, `crates/dav1d`,
and `crates/aom`; the original C builds remain their independent references
(ADRs 0010 to 0019). The optional engines (libgav1, rav1e, SVT-AV1, full external
libyuv, and AVM) remain native dependencies as they are upstream; their adapter
code belongs to the port. Applications and C/C++ test harnesses remain original
consumers of the port.

## Semantic mapping

| Concern | Rule |
| --- | --- |
| Layout and ABI | Use C representation, C-width types, and original calling conventions. |
| Ownership | Keep original allocation/free pairs and callback ownership. |
| Numbers | Preserve casts, unsigned wrapping, shifts, and floating-point evaluation order. |
| Text | Keep byte strings, NUL termination, and original diagnostics. |
| Errors | Keep return values, early exits, and cleanup paths; do not unwind across C. |
| Assertions | Preserve upstream debug/release semantics and document translation differences. |
| Features | Record the exact preprocessor configuration used for each translation. |
| Platforms | Validate each platform explicitly; a host build does not prove portability. |

## Completion gates

1. Record exact upstream provenance and verify the untouched snapshot.
2. Build and execute the original configured C baseline, recording test counts.
3. Translate all library code for the selected configuration with no C fallback.
4. Compile and link the Rust implementation through the original headers.
5. Execute the identical original suite against Rust and compare results/counts.
6. Cover remaining upstream feature configurations and supported platforms.
7. Record sanitizer, fuzzing, and release verification gaps explicitly, as
   GitHub issues.
