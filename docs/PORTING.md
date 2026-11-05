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

## Native platform expansion

The shared native root harness now passes complete Release and matching Debug
on GNU Linux x86-64 and macOS ARM64 for the default and combined Mini/Extended
PIXI configurations. Each default pair covers 9,473 original cases; each
combined pair covers 9,527, with exact ordered outcomes, ABI, strict bytes, and
all consumer/runtime ownership audits. Linux also passes the separate Mini
(9,526 cases) and Extended PIXI (9,474 cases) Release/Debug profiles with the
same checks, plus both configurations with all four optional external engines
(9,481 and 9,535 cases); see
[ADR 0053](adr/0053-generate-native-root-engine-profiles-from-pinned-external-engines.md).
Linux uses verified hosted artifacts;
the complete macOS proofs are local native executions, supplemented by the
hosted bounded pilot. See [platform evidence](PLATFORMS.md).

Linux Release AddressSanitizer parity also passes all 64 default registrations
and 9,473 ordered cases, 65 Mini registrations and 9,526 cases, 64 Extended
PIXI registrations and 9,474 cases, and all 65 combined Mini/Extended PIXI
registrations and 9,527 ordered cases, with separately instrumented consumers; see
[ADR 0052](adr/0052-compare-separately-instrumented-native-root-consumers.md).
Linux production admission still requires the AVM/AV2 configurations,
sanitizer checks for the engine configurations, and production profile
selection with final consumers.
Windows x86-64 follows that milestone. Removing the current target guard alone
does not establish support.

`root-translate-linux.yml` prepares isolated native root source profiles using
the actual C compilation database and pinned original decoder/conversion
archives. Its generated-source and typecheck artifacts are preparation evidence;
they do not establish native execution acceptance. The profile guards reject
missing source units, changed source/output hashes, cross-target evidence, and
changed module selectors. Existing macOS translation bodies remain unchanged.
Its `smoke` and `full` tiers additionally build isolated Rust libraries and run
the original consumers, named ABI probes, complete AOM ownership/runtime audits,
and strict AVIF/PNG comparisons. The smoke tier selects eight original test
executables; the full tier requires every registration from the actual native
CMake inventory and exact ordered original GoogleTest cases. Neither tier
changes the production root target guard or claims optional-engine acceptance.
See [platform CI](PLATFORMS.md) for the PR/nightly split and remaining gates.
The acceptance and CI boundaries are recorded in
[ADR 0050](adr/0050-admit-root-targets-through-native-profile-parity.md).
