# SIMD status

Updated: 2026-11-13. The quality gate is decision D4 of the Ferrisma owner
decisions and [ADR 0008](adr/0008-simd-modules.md): the port's scalar build is
bit-identical to libwebp's scalar build (`WEBP_ENABLE_SIMD=OFF`, oracle variant
`release`), and the port's SIMD build is bit-identical to libwebp's SIMD build
(`release-simd`) at the same instruction-set level, per target.
`python3 tools/check.py --oracle` runs both; `--arch x86_64` runs the x86-64
build on an ARM64 Mac under Rosetta 2 with `ROSETTA_ADVERTISE_AVX=1`.

## Gate modes

| Target | Mode | Port | Oracle | `LIBWEBP_SIMD_LEVEL` |
| --- | --- | --- | --- | --- |
| AArch64 | scalar | `--no-default-features` | `release` | — |
| AArch64 | `neon` | default features | `release-simd` (NEON) | `neon` |
| x86-64 | scalar | `--no-default-features` | `release` | — |
| x86-64 | `avx2` | default features | `release-simd` (SSE2, SSE4.1, AVX2) | `avx2` |
| x86-64 | `sse41` | default features | `release-simd` | `sse41` |
| x86-64 | `sse2` | default features | `release-simd` | `sse2` |

`avx2` is what an AVX2 machine selects, including the SSE2 and SSE4.1 kernels
that libwebp falls back to where it has no AVX2 variant; `sse41` and `sse2` are
what machines without AVX2 or without SSE4.1 select. Both sides cap the CPU
query at the level: `tools/oracle/probe_support/simd_level.c` wraps libwebp's
`VP8GetCPUInfo` before `main` in every oracle executable, and the port's x86
query applies the same rule with the `oracle` feature.
`tests/dispatch_features.rs` fails if the host reports less than the mode asks
for.

A kernel counts as proven SIMD-against-SIMD at a level when the differential
suite passes in that mode with the kernel selected; the suite reaches each
module through the tests named below. Per-kernel coverage was not measured:
a kernel that only very wide or very narrow inputs reach can be selected but
not exercised.

## AArch64 (NEON)

All NEON translations of libwebp v1.6.0 are selected as libwebp selects them,
including, since 2026-11-13, `RescalerExportRowExpand_NEON` and
`RescalerExportRowShrink_NEON` (issues #79, #80), which differ from libwebp's
C by one in a few bytes and now match libwebp's NEON build. Modules:
`alpha_processing_neon`, `cost_neon`, `dec_neon`, `enc_neon`, `filters_neon`,
`lossless_neon`, `lossless_enc_neon`, `rescaler_neon`, `upsampling_neon`,
`yuv_neon`, `sharpyuv_neon`.

Proven on AArch64 (native, my run of `python3 tools/check.py --stage test
--oracle` on 2026-11-13, after the #252 probe fix): the `neon` mode 411 tests
passed, 0 failed, and the scalar mode 368 passed, 0 failed, in each of the
dev, release and checked profiles (2,337 in all).

## x86-64

| Module | libwebp file | Level | Status |
| --- | --- | --- | --- |
| `dec_sse2`, `dec_sse41` | `dec_sse2.c`, `dec_sse41.c` | SSE2, SSE4.1 | translated (WP18), gated |
| `lossless_sse2`, `lossless_sse41`, `lossless_avx2` | `lossless_*.c` | SSE2, SSE4.1, AVX2 | translated (WP18), gated |
| `upsampling_sse2`, `upsampling_sse41` | `upsampling_*.c` | SSE2, SSE4.1 | translated (WP18), gated |
| `yuv_sse2`, `yuv_sse41` | `yuv_*.c` | SSE2, SSE4.1 | translated (WP18), gated |
| `alpha_processing_sse2`, `alpha_processing_sse41` | `alpha_processing_*.c` | SSE2, SSE4.1 | translated (WP18), gated |
| `filters_sse2` | `filters_sse2.c` | SSE2 | translated (WP18), gated |
| `rescaler_sse2` | `rescaler_sse2.c` | SSE2 | translated (WP18), gated |
| `lossless_enc_sse2`, `lossless_enc_sse41`, `lossless_enc_avx2` | `lossless_enc_*.c` | SSE2, SSE4.1, AVX2 | translated 2026-11-13 (#89), gated |
| `enc_sse2`, `enc_sse41` | `enc_sse2.c`, `enc_sse41.c` | SSE2, SSE4.1 | translated 2026-11-13 (#89), gated |
| `cost_sse2` | `cost_sse2.c` | SSE2 | translated 2026-11-13 (#89), gated |
| `ssim_sse2` | `ssim_sse2.c` | SSE2 | translated 2026-11-13 (#89), gated |
| `sharpyuv_sse2` | `sharpyuv/sharpyuv_sse2.c` | SSE2 | translated 2026-11-13 (#89), gated |

No x86 SIMD file of libwebp v1.6.0 remains untranslated; libwebp has no
AVX-512 code. The encoder variants also have the Rust-against-Rust parity test
`tests/wp19_parity.rs`.

Proven under Rosetta 2, as reported in #89 on 2026-11-13 for `54a0c458`:
the scalar, `avx2`, `sse41` and `sse2` modes pass in the dev, release and
checked profiles, 4,488 tests passed, 0 failed. This replaces the earlier
partial record (interrupted scalar mode, `wp5_cpu` failures before
`simd_level_sharpyuv.c`). Rosetta 2 does not fault on misaligned SSE loads,
so this result could not show #252 (below).

Native Linux x86-64 (hosted nightly run 43401560676 on `dbf7b095`, Clang
21.1.8, #252): the scalar variants pass; the `avx2` mode fails at once because
the `wp10/dsp` probe crashes (SIGSEGV), so `sse41` and `sse2` did not run. The
cause was the probe, not libwebp or the port: it handed libwebp's SSE2 luma
predictors left and top rows at arbitrary byte offsets, which
`enc_sse2.c` reads with `_mm_load_si128` (`movdqa`), while libwebp's encoder
always passes 32-byte aligned rows. The probe now copies the same bytes into
aligned rows (2026-11-13); no hosted rerun had finished when this was written.

## Whole-pipeline gate

`tests/wp13_tools.rs` runs in every gate mode, so in each `release-simd` mode
it compares the Rust-linked `cwebp` (the port's SIMD build) with libwebp's
SIMD `cwebp` at the same capped level: `test_cwebp.sh` in three option sets,
by default and with `-noasm`, and an option matrix of files, reports and exit
statuses. `the_cwebp_builds_follow_the_oracle_variant` fails if the
Rust-linked `cwebp` was linked against the scalar build in a SIMD mode, or the
reverse. Local run, `neon` mode (2026-11-13): 140 inputs,
280 identical outputs in each of the three `test_cwebp.sh` sets and 405
identical matrix runs; no difference from libwebp's SIMD build. The x86-64
modes ran this test under Rosetta in the 4,488 above; natively it has not run
yet (#252).

## Not covered

- Native x86-64: no complete hosted Linux run yet (#252, above). Until one
  passes, the x86-64 SIMD result rests on Rosetta 2, which emulates AVX2 and
  ignores SSE alignment faults.
- On the x86-64 cross build on macOS the oracle's tools read no PNG, JPEG, TIFF
  or GIF (the host's libraries are ARM64 only); the tool tests pick their
  inputs by what the oracle's `cwebp` reads.
