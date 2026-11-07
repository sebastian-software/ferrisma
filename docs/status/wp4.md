# WP4: pixel ops

- Status: every unit of the WP4 inventory is translated, except the parts that
  the reference configuration does not compile, the `Init` functions that only
  assign portable pointers (ADR 0003), and the SIMD variants (wave 4). Each of
  those is listed under not-applicable checks or recorded deviations.
- Sub-wave: 1b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1 only
- Probes: `tools/oracle/probes/wp4/`; tests: `tests/wp4_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`; WP4 owns no function- or type-level exception.

## Inventory

Every row names what is translated; each item carries a `// upstream:` marker.
Names follow ADR 0001 (`tools/rust_name.py`).

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dsp/dsp.rs` | `src/dsp/dsp.h` | partly: `BPS`, `MAX_COEFF_THRESH`, `VP8Histogram`, `VP8DistoStats`, `VP8_SSIM_KERNEL`, the `VP8_DITHER_*` constants, `WEBP_TRANSFORM_AC3_C1` and `_C2` with the macros `webp_transform_ac3_mul1` and `_mul2`, and the enum `WEBP_FILTER_TYPE` with its enumerators. The function-pointer typedefs have no counterpart of their own (deviation 1); the declarations of WP3, WP6 and WP10 are theirs. |
| `src/dsp/cpu.rs` | `src/dsp/cpu.c`, `src/dsp/cpu.h` | yes, for the x86 query and the reference configuration: `CPUFeature` with its enumerators, `VP8CPUInfo`, `get_cpu_info`, `x86cpu_info`, `check_slow_model`, and `vp8_get_cpu_info` (deviation 13). The platform queries of cpu.c (Android, ARM, WASM, MIPS) are not compiled in the reference configuration. |
| `src/dsp/yuv.rs` | `src/dsp/yuv.c`, `src/dsp/yuv.h` | yes, portable: the `YUV_*` constants, `mult_hi`, `vp8_clip8`, `vp8yuv_to_r/g/b`, `vp8_yuv_to_rgb`, `_bgr`, `_rgb565`, `_rgba4444`, `_argb`, `_bgra`, `_rgba`, `vp8_clip_uv`, `vp8rgb_to_y/u/v`, the row function `row_func` (ROW_FUNC) with its seven row kernels, `webp_sampler_process_plane`, the table `WEBP_SAMPLERS` behind `webp_samplers`, and the converters `webp_convert_argb_to_y`, `_argb_to_uv` (with `_c`), `_rgb24_to_y`, `_bgr24_to_y` and `_rgba32_to_uv` (with `_c`). The SSE2 and SSE41 wrappers belong to wave 4. |
| `src/dsp/upsampling.rs` | `src/dsp/upsampling.c` | yes: `upsample_line_pair` (UPSAMPLE_FUNC) with its seven line-pair upsamplers, the table `WEBP_UPSAMPLERS` behind `webp_upsamplers`, `webp_get_line_pair_converter`, `yuv444_row` (YUV444_FUNC) with its seven converters, and the table `WEBP_YUV444_CONVERTERS` behind `webp_yuv444_converters`. |
| `src/dsp/alpha_processing.rs` | `src/dsp/alpha_processing.c` | yes, little-endian and without the multiplication tables: `webp_mult_argb_row` (with `_c`), `webp_mult_row` (with `_c`), `webp_mult_argb_rows`, `webp_mult_rows`, `webp_apply_alpha_multiply` and `_4444`, `webp_dispatch_alpha`, `webp_dispatch_alpha_to_green`, `webp_extract_alpha`, `webp_extract_green`, `webp_pack_rgb`, `webp_has_alpha8b`, `webp_has_alpha32b`, `webp_alpha_replace`, and their `_c` kernels with the helpers `mult`, `get_scale`, `premultiply`, `multiply`, `dither_hi`, `dither_lo`, `make_argb32`. |
| `src/dsp/filters.rs` | `src/dsp/filters.c` | yes: `predict_line_c`, `gradient_predictor_c`, the three forward filters (`do_horizontal_filter_c` and its siblings), the four unfilters, the tables `WEBP_FILTERS` and `WEBP_UNFILTERS` behind `webp_filters` and `webp_unfilters`. |
| `src/utils/filters_utils.rs` | `src/utils/filters_utils.c`, `src/utils/filters_utils.h` | yes: `sdiff`, `gradient_predictor`, `webp_estimate_best_filter`. |
| `src/dsp/rescaler.rs` | `src/dsp/rescaler.c` | yes: the four row kernels `webp_rescaler_import_row_expand_c`, `_shrink_c`, `webp_rescaler_export_row_expand_c`, `_shrink_c`, their dispatch functions, and `webp_rescaler_import_row` and `webp_rescaler_export_row`. |
| `src/utils/rescaler_utils.rs` | `src/utils/rescaler_utils.c`, `src/utils/rescaler_utils.h` | yes: `WEBP_RESCALER_RFIX`, `WEBP_RESCALER_ONE`, `rescaler_t`, `webp_rescaler_frac`, the struct `WebPRescaler` (deviation 8), `webp_rescaler_init`, `webp_rescaler_get_scaled_dimensions`, `webp_rescale_needed_lines`, `webp_rescaler_import`, `webp_rescaler_export`, `webp_rescaler_input_done`, `webp_rescaler_output_done`, `webp_rescaler_has_pending_output`. |
| `src/utils/quant_levels_utils.rs` | `src/utils/quant_levels_utils.c`, `src/utils/quant_levels_utils.h` | yes: `NUM_SYMBOLS`, `MAX_ITER`, `ERROR_THRESHOLD`, `quantize_levels`. |
| `src/utils/quant_levels_dec_utils.rs` | `src/utils/quant_levels_dec_utils.c`, `src/utils/quant_levels_dec_utils.h` | yes, without dithering: the constants `FIX`, `LFIX`, `LUT_SIZE`, `DFIX`, `CLIP_8B_MASK`, `clip_8b`, the struct `SmoothParams` (offsets into its buffers), `v_filter`, `h_filter`, `apply_filter`, `init_correction_lut`, `count_levels`, `init_params`, and `webp_dequantize_levels`. `CleanupParams` is the drop of the vectors (not a function here). |
| `src/dsp/ssim.rs` | `src/dsp/ssim.c` | yes: `WEIGHT` (kWeight), `WEIGHT_SUM` (kWeightSum), `ssim_calculation`, `vp8ssim_from_stats`, `vp8ssim_from_stats_clipped`, `ssim_get_clipped_c` and `vp8ssim_get_clipped`, `ssim_get_c` and `vp8ssim_get`, `accumulate_sse_c` and `vp8_accumulate_sse`. |

Not translated, and why, is listed under not-applicable checks.

## Evidence

Every differential test runs the C probe in a separate process, against the
release oracle built from the pinned `v1.6.0` source with
`python3 tools/build_oracle.py`; `python3 tools/check.py --oracle` runs them
with the `oracle` feature, so a missing oracle fails instead of skipping.

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp4_constants.rs` | `header_constants_match_the_c_headers` | the constants, enumerators, macros and the two `repr(C)` statistics structs of the WP4 headers, with the offsets of their fields, against the C compiler. The C `sizeof(WebPRescaler)` is compared with `WebPRescaler::C_SIZE`; the Rust struct is not `repr(C)`. |
| `tests/wp4_cpu.rs` | `cpu_query_matches_the_c_query` | `VP8GetCPUInfo` for the ten features on the running host. On this arm64 host the reference configuration has the pointer NULL, and the Rust query must answer `false` for every feature. |
| `tests/wp4_cpu.rs` | `slow_model_table_matches_the_c_helper` | `CheckSlowModel` over 384 leaf-1 EAX values (six families, four extended models, sixteen low nibbles). x86 only; see not-applicable checks. |
| `tests/wp4_yuv.rs` | `yuv_conversions_match_the_c_functions` | the row samplers `WebPSamplers[mode]` for all eleven modes over lengths 0 to 65 with random planes; `WebPSamplerProcessPlane` over padded planes of odd sizes with loose and tight strides; the ARGB, RGB, BGR and RGBA converters to Y and UV for widths 0 to 33, both store modes; the inline YUV to RGB conversions over corner values and 1500 seeded triples; and `VP8RGBToY/U/V` for five roundings. |
| `tests/wp4_upsampling.rs` | `upsamplers_match_the_c_functions` | `WebPUpsamplers[mode]` for all eleven modes over lengths 1 to 40 and 63 to 65, with and without the bottom row; `WebPGetLinePairConverter` for both alpha orders; and `WebPYUV444Converters[mode]` over lengths 0 to 40 and 63 to 65. |
| `tests/wp4_alpha.rs` | `alpha_processing_matches_the_c_functions` | ARGB and single-value premultiplication and un-premultiplication with edge alpha values, the strided variants, the planes in RGBA, BGRA, ARGB and RGBA4444, the dispatch and extraction of alpha planes including all-opaque planes, the RGB and BGR packing, the alpha tests over lengths 0 to 40, and `WebPAlphaReplace`. |
| `tests/wp4_filters.rs` | `filters_match_the_c_functions` | the three forward filters over padded planes of odd sizes, smooth and random; the four unfilters with and without a previous row, in the three aliasing forms (distinct, in-place, and previous row equal to the output); and `WebPEstimateBestFilter` over random and smooth planes. |
| `tests/wp4_rescaler.rs` | `rescaler_matches_the_c_functions` | `WebPRescalerGetScaledDimensions` over zeros, negatives and large values, and the rescaler driven step by step (`WebPRescaleNeededLines`, `WebPRescalerImport`, `WebPRescalerExport`) for seventeen size pairs, identities, shrinks, expansions, extreme scales, the single-column special case, padded strides and one to four channels, with import chunks of 1, 3 and 1000 rows. Each step prints the scaling state and the output buffer is compared byte for byte. The side rows start at a nonzero value, so the comparison checks that `webp_rescaler_init` zeroes them as C does; the C probe starts from zeroed memory. `init_panics_on_a_work_vector_of_the_wrong_length` is a Rust-only test of the length check. |
| `tests/wp4_quant_levels.rs` | `quantize_levels_matches_the_c_function` | `QuantizeLevels` over seven plane sizes, the level counts 1 to 257, five levels of distinct values, with and without the error output; missing planes and invalid dimensions. |
| `tests/wp4_quant_levels.rs` | `dequantize_levels_matches_the_c_function` | `WebPDequantizeLevels` over seven plane sizes, the strengths -1 to 101, four level counts, padded strides, and the invalid parameters. |
| `tests/wp4_ssim.rs` | `ssim_and_squared_error_match_the_c_functions` | `VP8SSIMFromStats` and `VP8SSIMFromStatsClipped` on the statistics of 300 real windows, `VP8SSIMGetClipped` on 300 planes with edge windows, `VP8SSIMGet` on 200 7-row planes, and `VP8AccumulateSSE` for every third length from 0 to 300. Results are compared as their 64-bit patterns. |

Local result on this branch: `python3 tools/check.py --oracle` passed on the
final tree, and again after the rescaler shape change, with `--msrv` and with
`--oracle-variant asan-ubsan`. Each run covered format, Clippy with `-D warnings`
on all targets and features, the oracle build and the unit dependency check, and
the tests of the dev, release and `checked` profiles; the `--msrv` run also built
and tested with the toolchain named by `rust-version`.

## Not-applicable checks

These checks do not apply to WP4, or cannot be made on this host, with the reason:

1. **x86 query on this host.** `x86cpu_info`, `check_slow_model` and the x86
   branch of `vp8_get_cpu_info` are compiled only on x86 targets. The host of this
   session is arm64, so the x86 test `slow_model_table_matches_the_c_helper` and the
   x86 answers of `cpu_query_matches_the_c_query` are not run here. They are
   type-checked for `x86_64-unknown-linux-gnu` with clippy, and they run on the
   Linux x86-64 CI host (ADR 0002).
2. **SIMD variants.** The SSE2, SSE41 and NEON variants and their wrappers
   (`VP8YuvToRgba32_SSE2` and the others, `alpha_processing_sse2.c` and the like)
   are wave 4 (WP18 to WP20). The `WEBP_HAVE_*` guards around the `Init`
   calls are false in the reference configuration.
3. **Platform queries.** `AndroidCPUInfo`, `armCPUInfo`, `wasmCPUInfo` and
   `mipsCPUInfo` (cpu.c) are selected only by macros the reference configuration
   does not define.
4. **Big-endian sources.** `PackARGB_C` and the `WebPPackARGB` pointer
   (`WORDS_BIGENDIAN`) are not compiled on little-endian hosts, which the reference
   configuration is.
5. **Compile-time switches off in the reference configuration.** `WEBP_SWAP_16BIT_CSP`
   (the swapped RGB-565 and RGBA-4444 byte order, `YUV_*` writers under
   `WEBP_SWAP_16BIT_CSP == 1`, and `ApplyAlphaMultiply_16b_C` with 1) is 0, so the
   standard byte order is translated only. `WEBP_REDUCE_CSP` is unset, so
   `EmptyUpsampleFunc` and `EmptyYuv444Func` are not translated.
   `USE_DITHERING` is undefined, so the ordered dither table and its branches are
   not translated. `USE_TABLES_FOR_ALPHA_MULT` is 0, so `kMultTables` is not
   translated. `FANCY_UPSAMPLING` is defined by `dsp.h`, so the `DUAL_SAMPLE`
   functions are not compiled.
6. **`Init` functions.** `VP8FiltersInit`, `VP8SSIMDspInit`,
   `WebPInitAlphaProcessing`, `WebPInitConvertARGBToYUV`, `WebPInitSamplers`,
   `WebPInitUpsamplers`, `WebPInitYUV444Converters` and `WebPRescalerDspInit`
   only assign portable pointers, and ADR 0003 omits them. Their SIMD branches
   belong to wave 4.
7. **`WEBP_DSP_INIT`, `WEBP_DSP_INIT_FUNC` and `WEBP_DSP_INIT_STUB`** are the
   once-only guards around those `Init` functions and the stubs that expose a
   symbol per module. No counterpart is needed (ADR 0003).
8. **Preprocessor and sanitizer macros** (`LOCAL_GCC_*`, `LOCAL_CLANG_*`,
   `WEBP_NEON_*`, `WEBP_TSAN_IGNORE_FUNCTION`, `WEBP_UBSAN_*`, `WEBP_MSAN`,
   `WEBP_OFFSET_PTR`, `WEBP_RESTRICT`, `__has_builtin`): compiler and sanitizer
   plumbing that Rust expresses with `cfg` or has no need for.
9. **Floating point.** `QuantizeLevels`, the SSIM functions and the `double`
   results of the rescaler's helpers are compared bit for bit on this host (arm64,
   Apple clang, `-ffp-contract=off`). Equality on other hosts is recorded, not
   assumed (PORTING).
10. **Assertions.** The C `assert`s are `debug_assert!` (ADR 0003). The SSIM
    assertion `0 <= r <= 1` holds for the statistics of real windows, which the
    tests use; the statistics of arbitrary sums can fail it in the C debug build
    too, and no caller of the public API produces them.
11. **`VP8GetCPUInfo` NULL on non-x86.** In the reference configuration the pointer
    is NULL on arm64; the probe reports `null=1`, and the Rust query answers
    `false`. The NULL value exists only at the C ABI (ADR 0004).

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Function-pointer typedefs.** The typedefs of `dsp.h` for the dispatch
   pointers have no Rust type of their own. Each dispatch pointer is a plain
   function with the snake-case name of the pointer, and its signature is the
   one of the module that implements it (ADR 0003). The aliases
   `WebPSamplerRowFunc`, `WebPUpsampleLinePairFunc`, `WebPYUV444Converter`,
   `WebPFilterFunc` and `WebPUnfilterFunc` are kept where a parameter or a table
   has the type.
2. **Tables filled by `Init`.** `WebPSamplers`, `WebPUpsamplers`,
   `WebPYUV444Converters`, `WebPFilters` and `WebPUnfilters` are static arrays of
   the portable functions, indexed through functions that take the mode or the
   filter first (ADR 0003). The entries that C leaves NULL (the YUV modes, and the
   filter `WEBP_FILTER_NONE` for the forward filter) panic when called, where C
   calls NULL. The caller never reaches them in the library.
3. **`WebPGetLinePairConverter`** returns the function pointer of the table,
   `WebPUpsampleLinePairFunc`. The C caller `picture_csp_enc.c` calls the returned
   function, so a getter that returns a function type is the faithful form.
4. **`WebPSamplerProcessPlane`** takes the row function as a generic `Fn`. C passes
   `WebPSamplers[mode]`; `io_dec.c` does the same, so the Rust caller passes a
   closure that calls `webp_samplers`.
5. **Bottom row of a line pair.** C passes `bottom_y` and `bottom_dst`, which are
   NULL together in use. The Rust form takes one `Option<(&[u8], &mut [u8])>`.
   C also allows `bottom_dst` without `bottom_y`, where it is not written; the Rust
   callers pass `None` there.
6. **Pointers and strides.** A pointer into a buffer is a slice and an offset; an
   `int` stride is kept as `i32` in the signature and converted to `usize` with a
   checked conversion. A row outside its buffer panics where C would read or
   write outside it, which is undefined.
7. **`WebPMultARGBRows`** casts a byte pointer to `uint32_t*` in C. Rust has no
   safe reinterpretation, so the function takes `&mut [u32]` and a stride in `u32`
   elements. The probe calls the C function with the stride in bytes, which
   reaches the same pixels.
8. **`WebPRescaler`.** The C `dst` pointer is the caller's output buffer. The
   struct does not hold it: `webp_rescaler_export_row` and `webp_rescaler_export`
   take the buffer on each call, and the struct keeps `dst_pos`, the start of the
   next output row, and `dst_stride`. The stride may be negative, as for a
   flipped buffer, and the owner may set `dst_pos` to the start of the first row
   (fixed for WP6 on 2026-11-07). `irow` and `frow` are the two side rows,
   owned by the struct. `webp_rescaler_init` takes the work vector by value and
   splits it, and the import swaps the two vectors, which is what the C pointer
   swap does. The struct has no lifetime parameter, so a decoder can store it
   beside its work and output buffers (ADR 0003, readers over input).
   `fx_scale` is not set when the rescaler expands in x, and
   `fxy_scale` is not set when it expands in y; C leaves them unset and never reads
   them on those paths, and Rust stores 0. The flags are `bool`. The C size of the
   struct is `WebPRescaler::C_SIZE`, computed from the pointer size as WP3 computes
   its sizes: 104 on 64-bit targets, where the probe confirms the value, and 84 on
   32-bit targets, where it is computed but not run here. The caller that carves
   a block from one allocation (ADR 0003) will use it for the size check: WP6 and
   WP7 are the first callers.
9. **`WebPRescalerInit`** returns `Option` and never returns `None` on a 64-bit
   target, because `CheckSizeOverflow` accepts every size that fits `size_t`. The
   work vector must hold exactly `2 * dst_width * num_channels` elements; the size
   check runs first, then a vector of another length panics. The side rows are
   zeroed, as the C `memset` zeroes them. A non-positive `dst_width` or channel
   count panics, which the C code leaves undefined.
10. **`WebPRescalerGetScaledDimensions`** writes its two arguments only on
    success, as in C. Its 64-bit products wrap as the `uint64_t` arithmetic does.
11. **`QuantizeLevels` and `WebPDequantizeLevels`.** A NULL plane is `None`, and a
    NULL `sse` is `None`. The early returns leave `sse` and the plane unchanged,
    as in C. The conversions of the rounded centroids to `uint8_t` and of the error
    to `uint64_t` are in range for every valid input.
12. **Uninitialized scratch in `WebPDequantizeLevels`.** `VFilter` reads the first
    `2 * radius + 1` rows of its ring before any row has been stored there, and
    the C allocation (`WebPSafeMalloc`) is not initialized. Reading it is undefined
    in C, and Rust zero-initializes the scratch (ADR 0005, PORTING rule 8). The
    probe ran 400 random planes with `MallocScribble=1`, which fills allocations
    with `0xaa`, and the output did not change. The differential test compares the
    C output as the build prints it. The probe does not fill the heap itself: a fill
    before the call did not reach the allocation on this host.
13. **`VP8GetCPUInfo`.** The x86 query uses `core::arch::x86_64::__cpuid_count`
    (safe to call) for the cpuid leaves. The `xgetbv` test of `kAVX` is the standard
    library's `is_x86_feature_detected!("avx")`, which performs the same XCR0 check.
    On other targets the reference configuration's NULL pointer answers `false`
    for every feature.
14. **Enumerations.** `CPUFeature` and `WEBP_FILTER_TYPE` are `#[repr(transparent)]`
    newtypes over `u32` (ADR 0003). Their enumerators keep the C names; the
    lower-case `kSSE2`-style enumerators are under an impl-level
    `non_upper_case_globals` expectation.
15. **In-place unfilters.** The C unfilters may read their input and write their
    output in the same buffer. Each unfilter reads an input element before it
    writes the same index, so a copy of the input gives the same bytes. The
    filters test runs the in-place and previous-row-equals-output forms against C.
16. **Assertions and overflow.** `assert` is `debug_assert!`, which is active in the
    dev and `checked` profiles like upstream's debug build. Wrapping is spelled
    out where the C unsigned arithmetic wraps (the `uint32_t` and `uint64_t`
    products of the filters, the rescaler and the SSIM), and plain arithmetic is
    used where the C code cannot overflow.
17. **`ssim_calculation` uses `f64` division** of the two 64-bit sums, as the C
    `double` division does; the result is compared bit for bit on this host.
