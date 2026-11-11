# Ferrixel

**A Rust image pipeline and its codec ports, developed in one repository.**

Ferrixel combines an image processing engine, CPU kernels, and Rust ports of
JPEG, WebP, JPEG XL, and AVIF libraries. The goal is to make color, decoder
capabilities, intermediate buffers, and execution costs explicit throughout
the pipeline.

**Experimental and unpublished.** The JPEG/WebP/JPEG XL/AVIF → crop/resize → AVIF
pipeline, Rust facade, and CLI work for supported SDR inputs. M1 quality,
hardening, dependency, and release gates remain open; there is no stable API
or production-readiness claim.

## What's here

| Component | Current role |
| --- | --- |
| [Engine](src/pipeline.rs) | Experimental SDR pipeline with orientation, linear-light resampling, parallel execution, and reported materializations |
| [CPU kernels](crates/ferrixel-kernels) | Scalar, NEON, AVX2, and AVX-512 resampling behind a safe API |
| [JPEG port](codecs/libjpeg-turbo-rs) | In-scope 8-bit decoder with a safe row API; engine adoption pending |
| [WebP port](codecs/libwebp-rs) | Decoder, encoder, container, and SharpYUV APIs; still-image input is connected through `webp-read` |
| [JPEG XL port](codecs/libjxl-rs) | Incremental decoder with provisional APIs; encoder incomplete; engine adoption pending |
| [AVIF ports](codecs/libavif-rs) | libavif, AOM, dav1d, and SharpYUV; an [optional encoder adapter](crates/ferrixel-libavif) exercises the port |
| [Validation tooling](docs/codecs.md) | One Rust runner, scoped PR checks, and deeper independent oracle suites |

The four codec ports share the Cargo workspace, but importing them does not
automatically select them as engine backends. Ferrixel currently reads through
`zune-jpeg`, the workspace `libwebp-rs`, `jxl-oxide`, and `avif-decode`, and writes AVIF through `rav1e`.

**[Current status and remaining gates](docs/status.md)** gives the component
and integration overview. **[Documentation guide](docs/README.md)** maps the
architecture, development instructions, decisions, and recorded experiments.

## Try the experimental CLI

From a checkout with Rust installed (NASM is also needed on x86-64):

```sh
cargo run -p ferrixel --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8
```

Use `--assume-srgb` only when that input assumption is appropriate; supported
RGB ICC input can use `--use-rendered-icc`. Unknown or contradictory color,
HDR, and unsupported alpha cases fail explicitly. Crops use upright image
coordinates. Full-frame codec buffers and opaque native memory remain relevant
even when the transform streams rows.

For features, build prerequisites, and scoped checks, see
[getting started](docs/getting-started.md), [contributing](CONTRIBUTING.md),
and [codec development](docs/codecs.md). The [Ardo website](homepage/README.md)
presents these guides with the shared Ferramenta theme and can be previewed locally.

## License

The engine is **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**. Codec
packages retain their own licenses, notices, and publication gates. The
[dependency audit](docs/license-audit.md) includes the interim AVIF parser's
MPL-2.0 obligations; the root license does not relicense the workspace.
