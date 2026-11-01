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

Optional upstream configurations (other codec adapters, experimental syntax,
golden tests requiring MP4Box, FuzzTest, ComplianceWarden, other platforms) remain
in the untouched source snapshot and require separate validation.
