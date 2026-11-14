# Getting started

Ferrisma is an experimental Rust image pipeline. Its M1 release
and audit gates remain open; see [current status](status.md).

## Build the CLI

Clone the repository and run from its root. The root toolchain file selects the
nightly needed by the workspace's generated AVIF code. On x86-64, the AVIF
path needs NASM.

```sh
git clone https://github.com/sebastian-software/ferrisma.git
cd ferrisma
cargo run --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  photo.jpg scaled.webp 1600 900 --assume-srgb --quality=80 --speed=8

cargo run --release --features cli -- \
  drawing.webp resized.webp 800 600 --assume-srgb --webp-lossless --webp-level=6
```

`--version` (or `-V`) prints four lines and exits without reading input: the
package version, the target, the compiled engine features, and the build
identity, which is `unknown` unless `FERRISMA_BUILD_ID` was set at compile
time (`src/cli.rs`, #237).

The `cli` feature enables JPEG, still WebP, JPEG XL and AVIF input, AVIF and
still WebP output, and ICC support. Since 2026-11-12 the default AVIF encoder
is the workspace libavif/AOM writer (`avif-own-write`, 10-bit unless
`--avif-depth=8`); `--avif-encoder=rav1e` selects `rav1e` (`avif-write`)
explicitly. Rust callers use `AvifEncoder::Rav1e` or
`AvifEncoder::Libavif { depth: 8 }` in `TransformOptions`.
The separate [`ferrisma-libavif` adapter](../crates/ferrisma-libavif/README.md)
delegates to the same implementation. `--quality` and `--speed` go unchanged to
the selected encoder, on its own scale: rav1e accepts quality 1–100, libavif
0–100 in 64 quantizer steps, and the numbers are **not** comparable. At equal
SSIMULACRA2 on the recorded photographs, rav1e 70/80/90 matched roughly
libavif 50/55/78, so a quality value tuned for rav1e gives larger files with
libavif ([evidence](benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md)).
The current `avif-read` uses the same
workspace port's AOM decoder through an isolated safe wrapper, so the former
`rav1d`/dav1d symbol collision is gone. Own AVIF reading and writing require the pinned
nightly and a supported native target (macOS ARM64 or GNU Linux x86-64).
The codec-free core and features without `avif-read`, `avif-own-write`, or `cli` retain stable Rust 1.98
support. See the [measured AVIF input trade-off](benchmarks/M2-avif-input.md).

WebP lossy uses its own quality and speed settings. Lossless uses only
`--webp-level=0..9`; `--quality`, `--speed`, `--chroma`, and multiple encoder
threads are unsupported for that output. Still WebP output embeds an sRGB ICC
profile for the rendered samples and drops source EXIF orientation and XMP.
`--max-output-mb` caps the retained file; the WebP writer checks it during
encoding, while AVIF and external backends check after encoding.

JPEG XL input uses the workspace libjxl-rs reference decoder
(`jpeg-xl-own-read`, part of `cli`) by default since 2026-11-12. `jxl-oxide`
(`jpeg-xl-read`, also part of `cli`) stays available explicitly:

```sh
cargo run --release --features cli -- \
  photo.jxl scaled.avif 1600 900 --use-rendered-icc
cargo run --release --features cli -- \
  photo.jxl scaled.avif 1600 900 --use-rendered-icc --jxl-decoder=jxl-oxide
```

The default decoder has no region request: it decodes and budgets a complete
F32 input frame for every request, including crops, and reports a full-frame
fallback. The owner accepted the workspace decoder as the default even where
it is slower on 2026-11-13
([ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md) D1,
[ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)); its speed is
reported against libjxl, the original it ports, and jxl-oxide is context only
(ADR-0041 section 6). The [jxl-oxide comparison of 2026-11-11](benchmarks/M2-jxl-input-switch.md)
(about 3.6× slower for a full decode and 9.5× slower for a center crop, one
photo recompressed from a JPEG) describes the decoder before its SIMD path and has
not been repeated since; it is superseded as a description of today's
decoder. Since 2026-11-14 the decoder selects its NEON path on AArch64; on
x86-64 it runs the scalar path. The
[AArch64 record of 2026-11-14](benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md)
measured it at 1.3–2.9× the time of libjxl's NEON build with one thread on
eight inputs, and at 1.28× and 1.57× on the two inputs measured again after
the render-pipeline changes (one machine; no x86-64 numbers).

Use `--assume-srgb` only when you accept that assumption for the input. For a
supported rendered RGB ICC profile, `--use-rendered-icc` selects the explicit
ICC path. Untagged color is not automatically sRGB; unsupported HDR and
ambiguous color are rejected. Orientation is applied before crop coordinates;
JPEG and lossless WebP codec internals, as well as AVIF input, may require
full-frame work. The AVIF and WebP encoders materialize output frames. See the
[color contract](design/color-management-contract.md)
and [architecture](architecture.md).

The [supported matrix](supported-matrix.md) lists what each format accepts and
rejects, the default and explicit backends, feature flags, toolchains and
examples of rejected inputs with the error to expect.

For codec source setup and the contributor checks, see [codec development](codecs.md)
and [validation](validation.md).
