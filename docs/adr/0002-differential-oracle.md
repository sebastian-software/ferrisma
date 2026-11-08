# Compare against the original C decoder in separate processes

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-08

## Context

A Rust decoder can agree with itself and still differ from libjpeg-turbo.
Evidence must come from the pinned C code in a fixed configuration, and that C
code must never become part of the library Ferrixel links.

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
floating-point IDCT's multiplies and adds, which Rust never does implicitly. The
encoder and the TurboJPEG API are built only to produce test inputs. A SIMD
build is a second, separate oracle for [ADR 0004](0004-simd-after-the-portable-port.md).

**Unit probes.** Small C programs link the oracle's static library, call one
upstream function or one public API sequence, and print their observations as
`key=value` lines. Rust tests run them in a separate process and compare exact
bytes, exact error and warning codes, and the state a failing call leaves.

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
output) are listed with their reason, not silently skipped. Upstream's
fuzzers are C programs that would need a C ABI; the damaged-stream tests of
each package cover corrupt input against the oracle instead, and
coverage-guided fuzzing in Rust is a separate decision, since it needs a
development dependency ([ADR 0003](0003-toolchain-and-unsafe-policy.md)).

**Conformance.** Before Ferrixel adopts the decoder, it is checked with the
ISO/IEC 10918-2 (ITU-T T.83) compliance test data for the processes in scope:
baseline, extended (8-bit Huffman and arithmetic) and progressive. Obtaining
that data is part of the plan.

Floating-point results are compared for exact equality on the same host;
cross-host equality is recorded, not assumed. The CI hosts are macOS ARM64 with
Apple Clang and Linux x86-64 with Clang.

## Consequences

Checks need CMake and a C compiler; ordinary `cargo build` compiles and links
no C. Probes are C code that each work package writes and reviews alongside its
translation.

## Review triggers

Revisit when the pin, the reference configuration, the compiler or a CI host
changes, or when a unit cannot be observed through a probe.
