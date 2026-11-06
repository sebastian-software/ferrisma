//! A mechanical Rust port of the JPEG decoder in libjpeg-turbo 3.2.1.
//!
//! Bootstrap only: no upstream unit has been translated yet. See
//! `docs/PORT_PLAN.md` for the scope and the work packages, and
//! `docs/adr/` for the decisions that bind them.
//!
//! This software is based in part on the work of the Independent JPEG Group.

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
