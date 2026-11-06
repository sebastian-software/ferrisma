# WP1: types, memory, workers

- Status: every unit of the WP1 inventory is translated, or stays absent for a
  recorded reason that another package owns. The decisions of 2026-11-06 are
  applied in ADR 0001, ADR 0003 and `tools/units.json`; none of the questions of
  the first report is open for WP1.
- Sub-wave: 1a
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP0 only
- Probes: `tools/oracle/probes/wp1/`; tests: `tests/wp1_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

Every row names what is translated, with `// upstream:` markers on each item.
"Partly" rows name what stays with another package and why; nothing in a
"partly" row is a stub.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/webp/types.rs` | `src/webp/types.h` | partly: `webp_abi_is_incompatible`, and the storage types `PlaneStorage<'a, T>` and `PlaneOffset` of ADR 0003, which have no C counterpart. `WebPMalloc` and `WebPFree` are C-ABI memory and belong to `src/abi/types.rs` (WP12, ADR 0004). |
| `src/webp/format_constants.rs` | `src/webp/format_constants.h` | yes: every `#define`, `VP8LImageTransformType`, `MKFOURCC`. |
| `src/webp/decode.rs` | `src/webp/decode.h` | partly: `WEBP_DECODER_ABI_VERSION`, `VP8StatusCode`, `WEBP_CSP_MODE` and its values, the three mode predicates, `WebPBitstreamFeatures`, `WebPDecoderOptions`, `WebPRGBABuffer`, `WebPYUVABuffer`, the union enum `WebPDecBufferPlanes`, `WebPDecBuffer<'a>` and `WebPDecoderConfig<'a>`. Exception WP12: the inline wrappers `WebPInitDecBuffer`, `WebPGetFeatures`, `WebPInitDecoderConfig` and `WebPIDecGetYUV`. The declared functions belong to WP12. |
| `src/webp/encode.rs` | `src/webp/encode.h` | partly: `WEBP_ENCODER_ABI_VERSION`, `WEBP_MAX_DIMENSION`, `WebPImageHint`, `WebPConfig`, `WebPPreset`, `WebPAuxStats`, `WebPEncCSP`, `WebPEncodingError`, `WebPPicture<'a>`, the closure types `WebPWriterFunction<'a>` and `WebPProgressHook<'a>` without the picture parameter, and `WebPMemoryWriter`. Exception WP9: the inline wrappers `WebPConfigInit`, `WebPConfigPreset` and `WebPPictureInit`. The functions `WebPMemoryWriterInit`, `WebPMemoryWrite` and `WebPMemoryWriterClear` are defined in `src/enc/picture_enc.c`, so WP9 translates them. |
| `src/webp/demux.rs` | `src/webp/demux.h` | partly: `WEBP_DEMUX_ABI_VERSION`, `WebPDemuxState`, `WebPFormatFeature`, `WebPAnimDecoderOptions`, `WebPAnimInfo`. Exception WP8: `WebPDemux`, `WebPDemuxPartial`, `WebPIterator` and `WebPChunkIterator`. Exception WP14: `WebPAnimDecoderOptionsInit` and `WebPAnimDecoderNew`. |
| `src/webp/mux_types.rs` | `src/webp/mux_types.h` | yes: `WebPFeatureFlags`, `WebPMuxAnimDispose`, `WebPMuxAnimBlend`, `WebPData<'a>` with `WebPDataInit` as its `Default`, and `webp_data_clear` (`WebPDataClear`) and `webp_data_copy` (`WebPDataCopy`). |
| `src/utils/utils.rs` | `src/utils/utils.c`, `src/utils/utils.h`; except `WebPGetColorPalette` (WP3) | partly: `WEBP_MAX_ALLOCABLE_MEMORY`, `WEBP_ALIGN`, `CheckSizeOverflow`, the allocation helpers (`WebPSafeMalloc`, `WebPSafeCalloc`), `WebPCopyPlane`, `WebPCopyPixels`, `BitsLog2Floor`, `BitsCtz`, the `WebPMem*` and `GetLE*`/`PutLE*` helpers. Not here: `WebPMalloc` and `WebPFree` (WP12, ADR 0004); `WebPSafeFree` (dropped by the owner, deviation 5); `WebPLog2FloorC` and `WebPLogTable8bit` (the fallback that GCC and Clang never compile). |
| `src/utils/endian_inl_utils.rs` | `src/utils/endian_inl_utils.h` | yes: `HToLE16`, `HToLE32`, `BSwap16`, `BSwap32`, `BSwap64`. |
| `src/utils/random_utils.rs` | `src/utils/random_utils.c`, `src/utils/random_utils.h` | yes: the table, `VP8InitRandom`, `VP8RandomBits2`, `VP8RandomBits`. |
| `src/utils/thread_utils.rs` | `src/utils/thread_utils.c`, `src/utils/thread_utils.h` | yes: `WebPWorker<D1, D2>` with `Init`, `Reset`, `Sync`, `Launch`, `Execute` and `End` as methods, the `NOT_OK`, `OK` and `WORK` states, `had_error`, and `ThreadLoop`. Removed by decision (ADR 0003): `WebPWorkerInterface`, `WebPSetWorkerInterface` and `WebPGetWorkerInterface`. |

Structs whose layout cannot be compared in WP1 are listed under not-applicable
checks, with the package that owns their mirror.

## Evidence

Every differential test runs the C probe in a separate process, against the
release oracle built from the pinned `v1.6.0` source with
`python3 tools/build_oracle.py`. `python3 tools/check.py --oracle` runs them with
the `oracle` feature, so a missing oracle fails instead of skipping.

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp1_constants.rs` | `constants_and_enumerators_match_the_c_headers` | 173 values: every constant, enumerator and macro of the WP1 headers, the mode predicates for every `WEBP_CSP_MODE`, `WEBP_ABI_IS_INCOMPATIBLE` pairs, `MKFOURCC`, and the worker states. |
| `tests/wp1_constants.rs` | `public_struct_layout_matches_the_c_compiler` | size, alignment and every field offset of the six `repr(C)` structs without pointers: `WebPBitstreamFeatures`, `WebPDecoderOptions`, `WebPConfig`, `WebPAuxStats`, `WebPAnimDecoderOptions` and `WebPAnimInfo`. |
| `tests/wp1_alloc.rs` | `safe_allocation_decisions_match_the_c_helpers` | 43 accept-or-reject decisions of `WebPSafeMalloc` (27) and `WebPSafeCalloc` (16): zero counts, sizes 0 to 16 around `WEBP_MAX_ALLOCABLE_MEMORY`, sizes that fail the limit for one element, and products that wrap in 64 bits. |
| `tests/wp1_alloc.rs` | `rejected_sizes_never_reserve_memory` | the limit is `1 << 34` on this 64-bit target, and the rejected cases are rejected without a reservation. Needs no oracle. |
| `tests/wp1_random.rs` | `vp8_random_sequences_match_the_c_generator` | 4,200 generator values and 32 full-state dumps: 12 initializations (including `-1`, both zeros, `1.5` and `+inf`) with 250 values each, and 8 seeded states with arbitrary table words and indices, with 150 `VP8RandomBits2` calls each over every amplitude the encoder uses. |
| `tests/wp1_utils.rs` | `utility_helpers_match_the_c_functions` | 3,478 commands: `GetLE16`/`24`/`32`, `PutLE16`/`24`/`32`, `BitsLog2Floor`, `BitsCtz`, `WebPMemToUint32`/`Int32`, `WebPUint32ToMem`/`Int32ToMem`, `HToLE16`/`32`, `BSwap16`/`32`/`64`, `CheckSizeOverflow`, `WEBP_ALIGN`, and `WebPCopyPlane` on 200 random geometries with positive and negative strides and offsets. |
| `tests/wp1_copy_pixels.rs` | `copy_pixels_matches_the_c_function` | `WebPCopyPixels` on 200 random ARGB geometries: strides in both directions, offsets, empty pictures, a source that owns its pixels and a destination that borrows the caller's slice. Both pictures are initialized by the library's `WebPPictureInit`. |
| `tests/wp1_webp_data.rs` | `webp_data_helpers_match_the_c_helpers` | `WebPDataInit`, and `WebPDataCopy` followed by `WebPDataClear`, for 8 lengths from 1 to 65,537 bytes and for the NULL and empty sources: the copy result, the size, the bytes and the size after the clear. |
| `tests/wp1_webp_data.rs` | `clearing_leaves_borrowed_input_alone` | clearing a borrowed view leaves the input untouched. Needs no oracle. |
| `tests/wp1_thread.rs` | `worker_traces_match_the_c_worker_in_every_scenario` | 12 scenarios, each a trace of status, error flag, counter and return value after every call: sync on an idle worker, launch without reset, reset and launch, sync after a failing hook, reset in each of `NOT_OK`, `OK` and `WORK`, end on a never-launched worker, execute with failing and succeeding hooks, a null hook, two launches, and a failing job followed by a reset. |
| `tests/wp1_thread.rs` | `worker_status_follows_the_c_states` | the state sequence `NOT_OK`, `OK`, `OK` after sync, and `NOT_OK` after end. |

Local results on this branch, with the release oracle: `python3 tools/check.py
--oracle --msrv` passed. It runs format, Clippy with `-D warnings`, the upstream
pins, the module skeleton, the documentation links, `tools/unit_dependencies.py
--check`, and the tests of the dev, release and `checked` profiles, which report
39 test results with none failed or skipped. It then builds and tests with Rust
1.95, which reports 13 more results with none failed. The `asan-ubsan` variant
(`python3 tools/check.py --oracle-variant asan-ubsan --stage test`) reports the
same 39 results, with none failed and no sanitizer report.

## Not-applicable checks

These checks do not apply to WP1, or cannot be made, with the reason:

1. Layout of the `repr(C)` mirrors of the structs with pointers or Rust-only
   plane types: `WebPRGBABuffer`, `WebPYUVABuffer`, `WebPDecBuffer`,
   `WebPDecoderConfig`, `WebPPicture`, `WebPMemoryWriter`, `WebPData`, and the
   WP8 iterators. ADR 0004 gives the mirrors to WP12 to WP15, which compare them
   with the C compiler. WP1 has no C-visible behavior of these types to compare
   with, except through `WebPData` and `WebPCopyPixels`, which are compared.
2. Behavior of the buffers and the writer as a whole: `WebPInitDecBuffer` and
   `WebPFreeDecBuffer` (WP12), `WebPPictureAlloc`, `WebPPictureImport*` and the
   picture's conversions (WP9), and `WebPMemoryWrite` with its writer closure
   (WP9). No WP1 function operates on them.
3. The picture parameter of the writer and progress hook callbacks is omitted
   (ADR 0003), because no upstream callback reads a field of it. There is nothing
   to compare; the ABI mirror (WP13) passes the picture to C callbacks itself.
4. `WebPData` distinguishes `bytes == NULL` from empty data in C; a `Cow` slice
   cannot. The copy compares sizes, bytes and results; the NULL pointer is not
   observable through the Rust type.
5. `WebPDataCopy` for a size beyond the allocation limit: a Rust slice cannot have
   that length. The allocation test that `WebPDataCopy` applies is the one that
   `tests/wp1_alloc.rs` compares.
6. `WebPSafeCalloc` at the limit, `WebPSafeCalloc(WEBP_MAX_ALLOCABLE_MEMORY, 1)`.
   The C call is lazily zeroed, but the Rust calloc writes its zeros, which
   would touch 16 GiB. Malloc at the same boundary is compared, and calloc
   covers the same arithmetic on rejected and small cases.
7. Host refusal of a reservation. Rust reserves the same number of bytes as C
   only when the element size matches the C `size` argument; the cases use
   `[u8; N]` for `N` = 0, 1, 2, 4 and 8. The one exception, `(4096, 16)`, reserves
   4,096 bytes in Rust against 65,536 in C, which the host cannot refuse here.
8. `Reset` while the worker is `WORK` and the hook fails. In C the status is read
   without the lock, so the result depends on timing. The success variant is
   compared; the failure variant is not.
9. `BitsLog2Floor(0)` and `BitsCtz(0)`. The C builtins are undefined for zero.
   The Rust `bits_ctz(0)` returns 32, the value of the portable fallback, and
   `bits_log2_floor(0)` debug-asserts. Neither is compared.
10. `PRINT_MEM_INFO`, the debug memory bookkeeping, and `WEBP_SWAP_16BIT_CSP` are
    off in the reference configuration (ADR 0002). Nothing is ported for them.
11. `WebPGetColorPalette` (WP3), the MIPS and MSA sources (excluded in
    `tools/units.json`), and the SIMD variants (wave 4) are outside WP1.

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Worker design.** `WebPWorker<D1, D2>` holds a hook `fn(&mut D1, &mut D2) ->
   bool` and its two data values. `launch` moves them to the worker thread
   through the shared `Mutex` and `Condvar`, and `sync` or `end` returns them.
   While the worker works, the caller has no access to the data, which the borrow
   checker enforces. `set_job` panics while the worker works, where C would
   simply forbid the change. `Drop` ends the thread, because dropping a worker
   must not leak it; C leaves that to the caller. The owner accepted these on
   2026-11-06, and ADR 0003 records them.
2. **Panicking hook.** The worker thread catches a panic from the hook and counts
   it as a failed hook. C would terminate the process. Without the catch, `sync`
   would wait forever for a status that never arrives, because the panic poisons
   the state.
3. **Worker allocation.** The shared synchronization object is allocated in
   `WebPWorker::init` (C: `Init`) rather than in `Reset`, so a failed allocation
   cannot make `Reset` return `false`. A failed spawn still does: `Reset` returns
   `false` and the status stays `NOT_OK`, as ADR 0003 requires.
4. **Allocation helpers.** `webp_safe_malloc` and `webp_safe_calloc` apply
   `CheckSizeArgumentsOverflow` with the C `size` argument and reserve `nmemb`
   elements of the Rust type with `Vec::try_reserve_exact`. The vector is empty
   with a capacity of at least `nmemb`, so the caller initializes what it reads.
   `webp_safe_calloc` fills it with `T::default()`, which is the zero value of
   the numeric types.
5. **`WebPSafeFree`.** Dropping the owner's `Vec` is the Rust counterpart, so no
   function is translated.
6. **`VP8Random` indices.** `index1` and `index2` are `usize`, not C `int`. They are
   table positions in `0..55`, which ADR 0003 treats as offsets.
7. **Mixed-case enumerators.** `MODE_rgbA`, `MODE_bgrA`, `MODE_Argb` and
   `MODE_rgbA_4444` keep their exact C names under an item-level
   `#[expect(non_upper_case_globals)]`. ADR 0001 records the rule, decided
   2026-11-06.
8. **Slices and offsets.** Pointers into buffers are slices. Where a row or a
   field starts at an offset (`WebPCopyPlane`, `WebPCopyPixels`, `WebPMemToUint32`,
   `GetLE*`), the Rust code takes a slice or an offset. A row outside its slice
   panics where C reads or writes outside the buffer, which C leaves undefined.
9. **Byte order helpers.** `HToLE*` use `to_le`, and `BSwap*` use
   `swap_bytes`. The reference configuration compiles the portable path or the
   builtins; the assembly variants compute the same values.
10. **`MKFOURCC` and `WEBP_ALIGN`.** `MKFOURCC` takes `u8` characters; `WEBP_ALIGN`
    takes the address as a `usize`. Rust code keeps no raw pointers outside the
    C ABI.
11. **Storage type.** `PlaneStorage<'a, T>` is generic over the sample type (ADR 0003
    names the `u8` form), and `PlaneOffset` adds the region index that a
    multi-slice external storage needs. The `u32` ARGB storage uses the same type.
12. **`is_external_memory` is not stored.** The storage variant is the flag. Its
    value 2, which marks slow external memory, is a performance hint of WP12 and is
    not represented.
13. **`WebPPicture` fields.** `user_data`, `custom_ptr` and the unused pointer
    padding (`pad4`, `pad5`, `pad7`) are not kept; the closures of `writer` and
    `progress_hook` replace the first two. `extra_info` and `stats` are borrowed
    for `'a`, because the caller owns them. The `u32` padding is kept.
14. **Shared storage per plane group.** The Y, U, V and A planes of a picture share
    `yuva`, and its ARGB plane has `argb_storage`. A picture therefore holds
    either owned or borrowed planes per group, and a mixed picture cannot be
    represented. C allows a mix, which the library's own allocation never
    creates.
15. **Callback picture parameter omitted.** `WebPWriterFunction<'a>` is
    `Box<dyn FnMut(&[u8]) -> bool + 'a>` and `WebPProgressHook<'a>` is
    `Box<dyn FnMut(i32) -> bool + 'a>`. No upstream callback reads a field of the
    picture, and the context lives in the closure; the ABI mirror (WP13) passes
    the `repr(C)` picture pointer to C callbacks itself. The derivation is in ADR
    0003. A callback that needs a picture field would amend that record.
16. **`WebPWorkerInterface` removed.** ADR 0003 records the decision, and no
    public header declares the table.
17. **`WebPMemoryWriter.mem`.** The vector holds `max_size` bytes, and WP9's writer
    keeps the length and `size` in step.
18. **`WebPDataCopy`.** It returns `Option`, where C returns 0 on failure, and
    empty data copies to empty data, without the NULL pointer of C.
19. **`WebPDecBufferPlanes`.** This is the Rust name of the anonymous C union `u`.
20. **`WebPCopyPixels`.** It copies `u32` elements with the strides in pixels, where
    C copies bytes with four times the strides. The copied elements are the same;
    the byte form needs the reinterpretation that safe Rust forbids.

## Decisions applied on 2026-11-06

The planning session decided the five questions of the first report. The
decisions are applied as follows.

1. **Output buffers.** `WebPDecBuffer<'a>` and `WebPDecoderConfig<'a>` carry the
   lifetime. Their storage is one `PlaneStorage` (ADR 0003). The planes of
   `WebPRGBABuffer` and `WebPYUVABuffer` are offsets into it.
2. **`WebPData`.** One type `WebPData<'a>` with `Cow` semantics. `WebPDataInit` is
   `Default`, `WebPDataClear` resets to empty, and `WebPDataCopy` produces the
   owned form. `WebPIterator` and `WebPChunkIterator` are WP8's.
3. **Picture and callbacks.** `WebPPicture<'a>` with the same storage pattern.
   `writer` and `progress_hook` are boxed closures without the picture parameter
   (deviation 15). `WebPMemoryWriter` stays
   a struct, and its functions are WP9's. `WebPCopyPixels` is translated and
   compared.
4. **Worker interface.** Removed. The worker methods are called directly.
5. **Inline wrappers.** Each owner translates its wrapper next to its `Internal`
   function. The exceptions in `tools/units.json` record the assignment:
   - WP8: `WebPDemux`, `WebPDemuxPartial`, `WebPIterator`, `WebPChunkIterator`;
   - WP9: `WebPConfigInit`, `WebPConfigPreset`, `WebPPictureInit`;
   - WP12: `WebPInitDecBuffer`, `WebPGetFeatures`, `WebPInitDecoderConfig`,
     `WebPIDecGetYUV`;
   - WP14: `WebPAnimDecoderOptionsInit`, `WebPAnimDecoderNew`.

   The module headers of `src/webp/decode.rs`, `encode.rs` and `demux.rs` name
   them, and `tools/unit_dependencies.py --check` passes.

The naming rule for mixed-case enumerators is in ADR 0001.

## Open under ADR 0003, for other packages

- `WebPPictureView` reads its source picture, so it may need a shared borrow of
  the planes. WP9 raises this; a shared variant of `PlaneStorage` would amend
  ADR 0003.
- The `repr(C)` mirrors and their layout checks belong to WP12 to WP15.
- The decoder's lifetime `'a` reaches WP12 and WP16, and the ABI mirror binds the
  caller's pointer to it under `unsafe`.
