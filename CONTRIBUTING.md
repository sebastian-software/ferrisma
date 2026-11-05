# Working on the mechanical port

Use US English and Conventional Commits. Commit coherent verified steps
separately. Keep algorithms, names, data layout, numeric behavior, and ownership
close to the pinned C source. Record lasting decisions in `docs/adr/` and later
redesign or optimization candidates in `docs/FUTURE.md`.

Treat `upstream/libavif`, `upstream/libwebp`, `upstream/dav1d`, and `upstream/aom` as immutable.
`tools/verify_upstream.py` checks every file, including tests and fixtures.
Do not replace missing functionality with
stubs or a C fallback. The original C/C++ tests and applications are deliberately
retained as independent consumers of the Rust library.

Normal Rust builds always use the workspace AOM, dav1d, and SharpYUV crates.
They require native libvmaf and any selected optional engines, with no system
AOM implementation fallback. Use the complete pinned portable C AOM reference
for acceptance comparisons:

```sh
python3 tools/build_aom.py --jobs 1
python3 tools/build.py baseline --aom-reference-build build/aom-native-release
python3 tools/build.py port
```

The matching C results live under `build/baseline-aom-reference`. Pass that
directory to the differential tool and its results/log paths to the comparison
tool. The original standalone AOM suite includes long video-encoding cases;
ADR 0048 requires complete Release acceptance and focused Debug checks rather
than exhaustive unoptimized Debug execution. ASan acceptance requires the
separate complete instrumented C AOM reference closure.

## Generate source

C2Rust 0.21.0 and LLVM 16.0.6 are the recorded translation tools. Set
`C2RUST_LIBRARY_PATH` if the installed translator needs a local LLVM runtime;
this also works when macOS strips `DYLD_*` from the system Python interpreter.

```sh
export C2RUST_LIBRARY_PATH="$PWD/build/toolchain/llvm@16/16.0.6_1/lib:/opt/homebrew/lib"
python3 tools/transpile.py \
  --clang-include build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include
python3 tools/transpile.py --check \
  --clang-include build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include
```

Use an installed LLVM prefix instead if the task-local extracted toolchain is
unavailable. CMake's actual compiler commands determine the configuration.
`tools/import_translation.py` applies the documented mechanical corrections and
records raw/corrected hashes. Avoid hand-editing generated implementation files.

For `mini`, `extended-pixi`, `avm`, and their combinations, first build the
corresponding C reference using the matching switches to `tools/build.py`.
Then pass its build directory and profile name to `tools/transpile.py`:

```sh
python3 tools/build.py baseline --mini --extended-pixi --avm
python3 tools/transpile.py --variant mini-extended-pixi-avm \
  --build build/baseline-mini-extended-pixi-avm \
  --clang-include build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include
python3 tools/update_modules.py
python3 tools/check_translation.py
```

All seven non-default profile manifests must exist before updating module
selectors. Unchanged corrected files remain shared. Generated dispatcher label
numbers are canonicalized; the control-flow edges and conditions are preserved.

## Optional native dependencies and the complete configuration

The optional adapters require rav1e 0.8.1 and SVT-AV1 4.2.0 through pkg-config.
The helper builds pinned libgav1, libyuv, and optionally AVM in `build/deps`:

```sh
python3 tools/prepare_optional_deps.py --avm
python3 tools/build.py baseline --mini --extended-pixi --libgav1 --rav1e --svt --libyuv --avm
python3 tools/run_tests.py build/baseline-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm
python3 tools/build.py port --mini --extended-pixi --libgav1 --rav1e --svt --libyuv --avm
python3 tools/run_tests.py build/port-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm
python3 tools/compare_results.py \
  --baseline build/baseline-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm-results.xml \
  --port build/port-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm-results.xml
```

Task-local pkg-config files in `build/deps/pkgconfig` supplement the existing
search path when present. Selecting engines, for example
`python3 tools/prepare_optional_deps.py --libgav1 --rav1e --svt --libyuv`,
builds only those; `--rav1e` and `--svt` build rav1e v0.8.1 (with cargo-c)
and SVT-AV1 v4.2.0 from their pinned sources as shared libraries and record
every engine identity in `build/deps/optional-engines.json`. Native Linux root
profiles use these builds; see ADR 0053. The AVM helper repairs only its known stale macOS
export-list entry; see ADR 0008. AVM's own TensorFlow dependencies remain native.

To regenerate the additional AV1 adapters and the external libyuv path, build
the optional reference configuration and translate it:

```sh
python3 tools/build.py baseline --libgav1 --rav1e --svt --libyuv
python3 tools/translate_optional.py \
  --clang-include build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include
```

This also regenerates the all-codec registration table directly from `avif.c`.
After changing that table, re-import the base and experimental configurations
before running `tools/check_translation.py`; their manifests cover shared files.

## Sanitizer and differential checks

`--asan` uses LLVM 16 native compilers and the selected nightly's standard ASan
v8 runtime. Set `LIBAVIF_RS_LLVM_PREFIX` for a regular LLVM installation; otherwise
the helper uses the task-local prefix shown above. Extracted Homebrew toolchains
must have their native library paths relocated before use, as a normal Homebrew
installation does.

```sh
python3 tools/build.py baseline --asan
CARGO_BUILD_JOBS=1 python3 - <<'PY'
import pathlib
import sys
sys.path.insert(0, 'tools')
from aom_sanitizer import instrumented_libraries
original, _, _, _, _ = instrumented_libraries()
directory = pathlib.Path('build/aom-root-asan-libs')
directory.mkdir(exist_ok=True)
archive = directory / 'libaom.a'
if not archive.exists():
    archive.symlink_to(original.resolve())
assert archive.resolve() == original.resolve()
PY
cmake -S upstream/libavif -B build/baseline-asan-llvm16 \
  -DLIBAVIF_RS_AOM_REFERENCE_BUILD="$PWD/build/aom-root-asan-libs" \
  -DCMAKE_DISABLE_FIND_PACKAGE_aom=TRUE
cmake --build build/baseline-asan-llvm16 --parallel 1
python3 tools/run_tests.py build/baseline-asan-llvm16
python3 tools/build.py port --asan
python3 tools/run_tests.py build/port-asan-llvm16
python3 tools/compare_results.py \
  --baseline build/baseline-asan-llvm16-results.xml \
  --port build/port-asan-llvm16-results.xml
python3 tools/differential.py --baseline build/baseline-aom-reference \
  --port build/port --decoder-codec aom
```

Build the ordinary pinned AOM reference first. The sanitizer helper instruments
all 168 original portable units and the Rust crate with one matching runtime.
The explicit CMake overlay relinks the original baseline against that complete
instrumented archive without changing original test objects or parameters.
Native VMAF remains outside instrumentation. The differential command uses the
ordinary Release references and requires exact AVIF and decoded PNG bytes.

The sanitizer runner defaults to the same allocation-failure setting as upstream
CI. It does not disable memory findings. Run separate suites through
`tools/run_tests.py` to isolate upstream tests' fixed temporary file names.

The differential tool requires exact AVIF and PNG bytes against the primary C
reference. Its separate float diagnostic builds a C decoder to explain
contraction-related rounding; that diagnostic does not establish acceptance.

## Standalone SharpYUV crate

The workspace uses the pinned toolchain in `rust-toolchain.toml`. Original C
references explicitly use `-ffp-contract=off`; Rust uses normal arithmetic and
unoptimized development builds. Preserve that contract when supplying custom
compiler flags. Original source hashes and generated license records must match
before building a reference.

```sh
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
python3 tools/build_sharpyuv.py
python3 tools/build_sharpyuv.py --debug
cargo build -p libavif-sharpyuv --locked --release
cargo build -p libavif-sharpyuv --locked
python3 tools/check_sharpyuv.py
python3 tools/check_sharpyuv.py --debug
python3 tools/check_sharpyuv_abi.py
python3 tools/check_sharpyuv_abi.py --debug
python3 tools/check_sharpyuv_alloc.py
python3 tools/check_sharpyuv_cpu.py
python3 tools/check_sharpyuv_linkage.py
python3 tools/diagnose_sharpyuv_numerics.py
```

`python3 tools/transpile_sharpyuv.py --check` reproduces all seven translated
units and the x86 CPU-query variant with C2Rust and LLVM 16 on the recorded
translation host. The macOS-only
`python3 tools/check_sharpyuv_alloc.py --asan` instruments both implementations
and the allocation-failure driver with the matching Rust ASan runtime.

The original twelve WebP integration tests retain their assertions and fixtures:

```sh
export FUZZTEST_PRNG_SEED=OffQXb8u5_vZtH4-7wgVOLu_HNAhPIbLz7CFF13u3nk
python3 tools/build_sharpyuv_consumers.py c --fetch-deps
python3 tools/run_tests.py build/sharpyuv-webp-c
python3 tools/build_sharpyuv_consumers.py rust
python3 tools/run_tests.py build/sharpyuv-webp-rust
python3 tools/compare_results.py \
  --baseline build/sharpyuv-webp-c-results.xml \
  --port build/sharpyuv-webp-rust-results.xml
python3 tools/check_sharpyuv_linkage.py \
  --consumer build/sharpyuv-webp-rust \
  --executable build/sharpyuv-webp-rust/tests/fuzzer/enc_dec_fuzzer
```

Use `SHARPYUV_BUILD_ROOT` and `CARGO_TARGET_DIR` to keep another host's artifacts
separate. Do not share CMake caches between macOS and Linux. See ADR 0022 for
test-build compatibility overlays, and `docs/SHARPYUV-VALIDATION.md` for the
executed scope and remaining gaps.

For the separate original libavif compiler-default diagnostic, use
`python3 tools/build.py baseline --compiler-default-diagnostic`, then
`python3 tools/differential.py --baseline build/baseline-compiler-default --diagnose-float`.
Restore the primary report with `python3 tools/differential.py` afterward.

After downloading both native CI artifacts and the completed run's
`gh run view --json status,conclusion,jobs,url,headSha` output, record the evidence:

```sh
python3 tools/record_sharpyuv_validation.py \
  --ci-run build/sharpyuv-ci-success.json \
  --ci-artifacts build/sharpyuv-ci-artifacts
```

This also requires the local libavif integration and sanitizer reports above.

## Standalone dav1d crate

The selected configuration, complete reproduction commands, test-only DSP
bridge, and validation limits are in [docs/DAV1D-PORT.md](docs/DAV1D-PORT.md).
Fetch the pinned external test inputs before building its two C reference
profiles. `tools/transpile_dav1d.py --check` and `--release --check` reproduce
the matching host profiles; the Linux workflow builds C2Rust 0.21.0 with Rust
1.80.1 and LLVM 16, while the decoder uses the workspace nightly toolchain.

After downloading both successful native CI artifacts and saving completed
`gh run view --json status,conclusion,jobs,url,headSha` reports for the native
tests and Linux translation verification, record the evidence:

```sh
python3 tools/record_dav1d_validation.py \
  --ci-run build/dav1d-ci-success.json \
  --ci-artifacts build/dav1d-ci-artifacts \
  --translation-run build/dav1d-linux-verify-run.json \
  --translation-artifacts build/dav1d-linux-verification-artifact
```

This requires the local API sanitizer, downstream test inventories,
`python3 tools/check_dav1d_linkage.py --root-library`, and strict differential
reports. Run the root build with its default features before the linkage audit
if the shared Cargo output was most recently built with optional features.

## Hosted macOS checks during development

Pull requests run hosted checks on Linux x86-64 only; macOS runs nightly and on
manual dispatch (ADR 0055). Before merging a change that affects macOS
builds or translations, run the applicable checks locally on macOS ARM64 and
record the commands and results in the pull request. Hosted macOS runs are
still required for a release, a new target admission, or a hosted macOS
validation claim.
