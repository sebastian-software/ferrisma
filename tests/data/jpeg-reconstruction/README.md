# Original JPEG reconstruction fixtures

These inputs exercise the actual pinned JPEG coefficient/frame and writer
boundaries. No pixel decoder or re-encoder creates the expected JPEG bytes.

- `fixtures.tsv` retains all 25 original chroma pairs, all 128 geometry pairs,
  and nine additional public JPEG-encoder outputs: six from the existing writer
  corpus and three RGB baseline/progressive/restart files.
- `provenance.json` records each additional original encoder observation and
  input/output hashes. `tools/generate_jpeg_reconstruction_fixtures.py` reproduces
  these files through the explicit original oracle.
- `*.jpeg-data.txt` contains data-only observations from original ReadJpeg.
  The frame adapters clear every coefficient vector before DecodeFrame so these
  stored input coefficients cannot supply reconstructed output.
- `*.original.txt` records all initialized component coefficients, quantization
  tables, frame status/output fields and complete writer bytes from the actual
  original DecodeFrame/WriteJpeg boundary. Partial captures remove the final JXL
  byte and retain every reached mutation. `native-captures.json` records hashes;
  its TSV maps the same 18 cases to the ordinary Rust regression.

The independent source audit verifies both ordinary and nonrecovering
ASan/UBSan graphs, actual code addresses/source-object owners, exact compiled
source fragments, fixture hashes and complete defined-request retention. C++
remains an opt-in test oracle; ordinary Rust builds do not compile or link it.
Compressed reconstruction metadata and container/public API integration are
separate pending boundaries.

The source-only census separately retains qNaN/sNaN DC conversion and zero
quantizer division requests, ordinary observations and positive nonrecovering
UBSan diagnostics at the exact original lines. These undefined source outputs
supply no Rust expectations. Defined infinity clamps and invalid-table controls
remain exact paired comparisons.
