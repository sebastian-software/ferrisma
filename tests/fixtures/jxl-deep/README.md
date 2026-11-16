# JPEG XL references above 8 bits, written by the original (2026-11-15)

The byte reference of the engine's JPEG XL sink at depths above 8 bits
(ADR-0021 amendment of 2026-11-15, owner decision: the output depth follows
the input) is the pinned original's public tool, as in
[`../jxl-lossless/`](../jxl-lossless/README.md): every `.jxl` file here was
written by `cjxl v0.13.0 a16d32d0 [_SCALAR_] {AppleClang 21.0.0.21000401}`
(libjxl `a16d32d0ac113f5e54125e49bb571281c5d0b704`, the `build-scalar`
configuration of `scripts/bench/ci-perf/codec_jxl.sh`: CMake Release,
`-DHWY_COMPILE_ONLY_SCALAR=1`) on macOS AArch64 from the `rows16.py`
pattern, and `jxl_output_tests::deep_output_equals_the_original_cjxl_bytes`
requires the sink to write the same bytes. They are never refreshed from
the engine's output.

```sh
python3 -I rows16.py 37 19 16 0 d16-37x19-a0.ppm     # and so on: W H DEPTH ALPHA
cjxl d16-37x19-a0.ppm d16-37x19-a0-lossless.jxl -d 0 -e 2 --num_threads=0 \
  -x color_space=RGB_D65_SRG_Rel_SRG
cjxl d16-37x19-a0.ppm d16-37x19-a0-d1.jxl --patches=0 -e 7 -d 1 --num_threads=0 \
  -x color_space=RGB_D65_SRG_Rel_SRG
```

`cjxl`'s PNM reader takes the depth from `MAXVAL` (`lib/extras/dec/pnm.cc`
lines 276 and 300), gives a PAM's alpha the color depth (line 537), passes
the samples as codes of that depth (`JXL_BIT_DEPTH_FROM_CODESTREAM`, line
649), and the encoder copies the basic info, depth included
(`lib/extras/enc/jxl.cc` line 257). The engine's sink makes the same calls
through `ferrisma-jxl-encoder` (`uint16` input, `FROM_CODESTREAM`, alpha at
the color depth). Files whose lossless color or alpha has more than 12 bits
are ISOBMFF containers with a `jxll` box (codestream level 10, required by
libjxl's `VerifyLevelSettings`); the others are raw codestreams, as `cjxl`
writes them. `d16-2049x40-a1-lossless.jxl` has 9 groups: the chunked path.

The bridge's example `encode_pnm` wrote the same bytes as `cjxl` for every
file here and for `d8-37x19-a0`, `d16-300x260-a0` and `d16-2049x40-a1`
lossy (checked 2026-11-15, not kept).

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `d10-2049x40-a0-d1.jxl` | 26,067 | `e07712063b290aafedf417d6c61a6bef3295a23cae2950cc7bd523d4c5f367a8` |
| `d10-37x19-a0-d1.jxl` | 445 | `7158ca92f16c42d6ce9d38109d03f09d233a7d043ff0c7017c535ada9c2771cd` |
| `d10-37x19-a0-lossless.jxl` | 1,767 | `6d3ad9f9ce7c038fcb2ceb4820b72d79dfc4a15ef5425ba2234364cd0cd6a038` |
| `d12-37x19-a1-d1.jxl` | 275 | `60bb39de9f3f594728999a135f37e5a3874479246e5ba94c1a2b1328b411f95c` |
| `d12-37x19-a1-lossless.jxl` | 1,944 | `961383732c6e3b396cfd56065828636fd294320372d25f2779a0578055233937` |
| `d16-2049x40-a1-lossless.jxl` | 208,036 | `ad8153fd6a00d8885edd1e75f3dc54126006745749a4d243f2ae3bb0a3433c2b` |
| `d16-37x19-a0-d1.jxl` | 191 | `73af25db8cdedb7c29fc08ae06dbbd25406381c5fb491bcad930b1d1ebc03855` |
| `d16-37x19-a0-lossless.jxl` | 1,732 | `871e8bc714db3c7d9c5529c92a50d8726f8e072e685a5b76382397d54d6caa6c` |
| `d16-37x19-a1-lossless.jxl` | 2,055 | `c1058cea14f43250015a3ac2b677d7b4115c8b061a499a63c477b3a4c6ff22a7` |
