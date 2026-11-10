//! Translation of libwebp's `src/demux` directory.
//!
//! The WebP demuxer and the animation decoder.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod anim_decode;
#[expect(clippy::module_inception, reason = "keeps the upstream file name")]
pub mod demux;
