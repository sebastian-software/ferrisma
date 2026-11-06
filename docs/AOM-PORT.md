# Portable libaom port

Status: native Linux and macOS Release and focused Debug acceptance passes
under ADR 0048. Rust AOM is the mandatory root dependency under ADR 0049;
final root validation is recorded separately below.
All 168 portable C
translation units are imported for macOS ARM64 and GNU Linux x86-64 Debug and
Release, with a raw Rust API and passing focused native comparisons. Before
the input-alignment correction in ADR 0046, the explicit root candidate passed
libavif Debug, Release, supported-feature, and AddressSanitizer suites. Those
root checks are historical. Default Rust builds use the portable workspace AOM
implementation with native VMAF retained. Complete original Release suites,
focused Debug checks, and final runtime gates follow ADR 0048. Exhaustive unoptimized
Debug execution is no longer a default-switch prerequisite.

The first hosted macOS full Debug shard exposed an additional Rust alignment
requirement when the original small-width test's byte-array fixture had an odd
address. The unchanged C consumer passed; Rust aborted in `validate_img`.
ADR 0046 retains that external sample load through `read_unaligned` in all
four profiles. Preserve the original tests and supplement them with forced
address-parity and per-plane range checks. The incomplete macOS run
41508117713 and Linux run 41507995195 are superseded after this production
change and provide no complete-suite acceptance credit. Earlier validation
records retain their exact source/archive hashes as historical evidence.
The failure, negative reproduction, and corrected local original/supplemental
comparisons are recorded in
[`aom-unaligned-input-validation.json`](aom-unaligned-input-validation.json).

The replacement Linux Debug run reached the hosted six-hour job limit in
several original shards. ADR 0047 preserved immutable binaries and successful
partitions while recovering confirmed time-limit cancellations with finer
original GoogleTest shards. ADR 0048 subsequently replaced the full Debug
gate after measuring its runtime. The mixed full-suite and Debug-recovery
runs were stopped with their published artifacts retained. Complete Release
acceptance still requires every original active case and matching dynamic
skip exactly once; a passing subset is not enough.
See [`aom-shard-recovery-validation.json`](aom-shard-recovery-validation.json)
for the time-limit evidence and recovery checks.

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
SIMD/assembly paths and later optimization are separate work; the macOS Neon
layer is described under [Neon dispatch](#neon-dispatch-adr-0056).

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

[`aom-validation-results.json`](aom-validation-results.json) retains the historical
pilot results. Complete native standalone results are recorded in
[`aom-macos-release-acceptance.json`](aom-macos-release-acceptance.json) and
[`aom-linux-release-acceptance.json`](aom-linux-release-acceptance.json). The
finite source-role review below is closed for its declared inventory.
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

The finite direct header review now classifies all 170 bindings of those
40 header functions. Actual inline subjects use the optional Rust exports;
the other bindings construct inputs, compute original expected values, or
measure loops. Their native execution receives no Rust subject coverage.
`tools/review_aom_header_roles.py --check` rejects an unknown or changed
binding inventory. This bounded review does not establish correspondence for
all included template bodies or indirect calls. See
[`aom-header-role-review.json`](aom-header-role-review.json).

The extended compiler inventory also inspects included original C++ test
helpers and dependent lookup candidates. All 149 compilation inputs and 223
original source/header hashes are checked. It identifies 41 production-header
functions; the additional `av1_use_intra_edge_upsample` binding selects input
upsampling in `DrPredTest::SetParams` and keeps its native fixture role.
The extended review remains a static superset, including potentially unused
bodies, and assigns no runtime coverage. See
[`aom-included-header-role-review.json`](aom-included-header-role-review.json).

The finite scalar review covers all 36 original C++ inputs with a `_c` global
in the direct inventory, including physical template bodies missed by that
initial inventory. It classifies independent C++ arithmetic, separate C
oracles, original redundant comparisons, and the 16 SIMD-only inputs with no
generic registrations. Case-name hashes and disabled counts are checked
against all 18,729 original registrations. The static review assigns no pass
or runtime-coverage credit. See
[`aom-scalar-role-review.json`](aom-scalar-role-review.json).

The finite correspondence closure rechecks all 223 listed original file hashes,
the direct and scalar source hashes, and every classified role. It also verifies
the actual summaries and both C/Rust XML files for 17 current native Linux
oracle matrices per profile: 1,089 original independent-oracle cases and 226
separate supplementary cases. This closes the declared classification review;
it does not establish complete native acceptance or universal indirect-call or
runtime-path coverage. Scope, evidence hashes, native-helper boundaries, and
remaining execution gates are recorded in
[`aom-subject-oracle-correspondence.json`](aom-subject-oracle-correspondence.json).

Upstream builds `av1/ratectrl_rtc.cc` into the separate `aom_av1_rc` library,
which it does not install. That optional wrapper is outside the selected
168-C-unit libaom archive and is not a root libavif runtime dependency. The
original `test_aom_rc` executable retains it as a native consumer of the Rust
codec; its passing cases do not establish a Rust wrapper implementation.

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
production body hash. Remaining inline roles, other macro subjects, indirect
helper calls, and scalar self-comparisons stay open. Fixture and oracle uses
of the scan helper in other original test inputs retain their native bindings.

The optional `OD_DIVU_SMALL` subject is generated from a tiny C wrapper around
the original `odintrin.h` macro (ADR 0043). Both macOS preprocessing profiles
reproduce with C2Rust, and the matching Linux subjects are generated natively
with the same pinned compiler. The two unchanged original division cases pass
against the Rust macro and Rust constant table in macOS Debug and Release,
preserving every original loop, assertion, and all 18,729 registrations. Their
expected values still use native C++ unsigned division. Both cases also pass
natively on GNU Linux x86-64 in Debug and Release. These optional helper units
are outside the 168-unit production closure; generation alone receives no
execution credit. Results and native
Linux generation provenance are in
[`aom-macro-validation.json`](aom-macro-validation.json).

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
and all 18,729 registrations remain intact.

The remaining single-reference 2D and compound convolution roles now have
independent C oracles as well. Two 2D subjects pass 162 unchanged original
cases per profile, including `VmafPreprocessing`. Eight compound subjects pass
264 more cases per profile, preserving all original subpixel, filter, weight,
intermediate-buffer, and final-pixel assertions. Only six 2D and ten compound
reference sites change in the disposable compilation copies. Original
small-block early returns in the VMAF cases remain unchanged. Both macOS
profiles and both native GNU Linux profiles pass. Together with the
horizontal, vertical, copy, and IntraBC cases, this establishes independent
references for 22 Rust subjects across 912 original convolution cases per
profile. Other DSP roles remain open. See
[`aom-convolution-validation.json`](aom-convolution-validation.json).

Six more Rust subjects now retain independent C reference bindings in 101
unchanged original cases per profile on macOS and native GNU Linux: 13
scaled-convolution cases, eight one-dimensional mask cases, and 80 affine-warp
cases. The warp helper
body and its original parameter registrations live in separate C++ objects;
both external references and final definition ownership are checked. No
registration, assertion, input matrix, or disabled flag changes. See
[`aom-scaling-mask-warp-validation.json`](aom-scaling-mask-warp-validation.json).

The high-bit-depth d16 blend-mask fixture originally passes the same scalar
function in both parameter slots. Its first slot now resolves to the isolated
C reference while its second slot remains Rust. Both unchanged original random
and extreme cases pass in Debug and Release on macOS and native GNU Linux;
all 18,729 registrations and the disabled speed case remain unchanged. See
[`aom-blend-d16-validation.json`](aom-blend-d16-validation.json).

Five MSE and variance-summary subjects retain isolated C references in another
40 original cases per profile. These template bodies compare MSE, SSE, sums,
and variances; they were not all visible in the initial direct compiler
inventory. Only the 15 reference call sites change in the compilation copy;
subject parameters, assertions, dimensions, sample bounds, and disabled speed
cases stay unchanged. Both profiles pass on macOS and native GNU Linux. See
[`aom-variance-mse-sum-validation.json`](aom-variance-mse-sum-validation.json).

Both SATD functions also use separate C reference and Rust subject parameter
slots in 32 original cases per profile. The eight original identical-function
pairs are checked before selecting only their first addresses. Minimum/maximum
expectations, fixed random goldens, match assertions, lengths, and disabled
speed cases stay unchanged. Both macOS profiles pass; a changed original
parameter pair is rejected before compilation. Both native GNU Linux profiles
also pass. See
[`aom-satd-validation.json`](aom-satd-validation.json).

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

The original generic directional-prediction matrix leaves its comparison
subject null. `OperationCheck` returns early, and `SaturatedValues` copies the
reference result before comparing it. Separate `PortScalar/` registrations
reuse those unchanged test bodies and the original twelve combinations of
zone, bit depth, and start angle, with isolated C references and six Rust
scalar subjects. All 24 enabled supplementary cases pass in Debug and Release;
the twelve added speed registrations remain disabled. Native angle, upsampling,
and timer helpers retain their original bindings. The original 18,729
registrations remain unchanged and in order; these additions receive no
original-suite execution credit. See
[`aom-directional-prediction-validation.json`](aom-directional-prediction-validation.json).

Two more SIMD-only test classes now have separate scalar registrations.
Array rounding reuses all 19 original `txsize_to_bsize` entries and all nine
original bit values from -4 through 4, including zero. Its 171 supplementary
cases pass with exact original assertions against an isolated C oracle.
Cubic rate-distortion interpolation reuses the original 10,000 random
iterations and exact equality assertions in one supplementary case. Both
profiles pass; the 172 added speed registrations remain disabled. Original
test bodies, input generation, and all 18,729 original registrations remain
unchanged. See
[`aom-scalar-rounding-model-validation.json`](aom-scalar-rounding-model-validation.json).

These five supplementary scalar matrices also pass natively on GNU Linux
x86-64 in both profiles: 28 k-means cases, one neural-network case, 24
directional-prediction cases, 171 array-rounding cases, and one cubic-model
case. Their 225 cases per profile retain the original assertions and numerical
thresholds and receive no original-suite execution credit. All 336 production
Rust units in that artifact match the committed manifests. This scoped run
compiles the existing source; it does not rerun translation or the complete
original suites. See
[`aom-supplementary-linux-validation.json`](aom-supplementary-linux-validation.json).

The SIMD-only optical-flow fixture also has a separate scalar registration.
Its unchanged body compares the Rust kernel with an isolated C reference in
both macOS and native GNU Linux profiles, using the original checksum-verified
bus sequence. The original video helper reuses one image buffer, so both
pointers in this test refer to the second frame; distinct-frame coverage is
not claimed. The speed case stays disabled and the original suite receives no
additional execution credit. Fixture subsets have separate provenance records
and cannot replace the full-corpus record. See [the optical-flow record](aom-optical-flow-validation.json).

## Execution and acceptance

Normal `cargo build` and `tools/build.py port --jobs 1` builds use the mandatory
workspace Rust engine without changing the translated adapter body. Production
system-AOM discovery and the temporary candidate feature are removed.
`tools/check_aom_root_linkage.py` checks every original AOM global, every original
consumer map, and non-system runtime dependencies. ADR 0049 records the default
switch after complete native standalone acceptance; native VMAF and optional
engines retain their separate port boundaries.

The original C integration baseline can be rebuilt directly with
`tools/build.py baseline --aom-reference-build build/aom-native-release --jobs 1`.
Its separate `build/baseline-aom-reference` directory compiles the unchanged
original libavif and test sources against the complete portable C AOM archive.
The helper checks all generated headers, source-unit count, profile, and
noncontracting flags and disables system AOM discovery. With `--debug`, select
the matching `build/aom-native-debug` reference. AddressSanitizer requires its
separately instrumented AOM closure rather than this ordinary reference option.
A historical Release build through this path passed all 64 registrations and
9,473 ordered case identities against the Rust candidate, with ten exact
AVIF/PNG comparisons. All 55 original C consumers link the selected portable
archive and have no dynamic system-AOM fallback. The Rust candidate's 55 maps
and runtime graphs pass again. See
[`aom-reproducible-root-reference.json`](aom-reproducible-root-reference.json).

The historical root candidate passed all 64 default registrations (9,473 GoogleTest
cases) with both Debug and Release AOM, all 70 registrations with the complete
supported feature set (9,555 cases), and all 64 AddressSanitizer registrations.
Complete CTest logs establish identical ordered GoogleTest case names in all
four configurations; truncated JUnit output alone supplies only counts. The
comparison tool also rejects a changed case name with unchanged counts.
Debug keeps AOM assertions enabled and Rust libraries unoptimized; the
existing libavif translation and native libavif reference objects retain their
original Release preprocessing. The ASan C comparison instruments all 168
original portable AOM units. Native VMAF remains outside instrumentation.
All 2,743 Debug and 2,741 Release AOM globals are supplied by Rust; audits cover
all 55 default and 59 full-feature original consumers and their non-system
runtime dependencies. Three strict end-to-end matrices each match ten AVIF
files and ten decoded PNGs, including explicit AOM decoder selection in both
profiles.
These results predate the input-alignment correction and do not establish final
default acceptance. See [the historical candidate validation](aom-root-candidate-validation.json).
Current standalone and default validation records supersede that checkpoint.

The mandatory default passes fresh complete C/Rust root Release suites
(64 CTest registrations and 9,473 ordered GoogleTest cases), all supported
features (70 registrations and 9,555 cases), focused Debug regressions
(eight registrations and 64 cases), and the complete root AddressSanitizer
suites. Every original AOM global is supplied by Rust in all 55 default
consumers and all 59 feature-enabled consumers, including their runtime
closures and the shared library. Ten default-decoder and ten explicit-AOM-decoder
comparisons match both AVIF and decoded PNG bytes against the portable C
reference with `-ffp-contract=off`. Source, translation, license, and named root
ABI checks also pass. See [the final default validation](aom-default-validation.json).
These root results cover native macOS ARM64; standalone Linux acceptance does
not establish a supported Linux root pipeline. Native VMAF and the approved
optional engines remain separate porting milestones.

Work proceeds in a dedicated linked worktree with a Solo profile, coherent commits, generated
corrections, narrow comparisons after each correction, and full-suite
checkpoints. Default consumer wiring switches only after standalone acceptance
passes.
The primary checkout stays clean on the remote default branch; tracked changes
and commits belong to the task worktree.
Limit local execution to two CPU-intensive processes at a time across all
commands. The shard runner defaults to one worker so a C and a Rust run can
share that budget; additional verification/build work waits for a slot.
Preserve original encoder/decoder thread parameters. A multithreaded codec test
can use several CPU cores inside one process. Scheduling limits are separate
from test cases, assertions, numerical references, and preprocessing profiles.

`tools/run_aom_paired_shard.py` runs one unchanged GoogleTest shard first with
the C binary and then with its Rust consumer. It checks binary and configuration
hashes, original registrations, the Rust ownership audit, and matching executed
case/skip inventories. Collection requires every shard, rejects duplicate,
missing, unexpected, or failing cases, and checks the full active inventory.
The runner passes two-shard checks of the 62 original focused macOS cases in
both profiles and rejects missing shards and edited execution records. Optional
`--resume` reuses a shard only when its complete binary/configuration/fixture
identity, passing C/Rust results, and exact original shard assignment match.
An invalid record is retained and rejected. Collection still requires every
original active selected case. This verifies the runner, not the complete main
suite. Evidence is in
[`aom-paired-shard-validation.json`](aom-paired-shard-validation.json).

The consumer builder, symbol audit, and paired runner accept an explicit
`--reference-build` so a task worktree can reuse native reference sources,
objects, and executables without rebuilding them. The builder checks every
generated reference header against the selected translation manifest; the
runner rejects a different preprocessing profile/configuration. Rust libraries,
candidate consumers, and reports stay in the task worktree. The runner's
`--fixtures` option selects an existing verified corpus without copying it.
The current local corpus has been rehashed against all 568 original inputs.

The manual `aom-original-linux.yml` workflow prepares both profiles once and
shares the audited binaries with 32 paired main-suite shards per profile.
It also preserves the 38 rate-control registrations and all 38 upstream-disabled
intra-prediction speed registrations in each profile. At most two paired test
jobs run concurrently. Every main-suite job fetches and verifies all 568
selected original fixtures. Separate collection requires identical binary,
configuration, translation, and fixture identities across the complete run;
upstream-disabled speed cases remain disabled and receive no execution credit.
This workflow is prepared for full native Linux acceptance; it has not yet
established a passing full-suite result. The remaining subject/oracle role
review is an additional gate even after full-suite execution passes.

`aom-original-native.yml` builds both profiles on standard
`ubuntu-24.04` or native ARM64 `macos-26` / `macos-15` runners. Both macOS images
require the same native SDK layout checks. It uses 120 paired main-suite shards
for Release and the two original Release rate-control/speed jobs, with eight
Linux or four macOS jobs at a time. Debug runs 69 unchanged original API,
memory-boundary, and high-bit-depth alignment cases. Native macOS builds also
check named record layouts against that runner's SDK. Allocation and worker
failure comparisons remain separate focused checks. Local execution remains
limited to two CPU-intensive processes. ADR 0048 records the revised gate;
complete native Release acceptance on either platform is still pending.

Local macOS ARM64 execution now complements the hosted runs. It uses the exact
frozen source-run C/Rust executables and the original 120 main-suite partitions,
starting at the opposite end of the hosted queue to avoid repeating completed
work. The local corpus was rehashed against all 568 original fixture checksums
before launch. At most two paired runners execute locally; each runs C and Rust
sequentially. Complete native evidence may combine source-run, continuation,
and local results with explicit origins and unchanged binary, configuration,
manifest, fixture, case, skip, and XML identities. Linux continues independently
in CI. A macOS result does not count as Linux acceptance.

`aom-original-release.yml` continues from an existing native source run's exact
frozen binaries. It checks actual XML and the established execution identities
before reusing a complete source Release pair, then schedules only missing
partitions. Collection verifies that reused reports and XML remain unchanged,
requires all three Release inventories, and reparses the focused Debug proof.
It explicitly records the source run's cancellation and does not claim full
Debug acceptance. Original tests, inputs, and disabled flags are unchanged.
The transition, original-artifact integration checks, and focused fault
comparisons are recorded in
[`aom-release-policy-validation.json`](aom-release-policy-validation.json).

The original Linux continuation exposed a fixture-provisioning error in its
rate-control job: both C and Rust failed the same 16 file-open cases because
the job had not fetched the two required original videos. The actual failed
XML, logs, and identities are retained in
[`aom-rate-control-fixture-failure.json`](aom-rate-control-fixture-failure.json).
`aom-rate-control-fixtures.yml` validates that exact failure class and repeats
all 38 unchanged cases using the frozen binaries and verified original corpus.
The Release mode of `aom-original-collect.yml` composes passing original and
continuation main/speed evidence with a successful guarded repair. It retains
the original failed workflow conclusion and evidence, rejects any other job
failure, and still requires the complete Release and focused Debug collectors.
The first native composition correctly rejected eight missing, already passing
main partitions: the bulk artifact download had enumerated only the first 100
of 110 published artifacts. Bulk downloads now explicitly traverse every API
page, check ZIP digests, preserve existing evidence, and retain per-file hashes.
Incomplete source-run files remain in a separate staging directory; only the
source plan's hash-pinned passing pairs enter the composed inventory. This
preserves the cancelled partial C execution without replacing its XML or logs
with those from the complete continuation pair.
The rejected inventory and a real download of an omitted original partition are
recorded in
[`aom-artifact-pagination-validation.json`](aom-artifact-pagination-validation.json).

The corrected native Linux collection, run 41690098163, passes the complete
Release gate: all 17,228 active main registrations with 23 matching dynamic
skips, all 38 rate-control cases, and the unchanged 38 disabled speed
registrations with no enabled execution. Its focused Debug proof covers all
69 selected original cases. The downloaded artifact was checked against its
GitHub ZIP digest; every paired report was compared with actual C/Rust XML,
ordered original case identities, frozen binaries, source/configuration and
fixture identities. The 14 reused source pairs, 106 continuation main pairs,
classified original rate-control failure, and successful fixture repair remain
preserved. Exact evidence hashes and the independent ABI/oracle checkpoint are
recorded in
[`aom-linux-release-acceptance.json`](aom-linux-release-acceptance.json).
This completes the native Linux gate.

The native macOS composition also passes all 120 original main partitions,
the 38 rate-control cases, the unchanged disabled-speed boundary, and the
69 focused Debug cases. It combines five unchanged source-run pairs with
verified continuation and local native executions of the same frozen binaries.
All 17,228 active main registrations and 23 matching dynamic skips are retained.
Independently passing duplicate executions remain separate; their differing
XML timings and logs are never mixed or overwritten. Exact origins and actual
report/XML/log hashes are recorded in
[`aom-macos-release-acceptance.json`](aom-macos-release-acceptance.json).
The original named-layout ABI evidence is retained only after checking that
the complete source difference is the four scalar unaligned-load corrections;
no type definition, signature, original source, or header changed.
Both native standalone gates and the finite correspondence review are complete.
Final default root integration and validation remain required.

The hosted `macos-26` build now passes both original profiles and verifies
18,207 Debug / 18,157 Release named record instances with 204,984 / 204,684
layout checks and no differences. All original globals resolve to Rust in
all three original consumers per profile. The six published consumer hashes,
ARM64 Mach-O headers, generated configuration, and original case-name hashes
were checked against the downloaded artifact. These build and ABI checks
receive no full-suite execution credit. See
[`aom-hosted-macos-build-validation.json`](aom-hosted-macos-build-validation.json).

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
5. Require complete Release execution and focused Debug evidence on native
   macOS ARM64 and GNU Linux x86-64 under ADR 0048. Root Linux support remains
   separate. Preserve independent oracles, ABI and ownership audits.
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
python3 tools/check_aom_macros.py --debug
python3 tools/check_aom_macros.py
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
python3 tools/check_aom_focused_debug.py
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
the selected consumers. Each profile passes 14 unchanged original cases through
12 Rust inline subjects, 243 convolution cases and 243 IntraBC cases against
isolated C oracles, and both original softmax cases. The 28 supplementary
k-means cases and one supplementary neural-network case also pass; they receive
no original-suite execution credit. Both pinned VMAF model contracts pass:
12 exact integer-model streams and 12 exact original float-model rejections.

Both profiles also pass all 306 paired allocation scenarios, eight worker-creation
scenarios, and 62 original focused API/error cases, with matching results and
no dynamic skips in those focused cases. Named layout checks cover 18,598
records / 208,029 checks in Debug and 18,548 records / 207,729 checks in Release,
with zero differences. Anonymous records and bitfield offsets remain excluded.

Earlier failure gates exposed glibc's `_setjmp` spelling, which the initial
mapping rule missed. The generator now normalizes it before mapping all 32
scoped error targets per profile. Structural validation rejects surviving libc
nonlocal-jump calls and incorrect target/transfer counts. The corrected native
failure behavior and complete translation reproduction both pass; earlier
failed checkpoints remain identified in the validation record.

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
reproduces committed profiles from the native source and headers. `macros`
generates the optional original macro subjects separately. `dsp` checks only
the separate supplementary scalar matrices, without claiming the other
execution or source-reproduction gates. Full original-suite
execution, remaining subject/oracle correspondence, and broader validation
remain required before standalone acceptance.

GNU consumer links now emit cross-reference tables with their map files.
The ownership audit checks the defining archive/member for every loaded
original codec global and rejects native codec objects or archives. The same
parser supports the separate C oracle audit. Existing macOS audits still pass
for all 15 consumer maps and the selected oracle executables. The native GNU
consumer and isolated-oracle ownership audits also pass in both profiles.

## Neon dispatch (ADR 0056)

The macOS ARM64 profiles dispatch 242 RTCD functions to hand-translated Neon
kernels; GNU Linux keeps the generic headers. The kernels live under
`crates/aom/src/simd` at their original paths:

| Original file | Dispatched functions |
| --- | --- |
| `av1/encoder/arm/highbd_fwd_txfm_neon.c` | 19 `av1_fwd_txfm2d_*` |
| `av1/common/arm/highbd_inv_txfm_neon.c` | `av1_highbd_inv_txfm_add` |
| `av1/common/arm/cdef_block_neon.c` | 12 CDEF filter, direction, and copy functions |
| `aom_dsp/arm/highbd_intrapred_neon.c` | 190 high-bitdepth predictors, 3 directional predictors |
| `av1/common/arm/reconintra_neon.c`, `highbd_reconintra_neon.c` | filter-intra predictor, edge filter and upsampling |
| `av1/encoder/arm/av1_highbd_quantize_neon.c`, `aom_dsp/arm/highbd_quantize_neon.c` | 7 high-bitdepth quantizers |
| `av1/encoder/arm/av1_error_neon.c`, `highbd_rdopt_neon.c` | 3 block-error functions |
| `aom_dsp/arm/subtract_neon.c` | 2 subtract functions |

The shared helpers `mem_neon.h`, `sum_neon.h`, `transpose_neon.h`,
`shift_neon.h`, and `txfm_neon.h`, and the `HAVE_NEON` tables of
`av1_txfm.c`, are translated alongside. `crates/aom/rtcd/arm64.pl` lists the
dispatched names and records why the per-size inverse-transform entry points
are not dispatched; the high-bitdepth Hadamard kernels are excluded because
their coefficient order differs from C.

Evidence on macOS ARM64 (Apple M1 Ultra):

- `tools/transpile_aom.py --check` and `--release --check` reproduce both
  regenerated profiles, and `tools/check_translation.py` passes.
- `cargo test --release -p libavif-aom --test simd` runs 63 supplementary
  transcriptions of the original exact-match SIMD tests for these kernels
  (quantize, error block, subtract, CDEF, forward and inverse transforms,
  intra prediction, directional prediction, filter intra, intra edge); all
  pass against the translated `_c` functions.
- `tools/check_aom_simd_identity.py` encodes six images from the libavif test
  data in 76 configurations (speeds 4, 6, 7, 9; 8-, 10-, and 12-bit; 4:2:0 and
  4:4:4; lossless; one thread and all threads) with Neon dispatch and with
  `AOM_SIMD_CAPS_MASK=0`. All 76 AVIF outputs are byte-identical; see
  [the identity record](aom-simd-identity-validation.json).
- `tools/check_aom_roundtrip.py` passes its 12 exact public API round trips
  against the C reference with Neon active.
- The default root Release port passes the same 64 CTest registrations and
  9,473 GoogleTest cases as the baseline linked to the C AOM reference, and
  `tools/differential.py --decoder-codec aom` finds all 10 AVIF bitstreams and
  10 decoded PNGs byte-identical.

The original libaom SIMD tests do not run yet: the original test build uses the
generic configuration and compiles no Neon instantiations. The linkage and
ownership inventories of earlier acceptance records predate the new RTCD
pointer globals and `_neon` symbols and must be recorded again before the next
acceptance claim.

Encoder time for the 1600x2133 photo of the identity corpus, quality 56,
4:2:0, through `examples/encode_still.rs`, median of five runs, each started
with a 1-minute load average below 4 on the shared 20-core machine (see
[the performance record](aom-neon-performance.json)):

| Depth | Speed | Threads | Before (ms) | Neon (ms) | Change |
| --- | --- | --- | --- | --- | --- |
| 10 | 6 | 1 | 2686 | 1540 | -43% |
| 10 | 6 | 20 | 225 | 138 | -39% |
| 10 | 7 | 1 | 1322 | 799 | -40% |
| 10 | 7 | 20 | 126 | 86 | -32% |
| 8 | 6 | 1 | 2390 | 1395 | -42% |
| 8 | 6 | 20 | 196 | 122 | -38% |
| 8 | 7 | 1 | 1191 | 758 | -36% |
| 8 | 7 | 20 | 112 | 79 | -29% |

All three builds write the same bytes in every configuration. The scalar path
of this branch (`AOM_SIMD_CAPS_MASK=0`) stays within 2.5% of the portable
baseline, so the RTCD indirection costs little when SIMD is off.
