# Native Library License Inventory

This is a source inventory for port planning, checked on 2026-11-01. It records
the baseline versions used by libavif-rs; it does not select revisions for new
ports or establish a licensing policy for the workspace.

The table records primary project licenses. It is not an exhaustive license
expression for a future crate: individual files, vendored code, tests, and
transitive dependencies can carry additional terms.

| Library and baseline | Primary license | Versioned upstream source |
| --- | --- | --- |
| libavif 1.4.2, `9b07a9d7a874b159bb4e14b80a6f08bb469d06f7` | BSD-2-Clause for the library core | [LICENSE](https://github.com/AOMediaCodec/libavif/blob/9b07a9d7a874b159bb4e14b80a6f08bb469d06f7/LICENSE) |
| libaom 3.15.2 | BSD-2-Clause | [LICENSE](https://aomedia.googlesource.com/aom/+/refs/tags/v3.15.2/LICENSE), [PATENTS](https://aomedia.googlesource.com/aom/+/refs/tags/v3.15.2/PATENTS) |
| dav1d 1.5.4 | BSD-2-Clause | [COPYING](https://github.com/videolan/dav1d/blob/1.5.4/COPYING), [doc/PATENTS](https://github.com/videolan/dav1d/blob/1.5.4/doc/PATENTS) |
| SharpYUV in libwebp 1.6.0 | BSD-3-Clause | [COPYING](https://github.com/webmproject/libwebp/blob/v1.6.0/COPYING), [PATENTS](https://github.com/webmproject/libwebp/blob/v1.6.0/PATENTS), [source notice](https://github.com/webmproject/libwebp/blob/v1.6.0/sharpyuv/sharpyuv.c) |
| libgav1 v0.20.0, `c05bf9be660cf170d7c26bd06bb42b3322180e58` | Apache-2.0 | [LICENSE](https://chromium.googlesource.com/codecs/libgav1/+/c05bf9be660cf170d7c26bd06bb42b3322180e58/LICENSE) |
| libyuv `26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0` | BSD-3-Clause | [LICENSE](https://chromium.googlesource.com/libyuv/libyuv/+/26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0/LICENSE), [PATENTS](https://chromium.googlesource.com/libyuv/libyuv/+/26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0/PATENTS) |
| rav1e 0.8.1, already Rust | BSD-2-Clause | [LICENSE](https://github.com/xiph/rav1e/blob/v0.8.1/LICENSE) |
| SVT-AV1 4.2.0 | BSD-3-Clause-Clear at the project level; mixed file notices | [LICENSE.md](https://gitlab.com/AOMediaCodec/SVT-AV1/-/raw/v4.2.0/LICENSE.md), [PATENTS.md](https://gitlab.com/AOMediaCodec/SVT-AV1/-/raw/v4.2.0/PATENTS.md), [API header](https://gitlab.com/AOMediaCodec/SVT-AV1/-/raw/v4.2.0/Source/API/EbSvtAv1.h) |
| AVM v1.0.0, `966a7d7cd6fcf60360caf5dc413b2aeeb65e144d` | BSD-3-Clause-Clear | [LICENSE](https://github.com/AOMediaCodec/avm/blob/v1.0.0/LICENSE), [PATENTS](https://github.com/AOMediaCodec/avm/blob/v1.0.0/PATENTS) |

## Companion terms and source exceptions

libaom, dav1d, SVT-AV1, and AVM distribute separate AOM patent-license texts.
SharpYUV/libwebp and libyuv distribute separate Google patent grants.
BSD-3-Clause-Clear is distinct from BSD-3-Clause and explicitly excludes patent
grants from the copyright license; the accompanying patent text remains a
separate source of terms.

The top-level labels do not describe every file. libaom includes BSD-3-Clause,
MIT, and ISC third-party material. dav1d's
[x86inc assembly](https://github.com/videolan/dav1d/blob/1.5.4/src/ext/x86/x86inc.asm)
uses ISC. SVT-AV1 4.2.0 retains active BSD-2-Clause source notices as well as
MIT, ISC, and BSD-3-Clause material. Its alternate BSD-2-Clause license file
cannot simply be dismissed as irrelevant history.

The existing libavif snapshot also contains additional terms for bundled
libyuv, IJG image metadata helpers, and Android build tooling, as its full
[LICENSE](../LICENSE) records. The core library's BSD-2-Clause designation does
not make the entire source snapshot exclusively BSD-2-Clause.

## Configuration-dependent AVM scope

The current AVM reference configuration enables ML experiments that pull
TensorFlow Lite and support libraries. Its
[experiment configuration](https://github.com/AOMediaCodec/avm/blob/v1.0.0/cmake/avm_experiment_deps.cmake)
forces TensorFlow Lite on when the relevant ML features are enabled. The actual
generated build graph and linked library confirm this dependency.

The downloaded TensorFlow 2.17.0 source uses
[Apache-2.0](https://github.com/tensorflow/tensorflow/blob/v2.17.0/LICENSE).
The prepared Eigen source's `COPYING.README` and core matrix header identify
MPL-2.0 as its primary source license. Thus the full configured AVM dependency
closure extends beyond BSD-family licenses. The exact transitive revisions and
imported files need their own inventory if this configuration enters a port
milestone.

## Selected policy and standardization

The listed primary licenses are accepted. Preserve each imported file's
attribution and applicable terms, together with its license, notice, and patent
files. Future crate metadata should reflect the files actually distributed
rather than blindly inheriting the current root package's BSD-2-Clause field.
See [ADR 0014](adr/0014-retain-upstream-license-terms-for-ported-crates.md).

Use a common license-file layout, a consistent provenance manifest, and
generated per-crate attribution summaries. This standardizes presentation
without claiming all source material has one new license. See
[ADR 0017](adr/0017-standardize-license-records-and-generated-notices.md).
Any selected license for original project-owned additions is a separate
decision from the retained upstream conditions.

[Apache-2.0 section 4](https://www.apache.org/licenses/LICENSE-2.0) allows
different terms for modifications or a derivative work as a whole only while
the original conditions remain satisfied; its license, modification, retained
notice, and applicable `NOTICE` requirements still matter. A workspace-wide
replacement license does not follow automatically from combining permissive
projects.

[SPDX expressions](https://spdx.github.io/spdx-spec/v2.3.1/SPDX-license-expressions/)
distinguish an actual licensing choice (`OR`) from simultaneously applicable
terms (`AND`). A mixed tree is not automatically dual-licensed, and a crate's
own packaged files should be distinguished from separately linked dependencies.
Additional transitive scope and any broader relicensing proposal are outside
the first portable default-pipeline phase. They require separate evaluation
before that scope is implemented or distributed.
