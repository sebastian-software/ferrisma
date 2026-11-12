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
  (Lossy VarDCT landed later the same day; see the 2026-11-12 lossy
  amendment.)
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
  directly on `libjxl-rs` for the lossless entry point, which needs no
  `unsafe`, and on the adapter crate `ferrisma-jxl-encoder` for lossy output
  (amended 2026-11-12, below); the engine keeps `forbid(unsafe_code)`. The CLI
  selects the output by the `.jxl` extension when built with the feature;
  otherwise it returns `FeatureDisabled`. Since the 2026-11-12 amendment the
  feature is part of `cli`; distributing a binary still needs the license
  review below.
- **Facade.** `OutputFormat::JpegXl` and `TransformOptions::jpeg_xl`
  (`JpegXlOptions`, `non_exhaustive`): `Lossy { distance }` and
  `Lossless { effort: 2 }`. The default is `Lossy { distance: 1.0 }`,
  **pending owner confirmation** (see the amendment). `EncodeRequest` carries
  the same field to external backends. The report names the encoder
  `libjxl-rs`.
- **Admitted profile and rejected requests.** Everything else fails with a
  typed error before any pixel is decoded and never falls back:
  - another lossless effort, or a lossy distance outside 0.05 to below 3.0
    (including NaN and infinities): `InvalidOptions` (AVIF `quality`,
    `speed` or `chroma_subsampling` other than their defaults, and WebP
    options, are rejected as mismatched settings);
  - `encoder_threads` other than 1: `InvalidOptions` (the port has no
    parallel runner until step 13, #106);
  - output above the level 5 limits, 2^18 pixels per side or 2^28 pixels:
    `InvalidOptions`. The raw codestream carries no `jxll` box, so a decoder
    assumes level 5; the encoder's level 10 parameters are not signaled.
  - a managed budget too small for the frames below: `ResourceExhausted`;
  - an encoded result above `max_output_bytes`: `ResourceExhausted`; lossless
    checks after encoding because the codec returns the whole codestream,
    lossy stops copying out of the encoder at the limit;
  - lossy output of an image with any alpha below 255: `UnsupportedInput`
    ("lossy JPEG XL with alpha is not supported yet; use lossless") at the
    first transparent row; alpha is never dropped silently and there is no
    fallback to lossless;
  - lossy content for which FindTextLikePatches finds patches:
    `UnsupportedInput` ("this content needs the JPEG XL encoder's patch
    path, which is not available yet"), after the decode, because the
    decision depends on the pixels.

### Output semantics

These describe the emitted samples (ADR-0015):

| Property | Lossy (default) | Lossless |
| --- | --- | --- |
| Container | Raw codestream (`FF 0A`); no ISOBMFF container, no `jxll`, no Exif/XMP/JUMBF boxes | Same |
| Coding | VarDCT, effort 7, the requested Butteraugli distance; default settings otherwise (patches, dots, noise, splines off or absent as admitted) | Modular, effort 2; decoded samples equal the engine's finished rows exactly |
| Bit depth | 8-bit integer samples signaled (`bits_per_sample = 8`) | Same |
| Color | `xyb_encoded = true`; the signaled enum color encoding (RGB, D65, sRGB primaries, sRGB transfer, relative intent) describes the decoded samples; no ICC | `xyb_encoded = false`; same enum encoding; no ICC |
| Alpha | None; transparent input is rejected, fully opaque alpha is omitted | If any finished alpha sample is below 255: one 8-bit, unassociated (straight) alpha extra channel; color under zero alpha is kept exactly. Fully opaque output has no extra channel. |
| Orientation | 1; the engine has already applied source orientation, so samples are upright | Same |
| Other metadata | Default intensity target (255), no animation, no preview, no intrinsic size | Same |

Both write gray content as RGB. Not supported: other efforts, lossy alpha,
16-bit or float output, HDR, wide gamut, ICC output, gray-only output,
premultiplied alpha, animation, metadata passthrough, container and level
signaling, lossy content that needs patches, and distances of 3.0 and
above.

### Memory

The sink allocates one RGBA8 frame (4 bytes per pixel) from the managed
budget when it is created. Before encoding it compacts an opaque frame to
RGB8 in place.

Lossy: it reserves `ferrisma_jxl_encoder::estimated_codec_bytes`, an
**estimate** derived from the port's source, not a measurement: the queued
input copy (3 bytes per pixel), and on the one-shot path (at most eight
256x256 groups) seven `f32` samples per block-padded pixel (the XYB color
frame, the pre-Gaborish `orig_opsin` copy and the 1x1 masking plane) plus
the `i32` AC coefficients of every group (786,432 bytes per group); on the
streaming path one group's planes with an 8-pixel border, one group of
coefficients and about 28 bytes per 8x8 block for the DC-resolution planes.
Including the input frame that is 35 bytes per pixel plus 12 per
group-padded pixel one-shot (600x400: 13,118,592 bytes, 54.7 per pixel) and
about 8 bytes per pixel on the streaming path at large sizes (2268x1512:
28,365,424 bytes, 8.3 per pixel). Tokens, histograms, heuristics state,
section writers and the encoded output (held by the encoder and copied once
into the returned `Vec`) are opaque; the report sets
`opaque_encoder_memory`.

Lossless: it reserves the codec planes that the admitted one-shot path
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
not a notice review. Since the lossy amendment the review must also cover the
public API (`encode`, `encode_internal`) and every unit of the VarDCT path
it reaches (`enc_frame_vardct`, `enc_heuristics_lossy`, `enc_heuristics_ar`,
`enc_adaptive_quantization`, `enc_ac_strategy`, `enc_chroma_from_luma`,
`enc_patch_dictionary`, `enc_xyb`, `enc_gaborish`, `enc_transforms`,
`enc_cache`, `enc_coeff_order`, `enc_entropy_coder`, `quantizer` and their
helpers), and the `ferrisma-jxl-encoder` bridge's own unsafe boundary
([unsafe review](../unsafe-review.md)). Including `jpeg-xl-write` in `cli`
does not complete this gate.

## Consequences

Callers can produce JPEG XL with explicit color and a budgeted frame: lossy
by default, and lossless with exact pixels. Lossless files are large (3.2 MB
for the 2268x1512 test photograph below, 371,465 bytes for its 600x400
thumbnail); the lossy default writes the same thumbnail in 73,020 bytes.
No size or quality comparison against other formats or encoders is made
here. The memory cost per pixel is high, and the serial encoder
uses one core per image. `TransformOptions` and `EncodeRequest` gain a public
field and `OutputFormat` a variant: exhaustive struct literals and matches in
callers must be updated (experimental API, as in ADR-0037). The lossy
variant made `JpegXlOptions` `PartialEq` only (its `f32` has no `Eq`), and
the default moved from lossless to lossy d1.0: a caller that relied on the
default getting exact pixels must now select `Lossless { effort: 2 }`. With
the lossy default, transparent images and text-like content fail by default
with a typed error that names the lossless alternative.

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
  for the safe lossless entry point; adopted on 2026-11-12 for the lossy
  path, which uses the public API's raw-pointer functions.

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

Lossy evidence, macOS ARM64, 2026-11-12, debug profile:

- Bridge (`crates/ferrisma-jxl-encoder`, 10 unit tests): XYB enum-sRGB
  signaling decoded by jxl-oxide; gray and linear sRGB; the frame copy (the
  caller's buffer changed after `add_frame` gives the same bytes) and drop
  at every state; bridge misuse rejected before the port; port
  `API_USAGE` (distance 25.5, a side above 2^30) typed and poisoning the
  encoder; distances 3.0 and 4.5 reported as `FrameProfile`; two repeated-glyph
  images from the port's patch parity corpus reported as `PatchDictionary`
  and a single-glyph image encoded; the output limit at the exact byte with
  the caller's vector restored; a 2049x300 (18-group) streaming encode. The
  admission classification is inferred (the public API reports every
  checkpoint as `NOT_SUPPORTED`); the tests compare it with the port's own
  `take_admissions` record, available through the observation-only
  `cpp-oracle` feature in the bridge's dev-dependencies.
- Engine (`src/jxl_lossy_tests.rs`): the distance range before allocation
  and before the input is opened; the alpha error at the first transparent
  row; the patch error through the sink and through `transcode`; exact
  budget accounting for the estimate (one byte short fails, live memory back
  to zero); the output limit at the exact byte; default options of AVIF and
  WebP still match. Decoded by jxl-oxide with checked signaling (XYB, enum
  sRGB, orientation 1, no extra channels), the d1.0 maximum / mean absolute
  RGB error against the rows the sink received was 0 / 0 at 1x1, 37 / 4.00
  at 9x17, 19 / 2.22 at 300x260, 18 / 2.09 at 800x700 (12 groups,
  streaming), 22 / 2.60 and 23 / 1.67 for WebP transcodes, 61 / 4.79 for the
  orientation-6 16x24 pattern, and 79 / 2.76 for the flower photograph as a
  600x400 thumbnail. The tests pin these with about 30 % margin on the
  maximum and 50 % on the mean; the pins are regression checks, not
  conformance evidence (byte parity with libjxl is the port's gate). The
  workspace decoder agrees with jxl-oxide within one level (mean 0.006).
- CLI (`--features cli,jpeg-xl-write`, debug): the flower photograph as a
  600x400 thumbnail writes 73,020 bytes at the default d1.0, 114,319 at
  `--jxl-distance=0.5` and 33,850 at 2.9 (371,465 with `--jxl-lossless`);
  at full 2268x1512 size (54 groups, streaming) 464,590 bytes. Read back
  through the engine (`--use-rendered-icc`, both JPEG XL readers) and written
  as lossless WebP, the thumbnail is 35.73 dB PSNR from the direct JPEG to
  lossless WebP result (ImageMagick). `--jxl-distance=3`, `--quality`,
  `--speed`, `--chroma` and `--jxl-lossless` with `--jxl-distance` are
  rejected.
- `cargo +1.98 check -p ferrisma -p ferrisma-kernels --features
  jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-own-read,jpeg-xl-write,avif-write,icc`
  and the same with `--all-targets` for the engine and the bridge succeed:
  the feature stays stable-compatible.

Not verified for lossy: x86-64 (the port's lossy parity is recorded on macOS
AArch64 only), a C++ decoder on engine output, the encoder's actual peak
allocation against the estimate, fuzzing of the encoder path, and any speed
figure.

## Review when

Review when the port admits alpha in VarDCT (#115) or a patches-disabled
or patch-encoding profile (step 5), to lift the two lossy rejections; when
the license review (#66) completes, to confirm `cli` distribution; when the
owner decides the default; if a measured peak allocation contradicts the lossy
estimate; if a non-sRGB or HDR output contract is needed (step 6, #113);
if serial encoding is too slow for the supported path (step 13, #106); or if
a decoder rejects the unsignaled level or a byte mismatch against libjxl
appears for this profile.

## Amendment log

- 2026-11-12 — Initial experimental decision: opt-in lossless effort-2 output
  through the port's verified fixed profile; `jxl-encoder` stays excluded.
- 2026-11-12 — Lossy output (#27, #92). The port's public encoder API now
  admits lossy VarDCT at effort 7 for distances 0.05 to below 3.0 (gray or
  RGB without alpha, enum sRGB or linear sRGB, one frame, raw codestream,
  serial, one-shot up to 8 groups and streaming above), byte-identical with
  pinned libjxl on macOS AArch64. A new safe adapter crate
  `ferrisma-jxl-encoder` wraps it (RAII encoder, typed errors separating
  admission results from misuse and failures, bounded output), and
  `JpegXlOptions::Lossy { distance }` uses it. Still rejected: lossy alpha
  (typed "use lossless" error, no silent drop or fallback), content that
  needs patches (typed error), distances of 3.0 and above, other efforts,
  threads other than 1, ICC/HDR/wide gamut and metadata. **Default decision,
  pending owner confirmation:** `JpegXlOptions::default()` is now lossy d1.0,
  libjxl's own default and consistent with the lossy defaults of AVIF and
  WebP; the cost is that transparent images and text-like content fail by
  default instead of encoding losslessly. `jpeg-xl-write` joins `cli`
  because the full feature set works end to end; the license gate for
  distributed binaries is unchanged.
