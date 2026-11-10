//! Translation of libwebp's `src/mux` directory.
//!
//! The WebP muxer and the animation encoder.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod anim_encode;
pub mod animi;
pub mod muxedit;
pub mod muxi;
pub mod muxinternal;
pub mod muxread;
