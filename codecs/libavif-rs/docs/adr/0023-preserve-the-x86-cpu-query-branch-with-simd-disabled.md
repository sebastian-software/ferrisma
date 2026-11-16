# Preserve the x86 CPU-query branch with SIMD disabled

The original x86 CPU detector remains compiled when `WEBP_ENABLE_SIMD=OFF`.
Keep a separately generated x86 variant of `sharpyuv_cpu.c` and its included
`src/dsp/cpu.c` body, rather than carrying ARM's NULL callback initializer onto
Linux. The selected x86 function body has no OS-dependent branches; its integer
and pointer representations are checked on the supported Linux target.

C2Rust's CPUID assembly lowering uses a partial BX-register move and does not
preserve LLVM's reserved RBX register correctly. Replace only that lowering
with `core::arch::x86_64::__cpuid_count` for the same leaf and zero subleaf,
storing all four full-width register results. Retain the original XGETBV
instruction and its original OSXSAVE/AVX guard. All feature branches and slow
CPU model constants retain their translated source forms.

This restores the original CPU-query state; it does not enable SIMD conversion.
Compare the initial callback presence and all ten CPU-feature queries through
an original C driver, independently of the output corpus. Record the variant,
both source notices, its raw/corrected hashes, and its reproducible correction.

## Amendment: XGETBV through its intrinsic (2026-11-16)

Engine [ADR-0044](../../../../docs/adr/0044-no-assembly-acceleration-through-rust-intrinsics.md)
allows no inline assembly. The XGETBV of `xgetbv()`, which C2Rust lowered
from the original's raw opcode bytes (`.byte 0x0f, 0x01, 0xd0`) to an inline
assembly statement, is now read through `core::arch::x86_64::_xgetbv(0)`, the
call the original's own MSVC branch makes: the same instruction with the same
ECX, returning EDX:EAX as one value as the original function does. The
intrinsic needs the `xsave` target feature, so it is called through a
`#[target_feature(enable = "xsave")]` wrapper; the original OSXSAVE/AVX guard
(CPUID.1:ECX bits 27 and 28) around the only call is kept, and OSXSAVE is
set only on a CPU with XSAVE. The rewrite is `rewrite_xgetbv` in
`tools/stable_rust.py`, which `tools/transpile_sharpyuv.py` applies after its
corrections and whose `--check` mode CI runs; the same rewrite covers
libaom's six Linux RTCD units. The comparison through the original C driver
(`tools/check_sharpyuv_cpu.py`) is unchanged.
