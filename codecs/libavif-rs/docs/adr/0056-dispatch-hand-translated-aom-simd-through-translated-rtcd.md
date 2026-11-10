# Dispatch hand-translated AOM SIMD through translated RTCD headers

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
