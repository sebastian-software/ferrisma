# Compare against the original C decoder in separate processes

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-12

## Context

A Rust decoder can agree with itself and still differ from libjpeg-turbo.
Evidence must come from the pinned C code in a fixed configuration, and that C
code must never become part of the library Ferrisma links.

## Decision

**Reference configuration.** The oracle is the unmodified
`upstream/libjpeg-turbo`, built with CMake in this configuration:

```text
-DCMAKE_BUILD_TYPE=Release -DWITH_SIMD=0 -DWITH_ARITH_DEC=1 -DWITH_ARITH_ENC=1
-DWITH_TURBOJPEG=1 -DENABLE_SHARED=0 -DENABLE_STATIC=1
-DCMAKE_C_FLAGS=-ffp-contract=off
```

`WITH_SIMD=0` makes the C reference portable, which the scalar Rust port
matches bit for bit. `-ffp-contract=off` stops the compiler from fusing the
floating-point IDCT's multiplies and adds, which Rust never does implicitly.
Upstream's float bit tests pick their expected hashes with `FLOATTEST8`, so the
oracle sets `FLOATTEST8=no-fp-contract` to match the flag; upstream's default
for Clang 14 and later on ARM64 is `fp-contract`. The encoder and the
TurboJPEG API are built only to produce test inputs.

**Oracle variants.** `tools/build_oracle.py` builds each variant into
`build/oracle/<variant>/` and records the compiler, the CMake options, the
flags and the configuration defines in its `oracle.json`:

| Variant | Purpose |
| --- | --- |
| `release` | the reference oracle, in the configuration above |
| `asan-ubsan` | the same configuration, with the library, the tools and the probes instrumented with AddressSanitizer and UndefinedBehaviorSanitizer, so that undefined behavior on the C side of a comparison fails instead of passing by chance |
| `simd` | `WITH_SIMD=1`, upstream's default build: the oracle of the port's SIMD configuration ([ADR 0004](0004-simd-after-the-portable-port.md), [ADR 0007](0007-neon-kernels-and-unsafe.md), [ADR 0008](0008-x86-64-kernels-from-nasm.md)). On x86-64 it is built with NASM, and its float bit tests expect upstream's `FLOATTEST8=sse` hashes |

Upstream's whole CTest suite runs against every variant
(`tools/bittests.py --run --all`), as a check of the oracle itself, before the
variant serves as evidence.

**Unit probes.** Small C programs link the oracle's static library, call one
upstream function or one public API sequence, and print their observations as
`key=value` lines. Rust tests run them in a separate process and compare exact
bytes, exact error and warning codes, and the state a failing call leaves.
`tools/oracle/probe_support` records the error code, the message and the
warnings before it leaves a failing call with `longjmp`, so that a probe can
print that state. Probes that print the object or its memory link their own
copy of `jmemnobs.c`'s functions that allocates with `calloc` instead of
`malloc`: memory that a failing or suspended call leaves unwritten is then zero
in C, as it is in the port, and can be compared. The library itself stays
unmodified. Probes also print the memory pools, whose used sizes contain the C
size of every allocation, which verifies the C sizes that the port's
accounting names ([ADR 0005](0005-rust-representation.md)).

**Upstream tests.** Upstream's CMake test suite checks `djpeg` outputs by MD5
(`add_bittest` in `CMakeLists.txt`: DCT methods, sampling factors,
progressive and arithmetic files, scaling factors, `-skip` and `-crop`). The
port exports no C ABI: it serves Rust programs, and C compatibility is not a
goal. Instead, a Rust test runs the 8-bit decoder cases of that suite on the
port: it makes the library calls that `djpeg.c` makes for the options those
cases use and writes the output as `wrppm.c` does, both translated as test
code, and compares the result with the case's original expected hash. The
input is read with `jpeg_mem_src`, because the stdio source is out of scope;
for a complete file it delivers the same data. Cases outside the scope of
[ADR 0001](0001-mechanical-port.md) (12-bit, lossless, quantized and RGB565
output, and the `cjpeg` and `jpegtran` cases) are listed with their reason,
not silently skipped: [tools/bittests.json](../../tools/bittests.json)
classifies every `add_bittest` case that upstream's static build declares,
and `tools/bittests.py --check` verifies that list, its arguments and its
inputs against the configured CTest suite, so the inventory stays
upstream's. Upstream's `example-8bit-*-decompress` test is not an
`add_bittest` case and is not part of it. On an x86-64 `simd` oracle with
`JSIMD_FORCENONE=1`, the C float IDCT runs, in upstream's suite as in the
port, so the float cases' `FLOATTEST8=sse` hashes do not apply and the
driver skips them in that mode. Upstream's fuzzers are C programs that would
need a C ABI; the damaged-stream tests of each package cover corrupt input
against the oracle instead, and coverage-guided fuzzing of the safe API is
[ADR 0009](0009-coverage-guided-fuzzing.md)
([ferrisma#73](https://github.com/sebastian-software/ferrisma/issues/73), formerly libjpeg-turbo-rs#18).

**Mutations.** A passing test shows nothing unless it can fail. Each change
that adds or changes translated code or a check shows, before it merges, that
the tests catch a deliberate fault in what it covers. Such faults include a
changed constant, lane, sign, shift or bias, a dropped or reordered
operation, or an off-by-one at a row tail. Do this for each instruction set
where kernels differ. The change is reverted afterwards. The pull request
names the faults tried and the test that failed for each. A fault that no
test catches either gets a test or is shown to be equivalent, that is, to
give the same output for every input the code can receive, and the pull
request says why.

**Conformance.** Before Ferrisma adopts the decoder, it is checked with the
ISO/IEC 10918-2 (ITU-T T.83) compliance test data for the processes in scope:
baseline, extended (8-bit Huffman and arithmetic) and progressive. ITU
publishes the data as one zip file
(`https://www.itu.int/wftp3/Public/t/testsignal/SpeImage/T83/v1994_11/T83_data_files.zip`,
3,924,283 bytes, SHA-256
`9f5ed0a74a3087c6c93f4cbb0bd5128c2a87b128e4f59f5b6649e32941da7069`). Its
Readme says "(c) ITU & ISO/IEC 1994" and grants no license to redistribute
it, so the repository does not contain it: `tools/fetch_t83.py` downloads it
(or takes a local copy), verifies it and extracts the files in scope into
`build/t83/`, and CI caches the zip under its SHA-256.

In scope are the 8-bit DCT streams A1, B1, B2, C1, C2, D1, D2, G1, H1, K1 and
L1, the decoder reference test data `DREF_A8.DCT` to `DREF_D8.DCT` and the
encoder reference test data `EREF_A8.DCT` to `EREF_D8.DCT`. Out of scope are
the 12-bit and lossless streams ([ADR 0001](0001-mechanical-port.md)), the
encoder tests (the source images), and the streams T.83 never published.
B1 holds tables only: it returns `JPEG_HEADER_TABLES_ONLY` with
`require_image` false and fails with `JERR_NO_IMAGE` with true. B2 is an
abbreviated image with 255 components, and libjpeg's `MAX_COMPONENTS` is 10.
The port keeps that upstream limit, so B2 fails with `JERR_COMPONENT_COUNT` at
its first scan, also after B1's tables in the same object, as it does in the
oracle.

ISO/IEC 10918-2:1995 defines the reference data: EREF holds the quantized
coefficients of the source image (3.1.35), which the streams code exactly,
and DREF the result of the reference FDCT and reference quantizer applied to
the output of the reference decoder (3.1.34), that is entropy decoding,
dequantization and the reference IDCT (3.1.39 to 3.1.43; the DCTs are the
equations of T.81 A.3.3 in floating point, the quantizer is T.81 A.3.4).
DREF therefore differs from EREF by one in a few coefficients (24 of
component A and 52 of B), as an independent decoder author also observed
([A. Tarpai](https://halicery.no/DecodersImage/JPEG/JPEG81.html)). The tolerance
of T.83 clause 7 and Annex A was not available, so the conformance run
records the measured values instead of judging them against it. On
2026-11-12 the ITU page of the only edition, T.83 (11/94), in force, still
offered the text only to TIES users and "through payment", and its download
link led to the TIES login; no edition dated 09/92 exists. A bound taken from
secondary sources is not used: the test asserts T.83's bound only once the
authoritative text has been read.

The recorded checks:

1. The port's raw output with the islow IDCT equals `jpeg_idct_islow`
   applied to EREF with the stream's quantization tables, bit for bit over
   every block: the entropy decoders and dequantization reproduce the coded
   coefficients.
2. T.83's decoder test: the reference FDCT and quantizer applied to the
   port's raw output for each IDCT method, compared with DREF. The test's
   own reference decoder must turn EREF into DREF exactly. T.81 A.3.4 asks
   for the nearest integer and picks none at a tie, so a quotient within
   1e-9 of a half, a tie in exact arithmetic, matches if DREF holds either
   of its two nearest integers; the ties are counted and recorded.
3. The raw output of each IDCT method against the reference IDCT of EREF.
4. Every stream through the API probe and the port, compared exactly.

Floating-point results are compared for exact equality on the same host;
cross-host equality is recorded, not assumed. The CI hosts are macOS ARM64 with
Apple Clang and Linux x86-64 with Clang, which run the `release` and
`asan-ubsan` oracles; the `simd` oracle runs on both and on Linux ARM64.

## Consequences

Checks need CMake and a C compiler; ordinary `cargo build` compiles and links
no C. Probes are C code, written and reviewed alongside the translation they
check.

## Review triggers

Revisit when the pin, the reference configuration, the compiler or a CI host
changes, when a unit cannot be observed through a probe, or when the
tolerance of ISO/IEC 10918-2 clause 7 becomes available
([ferrisma#72](https://github.com/sebastian-software/ferrisma/issues/72), formerly libjpeg-turbo-rs#19).
