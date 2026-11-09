# Port status

Waves 0 to 3c and WP17 of wave 4 are merged (state of 9 November 2026, `main`
after PR 21). The shared utilities, the bit I/O, the DSP, SharpYUV, the lossless
shared code, both codecs in both directions (the VP8 and VP8L decoders and
encoders), the encoder's picture and configuration units, the decoder and
encoder integration, the demuxer and the animation decoder are translated and
verified against the C oracle in its release, debug and asan-ubsan variants, on
macOS ARM64 and Linux x86-64. The C ABIs of `decode.h`, `encode.h` and `demux.h`
are exported, with `WebPMalloc` and `WebPFree`: upstream's `dwebp`, `webpinfo`,
`cwebp` and `anim_dump`, unmodified and linked against the Rust static library,
give the oracle's output bytes, and the three test scripts pass. The safe Rust
encoder API of [ADR 0006](adr/0006-safe-encoder-api.md) is in `src/api/encode.rs`.
The `fuzz/` crate has seven targets. Still open are the muxer and the animation
encoder (WP15), the safe decoder API (WP16) and the SIMD packages. The
integrator aggregates this page from the package pages in [docs/status](status/)
after each wave; packages never edit it.

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
| WP8 demux | 3b | done, PR 17; the demuxer borrows the `WebPData` it parses, for its lifetime, and C's offsets stay offsets; the iterators hold `&WebPDemuxer` in place of `private_`; PR 20 adds a crate-visible `WebPDemuxer::input()` for the C ABI | [wp8](status/wp8.md) |
| WP9 encoder picture and config | 2a | done, PR 8 (PR 11 adapts `RescalePlane` to the PR 9 rescaler); shared views (ADR 0003) | [wp9](status/wp9.md) |
| WP10 VP8 encoder core | 2c | done, PR 15; encoder context per call, the alpha worker receives only owned data (ADR 0003, "Threads"), the token buffer is a vector of pages | [wp10](status/wp10.md) |
| WP11 VP8L encoder | 2b | done, PR 13; the side context runs on a scoped thread | [wp11](status/wp11.md) |
| WP12 decoder integration | 3a | done, PR 16; `decode.h`, `WebPMalloc` and `WebPFree` exported, C buffers converted per call; slow external memory is `WebPDecBuffer::slow_memory` (ADR 0003 amended on 7 November 2026); creates `fuzz/`; its consumer link shim went away with PR 20 | [wp12](status/wp12.md) |
| WP13 encoder integration | 3b | done, PR 18; `encode.h` exported, C pictures converted per call and caller planes lent per call; a view gets its own planes where `WebPEncode` writes; `cwebp` as consumer; PR 21 makes `LOSSLESS_DEFAULT_QUALITY` crate-visible | [wp13](status/wp13.md) |
| WP14 animation decode and demux ABI | 3c | done, PR 20; the animation decoder owns its demuxer and both canvases, keeps the previous frame's iterator fields as a plain struct and lends the canvas to a decoder configuration per `GetNext` call; `demux.h` exported, with a heap handle over a leaked `WebPData` for the C demuxer and decoder; `anim_dump` as consumer, `img2webp` in the C oracle | [wp14](status/wp14.md) |
| WP15 mux and animation encode | 4 | not started; its dependencies are merged | [wp15](status/wp15.md) |
| WP16 safe decoder API | 4 | not started; its dependencies are merged, `src/api/decode.rs` holds its header | [wp16](status/wp16.md) |
| WP17 safe encoder API | 4 | done, PR 21; [ADR 0006](adr/0006-safe-encoder-api.md) accepted; `webp::api::encode` with `Config`, `Picture`, `Encoder`, `encode_lossy`, `encode_lossless` and `EncodeError`, byte-identical to `webp_encode`; creates `src/api/` | [wp17](status/wp17.md) |
| WP18 SIMD x86 decoder | 4 | not started | [wp18](status/wp18.md) |
| WP19 SIMD x86 encoder | 4 | not started | [wp19](status/wp19.md) |
| WP20 NEON | 4 | not started | [wp20](status/wp20.md) |
| WP21 Ferrixel `webp-read` | 5 | not started; tracked in the Ferrixel repository | none |
| WP22 Ferrixel `webp-write` | 5 | not started; tracked in the Ferrixel repository; its encoder dependency (WP17) is merged | none |

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
  ADR; WP14 builds on it (the animation decoder owns the demuxer it creates
  from the caller's `WebPData`, so the data outlives the decoder, as `demux.h`
  requires) and resolves C's self-referential `prev_iter` into the plain
  fields that the decoder reads, with the current frame's iterator rebuilt per
  call. Each package page lists its not-applicable checks, its PORTING rule 8
  exclusions and its recorded deviations, and ends with notes for the packages
  that build on it.
- **Interface changes after a merge** go through their own PR or commit, and the
  next PR is checked against the new `main` before it merges: PR 8 and PR 9
  were each green on their own branch and did not compile together (PR 11 fixed
  `RescalePlane`). PR 12 and PR 13 were compiled together before merging. PR 17
  and PR 18, and again PR 20 and PR 21, changed no common file and passed
  `tools/check.py --oracle --msrv` on a merged tree (for PR 20 and PR 21: the
  tests in three profiles and on Rust 1.95, 324 test binaries and doctest
  runs) before the second of each pair merged; the merged tree of PR 20 and
  PR 21 is the tree of `main` after PR 21. Verification runs in a package's
  worktree only while its session is idle: a run that overlapped the WP13
  session's rebuild of the Rust-linked `cwebp` failed two tool tests
  spuriously; the merged-tree checks run in a separate scratch worktree.
- **C ABI exports** (ADR 0004): `src/abi/decode.rs` (WP12), `src/abi/encode.rs`
  (WP13), `src/abi/demux.rs` (WP14) and `src/abi/types.rs` (`WebPMalloc`,
  `WebPFree`). The pattern, which WP15 follows: one module per header;
  `#[repr(C)]` mirrors with a layout probe; `NULL` and invalid arguments first,
  with C's result; one unsafe operation per block; caller memory lent per call
  and never kept in a Rust borrow between calls; memory that C releases with
  `WebPFree()` from `malloc_copy` or `platform_malloc`. Two restrictions of the
  encode ABI are documented rather than removed: a writer or progress hook
  must not touch the picture being encoded (WP13 deviation 15), and a picture
  block that C frees outside `WebPPictureFree` stays recorded (deviation 16).
  The demux ABI adds the handle pattern for objects that borrow the caller's
  bytes for their lifetime: `WebPDemuxInternal()` and `WebPAnimDecoderNew()`
  leak a `Box<WebPData<'static>>` over the caller's bytes under the header's
  contract, return a heap handle whose first field is the Rust object, and
  the `Delete` functions drop the object before they free the box; the
  caller's bytes must not change while the object lives (WP14 deviation 13).
  `src/abi/demux.rs` also holds the public `repr(C)` `WebPData` of
  `mux_types.h`, for `src/abi/mux.rs` to reuse. The consumer link shim of WP12
  (`tools/oracle/probes/wp12/consumer/link_shim.c`) is gone: the Rust library
  exports everything that imageio references, and the copied upstream targets
  link `libwebpdemux` as the Rust library too.
- **Safe Rust API** (ADR 0006): `src/api/` is the top-level module for the safe
  APIs, separate from the C ABI, created by WP17 with the integrator's OK
  (`pub mod api;` in `src/lib.rs`, the `api` list in `tools/units.json`, and
  `tools/units.py` writes a header for modules that translate no upstream
  file). `src/api/encode.rs` wraps the encoder; `src/api/decode.rs` holds only
  its header and belongs to WP16. The API validates dimensions, strides (in
  elements of the slice's type), buffer lengths and floating-point parameters
  before it calls the translation, so caller input never reaches a panic, and
  adds no behavior: every encoding runs `webp_encode` with the stored
  `WebPConfig`, so the output is byte-identical to C, which WP17 replays from
  WP13's matrix (2,694 encodings) and the simple API (908 commands). Ten
  deliberate differences from the C API are recorded in the ADR and on the
  WP17 page, among them the early refusal of dimensions above 16383 and of
  non-finite qualities, stricter strides for the X layouts and no partial
  output on failure. WP17 also found that the port plan's gate (e) statement
  was wrong: libwebp's lossy conversion is BT.601 in limited range (Y 16 to
  235, Cb and Cr 16 to 240, `src/dsp/yuv.h`), not full range; PORT_PLAN.md and
  the WP17 page say so, and WP16's decoder statement must match.
- **Consumer builds.** `tools/build_oracle.py --rust-lib` links the copied
  upstream tools (`dwebp`, `webpinfo`, `cwebp`, `imageio` and, since PR 20,
  `anim_dump` from `examples/anim_dump.c`, `anim_util.c` and `gifdec.c` without
  giflib) against the Rust static library; `tools/check.py --oracle` uses it,
  and the CI oracle job runs the three scripts through `--baseline`. The copies
  add `upstream/libwebp/src` first in their include path: upstream's targets
  get it only through the `webp` target they link, and a system libwebp
  (Homebrew's on macOS) would otherwise supply `webp/*.h`. Keep that order when
  adding a consumer (WP15: `webpmux`, `anim_diff`; `img2webp` is already in the
  C oracle). The C oracle builds `img2webp` since PR 20
  (`-DWEBP_BUILD_IMG2WEBP=ON` in `tools/build_oracle.py`, documented in
  PORTING.md and ADR 0002), because the pinned test data has no animated file:
  `tests/wp14_support/mod.rs` writes PAM frame sequences and assembles
  animations with it, besides the hand-built containers of WP8's RIFF
  builders; WP15 can generate its inputs the same way.
- **For WP15.** `src/abi/encode.rs` keeps the per-call picture conversion
  (`picture_to_rust`, `picture_to_c`) private. If `src/abi/mux.rs` needs it, the
  integrator makes the two functions `pub(crate)` rather than letting WP15 copy
  them. The animation decoder (`webp_anim_decoder_new`, `get_next`,
  `has_more_frames`) is `anim_diff`'s and the fuzzers' dependency; the WP14 page
  says how to drive it. The handle pattern of the demux ABI fits
  `WebPMuxCreate` with `copy_data == 0`. The animation encoder's output is
  compared frame by frame through the animation decoder, which is verified.
- **For WP16.** `src/api/decode.rs` exists with its header, `src/api/mod.rs`
  declares it, and ADR 0006 is the model for the decoder's ADR (status
  `proposed`, checkpoint to the integrator before implementing). The WP14 page
  describes frames, timestamps, color modes, metadata chunks and memory of the
  animation decoder, and the WP12 page the decoder, the incremental decoder
  and the cropping measurement. The decoder's gate (e) statement is limited
  range BT.601, as the encoder's.
- **Cropping for Ferrixel.** WP12 measured which rows a cropped decoding touches
  ("Cropping measurement for Ferrixel" on its page; C and Rust agree on every
  count): a crop saves the rows below it for VP8 and VP8L, a horizontal crop
  saves nothing, and the incremental decoder saves nothing for VP8 parsing. WP21
  takes its cropping semantics from that table.
- **For WP22.** The WP17 page's notes map Ferrixel's `quality` and `speed` to
  `Config`, show how to hand over a budgeted frame as a `Vec<u32>` of
  `0xAARRGGBB` words through `Picture::from_argb` (pinned by a test to encode as
  `cwebp` does), how `owned_sample_bytes` reports the encoder's conversion
  planes, and what the output and cancellation semantics are. The ADR-0018
  gate (e) statements for the encoder are recorded there.
- **Open for wave 4.** WP15 (mux and animation encode; needs WP6, WP7, WP12,
  WP13, WP14) and WP16 (safe decoder API; needs WP12, WP14) can start now, in
  parallel; they share no module. The SIMD packages WP18 to WP20 as wanted.
  WP22 (Ferrixel `webp-write`) needs only WP17 and a pinnable revision; WP21
  follows WP16.

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
The oracle configuration gained `img2webp` with PR 20; the counts are unchanged.

## Platform evidence

| Host | Evidence |
| --- | --- |
| macOS ARM64, Apple clang 21, Rust 1.99.0 stable and 1.95.0 | `python3 tools/check.py --oracle --msrv`, the WP0 test against all three oracle variants, the baseline above |
| Linux x86-64, GitHub runner `ubuntu-26.04` (image 20261101.186.1), Ubuntu clang 21.1.8, CMake 4.4.5, Rust 1.99.1 stable and 1.95.0 | [CI run 42128339121](https://github.com/sebastian-software/libwebp-rs/actions/runs/42128339121) on `6e0315b`, 6 November 2026: `python3 tools/check.py`; the MSRV job (`--stage test --msrv`, build and tests on Rust 1.95.0); the oracle job, in which all three variants build natively (the first native evidence for the `-msse2` fix of ADR 0002), `tools/unit_dependencies.py --check` finds no undeclared reference, the WP0 test passes in all three profiles against the `release` and `asan-ubsan` oracles (LeakSanitizer active by default), and the baseline above passes for all three variants |
| macOS ARM64, GitHub runner `macos-26-arm64` (image 20261026.0455.2), Apple clang 21.0.0 (clang-2100.1.1.101), CMake 4.4.4, Rust 1.99.1 stable | same run: `python3 tools/check.py` and the oracle job as on Linux |
| Linux x86-64 and macOS ARM64 GitHub runners, state after wave 3b | [CI run 42488051703](https://github.com/sebastian-software/libwebp-rs/actions/runs/42488051703) on `1be9620`, 8 November 2026 (`main` after PR 18): `python3 tools/check.py`; the MSRV job; the oracle job with the consumer builds against the Rust static library (`dwebp`, `webpinfo`, `cwebp`), every package's differential tests against the `release` and `asan-ubsan` oracles, and the three script baselines for all three variants |
| Linux x86-64 and macOS ARM64 GitHub runners, state after wave 3c and WP17 | [CI run 42540593665](https://github.com/sebastian-software/libwebp-rs/actions/runs/42540593665) on `c0cffc2`, 9 November 2026 (`main` after PR 21): `python3 tools/check.py`; the MSRV job; the oracle job with the consumer builds against the Rust static library (`dwebp`, `webpinfo`, `cwebp`, `anim_dump`), the demux and animation probes linked against the Rust library as well as against C, every package's differential tests against the `release` and `asan-ubsan` oracles (the Linux job is the first LeakSanitizer run of the demux ABI's handles), and the three script baselines for all three variants |
