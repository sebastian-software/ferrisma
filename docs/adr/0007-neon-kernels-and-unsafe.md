# Translate the NEON kernels with unsafe code only for memory access and entry

- Status: proposed
- Created: 2026-11-08
- Updated: 2026-11-08

## Context

[ADR 0004](0004-simd-after-the-portable-port.md) ports upstream's SIMD
extensions after the portable decoder, and
[ADR 0003](0003-toolchain-and-unsafe-policy.md) requires an ADR that names the
modules with `unsafe` code, the operations and the local safety argument
before that work starts. Work package 9 translates the AArch64 kernels.

Upstream builds its SIMD extensions when `WITH_SIMD` is defined, which is the
default. The option does more than add kernels. In the decoder's C code it:

- raises `ALIGN_SIZE` in `jmemmgr.c` from 8 to 32, which pads every sample row
  that `alloc_sarray` allocates to a multiple of 64 bytes and changes the
  memory manager's accounting;
- makes `MULTIPLIER` (`jmorecfg.h`) and `DCTELEM` (`jdct.h`) 16-bit, which the
  C IDCTs and the multiplier tables of `jddctmgr.c` use;
- lets `jdapimin.c` mark the SIMD support as undetected and `jdcolor.c`,
  `jdsample.c`, `jdmerge.c` and `jddctmgr.c` ask `simd/jsimd.c` for a kernel
  before they fall back to their C routines.

`simd/jsimd.c` checks each kernel's preconditions, stores the kernel in a
`*_simd` function pointer of the module's object and calls it through a
wrapper. `init_simd` takes the CPU's support from `jpeg_simd_cpu_support`,
which on AArch64 always reports Neon, and lets the environment variable
`JSIMD_FORCENONE` override it. With `JSIMD_FORCENONE=1` a SIMD build runs its
C routines.

The decoder's Neon code is eight files in `simd/arm`: the YCbCr-to-RGB
converter, the merged upsampler and the plain and fancy upsamplers (each with
an `ext` file instantiated per pixel format), and the islow, ifast and reduced
(2×2 and 4×4) IDCTs. Their notes on memory access say that they read past the
last column of an input row, up to the padding that `alloc_sarray` adds, and
never write past `output_width`, because an output row may be the
application's unpadded buffer.

In Rust 1.95, a Neon value intrinsic of `core::arch::aarch64` is safe to call
inside a function marked `#[target_feature(enable = "neon")]`, and such
functions call each other without `unsafe`. Three operations stay unsafe:
the loads and stores, which take raw pointers; a call into a
`#[target_feature]` function from a function without the attribute; and a
call through an `unsafe fn` pointer, the only pointer type that a
`#[target_feature]` function coerces to.

## Decision

1. **The Cargo feature `simd` is `WITH_SIMD`.** It is on by default, as
   upstream's option is. It takes effect on targets for which the port has
   kernels, AArch64 from this package on and x86-64 from work package 10.
   Elsewhere `WITH_SIMD` stays undefined, as upstream's CMake falls back to a
   build without SIMD on a CPU it has no extensions for. `build.rs` makes
   that decision once and sets `cfg(with_simd)`, which the code tests; the
   constant `jconfig::WITH_SIMD` states it as a value. Every `WITH_SIMD`
   branch of the translated modules follows it: `ALIGN_SIZE`, `MULTIPLIER`,
   `DCTELEM`, `simd_support` and the `jsimd_set_*` calls. With
   `default-features = false` the crate is the reference configuration of
   [ADR 0002](0002-differential-oracle.md).
2. **One module per upstream file, as for the C code.** `src/jsimd.rs`
   translates `simd/jsimd.c` with the decoder's entry points and the
   declarations of `jsimd.h`, `jsimddct.h`, `jsimdint.h` and `jsimdconst.h`
   that they use. `src/jsimdcpu.rs` translates `simd/arm/aarch64/jsimdcpu.c`.
   The kernel files become `src/jdcolor_neon.rs`, `src/jdcolext_neon.rs`,
   `src/jdmerge_neon.rs`, `src/jdmrgext_neon.rs`, `src/jdsample_neon.rs`,
   `src/jidctint_neon.rs`, `src/jidctfst_neon.rs` and `src/jidctred_neon.rs`;
   an `ext` file that C includes once per pixel format is a generic function
   that its main file instantiates, as `jdcolext.rs` is. Each C kernel
   becomes one `#[target_feature(enable = "neon")]` function with the same
   intrinsics in the same order. The encoder's kernels, RGB565 and the AArch32 files are
   not translated, as the C code they serve is not
   ([ADR 0001](0001-mechanical-port.md)).
3. **`unsafe` code lives in two modules.**
   - `src/neon.rs`, a module of the port's own, holds one wrapper per load
     and store that the kernels use, with the intrinsic's name, for example
     `vld1q_u8`, `vst3q_u8` or `vst4_lane_u8`. Each wrapper takes a
     reference to an array of exactly the elements the instruction reads or
     writes, `&[u8; 16]` or `&mut [u8; 48]`. A whole-vector load or store
     passes the array's pointer to the intrinsic in one `unsafe` block.
     Safety: the reference covers every byte that the instruction touches,
     and these instructions need no alignment beyond the element type's,
     which the array has. A lane store writes the lane values in safe code;
     where upstream stores lanes through a byte row cast to `uint16_t *` or
     `uint64_t *`, the wrapper writes the bytes, because a Rust `u16` or `u64`
     pointer would need an alignment that the row does not have.
   - `src/jsimd.rs` calls the kernels: through the `*_simd` pointers, which
     the port adds to the objects as `unsafe fn` pointers, as `jpegint.h`
     declares them, and directly where upstream calls a kernel by name.
     Safety: `jsimd_set_*` stores only Neon kernels in the pointers, and the
     module is compiled only where `target_feature = "neon"` is enabled,
     which every AArch64 target of Rust is.

   The kernel modules contain no `unsafe` code and declare
   `#![forbid(unsafe_code)]`. A kernel takes its rows as slices and makes the
   arrays for the wrappers with checked conversions.
4. **Bounds checks where C relies on padding.** A kernel that reads past the
   last column, as upstream's do, converts a slice of the row's storage,
   which extends to the end of the allocation (ADR 0005, "Storage and row
   lists"). If a read ever reached past the allocation, the conversion would
   panic instead of reading outside it. Writes stay within `output_width`, so
   the application's rows need no padding.
5. **The environment as upstream reads it.** `init_simd` reads
   `JSIMD_FORCENONE`, and `JSIMD_NOHUFFENC` for the encoder's flag, with the
   port's `GETENV_S`, as the C code reads them for an AArch64 build. The tests
   use `JSIMD_FORCENONE=1` to check the C routines of the SIMD configuration
   against the same mode of the oracle.

## Consequences

- On AArch64 the default build decodes with Neon. A Rust program that wants
  no `unsafe` code in its build, or the reference configuration, disables the
  default features.
- The SIMD configuration accounts memory as upstream's SIMD build does:
  with `ALIGN_SIZE` at 32, `max_memory_to_use` and the out-of-memory errors
  match the `simd` oracle, not the reference oracle.
- `tools/check.py` builds the crate without default features for the
  `release` and `asan-ubsan` oracles and with them for the `simd` oracle,
  which it runs twice, with and without `JSIMD_FORCENONE=1`.
- The unsafe code is a set of small wrappers with one argument each, which
  review can check against the instruction's memory footprint, and one call
  per kernel entry.

## Alternatives considered

- **A feature off by default.** Every user of the crate would have to opt in
  to the speed that upstream's default build has, and Ferrixel would carry the
  difference. The reference configuration stays one flag away.
- **Unsafe kernels with raw pointers, as the C code has them.** The kernels
  would read like the C code, but every load and store would need its own
  safety argument, more than 300 of them. The wrappers concentrate the argument
  in one place per instruction.
- **Portable SIMD (`std::simd`) or a SIMD crate.** `std::simd` is unstable,
  and a crate is a dependency (ADR 0003). Neither maps one-to-one onto the
  upstream intrinsics, so the bit-exactness argument would be lost.
- **Calling the kernels directly instead of through `*_simd` pointers.** On
  AArch64 there is one kernel per slot, but on x86-64 `jsimd.c` chooses
  between SSE2 and AVX2 at run time. Keeping the pointers keeps the dispatch
  as upstream has it for work package 10.

## Validation

The whole differential test suite runs against the `simd` oracle, once with
Neon and once with `JSIMD_FORCENONE=1` in both the port and the oracle, and
the 30 in-scope `djpeg` bit tests produce upstream's MD5 sums with Neon. The
record is falsified if a Neon decode differs from the `simd` oracle, if a
kernel panics on a conversion where the C code reads within its allocation,
or if a load or store wrapper touches memory outside its array.

## Review when

- Work package 10 adds the x86-64 kernels and runtime feature detection.
- A Rust release makes pointer loads and stores of `core::arch` safe, or
  stabilizes `std::simd`.
- Upstream changes the layout of `simd/` or the decoder's `WITH_SIMD`
  branches.
