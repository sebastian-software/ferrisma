# Porting contract

libwebp-rs is a mechanical Rust translation of libwebp, the WebP reference
implementation: decoder, demuxer, encoder, muxer and SharpYUV. The pinned source
is the `upstream/libwebp` Git submodule at tag `v1.6.0`, commit
`4fa21912338357f89e4fd51cf2368325b59e9bd9`; the test vectors are the
`upstream/libwebp-test-data` submodule at
`260804d686f0491e0326a50eef491829dae09e9e`. Keep libwebp's BSD license and the
separate WebM patent grant with the derived source
([PROVENANCE.md](PROVENANCE.md)). The work is split into packages in
[WORKPLAN.md](WORKPLAN.md); the decisions are in the [ADRs](adr/README.md).

## Preserved behavior

Preserve algorithms, tables, constants, numeric widths and signedness,
operation order, accepted input, returned status codes, partial state on
error, incremental-decoding state, callbacks and their order, threading
behavior and cleanup obligations at each translated boundary. Existing portable
optimizations are part of the source behavior. Do not introduce new ones,
refactor algorithms or tune parameters during the translation. SIMD variants
are translated only in wave 4, behind runtime selection, with exact parity
against the scalar port.

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
   ([ADR 0005](adr/0005-toolchain-and-unsafe-policy.md)).
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
Rust translation, in the dev, release and `checked` profiles. The verification
row of each package in [WORKPLAN.md](WORKPLAN.md) names its minimum inputs:
boundary values, truncations, seeded generated data and the pinned vectors.
`tests/wp0_features.rs` shows the pattern end to end.

The integration packages add the end-to-end gates: `test_dwebp.sh`,
`test_lossless.sh` and `test_cwebp.sh` with the unmodified upstream tools linked
against the Rust library, the advanced-API and incremental-decoder matrices,
the encoder matrix of [PORT_PLAN.md](PORT_PLAN.md), `webpinfo` output, and the
upstream fuzzers as `fuzz/` targets with the oracle under the sanitizers.
No package-level result is a substitute for those gates, and passing a wave is
not "libwebp is ported". [PORT_STATUS.md](PORT_STATUS.md) names the exact
translated units, their evidence and the open gaps.
