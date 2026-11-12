# ADR-0039: JPEG XL output through the workspace encoder

- **Status:** Experimental
- **Date:** 2026-11-12
- **Last reviewed:** 2026-11-12
- **Supersedes:** None; settles the separate JPEG XL output gate left open by [0018](0018-optional-codec-and-cms-dependencies.md) and the [M1 plan](../milestones/M1-complete-image-pipeline.md)
- **Related:** [0005](0005-codec-capabilities-and-pixel-contracts.md), [0010](0010-dual-licensing.md), [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0019](0019-source-side-orientation.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0030](0030-still-webp-output-and-format-neutral-encoder-result.md), [0032](0032-jpeg-xl-workspace-decoder-input.md), [0037](0037-selectable-workspace-avif-encoder.md), libjxl-rs [encoder plan](../../codecs/libjxl-rs/docs/ENCODER_EXPANSION.md) and [first image path](../../codecs/libjxl-rs/docs/FIRST_IMAGE.md), issues [#27](https://github.com/sebastian-software/ferrisma/issues/27), [#92](https://github.com/sebastian-software/ferrisma/issues/92), [#102](https://github.com/sebastian-software/ferrisma/issues/102), [#66](https://github.com/sebastian-software/ferrisma/issues/66)

## Context

Ferrisma v1.0 includes JPEG XL output. The engine is MIT OR Apache-2.0
(ADR-0010), so the encoder must come from a compatible source. Three routes
exist:

- `jxl-encoder` is pure Rust but AGPL-3.0 or commercial. The license gate of
  ADR-0018 blocks it, and this decision keeps it excluded.
- The C++ reference libjxl (BSD-3-Clause) through FFI would add a native
  toolchain, an `unsafe` boundary and C++ to the runtime graph, which
  ADR-0025's own-port direction avoids.
- The workspace port libjxl-rs (BSD-3-Clause AND MIT AND Apache-2.0 WITH
  LLVM-exception) translates the pinned libjxl mechanically and compares it
  with a C++ oracle.

Verified state of the port's encoder on 2026-11-12 (this worktree's base,
`90b638dd`):

- The public encoder API (`jxl::jxl::encode`, translating `encode.cc`) admits
  **no** frame encode. AddImageFrame returns an admission result where the
  original selects `enc_fast_lossless.cc`, and ProcessOutput returns one where
  it would call ProcessOneEnqueuedInput. Lossy VarDCT (steps 1 and 2, issues
  #118 and #117) and alpha in VarDCT frames (step 4, #115) are in progress.
  The heuristics and transforms below VarDCT are ported with checkpoint
  parity, but no complete lossy image path exists yet.
- One complete image path exists and is oracle-verified:
  `LosslessThunderConfig::encode_pixels` in `enc_frame.rs`. It is libjxl's own
  configuration SetLossless, effort 2 (kThunder), codestream level 10
  parameters, buffering 0 (one-shot), one regular frame, raw codestream,
  enum sRGB metadata, 8- or 16-bit integer gray or RGB and optional
  unassociated alpha at the color depth. The [first image
  record](../../codecs/libjxl-rs/docs/FIRST_IMAGE.md) lists 140 color and 400
  alpha encoder requests that are byte-identical with the pinned C++ encoder,
  plus cross-decoding in both directions. Its entry point is safe Rust.

The engine hands every built-in sink straight-alpha sRGB8 RGBA rows in order
(ADR-0024, ADR-0030), after applying source orientation (ADR-0019).

## Decision

Add opt-in JPEG XL output now, restricted to the encoder profile the port
actually admits, and widen it only as the port's steps land.

- **Feature and dependency.** The engine feature `jpeg-xl-write` depends
  directly on `libjxl-rs`. The entry point needs no `unsafe`, so no separate
  adapter crate is added; the engine keeps `forbid(unsafe_code)`. The feature
  is not part of `cli` until the license review below is done and a lossy
  profile exists. The CLI selects the output by the `.jxl` extension when
  built with the feature; otherwise it returns `FeatureDisabled`.
- **Facade.** `OutputFormat::JpegXl` and `TransformOptions::jpeg_xl`
  (`JpegXlOptions::Lossless { effort: 2 }`, the default and the only admitted
  value; the enum is `non_exhaustive` for later lossy settings).
  `EncodeRequest` carries the same field to external backends. The report
  names the encoder `libjxl-rs`.
- **Admitted profile and rejected requests.** Everything else fails with a
  typed error before any pixel is decoded and never falls back:
  - another effort or any lossy request: `InvalidOptions` (AVIF `quality`,
    `speed` or `chroma_subsampling` other than their defaults, and WebP
    options, are rejected as mismatched settings);
  - `encoder_threads` other than 1: `InvalidOptions` (the port has no
    parallel runner until step 13, #106);
  - output above the level 5 limits, 2^18 pixels per side or 2^28 pixels:
    `InvalidOptions`. The raw codestream carries no `jxll` box, so a decoder
    assumes level 5; the encoder's level 10 parameters are not signaled.
  - a managed budget too small for the frames below: `ResourceExhausted`;
  - an encoded result above `max_output_bytes`: `ResourceExhausted`, checked
    after encoding because the codec returns the whole codestream.

### Output semantics

These describe the emitted samples (ADR-0015):

| Property | Emitted value |
| --- | --- |
| Container | Raw codestream (`FF 0A`); no ISOBMFF container, no `jxll`, no Exif/XMP/JUMBF boxes |
| Coding | Lossless Modular, effort 2; decoded samples equal the engine's finished rows exactly |
| Bit depth | 8-bit integer samples (`bits_per_sample = 8`, no exponent bits) |
| Color | `xyb_encoded = false`; enum color encoding RGB, D65 white point, sRGB primaries, sRGB transfer, relative intent; no ICC profile. This is what the engine's sRGB8 quantization produces. Gray content is still written as RGB. |
| Alpha | If any finished alpha sample is below 255: one 8-bit, unassociated (straight) alpha extra channel; color under zero alpha is kept exactly. Fully opaque output has no extra channel. |
| Orientation | 1; the engine has already applied source orientation, so samples are upright |
| Other metadata | Default intensity target (255), no animation, no preview, no intrinsic size |
| Not supported | Lossy coding, other efforts, 16-bit or float output, HDR, wide gamut, ICC output, gray-only output, premultiplied alpha, animation, metadata passthrough, container and level signaling |

### Memory

The sink allocates one RGBA8 frame (4 bytes per pixel) from the managed
budget when it is created. Before encoding it compacts an opaque frame to
RGB8 in place and reserves the codec planes that the admitted one-shot path
allocates at the same time: an `Image3<f32>` color frame padded to whole 8x8
blocks, one `f32` alpha plane, and one `i32` Modular channel per color and
alpha plane. That is about 28 bytes per pixel opaque and 36 with alpha,
including the input frame. The count is a lower bound taken from the source
path. Row padding, Modular transforms, tree learning, tokens, section
writers and the encoded output (held twice briefly, as the codec's buffer and
the returned `Vec`) remain opaque, so the report sets
`opaque_encoder_memory`. The encoded file is outside the managed budget, as
for the other formats.

### License gate

Selecting the port does not complete license compliance. Before this output
is enabled in a distributed binary, the review in #66 must cover: the libjxl
BSD-3-Clause notice for every translated encoder unit reached by this profile
(`encode`, `enc_frame`, `enc_modular`, `enc_ma`, `enc_encoding`,
`enc_palette`, `enc_rct`, `enc_squeeze`, `enc_transform`, `enc_ans`,
`enc_cluster`, `enc_context_map`, `enc_huffman`, `enc_lz77`, `enc_bit_writer`,
`enc_fields`, `enc_toc`, `enc_external_image`, `enc_quant_weights` and their
shared headers); the libjxl `PATENTS` grant, whose scope covers "this
implementation of JPEG XL" and may not extend to a translation without
review; and the Highway, Brotli, skcms and LLVM/libc++ notices in
`codecs/libjxl-rs/licenses` that the crate's license expression already
names. `scripts/audit-licenses.py` lists `libjxl-rs` as metadata-OK; that is
not a notice review.

## Consequences

Callers can produce JPEG XL today with exact pixels, explicit color and a
budgeted frame. Files are large because coding is lossless (3.2 MB for the
2268x1512 test photograph below); no lossy size comparison exists yet, and
this output is not presented as a size-efficient delivery format. The memory cost per pixel is high, and the serial encoder
uses one core per image. `TransformOptions` and `EncodeRequest` gain a public
field and `OutputFormat` a variant: exhaustive struct literals and matches in
callers must be updated (experimental API, as in ADR-0037). The facade must
grow a lossy setting later; the `non_exhaustive` options enum leaves room for
it without changing the lossless meaning.

## Alternatives considered

- **`jxl-encoder` (AGPL-3.0/commercial):** rejected by the license gate.
- **libjxl through FFI:** rejected for now; native toolchain, `unsafe`, and
  C++ in the runtime graph, while the own port already produces
  byte-identical output for this profile.
- **Wait for the public encoder API (steps 1, 2 and 4):** rejected as the
  only option. It would leave v1.0 without any JPEG XL output although a
  verified lossless path exists. The facade switches to the public API when
  it admits the lossy profile.
- **Write the 16-bit profile:** not useful; the engine's finished rows are
  8-bit.
- **A separate adapter crate, as for the decoder (ADR-0032):** unnecessary
  because the entry point is safe; revisit if the public API's raw-pointer
  functions are used.

## Validation

Evidence on macOS ARM64, 2026-11-12, debug profile:

- `src/jxl_output_tests.rs` (feature `jpeg-xl-write`): profile rejections
  before allocation (effort, threads, chroma, level 5 side and area limits);
  exact budget accounting (frame, frame plus codec planes, failure one byte
  short, live memory back to zero); row order and completeness; facade
  setting mismatches rejected before the input is opened.
- With `jpeg-xl-read`: sink output at 1x1, 9x17, 37x19 and 300x260 (2x2
  groups), opaque and with alpha including zero-alpha pixels, decoded by
  jxl-oxide with checked signaling (no XYB, orientation 1, 8-bit integer,
  enum RGB/D65/sRGB/sRGB/relative, unassociated 8-bit alpha) and **zero**
  sample difference. With `jpeg-xl-own-read`, the workspace decoder recovers
  the same samples (float output rounded to 8 bits, at most 1e-3 from an
  integer step).
- With `webp-read`: WebP to JPEG XL transcodes (downscale and same size,
  opaque and alpha) decode to exactly the rows the pipeline finished; the
  output byte limit holds at the exact byte. With `jpeg-read`: an EXIF
  orientation 6 JPEG is written upright at 16x24 with orientation 1, equal to
  the finished rows and to the independent libjpeg reference.
- CLI (`--features cli,jpeg-xl-write`): the 2268x1512 photograph
  `codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg`
  encodes at full size (3,207,665 bytes; reported encoder materialization
  96,090,624 bytes) and as a 600x400 thumbnail (371,465 bytes). The thumbnail
  read back through the engine (jxl-oxide, `--use-rendered-icc`) and written
  as lossless WebP matches the direct JPEG to lossless WebP result with zero
  differing pixels in ImageMagick's comparison.
- `cargo +1.98 check -p ferrisma --all-targets --features
  jpeg-xl-write,jpeg-xl-read,webp-read,jpeg-read` succeeds: the feature needs
  neither the AVIF nightly nor native code.
- Byte parity with libjxl for this profile is the port's evidence, not
  re-run here. The engine builds the port without its local no-vectorize
  flags; the exact round trips above show this does not change pixels.
- The contraction inventory of #102 (`codecs/libjxl-rs/tools/contraction_inventory.py`,
  [first report](../../codecs/libjxl-rs/docs/ENCODER_CONTRACTION_INVENTORY.md))
  shows the nine recorded maps match the oracle front end on AArch64 and
  lists ported units with unrecorded `fmuladd` sites. The admitted lossless
  path converts integers exactly; the open maps matter for the lossy steps.

Not verified: x86-64, a C++ decoder (`djxl`) on engine output, the fuzz lanes
(#65), and any speed or peak-RSS figure. No performance claim is made.

## Review when

Review when the public encoder API admits lossy VarDCT (#118, #117) or alpha
in VarDCT (#115); when the license review (#66) completes, to decide `cli`
inclusion; if a non-sRGB or HDR output contract is needed (step 6, #113);
if serial encoding is too slow for the supported path (step 13, #106); or if
a decoder rejects the unsignaled level or a byte mismatch against libjxl
appears for this profile.

## Amendment log

- 2026-11-12 — Initial experimental decision: opt-in lossless effort-2 output
  through the port's verified fixed profile; `jxl-encoder` stays excluded.
