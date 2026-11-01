# Validation evidence

## Reference baseline

- Upstream: `9b07a9d7a874b159bb4e14b80a6f08bb469d06f7` (libavif 1.4.2).
- Host: macOS, Apple Silicon (`aarch64-apple-darwin`).
- Build: Release, static library, AppleClang 21, CMake 4.4.5, Ninja.
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

Every registration above passes, and case/skip inventories match its C baseline.
The additional AV1 configuration uses libgav1 v0.20.0, rav1e 0.8.1, SVT-AV1
4.2.0, and libyuv revision `26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0`.
Use `tools/run_tests.py` to isolate temporary directories: upstream metadata
tests use fixed temporary file names, so separate CTest suites otherwise collide
when run concurrently. Initial concurrent reference runs exposed this harness
collision; reruns in isolated directories passed without changing the tests.

## ABI and differential evidence

`tools/check_abi.py` passes 3,136 size, alignment, and field-offset comparisons
across 389 named record instances in the default configuration. Seventeen
anonymous/bitfield record instances are explicitly excluded from this automated
layout probe; the original end-to-end tests still exercise their consumers.

Ten additional deterministic encoding scenarios produce byte-identical AVIF
files across C and Rust (8/10/12-bit, chroma sampling, lossless, alpha, SharpYUV,
progressive output, transforms, color profiles, and metadata). Nine decoded PNGs
are byte-identical. The large JPEG-derived case differs due to floating-point
contraction in the original Clang RGB conversion; its PNG is byte-identical to a
separate C decoder with only `reformat.c` compiled using `-ffp-contract=off`.
`tools/differential.py` fails strict PNG parity; `--diagnose-float` verifies and
reports the cause without certifying full PNG byte parity. See ADR 0005.

Optional upstream configurations (other codec adapters, experimental syntax,
golden tests requiring MP4Box, FuzzTest, ComplianceWarden, other platforms) remain
in the untouched source snapshot and require separate validation.
