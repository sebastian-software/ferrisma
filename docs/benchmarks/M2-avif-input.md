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
and Rust `ferrisma-avif-read` wrapper both select AOM and the primary image
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

## Measured engine comparison (repeated against committed revisions)

The input was generated from the repository's real flower JPEG
(`6ad9a79e9f26d2723ef954a0dca774429606655ab7184f8ab400016f228ca2ce`)
with ImageMagick 7.1.2-35 Q16-HDRI, HEIC delegate 1.23.7:

```sh
magick codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg \
  -strip -quality 80 /tmp/ferrisma-avif-flower-2268x1512-q80.avif
```

The generated AVIF SHA-256 was
`0222211a052d24f9e97f5f0666df3259749c7500847bd135b1d5adeb95e027de`.
The same committed [frozen pre-format-neutral harness](../../scripts/bench/avif-input-f0ff.rs)
(SHA-256 `7da0d7e3b69eab9f69056b4c7daff0be2b330baddef9f441d655fb781038730d`)
was compiled against clean reviewed base
`f0ffebf4fb759457a5a03726265931182e2882e8` (interim
`avif-decode`/rav1d) and clean final AVIF adapter
`63905312e2df72030351a3e9d56067052b20f0bd` (AOM). These are the
source revisions used for **this repeated measurement**; the earlier
uncommitted-candidate timing numbers are superseded below. Both temporary
Cargo packages used `features = ["avif-read", "avif-write"]`, release mode,
the pinned `nightly-2026-09-07` toolchain, and separate lockfiles. Their
lockfile SHA-256 values were `b1d8fa2c848001810620dbf0a0732c30636c8910cf0e79095afa84efcaa0640b`
(base) and `2a394b20e7f41786790ca2b5dcde866d6d7643dbfc9c4f054ae6cf4d23dd42c3`
(AOM). The compiled binary SHA-256 values were
`723b8023cc580b81eb59813a816c9f0f6f076092c61cedd8906f954872c781d7`
(base) and `b251acece34802adde5df0a67338da11db723badfaa5ad40c73fe5dc96d7b753`
(AOM).

The harness's only output work is a rolling digest over sRGB8 rows. The
pipeline used one transform thread, Lanczos3, and `AssumeSrgbIfMissing`.
The full run produced 2268×1512 pixels. The crop selected
`(400,300,640,427)` and produced 320×213 pixels. Each candidate/workload
ran in five fresh processes, alternated in an old/new and full/crop schedule.
Each process ran the harness's five timed iterations. The time below is the
median of the five per-process medians (milliseconds); peak process RSS is
the median of five `/usr/bin/time -l` observations (bytes). Build time and
AVIF generation are excluded. The raw order and 20 process logs are saved in
`/private/tmp/avif-remeasure-order.txt` and
`/private/tmp/avif-remeasure-{old,new}-{full,crop}-{1..5}.log`.

| Workload | Interim total / decode | AOM total / decode | AOM relative total | Managed peak interim / AOM | Process peak RSS interim / AOM |
| --- | ---: | ---: | ---: | ---: | ---: |
| Full photo | 139.417 / 98.514 | 150.441 / 112.063 | 1.08× | 124,740 / 10,405,584 bytes | 36,831,232 / 73,973,760 bytes (2.01×) |
| Crop from full decode | 100.991 / 98.638 | 115.019 / 112.673 | 1.14× | 85,120 / 10,370,848 bytes | 37,060,608 / 73,990,144 bytes (2.00×) |

The base process medians varied from 122.326–166.158 ms for full and
88.613–117.459 ms for crop, so these ratios describe this run, not a stable
cross-workload ranking. AOM was about 8% slower for full and 14% slower for
crop, with about twice the process RSS on this host and fixture. Both paths
requested a full-frame decode, including for the crop.
The managed-memory increase makes the complete RGB frame visible to the
engine budget; libavif/libaom scratch remains opaque. RSS can vary with
allocator and process state. This is a workload observation, not a general
decoder ranking.

The two engine output digests were stable within each candidate but differed
between candidates. This timing run did not rerun the earlier RGBA-pixel
difference check; the exact raw C/Rust parity for eight fixtures above is a
separate result. The interim decoder remains reproducible at `f0ffebf4`.

For the base, use the frozen harness as `src/main.rs` in a temporary binary
crate with a path dependency on that base checkout and
`features = ["avif-read", "avif-write"]`. Use the same source against
`63905312` (retained on `codex/avif-input-comparison-20261111`) for the AOM comparison. In the integrated tree after the
format-neutral output change, use the
[current example](../../examples/avif_input_compare.rs), which is adapted to
`EncoderBackend::format`, `EncodeRequest`, `EncodedImage::new`, and
`Output::bytes`; the frozen file preserves the exact old API harness. Invoke
each binary with:

```sh
old-or-new-binary /tmp/ferrisma-avif-flower-2268x1512-q80.avif 2268 1512
old-or-new-binary /tmp/ferrisma-avif-flower-2268x1512-q80.avif 320 213 400 300 640 427
```

## Independent reader cross-check (2026-11-12)

**Host:** macOS 27.0.1, Apple M1 Ultra, ARM64, pinned `nightly-2026-09-07`,
release build at `54da920e`. This is a correctness record for issue #54; it
has no timing.

[`scripts/avif_reference_crosscheck.py`](../../scripts/avif_reference_crosscheck.py)
runs [`examples/avif_reference_crosscheck.rs`](../../examples/avif_reference_crosscheck.rs)
on 43 fixtures (libavif test data, its Android 8/10/12-bit 4:2:0/4:2:2/4:4:4
and monochrome assets, and Ferrisma's eight orientation fixtures). For each
file it compares the safe wrapper's parsed metadata and unoriented samples
and the engine's upright sRGB8 rows with tools that share neither the
container parser nor the YUV-to-RGB conversion:

- **libheif 1.23.7** (`heif-info`, `heif-dec -C nn`): its own HEIF/ISOBMFF
  parser, its own application of `irot`/`imir`/`clap`, its own
  nearest-neighbor YUV-to-RGB, and the C libaom 3.15.3 decoder plugin.
- **ImageMagick 7.1.2-35** for the ICC bytes (through libheif).
- **FFmpeg 9.0.2 with libdav1d** (`ffprobe` color fields and, for opaque
  untransformed items, an advisory dav1d decode).

All 43 fixtures agree under tolerances fixed in the script; the report is
[input-crosscheck.json](evidence-2026-11-12-avif/input-crosscheck.json):

| Check | Result |
| --- | --- |
| Depth, alpha presence against libheif; container CICP where libheif reports `nclx` | equal for every file the wrapper parses |
| ICC bytes against ImageMagick/libheif | identical (596 and 3,144 bytes); both files are rejected by the engine without `UseRenderedIcc`/`icc` |
| Upright size (`irot`/`imir`, eight orientations, grids) against libheif | equal |
| Engine sRGB8 rows against libheif, visible pixels | max ≤ 1 code, mean ≤ 0.26 (8/10/12-bit, 4:2:0/4:2:2/4:4:4, grids, alpha) |
| Wrapper samples at source depth against libheif | 8-bit max 1 code; 10/12-bit max 1.44 codes (libavif emits 16-bit RGB directly; libheif rounds to the source depth first) |
| Alpha against libheif | equal, except two documented cases below |
| `clap` (2 files), monochrome, animation | rejected (`UnsupportedInput`) before pixel decode; libheif applies a crop to both `clap` files, and libavif's own parser also rejects `clap_irot_imir_non_essential.avif` |
| HDR PQ, BT.470M transfer | rejected (`UnsupportedColor`); ffprobe reports the same CICP |
| Malformed (`alpha_noispe`, repeated `dimg`) | rejected (`MalformedInput`) |
| FFmpeg/libdav1d, opaque untransformed items (advisory) | 8-bit max 1.49 codes; 10-bit max 3.95 codes; 12-bit max 15.8 codes (about 1 code at 8 bits), from FFmpeg's scaler |

Two fixtures differ deliberately. `abc_color_irot_alpha_NOirot.avif` and
`clop_irot_imor.avif` associate `irot` with the color item but not with the
alpha item. libavif applies the color item's transforms to alpha for
compatibility with files written by older libavif versions (its
`AlphaIrotNoIrot` test), so Ferrisma rotates the alpha with the color.
libheif leaves that alpha unrotated. Colors match; alpha differs. MIAF requires
the same transformative properties on both items, so such files are
nonconforming; Ferrisma follows libavif instead of failing closed.

The upright pixel check uses visible pixels only: where straight alpha is 0,
the engine does not preserve color values (234 such pixels in
`color_grid_alpha_nogrid.avif` differ by up to 14 codes; the wrapper's raw
samples equal libheif's).

Also verified on this host:

- `cross_module_image_layouts_match` asserts equal size, alignment and field
  offsets of the `avifImage` and `avifRGBImage` copies in the generated
  `read`, `reformat` and `avif` modules that the wrapper casts between.
- A `compile_fail` doctest (E0597) shows that a `Decoder` cannot outlive its
  input slice; the raw pointer keeps the decoder `!Send` and `!Sync`.
- Scoped tests pass with `avif-read`, `avif-read` + `avif-write` +
  `avif-own-write`, and `cli` (167 library tests and the CLI integration tests).

Reproduce from the repository root (the tools must be on `PATH`):

```sh
cargo build --release --features avif-read --example avif_reference_crosscheck
python3 -I scripts/avif_reference_crosscheck.py --output target/avif-crosscheck.json
```
