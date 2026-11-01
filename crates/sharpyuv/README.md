# libavif-sharpyuv

Mechanical portable translation of SharpYUV 0.4.2 from libwebp 1.6.0, commit
`4fa21912338357f89e4fd51cf2368325b59e9bd9`. The seven translation units correspond
to the original `sharpyuv` target with `WEBP_ENABLE_SIMD=OFF` and threading on.
The two SIMD translation units retain the original disabled-configuration
behavior. No C codec implementation is linked into this crate.

The Rust API keeps the original unsafe pointer, stride, ownership, callback,
and initialization contracts. It exports the original structures and functions,
including the inline options initializer as a Rust function. Read the preserved
headers under `upstream/libwebp/sharpyuv/` for caller preconditions.

`TRANSLATION.json` records each source, translation hash, and correction rule.
`tools/transpile_sharpyuv.py --check` reproduces the translation with C2Rust
0.21.0 and LLVM 16 using the recorded macOS preprocessing configuration.
Platform mutexes and assertions use the target libc; algorithms and numeric
expressions retain their source forms. See ADRs 0020 and 0021 for the mappings
and the explicit floating-point compiler configuration.

Supported validation targets are macOS ARM64 and Linux x86-64. Other targets,
SIMD implementations, safe API design, and optimization remain separate work.
Build with `cargo build -p libavif-sharpyuv --locked --release`; the root package
still has its own narrower target support. Registry publication is disabled.

The BSD-3-Clause license, original authors, and separate patent grant are
preserved verbatim in `LICENSES/` and indexed by `NOTICE.md`.
