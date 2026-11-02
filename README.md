# libjxl-rs

A mechanical Rust port of the JPEG XL reference implementation, starting from
the pinned [libjxl](https://github.com/libjxl/libjxl) source. Phase one preserves
source algorithms and behavior before any redesign or additional optimization.

**The first complete lossless image path is implemented in Rust.** The fixed
Thunder configuration encodes 8-/16-bit grayscale or RGB buffers into complete
raw JPEG XL codestreams. Rust parses their global metadata and frames and
recovers the original integer pixels. Encoded bytes, metadata, samples, and
error paths are compared with pinned libjxl, including cross-decoding in both
directions. See the [first-image evidence](docs/FIRST_IMAGE.md). General codec
configurations and the public C API remain unported.

**The first complete VarDCT decode path is also implemented.** `decode_vardct`
parses raw original codestreams through DC/AC groups, dequantization and scalar
inverse transforms to exact sRGB float and integer pixels. Its admitted profile
includes gray/RGB, 8/16-bit samples, optional full-resolution alpha, progressive
AC passes and optional Gaborish. See [the VarDCT milestone](docs/VARDCT_IMAGE.md)
for the precise configuration, source mapping, evidence and remaining scope.

## Port checklist

This checklist tracks the mechanical port of the core codec, public C API and
thread runners. Checked items are implemented and verified for the scope stated
below. Open items can already contain translated components; their complete
integration or acceptance gates remain unfinished. The detailed source mapping
and evidence live in [PORT_STATUS.md](docs/PORT_STATUS.md).

Keep checked items aligned with validated work on `main`. Update this checklist
in the PR that delivers each milestone, together with its source mapping and
validation evidence. Split partial milestones into smaller items as needed;
the items vary in size and do not represent equal amounts of work.

### Completed foundations and image paths

- [x] Pin the original source and dependencies, retain license/provenance records,
  and maintain [living ADRs](docs/adr/README.md).
- [x] Port bit I/O and the documented field-coding, metadata, frame-header,
  geometry and section-table boundaries.
- [x] Port the documented scalar entropy boundaries: ANS/Huffman and prefix
  coding, hybrid integers, LZ77, histograms and context maps.
- [x] Encode and decode standalone Modular streams with caller-supplied dimensions
  and bit depth, including predictors, learned trees, RCT, palettes and Squeeze.
- [x] Encode and decode complete raw codestreams for the fixed lossless Thunder
  profile: 8/16-bit gray/RGB and optional full-resolution unassociated alpha.
  See [the first-image milestone](docs/FIRST_IMAGE.md).
- [x] Connect VarDCT DC/AC globals, metadata, group scheduling, progressive AC
  accumulation, dequantization and all 27 scalar inverse-transform strategies.
- [x] Decode complete raw codestreams for the first VarDCT profile: 8/16-bit
  gray/RGB, optional full-resolution unassociated alpha, multiple DC/AC groups,
  progressive AC and optional Gaborish. See [the profile and evidence](docs/VARDCT_IMAGE.md).
- [x] Connect the original simple rendering pipeline, scalar Gaborish/XYB/sRGB
  stages and pixel-buffer export for the admitted image profiles.
- [x] Generate ICC profiles and serialize the documented color-encoding fields.
- [x] Compare translated boundaries with ordinary and ASan/UBSan original C++
  references in Debug, Release and Rust 1.85, with native Linux/macOS CI.
  Ordinary Rust builds have no external crate dependencies and do not compile
  or link C/C++.

### Remaining decoder work

The [decoder expansion plan](docs/DECODER_EXPANSION.md) records the delivery
sequence and acceptance gates for this work.

- [x] Port EPF rendering kernels and integrate them into complete-file decoding;
  sigma generation is already translated.
- [x] Complete noise decoding and rendering.
- [x] Complete patch decoding and rendering.
- [x] Complete spline decoding and rendering.
- [x] Expand complete-file Modular decoding beyond the fixed Thunder profile.
- [x] Support reference/DC frames, blending, animation and previews.
- [ ] Connect chroma subsampling, cropping, resampling and orientation in general
  complete-image decoding.
- [x] Support associated/subsampled alpha and the remaining extra-channel types.
- [ ] Connect alternate color profiles and HDR/tone-mapping paths beyond the
  admitted sRGB profiles.
- [ ] Reconstruct original JPEG bitstreams and restore their metadata.
- [ ] Port the public decoder API, including incremental input, events, callbacks
  and progressive output.

### Remaining encoder work

- [ ] Complete the VarDCT encoder, including strategy selection, perceptual
  quantization, coefficient-order orchestration and progressive coefficient splitting.
- [ ] Complete general Modular image/frame encoding beyond the fixed Thunder profile.
- [ ] Support JPEG input and lossless JPEG recompression encoding.
- [ ] Port the public encoder API, queued frames, input callbacks and output
  processors.

### Shared runtime and full-port acceptance

- [ ] Complete the remaining core helpers and source boundaries listed in
  [the source mapping](docs/PORT_STATUS.md).
- [ ] Read and write JPEG XL containers and their boxes.
- [ ] Resolve default CMS backend and ICC parsing integration.
- [ ] Complete custom allocators, thread runners and the low-memory rendering pipeline.
- [ ] Translate the remaining original SIMD target specializations and dispatch.
- [ ] Pass the full upstream-derived test inventory and codec conformance corpus,
  including image samples, metadata, frame sequences, JPEG reconstruction,
  encoder bitstreams and public API transitions.
- [ ] Complete fuzzing, applicable sanitizer gates and the supported platform/CPU matrix.
- [ ] Measure performance and memory use and complete release validation.

Tools, examples, plugins and third-party library ports are separate boundaries
under the [porting contract](docs/PORTING.md). Redesign and additional
optimization follow the mechanical port.

## Build and verify

With the source initialized as described in [CONTRIBUTING.md](CONTRIBUTING.md):

```sh
cargo test --locked --offline
python3 tools/check.py
```

The complete check builds the C++ oracle, runs Clippy, and compares encoded
bytes, decoded values, statuses, bit counts, truncated headers, and lifecycle
checkpoints in debug and release Rust builds. Missing oracle prerequisites are
an error. All binary16 input patterns are compared against the original code.

The provisional Rust API follows upstream's low-level module structure under
`jxl::jxl`. It is not a stable public API, and the crate is not publishable yet.

## First image path

```rust
use jxl::jxl::dec_frame::decode_lossless_thunder;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

fn round_trip() -> Result<(), jxl::jxl::base::status::StatusCode> {
    let pixels = [255, 0, 0, 0, 255, 0]; // Two RGB pixels.
    let mut config = LosslessThunderConfig::new(
        2, 1, false, LosslessSampleDepth::Uint8,
    )?;
    let encoded = config.encode_pixels(&pixels, None)?;
    let decoded = decode_lossless_thunder(encoded.as_slice());
    decoded.status?;
    assert_eq!(decoded.pixels.as_slice(), &pixels);
    Ok(())
}
```

The decoder obtains dimensions, channel layout, and bit depth from the file.
Output is interleaved bytes, with native-endian words for 16-bit samples.
The fixed profile uses sRGB, one final frame, and the original simple rendering
option. `LosslessThunderConfig::new_with_alpha` also accepts interleaved RGBA
or gray/alpha with one full-resolution, unassociated alpha channel at the color
sample depth. Lossless defaults preserve color values under zero alpha.
Animation and other extra channels remain outside this profile. Input outside
that profile has an explicit admission boundary; this is not the general
decoder API.

## Project records

- [Porting contract](docs/PORTING.md): scope, invariants, and gates.
- [Port status](docs/PORT_STATUS.md): implemented units and remaining work.
- [Living ADRs](docs/adr/README.md): current decisions and rationale.
- [Provenance](docs/PROVENANCE.md): source and test origins.
- [Contributing](CONTRIBUTING.md): prerequisites, checks, and commit conventions.

Derived source retains the upstream [BSD-3-Clause license](LICENSE) and
[additional IP rights grant](PATENTS). The C++ reference and Highway dependency
remain separate Git submodules with their own notices.
The translated Highway prefetch helper retains its
[BSD license notice](licenses/HIGHWAY-BSD-3-Clause.txt).
