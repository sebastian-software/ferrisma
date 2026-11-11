//! Optional read-only codec adapters; no codec depends on the graph itself.
#[cfg(feature = "jpeg-read")]
pub(crate) mod jpeg;
#[cfg(feature = "jpeg-xl-own-read")]
pub(crate) mod jxl;
#[cfg(feature = "jpeg-xl-read")]
pub(crate) mod jxl_oxide;

#[cfg(feature = "avif-read")]
pub(crate) mod avif;
#[cfg(feature = "avif-read")]
mod avif_meta;
#[cfg(feature = "webp-read")]
pub(crate) mod webp;
