//! Compatibility adapter for Ferrisma's workspace libavif/AOM encoder.
//!
//! The safe backend is also available directly through the engine's
//! `avif-own-write` feature. The libavif ABI is isolated in the independent
//! `ferrisma-avif-write` crate (ADR-0037).

pub use ferrisma::pipeline::LibavifBackend;
