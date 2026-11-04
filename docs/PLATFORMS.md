# Native platforms and root CI

Standalone codec acceptance and whole-pipeline support have different scopes.
The production root package remains restricted to macOS ARM64 until the new
native root evidence passes the admission gates in ADR 0050.

| Target | Existing evidence | Current work | Remaining admission boundary |
| --- | --- | --- | --- |
| macOS ARM64 | Complete root Release and supported-feature suites, focused Debug, ASan, ABI, ownership, and strict bytes; standalone default codecs accepted. | Shared isolated-default harness passes complete Release and matching original Debug: 64 registrations and 9,473 ordered cases each. See [actual preparation evidence](root-profile-preparation.json). | Automate the new profiles on macOS CI and extend matching Debug preprocessing to feature configurations; keep the existing accepted profile intact. |
| GNU Linux x86-64 | Complete standalone AOM, dav1d, and SharpYUV acceptance; default isolated root Release passes 64 registrations / 9,473 ordered cases, ABI, all consumer/runtime audits, and 20 strict byte pairs. See [actual native evidence](root-linux-native-validation.json). | Correct and verify the duplicated glibc assertion evaluation exposed by three original Debug executables; see [ADR 0051](adr/0051-preserve-single-evaluation-of-glibc-assertions.md). | Corrected focused and complete Debug proof, supported feature configurations, applicable sanitizers, and production profile selection with final consumer checks. |
| Windows x86-64 | No native root or workspace codec acceptance. | Follow Linux admission; start with the MSVC target. | Native headers/CRT layouts, variadic/thread/alignment mappings, pinned reference builds, PE/COFF ownership, all default codec profiles, and original C/Rust execution. |
| Other architectures | No whole-pipeline acceptance. | Follow the first Linux and Windows targets. | Independent native evidence; cross-compilation alone is insufficient. |

## Two CI tiers

`root.yml` uses the same native build and validation tools for both tiers:

| Trigger | Native root configuration | Executed scope |
| --- | --- | --- |
| Pull request or push to `main` | Linux x86-64 Release, default container configuration. | Eight unchanged original smoke executables, exact C/Rust case inventories, named layouts, all original AOM globals and consumer/runtime ownership, shared C ABI, and 20 strict AVIF/PNG comparisons. |
| Nightly at 02:17 UTC or manual dispatch | Linux x86-64 Debug and Release, default and combined Mini/Extended PIXI configurations. | Complete original root C/Rust suites for each configuration, exact ordered case/skip comparisons, and the same ABI, ownership, shared-library, and byte checks. |

The Linux pipeline generates and builds isolated root profiles during admission;
it does not remove the production target guard. Optional native engines and AV2
are outside this initial matrix. macOS nightly automation, Windows, and native
root sanitizer expansion remain explicit follow-up gates. Existing standalone
dependency workflows retain their own coverage.

Only PR runs cancel an earlier run of the same PR. Pushes, manual runs, and
nightlies have separate event concurrency groups. The two nightly container
configurations may run concurrently; each job limits compiler/test processes to
two. XML, complete CTest logs, paired summaries, source profiles, configuration
records, and binary hashes are retained as artifacts, including on failure.
Workflow status alone is not evidence of complete target acceptance.
Inventory queries use copies of the generated CTest registrations because even
CTest's metadata-only mode rewrites `LastTest.log`. Each validation attempt
preserves its actual XML, complete logs, configurations, paired report, and byte
products separately. Later tests do not replace that retained execution proof.

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
and Rust consumers. An isolated profile is preparation for admission, not a
fallback implementation or a supported production target.
