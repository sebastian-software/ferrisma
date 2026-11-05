# Provenance

libwebp-rs is derived from libwebp, the WebP reference implementation by Google
and the WebM project contributors.

## Source and test inputs

| Input | Pin | Path |
| --- | --- | --- |
| libwebp, <https://chromium.googlesource.com/webm/libwebp> | tag `v1.6.0`, commit `4fa21912338357f89e4fd51cf2368325b59e9bd9` (2025-06-30) | `upstream/libwebp` |
| libwebp-test-data, <https://chromium.googlesource.com/webm/libwebp-test-data> | commit `260804d686f0491e0326a50eef491829dae09e9e` (2022-01-12) | `upstream/libwebp-test-data` |

Both are Git submodules, recorded in [upstream.json](../upstream.json).
`tools/verify_upstream.py`, which `tools/check.py` runs first, checks that the
index and the checkouts sit at those commits, that the `v1.6.0` tag points at
the libwebp commit when it is fetched, and that neither checkout has modified,
staged or untracked files. The same libwebp revision is the one libavif-rs
records for its SharpYUV translation.

The test data is pinned at the last revision whose references match libwebp
v1.6.0. The next two commits, `05cd58b` and `06ddd96` (26 August 2025), add no
vectors; they regenerate seven PGM checksums in `libwebp_tests.md5` and the
`grid.pgm`, `peak.pgm` and `lossless_color_transform.pgm` references for libwebp
`0d14d84b`, a change made after v1.6.0. WP0 first pinned `06ddd96` and moved the
pin back for that reason ([WORKPLAN.md](WORKPLAN.md#findings-from-wp0)).

RFC 6386 (VP8) and RFC 9649 (WebP container and VP8L) are reading aids only;
the pinned source is the authority.

## License and patent grant

libwebp is distributed under a BSD-3-Clause license in its `COPYING` file, with
an additional patent grant from the WebM project in `PATENTS`. The repository
root holds unchanged copies: `LICENSE` is upstream `COPYING`, and `PATENTS` and
`AUTHORS` are upstream `PATENTS` and `AUTHORS`. `AUTHORS` is copied because the
notices in every source file refer to it. `tools/verify_upstream.py` checks the
three copies byte for byte. The crate's license is `BSD-3-Clause`; the patent
grant applies to the derived work as the `PATENTS` text describes.

Every module file starts with the leading comment block of each upstream file
it translates, copied verbatim and labeled with the file's path: Google's
copyright notice, the license reference and, where upstream has them, the
file's description and author line. `tools/units.py` checks that header
against the pinned source. The license reference in those notices names the
`COPYING` file, which is `LICENSE` here.

## Original material

The following files are original to libwebp-rs and contain no upstream code:
the documentation under `docs/`, the tools under `tools/` including
`tools/oracle/CMakeLists.txt` and the probe sources, the test helpers under
`tests/`, the CI workflows, and `src/lib.rs` and the `mod.rs` files. Probes call
the pinned upstream library; they do not copy its algorithms.

The plan documents [WORKPLAN.md](WORKPLAN.md) and [PORT_PLAN.md](PORT_PLAN.md)
come from the Ferrixel repository (`docs/research/`, commit
`2aa4f3d1bb226d9541a4e9270168b03d17cac2ec`), by the same maintainer.
