# ferrixel-libavif

[libavif-rs](https://github.com/sebastian-software/libavif-rs), the mechanical Rust port of libavif with its Rust AOM encoder, as an output encoder for Ferrixel ([ADR-0024](../../docs/adr/0024-pluggable-encoders-and-libavif-rs-backend.md)).

```rust
use ferrixel::pipeline::{ImageFormat, TransformOptions, transcode_with};
use ferrixel_libavif::LibavifBackend;

let output = transcode_with(&jpeg, ImageFormat::Jpeg, options, &LibavifBackend::default())?;
```

Ferrixel decodes, converts color, crops, resamples and quantizes as usual; this crate receives the finished sRGB8 rows, converts them with libavif's `avifImageRGBToYUV` and encodes with `avifEncoderWrite`. The output signals match Ferrixel's built-in `rav1e` sink: CICP 1/13/6, full range, 10 bits by default (`LibavifBackend { depth: 8 }` for 8 bits). `quality` (0–100) and `speed` (0–10) are libavif's own scales; compare backends at matched quality.

The `ferrixel-libavif` command (feature `cli`) is Ferrixel's command line with this backend: same arguments, same `report:` line, with `encoder=libavif-rs/aom`.

```sh
cargo run --release --features cli -- input.jpg output.avif 1600 1067 --assume-srgb --quality=56 --speed=6 --encoder-threads=0
```

## Requirements and limits

- **Toolchain:** the nightly that libavif-rs pins (`rust-toolchain.toml` here). This crate is not a member of the Ferrixel workspace, so the core stays on stable.
- **Platforms:** aarch64 macOS and x86-64 GNU Linux, the targets libavif-rs validates.
- **No native libraries.** libavif-rs builds its AOM without VMAF tuning ([libavif-rs ADR 0057](https://github.com/sebastian-software/libavif-rs/blob/main/docs/adr/0057-translate-libaom-without-vmaf-tuning.md)), so libvmaf, pkg-config and a C++ runtime are no longer needed. AOM rejects VMAF tuning modes, which Ferrixel never selects.
- **No AVIF input in the same binary.** Ferrixel's `avif-read` decodes with rav1d, which exports the same `dav1d_*` C symbols as libavif-rs's dav1d port; linking both is undefined behavior. The `cli` feature therefore enables JPEG and JPEG XL input only, and the tests decode with libavif-rs's own decoder.
- **Memory:** the backend materializes one RGBA frame and libavif's YUV(A) planes, reported as `encoder_output_frame_bytes`; AOM's own memory is opaque (`opaque_encoder_memory`).
- **`unsafe`:** all calls into libavif's C ABI live in `src/lib.rs`, each with its safety argument; every libavif object is owned by a guard that frees it.

## Licenses

libavif-rs is BSD-2-Clause; its AOM port is `BSD-2-Clause AND BSD-3-Clause AND MIT` and keeps libaom's patent grant (`crates/aom/LICENSES/PATENTS`); its dav1d port is BSD-2-Clause and its SharpYUV port BSD-3-Clause. libvmaf is BSD-2-Clause-Patent. This crate is not covered by Ferrixel's `scripts/audit-licenses.py`, which screens the workspace lockfile.
