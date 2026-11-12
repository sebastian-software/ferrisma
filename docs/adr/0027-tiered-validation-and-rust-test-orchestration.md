# ADR-0027: Tier validation and use Rust test orchestration

- **Status:** Accepted
- **Date:** 2026-11-10
- **Last reviewed:** 2026-11-12
- **Supersedes:** The cadence requirement in [0026](0026-codecs-in-one-repository-and-workspace.md), decision 7; the ordinary PR C++ oracle and every-change local macOS requirements in JXL's imported ADR-0002 and PR #61, as described below
- **Related:** [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md)

## Context

The consolidated workspace needs useful feedback for a scoped change without
running every multi-hour reference suite on every iteration. The independent
references, architecture coverage, and provenance remain essential evidence.
The maintainer chose a shared Rust control plane and a tiered cadence on
2026-11-10. A roughly ten-minute PR feedback budget is a target, not a measured
result; a 20-minute per-job timeout is a safety boundary, not proof of speed.

## Decision

An always-running changed-path detector fails closed. It selects affected codecs
and engine/adapter checks; changes to common configuration fan out to all
components, and documentation-only changes skip heavy checks. PR checks use
native Linux x86-64 and Linux ARM for JPEG, WebP, and JXL Rust SIMD regressions;
AVIF runs only on supported GNU Linux x86-64, with local macOS evidence as
appropriate. They include optimized checked Rust unit and retained reference
fixture tests without fresh live C/C++ builds, stable Clippy for handwritten
code, MSRV, provenance and license checks, 62 C-confirmed AVIF AOM goldens,
and 2,880 SharpYUV cross-port cases. JXL's retained captures, public decoder,
effect fixtures, and release-only test binaries provide reference coverage
without the C++ oracle. A live differential feature still fails if its required
oracle is missing; it must never silently skip comparisons.

A central daily nightly and manually selected branch runs exercise the same
commit, with at most two codec families in parallel. Nightly covers both
architectures, all source profiles, sanitizers, larger corpora, and audits where
supported. When all components are selected, the central run also includes the
macOS workspace, M1 fuzz and performance smokes, and kernel benchmarks.
Existing exhaustive AOM original-consumer, recovery, and translation
workflows remain manual: their absence is not full nightly certification.
Changes to oracle pins, goldens, unsafe ABI, SIMD, or floating-point semantics
require a relevant deep branch run or equivalent recorded native evidence before
acceptance. Routine iterations do not require all long suites. Nightly failures
block release and prompt a minimal regression fixture for the PR suite. Never
silently refresh goldens. Existing M1, license, and codec adoption gates remain.

Use the standard-library Rust `tools/xtask` as the shared control plane; use
TypeScript where warranted. Do not add a Python orchestrator. Existing Python
oracle harnesses remain compatibility assets until incrementally ported with
evidence: preserve probe protocol, fixtures, provenance, and error semantics,
and compare results before replacement. The ten engine, adapter, and codec
packages remain separate; the tooling crate is an additional workspace member.

## Consequences

Scoped feedback avoids unrelated expensive work, but retained fixtures cannot
replace live differential checks for high-risk changes. Nightly and branch-run
results must be visible and acted upon. An explicit fail-closed detector and
oracle contract prevent a missing suite from appearing successful.

## Alternatives considered

Running all references on every PR provides immediate breadth but makes ordinary
iteration impractical. Replacing references with fixtures permanently loses
independent evidence. Rewriting all Python harnesses immediately risks changing
protocols and losing provenance; incremental verified ports preserve both.

## Validation

Migration evidence on macOS ARM includes JPEG SIMD, JXL C++ oracle, WebP SIMD
and scalar profiles, SharpYUV cross-port and C comparisons, AOM goldens,
provenance hashes, MSRV, engine, and adapter checks. These results predate this
policy and do not validate its cadence or runtime target.

On 2026-11-10, `cargo check --workspace --all-targets --locked --offline`
passed for all 11 workspace members. The Rust orchestrator passed seven unit
tests, one CLI test, and strict Clippy; its tests cover actual Git rename,
deletion, and invalid-revision failure paths. All four codec PR stages passed
locally on a warm macOS ARM checkout. Actionlint passed for all 27 workflow
YAML files. These local results do not establish hosted timing or coverage.

Hosted PR runs [42805567476](https://github.com/sebastian-software/ferrisma/actions/runs/42805567476) and [42819064673](https://github.com/sebastian-software/ferrisma/actions/runs/42819064673) completed successfully on 2026-11-10. These runs demonstrate their tested revisions and scopes, not a timing guarantee for all PRs. Continue validating representative documentation, codec, common-configuration, and high-risk changes, including missing-oracle failure and native architecture coverage. Record timings and failures before claiming the budget is met.

## Review when

Review when selection misses a dependency, PR timing or nightly coverage is
measured, a supported host changes, or fixture and live oracle results diverge.

## Amendment log

- 2026-11-10 — Accepted the maintainer's scoped PR and deep-validation policy;
  the earlier cadence retained the imported workflows during migration and is
  superseded only where specified above.

- 2026-11-10 — Recorded local orchestration and workflow-lint evidence; hosted fast CI validation remains pending.

- 2026-11-10 — Recorded two successful hosted PR runs; broad timing and deep-tier evidence remain separate.

- 2026-11-12 — Issue #119: the JXL nightly shard `Tests (ubuntu-latest,
  sanitized oracle, 3/4)` was cancelled after the hosted 360-minute job limit
  on 2026-11-08 and 2026-11-11. The job log shows
  `thread_parallel_runner_test_pool` hanging in the Rust thread runner's
  readiness barrier, the duplicate-wake race fixed in commit `592a6b8b`
  (libjxl-rs ADR 0003); both failing commits predate it, and the shard passed
  at `90b638dd`. The JXL parity jobs now set a 120-minute timeout so a hang
  fails fast. The remaining JXL nightly failure, the decoder-state audit, ran
  workspace Clippy with the pinned nightly; it now uses the port's stable
  Clippy like `tools/check.py`.
