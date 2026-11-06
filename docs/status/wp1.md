# WP1: types, memory, workers

- Status: partly translated. Everything that ADR 0001, 0003 and 0004 decide is
  translated and verified; five questions stop the pointer-carrying structs, the
  worker interface table and the inline wrappers that call later packages
  (see [Open questions](#open-questions-stopped-decision-needed)).
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
"Partly" rows list what is left and why. Nothing in a "partly" row is a stub.

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/webp/types.rs` | `src/webp/types.h` | partly: `webp_abi_is_incompatible`. `WebPMalloc` and `WebPFree` are C-ABI memory and belong to `src/abi/types.rs` (WP12, ADR 0004), so they have no safe counterpart here. |
| `src/webp/format_constants.rs` | `src/webp/format_constants.h` | yes: every `#define`, `VP8LImageTransformType`, `MKFOURCC`. |
| `src/webp/decode.rs` | `src/webp/decode.h` | partly: `WEBP_DECODER_ABI_VERSION`, `VP8StatusCode`, `WEBP_CSP_MODE` and its values, `WebPIsPremultipliedMode`, `WebPIsAlphaMode`, `WebPIsRGBMode`, `WebPBitstreamFeatures`, `WebPDecoderOptions`. Open: `WebPRGBABuffer`, `WebPYUVABuffer`, `WebPDecBuffer` and `WebPDecoderConfig` (question 1); the inline wrappers (question 5). The declared functions belong to WP12. |
| `src/webp/encode.rs` | `src/webp/encode.h` | partly: `WEBP_ENCODER_ABI_VERSION`, `WEBP_MAX_DIMENSION`, `WebPImageHint`, `WebPConfig`, `WebPPreset`, `WebPAuxStats`, `WebPEncCSP`, `WebPEncodingError`. Open: `WebPPicture`, `WebPMemoryWriter`, `WebPWriterFunction`, `WebPProgressHook` (question 3); the inline wrappers (question 5). `WebPMemoryWriterInit`, `WebPMemoryWrite` and `WebPMemoryWriterClear` are defined in `src/enc/picture_enc.c`, so WP9 translates them. |
| `src/webp/demux.rs` | `src/webp/demux.h` | partly: `WEBP_DEMUX_ABI_VERSION`, `WebPDemuxState`, `WebPFormatFeature`, `WebPAnimDecoderOptions`, `WebPAnimInfo`. Open: `WebPIterator` and `WebPChunkIterator`, which hold `WebPData` (question 2); the inline wrappers (question 5). |
| `src/webp/mux_types.rs` | `src/webp/mux_types.h` | partly: `WebPFeatureFlags`, `WebPMuxAnimDispose`, `WebPMuxAnimBlend`. Open: `WebPData` and the inline helpers `WebPDataInit`, `WebPDataClear`, `WebPDataCopy` (question 2). |
| `src/utils/utils.rs` | `src/utils/utils.c`, `src/utils/utils.h`; except `WebPGetColorPalette` (WP3) | partly: `WEBP_MAX_ALLOCABLE_MEMORY`, `WEBP_ALIGN`, `CheckSizeOverflow`, the allocation helpers (`WebPSafeMalloc`, `WebPSafeCalloc`), `WebPCopyPlane`, `BitsLog2Floor`, `BitsCtz`, the `WebPMem*` and `GetLE*`/`PutLE*` helpers. Not here: `WebPMalloc` and `WebPFree` (WP12, ADR 0004); `WebPSafeFree` (dropped by the owner, see deviation 5); `WebPCopyPixels` (question 3); `WebPLog2FloorC` and `WebPLogTable8bit` (the fallback that GCC and Clang never compile). |
| `src/utils/endian_inl_utils.rs` | `src/utils/endian_inl_utils.h` | yes: `HToLE16`, `HToLE32`, `BSwap16`, `BSwap32`, `BSwap64`. |
| `src/utils/random_utils.rs` | `src/utils/random_utils.c`, `src/utils/random_utils.h` | yes: the table, `VP8InitRandom`, `VP8RandomBits2`, `VP8RandomBits`. |
| `src/utils/thread_utils.rs` | `src/utils/thread_utils.c`, `src/utils/thread_utils.h` | partly: `WebPWorker` with `Init`, `Reset`, `Sync`, `Launch`, `Execute` and `End` as methods, the `NOT_OK`, `OK` and `WORK` states, `had_error`, and `ThreadLoop`. Open: `WebPWorkerInterface`, `WebPSetWorkerInterface` and `WebPGetWorkerInterface` (question 4). |

## Evidence

Every differential test runs the C probe in a separate process, against the
release oracle built from the pinned `v1.6.0` source with
`python3 tools/build_oracle.py`. `python3 tools/check.py --oracle` runs them with
the `oracle` feature, so a missing oracle fails instead of skipping.

| Test file | Test | What it compares |
| --- | --- | --- |
| `tests/wp1_constants.rs` | `constants_and_enumerators_match_the_c_headers` | 173 values: every constant, enumerator and macro of the WP1 headers, the mode predicates for every `WEBP_CSP_MODE`, `WEBP_ABI_IS_INCOMPATIBLE` pairs, `MKFOURCC`, and the worker states. |
| `tests/wp1_constants.rs` | `public_struct_layout_matches_the_c_compiler` | size, alignment and every field offset of `WebPBitstreamFeatures`, `WebPDecoderOptions`, `WebPConfig`, `WebPAuxStats`, `WebPAnimDecoderOptions` and `WebPAnimInfo`, as ADR 0003 asks for `repr(C)` structs without pointers. |
| `tests/wp1_alloc.rs` | `safe_allocation_decisions_match_the_c_helpers` | 43 accept-or-reject decisions of `WebPSafeMalloc` (27) and `WebPSafeCalloc` (16): zero counts, sizes 0 to 16 around `WEBP_MAX_ALLOCABLE_MEMORY`, sizes that fail the limit for one element, and products that wrap in 64 bits. |
| `tests/wp1_alloc.rs` | `rejected_sizes_never_reserve_memory` | the limit is `1 << 34` on this 64-bit target, and the rejected cases are rejected without a reservation. Needs no oracle. |
| `tests/wp1_random.rs` | `vp8_random_sequences_match_the_c_generator` | 4,200 generator values and 32 full-state dumps: 12 initializations (including `-1`, both zeros, `1.5` and `+inf`) with 250 values each, and 8 seeded states with arbitrary table words and indices, with 150 `VP8RandomBits2` calls each over every amplitude the encoder uses. |
| `tests/wp1_utils.rs` | `utility_helpers_match_the_c_functions` | 3,478 commands: `GetLE16`/`24`/`32`, `PutLE16`/`24`/`32`, `BitsLog2Floor`, `BitsCtz`, `WebPMemToUint32`/`Int32`, `WebPUint32ToMem`/`Int32ToMem`, `HToLE16`/`32`, `BSwap16`/`32`/`64`, `CheckSizeOverflow`, `WEBP_ALIGN`, and `WebPCopyPlane` on 200 random geometries with positive and negative strides and offsets. |
| `tests/wp1_thread.rs` | `worker_traces_match_the_c_worker_in_every_scenario` | 12 scenarios, each a trace of status, error flag, counter and return value after every call: sync on an idle worker, launch without reset, reset and launch, sync after a failing hook, reset in each of `NOT_OK`, `OK` and `WORK`, end on a never-launched worker, execute with failing and succeeding hooks, a null hook, two launches, and a failing job followed by a reset. |
| `tests/wp1_thread.rs` | `worker_status_follows_the_c_states` | the state sequence `NOT_OK`, `OK`, `OK` after sync, and `NOT_OK` after end. |

Local result of `python3 tools/check.py --oracle` on the release oracle: format,
Clippy with `-D warnings`, upstream pins, module skeleton, documentation links,
`tools/unit_dependencies.py --check`, and 30 tests in the dev, release and
`checked` profiles, all passed with no skipped test.

With the `asan-ubsan` oracle (`python3 tools/check.py --oracle-variant asan-ubsan
--stage test`), the same 30 tests passed in the three profiles, and neither
sanitizer reported anything. The worker test, which runs real threads, passed 30
times in a row in the release profile.

## Not-applicable checks

These checks do not apply to WP1, or cannot be made, with the reason:

1. Layout of `repr(C)` mirrors of structs with pointers or callbacks
   (`WebPDecBuffer`, `WebPDecoderConfig`, `WebPPicture`, `WebPMemoryWriter`,
   `WebPData`, `WebPIterator`, `WebPChunkIterator`). Their safe representations
   are open (questions 1 to 3), and ADR 0004 gives their mirrors to WP12 to WP15.
2. `WebPSafeCalloc` at the limit, `WebPSafeCalloc(WEBP_MAX_ALLOCABLE_MEMORY, 1)`.
   The C call is lazily zeroed, but the Rust calloc writes its zeros, which
   would touch 16 GiB. Malloc at the same boundary is compared, and calloc
   covers the same arithmetic on rejected and small cases.
3. Host refusal of a reservation. Rust reserves the same number of bytes as C
   only when the element size matches the C `size` argument; the cases use
   `[u8; N]` for `N` = 0, 1, 2, 4 and 8. The one exception, `(4096, 16)`, reserves
   4,096 bytes in Rust against 65,536 in C, which the host cannot refuse here.
4. `Reset` while the worker is `WORK` and the hook fails. In C the status is read
   without the lock, so the result depends on timing. The success variant is
   compared; the failure variant is not.
5. `BitsLog2Floor(0)` and `BitsCtz(0)`. The C builtins are undefined for zero.
   The Rust `bits_ctz(0)` returns 32, the value of the portable fallback, and
   `bits_log2_floor(0)` debug-asserts. Neither is compared.
6. `PRINT_MEM_INFO`, the debug memory bookkeeping, and `WEBP_SWAP_16BIT_CSP` are
   off in the reference configuration (ADR 0002). Nothing is ported for them.
7. `WebPGetColorPalette` (WP3), the MIPS and MSA sources (excluded in
   `tools/units.json`), and the SIMD variants (wave 4) are outside WP1.

## Recorded deviations

Deviations are changes of representation that the ADRs allow, or behavior
that the Rust side cannot reproduce exactly. Each records the reason.

1. **Worker design.** ADR 0003 leaves the concrete form of a callback to the
   owning package, and WP1 owns `WebPWorker`. `WebPWorker<D1, D2>` holds a hook
   `fn(&mut D1, &mut D2) -> bool` and its two data values. `launch` moves them to
   the worker thread through the shared `Mutex` and `Condvar`, and `sync` or `end`
   returns them. While the worker works, the caller has no access to the data;
   the borrow checker enforces this. `set_job` panics while the worker works,
   where C would simply forbid the change. `Drop` ends the thread, because dropping a
   worker must not leak it; C leaves that to the caller.
2. **Panicking hook.** The worker thread catches a panic from the hook and counts
   it as a failed hook. C would terminate the process. Without the catch, `sync`
   would wait forever for a status that never arrives, because the panic poisons
   the state.
3. **Worker allocation.** The shared synchronization object is allocated in
   `WebPWorker::init` (C: `Init`) rather than in `Reset`, so a failed allocation
   cannot make `Reset` return `false`. A failed spawn still does: `Reset` returns
   `false` and the status stays `NOT_OK`, as ADR 0003 requires.
4. **Allocation helpers.** `webp_safe_malloc` and `webp_safe_calloc` apply
   `CheckSizeArgumentsOverflow` with the C `size` argument and reserve
   `nmemb` elements of the Rust type with `Vec::try_reserve_exact`. The Vec is
   empty with a capacity of at least `nmemb`, so the caller initializes what it
   reads, as ADR 0003 requires. `webp_safe_calloc` fills the vector with
   `T::default()`, which is the zero value of the numeric types.
5. **`WebPSafeFree`.** Dropping the owner's `Vec` is the Rust counterpart, so no
   function is translated.
6. **`VP8Random` indices.** `index1` and `index2` are `usize`, not C `int`. They are
   table positions in `0..55` (ADR 0003: an index becomes an offset of type
   `usize`).
7. **Mixed-case enumerators.** `MODE_rgbA`, `MODE_bgrA`, `MODE_Argb` and
   `MODE_rgbA_4444` keep their C names under an item-level
   `#[expect(non_upper_case_globals)]`. ADR 0001 says enumerators are upper snake
   case, and these are not. The integrator may want a rule for them.
8. **Slices and offsets.** Pointers into buffers are slices. Where a row or a
   field starts at an offset (`WebPCopyPlane`, `WebPMemToUint32`, `GetLE*`),
   the Rust code takes a slice or an offset. A row outside its slice panics where
   C reads or writes outside the buffer, which C leaves undefined.
9. **Byte order helpers.** `HToLE*` use `to_le`, and `BSwap*` use
   `swap_bytes`. The reference configuration compiles the portable path or the
   builtins; the assembly variants compute the same values.
10. **`MKFOURCC` and `WEBP_ALIGN`.** `MKFOURCC` takes `u8` characters; `WEBP_ALIGN`
    takes the address as a `usize`. Rust code keeps no raw pointers outside the
    C ABI.

### Open questions (stopped, decision needed)

These five questions are the points where the ADRs do not decide the form. The
package stopped at them instead of deciding, as the work plan asks. The other
parts of WP1 do not depend on them, and sub-wave 1b (WP2, WP4, WP5) needs only
what is translated.

1. **Output buffers.** `WebPRGBABuffer`, `WebPYUVABuffer`, `WebPDecBuffer` (its
   `u` union becomes an enum per ADR 0003) and `WebPDecoderConfig` hold pointers
   to memory that is either the decoder's own (`private_memory`, owned) or the
   caller's (`is_external_memory`, borrowed). ADR 0003 covers each case, but not a
   struct that holds both. Options: (a) a lifetime parameter `WebPDecBuffer<'a>`
   with a storage enum `Internal(Vec<u8>)` or `External(&'a mut [u8])` per plane,
   and views as offsets into the storage; (b) owned storage only, which would
   copy the caller's buffer and breaks the borrowed rule. Recommendation: (a).
   The lifetime reaches WP12, WP16 and the safe API, so the choice should be made
   before WP6.
2. **`WebPData`.** One C struct is borrowed input in some calls (for example the
   mux and demux entry points) and owned output in others (`WebPDataCopy`, the
   mux assembly), freed with `WebPDataClear`. Options: (a) one type with a
   `Cow<'a, [u8]>`-like representation; (b) two types, one borrowed and one owned,
   with the C name on the borrowed one. `WebPIterator` and `WebPChunkIterator`
   contain a `WebPData` and a `void*` private field, so their shape follows this
   choice. This blocks WP8 and WP15.
3. **Picture and writer.** `WebPPicture` has the same owned and borrowed planes
   as question 1, and its `writer` callback receives the picture while the
   picture holds the callback's context (`custom_ptr`). `WebPMemoryWriter` is the
   context of the default writer. `WebPCopyPixels` takes a picture as well.
   Options: decide together with question 1 for the planes; for the callbacks,
   (a) a `Box<dyn FnMut(&[u8], &WebPPicture) -> bool>` taken out of the picture
   before each call, or (b) a context passed separately. This blocks WP9, WP13
   and WP17.
4. **Worker interface table.** Every library call site uses
   `WebPGetWorkerInterface()` and its function pointers (`alpha_enc.c`,
   `frame_dec.c`, `vp8_dec.c` and others), and `WebPSetWorkerInterface` replaces
   the table. The Rust port calls the worker methods directly. Options: (a) drop
   the replacement mechanism, since no library code replaces the table and
   ADR 0004 keeps the internal header out of the exports; (b) keep a process-wide
   table of function pointers. Recommendation: (a). This removes a runtime
   extension point, so it needs your approval.
5. **Inline wrappers that call later packages.** The task asks WP1 to translate
   the `*Internal`-wrapping inline initializers, and it forbids placeholders for
   the functions they call. The conflict is real: a wrapper cannot compile
   without its callee. The wrappers and their owners are:
   - `WebPInitDecBuffer` (`WebPInitDecBufferInternal`, `src/dec/buffer_dec.c`, WP12);
   - `WebPGetFeatures` (`WebPGetFeaturesInternal`, `src/dec/webp_dec.c`, WP12);
   - `WebPInitDecoderConfig` (`WebPInitDecoderConfigInternal`, `src/dec/webp_dec.c`, WP12);
   - `WebPIDecGetYUV` (`WebPIDecGetYUVA`, `src/dec/idec_dec.c`, WP12);
   - `WebPConfigInit` and `WebPConfigPreset` (`WebPConfigInitInternal`, `src/enc/config_enc.c`, WP9);
   - `WebPPictureInit` (`WebPPictureInitInternal`, `src/enc/picture_enc.c`, WP9);
   - `WebPDemux` and `WebPDemuxPartial` (`WebPDemuxInternal`, `src/demux/demux.c`, WP8);
   - `WebPAnimDecoderOptionsInit` and `WebPAnimDecoderNew` (`src/demux/anim_decode.c`, WP14).
   Proposal: each owner translates its wrapper next to its `Internal` function.
   The WP1 header modules then stop at the structs and constants. Needs your
   confirmation.
