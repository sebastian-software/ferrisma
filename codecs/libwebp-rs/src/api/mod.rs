//! Safe Rust API over the translation, separate from the C ABI in [`crate::abi`].
//!
//! [`encode`] wraps the encoder (WP17, `docs/adr/0006-safe-encoder-api.md`) and
//! [`decode`] the decoder (WP16). The API validates its input before it calls
//! the translation, returns `Result` errors and adds no codec behavior: an
//! encoding is byte-identical to the translated `WebPEncode` with the same
//! configuration. Each module's header names the work package that owns it.
//! WP17 added this file and the module list in `tools/units.json`; work
//! packages do not edit it.

pub mod decode;
pub mod encode;
