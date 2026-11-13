# Select native Linux root profiles in the production package

Admit GNU Linux x86-64 to the production `libavif-rs` package by compiling
the generated native profile of its identical original C configuration, as
ADR 0050 requires. Keep the macOS ARM64 module tree and build-script output
unchanged. The target guard admits only `aarch64-apple-darwin` and
`x86_64-unknown-linux-gnu` (`target_env = "gnu"`).

On Linux, the canonical Cargo feature set names the profile variant in ADR
0053's order. Each Cargo feature takes part, so AVM/AV2, single or partial
engine selections, and engines with one container feature match no profile
and fail with `compile_error!`. Select original Debug preprocessing exactly
when `debug_assertions` is enabled, otherwise Release. The profiles differ in
`NDEBUG` and its assertion expansions (ADR 0051); `debug_assertions` is Rust's
corresponding setting, and Cargo's `dev` and `release` profiles produced the
Linux evidence. Optimization level does not select a profile; the root
manifest keeps overflow checks off and unwinding in both profiles.

`tools/update_production_selection.py` generates the selector from
`root_variants.LINUX_PRODUCTION` and the existing profile manifests. Admit a
configuration there only after the production consumer check passes in
Release and Debug. On Linux, the build script emits the isolated profile's
link directives: `-Bsymbolic` for the shared library and, only for all four
optional engines, unbundled pinned libgav1/libyuv archives plus rav1e and
SVT-AV1 through `pkg-config` with the shared library's run path.

The production consumer check regenerates the profile from the host's actual
original C reference and requires the committed sources and selector unchanged.
It builds the root package with Cargo and uses Cargo's dep-info to prove that
exactly the profile units compiled; on macOS the existing units must equal the
generated macOS profile byte for byte. The same original suites, ABI probes,
ownership audits, strict bytes, and shared-library smoke then run against that
package. Isolated profiles remain the admission path for new configurations
and sanitizer parity. Production sanitizer and AVM/AV2 claims remain separate.

## Amendment: the owned read seam (2026-11-13)

Commit `1ec31c68` (workspace AVIF input, Ferrisma ADR-0031) composes every
`read` module from the unchanged generated `read.rs` and the handwritten
`src/read_sample_access.rs`, through `CARGO_MANIFEST_DIR` includes that the
generators (`tools/transpile_root.py`, `tools/update_modules.py`) now emit. The
seam must share the module because it calls the file-local
`avifDecoderPrepareSample`. No translated unit and no unit hash changed, but
both root checks broke, and every root job of the nightly has failed since
(issue #204):

- The production check above counted the seam as a compiled unit outside the
  profile ("The production package compiled a different translation than the
  native profile" on Linux, "units differ" on macOS).
- The isolated profile crate has its own manifest directory, so its
  `CARGO_MANIFEST_DIR` includes named files that do not exist there.

The seam is project-owned code, not translation. `root_production.OWNED`
declares it: the check requires it to be compiled, excludes it from the
translation equality by name, rejects it as a profile unit, and keeps its
digest in the proof's compiled sources. The isolated crate links each file a
selector includes this way to the same tracked file
(`check_root_profile.link_manifest_includes`). Unit hashes and selector hashes
are unchanged. Another owned file in a translated module is declared the same
way. Working decision 2026-11-13 (delegated, revisable by the owner).
