# Corpus and reproducible comparison inputs

This inventory extends the historical [M2 comparison](M2-sharp-comparison.md);
it does not replace or update its measurements. The question for a new run is
how current Ferrixel and Sharp compare on a specified output geometry, color
intent, and matched visual quality. Results are limited to the measured files,
implementations, presets, and host. There are no new timing results here.

## Inventory

[`corpus.json`](../../scripts/bench/corpus.json) pins two existing photographs
and a correctness-only alpha/rotation AVIF. The dog is a 12.2 MP photograph
with EXIF orientation 6 and an sRGB ICC profile; the flower is 3.4 MP. The
alpha fixture is not a throughput photograph. Verify the committed files with:

```sh
node scripts/bench/corpus.cjs scripts/bench/corpus.json .
```

The script verifies SHA-256 and required inventory fields. Dimensions, color,
orientation, attribution, and licensing are recorded descriptions; a matching
hash does not independently prove those descriptions. Upstream attribution and
licenses are linked in the inventory and the respective test-data READMEs.

A separately downloaded, real 60.2 MP photo extends the available size range:

- **Beach volleyball sunset**, 9504×6336 pixels, by Joseph Gage,
  [Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Beach_volleyball_sunset_(53458847235).jpg),
  [original Flickr page](https://www.flickr.com/photos/181920661@N03/53458847235/),
  [CC BY-SA 2.0](https://creativecommons.org/licenses/by-sa/2.0/). The original
  JPEG is 13,658,158 bytes with a 3,144-byte sRGB IEC61966-2.1 ICC profile.
  ImageMagick reported undefined EXIF orientation. Preserve the attribution and
  applicable share-alike terms when distributing derivatives.
- Its pinned SHA-256 and source are in
  [`corpus-large.json`](../../scripts/bench/corpus-large.json). The fetched
  bytes also matched Commons' published SHA-1
  `6ca410d95eeca4dfe2110d9680cfd0db2e45eb7e` on 2026-11-11. Fetch to a
  user-selected cache directory with at least 14 MB free, then verify:

```sh
mkdir -p /tmp/ferrixel-corpus
curl --fail --location --max-filesize 14000000 \
  'https://upload.wikimedia.org/wikipedia/commons/e/e9/Beach_volleyball_sunset_%2853458847235%29.jpg' \
  -o /tmp/ferrixel-corpus/ferrixel-beach-volleyball-53458847235.jpg
node scripts/bench/corpus.cjs scripts/bench/corpus-large.json /tmp/ferrixel-corpus
```

A partial transfer fails the hash check. Resume it with `curl --continue-at -`
using the same URL and output path, then verify again. Never use a failed hash
as a benchmark input. The JPEG is not checked in; select one of the optional large-image lanes below.

The inventory does **not** contain independent 24 MP and 48 MP photographs.
The optional `beach-24` and `beach-48` lanes select native-pixel regions of the
same original: respectively `(1752, 1168, 6000, 4000)` and
`(752, 168, 8000, 6000)` as upright `(x, y, width, height)`. Both pipelines
crop before resizing, with no upscaling or derivative JPEG. These are regions
of one scene, not independent photographs. Both receive the full 60.2 MP JPEG,
so do not describe the measurement as a 24 or 48 MP input decode or assume
partial decoding. The `beach` lane uses the whole original. The 48 MP crop
resizes to 1600×1200; the other two resize to 1600×1067.

## Comparison and retained evidence

The existing `compare-sharp.py` remains the compatibility harness; its default
is the four fixed historical image cases. `prepare` verifies the pinned dog and flower
bytes, builds the CLI and independent `quality-match` scorer, and prepares the
references. `match` measures output sizes and SSIMULACRA2 against an independent
linear-light Lanczos3 reference; it records the hashes of inputs, references,
binaries, scripts, and Sharp versions in `match-provenance.json`. `time` refuses
an incomplete, changed, or non-bracketing match. It independently rescores the
retained output and requires it to be within 0.1 SSIMULACRA2 of the matched
nearest output and within 0.5 of the shared target. A completed
`time-provenance.json` binds the CSV hash and expected and actual lane counts;
an interrupted or failed run remains marked incomplete. Ferrixel's rav1e speed
is explicitly 6 in this lane; the preset sweep chooses its speed separately.

Run the setup and phases from the checkout with a compatible Sharp installation
and the other prerequisites in the [historical method](M2-sharp-comparison.md):

```sh
export BENCH_SHARP_NODE_MODULES=/path/to/node_modules
python3 -I scripts/bench/compare-sharp.py prepare /tmp/ferrixel-compare
python3 -I scripts/bench/compare-sharp.py match /tmp/ferrixel-compare
COMPARE_RUNS=20 python3 -I scripts/bench/compare-sharp.py time /tmp/ferrixel-compare
```

For a focused run, set `COMPARE_IMAGE_SET=dog` or `flower`; for the external
photo use `beach`, `beach-24`, or `beach-48` and set
`COMPARE_LARGE_CORPUS=/tmp/ferrixel-corpus`. Set the same image set on **each**
phase. These selections support only `prepare`, `match`, and `time`; they fail
for the batch, ceiling, and sweep modes. To limit the configuration matrix to
the all-core 4:4:4 pair and target A, set `COMPARE_CONFIGS=all-444` and
`COMPARE_TARGETS=A` on both `match` and `time`. Use a distinct output directory
for each selection. The large lanes request a 2 GiB Ferrixel memory budget and
the independent reference construction can require substantially more process
memory; they are not yet measured. Both comparators receive the same crop and
output geometry. The source's embedded ICC is sRGB: the historical Sharp
ignore-profile workaround applies to this specific profile, not arbitrary ICC.
The independent reference decodes and crops before its own linear-light resize.

Run timing only on a quiet machine, record power and background-load conditions,
and repeat with an alternate lane order before interpreting small differences.
This harness currently uses a fixed order, one warmup, and the specified number
of process samples. `time.csv` reports median (p50), nearest-rank p95, input,
reference and final output SHA-256, output bytes, shared target, actual
SSIMULACRA2 and their difference, exact argument
array, and process peak RSS from a separate invocation. At 20 samples p95 is the
19th sorted value; such a small sample does not characterize tail behavior.
Retain the raw Hyperfine JSON, matching TSV and provenance JSON, output files,
references, tool versions, machine details, and the exact commands alongside
any report. Process wall time includes startup and I/O; peak RSS is not an
allocation count and its sampling run is not one of the timed samples.

The `time` mode requires matching evidence produced by this version of the
harness. Missing Sharp, Hyperfine, the scorer, or another required tool fails the
relevant mode. A successful mode does not prove fairness: check output geometry,
ICC conversion, orientation, alpha, bit depth, subsampling, quality ceilings,
and preset differences before comparing. The current scorer supports opaque
sRGB RGB only, not alpha compositing or arbitrary ICC transforms. The historical
Sharp workaround ignores the dog photo's sRGB profile and does not generalize to
other profiles. The alpha fixture is inventoried but is not part of an automated
quality-matched lane. The historical 8-bit Sharp
versus 10-bit Ferrixel AVIF distinction and the independent scorer's decoder
bias remain material. There is no separate libvips or libavif-encoder comparison
in this lane.
