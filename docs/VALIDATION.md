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

Optional upstream configurations (other codec adapters, experimental syntax,
golden tests requiring MP4Box, FuzzTest, ComplianceWarden, other platforms) remain
in the untouched source snapshot and require separate validation.
