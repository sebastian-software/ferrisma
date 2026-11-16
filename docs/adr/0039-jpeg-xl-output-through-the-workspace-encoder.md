# ADR-0039: JPEG XL output through the workspace encoder

- **Status:** Experimental
- **Date:** 2026-11-12
- **Last reviewed:** 2026-11-16
- **Supersedes:** None; settles the separate JPEG XL output gate left open by [0018](0018-optional-codec-and-cms-dependencies.md) and the [M1 plan](../milestones/M1-complete-image-pipeline.md)
- **Related:** [0005](0005-codec-capabilities-and-pixel-contracts.md), [0010](0010-dual-licensing.md), [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [0019](0019-source-side-orientation.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0030](0030-still-webp-output-and-format-neutral-encoder-result.md), [0032](0032-jpeg-xl-workspace-decoder-input.md), [0037](0037-selectable-workspace-avif-encoder.md), libjxl-rs [encoder plan](../../codecs/libjxl-rs/docs/ENCODER_EXPANSION.md) and [first image path](../../codecs/libjxl-rs/docs/FIRST_IMAGE.md), issues [#27](https://github.com/sebastian-software/ferrisma/issues/27), [#249](https://github.com/sebastian-software/ferrisma/issues/249), [#238](https://github.com/sebastian-software/ferrisma/issues/238), [#92](https://github.com/sebastian-software/ferrisma/issues/92), [#102](https://github.com/sebastian-software/ferrisma/issues/102), [#66](https://github.com/sebastian-software/ferrisma/issues/66)

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
    lossy stops copying out of the encoder at the limit.

  Since the 2026-11-12 alpha and patches amendment no content-dependent
  rejection remains: lossy output with alpha encodes, and lossy output always
  runs with libjxl's patch search off. (Before it, transparent input failed
  with "lossy JPEG XL with alpha is not supported yet; use lossless" and
  text-like content with "this content needs the JPEG XL encoder's patch
  path"; both errors are removed.)

### Output semantics

These describe the emitted samples (ADR-0015):

| Property | Lossy (default) | Lossless |
| --- | --- | --- |
| Container | Raw codestream (`FF 0A`); no ISOBMFF container, no `jxll`, no Exif/XMP/JUMBF boxes | Same |
| Coding | VarDCT, effort 7, the requested Butteraugli distance, **patches off** (`JXL_ENC_FRAME_SETTING_PATCHES` = 0; the result equals `cjxl --patches=0 -e 7 -d <distance> -x color_space=RGB_D65_SRG_Rel_SRG` from the finished rows as PPM/PAM, see the 2026-11-15 amendment); default settings otherwise (dots, noise, splines off or absent as admitted) | Modular, effort 2, default buffering (chunked above 8 groups, as the original's default); decoded samples equal the engine's finished rows exactly; the file equals `cjxl -d 0 -e 2 -x color_space=RGB_D65_SRG_Rel_SRG` from the finished rows (2026-11-15 amendments; until the chunked amendment `--buffering=0`, the same file at most 8 groups) |
| Bit depth | 8-bit integer samples signaled (`bits_per_sample = 8`) | Same |
| Color | `xyb_encoded = true`; the signaled enum color encoding (RGB, D65, sRGB primaries, sRGB transfer, relative intent) describes the decoded samples; no ICC | `xyb_encoded = false`; same enum encoding; no ICC |
| Alpha | If any finished alpha sample is below 255: one 8-bit, unassociated (straight) alpha extra channel, coded losslessly (extra-channel distance 0), so decoded alpha equals the finished rows exactly. Color under zero alpha is **not** kept: libjxl's lossy default (`keep_invisible` off, SimplifyInvisible) replaces it. Fully opaque output has no extra channel; the engine omits it before encoding and does not rely on the encoder's `STRIP_ALPHA` decision. | If any finished alpha sample is below 255: one 8-bit, unassociated (straight) alpha extra channel; color under zero alpha is kept exactly. Fully opaque output has no extra channel. |
| Orientation | 1; the engine has already applied source orientation, so samples are upright | Same |
| Other metadata | Default intensity target (255), no animation, no preview, no intrinsic size | Same |
| Determinism | Per target and engine kernel level, not across targets (2026-11-15 amendment): the same input, options and target give the same bytes on every host and thread count; AArch64 and x86-64 may differ, and so may two kernel levels of one target when the output is resized | Same rule; the lossless coding itself is integer and was identical on both targets in every comparison so far |

Both write gray content as RGB. Not supported: other efforts, alpha deeper
than 8 bits (the engine's rows are 8-bit; the port needs a container above
12 bits), lossily coded alpha (an alpha distance above 0), 16-bit or float output, HDR, wide gamut, ICC output,
gray-only output, premultiplied alpha (the engine's rows are straight,
ADR-0014; the output always signals unassociated alpha), animation,
metadata passthrough, container and level signaling, patch encoding, and
distances of 3.0 and above.

### Memory

The sink allocates one RGBA8 frame (4 bytes per pixel) from the managed
budget when it is created. Before encoding it compacts an opaque frame to
RGB8 in place.

**Since 2026-11-16 (amendment of that date), both profiles reserve a
measured peak model** (`PeakModel` in `src/jxl_sink.rs`,
[record](../benchmarks/v1.0-jxl-encoder-memory-2026-11-16.md)); the
paragraphs below are the history it replaces.

Lossy (until 2026-11-16): it reserved `ferrisma_jxl_encoder::estimated_codec_bytes`, an
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
28,365,424 bytes, 8.3 per pixel). With alpha (amended 2026-11-12) the
estimate adds the alpha byte of the input copy and 12 bytes per pixel of
the encoded area, from the buffers the port allocates on that path: the
`f32` extra-channel plane of the VarDCT ComputeEncodingData, the Modular
encoder's global `i32` channel (`compute_encoding_data_extra_channels`) and
its `i32` per-group copies (`prepare_stream_params`). The encoded area is the
whole frame one-shot (13 more bytes per pixel) and one 272x272 bordered
group streaming (one more byte per pixel plus 887,808 bytes). Tokens, histograms, heuristics state,
section writers and the encoded output (held by the encoder and copied once
into the returned `Vec`) are opaque; the report sets
`opaque_encoder_memory`.

Lossless (amended 2026-11-15, #223, superseding the next paragraph): it
encodes through the bridge's `encode_lossless` with the default buffering
and reserves the **measured** peak of the path the encoder takes
([chunked memory record](../benchmarks/v1.0-jxl-lossless-chunked-memory-2026-11-15.md)):
at most 8 groups the one-shot path, 76 B/px opaque and 97 B/px with alpha;
above, the chunked path, 16 B/px of the frame (20 with alpha) plus 53 B/px
(70) of `min(frame, one 2048x2048 DC group)`; plus 12 MiB on both paths;
minus the input the measured process held (3 or 4 B/px), so that the 4 B/px
frame plus the reservation cover every measured peak. The alpha terms are
measured, not scaled. On the one-shot path the source-path planes remain a
floor. The report keeps `opaque_encoder_memory`.

Lossless (amended 2026-11-13, #223): it reserves the **measured** peak of
the admitted one-shot profile, `LOSSLESS_ONE_SHOT_PEAK_BYTES_PER_RGB_PIXEL`
= 76 bytes per pixel including the input frame (the port's 75.8 B/px RSS
at 3.4 MP, rounded up; the engine CLI measured 71-72 B/px at 24 MP; see the
[performance analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md#lossless-encode-28-bpx-reserved-about-70-bpx-used)).
The codec reservation is that peak minus the 4-byte input frame, scaled by
4/3 with alpha (four samples instead of three; derived, **not measured**),
and never less than the planes known from the source path, which remain the
floor for small images: an `Image3<f32>` color frame padded to whole 8x8
blocks, one `f32` alpha plane, and one `i32` Modular channel per color and
alpha plane. Until 2026-11-13 that floor was the whole reservation, about
28 bytes per pixel opaque and 36 with alpha including the input frame,
which understated the measured peak by about 1 GB at 24 MP. The codec's
allocations are still not individually tracked, so the report keeps
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
default getting exact pixels must now select `Lossless { effort: 2 }`. Since
the alpha and patches amendment the lossy default encodes transparent images
(alpha exact, color under zero alpha replaced) and text-like content (patches
off, at an unmeasured size cost against libjxl's default).

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

Not verified: x86-64, a C++ decoder (`djxl`) on engine output, the hosted
fuzz lanes (#65; the `transcode_jxl_out` lane and a bounded local ASan run of
it, 2026-11-15, are in [fuzzing](../fuzzing.md#jpeg-xl-output-lane)), and any
speed or peak-RSS figure. No performance claim is made.

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

Alpha and patches-off evidence, macOS ARM64, 2026-11-12, debug profile:

- Port (not re-run here): `PORT_STATUS.md`, "Lossy VarDCT with extra
  channels and patches off": whole public-API histories byte-identical with
  pinned libjxl for patches off (135 requests, contents 1 to 5 including
  text-like), 12 text-like histories (default: PatchDictionary admission;
  patches off: exact), RGBA and gray-alpha at small, large and streaming
  sizes, and the alpha variants (opaque stripped, premultiplied, depths).
- Bridge (`crates/ferrisma-jxl-encoder`, 13 unit tests): RGBA and
  gray+alpha at 1x1, 61x37, 300x260 and 2049x300 (streaming) decode with
  exact alpha and one unassociated 8-bit alpha channel through jxl-oxide, and
  the port records no admission result; text-like content with alpha
  encodes; the two repeated-glyph images still return `PatchDictionary` with
  patches on and encode with the default (off); a fully opaque alpha channel
  is stripped by the port and then gives the same bytes as the RGB encode;
  alpha buffer sizes are checked before the port; the alpha estimate.
- Engine (`src/jxl_lossy_tests.rs`): straight alpha decodes exactly through
  jxl-oxide (1x1, 9x17, 300x260, 800x700 streaming) and the workspace decoder
  (300x260, `alpha_premultiplied` false); color over pixels with nonzero
  alpha at d1.0 was 0 / 0, 17 / 3.75, 19 / 2.23 and 18 / 2.11 (max / mean),
  inside the opaque pins. WebP transcodes with alpha, default options: one
  transparent pixel 32x24 29 / 3.15, translucent 120x90 29 / 2.66, its
  64x48 downscale 41 / 2.62 (the highest: resampled straight alpha leaves
  low-alpha pixels whose straight color changes sharply), 800x700 18 / 2.11;
  pinned with the same margins. Text-like content through the sink and
  `transcode`: 64x64 (six glyphs, the bridge's patch case) 15 / 0.33 in 616
  bytes, a 300x120 page of 200 glyphs 20 / 1.19 in 13,719 bytes. The alpha
  reservation fails one byte short and succeeds at the exact estimate.
- CLI (`tests/jxl_cli.rs`, `--features cli`): transparent, text-like and
  page-like WebP input encodes to `.jxl` at the default options; the
  transparent file decodes with exact alpha through jxl-oxide, the opaque
  ones read back through `--jxl-decoder=jxl-oxide --use-rendered-icc`. The
  engine cannot read back its own transparent lossy output yet: the
  rendered-ICC stage accepts only opaque RGB, and without it the XYB samples
  have no verifiable color (`UnsupportedColor`); this is an input-side
  limitation, not an encoder one. (Amended 2026-11-13: the rendered-ICC stage
  now accepts straight alpha, so the transparent output reads back through
  `UseRenderedIcc` with either decoder; ADR-0015, #222. The strict policy
  still refuses it.)

Not measured: the size or quality cost of patches off against libjxl's
default for text-like content (the port cannot encode patches, and the C++
`cjxl` comparison was not run here), and the alpha estimate against a real
peak allocation.

## Review when

Review when the port translates the patch reference-frame encode (encoder
step 5), to measure patches on against off and revisit the patches-off
policy; when the license review (#66) completes, to confirm `cli`
distribution; when the owner decides the default or the patches-off policy;
if a measured peak allocation contradicts the lossy
estimate or the lossless reservation (in particular with alpha, whose 4/3
scaling is not measured); if a non-sRGB or HDR output contract is needed (step 6, #113);
if serial encoding is too slow for the supported path (step 13, #106); or if
a decoder rejects the unsignaled level or a byte mismatch against libjxl
appears for this profile; or if two hosts of one target at one kernel level
write different bytes (the determinism contract of the 2026-11-15
amendment), or the port's encoder starts selecting SIMD by default.

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
- 2026-11-12 — Lossy alpha and patches off (#27, #92, #115). The port now
  admits straight alpha (1 to 12 bits, coded losslessly by default) and the
  patches option at 0 in the lossy profile, byte-identical with pinned
  libjxl on macOS AArch64. The bridge adds `Alpha::Straight8` (RGBA and
  gray+alpha, 8-bit, unassociated) and `FrameOptions::patches` (default
  off); the sink writes real alpha instead of failing and keeps omitting
  fully opaque alpha itself. `LossyJpegXlAlpha` and `JpegXlPatchesRequired`
  are removed. Still rejected: alpha deeper than 12 bits and ICC (container
  and color steps 3 and 6; not reachable from the engine's 8-bit straight
  sRGB rows), distances of 3.0 and above, other efforts, threads other than
  1. **Patches policy, pending owner confirmation:** lossy output always
  passes patches = 0, so every image encodes deterministically and the
  result equals `cjxl --patches=0 -e 7 -d <distance>`. Rejected
  alternatives: libjxl's default with an error for text-like content (the
  previous behavior: screenshots fail by default), and trying the default
  first and retrying with patches off (a hidden second encode and an output
  mode that depends on the content). Cost: for text-like content libjxl's
  default would code repeated shapes once as patches; the size difference is
  **unmeasured**. Re-evaluate when the patch reference-frame encode lands in
  the port. Color under fully transparent pixels is replaced in lossy mode
  (libjxl's `keep_invisible` default); lossless keeps it.
- 2026-11-13 — The engine reads its own transparent lossy output back
  through `UseRenderedIcc` with either decoder (#222); evidence and the
  open strict-policy question are in ADR-0015 (2026-11-13).
- 2026-11-13 — Lossless reservation is the measured one-shot peak (#223,
  ADR-0004). **Deliberate contract change:** the sink reserved about 28 B/px
  (frame plus the planes known from the source path) while the admitted
  one-shot profile was measured at 70-76 B/px (24 MP: 1.71 GB RSS against
  672 MB reserved). It now reserves 76 B/px including the frame
  (`LOSSLESS_ONE_SHOT_PEAK_BYTES_PER_RGB_PIXEL`, from the cited
  measurement; no new measurement was taken), 4/3 of that with alpha
  (derived), with the source-path planes as a floor. Consequence: lossless
  output now fails with `BudgetExceeded` where it used to start and then
  exceed the budget in opaque codec memory; under the default 256 MiB
  budget the largest lossless output is about 3.5 MP instead of about
  9.5 MP. `jxl_output_tests::input_frame_and_measured_one_shot_peak_are_reserved`
  pins the new contract and checks that the reservation covers the cited
  24 MP measurement. Open: admitting libjxl's chunked buffering for
  lossless (about 27 B/px in C++, but different output bytes, so it needs
  oracle evidence), a measured peak with alpha, and the lossy estimate
  (`estimated_codec_bytes`, derived from the source, not measured).
- 2026-11-15 — **Byte reference is the original's public tool (#249).**
  The port's lossless codestream was 51 bytes smaller than `cjxl -d 0 -e 2
  --codestream_level=10 --buffering=0` for the same PPM. Decoded, the two
  differ in exactly two places and nowhere in the frame: (1) an explicit
  `--codestream_level=10` makes libjxl write the ISOBMFF container
  (`MustUseContainer`: signature 12 bytes, `ftyp` 20, `jxll` 9, `jxlc`
  header 8, together 49) around an otherwise identical codestream; the
  level-10 encoder parameters are the same at the automatic level, because
  `cparams.level` only bounds the Modular bit depth (16 at level 5, 32 at
  level 10), which 8-bit samples never reach; (2) cjxl's PPM/PAM reader
  leaves the rendering intent at zero (perceptual, `ApplyColorHints` in
  `lib/extras/dec/color_hints.cc`), while the port's fixed profile, like
  libjxl's `ColorEncoding::SRGB()` and the public `JxlColorEncodingSetToSRGB`,
  signals relative; the non-default intent costs 2 header bytes. The port's
  bytes are therefore the original's own output for `cjxl -d 0 -e 2
  --buffering=0 -x color_space=RGB_D65_SRG_Rel_SRG` (bare codestream), and
  for the public API with `JxlColorEncodingSetToSRGB`, SetLossless, effort 2,
  buffering 0 and no container; the sink does not change. Verified with
  `cjxl v0.13.0 a16d32d0` (the scalar oracle configuration of
  `scripts/bench/ci-perf/codec_jxl.sh`) on macOS AArch64: byte-identical
  with the port at 37x19 RGB and RGBA, 300x260 and 4032x3024, with
  `--num_threads=0` and with the default pool. The same flag explains the
  lossy profile: the engine's lossy output for a 600x400 thumbnail of the
  flower photograph equals `cjxl --patches=0 -e 7 -d 1 -x
  color_space=RGB_D65_SRG_Rel_SRG` on the finished rows, and is 3 bytes
  smaller than the same command without the color hint (the codec-level
  gate's `encode_lossy` example deliberately reproduces cjxl's PPM call
  sequence, perceptual intent included). The output-semantics table now
  names these commands. `jxl_output_tests::lossless_output_equals_the_original_cjxl_bytes`
  compares the sink's bytes with three files written by that cjxl command
  (`tests/fixtures/jxl-lossless/`, with the generator of their input);
  no existing pin changed. The CI performance record's lossless row compared
  a container with level box and perceptual intent against the raw
  codestream: the difference was signaling, not coding.
- 2026-11-15 — **Determinism is per target and kernel level (#238).** The
  engine's default lossy output for the same JPEG differed by a byte or two
  between AArch64 and x86-64 (CI: 12 MP 2,070,600 against 2,070,601 bytes;
  2048 px wide 539,604 against 539,603 on AVX2). Reproduced locally on
  macOS AArch64 and x86-64 under Rosetta 2 (`ROSETTA_ADVERTISE_AVX=1`) with
  the CI input rebuilt from its recipe (`photo12`: the libavif dog
  photograph decoded by libjpeg-turbo `djpeg`, re-encoded by `cjpeg
  -quality 90`; here the pinned libjpeg-turbo submodule's scalar build), the
  same byte counts on both targets. The RGBA rows each sink receives were
  dumped through an `EncoderBackend` and fed to the pinned original `cjxl
  --patches=0 -e 7 -d 1 -x color_space=RGB_D65_SRG_Rel_SRG`, built as the
  scalar oracle configuration natively and as an x86-64 binary
  (`CMAKE_OSX_ARCHITECTURES=x86_64`). Two separate sources, each attributed:
  - **No resize: the encoder's target, not the engine path.** JPEG decode,
    linearize and quantize give byte-identical rows on both targets and at
    every kernel level (equal to the decoder's own output; 4032x3024 and
    the 2268x1512 flower photograph). The bytes still differ, and the
    original differs the same way: AArch64 `cjxl` writes the AArch64 engine
    file, x86-64 `cjxl` the x86-64 engine file, from the same rows. The
    port's encoder runs its scalar translation on both targets
    (`ENCODER_SELECTS_SIMD = false`), so this is the original's own
    target-specific arithmetic in the scalar build, which ADR-0041 section 2
    keeps ("identity across targets is not required"); the likely mechanism
    is Clang's default contraction on AArch64 (FMA is baseline there, not on
    x86-64), not attributed to a site here. It is content-dependent: the
    codec gate's master PPM and the 2048 px rows below encode identically on
    both targets.
  - **Resize: the engine's f32 resample kernel level.** With the default
    ceiling, x86-64 runs `Avx2` and its 2048x1536 Lanczos3 rows differ from
    the `Scalar` rows in 19 of 12,582,912 samples, each by one code (600x400
    flower thumbnail: 4 of 960,000); with `kernel_level_ceiling =
    Scalar` the x86-64 rows equal the AArch64 rows (`Neon` and `Scalar`
    agreed on both inputs, an observation, not a guarantee). This is the
    bound ADR-0021 documents ("`f32` SIMD output can differ between ISAs by
    one code at ties"). The encoder then writes the same bytes from the
    same rows on both targets, and each original target writes the engine's
    bytes from that target's rows.
  No defect was found: on each target the engine's file equals the
  original's output for the rows the sink received, the rows stay within
  ADR-0021, and the transform's arithmetic is unchanged. **Contract:** the
  lossy bytes are deterministic per target and engine kernel level, across
  hosts, CPUs and transform or encoder thread counts of that target (the CI
  record); they are not promised across targets or kernel levels. Callers
  that need identical files on both targets need the same kernel level
  (`kernel_level_ceiling`) and still can get different bytes from the
  encoder; pixel equality to a tolerance is the cross-target contract, as
  for AVIF (#203). Lossless uses the same rule; its integer coding was
  identical on both targets in this comparison and in `jxl_output_tests`
  (the cjxl reference files above pass under Rosetta), but resized rows
  depend on the kernel level as for lossy.
- 2026-11-15 — Options note (ADR-0043 D4): the AVIF settings that a JPEG
  XL output refuses as mismatched are now `AvifOptions` (formerly
  `quality`, `speed`, `chroma_subsampling`), and the CLI flags rejected
  with `.jxl` output are `--avif-*` and `--webp-*` ("… requires .avif
  output"); `--quality=`, `--speed=` and `--chroma=` are unknown options.
  `JpegXlOptions` is unchanged.
- 2026-11-15 — Owner decision on the lossy default and the patches policy
  (#236): **lossy distance 1.0 by default, patches off; owner decision
  2026-11-15.** This confirms the 2026-11-12 "pending owner confirmation"
  items for the JPEG XL default and for the patches policy. The cost of
  patches off was measured against the pinned original `cjxl -e 7` on a
  generated text-like corpus with four photographs as control
  ([record](../benchmarks/v1.0-jxl-patches-cost-2026-11-15.md)): frames above
  8 groups of 256x256 pixels encode identically with and without patches
  (libjxl's default streams them and skips the patch search), while frames of
  at most 8 groups were 1.10x to 3.80x larger with patches off (median 2.18x
  at d1.0) and scored 0.33 to 3.89 SSIMULACRA2 points lower; photographs are
  unchanged. The "unmeasured" cost above is superseded by that record, and
  the review trigger (encoder step 5) stands.
- 2026-11-15 — **Lossless output is chunked like the original's default
  (#223). Owner decision 2026-11-15: the default budget stays at 256 MiB, and
  the chunked lossless encode is ported.** Path analysis: for the engine's
  profile (`cjxl -d 0 -e 2`, SetLossless at kThunder) the original's
  CanDoStreamingEncoding returns true at its default buffering (-1) once the
  frame has more than 8 groups of 256x256 (also at buffering 1 above 2048
  pixels per side, 2 and 3): Modular mode with `color_transform` kNone is the
  accepted transform, and as the modular part is lossless the extra-channel
  and `responsive` exclusions do not apply. EncodeFrame then runs
  EncodeFrameStreaming: one ComputeEncodingData per 2048x2048 DC group in
  streaming mode, whose ModularFrameEncoder::ComputeEncodingData runs
  groupwise (no global palette, the YCoCg RCT per DC group, stream
  parameters per group through PrepareStreamParams and
  ComputeStreamingAbsoluteAcGroupId); ComputeTree and ComputeTokens are
  skipped, EncodeGlobalInfo writes "no global tree", and every DC and AC
  group stream is coded by ModularGenericCompress with its own predefined
  kGradientFixedDC tree and histograms, cleared (ClearModularStreamData)
  before the next DC group; the Modular frame's AC-global section is empty.
  Encoder step 2 had ported the frame-level streaming (EncodeFrameStreaming,
  permutation, TOC, output modes, the output processor and chunked input)
  and the groupwise extra-channel path (`do_color = false`) for VarDCT; the
  lossless path needed beyond it the `do_color = true` ComputeEncodingData
  of the Modular frame encoder, the Modular branches of enc_frame's
  ComputeEncodingData, the empty AC-global section and the admission of the
  profile through the public API. All are now translated and byte-exact
  against the original public API in buffer and output-processor mode, with
  chunked input and the container, at sizes up to 4000x3000 (libjxl-rs
  `PORT_STATUS.md`, block "encoder lossless streaming"). **Bytes:** the
  chunked files differ from the one-shot files (different trees and
  histograms per group), so the reference is the original's default
  command, `cjxl -d 0 -e 2 -x color_space=RGB_D65_SRG_Rel_SRG`; at most 8
  groups both commands write the same file. `jxl_output_tests::lossless_output_equals_the_original_cjxl_bytes`
  now also compares two 2049x40 patterns (9 groups, two DC groups) with
  files the original wrote in its default mode
  (`tests/fixtures/jxl-lossless/README.md`); the three earlier fixtures are
  unchanged. **Engine:** the sink calls the bridge's new `encode_lossless`
  (effort 2, JxlEncoderSetFrameLossless, `uses_original_profile`, default
  buffering) instead of `LosslessThunderConfig::encode_pixels`; the output
  limit is still checked after the encoder returns. **Memory:** measured
  peak RSS at 24 MP opaque, port chunked 19.9 B/px against 74.1 one-shot
  (original 26.5 against 77.9), with alpha 28.9 against 96.9 (original 34.7
  against 100.5); below one DC group the chunked path still costs about
  52-66 B/px. The reservation follows the measured model above; frame plus
  reservation at 24 MP opaque are 643 MB instead of 1,824 MB. Under the
  default 256 MiB budget the largest lossless output moves from about
  3.53 MP to 3.66 MP opaque (2.84 MP with alpha), because images of a few
  megapixels are dominated by one DC group's working set;
  `supported_matrix::default_budget_rejects_lossless_jpeg_xl_above_about_3_7_megapixels`
  (1920x1920, renamed from `..._above_about_three_and_a_half_megapixels`
  with 2000x2000) and
  `jxl_output_tests::input_frame_and_measured_lossless_peaks_are_reserved`
  (every measured peak fits) pin it. Not measured: gray or 16-bit input
  (the engine writes neither), other hosts and allocators, and the lossy
  estimate.
- 2026-11-15 — Output depth follows the input (owner decision; ADR-0021
  amendment): both profiles write 8 to 16 integer bits, the input's by
  default (`JpegXlOptions` `depth`), from `uint16` rows with
  `JXL_BIT_DEPTH_FROM_CODESTREAM` and alpha at the color depth, as `cjxl`
  passes a PNM (`lib/extras/enc/jxl.cc:257`, `lib/extras/dec/pnm.cc:276`,
  `:300`, `:537`, `:649`). Checked in the port: `admitted_lossless_profile`
  admits integer samples up to 16 bits, the lossy profile any integer depth;
  above 12 bits of lossless color or of alpha libjxl requires level 10 and
  both write the container. Bytes equal `cjxl` for lossless and lossy at 10,
  12 and 16 bits with and without alpha
  (`jxl_output_tests::deep_output_equals_the_original_cjxl_bytes`, nine
  files in `tests/fixtures/jxl-deep/`); a 16-bit lossless round trip is
  exact. The lossless reservation for deep rows is measured
  (`docs/benchmarks/v1.0-jxl-deep-memory-2026-11-15.md`), not derived.
- 2026-11-16 — **Measured memory model for both profiles (ADR-0004,
  ADR-0042 D3).** Measured as the sink holds it (its frame plus the bridge
  call, one process at a time, macOS AArch64), the 2026-11-15 lossless model
  was up to about 10 % below 12.2 MP RGBA peaks (3 % for photographs, 10 %
  for noise) and below opaque noise at 6 and 12 MP and one-shot noise
  frames; the reported 50.1 B/px of 2026-11-15 was partly its tool, which
  held the input twice. The cause is resident memory the allocator keeps
  from the first DC group while a second group of another shape allocates
  its own (the live heap is far lower), and content that compresses worse
  than the photographs measured. The lossy estimate covered 7.6 to 68 B/px
  of 26 to 166 B/px measured: every lossy request was under-reserved, by
  56 to 92 %. The sink now reserves, for both profiles, `constant +
  one_shot * px` (at most 8 groups) or `constant + frame * px + dc_group *
  min(px, D) + retained * min(max(px - D, 0), D)` (D one DC group), per
  mode, alpha and depth, fitted 10 % above every peak on record
  (photographs, a 16-bit photograph, noise; distances 0.05, 1.0, 2.9), minus
  the frame. `jxl_output_tests::model_covers_every_measured_peak_with_margin`
  pins it against 103 recorded cases. Under the default 1 GiB the largest
  output is 49,133,275 px lossless opaque (was 49,344,752), 30,670,848 with
  alpha (38,377,881), 24,292,010 and 13,939,892 above 8 bits (28,272,715,
  20,185,088); lossy 40,544,938 opaque and 25,547,124 with alpha (about
  144 and 127 MP from the estimate), 22,282,240 and 11,240,734 above 8 bits
  (about 74 and 65 MP). The report still sets `opaque_encoder_memory`: the
  model is an envelope of one host's measurements, not a bound for other
  hosts or allocators.
