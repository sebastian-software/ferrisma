//! A mechanical Rust port of the JPEG decoder in libjpeg-turbo 3.2.1.
//!
//! One module per upstream file, named after it, as `tools/units.json` maps
//! them; each module starts with the upstream file's leading comment block.
//! Work packages 1 (common infrastructure: types, errors and messages, the
//! memory manager, utilities), 2 (input and markers, up to
//! `jpeg_read_header`), 3 (entropy decoding) and 4 (coefficients and IDCT)
//! are translated; the other modules hold only their provenance headers. See
//! `docs/PORT_PLAN.md` for the scope and the work
//! packages, `docs/PORT_STATUS.md` for what is translated and verified, and
//! `docs/adr/` for the decisions that bind them.
//!
//! This software is based in part on the work of the Independent JPEG Group.

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod jaricom;
pub mod jcomapi;
pub mod jconfig;
pub mod jdapimin;
pub mod jdapistd;
pub mod jdarith;
pub mod jdatasrc;
pub mod jdcoefct;
pub mod jdcolext;
pub mod jdcolor;
pub mod jdct;
pub mod jddctmgr;
pub mod jdhuff;
pub mod jdicc;
pub mod jdinput;
pub mod jdmainct;
pub mod jdmarker;
pub mod jdmaster;
pub mod jdmerge;
pub mod jdmrgext;
pub mod jdphuff;
pub mod jdpostct;
pub mod jdsample;
pub mod jerror;
pub mod jidctflt;
pub mod jidctfst;
pub mod jidctint;
pub mod jidctred;
pub mod jmemmgr;
pub mod jmemnobs;
pub mod jmemsys;
pub mod jmorecfg;
pub mod jpegint;
pub mod jpeglib;
pub mod jstdhuff;
pub mod jutils;
pub mod jversion;
