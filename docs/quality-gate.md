# Quality gate guide

This page turns [ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md)
into commands. The ADR owns the rules; this page only shows how to check
them and what cannot be checked yet. State on 2026-11-14, `main` at
`5004f34e`. The commands are collected from each port's `tools/check.py`,
`tools/build_oracle.py`, `tools/build_aom.py`,
`tools/check_aom_simd_original.py`, `CONTRIBUTING.md` and port ADRs; they
were not run for this page. The results quoted below are the recorded ones,
each with its source; they are not results of this page.

## The gate in one paragraph

For each port and each target (AArch64, x86-64), the port's scalar path must
be bit-identical to the original's scalar build, and each SIMD path in the
v1.0 scope (NEON on AArch64; AVX2 plus the SSE2/SSSE3/SSE4.1 kernels the
original selects on an AVX2 machine) must be bit-identical to the original's
SIMD build at the same level. Integer and byte outputs are exact. For
floating-point words, finite values, infinities and signed zeros are exact,
a NaN must meet a NaN, and NaN payloads are ignored, except on copy-only
paths. Inputs that contain NaN or infinity follow open question O1 (working
rule pending owner confirmation). AVX-512, SVE and NEON DotProd/I8MM are not
run. There is no performance gate.

Never refresh an expected output, golden, tolerance or oracle pin to make a
check pass. A failure is a finding: report it.

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

The machine of the later libjxl-rs work of 2026-11-13/14 has no Rosetta 2
(libjxl-rs [ADR 0004](../codecs/libjxl-rs/docs/adr/0004-simd-paths.md),
Evidence: "the local machine has no Rosetta 2"), so that work's x86-64
evidence comes from hosted runs only. Rosetta 2 results quoted below are
from earlier runs on another development Mac, as their sources state.

**Hosted cross-check: passed for libwebp-rs, open for the others.** The
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
- libjxl-rs has no passing hosted AVX2 run of the state on `main`; the
  details and the AVX2 reference decided on 2026-11-14 are below.
- #252 does not record a libjpeg-turbo-rs result.
- No workflow in `.github/workflows` runs the AOM SIMD-against-original gate
  (`tools/check_aom_simd_original.py`), so no hosted run of it exists.

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
assembly; the port's intrinsics are compared with its output (see open
question O4 for assembly-only kernels in general).

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
  every job of the run passed. The earlier `avx2` failure (#252) was the
  `wp10/dsp` probe passing misaligned rows to libwebp's SSE2 predictors,
  not libwebp or the port. The SIMD status page on `main` still says that
  no hosted rerun had finished; it predates this run.
- **Open:** a few test domains are narrowed for the `release-simd`
  comparison only, the `oracle` feature reads `LIBWEBP_SIMD_LEVEL`, and the
  full x86-64 gate takes about two and a half hours under Rosetta; the owner
  has not yet accepted these (open question O7,
  [#250](https://github.com/sebastian-software/ferrisma/issues/250)).
  The SIMD status lists no untranslated x86 SIMD file of libwebp 1.6.0;
  the tracking issues #89 and #83 are still open.

### libjxl-rs (`codecs/libjxl-rs`)

The scalar half exists on both targets. The SIMD half exists for the
decoder: every decode-path Highway export has a SIMD translation for NEON
and AVX2, compared with the original's single-target SIMD build (AVX2 is
compile-checked locally and runs only on hosted x86-64), and the decoder
selects NEON by default on AArch64. On x86-64 the decoder keeps the
scalar path, and the encoder's Highway exports are unported on both targets
(#231, #243), so the encoder can meet only the scalar half (port
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
  checks; `tests/dispatch_scope.rs` alone runs the default selection and
  checks that the encoder stays scalar (port ADR 0004, "Which path a test
  runs", "Scope of the selection").
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
  therefore provable only on native x86-64 (open question O9, #250).
- **NaN rule:** applied (port ADR 0002, 2026-11-13 section, `parity_eq` in
  `tests/parity.rs`). For requests that carry NaN or infinity, positions
  where the second scalar reference disagrees with the pinned one are not
  compared: open question O1's working rule, pending owner confirmation.
- **`rcpps`:** on AVX2 the SIMD DequantBlock executes the original's
  vendor-specific `_mm256_rcp_ps` estimate, so AVX2 output can differ
  between CPU vendors exactly as the original's does; identity is claimed
  per host against the original, never across hosts (port ADR 0004, VarDCT
  decode kernels, "Dispatch": working decision 2026-11-13 evening,
  delegated, revisable by the owner). Whether that is acceptable for
  shipped defaults stays open question O4.
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
- **Open:** x86-64 default selection (port ADR 0004 condition (d), #96);
  the dispatch-build AVX2 oracle on `main`; the encoder's Highway exports
  (#231, #243); `ToLinear` and `ToneMapping` in a whole decode (no HDR input
  in the gate set); O1, O4 and O9 (#250).

### libavif-rs with libaom (`codecs/libavif-rs`)

Both halves exist on AArch64. On x86-64 the comparison against the original
exists for the `sse4.1` and `scalar` levels; the `avx2` level has never run
(port ADR [0056](../codecs/libavif-rs/docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md)
and [0058](../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md),
amendments of 2026-11-13).

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
  needs a native x86-64 host and **has never run**. No workflow runs
  `check_aom_simd_original.py`; `.github/workflows/avif-aom-simd-linux.yml`
  runs the kernel tests, the identity check and the golden check only. The
  port's x86 kernels were not changed in the work of 2026-11-13 (#196; the
  port ADR 0056 amendment of that day leaves "all x86 counterparts"
  unchanged).
- The identity check compares the port's SIMD with its own `_c` path; it
  proves the gate only where the original's SIMD equals its C.
- **Nightly `root / generate` (#204):** the production-package and profile
  checks did not know the handwritten `src/read_sample_access.rs` that every
  `read` module includes; `root_production.OWNED` now declares it, with no
  unit or selector hash changed (`63d77626`, port ADR 0054 amendment,
  working decision 2026-11-13, delegated, revisable by the owner). No
  nightly has completed since the fix.
- **Open:** O2 limits SIMD work to kernels on the still-image path; O4 covers
  libaom's NASM-only predictors; O8 asks the owner to accept the dispatch
  route through `tools/enable_aom_dispatch.py` (#250). The two kernels O8
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
