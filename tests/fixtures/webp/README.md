# WebP reference fixture

`reference-lossy.webp` is a copy of the tracked
`codecs/libavif-rs/upstream/libwebp/examples/test.webp` from the imported
libwebp 1.6.0 source (BSD-3-Clause; see `codecs/libwebp-rs/LICENSE` and
`PATENTS`). It is a 128 × 128 lossy image.

`reference-lossy.rgb` is the 49,152-byte RGB output of the independent C
libwebp 1.6.0 `dwebp` tool. Generate it with:

```sh
dwebp reference-lossy.webp -ppm -o reference-lossy.ppm
```

The PPM starts with `P6\n128 128\n255\n`; remove that 15-byte header to
obtain the `.rgb` payload. The adapter test compares both full output and an
interior crop exactly. This retained fixture does not replace the port's live
reference-oracle and provenance suites.
