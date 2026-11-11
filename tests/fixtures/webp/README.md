# WebP reference fixture

`reference-lossy.webp` is a copy of the tracked
`codecs/libavif-rs/upstream/libwebp/examples/test.webp` from the imported
libwebp 1.6.0 source (BSD-3-Clause; see `codecs/libwebp-rs/LICENSE` and
`PATENTS`). The source entered this repository in commit `ead92e1a97c70bcf572885fed2ba930c0231987b`.
It is a 128 × 128 lossy image. Its SHA-256 is
`1bf9449d529b491c6eb22ec84d42217e43cf9af0ffe391fa7a30b29b037f098a`.

`reference-lossy.rgb` is the 49,152-byte RGB output of the independent C
libwebp `dwebp -version` 1.6.0 tool. Its SHA-256 is
`18f5c4d456175ab0d3215044b19c5e6cc0bf602c5418a1eaac0b3071d9130c6d`.
Generate it with:

```sh
dwebp reference-lossy.webp -ppm -o reference-lossy.ppm
```

The PPM starts with `P6\n128 128\n255\n`; remove that 15-byte header to
obtain the `.rgb` payload. The adapter test compares both full output and an
interior crop exactly. This retained fixture does not replace the port's live
reference-oracle and provenance suites.
