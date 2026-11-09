# Offer a safe row-streaming decoder over the translated API

- Status: accepted
- Created: 2026-11-08
- Updated: 2026-11-09

## Context

Work packages 1 to 6 translate the libjpeg API as it is: a decompression
object with public fields, functions that return `Result<_, ErrorExit>`, an
error manager whose default prints to standard error, and row lists for the
output ([ADR 0005](0005-rust-representation.md)). A Rust program can drive it,
as the WP7 `djpeg` driver does, but it has to know the call order, set fields
at the right time and read the error code out of `cinfo.err`. ADR 0005 leaves
two questions to work package 8: what the safe API re-exports, and the error
manager it installs.

Ferrixel's `jpeg-read` adapter (`src/codecs/jpeg.rs`) is the first user. It
reads the header, the ICC profile and the EXIF orientation when it opens a
file, and later emits RGB rows of a region to a sink, with a memory budget.
With `zune-jpeg` it decodes the full frame for every region. Shrink-on-load
needs libjpeg's scaled IDCT
([Ferrixel ADR-0023](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0023-decode-side-work-and-scaling-policy.md)).

## Decision

1. **An own module, re-exported at the crate root.** `src/decode.rs` holds the
   safe API, and `src/lib.rs` re-exports its types. It translates no upstream
   file, so [tools/units.json](../../tools/units.json) lists it under
   `own_modules`. The translated modules stay public, for the tests and for
   callers that need what the safe API omits.
2. **Two types, in the order of the libjpeg API.**
   - `Decoder::new` (or `with_options`) creates the object, attaches the data
     with `jpeg_mem_src`, saves APP1 and APP2 markers and reads the header.
     It offers the header, the EXIF data, the ICC profile and
     `output_info`, which runs `jpeg_calc_output_dimensions`.
   - `Decoder::start` sets the output parameters and calls
     `jpeg_start_decompress`. It returns `Scanlines`, which crops columns
     (`jpeg_crop_scanline`), skips rows (`jpeg_skip_scanlines`), reads rows
     into a caller buffer (`jpeg_read_scanlines`) and finishes
     (`jpeg_finish_decompress`).
   - Dropping either type calls `jpeg_destroy_decompress`. A region decode
     drops `Scanlines` after its last row.
3. **Options are libjpeg's parameters.** `DecoderOptions` sets
   `max_memory_to_use`, which overrides `JPEGMEM` as ADR 0005 decision 2
   expects, and `fail_on_warning`. `OutputOptions` sets `out_color_space`,
   `scale_num` and `scale_denom`, `dct_method`, `do_fancy_upsampling` and
   `do_block_smoothing`, with libjpeg's defaults. Only in-scope values can be
   expressed. The output formats are the 8-bit color spaces except RGB565.
   Raw data, buffered-image mode, color quantization and suspending sources
   are not offered.
4. **An error manager that records.** It replaces every default that prints:
   `error_exit` keeps the message, and `emit_message` counts and keeps
   warnings, the first 64 of them. Trace messages are dropped. A libjpeg error
   becomes `Error::Library` with upstream's code and formatted text. With
   `fail_on_warning`, `emit_message` stops the decode at the first warning
   with `Error::Warning`, as an application can stop it with `longjmp`.
   Arguments of the safe API that libjpeg would misread fail with
   `Error::InvalidArgument` before any call. Out-of-scope files fail with
   upstream's codes where libjpeg reaches them, for example
   `JERR_BAD_PRECISION` at the start of a 12-bit file. The safe API does not
   check them earlier.
5. **Metadata as libjpeg delivers it.** `icc_profile` is
   `jpeg_read_icc_profile`. `exif` returns the TIFF data of the first APP1
   segment that starts with `Exif\0\0`. Parsing EXIF, the orientation for
   example, is the caller's job: it is not part of libjpeg.

## Consequences

- Ferrixel's adapter can emit rows as they are decoded, skip the rows above
  a region, crop to its iMCU-aligned columns and request a scaled decode,
  without `unsafe` and without the translated API.
- Every call of the safe API is an upstream call in upstream's order, so the
  WP6 and WP2 probes can check it against the C oracle with the same options
  and plan.
- `max_memory_to_use` limits only the whole-image buffers of progressive and
  buffered decoding, as upstream's memory manager does. The other
  allocations are not capped, and the adapter has to account for them.
- `Decoder` and `Scanlines` are not `Send`: the decompression object holds
  boxed method objects, and the error manager shares its log through an
  `Rc`. A decode runs on the thread that started it.
- Warnings of a failed `start` or `finish` are lost with the decoder; the
  error itself is kept.
- A `read_rows` call that fails partway returns the error, not the number of
  rows it wrote before it. A caller that needs every good row of a damaged
  file reads one row per call.

## Alternatives considered

- **A separate crate for the safe API.** Not needed: the safe API adds no
  `unsafe` and no dependency, and one crate keeps Ferrixel's dependency
  simple. A separate crate stays the place for a test C interface
  ([ADR 0003](0003-toolchain-and-unsafe-policy.md)).
- **A one-call full-frame decode.** It would materialize the whole image,
  which is what Ferrixel wants to avoid. A caller can build it from
  `Scanlines`.
- **A row iterator.** An iterator would allocate a row per item or lend rows
  with lifetimes tied to the decoder. A caller buffer of whole rows maps
  directly onto `jpeg_read_scanlines`.
- **Parsing the EXIF orientation here.** It is not upstream code, and
  Ferrixel already has a parser for it.

## Validation

`tests/wp8_decode.rs` runs each decode through the safe API and through the
oracle's WP6 probe, with the same options and plan, and compares output
dimensions, crops, a hash of every row, warnings and errors. It compares
metadata with the WP2 probe, and the memory limit with the oracle's
`JPEGMEM`. The record is falsified if the safe API decodes any in-scope input
differently from the oracle under the same parameters, or if Ferrixel's
adapter needs a libjpeg call that the safe API does not offer.

## Review when

- Ferrixel's adapter adopts the decoder
  ([ferrixel#14](https://github.com/sebastian-software/ferrixel/issues/14))
  and needs a call the API lacks.
- A caller needs `Send` decoders or a suspending source.
- The SIMD packages add options, for example a choice of kernels.
