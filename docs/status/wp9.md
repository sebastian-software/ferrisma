# WP9: encoder picture and config

- Status: complete. Every unit of the WP9 inventory is translated, with the three
  decisions of the integrator (2026-11-07) applied: a shared view (`WebPPictureView`),
  the distortion on that view (`WebPPictureDistortion`) and a copy without callbacks
  (`WebPPictureCopy`). The deviations are in [Recorded deviations](#recorded-deviations).
- Sub-wave: 2a
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1, WP3, WP4, WP5 (all merged)
- Probes: `tools/oracle/probes/wp9/` (with `support/`); tests: `tests/wp9_*.rs`, with the
  shared test support in `tests/support/wp9_pictures.rs` and `tests/support/wp9_visible.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

Every row names what is translated, with `// upstream:` markers on each item.
"Partly" rows name what stays with another package and why.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/enc/config_enc.rs` | `src/enc/config_enc.c` | yes: `WebPConfigInitInternal`, `WebPValidateConfig`, `WebPConfigLosslessPreset`, `MAX_LEVEL` and the table `kLosslessPresets` (`LOSSLESS_PRESETS`). |
| `src/enc/picture_enc.rs` | `src/enc/picture_enc.c` | partly: `DummyWriter` (as the discarding writer), `WebPPictureInitInternal`, `WebPValidatePicture`, `WebPPictureResetBuffers` with the two resets, `WebPPictureAllocARGB`, `WebPPictureAllocYUVA`, `WebPPictureAlloc`, `WebPPictureFree`, `WebPMemoryWriterInit`, `WebPMemoryWrite` and `WebPMemoryWriterClear`. Not here: `Encode` and the eight `WebPEncode*` functions (WP13). The Rust-only plane helpers at the end of the file have no C counterpart and are named as such. |
| `src/enc/picture_csp_enc.rs` | `src/enc/picture_csp_enc.c` | partly: `WebPPictureHasTransparency`, the gamma tables (`LazyLock`), `RGBToY/U/V`, `PreprocessARGB`, `SUM4`, `SUM2`, `SUM2ALPHA`, `SUM4ALPHA`, `kInvAlpha` (`INV_ALPHA`), `DIVIDE_BY_ALPHA`, `LinearToGammaWeighted`, `ConvertRowToY`, `AccumulateRGBA`, `AccumulateRGB`, `ConvertRowsToUV`, `ImportYUVAFromRGBA`, `PictureARGBToYUVA`, `WebPPictureARGBToYUVA`, `WebPPictureARGBToYUVADithered`, `WebPPictureSharpARGBToYUVA`, `WebPPictureSmartARGBToYUVA`, `WebPPictureYUVAToARGB`, `Import`, and `WebPPictureImportBGR`, `BGRA`, `BGRX`, `RGB`, `RGBA`, `RGBX`. Not here: `SharpYuvInit` (WP5 deviation 1) and the `Init` functions of ADR 0003, which assign pointers only. |
| `src/enc/picture_rescale_enc.rs` | `src/enc/picture_rescale_enc.c` | yes: `WebPPictureIsView`, `WebPPictureView`, `WebPPictureCopy`, `WebPPictureCrop`, `WebPPictureRescale` with `RescalePlane`, `AlphaMultiplyARGB` and `AlphaMultiplyY`, `PictureGrabSpecs` (in the form that moves the callbacks), `AdjustAndCheckRectangle`, `SnapTopLeftPosition` and `HALVE`. |
| `src/enc/picture_tools_enc.rs` | `src/enc/picture_tools_enc.c` | yes: `WebPReplaceTransparentPixels`, `WebPCleanupTransparentArea` with `IsTransparentARGBArea`, `Flatten`, `FlattenARGB` and `SmoothenBlock`, `WebPBlendAlpha` with `BLEND`, `BLEND_10BIT` and `MakeARGB32`. |
| `src/enc/picture_psnr_enc.rs` | `src/enc/picture_psnr_enc.c` | yes: `WebPPlaneDistortion` with `AccumulateSSE`, `AccumulateSSIM`, `AccumulateLSIM`, `GetPSNR` and `GetLogSSIM`, and `WebPPictureDistortion` with `BLUE_OFFSET`. |
| `src/enc/webp_enc.rs` (exception) | `src/enc/webp_enc.c`: `WebPEncodingSetError`, `WebPReportProgress` | yes. |
| `src/webp/encode.rs` (exception) | `src/webp/encode.h`: the inline wrappers `WebPConfigInit`, `WebPConfigPreset` and `WebPPictureInit` | yes. The WP1 content of the file is unchanged. |
| `src/webp/types.rs` (WP1 file, integrator decision) | `src/webp/types.h`: not a C unit; the `PlaneStorage::Shared` variant of ADR 0003 | yes, added with its amendment: `PlaneStorage::Shared(Vec<&'a [T]>)`. |

## Evidence

Every differential test runs a C probe in a separate process, against the oracle
built by `python3 tools/build_oracle.py`, and compares exact values. The
probes free every picture they create (LeakSanitizer runs only on Linux CI).

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp9_config.rs` | `config_init_presets_and_wrappers_match_the_c_config` | `WebPConfigInitInternal` for 5 versions (one incompatible in each direction), 8 presets (one outside the enumeration), 8 qualities (0, 75, 100, 50.5, -0.5, 100.5, NaN, infinity); the `WebPConfigPreset` and `WebPConfigInit` wrappers; `WebPConfigLosslessPreset` for levels -1 to 10. Each config starts with `0xa5` bytes, as the probe's `memset` does, so untouched fields (`use_delta_palette`) are compared too. Floats are compared as bits. |
| `tests/wp9_config.rs` | `config_validation_at_the_boundaries_matches_the_c_validator` | `WebPValidateConfig` on the default config and on one-field changes for all 29 fields to 6 to 10 candidate values each, including the float boundaries, NaN and infinities. |
| `tests/wp9_picture.rs` | `alloc_and_free_match_the_c_picture` | `WebPPictureAlloc` and `WebPPictureFree` for 14 geometries (ARGB and YUVA, with and without alpha, zero and negative sizes, odd sizes), with the return value, the error code, the strides, the null flags and `WebPPictureIsView` before and after the free; two allocation-limit cases that the C code rejects before allocating (65536 by 65536 ARGB, 100000 by 100000 YUVA with alpha). |
| `tests/wp9_picture.rs` | `validation_and_initialization_match_the_c_picture` | `WebPValidatePicture` on 10 cases (zero, negative, invalid colorspace, the 16384 limit that C does not check), `WebPPictureInitInternal` for the three versions, and the `WebPPictureInit` wrapper. |
| `tests/wp9_picture.rs` | `importers_match_the_c_importers_on_padded_strides` | `WebPPictureImportRGB`, `RGBA`, `BGR`, `BGRA`, `RGBX` and `BGRX` in both the YUV and the ARGB form, on 6 geometries, with and without 3 bytes of row padding, on seeded random buffers: the return value, the error code, the colorspace, and the visible samples of the planes (the alpha plane only when it exists). The RGBA rows excluded below are not generated. |
| `tests/wp9_picture.rs` | `argb_to_yuva_conversions_match_the_c_conversions` | `WebPPictureARGBToYUVA`, `WebPPictureARGBToYUVADithered` (dithering 0, 0.5 and 1) and `WebPPictureSharpARGBToYUVA`, for colorspaces 0 and 4, an invalid colorspace, and sizes from 1 by 1 to 9 by 6 with padded strides. |
| `tests/wp9_picture.rs` | `yuva_to_argb_conversion_matches_the_c_conversion` | `WebPPictureYUVAToARGB` on caller-owned planes with and without alpha, odd and even sizes, padded strides, including the even and odd last rows. |
| `tests/wp9_tools.rs` | `transparent_area_cleanup_matches_the_c_cleanup` | `WebPCleanupTransparentArea` on 60 seeded pictures (ARGB and YUVA, with and without alpha, sizes from 1 to 20 with remainders, padded strides) with transparent 8 by 8 blocks; compares every byte of every buffer, padding included. Asserts that some case changed a sample. |
| `tests/wp9_tools.rs` | `alpha_blending_matches_the_c_blending` | `WebPBlendAlpha` on 60 seeded pictures with random backgrounds; every byte compared. Asserts that some case changed a sample. |
| `tests/wp9_tools.rs` | `transparency_detection_matches_the_c_detection` | `WebPPictureHasTransparency` on 60 seeded pictures, both forms. |
| `tests/wp9_tools.rs` | `transparent_pixel_replacement_matches_the_c_replacement` | `WebPReplaceTransparentPixels` on 40 seeded ARGB pictures with random colors. |
| `tests/wp9_rescale.rs` | `rescale_matches_the_c_rescale` | `WebPPictureRescale` on 60 seeded pictures to 10 target sizes: keep-aspect (0), down, up, 1 by 1, odd sizes, and the invalid -1. Compares the return value, the new size, the colorspace, the error code and the visible samples. Asserts that some rescale succeeded. |
| `tests/wp9_rescale.rs` | `crop_matches_the_c_crop` | `WebPPictureCrop` on 80 seeded pictures with random rectangles, including odd corners (the chroma position is snapped) and rectangles outside the picture. Asserts that some crop succeeded. |
| `tests/wp9_view.rs` | `view_matches_the_c_view` | `WebPPictureView` on 80 seeded pictures with random rectangles, inside and outside the picture, with odd corners (snapped), in both forms. Compares the return value and, for a view, its size, colorspace, error code and visible samples. Asserts that some view succeeded and that some was refused. |
| `tests/wp9_view.rs` | `copy_matches_the_c_copy` | `WebPPictureCopy` on 40 seeded pictures in both forms, with padded strides: the return value, the size, the colorspace, the error code and the copied samples. The C copy also shares the callback and reference pointers of its source; the expected output records that sharing (`writer_copied` and the other three `1`), which the Rust copy does not do (deviation 9). |
| `tests/wp9_view.rs` | `views_and_copies_hold_no_callbacks` | Rust-only: a source with a writer, a progress hook, `extra_info` and `stats` set. The view and the copy hold none of the four (`None`), the view's planes are `Shared`, and the copy's planes are owned. |
| `tests/wp9_view.rs` | `writing_into_a_yuva_view_panics`, `writing_into_an_argb_view_panics` | Rust-only: `WebPCleanupTransparentArea` on a view of a YUVA picture and of an ARGB picture panics, because the view's planes are shared (deviation 8). |
| `tests/wp9_distortion.rs` | `plane_distortion_matches_the_c_metrics_bit_for_bit` | `WebPPlaneDistortion` for the three metrics (0, 1, 2), with `x_step` 1 and 4, planes from 1 by 1 to 17 by 5, padded strides, identical planes, and a stride one byte short of a row (which fails). Compares the return value and the bits of both floats, which start at -1 so an untouched output is visible. Asserts that some result is not the maximum. |
| `tests/wp9_distortion.rs` | `picture_distortion_matches_the_c_distortion` | `WebPPictureDistortion` on 48 seeded pairs of pictures in both forms and both colorspaces, for the three metrics, and every eighth pair with different widths (refused, results untouched). Compares the return value and the bits of the five results. Asserts that some pair was measured and some was refused. |
| `tests/wp9_writer_errors.rs` | `writer_errors_and_progress_match_the_c_functions` | One sequence in one process: `WebPMemoryWriterInit`, six `WebPMemoryWrite` calls (through the 8192-byte minimum and a doubling, one empty write), `WebPMemoryWriterClear`, `WebPPictureInit`, five `WebPEncodingSetError` calls (the oldest error is kept), a progress hook that aborts below its threshold, five `WebPReportProgress` calls with a store and one with a `NULL` store. |

Local results on this branch (release oracle, the reference):
`python3 tools/check.py --oracle --msrv` passed, with format, Clippy with
`-D warnings` on all targets, the module skeleton, `tools/unit_dependencies.py
--check` (0 undeclared references), the tests in the dev, release and `checked`
profiles, and the build and tests with Rust 1.95: 172 test results, none failed.
The `debug` variant (`python3 tools/check.py --oracle-variant debug --stage test`)
reports 129 results, none failed, with upstream's assertions enabled. The
`asan-ubsan` variant reports 129 results, none failed, and no sanitizer report,
after the exclusion recorded below. LeakSanitizer does not run on macOS.

## Not-applicable checks

These checks do not apply to WP9, or cannot be made, with the reason:

1. `NULL` arguments of every WP9 function (`picture`, `config`, the source and
   destination pictures, the `bgr`, `rgba` and similar buffers, and the planes of
   `WebPPlaneDistortion`). Rust takes references, so the checks have no
   counterpart; the ABI mirror of WP13 (ADR 0004) keeps them.
2. Zero dimensions and zero-sized allocations. `WebPValidatePicture` rejects them
   before anything is allocated, which the allocation and validation tests compare.
3. Allocation limits at the byte level. The arithmetic of the allocation test is
   WP1's (`tests/wp1_alloc.rs`). WP9 compares the pictures that the limit rejects
   before allocating, and not successful large allocations: C leaves them
   uncommitted, while Rust zero-fills them (ADR 0003), which would touch the memory.
4. Samples that the library allocates and has not written yet. The import and
   conversion tests compare only the visible samples of the planes, never the
   padding, and the probes print the caller's buffers whole only where the caller
   supplied them.
5. Negative strides and negative dimensions. A Rust slice cannot address rows
   before its start, so a negative row start panics (deviation 7). The tests do not
   generate them. `WebPPlaneDistortion` with a negative width returns `false` in C and
   Rust by the same comparison, but is not compared.
6. The padding byte of `WebPPictureImportRGBX` and `BGRX` (not read), and the
   alignment of the ARGB plane (not observable, deviation 4).
7. SIMD and threads: none in the reference configuration.
8. Cross-host floating point. The distortion results are compared bit for bit on
   this host (macOS on Apple Silicon with Apple clang and `-ffp-contract=off`).
   Equality on other hosts is recorded, not assumed.
9. `NULL` planes of a view's source. C computes pointers from a `NULL` plane, which
   is a precondition violation; Rust keeps the plane `None` in the view.

## Recorded deviations

Deviations are changes of representation that ADR 0003 allows, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Errors.** `WebPEncodingSetError` returns the code that the picture holds
   afterwards, which is the code that a caller propagates with `Err`. The setter
   keeps the oldest error, as C does. Functions whose every failure path goes
   through the setter return `Result<(), WebPEncodingError>`: `WebPValidatePicture`,
   the allocators, `WebPPictureAlloc`, the ARGB-to-YUVA conversions, `WebPPictureYUVAToARGB`,
   `WebPPictureRescale`, `WebPPictureCopy` and `WebPReportProgress`. Functions with a
   failure that records no code stay `bool`, with the code left on the picture as in
   C: `WebPPictureImport*` (the stride test), `WebPPictureCrop` (the rectangle test),
   `WebPPlaneDistortion`, `WebPPictureDistortion`, `WebPMemoryWrite`, and the
   predicates. `WebPPictureView` returns `Option` (its failure records no code).
2. **Version checks.** `WebPConfigInitInternal` and `WebPPictureInitInternal` keep
   their version parameter and its ABI check. The wrappers pass the crate's
   version, so the check always succeeds through them.
3. **Mutable receivers.** `WebPValidatePicture`, `WebPEncodingSetError` and
   `WebPReportProgress` take `&mut WebPPicture`, where C takes `const` and casts it
   away to record the error.
4. **Plane storage.** The Y, U, V and A planes of a YUVA picture are one owned
   block with the C offsets, or the caller's regions 0 to 3. `yuva_planes_mut`
   gives several planes of one block mutable access at once by splitting the block.
   The ARGB plane starts at offset 0: C's `WEBP_ALIGN` address is not a value the
   Rust code can observe, and the allocation test still counts the
   `WEBP_ALIGN_CST` padding, so the same sizes fail.
5. **Zero-initialization.** Allocated planes are zero-filled, as ADR 0003 requires.
   A large successful allocation therefore touches its memory; C does not.
6. **Native-order byte views.** C reads and writes ARGB samples as bytes in five
   places: the ARGB-to-YUVA conversion, the YUVA-to-ARGB upsampler output, the
   RGBA import through `VP8LConvertBGRAToRGBA`, the rescaling of ARGB pictures, and
   the metric of `WebPPictureDistortion` on the ARGB views. The Rust code makes
   copies in native byte order (`to_ne_bytes`, `from_ne_bytes`), converts, and copies
   the words back where it writes. The values are the same; the copies cost one
   temporary of the plane size.
7. **Negative strides.** A row start before the start of its slice panics. C reads
   memory before the pointer in that case, which is valid only when the caller's
   memory extends there. The safe API (WP17) decides whether it takes a base offset;
   the integrator accepted the panic for now (2026-11-07).
8. **Shared views (decided 2026-11-07).** `WebPPictureView` borrows the source
   shared: its planes are a `PlaneStorage::Shared` with the source's regions and
   offsets, borrowed for the lifetime of the source (ADR 0003, amended). Reading a
   shared plane is allowed. Writing into one panics, because C would write through
   the `const` pointer that it casts away. The view is a value, not a write into the
   caller's `dst`: `webp_picture_view` returns `Option<WebPPicture<'s>>`.
9. **Callbacks of a copy and of a view (decided 2026-11-07).** `WebPPictureCopy` and
   `WebPPictureView` set `writer`, `progress_hook`, `extra_info` and `stats` to `None`.
   C copies these pointers, so source and copy share them. In Rust a boxed closure
   cannot be shared without shared ownership, and changing the writer and progress
   hook types for every user is not accepted. The internal callers (the animation
   encoder and WP14) set their own writer on the pictures they encode. The copy
   otherwise matches C: the specifications, the error code, new planes with the
   samples of the source, and the four `None` fields. The oracle test records the C
   sharing as its expected output.
10. **Rescale and crop.** C copies the whole picture struct, including the callbacks
    and the caller's `extra_info` and `stats`, into a temporary and replaces the
    picture with it. A boxed closure cannot be copied, so the Rust code moves the
    callbacks and the two references into the temporary and moves them back when the
    operation fails. The outcome is the same as C's.
11. **Missing planes.** `WebPCleanupTransparentArea` and `WebPBlendAlpha` return
    unchanged when a plane they need is missing, and `WebPReplaceTransparentPixels`
    returns when the ARGB plane is missing. C dereferences the missing plane for
    every block, which is a precondition violation; the cases are not generated.
12. **Init functions.** `WebPInitAlphaProcessing`, `WebPInitConvertARGBToYUV`,
    `VP8LDspInit` and `VP8SSIMDspInit` only assign portable pointers and are omitted
    (ADR 0003). `InitGammaTables` becomes two `LazyLock` tables, which
    `init_gamma_tables()` forces where C calls it. `SharpYuvInit` is omitted, as
    WP5 records.
13. **`WebPMemoryWriter.mem`.** Its length is `max_size`, and `WebPMemoryWrite`
    keeps `size` in step with it (WP1 deviation 17). `WebPMemoryWriterInit` leaves
    `mem` empty, which is C's `NULL`.
14. **`use_delta_palette`.** `WebPConfigInit` does not write this reserved field in
    C, so it keeps the caller's value. The Rust translation also leaves it alone,
    and the test compares it as unchanged.
15. **`WebPPictureIsView`.** A picture counts as a view when its storage holds no
    memory of its own, which is C's `memory_ == NULL`. A caller's storage (external
    or shared) is always a view.
16. **Rescaler work block (2026-11-07).** C allocates one `work` block of
    `2 * width` (YUV) or `2 * width * 4` (ARGB) elements and reuses it for every
    plane. `webp_rescaler_init` takes its two side rows by value and owns them (WP4
    deviation 8), so `rescale_into` keeps C's allocation check and `rescale_plane`
    gives each rescaler a vector of exactly `2 * num_channels * dst_width` elements.
    The samples are the same; only the number of allocations differs.

## Library functions that write planes in place

A consumer that needs to write into a picture's planes must know which functions
write into their argument. The list is the basis for the decision on a mutable view
(not added now, as the integrator directed). WP13 and WP14 encode views and must
check whether they call any of the functions below on one.

- **Write in place into the argument's planes:**
  - `WebPCleanupTransparentArea`: the Y, U and V planes (YUV), or the ARGB samples.
  - `WebPBlendAlpha`: the Y, U, V and A planes (YUV), or the ARGB samples.
  - `WebPReplaceTransparentPixels`: the ARGB samples.
  - `WebPPictureRescale`: before rescaling, multiplies the source's luma by its alpha
    (YUV with alpha) or the ARGB samples by their alpha. The call leaves them
    multiplied when it fails, as C does; on success the picture is replaced.
- **Write only to planes that they allocate for the argument**, which releases the
  caller's planes without writing them: `WebPPictureAlloc` and its allocators,
  `WebPPictureImport*`, `WebPPictureARGBToYUVA` and its forms, and `WebPPictureYUVAToARGB`.
- **Read the argument and write only new planes:** `WebPPictureCrop`,
  `WebPPictureCopy`, `WebPPictureView` (no write at all), `WebPPictureDistortion` (it
  writes only the ARGB planes of its two views), `WebPPictureHasTransparency` and
  `WebPPlaneDistortion`.

Each function in the first group panics when the argument is a view.

## Undefined behavior excluded from the oracle

Rule 8 of PORTING: the C code has undefined behavior on these inputs, so they are
excluded from the comparison, in every variant.

- **Misaligned `uint32_t` reads in the RGBA import.**
  `WebPPictureImportRGBA` with an ARGB picture reads each row through
  `VP8LConvertBGRAToRGBA((const uint32_t*)rgb, ...)` (`upstream/libwebp/src/dsp/lossless.c`).
  When the row stride is not a multiple of 4 and the picture has more than one row,
  the row starts off a 4-byte boundary, and the `asan-ubsan` variant reports the
  misaligned load. The import test excludes these inputs (`kind == rgba`, ARGB,
  `stride % 4 != 0`, `height > 1`). The Rust translation reads the same bytes in
  native order, which is the value the C code computes where unaligned loads are
  tolerated; the Rust side defines that reading, and the exclusion keeps the
  oracle from proving it.

No other undefined behavior appeared in the WP9 probes under `asan-ubsan`, and the
`debug` variant's upstream assertions held for every compared input.

## Upstream behavior preserved

- `WebPPlaneDistortion` reads an unpacked plane (`x_step` 1) with the stride
  `width`, not with `src_stride`. The metric receives `width` as its stride in both
  forms, so a padded unpacked plane is measured incorrectly. The Rust translation
  keeps this, and the distortion test compares it (padded cases with `x_step` 1).
- `WebPEncodingSetError` keeps the oldest error. This is the precedence the brief
  names; the test sequence compares it.
