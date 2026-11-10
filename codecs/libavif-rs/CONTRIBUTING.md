# Working on the mechanical port

Use US English and Conventional Commits. Commit coherent verified steps
separately. Keep algorithms, names, data layout, numeric behavior, and ownership
close to the pinned C source. Record lasting decisions in `docs/adr/`. Open work,
including later redesign and optimization candidates, belongs in a GitHub issue;
do not add planning or status documents to the repository.

Treat `upstream/libavif`, `upstream/libwebp`, `upstream/dav1d`, and `upstream/aom` as immutable.
`tools/verify_upstream.py` checks every file, including tests and fixtures.
Do not replace missing functionality with
stubs or a C fallback. The original C/C++ tests and applications are deliberately
retained as independent consumers of the Rust library.

Normal Rust builds always use the workspace AOM, dav1d, and SharpYUV crates.
They require any selected optional engines, with no system
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
all 166 original portable units and the Rust crate with one matching runtime.
The explicit CMake overlay relinks the original baseline against that complete
instrumented archive without changing original test objects or parameters.
The differential command uses the
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
test-build compatibility overlays. The docstrings of `tools/check_sharpyuv.py` and
`tools/check_sharpyuv_alloc.py` and `crates/sharpyuv/README.md` describe the
executed scope and its exclusions.

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
The record goes to `build/sharpyuv-validation-results.json` (`--output` changes
it). It is not committed; attach it to a release or an evidence tag if it has to
stay retrievable.

## Standalone dav1d crate

The selected configuration, the test-only bridges, and the validation limits
are in `crates/dav1d/README.md`. Fetch the pinned external test inputs before
building its two C reference profiles:

```sh
python3 tools/import_dav1d.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
```

Install Meson 1.7.2 in an isolated environment if Meson is unavailable. The
original checkasm framework also needs NASM on GNU Linux x86-64; that assembler
builds test infrastructure only, and codec assembly stays disabled. The static
fault-fuzzer bridge needs GNU objcopy on Linux or LLVM objcopy under
`LIBAVIF_RS_LLVM_PREFIX` on macOS (the task-local LLVM 16 prefix by default); its
AddressSanitizer mode currently targets macOS ARM64.

```sh
python3 -m venv build/dav1d-tools
build/dav1d-tools/bin/python -m pip install meson==1.7.2
python3 tools/build_dav1d.py --debug
python3 tools/build_dav1d.py
python3 tools/transpile_dav1d.py --check
python3 tools/transpile_dav1d.py --release --check
python3 tools/check_dav1d_abi.py
python3 tools/build_dav1d_consumers.py
python3 tools/run_dav1d_tests.py build/dav1d-rust-release --jobs 2
python3 tools/build_dav1d_consumers.py --debug
python3 tools/run_dav1d_tests.py build/dav1d-rust-debug --jobs 2 --timeout-multiplier 4
python3 tools/check_dav1d_dsp.py
python3 tools/check_dav1d_dsp.py --debug
python3 tools/check_dav1d_linkage.py
python3 tools/check_dav1d_linkage.py --debug
python3 tools/check_dav1d_api.py
python3 tools/check_dav1d_api.py --debug
python3 tools/check_dav1d_api.py --asan
python3 tools/check_dav1d_faults.py
python3 tools/check_dav1d_faults.py --debug
python3 tools/check_dav1d_faults.py --asan
python3 tools/build.py port
python3 tools/check_dav1d_linkage.py --root-library
```

Run the original C suites separately:

```sh
build/dav1d-tools/bin/meson test -C build/dav1d-reference-debug --no-rebuild --num-processes 2
build/dav1d-tools/bin/meson test -C build/dav1d-reference-release --no-rebuild --num-processes 2
```

`tools/transpile_dav1d.py --check` and `--release --check` reproduce the matching
host profiles; the Linux workflow builds C2Rust 0.21.0 with Rust 1.80.1 and
LLVM 16, while the decoder uses the workspace nightly toolchain. The candidate
consumers keep the exact original Meson test definitions with only the
executable location replaced.

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
if the shared Cargo output was most recently built with optional features. The
record goes to `build/dav1d-validation-results.json` (`--output` changes it). It is
not committed; attach it to a release or an evidence tag if it has to stay
retrievable.

## Standalone AOM crate

`crates/aom` translates the portable libaom 3.15.2 archive (166 C units, built
without VMAF tuning, ADR 0057). The reference boundary, the original test
inventories, and the consumer bridge are described in the docstrings of
`tools/build_aom.py` and `tools/build_aom_consumers.py`; the acceptance policy is
ADR 0048. The reference commands:

```sh
python3 tools/import_aom.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
python3 tools/build_aom.py --debug
python3 tools/build_aom.py
python3 tools/fetch_aom_test_data.py
python3 tools/transpile_aom.py
python3 tools/transpile_aom.py --release
python3 tools/build_aom_consumers.py --debug
python3 tools/build_aom_consumers.py
python3 tools/check_aom_linkage.py --debug
python3 tools/check_aom_linkage.py
python3 tools/check_aom_inlines.py --debug
python3 tools/check_aom_inlines.py
python3 tools/check_aom_macros.py --debug
python3 tools/check_aom_macros.py
python3 tools/check_aom_oracles.py --debug
python3 tools/check_aom_oracles.py
python3 tools/check_aom_oracles.py --debug --suite softmax
python3 tools/check_aom_oracles.py --suite softmax
python3 tools/check_aom_oracles.py --debug --suite intrabc
python3 tools/check_aom_oracles.py --suite intrabc
python3 tools/check_aom_oracles.py --debug --suite kmeans
python3 tools/check_aom_oracles.py --suite kmeans
python3 tools/check_aom_oracles.py --debug --suite nn-predict
python3 tools/check_aom_oracles.py --suite nn-predict
python3 tools/check_aom_abi.py
python3 tools/check_aom_abi.py --release
python3 tools/check_aom_cli.py --debug
python3 tools/check_aom_cli.py
python3 tools/check_aom_roundtrip.py --debug
python3 tools/check_aom_roundtrip.py
python3 tools/check_aom_faults.py --debug
python3 tools/check_aom_faults.py
python3 tools/check_aom_faults.py --debug --thread-failures
python3 tools/check_aom_faults.py --thread-failures
python3 tools/check_aom_roundtrip.py --asan
python3 tools/check_aom_faults.py --asan
python3 tools/check_aom_faults.py --asan --thread-failures
python3 tools/check_aom_focused_debug.py
python3 tools/run_aom_tests.py
python3 tools/run_aom_tests.py --rust
```

Native build evidence goes to `build/aom-native-{debug,release}`. Cache the test
corpus in `build/deps/aom-test-data` and set `LIBAOM_TEST_DATA_PATH` to that
absolute directory when running original tests. Downloaded fixtures and
disposable builds are not source modifications. The SIMD layers (ADR 0056 on
Arm, ADR 0058 on x86-64) and the golden outputs (ADR 0059) have their own
checks; the last two run in the pull-request tier:

```sh
cargo test --release -p libavif-aom --test simd
python3 tools/check_aom_simd_identity.py
python3 tools/check_aom_golden.py
```

Work in a dedicated linked worktree and keep the primary checkout clean on the
remote default branch; tracked changes and commits belong to the task worktree.
Limit local execution to two CPU-intensive processes at a time across all
commands. The shard runner defaults to one worker so a C and a Rust run can share
that budget, and further verification or build work waits for a slot. Keep the
original encoder and decoder thread parameters: one multithreaded codec test can
use several cores inside a single process. The scheduling limit is separate from
the test cases, assertions, numerical references, and preprocessing profiles.
`tools/run_aom_paired_shard.py` runs one unchanged GoogleTest shard with the C
binary and then with the Rust consumer; pass `--reference-build` to the consumer
builder, the symbol audit, and the runner so that a task worktree reuses native
reference sources, objects, and executables. Download the fixtures once and
select them with the runner's `--fixtures` option.

## Native root profiles

The root harness builds isolated native profiles with the actual C compilation
database and the pinned original decoder and conversion archives, and compares
the original consumers against the Rust build (ADRs 0050 to 0054). Prepare Meson
1.7.2, LLVM 16 with C2Rust 0.21.0, the pinned Rust nightly, and the native
development dependencies first. For a native Linux Release default profile:

```sh
python3 tools/build_root_references.py --jobs 2
python3 tools/build_aom.py --jobs 2 --library-only
python3 tools/build.py baseline --fetch-deps --jobs 2 \
  --aom-reference-build build/aom-native-release \
  --root-references build/root-references-x86_64-unknown-linux-gnu-release \
  --build-dir build/root-c-linux-release-default
python3 tools/transpile_root.py --build build/root-c-linux-release-default \
  --profile release --clang-include /usr/lib/llvm-16/lib/clang/16/include
python3 tools/build.py port --jobs 2 \
  --root-references build/root-references-x86_64-unknown-linux-gnu-release \
  --root-profile translations/root-linux-release-default.json \
  --build-dir build/root-rust-linux-release-default
python3 tools/validate_root_profile.py --baseline build/root-c-linux-release-default \
  --port build/root-rust-linux-release-default \
  --root-profile translations/root-linux-release-default.json --tier full --jobs 2
```

Use the Debug counterparts plus `--debug --upstream-debug` for both the original
C and the Rust consumers. `tools/root_pipeline.py` runs the same phases and
documents the AddressSanitizer, engine-variant, and production-package options.
The `smoke` tier selects eight original test executables; the `full` tier
requires every registration from the native CMake inventory and the exact ordered
original GoogleTest cases. With `--package production` both tiers link the
consumers to the real package.

Root CI (`root.yml`) uses these same tools in two tiers. Pull requests and pushes
to `main` run Linux x86-64 Release for the default container configuration with
the production package. The nightly run at 02:17 UTC and manual dispatch run the
complete suites for every admitted configuration, plus the AddressSanitizer job.
The standalone dav1d and SharpYUV workflows split the same way (nightly at 02:47
UTC, ADR 0055), and `aom-simd-linux.yml` runs on pull requests that touch
`crates/aom`. Only pull-request runs cancel an earlier run of the same pull
request; up to two nightly container configurations per native OS run
concurrently and each job limits compiler and test processes to two. The reusable
`root-translate-linux.yml` keeps its file name for existing dispatches. Its
`runner` input selects the native host. A workflow status alone is not evidence of
complete target acceptance: read the retained XML, logs, and paired reports.

## Hosted macOS checks during development

Pull requests run hosted checks on Linux x86-64 only; macOS runs nightly and on
manual dispatch (ADR 0055). Before merging a change that affects macOS
builds or translations, run the applicable checks locally on macOS ARM64 and
record the commands and results in the pull request. Hosted macOS runs are
still required for a release, a new target admission, or a hosted macOS
validation claim.
