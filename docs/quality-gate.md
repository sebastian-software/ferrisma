# Quality gate guide

This page turns [ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md)
into commands. The ADR owns the rules; this page only shows how to check
them and what cannot be checked yet. State on 2026-11-13. The commands are
collected from each port's `tools/check.py`, `tools/build_oracle.py` and
README; they were not run for this page.

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

```sh
# AArch64: scalar half (WEBP_ENABLE_SIMD=OFF), port with and without its SIMD modules
python3 tools/check.py --stage test --oracle
python3 tools/check.py --stage test --oracle --no-simd
# AArch64: SIMD half against libwebp's own SIMD build (today a diagnostic, port ADR 0008 rule 4)
python3 tools/check.py --stage test --oracle-variant release-simd
```

- **SIMD half:** the `release-simd` run exists but is not yet a required
  gate. It is expected to show the NEON rescaler export rows, where the port
  keeps the C translation (#79, #80); ADR-0041 requires the port to follow
  libwebp's NEON there.
- **x86-64 under Rosetta: not available yet.** Blocker:
  `tools/build_oracle.py` has no x86-64 cross build (no `-arch x86_64`
  option), so no x86-64 original can be built locally, scalar or SIMD.

### libjxl-rs (`codecs/libjxl-rs`)

```sh
# AArch64: scalar half (C++ oracle with HWY_COMPILE_ONLY_SCALAR)
python3 tools/check.py --stage test
# x86-64 under Rosetta: the port's own tests against retained captures, no live oracle
ROSETTA_ADVERTISE_AVX=1 cargo test -p libjxl-rs --locked --offline \
  --profile checked --target x86_64-apple-darwin
```

- **SIMD half: not available yet.** Blockers: the port has no SIMD paths
  (#96), and the oracle compiles only Highway's scalar target. A SIMD oracle
  is libjxl with Highway dispatch and every target above `HWY_NEON` or
  `HWY_AVX2` disabled (`HWY_DISABLED_TARGETS`).
- **x86-64 original under Rosetta: not available yet.** Blocker:
  `tools/check.py` has no x86-64 cross build, and the oracle's x86-64
  operand-order rules in `tools/oracle/CMakeLists.txt` apply only to Linux.
  Native Linux x86-64 runs the oracle in hosted CI.
- The NaN rule changes this port's oracle comparison (port ADR 0002/0003,
  2026-11-13 notes). Until that lands, the existing tests still compare NaN
  payloads.

### libavif-rs with libaom (`codecs/libavif-rs`)

```sh
# AArch64: retained golden outputs of the _c-only C reference, and SIMD-on/off identity
cargo xtask codecs --codec avif --stage pr        # from the repository root
python3 tools/check_aom_simd_identity.py
# C side of the golden record; needs the C builds of
# `python3 tools/root_pipeline.py --phase references` first
python3 tools/check_aom_golden.py --implementation c
```

- **Scalar half:** the golden outputs come from libavif with portable libaom
  (`_c` only, `-ffp-contract=off`, port ADR 0059). This is the scalar half.
- **SIMD half: not available yet.** Blocker: no native SIMD build of libaom
  serves as an oracle. The identity check compares the port's SIMD with its
  own `_c` path, which ADR-0041 no longer accepts as the gate. The benchmark
  script `scripts/bench/ci-perf/codec_avif.sh` builds C libavif + libaom
  with SIMD, but only for timing and a byte comparison; for the gate it would
  need `ENABLE_NEON_DOTPROD=0`, `ENABLE_NEON_I8MM=0`, `ENABLE_SVE=0`,
  `ENABLE_SVE2=0` on AArch64 and `ENABLE_AVX512=0` on x86-64.
- **x86-64 on macOS: not available yet.** Blocker: the generated native
  profiles cover macOS ARM64 and GNU Linux x86-64 only (pinned nightly), so
  the port does not build for `x86_64-apple-darwin`. x86-64 evidence comes
  from hosted Linux CI (`.github/workflows/avif-aom-simd-linux.yml`).
- Open question O2 limits SIMD work to kernels on the still-image path; O4
  covers libaom's NASM-only predictors.

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
