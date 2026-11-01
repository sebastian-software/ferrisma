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
