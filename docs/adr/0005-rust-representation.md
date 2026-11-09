# Represent the libjpeg object model in safe Rust

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-09

## Context

libjpeg is object-oriented C. A decompression object (`jpeg_decompress_struct`)
holds public parameters and state, plus pointers to about a dozen internal
modules (`master`, `inputctl`, `marker`, `entropy`, `coef`, `idct`, `main`,
`post`, `upsample`, `cconvert`). Each module is a struct of function pointers
and public state (`jpeg_entropy_decoder`, `jpeg_d_main_controller` and the
others in `jpegint.h`); the implementing file embeds it as the first member of
a private struct (`huff_entropy_decoder`, `my_main_controller`) and casts back
to that struct. Modules select their methods at run time, per configuration and
per scan: `jdphuff.c` installs one of four `decode_mcu` functions, and
`jdinput.c` points `inputctl->consume_input` at the coefficient controller's
`consume_data`. All memory comes from a pool allocator (`jmemmgr.c`) with
limits and error codes. Samples travel as arrays of row pointers (`JSAMPARRAY`,
`JSAMPIMAGE`) that several modules rotate and alias. Errors call
`err->error_exit`, which applications leave with `longjmp`. Data sources may
suspend. The compression and decompression objects share a prefix of fields
and are passed to common code as `j_common_ptr`.

Safe Rust cannot copy these constructs, and `#![deny(unsafe_code)]` applies to
every translated module ([ADR 0003](0003-toolchain-and-unsafe-policy.md)).
Work packages 1 to 6 translate in parallel once this record is accepted, so
they need the same answers without coordinating. libwebp-rs settled the same
class of questions in its ADR 0003; this record follows it where the
constructs match. A package that meets a
construct these rules do not cover stops and asks for an amendment.

## Decision

### Types, names and arithmetic

**Integer types.** Types map to fixed-width Rust types as the 8-bit
instantiation defines them: `JSAMPLE` to `u8`, `JCOEF` to `i16`, `JDIMENSION`
to `u32`, `UINT8` to `u8`, `UINT16` to `u16`, `INT16` to `i16`, `int` to `i32`,
`unsigned int` to `u32`, `size_t` to `usize`, `float` (`FAST_FLOAT`) to `f32`,
`double` to `f64`. `long`, `JLONG` and `INT32` (all `long` in upstream) map to
`core::ffi::c_long`, and `unsigned long` to `core::ffi::c_ulong`, so that every
target gets the width its C compiler uses: 64 bits on both CI hosts. Keep every
width, signedness and conversion point of the source. Unsigned C arithmetic
wraps, so use `wrapping_*` where the source can wrap; signed overflow is
undefined in C, so plain Rust arithmetic is correct there and the `checked`
profile reports a violation. Where upstream exempts a function from the
sanitizer's integer-overflow checks because corrupt input overflows it (the
DC prediction sums of `jdhuff.c`'s `decode_mcu_slow` and `decode_mcu_fast`),
the port uses `wrapping_*` there, as the compiled C wraps, so that such input
decodes in every profile. `RIGHT_SHIFT_IS_UNSIGNED` is undefined on both
hosts, so a signed right shift is Rust's arithmetic `>>`. A conversion from
`float` to an integer is `as`, which truncates as C does for every value in
range and saturates where C's result is undefined (an IDCT output far out of
range); the differential tests keep to inputs that C defines.

**`boolean`.** `boolean` is `int` in upstream. A `boolean` field, parameter or
return value becomes `bool`; `TRUE` and `FALSE` become `true` and `false`.

**Enums.** A C enum (`J_COLOR_SPACE`, `J_DCT_METHOD`, `J_DITHER_MODE`,
`J_BUF_MODE`) becomes a `#[repr(transparent)]` newtype over its C underlying
type with one associated constant per enumerator, keeping names and values:
`pub struct J_COLOR_SPACE(pub u32)` with `J_COLOR_SPACE::JCS_YCbCr`. Clang and
GCC give these enums the type `unsigned int`. No Rust enum with discriminants is
created from a C enum, so every value an application can store stays
representable, and range checks behave as in C.

**Names.** Translated items keep their C names verbatim: `jpeg_read_header`,
`decode_mcu_DC_first`, `jpeg_decompress_struct`, `JSAMPARRAY`, field
`MCU_ctr`. Most libjpeg names are lower case already, and verbatim names keep a
`grep` against upstream exact. Where a name violates a rustc naming lint, the
item carries an item-scoped `#[expect(non_camel_case_types, reason = "C type
name")]`, `#[expect(non_snake_case, reason = "C field names")]` or
`#[expect(non_upper_case_globals, ...)]`, as libwebp-rs does for its C type
names; a struct whose fields keep C names takes one `expect` on the struct.

**Macros and translation-time headers.** A function-like macro becomes an
`#[inline]` function in the module of the header or file that defines it
(`DESCALE`, `MULTIPLY`, `HUFF_DECODE` becomes a function with the macro's
arguments and its `goto` targets as return values). Three headers have no
module ([tools/units.json](../../tools/units.json), category `macro`):
`jinclude.h` (system headers; `MEMZERO` and `MEMCOPY` become slice `fill` and
`copy_from_slice` at each use, `SIZEOF` becomes the C size where it feeds
arithmetic), `jsamplecomp.h` (precision-dependent names resolve to the 8-bit
names: `_JSAMPLE` is `JSAMPLE`, `_jinit_color_deconverter` is
`jinit_color_deconverter`) and `jpegapicomp.h` (field aliases resolve for
`JPEG_LIB_VERSION` 62: `_DCT_scaled_size` is `DCT_scaled_size`,
`_jpeg_width` is `image_width`). Configuration macros become constants in
`src/jconfig.rs` and `src/jmorecfg.rs` with the values that the reference
oracle's `jconfig.h` defines (`oracle.json`, `jconfig_h`). A `#ifdef` on such a
constant becomes the branch that the reference configuration compiles.

### The common object

`jpeg_common_struct` exists in C so that `jcomapi.c`, `jerror.c` and
`jmemmgr.c` serve both object types. The encoder is out of scope
([ADR 0001](0001-mechanical-port.md)), so a `j_common_ptr` parameter becomes
`&mut jpeg_decompress_struct`: `pub type j_common_ptr<'a, 'src> = &'a mut
jpeg_decompress_struct<'src>`. The common fields (`err`, `mem`, `progress`,
`client_data`, `is_decompressor`, `global_state`) are ordinary fields of the
decompression struct. `is_decompressor` is always `true`. A branch that tests
`!is_decompressor` or casts to `j_compress_ptr` is untranslated, like the rest
of the encoder, and the function keeps an `// upstream:` comment that names it.
Casts from `j_common_ptr` to `j_decompress_ptr` (`jpeg_abort` reaching
`master->marker_list_end`) disappear.

### Modules and method tables

**Module objects.** Each module pointer of `jpeg_decompress_struct` becomes a
field `Option<Box<M>>`, where `M` is the module's Rust type. For a slot that
upstream fills with one implementation in scope, `M` is that implementation's
private struct (`master: Option<Box<my_decomp_master>>`). For a slot with
several implementations, `M` is an enum with one variant per in-scope
implementation, named after the C struct: `entropy` holds
`jpeg_entropy_decoder_impl::{huff_entropy_decoder, phuff_entropy_decoder,
arith_entropy_decoder}`, and `upsample` holds the plain or the merged
upsampler. `None` stands for C's `NULL` before `jinit_*` runs.

**Module interfaces arrive with their modules.** Work package 1 translates
the constants, enums, typedefs and macros of `jpegint.h` and the fields of
`jpeg_decompress_struct` that it can represent on its own: the common fields,
the source manager, the scalars, and the quantization and Huffman table
pointers. The package that translates a module adds, together, the module's
interface struct from `jpegint.h`, its private struct, its slot in
`jpeg_decompress_struct`, and the fields of the object whose representation
depends on how that module uses them (`comp_info`, `cur_comp_info`,
`coef_bits`, `colormap`, `sample_range_limit`, `marker_list`,
`jpeg_component_info::dct_table`), because only that package can settle their
signatures and borrow splits. `jpeg_abort`, which clears
`master->marker_list_end` and `marker_list`, is translated by work package 2
with `jpeg_CreateDecompress`, which creates both. Work package 2 also
translates the struct of `jdmaster.h` (the data fields of
`jpeg_decomp_master` and `my_decomp_master`), because `jpeg_CreateDecompress`
creates it, and the parts of `jpeg_entropy_decoder` and
`jpeg_d_coef_controller` that `jdinput.c`'s `start_input_pass` calls; their
slots were enums without variants until work package 3 added the three
entropy decoders and work package 4 the coefficient controller, whose slot
then holds `my_coef_controller` itself, its one implementation in scope. A
function that calls a module of a later package
and that its own package can leave out, such as `jpeg_finish_decompress`
calling `master->finish_output_pass`, arrives with that later package.

**Public part.** The public struct from `jpegint.h` is the first field `pub_`
of each private struct, as `pub` is in C: `entropy->pub.insufficient_data`
becomes `entropy.pub_.insufficient_data`. A module type that is an enum has an
accessor that returns the `pub_` of whichever variant it holds, so that other
modules read `cinfo->entropy->insufficient_data` as
`cinfo.entropy().insufficient_data`.

**Methods are function pointers.** A method field of a public struct
(`decode_mcu`, `consume_input`, `process_data`, `inverse_DCT[ci]`,
`color_convert`) stays a field of safe Rust `fn` type, assigned where C
assigns it. Its signature is the C signature with `j_decompress_ptr` replaced
by `&mut jpeg_decompress_struct` and buffers replaced as described below. Rust
`fn` pointers are `Copy` and need no `unsafe`, run-time selection stays where
upstream does it, and the SIMD packages later assign SIMD functions at the
same `jsimd_can_*` points ([ADR 0004](0004-simd-after-the-portable-port.md)).
The method slots that exist only for 12-bit and 16-bit samples, lossless mode
or SIMD (`process_data_12`, `decode_mcus`, `idct_simd` and the like) are absent
until a package that translates them adds them.

**Calls.** A call `(*cinfo->entropy->decode_mcu) (cinfo, MCU_data)` copies the
`fn` pointer out of the module and then calls it with the whole object:
`let decode_mcu = cinfo.entropy().decode_mcu; decode_mcu(cinfo, MCU_data)`.
The dispatcher holds no borrow of the module during the call, so a method may
point at another module's function, as `consume_input` points at
`consume_data`.

**A method's own state.** The C idiom `huff_entropy_ptr entropy =
(huff_entropy_ptr)cinfo->entropy;` becomes a scoped take: the function moves
the box out of `cinfo.entropy` (a pointer move) and matches the variant it
expects; a mismatch is `unreachable!`, where C's cast would be undefined. The
function puts the box back on every path out, errors included, before it
returns. While it holds the box, it uses `cinfo` for everything else, as C
does. Before it calls anything that can reach the same module through `cinfo`
(a method pointer, another module's function, a public API function), it puts
the box back and takes it again afterwards, so that state the callee writes
through `cinfo` is visible as it is in C. A module found absent where C would
find it is a translation error and panics; tests find it. A helper that C
gives only `cinfo` but that works on the module receives the module as a
second parameter. A function that holds no borrow of its module across a
call may instead reach the module through `cinfo` each time it uses it
(`cinfo.marker_mut().cur_marker`), which needs no put-back. The module's
documentation names the calls where it puts its module back. A call
that reaches only other modules and the application's hooks need not put the
box back when none of them reaches this module in upstream: the entropy
decoders keep theirs across the source's `fill_input_buffer`, the error
manager's `emit_message` and the marker reader's `read_restart_marker`,
because `jpegint.h`'s modules are not part of the application interface; a
hook that looked at `cinfo.entropy` meanwhile would find `None`. A helper that
several implementations share receives the public part instead of the module
(`jpeg_fill_bit_buffer` takes `&mut jpeg_entropy_decoder` for
`cinfo->entropy->insufficient_data`).

**Methods that only read the object.** A method that C gives `cinfo` but that
neither writes the object nor calls anything that does takes
`&jpeg_decompress_struct`, so that its other arguments may borrow from the
object as C's pointers point into it: an IDCT routine receives `compptr` from
`cinfo.comp_info` and, in multi-pass mode, its coefficient block from a
virtual array of `cinfo.mem`.

**Pointers into a module's data.** A pointer that a module keeps to data of
its own or of the object becomes an index or a copy. The tables that each
block of an MCU uses are numbers into the module's derived-table arrays;
`jdarith.c`'s statistics-bin pointer `st` is an area (a DC or AC table, or
the fixed bin) and an offset. A derived Huffman table copies the public table
it was derived from, which C links: the public tables change only between
scans, and every scan derives its tables again in `start_pass`.

**Public hooks.** The objects an application supplies (`jpeg_error_mgr`,
`jpeg_source_mgr`, `jpeg_progress_mgr`, the marker processors of
`jpeg_set_marker_processor`) keep their public fields as struct fields and
take their methods as a trait object owned by the struct
(`Box<dyn jpeg_source_methods + 'src>`), whose default methods are upstream's
functions. The trait object carries the application's context, which replaces
`client_data`. A hook method receives
`&mut jpeg_decompress_struct`. The trait object is taken out of its struct for
the duration of the call and put back afterwards, while the hook's public
fields stay in the object, where the method reaches them as C does
(`cinfo.err.msg_code`); a method reaches the other methods of its hook through
`self`. Upstream's default functions are also free functions, so that an
override can call them, as a C application calls the pointer that
`jpeg_std_error` installed.

### Error exits and warnings

C reports an error by storing `msg_code` and `msg_parm` in `cinfo->err` and
calling `error_exit`, which never returns: the default handler prints, destroys
the object and calls `exit`, and applications `longjmp` instead. Rust cannot
jump over Rust frames soundly, and a library may not exit the process.

**Errors return.** Every function that can reach `ERREXIT`, directly or
through a callee, returns `Result<T, ErrorExit>`, and callers propagate with
`?`. `ErrorExit` is a zero-sized marker: the error itself lives where C keeps
it, in `cinfo.err.msg_code` and `cinfo.err.msg_parm`. `ERREXIT(cinfo, code)`
and its variants become functions that store the code and parameters exactly
as the macros do, call the `error_exit` hook, and return `Err(ErrorExit)`; the
call site is `return Err(errexit(cinfo, JERR_...))`. Because errors return
along the ordinary path, every state change made before the error stays, as it
does after a `longjmp`, and taken modules are put back on the way out (see
above). Panics are reserved for translation errors and never carry a libjpeg
error, so the decoder behaves the same under `panic = "abort"`.

**The default error manager.** `jpeg_std_error` installs upstream's
`emit_message`, `output_message` (to standard error), `format_message` and
`reset_error_mgr`. Its `error_exit` calls `output_message` and returns; it
neither destroys the object nor exits. The caller decides what to do with an
object after `Err`, as a C application does after its `longjmp`. `jpeg_destroy`
is the owner dropping the object.

**Warnings and traces.** `WARNMS` and `TRACEMS` call `emit_message`, which a C
application may also leave with `longjmp` (TurboJPEG does so for its
stop-on-warning flag). `emit_message` therefore returns `Result<(),
ErrorExit>`, and every `WARNMS` and `TRACEMS` site propagates it. The default
keeps upstream's counting of `num_warnings` and its trace-level filtering and
returns `Ok`.

**Messages.** `msg_parm` is a union of `int i[8]` and `char s[80]`, and the
active member depends on the message text, not on a field. It becomes an
80-byte array with accessors that read and write the `int`s in native byte
order, so the union's aliasing is preserved. `format_message` implements the
`printf` conversions that `jerror.h` uses, and work package 1 proves every
message text against the oracle.

**Progress and other application callbacks** (`progress_monitor`, source
manager methods, marker processors) also return `Result`, so an application
can stop decoding from any point where a C application could `longjmp`.

### Suspending data sources

**Marker processors and the saved-marker list.** A marker processor is
`jpeg_marker_parser_method`, either a library routine (`fn`) or an
application's processor (`Box<dyn jpeg_marker_processor>`), which is taken out
of the marker reader for its call. The saved-marker list is a linked list of
boxes that drops iteratively; `master.marker_list_end` holds the position of
its tail, `None` where C's pointer is null. `jpeg_mem_src`'s test whether the
installed source is its own (`init_source == init_mem_source`) is the source
methods' `is_mem_source`.

**Source manager.** `jpeg_source_mgr<'src>` keeps `next_input_byte` as an
offset and `bytes_in_buffer` as a count into its current buffer, which is a
field `buffer: InputBuffer<'src>`, either `Borrowed(&'src [u8])` or
`Owned(Vec<u8>)`. Decoder code reads the bytes through the field, so it
borrows only the source, never the whole object. `fill_input_buffer` may
replace the buffer: `jpeg_mem_src` installs the static fake EOI as
`Borrowed`, and a streaming source refills an `Owned` buffer in place.
`resync_to_restart` defaults to `jpeg_resync_to_restart`.

**Suspension is kept.** `fill_input_buffer` returns `Result<bool, ErrorExit>`,
where `Ok(false)` means suspend. Every suspension path of the decoder is
translated as it is: return codes (`JPEG_SUSPENDED`, `FALSE` from a method),
the saved restart points in `jdmarker.c`, the MCU back-up in `jdhuff.c` and
the input controller's state machine. Suspension is not an error. The memory
source in scope never suspends, so work package 2 adds a probe with a
suspending source that feeds a few bytes per call, and every later package
that touches a suspension path tests it with that probe.

### The memory manager

`jmemmgr.c` decides which large or malformed inputs fail, so its arithmetic is
behavior; the C pointers it hands out are not.

**Exact accounting.** The Rust memory manager keeps `jmemmgr.c`'s bookkeeping
with the same arithmetic and order: small and large pool lists as size records
with their slop rules (`first_pool_slop`, `extra_pool_slop`, the 50-byte
minimum), header sizes, `ALIGN_SIZE` rounding, `total_space_allocated`,
`max_alloc_chunk` (`MAX_ALLOC_CHUNK`, 1 000 000 000) and
`max_memory_to_use`. Each request names the C size of what it allocates, not
the Rust size: every translated struct that upstream allocates has a constant
with its C `sizeof` on the target, verified by a probe, and `sizeof(small_pool_hdr)`
and `sizeof(large_pool_hdr)` are constants too. `ALIGN_SIZE` is
`MAX(sizeof(void *), sizeof(double))`, 8 on both hosts, because the reference
configuration has no SIMD; the SIMD packages revisit it (32 with
`WITH_SIMD`), since it also sets the row padding of `alloc_sarray`. Every
check fails at the same request with the same code: `JERR_OUT_OF_MEMORY` with
its `which` parameter (1 to 11), `JERR_BAD_POOL_ID`, `JERR_WIDTH_OVERFLOW`,
`JERR_BAD_VIRTUAL_ACCESS`, `JERR_VIRTUAL_BUG`, and `JERR_NO_BACKING_STORE` from
`jmemnobs.c` when `max_memory_to_use` is too small for the virtual arrays.
`jinit_memory_mgr` reads `JPEGMEM` from the environment as upstream does.

**Owned storage.** The storage behind each request is a Rust value that the
requesting code owns. The `alloc_small` and `alloc_large` slots account for a
request with its C size and make the caller's storage reservation where C's
`jpeg_get_small` or `jpeg_get_large` obtains memory; typed functions
(`alloc_small_box`, `alloc_small_vec`, `alloc_large_vec`) return the `Box` or
`Vec` built from the caller's value, and `alloc_sarray` and `alloc_barray`
return the buffers described below. Large requests and variable-length small
objects reserve with `Vec::try_reserve_exact` and report a refused reservation
as the `out_of_memory` code that a failing `jpeg_get_large` (4) or
`jpeg_get_small` (2) produces; a small fixed-size `Box` is allocated
infallibly. Rust initializes every allocation; C code that reads
`malloc`ed memory before writing it is undefined and would be recorded here.
No translated function does. Where C leaves memory uninitialized until it
first writes it (`last_rowsperchunk` until the first `alloc_sarray` or
`alloc_barray`, the tables of `jpeg_alloc_quant_table` and
`jpeg_alloc_huff_table`, the second half of `coef_bits` until `start_pass`),
the port starts it at zero.

**Pools.** `free_pool(JPOOL_IMAGE)` drops the image pool's records, so
`total_space_allocated` falls as it does in C, and resets every field of the
object that upstream fills from `JPOOL_IMAGE` to its empty value, which drops
the storage. Work package 1 derives that list of fields from upstream's
allocation calls (`tools/pools.json`), and `tools/pools.py` checks it against
upstream and against `release_pool_fields` in `jmemmgr`; later packages extend
it when they add a field. `JPOOL_PERMANENT` storage lives until the object is dropped.
`self_destruct` is the drop.

**Virtual arrays.** `request_virt_barray`, `realize_virt_arrays` and
`access_virt_barray` keep their checks, `maxaccess`, `pre_zero`,
`first_undef_row` and `dirty` logic; with no backing store, `rows_in_mem`
always equals `rows_in_array` or the request fails with
`JERR_NO_BACKING_STORE`, as in `jmemnobs.c`. Since `jpeg_open_backing_store`
never returns normally, no backing store is ever open, and the backing-store
I/O (`do_barray_io`, `b_s_open`, `b_s_info`) is not translated: every path
that needs it ends in its error exit. `access_virt_barray` returns the index
of the first requested row in the array's in-memory buffer, the offset C adds
to `mem_buffer`; the caller borrows the rows from the array's control block,
which owns the buffer. Virtual sample arrays are used only by two-pass
quantization and are not translated.

### Sample and coefficient buffers

**Storage and row lists.** A buffer from `alloc_sarray` is one
`Vec<JSAMPLE>` with the row stride that `alloc_sarray` computes (samples per
row rounded up to `2 * ALIGN_SIZE`), plus the list of row starts as offsets.
`alloc_sarray`'s chunking affects only the accounting, which stays exact. A
`JSAMPARRAY` becomes a pair of storage and row offsets: `JSAMPROWS<'a>` with
`&'a [JSAMPLE]` and `&'a [usize]` for input, `JSAMPROWS_MUT<'a>` with
`&'a mut [JSAMPLE]` for output. Pointer arithmetic on a row array
(`output_buf + *out_row_ctr`) slices the offset list. A module that keeps
several pointer lists into one buffer, such as the main controller's context
rows ("funny pointers" in `jdmainct.c`) or a wraparound list, keeps its lists
as `Vec<usize>` next to the storage and rotates them as C rotates pointers.
`JSAMPIMAGE` becomes one such pair per component.

**Caller rows.** `jpeg_read_scanlines` and `jpeg_read_raw_data` receive the
application's rows, which C allows to lie anywhere. `JSAMPROWS_MUT` therefore
has a second form, a slice of separate row slices (`&mut [&mut [JSAMPLE]]`),
and offers the row operations that modules use (`row`, `row_mut`, copying one
row onto another) for both forms. A copy within one array, such as
`jcopy_sample_rows(output_data, outrow, output_data, outrow + 1, ...)`, uses
`copy_within` or a split borrow.

**Blocks.** `JBLOCK` is `[JCOEF; 64]`. A `JBLOCKROW` or `JBLOCKARRAY` follows
the sample rules with `JBLOCK` elements. The `MCU_data` argument of
`decode_mcu` is `Option<&mut [&mut JBLOCK]>`: the caller collects mutable
borrows of the MCU's blocks from whichever buffers C points into, and `None`
is C's `NULL` that `jpeg_skip_scanlines` passes. A pointer list that a module
stores beyond one call (`coef->MCU_buffer`) is kept as block indexes into the
module's storage and turned into borrows per call.

**Disjoint borrows.** When C passes two pointers into one structure that the
callee mutates, the Rust code splits the structure into separate borrows, and
the module's documentation records the split. A pointer that may be `NULL`
becomes an `Option`.

### Out-of-scope branches inside in-scope files

In-scope files contain branches for features that are not translated: color
quantization (`jdmaster.c`, `jdapistd.c`, `jdpostct.c`), lossless mode
(`jdinput.c`, `jdmaster.c`), 12- and 16-bit precision (`jdmaster.c` selecting
`j12init_*`), RGB565 output (`jdcolor.c`, `jdmerge.c`) and the stdio source
(`jdatasrc.c`). Each such branch follows upstream's own behavior for a
compiled-out feature where upstream has one: with `QUANT_1PASS_SUPPORTED`,
`QUANT_2PASS_SUPPORTED` and `D_LOSSLESS_SUPPORTED` treated as undefined, the
branches raise `JERR_NOT_COMPILED` where upstream's `#else` does. Where
upstream has no switch, the branch raises the error upstream uses for the same
condition elsewhere: `JERR_BAD_PRECISION` for a data precision other than 8,
`JERR_CONVERSION_NOTIMPL` for RGB565 output. The stdio source functions are
absent ([tools/units.json](../../tools/units.json), `not_translated`). The
oracle supports all of these, so the differential tests list such inputs as
out of scope with this reason instead of comparing them. One default differs
as well: with `QUANT_2PASS_SUPPORTED` undefined, `default_decompress_parms`
sets `two_pass_quantize` to `false` where the oracle sets it to `TRUE`.

### Visibility

Every module is `pub`, so that tests and later packages can reach units
before the API exists, and so that unused translated items are not dead code.
They stay public beside the safe API of work package 8, which the crate root
re-exports ([ADR 0006](0006-safe-decoding-api.md)).

## Consequences

Safe Rust can express the 8-bit decoder under these rules, with explicit costs:
a `Result` on most signatures, a pointer move per method call that touches its
own module, row offset lists instead of row pointers, and memory accounting
that runs beside Rust's allocator. The structure stays upstream's: the same
modules, method slots, selection points and suspension paths, which keeps the
diff of a new upstream version applicable and leaves room for the SIMD
packages. The decoder never exits, prints only through the replaceable error
manager, and reports every libjpeg error through `cinfo.err` as C does.

## Decisions on the review questions (2026-11-06)

The maintainer accepted the proposed answers to the questions raised in review:

1. **Out-of-scope requests** fail with upstream's codes (`JERR_NOT_COMPILED`,
   `JERR_BAD_PRECISION`, `JERR_CONVERSION_NOTIMPL`) where the rules above put
   them, while the oracle succeeds. The safe API (work package 8) may check
   them up front.
2. **`JPEGMEM`** is kept for parity: `jinit_memory_mgr` reads it as upstream
   does. Ferrixel's adapter sets its memory limit explicitly.
3. **Small allocations.** An infallible `Box` is fine for fixed-size module
   structs; large buffers stay fallible.
4. **Default `error_exit`** prints to standard error and returns. The work
   package 8 API installs its own error manager, which records instead of
   printing ([ADR 0006](0006-safe-decoding-api.md)).
5. **`long`** maps to `core::ffi::c_long`. Windows is not a target and stays
   unverified.
6. **Common object.** `j_common_ptr` collapses into the decompression struct
   while the encoder is out of scope.
7. **Names.** C names stay verbatim, with item-scoped naming `expect`s.
8. **Header mapping.** The mapping in [tools/units.json](../../tools/units.json)
   is confirmed: `jdct.h`, `jmemsys.h`, `jconfig.h.in`, `jversion.h.in` and
   `jstdhuff.c` have modules of their own; `jerror.h`, `jdhuff.h`,
   `jdcoefct.h`, `jdmainct.h`, `jdmaster.h`, `jdsample.h` and `jdmerge.h`
   share the module of the C file with the same name; `jdmrgext.c` and
   `jdcolext.c` are modules of their own.

## Decisions for work package 1 (2026-11-07)

The maintainer decided two questions that work package 1 raised:

1. **Module interfaces and slots** arrive with the package that translates the
   module, together with the object fields that depend on it ("Module
   interfaces arrive with their modules" above).
2. **`jpeg_abort`** moves to work package 2, which creates `master` and the
   saved-marker list.

## Decisions for work package 2 (2026-11-07)

1. **Master struct.** Work package 2 translates `jdmaster.h`'s struct, and the
   functions that call later packages' modules arrive with those packages.
2. **Interfaces that `jdinput.c` calls.** Work package 2 adds the methods of
   `jpeg_entropy_decoder` and `jpeg_d_coef_controller` that
   `start_input_pass` uses, with empty slots, so that `jdinput.c` is
   translated in full.

## Refinements from work package 3 (2026-11-07)

Work package 3 refined three rules above, for the maintainer's review in its
pull request:

1. **Scoped take across hooks.** The entropy decoders keep their module
   across calls that reach only the source manager, the error manager and the
   marker reader, which never reach the entropy decoder in upstream ("A
   method's own state").
2. **Pointers into a module's data** become indexes and copies, as the new
   paragraph of that name says.
3. **Overflow that upstream exempts from the sanitizer** wraps ("Integer
   types").

## Refinements from work package 4 (2026-11-07)

Work package 4 refined these rules, for the maintainer's review in its pull
request:

1. **Methods that only read the object** take it shared (the paragraph of
   that name): `inverse_DCT_method_ptr` is
   `fn(&jpeg_decompress_struct, &jpeg_component_info, &JBLOCK,
   JSAMPROWS_MUT, JDIMENSION)`.
2. **Scoped take of the virtual arrays.** In multi-pass mode `consume_data`
   lends the MCU's blocks to `decode_mcu` straight from the virtual arrays.
   It takes the memory manager's list of arrays out of `cinfo.mem` while it
   decodes an iMCU row and puts it back on every path out, as a module takes
   its box; the entropy decoder and the hooks it reaches never use the
   memory manager in upstream ("A method's own state").
3. **Unions of arrays.** `jddctmgr.c`'s `multiplier_table`, a union of an
   `int`, an `int` and a `float` array, keeps one array of `int`s whose bits
   the float accessors reinterpret, as `msg_parm` keeps its union
   ("Messages").
4. **Float-to-integer conversions** saturate where C is undefined ("Integer
   types").

## Refinements from work package 5 (2026-11-07)

Work package 5 refined these rules, for the maintainer's review in its pull
request:

1. **Files included once per pixel layout.** `jdcolor.c` includes
   `jdcolext.c` and `jdmerge.c` includes `jdmrgext.c` once per RGB layout,
   with `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, `RGB_ALPHA` and `RGB_PIXELSIZE`
   defined and the routines renamed. Each routine is one function generic
   over these values as `const` parameters (`RGB_ALPHA` is -1 where C leaves
   it undefined), and each C instance is a constant of the method type that
   names the function with its values (`ycc_extbgrx_convert`), so the
   instances keep their names and the selection code its shape ("Macros and
   translation-time headers").
2. **Row lists read before their first row.** Where C passes a `JSAMPARRAY`
   that points into the middle of a longer pointer list, as `jdmainct.c`'s
   "funny" pointer lists do, the callee receives `JSAMPROWS_AT`: the storage,
   the whole list and the position of row 0, so that the fancy upsamplers can
   read the context row above it ("Storage and row lists").
3. **A pointer to the module's storage or to its input.** `jdsample.c`'s
   `color_buf[ci]` points either at the upsampler's own rows or, after
   `fullsize_upsample`, at the input rows. It is an enum of the two (and C's
   null pointer); the input case holds the position in the input's row list
   and is resolved against the `input_buf` of the call that uses it, which
   is the one the pointer was taken from ("Pointers into a module's data").
4. **A field lent for one call.** The merged upsampler's spare row can be
   the second output row of `upmethod`, which also reads the module's
   tables; the control routine takes the row out of the module for the call
   and puts it back, as a module takes its box ("A method's own state").

## Refinements from work package 6 (2026-11-07)

Work package 6 refined these rules, for the maintainer's review in its pull
request:

1. **Output into a module's own buffer.** To skip lines of the merged 2:1
   upsampler, `jdapistd.c`'s `read_and_discard_scanlines` reads them into
   the upsampler's own spare row, which the upsampler then also writes as
   its second output row. The port reads them into a row of the same
   length. The upsampler writes the second row after the first at every
   position, so the spare row ends as C leaves it, and the read row is
   discarded in both ("Disjoint borrows").
2. **Calls into the absent color quantizer.** `master_selection` fails with
   `JERR_NOT_COMPILED` for color quantization, so only an application that
   sets `quantize_colors` or an `enable_*_quant` flag after
   `jpeg_start_decompress` reaches the code that calls `cinfo->cquantize`
   (`prepare_for_output_pass`, `finish_output_pass`). There C would call
   through a quantizer that was never created. The port raises
   `JERR_NOT_COMPILED` at those calls, and the absent `colormap` counts as
   C's null pointer, so `jpeg_new_colormap` ends with `JERR_MODE_CHANGE`
   ("Out-of-scope branches inside in-scope files").

## Review triggers

Amend this record when a package meets a construct it does not cover, when a
rule forces a change of observable behavior, before the SIMD packages start, or when the encoder or another precision enters the scope.
