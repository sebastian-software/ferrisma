# Portable SharpYUV validation

The standalone `libavif-sharpyuv` crate mechanically translates SharpYUV 0.4.2
from libwebp 1.6.0 at `4fa21912338357f89e4fd51cf2368325b59e9bd9`. The selected
boundary is the complete original `sharpyuv` target with SIMD disabled and
threading enabled: five scalar/core sources and two original disabled-SIMD
translation units. WebP's codec remains original test infrastructure.

All 343 imported libwebp files and all 429 existing libavif files match their
source manifests. C2Rust 0.21.0 / LLVM 16.0.6 regeneration reproduces all seven
Rust units and their correction records. Original BSD terms, authorship, and
the separate patent grant are preserved and checked by the generated notice
workflow. No original test or fixture is edited.

An additional generated x86 variant preserves the CPU-query branch that remains
compiled even when SIMD conversion is disabled. ADR 0023 records the CPUID
register mapping; a separate original C probe compares callback initialization
and all ten feature-query results on each host.

## Exact comparisons

`tools/check_sharpyuv.py` runs C and Rust in separate processes. Each profile
passes 1,271 comparison groups with exact return values, matrices, and output
bytes. The inventory includes:

- All 960 RGB depth / YUV depth / transfer / predefined matrix combinations:
  RGB 8/10/12/16, YUV 8/10/12, all sixteen defined transfer functions, and all
  five predefined matrices.
- Interleaved RGB, RGBA channel steps, and planar input; padded rows; aligned
  16-bit storage; nine rotating shapes including narrow, odd, and even images.
- 256 supplemental seeded conversions with dimensions up to 64 by 32.
- Eighteen computed matrices, version/options behavior, stable matrix pointers,
  defined null/invalid-argument rejection, repeated callback initialization,
  and 24 cold conversions across eight concurrent threads.
- Input immutability and output guard bytes. Invalid cases that invoke C
  undefined behavior are excluded from the equivalence oracle.

This inventory does not claim the complete Cartesian product of shapes and
layouts, every possible input value, or a proof of equivalence.
The 1,271 groups contain multiple calls in some API and concurrency checks.

Development and Release comparisons pass on physical macOS ARM64 and on
independent native GitHub macOS ARM64 and Linux x86-64 runners. Their original
consumer inventories and durable evidence are linked in the machine-readable
record. Earlier Linux emulation runs remain supplemental local diagnostics.

The pinned Rust compiler is nightly-2026-09-07 (LLVM 23.1.1). Primary C references
explicitly use `-ffp-contract=off`; Rust uses its normal arithmetic and an
unoptimized development profile with assertions enabled. This selected contract
is recorded in ADR 0024. It supersedes the compiler-contraction experiment in
ADR 0021 without changing translated expressions.

The compiler-default Clang diagnostic retains the 47/1,271 differences observed
on the local macOS host. Its separate reports are explicitly excluded from
acceptance. The unchanged primary comparison continues to require exact results.

## ABI, cleanup, and memory checks

Each target passes 23 C/Rust size, alignment, offset, pointer-representation,
and direct Rust API checks. The matrix type shared by the original C headers
also has shared identity between translated Rust modules.

Test-only allocator symbols inject failure at each of the seven conversion
allocations. Both implementations return failure and free all successful
allocations; the successful control call also leaves no allocations live.
Production builds use normal libc allocation and contain no test allocator.

On macOS, the same C and Rust allocation paths and common C driver pass with
AddressSanitizer. The full original libavif default suite also passes with the
new Rust dependency instrumented. Native consumers use LLVM 16 and the matching
Rust standard ASan runtime, rather than mixing independent runtimes. This does
not establish complete race, undefined-behavior, or leak-sanitizer coverage.

## Original consumers and integration

All twelve original WebP FuzzTest executables pass against both the C reference
and Rust SharpYUV. Their twelve GoogleTest registrations and skip inventories
match. They use the recorded fixed PRNG seed and original bounded unit-test
mode, rather than a continuous fuzz campaign. Built-in source images exercise
encoding and decoding. The external WebP corpus is not part of this run.
ADR 0022 records root test registration, dependency build compatibility, and
the original ARM CPU-query configuration required by the native fuzzer.

The existing libavif integration passes again after replacing native SharpYUV:

| Configuration | CTest registrations | GoogleTest cases |
| --- | ---: | ---: |
| Default Release | 64 | 9,473 |
| Default Debug | 64 | 9,473 |
| All Cargo features | 70 | 9,555 |
| Default AddressSanitizer | 64 | 9,473 |

The root libavif Rust Debug row uses its existing C Release preprocessing and
C Release reference. The standalone SharpYUV Debug comparisons independently
use Debug C preprocessing and retain assertions in both implementations.

Case and skip inventories match the original C baselines. The shared C ABI
smoke test decodes the original 33-by-11 fixture and encodes a 502-byte AVIF.
All ten deterministic AVIF bitstreams and decoded PNGs match the primary C
reference byte-for-byte with the explicitly noncontracting C configuration.
The separate compiler-default libavif diagnostic retains the earlier result:
ten matching AVIF files and nine matching PNGs, with the remaining PNG matching
the noncontracting C decoder. It is excluded from the acceptance reference.

Archive-member, defined-symbol, compile-command, and dynamic-linkage checks
confirm that the Rust crate supplies SharpYUV, including `SharpYuvInit`, in the
WebP and libavif consumers. No original or system SharpYUV implementation masks
the port. C/C++ test code, native AV1 engines, libc, pthreads, and system math
remain allowed infrastructure/dependencies at this milestone.

## Remaining scope

dav1d and libaom are the next separate crates. Other targets, SharpYUV SIMD,
safe API design, optimization, a broader external corpus, continuous fuzzing,
and registry publication remain separate work. Linux support of this crate
does not enable the root libavif package on Linux. Optional golden tests and
libavif's own optional FuzzTest/ComplianceWarden matrix retain their prior gaps.

Reproduction commands are in [CONTRIBUTING.md](../CONTRIBUTING.md). Source,
artifact, and JUnit hashes are recorded in
[`sharpyuv-validation-results.json`](sharpyuv-validation-results.json).
