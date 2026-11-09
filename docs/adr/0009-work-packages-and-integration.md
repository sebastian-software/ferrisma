# Deliver the port in work packages that an integrator merges one at a time

- Status: accepted
- Created: 2026-11-09
- Updated: 2026-11-10

## Context

libwebp v1.6.0 is about 30,000 lines of portable C plus 15,000 lines of SIMD
variants. The port was translated by parallel Claude Code sessions, each in its
own worktree and branch, coordinated by one integrator session, between 5 and
9 November 2026, from a work plan that split the source into 23 packages (WP0
to WP22) with disjoint file ownership and ordered them by the call graph into
sub-waves. That plan, the research plan it came from, a status aggregate and
one status page per package were planning and status documents; the
maintainer removed them on 10 November 2026 so that the repository holds the
code, its documentation, the contract and the decisions, and the open work
lives in GitHub issues. This record keeps the rules that proved themselves and
the lessons, so that the remaining packages (the SIMD variants,
[ADR 0008](0008-simd-modules.md)) and later work follow them. Three lessons
shaped the rules: two pull requests that were each green on their own branch
did not compile together (PR 8 and PR 9); a check that ran in a package
session's worktree while that session was rebuilding failed spuriously; and a
C undefined behavior showed only on the Linux runner's UBSan, after the macOS
runs had passed.

## Decision

**Packages and ownership.** `tools/units.json` assigns every upstream unit to
one Rust module ([ADR 0001](0001-mechanical-port.md)) and every module to one
package, with the function- and type-level exceptions that break the C call
graph's cycles; `tools/unit_dependencies.py --check` verifies the declared
dependencies against the oracle's object files, and `tools/units.py` verifies
the module skeleton. A package edits only its own modules, its probes and
tests, and the exception items it is assigned; whoever later owns a module
keeps other packages' exception items unchanged. A change to a module another
package owns, and any interface change to a merged module, goes in its own
commit with the reason in the message, after the integrator agreed.

**One package, one session, one branch, one pull request.** A session gets a
brief with the package's units, dependencies, the representation decisions
taken so far, the verification required and the done criteria, and it stops
with a checkpoint (options and a recommendation, posted as a comment on the
package's issue and sent to the integrator) when it meets a question the ADRs
do not answer. A package that designs an API writes its ADR first, as a
checkpoint, and implements after the decision ([ADR 0006](0006-safe-encoder-api.md),
[ADR 0007](0007-safe-decoder-api.md)). Sessions never push to `main`, never
merge their own pull request, never force-push without asking, never advance
the submodules, never publish.

**Records live with the code.** What a package learned about its units is
written where a maintainer reads it: every module's `//!` documentation
records, under `# Deviations from upstream`, the places where the translation
departs from the C code's shape or observable behavior and why, and under
`# Excluded undefined behavior`, the inputs that are undefined behavior or
assertion failures in C and are therefore excluded from the oracle
comparison (PORTING rule 8); a test file's `//!` documentation records what
it compares, with which inputs, and which cases it excludes. The pull request
describes the evidence with its counts and the local runs. No status page
and no status aggregate exist; what is translated is what the module tree
contains, and the open work is in the issues.

**Integration.** The integrator reviews source correspondence (the C ABI line
by line, the translation against the C where a module records a decision or a
deviation), decides the checkpoints, merges one pull request at a time, and
checks the next one against the new `main` before merging it: a merged tree of
the two heads passes `python3 tools/check.py --oracle --msrv` in a scratch
worktree that no session uses, and the final `main` tree is compared with that
checked tree by id. Verification never runs in a package session's worktree
while that session works.

**Tracking.** The open work is tracked in GitHub issues of this repository:
one issue per package, finding or follow-up, a tracking issue per wave, the
label `work-package` for packages and `upstream` for findings to report to
libwebp. A package's pull request closes its issue (`Closes #N`), and a
checkpoint is also posted as a comment on the issue.

**Continuous integration.** Every pull request runs the `Checks` workflow on
Linux x86-64 and macOS ARM64: the static stage, the tests in three profiles
with and without the SIMD modules, the MSRV job and the oracle jobs against
the `release` and `asan-ubsan` variants with the baseline scripts. The single
required status check, `Checks passed`, accepts only `success` from every
job; an Actions incident once reported jobs as `abandoned`, which the gate
must fail.

**Models.** A package runs on the most capable model where it designs,
integrates, exports a C ABI, translates a codec core or SIMD intrinsics, and
on a smaller model where it is a mechanical, fully specified translation that
the differential tests check exhaustively.

## Consequences

The port's history is readable from the pull requests, the issues and the
module documentation rather than from a plan or a status table: each merged
package left its deviations and exclusions in the modules they concern and its
evidence in its pull request. The rules cost one full check per pull-request
pair. A later contributor adds a package the same way: an issue, a brief, a
branch, deviations in the module docs, a pull request that the integrator
checks against `main` before merging.

## Review triggers

Revisit when the port is complete and maintenance replaces packages, when the
C ABI moves into its own crate, or when the tracking leaves GitHub issues.
