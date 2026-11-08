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

## Open questions

- Whether hosted runners measure stably enough for the 2 % threshold, or the
  benchmark needs repeated paired runs (`main` and the pull request on the
  same runner) to stay below that noise.

- Which further upstream switches the final regeneration uses: whether the
  decoder, real-time paths or other units can be dropped for Ferrixel's use.
- The composition of the corpus and the settings matrix for the end-to-end
  golden outputs.
- Whether the macOS and Linux C references produce identical golden outputs
  under the non-contracting floating-point profile.
