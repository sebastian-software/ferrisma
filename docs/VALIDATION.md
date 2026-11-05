# Validation evidence

The compact machine-readable snapshot is
[`validation-results.json`](validation-results.json), generated from the actual
JUnit inventories by `tools/record_validation.py`.

The dependency-port extension has separate
[SharpYUV evidence](SHARPYUV-VALIDATION.md), including fresh default, Debug,
all-feature, and sanitizer libavif runs after replacing native SharpYUV.
The subsequent [dav1d evidence](DAV1D-PORT.md) records fresh runs of those four
configurations after replacing the decoder with its Rust workspace crate.
The [AOM milestone](AOM-PORT.md) and
[final default validation](aom-default-validation.json) record complete native
standalone Release acceptance on macOS ARM64 and Linux x86-64, followed by
fresh macOS root Release, supported-feature, focused Debug, and AddressSanitizer
checks with mandatory Rust AOM. The final root record includes exact original
case inventories, all consumer/runtime ownership audits, and strict AVIF/PNG
parity against the pinned portable C reference with `-ffp-contract=off`.
The original baseline and intermediate matrices below remain historical results.
The feature-specific intermediate rows below retain their earlier evidence.

The subsequent [native root profiles](PLATFORMS.md) add complete default and
combined Mini/Extended PIXI Release/Debug comparisons on Linux x86-64 and local
macOS ARM64, plus complete default and combined Mini/Extended PIXI Release
AddressSanitizer parity on Linux x86-64. Their evidence remains separate from production target admission
and the existing macOS production record. PRs use bounded native gates; nightly
and manual full tiers retain the complete original integration commands.

## Reference baseline

- Upstream: `9b07a9d7a874b159bb4e14b80a6f08bb469d06f7` (libavif 1.4.2).
- Host: macOS, Apple Silicon (`aarch64-apple-darwin`).
- Build: Release, static library, AppleClang 21, CMake 4.4.5, Ninja. Fresh
  dependency-port runs explicitly set `-ffp-contract=off`; earlier intermediate
  feature runs used compiler-default contraction.
- Codecs: system libaom 3.15.2 (encode/decode), dav1d 1.5.4 (decode).
- Color/scaling: system SharpYUV 1.6.0, bundled scalar libyuv fallback.
- Applications: PNG, JPEG, libxml2 gain-map conversion enabled.
- GoogleTest: v1.17.0 (`52eb8108c5bdec04579160ae17225d66034bd723`).
- libargparse: `ee74d1b53bd680748af14e737378de57e2a0a954`.

`ctest --test-dir build/baseline --output-on-failure -j 4` passed **64/64**
CTest registrations, with no failed or disabled registrations (18.16 seconds).
These registrations include parameterized GoogleTest executables and original
shell integration tests; they are not a count of individual assertions or cases.
Machine-readable results: `build/baseline-results.xml`.

## Rust parity

The corresponding Rust Release build passes **64/64** unchanged registrations.
`tools/compare_results.py` verifies the same names, statuses, GoogleTest case
counts (**9,473**), and in-program skip messages. CMake's exported compilation
database confirms that `avif_obj` contains only an empty linkage anchor; all 28
implementation translation units are Rust, including the bundled scaler.

`tools/import_translation.py --check` reproduces all translated sources exactly
from C2Rust output. `TRANSLATION.json` records per-module raw and corrected
hashes and the applied correction counts. Rust compiler warnings remain visible;
generated style warnings are not a parity failure and safety warnings are not
globally suppressed.

Rust used: `rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)`.

## Additional configurations

| Configuration | C registrations | Rust registrations | GoogleTest cases in each |
| --- | ---: | ---: | ---: |
| Default Release | 64 | 64 | 9,473 |
| Rust debug, same C Release preprocessing | 64 | 64 | 9,473 |
| Mini | 65 | 65 | 9,526 |
| Extended PIXI | 64 | 64 | 9,474 |
| Mini + Extended PIXI | 65 | 65 | 9,527 |
| libgav1 + rav1e + SVT-AV1 + external libyuv | 65 | 65 | 9,481 |
| AVM/AV2 | 67 | 67 | 9,485 |
| AVM/AV2 + Mini | 69 | 69 | 9,546 |
| AVM/AV2 + Extended PIXI | 67 | 67 | 9,486 |
| AVM/AV2 + Mini + Extended PIXI | 69 | 69 | 9,547 |
| All Cargo features and all six codec adapters | 70 | 70 | 9,555 |
| Default with AddressSanitizer | 64 | 64 | 9,473 |

Every registration above passes, and case/skip inventories match its C baseline.
The additional AV1 configuration uses libgav1 v0.20.0, rav1e 0.8.1, SVT-AV1
4.2.0, and libyuv revision `26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0`.
AVM uses v1.0.0 at `966a7d7cd6fcf60360caf5dc413b2aeeb65e144d`, with
the dependency-only export-list repair from ADR 0008. AddressSanitizer uses
LLVM 16 for native consumers and the matching standard v8 Rust runtime, with
the loader-array handling documented in ADR 0009. The Rust shared library
contains ASan load/store, memcpy, and global-registration instrumentation.
Use `tools/run_tests.py` to isolate temporary directories: upstream metadata
tests use fixed temporary file names, so separate CTest suites otherwise collide
when run concurrently. Initial concurrent reference runs exposed this harness
collision; reruns in isolated directories passed without changing the tests.

## ABI and differential evidence

`tools/check_abi.py` passes 3,136 size, alignment, and field-offset comparisons
across 389 named record instances in the default configuration. Seventeen
anonymous/bitfield record instances are explicitly excluded from this automated
layout probe; the original end-to-end tests still exercise their consumers.
With all features enabled, the same probe passes 4,484 comparisons across 522
named record instances, with 20 anonymous/bitfield instances excluded.

`tools/smoke_shared.py` loads the Rust dylib through its C ABI and exercises
initialization, allocation, decoding, encoding, and cleanup. It successfully
decodes the original 33x11 IDAT fixture and encodes a 502-byte AVIF.

Ten additional deterministic encoding scenarios produce byte-identical AVIF
files across C and Rust (8/10/12-bit, chroma sampling, lossless, alpha, SharpYUV,
progressive output, transforms, color profiles, and metadata). All ten decoded
PNGs are byte-identical against the primary noncontracting C reference without
changing source expressions or normal Rust arithmetic. The earlier nine-of-ten
compiler-default result is documented in ADR 0005; ADR 0024 establishes the
current compiler contract. Clang-default differences remain separate diagnostics.
`tools/differential.py` passes its strict check against the primary reference.

All eight Mini/Extended-PIXI/AVM preprocessor combinations regenerate exactly
through `tools/transpile.py --check`. Optional AV1 adapter and registry
regeneration also reproduces the recorded implementation files.

Remaining work includes other platforms and dependency versions, AOM
encode-only/decode-only configurations, optional golden tests requiring MP4Box,
libavif FuzzTest, the C++ ComplianceWarden integration, and the broader upstream CI
matrix. The original sources, feature gates, fixtures, and workflows remain
available in the untouched snapshot.
