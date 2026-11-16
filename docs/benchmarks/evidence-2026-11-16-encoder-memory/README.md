# Evidence: encoder memory against the engine's reservations, 2026-11-16

Record: [v1.0-jxl-encoder-memory-2026-11-16.md](../v1.0-jxl-encoder-memory-2026-11-16.md).

**Memory, not timing.** Host: Apple M1 Pro (10 cores, 32 GB), macOS 27.2.1,
shared with other agents; `rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)`
(the pinned toolchain), the workspace `release` profile, native AArch64,
`main` at `00e1e83a`.

## Files

- `results-jxl.txt`: the JPEG XL measurements the model is fitted on. One
  line per accepted fresh process: input, mode (`lossless` or the lossy
  distance), run, maximum resident set size, peak memory footprint, the
  probe's live-heap peak, output bytes, the first 16 hex digits of the
  output's SHA-256. Lines starting with `#` are the environment before and
  after each batch (`sysctl vm.swapusage`, `memory_pressure`, load) and the
  rejected runs. Written by `tools/measure_serial.sh` in three batches
  (03:01, 03:03 and 04:02). The first stopped early: its first acceptance
  rule (RSS and footprint within 5 %) also rejected small images for the
  ~2 MB of clean pages RSS counts and the footprint does not, and large ones
  for the allocator's reusable pages; its accepted lines are kept. The
  third was stopped at 04:37 because the machine did not calm down (the
  record lists what is unmeasured).
- `results-other.txt`: the AVIF and WebP probes (`tools/run_other.sh`).
  **Indicative only**: they ran in parallel with other probes while the
  machine swapped (see "Disturbed runs"), which lowers RSS, so they can only
  understate a shortfall.
- `pnm-check.txt`: the 2026-11-15 deep-memory tool (`encode_pnm`) against
  `jxl_memory_probe` on the same file, alternating.
- `inputs.sha256`: every input.

## Method

- One probe process at a time (`tools/measure_serial.sh`). A run is
  accepted when free memory was at least 20 % before it, swap use grew by at
  most 32 MiB during it, and its peak footprint is at most 2 % above its
  maximum RSS (the footprint counts compressed and swapped pages, RSS does
  not; a larger footprint means pages left memory during the run). RSS above
  the footprint is accepted: the allocator's freed, reusable pages stay
  resident and count in RSS but not in the footprint. Each case needs three
  accepted runs (at most six attempts); the model takes the largest RSS, the
  basis of the 2026-11-15 evidence, and the footprint is reported beside it.
- `tools/jxl_memory_probe.rs` holds what the engine's JPEG XL sink holds at
  the encode, its frame of four samples per pixel (compacted in place to
  three when opaque), drops the input file first, and makes the sink's
  bridge call (`encode_lossless`, or `encode` with
  `FrameOptions::with_distance(d)`, patches off). Built from the
  repository root with a separate target directory:

  ```sh
  cp tools/jxl_memory_probe.rs ../../../crates/ferrisma-jxl-encoder/examples/
  CARGO_TARGET_DIR=target/memprobe cargo build --locked --offline --release \
    -p ferrisma-jxl-encoder --example jxl_memory_probe
  ```

  `tools/avif_memory_probe.rs` (into `crates/ferrisma-avif-write/examples/`,
  `-p ferrisma-avif-write`) and `tools/webp_memory_probe.rs` (into
  `codecs/libwebp-rs/examples/`, `-p libwebp-rs`) do the same for the AVIF
  writer (`Frame` at the engine's default options, quality 60, speed 6,
  4:4:4, one thread) and the WebP sink (ARGB frame, `Config`, `encode_to`).
  The AVIF probe's heap counter does not see the port's C-style allocations.
- Inputs (`tools/tile.py`, `tools/noise.py`): `p-*` the 24 MP photograph of
  2026-11-15 (`photo24.ppm`) cropped or mirrored to size; `w8-*`/`w16-*`
  libavif's `tests/data/weld_16bit.png` mirrored to size at 8 and 16 bits;
  `n8-*`/`n16-*` seeded uniform noise, the content that compresses worst.
  `*a` add alpha. Sizes: 512x1024 (8 groups, the largest one-shot frame),
  700x700 (9 groups, streaming), 2048x2048 (one DC group), 2304x2048 (just
  above), 3000x2000, 4032x3024, 6000x4000.
- `tools/fit.py` fits the model and prints the tables of the record
  (`--table`) and the rows of
  `jxl_output_tests::model_covers_every_measured_peak_with_margin`
  (`--rust`):

  ```sh
  python3 -I tools/fit.py results-jxl.txt \
    --old-lossless ../evidence-2026-11-15-jxl-lossless-memory/results.txt \
    --old-deep ../evidence-2026-11-15-jxl-deep-memory/results.txt --table
  ```

## Disturbed runs (not used)

Between 02:00 and 03:00 the probes first ran in parallel loops (JPEG XL
lossless and lossy, AVIF, WebP) on a machine at a load average of about 150
with 12.4 of 13.3 GB swap in use. Several of those runs had a footprint far
above their RSS or an RSS below their own live heap (2048x2048 lossy at
distance 0.05: 200 MB RSS against a 299 MB live heap), so pages were
compressed or swapped during the run. Those JPEG XL results are discarded
and every case the model depends on was measured again serially. The AVIF
and WebP results of that period are kept as indicative (`results-other.txt`).
