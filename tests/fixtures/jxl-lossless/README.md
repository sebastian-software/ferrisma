# JPEG XL lossless references written by the original (issues #249, #223)

The byte reference of the engine's lossless JPEG XL sink (ADR-0039) is the
pinned original's public tool, not the port: each file here was written by
`cjxl` of libjxl `a16d32d0` (the submodule `codecs/libjxl-rs/upstream/libjxl`,
`cjxl v0.13.0 a16d32d0 [_SCALAR_]`) from the `rows()` test pattern of
`src/jxl_output_tests.rs`, and
`jxl_output_tests::lossless_output_equals_the_original_cjxl_bytes` requires
the sink to write the same bytes.

```sh
python3 -I rows.py 37 19 0 rgb-37x19.ppm
python3 -I rows.py 37 19 1 rgba-37x19.pam
python3 -I rows.py 300 260 0 rgb-300x260.ppm
for f in rgb-37x19.ppm rgba-37x19.pam rgb-300x260.ppm; do
  cjxl "$f" "${f%.*}.jxl" -d 0 -e 2 --buffering=0 --num_threads=0 \
    -x color_space=RGB_D65_SRG_Rel_SRG
done
```

`cjxl` was built with the configuration of
`scripts/bench/ci-perf/codec_jxl.sh` (`build-scalar`: Clang,
`-DHWY_COMPILE_ONLY_SCALAR=1`, CMake Release) on macOS AArch64. With the default
thread pool (`--num_threads` omitted) the same command wrote the same bytes
for the 300x260 pattern and a 4032x3024 one.

Why each flag:

- `-d 0 -e 2 --buffering=0`: lossless (SetLossless), effort 2 (Thunder), the
  one-shot path, the profile of `LosslessThunderConfig`.
- `-x color_space=RGB_D65_SRG_Rel_SRG`: enum sRGB with the **relative**
  rendering intent, what `JxlColorEncodingSetToSRGB` and libjxl's internal
  `ColorEncoding::SRGB()` signal. Without it, cjxl's PPM/PAM reader leaves the
  intent at zero (perceptual, `ApplyColorHints` in
  `lib/extras/dec/color_hints.cc`); the non-default intent makes the color
  encoding non-`all_default` and costs 2 bytes in the image header.
- No `--codestream_level=10`: an explicit level 10 makes cjxl write the
  ISOBMFF container (`JXL ` signature 12 bytes, `ftyp` 20, `jxll` 9, `jxlc`
  header 8: 49 bytes) around the same codestream. The engine writes a raw
  codestream. The level-10 encoder parameters are the same either way
  (`cparams.level` only bounds the Modular bit depth, 16 at level 5 against
  32, which 8-bit samples never reach).

Together these are the 51 bytes of issue #249.

| File | Input | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `rgb-37x19.jxl` | `rows.py 37 19 0` (PPM) | 1,891 | `e7097e61cbe690732f314662f330af7023874b57939e7d933359b245624df2b4` |
| `rgba-37x19.jxl` | `rows.py 37 19 1` (PAM, `RGB_ALPHA`) | 2,153 | `f771a37fde48d32bf4d44818bff7aedd4432a825a0164763cfb006b16ec6e6af` |
| `rgb-300x260.jxl` | `rows.py 300 260 0` (PPM, 2x2 groups) | 196,894 | `c9ee45da8e128b11ad15ccaf99aa1bd85975087e84c1ee6e53ee95a616e0e544` |

## Default (chunked) buffering, 2026-11-15 (issue #223)

Since 2026-11-15 the sink encodes through the port's public encoder with the
default buffering, as `cjxl` does without `--buffering`: above 8 groups the
original then encodes in chunks (EncodeFrameStreaming, one 2048x2048 DC group
at a time, with its own trees and histograms per group), which writes other
bytes than `--buffering=0`. The reference command is therefore `cjxl`'s
default:

```sh
python3 -I rows.py 2049 40 0 rgb-2049x40.ppm
python3 -I rows.py 2049 40 1 rgba-2049x40.pam
for f in rgb-37x19.ppm rgba-37x19.pam rgb-300x260.ppm rgb-2049x40.ppm rgba-2049x40.pam; do
  cjxl "$f" "${f%.*}.jxl" -d 0 -e 2 --num_threads=0 \
    -x color_space=RGB_D65_SRG_Rel_SRG
done
```

with `cjxl v0.13.0 a16d32d [_SCALAR_] {AppleClang 21.0.0.21000401}` (libjxl
`a16d32d0ac113f5e54125e49bb571281c5d0b704`, the same `build-scalar`
configuration, rebuilt for this change) on macOS AArch64. Results:

- The three files above are unchanged: they have at most 8 groups (37x19 one,
  300x260 four), where the original takes its one-shot path with every
  buffering value, so the default command writes the same bytes as
  `--buffering=0` (checked: identical SHA-256). They are kept as they are and
  are now the reference of both commands.
- `rgb-2049x40.jxl` and `rgba-2049x40.jxl` are new: 2049x40 has 9 groups in
  two DC groups, so the default command takes the chunked path. With
  `--buffering=0` the original writes other files (206,276 and 228,069 bytes);
  those one-shot files are not the engine's reference and are not kept.
- With the default thread pool (`--num_threads` omitted) the same command
  wrote the same bytes for all five inputs.

| File | Input | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `rgb-2049x40.jxl` | `rows.py 2049 40 0` (PPM, 9 groups, chunked) | 208,070 | `b9652d54090abeb2900148fa3fe411d387b26252f7f5656b0c80fbeabe41d64a` |
| `rgba-2049x40.jxl` | `rows.py 2049 40 1` (PAM, `RGB_ALPHA`, 9 groups, chunked) | 234,342 | `da0e26b893379754e3aa3f5a26bd36303fea234fa9db0e0d636eb894f77e8f69` |
