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

Transitive dependencies require configuration-specific review. The installed
Homebrew libaom package links VMAF, which is not a direct libavif API dependency.
The prepared AVM encoder enables ML experiments that pull TensorFlow Lite and
support libraries. Removing those experiments would change the reference
configuration and require new parity evidence; neither blindly porting every
host package nor silently disabling functionality is an established plan.

## Existing Rust implementations

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

## Decision tree

The first interview round addresses these independent roots:

1. Define the delivered runtime boundary, including the distinction between
   codec implementations, system facilities, and reference/test tooling.
2. Decide whether an existing Rust port can satisfy a dependency milestone
   after the same compatibility gates as a new port.
3. Choose complete-library ports or a deliberately smaller libavif-specific
   scope.
4. Choose a shared workspace or separate repositories for the independent
   crates.

Those answers determine later questions about assembly and SIMD, transitive
libraries, the first milestone and port order, supported platforms, ABI and
output parity, performance gates, maintenance, and publication. None of those
choices is implied by the inventory above.

## Proposed evidence for each milestone

The existing libavif suite supplies integration evidence. Each dependency also
needs its own pinned upstream source, native baseline, original test harness,
and applicable differential and ABI checks. Compilation or a passing libavif
subset alone cannot establish complete dependency parity.

The concrete gates will be selected after the target and scope decisions. The
existing validation limitations, including platform coverage and strict RGB
byte differences, remain recorded in [VALIDATION.md](VALIDATION.md).
