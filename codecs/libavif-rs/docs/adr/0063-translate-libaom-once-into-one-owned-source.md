# Translate libaom once into one owned source

- **Status:** Accepted
- **Date:** 2026-11-21
- **Source:** owner decisions of 2026-11-20 (route B: remove the variants at
  the source, translate once, the result is our fork) and 2026-11-21 (Debug-only
  C code and the Debug-only `MACROBLOCK` field are Debug-only in Rust as well;
  one assert message form, `upstream/aom/<path>`)
- **Amends:** [0059](0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md)
  (the frozen state is now one tree), [0060](0060-build-the-translations-on-stable-rust.md)
  (step (b), merging profiles, and step (c), merging platforms, are done here);
  [0048](0048-require-complete-release-and-focused-debug-aom-acceptance.md),
  [0056](0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md) and
  [0058](0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md) are unchanged
- **Related:** issues #128 and #208; `docs/publishing.md` section 2.
  Number 0061 is used by the unmerged branch `v1/avif-drop-dav1d`.

## Context

`crates/aom` shipped libaom four times, as C2Rust saw it:
`src/{macos,linux}/{debug,release}`. The package was 16.9 MiB compressed
(`cargo package`), above the 10 MiB crates.io limit, and every owned change
had to be made four times. The analysis of 2026-11-20 (orchestration evidence
`aom-translations-analysis`) sorted the differences between the trees:

| Difference | Items | Size |
| --- | ---: | ---: |
| Debug vs Release: assert form and `NDEBUG` code, per platform | 1,811 | 7.9–8.2 MB |
| macOS vs Linux: system-header declarations | 5,253 | 0.62 MB |
| macOS vs Linux: `va_list` | 224 | 0.26 MB |
| macOS vs Linux: CPU dispatch | 706 | 0.76 MB |
| macOS vs Linux: real platform details | 172 | 0.44 MB |
| C2Rust noise (order, numbering) | ~4,300 | 0.27 MB |

All of them come from how the C was presented to C2Rust: preprocessed with or
without `NDEBUG`, against the macOS SDK or glibc, for one CPU's dispatch
headers.

## Decision

### The C stays; only its presentation changes

The libaom 3.15.2 sources, configuration and build stay the reference. The C
reference builds, their tests and every golden output are unchanged.
`tools/aom_one_source.py` presents the same units to C2Rust 0.21.0
(LLVM 16.0.6) once per target, from the Debug compilation database with the
profile and host flags removed (`-g`, `-O*`, `-DNDEBUG`, `-fPIC`, `-arch`):

- **Forced include** `tools/aom_one_source/prelude.h` and a neutral
  `<assert.h>`: `assert(e)` becomes
  `if cfg!(debug_assertions) && !(e) { crate::platform::ferrisma_assert_fail(..) }`.
  Like C's `assert` under `NDEBUG`, the expression is not evaluated in Release.
  The message names the file as `upstream/aom/<path>` on both targets.
- **No `NDEBUG`, `CONFIG_DEBUG 1`** for every profile. The C statement blocks
  that the original compiles only in Debug are listed in
  `tools/aom_one_source/debug_blocks.json` with their reason. Each is
  presented, in a prepared copy of its file that keeps every line number,
  between `FERRISMA_DEBUG_ONLY_BEGIN`/`END` and becomes
  `#[cfg(debug_assertions)] { .. };`. So do the Debug-only `MACROBLOCK`
  member `last_set_offsets_loc`, its type `SetOffsetsLoc`, the one assert that
  reads it, and the exports `aom_is_pyramid_valid` and
  `aom_is_corner_list_valid` (`cfg_attr(debug_assertions, no_mangle)`). Release
  builds therefore have the original Release record layouts, symbols and code.
  Declarations that a later assert names (`mi_count`) stay as dead locals.
- **Neutral system headers** (`tools/aom_one_source/include`, `-nostdlibinc`):
  one spelling of every fixed-width type (both targets are LP64). Where the C
  libraries differ, the header calls a shim in `crates/aom/src/platform.rs`:
  the standard streams (`__stderrp`/`stderr`), `errno`, the floating-point
  exception calls that only the GNU Linux Debug build makes
  (`HAVE_FEXCEPT && CONFIG_DEBUG`), `jmp_buf`. The pthread types and
  `timeval` become `libc::` types; their placeholders give Clang each C
  library's layout for its record queries.
- **Target triple** `arm64-apple-macosx11.0.0` or `x86_64-unknown-linux-gnu`,
  so only target facts can still differ: predefined macros, the CPU dispatch
  headers that `cmake/rtcd.pl` generates with the port overlays (ADR 0056,
  0058), and C2Rust's `va_list` spelling.

Each unit then goes through the existing correction pipeline of
`tools/transpile_aom.py`, unchanged and in the same order (`corrected`,
`map_jumps`, `compatibility`, the CPUID mapping, `stable_text`, the inline
test bridges). `transpile_aom.py` itself stays frozen: it reproduces the
former trees, and its arguments (`--clang-include`, the host platform) cannot
express this presentation.

### One tree

`tools/merge_aom_platforms.py` merges both targets' output into
`crates/aom/src/libaom`. `tools/aom_va_list.py` first gives `va_list` one
spelling, `core::ffi::VaList<'_>`. An item that is the same on both targets is
written once; any other item is written per target under
`#[cfg(target_arch = "aarch64")]` or `#[cfg(target_arch = "x86_64")]`. The
crate builds only for macOS ARM64 and GNU Linux x86-64 (`lib.rs`), so the arch
names the target. `--verify` projects the tree back to each target and
requires byte equality with that target's translation. Projecting and
merging again reproduces every unit, so tools can edit one target's side.

Result at 2816e928: 156 of 166 units identical on both targets, 74,352
shared items, 11 functions per target (the byte swap of `entenc.c`, thread
naming in `aom_thread.c`, nine callers of dispatched kernels), and
1,279 macOS and 849 Linux items of their own: the RTCD tables, their kernel
declarations and dispatch pointers. Debug and Release are the same text.

`crates/aom/TRANSLATION.json` is the one record: the presentation's digests,
both targets' dispatch blocks, the original C configuration of each target and
profile, and every unit with one digest. `tools/aom_manifest.py` gives each
target/profile view in the former manifest shape to the tools that read it.
`tools/enable_aom_dispatch.py` (and `--check`, `--rehash`) works on the
target's projection and merges the result back; `check_translation.py`,
`stable_rust.py` and the gates read the one tree. The Linux regeneration modes
of `avif-aom-translate-linux.yml` are retired. Future upstream changes are
ported by diff, as ADR 0059 already says; no permanent re-translation is
needed. Every change above was made by these committed tools (engine
ADR-0041 O8).

## Consequences

- Package: 16.9 MiB compressed before, 4.8 MiB after (`cargo package
  --no-verify`, 59.6 MiB unpacked, 319 files), below the 10 MiB limit (#208).
- A fix, an optimization or a removed `unsafe` is made once.
- A Release build compiles the Debug checks and drops them; a Debug build of
  either target runs the original's Debug checks.
- Equivalence: see "Verification". The golden outputs, expected values and
  tolerances were not changed.
- `crates/aom/tests/dispatch_scope.rs` scans 3 RTCD units instead of 12: the
  count was the inventory of files the scan covers (three units in each of the
  four former trees), not an output. Each of the three units now holds the
  tables of both targets and every profile, and the test also requires both
  `cfg(target_arch)` tables in each, so it covers the same tables as before
  (accepted by the coordinator, 2026-11-21).

## Verification

At 47ee0e91 (the tree of this decision), on macOS ARM64 natively and on GNU
Linux x86-64 in the Colima `dav1dx86` container (Rosetta, AVX2; the SIGSEGV
trap preloaded, no trap fired). Scripts and logs: orchestration evidence
`aom-one-source` (`scripts/gates*.sh`, `macos-47ee0e91`, `macos2-47ee0e91`,
`x86-47ee0e91`).

| Gate | macOS ARM64 | x86-64 GNU Linux |
| --- | --- | --- |
| `check_translation.py` (4 target/profile views) | pass | pass |
| `enable_aom_dispatch.py --check` / `--linux --check` | 631 functions | 420 functions |
| `stable_rust.py --check` | pass | pass |
| `cargo test --test simd`, `release` and `checked` | 164/164 each | 265/265 each |
| `check_aom_golden.py` | 62/62, 62 C-confirmed | 62/62, 62 C-confirmed |
| `cargo test -p ferrisma-libaom` (all tests, dev) | 170 passed | 271 passed |
| `check_aom_simd_identity.py` | 76/76 | 76/76 |
| `check_aom_simd_original.py` | `neon` 82/82, `scalar` 82/82 | `avx2`, `sse4.1`, `scalar` 82/82 each |
| `check_aom_abi.py`, Debug | 17,555 records, 198,757 checks, 0 differences | 0 differences |
| `check_aom_abi.py --release` | 17,506 records, 198,463 checks, 0 differences | 0 differences |
| `check_aom_linkage.py` Debug / Release | 2,728 / 2,726 globals | 2,728 / 2,726 globals |
| `check_aom_focused_debug.py` (ADR 0048) | 69/69 | 69/69 |
| `check_aom_unaligned_input.py` Debug and Release | 24/24 each | 24/24 each with `AOM_SIMD_CAPS_MASK=0`; see below |

The two Debug-only exports account for the two extra Debug globals, as in the
original's Debug archive.

On x86-64 the unaligned-input check compares the port's archive, which
dispatches SIMD at run time, with the generic C reference. With SSE2 enabled
the 10-bit zero-sample encode differs from generic C, and equals libaom's own
AVX2 build (`build_aom.py --simd avx2`) byte for byte in every case tried
(10, 12 and 8 bit, both address parities). With the dispatch off the port
equals generic C in all 24 cases. This is the original's SIMD behavior, which
engine ADR-0041 compares SIMD with; the check's reference choice is an open
item for the check, not for this decision. The check now compares with both
references ([ADR 0046](0046-preserve-byte-aligned-high-bit-depth-input-validation.md),
amendment of 2026-11-21).

The complete original Release suites (ADR 0048) ran locally at 44e77b9f
(owner decision of 2026-11-21: local, not hosted), with the shards and
expectations of `avif-aom-original-native.yml` and
`avif-aom-original-release.yml` unchanged: all 120 `test_libaom` Release
shards, `test_aom_rc` and `test_intra_pred_speed`, then the collect step, plus
the native build job's checks (consumers, linkage, scalar roles,
`HighbdEncodeSmallWidthHeight.*` pairs, focused Debug, ABI).

| Suite (Release) | macOS ARM64 | x86-64 GNU Linux |
| --- | --- | --- |
| `test_libaom` | 17,228/17,228 active cases, 23 matching dynamic skips, valid | 17,228/17,228, 23 matching dynamic skips, valid |
| `test_aom_rc` | 38/38 | 38/38 |
| `test_intra_pred_speed` | 0/0 (every registration disabled upstream) | 0/0 |
| shards | 120/120 pass | 120/120 pass |
| wall clock | 9 h 35 min (4 shards in parallel) | 21 h 22 min (2 to 4 in parallel, Rosetta) |

The build-job checks passed on both, except the x86-64 unaligned-input check
described above, and `verify_upstream.py --tracked`, which needs a Git checkout
that the x86-64 container's copy of the tree does not have (the untracked
verification inside `check_translation.py` passed).
