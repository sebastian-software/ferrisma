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

## Resolved first-round decisions

- Replace the delivered C/C++ codec and pixel conversion implementations;
  normal system facilities and original reference/test tooling remain allowed.
- Prefer new mechanical ports of the original libraries for a verifiable 1:1
  match; do not assume an existing Rust implementation is equivalent.
- Aim for complete libraries where functionality is cleanly covered by tests,
  rather than an intentionally limited libavif-only subset.
- Keep separate crates in one Cargo workspace in the existing repository.

These decisions are recorded in
[ADR 0010](adr/0010-port-native-libraries-from-their-original-sources.md) and
[ADR 0011](adr/0011-keep-library-ports-in-one-cargo-workspace.md).

## Open decision tree

- License policy: the primary upstream licenses differ; see the
  [versioned license inventory](NATIVE-LICENSES.md). File-level and transitive
  terms need to be preserved according to the selected scope.
- Equivalence: define exactly what a 1:1 match preserves and under which source
  revision, build configuration, compiler rules, and target.
- Assembly and SIMD: decide the end state and the role of the original scalar
  implementations during migration.
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

## Proposed evidence for each milestone

The existing libavif suite supplies integration evidence. Each dependency also
needs its own pinned upstream source, native baseline, original test harness,
and applicable differential and ABI checks. Compilation or a passing libavif
subset alone cannot establish complete dependency parity.

The concrete gates will be selected after the target and scope decisions. The
existing validation limitations, including platform coverage and strict RGB
byte differences, remain recorded in [VALIDATION.md](VALIDATION.md).
