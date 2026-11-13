# Port SIMD after the portable decoder, as intrinsics on both architectures

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-13

## Context

libjpeg-turbo's speed comes partly from SIMD for the IDCT, upsampling and color
conversion (`simd/`). On AArch64 that code is C with NEON intrinsics (about 7k
lines in `simd/arm`); upstream replaced its former GAS assembly with it and
reports it as faster in several cases. On x86-64 it is NASM assembly (about 14k
lines in `simd/x86_64`) for SSE2 and AVX2. The serial Huffman decoder has no
SIMD and dominates decode time on photographs (Ferrisma's measurement
estimates about 70 % on a 12 Mpx photo).

## Decision

1. The portable decoder comes first and is verified against the `WITH_SIMD=0`
   oracle ([ADR 0002](0002-differential-oracle.md)).
2. NEON kernels are translated from `simd/arm` C intrinsics to `std::arch::aarch64`
   intrinsics, one Rust function per C function, dispatched where upstream's
   `jsimd.c` dispatches.
3. x86-64 kernels are rewritten from the NASM sources as `std::arch::x86_64`
   intrinsics (SSE2 and AVX2), with runtime feature detection replacing
   upstream's CPUID code. This is the one place where the port cannot be
   line-by-line, so each kernel is proven bit-identical to the assembly through
   probes against a `WITH_SIMD=1` oracle, and its speed is measured against it.
4. Every kernel's output equals the portable path's output wherever upstream's
   SIMD and C paths agree; where upstream's SIMD differs from its C code (for
   example rounding in the fast IDCT), the SIMD oracle is the reference and the
   difference is recorded.
5. All SIMD modules need the unsafe ADR required by
   [ADR 0003](0003-toolchain-and-unsafe-policy.md) before work starts.
6. Speed is measured against libjpeg-turbo, never estimated. `tools/bench.py`
   decodes the same inputs through the port's safe API (`examples/bench.rs`)
   and through the oracle's libjpeg API (the probe `wp9/bench`), requires the
   same output from both, and reports the median time per decode and the
   ratio of the port's time to C's. It compares each SIMD configuration with
   the `simd` oracle and the portable decoder with the reference oracle, and
   has cases that select the kernels one by one. It measures on native hosts
   only, never under an emulator such as Rosetta 2. CI's bench job runs it on
   Linux x86-64 and ARM64 and keeps the record as an artifact; on shared
   runners the ratios are comparable and the absolute times only roughly.

## Consequences

The first usable decoder is portable and slower than libjpeg-turbo with SIMD.
The x86-64 rewrite carries a performance risk against hand-written assembly; it
is bounded by the share of decode time in SIMD code and is measured per kernel.

## Amendment 2026-11-12: inline the Huffman fast path

The serial Huffman decoder, not the SIMD kernels, held most of the remaining
gap to libjpeg-turbo ([engine performance
analysis](https://github.com/sebastian-software/ferrisma/blob/main/docs/benchmarks/v1.0-engine-performance-analysis-2026-11-12.md),
issues [ferrisma#76](https://github.com/sebastian-software/ferrisma/issues/76)
and [ferrisma#221](https://github.com/sebastian-software/ferrisma/issues/221)).
Upstream's `HUFF_DECODE_FAST` is a macro; the port's function of that name
takes the bit-buffer state by `&mut` and carried `#[inline]`, which LLVM did
not follow, so the state lived in memory on every symbol. It is now
`#[inline(always)]` (with a scoped `clippy::inline_always` expectation). No
other code changed, and no `unsafe` was added.

Correctness: `tools/bench.py`'s cases produce the same checksums in C, the
previous port and the inlined port; upstream's in-scope bit tests pass against
the `release` and `simd` oracles (79 of 79 each), and the differential tests
pass with `LIBJPEG_ORACLE_DIR` set for the `release` oracle without default
features and for the `simd` oracle in the `neon` and `none` modes. The T.83
conformance tests did not run: their data is downloaded from ITU and was not
fetched for this change.

Speed, measured with `tools/bench.py`'s inputs (3632×2384 tiled photo, 5
decodes per run) and the `simd` oracle's `wp9/bench`, 7 interleaved runs
alternating the order of C, the previous port and the inlined port, medians.
Apple M1 Ultra, macOS 27.0.1. **Indicative; loaded host** (1-minute load
average 13.9–19.4, other builds running):

| case | before ms | after ms | C ms | after / before | before / C | after / C |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 40.82 | 29.16 | 25.87 | 0.71 | 1.58 | 1.13 |
| baseline 4:2:2, RGB | 47.52 | 34.42 | 30.18 | 0.72 | 1.57 | 1.14 |
| baseline 4:4:4, RGB | 58.90 | 42.82 | 36.34 | 0.73 | 1.62 | 1.18 |
| progressive 4:2:0, RGB | 96.20 | 95.96 | 64.95 | 1.00 | 1.48 | 1.48 |
| baseline 4:2:0, RGBX | 40.69 | 29.40 | 26.19 | 0.72 | 1.55 | 1.12 |
| baseline 4:2:0, fast IDCT | 38.39 | 27.01 | 24.77 | 0.70 | 1.55 | 1.09 |
| baseline 4:2:0, float IDCT | 50.48 | 39.02 | 37.12 | 0.77 | 1.36 | 1.05 |
| baseline 4:2:0, merged upsampling | 38.71 | 27.19 | 24.44 | 0.70 | 1.58 | 1.11 |
| baseline 4:2:2, merged upsampling | 46.53 | 33.27 | 29.27 | 0.71 | 1.59 | 1.14 |
| baseline 4:2:0, scale 1/2 | 35.02 | 23.50 | 21.64 | 0.67 | 1.62 | 1.09 |
| baseline 4:2:0, scale 1/4 | 32.44 | 21.04 | 19.79 | 0.65 | 1.64 | 1.06 |
| baseline 4:2:0, scale 1/8 | 31.91 | 19.20 | 18.01 | 0.60 | 1.77 | 1.07 |
| testorig.jpg (227×149) | 0.12 | 0.11 | 0.08 | 0.91 | 1.51 | 1.37 |

Baseline (sequential Huffman) decodes are 23–40 % faster and 1.05–1.18× C.
Progressive decoding uses the separate progressive entropy decoder and is
unchanged at 1.48× C; it is the next Huffman target. The remaining baseline
gap includes the IDCT pass-2 helpers, which `#[target_feature]` keeps out of
line. The script and raw runs are in the analysis's evidence directory
(`jpeg_ab.py`, `jpeg-ab-inline.json`). An idle-host and CI `bench` run should
confirm the ratios.

## Amendment 2026-11-13: translation losses that upstream's structure does not have

Owner decision D2 of 2026-11-13 reports speed against the original in three
columns: (a) the scalar original, one thread; (b) the original with SIMD,
one thread; (c) with SIMD and all threads. libjpeg-turbo decodes on one
thread, so (c) equals (b). Three changes remove costs that the C code does
not have. Each keeps upstream's statements and order and changes only how the
Rust expresses a C construct:

1. **Multi-scan MCU block list (`jdcoefct.rs`, `consume_data`).** The port
   built each MCU's block list in a `Vec`, a heap allocation per MCU; C fills
   the fixed array `coef->MCU_buffer[D_MAX_BLOCKS_IN_MCU]`
   (`src/jdcoefct.h:45`, filled at `src/jdcoefct.c:245-256`). The list is now
   a fixed array of `D_MAX_BLOCKS_IN_MCU` block references whose unused
   entries point to placeholder blocks, and `decode_mcu` receives the first
   `blkn`. This was the progressive gap: `malloc` was 17 % of a progressive
   decode. `jdphuff.rs` itself needed no change: `HUFF_DECODE`,
   `CHECK_BIT_BUFFER`, `GET_BITS` and `HUFF_EXTEND` (`src/jdhuff.h:160-223`,
   `src/jdphuff.c:203`) were already inlined, and the only calls left are
   `jpeg_fill_bit_buffer` and `jpeg_huff_decode`, which are out-of-line
   functions in C as well.
2. **Neon islow pass-2 helpers (`jidctint_neon.rs`).** Upstream declares
   `jsimd_idct_islow_pass2_regular` and `_sparse` `static INLINE`
   (`simd/arm/jidctint-neon.c:555`, `:702`), and its build defines `INLINE`
   as `__inline__ __attribute__((always_inline))`. Rust rejects
   `#[inline(always)]` on a `#[target_feature]` function, an `#[inline(always)]`
   helper without the attribute needs `unsafe` for every intrinsic, and LLVM
   kept the `#[inline]` helpers out of line. They are now `macro_rules!`
   macros with the unchanged body, which expand in `jsimd_idct_islow_neon`'s
   Neon context with the C parameters as `let` bindings. This adds no
   `unsafe` code; the two pass-1 helpers were already inlined.
3. **Bounds checks in the scalar sample loops (`jdsample.rs`, `jdcolext.rs`,
   `jdmrgext.rs`).** `h2v1_fancy_upsample`, `h2v2_fancy_upsample`,
   `ycc_rgb_convert` and both merged upsamplers indexed rows per sample, so
   each sample paid two to four bounds checks that C's pointer walks do not
   have. Each loop now splits its rows off once to the loop's length and walks
   them by chunks, and the color tables are viewed as arrays of their
   `MAXJSAMPLE + 1` entries. This affects the portable decoder and the
   `JSIMD_FORCENONE` mode; the Neon, SSE2 and AVX2 builds use kernels for
   these steps.

No change to `upstream/`, the oracles or any expected output. Each change
was checked with a deliberate fault that the tests catch (a lost `blkn`
increment; the right pass-2 half reading the left workspace; the h2v1 fancy
upsampler reading the wrong neighbor; green computed from the blue term).

**Verification.** On AArch64 natively and on x86-64 under Rosetta 2 with
`ROSETTA_ADVERTISE_AVX=1`: `tools/check.py`'s static stage; Clippy with
`-D warnings` under `+stable` (1.99.0) and `+1.100.0`, with all features and
with `--no-default-features --features oracle`, for both targets; upstream's
79 in-scope bit tests against the `release` and `simd` oracles; and the
differential tests in the dev, release and checked profiles against the
`release` oracle without default features and against the `simd` oracle in
every SIMD mode (`neon` and `none`; `avx2`, `sse2` and `none` on x86-64), with
`LIBJPEG_EXPECT_SIMD` set: 88 tests against `release` and 89 per mode against
`simd` on AArch64, 88 and 107 per mode on x86-64, all passing. On x86-64 the
checked profile ran against the `release` oracle only; its three `simd`
modes ran in the dev and release profiles (the checked runs were stopped for
time). The T.83 conformance target did not run: its data comes from ITU and
was not downloaded. `cargo xtask codecs --codec jpeg --stage pr` did not run. The engine analysis's
[evidence directory](https://github.com/sebastian-software/ferrisma/tree/main/docs/benchmarks/evidence-2026-11-13-jpeg-port)
has the gate script and its step list.

**Speed.** `tools/bench.py`'s inputs (3632×2384 tiled photo), the C bench
probe of the named oracle against the port before (`e4038a4a`) and after the
three changes, 7 interleaved runs of 5 decodes, order reversed every other
run, medians; every checksum equal across the three. Apple M1 Ultra, AArch64
native. **Indicative; loaded host** (1-minute load average 21–27, other
agents building and testing):

| case | (a) scalar C ms | portable before / C | portable after / C | (b) SIMD C ms | Neon before / C | Neon after / C |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 58.06 | 1.30 | 1.01 | 26.93 | 1.13 | 1.11 |
| baseline 4:2:2, RGB | 61.71 | 1.36 | 1.11 | 31.18 | 1.15 | 1.12 |
| baseline 4:4:4, RGB | 73.50 | 1.15 | 1.06 | 38.49 | 1.19 | 1.14 |
| progressive 4:2:0, RGB | 91.86 | 1.53 | 1.14 | 68.61 | 1.46 | 1.18 |
| baseline 4:2:0, RGBX | 52.76 | 1.38 | 1.05 | 27.60 | 1.09 | 1.10 |
| baseline 4:2:0, fast IDCT | 50.48 | 1.39 | 1.05 | 25.53 | 1.08 | 1.09 |
| baseline 4:2:0, float IDCT | 52.64 | 1.31 | 1.05 | 38.51 | 1.04 | 1.05 |
| baseline 4:2:0, merged upsampling | 44.25 | 1.20 | 1.04 | 25.63 | 1.09 | 1.06 |
| baseline 4:2:2, merged upsampling | 55.25 | 1.20 | 1.07 | 30.29 | 1.12 | 1.09 |
| baseline 4:2:0, scale 1/2 | 32.43 | 1.10 | 1.07 | 22.24 | 1.07 | 1.07 |
| baseline 4:2:0, scale 1/4 | 24.20 | 1.03 | 1.05 | 20.15 | 1.06 | 1.06 |
| baseline 4:2:0, scale 1/8 | 18.39 | 1.10 | 1.07 | 18.50 | 1.06 | 1.05 |
| testorig.jpg (227×149) | 0.18 | 1.51 | 1.22 | 0.09 | 1.25 | 1.20 |

Column (c) equals (b). Under Rosetta 2 the same runs agree on progressive
decoding (portable 1.75 → 1.16, AVX2 1.47 → 1.11 of C, emulated) but are too
noisy for the other cases; x86-64 ratios belong to CI's bench job. Rosetta
also cannot judge AVX2 against SSE2: emulated, upstream's own AVX2 build is
about 1.8 times slower than its SSE2 build.

**Remaining gap, measured.** From samply profiles of the port and C on
AArch64 after the changes (shares of process CPU, scaled to time):

- Neon 4:4:4 (1.14): the islow IDCT is 1.46 times C's time. The inlined
  kernel compiles to 1,178 instructions against C's 514: about 170 are bounds
  checks of the eight output-row stores (`RowListMut` row lookup, start and
  length) and the workspace is zero-initialized per block, which C leaves
  uninitialized. `decompress_onepass` is 2.1 times C (block-row access through
  the row lists). The Huffman decoder is within 1–2 % of C.
- Progressive (1.18): `consume_data` still takes about four times C's time
  (by code reading: the per-MCU search of the virtual arrays in
  `mcu_blocks` and the checked `cinfo.entropy()` and `cinfo.master_mut()`
  accesses per MCU; `decode_mcu`'s scoped take of `cinfo.entropy`, ADR 0005
  "A method's own state", adds to the entropy functions),
  and `decode_mcu_AC_refine` about 1.15 times C (index checks on
  `jpeg_natural_order` and the block).
- x86-64 (#74): the port's AVX2 islow IDCT compiles to 589 instructions
  against the NASM routine's 261 (SSE2: 763 against 555), and its color
  converter to 438 against about 250. The extra instructions are row-store
  bounds checks, reloads of constants and shuffles that LLVM chose
  differently. This is a static count, not a native measurement; a native
  x86-64 profile (CI) has to confirm it.

## Review triggers

Revisit when the portable decoder passes its gates, when an intrinsics kernel
cannot reach the assembly's speed
([ferrisma#74](https://github.com/sebastian-software/ferrisma/issues/74), formerly libjpeg-turbo-rs#17), or
when upstream changes its SIMD layout.
