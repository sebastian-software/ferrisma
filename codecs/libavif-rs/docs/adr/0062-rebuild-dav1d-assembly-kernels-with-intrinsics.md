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

## Review when

A family is completed on a target; the x86-64 AVX2 kernels arrive; the owner
reopens the SSSE3/SSE4.1 gap or the AVX-512 scope; dav1d is updated; or the
engine's decoder choice changes.

## Amendment log

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
