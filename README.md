# Ferrisma

**[ferrisma.dev](https://ferrisma.dev)**

**A Rust image pipeline and its codec ports, developed in one repository.**

Ferrisma combines an image processing engine, CPU kernels, and Rust ports of
JPEG, WebP, JPEG XL, and AVIF libraries. The goal is to make color, decoder
capabilities, intermediate buffers, and execution costs explicit throughout
the pipeline.

**Experimental.** The JPEG/WebP/JPEG XL/AVIF → crop/resize → AVIF/WebP
pipeline, Rust facade, and CLI work for supported SDR inputs. M1 quality,
hardening, dependency, and release gates remain open; there is no stable API
or production-readiness claim.

The published npm package and crate at version `0.0.0` are placeholders without
image processing functionality. Build the engine from this repository; see
[ADR-0038](docs/adr/0038-ferrisma-project-name.md) for the naming transition.

## What's here

| Component | Current role |
| --- | --- |
| [Engine](src/pipeline.rs) | Experimental SDR pipeline with orientation, linear-light resampling, parallel execution, and reported materializations |
| [CPU kernels](crates/ferrisma-kernels) | Scalar, NEON, AVX2, and AVX-512 resampling behind a safe API |
| [JPEG port](codecs/libjpeg-turbo-rs) | In-scope 8-bit decoder with a safe row API, used by `jpeg-read` |
| [WebP port](codecs/libwebp-rs) | Decoder, encoder, container, and SharpYUV APIs; still-image input and output are connected through `webp-read` and `webp-write` |
| [JPEG XL port](codecs/libjxl-rs) | Reference decoder is the default engine JPEG XL input (`jpeg-xl-own-read`) through a safe wrapper; lossy (d1.0 default) and lossless encoder profiles behind `jpeg-xl-write` (in `cli`); full encoder incomplete |
| [AVIF ports](codecs/libavif-rs) | libavif and its AOM, dav1d, and SharpYUV ports; `avif-read` uses the AOM reference decoder through an [isolated safe adapter](crates/ferrisma-avif-read), with a separate [optional encoder adapter](crates/ferrisma-libavif) |
| [Shared SharpYUV core](crates/libsharpyuv-core) | One safe numeric conversion used by WebP and AVIF through their separate API and allocation boundaries |
| [Validation tooling](docs/codecs.md) | One Rust runner, scoped PR checks, and deeper independent oracle suites |

The four codec ports share the Cargo workspace, but importing them does not
automatically select them as engine backends. Ferrisma reads JPEG and WebP
through the workspace `libjpeg-turbo-rs` and `libwebp-rs` ports, JPEG XL through
the workspace libjxl-rs reference decoder by default (with `jxl-oxide` selectable
explicitly), and AVIF through the workspace libavif/AOM reader; it writes AVIF
through the workspace libavif/AOM encoder by default (with `rav1e` selectable
explicitly; their quality scales differ), and still WebP through the workspace
`libwebp-rs` encoder ([ADR-0032](docs/adr/0032-jpeg-xl-workspace-decoder-input.md),
[ADR-0037](docs/adr/0037-selectable-workspace-avif-encoder.md)). These two
defaults changed on 2026-11-12 and are pending owner confirmation.
The own AVIF read/write features need the pinned nightly and a supported native
target; features without `avif-read`, `avif-own-write`, or `cli` retain stable Rust 1.98 support.
On one 2268×1512 photo the AOM input path took 8–14% longer and used about
2× peak RSS versus the former interim reader; see the
[scoped measurement](docs/benchmarks/M2-avif-input.md).

**[Current status and remaining gates](docs/status.md)** gives the component
and integration overview. **[Supported matrix](docs/supported-matrix.md)** lists
every accepted and rejected input, output option, backend, Cargo feature and
toolchain, with the test behind each claim. **[Documentation guide](docs/README.md)** maps the
architecture, development instructions, decisions, and recorded experiments.

## Try the experimental CLI

From a checkout with Rust installed (NASM is also needed on x86-64):

```sh
cargo run -p ferrisma --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8

cargo run -p ferrisma --release --features cli -- \
  photo.jpg scaled.webp 1600 900 --assume-srgb --quality=80 --speed=8
```

Use `--assume-srgb` only when that input assumption is appropriate; supported
RGB ICC input can use `--use-rendered-icc`. Unknown or contradictory color,
HDR, and unsupported alpha cases fail explicitly. Crops use upright image
coordinates. Codec-internal buffers and opaque memory remain relevant
even when the transform streams rows.

For the experimental Rust API, the [thumbnail example](examples/thumbnail.rs)
uses centered cover and avoids enlargement. `pipeline::transcode_resized` also
supports inside, fill, and a single aspect-preserving dimension; it reports
the actual output size. `pipeline::inspect` reads bounded JPEG/WebP headers
without requesting pixels. A header alone does not establish that a complete
image can be decoded or that its color is supported.

For features, build prerequisites, and scoped checks, see
[getting started](docs/getting-started.md), [contributing](CONTRIBUTING.md),
and [codec development](docs/codecs.md). The [Ardo website](homepage/README.md)
presents these guides with the shared Ferramenta theme and can be previewed locally.

## License

The engine is **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**. Codec
packages retain their own licenses, notices, and publication gates. The
[dependency audit](docs/license-audit.md) records the former interim AVIF parser's
MPL-2.0 review separately from the current dependency graph; the root license
does not relicense the workspace.
