# Generate native root engine profiles from pinned external engines

Extend ADR 0050's native profile parity to the optional AV1 engines of ADR
0007. Name each root configuration by its container features followed by
its selected engines, in the order `mini`, `extended-pixi`, `libgav1`,
`rav1e`, `svt`, and `libyuv`; the default container with all four engines is
`libgav1-rav1e-svt-libyuv`. Generate the profile from the actual original C
build of that configuration. Its source closure is the one upstream CMake
compiles: the selected `codec_*` adapters are added, and external libyuv
replaces the five bundled scalar units. The generated registry keeps the
shared feature-guarded codec table, so the profile is built with the
matching Cargo features.

On native Linux, build every selected engine from its pinned source inside
`build/deps`: libgav1 v0.20.0 and libyuv at the existing revisions as static
archives, rav1e v0.8.1 with cargo-c as a shared C library, and SVT-AV1
v4.2.0 as a shared library. This matches the macOS library types, where
rav1e and SVT-AV1 come from shared packages. Reject a C reference whose
rav1e or SVT-AV1 library resolves outside `build/deps`. The engines are
always built in Release; Debug and Release consumers of one configuration
link the same engine binaries. Record their revisions and library hashes.

The engines remain the original external libraries. Their static archives
are not bundled into the Rust staticlib; both consumer sets link the same
imported CMake targets, which follow the Rust archive on GNU linkers. Shared
engines resolve through the Rust shared library's run path. Translation uses
ADR 0007's libyuv header overlay only. Root sanitizer parity remains limited
to container configurations, and AVM/AV2 stays a separate configuration.

## Amendment: DAV1D=OFF (2026-11-16)

Every root configuration, `default` included, is now the original C build
with `AVIF_CODEC_DAV1D=OFF` ([ADR 0061](0061-build-the-root-with-avif-codec-dav1d-off.md)):
"default" names the container without optional engines, built without
dav1d. The closure loses `codec_dav1d.c` (27 units for the container
configurations, 25 with all four engines), the root references contain
SharpYUV only, and the shared registry has no dav1d entry.
