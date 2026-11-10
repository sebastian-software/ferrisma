# JPEG writer fixtures

These six JPEGs are generated from deterministic PPM/PGM pixels with the actual
libjpeg-turbo cjpeg 3.2.1 executable. The pixel recipe, exact encoder options,
source-image hashes, executable hash and JPEG hashes are recorded in
[provenance.json](provenance.json). No JPEG marker, Huffman code, or entropy data
is constructed by the fixture generator.

Reproduce them with:

```sh
python3 tools/generate_jpeg_writer_fixtures.py --cjpeg /absolute/path/to/cjpeg
```

Baseline, progressive, grayscale, 4:4:4/4:2:2/4:2:0, restart markers, byte stuffing
and multiple bit-writer chunks are represented. Tests independently ask the pinned
original ReadJpeg for the initialized JPEGData fields and then compare actual
original WriteJpeg with the Rust translation. Both writers reproduce every
original file byte-for-byte. Additional data-only mutations exercise the writer's
source-defined malformed-structure domain, callback errors and metadata payloads;
those mutations do not claim that the resulting bytes are valid JPEG images.

Three standalone regressions retain actual original callback captures for
short writes, a zero write, and null versus retained allocated empty tail data.
