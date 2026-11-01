# First complete image path

The first complete image path is implemented and verified for the fixed source
configuration below. It connects pixel buffers, complete raw JPEG XL
codestreams, parsed metadata/frames, and exact integer samples. This milestone
does not establish completion of the library port.

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
The complete decoder reads global metadata directly from the supplied
codestream, admits it to private configuration ownership, and executes the
frame/TOC/section path before the original integer buffer conversion. The
component frame entry point also remains available with caller-supplied metadata.

Represent this fixed configuration with private metadata/parameter ownership so
callers cannot select unported branches through these entry points. The general
image API remains absent until its additional paths are translated. Partial
pixel preparation or section output still does not satisfy the acceptance
evidence above.

## Completion evidence

Verified on 2026-11-01; the original acceptance boundary above is unchanged.

| Requirement | Evidence |
| --- | --- |
| Pixel buffers to complete codestream | `LosslessThunderConfig::encode_pixels` in [enc_frame.rs](../src/jxl/enc_frame.rs); 140 additional source/native encoder comparisons |
| Whole-file parsing to integer samples | `decode_lossless_thunder` in [dec_frame.rs](../src/jxl/dec_frame.rs), connected to [dec_external_image.rs](../src/jxl/dec_external_image.rs) |
| Exact C++ bytes, metadata, and cross-decoding | [Whole-image parity tests](../tests/parity/whole_image.rs): 140 original/public-decoder checks and 280 original/Rust complete-file decode comparisons across seven dimensions, gray/RGB, 8/16 bits, constants, ramps, palettes, and deterministic noise |
| Malformed/truncated integrated input | The same suite compares 661 marker, prefix, trailing-byte, and payload-corruption cases, including defined partial metadata and frame state |
| Ordinary Rust path without C++ | [first_image.rs](../tests/first_image.rs) runs without `cpp-oracle`; it checks 16 complete round trips and a 256x256 image containing every 16-bit sample |
| Original reference implementation | The oracle compiles the exact original DecodeFile/ReadICC bodies using [hash-checked source extraction](../tools/oracle/source_fragments.json), the real original frame decoder, and the public C++ decoder |
| Required checks | Both `python3 tools/check.py --sanitizers --msrv` and `python3 tools/check.py --msrv`: 323 tests, 183 differential groups, and 703,419 requests per configuration; zero failed or ignored cases |
| Delivery conventions | Repository artifacts use US English, living ADRs, and unsigned Conventional Commits; no publication or runtime fallback |

[PORT_STATUS.md](PORT_STATUS.md) records the remaining general codec, platform,
and release boundaries. The default low-memory pipeline, other configurations,
public C ABI, threading, SIMD targets, and full-library acceptance remain future
work.
