# WP5: SharpYUV

- Status: the color space and matrix functions, the transfer functions and gamma
  tables, the dispatch functions and the conversion core are translated and
  verified. Not yet translated: `sharpyuv/sharpyuv_cpu.c` (`sharpyuv_cpu.rs`), which
  waits for WP4 (`src/dsp/cpu.rs` is not on `main`). `SharpYuvInit` stays absent for
  the reason given under [decisions](#decisions-of-the-integrator). The package is a
  draft pull request until WP4 is merged and `sharpyuv_cpu.rs` is in.
- Sub-wave: 1b (merges after WP4)
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1; WP4 for `sharpyuv_cpu.rs` only
- Probes: `tools/oracle/probes/wp5/` (with `support/`); tests: `tests/wp5_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). The inventory comes from `tools/units.json`,
which assigns the five modules to this package; the package adds no exceptions.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/sharpyuv/sharpyuv.rs` | `sharpyuv/sharpyuv.h`, `sharpyuv/sharpyuv.c` | partly: `SHARPYUV_VERSION*`, `SHARPYUV_MAKE_VERSION`, `SharpYuvGetVersion`, `SharpYuvConversionMatrix`, `SharpYuvTransferFunctionType` with its 16 valid enumerators and the count, `SharpYuvOptions`, `SharpYuvOptionsInit`, `SharpYuvOptionsInitInternal`, `SharpYuvConvert` and `SharpYuvConvertWithOptions` with the whole conversion core (`GetPrecisionShift`, `clip*`, `RGBToGray`, `ScaleDown`, `UpdateW`, `UpdateChroma`, `StoreGray`, `Filter2`, `Shift`, `ImportOneRow`, `InterpolateTwoRows`, `RGBToYUVComponent`, `ConvertWRGBToYUV`, `SafeMalloc` and `DoSharpArgbToYuv`). Not here: `SharpYuvInit` (see decisions). |
| `src/sharpyuv/sharpyuv_csp.rs` | `sharpyuv/sharpyuv_csp.h`, `sharpyuv/sharpyuv_csp.c` | yes: `SharpYuvRange`, `SharpYuvColorSpace`, `SharpYuvMatrixType`, `SharpYuvComputeConversionMatrix`, the five predefined matrices and `SharpYuvGetConversionMatrix`. |
| `src/sharpyuv/sharpyuv_dsp.rs` | `sharpyuv/sharpyuv_dsp.h`, `sharpyuv/sharpyuv_dsp.c` | partly: `SharpYuvUpdateY`, `SharpYuvUpdateRGB` and `SharpYuvFilterRow` as plain functions over their portable `_c` implementations (ADR 0003). `SharpYuvInitDsp` is not translated (deviation 1). |
| `src/sharpyuv/sharpyuv_gamma.rs` | `sharpyuv/sharpyuv_gamma.h`, `sharpyuv/sharpyuv_gamma.c` | yes: both gamma tables as `LazyLock` statics with the C computation, `SharpYuvInitGammaTables` (which forces them), the 16 valid transfer functions, sRGB included, in both directions, and `SharpYuvGammaToLinear`, `SharpYuvLinearToGamma`. |
| `src/sharpyuv/sharpyuv_cpu.rs` | `sharpyuv/sharpyuv_cpu.h`, `sharpyuv/sharpyuv_cpu.c` | no: waits for WP4. The C file re-compiles `src/dsp/cpu.c` under the name `SharpYuvGetCPUInfo`; see decisions. |

`SharpYuvConvertWithOptions` and `SharpYuvConvert` do not call `SharpYuvInit` (deviation 1).
The SSE2 and NEON files (`sharpyuv_sse2.c`, `sharpyuv_neon.c`) are WP19 and WP20 and are
not touched.

## Decisions of the integrator

The integrator answered the three open questions of the first report and asked for a fix
to a CI failure. The decisions are recorded here and in the places named.

1. **Dispatch and init-once state (deviation 1).** Dispatch pointers are plain functions
   over the portable `_c` implementations, as WP4 does in its pull request #5. The
   functions that only assign pointers are absent: `SharpYuvInitDsp`, and the pointer and
   mutex state of `SharpYuvInit`. The runtime selection of the SSE2 and NEON variants
   belongs to WP19 and WP20.
   - `SharpYuvInit` stays absent. Its only remaining work is to force the gamma tables,
     which the Rust gamma tables already do on first use. Its CPU-information argument
     would have to be recorded in state, and that state stays absent, so the function
     cannot be translated faithfully. It is not translated.
   - When WP4 is merged, `sharpyuv_cpu.rs` translates `SharpYuvGetCPUInfo` as a function
     with that name that calls WP4's `vp8_get_cpu_info`, with the same `VP8CPUInfo`
     signature. It is the C file's re-compilation of `cpu.c` under another name.
2. **Untyped sample buffers.** ADR 0003 now has the rule: a `void*` buffer whose sample
   type depends on the bit depth is `&[u8]` or `&mut [u8]` with a byte offset, and
   samples of more than one byte are read and written in native byte order (deviation 2).
3. **No SharpYUV C ABI.** No package from WP0 to WP22 plans one, so the NULL-pointer cases
   are not applicable to the safe API (not-applicable check 1). A future SharpYUV ABI
   package must run them. The C-side checks that the probes reject them are kept.
4. **CI failure (fixed in this package).** In the first run of this pull request, the
   `computed-random` matrices failed in the Linux release oracle and aborted the macOS
   asan-ubsan probe. In `SharpYuvComputeConversionMatrix`, `ToFixed16` converts
   `f * 65536 + 0.5` to `int`, which is undefined when `|f| >= 32768`. The seeded color
   spaces are now restricted so that every intermediate fits `int`
   (exclusion 1). The Rust function is unchanged.

## Evidence

Every differential test runs the C probe in a separate process against an oracle built with
`python3 tools/build_oracle.py`. `LIBWEBP_ORACLE_DIR` names the variant, and
`--features oracle` makes a missing oracle fail. The probes use the generator of
`tools/oracle/probes/wp5/support/wp5_rng.c` (SplitMix64); the Rust tests draw the same
values in the same order, so no input is stored.

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp5_convert.rs` | `convert_matches_the_oracle_inventory` | 1,296 groups from `convert.c`: the 1,232 inventory groups and 64 extra seeded conversions with negative strides and steps. Each group compares the return value and each whole output plane, guard bytes included. The inventory groups are the 960 combinations of RGB depth (8, 10, 12, 16), YUV depth (8, 10, 12), transfer function (1 and 4 to 18) and predefined matrix (5), over nine shapes and three layouts; 256 seeded conversions with random shapes, layouts, matrices, transfers and depths; 15 rejected calls; and 24 cold conversions on eight threads, first in the process. For the sRGB transfer the test also checks that `SharpYuvConvert` agrees with `SharpYuvConvertWithOptions`. |
| `tests/wp5_matrix.rs` | `version_matrices_and_options_match_the_c_functions` | 31 inventory groups: `SharpYuvGetVersion`, the five predefined matrices, the three out-of-range types, 18 computed matrices, and six `SharpYuvOptionsInitInternal` versions. It also compares 256 further computed matrices from seeded color spaces with the bounds of exclusion 1; the inventory does not contain them. |
| `tests/wp5_matrix.rs` | `predefined_matrices_have_stable_addresses_and_invalid_types_have_none`, `options_accept_only_this_major_and_minor_version` | No oracle: matrix addresses are stable across calls, invalid types give `None`, and the options accept exactly major 0 and minor 4. |
| `tests/wp5_gamma.rs` | `gamma_and_transfer_tables_match_the_c_functions` | 128 groups: `SharpYuvGammaToLinear` and `SharpYuvLinearToGamma` for the 16 valid transfer functions and bit depths 8, 10, 12 and 16, as little-endian bytes. Inputs up to 12 bits and the sRGB table at every bit depth are swept exhaustively; other 16-bit sweeps take every 257th value. Both tables are covered through the sRGB path. Exclusion 3 applies. |
| `tests/wp5_dsp.rs` | `dispatch_functions_match_the_c_functions` | 600 groups: 200 seeded cases each of `SharpYuvUpdateY` (the returned sum and the whole destination), `SharpYuvUpdateRGB` and `SharpYuvFilterRow`, with lengths up to 40, bit depths 10, 12 and 14, and the entries beyond the length. |

Results on this host (macOS ARM64, Apple clang 21), with the code of this revision:

- The four differential tests pass in the dev, release and `checked` profiles against the
  release oracle, and `python3 tools/check.py --oracle` passed before this revision.
- `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --check` pass.
- Under the **asan-ubsan** oracle, all four WP5 probes exit 0 with no sanitizer report,
  and their outputs are byte-identical to the release oracle's. The sanitizer checks
  the C side only; the Rust tests against the asan-ubsan oracle are reported in the pull
  request.
- A check of the test itself: changing `+ 2` to `+ 1` in `ScaleDown`'s rounding makes
  `convert_matches_the_oracle_inventory` fail, so the comparison is not vacuous.

## Reproduction of the libavif-rs inventory

`/Users/sebastian/Workspace/libavif-rs/docs/SHARPYUV-VALIDATION.md` states 1,271
comparison groups for `tools/check_sharpyuv.py`. This package reproduces 1,263 of them
with the same structure: the 31 matrix groups and the 1,232 conversion groups. The eight
that are not reproduced need NULL pointers (not-applicable check 1). The random bytes
differ, because Python's `random` module is replaced by SplitMix64 on both sides; the
groups, shapes, layouts, parameters, guard bytes and checks are the same. The inventory's
24 cold conversions run first in the probe, as in the Python worker.

## Not-applicable checks

1. **NULL arguments (8 groups).** The C inventory passes NULL to
   `SharpYuvOptionsInitInternal` (2 groups) and to `SharpYuvConvertWithOptions` for each
   of the six pointer arguments (6 groups). The Rust API takes references, so NULL is not
   representable (PORTING rule 8). No SharpYUV C ABI is planned in WP0 to WP22, so these
   cases are not applicable to the safe API. A future SharpYUV ABI package must run them.
   The probes still run them on the C side and require the result 0 (`csp.c` and
   `convert.c` exit nonzero otherwise); they are not printed.
2. **`SharpYuvInit`, the CPU-information pointer and the repeated callback
   initialization.** The inventory's three calls with a callback and its `SharpYuvInit(NULL)`
   call depend on `SharpYuvInit`, which stays absent (decision 1). `SharpYuvGetCPUInfo`
   is translated after WP4 merges.
3. **Allocation failure.** The inventory injects failures into the seven conversion
   allocations. The library has no test-only allocator (ADR 0003), so the `try_reserve_exact`
   failure path, which returns `false`, is not tested.
4. **Input immutability.** The input planes are `&[u8]`, so the library cannot write them.
   The probe checks that its input buffer is unchanged and exits nonzero otherwise; the
   guard bytes of every output plane are checked the same way.
5. **Undefined behavior of the C code.** The transfer functions 0, 2, 3 and 19 hit
   `assert(0)` in C and are not swept. For the other exclusions see the list under
   deviations and [exclusions](#exclusions).
6. **Floating point across hosts.** Gamma and matrix computations use `float` and libm. The
   results are equal on this host; other hosts (the Linux x86-64 runner) are not claimed.
7. **Thread sanitizer.** Not run for this package. The 24 concurrent conversions pass in
   the normal build and under the asan-ubsan oracle.
8. **MSRV 1.95.** `check.py --msrv` was not run for this package.

## Exclusions

These inputs are undefined in C, or their results differ from C only in undefined cases,
and each is excluded from the comparison explicitly (ground rule 8).

1. **Seeded color spaces (`csp.c`, `wp5_matrix.rs`).** The 256 seeded color spaces draw
   kr and kb from [0, 0.5) and the bit depth from 8 to 15. In
   `SharpYuvComputeConversionMatrix`, `ToFixed16` converts `f * 65536 + 0.5` to `int`, which
   is undefined when `|f| >= 32768`. With these bounds every scale factor is at most 1 and
   every coefficient is at most 1 in magnitude, so every intermediate fits `int`. The
   earlier draw from [0, 1) produced the CI failure of decision 4.
2. **Bit depth 16 in `SharpYuvComputeConversionMatrix`.** The offset `128 << (bit_depth - 8)`
   is `2^15` at 16 bits, and `ToFixed16` of `2^15 * 65536` overflows `int`, for every color
   space. The C result is undefined: the CI run observed `INT_MIN`, and the Rust function
   saturates to `i32::MAX`. The bit depth 16 is excluded from the seeded color spaces. The
   inventory's computed matrices use 8, 10 and 12 bits. `SharpYuvConvert` does not use
   computed matrices, so no conversion reaches this case.
3. **Gamma sweep of 65,536 for non-sRGB transfers.** `SharpYuvLinearToGamma` at 16 bits
   converts a value above 65,535 from float to `uint16_t`, which is undefined; the probe
   observed a wrap to 0 for the inverse of the IEC 61966 transfer, and Rust saturates. The
   sweep stops at 65,535 for these transfers. The conversion never reaches this case,
   because it calls the functions with bit depths up to 14.
4. **Transfer functions 0, 2, 3 and 19.** The C `default:` branch asserts and returns a linear
   value of 0. The Rust branch does the same with `debug_assert!`. These inputs are not swept.

## Recorded deviations

1. **No `SharpYuvInit` call and no dispatch initialization.** C conversions call
   `SharpYuvInit`, which assigns the dispatch pointers, computes the gamma tables and stores
   the CPU-information function. In Rust the dispatch functions are direct calls (ADR 0003),
   the tables are `LazyLock` statics computed on first use, and the SSE2 and NEON selection
   of WP19 and WP20 is not yet present, so the conversion results do not depend on the call.
   This follows decision 1 of the integrator, which follows WP4's pattern in pull request #5.
2. **Buffers are byte slices with byte offsets.** A `void*` sample buffer becomes a `&[u8]`
   or `&mut [u8]` and a byte offset to its first sample, as ADR 0003 now records. Samples of
   8 bits are one byte; samples of 10, 12 and 16 bits are two bytes, read and written in
   native byte order at the byte positions that the C `uint16_t*` indexing addresses. The
   Rust code does not need the buffer to be aligned to two bytes, which C does. Strides and
   the `rgb_step` keep their C type, in bytes.
3. **Negative strides and steps.** Rows and pixels are addressed with a checked signed sum from
   the origin, so a flipped plane works as in C when the origin is its last row or pixel
   (tested, groups `flipped-*`). A row that would start before the slice panics, where C
   reads memory before the buffer, which C leaves undefined.
4. **Options on failure.** `SharpYuvOptionsInitInternal` returns `None` where C returns 0 and
   leaves `options` unchanged. `SharpYuvOptions` holds a reference to the matrix, so its NULL
   checks are not representable (not-applicable check 1).
5. **Out-of-range float conversions.** C converts a float that is out of range for an integer
   type with an undefined conversion. Rust `as` saturates. This is reachable only in the
   excluded cases 2 and 3 above.
6. **Enumerations.** `SharpYuvTransferFunctionType`, `SharpYuvRange` and `SharpYuvMatrixType`
   are `#[repr(transparent)]` newtypes over `u32`, with the C enumerator names as associated
   constants (ADR 0003). `SharpYuvGetConversionMatrix` returns `None` for values outside the
   five matrices, as C returns `NULL`.
7. **Predicates return `bool`.** `SharpYuvConvert`, `SharpYuvConvertWithOptions` and
   `SharpYuvOptionsInitInternal` return `bool` in place of `int` 0 and 1 (ADR 0003, predicate
   rule). `SharpYuvGetVersion` keeps its `i32`.
8. **Gamma tables.** `static` tables filled once by `SharpYuvInitGammaTables` become `LazyLock`
   statics with the same computation and values (ADR 0003). The function forces the
   computation. Its values are compared through the transfer functions (`tests/wp5_gamma.rs`),
   since the C tables are not visible outside their file.
9. **Invalid transfer functions.** The C `default:` branch asserts and returns a linear value
   of 0; the Rust branch does the same with `debug_assert!` (exclusion 4).
10. **Integer width of sizes.** Buffer sizes are computed in `usize` and `u64`, where the C code
    multiplies `int` values (`w * 3` and similar) that can overflow for widths near `INT_MAX`.
    The inventory has no such size; the allocation fails in both (not-applicable check 3).
11. **`SafeMalloc` limits.** The C `SafeMalloc` checks the product against `size_t`; Rust checks
    it against `usize` and then reserves with `try_reserve_exact`. The memory is
    zero-initialized, which C does not do. Reading `DoSharpArgbToYuv`, each scratch entry is
    written before it is read, so the zero fill is not observable.
12. **Integer overflow of `RGBToYUVComponent`.** The C `int` products are undefined when they
    overflow; the Rust arithmetic panics in the dev and `checked` profiles. No input in the
    inventory or the seeded cases overflows, which the `checked` runs confirm.

## Open under this package

- `sharpyuv_cpu.rs`: after WP4 is merged to `main`, rebase, translate `SharpYuvGetCPUInfo`
  as decision 1 says, and extend the evidence with the CPU-information cases of the inventory.
- The SIMD dispatch of `SharpYuvInitDsp` belongs to WP19 and WP20.
