# libjxl-rs

This port is maintained in the [Ferrisma workspace](../../README.md). See the
[project status](../../docs/status.md) and [codec guide](../../docs/codecs.md)
for integration and checks; local porting contracts and licenses still apply.
Ferrisma currently uses `jxl-oxide` for JPEG XL input, not this port
([ADR-0025](../../docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)).

A mechanical Rust port of the JPEG XL reference implementation, starting from
the pinned [libjxl](https://github.com/libjxl/libjxl) source. Phase one preserves
source algorithms and behavior before any redesign or additional optimization.

**The first complete lossless image path is implemented in Rust.** The fixed
Thunder configuration encodes 8-/16-bit grayscale or RGB buffers into complete
raw JPEG XL codestreams. Rust parses their global metadata and frames and
recovers the original integer pixels. Encoded bytes, metadata, samples, and
error paths are compared with pinned libjxl, including cross-decoding in both
directions. See the [first-image evidence](docs/FIRST_IMAGE.md). General encoder
configurations and the encoder C ABI remain unported. The public encoder API's
lifecycle, setters and input queueing are translated and return explicit
admission results where the original would enter an untranslated branch, so it
encodes no pixels yet.

**The first complete VarDCT decode path is also implemented.** `decode_vardct`
parses raw original codestreams through DC/AC groups, dequantization and scalar
inverse transforms to exact sRGB float and integer pixels. Its admitted profile
includes gray/RGB, 8/16-bit samples, optional full-resolution alpha, progressive
AC passes and optional Gaborish. See [the VarDCT milestone](docs/VARDCT_IMAGE.md)
for the precise configuration, source mapping, evidence and remaining scope.

**The public decoder API is ported to Rust.** `jxl::jxl::decode` translates
`decode.cc`: incremental input, raw codestreams and containers with their
boxes, events, output buffers and callbacks, progressive output, color and
HDR output through the translated default CMS, and JPEG reconstruction. Caller
histories, including every test in the pinned `decode_test.cc`, compare exactly
with the original public API. The thread and resizable runners are ported, and
`jxl_decoder_set_parallel_runner` runs the decoder's parallel schedules through
them; the task bodies still run one at a time. See [the port status](docs/PORT_STATUS.md#public-incremental-decoder-api-2026-11-04).

**The decoder and runner C ABI is exported.** The crate's static and dynamic
libraries (`libjxl.a`, `libjxl.so` or `libjxl.dylib`) export the functions of
the pinned `decode.h`, `thread_parallel_runner.h` and
`resizable_parallel_runner.h` under their original names, so a C program
written against those headers can link the Rust library instead of libjxl and
libjxl_threads. `JxlGetDefaultCms` comes from the separate original `jxl_cms`
library. A C driver linked against either library compares every public
decoder history exactly, with and without parallel runners. See [the port status](docs/PORT_STATUS.md#exported-decoder-c-abi-2026-11-04).

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
  references using optimized Rust with overflow checks and debug assertions,
  with native Linux/macOS CI and a Rust 1.95 compile check.
  Ordinary Rust builds have no external crate dependencies and do not compile
  or link C/C++.

### Decoder work

The [decoder expansion plan](docs/DECODER_EXPANSION.md) records the delivery
sequence and acceptance gates for this work.

- [x] Port EPF rendering kernels and integrate them into complete-file decoding;
  sigma generation is already translated.
- [x] Complete noise decoding and rendering.
- [x] Complete patch decoding and rendering.
- [x] Complete spline decoding and rendering.
- [x] Expand complete-file Modular decoding beyond the fixed Thunder profile.
- [x] Support reference/DC frames, blending, animation and previews.
- [x] Connect chroma subsampling, cropping, resampling and orientation in general
  complete-image decoding.
- [x] Support associated/subsampled alpha and the remaining extra-channel types.
- [x] Connect alternate color profiles and HDR/tone-mapping paths beyond the
  admitted sRGB profiles, including CMS output conversions through the
  public decoder.
- [x] Reconstruct original JPEG bitstreams and restore their Exif/XMP metadata.
- [x] Port the public decoder API from `decode.cc` to Rust, including incremental
  input, events, callbacks and progressive output.

### Remaining encoder work

The [encoder expansion plan](docs/ENCODER_EXPANSION.md) records the delivery
sequence and acceptance gates for this work, including the encoder items
under shared runtime below.

- [ ] Complete the VarDCT encoder, including strategy selection, perceptual
  quantization, coefficient-order orchestration and progressive coefficient splitting.
- [ ] Complete general Modular image/frame encoding beyond the fixed Thunder profile.
- [ ] Support JPEG input and lossless JPEG recompression encoding.
- [ ] Port the public encoder API, queued frames, input callbacks and output
  processors.

### Shared runtime and full-port acceptance

- [ ] Complete the remaining core helpers and source boundaries listed in
  [the source mapping](docs/PORT_STATUS.md).
- [x] Read JPEG XL containers and their boxes.
- [ ] Write JPEG XL containers and their boxes.
- [x] Connect the translated default CMS backend (the skcms branch of
  `jxl_cms.cc`) and incremental ICC parsing to the public decoder.
- [x] Port the low-memory rendering pipeline and select it as `decode.cc` does.
- [x] Route the caller's memory manager through every decoder owner, including
  the ICC reader, codec state, image bundle and frame owners.
- [ ] Route the memory manager through the encoder's owners with the public
  encoder API.
- [x] Port the thread and resizable runners and `JxlDecoderSetParallelRunner`,
  with the decoder's parallel schedules. Its task bodies run serialized under
  a runner, without a parallel speedup yet.
- [ ] Port `JxlEncoderSetParallelRunner` with the encoder's parallel schedules.
- [ ] Run the decoder's parallel task bodies concurrently, with thread-safe
  owners and measured performance.
- [x] Export the public decoder C ABI of `decode.h`, including
  `JxlDecoderSetParallelRunner`, and the `JxlThreadParallelRunner*` and
  `JxlResizableParallelRunner*` C ABI of the ported runners.
- [ ] Export the public encoder C ABI.
- [ ] Translate the remaining original SIMD target specializations and dispatch,
  and accept the translated skcms SKX engine on an AVX-512 host.
- [ ] Pass the full upstream-derived test inventory and codec conformance corpus,
  including image samples, metadata, frame sequences, JPEG reconstruction,
  encoder bitstreams and public API transitions.
- [ ] Complete fuzzing, applicable sanitizer gates and the supported platform/CPU matrix.
- [ ] Measure performance and memory use and complete release validation.

Tools, examples, plugins and third-party library ports are separate boundaries
under the [porting contract](docs/PORTING.md). Redesign and additional
optimization follow the mechanical port.

## Build and verify

From `codecs/libjxl-rs`, with the source initialized as described in
[CONTRIBUTING.md](CONTRIBUTING.md):

```sh
cargo test -p libjxl-rs --locked --offline
python3 tools/check.py
```

The complete check builds the C++ oracle, runs Clippy, and compares encoded
bytes, decoded values, statuses, bit counts, truncated headers, and lifecycle
checkpoints in an optimized Rust build with overflow checks and debug
assertions. Missing oracle prerequisites are an error. All binary16 input
patterns are compared against the original code.

To link a C program against the decoder C ABI, `cargo build --release --lib`
writes `../../target/release/libjxl.a` and the dynamic library in the workspace
target directory. Compile against the
pinned headers in `upstream/libjxl/lib/include` and the export and version
headers that upstream's CMake configuration generates (the oracle build writes
them to `target/oracle/upstream/lib/include`). With the static library, define
`JXL_STATIC_DEFINE` and link the system libraries that
`cargo rustc --release --lib --crate-type staticlib -- --print native-static-libs`
lists.

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
- [Conformance](docs/CONFORMANCE.md): the external JPEG XL conformance corpus run.
- [Living ADRs](docs/adr/README.md): current decisions and rationale.
- [Provenance](docs/PROVENANCE.md): source and test origins.
- [Contributing](CONTRIBUTING.md): prerequisites, checks, and commit conventions.

Derived source retains the upstream [BSD-3-Clause license](LICENSE) and
[additional IP rights grant](PATENTS). The C++ reference and Highway dependency
remain separate Git submodules with their own notices.
The translated Highway prefetch helper retains its
[BSD license notice](licenses/HIGHWAY-BSD-3-Clause.txt).
