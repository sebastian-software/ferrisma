# Portable libaom port

Status: standalone implementation and validation in progress. All 168 portable
C translation units are imported for macOS ARM64 and GNU Linux x86-64 Debug
and Release, with a raw Rust API. macOS pilot checks pass; Linux compilation
and validation are in progress. libavif still links the native libaom engine.
Complete original-test correspondence, platform, and acceptance gates must pass
before integration.

The immutable reference is libaom 3.15.2, tag `v3.15.2`, commit
`af3dc9aadc793c1b00edc003e3fc19a44fdeb574`. The complete 1,520-file snapshot,
including original tests and bundled third-party sources, is under
`upstream/aom`. `provenance/aom-source.json` records every SHA-256 digest.
The source BSD-2-Clause license and Alliance for Open Media patent license are
preserved, as are the BSD-3-Clause fastfeat and MIT vector notices.

## Reference boundary

Use upstream's `AOM_TARGET_CPU=generic` build, with both encoder and decoder,
8/10/12-bit support, threads, denoising, quantization matrices, the complete
non-realtime-only encoder, and the original default experiments. Compile with
`-ffp-contract=off`. Debug and Release retain their actual original assertion
and preprocessing differences; Rust development builds remain unoptimized.
SIMD/assembly paths and later optimization are separate work.

Keep `CONFIG_TUNE_VMAF=1`, matching the existing Homebrew engine's optional
VMAF encoding modes. libvmaf is a distinct runtime dependency and remains
native during this milestone. A libaom translation using it must not be
described as a pipeline with no native codec/quality implementations. Its
separate mechanical port and versioned model inputs require their own
contract. Do not silently disable VMAF modes to make that dependency disappear.
The installed package reports libvmaf 3.2.0 through pkg-config despite its
Homebrew package directory being 3.2.1; record both when comparing artifacts.
For isolated builds, `tools/build_aom_vmaf.py --fetch` builds native VMAF tag
`v3.2.1`, commit `f85a853692a8c730d0270cd733c8bb30b5b93b7c`, with scalar
extractors and built-in models. That tag's own Meson project reports 3.2.0.
This dependency bootstrap is not a VMAF Rust port.

The library archive contains 168 C translation units, including three generated
configuration/no-op units and four bundled fastfeat/vector units. Bundled
libyuv, WebM I/O, GoogleTest, apps, examples, and the optional C++ rate-control
wrapper are outside this archive. Retain them as original consumers/test
infrastructure and account for their implementation scope explicitly.

## Original test boundary

The initial macOS ARM64 Debug build registers 18,729 main-suite tests, 38
rate-control wrapper tests, and 38 intra-prediction speed tests. Registrations
include upstream-disabled tests; these counts are not executed-pass counts.
The main suite has 1,501 upstream-disabled and 17,228 active registrations;
all 38 intra-prediction speed registrations are upstream-disabled.
Preserve original names, parameters, assertions, disabled status, fixtures,
and expected output. Separate scalar-code coverage from architecture-specific
tests excluded by the declared generic configuration.

Upstream selects 568 test fixtures for this configuration. Their authoritative
checksums are in the immutable `test/test-data.sha1`; the download helper also
records SHA-256 and byte sizes. Do not regenerate goldens. Preserve failed and
skipped inventories, and account separately for header-only C/C++ algorithms
compiled into the test consumer. Linking a test executable alone does not prove
that every original assertion exercises Rust.

`tools/build_aom_consumers.py` relinks unchanged original test, app, and example
objects against the Rust archive. Upstream compiles `common/args_helper.c` and
`av1/arg_defs.c` into both the library and app object targets. The bridge removes
only those source-identical app copies so their functions and data resolve to
Rust in both profiles. It rejects other production-source overlaps and records
the exact link command and map for each consumer.

## Current macOS pilot evidence

Both complete generated profiles reproduce from the pinned translation inputs.
The raw API example initializes the encoder, allocates/frees an image, and
destroys the encoder in Debug and Release. Five runtime tests exercise the
typed nonlocal-error mapping described in ADR 0036.

The focused original suites execute the same 62 cases in each C and Rust
profile, covering images, LEB128, decoder/encoder errors, film-grain tables,
metadata, key/value controls, and allocation-size overflow. None fail or skip.
These are focused checks, not full-suite acceptance.

Supplemental comparisons in each profile preserve exact output:

- Twelve public API round trips cover 8/10/12-bit 4:2:0, 4:2:2, 4:4:4, and
  monochrome, odd dimensions, three frames, two threads, packet metadata,
  decoded planes, draining, and destruction.
- Twelve CLI encode/decode cases cover intra/inter, good-quality, multithreaded,
  and lossless paths. Two additional 12-bit non-4:2:0 CLI configurations preserve
  the original control-sequence rejection and diagnostic. The direct API round
  trips cover successful encoding of those formats independently.
- Named record size, alignment, and field-offset probes compare every selected
  translation unit against Clang. Anonymous records and bitfield offsets are
  explicitly excluded from that check's scope.
- Single-allocation failure sweeps run 306 paired scenarios per profile:
  270 encoder allocation failures, 34 decoder allocation failures, and two
  successful baselines. Initialization, controls, encode/decode, diagnostics,
  destruction, allocation counts, and outstanding codec allocations match.
  All observed allocations are released. Only codec object references to
  malloc/calloc/realloc/free are hooked; native VMAF, Rust runtime allocations,
  and the unwind payload allocation remain outside this single-threaded check.
- All four VMAF-dependent tuning modes produce identical streams and decoded
  planes for 8/10/12-bit input, with the pinned `vmaf_v0.6.1.json` integer model:
  12 cases per profile. An absent model preserves the original fatal diagnostic
  and process exit. The isolated source-built VMAF profile enables float
  extractors, but the original libaom rejects the pinned float model because it
  passes `"1"` for a boolean option that requires `"true"` or `"false"`. Both
  profiles preserve the exact fatal diagnostic and exit code in all twelve
  float-model scenarios. A direct native dependency probe verifies the model
  loads without the overload, rejects `"1"` with `-EINVAL`, and accepts `"true"`.
  These are rejection cases, not successful stream coverage; ADR 0042 records
  why the mechanical port preserves this original integration defect.
- Eight paired worker-creation scenarios per profile cover successful baselines
  and each of three `pthread_create` failures in both encoder and decoder,
  with four configured threads and row multithreading. Error state,
  diagnostics, teardown, and thread-call counts match.
- The 12 public round trips, 306 allocation scenarios, and eight worker-creation
  scenarios also pass with AddressSanitizer. All 168 original C units, the Rust
  archive, and the test callers are instrumented against the pinned Rust ASan
  runtime. That supplementary macOS build uses Clang 16.0.6 and original Release
  preprocessing. Native VMAF remains uninstrumented. ThreadSanitizer and broader
  fault/fixture coverage are still open.
- The 38 original C++ rate-control tests pass in both Debug and Release
  executables, with identical executed case inventories and no dynamic skips. The
  original C++ wrapper remains native; this is consumer/integration evidence,
  not a claim to have ported that wrapper.
- All original global definitions resolve to Rust codec codegen objects:
  2,743 globals (2,707 functions) in Debug and 2,741 globals (2,705 functions)
  in Release. Detailed macOS map audits cover five Debug and ten Release
  consumers, with no original codec objects or globals supplied outside Rust.
  This ownership audit does not establish header-inline test coverage.

Durable results and remaining gates are recorded in
[`aom-validation-results.json`](aom-validation-results.json). Full-suite
execution, header-inline and scalar DSP test correspondence, additional
allocation/thread failure scenarios, broader sanitizer coverage, Linux profiles, and
root integration remain open.
Native C++ rate-control, libyuv, WebM, and GoogleTest code in original consumers
must not be counted as Rust implementation coverage.

The compiler-based [direct binding inventory](aom-test-bindings-macos-release.json)
inspects 149 original C++ compilation inputs under the selected Release
configuration. It finds direct references to 881 original codec globals and
40 production header inline functions. Timer helpers, native oracle/setup
helpers, and actual inline test subjects require distinct role review. A name
match in translated source is recorded as such; it is not execution coverage.
References inside included C++ helper bodies and indirect calls need their
own review beyond this direct-reference inventory.

Twelve reviewed inline test subjects now have optional generated Rust exports
under `port-test-bridges` (ADR 0039). Nine original C++ inputs are recompiled
with forced includes that redirect those subjects while retaining native
reference and fixture helpers. All 18,729 registrations remain identical;
14 original cases pass against C and Rust in each profile, including the
original scan-order dependency test through Rust's `get_default_scan` and the
`round_shift` test's six positive/negative rounding assertions. The default
production translation bodies are byte-for-byte unchanged, verified by
separate `production_rust_sha256` entries in the translation manifests.
Both profiles reproduced with fresh alignment/layout extraction for the first
ten wrappers. The additional scan and rounding wrappers preserve every
production body hash. Remaining inline roles, macro subjects such as `OD_DIVU_SMALL`, indirect
helper calls, and scalar self-comparisons stay open. Fixture and oracle uses
of the scan helper in other original test inputs retain their native bindings.

Six convolution subjects now use independent C oracles in 243 existing
parameterized cases per profile (ADR 0041). A separate complete original C
archive has every defined codec global and its relocations renamed, preserving
all 168 object members, instructions, and data; its symbol index is rebuilt.
Ten reviewed oracle call sites are redirected in a disposable compilation copy
of `test/av1_convolve_test.cc`. Subject parameters keep their Rust bindings.
The original assertions, inputs, disabled flags, and all 18,729 registrations
are preserved. Both Debug and Release pass against the unchanged C baseline;
link maps verify the six subjects belong to Rust and their oracles to isolated
C objects. This covers horizontal, vertical, and copy kernels at 8/10/12 bits.
Other DSP roles and scalar self-comparisons still require review.

The same isolated archive also supplies `av1_nn_softmax` for the reference call
in the two original `C/FastSoftmaxTest.RandomValues` cases. The anchor call to
`av1_nn_softmax` and the `av1_nn_fast_softmax_16_c` parameter remain Rust
subjects. Debug and Release pass with original inputs and numerical thresholds
unchanged, all registrations preserved, and both sides' ownership checked in
the link map. This is original tolerance-based coverage, not a bitwise floating
point comparison. Disabled speed cases remain disabled.

Another 243 original IntraBC convolution cases pass in each profile against
isolated C oracles for six Rust horizontal, vertical, and 2D kernels at
8/10/12 bits. Only twelve reviewed oracle call sites change in the disposable
test compilation copy; original parameters, unequal source strides, assertions,
and all 18,729 registrations remain intact. Other 2D and compound convolution
roles still require review.

The original k-means classes register only SIMD implementations, leaving zero
k-means cases in the generic reference inventory. A separate supplementary
scalar matrix reuses their unchanged test bodies, random inputs, fourteen
original block sizes, and centroid counts 2 through 8. Its 28 enabled cases
compare two Rust scalar functions with isolated C oracles; another 28 original
speed bodies remain disabled. Both profiles pass. The original 18,729
registrations remain present and in order; the added `PortScalar/` cases are
counted separately and do not inflate original-suite execution totals.

Neural-network prediction has the same original SIMD-only registration gap.
A supplementary scalar registration runs the unchanged `RandomValues` body
against an isolated C oracle: all 17 original network shapes and 10,000
iterations per shape, with the original `1e-3` threshold. Debug and Release
pass; the speed body remains disabled. This adds one executed supplementary
case per profile and no original-suite execution credit.

## Execution and acceptance

Work proceeds in one checkout with a Solo profile, coherent commits, generated
corrections, narrow comparisons after each correction, and full-suite
checkpoints. No existing consumer switches until standalone acceptance passes.
Limit local execution to two CPU-intensive processes at a time across all
commands. The shard runner defaults to one worker so a C and a Rust run can
share that budget; additional verification/build work waits for a slot.
Preserve original encoder/decoder thread parameters. A multithreaded codec test
can use several CPU cores inside one process. Scheduling limits are separate
from test cases, assertions, numerical references, and preprocessing profiles.

1. Verify complete source/license hashes, the archive's exact source closure,
   generated headers, configurations, and original test inventories.
2. Prove the representative image/API and encoder/decoder state/error boundary.
   In particular, direct C2Rust `setjmp`/`longjmp` calls are not an accepted Rust
   implementation: retain error targets, diagnostics, return values, cleanup,
   and worker synchronization with an explicit supported control-flow mapping.
3. Import all selected units reproducibly. Preserve field, record, local, and
   static alignment; callback/variadic ABIs; ownership; atomics; and raw C
   address semantics. Record every correction and source/output digest.
4. Run original native and Rust consumers with matching case inventories,
   supplemental exact encode/decode comparisons, defined API errors, allocation
   failures, and instrumented cleanup checks. Audit symbols and link maps for
   original codec fallbacks and accidental symbol interposition.
5. Require Debug and Release evidence on native macOS ARM64 and GNU Linux
   x86-64 before standalone acceptance. Root Linux support remains separate.
6. Replace native libaom in the root archive, audit the full default linkage,
   and rerun original libavif suites and exact AVIF/PNG comparisons. Keep the
   VMAF dependency and any remaining unsupported configurations visible.

```sh
python3 tools/import_aom.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
python3 tools/build_aom.py --debug
python3 tools/build_aom.py
python3 tools/fetch_aom_test_data.py
python3 tools/transpile_aom.py
python3 tools/transpile_aom.py --release
python3 tools/build_aom_consumers.py --debug
python3 tools/build_aom_consumers.py
python3 tools/check_aom_linkage.py --debug
python3 tools/check_aom_linkage.py
python3 tools/check_aom_inlines.py --debug
python3 tools/check_aom_inlines.py
python3 tools/check_aom_oracles.py --debug
python3 tools/check_aom_oracles.py
python3 tools/check_aom_oracles.py --debug --suite softmax
python3 tools/check_aom_oracles.py --suite softmax
python3 tools/check_aom_oracles.py --debug --suite intrabc
python3 tools/check_aom_oracles.py --suite intrabc
python3 tools/check_aom_oracles.py --debug --suite kmeans
python3 tools/check_aom_oracles.py --suite kmeans
python3 tools/check_aom_oracles.py --debug --suite nn-predict
python3 tools/check_aom_oracles.py --suite nn-predict
python3 tools/check_aom_abi.py
python3 tools/check_aom_abi.py --release
python3 tools/check_aom_cli.py --debug
python3 tools/check_aom_cli.py
python3 tools/check_aom_roundtrip.py --debug
python3 tools/check_aom_roundtrip.py
python3 tools/check_aom_faults.py --debug
python3 tools/check_aom_faults.py
python3 tools/check_aom_vmaf.py --debug
python3 tools/check_aom_vmaf.py
python3 tools/check_aom_faults.py --debug --thread-failures
python3 tools/check_aom_faults.py --thread-failures
python3 tools/check_aom_roundtrip.py --asan
python3 tools/check_aom_faults.py --asan
python3 tools/check_aom_faults.py --asan --thread-failures
python3 tools/run_aom_tests.py
python3 tools/run_aom_tests.py --rust
```

Native build evidence is written to `build/aom-native-{debug,release}`. The
test corpus is cached in `build/deps/aom-test-data`; set
`LIBAOM_TEST_DATA_PATH` to that absolute directory when running original tests.
Downloaded fixtures and disposable builds are not source modifications.

## Native Linux checkpoint

Both complete x86-64 profiles were generated on Ubuntu 24.04 with Clang/LLVM
16.0.6, C2Rust 0.21.0, and the isolated native VMAF reference. Debug and Release
compile, and the original `aomenc`, `aomdec`, and `test_libaom` consumers link
against each Rust archive. Each profile passes 12 exact public API round trips
and 14 CLI scenarios: 12 exact streams/pixel outputs and two matching original
control-sequence rejections. Five nonlocal-error runtime tests and both raw
Rust API examples also pass. These scoped results are preserved in
[the Linux checkpoint](aom-linux-validation-results.json). Full standalone
acceptance and root Linux support remain open.

A fresh translation check also reproduces all 336 committed Rust units across
the two profiles. GNU link-map audits pass for all original codec globals in
the selected consumers. Each profile passes 13 unchanged original cases through
11 Rust inline subjects, 243 original convolution cases with isolated C
oracles for six Rust subjects, and 12 exact integer-model VMAF scenarios. That
run still fails on the original C float-model rejection described in ADR 0042;
later allocation, thread, focused API, and ABI gates were not reached.

The subsequent native Linux failure gates exposed an incomplete translation:
glibc expands `setjmp` to `_setjmp`, which the original mapping rule missed.
The generator now normalizes that spelling before mapping all 32 scoped error
targets in each Linux profile. Structural validation rejects surviving libc
nonlocal-jump calls and incorrect target/transfer counts. Alias regression
checks and artifact reproduction pass; native Linux compilation and failure
behavior must pass again before accepting the corrected candidate.

GNU pkg-config's `-pthread` spelling maps explicitly to the pthread library.
For glibc's array-decayed `va_list` control callback typedef, ADR 0040 maps the parameter
to Rust's C-compatible `VaList`, consistently with its callback definitions
and dispatcher. The correction affects five units per profile, is inventoried
and generated, and leaves all C inputs unchanged.

The retained native VMAF archive includes libsvm's `svm.cpp`. Its generated
pkg-config file omits the C++ standard library; original C++ consumers get
that dependency from their linker driver. The Rust package and C comparison
callers explicitly retain libc++ on macOS and libstdc++ on GNU Linux. This is
part of the native VMAF boundary, not an additional ported implementation.

`tools/import_aom_linux.py` verifies every artifact Rust hash, the exact
168-unit closure, required configuration, and generated header hashes before
importing a profile. It applies the same callback correction, glibc error-target
mapping, and optional inline wrappers as the current translator. Its `--check`
mode verifies the complete transformation against
the original artifact. The original artifact and imported manifest digests,
changed units, reference configurations, and immutable CI run are recorded in
[the import record](aom-linux-source-import.json).

`--refresh` reapplies reviewed rules only after matching every existing source
hash, raw artifact digest, and reference configuration. It updates recorded
Rust units and manifests; different reference headers or API/module files
remain errors.

The Linux workflow's `compile` mode builds and checks committed candidates
without rerunning translation. `generate` produces fresh profiles; `verify`
reproduces committed profiles from the native source and headers. Fresh
translation reproduction and the remaining Linux execution gates are still
required before acceptance.

GNU consumer links now emit cross-reference tables with their map files.
The ownership audit checks the defining archive/member for every loaded
original codec global and rejects native codec objects or archives. The same
parser supports the separate C oracle audit. Existing macOS audits still pass
for all 15 consumer maps and both oracle executables; native GNU execution of
this extended audit remains a Linux workflow gate.
