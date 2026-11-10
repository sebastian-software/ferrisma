# JPEG reconstruction metadata fixtures

The four JPEG files are exact copies from the independent JPEG writer fixture
set in commit `71e353827499a0dba9d95833854d1b347ff54065`. Its original cjpeg
commands, input hashes, file hashes and pixel recipe are retained in
[provenance.json](provenance.json). They cover baseline 4:4:4, grayscale,
progressive 4:2:0 and restart 4:2:2 JPEGs.

The pristine libjxl `ReadJpeg(kReadAll)` supplies all parsed initialized fields.
`gray.fields.txt` captures that actual source owner. `gray.bundle.hex` captures
the actual source Bundle::Write result with three leading one bits and 1,844
bits written before byte padding. Rust does not generate these golden bytes.
The adapters explicitly initialize unused scan-component fixture entries before
observing them; the original JPEG parser and visitor stay unchanged.

Reproduce and verify the complete original-source capture with:

```sh
python3 tools/jpeg_metadata_reference.py --output /tmp/jpeg-metadata-reference
```

Build both ordinary and ASan/UBSan oracle executables first. The output includes
exact requests, complete owner snapshots, symbol/compile evidence and a
separately diagnosed source out-of-bounds struct probe. No JPEG parser, Brotli
implementation or JPEG reconstruction algorithm is supplied by this fixture.
