//! Translation of libwebp's `src/enc` directory.
//!
//! The VP8 and VP8L encoders, alpha encoding, picture import,
//! conversion, rescaling and distortion, and the encoding entry points.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod alpha_enc;
pub mod analysis_enc;
pub mod backward_references_cost_enc;
pub mod backward_references_enc;
pub mod config_enc;
pub mod cost_enc;
pub mod filter_enc;
pub mod frame_enc;
pub mod histogram_enc;
pub mod iterator_enc;
pub mod near_lossless_enc;
pub mod picture_csp_enc;
pub mod picture_enc;
pub mod picture_psnr_enc;
pub mod picture_rescale_enc;
pub mod picture_tools_enc;
pub mod predictor_enc;
pub mod quant_enc;
pub mod syntax_enc;
pub mod token_enc;
pub mod tree_enc;
pub mod vp8i_enc;
pub mod vp8l_enc;
pub mod vp8li_enc;
pub mod webp_enc;
