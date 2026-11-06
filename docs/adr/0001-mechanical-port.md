# Port the libjpeg-turbo decoder mechanically, 8-bit first

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-06

## Context

Ferrixel needs a JPEG decoder that is fast, correct by reference lineage and
able to decode at reduced scale for shrink-on-load. After a measurement, its
maintainer chose libjpeg-turbo over jpegli and over a `zune-jpeg` fork
([Ferrixel ADR-0025](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md)).
Ferrixel's ports cover the still-image scope that Ferrixel uses; code outside
it is not translated, because every translated line must later follow
upstream changes. The sister ports (libavif-rs, libjxl-rs, libwebp-rs) first
translate upstream behavior exactly and optimize afterwards against that
translation as an oracle.

## Decision

**Pin.** `upstream/libjpeg-turbo` pins tag `3.2.1` at
`7b4993786d856cbccabfc0ed2f923ba000e326cc`. Advance it only as a separate,
reviewed migration with renewed parity evidence.

**Translate mechanically.** Preserve algorithms, tables, constants, numeric
widths, operation order, accepted input, error and warning codes, and partial
state on error. Do not redesign, tune or add optimizations while translating.
One Rust module per upstream C file, with the C file's name (`src/jdhuff.c`
becomes `src/jdhuff.rs`). Every translated item names its origin
(`// upstream: src/jdhuff.c decode_mcu`).

**Scope: the 8-bit decompression path of the libjpeg API.**

| In scope | Upstream units |
| --- | --- |
| Common infrastructure | `jerror.c`, `jcomapi.c`, `jutils.c`, `jmemmgr.c`, `jmemnobs.c`, the public and internal types of `jpeglib.h`, `jpegint.h`, `jmorecfg.h`, `jerror.h` |
| Input and markers | `jdapimin.c`, `jdatasrc.c` (memory source), `jdinput.c`, `jdmarker.c`, `jdicc.c` |
| Entropy decoding | `jdhuff.c`, `jdphuff.c` (progressive), `jaricom.c` and `jdarith.c` (arithmetic coding) |
| Coefficients and IDCT | `jdcoefct.c`, `jddctmgr.c`, `jidctint.c`, `jidctfst.c`, `jidctflt.c`, `jidctred.c` (reduced-size IDCT for scaled decoding) |
| Upsampling and color | `jdsample.c`, `jdmerge.c` with `jdmrgext.c`, `jdcolor.c` with `jdcolext.c`, `jdpostct.c`, `jdmainct.c` |
| Control and API | `jdmaster.c`, `jdapistd.c` (including `jpeg_crop_scanline` and `jpeg_skip_scanlines`) |

All of them as the 8-bit instantiation that upstream builds through
`src/wrapper/*-8.c` (`BITS_IN_JSAMPLE == 8`). Arithmetic decoding stays in
because upstream's default build includes it and a decoder that rejects valid
baseline-family files is incomplete for Ferrixel's input.

| Out of scope (not translated) | Reason |
| --- | --- |
| The encoder (`jc*.c`, `jf*.c`, `jdatadst.c`, `jpeg_nbits.c`) and transcoding (`jctrans.c`, `jdtrans.c`) | Ferrixel does not write JPEG |
| 12-bit and 16-bit instantiations, lossless JPEG (`jdlossls.c`, `jdlhuff.c`, `jddiffct.c`) | Rare in photographs; revisit when a consumer needs them |
| Color quantization (`jquant1.c`, `jquant2.c`) and RGB565 output (`jdcol565.c`, `jdmrg565.c`) | Palette and 16-bit-per-pixel output are not used |
| The TurboJPEG API (`turbojpeg.c`) and the stdio source `jpeg_stdio_src` | Ferrixel uses the libjpeg API from memory |
| Assembly and intrinsics | Ported afterwards under [ADR 0004](0004-simd-after-the-portable-port.md) |

The upstream programs (`djpeg`), the test images and the fuzzers are
consumers and test inputs, not translation units
([ADR 0002](0002-differential-oracle.md)). Unported functions stay absent: no
stubs, placeholders, partial functions or C fallbacks.

## Consequences

The decoder keeps upstream's structure, which makes the later diff of a new
upstream version against the old one directly applicable. The libjpeg API's
object-oriented C (method tables of function pointers, a pool allocator,
`setjmp`/`longjmp`-style error exits) needs an explicit Rust representation,
decided before translation starts
([PORT_PLAN.md](../PORT_PLAN.md), work package 0). Leaving out 12-bit and
lossless decoding means some valid JPEG files are rejected; the decoder reports
them as unsupported instead of misdecoding them.

## Review triggers

Revisit when a consumer needs 12-bit, lossless or quantized output, when the
upstream pin advances, or when a source construct has no sound mechanical Rust
representation.
