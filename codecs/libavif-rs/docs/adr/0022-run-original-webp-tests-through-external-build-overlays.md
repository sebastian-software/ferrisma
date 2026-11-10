# Run original WebP tests through external build overlays

Preserve every imported WebP source, test, fixture, and CMake file. Build all
twelve original FuzzTest consumers with the original optional mux and webpinfo
targets enabled. Use the pinned FuzzTest revision selected by WebP and apply
WebP's original dependency patch to a disposable export. Native and Rust
consumers have separate build directories and process execution.

The external CMake overlay enables testing at the project root because the
original project registers tests without calling `enable_testing()` there.
On macOS ARM64 it removes x86 AES flags from the two Abseil random targets:
CMake's option deduplication otherwise strips a repeated architecture prefix
and exposes `-msse4.1` to the ARM compiler. Keep the intended ARM crypto flag.
A local sed wrapper translates GNU in-place argument spelling to BSD sed for
the unchanged original dependency patch. These are test-build compatibility
adjustments applied equally to both consumers, not changes to test assertions.

Linux test consumers use Clang: the pinned FuzzTest adds
`-fsanitize-coverage=0` to an internal logging target even in unit-test mode,
which GCC rejects. The standalone native API reference keeps the system C
compiler default and is independent of this C++ test-framework requirement.

On x86-64, remove the original build's global `-mno-sse2` flag for Clang:
floating-point calls require baseline SSE2 registers under the platform ABI.
Keep all SIMD feature macros disabled and verify that configuration before
building. This permits ordinary compiler code generation, already allowed by
the portable reference contract, without selecting manual SIMD DSP routines.

The original EncDec fuzzer also dereferences its default CPU-query callback on
macOS ARM64 when `WEBP_ENABLE_SIMD=OFF` initializes that callback to NULL. LLDB
locates the native-reference crash in `GetCPUInfoForceSlowSSSE3`, called from
WebP decoding. Compile only WebP's original `src/dsp/cpu.c` with
`WEBP_HAVE_NEON=1` to provide its original ARM feature-query function. All DSP
and SharpYUV translation units retain the SIMD-disabled configuration. This
does not enable an optimized conversion, replace a test, or alter an assertion.

Replace only the original `sharpyuv` target's implementation with an empty
linkage anchor and the Rust static archive. Compile-command, archive, symbol,
and dynamic-linkage audits must establish that native SharpYUV cannot mask the
candidate. WebP remains an original C test consumer; this does not port WebP.
Run FuzzTest's original bounded unit-test mode with a recorded PRNG seed.
This smoke run does not claim a continuous fuzz campaign or an external corpus
that has not been fetched and executed.

Disable the test frameworks' own self-test suites, including ANTLR's. They are
outside the WebP test inventory, and ANTLR's self-test post-build step otherwise
copies binaries into the top-level WebP source directory. Source verification
rejects any such generated additions; all generated artifacts stay under build/.
