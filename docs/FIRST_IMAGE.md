# First complete image path

The active milestone is a complete image encode/decode path through the Rust
translation of the pinned source. Completing an entropy or header slice does
not complete this milestone.

## Acceptance evidence

- Feed pixel buffers into the translated encoder and obtain a complete JPEG XL
  codestream, including image metadata, frame header, sections, and image data.
- Decode complete codestreams in Rust and recover dimensions, channel layout,
  bit depth, and pixel samples.
- Exercise the same encoder configuration in the original C++ implementation.
  Compare encoded bytes and decoded samples, with Rust/C++ cross-decoding in
  both directions. Begin with the original lossless modular path and a corpus
  containing grayscale and RGB images, multiple dimensions, constant colors,
  ramps, and deterministic noise.
- Verify malformed and truncated inputs at the integrated boundaries. Preserve
  the source's status and partially completed state.
- Keep C++ exclusively in the test oracle. No runtime fallback, image-specific
  hardcoding, alternate codec implementation, or hand-built codestream replaces
  a missing source algorithm.
- Run the repository checks in debug, release, and the minimum Rust version;
  instrument the original C++ with ASan/UBSan. Keep all earlier parity cases.
- Commit coherent changes with unsigned Conventional Commits and maintain the
  living ADRs and source mapping in US English.

This milestone establishes one complete path. The full library port additionally
requires the remaining encoder configurations, VarDCT, animation, metadata,
public API, threading, target-specific code, and the full gates in
[PORTING.md](PORTING.md).

## Initial source configuration

Use the original Thunder speed tier (public effort 2), SetLossless, codestream
level 10, and buffering 0 to select the non-streaming EncodeFrameOneShot path.
Use one regular, final frame with integer 8-bit or 16-bit grayscale/RGB samples,
sRGB metadata, no extra channels, and no animation. Keep all other original
parameter defaults, including palette/compaction selection and the default
color transform policy. Use the scalar, null-pool configuration shared with the
existing differential oracle.

For the initial decoder, explicitly select the original
`use_slow_rendering_pipeline=true` option, which uses its simple render
implementation. The default low-memory pipeline remains a separate boundary.
The current fixed frame entry point consumes a complete frame header, TOC, and
sections with caller-supplied private configuration metadata. Connecting global
codestream metadata parsing and integer pixel export is still required by the
acceptance evidence above.

Represent this fixed configuration with private metadata/parameter ownership so
callers cannot select unported branches through these entry points. The general
image API remains absent until its additional paths are translated. Partial
pixel preparation or section output still does not satisfy the acceptance
evidence above.

## Dependency queue

1. Entropy streams: hybrid integers, ANS histogram parsing and encoding,
   context maps, symbol state, and LZ77.
2. Image metadata, frame headers, section tables, and the backing image/channel
   storage used by the source path.
3. Modular prediction trees, residual encoding/decoding, and the transforms
   reached by that configuration.
4. Connect the original image/frame entry points and run the cross-language
   image corpus above.

[PORT_STATUS.md](PORT_STATUS.md) records verified completed boundaries and
remaining work. Keep this acceptance boundary intact as dependencies land.
