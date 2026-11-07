# libavif-rs

A mechanical Rust port of [libavif](https://github.com/AOMediaCodec/libavif).

All 27 C library source files are represented in Rust, including all six codec
adapters, experimental container branches, and the bundled scalar scaler. Names,
control flow, C layouts, allocation ownership, and diagnostics stay close to the
source. Idiomatic Rust APIs and optimizations are deferred.

The complete upstream snapshot, including its tests and fixtures, lives in
`upstream/libavif`. See `UPSTREAM.json` for the exact source revision and
`docs/PORTING.md` for the migration contract and validation status.

The supported and tested targets are **macOS on AArch64** and **GNU Linux on
x86-64**. On Linux the package compiles the generated native profile of the
selected configuration; six verified Cargo feature sets are admitted, and
others fail at compile time (see
[ADR 0054](docs/adr/0054-select-native-linux-root-profiles-in-the-production-package.md)).
The original C/C++ applications and tests consume the Rust implementation
through the C ABI.
The default AOM, dav1d, and SharpYUV implementations are workspace Rust crates.
The separately scoped optional engines remain native dependencies. libaom is
translated without VMAF tuning, so libvmaf is not a dependency ([ADR 0057](docs/adr/0057-translate-libaom-without-vmaf-tuning.md)).

## Port checklist

The selected portable library implementations and their default integration
are complete. The tables distinguish delivered code, verified configurations,
and remaining work. Complete applies to the stated scope; milestones vary in
size and are not a percentage of the entire port. Keep affected rows and their
linked evidence current when delivering a milestone on `main`.

### Implementation and default dependencies

| Work item | Status | Delivered scope and evidence |
| --- | --- | --- |
| Source provenance and licenses | Complete | Pinned original sources, unchanged tests and fixtures, translation manifests, and generated notices. See [the porting contract](docs/PORTING.md) and [notice generator](tools/generate_licenses.py). |
| libavif library | Complete | All 27 original C library files, including allocation, I/O, metadata, encoding, decoding, and the C ABI. See [the contract](docs/PORTING.md) and [feature manifests](translations). |
| Container features and codec adapters | Complete | Mini, Extended PIXI, AV2 container branches, and all six codec adapters. Optional engine implementations retain their separate scope below. |
| Bundled scalar libyuv | Complete | The five bundled conversion/scaling files; full external libyuv remains a separate dependency. See [dependency boundaries](docs/NATIVE-PORTS.md). |
| Portable SharpYUV | Complete | Workspace Rust crate; native macOS ARM64 and Linux x86-64 validation. See [SharpYUV evidence](docs/SHARPYUV-VALIDATION.md). |
| Portable dav1d | Complete | Workspace Rust decoder, including 8/10/12-bit output and film grain; native macOS ARM64 and Linux x86-64 validation. See [dav1d evidence](docs/DAV1D-PORT.md). |
| Portable AOM | Complete | Workspace Rust encoder/decoder; complete original native Release suites and focused Debug checks on both targets. See [AOM evidence](docs/AOM-PORT.md). |
| Default Rust integration | Complete | AOM, dav1d, and SharpYUV are mandatory Rust dependencies, with no native implementation fallback. See [the default-switch decision](docs/adr/0049-use-rust-aom-as-the-mandatory-root-codec.md). |

### Validation and platform support

The root integration rows below describe the final mandatory Rust-AOM build on
macOS ARM64. The Linux production package has its own consumer evidence in the
platform rows; standalone validation of the three dependency crates is recorded
separately.

| Gate | Status | Verified scope and evidence |
| --- | --- | --- |
| Original default Release suite | Passing | C and Rust: 64 CTest registrations and 9,473 ordered GoogleTest cases. See [final root validation](docs/aom-default-validation.json). |
| All supported Cargo features | Passing | C and Rust: 70 registrations and 9,555 cases, including the optional codec adapters. See [final root validation](docs/aom-default-validation.json). |
| Focused root Debug regressions | Passing | Eight unchanged original executables and 64 cases. Complete unoptimized Debug execution is outside the [ADR 0048 acceptance gate](docs/adr/0048-require-complete-release-and-focused-debug-aom-acceptance.md). |
| Root AddressSanitizer | Passing | Complete default C/Rust suites: 64 registrations and 9,473 cases. The original AOM reference includes all 168 instrumented units; native VMAF remains uninstrumented. See [final root validation](docs/aom-default-validation.json). |
| ABI and implementation ownership | Passing | Named-layout probes, shared C ABI smoke test, every original AOM global, and all 55 default / 59 feature-enabled consumers and runtime closures. Probe exclusions are explicit in [the evidence](docs/aom-default-validation.json). |
| Exact AVIF and decoded PNG output | Passing | Ten default-decoder and ten explicit-AOM-decoder comparisons match the C reference byte-for-byte with `-ffp-contract=off`. See [final root validation](docs/aom-default-validation.json). |
| Whole-pipeline platform support | macOS ARM64, GNU Linux x86-64 | The production package is validated for `aarch64-apple-darwin` and, for six Cargo feature sets, `x86_64-unknown-linux-gnu`. Windows and other targets remain open. See [current limits](#current-limits). |

### Remaining work

| Work item | Status | Remaining boundary |
| --- | --- | --- |
| libgav1, SVT-AV1, full libyuv, and AVM | Native optional engines | Adapters are ported and tested; engine implementations and their selected transitive dependencies remain separate milestones. See [the dependency plan](docs/NATIVE-PORTS.md). |
| rav1e packaging | Retained C API integration | The encoder is already Rust. Direct workspace/dependency integration is separate work, rather than another mechanical codec port. See [the dependency plan](docs/NATIVE-PORTS.md). |
| Linux whole-pipeline support | Production for six configurations | The production package selects the native profile by Cargo features and `debug_assertions`. Its original consumers pass Release and Debug: 9,473 default, 9,526 Mini, 9,474 Extended PIXI, and 9,527 combined Mini/Extended PIXI cases, plus 9,481 and 9,535 with libgav1, rav1e, SVT-AV1, and external libyuv. Each run has ABI, ownership, and strict-byte checks. See [production evidence](docs/root-linux-production-validation.json). Isolated Release ASan parity covers the four container configurations. AVM/AV2, engine and production-package sanitizer checks remain open. See [platform CI and remaining gates](docs/PLATFORMS.md). |
| Windows and additional targets | Open | Platform-specific translations, runtime mappings, ABI checks, and native execution evidence. |
| Broader upstream validation | Open | Optional MP4Box golden tests, libavif FuzzTest, ComplianceWarden, continuous fuzzing, additional sanitizer coverage, and the wider upstream platform matrix. See [validation limits](docs/VALIDATION.md). |
| SIMD and performance work | Deferred | Architecture-specific optimized paths, benchmarks, and measured optimization follow the portable mechanical port. See [deferred work](docs/FUTURE.md). |
| Safe Rust API and stable toolchain | Deferred | The current interface retains unsafe C ownership/callback contracts and requires nightly Rust. See [deferred work](docs/FUTURE.md). |
| Registry publication and release validation | Open | Workspace crates currently set `publish = false`; packaging and release acceptance are separate from source-port completion. See [the milestone boundary](docs/NATIVE-PORTS.md). |

Original C/C++ applications, tests, and reference implementations remain
validation infrastructure. PNG, JPEG, libxml2, and normal system facilities
retain the boundaries documented in [the dependency plan](docs/NATIVE-PORTS.md).

## Build and test

Use Rust nightly, CMake, Ninja, Python 3.9+, pkg-config, and development packages
for PNG, JPEG, and libxml2. The original C baseline and CMake test setup
also require native dav1d and SharpYUV development packages; the Rust library
uses their workspace crates. The pinned portable C AOM archive supplies the
independent reference in the commands below. The validated dependency
versions and exact results are recorded in [docs/VALIDATION.md](docs/VALIDATION.md).

```sh
cargo build --release --locked
python3 tools/build_aom.py --jobs 1 --library-only
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

Cargo emits `target/release/libavif.a`, `libavif.dylib` (`libavif.so` on
Linux), and `libavif.rlib`. On Linux, the engine features additionally require
the pinned engine builds from `tools/prepare_optional_deps.py`.
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

Other platforms and unadmitted Linux feature sets, including AVM/AV2, require
their own platform translation and ABI validation.
Optional libavif golden tests, libavif FuzzTest, and the full upstream platform CI matrix are
preserved but have not been executed here. The optional C++ ComplianceWarden
integration remains preserved upstream and is not enabled by the Rust build.

All ten differential AVIF bitstreams and all ten decoded PNGs now match the
noncontracting C reference byte-for-byte. The source expressions remain unchanged;
C references explicitly use `-ffp-contract=off`, and Rust uses normal arithmetic.
Clang-default differences are recorded separately. No upstream test or tolerance
was changed; see [ADR 0024](docs/adr/0024-use-noncontracting-c-references-for-portable-ports.md).

## License

The upstream BSD-2-Clause license, authorship, and third-party notices are
preserved in the upstream snapshot. Translated files retain their source's
license and attribution.
