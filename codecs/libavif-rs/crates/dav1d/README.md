# libavif-dav1d

Mechanical portable translation of dav1d 1.5.4, commit
`54706fc6bc0cdecab7e9593974a4039cc038fca7`. The complete immutable source,
including the original tests, is under `upstream/dav1d`. The original checkasm
framework and the decoder corpus are fetched into `build/deps`
(`python3 tools/import_dav1d.py --fetch`) and every tracked file hash is in
`provenance/dav1d-test-inputs.json`.

The reference configuration has both bit-depth instantiations, logging,
threading, all scalar DSP functions, no assembly, and `-ffp-contract=off`.
The original CLI muxers, demuxers, test code, and system facilities stay native
test infrastructure. The optional xxHash output muxer is outside the decoder and
is disabled; the original built-in MD5 verifier stays enabled.

The Rust API keeps the original unsafe contracts; see the crate documentation.
Source translation uses C2Rust 0.21.0 and LLVM 16.0.6. The Linux translator is
built with Rust 1.80.1 for compatibility with its legacy dependencies; the
decoder itself uses the nightly in `rust-toolchain.toml` on both targets.
`tools/transpile_dav1d.py --check` and `--release --check` reproduce the matching
host profiles. ADRs 0025 to 0034 document the configuration, the mechanical
corrections, and the test bridges.

## Alignment

Both Debug and Release keep their own original C preprocessing. The corrections
preserve field alignment, C11 atomic operations and orderings, raw address-taking,
and target runtime types. ADR 0033 additionally restores explicit local and
static object alignment that C2Rust omitted. The original Linux Release suite
passed before that correction; 1,182 Debug failures in `lf_mask` exposed the
missing stack-buffer alignment.

## Test infrastructure

- The original suite labels containing `asan`, `ubsan`, or `msan` name the
  regression corpus (archived OSS-Fuzz inputs), not instrumented or continuously
  fuzzed runs. See `tools/run_dav1d_tests.py`.
- The original checkasm test bodies compare original C and Rust function
  pointers. The test-only DSP binding bridge and its native helpers are ADR 0029
  and are absent from production builds, as are the fault-fuzzer adapters of
  ADR 0034.
- The public API probes (`tools/check_dav1d_api.py`) compare plane hashes and
  frame metadata and require every tracked callback resource to be released
  after close. They also run under AddressSanitizer with LLVM 16 and the
  matching Rust nightly ASan runtime. That is not complete leak, race, or
  undefined-behavior sanitizer coverage.
- The optional external Argon conformance corpus is not downloaded. Archived
  fuzz regressions and bounded original tests do not establish continuous
  fuzzing.

Open work for this crate (SIMD, safe wrappers, sanitizer coverage, the Argon
corpus, the fault-fuzzer ASan mode on Linux) is tracked in the GitHub issues.
Registry publication is disabled. The BSD-2-Clause license and original authors
are preserved in `LICENSES/` and indexed by `NOTICE.md`.
