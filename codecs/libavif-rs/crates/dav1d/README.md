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

## Kernels rebuilt from the assembly

dav1d's speed is in its assembly, which this crate does not contain. Under
engine ADR-0044 D2 the kernels are rebuilt with `core::arch` intrinsics in
`src/simd/` and dispatched by `tools/enable_dav1d_dispatch.py` (list in
`dispatch.txt`): libavif-rs ADR 0062. Gates: `tools/check_dav1d_dsp.py --simd
neon|avx2` (original checkasm bodies, original assembly against the port at
the same level) and `tools/check_dav1d_simd_original.py` (whole decodes
against `tools/build_dav1d.py --asm`). `dav1d_set_cpu_flags_mask(0)` selects
the C kernels. Done so far on AArch64 NEON: the inverse transforms, the
entropy decoder (msac), the loop filter and CDEF at 8 and 16 bpc, and intra
prediction at 8 bpc.

Open work for this crate (the remaining SIMD kernels, safe wrappers, sanitizer coverage, the Argon
corpus, the fault-fuzzer ASan mode on Linux) is tracked in the GitHub issues.
Registry publication is disabled. The BSD-2-Clause license and original authors
are preserved in `LICENSES/` and indexed by `NOTICE.md`.
