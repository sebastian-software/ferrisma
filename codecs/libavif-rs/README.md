# libavif-rs

This port is maintained in the [Ferrixel workspace](../../README.md). See the
[project status](../../docs/status.md) and [codec guide](../../docs/codecs.md)
for integration and checks; local porting contracts and licenses still apply.
The separate [ferrixel-libavif adapter](../../crates/ferrixel-libavif/README.md)
connects its AOM encoder to Ferrixel; it is not Ferrixel's built-in AVIF backend.

A mechanical Rust port of [libavif](https://github.com/AOMediaCodec/libavif).

All 27 C library source files are represented in Rust, including all six codec
adapters, experimental container branches, and the bundled scalar scaler. Names,
control flow, C layouts, allocation ownership, and diagnostics stay close to the
source. Idiomatic Rust APIs and optimizations are deferred.

The complete upstream snapshot, including its tests and fixtures, lives in
`upstream/libavif`. See `UPSTREAM.json` for the exact source revision and
`docs/PORTING.md` for the migration contract.

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

## Status

Complete applies to the stated mechanical-port scope, not a stable safe Rust API
or an unrestricted release approval. The selected portable library
implementations and their default integration are delivered; open work is
tracked in [GitHub issues](https://github.com/sebastian-software/ferrixel/issues)
and decisions in [`docs/adr/`](docs/adr). The evidence in this table describes
recorded runs for their stated configurations; it is not a release claim for
the current workspace revision.

| Capability | Delivered scope |
| --- | --- |
| Source provenance and licenses | Pinned original sources, unchanged tests and fixtures, translation manifests, and generated notices ([notice generator](tools/generate_licenses.py), [`docs/NATIVE-LICENSES.md`](docs/NATIVE-LICENSES.md)). |
| libavif library | All 27 original C library files, including allocation, I/O, metadata, encoding, decoding, and the C ABI, with the Mini, Extended PIXI, and AV2 container branches, all six codec adapters, and the five bundled scalar libyuv files. See [the contract](docs/PORTING.md) and the [feature manifests](translations). |
| Default codecs and conversion | Portable libaom (encoder and decoder), dav1d (8/10/12-bit output and film grain), and SharpYUV are mandatory workspace Rust crates with no native implementation fallback ([ADR 0049](docs/adr/0049-use-rust-aom-as-the-mandatory-root-codec.md)). |
| Optional engines | libgav1, rav1e, SVT-AV1, full external libyuv, and AVM stay native dependencies behind their ported adapters. |
| Verified parity | The original suite passes unchanged in C and Rust: 64 CTest registrations and 9,473 GoogleTest cases by default, 70 and 9,555 with every Cargo feature, plus AddressSanitizer parity for the default configuration. ABI probes, ownership audits, and ten AVIF and ten decoded PNG scenarios match the C reference byte for byte. |
| Platforms | macOS ARM64, and GNU Linux x86-64 for six Cargo feature sets selected by the production package ([ADR 0054](docs/adr/0054-select-native-linux-root-profiles-in-the-production-package.md)). Windows and other targets are open. |
| SIMD | The AOM encoder dispatches hand-translated Neon kernels on macOS ARM64 and SSE2, SSE4.1, and AVX2 kernels on GNU Linux x86-64, byte-identical to its `_c` paths; all-intra encodes take 29 to 45% less time on x86-64 ([ADR 0056](docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md), [ADR 0058](docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md)). |

Original C/C++ applications, tests, and reference implementations remain
validation infrastructure. PNG, JPEG, libxml2, and normal system facilities stay
native ([ADR 0010](docs/adr/0010-port-native-libraries-from-their-original-sources.md)).

## Build and test

Run the commands below from `codecs/libavif-rs`. Use the workspace-pinned Rust
nightly, CMake, Ninja, Python 3.9+, pkg-config, and development packages
for PNG, JPEG, and libxml2. The original C baseline and CMake test setup
also require native dav1d and SharpYUV development packages; the Rust library
uses their workspace crates. The pinned portable C AOM archive supplies the
independent reference in the commands below. The toolchain baseline of the
recorded runs (compilers, CMake, GoogleTest, libargparse) is in the docstring of
`tools/check_abi.py`.

```sh
cargo build -p libavif-rs --release --locked
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

The recorded default-configuration runs passed the same 64 upstream test registrations in C
and Rust. The recorded complete supported-feature configuration passed 70 registrations
containing 9,555 GoogleTest cases, plus the original C and shell integration
tests. AddressSanitizer parity was also recorded for the 64 default registrations.
These are historical results for their tested revisions, not an assertion that
all release gates have passed for the current workspace revision.

Cargo emits `../../target/release/libavif.a`, `libavif.dylib` (`libavif.so` on
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

The AVM/AV2 configurations use AVM v1.0.0
(`966a7d7cd6fcf60360caf5dc413b2aeeb65e144d`) with the dependency-only export-list
repair of [ADR 0008](docs/adr/0008-av2-adapter-and-dependency.md). On macOS ARM64
each passes with matching case and skip inventories in C and Rust:

| Configuration | CTest registrations | GoogleTest cases |
| --- | ---: | ---: |
| AVM/AV2 | 67 | 9,485 |
| AVM/AV2 + Mini | 69 | 9,546 |
| AVM/AV2 + Extended PIXI | 67 | 9,486 |
| AVM/AV2 + Mini + Extended PIXI | 69 | 9,547 |

These configurations are not admitted on Linux.

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
