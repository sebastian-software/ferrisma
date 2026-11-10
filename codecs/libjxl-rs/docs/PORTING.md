# Porting contract

Phase one is a mechanical Rust translation of both the encoder and decoder in
libjxl. The pinned source is the `upstream/libjxl` Git submodule. The initial
revision is `a16d32d0ac113f5e54125e49bb571281c5d0b704`; its dependencies are
pinned by its own submodule entries. Keep its BSD license and additional IP
rights grant with the derived source.

## Preserved behavior

Preserve algorithms, tables, constants, numeric precision, operation order,
serialized fields, accepted input, returned status codes, streaming state,
metadata, callbacks, and cleanup obligations at each translated boundary.
Existing optimizations are part of the source behavior. Do not introduce new
ones, refactor algorithms, or replace third-party implementations during the
translation.

Ultimately compare image samples, metadata, frame sequences, reconstructed JPEG
bytes, API transitions, and encoder bitstreams under matching options and a
fixed reference configuration. Bit-exact results across different CPU targets
are not assumed. Timing, allocation addresses, and machine code are not expected
to be identical; measure performance and memory separately before release.

The eventual scope includes the public C API, thread runners, and core codec.
Tools, examples, plugins, third-party libraries, and test infrastructure have
separate boundaries and are not implicitly counted as translated. A fully Rust
dependency stack requires additional work beyond the libjxl core. Track exact
completed units and missing functionality in [PORT_STATUS.md](PORT_STATUS.md).

## Translation rules

1. Start from the pinned source, not a fresh implementation of the format.
2. Link each module to its source and preserve copyright notices.
3. Keep integer widths, signedness, truncation, and wrapping deliberate.
4. Preserve partially completed state and status propagation on error.
5. Keep ownership and lifetime changes explicit; see the
   [representation ADR](adr/0003-rust-representation.md).
6. Compare source and target independently after each batch. An internal Rust
   round trip alone cannot prove equivalence.
7. Keep unported APIs absent. Do not add placeholders or C++ runtime fallbacks.
8. Record ambiguous or undefined source behavior instead of silently defining a
   new product behavior.

## Verification gates

The first slice must prove header serialization and parsing against the actual
C++ implementation, including an unaligned bit position, truncated input,
padding, and the reader's close lifecycle. Verify debug and release builds.

Subsequent batches pass formatting, compilation, focused upstream-derived tests,
and differential cases before integration. The full port additionally requires
the complete test inventory, image-level differential and conformance suites,
supported platform/CPU configurations, applicable sanitizer and fuzzing gates,
and measured performance/memory evidence. No slice-level result is a substitute
for those full-codec gates.
