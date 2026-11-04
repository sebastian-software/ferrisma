# libavif-rs

A mechanical Rust port of [libavif](https://github.com/AOMediaCodec/libavif).

All 27 C library source files are represented in Rust, including all six codec
adapters, experimental container branches, and the bundled scalar scaler. Names,
control flow, C layouts, allocation ownership, and diagnostics stay close to the
source. Idiomatic Rust APIs and optimizations are deferred.

The complete upstream snapshot, including its tests and fixtures, lives in
`upstream/libavif`. See `UPSTREAM.json` for the exact source revision and
`docs/PORTING.md` for the migration contract and validation status.

The currently supported and tested target is **macOS on AArch64**. The original
C/C++ applications and tests consume the Rust implementation through the C ABI.
The default AOM, dav1d, and SharpYUV implementations are workspace Rust crates.
libvmaf and the separately scoped optional engines remain native dependencies.

SharpYUV is now a separate mechanical Rust crate in
[`crates/sharpyuv`](crates/sharpyuv), used directly by libavif-rs. Its portable
configuration is validated on macOS ARM64 and Linux x86-64 independently of the
root package. See [SharpYUV validation](docs/SHARPYUV-VALIDATION.md).

The complete portable dav1d 1.5.4 decoder is a separate mechanical Rust crate in
[`crates/dav1d`](crates/dav1d), used by the existing libavif decoder adapter.
Its Debug and Release profiles cover both upstream bit-depth instantiations,
including 8/10/12-bit decoding and film grain. See
[dav1d port and validation](docs/DAV1D-PORT.md) for the original test inventories,
native target evidence, and remaining limits.

## Build and test

Use Rust nightly, CMake, Ninja, Python 3.9+, pkg-config, and development packages
for libvmaf, PNG, JPEG, and libxml2. The original C baseline and CMake test setup
also require native dav1d and SharpYUV development packages; the Rust library
uses their workspace crates. The pinned portable C AOM archive supplies the
independent reference in the commands below. The validated dependency
versions and exact results are recorded in [docs/VALIDATION.md](docs/VALIDATION.md).

```sh
cargo build --release --locked
python3 tools/build_aom.py --jobs 1
python3 tools/build.py baseline --fetch-deps --aom-reference-build build/aom-native-release
python3 tools/run_tests.py build/baseline-aom-reference
python3 tools/build.py port
python3 tools/run_tests.py build/port
python3 tools/compare_results.py --baseline build/baseline-aom-reference-results.xml
python3 tools/check_translation.py
python3 tools/check_abi.py --build build/baseline-aom-reference
python3 tools/smoke_shared.py
```

The default configuration passes the same 64 upstream test registrations in C
and Rust. The complete supported feature configuration passes 70 registrations
containing 9,555 GoogleTest cases, plus the original C and shell integration
tests. AddressSanitizer parity also passes the 64 default registrations.

Cargo emits `target/release/libavif.a`, `libavif.dylib`, and `libavif.rlib`.
Use the original headers in `upstream/libavif/include`. Exported Rust functions
retain the original unsafe pointer and callback contracts.

## Optional features

| Cargo feature | Original configuration |
| --- | --- |
| `mini` | `AVIF_ENABLE_EXPERIMENTAL_MINI` |
| `extended-pixi` | `AVIF_ENABLE_EXPERIMENTAL_EXTENDED_PIXI` |
| `libgav1` | libgav1 decoding adapter |
| `rav1e` | rav1e encoding adapter |
| `svt` | SVT-AV1 encoding adapter |
| `libyuv` | External libyuv conversion and scaling |
| `avm` | Experimental AV2 codec and container support |

libaom, dav1d, and SharpYUV are enabled in the base generated configuration.
For optional native dependencies, translation, sanitizer commands, and the full
feature test procedure, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Current limits

Other platforms require their own platform translation and ABI validation.
Optional libavif golden tests, libavif FuzzTest, and the full upstream platform CI matrix are
preserved but have not been executed here. The optional C++ ComplianceWarden
integration remains preserved upstream and is not enabled by the Rust build.

All ten differential AVIF bitstreams and all ten decoded PNGs now match the
noncontracting C reference byte-for-byte. The source expressions remain unchanged;
C references explicitly use `-ffp-contract=off`, and Rust uses normal arithmetic.
Clang-default differences are recorded separately. No upstream test or tolerance
was changed; see [ADR 0024](docs/adr/0024-use-noncontracting-c-references-for-portable-ports.md).

## Planned dependency ports

The [dependency porting plan](docs/NATIVE-PORTS.md) defines separate mechanical
Rust crates in a shared workspace. Portable SharpYUV and dav1d are implemented;
portable libaom has passed complete native macOS and GNU Linux standalone
Release acceptance and focused Debug checks (see
[the libaom milestone](docs/AOM-PORT.md)). The plan records
bit-exact reference checks, original-test correspondence,
license provenance, and the macOS ARM64 / Linux x86-64 validation gates.

Rust AOM is mandatory in the default build; no temporary feature or production
system-AOM discovery is required. The selected portable configuration preserves
native VMAF and the separate optional-engine scope. See
[ADR 0049](docs/adr/0049-use-rust-aom-as-the-mandatory-root-codec.md) and the
[acceptance policy](docs/adr/0048-require-complete-release-and-focused-debug-aom-acceptance.md).

## License

The upstream BSD-2-Clause license, authorship, and third-party notices are
preserved in the upstream snapshot. Translated files retain their source's
license and attribution.
