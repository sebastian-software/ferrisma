# Embedded ICC profile

`sRGB2014.icc` is the unchanged ICC sRGB2014 (v2) profile, copyright
International Color Consortium, 2015. The `webp-write` feature embeds it in
WebP output to describe the emitted sRGB8 samples (ADR-0030).

- SHA-256: `384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`
- Copied byte-for-byte from the pinned libavif snapshot,
  `codecs/libavif-rs/upstream/libavif/tests/data/sRGB2014.icc` at libavif
  revision `9b07a9d7a874b159bb4e14b80a6f08bb469d06f7`, whose
  [test data README](../../codecs/libavif-rs/upstream/libavif/tests/data/README.md)
  attributes it to the International Color Consortium and names the
  [ICC sRGB profiles page](https://www.color.org/srgbprofiles.xalter#v2) as
  its source.
- Terms: the [ICC profile library terms](https://registry.color.org/profile-library/#licensing)
  permit copying, distribution, and embedding without restriction; altered
  versions must not be presented as the original profile. See
  [THIRD-PARTY-NOTICES.md](../../THIRD-PARTY-NOTICES.md).

The copy exists because the published `ferrisma` crate cannot include files
from the nested `libavif-rs` package directory. Do not modify this file. If
the upstream snapshot changes, compare both SHA-256 values before replacing it.
The profile is not covered by Ferrisma's MIT OR Apache-2.0 license or by the
libwebp BSD license.
