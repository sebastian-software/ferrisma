# Ferrixel

**A Rust image pipeline and its codec ports, developed in one repository.**

Ferrixel combines an image processing engine, CPU kernels, and Rust ports of
JPEG, WebP, JPEG XL, and AVIF libraries. The goal is to make color, decoder
capabilities, intermediate buffers, and execution costs explicit throughout
the pipeline.

**Experimental and unpublished.** The JPEG/WebP/JPEG XL/AVIF → crop/resize → AVIF/WebP
pipeline, Rust facade, and CLI work for supported SDR inputs. M1 quality,
hardening, dependency, and release gates remain open; there is no stable API
or production-readiness claim.

## What's here

| Component | Current role |
| --- | --- |
| [Engine](src/pipeline.rs) | Experimental SDR pipeline with orientation, linear-light resampling, parallel execution, and reported materializations |
| [CPU kernels](crates/ferrixel-kernels) | Scalar, NEON, AVX2, and AVX-512 resampling behind a safe API |
| [JPEG port](codecs/libjpeg-turbo-rs) | In-scope 8-bit decoder with a safe row API, used by `jpeg-read` |
| [WebP port](codecs/libwebp-rs) | Decoder, encoder, container, and SharpYUV APIs; still-image input and output are connected through `webp-read` and `webp-write` |
| [JPEG XL port](codecs/libjxl-rs) | Reference decoder available as experimental opt-in engine input through a safe wrapper; encoder incomplete |
| [AVIF ports](codecs/libavif-rs) | libavif and its AOM, dav1d, and SharpYUV ports; `avif-read` uses the AOM reference decoder through an [isolated safe adapter](crates/ferrixel-avif-read), with a separate [optional encoder adapter](crates/ferrixel-libavif) |
| [Shared SharpYUV core](crates/libsharpyuv-core) | One safe numeric conversion used by WebP and AVIF through their separate API and allocation boundaries |
| [Validation tooling](docs/codecs.md) | One Rust runner, scoped PR checks, and deeper independent oracle suites |

The four codec ports share the Cargo workspace, but importing them does not
automatically select them as engine backends. Ferrixel reads JPEG and WebP
through the workspace `libjpeg-turbo-rs` and `libwebp-rs` ports, JPEG XL through
`jxl-oxide` by default (with the local reference decoder selectable explicitly),
and AVIF through the workspace libavif/AOM reader; it writes AVIF through `rav1e`
and still WebP through the workspace `libwebp-rs` encoder.
The AVIF read feature needs the pinned nightly and a supported native target;
the codec-free core and features without `avif-read`/`cli` retain stable Rust 1.98 support.
On one 2268×1512 photo the AOM input path took 8–14% longer and used about
2× peak RSS versus the former interim reader; see the
[scoped measurement](docs/benchmarks/M2-avif-input.md).

**[Current status and remaining gates](docs/status.md)** gives the component
and integration overview. **[Documentation guide](docs/README.md)** maps the
architecture, development instructions, decisions, and recorded experiments.

## Try the experimental CLI

From a checkout with Rust installed (NASM is also needed on x86-64):

```sh
cargo run -p ferrixel --release --features cli -- \
  photo.jpg scaled.avif 1600 900 --assume-srgb --quality=80 --speed=8

cargo run -p ferrixel --release --features cli -- \
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
