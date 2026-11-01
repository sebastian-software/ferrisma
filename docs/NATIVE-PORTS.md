# Planning Separate Codec and Conversion Crates

Status: discovery interview in progress.

This follow-up explores Rust implementations of the external codec and pixel
conversion libraries currently used by libavif-rs. Each newly ported library is
intended to be a separate crate. The existing mechanical porting requirements
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
| libaom | Default AV1 encoding and decoding | C implementation with architecture-specific assembly and intrinsics; broader than still images. |
| dav1d | Default AV1 decoding | C and assembly; an existing Rust port, rav1d, is a candidate for evaluation. |
| SharpYUV | Default pixel conversion | Separate C library target distributed inside libwebp; it does not require porting the WebP codec. |
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
  terms per imported source; simplification remains a separate policy topic.
- Require bit-exact bitstreams and decoded pixels, with matching return values
  and errors, against a pinned original and declared reference configuration.
- Start with the portable upstream paths; keep optimization separate and allow
  existing crates to be evaluated in that later work.

These decisions are recorded in
[ADR 0010](adr/0010-port-native-libraries-from-their-original-sources.md) and
[ADR 0011](adr/0011-keep-library-ports-in-one-cargo-workspace.md),
[ADR 0012](adr/0012-require-bit-exact-results-against-declared-references.md),
[ADR 0013](adr/0013-separate-optimization-from-the-mechanical-port.md), and
[ADR 0014](adr/0014-retain-upstream-license-terms-for-ported-crates.md).

## Open decision tree

- License simplification: select a consistent presentation and tooling
  convention; keep any broader relicensing proposal distinct.
- Concrete references: choose exact source revisions, reproducible build
  configurations, and execution matrices for each milestone.
- Optimization: decide its later scope, supported CPU paths, and performance
  gates separately from the portable mechanical ports.
- Supported platforms: distinguish pilot evidence from release support.
- API and ABI: define the treatment of public C and C++ interfaces.

Those answers determine the first milestone and port order, transitive library
scope, concrete parity and performance gates, maintenance, package identities,
and publication. No implementation work is authorized by completing a single
interview round; the complete plan is still being developed.

### Public C++ interfaces need a separate compatibility decision

libgav1 exposes a public C++ `Decoder` class in addition to its C functions.
Its `GetFramesMeanQpInTemporalUnit()` method returns `std::vector<int>` and has
no C API counterpart. The public class also contains C++ standard-library
members. Original decoder tests construct this class directly. See the pinned
[decoder header](https://chromium.googlesource.com/codecs/libgav1/+/c05bf9be660cf170d7c26bd06bb42b3322180e58/src/gav1/decoder.h).

libyuv similarly exposes a public `MJpegDecoder` C++ class when JPEG support is
enabled; original conversion tests use it. See the pinned
[MJPEG header](https://chromium.googlesource.com/libyuv/libyuv/+/26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0/include/libyuv/mjpeg_decoder.h).

Preserving functionality, preserving the C ABI, retaining the original C++
tests, and shipping a binary-compatible C++ library are distinct requirements.
Any compatibility layer must have an explicit scope; it must not quietly
reintroduce the original algorithms or count original C++ execution as Rust
coverage. Internal C++ unit tests may require additional bridging beyond the
public API. The feasibility of unchanged linkage needs a pilot rather than an
assumption.

At the pinned libgav1 revision, 52 of 57 `src/**/*_test.{c,cc}` source files
directly include at least one implementation header outside the public
`src/gav1/` directory; the remaining five comprise two C API tests and three
tests of public C++ headers. These are static source counts, including
architecture-specific sources, rather than executed test counts. Some internal
tests instantiate header-only templates, including `Array2DView` and `Vector`.
Compiling those unchanged implementation headers into the candidate test
program would still exercise original C++ algorithms, not their Rust ports.

A possible policy is to preserve original C++ tests as the native reference,
retain unchanged consumer tests through bridges where feasible, and
mechanically translate internally coupled tests to Rust with an explicit
case/parameter/assertion mapping. This is a proposal, not an accepted exception
to the original-test requirement. Neither a public facade nor an exit code of
zero establishes whole-suite Rust coverage.

### Candidate pilot evidence

SharpYUV's public libwebp 1.6.0 source has five scalar/core C translation units,
approximately 1,230 physical lines including comments and tables. The source
closure includes shared CPU support outside its own directory. It is a small
port candidate, but the release contains no dedicated SharpYUV unit-test suite.
The available upstream WebP fuzzers exercise it indirectly, and their external
test-data repository would need a pinned revision.

A SharpYUV pilot would therefore need clearly identified new differential
tests against the pinned scalar C reference in addition to the available
original fuzz/integration tests and libavif suite. It must not claim to retain
a dedicated upstream SharpYUV suite that does not exist. Pilot selection and
acceptance gates remain open.

## Proposed evidence for each milestone

The existing libavif suite supplies integration evidence. Each dependency also
needs its own pinned upstream source, native baseline, original test harness,
and applicable differential and ABI checks. Compilation or a passing libavif
subset alone cannot establish complete dependency parity.

The concrete gates will be selected after the target and scope decisions. The
existing validation limitations, including platform coverage and strict RGB
byte differences, remain recorded in [VALIDATION.md](VALIDATION.md).
