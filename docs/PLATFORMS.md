# Native platforms and root CI

Standalone codec acceptance and whole-pipeline support have different scopes.
The production root package remains restricted to macOS ARM64 until the new
native root evidence passes the admission gates in ADR 0050.

| Target | Existing evidence | Current work | Remaining admission boundary |
| --- | --- | --- | --- |
| macOS ARM64 | Accepted production root and standalone default codecs. The shared native harness also passes complete Release/Debug locally: 64 registrations / 9,473 cases by default and 65 / 9,527 with combined Mini/Extended PIXI, plus ABI, all 55/56 consumers, and strict bytes. See [actual preparation evidence](root-profile-preparation.json). | The hosted native pilot passes both default profiles after a runner-capacity retry; its bounded execution is recorded separately from local full proofs. | Other container configurations and expanded sanitizer automation; keep the existing accepted production profile intact. |
| GNU Linux x86-64 | Complete standalone default codec acceptance; default root Release/Debug each pass 64 registrations / 9,473 ordered cases. Mini Release/Debug each pass 65 / 9,526, Extended PIXI 64 / 9,474, and combined Mini/Extended PIXI 65 / 9,527. Each has 31 matching skips, named ABI, all 55/56 consumer audits, and 20 strict byte pairs. Release AddressSanitizer parity passes all four container configurations with the same counts, 31 matching skips, and all 55/56 instrumented consumers. With all four optional external engines (libgav1, rav1e, SVT-AV1, and external libyuv), Release/Debug each pass 65 / 9,481 in the default container and 66 / 9,535 with combined Mini/Extended PIXI. Both have no dynamic skips, named ABI, all 56/57 consumer audits, and 20 strict byte pairs. See [actual native evidence](root-linux-native-validation.json) and [sanitizer evidence](root-linux-sanitizer-validation.json). | The glibc assertion correction and all four native container configurations have complete execution proofs; see [ADR 0051](adr/0051-preserve-single-evaluation-of-glibc-assertions.md). Root ASan compares separately instrumented consumers; see [ADR 0052](adr/0052-compare-separately-instrumented-native-root-consumers.md). Engine profiles link pinned engine builds; see [ADR 0053](adr/0053-generate-native-root-engine-profiles-from-pinned-external-engines.md). | AVM/AV2 configurations, sanitizer checks for the engine configurations, and production profile selection with final consumer checks. |
| Windows x86-64 | No native root or workspace codec acceptance. | Follow Linux admission; start with the MSVC target. | Native headers/CRT layouts, variadic/thread/alignment mappings, pinned reference builds, PE/COFF ownership, all default codec profiles, and original C/Rust execution. |
| Other architectures | No whole-pipeline acceptance. | Follow the first Linux and Windows targets. | Independent native evidence; cross-compilation alone is insufficient. |

## Two CI tiers

`root.yml` uses the same native build and validation tools for both tiers:

| Trigger | Native root configuration | Executed scope |
| --- | --- | --- |
| Pull request or push to `main` | Linux x86-64 Release, default container configuration. | Eight unchanged original smoke executables, exact C/Rust case inventories, named layouts, all original AOM globals and consumer/runtime ownership, shared C ABI, and 20 strict AVIF/PNG comparisons. |
| Nightly at 02:17 UTC or manual dispatch | Linux x86-64 and macOS ARM64, Debug and Release, default and combined Mini/Extended PIXI configurations; Linux also runs Mini and Extended PIXI separately, and both full containers with all four optional external engines. | Complete original root C/Rust suites for each configuration, exact ordered case/skip comparisons, and the same ABI, ownership, shared-library, and byte checks. |
| Nightly or manual dispatch, sanitizer job | Linux x86-64 Release, all four container configurations, AddressSanitizer. | Complete original root C/Rust suites with separately instrumented consumers, exact ordered case/skip comparisons, instrumentation and runtime checks, and the ownership audit. |

The Linux pipeline generates and builds isolated root profiles during admission;
it does not remove the production target guard. On Linux, the optional
external engines are built from their pinned sources for the default and
combined Mini/Extended PIXI containers. AV2 is outside this matrix. Native CI execution of the new macOS wiring,
Windows, and sanitizer checks beyond the four Linux container Release profiles
remain explicit gates.

The sanitizer job keeps the ordinary original C build as the source of the
generated profile. Its separate ASan C baseline must match the 28 libavif
compilation commands exactly, apart from the two appended instrumentation
flags, and links the complete instrumented 168-unit original AOM closure.
The Rust profile uses separate work and Cargo target directories. Both
consumer sets link the selected nightly's static ASan runtime once per
executable. Exact bytes, ABI probes, and the shared-library smoke are not
repeated under instrumentation; they remain part of the ordinary profile run.

The standalone dav1d and SharpYUV workflows also use two tiers on Linux x86-64
and macOS ARM64. PRs and pushes build both profiles and retain their bounded
API, ABI, and archive/consumer ownership checks. SharpYUV also retains its
allocation and CPU-selection checks. Nightly runs at 02:47 UTC and manual
full runs retain every existing original corpus command, including both dav1d
profiles, original DSP bodies, Linux allocation/thread failures, and the
original Release WebP integration suite. Their artifacts record the executed
scope explicitly. A manual `runner` selection can validate one native target;
the default runs both. The pinned `RUSTUP_TOOLCHAIN` selects the requested
compiler without an implicit formatter installation from `rust-toolchain.toml`.
The [actual tier validation](native-ci-tier-validation.json) records the native
macOS root pilot, both native SharpYUV bounded gates, and the complete corrected
macOS dav1d corpus and DSP checks. Earlier capacity failures and the original
formatter failure remain preserved; bounded runs receive no full-suite credit.

Only PR runs cancel an earlier run of the same PR. Pushes, manual runs, and
nightlies have separate event concurrency groups. Up to two nightly container
configurations per native OS run concurrently; each job limits compiler/test processes to
two. XML, complete CTest logs, paired summaries, source profiles, configuration
records, and binary hashes are retained as artifacts, including on failure.
Workflow status alone is not evidence of complete target acceptance.
The reusable workflow retains its `root-translate-linux.yml` filename for
existing dispatches; its `runner` input selects the native host and rejects
unsupported hosts through the shared pipeline's target checks. macOS installs
LLVM 16.0.6 for C2Rust and uses the native SDK and original C compiler for its
reference builds. Neither host uses the other host's headers or CMake cache.
Inventory queries use copies of the generated CTest registrations because even
CTest's metadata-only mode rewrites `LastTest.log`. Each validation attempt
preserves its actual XML, complete logs, configurations, paired report, and byte
products separately. Later tests do not replace that retained execution proof.
Native target and container configuration also have separate Cargo output
directories, so building another feature profile cannot replace the earlier
proof's archive or shared library.
Compare each ordered GoogleTest case's terminal result and dynamic skip from
the complete logs; truncated passing JUnit output alone cannot establish that
inventory. Skip comparisons use case names only; GoogleTest's elapsed time
is nondeterministic and excluded. The [retained-log recheck](root-complete-log-validation.json) records
the existing complete native runs and their actual matching skip lists.

The original AOM video suite retains ADR 0048's complete Release and focused
Debug policy. Root CI builds all 168 original portable AOM library units as its
independent C reference with `--library-only`; it neither builds nor reruns the
standalone video test matrix and claims no new standalone execution credit.
Native VMAF remains enabled. SharpYUV keeps scalar DSP selection, verified by
preprocessing its actual compilation flags. x86-64 scalar floating-point code
retains the required SSE2 calling convention without enabling hand-written DSP.

## Local commands for an isolated profile

Prepare the recorded Meson 1.7.2, LLVM 16/C2Rust 0.21.0, pinned Rust nightly,
and native development dependencies first. Then, for a native Linux Release
default profile:

```sh
python3 tools/build_root_references.py --jobs 2
python3 tools/build_aom.py --jobs 2 --library-only
python3 tools/build.py baseline --fetch-deps --jobs 2 \
  --aom-reference-build build/aom-native-release \
  --root-references build/root-references-x86_64-unknown-linux-gnu-release \
  --build-dir build/root-c-linux-release-default
python3 tools/transpile_root.py --build build/root-c-linux-release-default \
  --profile release --clang-include /usr/lib/llvm-16/lib/clang/16/include
python3 tools/build.py port --jobs 2 \
  --root-references build/root-references-x86_64-unknown-linux-gnu-release \
  --root-profile translations/root-linux-release-default.json \
  --build-dir build/root-rust-linux-release-default
python3 tools/validate_root_profile.py --baseline build/root-c-linux-release-default \
  --port build/root-rust-linux-release-default \
  --root-profile translations/root-linux-release-default.json --tier full --jobs 2
```

Use the Debug counterparts plus `--debug --upstream-debug` for both original C
and Rust consumers. For root AddressSanitizer parity, run the same phases
with `python3 tools/root_pipeline.py --phase all --profile release --tier full
--sanitizer address --clang-include /usr/lib/llvm-16/lib/clang/16/include`. For an
optional engine configuration, pass its name, such as `--variant
libgav1-rav1e-svt-libyuv`; the references phase first builds the pinned
engines with `tools/prepare_optional_deps.py`. Engine configurations do not
yet accept `--sanitizer`. An isolated profile is preparation for admission, not a
fallback implementation or a supported production target.
