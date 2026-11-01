# Keep codec implementations external and port their adapters

Port the libgav1, rav1e, and SVT-AV1 adapter implementations into Rust, preserving
the original codec registration order and feature guards. The codec engines
remain the original external libraries; selecting external libyuv similarly
replaces the bundled scalar fallback exactly as in upstream CMake.

C2Rust cannot translate libyuv's vector typedefs, although libavif only passes
pointers to their containing external constant records. Use a translation-only
header overlay that treats those records as opaque and leaves dependency builds
and the upstream snapshot unchanged. No vector arithmetic is reimplemented or
optimized in Rust.

Canonicalize C2Rust's generated control-flow dispatcher labels so translation
unit identities do not create meaningless module variants. The transformation
renumbers dispatcher assignments and their matching arms without changing edges,
conditions, or source data constants.
