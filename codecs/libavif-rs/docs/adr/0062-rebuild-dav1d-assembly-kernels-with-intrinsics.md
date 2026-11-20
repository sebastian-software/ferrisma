# Rebuild dav1d's assembly kernels with intrinsics

- **Status:** Accepted (infrastructure and the first itx slice on AArch64 NEON;
  amended 2026-11-20: CDEF and the inverse transforms are written from the
  algorithm on `fearless_simd`, see "Kernels written from the algorithm")
- **Date:** 2026-11-16
- **Source:** owner decision of 2026-11-16 (engine
  [ADR-0031](../../../../docs/adr/0031-workspace-libavif-input-experiment.md),
  amendment of that date) under engine
  [ADR-0044](../../../../docs/adr/0044-no-assembly-acceleration-through-rust-intrinsics.md)
  D2; sequencing delegated by the owner (coordinator, 2026-11-16)
- **Related:** [0025](0025-port-the-complete-dav1d-portable-configuration.md),
  [0029](0029-run-original-checkasm-bodies-across-c-and-rust.md),
  [0056](0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md),
  [0058](0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md),
  [0059](0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md),
  [0060](0060-build-the-translations-on-stable-rust.md); libjpeg-turbo-rs
  [ADR 0008](../../../libjpeg-turbo-rs/docs/adr/0008-x86-64-kernels-from-nasm.md).
  Number 0061 is used by the unmerged branch `v1/avif-drop-dav1d`, whose
  removal of dav1d is paused.

## Context

The engine reads AVIF through the translated libaom. On the hosted runners of
run 43789376239 that took 1.75 to 2.1 times as long as stock libavif with
dav1d, one thread. dav1d's speed is in its assembly: NEON in `src/arm/64/`,
SSSE3 to AVX-512 in `src/x86/`. This crate translates dav1d 1.5.4 with
`-Denable_asm=false` (ADR 0025), so it has none of that assembly; its
`*_dsp_init` functions assign C kernels and `dav1d_init_cpu` is empty. Engine
ADR-0044 excludes assembly in the project and linking the original's; a
kernel that exists only as assembly is rebuilt with `core::arch` intrinsics
(D2) and gated against the original's SIMD build at the same level.

A profile of the port's dav1d on still images (8/10-bit, 4:2:0/4:4:4, alpha,
lossless; Apple M1 Pro) puts the time in the inverse transforms (`itx`,
32–43 %), the entropy decoder `msac` (20–28 %, 70 % lossless), the loop
filter (12–18 %), CDEF (4–10 %) and intra prediction (3–4 %). Stills never
reach inter prediction (`mc`, warp, OBMC) except through intrabc or
superres. The assembly volume for those families is about 10,000 lines of
NEON and, at the AVX2 level, about 26,000 lines of x86 per bit depth pair.

## Decision

**Kernels.** Each assembly file of a family is rebuilt as one Rust module
below `crates/dav1d/src/simd/`: `arm64/<file>.rs` for `src/arm/64/<file>.S`,
`arm/<family>.rs` for the init header `src/arm/<family>.h` (the slot
assignments under the CPU flags). The original keeps its x86 headers and
assembly in one directory; here `x86/<family>.rs` is the header
`src/x86/<family>.h` and `x86_64/<file>.rs` rebuilds `src/x86/<file>.asm` in
its x86-64 System V form (amended 2026-11-17). One Rust
function per assembly entry point and per internal function or macro, named
as in the assembly; the vector registers are a local array indexed by
register number (`simd/arm64/util.rs`), so that each Rust line reads against
its assembly line, as libjpeg-turbo-rs ADR 0008 did for NASM. Instructions
become the intrinsics that compile to them; a 64-bit arrangement clears the
high half of its destination as the hardware does. Lane-wise instructions
are computed whole, which gives every lane the value of the assembly whatever
the issue order. Instruction scheduling, register allocation and the
assembly's private calling conventions between helpers (`blr x4`) are left to
the compiler; branches on the eob thresholds, coefficient clearing, the
order of loads and stores and the DC-only paths are kept, because they decide
the outputs and the coefficient buffer. No `asm!`; the kernels are not
`#[no_mangle]`, so the crate's exported symbols do not change.

**`unsafe` boundary, per family.** The translated caller passes the
original's arguments (destination block, stride, coefficient buffer, eob).
The family module states once which rows, columns and coefficient offsets its
loads and stores touch; they stay within the block the caller hands over.
Unaligned 32-bit lane accesses use `read_unaligned`/`write_unaligned`.

**Dispatch.** `tools/enable_dav1d_dispatch.py` (with `--apply`, `--rehash`,
`--check`; unit test `tests/ports/enable_dav1d_dispatch_test.py`; CI runs
`--check` next to `enable_aom_dispatch.py --check`) makes the original's
`HAVE_ASM` edits in the owned translated units of ADR 0059 and nothing else:
`dav1d_init_cpu` stores `simd::cpu::dav1d_get_cpu_flags_native()` (all four
profiles), and each entry of the committed list `crates/dav1d/dispatch.txt`
(`<family>_<bpc>bpc_<arm|x86>`) makes that family's init call the owned
mirror of the original's `<family>_dsp_init_<arch>` where the original calls
it (macOS profiles for `arm`, Linux profiles for `x86`). Each inserted
statement carries a marker comment; `--check` removes them, re-applies the
list and compares, and checks the digests. `tools/transpile_dav1d.py`
applies the same function after `tools/stable_rust.py`'s rewrites, so a
regeneration reproduces the edits. The function-pointer tables
(`Dav1dInvTxfmDSPContext`, …) keep their translated layout; the owned mirror
declares the same `#[repr(C)]` layout. `dav1d_set_cpu_flags_mask(0)` keeps
every C kernel selectable, which is the port's scalar path.

**msac, selected at compile time.** The original selects the entropy
decoder's assembly by renaming the seven calls of `src/msac.h` in every
unit that includes it (`src/arm/msac.h`); the translation calls the C
functions by name, the three symbol sizes as one C function. The entry
`msac_arm` (no bit depth) renames each call site in the translated
`decode`, `recon_tmpl` (both bit depths) and `msac` units to the owned
function `crate::simd::arm::msac::<macro name>` that the original's macro
names there; the symbol sizes follow `SYMBOL_ADAPT` in the tool (the
original's macros in source order, derived from `decode.c`, `recon_tmpl.c`
with its macro expansions and `msac.c` by the tool's unit test).
Definitions and declarations keep their C names; `--check` reverses the
renames by their path. Each owned function calls the rebuilt kernel, chosen
at compile time as in the original (amended 2026-11-16: a run-time check of
the CPU flags there cost two loads and a branch per symbol and kept both
paths in the callers; see the validation). The counterpart of the
original's portable build is the build configuration `--cfg
dav1d_portable` (`tools/build_dav1d_consumers.py --portable`, its own
target directory): there the owned functions call the C functions and
`dav1d_get_cpu_flags_native()` reports nothing, so every DSP slot keeps its
C kernel, as with `-Denable_asm=false`. `MsacContext` keeps the
portable layout (on AArch64 the original adds no field). The per-kernel gate
reaches the seven kernels without a DSP table: `check_dav1d_dsp.py --simd
neon` enables the test body's `HAVE_ASM` branch (two textual edits, listed
in the tool) and resolves the `_neon` names through test-only accessors, on
the original's side compiled into its oracle library, on the port's side
exported only with the crate feature `port-test-bridges` (a separate build
in `build/dav1d-bridges-target`; the product library's exported symbols do
not change). One deviation for memory safety: near the end of a tile
buffer the assembly loads the 8 bytes that end at `buf_end` and shifts out
those before `buf_pos`, which can reach before the buffer; the rebuild
loads only `buf_pos..buf_end`, which gives the same value. The feature `port-test-bridges` is test
tooling, not part of the crate's public contract: it adds those exported
accessors and nothing else (no kernel, dispatch or layout depends on it),
no dependent enables it, `check_dav1d_linkage.py` rejects a product library
that carries them, and the PR stage of `cargo xtask codecs --codec avif`
checks that the crate still compiles with it.

**msac on x86-64** (amended 2026-11-17). `src/x86/msac.h` renames six of
the seven calls to the SSE2 functions at compile time (SSE2 is part of
x86-64; `dav1d_set_cpu_flags_mask` does not reach them) and calls the
16-symbol function through a pointer in the context, which
`dav1d_msac_init` sets for every tile from the CPU flags: C, SSE2 under
`DAV1D_X86_CPU_FLAG_SSE2`, AVX2 under `DAV1D_X86_CPU_FLAG_AVX2`. The entry
`msac_x86` renames the call sites of the Linux profiles to
`crate::simd::x86::msac::<macro name>`, which call the rebuilt SSE2 kernels
directly. The translated `MsacContext` keeps the portable layout, which the
ABI gate compares and which is embedded in the tile state; it has no
`symbol_adapt16` member. The owned 16-symbol function therefore makes
`msac_init_x86`'s choice from `dav1d_get_cpu_flags()` at every call (two
loads, a test and a branch where the original loads the pointer and calls
through it); the flags do not change while a decoder runs, so the same
kernel runs as in the original. The per-kernel gate compiles the msac test
body with `HAVE_ASM` 1 on x86-64, so that the test's contexts have the
original's layout there (the original's `dav1d_msac_init` writes the
member); the port's functions use the members before it.
Decided 2026-11-17 (coordinator, on the open points of the first x86
kernels): the choice at every call and the portable context layout stay,
because the ABI gate compares that layout; and of the two functions the
pointer can hold only the AVX2 one is rebuilt. On a CPU without AVX2 the
owned 16-symbol function calls the translated C function, where the
original runs its SSE2 function (see "Levels"); the six functions that the
header selects at compile time are the rebuilt SSE2 kernels on every x86-64
CPU, as in the original. The module layout of "Kernels" (`x86/<family>.rs`
for the header, `x86_64/<file>.rs` for the assembly file) stays as well.

**CPU detection without assembly.** `simd/cpu.rs` mirrors
`src/x86/cpu.c` statement by statement with `__cpuid_count` and
`_xgetbv(0)` in place of `cpuid.asm` (the route libaom's RTCD took on
2026-11-16), including `SLOW_GATHER`. On AArch64 NEON is baseline; DotProd,
I8MM, SVE and SVE2 are above the v1.0 scope and not reported.

**Levels.** AArch64: NEON. x86-64: AVX2. On an AVX2 machine the original
selects an AVX2 kernel for every still-image slot that has one (the x86
init headers, preprocessed with assembly on), and SSE2 for msac's entry
points; the 12 bpc DCT_DCT transforms of 16x32 and larger have no x86
kernel and run C in the original as well. Those AVX2 and SSE2 kernels are
the x86-64 scope (engine ADR-0041 section 4: what the original selects on
an AVX2 machine). **Stated gap:** the kernels that the original selects
only on CPUs without AVX2 are out of scope for now: the SSSE3 and SSE4.1
kernels of the DSP families, the SSE2 kernels of DSP slots (WHT) and msac's
SSE2 16-symbol function. On such a CPU the port runs its C kernels and the
C 16-symbol function where the original runs those; its six msac
functions selected at compile time are the rebuilt SSE2 kernels there too.
On an AVX-512 (Ice Lake) machine the original selects
`avx512icl` kernels; the gate restricts it with
`dav1d_set_cpu_flags_mask(0x2f)` (SSE2 to AVX2, `SLOW_GATHER` passed
through), as libaom's oracle is built without AVX-512. The x86-64 gate has
the one level `avx2` beside scalar. (Amended 2026-11-17, twice: a first
form of this paragraph rebuilt msac's SSE2 16-symbol function and gated it
at a level of its own, `sse2`, with mask 0x1 in both gate tools; the
function and the level were removed the same day, validation below.)

**Plain Rust, no codegen steering** (amended 2026-11-18; owner principle
of 2026-11-18). No kernel contains code that outwits the compiler or
Rust's rules to mimic the original's assembly: no
`core::hint::black_box` on a value, an address or as a barrier; no
hand-written zeroing in place of a zero-initialized array; no
`#[inline(never)]` and no function split whose reason is the stack frame
or the instruction count; no `unreachable_unchecked` or
`NonZero*::new_unchecked`; no exposed provenance; no compile-time
resolution of a jump table for the instruction count; and no load
outside the memory the caller's pointer belongs to (a kernel that loads
around the intra edge runs on a local copy of the pixels the C kernel
reads, `crate::simd::copy_edge`). Straight, idiomatic Rust comes first;
a result that executes more instructions is accepted as long as the
gates show identical output, and one trick is not replaced by another.
Withdrawn with this: the rule "no `call` in an entry point" of the
validation tables below (the compiler may call `memset` or keep a helper
out of line), and the instruction-count reasons given for the deviations
that this amendment removes (they stay in the historical entries as
what was decided then). The instructions per call against the assembly
(`measure_dav1d_kernels.py`, the "within 1.3x" of the intra prediction
entries) are a measurement the validation reports, not a target that
justifies such code. The assembly-shaped structure of the kernels
("Kernels" above: register files, label functions, `SECTION_RODATA` byte
arrays, emulated frames and the wrapping pointer steps that come with
them) is not changed by this amendment; whether it stays is a separate
decision.

**Kernels written from the algorithm** (amended 2026-11-20; owner
decision of 2026-11-20). CDEF and the inverse transforms are no longer
rebuilt from the assembly: they are written from the C original
(`cdef_tmpl.c`; `itx_tmpl.c`, `itx_1d.c`) on the portable vector types of
the `fearless_simd` crate (1.2.0, Apache-2.0 OR MIT, no dependencies of its
own), one implementation per family for NEON on AArch64 and the library's
`Avx2` level on x86-64, in `libavif-dav1d-kernels` (`crates/dav1d-kernels`), installed through
`simd/cdef/` and `simd/itx/`. They replace the
1:1 ports of `cdef.S`, `cdef16.S`, `itx.S`, `itx16.S`, `cdef_avx2.asm`,
`cdef16_avx2.asm`, `itx_avx2.asm` and `itx16_avx2.asm` (removed with this
amendment, about 34,800 lines with 1,135 `unsafe`; the kernels written
from the algorithm are about 3,100 lines with 31, at the DSP-table
boundary and the level token), and the pilot switches (`--cfg dav1d_cdef_pilot`, `--cfg
dav1d_itx_pilot`, `DAV1D_CDEF_PILOT`, `DAV1D_ITX_PILOT`) and the pilot's
other CDEF variants (autovectorized, `core::arch` per architecture) are
gone: the crate has one SIMD implementation per job. The init mirrors
(`simd/{arm,x86}/{cdef,itx}.rs`) keep the original's selection under its
CPU flags statement by statement and fill exactly the slots where the
original assigns its NEON or AVX2 kernels (itx: one `Kernels::assign` call
per `assign_itx*_fn` statement of the header); the C kernels,
`dav1d_set_cpu_flags_mask(0)` and the scalar gate are unchanged. The
numerics follow engine ADR-0041 (amendment of 2026-11-18): a kernel written
from the algorithm gives the original's SIMD output on conformant input
and its C output on any input, which the unit tests check against the
translated C over the whole `int16_t` coefficient range (inverse
transforms) and checkasm's inputs (CDEF). `fearless_simd`'s `Avx2` level
needs x86-64-v3 (AVX2, BMI1/2, F16C, FMA, LZCNT, MOVBE, …), a little more
than dav1d's AVX2 flag (AVX2, BMI1/2 and the OS's YMM state): on a CPU with
the flag but without that level these two families keep the C kernels (and
`all_simd` 0), where the removed ports ran; no other family changes. The
"Kernels" paragraph above stays for the other families; whether they move
to the same form is a separate decision. The kernels are the crate
`libavif-dav1d-kernels` (`crates/dav1d-kernels`, BSD-2-Clause with dav1d's
license material, generated by `tools/generate_licenses.py`), which
`libavif-dav1d` installs from and which the workspace `Cargo.toml` compiles
with `opt-level = 1` and line tables in the `dev` profile, together with
`fearless_simd` (the reason is there; the standalone probe manifests of
`check_dav1d_abi.py`, `check_abi.py` and `check_root_profile.py` carry the
same): the library's generic code relies on inlining and optimization, and
at opt-level 0 the AVX2 inverse transforms needed stack frames of up to
2.4 MB, more than the 1 MB of dav1d's worker threads (a silent SIGSEGV in
the debug API check of run 44691276520). With the separate crate the
translated decoder stays at opt-level 0. The two passes of the inverse
transforms are shared by every block size with the same row length (first
pass) or column length (second pass): the other dimension, `rect2` and the
shift reach them as values. Generic over both, the crate compiled about
600 kernels with their own passes, and its compile time ran the hosted
macOS jobs past their timeouts (CI 44728440108, avif-dav1d 44728445301).
Shared, it compiles in 33 s in release and 29 to 38 s in dev
single-threaded (from 77 s and 205 s), for 1 to 2 % more instructions per
decode. The tests and gates of the kernels stay in `libavif-dav1d`,
against the translated C.

**Gate (engine ADR-0041, ADR-0044 D2).**

1. *Oracle.* `tools/build_dav1d.py --asm` builds the original from the same
   source with its assembly, `-ffp-contract=off`, `-Dtrim_dsp=false` (so the
   mask can select every level down to C), a static library and the
   original CLI (`build/dav1d-asm-<profile>`, `reference-profile.json` with
   the configuration, assembly-source and archive digests). With
   `--cross x86_64-linux-gnu` it is built in the `tools/qemu_x86_64`
   container and run under `qemu-x86_64 -cpu max` (no Rosetta). Since
   2026-11-17 the local x86-64 gates run instead in an x86-64 GNU Linux
   container that Rosetta executes (`tools/rosetta_x86_64`: Dockerfile and
   `run.sh`; a Colima VM with `--vz-rosetta`): the original, the port, the
   test bodies and the tools are built natively in it with the unchanged
   tools, as on the hosted Linux runner, and dav1d's own CPU detection
   finds AVX2 there.
2. *Per kernel.* `tools/check_dav1d_dsp.py --simd neon|avx2` runs all
   original checkasm bodies (ADR 0029) with the original's assembly pointers
   against the port's, both libraries detecting the CPU with their own
   `dav1d_init_cpu` and restricted to the same level: every block size, every
   transform type, eob sub-shapes, 8 and 10/12 bpc, padded destinations that
   catch writes outside the block, and the cleared coefficient buffer. Slots
   not yet rebuilt are compared through the port's C, which the original's
   assembly equals on the inputs the original tests feed. The scalar mode
   (no `--simd`) now pins the port to its C kernels explicitly.
3. *Whole images.* `tools/check_dav1d_simd_original.py --level neon|avx2|scalar`
   decodes the original test suite's MD5-verified streams (dav1d-test-data,
   Argon samples included) and the AV1 items of every tracked libavif test
   AVIF with the original (assembly at the level, or portable C for
   `scalar`) and with the original CLI relinked against the port (same
   `--cpumask`), and requires the same exit status and picture MD5, and the
   expected MD5 where the suite knows it. At `scalar` the port side is
   the portable build (`--cfg dav1d_portable`, also with `--cpumask 0`):
   the run-time mask cannot reach msac, which both sides select at compile
   time.

The SIMD comparison needs no limit on the input domain: it compares with
the assembly itself. (Upstream's `src/itx_1d.c` documents that SIMD inverse
transforms may differ from C on non-conformant coefficient overflows; that
is a difference between the original's C and its assembly, which the scalar
and SIMD comparisons each keep on their own side.)

## Consequences

- The port's default decode path (detection on) runs the rebuilt kernels;
  the engine still selects AOM (ADR-0031), so this changes no engine output.
- Each family is first rebuilt on NEON and then on AVX2 before the next
  family; 8 bpc before 16 bpc. Order of families by measured share: itx,
  loop filter, msac, CDEF, intra prediction; then loop restoration,
  palette, filter-intra and film grain.
- The benchmark (`scripts/bench/ci-perf/codec_avif.sh`) gains two context
  columns: `port-dav1d:t1` (the port's dav1d through
  `examples/decode_codec.rs`) and `c-dav1d-v1:t1` (stock dav1d restricted to
  the v1.0 SIMD scope).

## Validation

**2026-11-16, first slice** (macOS ARM64, Apple M1 Pro, loaded shared host;
`main` at `f1bb81fa` plus this branch): the 8 bpc DCT_DCT and IDTX entry
points of `itx.S` for 4x4, 4x8, 8x4, 8x8, 4x16, 16x4, 8x16, 16x8 and 16x16
(18 entry points, `dispatch.txt`: `itx_8bpc_arm`).

| Check | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd neon` (original assembly against the port, both at NEON) | 17 checkasm groups, 3,155 selections each, pass |
| `check_dav1d_dsp.py` (portable C against the port masked to C) | 3,155 selections each, pass |
| `check_dav1d_simd_original.py --level neon` / `--level scalar` | 994 of 994 decodes identical each (803 MD5-verified suite streams, 184 AV1 items of the libavif test AVIFs, 7 of the profiling stills) |
| Positive control: one constant of `idct_4` off by one | checkasm: 2 groups fail; whole decodes: 366 of 987 identical |
| `run_dav1d_tests.py` (original suite, Rust release decoder) | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,157 layout checks, 0 differences; 83 API comparisons; pass |
| x86-64 oracle (`build_dav1d.py --asm --cross x86_64-linux-gnu`, QEMU container) | builds (775 AVX2 entry points in the archive); 24 of 24 sampled suite streams give the expected MD5 with `--cpumask avx2` and `--cpumask 0` under `qemu-x86_64 -cpu max` |

Instructions per call against the original's assembly
(`measure_dav1d_kernels.py`, 10^6 calls on fixed inputs, NEON level, equal
outputs): 0.76 to 1.63 over the 27 kernel/path pairs, median 1.08; the full
DCT paths 1.03 (4x4) to 1.26 (8x16), 16x16 1.23; the DC-only paths of 4x8
(1.41) and 4x16 (1.63) are the outliers (the assembly's four-row loop in
`idct_dc_w4` is a per-row loop here).

Instructions per decode of the port's dav1d (one thread, 12 MP q60
stills): 8-bit 4:2:0 3.07 → 2.61 G (-15 %), 8-bit 4:4:4 4.25 → 3.71 G
(-13 %), 8-bit 4:2:0 with alpha 3.46 → 3.00 G (-13 %); 10-bit and lossless
unchanged (no 16 bpc kernel yet; lossless uses WHT). Stock dav1d with NEON
needs 0.99 G for the 8-bit 4:2:0 still. The itx share of the sampled 8-bit
4:2:0 profile fell from 40.5 % to 29.5 %; most of the rest is the 32-point
DCT and the ADST transforms, not rebuilt yet. Decoded pictures unchanged.

Not run: `transpile_dav1d.py --check` (needs C2Rust; CI's translate
workflow); the x86-64 per-kernel and whole-image gates (no x86 kernel yet;
they run in the QEMU container with the first AVX2 kernels); no hosted run.

**2026-11-16, 8 bpc NEON complete** (`64ce28a5`): all 156 entry points of
`itx_dsp_init_arm` at 8 bpc, `all_simd` set as in the original. Both
gates as above: 3,155 selections each at `neon` and scalar; 994 of 994
whole decodes at each level. Instructions per call against the assembly
over 175 kernel/path pairs (largest codable eob, 32 million pixels per
kernel), identical outputs: 0.77 to 1.39, median 1.17. Above 1.3: the
16-point ADST/flipADST combinations of 8x16, 16x8 and 16x16 (1.33 to 1.39)
and the 64x64 DC path (1.30). In the ADST kernels LLVM spills 40 to 60
vectors (counted in the disassembly) that the assembly keeps in its 32
registers; the instruction order is the assembly's, the register
assignment is the compiler's, so this cost cannot be removed from Rust
without assembly. Instructions per decode of the port's dav1d: 8-bit 4:2:0
1.93 G (from 3.07 G; the port's AOM path needs 2.21 G, stock dav1d with
NEON 0.99 G), 8-bit 4:4:4 2.50 G (AOM 2.56 G), 8-bit 4:2:0 with alpha
2.24 G (AOM 2.45 G); 10-bit and lossless unchanged. The itx share of the
8-bit 4:2:0 profile is now 8 %; msac (48 %) and the loop filter (19 %)
lead.

**2026-11-16, msac on NEON** (the seven entry points of `msac.S`,
`dispatch.txt`: `msac_arm`; all call sites of the `decode`, `recon_tmpl`
and `msac` units). Gates as above: `check_dav1d_dsp.py --simd neon` 17
groups, 3,155 selections each, now with msac's three test groups comparing
the original's `_neon` assembly with the rebuilt kernels; scalar 3,155
each; `check_dav1d_simd_original.py` 994 of 994 at `neon` and at `scalar`
(the 803 suite streams, 184 libavif test items, 7 profiling stills);
`run_dav1d_tests.py` 1,711 of 1,711; ABI 12,157 layout checks, 0
differences; API 83; linkage pass; dispatch `--check` and its unit test
pass. Positive control: `bool_equi`'s `v` off by 2 makes the
`msac.decode_bool` group fail. Instructions per call against the assembly
(`measure_dav1d_kernels.py --family msac`, 4 million calls each, identical
outputs): adapt4 1.10, adapt8 1.06, adapt16 1.16, hi_tok 1.06, bool_adapt
1.11, bool_equi 1.11, bool 1.10 (the rest is the register moves LLVM adds
around the packed `rng`/`v` lanes and, for adapt16, its stack buffer; a
first version that called `memcpy` near the end of the buffer needed a
stack frame in every kernel, 1.25 to 1.59). The dispatch functions' NEON
test (two loads and a branch per call) is not in these figures.
Instructions per decode of the port's dav1d (stock dav1d NEON in
brackets): 8-bit 4:2:0 1.68 G (from 1.93; 0.99), 8-bit 4:4:4 2.25 G
(2.50; 1.11), 8-bit 4:2:0 with alpha 1.99 G (2.24), 10-bit 4:2:0 3.12 G
(3.36; 1.16), 10-bit 4:4:4 4.30 G (4.54; 1.34), lossless 8-bit 4:4:4
11.70 G (16.22; 9.19); decoded pictures unchanged. Sampled 8-bit 4:2:0
profile: coefficient decoding and reconstruction in C 43 % (the kernels
are partly inlined there), loop filter 24 %, msac 23 %, itx 4 %.

**2026-11-16, loop filter 8 bpc on NEON** (`loopfilter.S`: the four
`dav1d_lpf_{v,h}_sb_{y,uv}_8bpc_neon`, the column filters and the four
filter cores; `dispatch.txt`: `loopfilter_8bpc_arm`, inserted at the end of
the translated init, where the original calls `loop_filter_dsp_init_arm`).
Gates: per-kernel 17 groups, 3,155 selections each at `neon` and scalar;
whole images 994 of 994 at each level; `run_dav1d_tests.py` 1,711 of 1,711;
ABI 12,157 checks, 0 differences; API 83; linkage pass. Positive control:
the `f + 4` constant of the wd4 filter off by one fails all four
`loopfilter_8bpc` groups. Instructions per call against the assembly
(`measure_dav1d_kernels.py --family lpf`, one 32-unit superblock edge with
wd4/8/16 mixed, identical pictures): row edges (`v`) 0.92 (y) and 0.91
(uv), column edges (`h`) 1.32 (y) and 1.33 (uv). The column edges load and
store 16 rows of 4 or 8 pixels per group by lane; the assembly walks two
row pointers with post-increment, the rebuild computes each row's address
and keeps more row pointers live (spilled), which is the excess.
Instructions per decode: 8-bit 4:2:0 1.44 G (from 1.68), 8-bit 4:4:4
1.87 G (2.25), 8-bit 4:2:0 with alpha 1.55 G (1.99); 10-bit and lossless
unchanged (3.12, 4.30, 11.70 G); pictures unchanged. Sampled 8-bit 4:2:0
profile: coefficient decoding in C 46 %, msac 36 %, itx 6 %, loop filter
5 %, CDEF 5 %.

**2026-11-16, loop filter 16 bpc on NEON** (`loopfilter16.S`: the four
`dav1d_lpf_*_16bpc_neon`, used at 10 and 12 bit as in the original;
`dispatch.txt`: `loopfilter_16bpc_arm`). Gates as for 8 bpc: per-kernel 17
groups, 3,155 selections each at `neon` and scalar (the 16 bpc groups run
at 10 and 12 bit); whole images 994 of 994 at each level; 1,711 of 1,711;
ABI, API, linkage pass. Positive control: the `f + 4` constant off by one
fails `loopfilter_16bpc.lpf_{h,v}_sb_y`. The unit tests that change the
process-wide CPU mask now hold a common lock (they ran in parallel).
Instructions per call against the assembly (10-bit pixels): row edges
0.88 (y) and 0.85 (uv), column edges 1.00 (y) and 0.96 (uv); none above
1.3. Instructions per decode: 10-bit 4:2:0 2.79 G (from 3.12; stock dav1d
NEON 1.16), 10-bit 4:4:4 3.75 G (4.30; 1.34); 8-bit and lossless
unchanged; pictures unchanged.

**2026-11-16, msac selected at compile time; the 8-bit gap accounted.**
The 8-bit 4:2:0 still needed 1.44 G instructions per decode against stock
dav1d's 0.99 G, with every rebuilt kernel near its assembly. Accounting
(macOS `sample` self time per function group of both decoders on the same
still, and instruction counts per decode): (1) the harness that measures
the port hashes the luma plane after each decode (6 instructions per byte,
0.073 G at 8 bit, 0.146 G at 10 bit); stock dav1d's figure has no such
step. (2) The msac dispatch functions tested `dav1d_get_cpu_flags()` at
every symbol (`adrp`, two loads, `and`, `tbz` before each call) and, being
inlined, kept the C call next to each kernel call in `decode_coefs`, whose
self time was twice the original's (75 against 37 samples per decode).
Selecting at compile time, as the original does, removed 0.084 G (8-bit
4:2:0 1.436 → 1.352 G) and brought `decode_coefs` to the original's self
time (39 against 37). The translated coefficient decoding itself is not
slower than the original's C. (3) What remains (1.352 − 0.073 = 1.28 G
against 0.99 G) is in the families not rebuilt yet: CDEF (C in the port,
NEON in stock; 7.7 % of the port's sampled decode time) and intra
prediction (8.1 %), then the kernel ratios above 1 (itx median 1.17, msac
1.06–1.16). The scalar whole-image gate now decodes with the portable
build; its consumer binary calls the C symbol decoder at 134 sites and the
NEON kernel at none (the default build: 31 and 0). Gates: per-kernel 17
groups, 3,155 selections each at `neon` and scalar; whole images 994 of 994
at `neon` and at `scalar`; `run_dav1d_tests.py` 1,711 of 1,711; ABI 12,157
checks, 0 differences; API, linkage, dispatch `--check` pass. Instructions
per decode: 8-bit 4:2:0 1.35 G (from 1.44), 8-bit 4:4:4 1.78 G (1.87),
8-bit 4:2:0 with alpha 1.47 G (1.55), 10-bit 4:2:0 2.72 G (2.79), 10-bit
4:4:4 3.69 G (3.75), lossless 8-bit 4:4:4 10.81 G (11.70); pictures
unchanged.

**2026-11-16/17, inverse transforms 16 bpc on NEON** (`itx16.S`: all 156
entry points of `itx_dsp_init_arm` at 16 bpc, `simd/arm64/itx16.rs`; the
second pass reuses the 16-bit transforms of `itx.rs` as the assembly
reuses those of `itx.S`; `dispatch.txt`: `itx_16bpc_arm`). As in the
original, the init assigns WHT at every bit depth and the rest only at
10 bit (`bpc != 10` returns), and sets `all_simd` only then. Gates:
per-kernel 17 groups, 3,155 selections each at `neon` and scalar (the
`itx_16bpc.add_tx4` … `add_tx64` groups now compare the original's
assembly with the rebuilt kernels at 10 bit); whole images 994 of 994 at
`neon` and at `scalar`; `run_dav1d_tests.py` 1,711 of 1,711; ABI 12,157
checks, 0 differences; API 83, linkage, dispatch `--check` and its unit
test pass; the init's selection test checks 10 bit (rebuilt), 12 bit
(WHT only) and mask 0 (C). Positive control: the idct4 coefficient 1567
of the 32-bit first pass set to 1568 fails all five `itx_16bpc` groups and
the 10-bit whole-image gate (767 of 989 decodes identical). Instructions
per call against the assembly (`measure_dav1d_kernels.py --family itx16`,
new 16 bpc mode of `dav1d-kernel-bench.c`: 32-bit coefficients, 10-bit
pixels; largest codable eob and the DC-only path, identical outputs):
0.85 to 1.29 over 175 kernel/path pairs, median 1.12; none above 1.3. The
highest are the ADST/flipADST combinations of 4x16 and 8x16 (1.26 to
1.29), as at 8 bpc register spills of the 16-point ADST.

The deviations stated in the module doc, and their effect on the
instruction count: (a) the `load_add_store*` macros run row by row
instead of interleaving four rows: the same instructions per row, only
their order differs, no effect on the count; done because the per-row
form reads against the macro body. (b) `idct_dc_w*` first loaded and
stored each row with one instruction per register, where the assembly
uses one multi-register `ld1`/`st1`; that put the DC-only paths of 32x32,
32x64 and 64x32 at 1.32 to 1.55. They now use `vld1q_s16_x2`/`_x4`, and
the 32- and 64-point dct_dct entry points call their coefficient path as a
separate function after the DC test, as the assembly sets up its stack
frame only after the `idct_dc` macro (one Rust function saved ten
registers and probed a 3 KiB frame before the test): the DC-only paths
are now 0.90 to 1.18. (c) The stack buffers are zero-initialized local
arrays: a `memset` per call on the full paths (the assembly leaves them
uninitialized; every element is written before it is read); it counts
against the port in the figures above and was kept so that no
uninitialized memory is read through a reference. (d) Pointer updates
that step past the block (post-increments after the last row, the
64-point first pass pointing `x9` past the coefficients) use wrapping
arithmetic or element offsets, so that no out-of-bounds pointer is formed
with `add`; no instruction difference.

Instructions per decode, from now on without the profiling harness's
luma hash (`--no-hash`; it cost 6 instructions per byte: 0.073 G at 8 bit,
0.146 G at 10 bit), stock dav1d with NEON on the same basis (the second and
third decode of a stream minus the first, `--cpumask neon`, null muxer) in
brackets; "before" is the msac entry's figure minus the hash: 10-bit 4:2:0
1.51 G (before 2.57; 1.13), 10-bit 4:4:4 2.08 G (3.54; 1.31); unchanged:
8-bit 4:2:0 1.28 G (0.96), 8-bit 4:4:4 1.71 G (1.09), 8-bit 4:2:0 with
alpha 1.39 G (0.96, color only), lossless 8-bit 4:4:4 10.72 G (9.14);
pictures unchanged.

**2026-11-17, CDEF on NEON, 8 and 16 bpc** (`cdef.S`, `cdef16.S` and the
shared `cdef_tmpl.S`: `simd/arm64/cdef.rs`, the template generic over the
bit depth, and `cdef16.rs`; the C glue of `src/arm/cdef.h`, the three
`DEFINE_FILTER` wrappers (padding into the 16-bit tmp buffer, then the
filter) and `cdef_dsp_init_arm`, translated by hand into the owned
`simd/arm/cdef.rs`; `dispatch.txt`: `cdef_8bpc_arm`, `cdef_16bpc_arm`, the
call at the end of the translated init, where `cdef_tmpl.c` calls
`cdef_dsp_init_arm`). Gates: per-kernel 17 groups, 3,155 selections each
at `neon` and scalar (`cdef_8bpc` and `cdef_16bpc` `cdef_dir` and
`cdef_filter` compare the original's assembly with the rebuilt kernels,
16 bpc at 10 and 12 bit); whole images 994 of 994 at `neon` and at
`scalar`; `run_dav1d_tests.py` 1,711 of 1,711; ABI 12,157 checks, 0
differences; API 83, linkage, dispatch `--check` (7 entries, 44 units)
and its unit test pass; layout and selection tests for both contexts.
Positive control: the primary tap table `[4, 2, 3, 3]` changed to
`[4, 2, 3, 4]` fails `cdef_filter` at both bit depths and the whole-image
gate (239 of 989 identical). Instructions per call against the assembly
(`measure_dav1d_kernels.py --family cdef`, new mode of
`dav1d-kernel-bench.c`: `dir` and the three block sizes through the
wrappers, edge sets 0xf, 0x0, 0x5, 0xa, identical outputs): `dir` 0.98 (8
bpc) and 0.97 (16 bpc), filters 0.80 to 0.98 (8 bpc) and 0.79 to 0.92
(16 bpc); the filter rows include the wrapper and the padding, which the
original runs as C around its assembly. Deviations: the dead
`mov w3, #8` of `find_dir` has no counterpart (one instruction less);
the wrappers leave the tmp buffer uninitialized as `ALIGN_STK_16` does,
written by the padding before the filter reads it through raw pointers;
the assembly's multi-register loads and stores are kept. Instructions
per decode (no harness hash; stock dav1d NEON in brackets): 8-bit 4:2:0
1.16 G (from 1.28; 0.96), 8-bit 4:4:4 1.34 G (1.71; 1.09), 8-bit 4:2:0
with alpha 1.28 G (1.39; 0.96 color only), 10-bit 4:2:0 1.36 G (1.51;
1.13), 10-bit 4:4:4 1.59 G (2.08; 1.31); lossless 8-bit 4:4:4 10.72 G
unchanged (CDEF is off in lossless; 9.14); pictures unchanged.

The 16 bpc chroma loop-filter checkasm groups (open since the loop filter
entry) do reach the wd4 core: a build with the `f + 4` fault there, run
through the original checkasm with seeds 1 to 40, fails `lpf_h_sb_uv` in
28 and `lpf_v_sb_uv` in 26 of the 40 seeds (`lpf_h_sb_y` 38, `lpf_v_sb_y`
35); the gate's single seed 42 is one where the chroma inputs miss the
changed rounding. No path is uncovered; one seed catches a one-constant
error in a narrow rounding step only with that probability.

**2026-11-17, the per-kernel gate runs eight seeds.** `check_dav1d_dsp.py`
ran the checkasm bodies once, with seed 42. It now runs them with the seeds
42, 1, 2, 3, 4, 5, 6 and 7 (`SEEDS` in the tool; 42 stays the first, the
other seven were fixed before the fault below was run against them), up to
four at a time (`--jobs`), and passes only if every seed passes with the
same selection inventory; `results.log` holds all eight outputs,
`inventory.json` lists the seeds. On this machine the gate takes 9.3 s
instead of 6.2 s at each level (27 s instead of 5.3 s of CPU time); eight
seeds one after the other would take about 28 s. All families rebuilt so far
pass at every seed: 17 groups, 3,155 selections each at `neon` and at scalar,
79 test rows per seed, 632 of 632 at each level. Positive control: the
`f + 4` constant of the 16 bpc wd4 core set to 5 fails all eight seeds;
`lpf_h_sb_uv` fails at seeds 1, 3, 4, 6 and 7 and `lpf_v_sb_uv` at 3, 6 and
7 (at 42, as before, only the two luma groups fail); restored, the gate
passes at both levels. Not run: the `--debug` mode of the tool (no debug
reference candidate on this machine; the seed loop is the same code).

**2026-11-17, intra prediction 8 bpc on NEON** (`ipred.S`:
`simd/arm64/ipred.rs`, all entry points: the ten DC, V, H, Paeth and
smooth predictors, the edge helpers and fill kernels of the directional
predictors Z1 to Z3, the filter predictor, the palette predictor, the four
chroma-from-luma predictors and the three `cfl_ac`; the C glue of
`src/arm/ipred.h`, the wrappers `ipred_z{1,2,3}_neon` and
`intra_pred_dsp_init_arm`, translated by hand into the owned
`simd/arm/ipred.rs`, with the two static functions of `ipred_tmpl.c` they
use, `get_filter_strength` and `get_upsample`; `dispatch.txt`:
`ipred_8bpc_arm`, the call at the end of the translated init, where
`ipred_tmpl.c` calls `intra_pred_dsp_init_arm`; all 22 slots the original
assigns under NEON: 14 `intra_pred`, 4 `cfl_pred`, 3 `cfl_ac`, `pal_pred`).
The file was translated in five parts (lines 35–1296, 1297–1850,
1851–3074, 3075–3685, 3686–5232) by five agents in private copies of the
crate, each gated against the assembly on its own, then joined into the
one module and gated again as a whole; the figures below are of the
joined state.

Gates: per-kernel 17 groups, 3,155 selections each at `neon` and scalar,
at each of the eight seeds (`ipred_8bpc.intra_pred`, `cfl_ac`, `cfl_pred`
and `pal_pred` compare the original's assembly with the rebuilt kernels);
whole images 994 of 994 at `neon` and at `scalar`; `run_dav1d_tests.py`
1,711 of 1,711; ABI 12,157 checks, 0 differences; API 83, linkage,
dispatch `--check` (8 entries, 52 units) and its unit test pass; layout,
selection (22 slots change under NEON, none with mask 0) and
edge-filter-decision tests in the glue. Positive controls, one constant
each, all eight seeds: the 1/3 multiplier of `ipred_dc` (`0x5556` to
`0x5756`) fails `intra_pred` and the whole-image gate (406 of 994
identical); the rounding shift of the chroma-from-luma splat (6 to 5)
fails `cfl_pred`; the index mask of `pal_pred` (7 to 3) fails `pal_pred`;
a shift of `cfl_ac` 4:2:0 (1 to 2) fails `cfl_ac`; restored, both gates
pass. The part agents ran further comparisons outside the gate, against
the hidden assembly symbols of the oracle archive: the edge helpers on
argument patterns only Z2 and Z3 produce (every `sz` and `end` up to 144
at each strength), the Z1 and Z3 fill kernels on random parameters
(400,000 cases each), and the three wrappers over every block size,
angle and flag set (106,704 cases); no difference.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family ipred --bpc 8`, new mode of `dav1d-kernel-bench.c`: every slot
through the DSP table for every block size of the original's test; Z1 to
Z3 over the test's 27 angles with the flag sets 0, 0x400 and 0x600, the
filter predictor over its five filters, `cfl_pred` over four alphas,
`cfl_ac` without and with the largest padding; at least 250,000 calls
per row, because a process's instruction count varies by some 10^5
between runs; identical outputs in all 518 rows): 0.80 to 1.26, median
1.06; none above 1.3. By kernel (lowest / median / highest): dc 0.83 /
1.09 / 1.15, v 0.92 / 0.99 / 1.12, h 0.95 / 1.04 / 1.09, dc_left 0.95 /
1.09 / 1.15, dc_top 0.93 / 1.05 / 1.12, dc_128 0.80 / 1.07 / 1.13,
paeth 1.04 / 1.09 / 1.17, smooth 1.04 / 1.07 / 1.20, smooth_v 0.98 /
1.08 / 1.23, smooth_h 1.01 / 1.10 / 1.20, z1 0.85 / 0.98 / 1.10, z2
1.02 / 1.10 / 1.26, z3 0.86 / 1.05 / 1.17, filter 1.00 / 1.06 / 1.15,
cfl_ac 1.01 / 1.07 / 1.16, cfl_pred 0.97 / 1.05 / 1.13, pal_pred 0.98 /
1.13 / 1.24. The highest rows and their cause, counted in the
disassembly: Z2 at widths 32 and 64 (1.19 to 1.26): where the assembly multiplies with `umull2`/`umlal2` and a
loop-constant operand, LLVM hoists the high half and emits `mov` +
`umull`, and in the top-only loop it merges the widened operands of two
loops and emits `ushll` + `mul`/`mla` (32 against 22 instructions per 16
pixels of two rows); `pal_pred` 4 and 8 wide (to 1.24) and the smooth
predictors 4 wide (to 1.23): `st1 {v.s}[i], [x0], x1`, a lane store with
a register post-increment, has no single-instruction form from Rust and
becomes `mov` + `str` + `add`. The Z rows include the wrapper, which the
original runs as C.

Deviations from the assembly, and whether they change the instruction
count: (a) vector registers are typed locals named after the registers,
not the register array of `util.rs` (bytes, 16-bit and 32-bit lanes
alternate; no effect). (b) Jump tables are a `match` on `clz(width) -
25` with an unreachable default, compiled to a compare chain: equal or
fewer. (c) `subs` + `b.gt` loop ends are "subtract, then not zero"
(`subs` + `b.ne`): same count, the same result for every block size; the
`> 0` form cost one more per iteration. (d) Four-row store loops address
the second row pair as `[x0, x1]` and advance each pointer once, and
single-lane stores are as described above: +1 to +4 per four rows.
(e) Large paths are separate functions reached by a tail call, so that
the small blocks keep no stack frame (`ipred_paeth_160`,
`ipred_smooth_160`, the 16- and 32-wide filter path, the per-width Z2
kernels, the tall Z3 path): +1 to +3 per call of a large block. (f) The
shared 16/32/64 loops of `ipred_z1_fill1` and the tall Z3 path are
instantiated per width or height, so that the compiler sees the column
count: fewer. (g) The DC sums are modeled with the sum in every lane,
and the tail of `ipred_dc` is written on both sides of its `b.eq`
(joined, LLVM computed the multiply on both paths, +6): +1 to +2.
(h) `uxtl` is `zip1` with zero, or `pmull` by one, where LLVM would
otherwise turn `mla` by element into `dup` + `ushll` + `mla` (filter
predictor, smooth weights): same count, two `movi` per call. (i) Z3:
`zip` + lane stores are `st2` lane stores, `xtn` + `xtn2` one `uzp1`,
the column step keeps the column top instead of `msub`: fewer; the
`tbx` of `ipred_z3_fill2` takes the two table registers the assembly
loads, where the instruction names four (differs only for indices the
wrapper cannot produce). (j) `cfl_ac`: the shared tails are functions
entered by tail calls, values the assembly computes twice are computed
once: count-neutral. (k) Dead instructions have no counterpart (`movi
v20.16b, #2` in Z2, `mov x4, #16` in `ipred_reverse`). (l) Pointers
stepped past a buffer after the last access use wrapping arithmetic,
unaligned scalar accesses `read_unaligned`/`write_unaligned`: no effect.

Deviations of the wrappers from the C, for memory safety. The kernels
load whole vectors, so they read stack elements the C wrappers never
write (the values do not reach the output). (m) `ipred_z2_neon`: `buf`
is zero-initialized and has four elements in front. In C it is
uninitialized and `flipped` is its first element; `ipred_z1_filter_edge`
at strength 3 loads `flipped[-1]`, and `ipred_z2_fill1` at width 64
loads down to `top[-69]`, four pixels before the array (angles 157 to
174): in the original an out-of-array stack read. (n) `ipred_z3_neon`:
`flipped` is zero-initialized with one element in front (the same
`in[-1]` load; `ipred_reverse` writes it in blocks of 16 and the edge
kernels read up to 15 elements more). (o) `top_out` (Z1) and `left_out`
(Z3) stay uninitialized as in C and are reached through raw pointers
only; the 16 elements behind the padded range are zeroed, because the
fill kernels load up to 15 elements past it (at width or height 16 and
more, the second row or column of a pair, whose base is not compared
with the limit). That bound comes from the part agents' enumeration of
the kernels' loads over every block size, angle and flag set (largest
index read against largest written), which the lead agent did not
re-derive; zeroing the whole buffers instead is a two-line change per
wrapper and was measured at 7 to 13 instructions per call. Cost as
built: 7 instructions per Z3 call for `flipped` and about 4 per Z1 or Z3
call for the 16 elements; the Z2 buffer's zeroing was not measured
apart. (Replaced the same day: both buffers are zeroed whole, see the
entry on the wrappers below.)

Kept as the assembly, stated as a finding: Z1 at 64x64 with the intra
edge filter makes `ipred_z1_filter_edge` load `topleft[-1..=142]`, 14
pixels past the caller's `edge[257]` array. In the decoder those bytes
are inside the same scratch union and the values are masked out
(`padding_mask`); in checkasm they are stack bytes. Every caller must
provide the whole edge array readable, not only the prepared range (the
kernels also read up to 48 pixels below `topleft[-2h]`, inside the
array). The rebuild performs the same loads.

Instructions per decode (no harness hash; stock dav1d NEON in
brackets): 8-bit 4:2:0 1.07 G (from 1.16; 0.96), 8-bit 4:4:4 1.21 G
(1.34; 1.09), 8-bit 4:2:0 with alpha 1.14 G (1.28; 0.96 color only),
lossless 8-bit 4:4:4 10.23 G (10.72; 9.14); 10-bit unchanged (1.36 and
1.59 G); pictures unchanged.

Formatting: the repository's `rustfmt.toml` ignores `codecs/libavif-rs`,
so `cargo fmt --all --check` passes without looking at these files. The
new modules are formatted with `max_width = 120` and checked with
`rustfmt --check --config-path` on a file holding that one setting; the
earlier `simd` modules follow it only approximately (up to 9 differences
per file), which this step did not change.

**2026-11-17, intra prediction 16 bpc on NEON** (`ipred16.S`:
`simd/arm64/ipred16.rs`, the same entry points as at 8 bpc, used at 10
and 12 bit as in the original; the filter predictor is the macro
`filter_fn bpc` as a generic function with a 10-bit and a 12-bit
instance, chosen by `bitdepth_max` as the assembly does; the wrappers
and the init of `simd/arm/ipred.rs` are the 8 bpc ones compiled for
`uint16_t` pixels with `bitdepth_max`, as the C header is compiled twice;
`dispatch.txt`: `ipred_16bpc_arm`, all 22 slots). Translated in the same
five parts by the same agents after their 8 bpc part, joined and gated as
a whole.

Gates: per-kernel 17 groups, 3,155 selections each at `neon` and scalar,
at each of the eight seeds (the four `ipred_16bpc` groups compare the
original's assembly with the rebuilt kernels at 10 and 12 bit); whole
images 994 of 994 at `neon` and at `scalar`; `run_dav1d_tests.py` 1,711
of 1,711; ABI 12,157 checks, 0 differences; API 83, linkage, dispatch
`--check` (9 entries, 52 units) and its unit test pass; layout and
selection tests for the 16 bpc context. Positive controls, one constant
each, all eight seeds: a 1/3 multiplier of `ipred_dc` (`0xAAAB` to
`0xABAB`) fails `intra_pred` and the whole-image gate (808 of 994
identical); the rounding shift of the chroma-from-luma splat (6 to 5)
fails `cfl_pred`; the index mask of `pal_pred` (7 to 3) fails `pal_pred`;
a shift of `cfl_ac` 4:2:0 (1 to 2) fails `cfl_ac`; restored, both gates
pass. The part agents' comparisons outside the gate were repeated at 16
bpc with at least the same case counts; no difference.

Instructions per call against the assembly (`--family ipred --bpc 16`,
10-bit pixels, 518 rows as at 8 bpc, identical outputs in all): 0.88 to
1.22, median 1.04; none above 1.3. By kernel (lowest / median /
highest): dc 0.92 / 1.07 / 1.16, v 0.92 / 1.01 / 1.12, h 0.97 / 1.14 /
1.22, dc_left 0.93 / 1.01 / 1.11, dc_top 0.93 / 1.01 / 1.12, dc_128
0.92 / 0.98 / 1.12, paeth 1.00 / 1.08 / 1.19, smooth 0.96 / 0.97 /
1.21, smooth_v 0.94 / 0.96 / 1.13, smooth_h 0.92 / 0.93 / 1.16, z1
0.94 / 1.01 / 1.08, z2 1.01 / 1.05 / 1.11, z3 0.89 / 1.00 / 1.12,
filter 1.01 / 1.09 / 1.14, cfl_ac 0.88 / 1.06 / 1.20, cfl_pred 0.96 /
0.99 / 1.17, pal_pred 1.00 / 1.04 / 1.16. The highest rows and their
cause: `ipred_h` 32 wide (1.20 to 1.22; the tail call into the 16/32/64
function and its second dispatch, and six `add`s for four row addresses,
18 against 15 instructions per four rows); smooth 4 and 8 wide (1.19 to
1.21; LLVM widens the loop-invariant `top - bottom` outside the row loop
and compiles the second `smlal` pair as `ushll` + `mla`); `cfl_ac` 4:2:0
16 wide (1.20; `vld1q_u16_x4` compiles to `ldp` pairs, eight for the
assembly's four `ld1`). The 12-bit instance of the filter predictor is
gated by checkasm but not measured (the bench runs 10 bit). Rows of
cheap kernels on large blocks still vary by a few hundredths between
measurement runs.

Deviations from the assembly beyond those of the 8 bpc entry, which
apply here as well ((a) to (g), (i) to (l)): (p) the 64-column store
loops of `ipred_dc_128` and `ipred_v` are functions with one pointer per
half row; they store both first halves of a row pair, then both second
halves, where the assembly alternates (disjoint memory; same count per
row, +2 per call). `ipred_h` for 16, 32 and 64 columns stores a row's
parts together. (q) The 16+ column loops of the smooth predictors are
instantiated per width; as run-time loops they measured 1.20 to 1.49.
(r) `ipred_z2_fill1` for 16+ columns multiplies by element (`umull v,
v, v.h[0]`) where the assembly multiplies by a vector whose lanes are
all equal: same values, same count; the by-vector form measured 1.18 to
1.33 for the reason given at 8 bpc. Its lane loads (`ld2`/`ld3`/`ld4`
of one lane) need an initial tuple value, two to four `mov` per block.
(s) `cfl_ac` 4:2:0 adds the two rows and then pairs (`add`, `addp`)
where the assembly pairs and then adds: same sums, same count; LLVM
reassociated the literal form into five instructions (1.24 to 1.37).
(t) Two oddities of the 16 bpc assembly are kept as they are, both
outside what the wrappers can reach and compared against the assembly
directly by the part agents: `ipred_z1_fill2` and `ipred_z3_fill2`
replicate one byte (`ld1r {v31.16b}`), not one pixel, as their padding
value, and the last padding loop of the three-tap `ipred_z1_filter_edge`
counts 16 pixels per 8 stored. (u) Dead instructions have no counterpart
(five `movi`, one `mov` and one `add` in the Z2 kernels).

The wrappers' buffers are as at 8 bpc ((m) to (o)). Zeroing the whole
`top_out` instead of 16 elements was measured at 16 bpc: a `bzero` call
of 604 bytes, about 68 instructions, which put Z1 4x4 and 8x4 at 1.32 to
1.35. At 16 bpc no fill kernel reads past the padded range of `left_out`
(largest index read 173 against 174 written), and Z1 reads at most 3
elements past it; `ipred_z2_fill1` reads single pixels down to
`left[-81]`, which is `top[-16]`, inside the buffer. The finding of the
8 bpc entry holds with other figures: Z1 64x64 with the edge filter
loads `topleft[-1..=134]`, 6 pixels past `edge[257]`, behind the union
inside the same scratch structure.

Instructions per decode (no harness hash; stock dav1d NEON in
brackets): 10-bit 4:2:0 1.24 G (from 1.36; 1.13), 10-bit 4:4:4 1.41 G
(1.59; 1.31); 8-bit and lossless unchanged (1.07, 1.21, 1.14 and
10.23 G); pictures unchanged. Against stock dav1d with NEON the port now
needs 1.11 times the instructions on the 8-bit 4:2:0 still, 1.12 on
8-bit 4:4:4 and lossless, 1.10 on 10-bit 4:2:0 and 1.08 on 10-bit
4:4:4. Intra prediction is complete on NEON; not rebuilt on any target:
loop restoration and film grain; on x86-64 nothing yet.

**2026-11-17, intra prediction wrappers: buffers zeroed whole, edge
slack checked** (follow-up to the two entries above, on two decisions they
left open). (1) `ipred_z1_neon` zeroes the whole `top_out` and
`ipred_z3_neon` the whole `left_out` before anything is written to them,
at both bit depths, instead of the 16 elements behind the padded range
(deviation (o)). The partial form was sound only if the enumeration of
every kernel load over every block size, angle and flag set was right,
and nobody had re-derived it; now no load from a wrapper's buffer can
find an element without a value whatever the kernels load, as for the Z2
and Z3 `buf` and `flipped` buffers and the 16 bpc inverse transforms.
`zeroed_scratch` in `simd/arm/ipred.rs` writes the buffer with 64-byte
multi-register stores (`st1 {v0.16b-v3.16b}`, the last one ending at the
buffer's end and overlapping the one before): five stores for the 302 or
286 bytes at 8 bpc, ten or nine for the 604 or 572 bytes at 16 bpc. As
compiled, counted in the disassembly of the Z1 wrapper: 14 instructions
per call at 8 bpc and 24 at 16 bpc (one `movi` and three `mov`, then
`add` + `st1` per store, because `st1` of several registers has no
offset addressing), against about 4 for the 16 elements, and against the
`bzero` call of about 68 instructions that a plain `[0; N]` compiles to
at 16 bpc. `--family ipred`, all 518 rows per bit depth, identical
outputs: 0.83 to 1.26, median 1.07 at 8 bpc and 0.88 to 1.23, median
1.05 at 16 bpc; none above 1.3, so no row had to be accepted. Z1
(lowest / median / highest) 0.85 / 1.00 / 1.12 at 8 bpc and 0.95 / 1.06
/ 1.15 at 16 bpc (before: 0.85 / 0.98 / 1.10 and 0.94 / 1.01 / 1.08), Z3
0.86 / 1.08 / 1.18 and 0.89 / 1.02 / 1.18 (0.86 / 1.05 / 1.17 and 0.89 /
1.00 / 1.12); the highest Z1 row is 8x4 with the edge filter at 16 bpc
(278 against 241 instructions), the highest Z3 rows 16x32 at 8 bpc and
4x4 at 16 bpc.

(2) The read behind the caller's edge array stays as the assembly (Z1 at
64x64 with the intra edge filter: `topleft[-1..=142]` at 8 bpc,
`topleft[-1..=134]` at 16 bpc, 14 and 6 pixels behind `edge[257]`), and
the assumption that makes it harmless is now a test,
`the_edge_array_has_readable_slack_behind_it` in `simd/arm/ipred.rs`.
The callers, read from the translated units: the `intra_pred` slots have
four call sites per bit depth, all in `recon_tmpl`;
`dav1d_recon_b_intra` passes `t->scratch.edge + 128` for luma and for
chroma, and `dav1d_recon_b_inter` (inter-intra) passes the same array
`+ 32` for blocks of at most 32x32 with the modes DC, vertical,
horizontal and smooth, which `dav1d_prepare_intra_edges` never turns
into Z1. The test takes the offsets and sizes of the translated
`Dav1dTaskContext` of each bit depth (`offset_of!` along the field path
the callers use) and requires the 14 bytes (8 bpc) and 12 bytes (16 bpc)
behind the edge array to lie inside the structure that holds it, and
that structure inside the scratch union. As translated there are 31 and
62 bytes (alignment padding and the palette array); the contexts are
allocated and zeroed whole by `dav1d_open`, so the padding has a value.
Demanding one byte more than there is makes the test fail (checked with
32 at 8 bpc). Outside the decoder the slot is called by the original's
checkasm test, with a 257-pixel stack array, as for the assembly, and by
`tools/dav1d-kernel-bench.c`, whose edge array now has 16 pixels behind
the 257 (same outputs). Stated limit: the callers derive the pointer
from the array field (`edge_8bpc.as_mut_ptr()` in the translation), so
under Rust's reference-based aliasing models it is a pointer to 257
pixels, although the bytes behind them are in the same allocation; the
test checks the layout, not that.

Gates at this state: per-kernel 17 groups, 3,155 selections each at
`neon` and scalar, at each of the eight seeds; whole images 994 of 994
at `neon` and at `scalar`; `run_dav1d_tests.py` 1,711 of 1,711; ABI
12,157 checks, 0 differences; API 83, linkage, dispatch `--check` (9
entries, 52 units) and its unit test; `cargo test --lib simd::` 17
tests; `rustfmt --check` with `max_width = 120` on the three ipred
modules.

**2026-11-17, loop restoration on NEON, 8 and 16 bpc**
(`looprestoration.S`, `looprestoration16.S` and the shared
`looprestoration_common.S` and `looprestoration_tmpl.S`:
`simd/arm64/looprestoration.rs`, `looprestoration16.rs`,
`looprestoration_common.rs` and `looprestoration_tmpl.rs`, the template
generic over the bit depth; the C glue of `src/arm/looprestoration.h` as
compiled for AArch64, that is the self-guided wrappers
`sgr_filter_{5x5,3x3,mix}_neon` with their helpers and
`loop_restoration_dsp_init_arm`, translated by hand into the owned
`simd/arm/looprestoration.rs`; `dispatch.txt`: `looprestoration_8bpc_arm`
and `looprestoration_16bpc_arm`, the call at the end of the translated
init with its `bpc` argument, where `looprestoration_tmpl.c` calls
`loop_restoration_dsp_init_arm(c, bpc)`). Slots as in the original: under
NEON the two `wiener` slots (7 and 5 taps) at every bit depth, and the
three `sgr` slots (5x5, 3x3, mix) at 8 bpc and for 10-bit pictures;
12-bit pictures keep the C self-guided kernels (`BITDEPTH == 8 || bpc ==
10`). The header's `wiener_filter_neon` and row kernels under `ARCH_ARM`
belong to the 32-bit build and have no counterpart. One step for both
bit depths: the vertical box sums, the template and the wrappers are
shared between them, as with CDEF. The assembly was translated in two
parts (the Wiener filters; the self-guided kernels) by two agents in
private copies of the crate, each gated against the assembly on its own,
then joined, changed in the points named below (the buffers) and gated
again as a whole; the figures below are of the joined state unless
marked.

Gates: per-kernel 17 groups, 3,155 selections each at `neon` and scalar,
at each of the eight seeds (`looprestoration_8bpc` and
`looprestoration_16bpc` `wiener` and `sgr` compare the original's
assembly with the rebuilt kernels: Wiener at 8, 10 and 12 bit, the
self-guided filters at 8 and 10 bit; the 12-bit self-guided rows compare
C with C, as the original's init leaves them); whole images 994 of 994
at `neon` and at `scalar`; `run_dav1d_tests.py` 1,711 of 1,711; ABI
12,157 checks, 0 differences; API 83, linkage, dispatch `--check` (11
entries, 60 units) and its unit test (10 tests, one new: the call gets
`bpc`); `cargo test --lib simd::` 20 tests (three new: context and
parameter layouts, selection at 8 bpc, selection at 16 bpc for 10 and
for 12 bit); `cargo check --all-targets`. Positive controls, one
constant each, all eight seeds, every one through both gates (decodes
identical of 994 in brackets): 8 bpc, the horizontal rounding constant
of the 7-tap Wiener filter (`8 << 8` to `7 << 8`) fails `wiener`, only
`wiener_7tap_8bpc` (531); a tap offset of the 5x5 box sums (`ext #3` to
`#2` in `sgr_box5_row_h`) fails `sgr`, only `sgr_5x5_8bpc` (869); a tap
offset of the 3x3 box sums (`sgr_box3_row_h`) fails `sgr`, only
`sgr_3x3_8bpc` (774); 16 bpc, the vertical rounding shift of the 7-tap
Wiener filter (`round_bits_v`, 11 to 12 subtracted) fails `wiener`,
`wiener_7tap_10bpc` and `wiener_7tap_12bpc` (834);
the tap offsets of the 5x5 and of the 3x3 box sums fail `sgr`, only
`sgr_5x5_10bpc` (925) and only `sgr_3x3_10bpc` (965); both bit depths,
the rounding shift of the mix weighting in the shared template
(`sgr_weighted2`, 11 to 10) fails both `sgr` groups, only `sgr_mix_8bpc`
and `sgr_mix_10bpc` (571). Restored, all gates pass.

What the gate's seeds do not reach, and what was run instead. Each
checkasm seed draws one width and one height per filter. With small
consecutive seeds, the gate's eight among them, the 5x5 and 3x3 tests
draw only heights that are multiples of 8 and the mix test only widths
that are multiples of 8 (counted by the part agent over the seeds 1 to
3,000 from the sizes of a failing control; not re-derived): the
`vert_1`, `vert_2` and `odd` tails of the wrappers and most right-edge
tails are then not run. The gate is unchanged; two runs outside it, on
the final state: (1) the original's checkasm tests at 3,000 scattered
seeds, `(i * 2654435761 + 12345) mod 2^32`, which the part agent counted
to cover every height and every width for the three self-guided filters:
3,000 of 3,000 pass at each bit depth. (2) An identity sweep in one
process, the original with its assembly against the port: all five
slots, every width from 1 to 384, 14 heights (1 to 9, 31 to 33, 63, 64),
all 16 edge sets, including what the test never draws (a right edge with
a width other than 256, a bottom edge with a height other than 64), on
four kinds of input: 430,080 calls each at 8 bit, at 10 bit, at 12 bit
(Wiener rebuilt, self-guided C on both sides) and with 12-bit data
through the 10-bit slots; no pixel of a `w x h` block differs and
neither side writes outside the rows `0..h` and the columns
`0..round_up(w, 16)`.

Finding in the original, from that sweep: in the columns from `w` to the
next multiple of 8, which the self-guided filters write and nobody
reads, the original's output depends on its uninitialized stack. The
original against itself, the stack filled with 0x00 before one call and
with 0xa5 before the other, differs there in about three of four calls
(63,644 to 64,492 of 86,016 per filter at 8 bit) and never inside the
block: its kernels load elements of the wrappers' row buffers that no
kernel has written. The port, whose buffers are zeroed, equals the
original run on a zero-filled stack in every byte (0 of 430,080 calls at
8 and at 10 bit) and equals itself whatever the stack holds. The Wiener
filters show no such dependence. In the decoder the columns belong to
the picture's padding (only the last unit of a row has a width that is
not a multiple of 8).

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family lr`, new mode `lr` of `dav1d-kernel-bench.c`: the five slots on
units of 256x64, 64x64, 32x32 and 16x16 pixels with the edge flags 0xf
and 0x0, 64x64 also with 0x5 and 0xa; the test's checkerboard pattern,
10-bit at 16 bpc; 250,000 calls per row; identical outputs in all 100
rows; the self-guided rows include the wrappers, which the original runs
as C): 8 bpc 1.02 to 1.27, median 1.07; 16 bpc 1.03 to 1.29, median
1.11; none above 1.3. By slot (lowest / median / highest): 8 bpc
wiener7 1.02 / 1.09 / 1.26, wiener5 1.03 / 1.11 / 1.27, sgr_5x5 1.05 /
1.07 / 1.14, sgr_3x3 1.06 / 1.07 / 1.09, sgr_mix 1.06 / 1.06 / 1.14;
16 bpc wiener7 1.03 / 1.11 / 1.29, wiener5 1.04 / 1.12 / 1.29, sgr_5x5
1.08 / 1.09 / 1.19, sgr_3x3 1.09 / 1.11 / 1.19, sgr_mix 1.07 / 1.09 /
1.21. By size: 256x64 1.02 to 1.08 and 1.03 to 1.09, 64x64 1.05 to 1.11
and 1.08 to 1.12, 32x32 1.06 to 1.17 and 1.11 to 1.17, 16x16 1.08 to
1.27 and 1.18 to 1.29. The highest rows are the Wiener filters on 16x16
(1.21 to 1.29; about 2,000 to 3,100 instructions per call): the cost
per call that does not shrink with the unit, which is the zeroing of
the row buffer (144 instructions for the 4,608 bytes of the 7-tap
filter, 96 for the 5-tap one) and the row functions' private register
convention as an explicit structure (about 11 instructions per row call
at 8 bpc and 25 at 16 bpc, part agent's count). Per 16 pixels of a row
the Wiener loops are 2 to 5 instructions above the assembly (for example
84 against 80 to 83 in the 7-tap horizontal-and-vertical pass at 8 bpc;
part agent's count in the disassembly), the self-guided kernels between
5 fewer and 9 more per block at 8 bpc (part agent's count; the most in
`sgr_box3_vert`, 85 against 76 per 16 columns: the pointer `add`s where
the assembly post-increments, and one spilled table pair). At 16 bpc the
self-guided kernels are inlined into the wrappers and were not counted
apart.

Deviations from the assembly, and whether they change the instruction
count. Wiener filters: (a) the internal functions
`wiener_filter{7,5}_{h,v,hv}` are separate functions, as in the
assembly, and take the registers the assembly keeps across `bl` as the
fields of a structure (`WienerRegs`) by reference; they store the
shifted row pointers at entry where the assembly pushes them shifted and
pops them at the end: the per-row cost named above. Inlined, LLVM shared
the widened coefficients between the copies of the loops and compiled
the `smull`/`smlal` chains as `sshll` + `mul`/`mla` (110 against 80
instructions per 16 pixels; 1.28 to 1.37 on 256x64; part agent's
measurement). (b) The labels of the entry points are labeled blocks, the
fall-through chain `v3` to `v2` to `v1` a count of the vertical passes
that remain: +1 to +2 per call. (c) The right-edge padding test is one
condition at the head of the row loop, evaluated where the assembly's
`tst`/`cmp` pairs are: +1 per 16 pixels with a right edge, -2 without.
(d) A two-register `st1` with post-increment is two stores (one `stp`
with post-index; the multi-register intrinsic compiled to `st1` and a
separate `add`); the lane load of `left` is a scalar load into a zero
vector: no effect. (e) Compiler choices, +2 to +5 per 16 pixels in all:
`mov` + `ssra` for `sshr` + `add`, a three-instruction loop end, one
vector move; at 16 bpc `saddl` + `mul`/`mla` for part of the vertical
chain and the saves of `d8` to `d11` per row call. (f) 16 bpc only: the
source and destination pointers of the row loops advance on their
address (`wiener_post_inc`: `expose_provenance` and
`with_exposed_provenance_mut`), so that LLVM keeps one post-indexed
pointer per row; with pointer arithmetic on all of them its loop
strength reduction uses one index and computes every address anew,
which measured 1.12 to 1.17 instead of 1.04 to 1.08 on 256x64 (the
16x16 rows, at 1.23 to 1.29, would then be expected above 1.3; not
measured). Defined behavior, but not expressible
under strict provenance; replacing the helper's body by
`x.wrapping_add(n)` removes it at that cost. (g) The six (four) rows of
intermediate values, which the assembly keeps on its stack
uninitialized, are zeroed whole (`util::zeroed`): which rows a vertical
pass loads depends on the edge flags and the height, so "written before
loaded" is not evident at the site. Self-guided kernels: (h) the row
kernels test the right-edge flag once and run one of two loops over the
same block (a function-local macro), where the assembly tests it after
every block: two instructions less per block (as one loop the compiler
spent five per block). (i) `ldr` + `dup` is `ld1r`, the lane load of
`left` a replicated load, a dead post-increment of `left` has no
counterpart: one less per call. (j) The template's `dup` of
`bitdepth_max` runs at 16 bpc only; the assembly runs it at 8 bpc on an
unset register and never reads the result: one less per call.
(k) Multiplications by a weight are by element (`v.h[0]`): same
instructions; the by-vector form compiled the two-row loop of
`sgr_weighted2` to 34 instead of 30 instructions per block (assembly
28); its one-row loop stays at 20 against 15 (`sshll` + `mul`/`mla` for
the widened weights). (l) The block that `sgr_finish_filter2_2rows` and
`sgr_finish_weighted2` share instruction for instruction is one helper;
no effect. Kept as the assembly: at 16 bpc the padding mask of the 5x5
and the combined row kernel is addressed with a byte offset where a
pixel offset is meant (harmless by the part agent's analysis: the pixel
it leaves equals the padding pixel; the sweep above agrees);
`sgr_finish_weighted2` for one row uses the first `A` row as its
unused second destination row; the swapped comments `RangeMins` /
`DiffMasks` in `looprestoration_common.S` (the registers are right).
The continuation of `sgr_weighted2` for more than two rows is translated
but never run: the wrappers pass one or two (not verified by any run).

Deviations of the wrappers from the C. (m) The `goto` tails (`vert_1`,
`vert_2`, `odd`, `output_1`, `output_2`) are a `match` on the label the
main part leaves through; `params->sgr` is read once at the top: no
effect on the kernels. (n) Every buffer the kernels load from is zeroed
whole before a kernel gets it, where `ALIGN_STK_16` leaves it
uninitialized: the `sumsq`, `sum`, `A` and `B` row buffers of the three
filters (14.6, 17.0 and 34.0 KB) and the two `tmp` buffers of
`sgr_finish_mix_neon`; this is the finding above turned into defined
behavior. The zeroing is `util::zeroed` (64-byte multi-register stores,
compiled to two instructions per 64 bytes without a loop or a call: 266
stores for the 5x5 filter), the helper
introduced for the intra prediction wrappers and now shared with them
(their instruction counts are unchanged: Z1 0.85 / 1.00 / 1.12 and 0.95
/ 1.05 / 1.15, Z3 0.86 / 1.09 / 1.18 and 0.89 / 1.02 / 1.18, 518 rows
per bit depth identical). A plain `[0; N]` is a `bzero` call, measured
by the part agent at 1,228, 1,398 and 2,805 instructions per call of
the three filters and 230 per pair of rows for the `tmp` buffers, which
put the self-guided 16x16 rows at 1.26 to 1.52 and mix on 32x32 at 1.28
to 1.29; with the explicit stores the 16x16 rows are 1.08 to 1.21. (o) The `tmp` buffers
are declared in `sgr_filter_mix_neon` and passed to
`sgr_finish_mix_neon`, so that they are zeroed once per call and not
once per pair of rows. No kernel has a stack buffer of its own besides
the Wiener rows of (g).

Reads the contract does not state, kept as the assembly: without a right
edge the row kernels still load whole blocks behind the last pixel of a
source row and replace the values by the padding pixel afterwards: up to
20 (7 taps) or 21 (5 taps) pixels in the Wiener filters, up to 15 in the
box sums, of the picture rows and of the `lpf` rows (the part agents'
derivation from the code, not checked with guard pages). The original
provides for it: pictures are allocated `DAV1D_PICTURE_ALIGNMENT` (64)
bytes longer, with a width that is a multiple of 128, and the `lpf`
buffer 128 bytes longer ("lr simd may overread the input", `decode.c`).
Nothing is read left of a row without a left edge, and writes stay
within the contract's aligned width.

Instructions per decode of the six stills (no harness hash; port with
the loop restoration slots not assigned, port now, stock dav1d NEON):
lossless 8-bit 4:4:4 10.239 G, 10.238 G, 9.14 G; 10-bit 4:2:0 1.243,
1.244, 1.13; 10-bit 4:4:4 1.414, 1.415, 1.31; 8-bit 4:2:0 1.071, 1.071,
0.96; 8-bit 4:2:0 with alpha 1.146, 1.146, 0.96 (color only); 8-bit
4:4:4 1.212, 1.212, 1.09; pictures unchanged. (The alpha still was 1.145
before the intra prediction wrappers zeroed their buffers whole.) No
change within the variation between runs: none of the six stills uses loop
restoration (no loop restoration function in their sampled profiles;
the encoder's still-image mode leaves it off). Two streams of the
original's test suite that use it, decoded with the original CLI linked
to either library, one thread, film grain synthesis off, whole process:
`av1-1-b8-23-film_grain-50` 135.4 M to 126.1 M instructions (stock
96.5 M), `av1-1-b10-23-film_grain-50` 180.3 M to 157.5 M (128.2 M);
equal MD5. These are inter-coded, so most of the remaining difference
is inter prediction, which is not rebuilt.

Formatting: as for the intra prediction modules, `rustfmt --check
--config-path` with `max_width = 120` on the five new files, on
`simd/arm/ipred.rs` and on `simd/arm64/util.rs`; `cargo fmt --all
--check` passes without looking at them.

Loop restoration is complete on NEON. Not rebuilt on any target: film
grain, and the inter prediction kernels, which stills reach only through
intra block copy and superres; on x86-64 nothing yet.

**2026-11-17, gate seeds: two more for loop restoration.** The loop
restoration test of checkasm draws one width (1 to 384) and one height (1 to
64) per function and seed, and tests them with the sixteen edge sets (256
wide with a right edge, 64 high with a bottom edge). checkasm seeds its
xorshift128 generator with the seed and its complement without mixing, so
neighboring seeds draw related sizes. Measured with a copy of the gate's
test executable that prints the sizes (the test body otherwise unchanged;
the sizes are the same at 8, 10 and 12 bit): at the gate's eight seeds the
5x5 and the 3x3 self-guided filter, which draw the same size, got 174x24,
135x16, 134x24, 133x32, 132x40, 131x48, 130x56 and 129x64, and the combined
filter 216x43, 16x2, 152x3, 288x4, 168x5, 304x6, 56x7 and 192x8; over the
seeds 8 to 1,200 every 5x5/3x3 height is a multiple of 8 and every width of
the combined filter a multiple of 8. One fault per tail of the wrappers
(`src/arm/looprestoration.h`; the strength of the tail's first box sum
halved, at 5x5 `vert_2` a row pointer) and one in the right-edge padding of
the combined row kernel at each bit depth (the mask offset one too large
unless the remaining width is 2, 8 or 10), each run through the gate:
`vert_2` of the three wrappers and `odd` of the combined filter failed at
the eight seeds; `vert_1` of the three wrappers (a height of 1), `odd` of
5x5 (an odd height of 3 or more) and both padding faults (a width that is
not a multiple of 8) passed all eight. The predecessor's statement was
therefore right about `vert_1`, 5x5's `odd` and the widths, and too wide
about `vert_2`. `SEEDS` gains 1129287872 (5x5 and 3x3 at 186x1, combined at
257x1) and 2326805120 (235x7 and 135x1), found among 4,000 scattered seeds
(multiples of 2654435761 modulo 2^32) as a pair that reaches all of it; the
eight earlier seeds stay. With the ten seeds the six faults that passed fail:
3x3 `vert_1`, 5x5 `vert_1` and both bit depths' padding at 1129287872, 5x5
`odd` at 2326805120, the combined filter's `vert_1` and the padding at both
(the wrapper faults at 8 and at 10 bit, each padding fault at its bit
depth), and at no earlier seed. Restored, all families pass
at every seed: 17 groups, 3,155 selections each, 79 rows per seed, 790 of
790, at `neon` and at scalar. No already rebuilt kernel fails at the new
seeds. A first form of the faults (strength plus one) was too weak to be a
control: the 3x3 tails passed with it at every seed, new ones included. Not
covered by this: the Wiener filters' short heights (the 5-tap filter draws
7 or 8 at the first eight seeds; 47 and 2 at the new ones), which the
3,000 scattered seeds of the loop restoration entry cover outside the gate.

**2026-11-17, gate seeds: one more for the 8 bpc loop filter's 16-pixel
filter.** Found while the AVX2 loop filter was translated (the part agent's
control in its copy of the crate): a wrong constant in the arithmetic of the
16-pixel luma filter (flat16) of the 8 bpc kernel passed all ten seeds and
failed at 12 of the seeds 1 to 200. The NEON kernel has the same hole:
with `p6 + p6` of `lpf_16_wd16` changed to `p6 + p5` in the low half (a copy
of the crate, the gate's test executable restricted to `loopfilter_8bpc`)
the ten seeds pass, and seed 15 fails `lpf_h_sb_y` and `lpf_v_sb_y`. At
8 bpc the test's random edges rarely produce a pixel that the flat16 filter
writes; at 16 bpc they do at most seeds. `SEEDS` gains 15; the ten earlier
seeds stay. All rebuilt families pass at the eleven seeds at `neon` and at
scalar (17 groups, 3,155 selections each, 869 of 869 rows). The whole-image
gate decodes streams that use the filter, so the kernels were compared
there before; the per-kernel gate now reaches it as well.

**2026-11-17, a local x86-64 loop.** The x86-64 gates run on the development
Mac in an x86-64 GNU Linux container that Rosetta executes
(`tools/rosetta_x86_64`; Colima 0.10.3 with `--vz-rosetta`, macOS 27.2.1,
Debian trixie, Clang 19 as `cc`, the pinned Rust toolchain installed in the
image). Everything is built natively inside it with the unchanged tools, as
on the hosted Linux runner; the checkout is mounted at its host path and the
port's `build` and `target` directories are container volumes, so the macOS
builds of the same checkout stay as they are. What the emulation reports
and executes, checked with a small CPUID program in the container: vendor
`GenuineIntel`, leaf 1 with OSXSAVE, AVX, SSSE3 and SSE4.1, XCR0 = 7, leaf 7
with AVX2, BMI1 and BMI2 and without AVX-512F, and an AVX2 function
(`vpmulhrsw`, `vpackuswb`) gives its result; no `ROSETTA_ADVERTISE_AVX` is
needed in the Linux VM. dav1d's own detection agrees: at the AVX2 level both
libraries report `cpu_flags=0xf` (SSE2 to AVX2), which the per-kernel gate
requires. Before any x86 kernel (tree `697b963d`, with GCC 14 as `cc` in a
first form of the image): the
per-kernel gate at `avx2` (the original's assembly against the port's C)
and at scalar, 17 groups and 3,155 selections each at the eight seeds; whole
images 994 of 994 at `avx2` and at `scalar`; 1,711 of 1,711; API 83;
linkage pass. One round of all x86-64 gates takes about nine minutes on the
loaded host (per-kernel gate 50 to 75 s per level, whole images 60 to 90 s
per level, the original's suite 80 to 90 s, a build of the crate 15 to 30
s), the one-off build of the three originals two to three minutes.

What the loop cannot show: Rosetta does not fault on a misaligned aligned
move, so an alignment defect passes it (Valgrind, below, does fault on one);
there is no AVX-512, so the `avx512icl` assignments of the original are
never taken (the gate masks them off anyway); and no time measured under it
is a measurement of x86-64 hardware. The hosted Linux run on hardware stays
the cross-check for all three.

Instructions per call on x86-64: `measure_dav1d_kernels.py` counts with
Valgrind's Callgrind there (`Collected`: the x86-64 instructions the process
executes, call minus baseline as on macOS). Valgrind 3.24 runs under Rosetta
and executes the AVX2 code with its own translator, so the counts are exact
and independent of the load and of the emulator; they are instruction
counts of x86-64 code, not times. Under Valgrind the original's aligned
moves fault on misaligned data: the bench driver's CDF array and its loop
filter destination are now aligned as the decoder's are (the latter on
x86-64 only, so the NEON figures keep their inputs), and it passes the
length of the edge in units, which the x86 loop filter kernels count down
and the C and NEON kernels ignore. Per-decode instruction counts and times
of the x86-64 port against stock dav1d wait for a hosted benchmark.

**2026-11-17, msac on x86-64** (the eight entry points of `msac.asm`:
`simd/x86_64/msac.rs`; the header `src/x86/msac.h`: `simd/x86/msac.rs`;
`dispatch.txt`: `msac_x86`, all call sites of the `decode`, `recon_tmpl` and
`msac` units of the two Linux profiles). The dispatch tool now leaves string
literals alone: glibc's assertion strings in the Linux debug profile carry
the enclosing function's C signature, which read like a call (unit test
added).

| Check (x86-64 GNU Linux under Rosetta, Clang 19) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each, pass at the eleven seeds; msac's three groups compare the original's `_sse2` functions and its `_avx2` 16-symbol function with the rebuilt kernels |
| `check_dav1d_dsp.py --simd sse2` | the same counts; msac's groups with the `_sse2` 16-symbol function |
| `check_dav1d_dsp.py` (scalar) | the same counts |
| `check_dav1d_simd_original.py --level avx2` / `sse2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 12 entries, 60 units; 10 tests |
| unit tests (`cargo test --lib simd::`) | 4 of 4: context layout, the eight kernels against the C functions on streams of 128, 61, 9 and 5 bytes, the 16-symbol choice under three masks, `zeroed` |

Positive controls, one constant each, restored afterwards (the per-kernel
gate at the level named, then whole images where given):
`bool_equi`'s `v` (`mov t1b, 8` as 12) fails `msac.decode_bool` at every
seed at `avx2`, whole images 4 of 994 identical; `psllw 7` as 6 in the
4-symbol function fails `msac.decode_symbol` at every seed; the same in the
AVX2 16-symbol function fails `msac.decode_symbol` at every seed at `avx2`
and passes at `sse2` (which does not select it), whole images at `avx2` 3
of 994; the same in the second half of the SSE2 16-symbol function passes
at `avx2` and fails `msac.decode_symbol` at every seed at `sse2`, whole
images at `sse2` 33 of 994, which is what the level `sse2` is for;
`add t6d, 5` as 4 in `hi_tok` fails `msac.decode_hi_tok` at every seed.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family msac`, Callgrind, 4 million calls each, identical outputs; assembly
and rebuild): adapt4 69 and 76 (1.10), adapt8 72 and 78 (1.08), adapt16
AVX2 67 and 71 (1.06), adapt16 SSE2 88 and 95 (1.08), hi_tok 73 and 87
(1.19), bool_adapt 53 and 62 (1.17), bool_equi 34 and 39 (1.12), bool 36
and 39 (1.09). Seen in the disassembly: every kernel saves two or three
callee-saved registers (`push` and `pop`: 4 to 6 instructions), which the
refill near the end of the buffer needs; the assembly uses caller-saved
registers only. Moving that refill out of line was tried and made every
kernel 3 to 7 instructions longer (the values that live across the call
then sit in callee-saved registers), so it stays inline as in the assembly.
The 8- and 16-symbol SSE2 functions were at 1.18 and 1.16 with a plain load
of `pw_0xff00`: the compiler, knowing the constant, replaced `punpcklqdq`
and `pand` by a nine-instruction byte shuffle; a volatile load of the
constant keeps the assembly's memory operand (deviation (f)). The
16-symbol function's test of the CPU flags at the call site is not in these
figures.

Deviations from the assembly: (a) near the end of the buffer only
`buf_pos..buf_end` is read, as on NEON (memory safety; a few instructions
on a path taken at the end of a tile). (b) Every vector access to caller
memory is an unaligned move, where the assembly loads and stores the CDF of
the 8- and 16-symbol functions with `mova`: same bytes, same count. (c) The
stack buffers are zero-initialized arrays whose loaded elements are stored
a few lines above the load; the zero stores do not appear in the code. (d)
`bsr` and `tzcnt` operands are stated as non-zero values (`rng` is never 0,
the comparison mask always has the bit of the count's lane), which compiles
to the bare instructions. (e) The 16-symbol function is chosen at each call
(decision paragraph above). (f) `pw_0xff00` is loaded with a volatile read
in the SSE2 8- and 16-symbol functions (same value; 7 instructions per call
fewer in each).

Formatting: `rustfmt --check --config-path` with `max_width = 120` on the
three new files; `cargo fmt --all --check` passes without looking at them.
No `asm!` in the new files (the pattern is checked against a positive).
ARM64 after the change: `check_dav1d_dsp.py --simd neon` and scalar pass
at the eleven seeds, whole images 994 of 994 at `neon` and `scalar`, 1,711
of 1,711, ABI 12,157 checks, API, linkage and the dispatch check pass (the
gate tools changed; no file of the NEON kernels did).

**2026-11-17, loop filter on AVX2, 8 and 16 bpc** (`loopfilter_avx2.asm`:
`simd/x86_64/loopfilter_avx2.rs`; `loopfilter16_avx2.asm`:
`simd/x86_64/loopfilter16_avx2.rs`, used at 10 and 12 bit as in the
original; the header `src/x86/loopfilter.h`: `simd/x86/loopfilter.rs`;
`dispatch.txt`: `loopfilter_8bpc_x86`, `loopfilter_16bpc_x86`). The four
entry points of each file are assigned under `DAV1D_X86_CPU_FLAG_AVX2`, as
the original's init does after its SSSE3 test; its SSSE3 assignments (taken
only on a CPU without AVX2) and its AVX-512 assignments have no counterpart
(stated gap and scope of "Levels"), so below AVX2 the C kernels stay. The
macro `FILTER width, dir` is one function generic over the two arguments in
each file, the transposes are `macro_rules!`, the vector registers typed
locals. The two files were translated at the same time by two part agents,
each in a private copy of the crate and gated there against the assembly
(ten gate seeds and the seeds 1 to 200 each; part agents' runs), then
joined; every gate and control below was run on the joined state.

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each, pass; the four `loopfilter_8bpc` and the four `loopfilter_16bpc` groups (10 and 12 bit) compare the original's AVX2 assembly with the rebuilt kernels |
| `check_dav1d_dsp.py --simd sse2`, scalar | the same counts (the loop filter slots hold C there) |
| `check_dav1d_simd_original.py --level avx2` / `sse2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` | 14 entries, 60 units |
| unit tests (`cargo test --lib simd::`) | 7 of 7: also the context layout and, per bit depth, that AVX2 selects the rebuilt kernel and a mask below AVX2 keeps C |

Positive controls on the joined state, one constant each, restored
afterwards:
8 bpc: `paddsb m8, [pb_4]` with `pb_3` in the wd4 core fails all four
`loopfilter_8bpc` groups (both luma groups at every seed, `lpf_h_sb_uv` at
9 and `lpf_v_sb_uv` at 8 of the eleven), whole images at `avx2` 463 of 994
identical; the flat8out threshold `pb_129` as `pb_254` (wd16 only) fails
the two luma groups at 6 seeds and no chroma group; `pb_7_1` as `pb_3_1` in
the first flat16 stage (wd16 arithmetic) fails the two luma groups at seed
15 and at no other seed, which is the hole that seed closes; `pw_4096` as
`pw_2048` in the flat6 filter (chroma only) fails the two chroma groups at
9 seeds and no luma group. 16 bpc: `pw_4` as 5 in the wd4 core fails all
four `loopfilter_16bpc` groups (`lpf_h_sb_y` at every seed), whole images
758 of 994; `psllw m8, 3` as 2 in the flat16 filter fails the two luma
groups at 10 seeds and no chroma group; `pw_4096` as 2048 in the flat6
filter fails the two chroma groups at 9 seeds and no luma group.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family lpf`, Callgrind, one superblock edge of 32 units with wd4/8/16
mixed, 12,500 calls each, identical pictures; assembly and rebuild):
8 bpc row edges (`v`) 1,540 and 1,636 (1.06) for luma, 758 and
754 (1.00) for chroma, column edges (`h`) 2,309 and 2,553 (1.11) and 1,261
and 1,306 (1.04); 16 bpc (10-bit pixels) row edges 1,888 and 2,080 (1.10)
and 1,032 and 1,062 (1.03), column edges 2,881 and 2,984 (1.04) and 1,547
and 1,627 (1.05). None above 1.3. Causes, counted by the part agents in the
disassembly (not re-derived): the compiler moves more vectors to and from
the stack than the assembly's own frame use in the luma `h` filters; in
`lpf_v_sb_y` its loop strength reduction keeps the row addresses as
separate pointers, computed in a prologue of about 60 instructions and
reloaded from the stack, where the assembly forms them with `lea`; six
`push`/`pop` pairs and `vzeroupper` per call. The kernels contain no call
to an intrinsic and only VEX-encoded vector instructions (part agents'
scan of the disassembly).

Deviations from the assembly, and whether they change the count: (a)
every vector access to caller memory is an unaligned move where the
assembly uses `mova` (the pixel rows of the `v` filters at both bit depths,
the two 16-byte halves of each row of the 16 bpc wd16 `h` filter): same
bytes, same count; the assembly would fault on a destination that is not
aligned to 32, the rebuild accepts it. (b) The stack frames (`32 * 11` and
`32 * 21` bytes at 8 bpc, `32 * 5` and `32 * 15` at 16 bpc) hold spilled
registers and the transposed columns of the wd16 `h` filter; at 8 bpc each
slot is a local, at 16 bpc the slots are one array of fifteen vectors,
zero-initialized whole and indexed by constants only, and the registers
that only some widths load start as zero. No zero store appears in the
code. (c) At 8 bpc the constants of `SECTION_RODATA` are loaded with
volatile reads (`mem`), so that each use stays the memory operand the
assembly has: with plain constants the compiler broadcasts each into a
register per path, runs out of registers for pixels and rewrites constant
arithmetic, and the four kernels measure 1.15, 1.16, 1.12 and 1.11 (part
agent's measurement); same values. (d) `movhps` to and from pixel rows is
written with unaligned reads and writes (`vmovhps`, `vpextrq [mem], xm, 1`:
one instruction each): the intrinsics `_mm_loadh_pd`/`_mm_storeh_pd` go
through an aligned `f64` pointer, which a pixel row is not. (e) `.v4` and
`.h4`, which the assembly `call`s from the luma and the chroma function,
are functions the compiler inlines (16 bpc: both; 8 bpc: `v4`) or calls
(8 bpc `h4`). (f) `FILTER n, h` advances `dstq` and `lq` inside the macro;
the function returns the advanced pointers. Pointer arithmetic is wrapping
throughout, so a pointer the assembly leaves past the last access is never
claimed to be in bounds.

Reads and writes of the assembly beyond what the C kernels touch, kept as
the assembly (part agents' derivation from the code; the bounds are the
original's, not checked with guard pages): (1) a group is 32 bytes of a row along
the edge (eight units at 8 bpc, four at 16 bpc) and is loaded and stored
whole, with the widest filter any of its units asks for, whatever the
other units' mask bits are and also when `w` ends inside it; pixels that
are not filtered are written back as read. (2) The masks are read as
doublewords at a byte offset that advances by one per group at 8 bpc, up to
3 bytes into the word behind `vmask[2]` (`vmask[1]` for chroma); the caller
provides it (`lf_apply_tmpl` builds four and three words, the last one 0;
the translated unit has `[uint32_t; 4]` and `[uint32_t; 3]`). (3) The level
loads are 32 (8 bpc `v`), 16 (16 bpc `v`) or 8 (`h`) bytes from the
caller's pointer into the entries' second to fourth byte, so they end up to
3 bytes behind the last entry; the original allocates `lf.level` with 3
spare bytes for it, and so does the translation (`decode.rs`:
`.wrapping_add(3)`). (4) The `h` filters read and rewrite outer columns the
C filter does not touch (wd16: 8 pixels on either side; wd8: `p3` and `q3`
unchanged; wd6 reads one or two pixels more), and the 16 bpc wd4 `h` filter
loads 16 bytes for the rows 8 to 15 of a group, four pixels behind `q1`
("this overreads by 8 bytes but the buffers are padded", the assembly's
comment).

Formatting: `rustfmt --check --config-path` with `max_width = 120` on the
files of `simd/x86` and `simd/x86_64`; `cargo fmt --all --check` passes
without looking at them. No `asm!`. ARM64 after the change:
the per-kernel gate at `neon` and scalar at the eleven seeds, whole
images 994 of 994 at both levels, 1,711 of 1,711, ABI 12,157 checks, API,
linkage and the dispatch check pass, and `measure_dav1d_kernels.py` gives
the NEON figures of the msac and loop filter entries above.

Instructions per decode on x86-64 (Callgrind; the whole process of the
original CLI linked to each library, one thread, no output; the color item
of four of the profiling stills; stock dav1d at the AVX2 level, the port
now, and the port's portable build, which is the port before any x86
kernel): 8-bit 4:2:0 1.01 G, 4.21 G, 5.04 G; 8-bit 4:4:4 1.10, 5.78, 6.84;
10-bit 4:2:0 1.16, 4.86, 5.88; 10-bit 4:4:4 1.29, 6.39, 7.74. msac and the
loop filter take 15 to 17 % off; the port still executes 4.2 to 5.2 times
the instructions of stock dav1d on x86-64. Not measured: the lossless still
(too long under Callgrind), and any time.

On x86-64 the port now has msac and the loop filter; the inverse
transforms, CDEF, intra prediction and loop restoration still run C there
(amended below: the 8 bpc inverse transforms followed).

**2026-11-17, decisions on the first x86 kernels** (coordinator's decisions
on the open points of the two entries above; the tree after `8cef14a4`).

1. *msac's SSE2 16-symbol function and the gate level `sse2` are removed.*
   Engine ADR-0041 section 4 puts in scope what the original selects on an
   AVX2 machine; that function is selected only without AVX2, and a third
   gate level for one function cost a round of both gates every time. Gone
   are the rebuilt `dav1d_msac_decode_symbol_adapt16_sse2`, its test-only
   accessor, its case in the unit test and its rows in
   `measure_dav1d_kernels.py`, and the value `sse2` of `check_dav1d_dsp.py
   --simd` and `check_dav1d_simd_original.py --level`. On a CPU without AVX2
   the port's 16-symbol function is the translated C function, as every DSP
   init mirror keeps its C kernels there ("Levels"; the unit test decodes
   with the AVX2 choice, under a mask of SSE2 alone and under mask 0 and
   requires the same symbols). The original's msac test body assigns the
   SSE2 16-symbol function and, two statements later under
   `DAV1D_X86_CPU_FLAG_AVX2`, the AVX2 one; the per-kernel gate takes the
   first assignment out of its copy of the body (third x86 entry of
   `MSAC_SIMD_EDITS`), because the port has no function of that name to
   resolve. The gate fails unless both sides report AVX2, so the second
   assignment always replaces the first before anything is called: the
   compared functions are the same with and without the line. The positive
   control of the msac entry that only the level `sse2` caught (the fault in
   the SSE2 16-symbol function) has no object anymore.
2. *The volatile loads of constants are removed*; the constants are plain
   loads and the compiler does with them what it does. Cost in instructions
   per call against the assembly, re-measured (`measure_dav1d_kernels.py`,
   Callgrind, identical outputs; assembly and rebuild): the SSE2 8-symbol
   msac function 72 and 85 (1.18; 78 and 1.08 with the volatile load: the
   compiler turns `punpcklqdq` and `pand` with the known constant into a
   nine-instruction byte shuffle); the 8 bpc loop filter, row edges 1,540
   and 1,779 (1.16; was 1.06) for luma and 758 and 842 (1.11; was 1.00) for
   chroma, column edges 2,309 and 2,647 (1.15; was 1.11) and 1,261 and 1,414
   (1.12; was 1.04): the compiler broadcasts each constant into a register
   per path and has fewer registers left for pixels. All far below 1.3; a
   volatile read to steer the compiler is not worth carrying. The other
   rows are unchanged: adapt4 1.10, adapt16 AVX2 1.06, hi_tok 1.19,
   bool_adapt 1.17, bool_equi 1.12, bool 1.09; the 16 bpc loop filter
   1.04 and 1.05 (column edges), 1.10 and 1.03 (row edges).
3. *Kept as they are:* the 16-symbol function's choice at every call with
   the portable `MsacContext` layout, and the module layout
   `simd/x86/<family>.rs` + `simd/x86_64/<file>.rs` (both recorded in
   "Decision").

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each, pass |
| `check_dav1d_dsp.py` (scalar) | the same counts |
| `check_dav1d_simd_original.py --level avx2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 14 entries, 60 units; pass |
| unit tests (`cargo test --lib simd::`) | 7 of 7 |

ARM64 after the change (no file of the NEON kernels changed; the gate
tools did): the per-kernel gate at `neon` and scalar at the eleven seeds
(17 groups, 3,155 selections each), whole images 994 of 994 at both levels,
1,711 of 1,711, ABI 12,157 checks, API 83, linkage, the dispatch check and
`cargo fmt --all --check` pass.

**2026-11-17, inverse transforms 8 bpc on AVX2** (`itx_avx2.asm`: all
156 entry points `dav1d_inv_txfm_add_*_8bpc_avx2`, 5,542 lines). The third
x86 family, in `simd/x86_64/itx_avx2.rs`; `simd/x86/itx.rs` mirrors
`itx_dsp_init_x86` and assigns them under the AVX2 flag; the dispatch
entry `itx_8bpc_x86` calls it from the two Linux profiles of the 8 bpc
init. The module documentation has the correspondence with the assembly
and the deviations.

Scope on an AVX2 machine, from `src/x86/itx.h` preprocessed with assembly
on and its assignments evaluated for the flags SSE2 to AVX2: at 8 bits all
156 slots end with an AVX2 kernel (WHT included: its SSE2 assignment is
overwritten), at 10 bits all 156 (155 `_10bpc_avx2`, WHT `_16bpc_avx2`),
at 12 bits 148 (147 `_12bpc_avx2`, WHT), and the eight 12-bit DCT_DCT
slots of 16x32 and larger stay C. No slot keeps an SSE2, SSSE3 or SSE4.1
kernel on such a machine, so the lower levels add nothing to the scope
("Levels").

Method: the file was cut into five parts (4-point sizes with 4x16 and
16x4; 8x8, 8x16, 16x8; 16x16, 8x32, 32x8; 16x32, 32x16, 32x32; the
64-point sizes) after a shared prelude (constants, the 1-D macros, the
write macros, the DC-only tails); agents translated them in private copies
of the crate, one Rust line per instruction, gating each size with the
original's checkasm at the eleven seeds and seeds 1 to 100. A later part
uses an earlier part's items where the assembly jumps or calls into them;
the joined module has no copies. All figures below are from the joined
state in the worktree.

Deviations and their cost (the module documentation has the list): (a)
unaligned moves for caller memory where the assembly uses `mova`, 4-byte
rows as unaligned `i32` accesses: same bytes, same count. (b) The frames
of the 32- and 64-point sizes (`32*35` … `32*199` bytes) hold the
intermediate rows between the passes; which slots the second pass reads
depends on the eob path, so each frame is zeroed whole before its first
use (`zeroed_frame`): 16 to 199 32-byte stores per call, counted in the
figures below. The zero goes through `core::hint::black_box`: with
`util::zeroed` or an array initializer the compiler calls `memset` for the
larger frames, whose `rep stosb` Callgrind counts once per byte (64x32:
1.52 instead of 1.06, part agent's measurement). (c) Flag arithmetic on general-purpose registers
(`adc` on the eob, the `add …, 0x80000000 / jnc` loop counters, pointer
tests) is Rust on the same widths, and jumps between labels are loops and
branches in the assembly's order. (d) 16x64's `.main_part1` runs twice,
the second time with `r6` eight bytes on, which shifts each of its
`o(pw_…)` operands by two entries of the read-only data; the function
takes the offset and selects the constants per call (same count). (e) The
compiler rewrites some shuffles (16x4 identity_identity, 1.06) and gives
the DC-only paths of the 64-point sizes the large functions' prologue (a
stack probe above 4 KiB; up to 1.13).

Reads and writes beyond the block or the coefficients: none (each part's
derivation from the code, checked against the contract): pixel rows of 4,
8, 16 or 32 bytes inside the block, coefficient accesses inside
`min(w, 32) * min(h, 32)` values, the DC-only paths' 16-byte load and
4-byte store at `c`.

Positive controls on the joined state (one constant each; the per-kernel
gate at `avx2`, the eleven seeds): `pw_16384` as 12288 in the first pass
of 8x8 (`idct_8x8_internal_pass1`) fails `itx_8bpc.add_tx8` at every seed
and the whole-image gate (369 of 994 identical); 401 as 1401 in the
128-bit `IDCT16_1D_PACKED` (4x16, 16x4), 1321 as 2321 in 16x4's ADST
`.main`, and `pw_8192` as 4096 in 16x16's first pass each fail `add_tx16`;
1697 as 1597 in 16x32's identity pass fails `add_tx32`; 1751 as 1752 in
16x32's `.main_oddhalf` (shared with 32x16, 32x32 and the 64-point sizes)
fails `add_tx32` and `add_tx64`; the first `idct64_mul` entry 101 as -101
fails `add_tx64`. Restored byte for byte, the per-kernel gate and the
whole-image gate at `avx2` pass again. A change by one unit of the identity
and ADST constants was not detected by the part agents' runs at one or two
seeds (the test's small inputs rarely reach the rounding); the controls
above use larger changes.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family itx`, Callgrind, 1.6 million pixels per kernel; 175 rows: the
largest codable eob of each of the 156 entry points and the DC-only path
of each size's dct_dct; all outputs identical): 0.83 to 1.13, no row
above 1.3. Above 1.1: the DC-only paths of 16x64 (1.125) and 64x16
(1.123), which share the large functions' prologue and count their loop
one instruction longer per row; identity_identity of 32x32 (1.115) and
32x16 (1.106), the frame-free identity loops with a few more moves. The
full rows of the frame sizes include the zeroing (32x32 1.085, 64x64
1.077). Below 1.0 (down to 0.83) mostly the sizes up to 16x16 and the DC-only
paths, where everything is inlined into one function per entry point and
the assembly jumps and calls between passes (not analyzed further).

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each; the five `itx_8bpc` groups compare the assembly with the rebuilt kernels; pass |
| `check_dav1d_dsp.py` (scalar) | the same counts; pass |
| `check_dav1d_simd_original.py --level avx2` / `scalar` | 994 of 994 decodes identical each (the 987 of the corpus and the seven AV1 items of the six profiling stills given with `--image`) |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 15 entries; pass |
| unit tests (`cargo test --lib simd::`) | 9 of 9 (two new: the mirror's context layout and its selection under each mask) |
| `rustfmt --check` (`max_width = 120`) on `simd/x86`, `simd/x86_64`; no `asm!` | pass |

ARM64 after the change (no file of the NEON kernels changed): the
per-kernel gate at `neon` and scalar at the eleven seeds, whole images 994
of 994 at both levels, 1,711 of 1,711, ABI, API, linkage, the dispatch
check and `cargo fmt --all --check` pass.

Instructions per decode on x86-64 (Callgrind as in the loop filter entry;
stock dav1d at AVX2, the port before this entry, the port now): 8-bit
4:2:0 1.01 G, 4.21 G, 1.52 G (1.50 times stock); 8-bit 4:4:4 1.10, 5.78,
2.14 (1.94 times); 10-bit 4:2:0 1.16, 4.86, 4.85; 10-bit 4:4:4 1.29, 6.39,
6.38 (16 bpc still runs C). The fifth still is the color item of the
8-bit 4:2:0 still with alpha (the same counts). Not measured: the lossless
still (too long under Callgrind), and any time.

16 bpc (10 and 12 bits, `itx16_avx2.asm`, 8,599 lines): the next entry.

**2026-11-17, inverse transforms 16 bpc on AVX2** (`itx16_avx2.asm`:
the 155 `dav1d_inv_txfm_add_*_10bpc_avx2`, the 147 `*_12bpc_avx2` and
WHT 4x4 `_16bpc_avx2`, 8,599 lines). The fourth x86 family, in
`simd/x86_64/itx16_avx2.rs`; `simd/x86/itx.rs` mirrors `itx_dsp_init_x86`
at 16 bpc and assigns them under the AVX2 flag (156 slots at 10 bits, 148
at 12 bits: the eight 12-bit DCT_DCT slots of 16x32 and larger keep C, as
in the original; `all_simd` follows the 10-bit assignments, the original's
value on an AVX2 machine); the dispatch entry `itx_16bpc_x86` calls it
from the two Linux profiles of the 16 bpc init. The module documentation
has the correspondence with the assembly and the deviations. With this
entry every slot the original gives an AVX2 kernel at 8, 10 and 12 bits
has its rebuilt kernel.

Method as for 8 bpc: a shared prelude (the constants, `ITX_MULSUB_2D` with
the packed coefficient pairs, the 4-point macros at both widths,
`TRANSPOSE_8X8_DWORD`, the DC-only tails, WHT as the gated pilot), then
eight parts (4x4 and 4x8; 4x16 and 8x4; 8x8 and 8x16; 16x4 and 16x8;
16x16; 8x32 and 32x8; 16x32, 32x16 and 32x32; the 64-point sizes)
translated by agents in private copies of the crate, one Rust line per
instruction, each size gated with the original's checkasm (`itx_16bpc`,
10 and 12 bits) at the eleven seeds and seeds 1 to 100. The assembly
calls 25 labels of the 8 bpc file (`cextern …_8bpc_avx2.main`); those are
the items of `itx_avx2.rs` (made `pub(crate)`, otherwise unchanged), each
checked against the label's code by the part that calls it. The joined
module has no duplicate item and no `call` in an entry point. All figures
below are from the joined state in the worktree.

Deviations and their cost (the module documentation has the list): (a)
unaligned moves for caller memory, as for 8 bpc. (b) The frames of 16x16,
the 32- and 64-point sizes and the 12-bit 8x32 and 32x8 (`32*24` …
`32*195` bytes) are zeroed whole before their first use (`zeroed_frame`,
24 to 195 32-byte stores per call, counted below); the small frames whose
reads all follow a write of the same pass (16x8's and the 12-bit 8x16's
`32*8`, 8x32's 10-bit `32*12`) are zero-initialized arrays or written
whole first, as said at each site. (c) Flag arithmetic and jumps between
labels as for 8 bpc. (d) The compiler rewrites two `vbroadcasti128` and a
`shufpd` into five instructions (16x4's DCT and 4x16's ADST first passes,
+2 per `shufpd`) and a few shuffles in 4x4 identity.

Reads and writes beyond the block or the coefficients: none (each part's
derivation from the code; several sizes use the coefficient buffer as
scratch, as the assembly does, and clear it again).

Positive controls on the joined state (one constant each; the per-kernel
gate at `avx2`, the eleven seeds; `fault-itx16.py` in the scratch area),
at least one 10-bit and one 12-bit constant per size class:
`pd_5793` as 4793 in 4x4's 10-bit identity
first pass fails `itx_16bpc.add_tx4` at every seed and the whole-image
gate (873 of 994 identical); `pd_1697` as 697 in 4x4's 12-bit identity
first pass fails `add_tx4`; `pw_2048` as 2047 in 8x8's 10-bit second pass
fails `add_tx8`; `pd_17408` as 16384 in 8x8's 12-bit ADST `.pass2_main2`
(also run by 16x8) fails `add_tx8` and `add_tx16`; 4091 as 4092 in 16x8's
10-bit ADST `.main_part2` fails `add_tx16`; the shift 8 as 7 in 16x16's
12-bit `.pass2_main` fails `add_tx16`; `pw_2048` as 2047 in 16x32's
`.pass2_end` (shared with the 64-point sizes) fails `add_tx32` and
`add_tx64`; `pd_8` (a shift of 8) as 9 in 8x32's 12-bit dct_dct and
`pixel_12bpc_max` as 0xffe in 8x32's 12-bit identity each fail `add_tx32`;
`pd_1567` as 1568 in 16x64's `.main_part2` fails `add_tx64`. Restored
byte for byte, the per-kernel gate and the whole-image gate at `avx2` pass
again. As for 8 bpc, the part agents found that a change by one unit of a
rounding offset or an identity constant often goes undetected at one seed
(the test's inputs rarely reach the rounding); the controls above use
larger changes.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family itx16` and `--family itx12`, Callgrind, 1.6 million pixels per
kernel; the largest codable eob of each entry point and the DC-only path
of each dct_dct; all outputs identical): 10 bits, 175 rows, 0.79 to 1.105
(median 0.95); 12 bits, 159 rows, 0.79 to 1.068 (median 0.96); no row
above 1.3, one above 1.1 (the DC-only path of 64x32, 1.105, which shares
the large function's prologue). The 64-point full rows include the frame
zeroing (16x64 1.089, 32x64 1.083, 64x64 1.052). The highest 12-bit rows:
16x8 identity_dct 1.068, the 16x16 ADST pairs 1.06, 8x32 and 32x8
identity_identity 1.06. Below 1.0 (129 of 175 and 118 of 159 rows; down to
0.79) mostly the sizes up to 32x32 and the DC-only paths, where everything
is inlined into one function per entry point and the assembly jumps and
calls between passes (not analyzed further).

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each; the five `itx_8bpc` and the five `itx_16bpc` groups compare the assembly with the rebuilt kernels; pass |
| `check_dav1d_dsp.py` (scalar) | the same counts; pass |
| `check_dav1d_simd_original.py --level avx2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 16 entries; pass |
| unit tests (`cargo test --lib simd::`) | 11 of 11 (two new: the 16 bpc context layout and the selection under each mask at 10 and 12 bits) |
| `rustfmt --check` (`max_width = 120`) on `simd/x86`, `simd/x86_64`; no `asm!` | pass |

ARM64 after the change (no file of the NEON kernels changed): the
per-kernel gate at `neon` and scalar at the eleven seeds, whole images 994
of 994 at both levels, 1,711 of 1,711, the unit tests, ABI, API, linkage,
the dispatch check and `cargo fmt --all --check` pass.

Instructions per decode on x86-64 (Callgrind as before; stock dav1d at
AVX2, the port before this entry, the port now): 10-bit 4:2:0 1.16 G,
4.85 G, 1.75 G (1.51 times stock); 10-bit 4:4:4 1.29, 6.38, 2.62 (2.04
times). 8-bit unchanged (1.52 G and 2.14 G). Not measured: the lossless
still, and any time.

**2026-11-17/18, which family next on x86-64.** Instructions per decode by
DSP family (Callgrind, the whole process of the original CLI linked to
each library, one thread, the color item of four profiling stills; for
each family the inclusive cost of every call from outside the family into
it, which counts nested helpers once and not the frame-level C code both
libraries share; the original's assembly has no symbol sizes, so its
figures are the self cost of the addresses each family's symbols cover),
port with msac, loop filter and inverse transforms rebuilt, against stock
dav1d at AVX2:

| Family | 8-bit 4:2:0 | 8-bit 4:4:4 | 10-bit 4:2:0 | 10-bit 4:4:4 |
| --- | --- | --- | --- | --- |
| CDEF (C in the port) | 247.8 M / 12.3 M | 745.3 M / 29.9 M | 287.2 M / 15.7 M | 991.9 M / 41.6 M |
| intra prediction (C) | 142.6 M / 12.2 M | 184.0 M / 14.3 M | 181.9 M / 18.7 M | 245.0 M / 22.2 M |
| msac | 430.3 M / 381.0 M | 439.5 M / 389.2 M | 422.8 M / 373.1 M | 431.5 M / 380.9 M |
| inverse transforms | 58.5 M / 56.2 M | 98.5 M / 92.8 M | 66.6 M / 67.2 M | 95.2 M / 94.5 M |
| loop filter | 49.2 M / 40.1 M | 64.2 M / 52.4 M | 67.6 M / 60.4 M | 97.9 M / 87.7 M |
| whole decode | 1,515 M / 1,009 M | 2,138 M / 1,105 M | 1,752 M / 1,159 M | 2,623 M / 1,286 M |

Loop restoration, motion compensation, film grain, palette index packing
and `refmvs` are not called on these stills (no instruction in either
library). CDEF carries the largest excess (C 20 to 25 times the
assembly), intra prediction the next (8 to 13 times); both followed, in
that order.

**2026-11-18, CDEF on AVX2, 8 and 16 bpc** (`cdef_avx2.asm`: the
filters `dav1d_cdef_filter_{8x8,4x8,4x4}_8bpc_avx2` and
`dav1d_cdef_dir_8bpc_avx2`, 1,771 lines; `cdef16_avx2.asm`: the same at
`_16bpc`, used at 10 and 12 bit, 877 lines). The fifth x86 family, in
`simd/x86_64/cdef_avx2.rs` and `cdef16_avx2.rs`; `simd/x86/cdef.rs`
mirrors `cdef_dsp_init_x86` and assigns the four slots under the AVX2
flag at both bit depths (on x86 the original assigns the assembly
directly: each kernel pads the block itself, there is no wrapper as on
ARM); the dispatch entries `cdef_8bpc_x86` and `cdef_16bpc_x86` call it
from the two Linux profiles. On an AVX2 machine the original's AVX2 block
assigns every slot its SSE2 (8 bpc), SSSE3 and SSE4.1 blocks assign, so
no lower level is left in scope ("Levels"). The module documentation has
the correspondence with the assembly, the deviations and the reads.

Method as for the inverse transforms: a prelude (the register file, the
8 bpc `SECTION_RODATA` as one byte array with an offset per label, taken
from the original's object file, because the assembly reads across its
labels), then three parts translated by agents in private copies (the
8 bpc fast path, `edges == 0xf`, with the sixteen tap loaders the
assembly `call`s through a jump table; the 8 bpc border path and
`cdef_dir` with its `.main`, which `cdef_dir_16bpc` jumps into; the
16 bpc file), each gated with the original's checkasm (`cdef_8bpc`,
`cdef_16bpc`) at the eleven seeds and seeds 1 to 100. Until a part was
written its entry points ran the port's C kernel (the assembly equals C
on the test's inputs), so every partial state was gated; those fallbacks
are gone from the joined modules.

Deviations and their cost (the module documentation has the list): (a)
the fast path's jump table is resolved at compile time: the entry point
selects one of eight bodies by the direction, in which each table index is
a constant, so the loaders' code stands in place of the computed
`call`/`ret` (with a run-time choice per tap the compiler carried pixels
between taps through general-purpose registers and stack slots: 1.68,
1.32, 1.67 times the assembly); larger code. (b) The 8 bpc fast path
reads `left` at offsets -16 to +20, beyond the `h` rows of two pixels the
caller passes; in the decoder they are `lr_bak[bit][pl]`, rows inside a
stack array of 96 bytes, so the assembly reads before its start or behind
its end for the first and the last plane, and the blends discard those
bytes. The rebuild copies exactly the `h` rows into a zeroed local with
16 bytes before and 32 behind them and runs the assembly's accesses on it
(memory safety, as msac's deviation (a); about five instructions). (c) The
frames are locals zeroed whole: the loaders' blends and the 16 bpc 4-pixel
loads read frame bytes the assembly never writes (or bytes just past its
frame) and discard them, and the 8 bpc border padding writes every
element of `px` the filter reads only by a case analysis over the edge
flags (10 to 14 stores per call at 8 bpc, 5, 7 and 13 at 16 bpc). (d)
`lzcnt`, `tzcnt`, `rorx` are `leading_zeros`, `trailing_zeros`,
`rotate_right` (no BMI; one instruction more each). (e) The shift counts
and the 8x8 copy of `bot` that the assembly keeps in its frame are locals;
unaligned moves for caller memory where the assembly uses `mova`.

Reads beyond what the C kernel reads (rows -2..h+1, columns -2..w+1, each
neighbor only with its edge flag), kept as the assembly makes them on the
original's buffers (derived from the code, not checked with guard pages):
the 8 bpc fast path reads block rows from column -2 (4x4: -4) to 17 and,
at 4x4, the rows below from column -20 and the rows above up to column 29;
the border paths read 16 bytes of a row (8 at w 4) with
`CDEF_HAVE_RIGHT` and the rows above and below from column -8 to 15
depending on the flags, the 16 bpc filters 8 or 16 pixels of each row;
the picture rows are the width rounded up to 128 pixels and the CDEF line
buffers have 32 bytes before and behind each row, in the original and the
translation alike. Writes: exactly the block.

Positive controls on the joined state (one constant each; the per-kernel
gate at `avx2`, the eleven seeds; `fault-cdef.py` in the scratch area):
8 bpc fast path, `pri_taps` (`tap_table+8+…`) two bytes on in
`.pri_only`, fails `cdef_8bpc.cdef_filter` at every seed and the
whole-image gate (460 of 994 identical); the fast path's `sec_taps`
(`tap_table+12`) one byte on in `.sec_only` fails `cdef_filter` at every
seed; the 8 bpc border path's `pri_taps` one byte on fails `cdef_filter`
at every seed; `cdef_dir`'s final `psrld xm2, 10` as 9 (in `.main`, which
`cdef_dir_16bpc` shares) fails `cdef_8bpc.cdef_dir` at every seed and
`cdef_16bpc.cdef_dir` at 7; 16 bpc: the primary tap offset `[dirq+5]` as
`+4` fails `cdef_16bpc.cdef_filter` at every seed and the whole-image gate
(812 of 994), the secondary offset `[dirq+9]` as `+8` and the right
padding of 8x8 (`[px+32*x+16]` as `+20`) each fail `cdef_filter` at
every seed, and `dir_shift`'s 12-bit entry as 0x0800 fails
`cdef_16bpc.cdef_dir` at 7 of 11 seeds, the seeds whose single direction
test draws 12 bit. Weaker controls, recorded: one tap loader's `bot`
offset (`.d7k1`, 4x4) failed at 4 seeds; `psrld 10` as 11 at 10 of 11 and
`div_table+44` as `+40` at 10 of 11 (8 bpc `cdef_dir`): the test runs the
direction search once per seed on one block, where a change can leave
the result. Restored byte for byte, the per-kernel gate passes again.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family cdef`, Callgrind, 10,000 calls per row; the filters over the
edge sets 0xf (the 8 bpc fast path), 0x0, 0x5, 0xa, the three strength
kinds and the eight directions in turn; all outputs identical): 8 bpc
`dir` 173.0 and 172.0 (0.994); 8x8 0.992 to 1.017, 4x8 1.023 to 1.061,
4x4 1.018 (fast path) to 1.166 (border, edges 0x0: the frame's ten zero
stores, `bsr`/`xor` for `lzcnt`, the compiler forwarding padded rows
through general-purpose registers); 16 bpc `dir` 0.988, 8x8 1.043 to
1.068, 4x8 1.039 to 1.080, 4x4 1.021 to 1.085. No row above 1.3.

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each; the `cdef_8bpc` and `cdef_16bpc` groups compare the assembly with the rebuilt kernels; pass |
| `check_dav1d_dsp.py` (scalar) | the same counts; pass |
| `check_dav1d_simd_original.py --level avx2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 18 entries, 60 units; pass |
| unit tests (`cargo test --lib simd::`) | 13 of 13 (two new: the CDEF context layout and the selection under each mask at 8 and 16 bpc) |
| `rustfmt --check` (`max_width = 120`) on `simd/x86`, `simd/x86_64`; no `asm!` | pass |

ARM64 after the change (no file of the NEON kernels changed): the
per-kernel gate at `neon` and scalar at the eleven seeds, whole images 994
of 994 at both levels, 1,711 of 1,711, 20 unit tests, ABI 12,157 checks,
API, linkage, the dispatch check and `cargo fmt --all --check` pass.

Instructions per decode on x86-64 (Callgrind as before; stock dav1d at
AVX2, the port before this entry, the port now): 8-bit 4:2:0 1.01 G,
1.52 G, 1.28 G (1.27 times stock); 8-bit 4:4:4 1.10, 2.14, 1.42 (1.29
times); 10-bit 4:2:0 1.16, 1.75, 1.48 (1.28 times); 10-bit 4:4:4 1.29,
2.62, 1.67 (1.30 times). Not measured: the lossless still, and any time.
Intra prediction is now the largest family still in C (143 M to 245 M per
decode against 12 M to 22 M): the next entry.

Seen while checking the rebuilt kernels for calls (`objdump` of the x86-64
library, every `call` inside an AVX2 entry point): the CDEF kernels have
none, but seven inverse-transform entry points call `memset` (8 bpc
`dct_dct` 32x16; 10-bit `dct_dct` 16x64, 32x32, 32x64, 64x16, 64x32 and
64x64), contrary to "no `call` in an entry point" in the two entries
above; the loop filter's `h4` call is the one recorded there. Not changed
here; the per-call figures of those entries include the calls.

**2026-11-18, intra prediction on AVX2, 8 and 16 bpc** (`ipred_avx2.asm`,
5,362 lines, and `ipred16_avx2.asm`, used at 10 and 12 bit, 4,987 lines:
the 22 entry points per file that `intra_pred_dsp_init_x86` assigns, the
fourteen `intra_pred` modes, the four `cfl_pred` modes, the three `cfl_ac`
layouts and `pal_pred`). The sixth x86 family, in
`simd/x86_64/ipred_avx2.rs` and `ipred16_avx2.rs`; `simd/x86/ipred.rs`
mirrors `intra_pred_dsp_init_x86` and assigns all 22 slots under the AVX2
flag at both bit depths (on x86 the original assigns the assembly
directly; the directional predictors prepare their edges themselves,
there is no C wrapper as on ARM); the dispatch entries `ipred_8bpc_x86`
and `ipred_16bpc_x86` call it from the two Linux profiles. The AVX2 block
of the header assigns every slot its SSSE3 block assigns, so no lower
level is left in scope ("Levels"). With this entry every slot the original
gives an AVX2 kernel in the families the stills reach (CDEF, intra
prediction, inverse transforms, loop filter, msac) has its rebuilt kernel
on x86-64; loop restoration, motion compensation, film grain, palette
index packing and `refmvs` are not called on the measured stills and stay
C on x86-64.

Method as for CDEF: a prelude per file (the register file and
`SECTION_RODATA` up to the jump tables as one byte array with an offset
per label, from the original's object file), thirteen parts translated by
agents in private copies (8 bpc: the dc family with v, h, paeth and
pal_pred; smooth; z1; z2; z3; filter with cfl; cfl_ac; 16 bpc: the same,
with smooth and cfl_ac in one part), stubs running the port's C kernel
until a part was written, each part gated with the original's checkasm
(`ipred_8bpc`, `ipred_16bpc`) at the eleven seeds and seeds 1 to 100.
No part depends on another; the joined modules have no fallback left and
no `call` in an entry point.

Deviations and their cost (the module documentation has the list, each
section its own): (a) `tzcnt`, `lzcnt`, `popcnt`, `shrx`/`shlx`/`sarx`
and `rorx` are Rust operations: the original selects these kernels on its
AVX2 flag (AVX2, BMI1, BMI2; POPCNT is not tested), the rebuild enables no
feature beyond AVX2, and `popcnt` (the edge filter strength of z1–z3)
becomes a software count of about 12 instructions per use. (b) The jump
tables are `match`es; indices outside the contract go to the last arm
(dc/h/v/paeth/pal_pred at 8 bpc and smooth_v at 16 bpc:
`unreachable_unchecked`, as the
assembly's jump beyond its table is undefined). (c) Frames are locals
zeroed whole or written whole before the first load; the z1, z2 and z3
frames are larger than the assembly's, whose main loops load lanes from
beyond the frame (below `rsp`, above the frame) and blend them away. (d)
`core::hint::black_box` keeps a value opaque where the compiler rewrote
the assembly's instruction pattern into a longer one: `stride*3` (dc
family, v, 16 bpc smooth_v), `top` in 8 bpc paeth, the read-only base
(smooth; the compiler otherwise folds the `pshufb` masks and expands each
broadcast and shuffle), the `psrlw 8` count in 8 bpc smooth, the edge
pointer, the shuffle masks and the x positions at the head of the z2
loops (its loop strength reduction otherwise splits the assembly's `lea`
chain into induction variables, runs out of registers and spills), `dst`
in z3's loops; empty `black_box(())` barriers keep it from vectorizing
z3's end loops and make it reload z2's frame slots where the assembly
does. One store and load each, or nothing. (e) The 16 bpc z3 entry point
jumps (a tail call) to one `#[target_feature(enable = "avx2")]` function
per height, so that the small heights do not carry the 12 KiB column
buffers of h32 and h64 in their frame (two 4 KiB stack probes and six
callee-saved registers per call before). (f) Unaligned moves for caller
memory.

Reads beyond what the C kernels read, kept as the assembly makes them
(derived from the code by the parts, not checked with guard pages; the
values never reach the output): the edge array from `topleft[-191]` (z3
at 64 rows) to `topleft[198]` (8 bpc z1 at 64 columns; 16 bpc `[191]`),
and at 8 bpc smooth through the inter-intra edge (`edge + 32`) 32 bytes
before the array; in the decoder these bytes are inside the task
context's `scratch` union (test `the_edge_array_has_readable_slack_around_it`
in `simd/x86/ipred.rs`), in the original's checkasm test outside its
257-pixel stack array for the original and the port alike; `cfl_ac`
loads 16 or 32 bytes of luma rows where C reads 8 or 16, inside the rows'
stride. `tools/dav1d-kernel-bench.c` now has 64 pixels of zeros on either
side of its edge array. Writes: exactly the block (`cfl_ac`: exactly
`cw*ch` values).

Positive controls on the joined state (one constant each, `fault-ipred.py`
in the scratch area; the original's checkasm `ipred_8bpc`/`ipred_16bpc`
against the joined module at the eleven seeds; the first two also through
the per-kernel gate and the whole-image gate on the worktree): 8 bpc dc
(`psrlw 3` as 2 in `.w4`; whole images 349 of 994 identical), paeth
(`pb_1` as `pb_3`), pal_pred (`psrlw 4` as 3), smooth_v (`pshufd q1111` as
`q1110`), z1 edge filter (`z_filter_s` one byte on) and main loop
(`pb_1to32` one byte on), z2 main loop (`lea r5, [rsp+60]` as 59), z3 edge
filter (`z_filter_k` one row back) and main loop (`pb_32` four bytes
back), filter (`vpblendd 0x0f` as 0x0e), cfl_left (`0x8000` as 0x7000),
cfl_ac 420 (`pb_2` as `pb_4`) and 444 (`q3333` as `q3232`); 16 bpc dc,
dc_128 at 12 bit (`shr 11` as 12), paeth, pal_pred, smooth (`psrld 8` as 7
in `SMOOTH_2D_END`; whole images 724 of 994), cfl_ac (`.dc`'s `sub 2` as
1), z1 edge filter (`pw_3` as `pw_1`), z2 main loop (`z_base_inc+2` as
+4), z3 main loop (`psllw 9` as 8), filter (`mov r7d, 4` as 3), cfl
(`0x6667` as 0x6567): each fails its group at every seed; a 12-bit-only
z3 control (the pixel maximum of the h4 upsampling masked to 0x7ff) fails
at 6 of 11, the seeds that reach that path at 12 bit. The z2, z3 (16 bpc)
and smooth (16 bpc) controls were re-run after the instruction work below.
Restored byte for byte, the gates pass again.

Instructions per call against the assembly (`measure_dav1d_kernels.py
--family ipred`, Callgrind, at least 2,000 calls per row; every slot over
the block sizes, angles, flags, filters, alphas and paddings of the
original's checkasm test; 499 rows per bit depth, all outputs identical):
8 bpc 0.971 to 1.308 (median 1.16), 16 bpc 1.028 to 1.307 (median 1.18).
One row per bit depth is above 1.3: z1 4x8 with the edge filter (1.308
and 1.307), whose software population count adds about 12 instructions
to a call of about 200. The first joined state had 22 rows above 1.3 at
8 bpc (z1 one, z2 21 up to 1.404) and 43 at 16 bpc (z1 one, z2 8 up to
1.377, z3 20 up to 1.505, smooth_v and smooth_h 14 up to 1.466); deviations
(d) and (e) brought them down. The highest classes now: z2 up to 1.27 and
1.28, z3 at 16 bpc up to 1.30, the smooth predictors and paeth up to 1.29
at 8 bpc, the dc family up to 1.27 at 16 bpc. Causes counted by the parts:
loop bookkeeping the compiler adds (`add`/`test`/`jg` for `sub`/`jg`, a
second induction variable), frame zeroing, `tzcnt` and `popcnt` without
BMI and POPCNT. Not analyzed further.

| Check (x86-64 GNU Linux under Rosetta, Clang 19, eleven seeds) | Result |
| --- | --- |
| `check_dav1d_dsp.py --simd avx2` | 17 groups, 3,155 selections each; the four `ipred_8bpc` and four `ipred_16bpc` groups compare the assembly with the rebuilt kernels; pass |
| `check_dav1d_dsp.py` (scalar) | the same counts; pass |
| `check_dav1d_simd_original.py --level avx2` / `scalar` | 994 of 994 decodes identical each |
| `run_dav1d_tests.py` | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,284 layout checks, 0 differences; 83 API comparisons; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 20 entries, 60 units; pass |
| unit tests (`cargo test --lib simd::`) | 16 of 16 (three new: the context layout, the selection under each mask at 8 and 16 bpc, the edge array's slack) |
| `rustfmt --check` (`max_width = 120`) on `simd/x86`, `simd/x86_64`; no `asm!`; no `call` in an AVX2 ipred entry point | pass |

ARM64 after the change (no file of the NEON kernels changed): the
per-kernel gate at `neon` and scalar at the eleven seeds, whole images 994
of 994 at both levels, 1,711 of 1,711, 20 unit tests, ABI 12,157 checks,
API, linkage, the dispatch check and `cargo fmt --all --check` pass.

Instructions per decode on x86-64 (Callgrind as before; stock dav1d at
AVX2, the port after CDEF, the port now): 8-bit 4:2:0 1.01 G, 1.28 G,
1.15 G (1.14 times stock); 8-bit 4:4:4 1.10, 1.42, 1.26 (1.14 times);
10-bit 4:2:0 1.16, 1.48, 1.32 (1.14 times); 10-bit 4:4:4 1.29, 1.67, 1.46
(1.13 times). By family (inclusive, as in the family table) the port is
now at 0.99 to 1.05 times stock dav1d for CDEF, 1.22 to 1.30 for intra
prediction, 0.99 to 1.06 for the inverse transforms, 1.12 to 1.23 for the
loop filter and 1.13 for msac; the remainder is
C code both libraries share (`decode_coefs` 394 M against 353 M at 8-bit
4:2:0, the translated bit-field accessors, `dav1d_prepare_intra_edges`).
Not measured: the lossless still, and any time. (The CDEF entry's
instructions per call were measured on the joined module in a
development copy, the same code before formatting, not on the committed
tree itself.)

**2026-11-18/19, the compiler tricks removed** (owner principle of
2026-11-18, decision paragraph "Plain Rust, no codegen steering" above;
branch `v1/dav1d-no-tricks` from `b5f4f931`). Removed: the 27
`core::hint::black_box` sites of the AVX2 intra prediction (deviation (d)
of that entry) and `zeroed_frame` of the AVX2 inverse transforms; the 34
calls of the hand-written zeroing helpers (`util::zeroed` on both
architectures, `zeroed_scratch`; `simd/x86_64/util.rs` is gone), now
zero-initialized arrays; 64 `#[inline(never)]` (ARM64 intra prediction 37,
ARM64 16 bpc inverse transforms 10, the Wiener row functions 12, the AVX2
16 bpc z3 per-height functions 5, which are one entry point again,
deviation (e)); the eight bodies of the AVX2 CDEF fast path (deviation
(a) of the CDEF entry: the tap table is a `match` on the run-time index);
the 36 `unreachable_unchecked` jump-table defaults (`unreachable!()`) and
the four `NonZeroU32::new_unchecked` (plain `trailing_zeros` /
`leading_zeros`); `wiener_post_inc` with its exposed provenance (the 16
bpc Wiener row loops index their rows, 24 wrapping pointer steps fewer).
Kept: the `#[inline(never)]` of `get_filter_strength` in
`simd/arm/ipred.rs`, which mirrors the C original's `NOINLINE`.

The reads behind or before the caller's edge array (the NEON Z1 edge
filter, the AVX2 z1 and z3 at both bit depths, the 8 bpc AVX2 smooth and
smooth_h from 16 columns on an inter-intra edge) now go to a
zero-initialized local copy of the pixels the C kernel reads
(`crate::simd::copy_edge`); the translated `Dav1dTaskContext` is
unchanged. The layout tests `the_edge_array_has_readable_slack_*` are
replaced by `the_kernels_read_only_the_edge_array` (both architectures):
every intra prediction slot at 8, 10 and 12 bit and every block size, Z1
to Z3 over 15 angles with the flags 0, 0x400 and 0x600, the 257-pixel
array placed directly after and directly before a page without access
(`topleft` at 128, and at 32 for the inter-intra modes up to 32x32); a
load outside the array faults. Each block is compared with the C
kernel's: on ARM64 for every slot, on x86-64 for z1, z3, smooth and
smooth_h (the AVX2 z2, untouched apart from its hints and equal to the
assembly at the gate, differs from C at 4x4, angle 147 with the edge
filter, `max_width` = `max_height` = 4; not investigated). Positive
control: with the NEON copy taken out the test dies with SIGBUS.

| Check | ARM64 (M1 Pro) | x86-64 (Rosetta container, Clang 19) |
| --- | --- | --- |
| `check_dav1d_dsp.py --simd neon\|avx2` and scalar, eleven seeds | 17 groups, 3,155 selections each, pass | the same, pass |
| `check_dav1d_simd_original.py` at the SIMD level and `scalar` | 994 of 994 (six stills) and 987 of 987 each | 994 of 994 and 987 of 987 each |
| `run_dav1d_tests.py` | 1,711 of 1,711 | 1,711 of 1,711 |
| ABI, API, linkage | 12,157 layout checks, 0 differences; 83; pass | 12,284, 0 differences; 83; pass |
| `enable_dav1d_dispatch.py --check` and its unit test | 20 entries, 60 units; pass | the same |
| unit tests (`cargo test --lib simd::`) | 20 of 20 | 15 of 15 |
| `cargo fmt --all --check`; `rustfmt --check` (`max_width = 120`); no `asm!` | pass | — |

Instructions per call against the assembly, before and after
(`measure_dav1d_kernels.py`, the same rows; all outputs identical): the
per-row ratio after/before has a median of 1.00 in every family but
intra prediction and loop restoration; the changes are the cost of the
straight code. x86-64 (Callgrind): intra prediction 1,036 rows, median
1.05, 513 rows above 1.05, 130 above 1.2, the largest 8 bpc smooth 64x64
2,612 to 5,742 (2.20; the read-only base and the `psrlw 8` count no
longer hidden, and the edge copy from 16 columns) and 16 bpc z3 4x4
122 to 240 (1.97; the edge copy and the joined height paths); against
the assembly 0.99 to 2.39, median 1.22 (before 0.97 to 1.31, median
1.16), 289 rows above 1.3 (before 2). Inverse transforms: median 1.00;
the frames of 32x64, 64x32 and 64x64 (8 bpc) and 16x64, 32x32, 32x64,
64x32, 64x64 (10 bit) are now a `memset` call, whose `rep stosb`
Callgrind counts per byte: 1.32 to 1.74 (16x64 10 bit 3,957 to 6,876);
12 bit at most 1.23. CDEF: 4x8 and 4x4 fast path 1.27 and 1.26 (the
run-time tap table), the rest within 1.14. msac: 1.00 to 1.03 (the zero
case of `trailing_zeros`/`leading_zeros`). ARM64 (instructions
retired): intra prediction 1,036 rows, median 1.01, 39 rows above 1.2,
the largest the 64-wide 16 bpc DC, V and dc_top/left rows (16 bpc
`dc_128` 64x64 198 to 362, 1.83: the 64-column store loop inlined, so
the compiler forms the second half's addresses with adds), Z1 at 16 bpc
up to 1.34 (the edge copy); loop restoration median 1.04, at most 1.19
(self-guided 16x16, the row buffers are now a `bzero` call), 16 bpc
Wiener up to 1.15 (indexed rows); inverse transforms 16 bpc median 1.00,
the DC-only paths of the 32- and 64-point sizes up to 1.22 (the split
coefficient paths inlined). No ratio is a target (decision above).

Instructions per decode (the color items of four profiling stills, one
thread, whole process; original with assembly / port before / port
after): ARM64 8-bit 4:2:0 0.990 / 1.111 / 1.112 G, 8-bit 4:4:4 1.115 /
1.258 / 1.259, 10-bit 4:2:0 1.156 / 1.288 / 1.292, 10-bit 4:4:4 1.345 /
1.476 / 1.478 (+0.1 to +0.3 %). x86-64 (Callgrind) 8-bit 4:2:0 1.009 / 1.152 / 1.164 G, 8-bit 4:4:4 1.105 / 1.257 /
1.273, 10-bit 4:2:0 1.159 / 1.323 / 1.338, 10-bit 4:4:4 1.286 / 1.458 /
1.482 (+1.0 to +1.6 %). Not measured: any time.

What remains of the inventory (not in this amendment's scope): the
assembly-shaped structure (register files, label functions,
`SECTION_RODATA` byte arrays, emulated frames, the flags arithmetic and
the hand-shaped source forms of the NEON kernels); about 1,914 `wrapping_*`
calls, most of them pointer steps of that structure (ARM64 intra
prediction 992, AVX2 intra prediction 450, the rest below 80 per file);
the 16 bpc z3 frames' lazy `write_bytes` zeroing and the two-stride
64-column store functions of the NEON 16 bpc DC/V paths (ordinary code,
kept).

**2026-11-20, CDEF and the inverse transforms written from the algorithm
are the shipped kernels** (decision paragraph "Kernels written from the
algorithm"; branch `v1/dav1d-kernel-rollout` from the itx pilot `60ed3a09`
merged with `main` at `61e20ecf`; evidence in the coordination directory,
`evidence/dav1d-kernel-rollout/`). The slot sets of the new init mirrors
equal those of the removed per-slot assignments in all six init branches
(ARM 8 bpc and 10 bits 156 each, WHT only at 12 bits; x86-64 8 bpc and 10
bits 156, 12 bits 148), checked by the conversion and by the selection
tests. Gates, on macOS ARM64 natively and on x86-64 in the Rosetta
container (both at the default level, no `RUSTFLAGS`, no environment
selection):

| Check | ARM64 (NEON) | x86-64 (AVX2) |
| --- | --- | --- |
| `cargo test -p libavif-dav1d --release --lib simd` | 24 passed | 20 passed |
| `check_dav1d_dsp.py --simd neon\|avx2` / scalar | 17 groups x 3,155 selections x 11 seeds, pass / pass | same, pass / pass |
| checkasm `itx_8bpc`, `itx_16bpc`, `cdef_8bpc`, `cdef_16bpc` alone, seeds 1-100 | 400 of 400 | 400 of 400 |
| `check_dav1d_simd_original.py --level neon\|avx2` and `--level scalar` (994 decodes with the six profiling stills) | 994 / 994 identical | 994 / 994 identical |
| `run_dav1d_tests.py` (release) | 1,711 of 1,711 | 1,711 of 1,711 |
| `check_dav1d_abi.py`, `check_dav1d_api.py`, `check_dav1d_linkage.py` | 12,157 checks 0 differences; 83; pass | not run (rustc killed for memory in the 8 GB VM); 83; pass |
| `cargo xtask codecs --codec avif --stage pr` | pass | in hosted CI |

Instructions per call against the original's assembly
(`measure_dav1d_kernels.py`, identical outputs; geometric mean, range):
ARM64 CDEF 1.45 (1.01-2.24), itx 8 bpc 2.86 (1.34-6.59), 16 bpc 2.64
(1.18-7.14); x86-64 CDEF 2.67 (2.04-3.74), itx 8 bpc 3.85 (2.51-11.18),
10 bits 3.52 (2.12-13.20), 12 bits 3.04 (1.87-13.31). Instructions per
decode, one thread (old port with the removed ports → this branch; stock
dav1d with its assembly at the same level), pictures identical:

| Still | ARM64 | x86-64 (Callgrind) |
| --- | --- | --- |
| 8-bit 4:4:4 q60 | 1.627 → 1.840 G (+13.1 %; stock 1.483) | 1.653 → 1.875 G (+13.5 %; stock 1.485) |
| 8-bit 4:2:0 q60 | 1.294 → 1.426 G (+10.2 %; 1.170) | 1.354 → 1.499 G (+10.7 %; 1.199) |
| 10-bit 4:4:4 q60 | 2.214 → 2.460 G (+11.1 %; 2.080) | 2.241 → 2.468 G (+10.1 %; 2.045) |
| 10-bit 4:2:0 q60 | 1.659 → 1.817 G (+9.5 %; 1.524) | 1.718 → 1.874 G (+9.1 %; 1.539) |
| 8-bit 4:4:4 lossless | 10.690 → 10.726 G (+0.3 %; 9.537) | 12.322 → 12.656 G (+2.7 %; 10.500) |

The port now runs 1.18 to 1.24 (ARM64) and 1.21 to 1.26 (x86-64) times
stock dav1d's instructions on the q60 stills, from 1.06 to 1.13; the cost
is accepted by the owner decision (optimization later). The cdylib grows
from 2,177,616 to 3,217,008 bytes on ARM64 (+1.04 MB) and from 3,075,488
to 3,965,800 bytes on x86-64 (+0.89 MB): the generic kernels are
instantiated per block size and transform type. Removed: the eight ported
kernel files (34,784 lines, 1,135 `unsafe`), the pilot's CDEF variants v1
and v3, the switches; `simd/cdef/` and `simd/itx/` hold 3,135 lines (31
`unsafe`, all in the two `mod.rs` boundaries and `cdef/kernels.rs`'s level
token) and 348 lines of tests.

**2026-11-20, the kernels' compile cost** (`06f8c1ef`, `d522dd15`): the
kernels moved into `libavif-dav1d-kernels` and the itx passes became shared
by row and column length (decision paragraph above). Gates at `2674a16e`
on ARM64 and x86-64 (Colima), as in the entry before: checkasm 17 groups x
3,155 selections x 11 seeds at `neon` / `avx2`, pass; itx and CDEF checkasm
seeds 1-100, 400 of 400 each; whole images 994 of 994 at the SIMD level and
at scalar each; `run_dav1d_tests.py` 1,711 of 1,711 in release and debug;
API 83 (release) and 82 (debug); ABI 12,157 (ARM64) and 12,284 (x86-64)
checks, 0 differences; linkage; tier-1 `cargo xtask codecs --codec avif
--stage pr` (ARM64). Instructions per decode against the old port, q60
stills: ARM64 +11.4 to +14.2 % (stock 1.20 to 1.25 times), x86-64 +10.4 to
+14.5 % (stock 1.22 to 1.28 times); lossless +0.2 % and +2.7 %; per call
against the assembly, itx 8 bpc 3.57 (ARM64) and 4.39 (x86-64), CDEF 1.46
and 2.66. The cdylib: 2,568,288 bytes on ARM64 and 3,352,456 on x86-64
(+0.39 and +0.28 MB over the removed ports). The CI native-adapters package
set builds cold in 1:55 with three jobs (main 1:30).

## Review when

`fearless_simd` changes its levels or is updated past 1.2.0 (the pin is
exact); a further family moves to kernels written from the algorithm; a
family is completed on a target; the x86-64 AVX2 kernels arrive; the owner
reopens the SSSE3/SSE4.1 gap or the AVX-512 scope; dav1d is updated; or the
engine's decoder choice changes.

## Amendment log

- 2026-11-20 — CDEF and the inverse transforms written from the algorithm on `fearless_simd` are the shipped kernels on NEON and AVX2; the 1:1 ports of their assembly and the pilot switches removed (decision paragraph "Kernels written from the algorithm" and validation above).
- 2026-11-19 — plain Rust, no codegen steering (owner principle of 2026-11-18): the compiler tricks removed, the edge reads kept inside the caller's array, the rule "no `call` in an entry point" withdrawn and the per-call ratio a measurement, not a target (decision paragraph and validation above).
- 2026-11-18 — intra prediction on AVX2, 8 and 16 bpc (validation above).
- 2026-11-18 — CDEF on AVX2, 8 and 16 bpc; the instructions per decode by family on x86-64 (validation above).
- 2026-11-17 — inverse transforms 16 bpc on AVX2 (validation above).
- 2026-11-17 — inverse transforms 8 bpc on AVX2 (validation above).
- 2026-11-17 — decisions on the first x86 kernels: msac's SSE2 16-symbol function and the gate level `sse2` removed (below AVX2 the port runs C there too), the volatile constant loads removed, the per-call 16-symbol choice and the module layout kept ("msac on x86-64", "Levels", validation above).
- 2026-11-17 — loop filter on AVX2, 8 and 16 bpc (validation above).
- 2026-11-17 — msac on x86-64 (SSE2 and AVX2), the first x86 kernels; a local x86-64 loop under Rosetta; the `sse2` gate level (decision paragraphs and validation above).
- 2026-11-17 — the per-kernel gate runs eleven seeds: one more for the 8 bpc loop filter's 16-pixel filter (validation above).
- 2026-11-17 — the per-kernel gate runs ten seeds: two more for the loop restoration tails (validation above).
- 2026-11-17 — loop restoration on NEON, 8 and 16 bpc (validation above).
- 2026-11-17 — intra prediction wrappers: `top_out` and `left_out` zeroed whole, the edge array's slack checked by a test (validation above).
- 2026-11-17 — intra prediction 16 bpc on NEON (validation above).
- 2026-11-17 — intra prediction 8 bpc on NEON (validation above).
- 2026-11-17 — the per-kernel gate runs eight seeds (validation above).
- 2026-11-17 — CDEF on NEON, 8 and 16 bpc (validation above).
- 2026-11-17 — inverse transforms 16 bpc on NEON (validation above).
- 2026-11-16 — msac selected at compile time; portable build configuration for the scalar gate (decision and validation above).
- 2026-11-16 — loop filter 16 bpc on NEON (validation above).
- 2026-11-16 — loop filter 8 bpc on NEON (validation above).
- 2026-11-16 — msac on NEON (decision paragraph and validation above).
- 2026-11-16 — 8 bpc inverse transforms complete on NEON (validation above).
- 2026-11-16 — Accepted with the infrastructure and the first slice.
