# Rebuild dav1d's assembly kernels with intrinsics

- **Status:** Accepted (infrastructure and the first itx slice on AArch64 NEON)
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
assignments under the CPU flags), and the x86 files likewise. One Rust
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
renames by their path. Each owned function calls the rebuilt kernel when
`dav1d_get_cpu_flags()` reports NEON and the C function otherwise: the
original has no run-time choice here, the check keeps
`dav1d_set_cpu_flags_mask(0)` a complete scalar path. `MsacContext` keeps the
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
the x86-64 scope. **Stated gap:** the SSSE3 and SSE4.1 kernels,
which the original selects only on CPUs without AVX2, are out of scope for
now; on such CPUs the port runs its C kernels where the original runs SSSE3
or SSE4.1. On an AVX-512 (Ice Lake) machine the original selects
`avx512icl` kernels; the gate restricts it with
`dav1d_set_cpu_flags_mask(0x2f)` (SSE2 to AVX2, `SLOW_GATHER` passed
through), as libaom's oracle is built without AVX-512.

**Gate (engine ADR-0041, ADR-0044 D2).**

1. *Oracle.* `tools/build_dav1d.py --asm` builds the original from the same
   source with its assembly, `-ffp-contract=off`, `-Dtrim_dsp=false` (so the
   mask can select every level down to C), a static library and the
   original CLI (`build/dav1d-asm-<profile>`, `reference-profile.json` with
   the configuration, assembly-source and archive digests). With
   `--cross x86_64-linux-gnu` it is built in the `tools/qemu_x86_64`
   container and run under `qemu-x86_64 -cpu max` (no Rosetta).
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
   expected MD5 where the suite knows it.

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

## Review when

A family is completed on a target; the x86-64 AVX2 kernels arrive; the owner
reopens the SSSE3/SSE4.1 gap or the AVX-512 scope; dav1d is updated; or the
engine's decoder choice changes.

## Amendment log

- 2026-11-16 — loop filter 16 bpc on NEON (validation above).
- 2026-11-16 — loop filter 8 bpc on NEON (validation above).
- 2026-11-16 — msac on NEON (decision paragraph and validation above).
- 2026-11-16 — 8 bpc inverse transforms complete on NEON (validation above).
- 2026-11-16 — Accepted with the infrastructure and the first slice.
