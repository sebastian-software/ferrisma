//! Translation of libwebp's `src/dec` directory.
//!
//! The VP8 and VP8L decoders, alpha decoding, output buffers, the
//! incremental decoder and the decoding entry points.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod alpha_dec;
pub mod alphai_dec;
pub mod buffer_dec;
pub mod common_dec;
pub mod frame_dec;
pub mod idec_dec;
pub mod io_dec;
pub mod quant_dec;
pub mod tree_dec;
pub mod vp8_dec;
pub mod vp8i_dec;
pub mod vp8l_dec;
pub mod vp8li_dec;
pub mod webp_dec;
pub mod webpi_dec;
