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
C/C++ applications and tests consume the Rust implementation through the C ABI;
AV1/AV2 codec engines remain external dependencies as they do upstream.

## Build and test

Use Rust nightly, CMake, Ninja, Python 3.9+, pkg-config, and development packages
for libaom, dav1d, SharpYUV, PNG, JPEG, and libxml2. The validated dependency
versions and exact results are recorded in [docs/VALIDATION.md](docs/VALIDATION.md).

```sh
cargo build --release --locked
python3 tools/build.py baseline --fetch-deps
python3 tools/run_tests.py build/baseline
python3 tools/build.py port
python3 tools/run_tests.py build/port
python3 tools/compare_results.py
python3 tools/check_translation.py
python3 tools/check_abi.py
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
Optional golden tests, FuzzTest, and the full upstream platform CI matrix are
preserved but have not been executed here. The optional C++ ComplianceWarden
integration remains preserved upstream and is not enabled by the Rust build.

Strict decoded RGB byte parity has one recorded compiler-rounding difference:
all ten differential AVIF bitstreams match, while nine of ten decoded PNGs match
the original Clang build byte-for-byte. The remaining PNG matches C when floating
point contraction is disabled. No upstream test or tolerance was changed; see
[ADR 0005](docs/adr/0005-floating-point-evaluation.md).

## License

The upstream BSD-2-Clause license, authorship, and third-party notices are
preserved in the upstream snapshot. Translated files retain their source's
license and attribution.
