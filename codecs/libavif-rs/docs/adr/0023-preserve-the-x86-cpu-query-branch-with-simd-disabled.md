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
