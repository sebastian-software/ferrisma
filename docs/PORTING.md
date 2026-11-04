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
its feature gates. AV1/AV2 codec implementations and external libraries remain
dependencies as they are upstream; their adapter code belongs to the port.
SharpYUV, the portable dav1d decoder, and portable libaom are now delivered by
separate Rust workspace crates. Their standalone references and evidence are
linked from `NATIVE-PORTS.md`. The remaining
engine dependencies follow the accepted sequence in `NATIVE-PORTS.md`.
Applications and C/C++ test harnesses remain original consumers of the port.

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
7. Record sanitizer, fuzzing, and release verification gaps explicitly.

## Status

All 27 upstream C library files have Rust implementations, including the six
codec adapters and the Mini, Extended-PIXI, and AV2 container branches. The five
bundled scalar libyuv files are also translated; external libyuv remains an
independent optional dependency exactly as upstream.

The default macOS AArch64 configuration passes the same 64 CTest registrations
containing 9,473 GoogleTest cases in C and Rust. The complete supported feature
configuration passes 70 registrations containing 9,555 GoogleTest cases in each.
The default AddressSanitizer comparison passes all 64 registrations. The port
test build compiles no C libavif implementation; upstream C/C++ consumers and
fixtures remain unchanged.

Source translation and host parity gates are satisfied for the recorded
configurations. Remaining gates concern other platforms, optional golden/fuzz/
compliance harnesses, the optional C++ `src/compliance.cc` integration,
additional dependency/build matrices, and numeric equivalence on additional
compilers and targets. The ten recorded decoded PNG scenarios now match the
primary C reference exactly with the compiler contract in ADR 0024.
No full cross-platform or release-certification
claim is made. See `VALIDATION.md` for the exact evidence and residual gaps.
