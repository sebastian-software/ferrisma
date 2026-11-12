# AVIF default encoder: non-timing evidence, 2026-11-12

**Scope:** issue #53 without wall-clock time. This record compares output
semantics, sizes at matched SSIMULACRA2, independent decodes, known
materializations, binary sizes and platform constraints of `rav1e` (the
current default) and the workspace libavif/AOM writer (`avif-own-write`). It
records **no timing**: the host was shared with other heavy jobs, so its
wall-clock numbers are not valid evidence. It does not change the default.

**Host and build:** macOS 27.0.1, Apple M1 Ultra, ARM64; `rustc 1.100.0-nightly
(5a2be9f5f 2026-09-06)` from the pinned toolchain; release profile; clean
tracked tree at `54da920e` (empty tracked diff, see
[match provenance](evidence-2026-11-12-avif/match-provenance.json)).
Independent tools: FFmpeg 9.0.2 with libdav1d, libheif 1.23.7 (C libaom
3.15.3 decoder plugin), ImageMagick 7.1.2-35, and the standalone
`ferrisma-quality-match` scorer (SSIMULACRA2 0.5.1, decoding AVIF with
`avif-decode` 3.0.0/rav1d, sharing no code with either writer).

## Output semantics

Both writers receive the same straight-alpha sRGB8 RGBA rows after
Ferrisma's decode, color, crop and resample. Observed with `ffprobe`,
`heif-info` and the source code
([`src/avif_sink.rs`](../../src/avif_sink.rs),
[`crates/ferrisma-avif-write`](../../crates/ferrisma-avif-write/src/lib.rs));
outputs and fields are in [encoder-semantics.json](evidence-2026-11-12-avif/encoder-semantics.json).

| Property | rav1e (default) | libavif/AOM (`--avif-encoder=libavif`) |
| --- | --- | --- |
| Bit depth | always 10 | 8 or 10, default 10 |
| Chroma | 4:4:4 or 4:2:0; 2×2 average of per-pixel BT.601 Cb/Cr in `f32` | 4:4:4 or 4:2:0; libavif `AVERAGE` (its `AUTOMATIC`) |
| AV1 sequence-header color | 1/13/6, full range | 1/13/6, full range |
| Container `colr nclx` | **absent**: `avif-serialize` omits the box for its default values; readers must use the AV1 header (libheif reports "color profile: no") | present, 1/13/6/full |
| Alpha | separate monochrome 10-bit item at a **fixed** quality 80; omitted when every alpha is 255 | auxiliary item at the **same** quality as color, in the selected depth; omitted when opaque (`avifImageIsOpaque`) |
| Quality scale | 1–100 through `ravif`'s curve onto AV1 quantizer 0–255 | 0–100 through `((100 - q) * 63 + 50) / 100`: **64 distinct quantizer levels** |
| Speed scale | 1–10 with `ravif` speed overrides | 0–10, libavif/AOM `cpu-used` |
| Tiles and threads | size-derived tiles when parallel; one tile for one thread | `autoTiling` unless one thread; `maxThreads` from the request |

The two quality scales are **not interchangeable**: at `--quality=80`
on the 800×533 flower, rav1e wrote 55,682 bytes (SSIMULACRA2 77.455) and
AOM 10-bit 100,512 bytes (88.307). A default switch therefore changes what an
existing `quality` value means unless a mapping is introduced. Matched pairs
below show the relation: rav1e 70/80/90 corresponds to AOM about 49–53,
54–58 and 77–81 on these images.

Independent decodes of every semantics output (FFmpeg/libdav1d, libheif/C
libaom, rav1d through the scorer) succeeded with the requested size, depth
(`yuv444p10le`, `yuv420p`, `gray10le` and so on), chroma and CICP. On the
256×512 alpha fixture, the decoded alpha differed from the source alpha by at
most 32 codes for rav1e (fixed alpha quality 80) and 13 (8-bit) or 9 (10-bit)
for AOM at quality 80.

## Sizes at matched quality

The existing [encoder-comparison procedure](encoder-adoption.md) ran its
`prepare`, `match` and `check` phases (no `time`) for dog, flower, kodim03 and
kodim23 at rav1e anchors 70, 80 and 90, with one thread and all 20 cores, AOM
speeds 6 and 7, 10-bit 4:4:4. Raw records: [matched.json](evidence-2026-11-12-avif/matched.json).
"Estimated" bytes interpolate between the bracketing integer qualities;
they are not sizes of real equal-score files.

| Image | AOM s6 vs rav1e s6, estimated bytes at equal score | AOM s7 vs rav1e s6 |
| --- | ---: | ---: |
| dog 1600×2133 | −12.4 % to −7.6 % | −9.0 % to −2.5 % |
| flower 1600×1067 | −13.9 % to −4.0 % | −7.0 % to −4.1 % |
| kodim03 512×341 | −14.4 % to −6.1 % | −8.9 % to −4.5 % |
| kodim23 512×341 | −8.4 % to +1.7 % | −0.2 % to +2.4 % |

AOM speed 6 was estimated smaller in 23 of 24 cells, speed 7 in 20 of 24.
`check` passed 42 of 48 cells and rejected six speed-6 cells whose nearest
integer quality missed its target by 0.536–0.805 SSIMULACRA2 (dog q70 t1,
flower q80 t1, kodim03 q70 t0 and t1, kodim23 q70 t0, kodim23 q80 t1). The
gate was not relaxed.

### Why calibration cells are rejected

The hosted native calibration
[run 43118009499](https://github.com/sebastian-software/ferrisma/actions/runs/43118009499)
rejected most Kodak cells on both hosts and the dog q60 diagnostic
(−1.585). Every logged rejection is an integer-quality gap: libavif maps
quality 0–100 onto 64 quantizers, so neighboring integer qualities often
encode identically, and one quantizer step moves SSIMULACRA2 by about one
point or more on small images. Each host logged 14 or 15 rejected
candidates; apart from the dog q60 diagnostic, all missed by 0.506–0.823,
so a neighboring anchor or thread setting often passes. This is
a granularity property of libavif's `quality` knob, not an encoder defect.
A default decision should therefore not hinge on single integer-quality
cells: compare byte–score curves over the quantizer range (for example a
BD-rate over all 64 levels) or accept the nearest cell with its recorded gap.

### Cross-host bitstream identity

rav1e produced the same bytes here as on the x86-64
[smoke run 43075697017](https://github.com/sebastian-software/ferrisma/actions/runs/43075697017)
for the dog (261,883 bytes at all cores; 252,291 at one thread), so the
encoder input rows matched. AOM speed 6/quality 55 at several threads also
matched (228,547), but speed 6/quality 55 with **one** thread wrote 227,178
bytes here versus 227,152 on x86-64, and speed 7/quality 56 wrote 252,575
versus 252,582. On this host, `AOM_SIMD_CAPS_MASK=0` reproduced the Neon bytes
for all four settings, and 4 and 20 threads gave the same bytes. The cause of
the cross-host difference is not established; the AOM writer revision did not
change between the two revisions except for the project rename. The C/Rust
golden corpus (62 cases) is byte-identical across both platforms, but it does
not cover these settings. Unlike rav1e, AOM output is not demonstrated to be
platform-independent.

## Memory and materializations

Both encoders materialize one complete encoder input frame before encoding;
neither streams. Reported `encoder_frame_bytes` for 800×533 (426,400 pixels):

| Layout | rav1e | AOM 8-bit | AOM 10-bit |
| --- | ---: | ---: | ---: |
| 4:4:4 | 4,237,824 | 2,984,800 | 4,264,000 |
| 4:2:0 | 2,141,696 | 2,345,600 | 2,985,600 |

rav1e converts each row into its own padded 10-bit frame and keeps no RGBA
copy; its alpha bytes and alpha frame are added when the output is
transparent. AOM keeps the full RGBA8 frame plus libavif's YUV(A) planes alive
together (10 bytes per pixel for 10-bit 4:4:4 opaque) and copies libavif's
encoded buffer into an owned `Vec`, so the compressed output briefly exists
twice. Both report their encoder scratch and compressed output as opaque to the
engine budget. A direct YUV-to-AOM path without the RGBA frame remains #60.

## Binary size

Minimal JPEG→AVIF binaries (one `transcode` call, release, `panic = "unwind"`)
built from `54da920e` against the root lockfile:

| Features | Bytes | `strip -x` |
| --- | ---: | ---: |
| `jpeg-read` only (no AVIF) | 1,323,936 | 1,143,864 |
| `jpeg-read`, `avif-write` (rav1e) | 2,970,624 | 2,557,624 |
| `jpeg-read`, `avif-own-write` (AOM) | 8,313,104 | 7,808,728 |
| `jpeg-read`, `avif-read`, `avif-write` | 9,990,624 | 9,254,672 |
| `jpeg-read`, `avif-read`, `avif-own-write` | 8,457,168 | 7,945,504 |
| `jpeg-read`, `avif-read`, `avif-write`, `avif-own-write` | 10,119,040 | 9,376,032 |

The own AVIF reader already links the workspace libavif/AOM library. With it,
the AOM writer adds 144,064 bytes, whereas rav1e adds 1,533,456. A v1.0 build
with the own reader and only the AOM writer is about 1.5 MB smaller than the
same build with rav1e. Without AVIF input, rav1e is about 5.3 MB smaller.

## Platform and safety constraints

The AOM writer is generated C2Rust code behind the safe bridge in
`ferrisma-avif-write`: it needs the pinned nightly (`extern_types`) and
builds only for macOS ARM64 and GNU Linux x86-64. rav1e builds on stable
Rust (with NASM on x86-64) on all of rav1e's targets. AVIF input already
carries the same nightly/native constraint, so a v1.0 build that uses the own
reader gains no new platform restriction from the own writer. Builds without
AVIF features remain stable-compatible either way.

## Recommendation

Switch the default AVIF encoder to the workspace libavif/AOM writer for
v1.0, **subject to the pending timing evidence**, and keep `rav1e` only as an
explicitly selected, non-default backend while stable-toolchain or
other-target users need AVIF output:

- It is the reference encoder (ADR-0025 point 1) and the v1.0 scope runs
  every format on the workspace ports.
- At matched SSIMULACRA2 it was smaller in 23 of 24 speed-6 cells on four
  images and three quality levels on this host, consistent with the earlier
  x86-64 results.
- With the own reader present it shrinks the binary by about 1.5 MB.
- Its color signaling is more explicit (container `colr` plus sequence
  header) and its alpha follows the color quality.

Before the switch, the owner must decide the following:

1. **Quality mapping.** Either map Ferrisma's existing `quality` to libavif's
   scale (an observed rav1e→AOM relation is 70→~50, 80→~55, 90→~78 here) or
   document the scale change as a breaking change of the experimental API.
2. **Alpha quality.** rav1e uses a fixed alpha quality of 80; AOM uses the
   color quality. Keep AOM's behavior or set `qualityAlpha` explicitly.
3. **Determinism.** Investigate the AOM cross-host byte difference above if
   platform-independent output is required.

Timing evidence that is still required, on quiet native ARM64 and x86-64
hosts: wall time with one thread and all cores for rav1e s6 and AOM s6/s7
at the selected cells, peak RSS from the same processes, and at least one
12–48 MP photograph. The existing `compare-encoders.sh time` phase and the
native-host workflow produce this once a calibration completes.

## Reproduce

```sh
export BENCH_SHARP_NODE_MODULES=/path/to/node_modules COMPARE_ENCODER_LEVELS=70,80,90
python3 -I scripts/bench/compare-sharp.py prepare OUT
scripts/bench/compare-encoders.sh OUT match
scripts/bench/compare-encoders.sh OUT check   # exit 3: six cells rejected
cargo build --release --features cli --bin ferrisma
python3 -I scripts/bench/avif-encoder-semantics.py SEMANTICS_OUT
```
