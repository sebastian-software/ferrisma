# Use Rust AOM as the mandatory root codec

Use the portable `libavif-aom` workspace crate in every normal libavif Rust
build. Complete original native Linux x86-64 and macOS ARM64 Release suites,
focused Debug checks, and the finite subject/oracle correspondence review
satisfy the standalone prerequisites from ADR 0048. Remove the temporary
`port-aom-candidate` feature and production system-AOM discovery so the default
build cannot silently use the original implementation.

Keep original C AOM builds as independent validation references, with the
declared `-ffp-contract=off` numerical contract. Retain native libvmaf and the
separately scoped optional engines. Publish the default change only after
final root Release, supported-feature, focused Debug, ASan, consumer/runtime
ownership, and strict AVIF/explicit-AOM-decoder PNG parity checks pass.

## Amendment: the only AV1 decoder (2026-11-16)

Since [ADR 0061](0061-build-the-root-with-avif-codec-dav1d-off.md) the root
is libavif with `AVIF_CODEC_DAV1D=OFF`: the libaom translation is also the
AV1 decoder that libavif's AUTO choice selects, in the original and the port
alike (libgav1 precedes it where that optional engine is selected). The
strict AVIF/PNG parity checks above now compare that configuration; the
"explicit AOM decoder" pass and the default pass decode with the same codec.
