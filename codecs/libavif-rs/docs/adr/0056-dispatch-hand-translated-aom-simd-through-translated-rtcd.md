# Dispatch hand-translated AOM SIMD through translated RTCD headers

> **2026-11-13: superseded in part by engine [ADR-0041](../../../../docs/adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md).** The exactness reference for a dispatched kernel changes from the translated `_c` function to libaom's own SIMD build at the v1.0 level on the same target: on AArch64, libaom built with NEON and without DotProd, I8MM, SVE and SVE2. A Neon kernel that equals `_c` but not libaom's NEON build does not meet the v1.0 gate; a kernel whose original NEON variant differs from `_c` (for example the reordering Hadamard kernels, #123) is ported to match the original's NEON variant when it is on the still-image path (ADR-0041 O2). Byte identity between detection on and `AOM_SIMD_CAPS_MASK=0` remains a useful check only where the original's SIMD equals its C. Mechanics change in separate work.

ADR 0013 kept SIMD out of the mechanical port. Profiling the portable Rust AOM
encoder on a 1600x2133 photo (all-intra, 10-bit 4:2:0, one thread) shows that
forward and inverse transforms, CDEF, quantization, and intra prediction take
about 58% of encode time at speed 6 and 49% at speed 7, all of them functions
that libaom specializes through its run-time CPU dispatch (RTCD). This ADR adds SIMD as a separate, measured layer that
keeps the scalar port as the reference and selectable at run time.

## Dispatch

libaom selects implementations in generated headers: `cmake/rtcd.pl` turns
`aom_dsp_rtcd_defs.pl`, `aom_scale_rtcd.pl`, and `av1_rtcd_defs.pl` into
`config/*_rtcd.h`. The generic configuration emits `#define fn fn_c`, so the
translation calls every `_c` function directly. A target with RTCD instead
declares `RTCD_EXTERN (*fn)(...)` and assigns the pointer in
`setup_rtcd_internal()` from the CPU flags.

The C reference build stays generic. For a platform with SIMD kernels,
`tools/transpile_aom.py` preprocesses all translation units with dispatch
headers that the original, unchanged `rtcd.pl` generates for the target
architecture (`--arch=arm64 --require-c`, every other extension family
disabled). A port overlay, `crates/aom/rtcd/<arch>.pl`, is evaluated after the
original definitions and withdraws every specialization that the port has not
translated. The generated header therefore has upstream's own shape: a
translated specialization becomes an RTCD pointer that defaults to `_c`, and
every other function remains a direct `_c` call. Upstream's explicit `_c` calls
stay direct because the choice is made during preprocessing, not by rewriting
the translation. The header digests, generator options, and overlay digest are
recorded as `rtcd_dispatch` in `crates/aom/TRANSLATION-<platform>-<profile>.json`
and the headers are kept under `crates/aom/reference/<platform>/<profile>/dispatch`.

The translated C keeps the generic configuration, including `HAVE_NEON 0`.
C code that upstream compiles only under `HAVE_NEON` and that kernels need,
such as the Q2.13 cosine tables of `av1_txfm.c`, is translated by hand next to
the kernels. Kernels whose C callers change behavior under `HAVE_*` (Wiener
statistics buffers in `pickrst.c`, the 3:4 scaler selection in `resize.h`) are
excluded until a configuration change is decided separately.

GNU Linux keeps the generic headers until AVX2 kernels exist; the x86
dispatcher depends on `x86_simd_caps()` and its CPUID lowering (ADR 0023).

## Kernels

C2Rust does not translate Neon intrinsics. Each original `*_neon.c` or helper
`*_neon.h` is translated by hand into `crates/aom/src/simd/<same path>.rs` with
`core::arch` intrinsics, keeping function names, helper structure, comments,
constants, and the intrinsic sequence; only the `AOM_ARCH_AARCH64` branches are
kept. Kernels are exported under their original names with the translation's
`extern "C-unwind"` ABI. AVX2 files will follow the same rule (`*_avx2.c` to
`src/simd/<path>/x86/*_avx2.rs`).

`aom_arm_cpu_caps()` is a hand translation of the Apple branch of
`aom_ports/aarch64_cpudetect.c`, including its environment overrides.
`AOM_SIMD_CAPS_MASK=0` selects every `_c` path for the process;
`AOM_SIMD_CAPS` forces a flag set, as upstream documents.

## Exactness

A specialization is dispatched only if its Neon function returns exactly what
`_c` returns for the same arguments. libaom's own tests are not always that
strict, so each case is checked:

- Kernels whose original tests accept a tolerance, such as the floating-point
  CNN and neural-network predictors, need their own decision.
- The high-bitdepth Hadamard kernels emit their coefficients in a different
  order; the original test sorts both outputs before comparing them. They are
  not dispatched.
- Several per-size `av1_inv_txfm2d_add_*_neon` entry points take `dest` as a
  `CONVERT_TO_BYTEPTR` value where `_c` takes `uint16_t *`. No libaom caller
  uses those dispatched names, so only `av1_highbd_inv_txfm_add` is
  dispatched; it calls them internally with the matching convention.

Each kernel file needs:

1. A supplementary transcription of the exact-match cases of the original test
   that instantiates its Neon functions, in `crates/aom/tests/simd`, with the
   original `ACMRandom` sequence, seeds, iteration counts, and parameters,
   comparing against the translated `_c` function.
2. Byte-identical AVIF output from the encoder with `AOM_SIMD_CAPS_MASK=0` and
   with detection enabled, for a recorded corpus, speeds, depths, and thread
   counts.

The original test executables are built with the generic configuration and
do not compile their Neon instantiations. Running them against Rust needs a
test build with an Arm configuration and a complete set of the kernels each
test file references; that remains open.

## Consequences

Changing the kernel set regenerates the macOS translation. Dispatched calls go
through a function pointer, so the `_c` path of a dispatched function is no
longer inlined into its callers. The Rust archive gains RTCD pointer globals
and `_neon` symbols, so linkage and ownership inventories must be recorded
again before the next acceptance claim. The portable C reference, its tests,
and the exact-output contract of ADR 0012 are unchanged.

## Amendment: compare with the original's own SIMD build (2026-11-13)

Owner decision D4 of 2026-11-13 asks for more than the exactness rule above.
The port's scalar path must be bit-identical to the original's scalar build,
and each SIMD path of the port to the original's SIMD build at the same
instruction-set level, per target. Until now every gate compared the port with
itself (`tests/simd`, `tools/check_aom_simd_identity.py`) or with the generic
C reference (`tools/check_aom_golden.py`). None ran the original's SIMD build.

### Reference builds

`tools/build_aom.py --library-only` gains three variants next to the generic
build. All keep `-ffp-contract=off` (ADR 0024). Each records its options,
`HAVE_*` settings, configuration digest, RTCD header digests and archive
digest in `reference-profile.json`.

| Option | Configuration | Units (assembly) | `config_sha256` |
| --- | --- | ---: | --- |
| none | `AOM_TARGET_CPU=generic` | 166 (0) | `c1acff87…` |
| `--simd neon` | `AOM_TARGET_CPU=arm64`; DotProd, I8MM, SVE, SVE2 off | 251 (0) | `44191d50…` |
| `--x86-64` | generic, `CMAKE_OSX_ARCHITECTURES=x86_64` (Rosetta 2) | 166 (0) | `c1acff87…` |
| `--simd avx2` | `AOM_TARGET_CPU=x86_64`, AVX-512 off; Rosetta 2 on macOS ARM64 | 330 (20) | `bde0e3b4…` |

The Neon build keeps `HAVE_ARM_CRC32 1`, which upstream enables with Neon on
arm64. CRC-32C is an exact integer function. The arm64 original requires
Neon: its RTCD headers define every Neon-specialized function as its `_neon`
version and have no run-time switch. `AOM_SIMD_CAPS_MASK=0` therefore cannot
select `_c` there, and the generic build is the scalar counterpart.
`--fp-contract on` builds the compiler's default contraction, for diagnosis
only.

### Gate

`tools/check_aom_simd_original.py` encodes every case twice: with the port,
and with the original (`tools/encode_still.c`, linked to the original C
libavif and the libaom build of the level). The AVIF bytes must be equal. It
then decodes the port's AVIF with the port (`examples/decode_planes.rs`) and
with the original (`tools/decode_still.c`), using the reader's settings. The
decoded planes and RGB must be equal.

| Host | Level | Port | Original |
| --- | --- | --- | --- |
| macOS ARM64 | `neon` | detection | `--simd neon` |
| macOS ARM64 | `scalar` | `AOM_SIMD_CAPS_MASK=0` | generic |
| x86-64 GNU Linux | `avx2` | detection | `--simd avx2` |
| x86-64 GNU Linux | `sse4.1` | `AOM_SIMD_CAPS` up to SSE4.2 | `--simd avx2`, same `AOM_SIMD_CAPS` |
| x86-64 GNU Linux | `scalar` | `AOM_SIMD_CAPS_MASK=0` | generic |

The corpus is the 62 golden cases plus 24 more. Eight use the libavif dog
photograph (djpeg, upright, 2x2 box downscale to 1512x2016) with the settings
of issue #203 and of the CI codec comparison: speed 6 quality 55 and speed 7
quality 56 at 10-bit 4:4:4 with one and four threads, and speed 6 quality 75.
The others add speeds 2, 3, 5, 7 and 8, quality 0, and alpha. `--image` adds
an image of your own with the #203 settings.

Results on macOS ARM64 (Apple M1 Ultra, macOS 27.0.1, Apple clang 21.0.0,
rustc 1.100.0-nightly 2026-09-06), with the engine's encoder input for the
#203 photograph added through `--image`: `neon` 86 of 86 and `scalar` 86 of
86 cases are identical in AVIF bytes, decoded planes and RGB. The 124
golden-case outputs also equal `crates/aom/golden/release.json`.

x86-64 on the same host (`--x86-64`, see below): the port's GNU Linux build
in an x86-64 Ubuntu 20.04 container (glibc 2.31) under Rosetta 2, and the
x86-64 originals under Rosetta 2 on macOS. `sse4.1` 86 of 86 and `scalar` 86
of 86 cases are identical, and the port reports `simd(sse4.1)` and `c`
respectively. The `avx2` level has not run locally.

### Issue #203

Neither a missing kernel nor a translation defect causes #203. Two separate
effects were mixed up.

1. **The C release build on AArch64 contracts libavif's RGB-to-YUV
   arithmetic.** The CI codec comparison builds libavif with the compiler's
   default flags. On AArch64, clang fuses `kr * R + kg * G + kb * B` in
   `avifImageRGBToYUV` (`src/reformat.c`) into multiply-add instructions. x86-64
   without `-mfma` cannot fuse, which is why x86-64 matched. On the CI
   photograph (4032x3024, 10-bit 4:4:4, quality 75, speed 6, one thread) the
   port wrote 1,759,074 bytes. So did C libavif with `-ffp-contract=off`
   linked to upstream's default libaom build, with DotProd, I8MM, SVE and
   contraction enabled. C libavif with default flags wrote 1,758,817 bytes
   with any libaom, the generic one included. Replacing only `reformat.c.o` in
   the noncontracting archive with the contracting object reproduces the
   difference. The libaom variant does not matter for these inputs.

   Over the 86-case gate corpus on AArch64, C libavif with default flags
   differs from the port in 18 cases, both with the generic libaom and with
   upstream's default libaom build. Noncontracting C libavif equals the port
   in 86 of 86 cases with the generic, Neon, contracting Neon, and upstream
   default (Neon, DotProd, I8MM, SVE, SVE2, default contraction) libaom
   builds. The contracting libaom builds contain 504 fused multiply-add
   instructions in 33 objects, among them still-path units such as
   `allintra_vis.c` (44), `rd.c` (21), `rdopt.c` (19), `tpl_model.c` (16),
   `cnn.c` (15), `pickrst.c` (11) and `tx_search.c` (7). None of them changes
   a corpus output. The noncontracting libavif has none. The contracting
   libavif has 95 `llvm.fmuladd` calls on 68 source lines in `reformat.c`
   (108 instructions; also `colrconvert.c` 147, `colr.c` 25, `gainmap.c` 18,
   `alpha.c` 3, `utils.c` 2). On the encode path they are lines 203
   (`v * rangeY + biasY` in `avifYUVColorSpaceInfoYToUNorm`) and 387 (`Y`),
   plus 372 and 373 for YCgCo. The other lines are YUV-to-RGB conversions on
   the read path.

   The reference configuration of this gate, the golden record and ADR 0024
   is noncontracting for both libaom and libavif (`-ffp-contract=off`). The
   port matches it on AArch64 and x86-64. To match a contracting AArch64 build
   instead, the port would need fused `mul_add` at those libavif sites on
   AArch64 only. AArch64 and x86-64 results would then differ, and the golden
   record would split. That is an owner decision, not a correction.
2. **The engine hands the encoder different pixels on each host.** For the dog
   photograph at 1600x2133 with rendered ICC, the engine's RGBA8 output
   differs by one code value in 29 of 10,238,400 samples between ARM64 Neon
   and x86-64 AVX2 kernels, and in 7 samples between Neon and the x86-64
   baseline (measured through lossless WebP output, the x86-64 engine under
   Rosetta 2). The same input bytes produce the same AVIF on every
   implementation measured: 227,163 bytes at speed 6 quality 55 with one
   thread, from the port with Neon and with `_c` on ARM64, the port's
   GNU Linux build with SSE4.1 and with `_c` (x86-64 container, glibc), and
   the original's generic, Neon, contracting Neon, AVX2 and SSE4.2 builds on
   macOS. The x86-64 engine's pixels give 226,996 bytes with the same port.
   The byte counts in #203 came from an older engine revision and cannot be
   reproduced exactly, but the mechanism is the encoder input.

Within the encoder, AOM output is therefore the same across hosts, SIMD
levels and C libraries for identical input. Cross-host identity of the engine
transform is a separate question for its kernels.

### Kernels whose original SIMD differs from `_c` (#123)

Every one of these stays on `_c` in the port on both targets. The original's
Neon or x86 build uses its SIMD version. On the gate corpus none of them
changes an output, but D4 requires the port's Neon and x86 levels to use the
same arithmetic as the original's.

| Kernel | Original Neon | Original x86 up to AVX2 | Difference | Still-image path |
| --- | --- | --- | --- | --- |
| `av1_nn_predict` | `ml_neon.c` | SSE3, AVX2 | float summation order; upstream excludes it with `CONFIG_EXCLUDE_SIMD_MISMATCH`, test tolerance 1e-3 | yes: ML partition and transform pruning |
| `av1_nn_fast_softmax_16` | none | SSE3 | float; excluded by `CONFIG_EXCLUDE_SIMD_MISMATCH` | yes: `prune_tx_2D` |
| `av1_cnn_convolve_no_maxpool_padding_valid` | `cnn_neon.c` | AVX2 (excluded by `CONFIG_EXCLUDE_SIMD_MISMATCH`) | float; test tolerance | yes: intra CNN partition pruning, speed 1 and up, not screen content |
| `av1_get_horver_correlation_full` | Neon | SSE4.1, AVX2 | float; test tolerance 1e-6 | yes: `prune_tx_2D`, transform search |
| `aom_hadamard_*`, `aom_hadamard_lp_*`, `aom_highbd_hadamard_*` | Neon | SSE2, AVX2 | coefficient order; the test sorts | no: only through `use_hadamard` (real-time and non-RD paths); all-intra calls `av1_quick_txfm` with `use_hadamard=0` |
| `av1_estimate_noise_from_single_plane` (and high bit depth), `av1_apply_temporal_filter` (and high bit depth) | Neon | AVX2 | float | no: temporal filtering, inter only |
| `aom_compute_flow_at_point`, `aom_compute_correlation`, `aom_compute_mean_stddev` | Neon (flow) | SSE4.1, AVX2 | float | no: global motion and optical flow, inter only |
| `aom_fft*_float`, `aom_ifft*_float` | none | SSE2, AVX2 | float | no: noise model, only with denoising |

`av1_interp_cubic_rate_dist_sse2` is also float but performs the C operations
in order (its source says so), so it is not on this list.

To close the D4 gap, the four kernels on the still-image path need translated
Neon and x86 versions that the gate then compares with the original's SIMD
build. A test against `_c` cannot decide them. That replaces the open question
in #123 with "translate them and compare with the original's SIMD", pending
owner confirmation.

### Local x86-64 validation

The `compile_error!` in `crates/aom/src/lib.rs` is a real dependency, not a
guard that can be lifted. The macOS profile is the macOS ARM64 translation: it
carries the Arm RTCD units, `aom_arm_cpu_caps()` and references to `_neon`
kernels, which are compiled only for AArch64. The Linux profile names glibc
symbols and layouts, such as 2,899 references to `__assert_fail` in the two
Linux profiles, glibc's
`pthread_mutex_t` internals (`__pthread_mutex_s`) and `stderr` (Darwin uses
`__stderrp`). `x86_64-apple-darwin` would need its own translation profile.

The port's GNU Linux x86-64 build runs locally instead. Host rustc builds it
(`--target x86_64-unknown-linux-gnu`, three to four minutes for the two
examples), `tools/docker_x86_64_linux_cc.sh` links it in an x86-64 Ubuntu
20.04 container (glibc 2.31, GCC 9.4) through Colima with Rosetta 2, and it
runs there. Rosetta for Linux reports neither AVX nor AVX2. Forcing them
through `AOM_SIMD_CAPS` stops the encoder with SIGILL, so the `avx2` level
needs a native x86-64 host. On macOS, Rosetta 2 reports AVX2 only with
`ROSETTA_ADVERTISE_AVX=1`. `check_aom_simd_original.py --x86-64 --docker-image
IMAGE` therefore compares the container port with the x86-64 originals built
for macOS and run under Rosetta 2, at the `sse4.1` and `scalar` levels. The
originals there use Darwin's libm, not glibc's. Building the original inside
that container failed: GCC 9.4 crashed with SIGSEGV under Rosetta 2 on
`palette.c` and `loopfilter.c`.
