# ADR-0044: No assembly; acceleration only through Rust intrinsics

- **Status:** Accepted (the principle and decisions D1–D4). The treatment of the ten inline assembly sites that remain in libjxl-rs is an [open question](#open-questions-pending-owner-decision).
- **Date:** 2026-11-15
- **Last reviewed:** 2026-11-16
- **Source:** owner decision of 2026-11-15 on open question O4 of [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (#250)
- **Supersedes:** In part: the O4 working rules of 2026-11-13 in [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (assembly-only kernels stay on the C path; `rcpps` translated as the same instruction), which this record turns into decisions.
- **Related:** [0020](0020-kernel-crate-and-isa-dispatch.md), [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md), [supported matrix](../supported-matrix.md#simd-instruction-set-levels-v10), [unsafe review](../unsafe-review.md), libjpeg-turbo-rs [ADR 0008](../../codecs/libjpeg-turbo-rs/docs/adr/0008-x86-64-kernels-from-nasm.md), libavif-rs [ADR 0058](../../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md), libjxl-rs [ADR 0004](../../codecs/libjxl-rs/docs/adr/0004-simd-paths.md), #242, #124

## Context

Two of the originals Ferrisma ports implement some of their fastest kernels
in assembly rather than in C or intrinsics:

- **libaom** has NASM sources for x86 (`aom_dsp/x86/*.asm` in the pinned
  3.15.2). Most of its x86 kernels also exist as C with intrinsics, and the
  port translates those (libavif-rs ADR 0058). Some exist only as NASM: the
  24 8-bit DC, DC_TOP, DC_LEFT, DC_128, V and H intra predictors for 4x4 to
  32x32 (`intrapred_asm_sse2.asm`), the high-bit-depth DC and V predictors
  for 4x4 to 32x32, and the SSSE3 `aom_quantize_b` and
  `aom_quantize_b_32x32` (`quantize_ssse3_x86_64.asm`). Under the working
  decision of 2026-11-13 they stay on the translated `_c` path
  ([libavif-rs `crates/aom/README.md`](../../codecs/libavif-rs/crates/aom/README.md),
  ADR 0058).
- **dav1d** has its kernels in assembly. The port is built with
  `-Denable_asm=false`, has no assembly and runs its scalar C translation
  (`dav1d_init_cpu` is empty; [supported matrix](../supported-matrix.md#simd-instruction-set-levels-v10)).
  rav1d, the Rust translation of dav1d, links dav1d's assembly for speed.
- **libjpeg-turbo** has NASM for x86-64. Its port already rewrote those
  kernels as SSE2 and AVX2 intrinsics (libjpeg-turbo-rs ADR 0008) and
  compares them with libjpeg-turbo's SIMD build, which runs the NASM
  (oracle variant `simd`; ADR-0041, per-port tables).

A further case is an x86 instruction whose result depends on the CPU vendor:
libjxl's AVX2 paths use `_mm256_rcp_ps` (`rcpps`), an approximation whose
bits Intel, AMD and Rosetta 2 need not agree on (libjxl-rs ADR 0004,
Consequences). Since the working decision of 2026-11-13 (evening) the port
executes the same instruction as the original's AVX2 build.

ADR-0041 left both cases open as O4. When the owner decided O4 on
2026-11-15 he stated a principle: he wants memory safety, so the project
contains no assembly, and acceleration comes only through Rust intrinsics
with narrowly bounded `unsafe` code. Linking an original's assembly, the
route rav1d takes, is excluded.

## Decision

**D1. No assembly in the project.** No Rust source of the workspace contains
inline or global assembly (`asm!`, `global_asm!`, `naked_asm!`), and no
package builds, links or ships an assembly source or an object assembled
from one (NASM, GAS, `.S`). SIMD acceleration is written with the
`core::arch`/`std::arch` intrinsics, with `unsafe` limited to what the
intrinsic and its memory access need, argued where it occurs (as in
ADR-0020 and the ports' SIMD ADRs). The originals' upstream snapshots, which
contain their assembly, are used only to build the reference builds the
gate compares against; they are not part of any package.

**D2. Kernels that exist only as assembly in an original run on the
translated scalar path until they are rebuilt with intrinsics.** For the
libaom kernels named above the owner leans towards rebuilding them, since
the original had a reason to hand-write them; they are measured first, in
the next hosted performance run, and then rebuilt unless the measurement
shows that it is clearly not worth it. The gate for a rebuilt kernel is
bit identity with the original's SIMD build at the same level, which runs
the original's assembly kernel (ADR-0041 section 2). dav1d's kernels follow
the same rule (see Consequences on dav1d's place).

**D3. A vendor-dependent instruction stays as in the original.** Where the
original's SIMD build executes an instruction whose result depends on the
CPU vendor, the port executes the same instruction (`rcpps` in libjxl's AVX2
paths). On the same machine the port equals the original, which is what
ADR-0041 section 2 requires. Between CPU vendors the results can differ,
as the original's do; the [supported matrix](../supported-matrix.md#simd-instruction-set-levels-v10)
says so.

**D4. Linking the original's assembly is excluded.** A port does not reach
the original's speed by linking its assembly objects (rav1d's route), not
even as an optional feature.

## Consequences

- The rule against assembly is a project rule in `AGENTS.md`. A change
  that adds assembly is not merged; a kernel that seems to need it is
  written with intrinsics or stays on the scalar path.
- The libaom kernels of D2 stay on `_c` until measured and rebuilt. Until
  then the AOM port's x86-64 SIMD path runs C where the original's SIMD
  build runs assembly. The output gate is unaffected on its corpus:
  libavif-rs ADR 0058 records `avx2`, `sse4.1` and `scalar` identical with
  the original (82 of 82 each) with these kernels on `_c`.
- dav1d is leaving the runtime and the published chain (owner decision of
  2026-11-15 on #54, recorded in [ADR-0031](0031-workspace-libavif-input-experiment.md));
  its kernels then matter only for its role as a test cross-check, where the
  scalar translation suffices.
- libjxl-rs AVX2 output can differ between an Intel and an AMD machine, as
  the original's does. A caller who needs the same bytes on every x86-64
  machine uses the scalar path; nothing in the engine promises identity
  across hosts (ADR-0041 section 2).
- Rebuilding an assembly kernel with intrinsics is a translation from a
  different source language, not a mechanical port of C. It is reviewed
  against the assembly instruction by instruction, as libjpeg-turbo-rs
  ADR 0008 did for its NASM kernels.

## Alternatives considered

- **Link the original's assembly (rav1d's route).** Rejected by the owner
  (D4): it brings unsafe code without Rust's checks into every build and
  makes the port's memory safety depend on hand-written assembly.
- **Inline assembly in Rust (`asm!`) for the assembly-only kernels.**
  Rejected by D1 for the same reason: it is assembly in the project.
- **Leave the assembly-only kernels on the scalar path for good.** Not
  chosen: the owner leans towards rebuilding them, and the measurement
  decides (D2).
- **Replace `rcpps` by an exact division or a fixed table so that every
  vendor gives the same bits.** Rejected (D3): the port would then differ
  from the original's AVX2 build on the same machine, which is what the
  gate compares.

## Validation

- **Mechanical guard (since 2026-11-16).** `cargo xtask assembly`
  (`tools/xtask/src/assembly.rs`; CI's `affected` job, which runs on every
  event) scans every tracked `.rs` file outside `codecs/*/upstream/` for
  invocations of `asm!`, `global_asm!` and `naked_asm!` and for `#[naked]`
  and `#[unsafe(naked)]`, and compares each file's count with
  `tools/xtask/assembly-allowlist.txt`. It fails on a site that is not
  listed, on a count that differs and on a stale entry; it also fails if
  `git ls-files '*.S' '*.s' '*.asm' '*.nasm'` lists anything outside
  `codecs/*/upstream/`, or if a tracked `build.rs` outside the snapshots
  mentions `cc::`, NASM or an `.S` source. Its tests include positive
  controls: in-memory samples of every form must be found, and a Git
  fixture with an unlisted site, a changed count, a stale entry, an
  assembly source and a `cc::` build script must fail. The allowlist holds
  only the sites of the open question below; under D1 it must not grow.
- A rebuilt kernel passes the ADR-0041 gate against the original's SIMD
  build at its level, on the target the kernel is for, and its port's
  kernel tests.
- D3 is falsified on one host by a difference between the port's and the
  original's AVX2 build; a difference between two hosts is expected.

**State on `main` at `679f382c` (2026-11-15).** The decision was taken on
the statement that no `.rs` file on `main` contains `asm!` or
`global_asm!`. A re-check with `git grep -n -E '(global_)?asm!' -- '*.rs'` found 21 `asm!`
sites in 16 files and no `global_asm!`:

| Files | Sites | What they are |
| --- | ---: | --- |
| `codecs/libavif-rs/crates/aom/src/linux/{debug,release}/aom_dsp/aom_dsp_rtcd.rs`, `aom_scale/aom_scale_rtcd.rs`, `av1/common/av1_rtcd.rs` | 6 | `xgetbv` as the byte sequence `.byte 0x0f, 0x01, 0xd0` in the translated x86 CPU query of the GNU Linux translations, in the form C2Rust gives inline `asm` statements |
| `codecs/libavif-rs/crates/sharpyuv/src/sharpyuv_cpu_x86_64.rs` | 1 | the same `xgetbv` in the translated SharpYUV CPU query |
| `codecs/libavif-rs/crates/dav1d/src/linux/{b8,b16,release/b8,release/b16}/recon_tmpl.rs` | 4 | an `asm!` with only a comment as its template and two register operands: no instruction, a barrier for the optimizer, in the form C2Rust gives inline `asm` statements |
| `codecs/libjxl-rs/src/brotli/bitfield.rs`, `src/brotli/raw_bytes.rs` | 6 | AArch64 read-modify-write of a byte or word whose other bits may be uninitialized (`ldr`/`bic`/`orr`/`str`, `ldrb`/`eor`/`strb`) |
| `codecs/libjxl-rs/src/jxl/cms/jxl_cms_profile.rs` | 1 | two AArch64 `fmul` with a fixed operand order, which its comment gives as preserving competing NaN payloads (ADR-0041 section 3 took NaN payloads out of the gate on 2026-11-13) |
| `codecs/libjxl-rs/src/jxl/hwy/neon.rs`, `src/skcms/transform_simd.rs` | 3 | AArch64 half-precision conversion (`fcvtn`, `fcvtl`) |

The "form C2Rust gives" in the first three rows is read from the code; it
was not compared with the upstream C on this day. No file changed with this
record. Assembly source files: `git ls-files '*.asm' '*.S' '*.s' '*.nasm'`
lists only files under `codecs/libavif-rs/upstream/`, and no `build.rs` or
manifest of a package refers to NASM (one research experiment,
`docs/research/encoder-experiment`, which is not a workspace package,
mentions it in its `Cargo.toml`).

**State on 2026-11-16: 21 sites became 10.** Branch `v1/avif-no-asm`
removed the eleven libavif-rs sites through the port's committed tools,
whose `--check` mode CI runs (ADR 0059 of that port):

| Sites | Commit | Change |
| ---: | --- | --- |
| 7 (`xgetbv`: AOM Linux RTCD units 6, SharpYUV 1) | `fix(avif): read XCR0 through core::arch::x86_64::_xgetbv` | `core::arch::x86_64::_xgetbv(0)` in a `#[target_feature(enable = "xsave")]` wrapper, under the original's unchanged OSXSAVE check: the same instruction and ECX (`rewrite_xgetbv` in `tools/stable_rust.py`; libavif-rs ADR 0023, amendment) |
| 4 (dav1d barrier) | `fix(avif): delete dav1d's empty x86-64 register barrier` | deleted: it emits no instruction and its read-write operands keep their values (`remove_register_barrier` in `tools/stable_rust.py`, called by `transpile_dav1d.py`; libavif-rs ADR 0031, amendment) |

The "form C2Rust gives" of the first three rows of the table above was
confirmed against the upstream C on that day: `aom_ports/x86.h` and
libwebp's `src/dsp/cpu.c` read XCR0 with `.byte 0x0f, 0x01, 0xd0`, and
dav1d's `src/recon_tmpl.c` has `__asm__("" : "+r"(mask), "+r"(mul))` for
x86-64 GNU compilers. The remaining ten are the libjxl-rs rows, unchanged and
listed in `tools/xtask/assembly-allowlist.txt`:
`src/brotli/bitfield.rs` (4), `src/brotli/raw_bytes.rs` (2),
`src/jxl/cms/jxl_cms_profile.rs` (1), `src/jxl/hwy/neon.rs` (1),
`src/skcms/transform_simd.rs` (2). The table's brotli row names only the
AArch64 instructions; three of its six sites are the x86-64 counterparts.

## Open questions (pending owner decision)

- **The ten inline assembly sites that remain in libjxl-rs** (the brotli,
  NaN-order `fmul` and half-precision rows of the table above, listed in
  `tools/xtask/assembly-allowlist.txt`). Whether D1 applies to them as they
  stand, and with what route: an intrinsic where one exists (the AArch64
  half-precision conversions), removal where the reason is gone (the
  NaN-order `fmul`, if ADR-0041 section 3 makes it unnecessary), or a
  narrow, named exception. Until the owner decides, these sites stay as
  they are, and no new site is added (the guard enforces both).

## Review when

- The hosted performance run measures the libaom kernels of D2.
- A kernel is rebuilt with intrinsics, or an original adds an assembly-only
  kernel on a path in scope.
- Rust stabilizes an intrinsic that replaces one of the sites in the table.
- The owner decides the open question.

## Amendment log

- 2026-11-16 — The eleven libavif-rs sites are removed through the port's
  tools (state paragraph of that date); the mechanical guard
  `cargo xtask assembly` with its allowlist enforces D1 for the rest; the
  open question concerns only the ten libjxl-rs sites.
- 2026-11-15 — Accepted as the owner's decision on O4 and the principle he
  stated with it; the inline assembly found by the re-check is recorded as
  an open question.
