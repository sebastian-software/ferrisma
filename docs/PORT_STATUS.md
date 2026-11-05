# Port status

Nothing of libwebp has been translated yet. WP0 bootstrapped the repository:
every upstream unit has its Rust module with a provenance header and no code,
the C oracle and the differential harness work, and the baseline below is
recorded. The integrator aggregates this page from the package pages in
[docs/status](status/) after each wave; packages never edit it.

## Packages

| Package | Wave | Status | Page |
| --- | --- | --- | --- |
| WP0 bootstrap | 0 | done (local host; CI not run) | [wp0](status/wp0.md) |
| WP1 types, memory, workers | 1 | not started | [wp1](status/wp1.md) |
| WP2 bit I/O | 1 | not started | [wp2](status/wp2.md) |
| WP3 lossless shared | 1 | not started | [wp3](status/wp3.md) |
| WP4 pixel ops | 1 | not started | [wp4](status/wp4.md) |
| WP5 SharpYUV | 1 | not started | [wp5](status/wp5.md) |
| WP6 VP8 decoder core | 2 | not started | [wp6](status/wp6.md) |
| WP7 VP8L decoder | 2 | not started | [wp7](status/wp7.md) |
| WP8 demux | 2 | not started | [wp8](status/wp8.md) |
| WP9 encoder picture and config | 2 | not started | [wp9](status/wp9.md) |
| WP10 VP8 encoder core | 2 | not started | [wp10](status/wp10.md) |
| WP11 VP8L encoder | 2 | not started | [wp11](status/wp11.md) |
| WP12 decoder integration | 3 | not started | [wp12](status/wp12.md) |
| WP13 encoder integration | 3 | not started | [wp13](status/wp13.md) |
| WP14 animation decode and demux ABI | 3 | not started | [wp14](status/wp14.md) |
| WP15 mux and animation encode | 4 | not started | [wp15](status/wp15.md) |
| WP16 safe decoder API | 4 | not started | [wp16](status/wp16.md) |
| WP17 safe encoder API | 4 | not started | [wp17](status/wp17.md) |
| WP18 SIMD x86 decoder | 4 | not started | [wp18](status/wp18.md) |
| WP19 SIMD x86 encoder | 4 | not started | [wp19](status/wp19.md) |
| WP20 NEON | 4 | not started | [wp20](status/wp20.md) |
| WP21 Ferrixel `webp-read` | 5 | not started; tracked in the Ferrixel repository | none |
| WP22 Ferrixel `webp-write` | 5 | not started; tracked in the Ferrixel repository | none |

[WORKPLAN.md](WORKPLAN.md#findings-from-wp0) records dependencies between
packages that the catalog does not declare and a proposed order; the maintainer
decides it before the fan-out.

## Oracle baseline

`python3 tools/oracle_baseline.py` runs upstream's `test_dwebp.sh` and
`test_lossless.sh`, unmodified, with the oracle's own `dwebp` on copies of the
test data. The pinned test data postdates v1.6.0: test-data commits `05cd58b` and
`06ddd96` regenerate seven PGM checksums and three PGM reference images for
libwebp `0d14d84b`, which landed after v1.6.0. The baseline therefore has two
views.

With the four updated files taken from test-data `260804d`, the last revision
before that update (v1.6.0-era references):

| Script | Result |
| --- | --- |
| `test_dwebp.sh` | passes: all 480 checksums of 96 vectors (five formats each) verified twice, with and without `-noasm`, 960 verifications |
| `test_lossless.sh` | passes: 33 vectors (32 `lossless_vec_*` and `lossless_color_transform.webp`) in five formats, with and without `-noasm`, 330 comparisons |

With the pinned references unchanged:

| Check | Result |
| --- | --- |
| `test_dwebp.sh --dump-md5s` against `libwebp_tests.md5` | 473 of 480 checksums match; the 7 others are exactly the PGM entries regenerated after v1.6.0: `bad_palette_index`, `color_cache_bits_11`, `lossless1`, `lossless2`, `lossless3`, `lossless4`, `near_lossless_75` |
| `test_lossless.sh --formats="ppm pam bmp tiff"` | passes: 264 comparisons |
| PGM output of the 33 lossless vectors against `grid.pgm`, `peak.pgm`, `lossless_color_transform.pgm` | 0 of 33 match, as expected for references regenerated after v1.6.0 |

Recorded on 5 November 2026 on macOS 27.0.1 (build 26A434), Apple M1 Ultra
(ARM64), with Apple clang 21.0.0 (clang-2100.3.34.2) and CMake 4.4.5. The
`release`, `asan-ubsan` and `debug` oracle variants give identical counts.
`test_dwebp.sh` needs an `md5sum` that accepts `-c -`. No substitution was
needed on this host: `md5sum` resolves to `/sbin/md5sum`, the FreeBSD-derived
`md5` of macOS in its GNU-compatible mode, which prints `name: OK` lines like
GNU coreutils. The script passes it explicitly as `--md5exec`; on macOS releases
before 14, which lack `md5sum`, it falls back to `gmd5sum` from Homebrew
coreutils.

## Platform evidence

| Host | Evidence |
| --- | --- |
| macOS ARM64, Apple clang 21, Rust 1.99.0 stable and 1.95.0 | `python3 tools/check.py --oracle --msrv`, the WP0 test against all three oracle variants, the baseline above |
| Linux x86-64, Clang | none yet: the CI workflow runs once the repository has a remote |
| macOS ARM64, GitHub runner | none yet, as above |
