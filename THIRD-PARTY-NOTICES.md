# Third-party notices for Ferrixel

This document covers specific material embedded in Ferrixel. It is not a
complete third-party license bundle; generate and review the target-specific
bundle described in [the license audit](docs/license-audit.md) for each
binary distribution. Do not remove upstream license, copyright, or patent
notices from redistributed source.

## JPEG decoder

This software is based in part on the work of the Independent JPEG Group.
The workspace port retains [README.ijg](codecs/libjpeg-turbo-rs/README.ijg)
and [LICENSE.md](codecs/libjpeg-turbo-rs/LICENSE.md).

## AV1 encoder tuning

`src/av1_tuning.rs` adapts code from ravif 0.13.0. Its notice is:

```text
BSD 3-Clause License

Copyright (c) 2020, Kornel
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## sRGB2014 profile

The unchanged ICC sRGB2014 profile, copyright International Color Consortium,
2015, is embedded when the WebP writer is enabled. Its SHA-256 is
`384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`.
The [ICC sRGB profiles page](https://registry.color.org/rgb-registry/srgbprofiles)
identifies the profile and applies the [ICC profile library terms](https://registry.color.org/profile-library/#licensing):

> This profile is made available by the International Color Consortium, and
> may be copied, distributed, embedded, made, used, and sold without
> restriction. Altered versions of this profile shall have the original
> identification and copyright information removed and shall not be
> misrepresented as the original profile.
