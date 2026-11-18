# ADR-0041: v1.0 quality gate: equivalence with the ported original

- **Status:** Accepted (decisions D1–D10). The former open questions O1–O9 and the further items listed with them were decided by the owner on 2026-11-15 ([Owner decisions of 2026-11-15](#owner-decisions-of-2026-11-15)); what is not yet implemented is named there.
- **Date:** 2026-11-13
- **Last reviewed:** 2026-11-16
- **Source:** owner decision (structured interview, 2026-11-13)
- **Supersedes:** In part: the performance criteria of [0007](0007-correctness-and-performance-evidence.md) as release evidence; the default ISA level of [0020](0020-kernel-crate-and-isa-dispatch.md) decision 4; the role of third-party comparisons in [0025](0025-codec-selection-reference-conformance-and-image-scope.md) point 3, [0032](0032-jpeg-xl-workspace-decoder-input.md) and [0037](0037-selectable-workspace-avif-encoder.md); the validation platforms of [0027](0027-tiered-validation-and-rust-test-orchestration.md); release naming in [0040](0040-stable-public-contract-and-compatibility-policy.md). In the ports: the NaN-payload rule of libjxl-rs ADR 0002/0003, the SIMD reference of libavif-rs ADR 0056/0058/0059, and the SIMD reference and NEON rescaler ruling of libwebp-rs ADR 0002/0008. Each amended record links back here.
- **Related:** [0026](0026-codecs-in-one-repository-and-workspace.md), [quality gate guide](../quality-gate.md), [v1.0 CI performance record](../benchmarks/v1.0-ci-performance-2026-11-12.md), [JPEG XL performance analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md), tracker #196

## Context

Ferrisma's four codec ports (libjpeg-turbo-rs, libwebp-rs, libjxl-rs,
libavif-rs with its libaom port) are translations of pinned originals. Until
now each port chose its own reference, and the references differ in what they
prove:

- libjpeg-turbo-rs compares its SIMD build with libjpeg-turbo's SIMD build
  (oracle variant `simd`) and its portable build with the portable original
  (`tools/check.py`, port ADR 0007/0008).
- libwebp-rs compares against an original built with `WEBP_ENABLE_SIMD=OFF`
  and admits a SIMD function by a Rust-against-Rust test against its own `_C`
  translation; the original's SIMD build (`release-simd`) is a local diagnostic
  only (port [ADR 0008](../../codecs/libwebp-rs/docs/adr/0008-simd-modules.md)).
  Where libwebp's NEON rescaler export rows differ from its C functions, the
  port keeps the C translation (#79, #80).
- libjxl-rs compares against libjxl built with `HWY_COMPILE_ONLY_SCALAR` and no
  auto-vectorization; it has no SIMD paths (#96). Its ADRs require NaN payloads
  to match the oracle, which forced out-of-line arithmetic helpers and kept the
  faster row kernels off x86-64
  ([port ADR 0002](../../codecs/libjxl-rs/docs/adr/0002-differential-oracle.md), 2026-11-12 amendments).
- libavif-rs dispatches a hand-translated AOM kernel only if it returns exactly
  what the port's own `_c` translation returns, and gates owned code on golden
  outputs recorded from a `_c`-only C reference
  ([ADR 0056](../../codecs/libavif-rs/docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md),
  [0058](../../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md),
  [0059](../../codecs/libavif-rs/docs/adr/0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md)).
  On macOS ARM64 the port's AVIF output therefore differs from the libaom
  release build, which uses libaom's own Neon/dotprod/i8mm kernels
  ([CI record, correctness side checks](../benchmarks/v1.0-ci-performance-2026-11-12.md#correctness-side-checks-all-hosts)).

At the same time, earlier ADRs and status pages treated speed against
third-party libraries (jxl-oxide, rav1e, Sharp) and quiet-host timing as open
release gates for the default codecs. The owner settled both questions on
2026-11-13. This record states those decisions as testable rules, so that the
ports, the engine and future contributors have one authoritative definition of
"v1.0".

## Decision

### 1. Terms

- **Original:** the pinned upstream source a port translates (libjpeg-turbo
  3.2.1 `7b499378`; libwebp 1.6.0 `4fa21912`; libjxl `a16d32d0` with its
  pinned Highway, skcms and Brotli; libavif 1.4.2 with libaom 3.15.2 and
  dav1d 1.5.4 from the tracked snapshots), unmodified.
- **Scalar reference build:** the original compiled with its own SIMD disabled
  through its own switch (`WITH_SIMD=0`, `WEBP_ENABLE_SIMD=OFF`,
  `HWY_COMPILE_ONLY_SCALAR`, libaom's generic `_c` configuration), with the
  compiler flags that the port's oracle tooling declares and records.
- **SIMD reference build at level L:** the original's release build with its
  SIMD enabled and its dispatch capped at L through its own switches (for
  example libaom `ENABLE_AVX512=0`, `ENABLE_NEON_DOTPROD=0`,
  `ENABLE_NEON_I8MM=0`, `ENABLE_SVE=0`, `ENABLE_SVE2=0`; Highway
  `HWY_DISABLED_TARGETS` for every target above `HWY_AVX2` or `HWY_NEON`).
- **Execution kind:** the scalar path, or the SIMD path at one level L, of the
  port and of the original, each at a given thread count and setting.
- **Target:** the CPU architecture a comparison runs on, AArch64 or x86-64.
  Port and original run on the same target and the same host.

### 2. The quality gate (D4)

> **A codec port meets the v1.0 quality gate on a target when, for every input
> and setting in its declared scope, (1) its scalar path produces output
> bit-identical to the original's scalar reference build, and (2) for every
> SIMD level in the v1.0 scope of that target, its SIMD path at that level
> produces output bit-identical to the original's SIMD reference build at the
> same level, at the same settings and thread count. Port and original run on
> the same target: the port on AArch64 against the original on AArch64, the
> port on x86-64 against the original on x86-64. "Output" is everything the
> port's differential protocol observes: encoded bytes, decoded samples,
> statuses and metadata. Integer and byte outputs must be identical without
> exception; floating-point words follow section 3. Ferrisma 1.0 requires the
> gate for all four ports on both targets; a pass on one target says nothing
> about the other.**

Consequences of the wording, stated so that they can be tested:

- Bit identity between the port's SIMD and its own scalar path is not the
  gate. Where the original's SIMD build differs from its scalar build, the
  port's SIMD path must reproduce the original's SIMD result, not the C one.
- Identity across targets is not required. A port may give different bytes
  on AArch64 and x86-64 exactly where the original does (for example through
  target-specific contraction, libjxl-rs ADR 0003).
- Thread count is a setting. Where the original's output depends on it (for
  example AOM autotiling when threads ≠ 1), the port is compared at the same
  count; where it does not, the port's must not either.
- Declared scope: the inputs of the port's existing differential, golden and
  conformance suites, the photographs of the
  [v1.0 CI performance record](../benchmarks/v1.0-ci-performance-2026-11-12.md),
  and the settings the engine can reach. A port may declare more; it may not
  declare less without an amendment here.
- **A test's input domain is never narrowed to make a comparison pass.**
  The single exception (owner decision of 2026-11-15 on O7): a comparison
  with the original's SIMD build may leave out exactly the inputs on which
  the original itself is undefined, that is, inputs that its own C asserts
  reject or on which it has undefined behavior. The reason is written in
  the test code next to the narrowed domain, and the comparison of the
  port's scalar path with the scalar reference build keeps the full domain.
  A difference, a run time or a defect of the port never justifies a
  narrower domain.
- **Expected values come from the original as its source defines them.**
  One named exception (owner decision of 2026-11-16): where a compared
  observation of the original depends on something the C++ standard library
  leaves unspecified and its versions resolve differently, the oracle
  adapter makes that observation the same on every platform, and the port
  is not changed to mirror one library version. The case that led to it is
  the order in which a `std::map` frees its elements when it is destroyed:
  libc++ 22 frees in key order, libc++ 19 and 21 in post-order, so the
  original's memory-manager trail at `JxlEncoderDestroy` and at the
  decoder's teardown differs between the macOS and the hosted Linux builds
  of the same source (libjxl-rs
  [ADR 0003](../../codecs/libjxl-rs/docs/adr/0003-rust-representation.md),
  "Public encoder object"). The adapter erases the map's entries in key
  order before it destroys the object. Each such normalization is written
  in the adapter next to the code, names the library behavior and the
  original's source lines, and is listed in the port's ADR. It applies only
  to an order or layout the library leaves open, never to a value the
  original's own source computes, and never to the port.
- The port follows the original's algorithms and must produce what this
  gate compares; it need not mirror the original's code shape. Straight,
  idiomatic, maintainable Rust is preferred over a 1:1 transliteration of
  the original's assembly or intrinsics, and no code outwits the compiler
  or Rust's rules to reproduce an instruction stream (owner, 2026-11-18;
  amendment log). Some speed loss is acceptable for that. An optimization
  that cannot meet the gate is not a v1.0 change.

### 3. Floating-point words and NaN (D8)

For every compared floating-point word:

1. If neither word is a NaN, the two words must be bit-identical. +0 and −0
   are different; infinities must match in sign.
2. If exactly one word is a NaN, the comparison fails.
3. If both words are NaNs, they compare equal, whatever their sign, quiet bit
   and payload. **NaN payload bits are not part of the gate.**
4. Exception: where the original only copies a word without arithmetic (for
   example lossless float images, plane and buffer copies, pass-through
   metadata), a NaN must stay bit-identical as well.

The port must not classify or rewrite NaN bits to pass a comparison, and a
NaN difference that changes an integer or byte output is a failure under
section 2. Inputs that themselves contain NaN or infinity follow the rule
the owner confirmed on 2026-11-15 (formerly open question O1; see
[Owner decisions of 2026-11-15](#owner-decisions-of-2026-11-15)).

Evidence for the rule (as reported in the fact-finding of 2026-11-13; the
raw comparison artifacts are not committed):

- The pinned libjxl changes NaN payloads between its own builds. With the
  same compiler and only the optimization level changed, 123,110 output words
  differed on arm64 and 116,971 on x86-64, all of them NaN payloads, with zero
  finite differences.
- The libjxl-rs oracle build already isolates competing NaN operands with
  source-scoped flags because the original's ordinary and ASan objects pick
  different NaN payloads: see the comments on `chroma_ycbcr_stage.cc`,
  `dequant.cc`, `dct.cc`, `encoder_transforms_unit.cc`, `gaborish.cc` and
  `skcms_concat.cc` in
  [`codecs/libjxl-rs/tools/oracle/CMakeLists.txt`](../../codecs/libjxl-rs/tools/oracle/CMakeLists.txt)
  ("Native x86 ordinary/ASan objects choose different competing NaN payloads").
- Highway's own test harness treats all NaNs as equal:
  `hwy/tests/test_util.h`, `detail::ComputeUlpDelta`, "Consider 'equal' if
  both are NaN" (verified in the Highway checkout of the pinned libjxl
  `a16d32d0`, Highway `457c8917`).
- libjxl's encoder API forbids NaN and infinite input samples:
  `lib/include/jxl/encode.h`, documentation of `JxlEncoderAddImageFrame`
  (floating-point samples "are however not allowed to be NaN or
  +-infinity"; verified at `a16d32d0`).

### 4. SIMD completeness and scope (D5, D6)

- **Completeness:** a port is not "1:1" for v1.0 until it has the original's
  SIMD paths for the levels in scope, each proven SIMD-against-SIMD under
  section 2. A scalar-only port meets at most half of the gate.
- **Scope for v1.0:** NEON on AArch64; AVX2 on x86-64, including the lower x86
  levels (SSE2, SSSE3, SSE4.1) that the original itself selects for a kernel
  on an AVX2 machine when that kernel has no AVX2 variant.
- **Out of scope and off by default:** AVX-512, SVE and SVE2, NEON DotProd
  and I8MM, and anything higher are post-v1.0. They are not ported, and in
  v1.0 they are **not executed by default**. Existing AVX-512 paths (the
  engine kernels' `Avx512` level, the translated skcms SKX engine in
  libjxl-rs, any AVX-512 branch in the AOM dispatch) are selectable only
  through an explicit opt-in. A run that opts in is outside the gate and must
  say so.
- Kernels that the original implements only in assembly, and x86 kernels
  whose results are vendor-specific, follow
  [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md)
  (owner decision of 2026-11-15 on O4).

### 5. Validation regime (D7)

- **Binding proof, local first:** AArch64 natively and x86-64 under Rosetta 2
  on the development Mac. x86-64 builds use `--target x86_64-apple-darwin`; C
  and C++ originals are built with `-arch x86_64`
  (`-DCMAKE_OSX_ARCHITECTURES=x86_64`). Rosetta executes AVX2, FMA, BMI2 and
  F16C, but run-time detection reports them only when
  `ROSETTA_ADVERTISE_AVX=1` is set in the process environment (measured on the
  development host, macOS 27.0.1). AVX-512 is not available under Rosetta,
  which matches section 4.
- **Cross-check:** one hosted Linux x86-64 run per release candidate on free
  GitHub runners, with the original built natively on that runner. Every run
  prints the detected and the selected SIMD level of the port and of the
  original.
- **Benchmarks:** Blacksmith and other CI benchmark runs are for performance
  only; they are not part of the gate.
- **Consequence:** every port's original must be buildable as an x86-64
  oracle, scalar and SIMD, on the development Mac under Rosetta.

### 6. Performance is reported, not gated (D2, D3)

There is no performance gate for v1.0. Performance is reported per codec and
operation against the original, in three columns:

| Column | Original build | Threads | Port runs | Ratio shows |
| --- | --- | --- | --- | --- |
| (a) | scalar reference build | one | scalar path | the translation loss |
| (b) | release build with SIMD (v1.0 levels) | one | SIMD path | translation plus missing SIMD |
| (c) | release build with SIMD (v1.0 levels) | all | SIMD path, all threads | everything, including threading |

Columns (b) and (c) cap the original at the v1.0 SIMD levels, so that port
and original run comparable code; a run of the original at its full default
level may be added as labeled context. (This cap is a best-estimate reading of
D2 together with D6.)

Optimization works top-down: close (a) first, then (b), then (c). Results for
jxl-oxide, rav1e, Sharp or other third-party libraries are context only; they
are neither a gate nor a selection criterion for v1.0. The
[benchmark protocol](../benchmarks/M1-protocol.md) carries the reporting rule;
[ADR-0007](0007-correctness-and-performance-evidence.md) still governs how a
number is measured.

### 7. Defaults (D1)

Correctness before speed: the workspace codec ports are the default paths even
where they are slower than third-party libraries. The workspace JPEG XL
decoder stays the default reader. External backends stay selectable.

### 8. Release naming (D10)

Pre-releases from `main` are 0.x. The version "1.0" is reserved for the state
that meets this gate completely: all four ports, both targets, scalar and the
SIMD levels of section 4. Releases use Release Please as the organization's
other packages do (`sebastian-software/ferromark` ADR-0020 "standards release
blueprint", and `sebastian-software/ferrugo`). All packages share one
version (owner decision of 2026-11-15 on O3; configuration A of
[release automation](../release-automation.md)).

## Per-port state on 2026-11-13

What the repository recorded when this record was accepted, with its source
(the gate tooling before the work of the night of 2026-11-13; the
[second table](#gate-tooling-on-main-at-dbf7b095-2026-11-13) gives the state
after it, the [third](#state-on-main-on-2026-11-14) the state of
2026-11-14). "Not available yet" names
a missing capability, not a failed check. Ratios are port ÷ original, one
thread unless noted, column (b) unless noted; they are indicative and carry
the limits of their records.

| Port | Scalar gate | SIMD gate | x86-64 original under Rosetta | Recorded state |
| --- | --- | --- | --- | --- |
| libjpeg-turbo-rs | Oracle variant `release` | Oracle variant `simd` (NEON; AVX2, SSE2 on x86-64) | `tools/check.py --arch x86_64` | C and port checksums identical for every input on every host of the CI record; (b) 1.10–1.18× x86-64, 1.20–1.30× Linux ARM64 |
| libwebp-rs | Oracle variants `release`, `asan-ubsan`, `debug` (`WEBP_ENABLE_SIMD=OFF`) | `release-simd` exists as a local diagnostic only; the NEON rescaler export rows keep the C translation (#79, #80) | Not available yet: `tools/build_oracle.py` has no x86-64 cross build | Every port output identical to C on every host of the CI record (release builds with SIMD); (b) 1.05–2.83× depending on operation and host |
| libjxl-rs | C++ oracle with `HWY_COMPILE_ONLY_SCALAR` | Not available yet: no SIMD paths in the port (#96) and no SIMD oracle | Not available yet: `tools/check.py` has no x86-64 cross build, and the oracle's x86-64 rules are Linux-scoped | AArch64 (a): lossy decode 1.72–1.87×, JPEG reconstruction 1.67×, lossless decode 1.10×, lossy encode 2.22× ([analysis, section 11](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md#11-combined-state-after-r1-r2-and-r3-2026-11-12-late-evening)); x86-64 runs the pre-work render stages; (b) VarDCT decode 9–22×; lossy encode byte-identical to `cjxl` in the scalar oracle configuration on every CI host |
| libavif-rs (AOM) | Golden outputs from a `_c`-only C reference (`-ffp-contract=off`) | Not available yet: kernels are gated against the port's own `_c` path; no native SIMD build of the original serves as oracle | Not available yet: the generated native profiles cover macOS ARM64 and GNU Linux x86-64 only | Encode byte-identical to C libavif + libaom release (SIMD) on both CI x86-64 hosts, one thread and all cores; different on macOS ARM64 (libaom's Neon/dotprod/i8mm versus the port's C-equivalent Neon); (b) encode 1.33–1.51×, decode 1.36–1.51× |

Sources: [v1.0 CI performance record](../benchmarks/v1.0-ci-performance-2026-11-12.md),
sections 2 and 3 and the later re-checks; the
[JPEG XL performance analysis](../benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md);
each port's `tools/check.py`, `tools/build_oracle.py` and ADRs. The CI
record's "C" columns are release builds with SIMD, so its ratios are
column (b); column (a) exists only for JPEG XL on AArch64.

### Gate tooling on `main` at `dbf7b095` (2026-11-13)

After the work of the night of 2026-11-13. Commands and details are in the
[quality gate guide](../quality-gate.md). Results are the recorded ones with
their source; "never run" names a check that exists but has no result.

| Port | Scalar gate | SIMD gate | x86-64 original under Rosetta | Recorded result |
| --- | --- | --- | --- | --- |
| libjpeg-turbo-rs | Unchanged (oracle variant `release`) | Unchanged (oracle variant `simd`) | Unchanged (`tools/check.py --arch x86_64`) | No change recorded in #196 or #241 |
| libwebp-rs | `tools/check.py --oracle`: `--no-default-features` against `release` | `release-simd` is now part of `tools/check.py --oracle`, once per mode (`SIMD_MODES`: `neon`; `avx2`, `sse41`, `sse2`), with `LIBWEBP_SIMD_LEVEL` capping both sides; NEON rescaler export rows follow libwebp's NEON (#79, #80) | `tools/check.py --arch x86_64` with `tools/build_oracle.py --arch x86_64` | AArch64 2,331 passed, 0 failed; Rosetta 2, all modes and profiles, 4,488 passed, 0 failed on `54a0c458` (#196 status comments); hosted Linux `avx2` mode failed on the oracle side (#252) |
| libjxl-rs | Pinned scalar oracle plus the second scalar reference `oracle-o3` (D8; O1 working rule) | NEON and AVX2 oracle variants (`-DORACLE_HWY_TARGET`, `JXL_CPP_ORACLE_SIMD`) and `tests/simd_parity.rs`: 63 Highway operations and the Gaborish, EPF, YCbCr and chroma upsampling stages proven at row level; **the decoder does not select the SIMD stages yet** (#96, #231) | `tools/check.py --target x86_64-apple-darwin`; the cross-built scalar references contain only skcms's baseline engine (O9) | AArch64 1,027 passed, 0 failed; Rosetta 2 1,023 passed, 0 failed (#196 status comment); hosted Linux SIMD stage tests failed on the oracle side (#252) |
| libavif-rs (AOM) | Golden outputs of the `_c`-only C reference, and the `scalar` level against the generic build | `tools/build_aom.py --library-only --simd neon\|avx2` and `tools/check_aom_simd_original.py`; five still-image Neon kernel families dispatched (509 RTCD functions) | Generic and `--simd avx2` originals build for x86-64 under Rosetta 2; the port's x86 kernels build only for GNU Linux x86-64, not for `x86_64-apple-darwin` | AArch64 `neon` 82 of 82 and `scalar` 82 of 82 identical (port ADR 0056); x86-64 `sse4.1` and `scalar` 86 of 86 through an x86-64 Linux container (port ADR 0056); `avx2` never run; no workflow runs the check |

Sources: the [quality gate guide](../quality-gate.md); libwebp-rs
`tools/check.py`, `tools/build_oracle.py`, port ADR 0008 and
`docs/SIMD-STATUS.md`; libjxl-rs `tools/check.py`, port ADR 0002 and 0004 and
`docs/PORT_STATUS.md`; libavif-rs `tools/build_aom.py`,
`tools/check_aom_simd_original.py` and port ADR 0056, 0058 and 0059; the
status comments of 2026-11-13 on #196 and #241; #252 for the first hosted
Linux x86-64 run (nightly 43401560676). No port meets the full gate on both
targets.

### State on `main` on 2026-11-14

`main` at `5004f34e`. Commands and details are in the
[quality gate guide](../quality-gate.md); results are the recorded ones with
their source, not rerun for this table. A hosted run is named with its
branch where that branch is not `main`.

| Port | Scalar gate | SIMD gate | x86-64 evidence | Recorded result |
| --- | --- | --- | --- | --- |
| libjpeg-turbo-rs | Unchanged (oracle variant `release`) | Unchanged (oracle variant `simd`) | `tools/check.py --arch x86_64` under Rosetta 2; hosted nightly run 43578105956 on `main` (`7d663784`, 2026-11-14) passed the job `JPEG: SIMD oracle (ubuntu-latest)` | No commit under `codecs/libjpeg-turbo-rs` since `dbf7b095`; no new result recorded |
| libwebp-rs | Unchanged (`release`) | Unchanged (`release-simd` per mode); `tests/wp13_tools.rs` also compares the Rust-linked `cwebp` with libwebp's SIMD `cwebp` in every mode | Rosetta 2: scalar and three SIMD modes, three profiles, 4,488 passed, 0 failed on `54a0c458` (#89). Hosted Linux x86-64: run 43471093624 (branch `v1/webp` at `690ad251`, whose `codecs/libwebp-rs` tree equals `main`'s), `avx2`, `sse41` and `sse2` 1,131 tests each, 0 failed; the earlier crash was a probe defect (#252) | AArch64 `neon` 411 and scalar 368 per profile, 2,337 passed, 0 failed (`docs/SIMD-STATUS.md`). Every recorded run passes on both targets; the narrowed comparison domains wait for O7 |
| libjxl-rs | Unchanged (pinned scalar oracle plus `oracle-o3`); scalar-reference tests select the scalar decoder path themselves, and `tools/check.py` sets `JXL_DECODER_TARGET=scalar` | Decoder: a SIMD translation of every decode-path Highway export for NEON and AVX2; `tests/simd_public.rs` compares whole public decodes (352 inputs, two modes), `tests/simd_threads.rs` worker counts, `tests/simd_vardct_lanes8.rs` the 8-lane structure. The decoder selects NEON by default on AArch64 and the scalar path on x86-64. The encoder's Highway exports are unported (#231, #243). On `main` the AVX2 oracle is a static AVX2 build | Hosted run 43533597186 (branch `v1/jxl-merge7`, static oracle): four row tests differ in one word each. The dispatch-build oracle (see [Decided since 2026-11-13](#decided-since-2026-11-13)) is on branch `v1/jxl-avx2-merge` only; with it, run 43558043274 (branch `v1/jxl-avx2-fixes`, based on `718b32d6`, before the NEON default) passed every job. No passing hosted AVX2 run of `main` | NEON: all 352 inputs identical in both modes; 0, 1, 2 and 8 workers identical with the NEON original; 8-lane structure equal (port ADR 0004, port status). No whole-suite count of `main` is recorded. The encoder meets at most the scalar half |
| libavif-rs (AOM) | Unchanged | Unchanged tools; four more still-image kernels dispatched to Neon (`aom_sum_sse_2d_i16`, `av1_get_horver_correlation_full`, `av1_get_nz_map_contexts`, `av1_round_shift_array`), 513 RTCD functions on the arm64 overlay | Unchanged: x86-64 originals build under Rosetta 2, the port's x86 kernels only for GNU Linux x86-64; `avx2` never run; no workflow runs the check | AArch64 `neon` 82 of 82 and `scalar` 82 of 82 on the final tree (port ADR 0056 amendment of 2026-11-13); x86-64 unchanged (`sse4.1` and `scalar` 86 of 86 through the container route). The nightly `root / generate` failure (#204) is fixed on `main` (`63d77626`, port ADR 0054 amendment); no nightly has completed since |

Sources: the [quality gate guide](../quality-gate.md) and the
[status page](../status.md); libwebp-rs `docs/SIMD-STATUS.md` and
`tools/check.py`; libjxl-rs `tools/check.py`, `CONTRIBUTING.md`,
`docs/PORT_STATUS.md` and port ADR 0004, on `main` and, for the
dispatch-build oracle, on branch `v1/jxl-avx2-merge`; libavif-rs port ADR
0054 and 0056 amendments; #204, #252; the job logs of runs 43471093624 and
43558043274 (`gh run view --log`). No port meets the full gate on both
targets.

Performance (section 6, reported, not gated): the
[JPEG XL decode record of 2026-11-14](../benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md)
(one AArch64 machine) measured the decoder at `7152fb41` at 1.3–3.3× in (a),
1.3–2.9× in (b) and 3.2–15× in (c) on eight inputs; its addenda give (c)
10.5×, 12.6× and 1.6× on three inputs after the concurrent AC group decode,
and (b) 1.28× and 1.57× on two inputs after the render-pipeline changes.
For the decoder on AArch64 these supersede the JPEG XL decode ratios of the
first table. No x86-64 record of the current decoder exists; the x86-64
JPEG XL rows of the
[three-column record of 2026-11-13](../benchmarks/v1.0-three-column-2026-11-13.md)
describe `e4038a4a`.

## Consequences

Existing gates that must change (the mechanics are separate work; this
record states the target):

1. **SIMD gates that compare against the port's own C path.** libavif-rs
   ADR 0056/0058 admit a kernel if it equals the port's `_c` translation, and
   ADR 0059 gates owned code on golden outputs from a `_c`-only C reference;
   the AOM SIMD identity corpus (`tools/check_aom_simd_identity.py`) compares
   the port's SIMD with its own scalar path. libwebp-rs admits a SIMD function
   by a Rust-against-Rust test against its `_C` translation and runs the
   reference oracle scalar. Both need an oracle that is the original's SIMD
   build at the v1.0 level (section 1), on both targets. The scalar
   references stay as the scalar half of the gate.
2. **libwebp NEON rescaler (#79, #80).** The port must follow upstream's NEON
   export rows (`vqrdmulhq_s32` by `scale >> 1`) and match the original's NEON
   build, instead of keeping the C translation on AArch64.
3. **libaom kernels whose SIMD differs from their C.** Kernels excluded or
   treated specially because the original's SIMD is not bit-exact with its C
   (the `CONFIG_EXCLUDE_SIMD_MISMATCH` family in `av1_rtcd_defs.pl`: the
   float `av1_nn_predict`, `av1_nn_fast_softmax_16` and
   `av1_cnn_convolve_no_maxpool_padding_valid` specializations; the reordering
   high-bitdepth Hadamard kernels; #123) are in scope when they are on the
   still-image path (O2) and are proven against the original's SIMD build,
   not against `_c`.
4. **libaom NEON level.** The port's Neon kernels reproduce libaom's C
   results; the gate compares them with libaom built at the NEON level
   (dotprod and i8mm off). Whether they then match is not yet measured.
5. **libjxl-rs NaN payloads.** Port ADR 0002/0003's requirement that NaN
   payloads match the oracle, and the out-of-line helpers that exist only to
   pin NaN operand order, are replaced by section 3. Copy-only paths keep
   bit-exact NaNs. This removes the stated reason for keeping the row kernels
   off x86-64; re-enabling them needs the gate on x86-64, not new codegen
   evidence about NaN operand order.
6. **AVX-512 default.** The engine kernel dispatch (ADR-0020 decision 4: the
   highest implemented level by default), the skcms SKX engine and any AOM
   AVX-512 dispatch must default to at most AVX2, with an explicit opt-in.
   The CI record already shows resized engine output that depends on the
   x86-64 kernel level (AVX2 versus AVX-512).
7. **x86-64 oracles.** WebP, JPEG XL and AOM need originals that build as
   x86-64 oracles under Rosetta, scalar and SIMD.
8. **Release gates in status pages.** Quiet-host timing, peak RSS against
   third-party libraries, and "performance gap accepted pending owner
   confirmation" are no longer release gates; they become reported context.
   Correctness and conformance evidence (including x86-64 conformance) stays
   required.

What becomes easier: one definition of "done" per port and target; NaN
payload engineering stops blocking faster exact code. What becomes harder:
four originals must build as SIMD oracles on two targets, and the WebP and
AOM gates must be rebuilt around the original's SIMD build. A performance
regression no longer blocks a release; it is visible in the three columns.

## Alternatives considered

- **A tolerance gate (ULP or perceptual bound).** Rejected: it hides
  translation defects that bit identity exposes, and every original already
  has an exact scalar oracle.
- **Port SIMD against port scalar.** Rejected as the gate: it proves the
  port's SIMD equals the original's C, which is wrong wherever the original's
  SIMD differs from its C (libaom Neon on macOS ARM64, libwebp NEON rescaler).
- **Identical output across targets.** Rejected: the originals themselves do
  not guarantee it (target-specific contraction).
- **Bit-exact NaN payloads.** Rejected: the original changes payloads between
  its own builds, so no single original build is a stable reference for them.
- **A performance gate against third-party libraries or the original.**
  Rejected for v1.0 (D3); performance is reported in three columns.
- **Hosted CI as the binding proof.** Rejected: hosted x86-64 runners vary in
  CPU and may expose AVX-512 (CI record); the local regime fixes the level and
  is reproducible. One hosted run per release candidate remains.

## Validation

- The gate is falsified for a port and target by one input and setting in
  its declared scope where the port's output and the matching original
  build's output differ outside section 3. Report the input, setting,
  execution kind, detected and selected levels, and both builds' flags.
- The NaN rule is falsified if a finite, infinite or signed-zero word, or a
  copy-only NaN, differs while the comparison passes. Comparison code must be
  tested with such words.
- Existing evidence is listed per port above. No port meets the full gate on
  both targets today.

## Open questions (pending owner decision)

None of the questions listed here on 2026-11-13 is open any more: the owner
decided O1–O9 and the further items on 2026-11-15. Each question is kept in
short with its decision under
[Owner decisions of 2026-11-15](#owner-decisions-of-2026-11-15); its full
wording of 2026-11-13 is in Git history and in
[#250](https://github.com/sebastian-software/ferrisma/issues/250). Not
decided, and outside this record: the inline assembly already in the tree
(an open question of [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md)).

## Decided since 2026-11-13

- **libjxl-rs AVX2 reference build (2026-11-14).** The owner decided that
  the libjxl-rs SIMD reference at AVX2 is upstream's dynamic-dispatch
  release build capped at `HWY_AVX2` through `HWY_DISABLED_TARGETS`, as
  section 1 defines a SIMD reference build, and not a static build with
  every translation unit compiled for AVX2 and FMA. This applies section 1;
  it changes no decision. Source: libjxl-rs ADR 0004, "AVX2 reference:
  upstream's dispatch build", on branch `v1/jxl-avx2-merge` (commit
  `d68e9a6d`). On 2026-11-14 that branch is not merged into `main`, and
  #250 does not record the decision.

### Owner decisions of 2026-11-15

Source: the owner's answers of 2026-11-15 to the questions of #250. Each
item names the question, the decision, the reason where the owner gave one,
and what is not done yet.

- **O1. Inputs that contain NaN or infinity** (#246). The working rule of
  2026-11-13 is confirmed as the general rule: a word is compared only
  where the original's two scalar reference builds (the pinned oracle build
  and the same sources at upstream's normal optimization level) agree after
  treating all NaNs as equal; at the other positions the port must only not
  crash and not commit a memory error. Its applications no longer carry the
  label "pending owner confirmation" (libjxl-rs
  [ADR 0002](../../codecs/libjxl-rs/docs/adr/0002-differential-oracle.md),
  [quality gate guide](../quality-gate.md)), and since 2026-11-16 neither
  do the two code comments that still carried it
  (`codecs/libjxl-rs/tests/parity.rs`,
  `codecs/libjxl-rs/tools/oracle/CMakeLists.txt`).
- **O2. libaom's video-only code.** Kept; nothing is removed. The owner's
  reason: animated AVIF may be wanted later, and it needs exactly this code.
  Animated AVIF itself is not decided and not in scope. SIMD stays ported
  only for kernels on the still-image path; the video kernels run on the
  translated C path. This replaces the leaning towards removal recorded on
  2026-11-13.
- **O3. Versions.** One shared version for all packages, configuration A of
  [release automation](../release-automation.md). The owner's reason: the
  organization's other packages do the same. Configuration B stays possible
  later. Release Please is not activated.
- **O4. Assembly-only kernels and vendor-dependent instructions.** Decided
  as a principle: [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md)
  (no assembly in the project; acceleration only through Rust intrinsics;
  assembly-only kernels run scalar until rebuilt with intrinsics, the libaom
  ones measured first; `rcpps` stays as in the original). It replaces the
  working rules of 2026-11-13.
- **O5. Confirmations pending from 2026-11-12:**
  - Lossy distance 1.0 as the JPEG XL output default, patches off: decided
    (ADR-0039, amendment of 2026-11-15; #236).
  - The rendered-ICC channel table: confirmed. moxcms stays the CMS; the
    table is built from and checked against it
    ([ADR-0016](0016-working-color-space-and-transform-boundaries.md),
    [ADR-0023](0023-decode-side-work-and-scaling-policy.md)). A 10- and
    12-bit table is a post-1.0 idea, to be measured first.
  - AVIF with ICC beside `nclx`: decided, MIAF (ADR-0015).
  - The default AVIF quality: decided, `avifenc`'s defaults (ADR-0037). The
    default output depth was decided the same day (the output depth follows
    the input depth); it is recorded and built by separate work, not on
    `main` when this entry was written.
  - The stable toolchain policy (#235): the minimum Rust versions are in
    ADR-0040 (amendment of 2026-11-15); the static stages' Clippy is pinned
    (ADR-0027, amendment of 2026-11-15, on branch `v1/clippy-pin` when this
    entry was written).
  - The licensing policy for additions inside a port (#136): confirmed
    ([license audit](../license-audit.md)).
  - The JPEG port's package name (#207): all four ports get the `ferrisma-`
    prefix with the originals' names ([publishing](../publishing.md),
    section 1). Not renamed yet.
- **O6. JPEG.** `macro_rules!` macros for the two always-inline Neon helpers
  are accepted; the kernels keep safe row access, without an `unsafe`
  row-pointer boundary; ITU T.83 clause 7 is not required for 1.0, the
  measured values stay documented, and #72 stays open as optional work
  (libjpeg-turbo-rs [ADR 0007](../../codecs/libjpeg-turbo-rs/docs/adr/0007-neon-kernels-and-unsafe.md),
  amendment of 2026-11-15).
- **O7. WebP.** All four points are accepted, with two conditions: (a) a
  test domain may be narrowed only under the single exception of
  section 2 (the original itself is undefined on the excluded inputs, the
  reason is in the code, the scalar comparison keeps the full domain);
  (b) the full gate of three x86-64 levels by three profiles runs hosted on
  real x86-64 in the nightly, and locally under Rosetta 2 only what a
  change affects. Checked on 2026-11-15 by reading the workflows: the
  scheduled `nightly.yml` runs `webp-check.yml` through
  `nightly-family.yml`, whose `oracle` job on `ubuntu-latest` runs
  `tools/check.py --stage test --oracle-variant release-simd`, which runs
  `avx2`, `sse41` and `sse2` (`SIMD_MODES`) each in the dev, release and
  checked profiles (`PROFILES`); on `macos-latest` the same step runs
  `neon`. No run result was checked for this entry. libwebp-rs
  [ADR 0008](../../codecs/libwebp-rs/docs/adr/0008-simd-modules.md),
  amendment of 2026-11-15.
- **O8. AOM.** The translated AOM code is owned code: it is changed only
  through committed, re-runnable tools that have a check mode run in CI,
  never by hand, and the frozen manifests stay as provenance (libavif-rs
  [ADR 0059](../../codecs/libavif-rs/docs/adr/0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md),
  amendment of 2026-11-15; `AGENTS.md`). The statement in #250 that
  `av1_get_nz_map_contexts` and `av1_round_shift_array` stay on `_c` is
  outdated (both are dispatched since 2026-11-13); dispatching
  `aom_sse`/`aom_highbd_sse` globally, video call sites included, matches
  the original, which calls them through RTCD everywhere.
- **O9. JPEG XL on x86-64, local gate.** Decided: the Rosetta cross builds
  of the libjxl original get `-DCMAKE_SYSTEM_PROCESSOR=x86_64`, so they
  contain the skcms HSW engine like a native x86-64 build; the original's
  source stays untouched. Not implemented yet (libjxl-rs
  [ADR 0004](../../codecs/libjxl-rs/docs/adr/0004-simd-paths.md)).
- **Floating-point contraction of the libavif/libaom reference** (#203).
  The reference stays the original built with `-ffp-contract=off`, one
  reference for AArch64 and x86-64. The owner asked whether the
  contracting build would be faster; no measurable gain is expected, which
  was not measured (libavif-rs
  [ADR 0056](../../codecs/libavif-rs/docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md),
  [supported matrix](../supported-matrix.md)).
- **rav1e's AVX-512 paths.** Moot: rav1e left every build on 2026-11-15
  (ADR-0037).
- **Argon conformance vectors and dav1d's place** (#54). Bit identity with
  libaom is the AVIF reader's evidence for 1.0; Argon is later extra
  evidence, earmarked for animated AVIF. dav1d leaves the runtime and the
  published chain and may stay in tests as an independent cross-check; not
  done yet ([ADR-0031](0031-workspace-libavif-input-experiment.md),
  amendment of 2026-11-15). **Paused on 2026-11-16** by the owner: dav1d
  stays, and its assembly kernels are rebuilt with intrinsics under
  [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md) D2,
  gated against dav1d's own SIMD build at the same level; which decoder
  the engine finally uses is open (ADR-0031, amendment of 2026-11-16).

## Review when

The owner revises a decision of 2026-11-15; an original is updated; a port cannot
build its original as an x86-64 oracle under Rosetta; Rosetta or macOS changes
the AVX2 advertisement; a hosted release-candidate run disagrees with the local
result on x86-64; or post-v1.0 SIMD levels (AVX-512, SVE2, DotProd/I8MM) are
planned.

## Amendment log

- 2026-11-13 — Accepted as owner decision D1–D10 of the 2026-11-13
  interview; open questions O1–O5 recorded with the O1 working rule pending
  owner confirmation.
- 2026-11-13 — State refresh; decisions D1–D10 unchanged. Added the table
  "Gate tooling on `main` at `dbf7b095`": libwebp-rs runs `release-simd` per
  SIMD level as part of its gate and builds an x86-64 oracle; libjxl-rs has
  NEON and AVX2 oracle variants, an x86-64 cross build and row-level SIMD
  stages that the decoder does not select yet; libavif-rs builds Neon and
  AVX2 originals and compares the port with them, with the `avx2` level never
  run. The first hosted Linux x86-64 run (nightly 43401560676, #252) failed
  for libwebp-rs and libjxl-rs on the original's side, so the hosted
  cross-check has not passed for them. Open questions O6–O9 and three further
  items copied from #250.
- 2026-11-14 — State refresh; decisions D1–D10 unchanged. Added the table
  "State on `main` on 2026-11-14": libwebp-rs passed the hosted Linux
  x86-64 SIMD gate at `avx2`, `sse41` and `sse2` (run 43471093624, #252 was
  a probe defect); the libjxl-rs decoder has a SIMD translation of every
  decode-path Highway export, passes the public-decoder gate on 352 inputs
  against the NEON original and selects NEON by default on AArch64, while
  x86-64 stays scalar and the encoder's exports are unported; the AOM port
  dispatches 513 RTCD functions to Neon and the #204 nightly failure is
  fixed. Performance note from the JPEG XL decode record of 2026-11-14.
  Open questions: O4 records the `rcpps` working rule of libjxl-rs ADR 0004;
  O8 notes that its two `_c` kernels are now dispatched; new section
  "Decided since 2026-11-13" with the owner's AVX2 reference decision for
  libjxl-rs (on branch `v1/jxl-avx2-merge`, not yet on `main`).
- 2026-11-14 (later) — The libjxl-rs dispatch-build AVX2 oracle named in
  "Decided since 2026-11-13" and in the 2026-11-14 table is on `main` since
  `81d2adab`; where this record says "on branch `v1/jxl-avx2-merge`", read
  "on `main`". Hosted run 43599382021 of that tree passed the four
  ordinary-oracle test shards on AVX2.
- 2026-11-15 — The owner decided O1–O9 and the further items of #250; new
  section "Owner decisions of 2026-11-15". O1's working rule becomes the
  general rule; O2 keeps libaom's video code; O3 one shared version; O4
  becomes [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md);
  O5's confirmations are given; O6, O7 (with the single exception to "never
  narrow a test domain", now in section 2) and O8 are accepted; O9, #203 and
  #54 are decided; rav1e's question is moot. Sections 3, 4 and 8 point to
  the decisions instead of the open questions, and the open-questions
  section keeps only a pointer. Decisions D1–D10 unchanged.
- 2026-11-16 — Section 2 gains one named exception to "expected values come
  from the original" (owner decision of 2026-11-16): an observation that
  depends on an order the C++ standard library leaves unspecified is made
  uniform in the oracle adapter, never in the port. Found through the
  hosted Linux run 43779456524, where `std::map`'s destruction order under
  libc++ 21 differed from libc++ 22 on macOS. Decisions D1–D10 unchanged.
- 2026-11-18 — Owner decision on how ports are written: sustainability over
  1:1 fidelity. Code that steers or outwits the compiler to match the
  original's instruction stream (optimizer barriers such as `black_box`,
  hand-written zeroing to avoid `memset`, splits for stack-frame shape,
  `unreachable_unchecked` on jump tables, exposed-provenance pointer steps,
  reads past an array that only a layout test justifies) is removed, and
  per-call instruction ratios against the original (such as the informal
  1.3x of libavif-rs ADR 0062) are measurements, not targets that justify
  such code. Section 2's "largely mechanical 1:1 port" is replaced
  accordingly. The quality gate itself (bit identity per sections 2-4) is
  unchanged. Context: an inventory of such code (it sits almost entirely in
  dav1d's rebuilt SIMD kernels) and a CDEF pilot that rewrote one family
  from the algorithm, in three variants, all bit-identical, 7-20x smaller,
  at +1 to +5 % instructions per whole decode; the owner chose the
  `fearless_simd` variant (one body for NEON and AVX2) for the next pilot.
- 2026-11-18 — Owner decision on the reference for kernels written from the
  algorithm (the idiomatic rewrite of the entry above). On input the
  format's specification allows, such a kernel stays bit-identical to the
  original's SIMD at the same level, as section 4 requires, and the
  existing gates check that. On input outside the specification, where the
  original's SIMD and its own C disagree and the format defines no result,
  the reference is the original's C code: the rewritten kernel matches the
  C, not the overflow or saturation behavior of the original's assembly.
  Found in the dav1d inverse-transform pilot (`v1/dav1d-itx-pilot`):
  coefficients over the full int16 range, which no conformant stream
  produces, made the original's 8 bpc SIMD and C differ in 11,025 (NEON) and
  11,327 (AVX2) of 21,000 calls; the pilot matched the C in every call and
  the original's SIMD wherever the input was conformant. A port ADR that
  adopts a rewritten kernel names the input range on which SIMD identity is
  gated and the test that compares against the C outside it. Memory safety
  on any input is unaffected.
