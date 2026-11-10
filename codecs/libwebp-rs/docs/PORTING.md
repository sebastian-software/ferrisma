# Porting contract

libwebp-rs is a mechanical Rust translation of libwebp, the WebP reference
implementation: decoder, demuxer, encoder, muxer and SharpYUV. The pinned source
is the `upstream/libwebp` Git submodule at tag `v1.6.0`, commit
`4fa21912338357f89e4fd51cf2368325b59e9bd9`; the test vectors are the
`upstream/libwebp-test-data` submodule at
`260804d686f0491e0326a50eef491829dae09e9e`. Keep libwebp's BSD license and the
separate WebM patent grant with the derived source
([PROVENANCE.md](PROVENANCE.md)). The work was delivered in packages
([ADR 0009](adr/0009-work-packages-and-integration.md)); the decisions are in
the [ADRs](adr/README.md).

## Preserved behavior

Preserve algorithms, tables, constants, numeric widths and signedness,
operation order, accepted input, returned status codes, partial state on
error, incremental-decoding state, callbacks and their order, threading
behavior and cleanup obligations at each translated boundary. Existing portable
optimizations are part of the source behavior. Do not introduce new ones,
refactor algorithms or tune parameters during the translation. SIMD variants
are translated behind runtime selection, with exact parity against the
scalar translation ([ADR 0008](adr/0008-simd-modules.md)).

Compare decoded samples, encoded bytes, `WebPAuxStats`, metadata chunks,
iterator results and API state transitions under matching options and the
reference configuration below. Floating-point results must match the oracle
exactly on the same host; equality across hosts and compilers is recorded, not
assumed. Timing, allocation addresses and machine code are not expected to
match; measure performance and memory separately after parity.

## Translation rules

1. Start from the pinned source, not from RFC 6386 or RFC 9649, which are
   reading aids only.
2. Translate into the module file that already exists for the upstream file
   ([ADR 0001](adr/0001-mechanical-port.md), `tools/units.json`). Never edit its
   provenance header, a `mod.rs` or `src/lib.rs`.
3. Name items with the deterministic rule of ADR 0001
   (`python3 tools/rust_name.py <CName>`), and mark every translated item with
   `// upstream: <path> <CName>`.
4. Keep integer widths, signedness, truncation and wrapping deliberate; map
   pointers, ownership, errors, allocation limits, dispatch and threads as
   [ADR 0003](adr/0003-rust-representation.md) prescribes.
5. Preserve partially completed state and the first-error precedence of the
   status setters.
6. Keep unported functions absent. Do not add stubs, placeholders, partial
   functions, `todo!()` or C fallbacks.
7. Write no `unsafe` code outside `src/abi`
   ([ADR 0005](adr/0005-toolchain-and-unsafe-policy.md)) and the SIMD modules,
   which may use it for the two operations that
   [ADR 0008](adr/0008-simd-modules.md) names.
8. Record ambiguous or undefined source behavior in the package's status file
   and exclude it from the equivalence oracle explicitly, never silently
   define it. Cases that need a `NULL` pointer exist only at the C ABI.
9. Compare source and target independently, in separate processes. A Rust
   round trip is not evidence.

## Reference configuration of the oracle

The oracle is the pinned libwebp built through its own CMake project with:

```sh
cmake -S upstream/libwebp -B build/oracle -DCMAKE_BUILD_TYPE=Release \
  -DWEBP_ENABLE_SIMD=OFF -DWEBP_USE_THREAD=ON -DWEBP_NEAR_LOSSLESS=ON \
  -DWEBP_ENABLE_SWAP_16BIT_CSP=OFF -DWEBP_BUILD_DWEBP=ON -DWEBP_BUILD_CWEBP=ON \
  -DWEBP_BUILD_WEBPINFO=ON -DWEBP_BUILD_LIBWEBPMUX=ON -DWEBP_BUILD_ANIM_UTILS=OFF \
  -DWEBP_BUILD_GIF2WEBP=OFF -DWEBP_BUILD_IMG2WEBP=ON -DWEBP_BUILD_VWEBP=OFF \
  -DWEBP_BUILD_WEBPMUX=OFF -DWEBP_BUILD_EXTRAS=OFF -DCMAKE_C_FLAGS=-ffp-contract=off
```

`python3 tools/build_oracle.py` applies exactly these options through
`tools/oracle/CMakeLists.txt`, which also builds the probes, and adds `-msse2`
on x86-64 to restore the psABI baseline that upstream's SIMD-off flags remove
([ADR 0002](adr/0002-differential-oracle.md)). The resulting `config.h` defines
`WEBP_USE_THREAD` and `WEBP_NEAR_LOSSLESS` and no `WEBP_HAVE_<SIMD>` macro;
`WEBP_SWAP_16BIT_CSP`, `WEBP_REDUCE_SIZE` and `WEBP_REDUCE_CSP` are unset.
`img2webp` is built (since WP14) to assemble animated test inputs; it changes no
library object. Other configurations are not claimed.

- SIMD off and `-ffp-contract=off` give a non-contracting scalar reference,
  as in libavif-rs ADR 0024. Rust does not contract floating-point expressions.
- Variants: `release` is the reference, `asan-ubsan` finds undefined behavior
  in the original per case and must agree with `release` on defined cases, and
  `debug` enables upstream's assertions. A SIMD-on oracle is diagnostic only;
  upstream's own `test_cwebp.sh` compares optimized with `-noasm` output, so a
  SIMD-versus-scalar difference is upstream's to explain.
- Floating point appears in `src/enc/quant_enc.c`, `picture_csp_enc.c`,
  `picture_psnr_enc.c`, `frame_enc.c`, `filter_enc.c`, `config_enc.c`,
  `webp_enc.c`, `picture_enc.c`, `src/dsp/lossless_enc.c`, `src/dsp/ssim.c`,
  `src/dec/io_dec.c`, `src/utils/random_utils.c`, `quant_levels_utils.c`,
  `sharpyuv/sharpyuv_gamma.c` and `sharpyuv_csp.c`. Rust's `f64::powf` and its
  relatives call the platform libm, so the expectation is host-local equality
  with the oracle.

## Verification gates

Every package proves its units differentially: C probes under
`tools/oracle/probes/<wp>/` call the oracle in separate processes, and tests in
`tests/<wp>_*.rs` compare exact bytes, status codes and partial state with the
Rust translation, in the dev, release and `checked` profiles. A test file's
`//!` documentation names its inputs (boundary values, truncations, seeded
generated data and the pinned vectors) and the cases it excludes.
`tests/wp0_features.rs` shows the pattern end to end.

The integration packages add the end-to-end gates, all of which pass on both
CI systems (the `Checks` workflow on `main` records the runs):

- Upstream's own scripts with the unmodified upstream tools linked against the
  Rust library: `test_dwebp.sh` (all 480 checksums of 96 vectors in five output
  formats, with and without `-noasm`), `test_lossless.sh` (33 vectors in five
  formats) and `test_cwebp.sh` (the 142 test-data files `cwebp` reads in three
  option sets); `webpinfo` prints identical output for every vector.
- The decoder matrix over the 131 vectors: `use_cropping` with odd and even
  origins and sizes, `use_scaling` to several targets, `bypass_filtering`,
  `no_fancy_upsampling`, `flip`, `use_threads`, every `WEBP_CSP_MODE` including
  the premultiplied modes, `WebPDecodeYUVInto` and the simple API, with exact
  buffer equality including guard bytes around caller memory; the incremental
  decoder fed in chunk sizes 1, 7, 64 and 4,096 bytes with every intermediate
  state compared; truncations at every chunk boundary and bit flips.
- The encoder matrix with byte-identical output and identical `WebPAuxStats`:
  the vectors decoded to `ppm`/`pam`, `grid.png` and seeded synthetic pictures
  (odd sizes, 1 by 1, the 16,383-pixel dimension limit, alpha patterns);
  lossy quality {0, 10, 50, 75, 90, 100} by method {0..6} by every preset,
  `segments`, `partitions`, `sns_strength` and `filter_strength` at defaults
  and extremes, `filter_type`, `autofilter`, `alpha_quality` {0, 50, 100},
  `alpha_filtering`, `alpha_compression`, `use_sharp_yuv`, `exact`,
  `emulate_jpeg_size`, `low_memory`, `thread_level` {0, 1}; lossless quality
  {0, 50, 100} by method {0..6}, `exact`, `near_lossless` {0, 40, 60, 80, 100},
  `use_delta_palette`; `target_size` and `target_PSNR` with `pass` {1, 3, 10};
  `show_compressed`, the progress hook with abort, `WebPMemoryWriter` and a
  custom writer; equality across `thread_level`.
- Containers and animations: the demuxer and the animation decoder on
  hand-assembled containers and `img2webp` animations (blend and dispose
  combinations, odd offsets), the muxer and the animation encoder on the same
  inputs plus seeded edit sequences and option matrices, with `webpmux`,
  `img2webp`, `anim_dump` and `anim_diff` giving identical results against both
  libraries.
- The safe APIs replay the decoder and encoder matrices through `webp::api`
  and compare with the C probes.
- Upstream's fuzzers as `fuzz/` targets, with the oracle under the sanitizers
  for the differential tests.

No package-level result is a substitute for those gates, and passing a wave is
not "libwebp is ported". Each module's documentation names its deviations and
exclusions; the issues name the open work.
