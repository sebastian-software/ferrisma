# Port status

Waves 0 to 3b are merged (state of 8 November 2026, `main` after PR 18). The
shared utilities, the bit I/O, the DSP, SharpYUV, the lossless shared code, both
codecs in both directions (the VP8 and VP8L decoders and encoders), the
encoder's picture and configuration units, the decoder and encoder integration
and the demuxer are translated and verified against the C oracle in its
release, debug and asan-ubsan variants, on macOS ARM64 and Linux x86-64. The C
ABIs of `decode.h` and `encode.h` are exported, with `WebPMalloc` and
`WebPFree`: upstream's `dwebp`, `webpinfo` and `cwebp`, unmodified and linked
against the Rust static library, pass `test_dwebp.sh`, `test_lossless.sh` and
`test_cwebp.sh` with the oracle's output bytes. The `fuzz/` crate has six
targets. Still open are the demux and animation ABI (WP14), the muxer and the
animation encoder (WP15), the safe Rust APIs (WP16, WP17) and the SIMD
packages. The integrator aggregates this page from the package pages in
[docs/status](status/) after each wave; packages never edit it.

## Packages

| Package | Sub-wave | Status | Page |
| --- | --- | --- | --- |
| WP0 bootstrap | 0 | done (local host and CI) | [wp0](status/wp0.md) |
| WP1 types, memory, workers | 1a | done, PR 1 (PR 6 excludes zero-size allocations from the differential) | [wp1](status/wp1.md) |
| WP2 bit I/O | 1b | done, PR 4; readers store positions, the input is passed per call (ADR 0003, "Readers over input") | [wp2](status/wp2.md) |
| WP3 lossless shared | 1c | done, PR 7; four `lossless_common.h` inline functions moved to WP11 | [wp3](status/wp3.md) |
| WP4 pixel ops | 1b | done, PR 5; the rescaler owns its side rows and takes the output per call since PR 9, and exports by a signed stride since PR 12 | [wp4](status/wp4.md) |
| WP5 SharpYUV | 1b | done, PR 3 | [wp5](status/wp5.md) |
| WP6 VP8 decoder core | 2b | done, PR 12; the `put` hook and the alpha rows run on the calling thread at `Sync` (ADR 0003, "Threads") | [wp6](status/wp6.md) |
| WP7 VP8L decoder | 2a | done, PR 10; decoding context per call (ADR 0003) | [wp7](status/wp7.md) |
| WP8 demux | 3b | done, PR 17; the demuxer borrows the `WebPData` it parses, for its lifetime, and C's offsets stay offsets; the iterators hold `&WebPDemuxer` in place of `private_`; the `demux.h` C ABI is WP14 | [wp8](status/wp8.md) |
| WP9 encoder picture and config | 2a | done, PR 8 (PR 11 adapts `RescalePlane` to the PR 9 rescaler); shared views (ADR 0003) | [wp9](status/wp9.md) |
| WP10 VP8 encoder core | 2c | done, PR 15; encoder context per call, the alpha worker receives only owned data (ADR 0003, "Threads"), the token buffer is a vector of pages | [wp10](status/wp10.md) |
| WP11 VP8L encoder | 2b | done, PR 13; the side context runs on a scoped thread | [wp11](status/wp11.md) |
| WP12 decoder integration | 3a | done, PR 16; `decode.h`, `WebPMalloc` and `WebPFree` exported, C buffers converted per call; slow external memory is `WebPDecBuffer::slow_memory` (ADR 0003 amended on 7 November 2026); creates `fuzz/` | [wp12](status/wp12.md) |
| WP13 encoder integration | 3b | done, PR 18; `encode.h` exported, C pictures converted per call and caller planes lent per call; a view gets its own planes where `WebPEncode` writes; `cwebp` as consumer | [wp13](status/wp13.md) |
| WP14 animation decode and demux ABI | 3c | not started; its dependencies are merged | [wp14](status/wp14.md) |
| WP15 mux and animation encode | 4 | not started; follows WP14 | [wp15](status/wp15.md) |
| WP16 safe decoder API | 4 | not started; follows WP14 | [wp16](status/wp16.md) |
| WP17 safe encoder API | 4 | not started; its dependency is merged | [wp17](status/wp17.md) |
| WP18 SIMD x86 decoder | 4 | not started | [wp18](status/wp18.md) |
| WP19 SIMD x86 encoder | 4 | not started | [wp19](status/wp19.md) |
| WP20 NEON | 4 | not started | [wp20](status/wp20.md) |
| WP21 Ferrixel `webp-read` | 5 | not started; tracked in the Ferrixel repository | none |
| WP22 Ferrixel `webp-write` | 5 | not started; tracked in the Ferrixel repository | none |

The sub-waves follow libwebp's call graph. [WORKPLAN.md](WORKPLAN.md) lists
each package's dependencies and the function- and type-level exceptions, and
`tools/check.py --oracle` checks them against the oracle's cross-unit
references.

## Integration notes

- **Representation decisions** taken between the packages are recorded as dated
  amendments of [ADR 0003](adr/0003-rust-representation.md): readers over input
  (WP2), untyped sample buffers (WP5), raw data and buffers and planes (WP1,
  WP9), shared views (WP9), decoding context per call (WP7), the thread rule
  for workers that outlive a call (WP6) or live inside one (WP11) and slow
  external memory as a buffer flag (WP12). WP8's decision that the demuxer
  borrows the `WebPData` value it parses is recorded on its page and changes no
  ADR. Each package page lists its not-applicable checks, its PORTING rule 8
  exclusions and its recorded deviations, and ends with notes for the packages
  that build on it.
- **Interface changes after a merge** go through their own PR or commit, and the
  next PR is checked against the new `main` before it merges: PR 8 and PR 9
  were each green on their own branch and did not compile together (PR 11 fixed
  `RescalePlane`). PR 12 and PR 13 were compiled together before merging. PR 17
  and PR 18 changed no common file and passed `tools/check.py --oracle` on a
  merged tree (76 probes, the tests in three profiles) before PR 18 merged.
  Verification runs in a package's worktree only while its session is idle: a
  run that overlapped the WP13 session's rebuild of the Rust-linked `cwebp`
  failed two tool tests spuriously.
- **C ABI exports** (ADR 0004): `src/abi/decode.rs` (WP12), `src/abi/encode.rs`
  (WP13) and `src/abi/types.rs` (`WebPMalloc`, `WebPFree`). The pattern, which
  WP14 and WP15 follow: one module per header; `#[repr(C)]` mirrors with a
  layout probe; `NULL` and invalid arguments first, with C's result; one unsafe
  operation per block; caller memory lent per call and never kept in a Rust
  borrow between calls; memory that C releases with `WebPFree()` from
  `malloc_copy` or `platform_malloc`. Two restrictions of the encode ABI are
  documented rather than removed: a writer or progress hook must not touch the
  picture being encoded (WP13 deviation 15), and a picture block that C frees
  outside `WebPPictureFree` stays recorded (deviation 16). A package that
  exports a function which `tools/oracle/probes/wp12/consumer/link_shim.c`
  stands in for deletes the stand-in, or the consumer link fails on duplicate
  symbols (WP14: the demux API).
- **Consumer builds.** `tools/build_oracle.py --rust-lib` links the copied
  upstream tools (`dwebp`, `webpinfo`, `cwebp` and `imageio`) against the Rust
  static library; `tools/check.py --oracle` uses it, and the CI oracle job runs
  the three scripts through `--baseline`. The copies add `upstream/libwebp/src`
  first in their include path: upstream's targets get it only through the
  `webp` target they link, and a system libwebp (Homebrew's on macOS) would
  otherwise supply `webp/*.h`. Keep that order when adding a consumer (WP14:
  `anim_dump`; WP15: `webpmux`, `img2webp`, `anim_diff`).
- **For WP14.** The demuxer borrows the `WebPData` value, not only its bytes, so
  the ABI keeps the demuxer behind a pointer made from the caller's memory under
  the `demux.h` contract (the data outlives the demuxer) and frees it in
  `WebPDemuxDelete`. `webp_demux_internal` returns the state with the demuxer;
  a failed `WebPDemuxGetFrame` or `WebPDemuxGetChunk` is `None` where C leaves a
  zeroed iterator with `private_` set; the chunk iterator keeps its tag
  (`fourcc`, `pub(crate)`). Upstream's `AddFrame` guard against a frame after an
  incomplete one never fires (`*frames_tail` is NULL when it is tested), and
  `IsValidExtendedFormat` rejects such files later; the port appends
  unconditionally and documents the reason (WP8 deviation 1). Whether to report
  the guard upstream is the maintainer's call.
- **For WP15.** `src/abi/encode.rs` keeps the per-call picture conversion
  (`picture_to_rust`, `picture_to_c`) private. If `src/abi/mux.rs` needs it, the
  integrator makes the two functions `pub(crate)` rather than letting WP15 copy
  them.
- **Cropping for Ferrixel.** WP12 measured which rows a cropped decoding touches
  ("Cropping measurement for Ferrixel" on its page; C and Rust agree on every
  count): a crop saves the rows below it for VP8 and VP8L, a horizontal crop
  saves nothing, and the incremental decoder saves nothing for VP8 parsing. WP21
  takes its cropping semantics from that table.
- **Open for sub-wave 3c and wave 4.** WP14 (animation decode and demux ABI;
  needs WP8 and WP12) and WP17 (safe encoder API; needs WP13) can start now, in
  parallel. WP15 and WP16 follow WP14; the SIMD packages WP18 to WP20 as wanted.

## Oracle baseline

`python3 tools/oracle_baseline.py` runs upstream's `test_dwebp.sh`,
`test_lossless.sh` and, since PR 18, `test_cwebp.sh`, unmodified, with the
oracle's own `dwebp` and `cwebp` on a copy of the pinned test data (`260804d`):

| Script | Result |
| --- | --- |
| `test_dwebp.sh` | passes: all 480 checksums of 96 vectors (five formats each) verified twice, with and without `-noasm`, 960 verifications |
| `test_lossless.sh` | passes: 33 vectors (32 `lossless_vec_*` and `lossless_color_transform.webp`) in five formats, with and without `-noasm`, 330 comparisons |
| `test_cwebp.sh` | passes: the 142 test-data files that `cwebp` reads (131 vectors and the PAM, PGM, PPM and PNG files; TIFF needs libtiff, BMP has no reader) in each of three option sets (default, `--lossless`, `--mt --noalpha --extra_args=-q 30 -m 6 -sharp_yuv -af`), with and without `-noasm`, 284 checksums per set |

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
coreutils. On the Ubuntu 26.04 CI runner, `md5sum` resolves to the uutils
coreutils binary (`/usr/lib/cargo/bin/coreutils/md5sum`), which accepts `-c -`
as well.

The first CI run reproduced the same counts for all three variants on the
Linux x86-64 and macOS ARM64 runners listed under Platform evidence. The
`test_cwebp.sh` row was recorded on 8 November 2026 on the same host, with PR
18; the Rust-linked `cwebp` records the same checksums (`tests/wp13_tools.rs`).

## Platform evidence

| Host | Evidence |
| --- | --- |
| macOS ARM64, Apple clang 21, Rust 1.99.0 stable and 1.95.0 | `python3 tools/check.py --oracle --msrv`, the WP0 test against all three oracle variants, the baseline above |
| Linux x86-64, GitHub runner `ubuntu-26.04` (image 20261101.186.1), Ubuntu clang 21.1.8, CMake 4.4.5, Rust 1.99.1 stable and 1.95.0 | [CI run 42128339121](https://github.com/sebastian-software/libwebp-rs/actions/runs/42128339121) on `6e0315b`, 6 November 2026: `python3 tools/check.py`; the MSRV job (`--stage test --msrv`, build and tests on Rust 1.95.0); the oracle job, in which all three variants build natively (the first native evidence for the `-msse2` fix of ADR 0002), `tools/unit_dependencies.py --check` finds no undeclared reference, the WP0 test passes in all three profiles against the `release` and `asan-ubsan` oracles (LeakSanitizer active by default), and the baseline above passes for all three variants |
| macOS ARM64, GitHub runner `macos-26-arm64` (image 20261026.0455.2), Apple clang 21.0.0 (clang-2100.1.1.101), CMake 4.4.4, Rust 1.99.1 stable | same run: `python3 tools/check.py` and the oracle job as on Linux |
| Linux x86-64 and macOS ARM64 GitHub runners, state after wave 3b | [CI run 42488051703](https://github.com/sebastian-software/libwebp-rs/actions/runs/42488051703) on `1be9620`, 8 November 2026 (`main` after PR 18): `python3 tools/check.py`; the MSRV job; the oracle job with the consumer builds against the Rust static library (`dwebp`, `webpinfo`, `cwebp`), every package's differential tests against the `release` and `asan-ubsan` oracles, and the three script baselines for all three variants |
