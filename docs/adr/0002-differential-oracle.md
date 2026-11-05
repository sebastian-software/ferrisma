# Compare against the original C implementation in separate processes

- Status: accepted
- Created: 2026-11-05
- Updated: 2026-11-05

## Context

A Rust round trip can agree with itself while both directions differ from
libwebp. Evidence must come from the pinned C code, built in a fixed
configuration, and the C code must never become part of the library that
Ferrixel links.

## Decision

**Oracle build.** `tools/build_oracle.py` configures `tools/oracle/CMakeLists.txt`,
which adds the unmodified `upstream/libwebp` through its own CMake project in
the reference configuration of the port plan:

```text
-DCMAKE_BUILD_TYPE=Release -DWEBP_ENABLE_SIMD=OFF -DWEBP_USE_THREAD=ON
-DWEBP_NEAR_LOSSLESS=ON -DWEBP_ENABLE_SWAP_16BIT_CSP=OFF -DWEBP_BUILD_DWEBP=ON
-DWEBP_BUILD_CWEBP=ON -DWEBP_BUILD_WEBPINFO=ON -DWEBP_BUILD_LIBWEBPMUX=ON
-DWEBP_BUILD_ANIM_UTILS=OFF -DWEBP_BUILD_GIF2WEBP=OFF -DWEBP_BUILD_IMG2WEBP=OFF
-DWEBP_BUILD_VWEBP=OFF -DWEBP_BUILD_WEBPMUX=OFF -DWEBP_BUILD_EXTRAS=OFF
-DCMAKE_C_FLAGS=-ffp-contract=off
```

The build fails if the generated `config.h` defines any `WEBP_HAVE_<SIMD>`
macro. One deliberate addition: on x86-64, upstream's `WEBP_ENABLE_SIMD=OFF`
also appends `-mno-sse2`, which removes the SSE2 baseline of the x86-64 psABI,
and Clang then rejects every function that returns a `double`
(`SSE2 register return with SSE2 disabled`; `src/dsp/ssim.c` has several). The
oracle appends `-msse2` on x86-64 only. That restores the ABI baseline, which
the Rust port uses as well; `-mno-sse4.1` and `-mno-avx2` stay and no intrinsics
file is compiled. A cross-compile on macOS confirmed both the failure and the
fix; the Linux CI job is the first native evidence.

Three variants land in `build/oracle/<variant>/`: `release` is the reference;
`asan-ubsan` is the same build with AddressSanitizer and
UndefinedBehaviorSanitizer on the library, the tools and the probes, used to
find undefined behavior in the original per case; `debug` enables upstream's
assertions. Each variant writes `oracle.json` with the compiler, the effective
flags, the `config.h` defines and the probe list. A SIMD-on oracle is
diagnostic only and is not built by default.

**Probes.** Every `tools/oracle/probes/<wp>/<name>.c` becomes the executable
`build/oracle/<variant>/probes/<wp>/<name>`, compiled with the library's own
effective flags and definitions and linked against its static libraries.
Sources in `probes/<wp>/support/` form a helper library for that package's
probes, and an optional `probes/<wp>/oracle.cmake` adds further targets, such as
consumer builds of the upstream tools. Probes may include internal headers
(`"src/dec/vp8i_dec.h"`) and call internal non-static functions. They call
upstream code and never reimplement its algorithms; a probe that needs a
`static` function reaches it through the function that calls it, or records why
it cannot. Probes print one `key=value` line per observation, integers in
decimal and byte strings in lowercase hex (`tools/oracle/probe_support`).

**Tests.** Differential tests live in `tests/<wp>_*.rs` and use
`tests/support/oracle.rs`: `LIBWEBP_ORACLE_DIR` names the variant directory,
and the helper runs a probe with arguments and standard input in a separate
process and returns its standard output, standard error and exit status. A test
that needs the oracle skips with a message when the variable is unset; with the
`oracle` Cargo feature it fails instead. Ordinary `cargo build` compiles and
links no C: there is no build script and no C dependency.

Compare exact bytes, exact status codes and the partial state that a failing
call leaves behind. Floating-point results are compared for exact equality on
the same host; cross-host equality is recorded, not assumed. Inputs on which
the C code has undefined behavior are excluded explicitly with a record in the
package's status file. Tests pass in the dev, release and `checked` profiles.

**Baseline.** `tools/oracle_baseline.py` runs upstream's `test_dwebp.sh` and
`test_lossless.sh` against the oracle's own `dwebp` and checks exact counts.
[PORT_STATUS.md](../PORT_STATUS.md) records them. The test data is pinned at the
last revision whose references match v1.6.0.

## Consequences

The checks need CMake and a C compiler; ordinary builds do not. Probes are C
code that packages write and review alongside their Rust translation. Bit
equality is established per host and per compiler; macOS ARM64 with Apple Clang
and Linux x86-64 with Clang are the CI hosts.

## Review triggers

Revisit when the upstream pin, the reference configuration, the compiler or a
CI host changes, or when a package cannot observe a translated function through
a probe.
