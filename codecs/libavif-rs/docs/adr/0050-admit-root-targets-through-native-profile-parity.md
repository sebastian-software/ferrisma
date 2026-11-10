# Admit root targets through native profile parity

Extend the verified macOS ARM64 root pipeline to GNU Linux x86-64 first, then
Windows x86-64. Generate isolated profiles from each native original C build,
with matching Debug/Release preprocessing, pinned dependencies, and unchanged
upstream algorithms and tests. Keep the existing macOS source profiles and
acceptance evidence intact while new profiles are prepared. Source generation
and typechecking are preparation gates; admit a target only after its actual
original C/Rust test inventories, ABI, exact AVIF/PNG comparisons, and complete
implementation/linkage audits pass.

Use a short PR gate and a nightly/manual deep gate. The deep root gate runs
the complete original root integration suites in Debug and Release, including
the supported feature configuration and applicable sanitizer checks. Preserve
per-target cases, dynamic skips, XML, logs, binary identities, and configurations.
Keep nightly concurrency separate from PR cancellation. The separate long
original AOM video suite retains ADR 0048's complete Release and focused Debug
policy; ordinary root CI does not restart exhaustive unoptimized AOM matrices.
