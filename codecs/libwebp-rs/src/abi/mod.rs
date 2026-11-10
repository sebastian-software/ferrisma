//! C ABI export of libwebp's public headers.
//!
//! Each module mirrors one header's structs with `repr(C)`, exports its functions
//! under their C names and converts to the safe representations at the boundary
//! (ADR 0004). This is the only module that may contain `unsafe` code: every
//! unsafe operation sits in its own `unsafe` block with a `// SAFETY:` comment
//! that argues locally why it is sound. The lints below enforce that shape.
//!
//! Each module exports the public header of the same name; its header names the
//! header and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block
)]

pub mod decode;
pub mod demux;
pub mod encode;
pub mod mux;
pub mod types;
