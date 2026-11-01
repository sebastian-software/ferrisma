# Working on the mechanical port

Use US English and Conventional Commits. Commit coherent verified steps
separately. Keep algorithms, names, data layout, numeric behavior, and ownership
close to the pinned C source. Record lasting decisions in `docs/adr/` and later
redesign or optimization candidates in `docs/FUTURE.md`.

Treat `upstream/libavif` as immutable. `tools/verify_upstream.py` checks every
file, including tests and fixtures. Do not replace missing functionality with
stubs or a C fallback. The original C/C++ tests and applications are deliberately
retained as independent consumers of the Rust library.

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
search path when present. The AVM helper repairs only its known stale macOS
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
v8 runtime. Set `LIBAWFRS_LLVM_PREFIX` for a regular LLVM installation; otherwise
the helper uses the task-local prefix shown above. Extracted Homebrew toolchains
must have their native library paths relocated before use, as a normal Homebrew
installation does.

```sh
python3 tools/build.py baseline --asan
python3 tools/run_tests.py build/baseline-asan-llvm16
python3 tools/build.py port --asan
python3 tools/run_tests.py build/port-asan-llvm16
python3 tools/compare_results.py \
  --baseline build/baseline-asan-llvm16-results.xml \
  --port build/port-asan-llvm16-results.xml
python3 tools/differential.py --diagnose-float
```

The sanitizer runner defaults to the same allocation-failure setting as upstream
CI. It does not disable memory findings. Run separate suites through
`tools/run_tests.py` to isolate upstream tests' fixed temporary file names.

The differential tool checks exact AVIF bytes and records strict PNG differences.
Its float diagnostic builds a separate C decoder to explain contraction-related
rounding; it does not replace the primary C baseline or certify exact PNG parity.
