# Portable dav1d port

Status: the selected portable milestone is implemented and integrated. All four
source profiles pass native macOS ARM64 and Ubuntu 24.04 x86-64 comparisons.
The workspace crate supplies the existing libavif dav1d adapter on macOS ARM64.
Source, binary, inventory, and CI hashes are recorded in
[`dav1d-validation-results.json`](dav1d-validation-results.json).

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

The milestone's acceptance boundary covers:

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

The native validation results are:

| Target | Original registrations per implementation/profile | Named ABI checks (Debug) | DSP selections per implementation/profile |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1,711 | 12,157 across 1,201 record instances | 3,155 |
| Ubuntu 24.04 x86-64 | 1,711 | 12,592 across 1,242 record instances | 3,155 |

The ABI probes report zero differences. Anonymous records and bitfield offsets
excluded from the automatic probes are listed explicitly in their reports.
Both Debug and Release retain their own original C preprocessing. Reproducible
corrections preserve field alignment, C11 atomic
operations and orderings, raw address-taking, and target runtime types.
ADR 0033 additionally restores explicit local and static object alignment that
C2Rust omitted. The original Linux Release suite passed before this correction;
1,182 Debug failures in `lf_mask` exposed the missing stack-buffer alignment.

The initial Rust Debug run passed 1,708 corpus tests with three wall-clock
timeouts. The subsequent run passes all 1,711 registrations with an explicit
fourfold time budget; original commands, input data, and expected hashes remain unchanged.
Both original C profiles and Rust Release pass all 1,711 tests at the original
limits. This is not a performance parity claim. The inventory contains six
original public-header checks and 1,705 decoder consumers: 8/10/12-bit and mixed
streams, seek stress, and archived OSS-Fuzz regression inputs. Suite labels
containing `asan`, `ubsan`, or `msan` identify the original regression corpus;
they do not mean those runs were sanitizer-instrumented or continuously fuzzed.

The original checkasm bodies now pass in both profiles: seventeen source
instantiations, 3,155 nonnull C/Rust function selections each, and 1,923 matching
unsupported null combinations. ADR 0029 records the test-only binding bridge
and native helpers.

Supplemental public API probes compare 83 Release and 82 Debug scenarios:
8/10/12-bit pictures, film grain, four thread/delay combinations, custom packet
and user-data ownership, custom picture allocators, their first-allocation failure,
automatic thread selection, flush/drain, variadic logging, cached decode-error
properties, and defined Release argument rejection. Compared results include
exact plane hashes and frame
metadata. All tracked callback resources are released after close.

All 83 Release API scenarios also pass with the complete original C decoder,
Rust decoder, and C caller instrumented by AddressSanitizer. The native compiler
is LLVM 16 and both consumers link the matching Rust nightly ASan v8 runtime.
This does not claim complete leak, race, or undefined-behavior-sanitizer coverage.

The original conditional static fault fuzzer is also exercised through its
unchanged `dav1d_fuzzer.c`, `main.c`, and `alloc_fail.c` bodies. Four original
8/10/12-bit and film-grain payloads receive 512 reserved-header variants each,
controlling the original input-derived fault seed. Each profile runs 2,048
paired scenarios and requires observed calls and failures for malloc, aligned
allocation, thread creation, condition initialization, and mutex initialization
in both implementations. Local Debug, Release, and fully instrumented Release
pass; the native Linux Debug and Release runs also pass. The supplemental
Linux run is recorded separately from the full corpus CI. ADR 0034
documents object rewriting, runtime isolation, and recovery of compiler-folded
malloc/zeroing operations for the original fault hook. These test adapters are
absent from production builds.

## Downstream integration

The original libavif adapter now links the `libavif-dav1d` workspace crate.
Archive, symbol, and dynamic dependency checks establish that Rust supplies
the decoder, with no native dav1d fallback. Original C/C++ application and test
code still uses the original headers. libc, pthreads, system math, libaom, and
optional native codec engines remain allowed dependencies at this milestone.

The local macOS ARM64 integration retains the original passing case and skip
inventories:

| Configuration | CTest registrations | GoogleTest cases |
| --- | ---: | ---: |
| Default Release | 64 | 9,473 |
| Default Debug | 64 | 9,473 |
| All Cargo features | 70 | 9,555 |
| Default AddressSanitizer | 64 | 9,473 |

The root libavif Rust Debug row retains its existing C Release preprocessing;
the dependency independently selects its actual Debug translation, assertions,
and original C Debug behavior. The shared-library smoke test decodes the 33-by-11
fixture and encodes a 502-byte AVIF. Ten deterministic AVIF bitstreams and ten
decoded PNGs match the primary C reference byte-for-byte. ADR 0032 records the
workspace integration boundary.

## Remaining scope

This is the complete selected scalar decoder configuration. Assembly and SIMD,
other platforms, a safe Rust API, optimization, and registry publication remain
separate work. The optional external Argon conformance corpus was not downloaded.
Archived fuzz regressions and bounded original tests do not establish continuous
fuzz, complete leak, race, or undefined-behavior sanitizer coverage. Fault-fuzzer
sampling does not enumerate every failure position or possible thread schedule.

Linux validation of this crate does not enable the root libavif package on Linux.
The subsequent [libaom milestone](AOM-PORT.md) is implemented and integrated.
Optional libavif golden tests, FuzzTest, ComplianceWarden, and the broader
upstream platform matrix retain their previous gaps. See the current
[port checklist](../README.md#port-checklist).

## Current reproduction commands

Install Meson 1.7.2 in an isolated environment if Meson is unavailable. The
original checkasm framework also requires NASM on GNU Linux x86-64; this
assembler builds test infrastructure while codec assembly remains disabled.
The static fault-fuzzer bridge requires GNU objcopy on Linux or LLVM objcopy
under `LIBAVIF_RS_LLVM_PREFIX` on macOS, using the task-local LLVM 16 prefix by
default. Its AddressSanitizer mode currently targets macOS ARM64.

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

Translation uses C2Rust 0.21.0 and LLVM 16.0.6; the Linux
translator is built with Rust 1.80.1 for compatibility with its legacy
dependencies. The decoder uses `nightly-2026-09-07` on both targets. ADRs
0025–0034 document the selected configuration, mechanical corrections, and test
bridges.
Compile/link records are saved alongside the candidate
consumers; they use the Rust archive and retain the exact original Meson test
definitions with only the executable location replaced.

The complete native corpus, ABI, API, and DSP evidence is in
[the native validation run](https://github.com/sebastian-software/libavif-rs/actions/runs/41355195932).
Exact Linux regeneration and the additional original static fault fuzzer are in
[the verification run](https://github.com/sebastian-software/libavif-rs/actions/runs/41358897035).
