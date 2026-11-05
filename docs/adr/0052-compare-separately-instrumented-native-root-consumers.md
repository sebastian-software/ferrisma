# Compare separately instrumented native root consumers

Extend ADR 0009's AddressSanitizer parity to native root profiles. Generate
each source profile only from the ordinary original C reference. Build a
separate sanitized C baseline whose 28 libavif units match that reference's
compilation commands exactly, apart from the appended `-fsanitize=address`
and `-fno-omit-frame-pointer` flags. Link it to the complete instrumented
168-unit original AOM closure compiled from the matching Release commands.
Build the Rust profile with ASan in separate work and Cargo target
directories, so no ordinary proof, archive, or shared library is replaced.

Link both sets of unchanged original consumers to the selected Rust
nightly's runtime. On GNU Linux x86-64, that runtime is a static archive;
link it once into each original executable, as rustc does for instrumented
Rust executables, and do not mix in Clang's runtime. Keep LeakSanitizer's
platform default and upstream's `allocator_may_return_null=1` setting.
Native VMAF, the precompiled Rust standard library, and the pinned original
dav1d/SharpYUV archives used by the C baseline remain uninstrumented.

Require every compared C and Rust workspace codegen unit to reference the
runtime and every consumer to contain it. Run the root ownership audit and
the complete original suites, with exact ordered case and dynamic-skip
comparison. Exact bytes, ABI probes, and the shared-library smoke use the
ordinary profile and are not repeated here. Sanitizer parity uses original
Release preprocessing; a passing run does not claim Debug, optional-engine,
or other sanitizer coverage.
