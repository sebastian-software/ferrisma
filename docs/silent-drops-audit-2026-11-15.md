# Silent-drop audit (2026-11-15)

**Question.** What does a *successful* `transcode` drop, ignore or change
without an error and without telling the caller? The owner's principle,
stated on 2026-11-15: behavior is negotiable, but nothing may disappear
implicitly and no error may be hidden or swallowed. The engine's explicit,
typed refusals ([supported matrix](supported-matrix.md),
[compatibility guide](compatibility.md#refusal-reasons)) satisfy it; this
page lists what passes while something is lost.

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
| A | 25 | 20 | 2 | 1 | 2 |
| B | 29 | 12 | 8 | 6 | 3 |
| C | 21 | 15 | 0 | 2 | 4 |
| D | 11 | 8 | 3 | 0 | 0 |

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
| J1 | EXIF beyond orientation (camera, date, GPS, resolution, embedded thumbnail) | read for the orientation tag only, then dropped; no output writes EXIF | `src/codecs/jpeg.rs` `JpegSource::new`, `exif_orientation` | matrix, Outputs "Metadata" (AVIF, WebP) | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels`, `webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp`; `silent_drops::source_exif_and_xmp_are_dropped_from_jpeg_xl_output_today` | B |
| J2 | XMP, extended XMP | ignored, dropped | not read anywhere | matrix, Outputs "Metadata" | same; `paris_extended_xmp.jpg` (probe) | B |
| J3 | IPTC (APP13 `Photoshop 3.0`) | ignored, dropped | not read | no | `silent_drops::jpeg_iptc_and_comments_are_dropped_today` (`seine_sdr_gainmap_srgb.jpg`) | A |
| J4 | `COM` comments | ignored, dropped | not read | no | same (`apple_gainmap_new.jpg`) | A |
| J5 | JUMBF / C2PA (APP11) | ignored, dropped; a content-provenance manifest disappears | not read | no | no fixture in the repository | A |
| J6 | ICC profile | `UseRenderedIcc`: converted to sRGB, `color_signal = RenderedIcc`; otherwise refused (`RenderedIccRequiresOptIn`) | `src/cms.rs` `IccSrgbSource` | matrix, Color policy | `supported_matrix::jpeg_embedded_icc_needs_the_rendered_icc_policy`, `tests/icc_photo.rs` | C |
| J7 | MPF secondary images: Ultra HDR and Apple gain maps | the SDR base image is read; the gain map and its `hdrgm` XMP are ignored; no signal that HDR content existed | not read | matrix, JPEG evidence ("MPF secondary images are not read") | `silent_drops::jpeg_gain_maps_are_ignored_and_the_base_image_is_read_today` | B |
| J8 | Pixel aspect ratio (JFIF density with units 0, non-square) | ignored: display size is the stored pixel size | `JpegSource::new` reads no JFIF fields | no | `silent_drops::jpeg_pixel_aspect_ratio_is_ignored_today` (patched `sample.jpg`) | A |
| J9 | Resolution (JFIF density in dpi, EXIF `XResolution`) | dropped; no output carries it | not read | no (EXIF in general: J1) | `paris_exif_xmp_icc.jpg` (300 dpi) through J1's tests | A |
| J10 | Orientation in XMP (`tiff:Orientation`) without EXIF | ignored | only EXIF is read | no | no fixture | A |
| J11 | EXIF orientation 1–8 | applied before crop | `src/orientation.rs` | matrix | `supported_matrix::jpeg_exif_orientations_are_applied_before_crop` | D |
| J12 | EXIF orientation outside 1–8, malformed EXIF | refused (`GeometricTransform`; `MalformedInput`) | `JpegSource::new` | matrix | `codecs::jpeg::tests::exif_orientation_is_reported_and_invalid_values_fail_closed` | C |
| J13 | Grayscale | expanded to RGB (value-preserving) | `output_options` | matrix | `supported_matrix::jpeg_grayscale_is_expanded_to_rgb` | B |
| J14 | Progressive scans | all decoded | libjpeg | — | `entropy_rows` = full height in the report | D |
| J15 | Decoder warnings: missing EOI, truncation, marker damage | `fail_on_warning: true`: every libjpeg warning is `MalformedInput`, no partial image | `codecs/jpeg.rs` `decoder` | matrix ("truncated data") | `supported_matrix::jpeg_truncated_is_malformed`, `codecs::jpeg::tests::cropped_read_rejects_a_truncated_tail` | C |
| J16 | Corrupted scan data that still parses | decodes without an error: libjpeg raises no warning for Huffman-coded garbage that stays decodable, so `fail_on_warning` has nothing to catch; nothing is swallowed by the engine | — | no | `silent_drops::corrupted_jpeg_scan_data_decodes_without_an_error_today` (16 inverted bytes in `paris-420.jpg`'s scan) | outside A–D (undetectable) |

### Still WebP (`webp-read`, libwebp-rs)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| W1 | `EXIF` beyond orientation, `XMP ` | orientation read, rest dropped | `src/codecs/webp.rs` `read_exif_orientation` | matrix, Outputs | `supported_matrix::webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp` | B |
| W2 | `ICCP` | rendered ICC, converted under `UseRenderedIcc` (reported) | `WebpSource::new_bounded` | matrix | `supported_matrix::webp_straight_alpha_input_keeps_alpha` | C |
| W3 | Unknown chunks | ignored (the container allows it) | libwebp `Container` | no | no fixture | A |
| W4 | Animation | refused, no first frame (`Animation`) | `new_bounded` | matrix | `supported_matrix::webp_animation_is_rejected_without_a_first_frame_fallback` | C |
| W5 | Straight alpha | kept | `new_bounded` | matrix | `supported_matrix::webp_straight_alpha_input_keeps_alpha` | D |
| W6 | Truncated file | `MalformedInput` | `decode_into` | matrix ("malformed containers") | `silent_drops::truncated_inputs_are_malformed` | C |
| W7 | Corrupted VP8/VP8L payload | decodes without an error. VP8 and VP8L carry no checksum, so the codec cannot notice; nothing is swallowed by the engine | — | no | `silent_drops::corrupted_webp_payload_decodes_without_an_error_today` | outside A–D (undetectable) |

### JPEG XL (libjxl-rs default, jxl-oxide explicit)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| X1 | `Exif` and `xml ` boxes | ignored by both readers, dropped | `crates/ferrisma-jxl-decoder` subscribes to `BASIC_INFO`, `COLOR_ENCODING`, `FULL_IMAGE` only; jxl-oxide ignores boxes | matrix, Outputs "Metadata" | `silent_drops::jpeg_xl_container_boxes_are_dropped_today` (`decode-test/1x1_exif_xmp.jxl`) | B |
| X2 | JPEG reconstruction data (`jbrd`) | ignored: the pixels are decoded, bit-exact JPEG recovery is lost | same | no | same fixture | A |
| X3 | JUMBF / C2PA (`jumb`), gain map (`jhgm`) | ignored | same | no | no fixture | A |
| X4 | DATA ICC profile | converted under `UseRenderedIcc` (reported), otherwise refused | `src/codecs/jxl.rs`, `jxl_oxide.rs` | matrix | `supported_matrix::jpeg_xl_rgb_needs_the_rendered_icc_policy_on_both_readers` | C |
| X5 | Extra channels other than one alpha, **libjxl-rs** | refused (`UnsupportedLayout`) before pixels | `ferrisma-jxl-decoder` `Decoder::info` | matrix | `supported_matrix::jpeg_xl_float_alpha_spot_and_extra_channels_are_rejected` | C |
| X6 | Depth, selection mask, black next to RGB, CFA, thermal and optional extra channels, **jxl-oxide** | accepted; the channels are left out of the pixel stream (a black channel next to RGB is lost, so the colors change) | jxl-oxide 0.12.6 `PixelFormat`; `src/codecs/jxl_oxide.rs` sees no channel count | no (matrix: "jxl-oxide reports some extra-channel files as unspecified alpha") | `silent_drops::jxl_oxide_drops_extra_channels_that_libjxl_rs_refuses_today` (`extra-channel-type-{1,3,4,5,6,16}.jxl`) | A |
| X7 | Spot colors, **jxl-oxide** | accepted; composited into the RGB samples (`render_spot_color`, default on for color images), an unrequested change of the color values | jxl-oxide `fb.rs` | no | same test (`extra-channel-type-2.jxl`) | A |
| X8 | Several alpha channels, **jxl-oxide** | the first is rendered, the others dropped | jxl-oxide `fb.rs` ("Find alpha") | no | `silent_drops::jxl_oxide_keeps_only_the_first_of_several_alpha_channels_today` | A |
| X9 | Reserved extra-channel type that libjxl rejects | libjxl-rs: `MalformedInput`; jxl-oxide decodes it and drops the channel | — | no | `silent_drops::jxl_oxide_accepts_a_reserved_extra_channel_type_that_libjxl_rejects_today` (`extra-channel-rejected-15.jxl`) | A |
| X10 | Layers, patches, blending, reference frames | composited into the displayed image by both readers | decoders | jxl-oxide adapter tests | `codecs::jxl_oxide::tests::animated_input_is_rejected_rather_than_reduced_to_its_first_frame` ("several frames composited into one still keyframe") | D |
| X11 | Preview frame | ignored | decoders | no | no fixture | A |
| X12 | `intrinsic_size` (display size hint) | ignored | decoders | no | no fixture | A |
| X13 | Animation | refused (`Animation`) | both adapters | matrix | `supported_matrix::jpeg_xl_animation_is_rejected_by_both_readers` | C |
| X14 | Gray, CMYK, float, premultiplied or unspecified alpha, PQ/HLG | refused with a reason | adapters, `cms.rs` | matrix | `supported_matrix` JPEG XL cases | C |
| X15 | Orientation | rendered upright by the decoder | decoders | matrix | `orientation_tests::jpeg_xl_orientations_are_rendered_upright_for_every_crop` | D |
| X16 | Progressive passes, truncation | the full image is decoded; a truncated file is `MalformedInput` with both readers (no partial render) | `drive`; jxl-oxide `render_frame_cropped` | no | `silent_drops::truncated_inputs_are_malformed` | C |
| X17 | Bit depth above 8 (16-bit, 31-bit integer samples) | decoded as float, then 8-bit output | — | matrix, Color policy ("8-bit … rows") | `modular-rgb31.jxl` with libjxl-rs (probe) | B |

Finding, not a silent drop: jxl-oxide panics in a debug build on
`modular-rgb31.jxl` (`jxl-image` 0.13.0 `parse_integer_sample`, `1 << 31`
overflow); the release build wraps to the correct divisor. And jxl-oxide
reports the float layout of `modular-rgb-alpha-float24e7.jxl` as
`MalformedInput` ("straight alpha must be finite…") where libjxl-rs says
`UnsupportedLayout`: a misreported kind, not a hidden one.

### Still AVIF (`avif-read`, libavif/AOM)

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| V1 | `Exif` and `mime` (XMP) items | ignored, dropped; an EXIF orientation inside is not applied (`irot`/`imir` govern) | `src/codecs/avif_meta.rs` reads the primary item's properties only | matrix, Outputs "Metadata" | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels` | B |
| V2 | `colr prof`/`rICC`, `nclx` | ICC converted under `UseRenderedIcc`; `nclx` checked against the AV1 header and beside the ICC | `avif.rs`, `cms.rs` | matrix | `supported_matrix::avif_container_icc_needs_the_rendered_icc_policy` | C |
| V3 | Gain map (`tmap`) | ignored, SDR base read | libavif `imageContentToDecode` default | matrix, AVIF details | `supported_matrix::avif_gain_map_is_ignored_and_the_base_image_is_read` | B |
| V4 | Alpha item with an unknown **essential** property | libavif skips the item (`hasUnsupportedEssentialProperty`); the image is read as **opaque** without an error; the engine checks only the primary item | `avif_meta.rs` `primary_properties`; libavif `read.c` | no | `silent_drops::avif_alpha_item_with_an_unknown_essential_property_is_dropped_today` (patched `abc_color_irot_alpha_irot.avif`) | A |
| V5 | Unknown essential property of the primary item | refused, `UnsupportedEssentialProperty` (since this branch; before, `MalformedInput`) | `avif.rs` `new_bounded` | matrix, compatibility | `supported_matrix::avif_unknown_essential_property_is_refused_as_unsupported` | C |
| V6 | Unknown non-essential properties, unknown `colr` types | ignored, as ISO/IEC 23008-12 permits | `avif_meta.rs` `read_property` (`_ => {}`) | no | `silent_drops::avif_non_essential_unknown_properties_are_ignored_today` (`circle_custom_properties.avif`) | A |
| V7 | `pasp` (pixel aspect ratio) | ignored (same path) | same | no | no fixture | A |
| V8 | Sample-transform (`sato`) alternative with more precision | the 12-bit primary is read | libavif without its experimental `sato` support | no | `silent_drops::avif_sample_transform_alternative_is_ignored_today` | A |
| V9 | Premultiplied alpha (`prem`) | libavif un-premultiplies into straight RGBA (`rgb.alphaPremultiplied = 0`); JPEG XL premultiplied alpha is refused instead | `crates/ferrisma-avif-read` `rgb` | no | no fixture | A |
| V10 | Depth and other auxiliary images in a still | ignored (only alpha is read) | libavif | matrix, AVIF details | no still fixture | B |
| V11 | Grid, `pymd` pyramid | composed grid / primary item at full size | libavif | matrix | `supported_matrix::avif_alpha_and_grid_stills_are_accepted`; `pyramid_pymd.avif` (probe) | D |
| V12 | Sequences, `clap`, `imir` before `irot`, monochrome, PQ/HLG | refused with a reason | `avif.rs` | matrix | `supported_matrix` AVIF cases | C |
| V13 | `irot`/`imir` | applied before crop | `orientation.rs` | matrix | `supported_matrix::avif_irot_and_imir_are_applied_for_all_eight_orientations` | D |
| V14 | 10/12-bit samples | normalized float, then 8-bit output | `avif.rs` `read_region` | matrix | `supported_matrix::avif_eight_ten_and_twelve_bit_in_every_chroma_layout_are_accepted` | B |
| V15 | Limited range | expanded to full range by libavif (value-preserving) | libavif | `avif.rs` module doc | — | D |
| V16 | Chroma upsampling | nearest neighbor, forced by the wrapper | `ferrisma-avif-read` `rgb` | `avif.rs` module doc only | — | B |
| V17 | Truncation, corrupted payload | `MalformedInput` | `bridge_error` | matrix | `silent_drops::truncated_avif_is_malformed`; 16 inverted bytes in `paris_icc_exif_xmp.avif` (probe) | C |

## Outputs

| # | Output | Item | Today | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| O1 | AVIF, both encoders | source EXIF/XMP/ICC | not written; sRGB signaled (libavif `nclx` 1/13/6 full; rav1e AV1 header only) | matrix, Outputs | `supported_matrix::avif_output_drops_source_metadata_and_writes_upright_pixels` | B |
| O2 | AVIF libavif 10-bit (default) | precision | the 10-bit file carries 8-bit rows | matrix ("8-bit straight-alpha RGBA rows") | `libavif_is_the_default_avif_encoder_at_ten_bit_444_with_nclx` | B |
| O3 | AVIF libavif | `quality` fraction | rounded to an integer | rustdoc of `quality` | `libavif_quality_speed_and_depth_ranges` | B |
| O4 | AVIF rav1e | depth, alpha quality | always 10-bit; alpha at fixed quality 80 whatever `quality` says | rustdoc, matrix | `rav1e_writes_ten_bit_without_a_container_colr`, `rav1e_writes_an_alpha_item_for_transparent_input` | B |
| O5 | All | orientation | applied, written upright with no transform property | matrix | `jpeg_xl_output_is_written_upright` and the AVIF/WebP cases | D |
| O6 | All | fully opaque alpha | omitted (no information lost) | matrix | `libavif_writes_an_alpha_item_only_for_transparent_input` | D |
| O7 | WebP lossy | chroma | always 4:2:0; a nondefault `chroma_subsampling` is refused | matrix | `avif_settings_with_webp_output_are_mismatched_options` | B |
| O8 | WebP lossy | `speed` | 2/3, 5/6 and 8/9 give the same method and the same bytes | no (rustdoc promises 1–10) | `silent_drops::webp_lossy_speeds_collapse_in_pairs_today` | A |
| O9 | WebP | metadata | only the engine's sRGB `ICCP`; EXIF/XMP dropped | matrix | `webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp` | B |
| O10 | WebP lossless | color under alpha 0 | libwebp keeps it (`exact`), but the engine has already made it black (P5), so the "lossless" file does not keep hidden color | ADR-0014 | P5 tests | B |
| O11 | JPEG XL, both profiles | source EXIF/XMP | a raw codestream cannot carry them; the matrix does not say so for JPEG XL | no | `silent_drops::source_exif_and_xmp_are_dropped_from_jpeg_xl_output_today` | A |
| O12 | JPEG XL lossy | color under alpha 0, patches | color replaced (`keep_invisible`), patch search off | matrix, rustdoc (fixed on this branch) | `jxl_lossy_tests::text_like_content_encodes_with_patches_off` | B |
| O13 | JPEG XL lossless | samples | exact for the 8-bit rows | ADR-0039 | `jxl_output_tests::sink_round_trips_exactly_through_the_workspace_decoder` | D |

## Pipeline: precision, range, color and alpha

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| P1 | Bit depth | every input reaches the encoder as 8-bit RGBA rows | `fast_execute.rs` `quantize_row` | matrix, Color policy | — | B |
| P2 | Out-of-gamut and over-range values (Display P3, float JPEG XL, Lanczos overshoot) | kept through resampling, clipped to [0, 1] at final quantization; nothing reports how much was clipped | `quantize_row` (`clamp(0.0, 1.0)`) | ADR-0013, ADR-0016 (rule 5 asks for a "visible decision … with output metadata", not delivered) | `tests/icc_photo.rs::photograph_as_display_p3_matches_littlecms_out_of_gamut` | B |
| P3 | ICC profiles outside moxcms' extended matrix-shaper path (LUT profiles) | clamped to the destination gamut inside moxcms, before resampling | `cms.rs` `IccSrgbSource::new` | ADR-0016 (2026-11-06), ADR-0023 | `cms::tests` | B |
| P4 | Rendering intent | moxcms' default `Perceptual` is used for every profile; the profile header's intent is not consulted. ADR-0016 calls intents other than relative colorimetric "open", which reads as if relative colorimetric were used | `cms.rs` (`TransformOptions::default()`) | no (misleading) | no LUT-based fixture | A |
| P5 | Color under alpha 0 | made transparent black | `quantize_row` | ADR-0014 | `tests/resize_oracle.rs` (transparent black), `linear_srgb_tests::all_transparent_pixels_yield_canonical_transparent_black` | B |
| P6 | Alpha precision | 10/12/16-bit or float alpha quantized to 8 bits | `quantize_row` | matrix | — | B |
| P7 | Miscoded `cicp` tag inside an ICC profile | ignored as a description (its PQ/HLG still refuses) | `cms.rs` | ADR-0016 (#17, #41) | `cms::tests` | B |
| P8 | Missing color signal under `AssumeSrgbIfMissing` | sRGB assumed, `color_signal = AssumedSrgb` | `pipeline.rs` `run` | matrix | `tests/color_signal_and_refusals.rs` | C |
| P9 | Display P3 or linear-sRGB CICP | converted, `color_signal = Cicp` | `fast_execute.rs` | matrix | `pipeline_tests` | C |

## Options and the command line

| # | Item | Today | Code | Documented | Test | Class |
| --- | --- | --- | --- | --- | --- | --- |
| Q1 | `output_width`/`output_height` with `transcode_resized` | ignored; the actual size is in `Output` | `pipeline.rs` | rustdoc | `tests/thumbnail_facade.rs` | B |
| Q2 | Default `avif_encoder` with an external backend | ignored, the backend decides | `run` | rustdoc of `transcode_with` | `pipeline_tests` | B |
| Q3 | `jxl_decoder` for non-JPEG XL input | ignored; `decoder_backend` names the reader that ran | `open_source` | no | `silent_drops::jpeg_xl_reader_selection_is_ignored_for_jpeg_input_and_reported` | C |
| Q4 | `kernel_level_ceiling`, `transform_threads` with `F64Reference` | ignored; the report says `Scalar`/`Ceiling` and 1 thread | `run` | rustdoc of `ProcessingReport` | `pipeline_tests` | C |
| Q5 | Thread counts in `Engine::transcode_batch` | replaced with 1 (reported); values above 256, which `transcode` refuses, are accepted and discarded | `batch` | `BatchItem::options` ("overrides … with 1"); the acceptance of invalid values is not | `silent_drops::batch_accepts_thread_counts_that_transcode_rejects_today` | A |
| Q6 | `Engine::new` above 256 | capped, `threads()` reports it | `Engine::new` | rustdoc | — | C |
| Q7 | Repeated or contradictory CLI flags (`--assume-srgb` after `--use-rendered-icc`, two `--jxl-decoder=`, two `--quality=`) | the last one wins silently; only `--avif-encoder`/`--avif-depth` are refused when repeated | `src/cli.rs` `apply_option` | no | `silent_drops::cli_later_flags_silently_replace_earlier_ones_today` | A |
| Q8 | `--version` with other arguments | prints the version, ignores the rest | `is_version_request` | compatibility guide | `cli::tests` | B |
| Q9 | Mismatched encoder settings (`quality` with WebP or JPEG XL, `webp` with AVIF, `--chroma` with WebP) | refused before decoding | `run`, `cli.rs` | matrix | `supported_matrix::avif_settings_with_webp_output_are_mismatched_options`, `jpeg_xl_distance_effort_and_foreign_settings_are_checked_before_decoding` | C |

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
| `src/codecs/jxl_oxide.rs` `rendered_cicp` (`_ => return None`) | no CICP is claimed without an exact H.273 equivalent; the ICC describes the samples | D |
| `src/codecs/avif_meta.rs` `read_property` (`_ => {}`) | unknown properties and `colr` types ignored: V6, V7 | A |

## Part 2: proposal (not implemented; the owner decides)

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
