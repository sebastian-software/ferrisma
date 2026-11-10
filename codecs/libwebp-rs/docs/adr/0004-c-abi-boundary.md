# Export the C ABI from one audited module

- Status: accepted
- Created: 2026-11-05
- Updated: 2026-11-05

## Context

The unmodified upstream tools (`dwebp`, `cwebp`, `webpinfo`, `webpmux` and the
animation tools) and the upstream fuzzers are the strongest end-to-end
conformance evidence when they link the Rust library through the original
public headers, as in both sister ports. Exporting a C ABI needs raw pointers
and therefore `unsafe` code, which the rest of the crate denies.

## Decision

`src/abi` exports the public headers, one module per header:
`src/abi/types.rs` (`WebPMalloc`, `WebPFree`), `decode.rs`, `encode.rs`,
`demux.rs` and `mux.rs`, owned by WP12, WP12, WP13, WP14 and WP15. SharpYUV's
header and the internal headers (`thread_utils.h`, `dsp/cpu.h` apart from the
`VP8GetCPUInfo` data symbol) are not exported.

- **Structs.** Public structs without pointers are shared with `src/webp`
  ([ADR 0003](0003-rust-representation.md)). Structs with pointers or callbacks
  get a `#[repr(C)]` mirror with the C field order, types and padding, and are
  converted to and from the safe representation at the boundary. Each package
  verifies size, alignment and field offsets of its mirrors against the C
  compiler with a probe.
- **Functions.** Exported functions keep snake-case Rust names and take their C
  name from `#[unsafe(export_name = "WebPDecodeRGBA")]`. The header's
  `static inline` wrappers (`WebPGetFeatures`, `WebPInitDecoderConfig`,
  `WebPConfigInit` and similar) are not exported; their `...Internal` targets are,
  with the same ABI version checks (`WEBP_DECODER_ABI_VERSION` 0x0210,
  `WEBP_ENCODER_ABI_VERSION` 0x0210, `WEBP_DEMUX_ABI_VERSION` 0x0107,
  `WEBP_MUX_ABI_VERSION` 0x0109).
- **Arguments.** `NULL` and invalid arguments produce the status the C function
  returns for them, checked before any conversion. A pointer and size become a
  slice only after those checks, under the header's documented caller contract.
- **Memory.** Buffers that C callers release with `WebPFree` (decoded images,
  encoded output, mux assemblies) are allocated with the platform `malloc`,
  declared in this module, because upstream's `WebPFree` is `free` and callers
  have long used either. `WebPMalloc` and `WebPFree` export the same pair.
- **Data symbols.** `VP8GetCPUInfo` is a nullable function pointer that the
  upstream tools assign; it is exported as a pointer-sized static with interior
  mutability, never as a `static mut`, and read by the dispatch under ADR 0003.
- **Panics.** A panic must not cross the boundary; since Rust 1.81 a panic
  that reaches an `extern "C"` function aborts the process. ABI functions must
  not panic on any input that the C function defines.

This is the only module where `unsafe` code is allowed (`#[allow(unsafe_code)]`
on `pub mod abi` in `src/lib.rs`). Every unsafe operation sits in its own
`unsafe` block with a `// SAFETY:` comment that argues locally why it is sound;
`src/abi/mod.rs` denies `unsafe_op_in_unsafe_fn`,
`clippy::undocumented_unsafe_blocks` and `clippy::multiple_unsafe_ops_per_block`
to enforce that shape.

The consumer builds (upstream tools and fuzzers linked against the Rust static
library) are added by the integration packages through
`tools/oracle/probes/<wp>/oracle.cmake` ([ADR 0002](0002-differential-oracle.md)).

## Consequences

All `unsafe` code sits in five files with a reviewable argument per operation.
The ABI layer costs a conversion per call, which the safe API (WP16, WP17)
avoids. Linking the platform `malloc` adds no dependency beyond the C library
that Rust's standard library already links.

## Review triggers

Revisit when a header changes with an upstream migration, when a consumer
needs an internal symbol, or when a conversion shows up in measurements.
