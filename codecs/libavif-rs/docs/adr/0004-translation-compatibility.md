# Apply explicit mechanical corrections at the C-to-Rust boundary

Keep C2Rust's control flow and repeated header declarations, applying documented
corrections through `tools/import_translation.py`: Darwin AArch64 `long double`
maps to `double`, variadic diagnostics use Rust's ABI-compatible `VaList`, SDK
inline helpers remain local, and addresses of static arrays use raw address
operations. These changes repair language/ABI representation without changing
the library's algorithms or adopting a Rust ownership model.

Use `panic = "abort"` so Rust unwinding cannot cross C, and disable integer
overflow checks in both Cargo profiles to retain the source's integer execution
model. The generated Release preprocessor configuration has `NDEBUG`; a Cargo
debug build does not restore removed C assertions. Nightly is currently required
for the translated opaque external types; toolchain stabilization is deferred.

## Supersession review, 2026-11-10

[ADR 0036](0036-map-libaom-nonlocal-errors-to-rust-unwinding.md) supersedes the
`panic = "abort"` decision for libaom's nonlocal error handling. The shared
Ferrixel profiles explicitly use `panic = "unwind"`; the original rationale
above is retained as history. [Ferrixel ADR-0026](../../../../docs/adr/0026-codecs-in-one-repository-and-workspace.md)
moves profile ownership to the repository root and scopes disabled overflow
checks to the generated AVIF packages. It does not change their numeric contract.
