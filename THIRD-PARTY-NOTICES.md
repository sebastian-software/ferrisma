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
