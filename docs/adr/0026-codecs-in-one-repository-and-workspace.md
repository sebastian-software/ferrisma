# ADR-0026: Develop the image engine and codecs in one repository and workspace

- **Status:** Accepted
- **Date:** 2026-11-10
- **Last reviewed:** 2026-11-10
- **Supersedes:** The separate-workspace requirement of [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), decision 2; codec-local development toolchain and profile placement where specified below
- **Related:** [0018](0018-optional-codec-and-cms-dependencies.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md)

## Context

Ferrixel and the four sister Rust ports have the same integration destination.
Changes to buffer ownership, rendered color, decoder access and encoder input
need a codec change and a matching pipeline change. Separate repositories make
that work depend on coordinated pull requests and Git dependency revisions.
The ports also maintain related oracle runners, provenance checks and CI.

The maintainer approved consolidation on 2026-11-10, including one Cargo
workspace, prompt review of duplicate implementations, and no permanent archive
arrangement for the young source repositories. Importing source does not approve
new codec selection, a public API, publication or changes to image semantics.

Two existing differences affect correctness, rather than only convenience:
the generated AVIF packages rely on wrapping integer arithmetic with overflow
checks disabled, and JXL's differential reference disables vectorization to
preserve signaling-NaN observations. Consolidation must preserve those checks
until a separately verified implementation makes the exception unnecessary.

## Decision

1. Ferrixel owns the four ports under `codecs/`. Import the recorded `main`
   revisions together with the eight pull requests open at migration start and
   their commit history, retaining copyright notices, licenses, patent grants,
   source mappings, test inputs and codec-local ADRs. Each `IMPORT.json` records
   the original repository, main and integrated commits, tree and PR numbers. The original
   repositories need not remain as historical archives. Open changes and issue
   references must be accounted for before their retirement; import alone is
   not evidence that unmerged work has been transferred.
2. Use one Cargo workspace and one lockfile for the engine, adapter and codec
   packages. Use the existing AVIF pinned nightly as the common development
   toolchain. Preserve the engine's stable build and each package's declared
   MSRV through explicit CI commands. JPEG, WebP and JXL retain stable Clippy: the pinned AVIF nightly has different lint expectations and fails unchanged mechanical loops. This shared verifier exception preserves strict linting without rewriting translated algorithms. Internal dependencies use local paths.
   Independent versions and eventual crates.io packages remain possible.
3. Centralize profiles. `checked` means optimized code with debug assertions
   and overflow checks. Preserve the generated AVIF packages' overflow setting
   through explicit package overrides, without weakening checks for the engine
   or the other codecs. Select JXL's reference flags explicitly for its parity
   run; do not impose them on the engine's performance builds.
4. Preserve crate boundaries: the engine forbids unsafe code; codec and SIMD
   boundaries retain their scoped policies. Codecs do not depend on Ferrixel.
   The existing adapter owns the integration with libavif. The known dav1d
   symbol collision still forbids linking that adapter with the interim AVIF
   reader; validate supported feature combinations explicitly.
5. Reuse verification infrastructure after identifying equivalent behavior.
   Codec probes, C/C++ references, comparison rules and provenance remain
   independently reviewable. Missing required oracles fail their CI jobs.
   Golden outputs change only with their declared reference or an explicitly
   approved semantic change. Mechanical import never refreshes expected output.
6. Review duplicate SharpYUV implementations during consolidation. Compare
   source revisions, public contracts, allocation/callback behavior, numeric
   results and link symbols before selecting a shared implementation. Record
   the outcome and the smallest remaining adoption gate in the owning ADR;
   coexistence is a reviewed migration state, not an indefinite default.
7. Preserve the existing codec verification tiers when moving workflows to the
   repository root. Namespace check and artifact names; keep component working
   directories explicit. Share orchestration without silently dropping oracle,
   sanitizer, architecture, MSRV or nightly checks.
8. Retain package-specific licenses and publication decisions. Codec membership
   does not change the M1 acceptance criteria or the adoption gates of ADR-0025.

## Consequences

- One commit can contain a codec contract change, its Ferrixel consumer and
  their end-to-end regression evidence.
- Development uses one resolved dependency graph, while package APIs, licenses
  and release versions stay independently defined.
- The common checkout includes AVIF's substantial generated source. Component
  checks must avoid unrelated expensive work.
- Nightly becomes the development default. Explicit stable/MSRV checks prevent
  accidental nightly requirements in packages that promise stable support.
- Root profiles and feature unification are correctness boundaries. A generic
  workspace-wide `--all-features` command is not a substitute for the supported
  codec configurations or independent oracle execution.

## Alternatives

- Separate repositories with shared scripts: retains cross-repository changes
  and dependency-pin coordination.
- One repository with permanently separate workspaces: rejected by the
  maintainer; central configuration is the intended outcome.
- Submodules for the Rust ports: retains multiple implementation commits per
  change. Pinned upstream C/C++ references may remain submodules.
- Merge codec implementations or APIs during import: mixes structural and
  behavioral changes and weakens attribution of failures.

## Validation

Verify workspace membership and the local dependency graph with Cargo metadata.
Run engine, codec and adapter checks in their declared profiles and supported
feature combinations. Verify upstream pins and translation manifests unchanged.
Run the native oracle checks after relocating their harnesses. Record actual
results when available; this decision alone makes no validation or performance
claim. A clean checkout must reproduce source setup without sibling repositories.

Migration evidence recorded on 2026-11-10 (macOS ARM64):

- Formatting, workspace compilation, engine tests and strict Clippy, adapter
  release tests and strict Clippy, and the declared Rust 1.95/1.98 build checks
  pass. The generated AVIF packages retain their existing compiler warnings.
- JPEG's complete SIMD oracle run, JXL's ordinary C++ oracle run, and WebP's
  SIMD and scalar oracle runs pass. JPEG and WebP exercise dev, release, and
  checked profiles. The relocated AVIF tooling passes 114 tests (one existing
  platform skip); all 62 AOM golden cases match their C-confirmed expectations.
- The SharpYUV results below pass. All 24 workflows pass Actionlint.
- A separate clean clone builds all ten workspace members offline. Its local
  dependency paths stay inside the checkout; reference setup fetches the
  original public repositories and passes each port's pin checks. All 2,651
  embedded upstream Git blobs, 1,380 generated Rust files, and generated license
  records retain their recorded identities. No golden outputs were refreshed.
- All eight original PR heads are ancestors of the migration branch. The
  history-only merge leaves the imported workspace tree unchanged.

These local results do not replace native GNU Linux or the full sanitizer and
original-consumer acceptance tiers. Those remain in the relocated workflows.
Additional WebP x86 tests pass under Rosetta; that is emulated coverage, not
native Linux acceptance. Reproduce the local tiers with the commands in the
[codec guide](../codecs.md).

## Review when

- A generated AVIF package can enable overflow checks or use stable Rust.
- JXL no longer needs its reference-only vectorization flags.
- SharpYUV parity and adapter requirements permit one implementation.
- Workspace feature unification changes a supported integration's behavior.
- A codec acquires an independent ownership or release requirement that the
  shared repository cannot serve.

## SharpYUV consolidation review, 2026-11-10

The two ports use SharpYUV 0.4.2 from libwebp 1.6.0. All 12 imported C/header
files in `sharpyuv/` compare byte-for-byte equal. The new
[`cross_port` test](../../codecs/libavif-rs/crates/sharpyuv/tests/cross_port.rs)
compares the existing safe WebP conversion with the existing AVIF C ABI in the
same process: 2,880 combinations of four RGB depths, three YUV depths, sixteen
transfer functions, five predefined matrices, and three dimensions, including
odd and one-pixel shapes. On macOS ARM64 it passes in both `release` and
`checked`, including padded output bytes and unchanged input. The AVIF port's
independent C comparison also passes all 1,271 API/corpus groups. These results
support sharing conversion logic; they do not establish interchangeable APIs.

Select the safe WebP implementation as the candidate for a shared conversion
core. The immediate migration shares the cross-port acceptance gate in CI. The
next SharpYUV implementation change should extract that core and retain the
AVIF entry points as a thin compatibility adapter, subject to these specific
adoption gates:

- Preserve `SharpYuvInit`, the independent CPU callback, and concurrent cold
  initialization. WebP deliberately has no SharpYUV C ABI and routes its CPU
  query through WebP's state; using it directly would change this contract.
- Preserve C allocation-failure results, ownership, alignment, defined invalid
  arguments, signed strides, and the existing 16-bit truncation behavior.
- Pass the existing C API, ABI, allocation, callback, linkage and original
  consumer suites on native macOS ARM64 and GNU Linux x86-64, plus both ports'
  numeric suites. Keep the independent C oracle after removing duplicate Rust.
- Measure conversion time and peak memory before selecting a replacement;
  this import makes no performance claim about either implementation.

Replacing AVIF's ABI with a dependency on the entire WebP codec is rejected:
it would couple unrelated codecs and leave the callback contract unresolved.
This review therefore establishes an extraction direction and executable parity
evidence now, without declaring an unimplemented adapter equivalent.

## Amendment log

- 2026-11-10 — Accepted the maintainer's consolidation decision and corrections:
  one workspace, early duplication review, no permanent source-repository
  archives. Implementation and verification are separate from acceptance.

- 2026-11-10 — The maintainer requested inclusion of all open changes immediately: JPEG #23, WebP #2/#40/#41/#42/#44, JXL #61 and AVIF #65. Their original commits are retained; x86 and NEON dispatch conflicts are resolved together. Stable Clippy remains explicit after verifying that the AVIF nightly rejects previously passing lint expectations.
