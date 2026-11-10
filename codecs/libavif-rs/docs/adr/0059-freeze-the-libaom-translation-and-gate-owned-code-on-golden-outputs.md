# Freeze the libaom translation and gate owned code on golden outputs

This ends the mechanical phase that [ADR 0001](0001-mechanical-port.md),
[ADR 0003](0003-automated-translation.md) and
[ADR 0013](0013-separate-optimization-from-the-mechanical-port.md) set up for
libaom, and adds a pull-request tier to
[ADR 0055](0055-gate-pull-requests-on-linux-ci-and-local-macos-checks.md).
It changes no evidence that is already recorded.

## Context

ADR 0001 kept the port C-shaped "before any idiomatic Rust API, ownership
redesign, or optimization is considered". That phase has done its job: both
native profiles pass the complete original Release suites (ADR 0048), and the
Neon and x86 kernels show that optimization can stay bit-exact (ADR 0056,
ADR 0058).

Two things now stand in the way of the next phase.

**Regenerating the translation overwrites owned work.** The gains Ferrixel
needs mostly come from treating the encoder as part of one still-image
pipeline. Examples are writing pipeline output straight into the encoder's
buffer layout, allocating for one frame instead of a video lookahead,
reusing encoder contexts, and sharing a thread pool. Each of these edits
translated code, and the next C2Rust run would discard it. A patch queue on
top of regenerated code does not hold up over time.

**Verification takes too long for a pull-request loop.** Regenerating the Linux
profiles takes about three hours. Run 42505250671 exceeded its 180-minute
limit. Of the last complete run (42370060809), the independent C oracles
(ADR 0041) took two hours: each suite and profile compiles a test copy, relinks
the complete `test_libaom`, and runs C and Rust. Kernel work also has to wait
for this run, because enabling a kernel in an RTCD overlay changes generated
units.

Upstream libaom keeps improving the still-image encoder in bursts. The
CHANGELOG has 10 to 12 still-image entries in each of 3.12, 3.13 and 3.14
(AOM_TUNE_IQ, AOM_TUNE_SSIMULACRA2, Variance Boost) and none in 3.15. The AV1
format itself has been fixed since 2018. Following every release is therefore
not needed. Selected features can be ported on purpose.

The encoder is deterministic for a fixed source revision, configuration, thread
setting and declared floating-point reference (ADR 0021, ADR 0024). The SIMD
identity corpus already relies on this: separate runs produce byte-identical
AVIF files.

## Decision

### Freeze

The translated libaom sources are frozen at libaom 3.15.2 after one final
regeneration. That run includes the x86 dispatch of ADR 0058 and any further
still-image scope reduction through upstream switches that is decided before
it. After that, the committed Rust is the source of truth: it is reviewed and
changed like any other code. `tools/transpile_aom.py` and the translation
workflows are kept for reproducing the frozen state, not for producing new
code.

### Reference and golden outputs

The bit-exact contract of
[ADR 0012](0012-require-bit-exact-results-against-declared-references.md)
stays. Its reference becomes a declared C reference: the pinned upstream
snapshot plus the upstream commits that the port has taken over, applied in C.
At first this is unmodified 3.15.2.

Golden outputs are recorded from that C reference and committed with their
provenance: source and patch hashes, configuration, compiler flags, platform
and floating-point profile. They come at two levels:

- **End to end:** a still-image corpus crossed with the settings Ferrixel uses
  (bit depth, subsampling, speed, quality, tuning). For each combination, the
  SHA-256 of the AVIF output is recorded, and of the decoded pixels where it
  applies.
- **Per function:** each DSP role covered by the oracles is fed the original
  tests' seeded inputs, transcribed as in `crates/aom/tests/simd`. One output
  hash is recorded per case. These hashes locate a failure and cover edge
  cases that the corpus does not reach.

Golden outputs are recorded again only when the C reference changes. That
change is a reviewed diff in the same pull request as the Rust port of the
upstream change.

### Tiers

| Tier | Runs on | Content | Target |
| --- | --- | --- | --- |
| Pull request | every push | build, `cargo test`, root smoke tier, SIMD kernel tests (Release and Debug in parallel jobs), SIMD/scalar identity corpus, golden outputs, safety ratchet, end-to-end benchmark against `main` for changes to hot paths; no C build | about 10 minutes |
| Nightly | `main` | re-record golden outputs from the C reference and require equality, live independent C oracles as a parallel matrix, fault injection, ABI layouts, ASan, macOS | hours allowed |
| Reference change | upstream feature port | rebuild the C reference, record golden outputs, review their diff | once per change |

A pull request that fails the golden outputs is not mergeable. A nightly run
that finds golden outputs which no longer match the C reference is a defect in
the golden outputs or in the reference build, and is treated as one.

### Owned code

Translated code is changed by layer:

- **API boundary** (configuration, input, output): safe, idiomatic Rust under
  `#![forbid(unsafe_code)]`.
- **Pipeline integration** (buffers, allocation, context reuse, threading): the
  main area for Ferrixel-specific optimization, safe wherever practical.
- **DSP and SIMD hot paths:** owned kernels behind RTCD. `unsafe` is limited to
  memory access and feature dispatch.
- **Encoder decision logic** (RD search, speed features, tuning): kept close to
  upstream so that ported features stay easy to apply. It is changed only for a
  ported feature or for a measured bottleneck.

### Safety target

The goal is a crate that is at least 95 % safe Rust. The share is measured over
function-body lines in the non-test sources of `crates/aom/src`: the lines
outside `unsafe` blocks and `unsafe fn` bodies. Tables, type definitions and
declarations are left out, because they would inflate the share without saying
anything about memory safety. Complete absence of `unsafe` is not required.

The frozen starting point is far from this. In the macOS Release profile, 7,683
of 11,598 translated functions are `unsafe fn`, and so are 1,072 of 1,091
functions in `crates/aom/src/simd`. Most of the translated functions that are
safe are small arithmetic helpers.

`unsafe` stays only where one of these holds:

- **No safe form exists:** foreign calls, CPU feature dispatch, and the memory
  accesses of SIMD intrinsics.
- **Measured benefit:** the safe replacement slows the end-to-end benchmark
  (the golden corpus encode, median of repeated runs on the CI runner and on
  ARM64) by more than 2 %.

Each remaining `unsafe` region carries a `SAFETY:` comment with its invariant.
Its reason, and for a measured exception the benchmark result, are listed in
`crates/aom/UNSAFE.md`, which CI checks against the source.

A ratchet makes progress permanent. CI reports the safe share and counts the
`unsafe` regions per module, and a pull request may lower neither. A module
that reaches zero gets `#![forbid(unsafe_code)]`. A pull request that adds an
`unsafe` region must add its entry to `UNSAFE.md` with the measurement.

Changes that keep the output identical are accepted on golden outputs alone.
Changes that alter the output on purpose need a C reference change (for a
ported upstream feature) or, for Ferrixel's own encoder changes, a measured
quality and size comparison on the corpus. Such changes leave the libaom
reference, so their golden outputs only guard against unintended change.

### Upstream features

Upstream releases are not followed as a whole. A feature is ported when it
pays off for Ferrixel. Its upstream commits are applied to the C reference,
reproduced in the Rust, and accepted on the new golden outputs.

## Consequences

- Kernel and optimization pull requests get a loop of about ten minutes. The
  RTCD units become owned code: enabling a kernel edits them directly and no
  longer needs a regeneration run.
- The two-hour oracle suites move to nightly. A defect in scalar code that the
  corpus and per-function golden outputs miss can therefore reach `main` and
  is found the next night. This is accepted for the shorter loop.
- Recording golden outputs requires the C reference on each platform whose
  floating-point profile differs. Where the profiles agree, one recording is
  enough.
- Correspondence with upstream source (ADR 0001) no longer holds for changed
  modules. Source correspondence checks are limited to modules still in their
  translated form, and the inventory records which modules are owned.
- Ferrixel's
  [ADR-0025](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)
  says scope is reduced "never by editing generated code". After the freeze
  that rule applies only up to the final regeneration and needs a matching
  update in Ferrixel.
- The same approach can later apply to the other ports, each in its own
  decision.

## Amendment: first pull-request slice (2026-11-09)

The x86 SIMD workflow (`aom-simd-linux.yml`) now runs four parallel jobs that
build no C library. Before, one job ran Release and Debug kernel tests and the
identity corpus in sequence and took 41 minutes (run 42584921441). Run
42605114224 on four-core ubuntu-24.04 runners:

| Job | Content | Wall clock |
| --- | --- | --- |
| `kernels (release)` | 79 SIMD kernel tests, Release translation profile | 3 min 23 s |
| `kernels (checked)` | the same tests, Debug translation profile at Release optimization | 3 min 35 s |
| `identity` | 76-case SIMD/SSE4.1/C identity corpus | 3 min 18 s |
| `golden` | 62 golden AVIF outputs | 1 min 29 s |
| workflow | | 3 min 38 s |

Each job spends about a minute compiling. Test times vary with the runner CPU
(AMD EPYC 7763, 9V74 or 9V45): 66 to 114 s for Release, 137 to 169 s for
`checked`.

**Debug kernel tests.** The unoptimized Debug run took 2,085 s of tests in
run 42584921441, dominated by the CDEF, high-bitdepth inverse transform and
quantize tests at `opt-level = 0`. What it adds over Release is the Debug
translation profile, which `debug_assertions` selects, and the assertions it
enables, including the standard library's precondition checks on raw pointers
and slices. The `checked` Cargo profile keeps both: it inherits `release` and
sets `debug-assertions = true` (overflow checks are off in both profiles).
Only the optimization level differs, and for Rust code without undefined
behavior that changes no result, because Rust never contracts or relaxes
floating-point operations. Pull requests therefore run `checked`. The
unoptimized run moves to the nightly schedule and to manual dispatch
(`debug: true`). As its own job it took 20 min 48 s in run 42604319762
(1,197 s of tests on an EPYC 9V45; the 2,085 s above ran on an EPYC 7763). A
defect whose symptom depends on the optimization level is found there a night
later.

**Golden outputs.** `tools/check_aom_golden.py` derives its corpus from
upstream libavif test PNGs with the Python standard library alone (decode,
16-to-8-bit rounding, crop, transpose, mirror), so runner image updates of
ImageMagick or `sips` cannot change the inputs. Its PNG decoding matches
ImageMagick byte for byte on all sources. The 62 cases cross six images (up to
1368x2048, odd sizes, grayscale, text with alpha) with Ferrixel's settings: 8,
10 and 12 bits, 4:2:0 and 4:4:4, quality 20 to 100 including lossless, speeds
4 to 10, one thread and four threads with automatic tiling, and RGBA input with
an alpha plane. Thread counts are fixed because libavif derives the tile
layout from them. Decoded-pixel hashes are not recorded: the decoder is
deterministic, so they follow from the AVIF bytes.

macOS ARM64 with Neon and GNU Linux x86-64 with AVX2 produce byte-identical
output in all 62 cases, so both share `crates/aom/golden/release.json`, which
lists each profile with its translation record digest and compiler. A profile
whose results differ would get its own `crates/aom/golden/<profile>.json`.

The golden outputs were first recorded from the Rust port at the frozen
translation.

**C confirmation (2026-11-09).** The record now holds the C reference output
of each case next to the Rust one. `tools/encode_still.c` makes the same
libavif calls and links the original libavif baseline, the portable libaom
reference (`_c` functions only, `-ffp-contract=off`) and the SharpYUV and
dav1d references that root parity validation builds. All 62 C outputs equal
the Rust golden outputs on GNU Linux x86-64 (clang 16.0.6, run 42611941653)
and on macOS ARM64 (Apple clang 21.0.0, local, libavif without the dav1d
decoder). Because the pull-request tier builds no C library, the C side runs
in `aom-golden-c.yml` nightly, on dispatch and on pull requests that change
the record; on Linux it takes 3 min 22 s.

## Open questions

- Whether hosted runners measure stably enough for the 2 % threshold, or the
  benchmark needs repeated paired runs (`main` and the pull request on the
  same runner) to stay below that noise.

- Which further upstream switches the final regeneration uses: whether the
  decoder, real-time paths or other units can be dropped for Ferrixel's use.
- Whether the end-to-end corpus needs more images or settings, such as tuning
  modes or larger images, once Ferrixel's settings settle. The first corpus is
  described in the amendment above.
- Whether the macOS and Linux C references produce identical golden outputs
  under the non-contracting floating-point profile. Resolved for the first
  corpus: both C references and the Rust port produce the same 62 outputs.
