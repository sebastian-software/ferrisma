# Dispatch hand-translated x86 SIMD on GNU Linux

> **2026-11-13: superseded in part by engine [ADR-0041](../../../../docs/adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md).** The reference for the x86 kernels becomes libaom's own SIMD build capped at AVX2 (`ENABLE_AVX512=0`; SSE2, SSSE3 and SSE4.1 kernels included where libaom selects them on an AVX2 machine), not the translated `_c` function. Kernels removed under `CONFIG_EXCLUDE_SIMD_MISMATCH` or kept on `_c` because libaom's SIMD differs from its C are in scope when they are on the still-image path (ADR-0041 O2) and are proven against libaom's SIMD build (#123); NASM-only kernels are ADR-0041 open question O4. AVX-512 stays excluded and must not be executed by default. x86-64 evidence may now also come from macOS under Rosetta 2 with `ROSETTA_ADVERTISE_AVX=1` (ADR-0041 section 5), once the port builds for `x86_64-apple-darwin`; one hosted Linux x86-64 run per release candidate remains. Mechanics change in separate work.

This extends [ADR 0056](0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md)
to x86-64 GNU Linux. Everything ADR 0056 says about dispatch, kernels and
exactness applies; this ADR records only what differs on x86.

## Context

ADR 0056 added Neon kernels on macOS ARM64 and left GNU Linux on the generic
headers "until AVX2 kernels exist". The translated Linux encoder is therefore
entirely scalar. Ferrisma's consumer measurements show the Neon build of the
same encoder 21 to 40 % faster than `rav1e`; on x86-64 that gap does not exist
yet, and x86 servers are where Sharp-style pipelines usually run.

C2Rust 0.21.0 cannot translate the x86 kernels mechanically. Translating
`aom_dsp/x86/sad_avx2.c` succeeds but silently drops every vector statement
(the SAD loop body is gone), and `highbd_inv_txfm_avx2.c` aborts on calls with
vector arguments. The x86 files are therefore translated by hand under the
same rule as the Neon files.

## Kernel set

The scalar code is the same on both architectures, so ADR 0056's profile still
selects the work: forward and inverse transforms, CDEF, quantization, block
error, subtraction and intra prediction. The x86 set is the x86 counterpart of
the Neon set, function by function:

- For each function, the highest upstream variant up to AVX2 is translated,
  plus every lower variant that it calls internally (the AVX2 transforms
  delegate several sizes to SSE4.1). Many functions have no AVX2 variant; their
  best one is SSE4.1 (`intra_edge_sse4.c`, `filterintra_sse4.c`, 13 forward
  transform sizes) or SSE2 (`highbd_intrapred_sse2.c`,
  `highbd_subtract_sse2.c`, `aom_highbd_quantize_b_64x64_adaptive`).
- Specializations implemented only in NASM files are not translated: hand
  translation keeps an intrinsic sequence, and assembly has none. This leaves
  the high-bitdepth DC and V predictors for 4x4, 8x8, 16x16 and 32x32
  (`highbd_intrapred_asm_sse2.asm`) on `_c`.
- AVX-512 is not included. libaom enables it only on CPUs with AVX512-DL, the
  standard runners do not guarantee such CPUs, and its only overlap with this
  set is the high-bitdepth forward transform. It needs its own measurement.
- High-bitdepth smooth and Paeth predictors have no x86 specialization and stay
  on `_c`.

## Dispatch

`tools/transpile_aom.py` generates the Linux dispatch headers with the original
`cmake/rtcd.pl --arch=x86_64 --require-c`, every family other than SSE2, SSE4.1
and AVX2 disabled, and the port overlay `crates/aom/rtcd/x86_64.pl`, which
withdraws every specialization not in the set above. With `--require-c` the
generator dispatches SSE2 through a pointer as well instead of assuming the
x86-64 baseline, which keeps every `_c` path selectable.

The generated `setup_rtcd_internal()` calls `x86_simd_caps()`, a static inline
function in `aom_ports/x86.h`, so it is translated into the RTCD units rather
than written by hand as `aom_arm_cpu_caps()` is. C2Rust's CPUID lowering has the
BX-register defect described in [ADR 0023](0023-preserve-the-x86-cpu-query-branch-with-simd-disabled.md);
the same narrow correction replaces only that lowering with
`core::arch::x86_64::__cpuid_count` for the same leaf and subleaf. The XGETBV
instruction, the AVX and AVX-512 guards, and the `AOM_SIMD_CAPS` and
`AOM_SIMD_CAPS_MASK` overrides keep their translated form.

## Kernels

Each original x86 file is translated into
`crates/aom/src/simd/<same path>.rs` (for example
`av1/encoder/x86/error_intrin_avx2.rs`), with the ADR 0056 rules for names,
helpers, comments, constants and intrinsic order, compiled only for
`target_arch = "x86_64"`.

The high-bitdepth inverse and forward transform files (about 17,000 lines of
C) were transliterated statement by statement with C-subset printers and
rewriting scripts written for the purpose, not with C2Rust, and then compared with the C
function by function: the same sequence of intrinsic and helper calls, and
the same identifiers, constants and operators in order, with the remaining
differences reviewed by hand. The printer is not part of the build or of the
reproduction contract; the committed Rust is reviewed and tested like any
other hand translation, and its file headers say how it was produced.
Signatures, tables and macros in those files were written by hand.

Upstream compiles each x86 file with the flag its suffix names (`-msse2`,
`-msse4.1`, `-mavx2`). The Rust crate is compiled for the x86-64 baseline, so
every translated function carries `#[target_feature(enable = "...")]` for the
flag of the file it comes from. Helpers translated from shared headers carry
the smallest feature that their own intrinsics need; Rust inlines them into
callers with a superset of features, as the C compiler does. Functions are never
called without a feature check: only RTCD after `x86_simd_caps()` and tests
after `is_x86_feature_detected!` reach them.

## Evidence

The development machine is ARM64 and runs no x86 code, and no emulation is
used. x86 evidence therefore comes from native x86-64 CI runners, consistent
with [ADR 0055](0055-gate-pull-requests-on-linux-ci-and-local-macos-checks.md):

1. Locally, `cargo check --target x86_64-unknown-linux-gnu` for the crate and
   its SIMD tests.
2. On Linux CI, the supplementary transcriptions in `crates/aom/tests/simd` for
   each x86 instantiation of the original tests (SSE2, SSE4.1 and AVX2
   parameter lists), compared exactly against `_c` as in ADR 0056.
3. On Linux CI, byte-identical AVIF output for the identity corpus with
   detection enabled, with `AOM_SIMD_CAPS_MASK=0`, and with SSE4.1 as the
   highest family, so that the SSE4.1 and SSE2 paths that an AVX2 machine would
   not select are covered end to end.

## Consequences

The Linux profiles are regenerated by `aom-translate-linux` with dispatch
headers and imported with their `rtcd_dispatch` record; ABI, linkage and
ownership inventories are recorded again. Because a translation run takes about
three hours, the kernel set is completed before the Linux profiles are
regenerated, and later changes to the set regenerate them again. The macOS
profiles and the portable C reference are unchanged.

## Amendment: local x86-64 evidence (2026-11-13)

The statement under Evidence that no emulation is used no longer holds for the
SSE4.1 and scalar paths. Owner decision D7 makes local validation binding.
The port's GNU Linux x86-64 build now runs on the ARM64 development host in an
x86-64 container under Rosetta 2, and it is compared with the original's
x86-64 builds (`check_aom_simd_original.py --x86-64`; see the ADR 0056
amendment of 2026-11-13). Rosetta for Linux does not execute AVX, so the AVX2
kernels still need native x86-64 runners. `x86_64-apple-darwin` stays
excluded because both translation profiles depend on their platform: the macOS
profile carries Arm RTCD, and the Linux profile uses glibc symbols and layouts.

## Amendment: the AVX2 level runs locally under QEMU (2026-11-14)

The amendment above is out of date on this host: Colima's VM is started with
`rosetta: false` and `binfmt: true`, so `docker run --platform linux/amd64`
runs x86-64 binaries under the VM's own `qemu-x86_64` 7.0, which has no AVX.
Reconfiguring Colima is not part of this work. The AVX2 level instead runs
in an AArch64 Debian trixie container (`tools/qemu_x86_64/Dockerfile`)
that runs natively in the VM and contains Debian's qemu-user 10.0 and its
x86-64 cross GCC 14.2. Every x86-64 binary is started explicitly as
`qemu-x86_64 -cpu max -L /usr/x86_64-linux-gnu`. A probe there reports
SSE4.1, AVX, AVX2, FMA and BMI2, XCR0 `0x21f`, and no AVX-512.

The route (`check_aom_simd_original.py --qemu`, run inside the container):

- the originals: `build_aom.py --library-only --simd avx2` and
  `--library-only --x86-64`, run inside the container, build GNU Linux
  x86-64 references with the cross GCC (`build/aom-native-linux-avx2-x86_64-release`,
  `config_sha256` `1b033f4f…`, 330 units of which 20 NASM, and
  `build/aom-native-linux-x86_64-release`, `989d9a59…`, 166 units); C libavif
  and `tools/encode_still.c`/`decode_still.c` are built with the same compiler;
- the port: host rustc for `x86_64-unknown-linux-gnu`, linked by the
  container's cross GCC through `tools/docker_x86_64_linux_cc.sh`
  (`AOM_LINUX_X86_64_PLATFORM=linux/arm64`,
  `AOM_LINUX_X86_64_CC=x86_64-linux-gnu-gcc`);
- both sides run under the same QEMU and use the same glibc 2.41 libm, so all
  three levels (`avx2`, `sse4.1`, `scalar`) compare port and original in one
  environment.

Limits: QEMU is an emulator. Unlike Rosetta 2 it does raise the fault of a
misaligned aligned move (`movdqa`, `vmovdqa`, checked with a probe), but it
is not x86-64 hardware, so one hosted Linux x86-64 run per release candidate
remains (ADR-0041), and no timing taken under it means anything for x86-64.
The full three-level gate took 36 minutes on the loaded host.

**First AVX2 result (2026-11-14, `main` at `776747fd`, the 104 RTCD functions
of this ADR):** `avx2` 82 of 82, `sse4.1` 82 of 82 and `scalar` 82 of 82 cases
identical to the original in AVIF bytes, decoded planes and RGB; the port
reported `simd(avx2)`, `simd(sse4.1)` and `c` for the three levels.

The x86 profiles now also dispatch SSSE3 (`tools/transpile_aom.py` no longer
passes `--disable-ssse3`, and `rtcd/x86_64.pl` withdraws every SSSE3
specialization the port does not implement); with the unchanged overlay the
regenerated headers were byte-identical. Since the freeze (ADR 0059),
`tools/enable_aom_dispatch.py --linux NAME_EXT...` makes the x86 dispatch
change the way the Arm tool does (ADR 0056, amendment of 2026-11-13): overlay,
dispatch headers, the owned RTCD and caller units of both Linux profiles,
and their recorded hashes.

### Still-image families on x86 (2026-11-14)

The x86 counterparts of the 2026-11-13 Neon families follow the route of
that amendment: each original intrinsics file is translated to
`src/simd/<same path>.rs`, dispatched with `tools/enable_aom_dispatch.py
--linux` (working decision O8, 2026-11-13, delegated, revisable by the
owner), and committed only when `check_aom_simd_original.py --qemu` stays
identical at `avx2`, `sse4.1` (where the original selects the SSSE3 or SSE2
variant) and `scalar`. The files are transliterated with a typing-aware
C-subset printer as the high-bitdepth transforms were (`tools/x86_c2r/c2r.py`,
pycparser based, not part of the build; `tools/x86_c2r/assemble.py --check`
regenerates every printed file from the original C, `configs/`, `preludes/`
and the hand-written `parts/`, and requires it to equal the committed file): C's integer conversions are explicit, immediate operands
become const generics, run-time shift counts use the count-register
intrinsics, scalar accesses through cast pointers are unaligned, and
DECLARE_ALIGNED arrays keep their alignment. For every function the
sequence of called intrinsics and helpers was compared with the C.
Kernels that exist only as NASM stay on `_c` (working decision O4).

| Family | Original files | RTCD functions (variants) | Gate (`avx2`/`sse4.1`/`scalar`) |
| --- | --- | --- | --- |
| 8-bit inverse transform | `av1/common/x86/av1_inv_txfm_ssse3.c` (+ `.h`), `av1_inv_txfm_avx2.c` (+ `.h`), the rest of `av1_txfm_sse2.h` | `av1_inv_txfm_add` (SSSE3, AVX2) | 82/82, 82/82, 82/82 |
| Loop filter | `aom_dsp/x86/loopfilter_sse2.c`, `loopfilter_avx2.c`, `highbd_loopfilter_sse2.c`, `highbd_loopfilter_avx2.c`, the rest of `lpf_common_sse2.h` | 40 (40 SSE2, 4 8-bit quad and 6 high-bitdepth dual AVX2) | 82/82, 82/82, 82/82 |
| 8-bit intra prediction | `aom_dsp/x86/intrapred_sse2.c`, `intrapred_ssse3.c`, `intrapred_sse4.c`, the 8-bit part of `intrapred_avx2.c`, `intrapred_x86.h`, `intrapred_utils.h` | 175 (90 SSE2, 76 SSSE3, 3 SSE4.1, 44 AVX2 specializations) | 82/82, 82/82, 82/82 |
| 8-bit quantization | `aom_dsp/x86/quantize_sse2.c`, `quantize_ssse3.c`, `quantize_avx2.c`, the rest of `quantize_x86.h`, `av1/encoder/x86/av1_quantize_sse2.c`, `av1_quantize_avx2.c` | 7 (11 specializations; the NASM-only SSSE3 `aom_quantize_b` and `_32x32` stay withdrawn) | 82/82, 82/82, 82/82 |

Kernel tests: `tests/simd/lowbd_inv_txfm.rs` now also transcribes the SSSE3
and AVX2 instantiations of AV1LbdInvTxfm2d (`match`, `gt_int16`) and checks
`av1_inv_txfm_add_ssse3`/`_avx2` against `av1_inv_txfm_add_c`;
`tests/simd/loopfilter.rs` transcribes the SSE2 and AVX2 parameter lists of
`lpf_test.cc` (kLoop8Test6, kLoop8Test9, kHbdLoop8Test6, kHbdLoop8Test9,
kLoop8Test6Avx2, kHbdLoop8Test9Avx2). The SSE2 quad vertical filters call
`aom_lpf_horizontal_*_quad` through RTCD, as upstream does, so the AVX2 level
runs the AVX2 horizontal kernel there; the test sets RTCD up with
`aom_dsp_rtcd()` as the original test main does. Under the QEMU container
all 91 tests of `--test simd` pass for x86-64. `lowbd_intrapred.rs` and
`dr_prediction.rs` add the SSE2, SSSE3 and AVX2 instantiations of
LowbdIntraPredTest and the SSE4_1 and AVX2 ones of LowbdDrPredTest; the 24
SSE2 entries whose kernels exist only as NASM (DC, DC_TOP, DC_LEFT, DC_128, V,
H at 4x4, 8x8, 16x16, 32x32) are not transcribed and stay on `_c` (O4).
With them, 98 of 98 tests pass under QEMU. `quantize.rs` adds the 8-bit x86
tuples of `quantize_func_test.cc` (kQParamArraySSE2, kLPQParamArraySSE2, the
SSSE3 `aom_quantize_b_64x64`, kLPQParamArrayAvx2, kQParamArrayAvx2); 119 of
119 tests pass, in the release and the `checked` profile.

Since 2026-11-15 every family is verified before its push with all four
translation profiles compiled (`cargo check -p libavif-aom --lib --tests` for
GNU Linux x86-64 and macOS ARM64, dev and release), `--test simd` under QEMU
in the release and the `checked` profile, both gates, the PR stage,
`check_translation.py` and `tools/x86_c2r/assemble.py --check`. The dispatch
rewrite of `enable_aom_dispatch.py` leaves string, byte-string and char
literals and comments alone (the Linux debug profile keeps the C text of
`assert` as byte strings).
