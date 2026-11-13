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

Proven in the `neon` mode (native, release profile, 2026-11-13): the whole
differential suite, 410 tests passed, 0 failed (before the later
test-domain commits; the changed tests `wp4_filters`, `wp4_rescaler`,
`wp5_cpu`, `wp5_dsp`, `wp6_decode` and `wp11_lossless_enc` were rerun in the
`neon` mode and pass). The scalar mode: 367 passed, 0 failed.

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

Proven under Rosetta 2 (release profile, 2026-11-13): `avx2` 376 tests
passed, 0 failed; `sse41` and `sse2` 375 passed and 1 failed each
(`wp5_cpu`: the probe read libsharpyuv's own `SharpYuvGetCPUInfo`, which the
level cap did not wrap yet). After `simd_level_sharpyuv.c` was added,
`wp5_cpu`, `wp5_dsp` and `wp20_sharpyuv` pass in all three modes; the full
suites were not rerun after that fix. The x86-64 scalar mode was interrupted
after 71 passing tests and has no complete result.

## Not covered

- The gate ran the release profile per mode locally; `tools/check.py` runs the
  dev, release and checked profiles. The hosted Linux x86-64 run of decision D7
  (native AVX2) has not run yet.
- On the x86-64 cross build on macOS the oracle's tools read no PNG, JPEG, TIFF
  or GIF (the host's libraries are ARM64 only); the tool tests pick their
  inputs by what the oracle's `cwebp` reads.
