# ferrisma-libavif

See the [Ferrisma README](../../README.md), [project status](../../docs/status.md),
and [codec guide](../../docs/codecs.md) for workspace context and current gates.

This compatibility adapter re-exports the engine's `LibavifBackend` and uses [libavif-rs](../../codecs/libavif-rs), the Rust port of libavif
with its AOM encoder, as an output encoder for Ferrisma
([ADR-0024](../../docs/adr/0024-pluggable-encoders-and-libavif-rs-backend.md)).

```rust
use ferrisma::pipeline::{ImageFormat, TransformOptions, transcode_with};
use ferrisma_libavif::LibavifBackend;

let output = transcode_with(&jpeg, ImageFormat::Jpeg, options, &LibavifBackend::default())?;
```

Ferrisma decodes, converts color, crops, resamples and quantizes as usual; this crate receives the finished sRGB8 rows, converts them with libavif's `avifImageRGBToYUV` and encodes with `avifEncoderWrite`. The output signals match Ferrisma's built-in `rav1e` sink: CICP 1/13/6, full range, 10 bits by default (`LibavifBackend { depth: 8 }` for 8 bits). `quality` (0–100) and `speed` (0–10) are libavif's own scales; compare backends at matched quality.

The `ferrisma-libavif` command (feature `cli`) is Ferrisma's command line with this backend: same arguments, same `report:` line, with `encoder=libavif-rs/aom`.

```sh
cargo run -p ferrisma-libavif --release --features cli -- input.jpg output.avif 1600 1067 --assume-srgb --quality=56 --speed=6 --encoder-threads=0
```

## Requirements and limits

- **Toolchain:** the root workspace pins the nightly required by libavif-rs. The adapter uses the local codec package and root lockfile; the engine retains explicit stable/MSRV checks (ADR-0026).
- **Platforms:** aarch64 macOS and x86-64 GNU Linux, the targets libavif-rs validates.
- **No native libraries.** libavif-rs builds its AOM without VMAF tuning ([libavif-rs ADR 0057](../../codecs/libavif-rs/docs/adr/0057-translate-libaom-without-vmaf-tuning.md)), so libvmaf, pkg-config and a C++ runtime are no longer needed. AOM rejects VMAF tuning modes, which Ferrisma never selects.
- **AVIF input in the same binary:** Ferrisma's `avif-read` now selects the
  workspace libavif/AOM decoder through `ferrisma-avif-read`. The former
  `rav1d`/dav1d duplicate-symbol path is gone. The adapter's `cli` feature
  includes JPEG, WebP, JPEG XL and AVIF input. Scoped native CI tests the
  combined build; local CLI smoke checks also cover AVIF and WebP input.
- **Memory:** the backend preflights the combined RGBA frame and tightly packed
  libavif YUV(A) planes, which coexist during conversion. Ferrisma reserves these
  known pixel bytes before calling `begin` and reports them as
  `encoder_output_frame_bytes`. Allocation overhead, conversion scratch, AOM
  memory, and compressed output remain outside that reservation
  (`opaque_encoder_memory`). `max_output_bytes` is checked after encoding and
  does not cap the codec's temporary output allocation.
- **`unsafe`:** calls into libavif's C ABI live in the independent `ferrisma-avif-write` bridge, each with its safety argument; guards release every libavif object.

## Licenses

libavif-rs is BSD-2-Clause; its AOM port is `BSD-2-Clause AND BSD-3-Clause AND MIT` and keeps libaom's patent grant (`crates/aom/LICENSES/PATENTS`); its dav1d port is BSD-2-Clause and its SharpYUV port BSD-3-Clause. This crate is not covered by Ferrisma's `scripts/audit-licenses.py`, which screens the engine's reachable dependency graph. Codec notice and provenance checks remain separate.
