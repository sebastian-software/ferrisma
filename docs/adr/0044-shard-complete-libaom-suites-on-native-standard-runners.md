# Shard complete libaom suites on native standard runners

Keep the full original Debug and Release suites and change their scheduling
when unoptimized codec execution exceeds practical job budgets. A measured
local sample of the same first 35 completed C cases took 2,830 seconds in
Debug versus 456 seconds in Release. This is a partial timing sample, not a
full-suite runtime estimate or a passing Debug result. Pairing both languages
inside a coarse 32-way shard risks the hosted runner's six-hour job limit.

Use 120 paired main-suite shards per profile, plus the four original
rate-control/speed jobs: 244 matrix jobs, below GitHub's 256-job matrix limit.
Interleave profiles and use at most eight simultaneous Ubuntu jobs or four
native macOS ARM64 jobs. Keep local execution limited to two CPU-intensive
processes. Standard `ubuntu-24.04` and `macos-26` runners are free for this public
repository; larger runners are outside this decision. See GitHub's
[runner specifications](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
and [workflow limits](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax).

Build each host's native and Rust binaries once. Verify the committed source
closure, configuration, original registrations, and Rust symbol ownership
before distributing those exact binaries. Check named layouts against the
hosted macOS SDK as well. Fetch and checksum every selected original fixture
in each main-suite job. Collect only complete, matching per-case C/Rust results
with identical binary, translation, configuration, and fixture identities.

Shard boundaries do not change assertions, inputs, thread parameters, numerical
references, disabled flags, or expected outputs. Preserve the earlier 32-shard
workflow and its diagnostics; partial or superseded execution receives no full
acceptance credit. A timeout or any missing/failing shard remains a failed
gate. This scheduling decision does not approve the final root integration.
