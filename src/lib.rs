//! A mechanical Rust port of libwebp v1.6.0, the WebP reference implementation.
//!
//! The modules mirror the upstream source tree: one module per upstream C file,
//! with the same name, and translated functions carry `// upstream:` references
//! to the C function they translate. See `docs/PORTING.md` for the porting
//! contract and `docs/PORT_STATUS.md` for what has been translated; a module
//! that holds only its provenance header has not been ported yet.
//!
//! Ordinary builds compile and link no C. The C ABI export in [`abi`] is the
//! only module that may contain `unsafe` code (ADR 0004 and ADR 0005). The safe
//! Rust API in [`api`] wraps the translation for Rust callers (ADR 0006).

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod api;
pub mod dec;
pub mod demux;
pub mod dsp;
pub mod enc;
pub mod mux;
pub mod sharpyuv;
pub mod utils;
pub mod webp;

#[allow(
    unsafe_code,
    reason = "ADR 0004: the C ABI export is the only unsafe module"
)]
pub mod abi;
