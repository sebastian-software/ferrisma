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
- Highway at `457c891775a7397bdb0376bb1031e6e027af1c48`, selected by upstream's
  submodule: C++ oracle compilation only. Its code remains in that submodule
  with its original LICENSE; it is not compiled into the Rust library.

The differential protocol adapter and corpus generators are project test
infrastructure. Their reference side calls original upstream implementations,
not copied algorithm rewrites. The local `DefaultedBundle` fixture exercises
the upstream `AllDefault`/`SetDefault`/`VisitNested` pattern and is not presented
as a preexisting upstream test case. No external image corpus has been imported
yet.
