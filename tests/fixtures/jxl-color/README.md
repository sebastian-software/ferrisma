# JPEG XL color-signalling fixtures (issue #41)

No JPEG XL encoder is available on the development host (the workspace
libjxl-rs encoder API is not finished, and the C libjxl submodule is not
checked out). These files are therefore derived from two independently
encoded fixtures by `tools/respell.py`, which rewrites only the enum
`colour_encoding` bundle of the image header and copies every other header
bit and all byte-aligned frame data unchanged:

- `orientation/jxl-1.jxl`: lossless modular, non-XYB, 24 x 16 RGB8, written
  by libjxl `a16d32d` (see `../orientation/README.md`). Its samples are
  known exactly: `../orientation/jxl-1.rgb`.
- `sample.jxl`: lossy XYB VarDCT, 7 x 5.

For a non-XYB file, a new enum declares a new meaning for the same stored
samples. For an XYB file, the decoder renders XYB into the declared
encoding. Regenerate and check with:

```sh
python3 -I tests/fixtures/jxl-color/tools/respell.py
shasum -a 256 tests/fixtures/jxl-color/*.jxl
```

| File | Source | Declared encoding (enum values) | SHA-256 |
| --- | --- | --- | --- |
| `lossless-p3.jxl` | `jxl-1.jxl` | Display P3: D65, P3 primaries, sRGB transfer | `ceac36219856c3f58d1598ba2cca2daa05b09d6bbef2dae194afbbf6be4b6de4` |
| `lossless-pq.jxl` | `jxl-1.jxl` | D65, BT.2100 primaries, PQ | `6daeb5120834b7a2632887e82c73158dfa7058b64e81610491865d4eb0d25839` |
| `lossless-hlg.jxl` | `jxl-1.jxl` | D65, BT.2100 primaries, HLG | `f25373bbd0577dc37e8fa20b4c4ab43e646675dde7bf28a107a1e925425cd421` |
| `lossless-linear-srgb.jxl` | `jxl-1.jxl` | D65, sRGB primaries, linear | `0069058bbad811d40a5459527e4034d8a227ea041891ff5f9106fdfc8c0b5594` |
| `lossless-dci-p3.jxl` | `jxl-1.jxl` | DCI white, P3 primaries, DCI transfer | `09f13bcab3fc7f78a848fed731e81bfe093e9cfbb045e38d0037281b873688b6` |
| `xyb-p3.jxl` | `sample.jxl` | Display P3 | `5d7d01eb633e69e1789ad0a65bd60c9e3451ba627e910a96349fff10e129a007` |
| `xyb-pq.jxl` | `sample.jxl` | D65, BT.2100 primaries, PQ | `9b49c29d490c53c4bbe672e1c069edf343fefdd73dad007adf60e51d8ee7dac0` |
| `lossless-custom-pq.jxl` | `jxl-1.jxl` | D65, custom primaries, PQ | `b32ad33224a0b98ccc2805625ac48887af6e504bea9540541b8345b6ab1f8762` |
| `lossless-custom-hlg.jxl` | `jxl-1.jxl` | D65, custom primaries, HLG | `adef0537414c83a7a3ab61237a2884afa0b674a6f456f662ca0bf0cc7f8fdd49` |
| `lossless-e-white-pq.jxl` | `jxl-1.jxl` | white point E, BT.2100 primaries, PQ | `7d7ce5d242255ea6d6cdeee74f9653b8df1194083506213d47fdb41c729cd455` |
| `xyb-custom-pq.jxl` | `sample.jxl` | D65, custom primaries, PQ | `03d913e3c1bfad714fa06232cfcfd84894e473ba71592a979372896fed7b2547` |

The custom primaries are red (0.70, 0.29), green (0.17, 0.79) and blue
(0.13, 0.045), written as `Customxy` chromaticities. These four files were
added on 2026-11-12 for the independent review finding that PQ/HLG input
without any H.273 code point for its gamut was converted as SDR under
`UseRenderedIcc`: neither decoder reports CICP for them, and the
synthesized ICC carries no `cicp` tag. The regenerated earlier files are
byte-identical to their previous hashes.

Both jxl-oxide 0.12.6 and the libjxl-rs port decode every file. No file
carries an embedded ICC profile: writing one needs a JPEG XL ICC-stream
encoder, which this tool does not implement. ICC-signalled JPEG XL input
therefore remains covered only by the source review recorded in ADR-0015
(2026-11-12), not by a fixture.

These files inherit the license of their sources (BSD-3-Clause libjxl test
output and the repository's own `sample.jxl`).
