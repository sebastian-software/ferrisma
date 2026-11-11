# JPEG XL input adapter comparison, 2026-11-11

**Question.** What happens to full and center-crop decode time and peak process memory when the experimental engine selects the local libjxl-rs decoder in place of its default `jxl-oxide` input?

## Method

Host: Apple M1 Ultra (Mac13,2), 64 GiB, Darwin 27.0.0 arm64. Rust: `rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)`, Cargo `1.100.0-nightly (3c0b53475 2026-09-04)`. Base engine commit: `f0ffebf4fb759457a5a03726265931182e2882e8`; comparison targets: local `libjxl-rs` at the workspace pin and `jxl-oxide 0.12.6` with default features disabled. Both use one decoder thread. No other Cargo builds or benchmarks ran during the final timed samples.

`cargo build -p ferrixel-jxl-decoder --example jxl_decode_compare --release --locked --offline` builds one binary containing both candidates. Each separate process runs the selected candidate for the specified number of iterations. Its timer covers opening/parsing the encoded input, output allocation, complete decode, and consuming a sample from every emitted row; it excludes reading the input file and one candidate-specific dimensions probe. Center crop takes the middle half of each dimension. `own` reconstructs the full F32 image before slicing; `oxide` requests the crop through `set_image_region`. Both use native F32 samples with the decoder's default upright rendering and no thread pool. `/usr/bin/time -l` records maximum resident set size for that process, including the harness, encoded bytes, and both linked implementations; it is **not** a codec-only allocation measure. Run each mode in a fresh process. The harness also provides a separate full-image float comparison.

Inputs:

- Tiny regression fixture: `tests/fixtures/sample.jxl`, 7×5. It is not a performance representative.
- Photo: tracked `codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg`, 2268×1512 JPEG, SHA-256 `6ad9a79e9f26d2723ef954a0dca774429606655ab7184f8ab400016f228ca2ce`. Reconstructed as a JPEG XL file by the pinned libjxl C++ reference oracle (`a16d32d0ac113f5e54125e49bb571281c5d0b704`), oracle executable SHA-256 `d9bed0c66f55dd67f2836f7d1c6d3347384fda53a372b263e062954e526db5fa`. Command: `{ printf 'ycc-jpeg-encode '; xxd -p <input.jpg> | tr -d '\n'; printf '\n'; } | <pinned-jxl-oracle> | awk '{print $NF}' | xxd -r -p > <output.jxl>`. The generated 445 KiB JXL has SHA-256 `f202533a6bf4565a36f30eefb529bfe13cac48acb745abbdaa36da3ccd6d4c14` and is retained at `/private/tmp/ferrixel-jxl-flower-20261111.jxl` for this local run.

Direct command pattern: `/usr/bin/time -l target/release/examples/jxl_decode_compare <input.jxl> <own|oxide> <full|crop> <iterations>`. The binary reports elapsed milliseconds and a checksum so a failed decode cannot appear as an empty timing. Timing figures are cumulative per process, so divide by iterations for approximate per-image time. The photo's 10,287,648 F32 channel samples compared with maximum absolute difference `2.98e-7` and mean absolute difference `1.4e-8`; all samples were finite. The retained eight orientation fixtures separately match their independently encoded expected samples exactly at several crops.

## Results

Three fresh processes per mode, three decodes per process on the 2268×1512 flower photo. Times are cumulative for three decodes; max RSS belongs to each entire process.

| Decoder | Request | Elapsed range, 3 decodes | Max RSS range |
| --- | --- | ---: | ---: |
| local libjxl-rs | full | 819.7–867.9 ms | 56.8–60.6 MB |
| local libjxl-rs | center crop | 813.2–841.3 ms | 56.8 MB |
| jxl-oxide | full | 231.8–234.3 ms | 100.1–104.6 MB |
| jxl-oxide | center crop | 85.6–88.4 ms | 35.6–35.7 MB |

Full decode through the local port is about 3.6× slower here; center crop is about 9.5× slower because the local port reconstructs the full image. Its full-frame process RSS is lower, but its crop RSS is higher. The first pilot used a jxl-oxide dimensions probe for both candidates before the timer and is excluded from max-RSS conclusions. On a **257×129** JPEG reconstruction fixture, pilot full-decode times were approximately 78 ms for 20 own iterations and 103–107 ms for 20 oxide iterations; the tiny 7×5 fixture took 17 ms versus 1.5 ms for 100 iterations. Those workloads show that relative behavior changes with input size and content, and are not photo-scale selection evidence.

## Interpretation and limits

The local libjxl decoder has no ROI request, so center-crop timing cannot demonstrate avoided codec reconstruction. It allocates and budgets one complete F32 frame even for a crop. `jxl-oxide` may avoid region work, but this harness does not instrument its internals. The measured photo regression makes the local decoder an opt-in experiment; the engine's existing `jpeg-xl-read` feature should keep jxl-oxide as its default backend pending the selection gates. The full external JPEG XL conformance corpus, x86-64 measurements, a broader 12–48 Mpx photo corpus, and native peak-memory attribution remain open under ADR-0025. This record supports only an experimental adapter decision and should be revisited before final codec selection or release acceptance.
