# Silent-drop audit (2026-11-15)

**Question.** What does a *successful* `transcode` drop, ignore or change
without an error and without telling the caller? The owner's principle,
stated on 2026-11-15: behavior is negotiable, but nothing may disappear
implicitly and no error may be hidden or swallowed. The engine's explicit,
typed refusals ([supported matrix](supported-matrix.md),
[compatibility guide](compatibility.md#refusal-reasons)) satisfy it; this
page lists what passes while something is lost.

The principle and the decisions derived from this audit are recorded in
[ADR-0042](adr/0042-no-silent-loss-refuse-or-report.md); this page is its
inventory.

**Base.** `origin/main` at `64284177`, branch `v1/engine-silent-drops`.
Read-only audit of `src/`, the wrapper crates under `crates/` and the
third-party decoders' relevant code (jxl-oxide 0.12.6, moxcms 0.9.1, the
translated libavif `read.c`), plus probes through the public API. Every
claim marked with a test is pinned in [`tests/silent_drops.rs`](../tests/silent_drops.rs)
(`silent_drops::…`) or in an existing test; those tests record today's
behavior and are **not** an endorsement. Where the repository has no
fixture, the row says so.

**Classes.**
(A) discarded or changed silently and undocumented;
(B) discarded silently, but documented somewhere;
(C) reported to the caller (an error, a report field, or CLI output);
(D) preserved (or the loss is the format's own meaning, such as
compositing JPEG XL layers into the displayed image).

| Class | Items | Inputs (J, W, X, V) | Outputs (O) | Pipeline (P) | Options (Q) |
| --- | --- | --- | --- | --- | --- |
| A | 25 → 6 → 3 | 20 → 3 | 2 → 1 → 0 | 1 → 0 | 2 → 0 |
| B | 29 → 13 → 12 → 9 → 8 | 12 → 3 | 8 → 4 → 3 → 2 → 1 | 6 → 3 | 3 → 1 |
| C | 21 → 55 → 50 → 54 → 50 | 15 → 41 → 37 → 35 | 0 → 5 | 2 → 5 → 3 | 4 → 3 → 7 |
| D | 11 → 16 | 8 → 10 | 3 → 4 | 0 → 2 | 0 |
| resolved by decision | 0 → 1 → 3 | 0 | 0 → 2 | 0 → 1 | 0 |
| resolved by removal | 0 → 6 | 0 → 4 | 0 → 1 | 0 | 0 → 1 |

The first number is the state of the audit (`main` at `64284177`), the
second the state after branch `v1/engine-discarded` (2026-11-15), a third
the state after branch `v1/remove-alt-backends` (2026-11-15, see
[removal of the alternative backends](#update-removal-of-the-alternative-backends-2026-11-15)), a
fourth the state after branch `v1/engine-options` (2026-11-15, see
[options and the command line](#update-options-and-the-command-line-2026-11-15)), a
fifth the state after branch `v1/engine-depth` (2026-11-15, owner decision:
the output depth follows the input, ADR-0021 amendment), which moves X17,
V14, O2, P1 and P6 to (D): the precision is preserved wherever the output
can hold it, and `PrecisionReduced` is reported only where it cannot: the
refusals of ADR-0042 D1 and D4 (V4, X6 to X9) and the report of D2,
`ProcessingReport::discarded` with its CLI key `discarded=`, moved 48
items to (C); each row says what reports or refuses it and which test of
`tests/discarded_report.rs` or `tests/silent_drops.rs` proves it. Still
open under ADR-0042 D6: the remaining (A) items (J10 XMP-only
orientation, X12 `intrinsic_size`, V8 the `sato` alternative, and the
P4, the rendering intent, was resolved by the owner's decision of
2026-11-15, and O8, Q5 and Q7, options without effect and repeated
flags, by the owner's decisions of the same day, which
`v1/engine-options` implements) and the (B)
items that are documented but not reported. Of these, out-of-gamut
clipping (P2, P3) cannot be detected cheaply and exactly: a value outside
[0, 1] at quantization may be resampling overshoot as well as gamut, and
moxcms clamps inside its LUT path without a signal; it is not reported on
a guess.

Two more input items (J16 and W7, corrupted JPEG scan data and a corrupted
WebP payload) are outside the classes: the codec cannot detect them. The
table of caught errors further down judges code sites and is not counted
again. "Probe" in a test column means a one-off run through `transcode`
during this audit that is not pinned by a test.

The one behavior change of this branch (Part 3 below) moves the unknown
essential AVIF property from a misreported `MalformedInput` to a correctly
classified refusal; it is counted as (C).

## Inputs

### JPEG (`jpeg-read`, libjpeg-turbo-rs)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| J1 | EXIF beyond orientation (camera, date, GPS, resolution, embedded thumbnail) | read for the orientation tag only, then dropped; no output writes EXIF. **Since `v1/engine-discarded`: reported: `Discarded::Exif`** | `src/codecs/jpeg.rs` `JpegSource::new`, `src/codecs/exif.rs` `orientation` | matrix, Outputs "Metadata" (AVIF, WebP) | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels`, `webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp`; `silent_drops::source_exif_and_xmp_are_dropped_from_jpeg_xl_output_today`; `discarded_report::jpeg_exif_is_reported_even_when_its_orientation_is_applied` | C |
| J2 | XMP, extended XMP | ignored, dropped. **Since `v1/engine-discarded`: reported: `Discarded::Xmp`** | not read anywhere | matrix, Outputs "Metadata" | same; `paris_extended_xmp.jpg` (probe); `discarded_report::jpeg_xmp_icc_and_other_metadata_are_reported` | C |
| J3 | IPTC (APP13 `Photoshop 3.0`) | ignored, dropped. **Since `v1/engine-discarded`: reported: `Discarded::OtherMetadata`** | not read | no | `silent_drops::jpeg_iptc_and_comments_are_dropped_today` (`seine_sdr_gainmap_srgb.jpg`); `discarded_report::jpeg_gain_maps_iptc_and_comments_are_reported` | C |
| J4 | `COM` comments | ignored, dropped. **Since `v1/engine-discarded`: reported: `Discarded::OtherMetadata`** | not read | no | same (`apple_gainmap_new.jpg`); `discarded_report::jpeg_gain_maps_iptc_and_comments_are_reported` | C |
| J5 | JUMBF / C2PA (APP11) | ignored, dropped; a content-provenance manifest disappears. **Since `v1/engine-discarded`: reported: `Discarded::OtherMetadata` (APP11), code path only, no fixture** | not read | no | no fixture in the repository | C |
| J6 | ICC profile | `UseRenderedIcc`: converted to sRGB, `color_signal = RenderedIcc`; otherwise refused (`RenderedIccRequiresOptIn`) | `src/cms.rs` `IccSrgbSource` | matrix, Color policy | `supported_matrix::jpeg_embedded_icc_needs_the_rendered_icc_policy`, `tests/icc_photo.rs` | C |
| J7 | MPF secondary images: Ultra HDR and Apple gain maps | the SDR base image is read; the gain map and its `hdrgm` XMP are ignored; no signal that HDR content existed. **Since `v1/engine-discarded`: reported: `Discarded::GainMap`** | not read | matrix, JPEG evidence ("MPF secondary images are not read") | `silent_drops::jpeg_gain_maps_are_ignored_and_the_base_image_is_read_today`; `discarded_report::jpeg_gain_maps_iptc_and_comments_are_reported` | C |
| J8 | Pixel aspect ratio (JFIF density with units 0, non-square) | ignored: display size is the stored pixel size. **Since `v1/engine-discarded`: reported: a JFIF density other than 1:1 aspect is `Discarded::OtherMetadata`** | `JpegSource::new` reads no JFIF fields | no | `silent_drops::jpeg_pixel_aspect_ratio_is_ignored_today` (patched `sample.jpg`); `discarded_report::jpeg_xmp_icc_and_other_metadata_are_reported` | C |
| J9 | Resolution (JFIF density in dpi, EXIF `XResolution`) | dropped; no output carries it. **Since `v1/engine-discarded`: reported: `Discarded::OtherMetadata` (JFIF density), `Discarded::Exif`** | not read | no (EXIF in general: J1) | `paris_exif_xmp_icc.jpg` (300 dpi) through J1's tests; `discarded_report::jpeg_xmp_icc_and_other_metadata_are_reported` | C |
| J10 | Orientation in XMP (`tiff:Orientation`) without EXIF | ignored | only EXIF is read | no | no fixture | A |
| J11 | EXIF orientation 1–8 | applied before crop | `src/orientation.rs` | matrix | `supported_matrix::jpeg_exif_orientations_are_applied_before_crop` | D |
| J12 | EXIF orientation outside 1–8, malformed EXIF | refused (`GeometricTransform`; `MalformedInput`) | `JpegSource::new` | matrix | `codecs::jpeg::tests::exif_orientation_is_reported_and_invalid_values_fail_closed` | C |
| J13 | Grayscale | expanded to RGB (value-preserving). **Since 2026-11-15: the adapter emits libjpeg's gray component and the pipeline's one expansion copies it (`src/gray.rs`, the same bits as libjpeg's own gray-to-RGB output); a gray ICC profile is converted under `UseRenderedIcc`** | `output_options`, `src/gray.rs` | matrix | `supported_matrix::jpeg_grayscale_is_expanded_to_rgb`; `format_parity::gray_is_expanded_to_rgb_in_every_format_that_carries_it`, `a_gray_icc_profile_is_converted_by_its_tone_curve` | B |
| J14 | Progressive scans | all decoded | libjpeg | — | `entropy_rows` = full height in the report | D |
| J15 | Decoder warnings: missing EOI, truncation, marker damage | `fail_on_warning: true`: every libjpeg warning is `MalformedInput`, no partial image | `codecs/jpeg.rs` `decoder` | matrix ("truncated data") | `supported_matrix::jpeg_truncated_is_malformed`, `codecs::jpeg::tests::cropped_read_rejects_a_truncated_tail` | C |
| J16 | Corrupted scan data that still parses | decodes without an error: libjpeg raises no warning for Huffman-coded garbage that stays decodable, so `fail_on_warning` has nothing to catch; nothing is swallowed by the engine | — | no | `silent_drops::corrupted_jpeg_scan_data_decodes_without_an_error_today` (16 inverted bytes in `paris-420.jpg`'s scan) | outside A–D (undetectable) |

### Still WebP (`webp-read`, libwebp-rs)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| W1 | `EXIF` beyond orientation, `XMP ` | orientation read, rest dropped. **Since `v1/engine-discarded`: reported: `Discarded::Exif`, `Discarded::Xmp`** | `src/codecs/webp.rs` `WebpSource::new_bounded`, `src/codecs/exif.rs` `orientation` | matrix, Outputs | `supported_matrix::webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp`; `discarded_report::webp_chunks_are_reported` | C |
| W2 | `ICCP` | rendered ICC, converted under `UseRenderedIcc` (reported) | `WebpSource::new_bounded` | matrix | `supported_matrix::webp_straight_alpha_input_keeps_alpha` | C |
| W3 | Unknown chunks | ignored (the container allows it). **Since `v1/engine-discarded`: reported: `Discarded::UnknownContainerData`** | libwebp `Container` | no | no fixture; `discarded_report::webp_chunks_are_reported` (renamed `EXIF` chunk) | C |
| W4 | Animation | refused, no first frame (`Animation`) | `new_bounded` | matrix | `supported_matrix::webp_animation_is_rejected_without_a_first_frame_fallback` | C |
| W5 | Straight alpha | kept | `new_bounded` | matrix | `supported_matrix::webp_straight_alpha_input_keeps_alpha` | D |
| W6 | Truncated file | `MalformedInput` | `decode_into` | matrix ("malformed containers") | `silent_drops::truncated_inputs_are_malformed` | C |
| W7 | Corrupted VP8/VP8L payload | decodes without an error. VP8 and VP8L carry no checksum, so the codec cannot notice; nothing is swallowed by the engine | — | no | `silent_drops::corrupted_webp_payload_decodes_without_an_error_today` | outside A–D (undetectable) |

### JPEG XL (libjxl-rs; jxl-oxide explicit until 2026-11-15)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| X1 | `Exif` and `xml ` boxes | ignored by both readers, dropped. **Since `v1/engine-discarded`: reported: `Discarded::Exif`, `Discarded::Xmp`, by both readers** | `crates/ferrisma-jxl-decoder` subscribes to `BASIC_INFO`, `COLOR_ENCODING`, `FULL_IMAGE` only; jxl-oxide ignores boxes | matrix, Outputs "Metadata" | `silent_drops::jpeg_xl_container_boxes_are_dropped_today` (`decode-test/1x1_exif_xmp.jxl`); `discarded_report::jpeg_xl_boxes_are_reported` | C |
| X2 | JPEG reconstruction data (`jbrd`) | ignored: the pixels are decoded, bit-exact JPEG recovery is lost. **Since `v1/engine-discarded`: reported: `Discarded::JpegReconstructionData`** | same | no | same fixture; `discarded_report::jpeg_xl_boxes_are_reported` | C |
| X3 | JUMBF / C2PA (`jumb`), gain map (`jhgm`) | ignored. **Since `v1/engine-discarded`: reported: `jumb` as `OtherMetadata`, `jhgm` as `GainMap` (code path, no fixture)** | same | no | no fixture | C |
| X4 | DATA ICC profile | converted under `UseRenderedIcc` (reported), otherwise refused. **Since 2026-11-15 (owner decision, #222): an enum encoding with an H.273 code point is reported as that CICP alone and the profile libjxl synthesizes from it is not reported (it is derived from the enum, nothing is lost); an embedded profile is reported as `Discarded::IccProfile` from the codestream's own signal (`original_icc`), which also covers an XYB image whose DATA encoding is linear sRGB (before, keyed on the DATA encoding, that case was not reported)** | `src/codecs/jxl.rs` | matrix | `supported_matrix::jpeg_xl_enum_rgb_is_typed_and_embedded_icc_needs_the_rendered_icc_policy` (until 2026-11-15 `jpeg_xl_rgb_needs_the_rendered_icc_policy`); `format_parity::embedded_icc_needs_the_rendered_icc_policy_in_every_format` (`tests/fixtures/parity/rgb-p3-icc.jxl`) | C |
| X5 | Extra channels other than one alpha, **libjxl-rs** | refused (`UnsupportedLayout`) before pixels | `ferrisma-jxl-decoder` `Decoder::info` | matrix | `supported_matrix::jpeg_xl_float_alpha_spot_and_extra_channels_are_rejected` | C |
| X6 | Depth, selection mask, black next to RGB, CFA, thermal and optional extra channels, **jxl-oxide** | accepted; the channels are left out of the pixel stream (a black channel next to RGB is lost, so the colors change). **Since `v1/engine-discarded`: refused by jxl-oxide as `UnsupportedLayout`, as by libjxl-rs (ADR-0042 D1, D4); D4 for JPEG XL is also met by removing the jxl-oxide runtime reader (owner decision 2026-11-15, pending the libjxl-rs AVX2 fix). Since `v1/remove-alt-backends`: resolved by removal; the only reader refuses these files** | jxl-oxide 0.12.6 `PixelFormat` (removed) | no (matrix: "jxl-oxide reports some extra-channel files as unspecified alpha") | `silent_drops::jpeg_xl_reader_refuses_extra_channels_other_than_alpha` (`extra-channel-type-{1,2,3,4,5,6,16}.jxl`, all policies) | removed |
| X7 | Spot colors, **jxl-oxide** | accepted; composited into the RGB samples (`render_spot_color`, default on for color images), an unrequested change of the color values. **Since `v1/engine-discarded`: refused, as X6; since `v1/remove-alt-backends`: resolved by removal** | jxl-oxide `fb.rs` | no | `silent_drops::jpeg_xl_reader_refuses_extra_channels_other_than_alpha` (`extra-channel-type-2.jxl`) | removed |
| X8 | Several alpha channels, **jxl-oxide** | the first is rendered, the others dropped. **Since `v1/engine-discarded`: refused, as X6; since `v1/remove-alt-backends`: resolved by removal** | jxl-oxide `fb.rs` ("Find alpha") | no | `silent_drops::jpeg_xl_reader_refuses_a_second_alpha_channel` | removed |
| X9 | Reserved extra-channel type that libjxl rejects | libjxl-rs: `MalformedInput`; jxl-oxide decodes it and drops the channel. **Since `v1/engine-discarded`: refused as `MalformedInput` by both readers, as X6; since `v1/remove-alt-backends`: resolved by removal** | — | no | `silent_drops::jpeg_xl_reader_rejects_a_reserved_extra_channel_type` (`extra-channel-rejected-15.jxl`) | removed |
| X10 | Layers, patches, blending, reference frames | composited into the displayed image by the reader, as libjxl does | decoders | jxl-oxide adapter tests (until 2026-11-15) | `tests/animated_input.rs::animated_jpeg_xl_is_rejected` (no first-frame fallback); the jxl-oxide adapter test that pinned the compositing left with the adapter on 2026-11-15, and no engine test pins it for libjxl-rs | D |
| X11 | Preview frame | ignored. **Since `v1/engine-discarded`: reported: `Discarded::Preview`** | decoders | no | no fixture; `discarded_report::jpeg_xl_preview_is_reported` (the former jxl-oxide reader rejected that fixture as malformed) | C |
| X12 | `intrinsic_size` (display size hint) | ignored | decoders | no | no fixture | A |
| X13 | Animation | refused (`Animation`) | the adapter | matrix | `supported_matrix::jpeg_xl_animation_is_rejected` | C |
| X14 | Gray, CMYK, float, premultiplied or unspecified alpha, PQ/HLG | refused with a reason. **Since 2026-11-15 gray and gray with alpha are accepted and expanded to RGB(A) (owner decision, ADR-0043; nothing is lost: R = G = B)** | adapters, `cms.rs` | matrix | `supported_matrix` JPEG XL cases | C |
| X15 | Orientation | rendered upright by the decoder | decoders | matrix | `orientation_tests::jpeg_xl_orientations_are_rendered_upright_for_every_crop` | D |
| X16 | Progressive passes, truncation | the full image is decoded; a truncated file is `MalformedInput` (no partial render) | `drive` | no | `silent_drops::truncated_inputs_are_malformed` | C |
| X17 | Bit depth above 8 (16-bit, 31-bit integer samples) | decoded as float, then 8-bit output. Since `v1/engine-discarded`: reported: `Discarded::PrecisionReduced`. **Since `v1/engine-depth`: preserved up to the output's depth (JPEG XL 16, AVIF 12), quantized at it; `PrecisionReduced` only where the output holds fewer bits (WebP, AVIF from 13+ bits, 31-bit to JPEG XL, an explicit lower depth)** | `pipeline.rs` depth rule | matrix (Bit depth rows) | `format_parity::depth_follows_the_input_in_every_output`; `jxl_output_tests::a_sixteen_bit_lossless_round_trip_is_exact`; `discarded_report::jpeg_xl_precision_above_eight_bits_is_reported` (WebP output) | D |

Finding, not a silent drop: jxl-oxide panics in a debug build on
`modular-rgb31.jxl` (`jxl-image` 0.13.0 `parse_integer_sample`, `1 << 31`
overflow); the release build wraps to the correct divisor. And jxl-oxide
reports the float layout of `modular-rgb-alpha-float24e7.jxl` as
`MalformedInput` ("straight alpha must be finite…") where libjxl-rs says
`UnsupportedLayout`: a misreported kind, not a hidden one.

### Still AVIF (`avif-read`, libavif/AOM)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| V1 | `Exif` and `mime` (XMP) items | ignored, dropped; an EXIF orientation inside is not applied (`irot`/`imir` govern). **Since `v1/engine-discarded`: reported: `Discarded::Exif`, `Discarded::Xmp`, `OtherMetadata` for other `mime`/`uri ` items** | `src/codecs/avif_meta.rs` reads the primary item's properties only | matrix, Outputs "Metadata" | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels`; `discarded_report::avif_metadata_items_icc_and_gain_maps_are_reported` | C |
| V2 | `colr prof`/`rICC`, `nclx` | ICC converted under `UseRenderedIcc`; `nclx` checked against the AV1 header and beside the ICC. **Since 2026-11-15 (owner decision): beside an ICC profile the file follows MIAF; the profile governs primaries and transfer and the `nclx` primaries and transfer are not used (superseded by the standard, not reported); matrix and range are used by libavif** | `avif.rs`, `cms.rs` | matrix | `supported_matrix::avif_container_icc_needs_the_rendered_icc_policy` | C |
| V3 | Gain map (`tmap`) | ignored, SDR base read. **Since `v1/engine-discarded`: reported: `Discarded::GainMap`** | libavif `imageContentToDecode` default | matrix, AVIF details | `supported_matrix::avif_gain_map_is_ignored_and_the_base_image_is_read`; `discarded_report::avif_metadata_items_icc_and_gain_maps_are_reported` | C |
| V4 | Alpha item with an unknown **essential** property | libavif skips the item (`hasUnsupportedEssentialProperty`); the image is read as **opaque** without an error; the engine checks only the primary item. **Since `v1/engine-discarded`: refused with `UnsupportedEssentialProperty`, like the primary item; so are grid cells; an unused item is no reason (ADR-0042 D1)** | `avif_meta.rs` `primary_properties`; libavif `read.c` | no | `silent_drops::avif_alpha_item_with_an_unknown_essential_property_is_refused` (patched `abc_color_irot_alpha_irot.avif`), `avif_grid_cell_with_an_unknown_essential_property_is_refused`, `avif_unknown_essential_property_on_an_unused_item_is_not_a_refusal` | C |
| V5 | Unknown essential property of the primary item | refused, `UnsupportedEssentialProperty` (since this branch; before, `MalformedInput`) | `avif.rs` `new_bounded` | matrix, compatibility | `supported_matrix::avif_unknown_essential_property_is_refused_as_unsupported` | C |
| V6 | Unknown non-essential properties, unknown `colr` types | ignored, as ISO/IEC 23008-12 permits. **Since `v1/engine-discarded`: reported: `Discarded::UnknownContainerData`** | `avif_meta.rs` `read_property` (`_ => {}`) | no | `silent_drops::avif_non_essential_unknown_properties_are_ignored_today` (`circle_custom_properties.avif`); `discarded_report::avif_properties_the_engine_does_not_interpret_are_reported` | C |
| V7 | `pasp` (pixel aspect ratio) | ignored (same path). **Since `v1/engine-discarded`: reported: `Discarded::UnknownContainerData`** | same | no | no fixture; `discarded_report::avif_properties_the_engine_does_not_interpret_are_reported` (Android `fox.*.avif` carry `pasp`) | C |
| V8 | Sample-transform (`sato`) alternative with more precision | the 12-bit primary is read | libavif without its experimental `sato` support | no | `silent_drops::avif_sample_transform_alternative_is_ignored_today` | A |
| V9 | Premultiplied alpha (`prem`) | libavif un-premultiplies into straight RGBA (`rgb.alphaPremultiplied = 0`); JPEG XL premultiplied alpha is refused instead. **Since `v1/engine-discarded`: reported: `Discarded::Unpremultiplied` (refusing it instead is an open question of ADR-0042 (premultiplied AVIF alpha)). Since 2026-11-15: decided by the owner (accept and convert, ADR-0042 and ADR-0043); JPEG XL is converted by libjxl and reported the same way** | `crates/ferrisma-avif-read` `rgb` | matrix | `tests/fixtures/parity/premultiplied-alpha.avif` (the same `prem` insertion, committed); `discarded_report::premultiplied_avif_alpha_is_reported_as_unpremultiplied`; `format_parity::premultiplied_alpha_is_converted_and_reported_alike` | C |
| V10 | Depth and other auxiliary images in a still | ignored (only alpha is read) | libavif | matrix, AVIF details | no still fixture | B |
| V11 | Grid, `pymd` pyramid | composed grid / primary item at full size | libavif | matrix | `supported_matrix::avif_alpha_and_grid_stills_are_accepted`; `pyramid_pymd.avif` (probe) | D |
| V12 | Sequences, `clap`, `imir` before `irot`, monochrome, PQ/HLG | refused with a reason. **Since 2026-11-15 monochrome is accepted through libavif's gray output and expanded to RGB (owner decision, ADR-0043)** | `avif.rs` | matrix | `supported_matrix` AVIF cases | C |
| V13 | `irot`/`imir` | applied before crop | `orientation.rs` | matrix | `supported_matrix::avif_irot_and_imir_are_applied_for_all_eight_orientations` | D |
| V14 | 10/12-bit samples | normalized float, then 8-bit output. Since `v1/engine-discarded`: reported: `Discarded::PrecisionReduced`. **Since `v1/engine-depth`: preserved in AVIF and JPEG XL output; reported for WebP output and an explicit lower depth** | `avif.rs` `read_region`, `pipeline.rs` depth rule | matrix | `supported_matrix::avif_eight_ten_and_twelve_bit_in_every_chroma_layout_are_accepted`; `discarded_report::avif_properties_the_engine_does_not_interpret_are_reported` (10-bit against 8-bit, WebP output), `discarded_report::avif::deeper_input_keeps_its_depth_and_an_explicit_lower_depth_is_reported`, `pipeline_tests::deep_rows::a_deep_avif_keeps_its_depth_through_avif_output` | D |
| V15 | Limited range | expanded to full range by libavif (value-preserving) | libavif | `avif.rs` module doc | — | D |
| V16 | Chroma upsampling | nearest neighbor, forced by the wrapper | `ferrisma-avif-read` `rgb` | `avif.rs` module doc only | — | B |
| V17 | Truncation, corrupted payload | `MalformedInput` | `bridge_error` | matrix | `silent_drops::truncated_avif_is_malformed`; 16 inverted bytes in `paris_icc_exif_xmp.avif` (probe) | C |

## Outputs

| # | Output | Item | Today | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| O1 | AVIF (both encoders until 2026-11-15; libavif only since) | source EXIF/XMP/ICC | not written; sRGB signaled (libavif `nclx` 1/13/6 full; the removed rav1e in the AV1 header only). **Since `v1/engine-discarded`: reported on the input side: `Exif`, `Xmp`, `IccProfile`** | matrix, Outputs | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels` | C |
| O2 | AVIF libavif 10-bit (default) | precision | the 10-bit file carries 8-bit rows. **Since `v1/engine-depth`: a 10- or 12-bit AVIF carries content quantized at that depth (`u16` rows); the default depth follows the input** | matrix (Outputs, Bit depth) | `ferrisma_avif_write::deep_output_equals_the_c_original`; `pipeline_tests::deep_rows::a_deep_avif_keeps_its_depth_through_avif_output` | D |
| O3 | AVIF libavif | `quality` fraction | rounded to an integer. **Since `v1/engine-options` (owner decision of 2026-11-15, ADR-0043 D4): resolved by decision; `AvifOptions::quality` and `alpha_quality` are libavif's integers 0–100, passed unchanged** | `crates/ferrisma-avif-write` | rustdoc of `AvifOptions` | `supported_matrix::libavif_quality_alpha_quality_speed_and_depth_ranges` | resolved |
| O4 | AVIF rav1e | depth, alpha quality | always 10-bit; alpha at fixed quality 80 whatever `quality` says. **Since `v1/remove-alt-backends`: resolved by removal; rav1e is no longer an encoder of the engine** | rustdoc, matrix | the tests `rav1e_writes_ten_bit_without_a_container_colr` and `rav1e_writes_an_alpha_item_for_transparent_input` were removed with it | removed |
| O5 | All | orientation | applied, written upright with no transform property | matrix | `jpeg_xl_output_is_written_upright` and the AVIF/WebP cases | D |
| O6 | All | fully opaque alpha | omitted (no information lost) | matrix | `libavif_writes_an_alpha_item_only_for_transparent_input` | D |
| O7 | WebP lossy | chroma | always 4:2:0; a nondefault `chroma_subsampling` is refused | matrix | `avif_settings_with_webp_output_are_mismatched_options` | B |
| O8 | WebP lossy | `speed` | 2/3, 5/6 and 8/9 give the same method and the same bytes. **Since `v1/engine-options` (owner decision of 2026-11-15, ADR-0043 D4): resolved by decision; `WebpOptions::Lossy { method }` is libwebp's method 0–6, passed unchanged, and every method gives its own bytes** | `src/webp_sink.rs` | rustdoc of `WebpOptions` | `silent_drops::webp_lossy_methods_are_libwebps_own_and_distinct` | resolved |
| O9 | WebP | metadata | only the engine's sRGB `ICCP`; EXIF/XMP dropped. **Since `v1/engine-discarded`: reported on the input side: `Exif`, `Xmp`, `IccProfile`** | matrix | `webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp` | C |
| O10 | WebP lossless | color under alpha 0 | libwebp keeps it (`exact`), but the engine has already made it black (P5), so the "lossless" file does not keep hidden color. **Since `v1/engine-discarded`: reported: `Discarded::HiddenColor`** | ADR-0014 | P5 tests; `discarded_report::hidden_color_under_zero_alpha_is_reported` | C |
| O11 | JPEG XL, both profiles | source EXIF/XMP | a raw codestream cannot carry them; the matrix does not say so for JPEG XL. **Since `v1/engine-discarded`: reported on the input side: `Exif`, `Xmp`** | no | `silent_drops::source_exif_and_xmp_are_dropped_from_jpeg_xl_output_today` | C |
| O12 | JPEG XL lossy | color under alpha 0, patches | color replaced (`keep_invisible`), patch search off. **Since `v1/engine-discarded`: hidden color reported: `Discarded::HiddenColor` (patches are an encoder setting, not input data)** | matrix, rustdoc (fixed on this branch) | `jxl_lossy_tests::text_like_content_encodes_with_patches_off` | C |
| O13 | JPEG XL lossless | samples | exact for the 8-bit rows | ADR-0039 | `jxl_output_tests::sink_round_trips_exactly_through_the_workspace_decoder` | D |

## Pipeline: precision, range, color and alpha

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| P1 | Bit depth | every input reaches the encoder as 8-bit RGBA rows. Since `v1/engine-discarded`: reported when the input is deeper. **Since `v1/engine-depth`: the executors quantize at the output depth (`quantize_row_deep`, `u16` rows), which follows the input; `PrecisionReduced` exactly when the output depth is below the input's** | `fast_execute.rs`, `quality_execute.rs`, `output_depth.rs` | matrix, ADR-0021 amendment | `fast_execute_tests::deep_output_meets_the_bound_against_the_f64_oracle`, `every_deep_code_survives_an_identity_transform_at_its_depth`; `format_parity::depth_follows_the_input_in_every_output` | D |
| P2 | Out-of-gamut and over-range values (Display P3, float JPEG XL, Lanczos overshoot) | kept through resampling, clipped to [0, 1] at final quantization; nothing reports how much was clipped | `quantize_row` (`clamp(0.0, 1.0)`) | ADR-0013, ADR-0016 (rule 5 asks for a "visible decision … with output metadata", not delivered) | `tests/icc_photo.rs::photograph_as_display_p3_matches_littlecms_out_of_gamut` | B |
| P3 | ICC profiles outside moxcms' extended matrix-shaper path (LUT profiles) | clamped to the destination gamut inside moxcms, before resampling | `cms.rs` `IccSrgbSource::new` | ADR-0016 (2026-11-06), ADR-0023 | `cms::tests` | B |
| P4 | Rendering intent | moxcms' default `Perceptual` is used for every profile; the profile header's intent is not consulted. Verified 2026-11-15: the intent is set nowhere in the engine; `src/cms.rs` `IccSrgbSource::new` builds `TransformOptions { allow_extended_range_rgb_xyz, ..TransformOptions::default() }`, and moxcms 0.9.1 `TransformOptions::default()` sets `rendering_intent: RenderingIntent::default()` (`transform.rs:141`), which is `Perceptual` (`profile.rs:599`). ADR-0016 (amendment #17/#41) lists "LUT-based profiles, intents other than relative colorimetric" as open, which reads as if relative colorimetric were used. Where it matters: moxcms reads the intent only in its LUT paths (`conversions/lut_transforms.rs`, `md_luts_factory.rs`: `get_device_to_pcs` picks `A2B0` for perceptual and `A2B1` for relative colorimetric, `transform.rs:1202`, plus black-point detection); when source and destination are both matrix/TRC profiles (`transform.rs:673`, `800`), as for every profile in the repository's tests, the intent has no effect and the output is the same. A LUT-based source profile with distinct perceptual and colorimetric tables gives different output. **Resolved 2026-11-15 by owner decision:** the intent is relative colorimetric, set explicitly in `src/cms.rs` ([ADR-0016](adr/0016-working-color-space-and-transform-boundaries.md) amendment of that date); matrix/TRC output unchanged, LUT profiles use `A2B1`. A fixed, documented part of the contract, not per-input information, so not reported | `cms.rs` (`TransformOptions::default()`) | no (misleading) | `cms::tests::lut_profiles_convert_with_the_relative_colorimetric_table` (a profile built with moxcms' writer; no real LUT-based fixture in the repository, open under #17/#211) | resolved |
| P5 | Color under alpha 0 | made transparent black. **Since `v1/engine-discarded`: reported: `Discarded::HiddenColor` (input pixels of the processed region with alpha exactly 0 and nonzero color)** | `quantize_row` | ADR-0014 | `tests/resize_oracle.rs` (transparent black), `linear_srgb_tests::all_transparent_pixels_yield_canonical_transparent_black`; `discarded_report::hidden_color_under_zero_alpha_is_reported` | C |
| P6 | Alpha precision | 10/12/16-bit or float alpha quantized to 8 bits. Since `v1/engine-discarded`: reported. **Since `v1/engine-depth`: alpha is quantized at the output depth too; the input depth is the larger of color and alpha, so a deeper alpha raises the output depth or is reported (float alpha stays refused)** | `quantize_row_deep` | matrix | `jxl_output_tests::deep_output_equals_the_original_cjxl_bytes` (12- and 16-bit alpha) | D |
| P7 | Miscoded `cicp` tag inside an ICC profile | ignored as a description (its PQ/HLG still refuses) | `cms.rs` | ADR-0016 (#17, #41) | `cms::tests` | B |
| P8 | Missing color signal under `AssumeSrgbIfMissing` | sRGB assumed, `color_signal = AssumedSrgb` | `pipeline.rs` `run` | matrix | `tests/color_signal_and_refusals.rs` | C |
| P9 | Display P3 or linear-sRGB CICP | converted, `color_signal = Cicp` | `fast_execute.rs` | matrix | `pipeline_tests` | C |

## Options and the command line

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| Q1 | `output_width`/`output_height` with `transcode_resized` | ignored; the actual size is in `Output`. **Since `v1/engine-options` (owner decision of 2026-11-15): a value other than the default (1) is refused before decoding by `transcode_resized` and `transcode_resized_with` (`InvalidOptions`, "output_width and output_height are not used with ResizeOptions…")** | `pipeline.rs` `run` | rustdoc | `thumbnail_facade::resize_entry_points_refuse_an_exact_output_size` | C |
| Q2 | Default `avif_encoder` (since 2026-11-15: default `avif_depth`) with an external backend | ignored, the backend decides; a nondefault value is refused. **Since `v1/engine-options`: every other encoder setting reaches the backend in its `EncodeRequest`; the depth, which the request does not carry, is refused when nondefault (pinned through the public API)** | `run` | rustdoc of `transcode_with` | `silent_drops::external_backends_refuse_settings_the_request_does_not_carry` | C |
| Q3 | `jxl_decoder` for non-JPEG XL input | ignored; `decoder_backend` names the reader that ran. **Since `v1/remove-alt-backends`: resolved by removal; the option does not exist** | `open_source` | no | the test `silent_drops::jpeg_xl_reader_selection_is_ignored_for_jpeg_input_and_reported` was removed with the option | removed |
| Q4 | `kernel_level_ceiling`, `transform_threads` with `F64Reference` | ignored; the report says `Scalar`/`Ceiling` and 1 thread | `run` | rustdoc of `ProcessingReport` | `pipeline_tests` | C |
| Q5 | Thread counts in `Engine::transcode_batch` | replaced with 1 (reported); values above 256, which `transcode` refuses, are accepted and discarded. **Since `v1/engine-options` (owner decision of 2026-11-15): values above 256 are refused for the item as by `transcode` (`InvalidOptions`, same message); valid values are still replaced with 1, documented and reported** | `batch` | `BatchItem::options` | `silent_drops::batch_refuses_thread_counts_that_transcode_rejects` | C |
| Q6 | `Engine::new` above 256 | capped, `threads()` reports it | `Engine::new` | rustdoc | — | C |
| Q7 | Repeated or contradictory CLI flags (`--assume-srgb` after `--use-rendered-icc`, two `--transform-threads=` (two `--jxl-decoder=` until that flag was removed on 2026-11-15), two `--quality=`) | the last one wins silently; only `--avif-encoder`/`--avif-depth` are refused when repeated. **Since `v1/engine-options` (owner decision of 2026-11-15): refused before any input is read: a repeated flag is "--… may only be given once", a pair of mutually exclusive flags (`--assume-srgb`/`--use-rendered-icc`, `--jxl-lossless`/`--jxl-distance=`, lossless WebP with lossy-only settings) names both flags; exit status 1 (2, a usage error, since 2026-11-16)** | `src/cli.rs` `run` (`refuse_exclusive_flags`, the repetition check) | usage text, compatibility guide | `silent_drops::cli_refuses_repeated_and_contradictory_flags`, `cli::tests::refuses_repeated_and_contradictory_flags_before_reading_input` | C |
| Q8 | `--version` with other arguments | prints the version, ignores the rest | `is_version_request` | compatibility guide | `cli::tests` | B |
| Q9 | Mismatched encoder settings (`quality` with WebP or JPEG XL, `webp` with AVIF, `--chroma` with WebP; since `v1/engine-options`: a nondefault `AvifOptions`, `WebpOptions` or `JpegXlOptions` beside another output, `--avif-*`/`--webp-*` flags with another output) | refused before decoding | `run`, `cli.rs` | matrix | `supported_matrix::avif_settings_with_webp_output_are_mismatched_options`, `avif_settings_conflict_with_other_outputs`, `jpeg_xl_distance_effort_and_foreign_settings_are_checked_before_decoding` | C |

## Caught errors and fallbacks in the code

Searched in `src/` and `crates/ferrisma-{jxl-decoder,jxl-encoder,avif-read,avif-write,libavif}`:
`.ok()`, `unwrap_or`, `if let Err(_)`, `let _ =`, catch-all `_ =>` arms and
"fallback". Every hit was judged; the ones that are not plain integer
conversions into a typed error:

| Hit | Judgment | Class |
| --- | --- | --- |
| `src/codecs/avif.rs` `new_bounded`: `_ => Error::DecodeFailed` for every `MetaError` | swallowed the class of an unknown essential property into `MalformedInput`; **fixed** (Part 3). `UnsupportedVersion` (a `meta`, `pitm` or `ipma` version the parser does not know) still maps to `MalformedInput`: misreported, not silent | C |
| `src/cms.rs` `ChannelTable::build` (`.ok()?`) | the 768-entry fast table is not built; the exact moxcms path runs instead. Same samples within `CHANNEL_TABLE_BOUND`, no error lost | D |
| `crates/ferrisma-jxl-decoder` `data_transfer` (`None` when the getter fails) | treated as an ICC-signalled encoding; the rendered-ICC stage then checks the profile, so it fails closed | D |
| `crates/ferrisma-jxl-decoder` `region_callback` (`catch_unwind … unwrap_or(false)`) | a panic marks the sink failed, the decode returns `Decode` | C |
| `src/codecs/jpeg.rs` `fail_on_warning: true` | libjpeg's recoverable warnings become errors instead of a repaired image (scan corruption without a warning still decodes: J16) | C |
| `src/codecs/jxl.rs` `drive`, `_ => Err(Unsupported)` | an unexpected decoder event is an error | C |
| `src/parallel_execute.rs` `let _ = done.send(Err(error))` | the driver is already gone; it reports its own error | C |
| `src/pipeline.rs` `batch` (`unwrap_or_else` → `Internal`), `crates/ferrisma-avif-write` band `join` | a worker that died is an error, never a missing image | C |
| `src/codecs/jxl_oxide.rs` `rendered_cicp` (`_ => return None`) | no CICP is claimed without an exact H.273 equivalent; the ICC describes the samples. The file was removed on 2026-11-15 | D |
| `src/codecs/avif_meta.rs` `read_property` (`_ => {}`) | unknown properties and `colr` types ignored: V6, V7 | A |

## Update: removal of the alternative backends (2026-11-15)

The owner decided on 2026-11-15 that jxl-oxide and rav1e leave the
engine's runtime (ADR-0032, ADR-0037 amendments). Branch
`v1/remove-alt-backends` removes the jxl-oxide reader and the rav1e writer,
so every JPEG XL file has one reader and AVIF one writer and ADR-0042 D4 is met for JPEG XL by construction:

- X6 to X9 (the jxl-oxide reader's extra channels, spot colors, extra
  alpha, reserved types) are **resolved by removal**: the cases cannot
  occur any more. Their tests stay and now pin the refusals of the only
  reader (`silent_drops::jpeg_xl_reader_*`).
- Q3 (`jxl_decoder` ignored for other formats) is **resolved by removal**:
  the option is gone, and its test went with it.
- The same branch removes rav1e: O4 (rav1e's fixed 10-bit depth and fixed
  alpha quality 80) is **resolved by removal**; O1 holds for libavif only;
  Q2 now concerns `TransformOptions::avif_depth`, which replaces the removed
  `avif_encoder` (a nondefault depth with a backend or another output is
  refused, as a nondefault encoder selection was).
- X10 lost its test: the jxl-oxide adapter's own tests were removed with
  the adapter. The compositing of layers into the displayed image by
  libjxl-rs has no engine test.
- The jxl-oxide findings below the JPEG XL table (the debug-build panic on
  31-bit samples, the misreported float layout) no longer reach the
  engine; they remain upstream reports (#213).
- An unknown colour space (the fuzz regression
  `jxl-unknown-colour-space.jxl`) is `MalformedInput` without a reason on
  the only reader, as libjxl rejects it; on jxl-oxide it was refused as
  `UnsupportedColor` with `UnsupportedColorEncoding`. A change of kind,
  not a silent drop.

## Update: format parity (2026-11-15)

The owner's decisions of 2026-11-15 that make the formats behave alike
(ADR-0043; branch `v1/engine-parity`) change these rows. Each item names
its test in `tests/format_parity.rs`:

- X4: a JPEG XL enum encoding with an H.273 code point is a typed signal
  (CICP), read under every policy like AVIF `nclx` (#222); only an embedded
  profile needs `UseRenderedIcc`, and `Discarded::IccProfile` is now keyed
  on the codestream's own ICC signal, so an XYB image with an embedded
  profile (whose DATA encoding is linear sRGB) is no longer silent. No item
  is lost: the profile libjxl synthesizes from an enum is derived from it.
- J13, X14, V12: gray is accepted in JPEG, JPEG XL and AVIF and expanded
  to RGB (gray with alpha to RGBA) by one exact copy after color handling;
  a gray ICC profile is converted through its tone curve, a profile of the
  other color space is refused (`IccColorSpaceMismatch`). Nothing is
  dropped (R = G = B carries the gray value); hidden color under zero alpha
  is now also noticed for gray with alpha.
- V9, X14: premultiplied JPEG XL alpha is converted by libjxl's own step
  and reported as `Discarded::Unpremultiplied`, as AVIF's by libavif
  (owner decision; ADR-0042's former question 3). The two steps' rounding
  differs (matrix). The `Unspecified` alpha mode left with jxl-oxide.
- V2: AVIF ICC beside `nclx` follows MIAF; files with a concrete non-sRGB
  `nclx` beside a profile, refused before, are converted through the
  profile. The `nclx` primaries and transfer are superseded, not dropped
  information; they are named in the matrix and not reported.

## Update: options and the command line (2026-11-15)

The owner decided on 2026-11-15 how options without effect and the command
line behave (ADR-0042, open questions on options, repeated flags and a
strict mode). Branch `v1/engine-options` implements them:

- Q1, Q2, Q5: an option that would have no effect is refused where the
  caller set a value other than the default: an exact output size with the
  resize entry points, a thread count above 256 in a batch (as `transcode`
  refuses it; valid counts are still replaced with 1 and reported), and an
  AVIF depth with an external backend, whose request does not carry it.
  Kept, because the report states the value that was used and the rustdoc
  documents it: Q4 (`kernel_level_ceiling` and `transform_threads` under
  `F64Reference`: the report says `Scalar`/`Ceiling` and 1 thread), Q5's
  replacement of valid batch thread counts with 1 (the report says 1), Q6
  (`Engine::new` caps at 256, `threads()` says so) and Q8 (`--version`
  takes precedence over every other argument and produces no image).
  The former `_today` test of Q5 is now
  `batch_refuses_thread_counts_that_transcode_rejects`.
- Q7: a repeated command-line flag, and a pair of contradictory flags, are
  errors that name the flags; nothing is "last one wins" any more. The
  former `_today` test is now `cli_refuses_repeated_and_contradictory_flags`.
- O8, O3: encoder controls are native per format (ADR-0043 D4). WebP lossy
  takes libwebp's own `method` 0–6 instead of a `speed` mapped onto it
  (the former `_today` test is now
  `webp_lossy_methods_are_libwebps_own_and_distinct`); AVIF takes
  `AvifOptions` with libavif's integer quality and a separate alpha
  quality, at `avifenc`'s defaults (ADR-0037). The general `quality`,
  `speed`, `chroma_subsampling` and `avif_depth` are removed; a setting of
  another output is refused (Q9).
- No strict mode: no option turns `ProcessingReport::discarded` into a
  refusal; a caller or script checks the report. The CLI's `report:` line
  ends with `discarded=none` when nothing was left out
  (`cli::tests::report_line_names_the_decoder_and_encoder_backends`).

## Part 2: proposal (not implemented; the owner decides)

_Update 2026-11-15:_ ADR-0042 accepted the principle. Implemented on
`v1/engine-discarded`: the refusals (V4 for every item the AVIF reader
uses, X6 to X9 in the jxl-oxide reader) and the report, as
`ProcessingReport::discarded: DiscardedSet` of `Discarded` classes
(`GainMap`, `Exif`, `Xmp`, `OtherMetadata`, `IccProfile`,
`UnknownContainerData`, `JpegReconstructionData`, `Preview`,
`HiddenColor`, `Unpremultiplied`, `PrecisionReduced`), a `Copy` set rather
than a `Vec` so that the report stays `Copy`. Not implemented, as below
and by the owner's brief: `GamutClipped` (see D6 above), separate
`ContentProvenance`, `AuxiliaryImage`, `HigherPrecisionAlternative` and
`PixelAspectRatio` classes (C2PA, PAR and resolution are inside
`OtherMetadata`/`UnknownContainerData`), and every change to options,
CLI flags and premultiplied AVIF alpha. The proposal below is kept as
written.

**Recommended shape: a report field, plus refusals where the result would
be wrong.** Add `ProcessingReport::discarded: Vec<Discarded>` (a
`#[non_exhaustive]` enum; the report is `#[non_exhaustive]` and cannot be
constructed outside the crate, so adding the field is not breaking), and
print `discarded=` on the CLI's `report:` line (appending a key is allowed by
ADR-0040). A `Vec` rather than `&[Discarded]` keeps the report `'static`;
a bit set would also do. Each variant names a class, like `RefusalReason`,
not one value per file:

| Variant | Items | Why a report and not a refusal |
| --- | --- | --- |
| `Metadata` (or `Exif`, `Xmp`, `Iptc`, `Comment` separately) | J1–J4, J9, W1, X1, V1, O1, O9, O11 | dropping metadata is the product's normal job (thumbnails); the caller must know, not be stopped |
| `ContentProvenance` (C2PA/JUMBF) | J5, X3 | separate from `Metadata` because removing a signed manifest has legal and trust consequences a caller may want to refuse on |
| `GainMap` | J7, V3, X3 (`jhgm`) | the SDR base is a faithful image |
| `AuxiliaryImage` (depth, other `auxl`, MPF images, preview) | V10, X11 | as above |
| `HigherPrecisionAlternative` | V8 | as above |
| `JpegReconstruction` | X2 | as above |
| `PixelAspectRatio`, `Resolution`, `IntrinsicSize` | J8, J9, V7, X12 | a hint, not pixels; reporting it lets the caller correct the geometry |
| `UnknownProperties` (non-essential) | V6, W3 | permitted by the formats; low value, so optional |
| `GamutClipped` (with a count or a flag) | P2, P3 | ADR-0016 rule 5 already asks for it; a cheap flag in `quantize_row` |
| `HiddenColor` | P5, O10, O12 | documented behavior; a flag lets lossless callers notice |

**Refusals** (by default, with an explicit opt-in to discard where a caller
wants the base image anyway) for the cases where the output is *wrong*, not
merely smaller:

- X6, X7, X8, X9 (jxl-oxide extra channels, spot colors, extra alpha,
  reserved types): refuse in the jxl-oxide adapter exactly as libjxl-rs does
  (`UnsupportedLayout`), by checking `metadata.ec_info` before rendering. The
  two readers then agree. A behavior change for jxl-oxide users (files that
  decode today would be refused); not an API break.
- V4 (alpha item with an unknown essential property): check the essential
  properties of every item the primary depends on (alpha, grid cells), as
  libavif's comment says the rule should apply, and refuse with
  `UnsupportedEssentialProperty`. A behavior change; not an API break.
- V9 (premultiplied AVIF alpha): refuse with `AlphaAssociation`, consistent
  with JPEG XL, or report the conversion; the owner's call, since it is
  value-changing at low alpha.
- J10 (XMP-only orientation): report rather than apply; applying would be a
  new precedence rule.

**Options** should never be accepted and ignored:

- O8: either map `speed` 1–10 onto seven distinct methods and document it
  as 1–7, or document the collapse. Changing the mapping changes output bytes
  for some speeds (allowed in a minor release by the compatibility guide).
- Q5: validate thread counts in `transcode_batch` as `transcode` does
  (`InvalidOptions`). A behavior change for callers who pass invalid values.
- Q7: refuse a repeated or contradictory CLI flag ("--assume-srgb and
  --use-rendered-icc are mutually exclusive"), as already done for
  `--avif-encoder`. A breaking change for scripts that rely on last-wins,
  which ADR-0040 would have to allow for the CLI.
- Q3: optionally refuse a nondefault `jxl_decoder` for other formats; it is
  reported today, so this is low priority.

**Documentation only:** P4 (state that the intent is perceptual, or pass
the profile's intent explicitly), O11 (JPEG XL output carries no metadata),
the matrix rows of X6–X9.

## Part 3: the fix on this branch

An unknown property that the primary AVIF item marks essential was reported
as `MalformedInput` with no reason, although the file is well-formed and
merely uses something the engine does not understand. It is now
`UnsupportedInput` with the new `RefusalReason::UnsupportedEssentialProperty`
and the message "an essential container property is not supported:
'<type>'". No existing reason fits by its documented meaning
(`UnsupportedLayout` covers channels, sample formats and YUV matrices,
`GeometricTransform` orientation and cropping), so one variant is added,
named after the HEIF concept (ISO/IEC 23008-12 section 10.2.1) so that
another HEIF-family reader can share it. Working decision 2026-11-15
(delegated, revisable by the owner): the name and the class boundary. A
property box that is really malformed (an association that points outside
`ipco`) is still `MalformedInput`:
`supported_matrix::avif_property_association_outside_ipco_is_malformed`.

## Not covered

No fixture in the repository for: C2PA/JUMBF in any format, JPEG XL preview
frames, `intrinsic_size` and `jhgm`, AVIF `pasp` and premultiplied alpha,
LUT-based ICC profiles, XMP-only JPEG orientation, unknown WebP chunks, and a
progressive AVIF with a supported color signal (`draw_points_idat_progressive.avif`
is refused for its color first). 12-bit, 16-bit, arithmetic-coded and
lossless JPEG are outside the port's scope and were not probed. Inputs
through an external `EncoderBackend` were not audited beyond what the engine
passes in `EncodeRequest`.
