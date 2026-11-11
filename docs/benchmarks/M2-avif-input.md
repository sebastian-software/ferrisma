# AVIF input experiment: pinned C parity and measured interim comparison

**Date:** 2026-11-11. **Host:** macOS 27.0.1, Apple M1 Ultra, 64 GB RAM, ARM64, Apple
clang 21.0.0, `rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)` from the pinned
`nightly-2026-09-07` toolchain. These observations apply to this host, build,
and corpus. See [ADR-0031](../adr/0031-workspace-libavif-input-experiment.md).

## Independent reference and output contract

The original C libavif source and pinned portable C libaom 3.15.2 were built
from this repository's provenance-checked snapshots. The AOM reference has
166 original C translation units, `CONFIG_TUNE_VMAF=0`, and
`-ffp-contract=off`. The C library was built with the same FP contraction
setting. The supplemental [C consumer](../../codecs/libavif-rs/tests/ports/avif_input_rgb.c)
and Rust `ferrixel-avif-read` wrapper both select AOM and the primary image
item explicitly, decode image zero, request nearest-neighbor chroma upsampling,
straight interleaved RGB(A),
and 8-bit or 16-bit emitted samples according to source depth. They keep AVIF
orientation unapplied. The Rust engine then normalizes 10/12-bit U16 samples
by `65535` for F32 rows; its focused test verifies this exact calculation.

The C archive SHA-256 values were `b4019ed3f6dc118d61c4e46042984a52c4e4f088984c1fc9921b71c87f8d0cc0`
for `libaom.a` and `430e4327eaab1dabad3e00b1833c7122b983592dc55996f160b680ecf9879e39`
for `libavif_internal.a`. No translated `read.rs` changed. All 16 native
profile `read.rs` hashes still matched their translation manifests, and
`python3 tools/check_translation.py` passed.

| Input | Source geometry and contract | C/Rust raw result |
| --- | --- | --- |
| Flower photo, temporary AVIF | 2268×1512, 8-bit RGB | Exact, SHA-256 `3d069f3c6d673180bd2db730db934841a4b6cf888b7494fc96cb7bcbbce6d63b` |
| Pinned Kodak photo | 768×512, 8-bit RGB | Exact, `87435c26f9b6604eb3c77e0a93b504f32e4673bc651cf19e038650e24ee02b03` |
| Pinned fox 10-bit | 1204×800, 16-bit RGB output | Exact, `d05e2a4c1704fb16554adcd9ac2d93ce743838c87f1cc5513170fb3171c7c885` |
| Pinned fox 12-bit | 1204×800, 16-bit RGB output | Exact, `07ddf6e578be0854464c48e991e77b64883371f28966bc75a4d24441fd6aa69c` |
| Pinned sofa grid | 1024×770, 8-bit RGB | Exact, `dac5f75d9d5a530b989b75898bdff5558ef048a575e7a8a6d349d7a73c5d59b1` |
| Pinned alpha and orientation | 512×256, 8-bit RGBA | Exact, `5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82` |
| Pinned alpha grid | 80×80, 8-bit RGBA | Exact, `816eeaa9e291fc6b7fa5bb2735e165de7de66bb5512cb15ab8f714b72ad2c77d` |
| Pinned alpha and gain map | 128×200, 16-bit RGBA | Exact, `5c86e460c3b35c3ec7c88a2ac863164b4b5a67fb8da85b04418fa8e68a40e262` |

An upstream malformed `alpha_noispe.avif` was rejected by the C parser and
excluded from the matching set. The result proves the bridge's selected sample
conversion for these eight cases. It is not a full AV1 conformance or Argon
corpus result.

Rebuild the C oracle from `codecs/libavif-rs`:

```sh
python3 tools/build_aom.py --jobs 4 --library-only
cmake -S upstream/libavif -B build/avif-input-c-oracle -G Ninja \
  -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_FLAGS=-ffp-contract=off \
  -DBUILD_SHARED_LIBS=OFF -DAVIF_BUILD_TESTS=OFF -DAVIF_BUILD_APPS=OFF \
  -DAVIF_CODEC_AOM=SYSTEM -DAVIF_CODEC_DAV1D=SYSTEM \
  -DAVIF_LIBSHARPYUV=SYSTEM -DAVIF_LIBXML2=SYSTEM -DAVIF_LIBYUV=OFF \
  -DLIBAVIF_RS_AOM_REFERENCE_BUILD="$PWD/build/aom-native-release" \
  -DCMAKE_PROJECT_libavif_INCLUDE="$PWD/tools/local-dependencies.cmake"
cmake --build build/avif-input-c-oracle --target avif -- -j 4
cc -O2 -I upstream/libavif/include tests/ports/avif_input_rgb.c \
  build/avif-input-c-oracle/libavif_internal.a build/aom-native-release/libaom.a \
  $(pkg-config --libs libsharpyuv dav1d) -lm -lz -lc++ -o build/avif-input-c-rgb
build/avif-input-c-rgb path/to/input.avif /tmp/c.raw
```

The Rust comparison used the same wrapper's `Decoder::parse`, `decode_u8`, and
`decode_u16` methods to write native-endian raw samples, then `cmp` and
`shasum -a 256`. The 8/10/12-bit and alpha checks are also independently
covered by the adapter's scoped tests. The original port's ten C/Rust
AVIF/PNG differential scenarios remain part of its prior evidence; they were
not rerun as a full port suite here.

## Measured engine comparison

The input was generated from the repository's real flower JPEG
(`6ad9a79e9f26d2723ef954a0dca774429606655ab7184f8ab400016f228ca2ce`)
with ImageMagick 7.1.2-35 Q16-HDRI, HEIC delegate 1.23.7:

```sh
magick codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg \
  -strip -quality 80 /tmp/ferrixel-avif-flower-2268x1512-q80.avif
```

The generated AVIF SHA-256 was
`0222211a052d24f9e97f5f0666df3259749c7500847bd135b1d5adeb95e027de`.
The [frozen pre-format-neutral harness](../../scripts/bench/avif-input-f0ff.rs)
was compiled in release mode as a path-dependent binary against reviewed base
`f0ffebf4` (interim `avif-decode`/rav1d) and this experimental AOM adapter
separately, with
`avif-read,avif-write`. Its only output work is a rolling digest over sRGB8
rows. The pipeline used one transform thread, Lanczos3, and
`AssumeSrgbIfMissing`. The full run produced 2268×1512 pixels. The crop run
selected `(400,300,640,427)` and produced 320×213 pixels. Five warmed runs
of each binary were measured serially in a quiet CPU window. Values are
medians, in milliseconds except where shown.

| Workload | Interim total / decode | AOM total / decode | AOM relative total | Managed peak interim / AOM | Process peak RSS interim / AOM |
| --- | ---: | ---: | ---: | ---: | ---: |
| Full photo | 120.008 / 82.455 | 139.039 / 102.175 | 1.16× | 124,740 / 10,405,584 bytes | 35,717,120 / 65,290,240 bytes |
| Crop from full decode | 85.387 / 83.145 | 106.159 / 103.880 | 1.24× | 85,120 / 10,370,848 bytes | 35,651,584 / 65,437,696 bytes |

Both paths requested one full-frame decode and avoided zero codec work for the
crop. The managed-memory increase makes the complete RGB frame visible to the
engine budget; libavif/libaom scratch remains opaque. RSS came from one
additional `/usr/bin/time -l` process per workload and can vary with allocator
and process state. The measured AOM path was slower and used more process
memory on this photo. This is a workload observation, not a general decoder
ranking.

The two engine output digests were stable across runs but differed. Dumping
full RGBA output and comparing with ImageMagick gave a maximum difference of
one 8-bit channel code and mean absolute difference about 0.06 code on this
photo. That is separate from the exact raw C parity above and is not silently
treated as equivalent output. The interim decoder comparison path remains
available at base `f0ffebf4` for subsequent quality and performance review.

The recorded run used separate small Cargo binaries whose source was the
frozen harness above. For the `f0ffebf4` baseline, use that file as
`src/main.rs` in a temporary binary crate with a path dependency on the base
checkout and `features = ["avif-read", "avif-write"]`. Use the same source against
this AVIF branch before the format-neutral output change. After that change,
use the [current example](../../examples/avif_input_compare.rs), which must
adapt `EncoderBackend::format`, `EncodeRequest`, `EncodedImage::new`, and
`Output::bytes`; the frozen file preserves the exact old API harness. Invoke
the binaries with:

```sh
old-or-new-binary /tmp/ferrixel-avif-flower-2268x1512-q80.avif 2268 1512
old-or-new-binary /tmp/ferrixel-avif-flower-2268x1512-q80.avif 320 213 400 300 640 427
```

The final raw timing logs were kept under
`/private/tmp/avif-final-{old,new}-{full,crop}.log`; byte-parity samples under
`/private/tmp/avif-{c,rust}-*.raw`. Build time and AVIF input generation are
excluded from the per-run timings.
