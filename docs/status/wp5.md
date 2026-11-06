# WP5: SharpYUV

- Status: the conversion core, the color space and matrix functions, the transfer
  functions and gamma tables, and the dispatch functions are translated and
  verified. Not yet translated: `sharpyuv/sharpyuv_cpu.c` (`sharpyuv_cpu.rs`) and
  `SharpYuvInit`, which wait for WP4 (`src/dsp/cpu.rs` is not on `main`) and for the
  decision in [open question 1](#open-questions-for-the-integrator). The package is
  therefore a draft pull request until WP4 is merged and this page is updated.
- Sub-wave: 1b (merges after WP4)
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1; WP4 for `sharpyuv_cpu.rs` and `SharpYuvInit` only
- Probes: `tools/oracle/probes/wp5/` (with `support/`); tests: `tests/wp5_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). The inventory comes from `tools/units.json`,
which assigns the five modules to this package; the package adds no exceptions.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/sharpyuv/sharpyuv.rs` | `sharpyuv/sharpyuv.h`, `sharpyuv/sharpyuv.c` | partly: `SHARPYUV_VERSION*`, `SHARPYUV_MAKE_VERSION`, `SharpYuvGetVersion`, `SharpYuvConversionMatrix`, `SharpYuvTransferFunctionType` with its 16 valid enumerators and the count, `SharpYuvOptions`, `SharpYuvOptionsInit`, `SharpYuvOptionsInitInternal`, `SharpYuvConvert` and `SharpYuvConvertWithOptions` with the whole conversion core (`GetPrecisionShift`, `clip*`, `RGBToGray`, `ScaleDown`, `UpdateW`, `UpdateChroma`, `StoreGray`, `Filter2`, `Shift`, `ImportOneRow`, `InterpolateTwoRows`, `RGBToYUVComponent`, `ConvertWRGBToYUV`, `SafeMalloc` and `DoSharpArgbToYuv`). Not here: `SharpYuvInit` (open question 1). |
| `src/sharpyuv/sharpyuv_csp.rs` | `sharpyuv/sharpyuv_csp.h`, `sharpyuv/sharpyuv_csp.c` | yes: `SharpYuvRange`, `SharpYuvColorSpace`, `SharpYuvMatrixType`, `SharpYuvComputeConversionMatrix`, the five predefined matrices and `SharpYuvGetConversionMatrix`. |
| `src/sharpyuv/sharpyuv_dsp.rs` | `sharpyuv/sharpyuv_dsp.h`, `sharpyuv/sharpyuv_dsp.c` | partly: `SharpYuvUpdateY`, `SharpYuvUpdateRGB` and `SharpYuvFilterRow` as plain functions over their portable `_c` implementations (ADR 0003). `SharpYuvInitDsp` is not translated: it assigns those pointers, and its SSE2 and NEON selection belongs to WP19 and WP20 and to open question 1. |
| `src/sharpyuv/sharpyuv_gamma.rs` | `sharpyuv/sharpyuv_gamma.h`, `sharpyuv/sharpyuv_gamma.c` | yes: both gamma tables as `LazyLock` statics with the C computation, `SharpYuvInitGammaTables` (which forces them), the 16 valid transfer functions, sRGB included, in both directions, and `SharpYuvGammaToLinear`, `SharpYuvLinearToGamma`. |
| `src/sharpyuv/sharpyuv_cpu.rs` | `sharpyuv/sharpyuv_cpu.h`, `sharpyuv/sharpyuv_cpu.c` | no: the file includes `src/dsp/cpu.c` under the name `SharpYuvGetCPUInfo`; it waits for WP4 and open question 1. |

`SharpYuvConvertWithOptions` and `SharpYuvConvert` do not call `SharpYuvInit` (see
[deviation 1](#recorded-deviations) and open question 1). The SSE2 and NEON files
(`sharpyuv_sse2.c`, `sharpyuv_neon.c`) are WP19 and WP20 and are not touched.

## Evidence

Every differential test runs the C probe in a separate process against the release
oracle (`python3 tools/build_oracle.py`). `LIBWEBP_ORACLE_DIR` names the variant, and
`--features oracle` makes a missing oracle fail. The probes use the generator of
`tools/oracle/probes/wp5/support/wp5_rng.c` (SplitMix64); the Rust tests draw the
same values in the same order, so no input is stored.

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp5_convert.rs` | `convert_matches_the_oracle_inventory` | 1,296 groups from `convert.c`: the 1,232 inventory groups and 64 extra seeded conversions with negative strides and steps. Each group compares the return value and each whole output plane, guard bytes included. The inventory groups are the 960 combinations of RGB depth (8, 10, 12, 16), YUV depth (8, 10, 12), transfer function (1 and 4 to 18) and predefined matrix (5), over nine shapes and three layouts; 256 seeded conversions with random shapes, layouts, matrices, transfers and depths; 15 rejected calls; and 24 cold conversions on eight threads, first in the process. For the sRGB transfer the test also checks that `SharpYuvConvert` agrees with `SharpYuvConvertWithOptions`. |
| `tests/wp5_matrix.rs` | `version_matrices_and_options_match_the_c_functions` | 31 inventory groups: `SharpYuvGetVersion`, the five predefined matrices, the three out-of-range types, 18 computed matrices, and six `SharpYuvOptionsInitInternal` versions. It also compares 256 further computed matrices from seeded color spaces, which the inventory does not contain. |
| `tests/wp5_matrix.rs` | `predefined_matrices_have_stable_addresses_and_invalid_types_have_none`, `options_accept_only_this_major_and_minor_version` | No oracle: matrix addresses are stable across calls, invalid types give `None`, and the options accept exactly major 0 and minor 4. |
| `tests/wp5_gamma.rs` | `gamma_and_transfer_tables_match_the_c_functions` | 128 groups: `SharpYuvGammaToLinear` and `SharpYuvLinearToGamma` for the 16 valid transfer functions and bit depths 8, 10, 12 and 16, as little-endian bytes. Inputs up to 12 bits and the sRGB table at every bit depth are swept exhaustively; other 16-bit sweeps take every 257th value. Both tables are covered through the sRGB path. |
| `tests/wp5_dsp.rs` | `dispatch_functions_match_the_c_functions` | 600 groups: 200 seeded cases each of `SharpYuvUpdateY` (the returned sum and the whole destination), `SharpYuvUpdateRGB` and `SharpYuvFilterRow`, with lengths up to 40, bit depths 10, 12 and 14, and the entries beyond the length. |

Results on this host (macOS ARM64, Apple clang 21, release oracle) with the final
code: the four differential tests pass in the dev, release and `checked` profiles,
and `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --check`
pass. A check of the test itself: changing `+ 2` to `+ 1` in `ScaleDown`'s rounding
makes `convert_matches_the_oracle_inventory` fail, so the comparison is not vacuous.

`python3 tools/check.py --oracle` (release oracle, all three profiles, formatting,
Clippy, provenance, links and the unit dependency check) passes on this host. The
MSRV build with Rust 1.95 was not run (not-applicable check 8).

## Reproduction of the libavif-rs inventory

`/Users/sebastian/Workspace/libavif-rs/docs/SHARPYUV-VALIDATION.md` states 1,271
comparison groups for `tools/check_sharpyuv.py`. This package reproduces 1,263 of
them with the same structure: the 31 matrix groups and the 1,232 conversion groups.
The eight that are not reproduced need NULL pointers, which exist only at the C ABI
(not-applicable check 1). The random bytes differ, because Python's `random` module
is replaced by SplitMix64 on both sides; the groups, shapes, layouts, parameters,
guard bytes and checks are the same. The inventory's 24 cold conversions run first
in the probe, as in the Python worker.

## Not-applicable checks

1. **NULL arguments (8 groups).** The C inventory passes NULL to `SharpYuvOptionsInitInternal`
   (2 groups) and to `SharpYuvConvertWithOptions` for each of the six pointer
   arguments (6 groups). The Rust API takes references, so NULL is not representable
   (PORTING rule 8). The probes run them and require the C result 0 (`csp.c` and
   `convert.c` exit nonzero otherwise), but do not print them, so no Rust
   comparison exists. Needs a C ABI for SharpYuv, which is not in the plan (open
   question 3).
2. **`SharpYuvInit` and the CPU information pointer.** The inventory's repeated
   callback initialization and its `SharpYuvInit(NULL)` call, and the
   `SharpYuvGetCPUInfo` cases of the SIMD selection, depend on open question 1.
3. **Allocation failure.** The inventory injects failures into the seven
   conversion allocations. The library has no test-only allocator (ADR 0003), so
   the `try_reserve_exact` failure path, which returns `false`, is not tested.
4. **Input immutability.** The input planes are `&[u8]`, so the library cannot
   write them. The probe checks that its input buffer is unchanged and exits
   nonzero otherwise; the guard bytes of every output plane are checked the same way.
5. **Undefined behavior of the C code.** The transfer functions 0, 2, 3 and 19 hit
   `assert(0)` in C and are not swept. `SharpYuvLinearToGamma` with a value of
   65,536 for a non-sRGB transfer function at 16 bits converts an out-of-range float
   to `uint16_t`, which is undefined, and is not swept (deviation 5). Signed
   overflow of the `int` arithmetic in `RGBToYUVComponent` is undefined in C; the
   `checked` profile would panic on it, and none occurred in the inventory.
6. **Floating point across hosts.** Gamma and matrix computations use `float` and
   libm. The results are equal on this host; other hosts (the Linux x86-64 CI runner
   is not run yet) are not claimed.
7. **Address sanitizer and thread sanitizer.** Not run for this package. The 24
   concurrent conversions pass in the normal build.
8. **MSRV 1.95.** `check.py --msrv` was not run for this package in this session.

## Recorded deviations

1. **No `SharpYuvInit` call in the conversion.** C conversions call `SharpYuvInit`,
   which assigns the dispatch pointers, computes the gamma tables and stores the CPU
   information function. In Rust the dispatch functions are direct calls (ADR 0003),
   the tables are `LazyLock` statics computed on first use, and the CPU selection
   of WP19 and WP20 is not yet present, so the conversion results do not depend on
   the call. This is the one point in this package that relies on ADR 0003 for an
   Init function; open question 1 asks the integrator to confirm it.
2. **Buffers are byte slices with byte offsets.** A `void*` sample buffer becomes a
   `&[u8]` or `&mut [u8]` and a byte offset to its first sample (ADR 0003). Samples of
   8 bits are one byte; samples of 10, 12 and 16 bits are two bytes, read and written
   in native byte order at the byte positions that the C code's `uint16_t` indexing
   addresses. The Rust code does not need the buffer to be aligned to two bytes,
   which C does. Strides and the `rgb_step` keep their C type, in bytes.
3. **Negative strides and steps.** Rows and pixels are addressed with a checked
   signed sum from the origin, so a flipped plane works as in C when the origin is
   its last row or pixel (tested, groups `flipped-*`). A row that would start before
   the slice panics, where C reads memory before the buffer, which C leaves undefined.
4. **Options on failure.** `SharpYuvOptionsInitInternal` returns `None` where C returns
   0 and leaves `options` unchanged. `SharpYuvOptions` holds a reference to the
   matrix, so its NULL checks are not representable (not-applicable check 1).
5. **Out-of-range float conversions.** C converts a float that is out of range for
   `uint16_t` with an undefined conversion; the probe observed a wrap to 0. Rust `as`
   saturates. Reachable only with bit depth 16, which the conversion never passes:
   the conversion's bit depths are at most 14 (`GetPrecisionShift`), so the outputs
   stay in range. Excluded from the sweep at 65,536 for non-sRGB transfers.
6. **Enumerations.** `SharpYuvTransferFunctionType`, `SharpYuvRange` and
   `SharpYuvMatrixType` are `#[repr(transparent)]` newtypes over `u32`, with the
   C enumerator names as associated constants (ADR 0003). `SharpYuvGetConversionMatrix`
   returns `None` for values outside the five matrices, as C returns `NULL`.
7. **Predicates return `bool`.** `SharpYuvConvert`, `SharpYuvConvertWithOptions` and
   `SharpYuvOptionsInitInternal` return `bool` in place of `int` 0 and 1 (ADR 0003,
   predicate rule). `SharpYuvGetVersion` keeps its `i32`.
8. **Gamma tables.** `static` tables filled once by `SharpYuvInitGammaTables` become
   `LazyLock` statics with the same computation and values (ADR 0003). The function
   forces the computation. Its values are compared through the transfer functions
   (`tests/wp5_gamma.rs`), since the C tables are not visible outside the file.
9. **Invalid transfer functions.** The C `default:` branch asserts and returns a
   linear value of 0. The Rust branch does the same with `debug_assert!`; the input
   is undefined and is not swept (not-applicable check 5).
10. **Integer width of sizes.** Buffer sizes are computed in `usize` and `u64`, where
    the C code multiplies `int` values (`w * 3` and similar) that can overflow for
    widths near `INT_MAX`. The inventory has no such size; the allocation fails in
    both (not-applicable check 3).
11. **`SafeMalloc` limits.** The C `SafeMalloc` checks the product against `size_t`;
    Rust checks it against `usize` and then reserves with `try_reserve_exact`. The
    memory is zero-initialized, which C does not do. Reading `DoSharpArgbToYuv`, each
    scratch entry is written before it is read, so the zero fill is not observable.

## Open questions for the integrator

1. **`SharpYuvInit`, `SharpYuvGetCPUInfo` and the init-once state.** `SharpYuvInit`
   is public. It stores a `VP8CPUInfo` function pointer in the global
   `SharpYuvGetCPUInfo`, runs `SharpYuvInitDsp` (portable pointers, then the SSE2 and
   NEON variants that the CPU query selects) and `SharpYuvInitGammaTables`, and
   guards this with a mutex and a `static` record of the last CPU function used. The
   Rust side has no global function pointer (ADR 0003 keeps `VP8GetCPUInfo` as a
   query, with the C ABI overriding it). Proposed answer, which this package
   assumes: the safe Rust API has no `SharpYuvInit`; `SharpYuvGetCPUInfo` is a query
   in `sharpyuv_cpu.rs` that calls WP4's `vp8_get_cpu_info`; the ABI, if it is ever
   built, owns the callback override and the init-once record. Please confirm or
   redirect; `sharpyuv_cpu.rs` and `SharpYuvInit` wait for this and for WP4.
2. **Representation of `void*` sample buffers** (deviation 2). It follows ADR 0003's
   slice-plus-offset rule, but the bit-depth dependent sample type is not written
   there. Should ADR 0003 record it, or is the deviation enough?
3. **A SharpYuv C ABI.** The eight NULL-argument groups and the `SharpYuvInit`
   callback cases need a C-ABI module for `sharpyuv/*.h`. No package in the plan
   owns it (ADR 0004 covers the WebP headers). Does WP5 own it, or does the integrator
   assign it?

## Open under this package

- `sharpyuv_cpu.rs` and `SharpYuvInit`: after WP4 is merged to `main`, rebase, then
  translate them (after question 1) and extend the evidence with the
  `SharpYuvGetCPUInfo` and repeated-initialization cases.
- The SIMD dispatch of `SharpYuvInitDsp` belongs to WP19 and WP20.
