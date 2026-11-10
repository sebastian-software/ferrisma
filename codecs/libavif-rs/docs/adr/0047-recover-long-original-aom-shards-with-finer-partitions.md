# Recover long original AOM shards with finer partitions

Keep the existing complete-suite runs and every successful original shard.
Recover timed-out shards from their exact original C and Rust executable
artifacts. Do not rebuild or optimize the codec, change assertions, shorten
input loops, disable cases, or change thread parameters to fit the runner.

The Linux Debug shards 4 and 5 in run 41548531529 reached GitHub's hosted
six-hour job limit. GitHub records this as cancellation. Shard 4 completed
all 144 C cases with no failures and one original dynamic skip; Rust was
still running its original SVC cases when the job was terminated. This is
incomplete acceptance, not a passing result or a demonstrated numeric defect.

Partition each recovered 120-way shard into eight original GoogleTest shards.
For original index `i`, use total 960 and indices `i + 120*p` for `p` in
0 through 7. These partitions cover exactly the original active cases for
index `i`, preserve their relative registration order within each process,
and distribute long test families across jobs. Each finer partition runs
unchanged C and Rust binaries with the same original inputs and parameters.

Require all eight passing C/Rust pairs, identical binary/configuration/source
and fixture identities, and exact ordered case/skip identities. Reparse every
original XML before merging. Interleave the finer partitions into the original
registration order, rejecting missing parts, duplicates, failures, differing
skips, or changed execution contracts. Retain each individual report and XML
digest. A recovered record explicitly describes separate process executions;
it does not claim a single combined XML or process ran successfully.

The complete-suite collector must validate recovered evidence as well as
ordinary pairs, preserve the exact original per-shard case identities, and
require all active original cases once across the complete suite. Recovery
of selected shards alone is not complete acceptance. Record all source and
recovery run IDs when the complete result is assembled.

Use `aom-original-collect.yml` for final native collection. Download all
original source artifacts and only complete successful, disjoint recovery
artifacts. Reject attempts to replace passing source results or hide explicit
original failures. Reparse ordinary XML as well as every recovered subshard's
XML, then require all six profile/program inventories through the complete
collector. Preserve the final report, source/recovery run IDs, original
summaries, and individual XML evidence. The original source workflow's
time-limit conclusion remains recorded; validated recovery does not rewrite it.

The [GitHub Actions limits](https://docs.github.com/en/actions/reference/limits)
document the hosted six-hour execution limit and the 256-job matrix limit.
Keep each recovery matrix at most 240 jobs and retain the existing native
macOS ARM64 and GNU Linux x86-64 reference boundaries.
