# Supported input, output, backend, and feature matrix

**Reviewed: 2026-11-12** (issues #67 and #26; base `1593d6a8` plus the default
switches of the same day). This page is the single place that says what
Ferrisma accepts, what it rejects, which backend does the work, and which
build provides it. It does not promise a stable API: the stability tiers come
from the *proposed* [ADR-0040](adr/0040-stable-public-contract-and-compatibility-policy.md)
and the [compatibility guide](compatibility.md).

Every row names the test or fixture that supports it. A claim without a named
test is marked **untested** and is a statement about the code path only. Header
inspection alone is never treated as proof that a file decodes or is
supported.

Two defaults changed on 2026-11-12 under the owner's v1.0 direction and are
**pending owner confirmation**: JPEG XL input now defaults to the workspace
libjxl-rs reader ([ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)) and
AVIF output to the workspace libavif/AOM writer
([ADR-0037](adr/0037-selectable-workspace-avif-encoder.md)).

## Cargo features and builds

`default = []`: without features the engine has no codec. Each feature adds
one adapter. The default backend of a format follows the compiled features;
because Cargo unifies features, a dependency that enables a workspace feature
changes the default of the whole build. Select a backend explicitly
(`TransformOptions::jxl_decoder`, `TransformOptions::avif_encoder`) when that
matters.

| Feature | Provides | Backend | Default role | Toolchain | Extra prerequisites | Proposed tier (ADR-0040) |
| --- | --- | --- | --- | --- | --- | --- |
| `jpeg-read` | JPEG input | workspace `libjpeg-turbo-rs` | only JPEG reader | stable 1.98 | none | Stable |
| `webp-read` | still WebP input | workspace `libwebp-rs` | only WebP reader | stable 1.98 | none | Stable |
| `webp-write` | still WebP output | workspace `libwebp-rs` | only WebP writer | stable 1.98 | none | Stable |
| `jpeg-xl-own-read` | JPEG XL input | workspace `libjxl-rs` via `ferrisma-jxl-decoder` | **default JPEG XL reader** when compiled in | stable 1.98 | none | Experimental |
| `jpeg-xl-read` | JPEG XL input | `jxl-oxide` 0.12 | default only without `jpeg-xl-own-read`; otherwise `--jxl-decoder=jxl-oxide` | stable 1.98 | none | Stable as capability |
| `avif-read` | still AVIF input | workspace libavif/AOM via `ferrisma-avif-read` | only AVIF reader | pinned nightly | native macOS ARM64 or GNU Linux x86-64 | Experimental |
| `avif-own-write` | AVIF output | workspace libavif/AOM via `ferrisma-avif-write` | **default AVIF encoder** when compiled in | pinned nightly | native macOS ARM64 or GNU Linux x86-64 | Experimental |
| `avif-write` | AVIF output | `rav1e` 0.8 + `avif-serialize` | default only without `avif-own-write`; otherwise `--avif-encoder=rav1e` | stable 1.98 | NASM on x86-64 (rav1e assembly, ADR-0018) | Stable as capability |
| `jpeg-xl-write` | lossless JPEG XL output | workspace `libjxl-rs` fixed profile | only JPEG XL writer; **not in `cli`** | stable 1.98 (check only, see below) | none | Experimental |
| `icc` | rendered-ICC → sRGB conversion | `moxcms` 0.9 | needed for `UseRenderedIcc` / `--use-rendered-icc` | stable 1.98 | none | Stable |
| `cli` | the `ferrisma` binary | `jpeg-read`, `webp-read`, `jpeg-xl-read`, `jpeg-xl-own-read`, `avif-read`, `avif-write`, `avif-own-write`, `webp-write`, `icc` | — | pinned nightly | native macOS ARM64 or GNU Linux x86-64; NASM on x86-64 | Stable command line, experimental backends |

The pinned nightly is `nightly-2026-09-07` from
[`rust-toolchain.toml`](../rust-toolchain.toml); the generated AVIF code needs
`extern_types` and `c_variadic`. Stable Rust 1.98 is the declared
`rust-version`. The stable-compatible set is the one CI's `minimum-rust` job
checks: `jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-own-read,avif-write,icc`.
A stable-toolchain build that needs AVIF output therefore uses rav1e; a
stable-toolchain build cannot read AVIF.

Evidence: CI `rust` job checks each feature alone (`cargo check
--no-default-features --features <feature>` for every feature above except
`jpeg-xl-write`); `minimum-rust` checks the stable set with `cargo +1.98.0`;
the local run in [Verification record](#verification-record-2026-11-12-macos-arm64)
repeats both, adds `jpeg-xl-write` alone on nightly and on stable, and confirms
that `avif-own-write` fails on stable with `E0554`. `jpeg-xl-write` on stable
is a local `cargo check` only; no CI job covers it.

## Inputs

All inputs are still images. Every decoder's emitted samples, not the file's
original profile, define the color meaning (ADR-0015). Orientation is applied
before crop; crop coordinates are upright display coordinates (ADR-0019).

| | JPEG | still WebP | JPEG XL | still AVIF |
| --- | --- | --- | --- | --- |
| Feature | `jpeg-read` | `webp-read` | `jpeg-xl-own-read` (default) / `jpeg-xl-read` | `avif-read` |
| Samples delivered | 8-bit RGB (adapter requests RGB) | 8-bit RGB or RGBA | F32 gray, gray+alpha, RGB, RGBA | 8-bit RGB(A); 10/12-bit as normalized float |
| Alpha | none | straight | straight or premultiplied, from basic info | straight (libavif output) |
| Color evidence | embedded ICC | embedded RGB ICC | libjxl-rs: `DATA` ICC of the emitted samples; jxl-oxide: rendered ICC/CICP | container `nclx`/ICC cross-checked with the AV1 sequence header; reported as rendered CICP |
| Orientation | EXIF 1–8 | EXIF 1–8 | codestream orientation, rendered upright by the decoder | `irot`/`imir` applied by the pipeline |
| Crop access | codec crop request, iMCU column overfetch | lossy: crop from full-width rows; lossless: full reconstruction | libjxl-rs: **full-frame fallback for every crop**; jxl-oxide: codec region request | full-frame fallback |
| Rejected | CMYK/YCCK (`UnsupportedInput`); malformed ICC; truncated data | animation (`UnsupportedInput`); malformed containers; canvas above the pixel cap | animation, CMYK, spot colors, float/HDR exponent channels, extra channels, PQ/HLG, unknown colour spaces, ICC > 16 MiB | animation/`avis`, `clap`, `imir` before `irot`, monochrome, contradictory `nclx`, PQ/HLG |

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
  `lossless_webp_crop_is_small_but_reconstruction_is_not`.
- **JPEG XL** — default reader: `pipeline_tests::default_jpeg_xl_decoder_follows_the_compiled_backends`,
  `selected_local_jpeg_xl_decoder_reports_full_frame`;
  `tests/decoder_work.rs::libjxl_rs_crop_reports_the_full_frame_fallback`;
  `ferrisma_jxl_decoder` unit tests `preserves_grayscale_and_alpha_association`,
  `rejects_layouts_outside_the_engine_contract_before_pixels`,
  `checks_pixel_limit_before_output`; `jxl_color_tests::rendered_icc_reproduces_known_samples_for_sdr_families`,
  `hdr_and_unverifiable_encodings_are_rejected`,
  `both_decoders_agree_on_xyb_rendered_into_display_p3`;
  `orientation_tests::jpeg_xl_orientations_are_rendered_upright_for_every_crop`,
  `lossy_jpeg_xl_orientations_and_crops_through_libjxl_rs`;
  `tests/animated_input.rs::animated_jpeg_xl_is_rejected_by_libjxl_rs`;
  `hardening_tests::workspace_jxl_unknown_colour_space_is_a_controlled_error`.
  External: the pinned JPEG XL conformance corpus passes 27/27 level-10 and
  24/24 level-5 entries on macOS ARM64; through the engine wrapper 32 accepted
  cases pass and 8 (animation, CMYK, spot colors, float samples) are rejected
  before pixel allocation ([record](../codecs/libjxl-rs/docs/CONFORMANCE.md)).
  jxl-oxide (explicit): `tests/decoder_work.rs::jxl_oxide_crop_reports_only_a_lower_bound`,
  `orientation_tests::interim_jpeg_xl_orientations_are_rendered_upright_for_every_crop`,
  `tests/animated_input.rs::animated_jpeg_xl_is_rejected_by_jxl_oxide`,
  `hardening_tests::public_transcode_rejects_unknown_jxl_colour_space`.
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
  `coded_pixel_limit_reports_exact_sequence_geometry`. Monochrome AVIF is
  rejected by the adapter (`UnsupportedInput`) but has **no named test**.
  Argon AV1 conformance was not run (ADR-0025); the reader relies on the
  reference-origin route.

## Color policy

| `InputProfilePolicy` / CLI | Behavior | Evidence |
| --- | --- | --- |
| `Strict` (default, no flag) | Accept only decoder-reported sRGB (for example signalled sRGB AVIF); untagged or ICC-tagged input fails with `UnsupportedColor` | `pipeline_tests::missing_color_information_requires_an_explicit_override`, `signalled_srgb_avif_transcodes_under_the_strict_profile_policy` |
| `AssumeSrgbIfMissing` / `--assume-srgb` | Treat input as sRGB **only** when it reports neither ICC nor CICP; an ICC-tagged file still needs `--use-rendered-icc` | same tests; upstream corpus pins `paris_icc_exif_xmp.avif` as `UnsupportedColor` without `UseRenderedIcc` |
| `UseRenderedIcc` / `--use-rendered-icc` | Convert the decoder's rendered RGB ICC to the working space through `moxcms` (`icc`); RGB only, ICC with alpha is rejected | `tests/icc_photo.rs::real_srgb_icc_photograph_matches_littlecms`, `photograph_as_display_p3_matches_littlecms_out_of_gamut`; `tests/resize_oracle.rs::icc_tagged_alpha_input_is_rejected_before_output`; `cms::tests::malformed_and_hostile_profiles_are_controlled_errors` |
| PQ, HLG, unverifiable or contradictory signals | Always `UnsupportedColor`; never reinterpreted as sRGB | `jxl_color_tests::hdr_and_unverifiable_encodings_are_rejected`; `codecs::avif_meta_tests::contradictory_container_and_bitstream_signals_fail_closed`; `color_contract_tests::two_present_signals_require_explicit_precedence_even_when_both_look_srgb` |

Output is always SDR sRGB, 8-bit straight-alpha RGBA rows handed to the
encoder (ADR-0024, ADR-0030). HDR, tone mapping and wide-gamut output are not
supported.

## Outputs

| | AVIF, libavif/AOM (default) | AVIF, rav1e (explicit) | still WebP | JPEG XL (lossless) |
| --- | --- | --- | --- | --- |
| Feature / selection | `avif-own-write`; `AvifEncoder::Libavif { depth }`, `--avif-encoder=libavif` | `avif-write`; `AvifEncoder::Rav1e`, `--avif-encoder=rav1e` | `webp-write`; `.webp` | `jpeg-xl-write`; `.jxl` (not in `cli`) |
| Bit depth | 10 (default) or 8 (`--avif-depth=8`) | always 10 | 8 | 8 |
| Chroma | 4:4:4 (default) or 4:2:0 | 4:4:4 (default) or 4:2:0 | lossy YUV 4:2:0 / lossless ARGB | RGB, no subsampling |
| Quality | `quality` 0–100, rounded, 64 quantizer steps | `quality` 1–100 through `ravif`'s curve | `WebpOptions::Lossy { quality 1–100, speed }` or `Lossless { level 0–9 }` | none; `--quality` is rejected |
| Speed | 0–10 (`cpu-used`) | 1–10 | lossy speed 1 (slowest)–10; lossless uses the level | effort 2 only |
| Alpha | auxiliary item at the color quality, omitted when opaque | separate 10-bit item at fixed quality 80, omitted when opaque | straight alpha | unassociated alpha at 8 bits |
| Color signaling | AV1 header 1/13/6 full range **and** container `colr nclx` | AV1 header 1/13/6 full range; no container `colr` | embedded sRGB ICC | enum sRGB |
| Metadata | source EXIF/XMP/ICC not copied; orientation applied | same | same; EXIF orientation and XMP dropped | orientation applied, written as 1 |
| Materialization | full RGBA8 frame plus libavif YUV(A) planes, reserved before encoding; encoded output briefly twice | padded 10-bit frame(s) | ARGB frame and lossy planes reserved | input frame and known codec planes reserved |
| Determinism | bytes can depend on thread count and host | tiles derived from size: bytes independent of thread count | — | byte-identical with the C++ encoder on the port's recorded requests |

**The two AVIF quality scales are not comparable and are never remapped.**
`quality` and `speed` go unchanged to the selected encoder. At equal
SSIMULACRA2 on the recorded photographs, rav1e 70/80/90 matched roughly
libavif 50/55/78; at `--quality=80` on the 800×533 flower, rav1e wrote 55,682
bytes (SSIMULACRA2 77.455) and libavif 10-bit 100,512 bytes (88.307)
([evidence](benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md)). A
quality value tuned for rav1e gives larger files with libavif. The default
`quality` is 75 on whichever scale is selected.

Evidence: AVIF/libavif — `crates/ferrisma-libavif/tests/encode.rs`
(`encodes_what_ferrisma_produced_with_the_same_color_signals`,
`straight_alpha_survives_the_encode`, `invalid_settings_are_rejected_before_decoding`,
`frame_preflight_counts_odd_chroma_and_alpha_planes`,
`output_limit_is_enforced_after_backend_finish`,
`own_encoder_facade_handles_reference_batch_and_invalid_settings`,
`independent_oriented_avif_reference_reaches_own_encoder`);
`tests/own_avif_cli.rs::normal_cli_selects_own_encoder_with_order_independent_depth`;
`hardening_tests::api_rejects_invalid_quality_and_mismatched_input_format`.
AVIF/rav1e — `avif_sink_tests::opaque_output_decodes_without_alpha_in_both_chroma_layouts`,
`transparent_output_carries_an_alpha_item`,
`size_derived_tiles_make_bytes_independent_of_the_thread_count`,
`the_encoder_frame_is_charged_to_the_budget`;
`pipeline_tests::produced_avif_is_accepted_by_an_independent_decoder`.
The signaling and alpha-quality differences were observed with FFmpeg/libdav1d,
libheif and rav1d in the evidence record above. WebP —
`webp_output_tests::still_webp_outputs_decode_with_straight_alpha_and_srgb_metadata`,
`webp_output_frame_and_file_limits_fail_before_retaining_result`,
`lossy_converted_planes_are_reserved_before_encoding`. JPEG XL —
`jxl_output_tests::sink_round_trips_exactly_through_the_workspace_decoder`,
`sink_round_trips_exactly_through_jxl_oxide`,
`oriented_input_is_written_upright_with_orientation_one`,
`rejects_requests_outside_the_admitted_profile_before_allocating`,
`facade_rejects_mismatched_and_disabled_settings_before_decoding`
([ADR-0039](adr/0039-jpeg-xl-output-through-the-workspace-encoder.md)).
JPEG output does not exist and is not planned. Lossy JPEG XL output waits for
the port's encoder (#92).

## Limits and memory

| Option / CLI | Default | Checked | Evidence |
| --- | --- | --- | --- |
| `max_input_bytes` / `--max-input-mb` (MiB) | 64 MiB | before any codec starts; CLI stream read is capped | `pipeline_tests::compressed_bytes_and_decoded_pixel_caps_reject_before_processing`; `cli::tests::accepts_input_at_the_limit_and_rejects_one_byte_more` |
| `max_input_pixels` / `--max-input-mpx` (10^6 px) | 100,000,000 | from headers before pixel allocation (AVIF also checks coded frame size) | same; `ferrisma_avif_read::coded_pixel_limit_reports_exact_sequence_geometry`; `ferrisma_jxl_decoder::checks_pixel_limit_before_output` |
| `max_managed_bytes` / `--budget-mb` | 256 MiB | engine buffers, known encoder frames | `tests.rs::insufficient_budget_releases_all_owned_buffers`; `avif_sink_tests::the_encoder_frame_is_charged_to_the_budget` |
| `max_output_bytes` / `--max-output-mb` | 256 MiB | WebP while writing; AVIF and JPEG XL after the encoder returns | `webp_output_tests::exact_final_byte_limit_accepts_opaque_lossy_alpha_and_lossless`; `jxl_output_tests::encoded_output_limit_applies_at_the_exact_byte`; `ferrisma-libavif` `output_limit_is_enforced_after_backend_finish` |
| threads | `transform_threads` 0 (all cores), `encoder_threads` 1; at most 256 | before decoding | `pipeline_tests::encoder_threads_trade_determinism_for_latency_explicitly` |

Codec-internal allocations (all four decoders, all encoders) are opaque: the
report sets `opaque_decoder_memory`/`opaque_encoder_memory`, and there is no
process-wide memory cap. Full-frame decoders (libjxl-rs, AVIF, lossless WebP
internals) need the whole frame for any crop.

## Native targets

| Target | Engine features on stable | `avif-read`, `avif-own-write`, `cli` | Verified by |
| --- | --- | --- | --- |
| macOS ARM64 | yes | yes (pinned nightly) | local run below; CI `native-adapters` (`macos-15`) |
| GNU Linux x86-64 | yes | yes (pinned nightly, NASM) | CI `rust`, `minimum-rust`, `workspace`, `codecs` (`ubuntu-24.04`); **not re-run for this branch** |
| GNU Linux ARM64 | codec-free engine and kernels tested in CI | not supported | CI `kernels-arm` (`ubuntu-24.04-arm`: `cargo test -p ferrisma -p ferrisma-kernels --all-targets`, no codec features) |
| Windows, other targets | not verified | not supported | out of v1.0 scope (#158, #159) |

## Examples

Supported (output of the debug `cli` build on 2026-11-12, report line
abbreviated):

```sh
# JPEG → AVIF with the default libavif/AOM encoder, 10-bit 4:4:4
ferrisma photo.jpg out.avif 64 48 --assume-srgb
#   report: … encoder=libavif-rs/aom
# The same with rav1e; its quality 80 is not libavif's quality 80
ferrisma photo.jpg out.avif 64 48 --assume-srgb --avif-encoder=rav1e --quality=80
#   report: … encoder=rav1e
# libavif at 8 bits, 4:2:0 (--avif-depth alone selects the default libavif)
ferrisma photo.jpg out.avif 64 48 --assume-srgb --avif-depth=8 --chroma=420
# ICC-tagged AVIF through the rendered-ICC path
ferrisma paris_icc_exif_xmp.avif out.webp 64 48 --use-rendered-icc
# JPEG XL with the default reader (full-frame fallback) and with jxl-oxide
ferrisma sample.jxl out.avif 6 4 --use-rendered-icc                          # decoder_full_frames=1
ferrisma sample.jxl out.avif 6 4 --use-rendered-icc --jxl-decoder=jxl-oxide  # decoder_full_frames=0
# Lossless WebP; lossless JPEG XL (needs --features cli,jpeg-xl-write)
ferrisma photo.jpg out.webp 64 48 --assume-srgb --webp-lossless --webp-level=6
ferrisma photo.jpg out.jxl 64 48 --assume-srgb
```

Rejected, with the error the CLI prints (exit status 1):

| Command | Error | What to do |
| --- | --- | --- |
| `ferrisma photo.jpg out.avif 64 48` (untagged) | `UnsupportedColor` "unsupported or ambiguous rendered color profile…" | add `--assume-srgb` if that assumption is acceptable |
| `ferrisma sample.jxl out.avif 6 4 --assume-srgb` (ICC-tagged) | `UnsupportedColor` | use `--use-rendered-icc` (needs `icc`) |
| `ferrisma cmyk.jpg out.avif 64 48 --assume-srgb` | `UnsupportedInput` "decoder pixel or color layout is not supported" | convert CMYK to RGB first |
| `ferrisma animated.webp out.avif 64 48 --assume-srgb` | `UnsupportedInput` "animated input is not supported…" | extract one frame first |
| `ferrisma colors-animated-8bpc.avif …` | `UnsupportedInput` | same |
| `ferrisma clap_irot_imir_non_essential.avif …` | `UnsupportedInput` "orientation, mirroring or clean-aperture transforms are not supported yet" | remove `clap` or crop explicitly with `--crop` on a file without it |
| `ferrisma colors_hdr_rec2020.avif …` (PQ) | `UnsupportedColor` | tone-map to SDR elsewhere; HDR is out of scope |
| `ferrisma photo.jpg out.avif … --avif-encoder=rav1e --quality=0` | `InvalidOptions` | rav1e's scale starts at 1; libavif accepts 0 |
| `ferrisma photo.jpg out.webp … --avif-encoder=rav1e` | "AVIF encoder options require .avif output" | drop AVIF flags for WebP |
| `ferrisma photo.jpg out.jxl … --quality=80` | "--quality is unsupported for lossless JPEG XL" | JPEG XL output is lossless only |
| `ferrisma photo.jpg out.png …` | "output must have a .avif, .webp or .jxl extension" | choose a supported output |
| `ferrisma photo.jpg out.avif … --max-input-mpx=0` | `InvalidOptions` "input byte/pixel limits must be nonzero" | use a positive limit |

The Rust facade returns the same `ProcessingErrorKind` values;
[`examples/consumer.rs`](../examples/consumer.rs) maps each kind to a
reject/retry/alert decision and its `--self-check` asserts them.

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
