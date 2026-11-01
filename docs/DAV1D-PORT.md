# Portable dav1d port

Status: both portable source profiles are translated. All 1,711 original corpus
tests pass against the Rust Release decoder on macOS ARM64; standalone
acceptance and integration remain in progress.

The selected decoder is dav1d 1.5.4 at
`54706fc6bc0cdecab7e9593974a4039cc038fca7`. Its complete immutable source,
including original tests, is under `upstream/dav1d`. The separate original
checkasm framework and decoder corpus are fetched into `build/deps` with all
tracked file hashes recorded in `provenance/dav1d-test-inputs.json`.

```sh
python3 tools/import_dav1d.py --fetch
python3 tools/verify_upstream.py
python3 tools/generate_licenses.py --check
```

The reference configuration includes both bit-depth instantiations, logging,
threading, all scalar DSP functions, no assembly, and `-ffp-contract=off`.
Original CLI muxers, demuxers, test code, and system facilities remain native
test infrastructure. The optional xxHash output muxer is outside the decoder
and is disabled; the original built-in MD5 verifier remains enabled.

Required evidence before integration:

- Exact original test-data expectations for 8/10/12-bit streams, film grain,
  mixed bit depths, scalable streams, and malformed/regression inputs.
- Original checkasm DSP test bodies comparing original C and Rust pointers,
  with nonzero matching function inventories in both bit-depth instantiations.
- Public API, ABI, packet/frame ownership, flush/drain, custom allocation,
  callback, error, and multithreaded behavior compared independently.
- Debug and Release comparisons, instrumented cleanup checks, and native
  macOS ARM64 and Linux x86-64 validation.
- Reproducible source translation, complete licenses, symbol/linkage audit,
  and the original libavif integration suite after dependency replacement.

The independent layout probe currently matches all 12,157 checks across 1,201
named record instances. Both Debug and Release retain their own original C
preprocessing. Reproducible corrections preserve field alignment, C11 atomic
operations and orderings, raw address-taking, and target runtime types.

The initial Rust Debug run passed 1,708 corpus tests with three wall-clock
timeouts. The subsequent run passes all 1,711 tests with an explicit fourfold
time budget; original commands, input data, and expected hashes remain unchanged.
Both original C profiles and Rust Release pass all 1,711 tests at the original
limits. This is not a performance parity claim. Public API edge cases,
sanitizers, and Linux validation are still pending acceptance gates.

The original checkasm bodies now pass in both profiles: seventeen source
instantiations, 3,155 nonnull C/Rust function selections each, and 1,923 matching
unsupported null combinations. ADR 0029 records the test-only binding bridge
and native helpers. Public API edge cases, sanitizers, and Linux validation
remain pending.

The existing libavif implementation continues to use native dav1d while this
standalone crate is validated.

## Current reproduction commands

Install Meson 1.7.2 in an isolated environment if Meson is unavailable:

```sh
python3 -m venv build/dav1d-tools
build/dav1d-tools/bin/python -m pip install meson==1.7.2
python3 tools/build_dav1d.py --debug
python3 tools/build_dav1d.py
python3 tools/transpile_dav1d.py --check
python3 tools/transpile_dav1d.py --release --check
python3 tools/check_dav1d_abi.py
python3 tools/build_dav1d_consumers.py
python3 tools/run_dav1d_tests.py build/dav1d-rust-release
python3 tools/build_dav1d_consumers.py --debug
python3 tools/run_dav1d_tests.py build/dav1d-rust-debug --timeout-multiplier 4
python3 tools/check_dav1d_dsp.py
python3 tools/check_dav1d_dsp.py --debug
python3 tools/check_dav1d_linkage.py
python3 tools/check_dav1d_linkage.py --debug
```

The C baseline uses `meson test -C build/dav1d-reference-debug --no-rebuild`
and the corresponding Release directory. The pinned C2Rust/LLVM requirements
are the same as SharpYUV. Compile/link records are saved alongside the candidate
consumers; they use the Rust archive and retain the exact original Meson test
definitions with only the executable location replaced.
