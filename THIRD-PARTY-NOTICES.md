# Third-party notices for Ferrisma

This document covers specific material embedded in the `ferrisma` package. It is not a
complete third-party license bundle; generate and review the target-specific
bundle described in [the license audit](docs/license-audit.md) for each
binary distribution. Do not remove upstream license, copyright, or patent
notices from redistributed source.

## JPEG decoder

This software is based in part on the work of the Independent JPEG Group.
The workspace port retains [README.ijg](codecs/libjpeg-turbo-rs/README.ijg)
and [LICENSE.md](codecs/libjpeg-turbo-rs/LICENSE.md).

## CMYK JPEG XL test fixture

`tests/fixtures/jxl-cmyk/cmyk_layers.jxl` is the `cmyk_layers` case of the
public [JPEG XL conformance corpus](https://github.com/libjxl/conformance)
(revision `b1d0f990b03e57bf6d137c365cd5dc8b470b9191`), copied unchanged and
dedicated to the public domain under CC0 (the corpus's `testcases/README.md`
lists the license per case). SHA-256
`d732c8836bf1abeadf310d2e07387a32813ed4690d32650c1c25e541b80eed4a`. The
provenance is in [tests/fixtures/jxl-cmyk/README.md](tests/fixtures/jxl-cmyk/README.md).

## sRGB2014 profile

The unchanged ICC sRGB2014 profile, copyright International Color Consortium,
2015, is embedded when the WebP writer is enabled. The package-local copy is
[assets/icc/sRGB2014.icc](assets/icc/sRGB2014.icc), with its provenance in
[assets/icc/README.md](assets/icc/README.md). Its SHA-256 is
`384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`.
The [ICC sRGB profiles page](https://registry.color.org/rgb-registry/srgbprofiles)
identifies the profile and applies the [ICC profile library terms](https://registry.color.org/profile-library/#licensing):

> This profile is made available by the International Color Consortium, and
> may be copied, distributed, embedded, made, used, and sold without
> restriction. Altered versions of this profile shall have the original
> identification and copyright information removed and shall not be
> misrepresented as the original profile.
