# Quality gate guide

This page turns [ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md)
into commands. The ADR owns the rules; this page only shows how to check
them and what cannot be checked yet. State on 2026-11-13, `main` at
`dbf7b095` (after the gate work of the night of 2026-11-13). The commands are
collected from each port's `tools/check.py`, `tools/build_oracle.py`,
`tools/build_aom.py`, `tools/check_aom_simd_original.py` and port ADRs; they
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

**Hosted cross-check: not passed yet (open).** The first hosted Linux x86-64
run of the SIMD-against-SIMD gates, nightly run
[43401560676](https://github.com/sebastian-software/ferrisma/actions/runs/43401560676)
on `main` at `dbf7b095` (`ubuntu-latest`, the `ubuntu-26.04` image with
Clang 21.1.8), failed for libwebp-rs and libjxl-rs, in both cases on the
original's side of the comparison (the C and C++ oracle processes)
([#252](https://github.com/sebastian-software/ferrisma/issues/252)). That is
not yet evidence that a port differs from its original, but the hosted
cross-check has not passed for either port. The same checks passed on macOS
ARM64 in that run and locally under Rosetta 2 (#252). #252 does not record a
libjpeg-turbo-rs result. No workflow in `.github/workflows` runs the AOM
SIMD-against-original gate (`tools/check_aom_simd_original.py`), so no hosted
run of it exists. Tracking: [#241](https://github.com/sebastian-software/ferrisma/issues/241).

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
- **Recorded results.** On the merged tree: AArch64 2,331 passed, 0 failed
  ([#196 status of 2026-11-13](https://github.com/sebastian-software/ferrisma/issues/196));
  x86-64 under Rosetta 2, all four modes and three profiles, 4,488 passed,
  0 failed on `54a0c458` (follow-up comment on #196, details in #89).
- **Open:** a few test domains are narrowed for the `release-simd`
  comparison only, the `oracle` feature reads `LIBWEBP_SIMD_LEVEL`, and the
  full x86-64 gate takes about two and a half hours under Rosetta; the owner
  has not yet accepted these (open question O7,
  [#250](https://github.com/sebastian-software/ferrisma/issues/250)). The
  hosted Linux run failed in the `avx2` mode: the C probe
  `build/oracle/release-simd/probes/wp10/dsp` exited with SIGSEGV in
  `tests/wp10_dsp.rs`, so `sse41` and `sse2` did not run there (#252).
  Remaining x86 encoder SIMD work: #89, #83.

### libjxl-rs (`codecs/libjxl-rs`)

The scalar half exists on both targets. The SIMD half exists at row level
only: the SIMD stages are proven against the original's SIMD build, but the
decoder does not select them yet (port
[ADR 0004](../codecs/libjxl-rs/docs/adr/0004-simd-paths.md),
[port status](../codecs/libjxl-rs/docs/PORT_STATUS.md)).

```sh
# AArch64: scalar references (pinned HWY_COMPILE_ONLY_SCALAR oracle and the
# second scalar reference target/oracle-o3) plus the NEON oracle
# (target/oracle-neon) for tests/simd_parity.rs
python3 tools/check.py --stage test
# x86-64 under Rosetta: the same three oracles cross-built for x86-64
# (target/oracle-x86_64, target/oracle-o3-x86_64, target/oracle-avx2)
python3 tools/check.py --stage test --target x86_64-apple-darwin
```

- **SIMD oracle:** `tools/oracle` with `-DORACLE_HWY_TARGET=NEON` or `AVX2`
  restricts Highway to that target through `HWY_DISABLED_TARGETS`;
  `tools/check.py` passes it to the tests as `JXL_CPP_ORACLE_SIMD`. The AVX2
  oracle is a static AVX2 build, not upstream's dispatch build, so
  whole-decode AVX2 comparisons of non-kernel scalar code need a dispatch
  `djxl` (port ADR 0004, Consequences).
- **What exists:** a Highway operations layer (`src/jxl/hwy`, 63 operations
  for `HWY_NEON` and `HWY_AVX2`) and SIMD Gaborish, EPF (passes 0–2), YCbCr
  and chroma upsampling stages, which match the NEON and AVX2 oracles at row
  level in `tests/simd_parity.rs`. **They are not selected by the decoder.**
  The other Highway exports, the decoder wiring (which first needs the
  target's row padding) and the whole-decode comparison against a NEON/AVX2
  `djxl` are open (#96, #231; encoder SIMD #243).
- **Under Rosetta**, `tools/check.py` sets `ROSETTA_ADVERTISE_AVX=1` only for
  `tests/simd_parity.rs`. The other tests run without it, because the
  cross-built scalar references contain only skcms's baseline engine
  (upstream's `skcms.cmake` keys the HSW engine on `CMAKE_SYSTEM_PROCESSOR`,
  the ARM64 host). The port's skcms HSW engine against the original's is
  therefore provable only on native x86-64 (open question O9, #250).
- **NaN rule:** applied (port ADR 0002, 2026-11-13 section, `parity_eq` in
  `tests/parity.rs`). For requests that carry NaN or infinity, positions
  where the second scalar reference disagrees with the pinned one are not
  compared: open question O1's working rule, pending owner confirmation.
- **Recorded results.** Row-level SIMD evidence on 2026-11-13 (macOS
  27.0.1, M1 Ultra): all 63 operations on 85,085 requests per target, plus
  the Gaborish, YCbCr/chroma upsampling and EPF rows, with no finite word
  different (port ADR 0004, Evidence). On the merged tree: AArch64 1,027
  passed, 0 failed; x86-64 under Rosetta 2, 1,023 passed, 0 failed
  ([#196 status of 2026-11-13](https://github.com/sebastian-software/ferrisma/issues/196)).
- **Open:** on hosted Linux x86-64, the three SIMD stage tests of
  `tests/simd_parity.rs` failed with "write the oracle requests: Broken
  pipe" (the SIMD oracle exited before reading its input; the cause is not
  known), while the Highway operation test passed (#252). `rcpps`-based
  estimates are vendor-specific (open question O4).

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
- **Recorded results.** AArch64, with all five still-image Neon kernel
  families dispatched (509 RTCD functions): `neon` 82 of 82 and `scalar` 82
  of 82 cases identical to the original (port ADR 0056, SIMD-against-SIMD
  confirmation of 2026-11-13; #241). x86-64 through the container route:
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
  port's x86 kernels were not changed in the work of 2026-11-13 (#196).
- The identity check compares the port's SIMD with its own `_c` path; it
  proves the gate only where the original's SIMD equals its C.
- **Open:** O2 limits SIMD work to kernels on the still-image path; O4 covers
  libaom's NASM-only predictors; O8 asks the owner to accept the dispatch
  route through `tools/enable_aom_dispatch.py` and two kernels that stay on
  `_c` although the original dispatches Neon (`av1_get_nz_map_contexts`,
  `av1_round_shift_array`) (#250). Kernel work: #242, #125, #124, #122.

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
