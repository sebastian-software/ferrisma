# Supported input, output, backend, and feature matrix

**Reviewed: 2026-11-15** (issue #67, base `aba85c54`): every input, output and
limit row was checked against the code and against
[`tests/supported_matrix.rs`](../tests/supported_matrix.rs), which exercises
each claim through the public processing API with a real file from the
repository; the corrections of that review are listed under
[Corrections of 2026-11-15](#corrections-of-2026-11-15). Earlier: 2026-11-12
(issues #67 and #26; base `1593d6a8` plus the default switches of the same
day) and 2026-11-14 (straight-alpha rendered-ICC path #222, lossless JPEG XL
reservation #223, the libjxl-rs decoder's run-time SIMD selection). The
[verification records](#verification-record-2026-11-15-macos-arm64-and-x86-64-under-rosetta-2)
are at the end. This page is the single place that says what
Ferrisma accepts, what it rejects, which backend does the work, and which
build provides it. It does not promise a stable API: the stability tiers come
from the *proposed* [ADR-0040](adr/0040-stable-public-contract-and-compatibility-policy.md)
and the [compatibility guide](compatibility.md).

What a *successful* transcode leaves out is named in the result
(`ProcessingReport::discarded`, CLI `discarded=`; [ADR-0042](adr/0042-no-silent-loss-refuse-or-report.md)
D2, see [Discarded report](#discarded-report)); everything it drops,
ignores or changes, reported or not, is listed with its test in the
[silent-drop audit of 2026-11-15](silent-drops-audit-2026-11-15.md),
the inventory of ADR-0042.

Every row names the test or fixture that supports it. A claim without a named
test is marked **untested** and is a statement about the code path only. Header
inspection alone is never treated as proof that a file decodes or is
supported. A name of the form `supported_matrix::<case>` is a case of
`tests/supported_matrix.rs`: it runs `transcode` (or `inspect` on an engine
output) with a real file, a supported case decodes and encodes completely, and
a rejected case asserts the public `ProcessingErrorKind`, the engine's
reason in the message **and** the typed `ProcessingError::refusal_reason()`
(since 2026-11-15, issue #212; `None` for malformed data, limits and invalid
options), because one kind is shared by unrelated causes. Run it with
`cargo test -p ferrisma --features cli --test supported_matrix`.

Two defaults changed on 2026-11-12 under the owner's v1.0 direction, and the
owner confirmed both on 2026-11-13
([ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md) D1):
JPEG XL input defaults to the workspace libjxl-rs reader
([ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)), the only JPEG XL
reader since 2026-11-15 (owner decision; `jxl-oxide` left the runtime and is a
test-only cross-check), and AVIF output to
the workspace libavif/AOM writer
([ADR-0037](adr/0037-selectable-workspace-avif-encoder.md)), the only AVIF
writer since 2026-11-15 (owner decision; rav1e left the engine). The default
AVIF quality (#216) and lossy d1.0 as the JPEG XL output default are
still **pending owner confirmation** (ADR-0041 O5).

## Cargo features and builds

`default = []`: without features the engine has no codec. Each feature adds
one adapter. Since 2026-11-15 every format has one reader and one writer
(ADR-0043 D5), so no backend follows feature unification any more.

| Feature | Provides | Backend | Default role | Toolchain | Extra prerequisites | Proposed tier (ADR-0040) |
| --- | --- | --- | --- | --- | --- | --- |
| `jpeg-read` | JPEG input | workspace `libjpeg-turbo-rs` | only JPEG reader | stable 1.98 | none | Stable |
| `webp-read` | still WebP input | workspace `libwebp-rs` | only WebP reader | stable 1.98 | none | Stable |
| `webp-write` | still WebP output | workspace `libwebp-rs` | only WebP writer | stable 1.98 | none | Stable |
| `jpeg-xl-read` | JPEG XL input | workspace `libjxl-rs` via `ferrisma-jxl-decoder` | only JPEG XL reader (since 2026-11-15; this name selected `jxl-oxide` before, and the workspace reader was `jpeg-xl-own-read`) | stable 1.98 | none | Stable as capability |
| `avif-read` | still AVIF input | workspace libavif/AOM via `ferrisma-avif-read` | only AVIF reader | pinned nightly | native macOS ARM64 or GNU Linux x86-64 | Experimental |
| `avif-write` | AVIF output | workspace libavif/AOM via `ferrisma-avif-write` | only AVIF writer (since 2026-11-15; this name selected `rav1e` 0.8 + `avif-serialize` before, and the workspace writer was `avif-own-write`) | pinned nightly | native macOS ARM64 or GNU Linux x86-64 | Experimental until it builds on stable (working decision 2026-11-15) |
| `jpeg-xl-write` | JPEG XL output: lossy d1.0 (default) or lossless | workspace `libjxl-rs`: public encoder through `ferrisma-jxl-encoder` (lossy), fixed-profile entry point (lossless) | only JPEG XL writer; in `cli` since 2026-11-12 | stable 1.98 | none | Experimental |
| `icc` | rendered-ICC → sRGB conversion | `moxcms` 0.9 | needed for `UseRenderedIcc` / `--use-rendered-icc` | stable 1.98 | none | Stable |
| `cli` | the `ferrisma` binary | `jpeg-read`, `webp-read`, `jpeg-xl-read`, `avif-read`, `avif-write`, `webp-write`, `jpeg-xl-write`, `icc` | — | pinned nightly | native macOS ARM64 or GNU Linux x86-64 | Stable command line, experimental backends |

The pinned nightly is `nightly-2026-09-07` from
[`rust-toolchain.toml`](../rust-toolchain.toml); the generated AVIF code needs
`extern_types` and `c_variadic`. Stable Rust 1.98 is the declared
`rust-version`. The stable-compatible set is the one CI's `minimum-rust` job
checks: `jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-write,icc`.
**Known cost of the 2026-11-15 removal of rav1e:** a stable-toolchain build
can neither read nor write AVIF (before, it wrote AVIF through rav1e) until
the workspace AVIF port builds on stable Rust. NASM is no longer needed.

Evidence: CI `rust` job checks each feature alone (`cargo check
--no-default-features --features <feature>` for every feature above);
`minimum-rust` checks the stable set with `cargo +1.98.0`, including
`jpeg-xl-write` since 2026-11-12 (before, a local check only);
the local run in [Verification record](#verification-record-2026-11-12-macos-arm64)
repeats both and confirms that `avif-own-write` (now `avif-write`) fails on stable with `E0554`.

## Inputs

All inputs are still images. Every decoder's emitted samples, not the file's
original profile, define the color meaning (ADR-0015). Orientation is applied
before crop; crop coordinates are upright display coordinates (ADR-0019).

| | JPEG | still WebP | JPEG XL | still AVIF |
| --- | --- | --- | --- | --- |
| Feature | `jpeg-read` | `webp-read` | `jpeg-xl-read` (libjxl-rs) | `avif-read` |
| Samples delivered | 8-bit RGB (adapter requests RGB; grayscale JPEG is expanded to RGB) | 8-bit RGB or RGBA | F32 RGB or RGBA; the decoder also delivers gray and gray+alpha, which the pipeline refuses under every policy | 8-bit RGB(A); 10/12-bit as normalized float; 4:2:0, 4:2:2 and 4:4:4 at 8, 10 and 12 bits |
| Alpha | none | straight | straight only: premultiplied association is refused under every policy | straight (libavif output) |
| Color evidence | embedded ICC | embedded RGB ICC | `DATA` ICC of the emitted samples | container `nclx`/ICC cross-checked with the AV1 sequence header; reported as rendered CICP |
| Orientation | EXIF 1–8 | EXIF 1–8 | codestream orientation, rendered upright by the decoder | `irot`/`imir` applied by the pipeline |
| Crop access | codec crop request, iMCU column overfetch | lossy: crop from full-width rows; lossless: full reconstruction | **full-frame fallback for every crop** | full-frame fallback |
| Rejected | CMYK/YCCK (`UnsupportedInput`, `UnsupportedLayout`); untagged under `Strict` (`ColorSignalMissing`); ICC without `UseRenderedIcc` (`RenderedIccRequiresOptIn`); EXIF orientation outside 1–8 (`GeometricTransform`, **untested** through the API); malformed ICC; truncated data | animation (`UnsupportedInput`, `Animation`); malformed containers; canvas above the pixel cap | animation (`Animation`); any input without `UseRenderedIcc` (`RenderedIccRequiresOptIn`); grayscale, CMYK, spot colors, float/HDR exponent channels, extra channels (`UnsupportedLayout` under `UseRenderedIcc`; the former jxl-oxide reader accepted some of these files while dropping or compositing their channels until 2026-11-15, [audit](silent-drops-audit-2026-11-15.md) X6 to X9); premultiplied alpha (`AlphaAssociation`); PQ/HLG (`HdrTransfer`); an unknown colour space (`MalformedInput`, no reason, as libjxl; `UnsupportedColorEncoding` on jxl-oxide until 2026-11-15); ICC > 16 MiB (**untested**, code path, no reason) | animation/`avis` (`Animation`), `clap` and `imir` before `irot` (`GeometricTransform`), monochrome (`UnsupportedLayout`), contradictory `nclx` (`ContradictoryColorSignals`), PQ/HLG (`HdrTransfer`), SDR CICP outside sRGB/P3/linear sRGB (`UnsupportedColorEncoding`), CICP-only under `UseRenderedIcc` (`RenderedIccMissing`), an unknown essential property of the primary item (`UnsupportedEssentialProperty`) ([details](#avif-input-transforms-animation-channels-and-caps)) |

The names in parentheses after a kind are the `RefusalReason` that
`refusal_reason()` returns (the [compatibility guide](compatibility.md#refusal-reasons)
lists all twelve); where one input meets several classes, the reason names the
check that refused it first, which can depend on the policy. Every reason is
produced by a fixture in `refusal_reason_tests::every_refusal_reason_is_produced_by_a_fixture`
and `tests/color_signal_and_refusals.rs`.

Evidence per format:

- **JPEG** — `codecs::jpeg::tests::does_not_label_cmyk_samples_as_rgb`
  (`tests/fixtures/cmyk.jpg`), `reports_embedded_icc_without_changing_rgb_samples`,
  `malformed_icc_is_not_treated_as_untagged_color`,
  `exif_orientation_is_reported_and_invalid_values_fail_closed`,
  `cropped_read_rejects_a_truncated_tail`;
  `orientation_tests::jpeg_exif_orientations_match_imagemagick`,
  `every_exif_orientation_matches_the_reference_for_every_crop`;
  `pipeline_tests::exif_rotated_jpeg_is_cropped_in_display_coordinates`;
  `tests/decoder_work.rs::jpeg_crop_reports_imcu_column_overfetch_and_whole_entropy_pass`.
  Through the public API: `supported_matrix::jpeg_rgb_is_accepted_when_srgb_is_assumed`,
  `jpeg_untagged_is_rejected_under_the_strict_policy`,
  `jpeg_cmyk_is_rejected_as_a_layout` (`decoder pixel or color layout is not
  supported`), `jpeg_grayscale_is_expanded_to_rgb`
  (`codecs/libjxl-rs/tests/data/ycbcr-jpeg-gray.jpg`), `jpeg_truncated_is_malformed`,
  `jpeg_embedded_icc_needs_the_rendered_icc_policy` (libavif's
  `paris_exif_xmp_icc.jpg`), `jpeg_exif_orientations_are_applied_before_crop`
  (all eight orientation fixtures).
  12-bit, 16-bit and lossless JPEG are outside the port's scope (**untested**
  in the engine). JPEG has no animation; MPF secondary images are not read.
- **WebP** — `codecs::webp::tests::keeps_rgb_icc_and_reports_source_orientation`,
  `rejects_malformed_animation_and_excessive_canvas_before_output`,
  `retained_libwebp_reference_matches_full_and_cropped_rgb`;
  `webp_pipeline_tests::straight_alpha_webp_reaches_avif`,
  `still_webp_reaches_avif_and_untagged_color_requires_policy`;
  `tests/animated_input.rs::animated_webp_is_rejected`
  (`tests/fixtures/decoder-work/animated.webp`);
  `tests/decoder_work.rs::lossy_webp_crop_decodes_full_width_rows_from_the_top`,
  `lossless_webp_crop_is_small_but_reconstruction_is_not`. Through the public
  API: `supported_matrix::webp_lossy_still_is_accepted`,
  `webp_exif_orientation_is_applied_lossy_and_lossless`,
  `webp_animation_is_rejected_without_a_first_frame_fallback`,
  `webp_straight_alpha_input_keeps_alpha` (an engine-written transparent WebP
  read back through its embedded ICC), `webp_canvas_above_the_pixel_limit_is_rejected_from_the_header`.
- **JPEG XL** — the only reader (libjxl-rs): `pipeline_tests::jpeg_xl_icc_is_explicitly_transformed_and_encoded_from_a_full_frame`;
  `tests/decoder_work.rs::libjxl_rs_crop_reports_the_full_frame_fallback`;
  `ferrisma_jxl_decoder` unit tests `preserves_grayscale_and_alpha_association`,
  `rejects_layouts_outside_the_engine_contract_before_pixels`,
  `checks_pixel_limit_before_output`; `jxl_color_tests::rendered_icc_reproduces_known_samples_for_sdr_families`,
  `hdr_and_unverifiable_encodings_are_rejected`,
  `the_reader_agrees_with_jxl_oxide_on_xyb_rendered_into_display_p3` (jxl-oxide
  as a test-only cross-check);
  `orientation_tests::jpeg_xl_orientations_are_rendered_upright_for_every_crop`,
  `lossy_jpeg_xl_orientations_and_crops_through_libjxl_rs`;
  `tests/animated_input.rs::animated_jpeg_xl_is_rejected`;
  `hardening_tests::workspace_jxl_unknown_colour_space_is_a_controlled_error`,
  `public_transcode_rejects_unknown_jxl_colour_space` (`MalformedInput`);
  `jxl_alpha_tests::associated_alpha_is_refused` (premultiplied alpha under
  every policy). Through the public API:
  `supported_matrix::jpeg_xl_rgb_needs_the_rendered_icc_policy` (also pins the
  full-frame fallback),
  `jpeg_xl_straight_alpha_is_kept_through_rendered_icc`,
  `jpeg_xl_grayscale_is_rejected_under_every_policy` (`UnsupportedColor` under
  `Strict`/`AssumeSrgbIfMissing`, `UnsupportedInput` "decoder pixel or color
  layout is not supported" under `UseRenderedIcc`, which takes RGB(A) only),
  `jpeg_xl_float_alpha_spot_and_extra_channels_are_rejected`,
  `jpeg_xl_animation_is_rejected`,
  `jpeg_xl_pq_and_hlg_are_rejected`,
  `jpeg_xl_pixel_limit_is_checked_before_decoding`. CMYK JPEG XL has no
  fixture in the repository; its rejection rests on the external conformance
  run below.
  External: the pinned JPEG XL conformance corpus passes 27/27 level-10 and
  24/24 level-5 entries on macOS ARM64; through the engine wrapper 32 accepted
  cases pass and 8 (animation, CMYK, spot colors, float samples) are rejected
  before pixel allocation ([record](../codecs/libjxl-rs/docs/CONFORMANCE.md)).
  Until 2026-11-15 these cases also ran on the jxl-oxide reader; its own
  tests (region requests, its allocation bound, its refusals) left with it.
- **AVIF** — `codecs::avif::tests::pinned_grid_and_twelve_bit_still_images_read_through_engine`,
  `explicit_nclx_ten_bit_rgb_is_normalized_and_reported_as_srgb`,
  `container_signal_that_contradicts_the_bitstream_is_rejected`;
  `codecs::avif_meta_tests::hdr_signals_are_reported_and_never_treated_as_srgb`,
  `records_mirror_before_rotation_association_order`,
  `rejects_ambiguous_and_unknown_essential_properties`,
  `oversized_icc_property_is_rejected_before_copy`;
  `orientation_tests::avif_irot_imir_match_libheif`,
  `avif_rotation_and_mirror_compose_in_association_order`;
  `tests/animated_input.rs::animated_avif_is_rejected`;
  `hardening_tests::upstream_libavif_corpus_returns_controlled_public_results`
  (every upstream libavif test file under all three color policies; pins
  `clap_irot_imir_non_essential.avif` and `colors-animated-8bpc.avif` as
  `UnsupportedInput`, the grid/alpha stills as accepted, and
  `colors_hdr_rec2020.avif`/`seine_hdr_rec2020.avif` as `UnsupportedColor`);
  `ferrisma_avif_read` unit tests `twelve_bit_native_fixture_emits_sixteen_bit_rgb`,
  `animation_is_not_accepted_as_a_still_image`,
  `coded_pixel_limit_reports_exact_sequence_geometry`. Through the public API:
  see [AVIF input](#avif-input-transforms-animation-channels-and-caps).
  Argon AV1 conformance was not run (ADR-0025); the reader relies on the
  reference-origin route.

### AVIF input: transforms, animation, channels and caps

Checked in `AvifSource::new_bounded` (`src/codecs/avif.rs`) from the container
and the AV1 sequence header, before any pixel is decoded. Fixtures are
libavif's (`codecs/libavif-rs/upstream/libavif/tests/data/` and its Android
test assets) and the libheif orientation set (`tests/fixtures/orientation/`).

| Configuration | Behavior | Evidence (`supported_matrix::…` unless noted) |
| --- | --- | --- |
| `irot` and `imir`, all eight combinations | supported: reported as the source orientation and applied by the pipeline before crop; crops are upright display coordinates | `avif_irot_and_imir_are_applied_for_all_eight_orientations` (`avif-1…8.avif`, display size pinned by the crop bounds); `orientation_tests::avif_irot_imir_match_libheif` (pixels against libheif) |
| `imir` associated before `irot` | `UnsupportedInput`, `GeometricTransform`, "orientation, mirroring or clean-aperture transforms are not supported yet" | `avif_imir_before_irot_is_rejected` (`avif-5.avif` with the two `ipma` associations swapped); `codecs::avif_meta_tests::records_mirror_before_rotation_association_order` |
| `clap` (essential or not) | `UnsupportedInput`, `GeometricTransform`, same message; there is no implicit crop | `avif_clean_aperture_is_rejected` (`clap_irot_imir_non_essential.avif`, Android `blue-and-magenta-crop.avif`); libavif's `clop_irot_imor.avif` (the same file with `clap`/`imir` renamed to unknown non-essential properties) is accepted |
| Unknown essential property of an item the reader uses (primary, grid cells, alpha and its cells) | `UnsupportedInput`, `UnsupportedEssentialProperty`, "an essential container property is not supported: '…'" (since 2026-11-15; before, `MalformedInput` without a reason, corrected under #67). A broken property association (index outside `ipco`) stays `MalformedInput`. Alpha items and grid cells are checked since 2026-11-15 (ADR-0042 D1; before, libavif skipped such an alpha item and the image came out opaque, a grid cell surfaced as `MalformedInput`); a property of an item the reader never decodes (gain-map image, other auxiliary images) is not a reason to refuse | `avif_unknown_essential_property_is_refused_as_unsupported` (`avif-1.avif` with `pixi` renamed and marked essential), `avif_property_association_outside_ipco_is_malformed`; `codecs::avif_meta_tests::rejects_ambiguous_and_unknown_essential_properties`; `silent_drops::avif_alpha_item_with_an_unknown_essential_property_is_refused`, `avif_grid_cell_with_an_unknown_essential_property_is_refused`, `avif_unknown_essential_property_on_an_unused_item_is_not_a_refusal` |
| Image sequence (`avis`), with or without alpha, depth, audio, EXIF/XMP | `UnsupportedInput`, `Animation`, "animated input is not supported; only still images are decoded"; **no first-frame fallback** | `avif_image_sequences_are_rejected_without_a_first_frame_fallback` (five `colors-animated-*.avif` files and `alpha_video.avif`); `tests/animated_input.rs::animated_avif_is_rejected` |
| Monochrome (4:0:0) at 8, 10, 12 bits | `UnsupportedInput`, `UnsupportedLayout`, "decoder pixel or color layout is not supported" | `avif_monochrome_is_rejected_at_every_depth` (Android `fox.*.monochrome.avif`) |
| Color 4:2:0, 4:2:2, 4:4:4 at 8, 10, 12 bits | supported | `avif_eight_ten_and_twelve_bit_in_every_chroma_layout_are_accepted` (nine Android `fox.*.avif`) |
| Alpha auxiliary image | supported, straight | `avif_alpha_and_grid_stills_are_accepted` (`abc_color_irot_alpha_irot.avif`, alpha read back from the WebP output) |
| Grid stills | supported | same test (`color_grid_alpha_grid_shared_tiles.avif`, `sofa_grid1x5_420.avif`) |
| Gain map (`tmap`) | ignored: the SDR base image is read, no tone mapping; reported as `Discarded::GainMap` | `avif_gain_map_is_ignored_and_the_base_image_is_read` (`seine_sdr_gainmap_srgb.avif` under `Strict`) |
| Other extra channels | only alpha is read; depth auxiliary images occur only in the rejected sequence fixture (**untested** for stills); unknown non-essential properties and `pasp` of a used item are ignored and reported as `Discarded::UnknownContainerData`; a `sato` alternative is ignored without a signal ([audit](silent-drops-audit-2026-11-15.md), V6 to V8) | `discarded_report::avif::avif_properties_the_engine_does_not_interpret_are_reported`; `silent_drops::avif_sample_transform_alternative_is_ignored_today` |
| Constant-luminance BT.2020 matrix (code 10) | `UnsupportedInput`, `UnsupportedLayout` (**untested**, code path) | `codecs::avif_meta_tests::contradictory_container_and_bitstream_signals_fail_closed` (synthetic sequence header, internal error only) |
| Container `nclx` that contradicts the AV1 sequence header | `UnsupportedColor`, `ContradictoryColorSignals`, "container color information contradicts the coded bitstream" | `codecs::avif::tests::container_signal_that_contradicts_the_bitstream_is_rejected` (engine-written file, patched); `tests/color_signal_and_refusals.rs::contradictory_avif_color_signals_are_refused_with_their_reason` (`colors_sdr_srgb.avif` with its `nclx` matrix patched to 1) |
| PQ (`colors_hdr_rec2020.avif`, `seine_hdr_rec2020.avif`) | `UnsupportedColor`, `HdrTransfer`, under every policy | `avif_pq_is_rejected_under_every_policy` |
| SDR CICP outside sRGB, Display P3 and linear sRGB, no ICC (`colors_text_wcg_sdr_rec2020.avif`, BT.2020 SDR) | `UnsupportedColor`, `UnsupportedColorEncoding` under `Strict`/`AssumeSrgbIfMissing`; `RenderedIccMissing` under `UseRenderedIcc` | `tests/color_signal_and_refusals.rs::sdr_cicp_outside_the_supported_families_is_refused_with_its_reason` |
| Container ICC | needs `UseRenderedIcc` (`RenderedIccRequiresOptIn` otherwise) | `avif_container_icc_needs_the_rendered_icc_policy` (`paris_icc_exif_xmp.avif`) |
| Signalled sRGB | passes `Strict`; refused under `UseRenderedIcc`, which needs a profile (`UnsupportedColor`, `RenderedIccMissing`) | `avif_signalled_srgb_passes_the_strict_policy` (`colors_sdr_srgb.avif`); `tests/color_signal_and_refusals.rs::rendered_icc_policy_without_a_profile_is_refused_with_its_reason` |
| Rendered size, then coded sequence size, above `max_input_pixels` | `InputLimitExceeded` before decoding | `avif_pixel_limit_checks_the_rendered_and_the_coded_size`; `ferrisma_avif_read::coded_pixel_limit_reports_exact_sequence_geometry` |
| ICC property above the parser's cap | `UnsupportedColor` before the copy, no `RefusalReason` | `codecs::avif_meta_tests::oversized_icc_property_is_rejected_before_copy` (synthetic) |

## Discarded report

`ProcessingReport::discarded` (experimental; [ADR-0042](adr/0042-no-silent-loss-refuse-or-report.md)
D2, working decision 2026-11-15, delegated, revisable by the owner) names
each class of input information a successful transcode left out of a
correct picture, only when it applied to this input; the CLI's `report:`
line ends with `discarded=` and the names, comma-separated, or `none`.
Every class has a test with a fixture that has it and one that does not
(`tests/discarded_report.rs`).

| `Discarded` | Reported when | Inputs |
| --- | --- | --- |
| `GainMap` | a gain map exists and the base image was read | AVIF `tmap`, JPEG MPF secondary image, JPEG XL `jhgm` |
| `Exif` | EXIF exists (its orientation may have been applied) | JPEG, WebP, AVIF item, JPEG XL box |
| `Xmp` | XMP exists, including extended XMP | JPEG, WebP, AVIF `mime` item, JPEG XL `xml ` |
| `OtherMetadata` | IPTC, comments, JUMBF, other JPEG application segments, a JFIF density other than 1:1 aspect, other AVIF `mime`/`uri ` items, JPEG XL `jumb` | JPEG, AVIF, JPEG XL |
| `IccProfile` | an embedded profile, converted under `UseRenderedIcc` and not written | JPEG, WebP, AVIF, ICC-signalled JPEG XL |
| `UnknownContainerData` | data the format lets a reader ignore: AVIF properties of a used item the engine does not interpret (`pasp`, `clli`, custom), unknown WebP chunks, unknown JPEG XL boxes | AVIF, WebP, JPEG XL |
| `JpegReconstructionData` | a `jbrd` box | JPEG XL |
| `Preview` | a preview frame | JPEG XL |
| `HiddenColor` | an input pixel of the processed region with alpha exactly 0 and nonzero color (written as transparent black, ADR-0014) | any RGBA input |
| `Unpremultiplied` | premultiplied alpha converted to straight by libavif (`prem`) | AVIF |
| `PrecisionReduced` | more than 8 bits per color or alpha sample reached the 8-bit rows | AVIF, JPEG XL |

Not reported, and documented here instead (open under ADR-0042 D6): values
clipped to [0, 1] at quantization or inside moxcms' LUT path (not
detectable cheaply and exactly apart from resampling overshoot), JPEG XL
`intrinsic_size`, an XMP-only JPEG orientation and an AVIF `sato`
alternative.

## Color policy

| `InputProfilePolicy` / CLI | Behavior | Evidence |
| --- | --- | --- |
| `Strict` (default, no flag) | Accept only decoder-reported sRGB (for example signalled sRGB AVIF); untagged or ICC-tagged input fails with `UnsupportedColor` | `pipeline_tests::missing_color_information_requires_an_explicit_override`, `signalled_srgb_avif_transcodes_under_the_strict_profile_policy` |
| `AssumeSrgbIfMissing` / `--assume-srgb` | Treat input as sRGB **only** when it reports neither ICC nor CICP; an ICC-tagged file still needs `--use-rendered-icc` | same tests; upstream corpus pins `paris_icc_exif_xmp.avif` as `UnsupportedColor` without `UseRenderedIcc` |
| `UseRenderedIcc` / `--use-rendered-icc` | Convert the decoder's rendered RGB ICC to the working space through `moxcms` (`icc`) with the **relative colorimetric** intent, set explicitly (owner decision 2026-11-15, [ADR-0016](adr/0016-working-color-space-and-transform-boundaries.md); matrix/TRC profiles are not affected by the intent, LUT profiles use their `A2B1` table); opaque RGB and RGB with straight alpha (the alpha sample bypasses the transform); premultiplied or unspecified alpha is rejected | `tests/icc_photo.rs::real_srgb_icc_photograph_matches_littlecms`, `photograph_as_display_p3_matches_littlecms_out_of_gamut`; `tests/resize_oracle.rs::icc_tagged_straight_alpha_input_converts_color_and_keeps_alpha`; `cms::tests::malformed_and_hostile_profiles_are_controlled_errors`; `cms::tests::lut_profiles_convert_with_the_relative_colorimetric_table` |
| PQ, HLG, unverifiable or contradictory signals | Always `UnsupportedColor` (`HdrTransfer`, `CicpBesideIcc`, `ContradictoryColorSignals`, `UnsupportedColorEncoding`); never reinterpreted as sRGB | `jxl_color_tests::hdr_and_unverifiable_encodings_are_rejected`; `codecs::avif_meta_tests::contradictory_container_and_bitstream_signals_fail_closed`; `color_contract_tests::two_present_signals_require_explicit_precedence_even_when_both_look_srgb` |

Output is always SDR sRGB, 8-bit straight-alpha RGBA rows handed to the
encoder (ADR-0024, ADR-0030). HDR, tone mapping and wide-gamut output are not
supported.

## Outputs

| | AVIF, libavif/AOM (the only AVIF writer; rav1e until 2026-11-15) | still WebP | JPEG XL |
| --- | --- | --- | --- |
| Feature / selection | `avif-write`; `.avif`; `avif_depth` / `--avif-depth=` | `webp-write`; `.webp` | `jpeg-xl-write`; `.jxl`; `JpegXlOptions::Lossy { distance }` (default 1.0, `--jxl-distance=`) or `Lossless { effort: 2 }` (`--jxl-lossless`) |
| Bit depth | 10 (default) or 8 (`--avif-depth=8`) | 8 | 8 |
| Chroma | 4:4:4 (default) or 4:2:0 | lossy YUV 4:2:0 / lossless ARGB | lossy: XYB VarDCT, no subsampling, patches off (equals `cjxl --patches=0 -e 7`); lossless: RGB Modular |
| Quality | `quality` 0–100, rounded, 64 quantizer steps | `WebpOptions::Lossy { quality 1–100, speed }` or `Lossless { level 0–9 }` | lossy: Butteraugli distance 0.05 to below 3.0; `--quality` is rejected |
| Speed | 0–10 (`cpu-used`) | lossy speed 1 (slowest)–10; lossless uses the level | fixed: effort 7 lossy, effort 2 lossless; `--speed` is rejected |
| Alpha | auxiliary item at the color quality, omitted when opaque | straight alpha | unassociated (straight) alpha at 8 bits, coded losslessly in both profiles (lossy since 2026-11-12); lossy replaces color under zero alpha (libjxl's `keep_invisible` default); fully opaque alpha is omitted by the engine in both |
| Color signaling | AV1 header 1/13/6 full range **and** container `colr nclx` | embedded sRGB ICC | enum sRGB (lossy: XYB-encoded) |
| Metadata | source EXIF/XMP/ICC not copied (no `Exif`/`mime` item, no `prof` `colr`); orientation applied, no `irot`/`imir`/`clap` written | source EXIF/XMP dropped, only the engine's sRGB `ICCP` chunk; orientation applied | orientation applied, written as 1 |
| Materialization | full RGBA8 frame plus libavif YUV(A) planes, reserved before encoding; encoded output briefly twice | ARGB frame and lossy planes reserved | lossless: the measured one-shot peak, 76 bytes per pixel of opaque RGB including the input frame (scaled by 4/3 with alpha, derived, not measured), at least the codec planes known from the source path; lossy: input frame plus a source-derived codec estimate; reserved before encoding |
| Determinism | bytes can depend on thread count and host | — | both profiles byte-identical with the C++ encoder on the port's recorded requests (macOS AArch64) |
| Threads | `encoder_threads` 0–256 | serial: `encoder_threads` must be 1 | serial: `encoder_threads` must be 1 |
| `max_output_bytes` | checked after the encoder returns: "encoded output exceeds max_output_bytes" | checked while writing: "managed buffer needs … bytes with … available" | lossy: checked while writing, as WebP; lossless: after the encoder returns, as AVIF |

**One AVIF quality scale since 2026-11-15:** `quality` and `speed` go
unchanged to libavif. A quality tuned for the former rav1e encoder is not
comparable: at equal SSIMULACRA2 on the recorded photographs, rav1e 70/80/90
matched roughly libavif 50/55/78; at `--quality=80` on the 800×533 flower,
rav1e wrote 55,682 bytes (SSIMULACRA2 77.455) and libavif 10-bit 100,512
bytes (88.307) ([evidence](benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md)).
The default `quality` is 75 on libavif's scale; whether that default stays
is open (#216).

Evidence through the public API (`supported_matrix::…`): libavif —
`libavif_is_the_default_avif_encoder_at_ten_bit_444_with_nclx` (encoder name
`libavif-rs/aom`, `av1C` 10-bit 4:4:4, `colr nclx` 1/13/6 full range, no alpha
item when opaque, read back under `Strict`),
`libavif_writes_eight_bit_and_420_on_request`,
`libavif_writes_an_alpha_item_only_for_transparent_input`,
`libavif_quality_speed_and_depth_ranges` (0–100, 0–10, depth 8/10, each
rejection with libavif's reason),
`avif_output_drops_source_metadata_and_writes_upright_pixels`,
`output_byte_limit_applies_at_the_exact_byte_for_avif`,
`avif_depth_conflicts_with_other_outputs` (the rav1e cases and the encoder
selection were removed with rav1e on 2026-11-15); WebP —
`webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp` (`VP8 `/`VP8L`
plus `ICCP`, no `EXIF`/`XMP `, read back through the ICC),
`webp_straight_alpha_output_in_both_profiles` (`ALPH` for lossy),
`webp_quality_speed_and_level_ranges`,
`avif_settings_with_webp_output_are_mismatched_options`; JPEG XL —
`jpeg_xl_output_is_a_raw_codestream_that_reads_back` (the reader, and jxl-oxide as a test-only independent decoder),
`jpeg_xl_output_keeps_straight_alpha_in_both_profiles`,
`jpeg_xl_output_is_written_upright`,
`jpeg_xl_distance_effort_and_foreign_settings_are_checked_before_decoding`
(distance 0.05 and 2.99 accepted, 0.049 and 3.0 rejected, effort 3 rejected,
`quality`/`speed` with JPEG XL output rejected),
`output_byte_limit_applies_at_the_exact_byte_for_webp_and_jpeg_xl`.
The AVIF alpha quality (the color quality) and the materialization rows are
read from the code (`src/own_avif.rs`, `src/webp_sink.rs`, `src/jxl_sink.rs`)
and pinned by the unit tests below, not by the public-API cases.

Evidence: AVIF/libavif — `crates/ferrisma-libavif/tests/encode.rs`
(`encodes_what_ferrisma_produced_with_the_same_color_signals`,
`straight_alpha_survives_the_encode`, `invalid_settings_are_rejected_before_decoding`,
`frame_preflight_counts_odd_chroma_and_alpha_planes`,
`output_limit_is_enforced_after_backend_finish`,
`own_encoder_facade_handles_reference_batch_and_invalid_settings`,
`independent_oriented_avif_reference_reaches_own_encoder`);
`tests/own_avif_cli.rs::cli_writes_avif_through_libavif_at_either_depth`;
`hardening_tests::api_rejects_invalid_quality_and_mismatched_input_format`.
`pipeline_tests::produced_avif_is_accepted_by_an_independent_decoder`
(the `avif_sink_tests` of rav1e were removed with it).
The signaling and alpha-quality differences were observed with FFmpeg/libdav1d,
libheif and rav1d in the evidence record above. WebP —
`webp_output_tests::still_webp_outputs_decode_with_straight_alpha_and_srgb_metadata`,
`webp_output_frame_and_file_limits_fail_before_retaining_result`,
`lossy_converted_planes_are_reserved_before_encoding`. JPEG XL —
`jxl_output_tests::sink_round_trips_exactly_through_the_workspace_decoder`,
`sink_round_trips_exactly_through_jxl_oxide`,
`oriented_input_is_written_upright_with_orientation_one`,
`rejects_requests_outside_the_admitted_profile_before_allocating`,
`facade_rejects_mismatched_and_disabled_settings_before_decoding`,
`input_frame_and_measured_one_shot_peak_are_reserved`; lossy —
`jxl_lossy_tests::lossy_output_decodes_within_the_pinned_error` (one-shot and
streaming), `the_workspace_decoder_agrees_with_jxl_oxide`,
`photograph_thumbnail_decodes_near_the_finished_rows`,
`oriented_input_is_written_upright_with_orientation_one`,
`real_alpha_encodes_and_opaque_alpha_is_omitted_by_the_sink`,
`lossy_alpha_decodes_exactly_and_color_within_the_pinned_error` (one-shot and
streaming), `the_workspace_decoder_reads_straight_alpha_exactly`,
`transparent_input_encodes_with_exact_alpha`,
`text_like_content_encodes_with_patches_off`,
`text_like_input_encodes_with_patches_off`,
`distances_outside_the_admitted_range_are_rejected_before_allocating`,
`input_frame_and_codec_estimate_are_reserved`,
`the_output_limit_applies_at_the_exact_byte`;
`tests/jxl_cli.rs::transparent_and_text_like_input_encode_to_lossy_jpeg_xl_by_default`;
the `ferrisma-jxl-encoder` unit tests
([ADR-0039](adr/0039-jpeg-xl-output-through-the-workspace-encoder.md)).
Lossy output always turns libjxl's patch search off, so text-like content
encodes; its size cost against libjxl's default is not measured. Since
2026-11-13 the engine reads its own transparent lossy output back with
`--use-rendered-icc`: the rendered-ICC stage accepts RGBA with straight
alpha, and the reader declares straight alpha only when the alpha channel
is not associated (#222,
[ADR-0015](adr/0015-typed-output-color-signals.md)). Under the default
`Strict` policy that output is still refused; whether XYB with enum sRGB
should pass it is an open owner question in ADR-0015. Evidence:
`jxl_alpha_tests::rendered_icc_reads_engine_written_straight_alpha`,
`strict_policy_still_refuses_engine_written_jpeg_xl`,
`associated_alpha_is_refused`.

The lossless reservation follows the measured one-shot peak since 2026-11-13
(#223, ADR-0039 amendment): under the default 256 MiB `--budget-mb` the
largest lossless JPEG XL output falls from about 9.5 MP to about 3.5 MP, and
larger requests fail with `BudgetExceeded` (`ResourceExhausted`, "managed
buffer needs …") before encoding instead of exceeding the budget in opaque
codec memory (`supported_matrix::default_budget_rejects_lossless_jpeg_xl_above_about_three_and_a_half_megapixels`:
a 2000×2000 request). A measured peak with alpha and
the lossy estimate are still open (#223). JPEG output does not exist and is
not planned.

## Limits and memory

| Option / CLI | Default | Checked | Evidence |
| --- | --- | --- | --- |
| `max_input_bytes` / `--max-input-mb` (MiB) | 64 MiB | before any codec starts (`InputLimitExceeded`, "compressed image exceeds max_input_bytes"); zero is `InvalidOptions`; CLI stream read is capped | `supported_matrix::input_byte_limit_accepts_the_exact_size_and_rejects_one_byte_less`, `zero_input_limits_are_invalid_options`; `pipeline_tests::compressed_bytes_and_decoded_pixel_caps_reject_before_processing`; `cli::tests::accepts_input_at_the_limit_and_rejects_one_byte_more` |
| `max_input_pixels` / `--max-input-mpx` (10^6 px) | 100,000,000 | from headers before pixel allocation (`InputLimitExceeded`): WebP, AVIF and libjxl-rs in the adapter ("input has N pixels, exceeding …"; AVIF also checks the coded sequence size), JPEG by the facade after the header ("image has N pixels; configured limit is M") | `supported_matrix::jpeg_pixel_limit_is_checked_after_the_header`, `webp_canvas_above_the_pixel_limit_is_rejected_from_the_header`, `jpeg_xl_pixel_limit_is_checked_before_decoding`, `avif_pixel_limit_checks_the_rendered_and_the_coded_size`; `ferrisma_avif_read::coded_pixel_limit_reports_exact_sequence_geometry`; `ferrisma_jxl_decoder::checks_pixel_limit_before_output` |
| `max_managed_bytes` / `--budget-mb` | 256 MiB | engine buffers, known encoder frames, the lossless JPEG XL one-shot peak (`ResourceExhausted`, "managed buffer needs …") | `supported_matrix::managed_budget_exhaustion_is_reported`, `default_budget_rejects_lossless_jpeg_xl_above_about_three_and_a_half_megapixels`; `tests.rs::insufficient_budget_releases_all_owned_buffers`; `avif_sink_tests::the_encoder_frame_is_charged_to_the_budget` |
| `max_output_bytes` / `--max-output-mb` | 256 MiB | `ResourceExhausted` in every case. WebP and lossy JPEG XL while writing ("managed buffer needs …"); AVIF and lossless JPEG XL after the encoder returns ("encoded output exceeds max_output_bytes"); zero is `InvalidOptions` | `supported_matrix::output_byte_limit_applies_at_the_exact_byte_for_webp_and_jpeg_xl`, `output_byte_limit_applies_at_the_exact_byte_for_both_avif_encoders`; `webp_output_tests::exact_final_byte_limit_accepts_opaque_lossy_alpha_and_lossless`; `jxl_output_tests::encoded_output_limit_applies_at_the_exact_byte`, `jxl_lossy_tests::the_output_limit_applies_at_the_exact_byte`; `ferrisma-libavif` `output_limit_is_enforced_after_backend_finish` |
| threads | `transform_threads` 0 (all cores), `encoder_threads` 1; at most 256; WebP and JPEG XL output need `encoder_threads` 1 | before decoding (`InvalidOptions`) | `supported_matrix::thread_counts_above_256_and_parallel_webp_encoding_are_invalid`; `pipeline_tests::encoder_threads_trade_determinism_for_latency_explicitly` |

Codec-internal allocations (all four decoders, all encoders) are opaque: the
report sets `opaque_decoder_memory`/`opaque_encoder_memory`, and there is no
process-wide memory cap. Full-frame decoders (libjxl-rs, AVIF, lossless WebP
internals) need the whole frame for any crop.

## Native targets

| Target | Engine features on stable | `avif-read`, `avif-write`, `cli` | Verified by |
| --- | --- | --- | --- |
| macOS ARM64 | yes | yes (pinned nightly) | local run below; CI `native-adapters` (`macos-15`) |
| GNU Linux x86-64 | yes | yes (pinned nightly) | CI `rust`, `minimum-rust`, `workspace`, `codecs` (`ubuntu-24.04`); **not re-run for this branch** |
| GNU Linux ARM64 | codec-free engine and kernels tested in CI | not supported | CI `kernels-arm` (`ubuntu-24.04-arm`: `cargo test -p ferrisma -p ferrisma-kernels --all-targets`, no codec features) |
| Windows, other targets | not verified | not supported | out of v1.0 scope (#158, #159) |

## SIMD instruction-set levels (v1.0)

**Reviewed: 2026-11-13** (decision D6; [ADR-0020](adr/0020-kernel-crate-and-isa-dispatch.md)
amendment of that date). v1.0 executes SIMD up to **AVX2 on x86-64**
(including the SSE2/SSSE3/SSE4.1 kernels an original selects on an AVX2
machine) and up to **baseline NEON on AArch64**. AVX-512 (any subset), SVE,
SVE2, NEON DotProd, I8MM, BF16 and higher are post-v1.0: code that exists for
them stays in the tree but does not run unless the opt-in of its component is
used. Every place in the workspace that selects code at run time:

| Component | Run-time selection | Above-scope code in the tree | v1.0 default | Opt-in | Upstream equivalent of the default |
| --- | --- | --- | --- | --- | --- |
| Engine kernels (`crates/ferrisma-kernels`) | `src/dispatch.rs` `supports`/`detect_machine`; `src/lib.rs` `select` | `Avx512` (`dispatch.rs` `mod avx512`, `lanes.rs`); `Sve2` detected only | `Avx2` / `Neon` (`DEFAULT_CEILING`); reason `DefaultCeiling` when capped | `kernel_level_ceiling = Some(KernelLevel::Avx512)`, `--kernel-level=avx512`, `Kernels::with_ceiling(IsaLevel::Avx512)` | first-party |
| skcms in `libjxl-rs` (ICC transforms) | `src/skcms/transform.rs` `cpu_type` and `engine` | SKX engine, `src/skcms/transform_x86.rs` `run_program_skx` (AVX-512F/DQ/CD/BW/VL) | HSW (AVX2/F16C) on AVX-512 CPUs; baseline NEON on AArch64 | Cargo feature `libjxl-rs/skcms-skx` | skcms built with `-DSKCMS_DISABLE_SKX` |
| `libjxl-rs` decoder outside skcms | `src/jxl/hwy/mod.rs` `decoder_default_target` (`Target::detect()` when `DECODER_SELECTS_SIMD`), applied only inside decoder entry points (port ADR 0004, "Scope of the selection") | none: only `HWY_NEON` and `HWY_AVX2` are translated, and `Target::detect` never selects AVX-512 or SVE | NEON on AArch64 since 2026-11-14; AVX2 on x86-64 since `7b48bc81` (AVX2 output can differ between CPU vendors exactly as the original's AVX2 build does); the encoder runs scalar on every target | none for callers; tests use `set_decoder_simd_for_test` and, in oracle builds, `JXL_DECODER_TARGET=scalar` | Highway dispatch with every target above `HWY_NEON`/`HWY_AVX2` in `HWY_DISABLED_TARGETS` |
| AOM (`libavif-rs/crates/aom`) | translated RTCD: `x86_simd_caps()` (generated `src/linux/*/…_rtcd.rs`, reports `HAS_AVX512`), `aom_arm_cpu_caps()` (`src/simd/aom_ports/aarch64_cpudetect.rs`, reports DotProd and I8MM) | none: no AVX-512, DotProd, I8MM or SVE kernel is translated, and the RTCD generator runs with `--disable-avx512` and `--disable-neon_dotprod/neon_i8mm/sve/sve2`, so no table entry dispatches on those flags (`tests/dispatch_scope.rs`) | SSE2/SSE4.1/AVX2; Neon | none (nothing to opt into; `AOM_SIMD_CAPS`/`AOM_SIMD_CAPS_MASK` only lower the selection) | CMake `-DENABLE_AVX512=0` (x86-64); `-DENABLE_NEON_DOTPROD=0 -DENABLE_NEON_I8MM=0 -DENABLE_SVE=0 -DENABLE_SVE2=0` (arm64); at run time `AOM_SIMD_CAPS_MASK=0x1ff` (x86-64) or `0x3` (arm64) |
| dav1d (`libavif-rs/crates/dav1d`) | none: no assembly, `dav1d_init_cpu` is empty | none | scalar | — | `-Denable_asm=false` |
| SharpYUV (`libavif-rs/crates/sharpyuv`, `crates/libsharpyuv-core`) | CPU query translated, SIMD conversion disabled | none | scalar | — | — |
| `libwebp-rs` | `src/dsp/cpu.rs` `vp8_get_cpu_info` | none (libwebp has nothing above `kAVX2`/`kNEON`) | up to AVX2 / NEON | — | — |
| `libjpeg-turbo-rs` | `src/jsimdcpu.rs` `jpeg_simd_cpu_support` | none (libjpeg-turbo has nothing above AVX2/NEON) | up to AVX2 / NEON | — | — |
| rav1e 0.8.1 (third-party; until 2026-11-15 the `avif-write` encoder with `asm`; now only a test-only dev-dependency through `ravif`, without assembly) | `src/cpu_features/x86.rs` in the crate | AVX-512 and AVX512ICL assembly | **not capped by Ferrisma**: AVX-512 hosts ran its AVX-512 assembly | — | set `RAV1E_CPU_TARGET=avx2` in the process environment to cap it |
| moxcms 0.9.1 (third-party, `icc`) | crate-internal | `avx512` feature, not enabled | up to AVX2 / NEON | — | — |
| jxl-oxide 0.12.6 (third-party; a test-only dev-dependency since 2026-11-15, not linked into the engine) | crate-internal | none (its detection stops at AVX2/FMA/SSE4.1/NEON) | up to AVX2 / NEON | — | — |

Comparisons against an original (D4) must use the original with the switch
in the last column. The skcms reference tools (`tools/skcms_transform_reference.py`,
`tools/default_cms_reference.py`) and `tools/check.py` build `libjxl-rs` with
`--all-features`, which includes `skcms-skx`, so on an AVX-512 host they keep
comparing SKX against the unchanged original's SKX; the default HSW selection
is compared on AVX2 hosts. On hosts without AVX-512 (all local hosts,
including Rosetta 2) the default and the opt-in select the same code.

**Reproducibility across x86-64 hosts (issue #239).** Up to 2026-11-12,
resized output of the `f32` executor depended on the host's kernel level: on
GitHub's x86-64 runners, AVX2 and AVX-512 hosts produced different hashes for
some resized outputs (JPEG XL inputs, AVIF outputs, a lossy WebP input), within
ADR-0021's bound (one code after quantization), while full-size outputs and
hosts at the same level agreed ([CI performance record](benchmarks/v1.0-ci-performance-2026-11-12.md),
CI run 43293971731). With the default ceiling, every x86-64 host with AVX2
runs `avx2` and produces the same bytes. An x86-64 host without AVX2 runs
`scalar` and can still differ from AVX2 hosts within the same bound; AArch64
runs `neon`, which is bit-identical to `scalar`. Setting
`--kernel-level=avx2` (or `scalar`) pins the bytes explicitly; `--kernel-level=avx512`
reintroduces the host dependence on purpose. Not yet confirmed on AVX-512
hardware: the next hosted run on an AVX-512 runner must show
`chosen level: avx2 (DefaultCeiling)` in the `ferrisma-kernels` `detect` step
and resized-output hashes equal to those of an AVX2 runner.

## Examples

Supported (output of the debug `cli` build on 2026-11-12, report line
abbreviated):

```sh
# Build identity: version, target, compiled-in features, FERRISMA_BUILD_ID
ferrisma --version          # or -V; takes precedence over every other argument
#   ferrisma 0.1.0
#   target: aarch64-macos
#   features: jpeg-read, webp-read, webp-write, jpeg-xl-read, jpeg-xl-write, avif-read, avif-write, icc
#   build: unknown
# JPEG → AVIF with the libavif/AOM encoder, 10-bit 4:4:4
ferrisma photo.jpg out.avif 64 48 --assume-srgb
#   report: … encoder=libavif-rs/aom
# libavif at 8 bits, 4:2:0
ferrisma photo.jpg out.avif 64 48 --assume-srgb --avif-depth=8 --chroma=420
# ICC-tagged AVIF through the rendered-ICC path
ferrisma paris_icc_exif_xmp.avif out.webp 64 48 --use-rendered-icc
# JPEG XL through its only reader (full-frame fallback)
ferrisma sample.jxl out.avif 6 4 --use-rendered-icc  # decoder_full_frames=1
# Lossless WebP; lossy JPEG XL d1.0, also with alpha (needs --features cli,jpeg-xl-write)
ferrisma photo.jpg out.webp 64 48 --assume-srgb --webp-lossless --webp-level=6
ferrisma photo.jpg out.jxl 64 48 --assume-srgb
```

Rejected, with the error the CLI prints (exit status 1):

| Command | Error | What to do |
| --- | --- | --- |
| `ferrisma photo.jpg out.avif 64 48` (untagged) | `UnsupportedColor` (`ColorSignalMissing`) "unsupported or ambiguous rendered color profile…" | add `--assume-srgb` if that assumption is acceptable |
| `ferrisma sample.jxl out.avif 6 4 --assume-srgb` (ICC-tagged) | `UnsupportedColor` (`RenderedIccRequiresOptIn`) | use `--use-rendered-icc` (needs `icc`) |
| `ferrisma cmyk.jpg out.avif 64 48 --assume-srgb` | `UnsupportedInput` (`UnsupportedLayout`) "decoder pixel or color layout is not supported" | convert CMYK to RGB first |
| `ferrisma animated.webp out.avif 64 48 --assume-srgb` | `UnsupportedInput` (`Animation`) "animated input is not supported…" | extract one frame first |
| `ferrisma colors-animated-8bpc.avif …` | `UnsupportedInput` (`Animation`) | same |
| `ferrisma clap_irot_imir_non_essential.avif …` | `UnsupportedInput` (`GeometricTransform`) "orientation, mirroring or clean-aperture transforms are not supported yet" | remove `clap` or crop explicitly with `--crop` on a file without it |
| `ferrisma colors_hdr_rec2020.avif …` (PQ) | `UnsupportedColor` (`HdrTransfer`) | tone-map to SDR elsewhere; HDR is out of scope |
| `ferrisma photo.jpg out.avif … --quality=101` | `InvalidOptions` | libavif's scale is 0–100 |
| `ferrisma photo.jpg out.webp … --avif-depth=8` | "--avif-depth requires .avif output" | drop AVIF flags for WebP |
| `ferrisma photo.jpg out.avif … --avif-encoder=rav1e` | "unknown option: --avif-encoder=rav1e" (removed 2026-11-15) | drop the flag; libavif is the only AVIF writer |
| `ferrisma photo.jpg out.jxl … --quality=80` | "--quality is unsupported for JPEG XL; use --jxl-distance or --jxl-lossless" | use `--jxl-distance` (lossy) or `--jxl-lossless` |
| `ferrisma photo.jpg out.png …` | "output must have a .avif, .webp or .jxl extension" | choose a supported output |
| `ferrisma photo.jpg out.avif … --max-input-mpx=0` | `InvalidOptions` "input byte/pixel limits must be nonzero" | use a positive limit |

The CLI prints the message, not the reason in parentheses, which is what
`refusal_reason()` returns through the Rust facade. The facade returns the
same `ProcessingErrorKind` values;
[`examples/consumer.rs`](../examples/consumer.rs) maps each kind to a
reject/retry/alert decision and its `--self-check` asserts them.

## Corrections of 2026-11-15

The review against the code and `tests/supported_matrix.rs` changed these
statements of the 2026-11-12/14 matrix to what the code does. None of them
changed engine code; whether the behavior or the earlier text was the defect
is left to the owner.

1. **Grayscale JPEG XL** was listed as delivered (gray, gray+alpha). Both
   readers deliver it, but the pipeline refuses it under every policy:
   `UnsupportedColor` under `Strict` and `AssumeSrgbIfMissing` (the decoder
   reports an ICC profile, not sRGB CICP), `UnsupportedInput` under
   `UseRenderedIcc`, whose transform takes RGB(A) only.
2. **Premultiplied JPEG XL alpha** was listed as accepted ("straight or
   premultiplied"). JPEG XL input needs `UseRenderedIcc`, which refuses
   premultiplied (libjxl-rs) and unspecified (jxl-oxide) association, so only
   straight alpha is transcoded.
3. **`max_output_bytes` for JPEG XL** was "after the encoder returns". That
   holds for lossless; lossy JPEG XL is stopped while writing, like WebP, with
   "managed buffer needs … bytes with … available". Every output-limit
   failure is `ResourceExhausted`; the reason differs by encoder.
4. **Monochrome AVIF** had no named test; it is now pinned at 8, 10 and 12
   bits, as are `imir` before `irot` (previously only a synthetic unit test),
   4:2:2 input (not listed before) and the gain-map behavior (the SDR base
   image is read; not stated before).
5. **The pixel-limit reason** differs by reader: the adapter checks WebP,
   AVIF and libjxl-rs input, the facade checks JPEG and jxl-oxide input after
   the header, with a different message. Both are `InputLimitExceeded` before
   any pixel is decoded.
6. An unknown essential AVIF property is rejected as `MalformedInput`, not
   `UnsupportedInput`; this was not stated before. (Corrected in the code
   later the same day: it is now `UnsupportedInput` with
   `UnsupportedEssentialProperty`, see the AVIF input table.)
7. The CLI's `--version` (`d9146dd0`) was missing from the examples.

Still pending owner decisions, described as today's behavior above: the
default AVIF quality per encoder (#216; 75 on the selected scale), lossy d1.0
as the JPEG XL output default (ADR-0041 O5), and whether the strict policy
should accept transparent XYB JPEG XL written by the engine (#222, ADR-0015;
today refused under `Strict`). Open questions of #250 are not decided here.

## Verification record (2026-11-15, macOS ARM64 and x86-64 under Rosetta 2)

Host: Apple Silicon, macOS (Darwin 27.2.0) arm64, shared with other build
jobs (no timing recorded); pinned `nightly-2026-09-07`, `CARGO_BUILD_JOBS=4`,
branch `v1/matrix` on `aba85c54`.

| Command | Result |
| --- | --- |
| `cargo test -p ferrisma --features cli --test supported_matrix` | pass, 57 tests |
| `cargo test -p ferrisma --features jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-own-read,jpeg-xl-write,icc --test supported_matrix` | pass, 35 tests (the AVIF cases are gated on their adapters) |
| the same with `--target x86_64-apple-darwin` under Rosetta 2 (`ROSETTA_ADVERTISE_AVX=1`, `SDKROOT` set) | pass, 35 tests. The AVIF adapters cannot be built for x86-64 macOS (the generated libaom/dav1d/SharpYUV sources `compile_error!` outside macOS ARM64 and GNU Linux x86-64), so the AVIF cases have no local x86-64 run; GNU Linux x86-64 stays CI's |
| `cargo clippy -p ferrisma --features cli --test supported_matrix --no-deps -- -D warnings`, and with the non-AVIF set | pass |
| `cargo test -p ferrisma --all-targets --features cli` (under the gate lock) | pass: 326 tests in 18 binaries, 0 failed, 1 ignored (pre-existing `#[ignore]`) |

## Verification record (2026-11-12, macOS ARM64)

Host: Apple M1 Ultra, macOS 27.0 (Darwin 27.0.0) arm64, shared with other
build jobs (no timing recorded). Toolchains: pinned `nightly-2026-09-07`
(`rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)`) and `1.98.0`. NASM is
installed but not needed on ARM64. Commands ran from this branch with
`CARGO_BUILD_JOBS=4`; the shared `target/` was reused, so these are not
from-scratch clean builds.

| Command | Result |
| --- | --- |
| `cargo test -p ferrisma --all-targets --features cli,jpeg-xl-write` | pass (204 lib tests + integration tests) |
| `cargo test -p ferrisma --all-targets --all-features` (scoped to the engine) | pass |
| `cargo test -p ferrisma --all-targets --no-default-features --features <set>` for `jpeg-read`; `jpeg-read,avif-write`; `jpeg-read,avif-own-write`; `jpeg-read,webp-read,webp-write,jpeg-xl-read,avif-write,icc`; the same with `jpeg-xl-own-read` instead of `jpeg-xl-read`; and with both | pass |
| `cargo test -p ferrisma-libavif --all-targets --features cli` | pass (11 tests) |
| `cargo test -p ferrisma-avif-write -p ferrisma-avif-read -p ferrisma-jxl-decoder --lib` | pass |
| `cargo test --manifest-path fuzz/Cargo.toml --lib --features own-jxl` | pass (5 tests) |
| `cargo run -p ferrisma --example consumer --features cli -- --self-check` | `self-check passed` |
| `cargo clippy -p ferrisma --all-targets --no-deps -- -D warnings` with `--all-features`, `--features cli,jpeg-xl-write`, no features, `jpeg-read,avif-own-write`, and the stable set without `jpeg-xl-own-read` | pass |
| `cargo clippy -p ferrisma --all-targets --no-deps --no-default-features --features jpeg-read,avif-write -- -D warnings`, and the same with `jpeg-read,webp-read,webp-write,jpeg-xl-own-read,avif-write,icc` | **fails**: pre-existing `dead_code` on `CodecWork::region_at_least`, which only jxl-oxide uses outside tests; CI does not lint these combinations |
| `cargo check -p ferrisma --no-default-features --features <feature>` for each of the 11 features, `jpeg-xl-write` included | pass |
| `cargo +1.98.0 check -p ferrisma -p ferrisma-kernels --features jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-own-read,avif-write,icc --locked` | pass |
| `cargo +1.98.0 check -p ferrisma --no-default-features --features jpeg-xl-write --locked` | pass (one warning) |
| `cargo +1.98.0 check -p ferrisma-xtask --all-targets --locked` | pass |
| `cargo +1.98.0 check -p ferrisma --no-default-features --features avif-own-write --locked` | fails as expected (`E0554`, `c_variadic`): nightly only |
| `cargo xtask codecs --codec avif --stage pr --tests-only` (the ARM64 form CI uses) | pass: release tests of `libavif-rs`, `libavif-aom`, `libavif-dav1d`, `libavif-sharpyuv` (5 + 63 + 3 + 1 tests), the checked SharpYUV cross-port test, and `check_aom_golden.py`: 62 of 62 cases match `golden/release.json` for macos-aarch64-release |
| `cargo xtask codecs --codec jpeg --stage pr --tests-only` | pass |
| `cargo xtask codecs --codec jxl --stage pr --tests-only` | pass (exit 0, 114 test binaries) |
| `cargo xtask codecs --codec webp --stage pr --tests-only` | **fails here for setup reasons**: `wp12_incremental` reads `codecs/libwebp-rs/upstream/libwebp-test-data/*.webp`, and that pinned submodule is not initialized in this worktree; CI initializes it first |

Not verified here: GNU Linux x86-64 (including NASM and the x86-64 JPEG XL
conformance run), clean from-scratch builds, the full (not `--tests-only`) codec `pr`
stages, and the WebP `pr` stage, which need the pinned upstream submodules
(`codecs/*/upstream`) that are not initialized in this worktree. The x86-64 claims
above rest on these CI workflows, which still need a hosted run on this
branch: `ci.yml` jobs `rust` (Ubuntu 24.04 x86-64 with NASM: engine tests with
and without all features, per-feature checks, consumer self-check),
`minimum-rust` (stable 1.98 set), `workspace`, `codecs` (per-codec
`cargo xtask codecs --stage pr`, x86-64 full and ARM64 `--tests-only`), and
`native-adapters` (macOS 15 ARM64 adapter tests); `nightly-avif.yml` and
`avif-aom-golden-c.yml` for the AVIF port; `jxl-check.yml` for libjxl-rs.
