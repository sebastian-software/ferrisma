# WP9: encoder picture and config

- Status: partly. Every unit of the WP9 inventory is translated except three
  functions that stay absent until the integrator decides a design question that
  ADR 0003 leaves open: `WebPPictureView`, `WebPPictureCopy` and
  `WebPPictureDistortion` (see [Open design questions](#open-design-questions)).
- Sub-wave: 2a
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1, WP3, WP4, WP5 (all merged)
- Probes: `tools/oracle/probes/wp9/` (with `support/`); tests: `tests/wp9_*.rs`
  and the shared test support `tests/support/wp9_pictures.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

Every row names what is translated, with `// upstream:` markers on each item.
"Partly" rows name what stays absent and why.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/enc/config_enc.rs` | `src/enc/config_enc.c` | yes: `WebPConfigInitInternal`, `WebPValidateConfig`, `WebPConfigLosslessPreset`, `MAX_LEVEL` and the table `kLosslessPresets` (`LOSSLESS_PRESETS`). |
| `src/enc/picture_enc.rs` | `src/enc/picture_enc.c` | partly: `DummyWriter` (as the discarding writer), `WebPPictureInitInternal`, `WebPValidatePicture`, `WebPPictureResetBuffers` with the two resets, `WebPPictureAllocARGB`, `WebPPictureAllocYUVA`, `WebPPictureAlloc`, `WebPPictureFree`, `WebPMemoryWriterInit`, `WebPMemoryWrite` and `WebPMemoryWriterClear`. Not here: `Encode` and the eight `WebPEncode*` functions (WP13). The Rust-only plane helpers at the end of the file have no C counterpart and are named as such. |
| `src/enc/picture_csp_enc.rs` | `src/enc/picture_csp_enc.c` | partly: `WebPPictureHasTransparency`, the gamma tables (`LazyLock`), `RGBToY/U/V`, `PreprocessARGB`, `SUM4`, `SUM2`, `SUM2ALPHA`, `SUM4ALPHA`, `kInvAlpha` (`INV_ALPHA`), `DIVIDE_BY_ALPHA`, `LinearToGammaWeighted`, `ConvertRowToY`, `AccumulateRGBA`, `AccumulateRGB`, `ConvertRowsToUV`, `ImportYUVAFromRGBA`, `PictureARGBToYUVA`, `WebPPictureARGBToYUVA`, `WebPPictureARGBToYUVADithered`, `WebPPictureSharpARGBToYUVA`, `WebPPictureSmartARGBToYUVA`, `WebPPictureYUVAToARGB`, `Import`, and `WebPPictureImportBGR`, `BGRA`, `BGRX`, `RGB`, `RGBA`, `RGBX`. Not here: `SharpYuvInit` (WP5 deviation 1) and the `Init` functions of ADR 0003, which assign pointers only. |
| `src/enc/picture_rescale_enc.rs` | `src/enc/picture_rescale_enc.c` | partly: `WebPPictureIsView`, `WebPPictureCrop`, `WebPPictureRescale` with `RescalePlane`, `AlphaMultiplyARGB`, `AlphaMultiplyY`, and `PictureGrabSpecs` (in the form that moves the callbacks), `AdjustAndCheckRectangle`, `SnapTopLeftPosition`, `HALVE`. Absent: `WebPPictureCopy` and `WebPPictureView` (open design questions). |
| `src/enc/picture_tools_enc.rs` | `src/enc/picture_tools_enc.c` | yes: `WebPReplaceTransparentPixels`, `WebPCleanupTransparentArea` with `IsTransparentARGBArea`, `Flatten`, `FlattenARGB` and `SmoothenBlock`, `WebPBlendAlpha` with `BLEND`, `BLEND_10BIT` and `MakeARGB32`. |
| `src/enc/picture_psnr_enc.rs` | `src/enc/picture_psnr_enc.c` | partly: `WebPPlaneDistortion` with `AccumulateSSE`, `AccumulateSSIM`, `AccumulateLSIM`, `GetPSNR` and `GetLogSSIM`. Absent: `WebPPictureDistortion` (it builds two views; open design question). |
| `src/enc/webp_enc.rs` (exception) | `src/enc/webp_enc.c`: `WebPEncodingSetError`, `WebPReportProgress` | yes. |
| `src/webp/encode.rs` (exception) | `src/webp/encode.h`: the inline wrappers `WebPConfigInit`, `WebPConfigPreset` and `WebPPictureInit` | yes. The WP1 content of the file is unchanged. |

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
| `tests/wp9_picture.rs` | `importers_match_the_c_importers_on_padded_strides` | `WebPPictureImportRGB`, `RGBA`, `BGR`, `BGRA`, `RGBX` and `BGRX` in both the YUV and the ARGB form, on 6 geometries, with and without 3 bytes of row padding, on seeded random buffers: the return value, the error code, the colorspace, and the visible samples of the planes (the alpha plane only when it exists). |
| `tests/wp9_picture.rs` | `argb_to_yuva_conversions_match_the_c_conversions` | `WebPPictureARGBToYUVA`, `WebPPictureARGBToYUVADithered` (dithering 0, 0.5 and 1) and `WebPPictureSharpARGBToYUVA`, for colorspaces 0 and 4, an invalid colorspace, and sizes from 1 by 1 to 9 by 6 with padded strides. |
| `tests/wp9_picture.rs` | `yuva_to_argb_conversion_matches_the_c_conversion` | `WebPPictureYUVAToARGB` on caller-owned planes with and without alpha, odd and even sizes, padded strides, including the even and odd last rows. |
| `tests/wp9_tools.rs` | `transparent_area_cleanup_matches_the_c_cleanup` | `WebPCleanupTransparentArea` on 60 seeded pictures (ARGB and YUVA, with and without alpha, sizes from 1 to 20 with remainders, padded strides) with transparent 8 by 8 blocks; compares every byte of every buffer, padding included. Asserts that some case changed a sample. |
| `tests/wp9_tools.rs` | `alpha_blending_matches_the_c_blending` | `WebPBlendAlpha` on 60 seeded pictures with random backgrounds; every byte compared. Asserts that some case changed a sample. |
| `tests/wp9_tools.rs` | `transparency_detection_matches_the_c_detection` | `WebPPictureHasTransparency` on 60 seeded pictures, both forms. |
| `tests/wp9_tools.rs` | `transparent_pixel_replacement_matches_the_c_replacement` | `WebPReplaceTransparentPixels` on 40 seeded ARGB pictures with random colors. |
| `tests/wp9_rescale.rs` | `rescale_matches_the_c_rescale` | `WebPPictureRescale` on 60 seeded pictures to 10 target sizes: keep-aspect (0), down, up, 1 by 1, odd sizes, and the invalid -1. Compares the return value, the new size, the colorspace, the error code and the visible samples. Asserts that some rescale succeeded. |
| `tests/wp9_rescale.rs` | `crop_matches_the_c_crop` | `WebPPictureCrop` on 80 seeded pictures with random rectangles, including odd corners (the chroma position is snapped) and rectangles outside the picture. Asserts that some crop succeeded. |
| `tests/wp9_distortion.rs` | `plane_distortion_matches_the_c_metrics_bit_for_bit` | `WebPPlaneDistortion` for the three metrics (0, 1, 2), with `x_step` 1 and 4, planes from 1 by 1 to 17 by 5, padded strides, identical planes, and a stride one byte short of a row (which fails). Compares the return value and the bits of both floats, which start at -1 so an untouched output is visible. Asserts that some result is not the maximum. |
| `tests/wp9_writer_errors.rs` | `writer_errors_and_progress_match_the_c_functions` | One sequence in one process: `WebPMemoryWriterInit`, six `WebPMemoryWrite` calls (through the 8192-byte minimum and a doubling, one empty write), `WebPMemoryWriterClear`, `WebPPictureInit`, five `WebPEncodingSetError` calls (the oldest error is kept), a progress hook that aborts below its threshold, five `WebPReportProgress` calls with a store and one with a `NULL` store. |

Local results on this branch (release oracle, the reference):
`python3 tools/check.py --oracle --msrv` passed, with format, Clippy with
`-D warnings` on all targets, the module skeleton, `tools/unit_dependencies.py
--check` (0 undeclared references), the tests in the dev, release and `checked`
profiles, and the build and tests with Rust 1.95: 168 test results, none failed.
The `debug` variant (`python3 tools/check.py --oracle-variant debug --stage test`)
reports 126 results, none failed, with upstream's assertions enabled. The
`asan-ubsan` variant reports 126 results, none failed, and no sanitizer report,
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
   before its start, so a negative row start panics (see the deviations). The
   tests do not generate them. `WebPPlaneDistortion` with a negative width returns
   `false` in C and Rust by the same comparison, but is not compared.
6. The padding byte of `WebPPictureImportRGBX` and `BGRX` (not read), and the
   alignment of the ARGB plane (not observable, see the deviations).
7. `WebPPictureView`, `WebPPictureCopy` and `WebPPictureDistortion` (absent; open
   design questions).
8. SIMD and threads: none in the reference configuration.
9. Cross-host floating point. The distortion results are compared bit for bit on
   this host (macOS on Apple Silicon with Apple clang and `-ffp-contract=off`).
   Equality on other hosts is recorded, not assumed.

## Recorded deviations

Deviations are changes of representation that ADR 0003 allows, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Errors.** `WebPEncodingSetError` returns the code that the picture holds
   afterwards, which is the code that a caller propagates with `Err`. The setter
   keeps the oldest error, as C does. Functions whose every failure path goes
   through the setter return `Result<(), WebPEncodingError>`: `WebPValidatePicture`,
   the allocators, `WebPPictureAlloc`, the ARGB-to-YUVA conversions, `WebPPictureYUVAToARGB`,
   `WebPPictureRescale` and `WebPReportProgress`. Functions with a failure that
   records no code stay `bool`, with the code left on the picture as in C:
   `WebPPictureImport*` (the stride test), `WebPPictureCrop` (the rectangle test),
   `WebPPlaneDistortion`, `WebPMemoryWrite`, and the predicates.
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
6. **Native-order byte views.** C reads and writes ARGB samples as bytes in four
   places: the ARGB-to-YUVA conversion, the YUVA-to-ARGB upsampler output, the
   RGBA import through `VP8LConvertBGRAToRGBA`, and the rescaling of ARGB pictures.
   The Rust code makes copies in native byte order (`to_ne_bytes`,
   `from_ne_bytes`), converts, and copies the words back. The values are the same;
   the copies cost one temporary of the plane size.
7. **Negative strides.** A row start before the start of its slice panics. C reads
   memory before the pointer in that case, which is valid only when the caller's
   memory extends there. The safe API (WP17) decides whether it takes a base offset.
8. **Callbacks.** The writer and the progress hook are closures with no picture
   parameter (ADR 0003). `WebPMemoryWrite` takes the `WebPMemoryWriter` as the
   closure's context: `picture.writer = Some(Box::new(move |data| webp_memory_write(&mut writer, data)))`.
   `DummyWriter` is a plain function boxed as the default writer.
9. **Rescale and crop.** C copies the whole picture struct, including the callbacks
   and the caller's `extra_info` and `stats`, into a temporary and replaces the
   picture with it. A boxed closure cannot be copied, so the Rust code moves the
   callbacks and the two references into the temporary and moves them back when the
   operation fails. The outcome is the same as C's.
10. **Missing planes.** `WebPCleanupTransparentArea` and `WebPBlendAlpha` return
    unchanged when a plane they need is missing, and `WebPReplaceTransparentPixels`
    returns when the ARGB plane is missing. C dereferences the missing plane for
    every block, which is a precondition violation; the cases are not generated.
11. **Init functions.** `WebPInitAlphaProcessing`, `WebPInitConvertARGBToYUV`,
    `VP8LDspInit` and `VP8SSIMDspInit` only assign portable pointers and are omitted
    (ADR 0003). `InitGammaTables` becomes two `LazyLock` tables, which
    `init_gamma_tables()` forces where C calls it. `SharpYuvInit` is omitted, as
    WP5 records.
12. **`WebPMemoryWriter.mem`.** Its length is `max_size`, and `WebPMemoryWrite`
    keeps `size` in step with it (WP1 deviation 17). `WebPMemoryWriterInit` leaves
    `mem` empty, which is C's `NULL`.
13. **`use_delta_palette`.** `WebPConfigInit` does not write this reserved field in
    C, so it keeps the caller's value. The Rust translation also leaves it alone,
    and the test compares it as unchanged.
14. **`WebPPictureIsView`.** A picture counts as a view when its storage holds no
    memory of its own, which is C's `memory_ == NULL`. A caller's storage is always
    a view.

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

## Open design questions

These two stop the translation. ADR 0003 names the first one and leaves the second
open; neither has a decision yet, and nothing in this package decides them.

1. **`WebPPictureView` and `WebPPictureDistortion` need a shared borrow of another
   picture's planes.** A view points into the source's Y, U, V, A or ARGB planes
   and is read by the conversions, which write only to their own planes. The owned
   and mutable `PlaneStorage` of ADR 0003 cannot express that borrow.
   `WebPPictureDistortion` builds two views and converts them with
   `WebPPictureYUVAToARGB`, so it depends on the same decision.
   *Options:* (A) add `PlaneStorage::Shared(Vec<&'a [T]>)`, which amends ADR 0003
   (ADR 0003 already records this); a view then has shared storage, and its
   conversion writes only the owned ARGB plane. (B) return a separate read-only
   view type, which changes the API of every consumer.
   *Recommendation:* A.

2. **`WebPPictureCopy` duplicates the picture's callbacks and its caller's
   references.** C copies the writer and the progress hook as pointers, so both
   pictures share one context; it also copies the `extra_info` and `stats`
   pointers. A boxed closure cannot be copied from a `&WebPPicture`, and a `&mut`
   reference cannot be shared at all.
   *Options:* (A) keep the writer and the progress hook in shared
   `Rc<RefCell<...>>` cells, which amends the concrete callback forms of ADR 0003,
   and give the copy `extra_info` and `stats` as `None`, a documented deviation.
   (B) take `&mut` source, which would leave the source without its callbacks
   afterwards, so it is not faithful. (C) give the copy no writer or hook, which
   changes what a later encode does with it.
   *Recommendation:* A, with the `extra_info` and `stats` deviation accepted
   explicitly, since no shared form of a `&mut` exists.
