//! Translation of libwebp's `src/utils` directory.
//!
//! Bit readers and writers, entropy coding helpers, the allocation
//! helpers, the worker threads and other shared utilities.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod bit_reader_inl_utils;
pub mod bit_reader_utils;
pub mod bit_writer_utils;
pub mod color_cache_utils;
pub mod endian_inl_utils;
pub mod filters_utils;
pub mod huffman_encode_utils;
pub mod huffman_utils;
pub mod palette;
pub mod quant_levels_dec_utils;
pub mod quant_levels_utils;
pub mod random_utils;
pub mod rescaler_utils;
pub mod thread_utils;
#[expect(clippy::module_inception, reason = "keeps the upstream file name")]
pub mod utils;
