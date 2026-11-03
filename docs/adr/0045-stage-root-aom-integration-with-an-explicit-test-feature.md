# Stage root AOM integration with an explicit test feature

[ADR 0048](0048-require-complete-release-and-focused-debug-aom-acceptance.md)
replaces the full Debug standalone gate below with focused Debug checks.
The complete native Release and final root integration gates still apply.

Use the temporary `port-aom-candidate` Cargo feature to test the existing
libavif adapter against the workspace Rust AOM crate while complete standalone
acceptance is pending. Keep the adapter's translated implementation, public
headers, registry, and original tests unchanged. The default feature set keeps
its existing native engine during this gate.

When the candidate is selected, skip native `aom` pkg-config discovery and
retain the Rust dependency's symbols in the root artifacts. Original CMake
consumers use the pinned AOM headers and retained native VMAF runtime through
an interface target, with system AOM discovery disabled. They compile no C
libavif or AOM implementation. Audit every original AOM global in the root
archive and all loaded definitions in consumer maps, then resolve non-system
runtime dependencies to reject native AOM, dav1d, or SharpYUV regressions.

Keep candidate Cargo targets separate by feature configuration and sanitizer
so validation cannot overwrite archives used by running standalone shards.
An explicit native dependency root may reuse pinned test dependencies and
optional engines from another checkout without moving the tracked work there.
Repository defaults continue to use the current checkout.

Keep the original C integration reference reproducible through
`tools/build.py baseline --aom-reference-build PATH`. Require the matching
Debug/Release profile, all 168 original C units, the noncontracting numeric
flags, and every generated header from the selected translation manifest.
Use an explicit original archive with pinned public headers and retained VMAF;
disable system AOM discovery. Keep its build separate from the existing native
baseline and Rust candidate. The ordinary reference option does not accept
ASan: that comparison requires the separately instrumented 168-unit closure.

Remove this test feature and make Rust AOM unconditional after the complete
Debug/Release standalone gates pass on native macOS ARM64 and GNU Linux
x86-64, and the original root suites and strict AVIF/PNG comparisons pass with
the final wiring. Remove the root native AOM discovery path at the same time.
Remove the temporary selector when that gate passes. Native VMAF and the
separately scoped optional engines remain explicitly inventoried.
