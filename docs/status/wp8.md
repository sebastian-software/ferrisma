# WP8: demux

- Status: every unit of the WP8 inventory is translated: `src/demux/demux.c`
  and the four exceptions that `tools/units.json` assigns to WP8 in
  `src/webp/demux.h`. Every differential test passes against the release
  oracle. Recorded deviations are listed below; none changes an observable
  result that the probes compare.
- Sub-wave: 3b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1 (`WebPData`, `WebPDemuxState`, `WebPFormatFeature`, the
  `GetLE*` helpers, `WebPFeatureFlags`, `WebPMuxAnimDispose`), WP12
  (`webp_get_features`, `webp_abi_is_incompatible`)
- Probes: `tools/oracle/probes/wp8/demux.c`; tests: `tests/wp8_demux.rs` with
  `tests/wp8_support/mod.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Decision: the demuxer borrows its input

`WebPDemuxer<'a>` borrows the bytes of the `WebPData` that it parses, for its
lifetime, as C keeps `mem.buf` without a copy (decided for WP8; the demux
header requires the data to outlive the demuxer). The Rust signature borrows
the `WebPData` value, not only its bytes, so the value must outlive the
demuxer. That is stricter than C, which needs only the bytes; it is the
conservative form, because a `Cow::Owned` payload has no `'a` slice to lend.
C's offsets stay offsets:
`start`, `end`, `riff_end`, `buf_size`, `ChunkData.offset` and `size`. The
frame and chunk lists are `Vec`s in C's order. `MemBuffer` holds positions
only and each read takes the input that the demuxer holds, so no reader stores
a borrow of the bytes (ADR 0003, "Readers over input"). ADR 0003's per-call
rule for decoders does not apply here, and no ADR changes.

The iterators are defined in `src/webp/demux.rs`, appended below WP1's items,
as the exceptions require. `WebPIterator<'d>` and `WebPChunkIterator<'d>` hold
`dmux: &'d WebPDemuxer<'d>` in place of C's `private_`, and the fragment or
chunk payload as a borrowed `WebPData<'d>`. The demuxer is immutable after
construction, so the shared borrow is sound.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/demux/demux.rs` | `src/demux/demux.c` | yes. Public: `webp_get_demux_version`, `webp_demux_internal`, `webp_demux_delete`, `webp_demux_get_i`, `webp_demux_get_frame`, `webp_demux_next_frame`, `webp_demux_prev_frame`, `webp_demux_release_iterator`, `webp_demux_get_chunk`, `webp_demux_next_chunk`, `webp_demux_prev_chunk`, `webp_demux_release_chunk_iterator`, and the type `WebPDemuxer`. Private: `DMUX_*_VERSION`, `ParseStatus` (as a newtype with `PARSE_OK`, `PARSE_NEED_MORE_DATA`, `PARSE_ERROR`), `MemBuffer` with its reader methods, `ChunkData`, `Frame`, `Chunk`, `MasterChunk` (`ChunkParser`, `kMasterChunks`), `image_area`, `remap_mem_buffer`, `init_mem_buffer`, `read_header`, `store_frame`, `set_frame_info`, `get_frame_payload`, `check_frame_bounds`, `init_demux`, `create_raw_image_demuxer`, `synthesize_frame`, `set_frame`, `set_chunk`, and the `WebPDemuxer` methods `add_chunk`, `store_chunk`, `reserve_frame`, `add_frame`, `new_frame`, `parse_animation_frame`, `parse_single_image`, `parse_vp8x_chunks`, `parse_vp8x`, `is_valid_simple_format`, `is_valid_extended_format`, `get_frame`, `chunk_header`, `chunk_count`, `get_chunk`. `MemBuffer::read_fourcc` stands in for the `ReadLE32` read of a chunk tag that C compares with `MKFOURCC`, and `WebPDemuxer::chunk_header` for the inline `mem_buf + offset` read of a tag in `ChunkCount` and `GetChunk`. |
| `src/webp/demux.rs` (exceptions) | `WebPDemux`, `WebPDemuxPartial` (functions) and `WebPIterator`, `WebPChunkIterator` (types) from `src/webp/demux.h` | yes, appended below WP1's items. `webp_demux` returns `Option<WebPDemuxer>`; `webp_demux_partial` returns `(Option<WebPDemuxer>, WebPDemuxState)`. |

Every translated item carries an `// upstream:` marker. Names follow
`tools/rust_name.py`.

## Evidence

The probe `tools/oracle/probes/wp8/demux.c` reads commands on standard input.
For each `demux` it prints the state, whether the public wrapper returns a
demuxer (and its state for the partial one), `WebPDemuxGetI` for every feature
(including two unknown values), every frame that `WebPDemuxGetFrame` returns for
frames 0 to frame count + 1 with all `WebPIterator` fields, the forward walk
from frame 1 and the backward walk from the last frame, and the same for eleven
chunk tags (`ICCP`, `EXIF`, `XMP `, `ALPH`, `VP8 `, `VP8L`, `VP8X`, `ANIM`,
`ANMF`, an unknown tag and an absent tag), with all `WebPChunkIterator`
fields. A fragment or payload is printed as its size, its 64-bit FNV-1a digest
and its first 16 bytes. Each probe run is a separate process.

Tests (`cargo test --features oracle --test wp8_demux`):

- `version_matches_the_c_probe`: `WebPGetDemuxVersion`.
- `test_vectors_match_the_c_demuxer`: all 131 vectors of
  `upstream/libwebp-test-data`, complete and with `allow_partial` on and off,
  and up to 14 truncation points per file at chunk boundaries, their
  neighbours and inside payloads.
- `still_images_with_every_flag_combination_match_the_c_demuxer`: all 256 values
  of the `VP8X` flag byte (reserved bits included), `VP8L` and `VP8`+`ALPH`
  images, with and without ICC, EXIF and XMP chunks, and truncations.
- `animations_match_the_c_demuxer`: zero, one, two, three and six frames, the
  dispose and blend bits, random canvas sizes, frames that do not fit, and
  metadata around frames.
- `synthetic_frames_match_the_c_demuxer_for_every_bit_combination`: all 16 dispose
  and blend combinations, durations 0, 1, 3, 255, 0xFFFF and 0xFFFFFF, and odd
  payload sizes.
- `malformed_chunks_match_the_c_demuxer`: odd sizes with and without the pad
  byte, sizes beyond the RIFF size, RIFF sizes smaller and larger than the data
  and below the minimum, zero-sized metadata, duplicated `VP8X` and `ANIM`,
  `ANMF` before `ANIM`, short `ANIM` and `ANMF`, sub-chunk overflow, frames with
  unknown, duplicated or misordered sub-chunks, an invalid `VP8` payload, a huge
  canvas, a `VP8X` size beyond the file, alpha without an image, non-RIFF
  headers, and empty and one-byte inputs.
- `raw_bitstreams_match_the_c_demuxer`: `VP8`, `VP8L` and `ALPH` payloads without
  the RIFF header, with prefixes and trailing bytes.
- `random_mutations_match_the_c_demuxer`: 600 seeded mutations of four base
  files (bit flips, chunk sizes, truncation and tag-like bytes), complete and
  partial.
- `the_wrappers_match_the_c_demuxer`: `WebPDemux` and `WebPDemuxPartial` against
  the internal entry point.

Each test prints the number of commands it compared (`cargo test -- --nocapture`).

Commands run from the worktree, on macOS ARM64 (the Linux x86-64 CI job was not
run locally):

- `python3 tools/check.py --oracle --msrv`: result recorded in the pull request.
- `cargo +stable clippy --all-targets -- -D warnings` and `cargo fmt --all --
  --check`: clean.
- `LIBWEBP_ORACLE_DIR=build/oracle/release cargo test --features oracle --test wp8_demux`.

## Not-applicable checks

- **NULL arguments.** `WebPDemux(NULL)`, `WebPDemuxPartial(NULL)`,
  `WebPDemuxInternal` with `data == NULL`, `WebPDemuxGetI(NULL)`,
  `WebPDemuxGetFrame(NULL, ...)`, `WebPDemuxGetFrame(..., NULL)`,
  `WebPDemuxGetChunk` with `fourcc == NULL`, `WebPDemuxNextFrame(NULL)` and the
  other iterator calls, `WebPDemuxDelete(NULL)`, and the state pointer. References
  make these unrepresentable. The empty-data case of `WebPDemuxInternal` (C's
  `data->size == 0`) is the empty slice. WP14 keeps the NULL results in the C ABI.
- **`private_`.** Replaced by `dmux: &WebPDemuxer` (see the decision above).
- **`WebPDemuxReleaseIterator`, `WebPDemuxReleaseChunkIterator`.** No-ops in C
  (they free nothing); kept as functions that take the iterator.
- **`WebPDemuxDelete`.** The release is the drop of the value; the function takes
  the demuxer by value.
- **`WebPSafeFree`.** Drops the frame and chunk values that C frees.
- **`assert(IsValidSimpleFormat(dmux))`** in `CreateRawImageDemuxer` is a
  `debug_assert!`, as ADR 0003 maps `assert`.
- **ABI version.** `WebPDemuxInternal`'s version check
  (`WEBP_ABI_IS_INCOMPATIBLE`) is kept in `webp_demux_internal`; the public
  `webp_demux` and `webp_demux_partial` pass `WEBP_DEMUX_ABI_VERSION`.
- **`RemapMemBuffer`.** It refuses a size below the buffer size of the reader it
  remaps. `InitMemBuffer` calls it on a zeroed reader, so the refusal is never
  reached; it is translated as is.
- **`src/webp/mux.h`** (reviewed include). Not used; WP8 has no dependency on WP15.

## Recorded deviations

1. **Upstream guard in `AddFrame` never fires.** C checks
   `*dmux->frames_tail` before linking a frame. `frames_tail` points at the
   `next` link of the last frame, which `AddFrame` has just set to `NULL`, so the
   check `last_frame != NULL && !last_frame->complete` is always false. C's
   `AddFrame` therefore appends every frame. The port does the same
   (`add_frame` appends unconditionally, with a comment that names the reason).
   A consequence is observable: an `ANMF` whose frame is incomplete (an `ALPH`
   chunk without its image) followed by another frame is accepted by the parser,
   and `IsValidExtendedFormat` rejects the file ("There shouldn't be any frames
   after an incomplete one"). The animation test covers this case. The
   integrator may want to report the guard upstream.
2. **Zeroed iterators on failure.** `WebPDemuxGetFrame` and `WebPDemuxGetChunk`
   return `None` where C returns 0 and leaves a zeroed iterator with `private_`
   set. Two C behaviours follow from a zeroed iterator and are therefore
   unreachable: `WebPDemuxNextFrame` on it synthesizes frame 1 (its `frame_num`
   is 0), and `WebPDemuxNextChunk` on it computes `chunk.bytes - 8` from NULL,
   which is undefined behaviour. Neither is compared by the probe; the walks
   start only from a successful call.
3. **Chunk tag kept in the iterator.** C's `WebPDemuxNextChunk` recovers the
   tag from the four bytes before the payload. The port stores the tag that
   `WebPDemuxGetChunk` was given (`fourcc`, `pub(crate)`). The two are equal,
   because `GetChunk` matched the header against the tag.
4. **`WebPDemuxInternal` returns the state with the demuxer.** It returns
   `(Option<WebPDemuxer>, WebPDemuxState)`. The state is what C writes to
   `*state`, including `WEBP_DEMUX_PARSING_HEADER` for a file whose first chunk is
   not `VP8 `, `VP8L` or `VP8X`: C returns the parser state before it records
   the error. `WebPDemuxPartial` returns the same pair.
5. **Null-frame dereference in `IsValidSimpleFormat`.** C reads `frame->width`
   after checking `state == DONE && frame == NULL`, so a parsed header without a
   frame would crash. `ParseSingleImage` sets `PARSED_HEADER` only after it
   stores its frame, so the case is unreachable. The port returns false there.
6. **Allocation.** The frame and chunk lists are `Vec`s. Each C `WebPSafeCalloc`
   of a `Frame` or a `Chunk` becomes `try_reserve(1)` at the same point, so a
   refused host allocation ends the parse as C's NULL does, and the following
   push cannot reallocate. The demuxer itself is a struct value and cannot fail
   in Rust; C's failure path for `WebPSafeCalloc(dmux)` is not modelled, and no
   test can reach it.
7. **Arithmetic that relies on invariants.** `SizeIsInvalid`'s `riff_end -
   start` and `GetFramePayload`'s `image->offset - (alpha->offset + alpha->size)`
   rely on the order `start <= end <= riff_end` and on the alpha chunk ending
   where the image starts, which the parser establishes. The port uses plain
   arithmetic, so a violation panics in the dev and `checked` profiles instead of
   wrapping. `ANMF`'s `frame_chunk_size - ANMF_CHUNK_SIZE` is `wrapping_sub`, as in
   C's unsigned difference, which is used only after `NewFrame` accepts the size.
8. **Image area.** `width * (uint64_t)height` is computed as C converts the `int`
   operand (`image_area`), with wrapping multiplication.
9. **Undefined behaviour in C.** The asan-ubsan oracle (ASan and UBSan with
   `-fno-sanitize-recover=all`) and the debug oracle (upstream's assertions
   enabled) agree with the release oracle on every compared case. No case is
   excluded from the oracle.

## Notes for the packages that build on WP8

### WP14 (animation decode and demux ABI)

- **Creating a demuxer.** Call `webp_demux_internal(&data, allow_partial,
  version)` with a `WebPData<'a>` built from the caller's bytes (`Cow::Borrowed`).
  The demuxer borrows those bytes for its lifetime, as the header requires (the
  data must outlive the demuxer). A C pointer cannot carry that lifetime, so the
  ABI keeps the demuxer as a `Box<WebPDemuxer<'static>>` made from the caller's
  memory under the header's contract, with a `// SAFETY:` argument for each
  operation, and frees it in `WebPDemuxDelete` with `webp_demux_delete`. Under
  ADR 0004 the `private_` pointer of the C-side structs is that pointer.
- **NULL and invalid arguments.** `WebPDemux` and `WebPDemuxPartial` return NULL
  when `data` is NULL, with state `WEBP_DEMUX_PARSE_ERROR`. `WebPDemuxGetI`
  returns 0 for a NULL demuxer. `WebPDemuxGetFrame`, `WebPDemuxGetChunk` and the
  `Next` and `Prev` functions return 0 for a NULL iterator or demuxer, as C does.
  `WebPDemuxGetChunk` with a NULL `fourcc` returns 0.
- **Iterators.** `webp_demux_get_frame(&dmux, n)` returns `Option<WebPIterator>`.
  Map `Some` to the C result (1, with the fields written, `fragment` the frame's
  bytes from the ALPH chunk, if any, through the image chunk) and `None` to 0
  with a zeroed C iterator whose `private_` is the demuxer. The C struct also
  keeps `private_` for `Next` and `Prev`.
- **Chunk iterators from C.** Rebuild `WebPChunkIterator` from the C struct: its
  `private_` gives `dmux`, and `fourcc` is the four bytes before `chunk.bytes`,
  as C reads them. The field is `pub(crate)`, so the ABI can set it.
- **Zeroed-iterator `Next`.** C's `WebPDemuxNextFrame` on a zeroed iterator
  synthesizes frame 1. The ABI can reproduce it with a `WebPIterator` whose
  `frame_num` is 0, which `webp_demux_next_frame` handles like C.
- **Structs.** `WebPIterator` and `WebPChunkIterator` have pointers and
  references, so WP14 writes a `#[repr(C)]` mirror in `src/abi/demux.rs` with the
  C field order and `private_` (ADR 0004). `WebPDemuxState` and
  `WebPFormatFeature` are WP1's newtypes and convert directly.
- **Version.** `webp_abi_is_incompatible` is checked inside `webp_demux_internal`;
  the ABI passes the caller's version through.
- **Animation decoder.** `anim_decode.c` (WP14) reads the demuxer's frames
  through `webp_demux_get_frame`, `webp_demux_next_frame` and
  `webp_demux_get_i`. It does not need `WebPDemuxGetFrame`'s failed-iterator
  behaviour.

### WP16 (safe decoder API)

- `webp_demux`, `webp_demux_partial` and the iterators are the metadata view.
  `webp_demux_get_chunk(&dmux, *b"XMP ", 1)` returns the payload of the first
  `XMP ` chunk; `webp_demux_get_i` reports the flags, canvas and frame count.

## Verification record

Recorded after the final run of each command:

- `LIBWEBP_ORACLE_DIR=build/oracle/release cargo test --features oracle --test
  wp8_demux`: all nine tests pass in the dev profile.
- The same command with `build/oracle/asan-ubsan` and `build/oracle/debug`:
  all nine tests pass against each variant.
- `python3 tools/check.py --oracle --msrv`: see the pull request description
  for the result of the full run.
