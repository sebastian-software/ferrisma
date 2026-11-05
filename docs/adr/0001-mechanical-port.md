# Preserve pinned upstream behavior before redesign

- Status: accepted
- Created: 2026-11-05
- Updated: 2026-11-06

## Context

libwebp-rs translates libwebp v1.6.0 so that Ferrixel can read and write WebP
without a C dependency. The translation runs as parallel work packages
([WORKPLAN.md](../WORKPLAN.md)), each verified against the original C code.
Combining the port with algorithm, API or performance changes would make
discrepancies hard to locate, and parallel sessions need rules that let them
name and place code identically without coordinating.

## Decision

Translate the pinned source mechanically. Preserve algorithms, tables,
constants, numeric widths, operation order, accepted input, status codes,
partial state on error, callbacks, threading behavior and the existing portable
optimizations. Do not redesign, tune or add optimizations. SIMD variants arrive
only in wave 4 under [ADR 0003](0003-rust-representation.md) and
[ADR 0005](0005-toolchain-and-unsafe-policy.md).

The `upstream/libwebp` submodule pins tag `v1.6.0` at
`4fa21912338357f89e4fd51cf2368325b59e9bd9`. Advance it only as a separate,
reviewed migration with renewed parity evidence.

**Module correspondence.** One Rust module per upstream C file, with the C
name: `src/dec/vp8_dec.c` becomes `src/dec/vp8_dec.rs`. A `.c` file and the
header of the same name share one module. A header without a `.c` file of the
same name that defines types, constants or inline functions gets its own
module (`src/utils/bit_reader_inl_utils.h` becomes
`src/utils/bit_reader_inl_utils.rs`). `tools/units.json` is the authoritative
map from upstream files to modules and owning packages. Where libwebp's call or
type graph forms a cycle across packages, it assigns single functions or types
inside a module to the package that needs them first; that package appends
exactly those items and every later owner keeps them. WP0 created every
module file with its provenance header (the upstream file's leading comment
block, verbatim, and a module doc comment naming the files and the package);
packages append below it and never edit `mod.rs` or `lib.rs`. `tools/units.py`
checks the map, the headers and the declarations.

**Source references.** Every translated function, type and table carries a
comment naming its origin, for example `// upstream: src/dec/vp8_dec.c
VP8GetHeaders`. Translated modules keep Google's copyright notice.

**Names.** Rust naming conventions apply, through a deterministic mapping so
that two packages that never coordinate produce the same names.
`tools/rust_name.py` implements it.

- Functions, including `static` helpers and function-like macros, use the
  snake case of the C name. "WebP" counts as one word; a word boundary falls
  before an uppercase letter that follows a lowercase letter, and before an
  uppercase letter that follows an uppercase letter or digit when a lowercase
  letter comes next; digits stay with the preceding word; underscores separate
  words. Examples: `VP8GetHeaders` → `vp8_get_headers`, `VP8LDecodeImage` →
  `vp8l_decode_image`, `WebPSafeMalloc` → `webp_safe_malloc`,
  `WebPIDecGetRGB` → `webp_i_dec_get_rgb`, `VP8GetCPUInfo` →
  `vp8_get_cpu_info`, `TransformWHT_C` → `transform_wht_c`, `GetLE16` →
  `get_le16`, `SharpYuvConvert` → `sharp_yuv_convert`. A result that is a Rust
  keyword uses a raw identifier.
- Types (structs, enums, typedefs) keep their C names (`VP8Decoder`,
  `WebPDecBuffer`, `VP8StatusCode`). A C type name in upper snake case
  (`WEBP_CSP_MODE`, `WEBP_FILTER_TYPE`) keeps its name with an item-level
  `#[expect(non_camel_case_types, reason = "C type name")]`.
- Enum values, object-like macros and constants keep their upper-snake C names
  (`VP8_STATUS_OK`, `MODE_RGBA`, `NUM_MB_SEGMENTS`). A `k`-prefixed constant or
  table drops the `k` and takes the upper snake case of the rest:
  `kZigzag` → `ZIGZAG`, `kYModesIntra4` → `Y_MODES_INTRA4`. Other lower-case
  globals take the upper snake case of their name.
- Struct fields keep their C names; a field that is not snake case takes the
  function rule.

**Scope.** The libraries under `src/` and `sharpyuv/` are in scope. The
upstream tools (`examples/`), `imageio/`, `extras/`, `tests/` and the fuzzers are
consumers and test inputs, not translation units; they link the Rust library
through the original headers ([ADR 0004](0004-c-abi-boundary.md)). MIPS and MSA
variants are excluded and listed in `tools/units.json`.

Unported functions stay absent: no stubs, placeholders, partial functions or C
fallbacks. Record ambiguous or undefined source behavior instead of silently
defining it ([PORTING.md](../PORTING.md)).

## Consequences

Source and target stay easy to compare, function by function, at the cost of
a less idiomatic internal API; the safe public API is a later, separate layer
(WP16, WP17). The fixed module tree removes shared-file conflicts between
packages, but it cannot remove call dependencies between them; WORKPLAN.md
records those.

## Review triggers

Revisit after the decoder and encoder pass their full gates, when a source
construct has no sound mechanical Rust representation, or when the upstream
pin advances.
