//! Translation of libwebp's public headers in `src/webp`.
//!
//! The public types, enums and constants of the decoding, encoding,
//! demuxing and muxing APIs, with their original field order.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod decode;
pub mod demux;
pub mod encode;
pub mod format_constants;
pub mod mux;
pub mod mux_types;
pub mod types;
