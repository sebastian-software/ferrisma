# Separate Codec and Conversion Crates

Status: portable SharpYUV and dav1d are implemented and integrated. See
[SharpYUV validation](SHARPYUV-VALIDATION.md) and the
[dav1d port and validation](DAV1D-PORT.md). Portable libaom has passed complete
native macOS/Linux standalone acceptance and is the mandatory root dependency;
see the [libaom milestone](AOM-PORT.md). Optional engines retain their separate later
milestones; libaom is translated without VMAF tuning
([ADR 0057](adr/0057-translate-libaom-without-vmaf-tuning.md)).

This follow-up ports the external codec and pixel conversion libraries used by
libavif-rs into separate Rust crates. The first program phase covers SharpYUV,
dav1d, and libaom; later optional libraries have separate milestones. The
existing mechanical porting requirements
remain the starting point: preserve source correspondence, original tests,
behavior, provenance, and separate Conventional Commits. Repository work stays
in US English.

The accepted decisions in [ADR 0007](adr/0007-optional-codec-boundaries.md) and
[ADR 0008](adr/0008-av2-adapter-and-dependency.md) describe the existing external
engine boundary. This planning document does not change that implementation or
rewrite those historical decisions. New accepted decisions will record any
agreed extension of the boundary.

## Observed dependency boundary

The current [build script](../build.rs) and [feature declarations](../Cargo.toml)
establish these direct runtime library relationships. Versions describe the
recorded baseline, not a selected source revision for a future port.

| Library | Existing use | Porting considerations |
| --- | --- | --- |
| libaom | Default AV1 encoding and decoding | Replaced by the portable `libavif-aom` workspace crate; original C remains the independent reference; translated without VMAF tuning (ADR 0057). |
| dav1d | Default AV1 decoding | Replaced by the portable `libavif-dav1d` workspace crate; original C remains its reference. |
| SharpYUV | Default pixel conversion | Replaced by the portable `libavif-sharpyuv` workspace crate; the original libwebp target remains its reference. |
| libgav1 | Optional AV1 decoding | C++ with platform-specific DSP implementations. |
| rav1e | Optional AV1 encoding | Already Rust, currently consumed through its C API; optional assembly remains a separate question. |
| SVT-AV1 | Optional AV1 encoding | C with architecture-specific SIMD and assembly. |
| Full libyuv | Optional pixel conversion and scaling | C++ with architecture-specific implementations; substantially broader than the bundled scalar subset already ported. |
| AVM | Optional experimental AV2 encoding and decoding | Native code with assembly, intrinsics, and configuration-dependent ML dependencies. |

The five bundled scalar libyuv files already translated inside libavif-rs are
distinct from a port of the complete external libyuv library. See the current
[migration contract](PORTING.md) and [validation evidence](VALIDATION.md).

Application input/output libraries, including PNG, JPEG, and libxml2, and the
original C/C++ test infrastructure are separate from this core runtime boundary.
Whether they belong in a later expansion is an open scope decision.

Complete-library scope can introduce additional runtime dependencies that the
current configuration disables. For example, libyuv's public MJPEG decoder is
conditional on JPEG support, which the prepared reference currently disables.

Transitive dependencies require configuration-specific review. The installed
Homebrew libaom package links VMAF, which is not a direct libavif API dependency.
The prepared AVM encoder enables ML experiments that pull TensorFlow Lite and
support libraries. Removing those experiments would change the reference
configuration and require new parity evidence; neither blindly porting every
host package nor silently disabling functionality is an established plan.

## Existing Rust implementations

The selected policy is to prefer new mechanical translations of the original
libraries, because source correspondence and a 1:1 match matter more than
reusing an independently evolved Rust port. The projects below remain useful
comparators; they are not the selected replacement strategy. See
[ADR 0010](adr/0010-port-native-libraries-from-their-original-sources.md).

[rav1e](https://github.com/xiph/rav1e/tree/v0.8.1) is already a Rust AV1 encoder.
Using it instead of libaom would change the codec implementation and would not
establish a mechanical libaom port.

[rav1d](https://github.com/memorysafety/rav1d) is a Rust port of dav1d and retains
the original Meson test harness. It supports a build without its optional
assembly routines. Its suitability for this project remains unverified: the
current baseline uses dav1d 1.5.4, while the
[rav1d 1.1.0 release](https://github.com/memorysafety/rav1d/releases/tag/v1.1.0)
records synchronization with dav1d 1.5.1. An exact candidate revision needs API,
ABI, feature, and behavioral comparison before adoption.

## Resolved decisions

- Replace the delivered C/C++ codec and pixel conversion implementations;
  normal system facilities and original reference/test tooling remain allowed.
- Prefer new mechanical ports of the original libraries for a verifiable 1:1
  match; do not assume an existing Rust implementation is equivalent.
- Aim for complete libraries where functionality is cleanly covered by tests,
  rather than an intentionally limited libavif-only subset.
- Keep separate crates in one Cargo workspace in the existing repository.
- Accept the inventoried primary upstream licenses while retaining original
  terms per imported source; standardize records and generated license notices.
- Require bit-exact bitstreams and decoded pixels, with matching return values
  and errors, against a pinned original and declared reference configuration.
- Start with the portable upstream paths; keep optimization separate and allow
  existing crates to be evaluated in that later work.
- Target Rust consumers and full behavioral parity; existing compiled C++
  applications are outside the compatibility goal. Foreign interfaces remain
  useful for validation without creating a product support obligation.
- Keep unchanged tests where they exercise Rust faithfully; mechanically
  translate coupled C++ tests with explicit case/parameter/assertion mapping
  and preserve the original tests as the native reference.
- Pilot on macOS ARM64 and require Linux x86-64 evidence before releasing a new
  crate. Full-pipeline platform claims need their own integration evidence.
- Start with standalone portable SharpYUV, followed by dav1d and libaom, to
  replace the default native codec/conversion implementations before expanding
  into optional library and transitive dependency closures.

These decisions are recorded in
[ADR 0010](adr/0010-port-native-libraries-from-their-original-sources.md) and
[ADR 0011](adr/0011-keep-library-ports-in-one-cargo-workspace.md),
[ADR 0012](adr/0012-require-bit-exact-results-against-declared-references.md),
[ADR 0013](adr/0013-separate-optimization-from-the-mechanical-port.md),
[ADR 0014](adr/0014-retain-upstream-license-terms-for-ported-crates.md),
[ADR 0015](adr/0015-target-rust-consumers-without-cpp-binary-compatibility.md),
[ADR 0016](adr/0016-preserve-test-cases-when-translating-coupled-tests.md),
[ADR 0017](adr/0017-standardize-license-records-and-generated-notices.md),
[ADR 0018](adr/0018-validate-new-crates-on-macos-arm64-and-linux-x86-64.md), and
[ADR 0019](adr/0019-port-the-default-pipeline-starting-with-sharpyuv.md).

## Planning outcome and execution boundaries

The first crate, initial program scope, ordering, supported targets, test
policy, license-record policy, and exact-match requirements are settled.
The milestones below are the execution plan for this phase.

Before translating each library, complete its immutable source manifest,
reference-build configuration, source closure, and executed test inventory.
These are mandatory preparation tasks, not assumptions of successful parity.
The foundation starts with the existing C2Rust 0.21.0 / LLVM 16.0.6 translator
stack and the installed Rust distribution `nightly-2026-09-07`
(`rustc 1.100.0-nightly`, commit
`5a2be9f5f075d31e3ca5526b5b029881ce441253`). Record and verify platform-specific
native compiler versions and sanitizer compatibility before using their output
as release evidence.

The first phase excludes optional JPEG/ML closures, optimization work,
additional platforms, and broader relicensing. Public package names and actual
crate publication are release tasks; no registry publication is implied by
completing the source port. Those boundaries can be expanded through separate
milestone contracts without changing the agreed default-pipeline scope.

### C++ interfaces matter to test correspondence

Existing C++ application compatibility is excluded by
[ADR 0015](adr/0015-target-rust-consumers-without-cpp-binary-compatibility.md).
The source facts below now inform validation bridges and Rust test coverage,
rather than a requirement to ship a binary-compatible C++ library.

libgav1 exposes a public C++ `Decoder` class in addition to its C functions.
Its `GetFramesMeanQpInTemporalUnit()` method returns `std::vector<int>` and has
no C API counterpart. The public class also contains C++ standard-library
members. Original decoder tests construct this class directly. See the pinned
[decoder header](https://chromium.googlesource.com/codecs/libgav1/+/c05bf9be660cf170d7c26bd06bb42b3322180e58/src/gav1/decoder.h).

libyuv similarly exposes a public `MJpegDecoder` C++ class when JPEG support is
enabled; original conversion tests use it. See the pinned
[MJPEG header](https://chromium.googlesource.com/libyuv/libyuv/+/26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0/include/libyuv/mjpeg_decoder.h).

Test compatibility layers must have an explicit scope; they must not quietly
reintroduce the original algorithms or count original C++ execution as Rust
coverage. Internal C++ unit tests may require additional bridging or mechanical
test translation beyond the public API. The feasibility of unchanged test
linkage needs a pilot rather than an assumption.

At the pinned libgav1 revision, 52 of 57 `src/**/*_test.{c,cc}` source files
directly include at least one implementation header outside the public
`src/gav1/` directory; the remaining five comprise two C API tests and three
tests of public C++ headers. These are static source counts, including
architecture-specific sources, rather than executed test counts. Some internal
tests instantiate header-only templates, including `Array2DView` and `Vector`.
Compiling those unchanged implementation headers into the candidate test
program would still exercise original C++ algorithms, not their Rust ports.

The selected policy is to preserve original C++ tests as the native reference,
retain unchanged consumer tests through bridges where feasible, and
mechanically translate internally coupled tests to Rust with an explicit
case/parameter/assertion mapping. See
[ADR 0016](adr/0016-preserve-test-cases-when-translating-coupled-tests.md).
Neither a public facade nor an exit code of zero establishes whole-suite Rust
coverage.

### Pilot evidence

SharpYUV's public libwebp 1.6.0 source has five scalar/core C translation units,
approximately 1,230 physical lines including comments and tables. The source
closure includes shared CPU support outside its own directory. It is a small
port candidate, but the release contains no dedicated SharpYUV unit-test suite.
The available upstream WebP fuzzers exercise it indirectly, and their external
test-data repository would need a pinned revision.

A SharpYUV pilot therefore needs clearly identified new differential
tests against the pinned scalar C reference in addition to the available
original fuzz/integration tests and libavif suite. It must not claim to retain
a dedicated upstream SharpYUV suite that does not exist. Its selected boundary
and acceptance gates follow below.

## First milestone: standalone portable SharpYUV

The selected source boundary is the standalone `sharpyuv` target from libwebp
v1.6.0, whose SharpYUV API version is 0.4.2. The annotated tag resolves to
[commit `4fa21912338357f89e4fd51cf2368325b59e9bd9`](https://github.com/webmproject/libwebp/tree/4fa21912338357f89e4fd51cf2368325b59e9bd9).
Record archive and file hashes before translation. Preserve the complete
source and notices, including shared CPU-support source outside `sharpyuv/`.
The `libwebpextras` risk-estimation functions are a different library boundary.

Use the original `WEBP_ENABLE_SIMD=OFF` configuration for the portable native
reference and the native test consumers. Record threading and compiler options
independently. This selects the upstream portable path; it does not require
disabling ordinary compiler code generation or prove optimized CPU coverage.

### Implementation and API coverage

- Preserve the six declared exported entry points: `SharpYuvGetVersion`,
  `SharpYuvConvert`, `SharpYuvConvertWithOptions`,
  `SharpYuvOptionsInitInternal`, `SharpYuvComputeConversionMatrix`, and
  `SharpYuvGetConversionMatrix`.
- Preserve the initialization behavior behind the inline `SharpYuvOptionsInit`
  wrapper and the exported `SharpYuvInit` entry point used by original libwebp.
- Cover RGB depths 8/10/12/16, YUV420 depths 8/10/12, all sixteen defined
  transfer-function choices, the five predefined matrices, and documented
  matrix computation. The original truncation of sixteen-bit RGB input is
  preserved rather than redesigned.
- Exercise interleaved and planar layouts, padded rows, valid aligned
  high-bit-depth buffers, odd/even dimensions, narrow images, initialization,
  defined argument-rejection behavior, and allocation-failure cleanup.
- Keep source preconditions explicit. Arbitrary malformed inputs that trigger
  undefined behavior in C are not automatically a defined equivalence oracle.

The versioned
[conversion header](https://raw.githubusercontent.com/webmproject/libwebp/v1.6.0/sharpyuv/sharpyuv.h),
[colorspace header](https://raw.githubusercontent.com/webmproject/libwebp/v1.6.0/sharpyuv/sharpyuv_csp.h),
[implementation](https://raw.githubusercontent.com/webmproject/libwebp/v1.6.0/sharpyuv/sharpyuv.c),
and [native consumer](https://raw.githubusercontent.com/webmproject/libwebp/v1.6.0/src/enc/picture_csp_enc.c)
establish this boundary.

### Acceptance evidence

1. Upstream source hashes and generated Rust correspondence are reproducible;
   required license and attribution records are complete for the imported code.
2. Native and Rust entry-point results match exactly on a deterministic corpus
   and supplemental seeded differential cases covering the declared API matrix.
3. Available original WebP fuzz/integration tests and libavif tests exercise the
   Rust implementation through an explicit target replacement. Keep their
   native-reference results and executed inventories separately.
4. Compile commands, archive members, symbols, and linker evidence establish
   that the candidate consumer uses Rust SharpYUV, including `SharpYuvInit`,
   with no original or system SharpYUV implementation masking the port.
5. Native-reference and Rust differential execution are isolated to avoid
   duplicate unprefixed symbols or dynamic interposition hiding the candidate.
6. The standalone crate passes its native comparisons on macOS ARM64 and Linux
   x86-64. Existing libavif integration runs on its supported host; Linux
   integration of the root package remains a separate validation gate.
7. Record remaining configurations and unavailable cases explicitly. Passing
   indirect fuzzers alone does not establish coverage of the complete API, and
   this milestone does not claim SIMD or complete WebP codec coverage.
8. Run applicable instrumented memory/cleanup checks and both development and
   release comparisons. Record sanitizer runtime/toolchain compatibility and
   allocation-failure evidence instead of inferring them from normal output
   equality.

### Commit-sized execution steps

1. Introduce the workspace member, immutable upstream manifest, standard license
   records, and target-specific reference profiles. Verify the existing
   libavif snapshot and retain its passing integration boundary.
2. Establish the original scalar C reference and deterministic API corpus,
   including version/options behavior, pixel layouts, matrix/transfer choices,
   defined failures, and the full exported-symbol closure.
3. Import the portable Rust translation with reproducible correction rules and
   recorded hashes. Preserve source structure and numeric behavior.
4. Add exact differential evidence in development and release configurations,
   memory/cleanup checks, and independent macOS ARM64 / Linux x86-64 jobs.
5. Replace SharpYUV explicitly in original test consumers and libavif
   integration. Audit sources, symbols, and linkage before interpreting test
   results as Rust coverage.
6. Record the actual case inventory, target/compiler configuration, comparison
   results, source and artifact hashes, and remaining gaps. Keep registry
   publication separate from implementation and validation.

Commit each coherent verified step separately using Conventional Commits.
Do not combine optimization or an upstream version update with a mechanical
porting step.

## Program sequence

| Stage | Deliverable | Required boundary |
| --- | --- | --- |
| Foundation | Shared workspace, provenance, native reference isolation, test correspondence records, license generation, and target-specific CI | Keep current libavif behavior and original snapshot intact; independent Linux jobs do not imply root Linux support. |
| SharpYUV | Complete selected portable SharpYUV implementation and its integration | The selected milestone above; no dedicated upstream SharpYUV suite is invented. |
| dav1d | Portable original AV1 decoder as its own crate | Public behavior, bit depths, frame/thread state, film grain, ownership, errors, and internal DSP evidence; assembly-gated checkasm coverage cannot silently disappear. |
| libaom | Portable original AV1 encoder and decoder as its own crate | Pin enabled features and reference builds; preserve full selected-library behavior rather than only AVIF still-image smoke paths. |
| Default integration | libavif's default codec/conversion implementations resolve to Rust crates | Complete on macOS ARM64, including actual linked-implementation audits, original suites, and strict AVIF/PNG comparisons against the noncontracting C reference; root Linux support remains separate. |
| Later library ports | Full libyuv, libgav1, and SVT-AV1 in a separately detailed sequence | Prove internal C++ test correspondence before scaling C++ ports; resolve optional transitive features explicitly. |
| Separate AV2 work | AVM and its selected experiment/dependency closure | Resolve ML and transitive scope before accepting an AVM completion contract. |

rav1e is already Rust. Its existing implementation can be integrated as a pinned
Rust dependency rather than mechanically ported again; any change from its
current native-library packaging still needs matching behavior and feature
evidence. It does not replace the selected libaom implementation.

Full libyuv's MJPEG path delegates to JPEG and therefore needs a separate JPEG
implementation decision. AVM's configured ML experiments introduce another
substantial dependency graph. Neither is silently included as an unbounded
porting obligation or removed while claiming complete library parity. Their
full-library completion claims remain blocked until the selected closure is
explicitly implemented and tested.

Detailed contracts for later optional libraries will be prepared before their
implementation. SharpYUV, dav1d, and libaom are implemented and integrated.
The libaom contract pins the complete portable 3.15.2 archive and original test
closure in [AOM-PORT.md](AOM-PORT.md). Native macOS/Linux standalone acceptance
and final macOS root validation pass. It is translated without VMAF tuning, so
libvmaf is not a dependency (ADR 0057). The current overview is the
[port checklist](../README.md#port-checklist).

## Evidence for each milestone

The existing libavif suite supplies integration evidence. Each dependency also
needs its own pinned upstream source, native baseline, original test harness,
and applicable differential and ABI checks. Compilation or a passing libavif
subset alone cannot establish complete dependency parity.

Apply the concrete SharpYUV gates above to its first milestone. The libaom
references and case inventories are recorded in `AOM-PORT.md`; the dav1d
milestone records its evidence in `DAV1D-PORT.md`. Existing
validation limitations, including root platform coverage and compiler-default
RGB diagnostics, remain recorded in
[VALIDATION.md](VALIDATION.md).
