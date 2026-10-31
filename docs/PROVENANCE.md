# Provenance

The source revision is recorded in [upstream.json](../upstream.json) and pinned
as the `upstream/libjxl` Git submodule. Its original LICENSE and PATENTS are
copied without changes to the repository root. Ported modules retain the JPEG
XL Project Authors' copyright notice and name the source files they translate.

## Source and test inputs

- `libjxl/libjxl` at `a16d32d0ac113f5e54125e49bb571281c5d0b704`:
  `dec_bit_reader`, `enc_bit_writer`, `field_encodings`, `fields`, `enc_fields`,
  `headers`, the relevant `base/status` semantics, writer storage, alignment,
  and layer accounting. See [PORT_STATUS.md](PORT_STATUS.md) for partial units.
- Upstream `bit_reader_test.cc`, `enc_bit_writer_test.cc`, and `fields_test.cc`:
  fixed cases, test scenarios, and old/new bundle fixtures translated into Rust.
  Generated Rust test sequences use a local deterministic generator rather than
  claiming to reproduce the C++ RNG sequences exactly. Test threads are
  sequential in these adapted core cases; upstream thread-pool behavior is not
  yet translated or verified.
- Upstream `padded_bytes_test.cc`: first-byte initialization scenarios, extended
  with local checks for storage preservation, padding, and alignment.
- `base/bits.h`, `base/byte_order.h`, the endianness definition in
  `lib/include/jxl/types.h`, enum helpers in `field_encodings.h`/`fields.h`,
  `ans_params.h`, and `ans_common.{h,cc}` from the same pinned source: shared
  helpers and ANS alias tables. Native cases are adapted from `bits_test.cc`,
  `byte_order_test.cc`, and `ans_common_test.cc`; the raw enum fixture and
  generated differential distributions are local test infrastructure.
- Highway at `457c891775a7397bdb0376bb1031e6e027af1c48`, selected by upstream's
  submodule: C++ oracle compilation only. Its code remains in that submodule
  with its original LICENSE; it is not compiled into the Rust library.
- `enc_huffman_tree`, `huffman_table`, `enc_huffman`, and `dec_huffman` headers
  and implementations from the same pinned source: tree construction,
  serialization, lookup tables, and prefix decoding. The histogram corpora,
  code-length RLE cases, malformed/truncated streams, and table-reuse fixtures
  are local test infrastructure, not claimed as translated upstream tests.
- `dec_ans`, `dec_context_map`, token/state/config portions of `enc_ans`, the
  scalar specialization in `inverse_mtf-inl.h`, and the used prefetch operation
  from Highway's `hwy/cache_control.h`: entropy streams and supporting storage.
  The uint-config vector round trip adapts `ans_test.cc`; deterministic stream,
  corruption, ring-wrap, and fast-path fixtures are local test infrastructure.
  Original `BuildAndStoreANSEncodingData` and `WriteTokens` generate reference
  fixtures without changing or copying their algorithms into the oracle.
  The oracle generates public export/version headers through CMake using the
  pinned source's template, version settings, and export-target pattern.
  The translated Highway prefetch helper uses Highway's offered BSD-3-Clause
  alternative; its unchanged notice is in
  [licenses/HIGHWAY-BSD-3-Clause.txt](../licenses/HIGHWAY-BSD-3-Clause.txt).
- Histogram storage and ANS strategy in `enc_ans_params.h`, and the anonymous
  `ANSEncodingHistogram` implementation plus table-building/cost entry points
  in `enc_ans.cc`: normalization, fixed-point cost tables, histogram output,
  and reciprocal/reverse encoding maps. Local differential cases cover sparse,
  tied, high-count, empty, oversized, and prefix histograms with all original
  search strategies and both writer-present/absent paths.
- `enc_cluster.{h,cc}`, HistogramParams defaults/strategies and SpeedTier, plus
  scalar `FastLog2f` and its rational polynomial: clustering and encoder cost
  dependencies. The approximation's bounded native test adapts
  `fast_math_test.cc`; bit-pattern and cluster corpora are local differential
  infrastructure. SIMD targets and other fast-math functions remain unported.
- `enc_lz77.{h,cc}`: original cost tables/model, RLE, greedy bucket variants,
  optimal hash-chain variants, and dispatch. The oracle includes the pristine
  `.cc` in its adapter translation unit to observe the private cost helper
  directly. Generated token/cost corpora and the native copy-expansion test are
  local infrastructure. Explicit float-bit regressions retain the AArch64
  reference compiler's fused scalar accumulation and final penalty. The oracle
  preserves upstream's Clang `-fno-slp-vectorize -fno-vectorize` settings from
  `lib/CMakeLists.txt`; omitting them changed this cost model's float rounding.
- Remaining release-state entropy orchestration in `enc_ans.cc`, the scalar
  token-cost specialization in `enc_ans_simd.cc`, scalar MaxValue/vector-size
  support in `simd_util`, and context output in `enc_context_map.cc`: uint
  strategy selection, complete histogram/code output, streaming state, and MTF
  cost decisions. The new oracle adapter calls the original complete encoder
  entry points. Its corpus, cross-decoding checks, and native entropy pipeline
  test are local infrastructure; they are not image-codec acceptance evidence.
- `base/matrix_ops.h`, `cms/opsin_params.h`, `cms/transfer_functions.h`,
  `cms/tone_mapping.h`, and `cms/jxl_cms_internal.h`: profile-generation
  dependencies, original scaling/cube construction, transfer/tone functions,
  descriptions, ICC tags/tables, and profile-ID MD5. The enum masks and external
  color fields originate in `color_encoding_internal.h` and
  `lib/include/jxl/color_encoding.h`. The `%g` mapping follows `base/common.h`
  through Rust formatting, without linking a C formatter. Differential corpora
  are local infrastructure, including parameter ranges from the upstream
  transfer/tone test scenarios; they do not claim to port those SIMD tests.
  Native MD5 known answers and ICC tag-layout checks are local tests.
- `cms/color_encoding_cms.h`, `color_encoding_internal.{h,cc}`,
  `image_metadata.{h,cc}`, the name visitor in `frame_header.h`, the used
  quantizer bias constants, and codestream/metadata writers in `enc_fields.cc`:
  metadata field storage, conversion, visitors, and header output. Native
  coordinate/gamma scenarios adapt `color_encoding_internal_test.cc`; complete
  header, malformed input, custom-weight, and reused-object corpora are local
  infrastructure. These compare original source entry points, without
  substituting test framing for the codestream-header writer.

The differential protocol adapter and corpus generators are project test
infrastructure. Their reference side calls original upstream implementations,
not copied algorithm rewrites. The local `DefaultedBundle` fixture exercises
the upstream `AllDefault`/`SetDefault`/`VisitNested` pattern and is not presented
as a preexisting upstream test case. No external image corpus has been imported
yet.
