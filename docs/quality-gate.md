# Quality gate guide

This page turns [ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md)
into commands. The ADR owns the rules; this page only shows how to check
them and what cannot be checked yet. **Reviewed: 2026-11-16**, `main` at
`abd11e6a` (first written for `5004f34e` on 2026-11-14); the owner's
decisions of 2026-11-15 on O1–O9 (ADR-0041) are entered where they apply,
and [Where each port stands](#where-each-port-stands-2026-11-16) summarizes
the state. The commands are collected from each port's `tools/check.py`,
`tools/build_oracle.py`, `tools/build_aom.py`,
`tools/check_aom_simd_original.py`, `CONTRIBUTING.md` and port ADRs; they
were not run for this page. The results quoted below are the recorded ones,
each with its source; they are not results of this page.

## Where each port stands (2026-11-16)

No port meets the whole gate on both targets yet. What `main` at `abd11e6a`
records, per port (sources in the sections below). The homepage shows the
column "In short" of this table, generated at build time.

| Port | In short | AArch64: scalar and NEON | x86-64: scalar, SSE levels and AVX2 | What is open |
| --- | --- | --- | --- | --- |
| libjpeg-turbo-rs | Both halves of the check, scalar and SIMD, exist on AArch64 and x86-64; the x86-64 SIMD is written with intrinsics and compared with the original's assembly. | Both halves exist | Both halves exist (under Rosetta 2) | No recorded hosted x86-64 result (#252 records none) |
| libwebp-rs | Identical at every level in the recorded runs: NEON and scalar on AArch64; AVX2, SSE4.1 and SSE2 on hosted x86-64 Linux, also in the nightly. | Passed (`neon`, scalar) | Passed on hosted Linux x86-64 (`avx2`, `sse41`, `sse2`; run [43471093624](https://github.com/sebastian-software/ferrisma/actions/runs/43471093624)); the nightly's hosted oracle job passed on 2026-11-15 and 2026-11-16 (runs [43698564297](https://github.com/sebastian-software/ferrisma/actions/runs/43698564297), [43840953408](https://github.com/sebastian-software/ferrisma/actions/runs/43840953408)) | Tracking issues #89 and #83 are open |
| libjxl-rs decoder | NEON and AVX2 match the original word for word, AVX2 also in a hosted run on x86-64 hardware. | Passed (row level and public-decoder gate) | Passed on hosted Linux x86-64 at `7b48bc81`, where the decoder selects AVX2 by default (run [43656118559](https://github.com/sebastian-software/ferrisma/actions/runs/43656118559), every job successful) | `ToLinear` and `ToneMapping` only at row level |
| libjxl-rs encoder | SIMD by default since 2026-11-16; matches the original on NEON, and on AVX2 under Rosetta 2. A run on x86-64 hardware is open. | Passed: the encoder's SIMD path is selected by default since 2026-11-16 and matches the NEON original | Matches the AVX2 original under Rosetta 2 | No run on x86-64 hardware yet (port ADR 0004, condition (e)) |
| libavif-rs with libaom | Scalar and NEON identical on AArch64; scalar, SSE4.1 and AVX2 identical under QEMU emulation. A run on x86-64 hardware is open. | Passed (`neon` and `scalar`, 82 of 82) | `avx2`, `sse4.1` and `scalar` 82 of 82 each, under QEMU emulation; 358 RTCD functions dispatch on x86-64 | No run on x86-64 hardware: no workflow runs `check_aom_simd_original.py`; NASM-only kernels stay on `_c` (ADR-0044 D2) |

## The gate in one paragraph

For each port and each target (AArch64, x86-64), the port's scalar path must
be bit-identical to the original's scalar build, and each SIMD path in the
v1.0 scope (NEON on AArch64; AVX2 plus the SSE2/SSSE3/SSE4.1 kernels the
original selects on an AVX2 machine) must be bit-identical to the original's
SIMD build at the same level. Integer and byte outputs are exact. For
floating-point words, finite values, infinities and signed zeros are exact,
a NaN must meet a NaN, and NaN payloads are ignored, except on copy-only
paths. Inputs that contain NaN or infinity follow the rule the owner
confirmed on 2026-11-15 (formerly open question O1, ADR-0041). AVX-512, SVE
and NEON DotProd/I8MM are not run. There is no performance gate.

Never refresh an expected output, golden, tolerance or oracle pin to make a
check pass. A failure is a finding: report it.

**The gate holds on the toolchains users build with, not only on the pinned
nightly.** The workspace pins a nightly in `rust-toolchain.toml`, but users
build with a stable compiler at or above the declared `rust-version`
([supported matrix](supported-matrix.md#minimum-rust-versions)). The same
source can give different results on the two: on 2026-11-15 libjxl-rs's
`std::max(kSmallAlpha, alpha)` returned a NaN for a signaling-NaN alpha with
Rust 1.88 to 1.98 on AArch64 (LLVM 20 to 22 lower the compare-and-select
to `FMAXNM`), and the original's `kSmallAlpha` with the nightly and Rust
1.100 (LLVM 23) (libjxl-rs [ADR 0002](../codecs/libjxl-rs/docs/adr/0002-differential-oracle.md),
"Results that depend on the Rust compiler"). A result counts only if it
holds on the pinned nightly and on stable. Where the source's semantics
can be written so that no compiler choice decides them, the port writes
them that way; where a stage can run with another toolchain (libjxl-rs
`tools/check.py --toolchain NAME`), run it with the port's `rust-version`
and with current stable before calling a change done. CI's codec job runs
libjxl-rs's no-oracle checked-profile tests on the port's `rust-version`
and on stable, on Linux x86-64 and ARM64.

## Targets on the development Mac

| Target | How | Notes |
| --- | --- | --- |
| AArch64 | native | Apple M1 Ultra reports NEON, DotProd and FP16, no SVE or I8MM (ADR-0020). Build the original's SIMD build without DotProd/I8MM so that it uses the NEON level only. |
| x86-64 | `--target x86_64-apple-darwin` under Rosetta 2 | Set `ROSETTA_ADVERTISE_AVX=1` in the environment of every test process, or run-time detection does not report AVX2, FMA, BMI2 and F16C. Build C/C++ originals with `-arch x86_64` (`-DCMAKE_OSX_ARCHITECTURES=x86_64`). AVX-512 is not available. |

Install the target once: `rustup target add x86_64-apple-darwin` (for the
pinned toolchain and for each stable toolchain a port's checks use). Every run
should print the SIMD level detected and selected on both sides.

One hosted Linux x86-64 run per release candidate repeats the x86-64 checks
with the original built natively on the runner. Benchmark runners
(Blacksmith, the v1.0 performance workflow) do not count for the gate.

The machine of the later libjxl-rs work of 2026-11-13/14 had no Rosetta 2
until the afternoon of 2026-11-14 (libjxl-rs
[ADR 0004](../codecs/libjxl-rs/docs/adr/0004-simd-paths.md), Evidence: "the
local machine has no Rosetta 2"), so the x86-64 evidence of that work up to
then comes from hosted runs only. Since then the x86-64 gates run on that
machine too; on it, a cross link can fail with `ld: library 'c++' not found`
unless `SDKROOT=$(xcrun --show-sdk-path)` is set. Rosetta 2 results quoted below are
from earlier runs on another development Mac, as their sources state.

**Hosted cross-check: passed for libwebp-rs and the libjxl-rs decoder, open
for the others.** The
first hosted Linux x86-64 run of the SIMD-against-SIMD gates, nightly run
[43401560676](https://github.com/sebastian-software/ferrisma/actions/runs/43401560676)
on `main` at `dbf7b095` (`ubuntu-latest`, the `ubuntu-26.04` image with
Clang 21.1.8), failed for libwebp-rs and libjxl-rs on the original's side of
the comparison
([#252](https://github.com/sebastian-software/ferrisma/issues/252)). Both
causes were test scaffolding that handed the originals buffers not aligned
or sized as their own callers do; both are fixed on `main`. Since then:

- libwebp-rs passed every level on hosted Linux x86-64 (run
  [43471093624](https://github.com/sebastian-software/ferrisma/actions/runs/43471093624),
  details below).
- libjxl-rs's decoder passed on hosted Linux x86-64 with its AVX2 default
  (run [43656118559](https://github.com/sebastian-software/ferrisma/actions/runs/43656118559)
  at `7b48bc81`; every job successful, the sanitized shards included). The
  encoder's SIMD path, selected by default since 2026-11-16, has no hosted
  x86-64 run yet.
- #252 does not record a libjpeg-turbo-rs result.
- No workflow in `.github/workflows` runs the AOM SIMD-against-original gate
  (`tools/check_aom_simd_original.py`), so no hosted run of it exists. Its
  `avx2` level runs locally under QEMU emulation since 2026-11-14 (below).

Tracking: [#241](https://github.com/sebastian-software/ferrisma/issues/241).
The [status page](status.md#next-integration-and-release-gates) has the
same state in short.

## Per port

Run from the port's directory, after the setup in [codec development](codecs.md).

### libjpeg-turbo-rs (`codecs/libjpeg-turbo-rs`)

Both halves of the gate exist on both targets.

```sh
# AArch64: scalar half (portable original, crate without default features)
python3 tools/check.py --stage test --oracle-variant release
# AArch64: SIMD half (original with WITH_SIMD=1; NEON, then JSIMD_FORCENONE)
python3 tools/check.py --stage test --oracle-variant simd
# x86-64 under Rosetta: the same, with an x86-64 oracle in build/oracle/<variant>-x86_64
python3 tools/check.py --stage test --oracle-variant release --arch x86_64
python3 tools/check.py --stage test --oracle-variant simd --arch x86_64
```

`--arch x86_64` sets `ROSETTA_ADVERTISE_AVX=1` itself. The `simd` variant
checks AVX2, SSE2 (`JSIMD_FORCESSE2=1`) and none (`JSIMD_FORCENONE=1`) on
x86-64, and NEON and none on AArch64. The original's x86 SIMD is NASM
assembly; the port's intrinsics are compared with its output, which is the
route [ADR-0044](adr/0044-no-assembly-acceleration-through-rust-intrinsics.md)
prescribes for assembly-only kernels in general (owner decision 2026-11-15).

### libwebp-rs (`codecs/libwebp-rs`)

Both halves of the gate exist on both targets (port
[ADR 0008](../codecs/libwebp-rs/docs/adr/0008-simd-modules.md), parity
section revised 2026-11-13;
[SIMD status](../codecs/libwebp-rs/docs/SIMD-STATUS.md)).

```sh
# AArch64: scalar half (port --no-default-features against the `release`
# oracle, WEBP_ENABLE_SIMD=OFF) and SIMD half (default features against
# `release-simd`, LIBWEBP_SIMD_LEVEL=neon)
python3 tools/check.py --stage test --oracle
# Scalar half only
python3 tools/check.py --stage test --oracle --no-simd
# x86-64 under Rosetta: x86-64 oracle from tools/build_oracle.py --arch x86_64;
# scalar, then release-simd in the SIMD modes avx2, sse41 and sse2
python3 tools/check.py --stage test --oracle --arch x86_64
```

- **Modes** (`SIMD_MODES` in `tools/check.py`): `neon` on AArch64; `avx2`,
  `sse41` and `sse2` on x86-64. Each test stage runs the dev, release and
  checked profiles. `LIBWEBP_SIMD_LEVEL` caps the CPU query on both sides:
  `tools/oracle/probe_support/simd_level.c` wraps libwebp's `VP8GetCPUInfo`
  in every oracle executable, and the port's x86 query applies the same cap
  when its `oracle` feature is on. `avx2` includes the SSE2 and SSE4.1
  kernels libwebp falls back to. `tests/dispatch_features.rs` fails if the
  host reports less than the mode asks for.
- `--arch x86_64` builds the port for `x86_64-apple-darwin`, builds the
  oracle with `-DCMAKE_OSX_ARCHITECTURES=x86_64` into
  `build/oracle/<variant>-x86_64`, and sets `ROSETTA_ADVERTISE_AVX=1`.
- **NEON rescaler (#79, #80):** the port now selects libwebp's NEON export
  rows on AArch64 as libwebp does and matches its NEON build (port ADR 0008,
  "Upstream's own divergences").
- **Whole pipeline:** `tests/wp13_tools.rs` runs in every gate mode and, in
  each `release-simd` mode, compares the Rust-linked `cwebp` with libwebp's
  SIMD `cwebp` at the same capped level (`test_cwebp.sh` in three option
  sets and an option matrix); `the_cwebp_builds_follow_the_oracle_variant`
  fails if the Rust-linked `cwebp` is linked against the wrong build
  ([SIMD status](../codecs/libwebp-rs/docs/SIMD-STATUS.md), "Whole-pipeline
  gate").
- **Recorded results.** AArch64, after the #252 probe fix: `neon` 411 and
  scalar 368 tests passed, 0 failed, in each of the three profiles (2,337 in
  all; SIMD status, 2026-11-13). x86-64 under Rosetta 2, the scalar and the
  three SIMD modes in three profiles: 4,488 passed, 0 failed on `54a0c458`
  (#89, cited in the SIMD status; Rosetta 2 does not fault on misaligned SSE
  loads, so it could not show #252). Hosted Linux x86-64: run
  [43471093624](https://github.com/sebastian-software/ferrisma/actions/runs/43471093624)
  (`WEBP: Checks`, branch `v1/webp` at `690ad251`; its `codecs/libwebp-rs`
  tree is identical to `main`'s at `5004f34e`), job `WEBP: Oracle
  (ubuntu-latest)`: `avx2`, `sse41` and `sse2` 1,131 tests each, 0 failed;
  every job of the run passed. Since then the scheduled nightly's hosted
  `WEBP: Oracle (ubuntu-latest)` and `WEBP: Oracle (macos-latest)` jobs,
  which run the `release-simd` comparison per mode as O7 requires, passed
  on 2026-11-15 (run
  [43698564297](https://github.com/sebastian-software/ferrisma/actions/runs/43698564297))
  and 2026-11-16 (run
  [43840953408](https://github.com/sebastian-software/ferrisma/actions/runs/43840953408));
  their logs were not read for test counts. The earlier `avx2` failure (#252) was the
  `wp10/dsp` probe passing misaligned rows to libwebp's SSE2 predictors,
  not libwebp or the port. The SIMD status page on `main` still says that
  no hosted rerun had finished; it predates this run.
- **Accepted by the owner on 2026-11-15 (O7, ADR-0041):** the narrowed
  test domains of the `release-simd` comparison, under the single
  exception of ADR-0041 section 2 (only where libwebp itself is undefined,
  with the reason in the code; the scalar comparison keeps the full
  domain); the `oracle` feature reading `LIBWEBP_SIMD_LEVEL`; and the full
  x86-64 gate (three levels by three profiles) running hosted in the
  nightly on real x86-64, while a local Rosetta run covers what a change
  affects.
  The SIMD status lists no untranslated x86 SIMD file of libwebp 1.6.0;
  the tracking issues #89 and #83 are still open.

### libjxl-rs (`codecs/libjxl-rs`)

The scalar half exists on both targets. The SIMD half exists for the
decoder: every decode-path Highway export has a SIMD translation for NEON
and AVX2, compared with the original built for the same Highway target
(AVX2 runs locally under Rosetta 2 and natively on hosted x86-64), and the
decoder selects NEON by default on AArch64 and AVX2 on x86-64 when the CPU
has Highway's complete AVX2 group. The encoder's kernels `enc_xyb`,
`enc_transforms`, `enc_group` and `enc_ac_strategy` are translated and
compared the same way. **Since 2026-11-16 the encoder selects its SIMD
path by default** on AArch64 (NEON) and x86-64 (AVX2 by run-time detection),
as the decoder does; whole encodes with the encoder's SIMD target match the
original's NEON build natively and its AVX2 build under Rosetta 2, and the
scalar path is selected explicitly by the scalar-reference tests. What
remains open for the encoder is a run on x86-64 hardware (port ADR 0004,
"Encoder selection", condition (e)). Before 2026-11-16 the encoder met only
the scalar half (#231, #243; port
[ADR 0004](../codecs/libjxl-rs/docs/adr/0004-simd-paths.md),
[port status](../codecs/libjxl-rs/docs/PORT_STATUS.md),
[contributing](../codecs/libjxl-rs/CONTRIBUTING.md), "Which decoder path a
test runs").

```sh
# AArch64: the scalar references (pinned HWY_COMPILE_ONLY_SCALAR oracle and
# the second scalar reference target/oracle-o3) and the NEON oracle
# (target/oracle-neon) for the SIMD test binaries; every test invocation
# gets JXL_DECODER_TARGET=scalar
python3 tools/check.py --stage test
# the same against the ASan/UBSan-instrumented original
python3 tools/check.py --stage test --sanitizers
# one of N disjoint shares of the test names (as the hosted jobs run it), or
# every test binary to the end after a failure
python3 tools/check.py --stage test --partition 1/4
python3 tools/check.py --stage test --no-fail-fast
# x86-64 under Rosetta: the same oracles cross-built for x86-64
# (target/oracle-x86_64, target/oracle-o3-x86_64, target/oracle-avx2)
python3 tools/check.py --stage test --target x86_64-apple-darwin
```

- **SIMD test binaries** (`SIMD_TESTS` in `tools/check.py`, oracle in
  `JXL_CPP_ORACLE_SIMD`): `simd_parity`, `simd_render`, `simd_vardct`,
  `simd_public`, `simd_modular`, `simd_stages2`, `simd_color` and
  `simd_threads`. They force the detected target with
  `set_decoder_simd_for_test(Some(true))`. `tests/simd_public.rs` is the
  public-decoder gate: whole decodes through the public decoder and its
  low-memory pipeline, float, 8- and 16-bit output, over the `decode_vardct`
  gate files, the scalar suite's public-decoder inputs and the pinned
  conformance corpus, once as the engine decodes and once with the default
  CMS and a Rec. 2020 PQ output profile. `tests/simd_threads.rs` decodes
  with 0, 1, 2 and 8 runner workers. `tests/simd_vardct_lanes8.rs` needs no
  oracle: it runs the generic VarDCT kernels with an 8-lane descriptor (two
  NEON halves) against NEON's 4 lanes, for AVX2's loop structure on an
  AArch64 host; it does not test AVX2's instructions.
- **Which path a test runs:** scalar-reference tests select the scalar
  decoder path themselves (`hwy::select_scalar_decoder_for_test()`, 245
  tests in 47 files); `tools/check.py` also sets `JXL_DECODER_TARGET=scalar`
  as a second guard, which `the_scalar_gate_forces_the_scalar_decoder_path`
  checks; `tests/dispatch_scope.rs` runs the default selection and checks
  where it applies (port ADR 0004, "Which path a test runs", "Scope of the
  selection"). The encoder has the same mechanism since 2026-11-16
  (`hwy::select_scalar_encoder_for_test()`, `JXL_ENCODER_TARGET=scalar`).
- **SIMD oracle:** `tools/oracle` with `-DORACLE_HWY_TARGET=NEON` or `AVX2`
  restricts Highway to that target through `HWY_DISABLED_TARGETS`. On
  `main` the AVX2 oracle is a static AVX2 build, which also contracts scalar
  code outside Highway's per-target code with FMA. The owner decided on
  2026-11-14 that the AVX2 reference is upstream's dynamic-dispatch release
  build capped at `HWY_AVX2`, as ADR-0041 section 1 defines it; that build
  is on `main` since `81d2adab` (port ADR 0004, "AVX2 reference: upstream's
  dispatch build"); the sentence above describes `main` before that commit.
- **Under Rosetta**, `tools/check.py` sets `ROSETTA_ADVERTISE_AVX=1` only for
  the SIMD test binaries. The other tests run without it, because the
  cross-built scalar references contain only skcms's baseline engine
  (upstream's `skcms.cmake` keys the HSW engine on `CMAKE_SYSTEM_PROCESSOR`,
  the ARM64 host). The port's skcms HSW engine against the original's is
  therefore provable only on native x86-64. Owner decision of 2026-11-15
  (O9): the cross builds contain the HSW engine; implemented on 2026-11-16
  with `-DCMAKE_APPLE_SILICON_PROCESSOR=x86_64`, since CMake ignores
  `CMAKE_SYSTEM_PROCESSOR` outside cross-compiling mode (port ADR 0004,
  "The skcms HSW engine in its attribute context").
- **NaN rule:** applied (port ADR 0002, 2026-11-13 section, `parity_eq` in
  `tests/parity.rs`). For requests that carry NaN or infinity, positions
  where the second scalar reference disagrees with the pinned one are not
  compared: rule O1, confirmed by the owner on 2026-11-15 (ADR-0041).
- **`rcpps`:** on AVX2 the SIMD DequantBlock executes the original's
  vendor-specific `_mm256_rcp_ps` estimate, so AVX2 output can differ
  between CPU vendors exactly as the original's does; identity is claimed
  per host against the original, never across hosts (port ADR 0004, VarDCT
  decode kernels, "Dispatch"). Owner decision of 2026-11-15 (O4,
  [ADR-0044](adr/0044-no-assembly-acceleration-through-rust-intrinsics.md)
  D3): the instruction stays as in the original, also in shipped defaults.
- **Recorded results, AArch64 (NEON).** Row level on 2026-11-13 (macOS
  27.0.1, M1 Ultra): all 63 operations of that day on 85,085 requests, plus
  the Gaborish, YCbCr/chroma upsampling and EPF rows, with no finite word
  different (port ADR 0004, Evidence). Public-decoder gate on the combined
  branch (2026-11-14): all 352 distinct inputs match the NEON original word
  for word in f32, u8 and u16, in both modes; no input reaches the
  `ToLinear` and `ToneMapping` stages, whose evidence is row-level parity
  only (port ADR 0004, "Public decoder gate", "Default selection per
  target"). Worker counts: 12 multi-group files and the 40 conformance
  inputs at 0, 1, 2 and 8 workers, zero differing words against the
  runner-free decode and against the NEON original (port status,
  "Concurrent AC group decode"). 8-lane structure: 1,120 transform, 162
  DequantBlock and 929 DC smoothing, DequantDC and quant-weight cases with
  equal words (port ADR 0004). The last whole-suite counts quoted on #196
  (AArch64 1,027 passed, 0 failed; Rosetta 2 1,023 passed, 0 failed) are
  from the merged tree of 2026-11-13, before the SIMD decode work; no
  whole-suite count of the state on `main` is recorded.
- **Recorded results, x86-64 (AVX2).** Under Rosetta 2 on 2026-11-13: the 63
  operations and the three row stages (port ADR 0004, Evidence). Hosted
  Linux x86-64: run
  [43533597186](https://github.com/sebastian-software/ferrisma/actions/runs/43533597186)
  (branch `v1/jxl-merge7` at `718b32d6`, static AVX2 oracle) failed four row
  tests (CMS, tone mapping, splines, blending), each in one word: by one to
  three units in the last place, and once in the sign of a zero (the table
  in port ADR 0004). Run
  [43558043274](https://github.com/sebastian-software/ferrisma/actions/runs/43558043274)
  (branch `v1/jxl-avx2-fixes`, based on `718b32d6`, with the dispatch-build
  oracle) passed every job of `JXL: Checks`, including the SIMD test
  binaries and the public-decoder gate on AVX2; that branch predates the
  NEON default, the concurrent AC group decode and the row changes. In
  run
  [43599382021](https://github.com/sebastian-software/ferrisma/actions/runs/43599382021)
  (the tree merged to `main` as `81d2adab`, with all of those changes) the
  four ordinary-oracle test shards passed on AVX2; its sanitized shards had
  not finished when this page was written.
- **Recorded results, hosted x86-64 with the AVX2 default:** run
  [43656118559](https://github.com/sebastian-software/ferrisma/actions/runs/43656118559)
  (`JXL: Checks` at `7b48bc81`, where the decoder selects AVX2 by default)
  completed with every job successful, the ordinary and sanitized oracle
  shards and the public-decoder audit included (#96 is closed).
- **Encoder, recorded results (2026-11-15/16):** the encoder kernels'
  parity binaries compare 21,850 requests per target with zero differing
  words on NEON and on AVX2 under Rosetta 2, except the AVX2 `aq_map` words
  the owner's decision of 2026-11-15 excludes; whole encodes with the SIMD
  target (lossy, lossy alpha, streaming, lossless effort 2) show zero
  differing encodes on both; with the switch on, the whole NEON
  `--stage test` passes, 1,162 tests (port ADR 0004, "Encoder selection").
  The lossy path was then profiled on hosted x86-64 and ARM64 and tightened
  in form only (port ADR 0004, 2026-11-16: blocked DCT transposes,
  checked-once loops, row access), with every output unchanged.
- **Open:** the encoder on x86-64 hardware (port ADR 0004, condition (e));
  the lossless encoder kernels and the color-management kernel have no SIMD
  translation yet (#243, #231); `ToLinear` and `ToneMapping` in a whole
  decode (no HDR input in the gate set).

### libavif-rs with libaom (`codecs/libavif-rs`)

Both halves exist on AArch64. On x86-64 all three levels, `avx2`, `sse4.1`
and `scalar`, run locally in one GNU Linux environment under QEMU emulation
since 2026-11-14 (port ADR
[0058](../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md),
amendment "the AVX2 level runs locally under QEMU"); before, only `sse4.1`
and `scalar` had run (port ADR
[0056](../codecs/libavif-rs/docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md),
amendment of 2026-11-13). QEMU is not x86-64 hardware, so a hosted run
remains.

```sh
# Reference builds of the original (library only), once per level
python3 tools/build_aom.py --library-only                # generic (_c) scalar reference, arm64
python3 tools/build_aom.py --library-only --simd neon    # Neon only: no DotProd, I8MM, SVE, SVE2
python3 tools/build_aom.py --library-only --x86-64       # generic, x86-64 (Rosetta 2)
python3 tools/build_aom.py --library-only --simd avx2    # x86-64 up to AVX2, no AVX-512
# Port against the original at the same level: AVIF bytes, decoded planes, RGB
python3 tools/check_aom_simd_original.py                 # the levels of this host
# macOS ARM64: the port's GNU Linux build in an x86-64 container, the x86-64
# originals under Rosetta 2; sse4.1 and scalar only
python3 tools/check_aom_simd_original.py --x86-64 --docker-image IMAGE
# x86-64 avx2, sse4.1 and scalar under QEMU, inside the tools/qemu_x86_64
# container (port ADR 0058, amendment of 2026-11-14)
python3 tools/check_aom_simd_original.py --qemu
# Retained checks: golden outputs of the _c-only C reference, and SIMD-on/off identity
cargo xtask codecs --codec avif --stage pr               # from the repository root
python3 tools/check_aom_golden.py --implementation c
python3 tools/check_aom_simd_identity.py
```

- **Levels** (`tools/check_aom_simd_original.py`): `neon` and `scalar` on
  macOS ARM64; `avx2`, `sse4.1` and `scalar` on x86-64 GNU Linux. Every
  original is built with `-ffp-contract=off`, the declared noncontracting
  reference (port ADR 0024); `--fp-contract on` builds upstream's default
  contraction for diagnosis only. The arm64 original requires Neon, so the
  generic build is its scalar counterpart.
- **Scalar half:** the golden outputs from the `_c`-only C reference (port
  ADR 0059) and the `scalar` level against the generic build.
- **Recorded results.** AArch64, after four more still-image Neon kernels
  were dispatched (`aom_sum_sse_2d_i16`, `av1_get_horver_correlation_full`,
  `av1_get_nz_map_contexts`, `av1_round_shift_array`; 513 RTCD functions on
  the arm64 overlay): `neon` 82 of 82 after each kernel, and on the final
  tree `neon` 82 of 82 and `scalar` 82 of 82 cases identical to the
  original; `cargo test --release -p libavif-aom --test simd` 152 passed;
  the PR stage passes with golden outputs 62 of 62 (port ADR 0056,
  amendment "the remaining still-image Neon kernels", 2026-11-13, loaded
  host). Before that, with five kernel families (509 RTCD functions), the
  same 82 of 82 (port ADR 0056, SIMD-against-SIMD confirmation; #241). x86-64 through the container route:
  `sse4.1` 86 of 86 and `scalar` 86 of 86 identical (port ADR 0056,
  Results). In that route the port runs in an x86-64 Ubuntu 20.04 container
  and the originals run on macOS, so the two sides use different C
  libraries and libm.
- **x86-64 limits.** The port builds only for macOS ARM64 and GNU Linux
  x86-64: its translation profiles are platform-specific, so
  `x86_64-apple-darwin` is not a target (port ADR 0058, amendment of
  2026-11-13). Rosetta for Linux does not execute AVX, so the `avx2` level
  runs under QEMU instead (since 2026-11-14), which is an emulator, not
  x86-64 hardware: it has **never run on x86-64 hardware**. No workflow runs
  `check_aom_simd_original.py`; `.github/workflows/avif-aom-simd-linux.yml`
  runs the kernel tests, the identity check and the golden check only.
- **x86-64 kernels (2026-11-16).** 358 RTCD functions dispatch through 444
  kernel assignments on x86-64 (SSE2 to AVX2;
  [`crates/aom/README.md`](../codecs/libavif-rs/crates/aom/README.md)); each
  family was committed with `avx2`, `sse4.1` and `scalar` 82 of 82
  identical under QEMU (port ADR 0058, "Still-image families on x86"). The
  kernels that exist only as NASM (listed in ADR-0044) stay on `_c` until they are rebuilt with intrinsics
  (ADR-0044 D2). On 2026-11-16 the `avx2` level also passed 82 of 82 with
  overflow checks on ([publishing](publishing.md)).
- The identity check compares the port's SIMD with its own `_c` path; it
  proves the gate only where the original's SIMD equals its C.
- **Nightly `root / generate` (#204):** the production-package and profile
  checks did not know the handwritten `src/read_sample_access.rs` that every
  `read` module includes; `root_production.OWNED` now declares it, with no
  unit or selector hash changed (`63d77626`, port ADR 0054 amendment,
  working decision 2026-11-13, delegated, revisable by the owner). No
  nightly has completed since the fix.
- **Decided by the owner on 2026-11-15 (ADR-0041):** O2 keeps libaom's
  video code and limits SIMD work to kernels on the still-image path; the
  NASM-only kernels stay on `_c` until they are measured and rebuilt with
  intrinsics (O4,
  [ADR-0044](adr/0044-no-assembly-acceleration-through-rust-intrinsics.md));
  the translated code is owned code, changed only through committed tools
  with a check mode in CI (O8, port ADR 0059). The two kernels #250
  names as staying on `_c` (`av1_get_nz_map_contexts`,
  `av1_round_shift_array`) are dispatched to Neon since the amendment above;
  `aom_quantize_b_helper` stays on `_c` because libaom calls only
  `aom_quantize_b_helper_c` by name. Kernel work: #242, #125, #124, #122.

### Engine kernels (not part of this gate)

`ferrisma-kernels` has no original. Its scalar/SIMD contract stays in
[ADR-0020](adr/0020-kernel-crate-and-isa-dispatch.md); its AVX-512 level
must be off by default in v1.0 (ADR-0041 section 4). To exercise AVX2 under
Rosetta:

```sh
ROSETTA_ADVERTISE_AVX=1 cargo test -p ferrisma-kernels --target x86_64-apple-darwin
```

## Performance table template

Performance is reported, not gated ([benchmark protocol](benchmarks/M1-protocol.md),
ADR-0041 section 6). One table per codec, operation, input and target.
Port and original run in the same job, interleaved; the original's builds
for (b) and (c) are capped at the v1.0 SIMD levels.

```markdown
Codec: <name>, original <version and commit>, port <commit>
Operation: <decode/encode, settings>, input <file, dimensions>
Target: <AArch64 native | x86-64 under Rosetta (ROSETTA_ADVERTISE_AVX=1) | hosted Linux x86-64>
Host: <CPU, cores, OS>, load average <before–after>; <idle | indicative; loaded host>
Repetitions: <warm-up, n>, interleaved; median (min–max)

| Column | Original build | Threads | SIMD level (original / port) | Original ms | Port ms | Port ÷ original |
| --- | --- | ---: | --- | ---: | ---: | ---: |
| (a) translation loss | scalar reference build | 1 | none / none | | | |
| (b) | release, SIMD | 1 | <NEON or AVX2> / <same> | | | |
| (c) | release, SIMD | <N> | <NEON or AVX2> / <same> | | | |
| context (optional) | <third-party library or original at full level> | | | | | not a criterion |

Equivalence: <gate result for this input and target, or "not checked">
```

Close column (a) first, then (b), then (c).
