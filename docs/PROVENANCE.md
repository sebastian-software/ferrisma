# Provenance

The source revision is recorded in [upstream.json](../upstream.json) and pinned
as the `upstream/libjxl` Git submodule. Its original LICENSE and PATENTS are
copied without changes to the repository root. Ported modules retain the JPEG
XL Project Authors' copyright notice and name the source files they translate.

## Initial inputs, 2026-10-31

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

The differential protocol adapter and corpus generators are project test
infrastructure. Their reference side calls original upstream implementations,
not copied algorithm rewrites. The local `DefaultedBundle` fixture exercises
the upstream `AllDefault`/`SetDefault`/`VisitNested` pattern and is not presented
as a preexisting upstream test case. No external image corpus has been imported
yet.
