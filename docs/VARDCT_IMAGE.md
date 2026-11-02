# First complete VarDCT decode path

`dec_frame::decode_vardct` reads a complete raw JPEG XL codestream and returns
parsed metadata, exact sRGB integer samples at its original bit depth, and
interleaved sRGB color floats before integer conversion. C++ is used exclusively
by the test oracle. This milestone completes the first VarDCT path; general
JPEG XL decoding and the full library port have additional boundaries.

## Original configuration

Follow `GeneratePipelineTests` in the pinned
`lib/jxl/render_pipeline/render_pipeline_test.cc`: distance 1, XYB, patches and
dots off, EPF zero, with both its base and Gaborish configurations. The complete
entry point admits one final regular 4:4:4 frame with sRGB metadata, 8/16-bit
gray/RGB, and optionally one full-resolution unassociated alpha plane at the
color sample depth. Actual progressive AC passes, coefficient-order selection,
both coefficient widths, default adaptive DC smoothing and multiple DC/AC groups
are decoded. The source simple rendering pipeline is selected explicitly.

EPF rendering, effects, references/DC frames, animation, previews, cropping,
resampling, associated/subsampled alpha, arbitrary extra channels, alternate
color profiles, container boxes and the public decoder API remain separate.
Admission returns `Unsupported` for these profiles rather than supplying a
replacement implementation. Malformed admitted streams follow the source's
status, partial writes, bounds checks and reader-close lifecycle.

## Connected source boundaries

1. Original global size/metadata/transform parsing, frame header and TOC.
2. `ProcessDCGlobal`: matrices, quantizer fields, block contexts, DC color
   correlation and modular global stream.
3. `DecodeVarDCTDC`, `DequantDC`, modular DC groups and `DecodeAcMetadata`:
   actual strategies, quantization fields, color-correlation and sharpness maps.
4. Pipeline preparation, adaptive DC smoothing, AC allocation, matrix decoding,
   coefficient orders and histogram setup, including source padding contexts.
5. Group cache initialization, actual pass/channel readers and histogram
   selection, AC symbol reconstruction, coefficient accumulation and ANS checks.
6. Scalar quantization-bias adjustment, block dequantization, DC/AC composition
   and `TransformToPixels` for all 27 original strategies.
7. Optional Gaborish, XYB conversion, original high-precision sRGB transfer,
   image-bundle output and integer export.
8. Deferred modular alpha conversion, section-reader close, finalization,
   consumed-byte accounting and complete-file EOF/metadata checks.

## Evidence

The component corpus adds four DC differential groups with 1,147 requests, three scheduling
groups with 190 requests and five dequantization/traversal groups with 814
requests. It covers source mutations on failure, all strategies and coefficient
widths, actual LZ77 activation, partial passes, draw modes, reused owners and
exceptional numeric words. Existing corpora and expected words remain intact.

The whole-file corpus compares 141 genuinely encoded original files, every
initialized output color float word and every integer sample. It includes
gray/RGB and alpha, 8/16 bits, odd sizes, multiple AC groups in both axes,
multiple DC groups, efforts 2/3/7/9, Gaborish and progressive AC. Fixtures assert
actual frame encoding, filter flags and multiple pass counts. Another 1,624
observations cover prefixes, payload bit mutations and trailing bytes. Two
4,135-request groups compare the original fast and high-precision sRGB helpers.

Ordinary Rust tests decode captured original RGB8 and gray-alpha16 files with
progressive AC and Gaborish, using independently captured original integer
samples. Their provenance is in [the fixture record](../tests/data/README.md).

The complete integrated native ARM64 tree runs 425 tests, including 246 differential groups
and 868,587 exact requests per configuration. Required commands are
`python3 tools/check.py --msrv` and
`python3 tools/check.py --sanitizers --msrv`, each in Debug, Release and Rust
1.85, followed by native Linux/macOS CI on the integrated PR tree.
Both complete local commands pass on native ARM64 macOS with zero failed or
ignored tests. The ARM64-only captured smoothing regression is additional to
the shared corpus. Native x86_64 therefore runs 424 tests with the same 246
differential groups and request count. Hosted validation is recorded in the
integration PRs.

The original compressed-DC observer's scoped numerical configuration is recorded
in [ADR 0002](adr/0002-differential-oracle.md). Source algorithms, source pins,
existing numeric flags, sanitizers and previous expected values remain intact.
SIMD, performance, general codec conformance and a complete encoder are separate
acceptance gates.
