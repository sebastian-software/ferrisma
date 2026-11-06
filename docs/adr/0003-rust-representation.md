# Map pointers, errors, allocation, dispatch and threads explicitly

- Status: accepted
- Created: 2026-11-05
- Updated: 2026-11-06

## Context

libwebp passes raw pointers with strides, pairs `malloc` with `free`, reports
errors through integer flags and status fields, selects DSP functions through
global function pointers and runs work on pthreads. Safe Rust cannot copy these
constructs literally, and `#![deny(unsafe_code)]` applies outside the C ABI
([ADR 0005](0005-toolchain-and-unsafe-policy.md)). Parallel packages need the
same answers without coordinating; a package that meets a construct these rules
do not cover stops and asks for an amendment instead of deciding alone.

## Decision

**Integers and conversions.** C types map to fixed-width Rust types: `int` to
`i32`, `uint32_t` to `u32`, `size_t` to `usize`, `uint64_t` to `u64`, `int64_t` to
`i64`, `uint8_t` to `u8`, `float` to `f32`, `double` to `f64`. Keep every width,
signedness and conversion point of the source. Unsigned C arithmetic wraps, so
use `wrapping_*` operations where the source can wrap; signed overflow is
undefined in C, so plain Rust arithmetic is correct there and the `checked`
profile reports a violation. Keep shifts, truncating casts and float-to-integer
conversions where the source has them; C float-to-integer conversion of an
out-of-range value is undefined, Rust's `as` saturates, and such inputs are
recorded and excluded.

**Pointers.** A pointer into a buffer becomes a slice plus a byte or element
offset (`usize`); a pointer and length pair becomes a slice. A C stride keeps
its C type, including the negative stride that `flip` produces; a row start is
computed from the offset and the signed stride with checked conversion to
`usize`. A nullable pointer becomes an `Option`. Sentinels such as
end-of-buffer pointers become offsets with the same meaning. Pointers that
alias parts of one structure while it is mutated are split into separate
borrows or owned parts; record each such split in the package's status file.

**Readers over input (decided 2026-11-06).** A reader or cursor that consumes an
input stores positions and the input length as `usize` values, and never a
borrow of the input. The owner holds the input and passes it to each function
that reads it, as in `vp8_get_bit(br, data, prob)`. A pointer that may be `NULL`
before the reader is set up becomes an explicit flag (`VP8BitReader::has_buf`).
`VP8RemapBitReader(br, offset)` adds the signed offset to the positions, as C
adds it to the pointers, when the owner moves the same bytes by that many indexes
of its input. A reader then sits next to the buffer it reads in one struct, such
as the incremental decoder of WP12, without a self-reference. The rule covers
every reader over a buffer that its owner also holds; the buffer's own type is
unchanged.

**Ownership.** `malloc`/`free` pairs become owned values (`Vec`, `Box`) dropped
where the source frees them. A buffer that C callers own stays borrowed
(`&[u8]`, `&mut [u8]`). Memory handed to or received from C belongs to the
ABI module ([ADR 0004](0004-c-abi-boundary.md)).

**Raw data (decided 2026-11-06).** `WebPData<'a>` has `Cow<'a, [u8]>` semantics:
borrowed input or owned output in one type. `WebPDataInit` is `Default`, which is
empty and borrowed. `WebPDataClear` resets the value to empty, which drops owned
bytes. `WebPDataCopy` returns the owned variant, and it applies the allocation test
of `WebPMalloc`. `WebPIterator` and `WebPChunkIterator` are WP8's, because they hold
the demuxer's state; `tools/units.json` records them as exceptions.

**Allocation limits.** `WebPSafeMalloc` and `WebPSafeCalloc` decide which large
or malformed inputs fail, so their checks are part of the behavior. WP1
provides a fallible helper that applies the same test with the same arithmetic:
success for `nmemb == 0`; failure when `size > WEBP_MAX_ALLOCABLE_MEMORY / nmemb`
or when `nmemb * size` does not fit in `usize`; then a fallible reservation
(`Vec::try_reserve_exact`) so that a refused host allocation also fails instead
of aborting. `WEBP_MAX_ALLOCABLE_MEMORY` is `1 << 34` on targets whose `usize`
exceeds 34 bits and `(1 << 31) - (1 << 16)` otherwise, as in `utils.h`. The
`size` argument is the C `sizeof` of the element, not the Rust size: each
translated type whose arrays are allocated has a constant with its C size,
verified by a probe. Where C allocates one block and carves several arrays
from it, the Rust code checks the same total, including alignment padding,
before it allocates its separate buffers. Rust initializes every allocation;
a source path that reads `malloc`ed memory before writing it is undefined and
is recorded. Allocation failure produces the source's status
(`VP8_STATUS_OUT_OF_MEMORY`, `VP8_ENC_ERROR_OUT_OF_MEMORY`). The debug-only
`PRINT_MEM_INFO` counters are not translated.

**Enums.** A C enum becomes a `#[repr(transparent)]` newtype over its C
underlying type with one associated constant per enumerator, keeping the
enumerator names and values: `pub struct VP8StatusCode(pub u32)` with
`VP8StatusCode::VP8_STATUS_OK`. Clang and GCC give an enum without negative
enumerators the type `unsigned int` and one with negative enumerators (for
example `WebPMuxError`, `WebPDemuxState`) the type `int`, so the newtype wraps
`u32` or `i32` accordingly, and range checks such as `image_hint >=
WEBP_HINT_LAST` behave as in C for every stored value. No Rust enum with
discriminants is created from a C enum, so any value C stores is representable.

**Structs and unions.** Structs keep their C field order and field names. A
public struct without pointers or callbacks (`WebPBitstreamFeatures`,
`WebPDecoderOptions`, `WebPConfig`, `WebPAuxStats` and similar) is
`#[repr(C)]` with the same field types, including its `pad` arrays, so that the
C ABI can use it directly; WP1 verifies size, alignment and field offsets
against the C compiler. A struct with pointers or callbacks gets a safe
representation in its module and a `repr(C)` mirror in `src/abi`. A union whose
active member is selected by another field (`WebPDecBuffer::u`, selected by
`colorspace`) becomes a Rust `enum` with one variant per member; the ABI mirror
keeps the union.

**Buffers and planes (decided 2026-11-06).** The samples of a buffer are one
storage, `PlaneStorage<'a, T>` in `src/webp/types.rs`. `Internal(Vec<T>)` holds
memory that the object allocated (`private_memory`, `memory_`, `memory_argb_`).
`External(Vec<&'a mut [T]>)` holds the caller's slices: one slice for an RGBA or
ARGB buffer, and up to four (Y, U, V and A, in that order) for a YUVA buffer. A
plane is a `PlaneOffset { region, offset }`, where `region` is the slice index
(always 0 for `Internal`, which holds every plane of a buffer) and `offset` counts
elements. `WebPRGBABuffer` and `WebPYUVABuffer` describe their planes with these
offsets, strides and sizes. The `u` union of `WebPDecBuffer` becomes the enum
`WebPDecBufferPlanes`, because the C union is anonymous. `WebPDecBuffer<'a>`,
`WebPDecoderConfig<'a>` and `WebPPicture<'a>` carry the lifetime. The C field
`is_external_memory` is the variant of the storage and is not stored again. Its
value 2, which marks slow external memory, is a performance hint of WP12 and is not
represented. The lifetime reaches `WebPDecParams`, `WebPIDecoder` and the safe
decoder API (WP12, WP16). In the ABI mirror (ADR 0004), the caller's pointer is
bound to that lifetime under `unsafe`, with a `// SAFETY:` argument for each
operation. The planes of one picture share one storage, so a picture is owned or
borrowed as a whole. `WebPPicture` holds `stats` and `extra_info` as `&'a mut`
values, because the caller owns them.

**Errors.** Status codes keep their numeric values.

- A function that returns a `VP8StatusCode`, `WebPEncodingError` or another
  status enum with one success value returns `Result<T, Code>`, where `Err`
  never carries the success value (`VP8_STATUS_OK`, `VP8_ENC_OK`,
  `WEBP_MUX_OK`). `VP8_STATUS_SUSPENDED` is an `Err`, as it is not `OK` in C.
  Out-parameters that C writes only on success become part of `T`; those it
  also writes on failure stay `&mut` parameters, so partial state survives.
- The setters that record the first error (`VP8SetError`, `VP8LSetError`,
  `WebPEncodingSetError`) keep their precedence rules and their status fields
  (`dec->status`, `dec->error_msg`, `picture->error_code`) and return the code
  that the field holds afterwards. A function that returns an `int` failure
  after calling such a setter returns `Result<T, Code>` with that recorded code,
  so `?` propagates exactly what a C caller would read from the field.
- An `int` used only as a predicate stays a `bool`; counts and values keep
  their integer types.
- `assert` becomes `debug_assert!`, active in the dev and `checked` profiles
  like an upstream debug build and absent in release like `NDEBUG`.

**Callbacks.** A callback with a `void*` context (`VP8Io` hooks,
`WebPWriterFunction`, `WebPProgressHook`, the worker hook) becomes a Rust
callback whose context is part of the callback value (a closure, or a trait
object owned or borrowed by the structure). Keep every call point, the order of
calls and the meaning of their return values. The package that owns the
structure chooses the concrete form and records it.

**Concrete callback forms (decided 2026-11-06).** `WebPWriterFunction` is
`Box<dyn FnMut(&[u8]) -> bool + 'a>` and `WebPProgressHook` is
`Box<dyn FnMut(i32) -> bool + 'a>`. Each closure owns its context, which replaces
`custom_ptr` and `user_data`. The picture parameter of both C callbacks is omitted:
no upstream callback reads a field of it, and the context lives in the closure.
The derivation is as follows. `WebPMemoryWrite` (`src/enc/picture_enc.c`) reads only
`custom_ptr`. `DummyWriter` (`src/enc/picture_enc.c`) reads no field. The `MyWriter`
of `examples/cwebp.c` reads only `custom_ptr`, and its `ProgressReport` reads only
`user_data`. The library passes the picture on to the hook in `WebPReportProgress`
(`src/enc/webp_enc.c`) and `VP8IteratorProgress` (`src/enc/iterator_enc.c`), and
`imageio/` installs neither a writer nor a hook. The ABI mirror (WP13) passes its
`repr(C)` picture pointer to C callbacks itself, so the C signature stays intact at
the boundary. `WebPMemoryWriter` stays a struct in `src/webp/encode.rs`, and WP9's
functions implement the writer form. `WebPCopyPixels` copies the ARGB plane with the
row arithmetic of `WebPCopyPlane`, on `u32` samples, with the strides in pixels.

**DSP dispatch.** A global function pointer such as `VP8Transform` becomes a
plain function with the snake-case name of the pointer (`vp8_transform`) that
calls the portable `_C` implementation; a global table such as
`VP8PredLuma4[NUM_BMODES]` becomes a function that takes the index as its first
argument and calls through a `static` table of portable functions. Callers
never see function pointers, so the wave 4 SIMD packages can add runtime
selection inside these functions without changing a signature. An `Init`
function that only assigns portable pointers (`VP8DspInit`, `VP8LDspInit`,
`WebPInitSamplers` and the like) has no Rust counterpart, and its calls are
omitted with an `// upstream:` comment naming it. Work beyond pointer
assignment, such as filling a table, is kept: a lazily initialized table
becomes a `std::sync::LazyLock` static with the same values and computation; a
constant table becomes a `const` or `static` array.
`VP8GetCPUInfo` stays a query: WP4 translates it as a function that answers for
the running CPU, and in the reference configuration, where upstream compiles no
SIMD, no dispatch consults it. In wave 4 the dispatch functions consult the
query, and the ABI module's exported `VP8GetCPUInfo` pointer, which the upstream
tools set to `NULL` for `-noasm`, overrides it.

**Threads.** `WebPWorker` keeps the `NOT_OK`, `OK` and `WORK` states, `had_error`
and the `Reset`, `Sync`, `Launch`, `Execute` and `End` semantics, including
the failure of `Reset` when no thread can be started. The thread is a
`std::thread`; a `Mutex` and a `Condvar` replace the pthread mutex and condition
variable. The hook and its two data pointers become a hook over data that the
worker owns while it is in `WORK` and returns at `Sync`; data shared in C
between the calling thread and the hook is split so that each side owns what
it mutates, at the source's `Launch` and `Sync` points (WP6, WP10, WP11, WP13
apply this to their workers and record the split). WP1 chose the concrete form of
`WebPWorker<D1, D2>`: the hook is `fn(&mut D1, &mut D2) -> bool`; the job moves to
the thread at `Launch` and comes back at `Sync` or `End`; `set_job` panics while the
worker works; a panicking hook counts as a failed hook, so that `Sync` returns; `Drop`
ends the thread (decided 2026-11-06). The replaceable worker interface
(`WebPWorkerInterface`, `WebPSetWorkerInterface` and `WebPGetWorkerInterface`) is
removed (decided 2026-11-06). It is an internal extension point that no public
header declares, and the ABI export does not need it. Callers use the worker methods
directly.

**Floating point.** Keep `f32` and `f64` operations in source order with the
same constants and the same libm functions (`f64::powf` for `pow`, `f32::exp`
for `expf`, and so on). Rust never contracts `a * b + c` into a fused
multiply-add, matching the oracle's `-ffp-contract=off`; use `mul_add` only
where the source calls `fma`.

## Consequences

Safe Rust can express the whole portable library under these rules, with
explicit costs: zero-initialized allocations, enum newtypes instead of Rust
enums, and some data splits around worker threads. Every translated module
follows the same patterns, and the wave 4 SIMD work does not touch call sites.

## Review triggers

Amend this record when a package meets a construct it does not cover, when a
rule forces a change of observable behavior, or before wave 4 adds runtime
selection.

Open under this record (2026-11-06): `WebPPictureView` reads its source picture,
so it may need a shared borrow of the planes. `PlaneStorage` would then gain a
shared variant, which amends this record; WP9 raises it. A callback that needs a
field of the picture would bring the picture parameter back, which amends this
record.
