# Require complete Release and focused Debug AOM acceptance

Require the complete unchanged original C/Rust Release suites on native Linux
x86-64 and macOS ARM64 before making Rust AOM the root default. Replace the
complete unoptimized Debug gate from ADRs 0044, 0045, and 0046 with focused
Debug checks for API errors, allocation and worker failures, and the known
high-bit-depth input-alignment regression. Keep final root sanitizer checks,
source correspondence, ABI, symbol ownership, and strict AVIF/PNG parity gates.
Final root integration uses the complete Release and supported-feature suites,
focused Debug regressions, and the existing root ASan checks.

The first four matching complete Linux shard pairs took 15–19 times longer
in Debug than Release; the corresponding macOS pairs took 13–15 times longer.
For example, Linux shard 0 took 343 minutes in Debug and 22 minutes in Release.
These are measured shard samples, not whole-suite estimates or codec benchmarks.
Long original video-encoding cases dominate the runtime. Repartitioning avoids
individual job limits but does not reduce the underlying unoptimized work.

Debug adds internal assertions and Rust runtime checks; symbols alone do not
require an unoptimized build. Keep focused checks that can discover failures
such as the corrected unaligned sample load. Use additional Debug execution
when a specific failure warrants it. Full unoptimized Debug suites remain
available on explicit request and are no longer a default-switch prerequisite.
This deliberately gives up exhaustive Debug-configuration execution coverage.

Preserve every original test source, assertion, parameter, fixture, golden,
registration, and disabled flag. Complete Release acceptance still requires
all 17,228 active main-suite registrations, the 38 rate-control registrations,
and the original disabled-speed boundary per platform, with matching C/Rust
case identities and dynamic skips. Keep independent C oracles and separate
supplementary cases identified as such. Do not claim complete Debug acceptance.

Stop the old mixed full-suite and Debug-recovery runs after retaining their
published artifacts. Continue the missing Release partitions from the exact
source-run binaries, validating and reusing complete original Release evidence.
Retain the old workflow conclusions and partial Debug evidence as historical
results. This changes the acceptance policy, not the codec implementation or
its compiler profiles. Native VMAF and the separately scoped optional engines
remain unchanged.
