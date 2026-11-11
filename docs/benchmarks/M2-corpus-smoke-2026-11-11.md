# M2 corpus and quality smoke, 2026-11-11

This is diagnostic evidence for the comparison tooling on macOS 27.0.1 ARM64,
not a performance comparison or release gate. The Ferrixel binary was built
from commit `19d8be23a57a4c676bf1ee25fa4e09634cdba416`, before the later
integration work; the benchmark worktree contained the harness changes in this
record. Rust was `1.100.0-nightly (5a2be9f5f 2026-09-06)`. Sharp was 0.35.5,
libvips 8.18.7, and libaom 3.15.1. The scorer was the separately built
`ferrixel-quality-match` using `avif-decode` 3.0.0 and SSIMULACRA2 0.5.1. Its
binary and harness hashes and the complete Sharp dependency versions are in
[match provenance](evidence-2026-11-11/match-provenance.json).

## Quality-matched dog photo

For the hash-pinned 4032×3024 dog JPEG, orientation 6, sRGB ICC, the harness
made a 1600×2133 output and independently scored it against the Sharp-decoded,
linear-light f64 Lanczos3 reference. Ferrixel used the rendered sRGB ICC and
rav1e speed 6; Sharp ignored this particular sRGB ICC before its linear scRGB
pipeline and used libaom effort 4. Ferrixel's output was 10-bit AVIF and Sharp's
was 8-bit. The reference shares its JPEG decoder with the Sharp candidate but
not with Ferrixel. The ICC workaround does not apply to arbitrary profiles.

Target A was Ferrixel quality 80: score 73.467 and 261,883 bytes. The recorded
nearest bracket entries were Ferrixel quality 79.77, score 73.467, 261,883
bytes, and Sharp quality 56, score 73.492, 231,260 bytes. The raw
[anchors](evidence-2026-11-11/anchors.tsv),
[match points](evidence-2026-11-11/matched.tsv), and
[all output hashes](evidence-2026-11-11/output-hashes.json) are retained.
Only one image and one pair of presets were checked; the scores and file sizes
do not establish behavior across other images, formats, or encoder settings.
The matching method and its limitations are in the [historical comparison](M2-sharp-comparison.md).

Preparation and matching used a shared target directory and this shell setup,
with Sharp already installed at the indicated path. Use another `node_modules`
path containing a compatible Sharp installation on another machine:

```sh
export CARGO_TARGET_DIR=/Users/sebastian/Workspace/ferrixel/target
export BENCH_SHARP_NODE_MODULES=/private/tmp/ferramenta300-registry-consumer/node_modules
export COMPARE_IMAGE_SET=dog COMPARE_CONFIGS=all-444 COMPARE_TARGETS=A
CARGO_NET_OFFLINE=true python3 -I scripts/bench/compare-sharp.py prepare /private/tmp/ferrixel-evidence-dog-20261111
python3 -I scripts/bench/compare-sharp.py match /private/tmp/ferrixel-evidence-dog-20261111
```

A subsequent 10-sample timing attempt was stopped after the first lane when
system load prevented the Sharp lane from passing the existing load gate. The
[provenance](evidence-2026-11-11/time-incomplete.json) is explicitly incomplete;
there is no paired latency or peak-RSS result here. A previous sandboxed attempt
also failed to obtain macOS process peak RSS. Raw partial timings are not used as
comparison evidence.

## Large native-region correctness smoke

The [separately attributed](corpus-and-comparison.md) Joseph Gage photograph was
verified at SHA-256
`f3194042c19e9f76cdd6c5e3f32abaeb97822fd2f4eb368edc66a28e9ec732fe`.
Both tools read the **full 9504×6336 JPEG** and used upright crop
`1752,1168,6000,4000` (24 million pixels), resized to 320×213. They emitted RGB
AVIF files that ImageMagick independently identified as 320×213. Against a
Sharp-decoded, cropped source passed through the independent linear-light
reference scorer, the Ferrixel quality-80 output scored 73.838 and the Sharp
quality-50 output 64.376. These scores demonstrate successful geometry and
scoring on the two files; the different quality settings are **not matched**
and do not support a file-size or performance comparison. This is one image,
not evidence of partial JPEG decoding.

The reference SHA-256 was
`67b425bca62d5ede3a8228028e20c1944651635f1f6d9b7bf633a8afdf9a3c1e`;
the Ferrixel and Sharp outputs were respectively
`98f2596c085cc6f560df1cd0e4946065a8659d48ab6611c9cd0a3fc4f2d1f4af`
and `56b51c8873ec4f8c6c0379a3d50eac2eea316f0400881a3f987f7c65ed12fd68`.
The following commands reproduce the smoke after obtaining the source using the
[corpus instructions](corpus-and-comparison.md); the paths are local examples.

```sh
ferrixel=/Users/sebastian/Workspace/ferrixel/target/release/ferrixel
scorer=/Users/sebastian/Workspace/ferrixel/target/release/ferrixel-quality-match
photo=/private/tmp/ferrixel-beach-volleyball-53458847235.jpg
export NODE_PATH=/private/tmp/ferramenta300-registry-consumer/node_modules
"$ferrixel" "$photo" /private/tmp/ferrixel-beach-crop-smoke.avif 320 213 \
  --use-rendered-icc --crop=1752,1168,6000,4000 --budget-mb=2048 \
  --quality=80 --speed=8 --encoder-threads=1 --transform-threads=1
node scripts/bench/sharp-avif.cjs "$photo" /private/tmp/ferrixel-beach-crop-sharp.avif 320 213 \
  quality=50 icc=ignore threads=1 crop=1752,1168,6000,4000
node -e 'require("sharp")(process.argv[1],{ignoreIcc:true}).autoOrient().extract({left:1752,top:1168,width:6000,height:4000}).toColourspace("srgb").png({compressionLevel:1}).toFile(process.argv[2])' \
  "$photo" /private/tmp/ferrixel-beach-crop-reference.png
"$scorer" ref /private/tmp/ferrixel-beach-crop-reference.png 320 213 /private/tmp/ferrixel-beach-crop-reference.ref
"$scorer" score /private/tmp/ferrixel-beach-crop-reference.ref /private/tmp/ferrixel-beach-crop-smoke.avif /private/tmp/ferrixel-beach-crop-sharp.avif
```
