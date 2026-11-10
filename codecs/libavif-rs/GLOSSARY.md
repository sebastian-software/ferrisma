# AVIF and Codec Ports

Terms used when discussing the image container and the codec libraries it uses.

## Language

**AVIF**:
An image file format that stores AV1-coded images and associated metadata in an
ISO Base Media File Format container.
_Avoid_: AV1, image codec

**AV1**:
The compression format used for the image bitstreams in AVIF files.
_Avoid_: AVIF, container format

**AV2**:
The codec family following AV1, supported experimentally by the pinned libavif
configuration through AVM.
_Avoid_: Another AV1 backend

**Codec engine**:
An implementation that encodes or decodes a compressed image or video bitstream.
_Avoid_: Container parser

**Pixel conversion**:
A transformation between pixel representations, such as RGB and YUV, rather
than compression into or decompression from a codec bitstream.
_Avoid_: Encoding, decoding

**Bit-exact match**:
Equality of every compared encoded byte or decoded pixel sample, rather than
similar visual appearance or a result within a numerical tolerance.
_Avoid_: Visually identical, approximately equivalent

**Reference configuration**:
The fixed source revision, enabled features, target, and compiler settings that
define a particular equivalence comparison.
_Avoid_: Universal parity, compiler-independent baseline

**Portable configuration**:
The selected upstream scalar algorithms with manually optimized SIMD paths
disabled; ordinary compiler code generation remains part of the reference configuration.
_Avoid_: Instruction-set-free code, no machine SIMD instructions

**Primary reference**:
The original implementation under the declared reference configuration against
which the port's results are accepted.
_Avoid_: Diagnostic oracle, alternate baseline

**Diagnostic reference**:
A deliberately varied original build used to investigate a discrepancy without
replacing the primary reference.
_Avoid_: Acceptance baseline
