# Port status

Nothing of libwebp has been translated yet. WP0 bootstrapped the repository:
every upstream unit has its Rust module with a provenance header and no code,
the C oracle and the differential harness work, and the baseline below is
recorded. The integrator aggregates this page from the package pages in
[docs/status](status/) after each wave; packages never edit it.

## Packages

| Package | Sub-wave | Status | Page |
| --- | --- | --- | --- |
| WP0 bootstrap | 0 | done (local host; CI not run) | [wp0](status/wp0.md) |
| WP1 types, memory, workers | 1a | not started | [wp1](status/wp1.md) |
| WP2 bit I/O | 1b | not started | [wp2](status/wp2.md) |
| WP3 lossless shared | 1c | not started | [wp3](status/wp3.md) |
| WP4 pixel ops | 1b | not started | [wp4](status/wp4.md) |
| WP5 SharpYUV | 1b | not started | [wp5](status/wp5.md) |
| WP6 VP8 decoder core | 2b | not started | [wp6](status/wp6.md) |
| WP7 VP8L decoder | 2a | not started | [wp7](status/wp7.md) |
| WP8 demux | 3b | not started | [wp8](status/wp8.md) |
| WP9 encoder picture and config | 2a | not started | [wp9](status/wp9.md) |
| WP10 VP8 encoder core | 2c | not started | [wp10](status/wp10.md) |
| WP11 VP8L encoder | 2b | not started | [wp11](status/wp11.md) |
| WP12 decoder integration | 3a | not started | [wp12](status/wp12.md) |
| WP13 encoder integration | 3b | not started | [wp13](status/wp13.md) |
| WP14 animation decode and demux ABI | 3c | not started | [wp14](status/wp14.md) |
| WP15 mux and animation encode | 4 | not started | [wp15](status/wp15.md) |
| WP16 safe decoder API | 4 | not started | [wp16](status/wp16.md) |
| WP17 safe encoder API | 4 | not started | [wp17](status/wp17.md) |
| WP18 SIMD x86 decoder | 4 | not started | [wp18](status/wp18.md) |
| WP19 SIMD x86 encoder | 4 | not started | [wp19](status/wp19.md) |
| WP20 NEON | 4 | not started | [wp20](status/wp20.md) |
| WP21 Ferrixel `webp-read` | 5 | not started; tracked in the Ferrixel repository | none |
| WP22 Ferrixel `webp-write` | 5 | not started; tracked in the Ferrixel repository | none |

The sub-waves follow libwebp's call graph. [WORKPLAN.md](WORKPLAN.md) lists
each package's dependencies and the function- and type-level exceptions, and
`tools/check.py --oracle` checks them against the oracle's cross-unit
references.

## Oracle baseline

`python3 tools/oracle_baseline.py` runs upstream's `test_dwebp.sh` and
`test_lossless.sh`, unmodified, with the oracle's own `dwebp` on a copy of the
pinned test data (`260804d`):

| Script | Result |
| --- | --- |
| `test_dwebp.sh` | passes: all 480 checksums of 96 vectors (five formats each) verified twice, with and without `-noasm`, 960 verifications |
| `test_lossless.sh` | passes: 33 vectors (32 `lossless_vec_*` and `lossless_color_transform.webp`) in five formats, with and without `-noasm`, 330 comparisons |

The test data was first pinned at `06ddd96`, whose last two commits regenerate
seven PGM checksums and three PGM reference images for libwebp `0d14d84b`, a
change made after v1.6.0. `--diagnose-post-v1.6.0` still compares against those
references: 473 of 480 checksums match, the seven others being exactly the
regenerated PGM entries (`bad_palette_index`, `color_cache_bits_11`,
`lossless1` to `lossless4`, `near_lossless_75`), and none of the 33 lossless PGM
outputs matches the regenerated images.

Recorded on 6 November 2026, after the re-pin, on macOS 27.0.1 (build 26A434), Apple M1 Ultra
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
