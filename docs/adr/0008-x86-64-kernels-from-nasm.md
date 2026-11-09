# Rewrite the x86-64 kernels from NASM as SSE2 and AVX2 intrinsics

- Status: accepted
- Created: 2026-11-08
- Updated: 2026-11-09

## Context

[ADR 0004](0004-simd-after-the-portable-port.md) rewrites the x86-64 kernels
from upstream's NASM sources as `core::arch::x86_64` intrinsics and replaces
the CPUID code with runtime feature detection.
[ADR 0007](0007-neon-kernels-and-unsafe.md) settles the `simd` feature, the
module layout and the unsafe code for AArch64 and names work package 10 as a
reason to review it.

The decoder uses 16 files of `simd/x86_64`:

- `jdcolor` and `jdcolext` (YCbCr to RGB), `jdmerge` and `jdmrgext` (merged
  upsampling) and `jdsample` (plain and fancy upsampling), each for SSE2 and
  for AVX2;
- `jidctint` for SSE2 and AVX2, and `jidctfst`, `jidctflt` and `jidctred`
  (2x2 and 4x4) for SSE2;
- `jsimdcpu.asm`, which reports SSE2 on every CPU and AVX2 when CPUID leaf 7
  reports it and the operating system saves the YMM registers (`XGETBV`).

As in C, `jdcolor-*.asm` and `jdmerge-*.asm` include their `ext` file once
per pixel format. The include files in `simd/nasm` hold macros: the stack
slots `wk(i)`, the register names for each pixel layout (`jcolsamp.inc`), the
DCT constants (`jdct.inc`) and the calling convention. Each kernel file has a
constant table `jconst_*`, aligned to 32 bytes.

The asm is register-level code. It loads and stores rows with aligned moves
(`movdqa`, `vmovdqa`), which rely on `alloc_sarray` aligning rows to
`ALIGN_SIZE`, 32 bytes with `WITH_SIMD`. Like the Neon kernels, it reads past
the last column into that padding, and the color converters and merged
upsamplers end each row with partial stores, so that they write no byte past
`output_width`. The plain and fancy upsamplers write whole vectors into the
upsampler's own rows, which `alloc_sarray` pads.

On x86-64, `jsimd.c` prefers AVX2 to SSE2 for each kernel that has both. It
checks that the kernel's constant table is aligned (`IS_ALIGNED_SSE`,
`IS_ALIGNED_AVX`), and `JSIMD_FORCESSE2=1` limits the selection to SSE2.

Every x86-64 target of Rust enables SSE2, so SSE2 intrinsics are safe to call
anywhere in the crate. AVX2 needs a check at run time: a call into a
`#[target_feature(enable = "avx2")]` function from code without the feature
is unsafe. `std::is_x86_feature_detected!` checks the CPUID bits and the
operating system's register state. Rosetta 2 runs x86-64 code with AVX2 on an
ARM64 Mac, but reports AVX2 through CPUID only when the environment sets
`ROSETTA_ADVERTISE_AVX=1`.

## Decision

1. **The `simd` feature takes effect on x86-64 too.** `build.rs` sets
   `cfg(with_simd)` for `x86_64` as for `aarch64`. Decisions 1, 3, 4 and 5 of
   ADR 0007 apply unchanged:
   - `unsafe` code stays in a module of loads and stores, `src/x86.rs`, and
     in `src/jsimd.rs`;
   - the kernel modules declare `#![forbid(unsafe_code)]`;
   - a read past the last column converts a slice that ends at the end of the
     allocation;
   - `init_simd` reads the environment as upstream does, with
     `JSIMD_FORCESSE2` for x86-64.
2. **One module per asm file.** The kernel files become `src/jdcolor_sse2.rs`,
   `src/jdcolext_sse2.rs` and so on, named as `tools/units.json` maps them.
   - An `ext` file is a generic function with const parameters, which its
     main file instantiates per pixel format, as `jdcolext.rs` is.
   - `jsimdcpu.asm` is translated in `src/jsimdcpu.rs` next to the AArch64
     file, each under its `target_arch`.
   - The include files of `simd/nasm` are not modules: the kernels expand
     their macros where they use them.
   - The encoder's files are not translated (ADR 0001).
3. **The rewrite keeps the asm's order and names.**
   - Each `GLOBAL_FUNCTION` becomes one Rust function of the same name with
     the signature of its `*_simd` slot, marked
     `#[target_feature(enable = "sse2")]` or `#[target_feature(enable = "avx2")]`.
   - Each vector instruction becomes the intrinsic that compiles to it, in
     the asm's order.
   - A register is a local variable with the register's name (`xmm0`,
     `ymmA`), so that each Rust line reads against its asm line. A
     register-to-register move is an assignment, and the stack slots `wk(i)`
     are a local array `wk`.
   - An instruction whose result does not depend on its register's earlier
     value (`pcmpeqw x, x`, `pxor x, x`) becomes the constant intrinsic
     (`_mm_set1_epi16(-1)`, `_mm_setzero_si128()`).
   - A NASM macro of the file becomes a function of the same name. A `%if`
     on a configuration constant keeps the branch that the oracle's
     configuration takes.
   - The asm comments are kept.
   - The calling convention (`COLLECT_ARGS`, pushes and pops, `ENDBR64`), the
     pointer arithmetic and the loop counters become Rust control flow over
     indices. A label becomes a loop or block, with the label's name in a
     comment.
   - A constant table becomes a `static` with the asm's layout, its labels as
     fields and `#[repr(C, align(32))]`, as `ALIGNZ 32` aligns it. The
     `IS_ALIGNED_*` checks of `jsimd.c` test its address as upstream does.
4. **Unaligned memory access.** `src/x86.rs` holds the loads and stores, named
   after their intrinsics and taking references to arrays of exactly the
   bytes they touch, as `src/neon.rs` does. Aligned and unaligned moves of
   the asm both become the unaligned intrinsics (`_mm_loadu_si128`,
   `_mm256_storeu_si256`), because a row slice carries no alignment. They
   move the same values. On a misaligned address, the asm's aligned move
   would fault, but the port's doesn't, and there is no fault to reproduce.
   The non-temporal stores (`movntdq`, `vmovntdq`) become the same
   unaligned stores, so the asm's test of the output address that chooses
   between them falls away; the `sfence` after them stays. `vzeroupper` is
   left to the compiler, which emits it where AVX code returns to SSE code.
   A partial store, such as `movq`, `movd` or the byte stores at a row's
   tail, writes the bytes in safe code.

   Where the asm writes into its input rows, the port's kernel reads a local
   copy of the affected vector with the asm's change applied, because the
   kernel slots take input rows read-only (ADR 0005). The fancy upsamplers
   store a "dummy sample" (`inptr[w] = inptr[w-1]`) in the padding after the
   last column. The kernel's loads see the same samples, so the outputs are
   the same. Only that padding byte of the input row differs afterwards,
   and the decoder reads it only through these kernels, which set it again
   first.
5. **Detection.** On x86-64, `jpeg_simd_cpu_support` reports `JSIMD_SSE2`
   and `JSIMD_SSE`, and adds `JSIMD_AVX2` when
   `is_x86_feature_detected!("avx")` and `is_x86_feature_detected!("avx2")`
   both hold. Together they test the bits that `jsimdcpu.asm` tests: the
   AVX2 bit of leaf 7, the OSXSAVE and AVX bits of leaf 1, and the XMM and
   YMM state of `XGETBV`. `jsimd.c`'s entry points select a kernel only for
   a bit that this function reported, and the environment can only clear
   bits. An `unsafe` call of an AVX2 kernel in `src/jsimd.rs` is therefore
   sound.
6. **Verification on x86-64.** The `simd` oracle is built with NASM. On
   x86-64, `tools/check.py` runs the test suite against it three times: with
   AVX2, with `JSIMD_FORCESSE2=1` and with `JSIMD_FORCENONE=1`, in the port
   and the oracle alike. It sets `LIBJPEG_EXPECT_SIMD` to the instruction
   set that the dispatch test must find selected, so that a pass can't come
   from the C routines alone.
   - CI runs these checks on `ubuntu-latest`, an x86-64 host with AVX2.
   - On an ARM64 Mac, `tools/build_oracle.py --arch x86_64` builds the oracle
     for Rosetta 2. `tools/check.py --arch x86_64` runs the tests for
     `x86_64-apple-darwin` with `ROSETTA_ADVERTISE_AVX=1`.
   - Speed is measured only on native x86-64.

## Consequences

- On x86-64 the default build decodes with AVX2 where the CPU has it and with
  SSE2 elsewhere. As on AArch64, `default-features = false` gives the
  reference configuration without unsafe code.
- The float IDCT is upstream's SSE code. Its results equal the `simd`
  oracle's, which upstream checks with its `FLOATTEST8=sse` hashes, and
  differ from the reference oracle's C float IDCT.
- The kernel modules read against the asm line by line, but not against C.
  The bit-exactness argument rests on the instruction order and on the
  differential tests, not on a C original.
- The loads can't fold into an SSE2 instruction's memory operand when the
  compiler can't prove the alignment, which costs a move per load. The
  speed measurement shows whether that matters.

## Alternatives considered

- **Assemble and link upstream's NASM objects.** That is a C and assembler
  build dependency, not a Rust port, and it would bypass the unsafe policy of
  ADR 0003.
- **Inline assembly (`asm!`).** It would copy the instructions, but every
  kernel would be one `unsafe` block, with register allocation and memory
  operands that review cannot check against slices.
- **Translate the AArch64 C kernels' structure instead of the asm.** The x86
  arithmetic, such as `pmulhw` scaling and its rounding, differs from the
  Neon code. Only the asm's own instruction order gives its results.
- **Aligned loads behind a runtime alignment check.** It would reproduce the
  `movdqa` forms but add a branch per access. Upstream relies on
  `alloc_sarray` rather than checking, and the values are the same.

## Validation

The record is falsified if a decode or a kernel's output differs from the
`simd` oracle in any of the three modes, if a dispatch test passes with a
kernel set other than `LIBJPEG_EXPECT_SIMD`, if a kernel panics on a
conversion where the asm stays within its allocation, or if a load or store
wrapper touches memory outside its array.

## Review when

- Upstream adds or replaces x86-64 kernels, for example with AVX-512, or
  changes the decoder's `WITH_SIMD` branches.
- The speed measurement shows a kernel clearly slower than its asm.
- Rust makes the pointer loads and stores of `core::arch` safe.
