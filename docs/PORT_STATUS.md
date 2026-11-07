# Port status

| WP | State | Evidence |
| --- | --- | --- |
| 0 Tooling and representation | done | Oracle, probes, bit-test inventory, unit map, check script and CI below; [ADR 0005](adr/0005-rust-representation.md) accepted |
| 1 Common infrastructure | done | 11 modules translated; message, memory and utility probes below |
| 2 Input and markers | done | `jpeg_read_header` and the input side translated; header probe below |
| 3 Entropy decoding | in review | Huffman, progressive Huffman and arithmetic decoders translated; entropy probe below |
| 4–11 | not started | — |

Work packages 1 to 3 are translated: 21 modules, and the struct of
`jdmaster.h`. The other 16 modules under `src/` hold only their provenance
headers ([tools/units.json](../tools/units.json)).

## Work package 0

**Oracle.** `tools/build_oracle.py` builds the unmodified
`upstream/libjpeg-turbo` (3.2.1, `7b49937`) in the reference configuration of
[ADR 0002](adr/0002-differential-oracle.md) into `build/oracle/<variant>/` and
records the compiler, the CMake options, the flags and the `jconfig.h` and
`jconfigint.h` defines in `oracle.json`. Upstream's float bit tests expect
`FLOATTEST8=no-fp-contract` with `-ffp-contract=off`; the script sets it,
because upstream's default for Clang 14 and later on ARM64 is `fp-contract`.

| Variant | Purpose | Upstream CTest suite | In-scope bit tests |
| --- | --- | --- | --- |
| `release` | reference oracle | 332 of 332 pass (macOS ARM64, Linux x86-64) | 79 of 79 tests pass on both hosts |
| `asan-ubsan` | same, with ASan and UBSan | 332 of 332 pass on both hosts | 79 of 79 tests pass on both hosts |
| `simd` | `WITH_SIMD=1`, diagnostic for [ADR 0004](adr/0004-simd-after-the-portable-port.md) | 332 of 332 pass on macOS ARM64 (local) | not run in CI; on x86-64 it needs NASM |

The CI hosts are macOS ARM64 with Apple Clang and Linux x86-64 with Clang 21.

**Probes.** C probes live in `tools/oracle/probes/<wp>/` and link the oracle's
static `libjpeg.a`. `tools/oracle/probe_support` records error codes, messages
and warning codes before it leaves a failing call with `longjmp`, so probes can
print the state a failing call leaves behind. `tests/support/oracle.rs` runs a
probe in a separate process; without `LIBJPEG_ORACLE_DIR` it skips with a
message, and with the `oracle` feature it fails instead.
`tests/wp0_read_header.rs` checks the `wp0/read_header` probe
(`jpeg_read_header` on `testorig.jpg`, `testimgint.jpg`, `testimgari.jpg`,
`monkey12.jpg`, standard input, a truncated and an empty input) against
expectations derived by hand from the file bytes and the upstream headers.

**Bit tests.** [tools/bittests.json](../tools/bittests.json) lists all 97
`add_bittest` cases that the static build declares; `tools/bittests.py --check`
verifies the list, arguments and inputs against the configured CTest suite.

| Class | Cases |
| --- | --- |
| In scope: 8-bit `djpeg` | 30 |
| Out: 12-bit `djpeg` | 27 |
| Out: RGB565 output | 10 |
| Out: lossless (8-bit) | 1 |
| Out: 16-bit lossless | 1 |
| Out: quantized `-colors` | 1 |
| Out: encoder (`cjpeg`) | 21 |
| Out: transcoding (`jpegtran`) | 6 |

The 30 in-scope cases cover the islow, ifast and float IDCTs, every scaled
size from 1/8 to 2/1, fancy and merged upsampling for h2v1, h1v2, h2v2 and 3x2
sampling, grayscale to grayscale and to RGB, baseline, progressive and
arithmetic decoding, ICC profile extraction, `jpeg_skip_scanlines` and
`jpeg_crop_scanline`. `tools/bittests.py --run` runs them with their MD5
checks, the extra ICC check of `rgb-islow` and the 11 `cjpeg` runs (with their
own MD5 checks) that produce their inputs: 79 CTest tests. Upstream's
`example-8bit-*-decompress` test is not an `add_bittest` case and is not in
the inventory.

**Unit map.** [tools/units.json](../tools/units.json) maps every file under
`upstream/libjpeg-turbo/src` to one of 37 modules and work packages 1 to 6, or
to the reason it is not translated (88 entries, including `src/wrapper/`,
`src/md5/`, `src/spng/` and `simd/`). `tools/units.py` verifies that the map
covers upstream, that each module starts with its provenance header and that
`src/lib.rs` declares exactly the mapped modules.

**Checks.** `tools/check.py` runs the pin, skeleton and link checks,
`cargo fmt --check`, Clippy with `-D warnings` and the tests in the dev,
release and `checked` profiles; `--oracle` adds the oracle build, the bit
tests and the differential tests with the `oracle` feature. The CI oracle job
does this for the `release` and `asan-ubsan` variants on both hosts.

## Work package 1

**Translated.** Every item of these units, except those listed below:

| Module | Upstream | Content |
| --- | --- | --- |
| `jconfig` | `jconfig.h.in` | `JPEG_LIB_VERSION` 62 and the other defines of the oracle's `jconfig.h` |
| `jversion` | `jversion.h.in` | `JVERSION` for version 62, `JCOPYRIGHT` with upstream's `COPYRIGHT_YEAR` |
| `jmorecfg` | `jmorecfg.h` | 8-bit sample and coefficient types, limits, capability options, the `rgb_*` tables, `MULTIPLIER`, `FAST_FLOAT`; `CLayout` for C struct sizes |
| `jpegint` | `jpegint.h` | `J_BUF_MODE`, `DSTATE_*`, `JLONG`, `JUINTPTR`, `IsExtRGB`, `LEFT_SHIFT`, `MAX`, `MIN`, `RIGHT_SHIFT` |
| `jpeglib` | `jpeglib.h` | constants, sample and block buffers, `JQUANT_TBL`, `JHUFF_TBL`, `jpeg_component_info`, the color, DCT and dither enums, `jpeg_decompress_struct`, the error, progress and source managers, the memory manager interface, return and marker codes |
| `jerror` | `jerror.h`, `jerror.c` | the 130 message codes and the message table, the five default methods, `jpeg_std_error`, `ERREXIT` to `TRACEMSS`, and `SNPRINTF` from `jinclude.h` |
| `jcomapi` | `jcomapi.c` | `jpeg_destroy`, `jpeg_alloc_quant_table`, `jpeg_alloc_huff_table` |
| `jutils` | `jutils.c` | `jpeg_natural_order`, `jdiv_round_up`, `jround_up`, `jcopy_sample_rows`, `jcopy_block_row`, `jzero_far` |
| `jmemsys` | `jmemsys.h` | `MAX_ALLOC_CHUNK` |
| `jmemnobs` | `jmemnobs.c` | all eight functions |
| `jmemmgr` | `jmemmgr.c` | pools, `alloc_small`, `alloc_large`, `alloc_sarray`, `alloc_barray`, the virtual block arrays, `free_pool`, `self_destruct`, `jinit_memory_mgr` with `JPEGMEM` (and `GETENV_S` from `jinclude.h`) |

**Not translated.** `jpeg_abort` moves to work package 2 (ADR 0005, decision
of 2026-11-07). The jpegint.h module interfaces, the module slots of
`jpeg_decompress_struct` and the fields that depend on a module (`comp_info`,
`cur_comp_info`, `coef_bits`, `colormap`, `sample_range_limit`,
`marker_list`, `jpeg_component_info::dct_table`, `jpeg_marker_struct`,
`jpeg_marker_parser_method`) arrive with the package that translates the
module. Out of scope (ADR 0001): the compression object and its types, the
`CSTATE_*` values, 12- and 16-bit samples, `JDIFF`, the virtual sample arrays
of two-pass quantization, `WITH_PROFILE`, `MEM_STATS`,
`USE_WINDOWS_MESSAGEBOX`. Unreachable with `jmemnobs.c`, which never opens a
backing store: `backing_store_info`, `do_barray_io` and the reload and close
paths of `jmemmgr.c` (ADR 0005, "Virtual arrays").

**Representation.** The memory manager's slots account for each request with
its C size and make the caller's reservation where C calls `malloc`; the
typed `alloc_small_box`, `alloc_small_vec` and `alloc_large_vec` return the
storage. Sample and block arrays are one buffer with row offsets
(`RowArray`, `RowList`, `RowListMut`), with a second form for application
rows and `RowListMut::copy_row` for copies within one array.
`access_virt_barray` returns the index of the first requested row. The error
manager's hook calls (`jerror::call_*`) take its trait object out of
`cinfo.err` and put it back; work package 1 takes no module out of the object.
These refinements are recorded in ADR 0005.

**Pools.** [tools/pools.json](../tools/pools.json) lists all 65 allocation
calls of the upstream files that `tools/units.json` maps, with their pool, what
they fill and the object field whose reset releases the storage, and the
fields each pool fills with the package that adds them. `tools/pools.py`
(part of `tools/check.py`) checks the list against upstream and checks that
`release_pool_fields` resets every listed field that `jpeg_decompress_struct`
has: in work package 1, `quant_tbl_ptrs`, `dc_huff_tbl_ptrs`,
`ac_huff_tbl_ptrs` and `src` for the permanent pool.

**Verified against the oracle.** Three probes in `tools/oracle/probes/wp1/`,
each compared field for field with the port's results for the same input:

| Test | Probe | Compared |
| --- | --- | --- |
| `tests/wp1_messages.rs` | `messages` | name and value of all 130 codes, every table string, `JMSG_LASTMSGCODE`, `last_jpeg_message` and the final null entry; `format_message` for codes -2 to 131 with three sets of integer parameters and with a string parameter; an addon table of 12 strings covering flags, widths, precisions, `*`, `hh` and `h`, `%c`, `%%`, `%s`, a null entry, overlapping and absent addon ranges, bogus codes and truncation at 199 bytes; `ERREXIT` to `ERREXIT6`, `ERREXITS` (cut at 79 bytes, NUL-padded), `WARNMS` to `WARNMS2`, `TRACEMS` to `TRACEMS5`, `TRACEMS8`, `TRACEMSS`; `emit_message`'s first-warning and trace-level policy and `num_warnings`; `reset_error_mgr`; `jpeg_std_error`. `msg_parm` is compared byte for byte after every step. |
| `tests/wp1_memory.rs` | `memory` | after every operation: `total_space_allocated`, every pool record of both pools, `last_rowsperchunk`, the limits and the virtual array control blocks; the slop rules; `out_of_memory` cases 1, 3, 6, 7, 8, 9 and 11, `JERR_BAD_POOL_ID` for every allocator, `JERR_WIDTH_OVERFLOW` (sample, block and virtual arrays), `JERR_NO_BACKING_STORE` with the `rows_in_mem` it leaves, `JERR_BAD_VIRTUAL_ACCESS` in each of its four checks; pre-zeroing and buffer contents; row strides and row chunking above `MAX_ALLOC_CHUNK`; `free_pool`, `jpeg_destroy`, the table allocators; `JPEGMEM` for 17 values (unset, empty, blanks, signs, `m` and `M`, junk, the 29- and 30-character limit) |
| `tests/wp1_utils.rs` | `sizes` | `sizeof` of `JQUANT_TBL`, `JHUFF_TBL`, `jpeg_component_info`, `jpeg_error_mgr`, `JBLOCK`, `JSAMPROW`, `JBLOCKROW`; `ALIGN_SIZE`; `MAX_ALLOC_CHUNK`; `jpeg_natural_order`; `jdiv_round_up` and `jround_up` for 13 operand pairs; `jcopy_sample_rows`, `jcopy_block_row`, `jzero_far` |

The sizes of the private `my_memory_mgr` (168 bytes), `jvirt_barray_control`
(152) and pool headers (24) are verified through the accounting:
`total_space_allocated` starts at the C library's `sizeof(my_memory_mgr)`, and
each control block takes 152 bytes of a small pool. All of this passes
against the `release` and `asan-ubsan` oracles in the dev, release and
`checked` profiles on both CI hosts (macOS ARM64, Linux x86-64), and locally
on macOS ARM64 with Apple Clang 21.

**Out of scope, listed instead of compared.** A data precision of 12 or 16:
the oracle allocates the sample array, the port fails with
`JERR_BAD_PRECISION` (ADR 0005); `tests/wp1_memory.rs` asserts both and the
`last_rowsperchunk` before the branch.

**Checked on the port only**, because the oracle cannot observe it: a refused
reservation reports case 2 after halving the small pool's slop to `MIN_SLOP`,
or case 4 for a large pool, without accounting for the request; the typed
allocators account exactly as the slots do; `jpeg_destroy` drops the tables
of the permanent pool; copies within one array, in both row forms.

**Not compared.** The default `error_exit`: upstream's exits the process,
the port's returns (ADR 0005). `printf` conversions that are undefined with
`format_message`'s arguments (a conversion other than an exact first `%s` in a
string message, more than eight integers, other length modifiers); the port
copies them to the output unchanged. A `JPEGMEM` value that overflows `long`
when multiplied by 1000. Unreachable on both hosts: `JERR_VIRTUAL_BUG` (the
in-memory buffer always holds the whole array), `JERR_BAD_ALIGN_TYPE`,
`JERR_BAD_ALLOC_CHUNK` and `out_of_memory` cases 0 and 5; cases 2 and 4 need a
failing `malloc`.

**Uninitialized C memory.** No translated function reads memory before
writing it. `jinit_memory_mgr` leaves `last_rowsperchunk` uninitialized until
the first `alloc_sarray` or `alloc_barray`, and the tables of
`jpeg_alloc_quant_table` and `jpeg_alloc_huff_table` come back uninitialized;
the port starts them at zero.

## Work package 2

**Translated.**

| Module | Upstream | Content |
| --- | --- | --- |
| `jdapimin` | `jdapimin.c` | `jpeg_CreateDecompress` and `jpeg_create_decompress`, `jpeg_destroy_decompress`, `jpeg_abort_decompress`, `default_decompress_parms`, `jpeg_read_header`, `jpeg_consume_input`, `jpeg_input_complete`, `jpeg_has_multiple_scans` |
| `jdatasrc` | `jdatasrc.c` | the memory source: `init_mem_source`, `fill_mem_input_buffer`, `skip_input_data`, `term_source`, `jpeg_mem_src` |
| `jdinput` | `jdinput.c` | all functions, including `start_input_pass` |
| `jdmarker` | `jdmarker.c` | all functions and the input macros |
| `jdicc` | `jdicc.c` | `jpeg_read_icc_profile` |
| `jcomapi` | `jcomapi.c` | `jpeg_abort`, moved from work package 1 |
| `jdmaster` | `jdmaster.h` | `my_decomp_master` with the data fields of `jpeg_decomp_master` |
| `jpegint`, `jpeglib` | `jpegint.h`, `jpeglib.h` | `jpeg_input_controller`, `jpeg_marker_reader`, the methods of `jpeg_entropy_decoder` and `jpeg_d_coef_controller` that `start_input_pass` calls, `jpeg_marker_struct`, `jpeg_marker_parser_method`, the object's `comp_info`, `cur_comp_info`, `coef_bits`, `marker_list` and its `master`, `coef`, `inputctl`, `marker` and `entropy` slots, `SIZEOF_jpeg_decompress_struct` |

Work package 1's `jmorecfg` lacked `MAX_COMPONENTS`; it is added.

**Not translated.** `jpeg_finish_decompress`, which calls
`master->finish_output_pass`, arrives with `jdmaster.c` in work package 6; the
methods of `jpeg_decomp_master` with it. The `entropy` and `coef` slots stay
empty until work packages 3 and 4, so `start_input_pass` is translated but not
reached: `jpeg_read_header` stops at the first SOS (ADR 0005, decisions of
2026-11-07). The stdio source is out of scope (`tools/units.json`).

**Representation.** Modules are reached through `cinfo` each time they are
used; no function holds one across a call, so no module is taken out and put
back. The source manager's, the error manager's and an application marker
processor's trait objects are taken out for their calls
(`jpeglib::call_*_source`, `jerror::call_*`, `jdmarker`'s `call_processor`).
`cur_comp_info` holds indexes into `comp_info`. The saved-marker list is a
linked list of boxes whose tail position is `master.marker_list_end`. The
input macros work on a copy of the source position (`InputVars`) and keep the
partial state of a suspension: a two-byte value whose second byte is missing
holds the first byte shifted left by 8, as C's `INPUT_2BYTES` leaves it.

**Verified against the oracle.** The probe `tools/oracle/probes/wp2/header`
runs an API sequence and prints every message that `emit_message` receives
(level, code and text), each call's result, a line per suspension, and then
the object: header fields, decompression defaults, component layout,
quantization, Huffman and arithmetic tables, saved markers, the master, input
and marker modules, the source position and the memory pools.
`tests/wp2_header.rs` runs the same sequences on the port
(`tests/support/wp2.rs`) and compares the outputs exactly:

| Test | Inputs |
| --- | --- |
| `test_images_match_the_oracle` | `testorig.jpg`, `testimgint.jpg`, `testimgari.jpg`, `monkey12.jpg` (12-bit), with tracing, saved markers, ICC extraction, application processors, `jpeg_consume_input`, `jpeg_input_complete`, `jpeg_has_multiple_scans`, a second `jpeg_read_header` and `jpeg_abort_decompress` |
| `encoder_variants_match_the_oracle` | 20 `cjpeg` variants of `testorig.ppm`: progressive, arithmetic, both, restart intervals in rows and blocks, sampling 1x1, 2x1, 1x2, 4x1, 3x2, grayscale, RGB, optimized tables, two quality levels, swapped quantization slots, baseline at quality 5, 12-bit sequential and progressive, embedded `test1.icc` (nine APP2 markers) and `test3.icc`; read whole and suspending |
| `restart_markers_match_the_oracle` | `read_restart_marker` and `jpeg_resync_to_restart` on the restart variants, scanning the entropy-coded data, also suspending |
| `marker_streams_match_the_oracle` | 95 streams built from `testorig.jpg`'s segments, one branch each: JFIF versions and thumbnails, JFXX extensions, Adobe transforms for three and four components, component IDs without markers, two, five and eleven components, every SOF error and unsupported SOF type, SOS errors, DQT, DHT, DAC and DRI variants and errors, COM, every APPn, DNL, reserved markers, garbage, fill bytes, stuffed zeros, duplicate SOI, missing SOI, tables-only streams (with `require_image` off and a second datastream), bogus marker lengths; each with tracing, saved markers, suspension and application processors |
| `icc_profiles_match_the_oracle` | 11 sets of APP2 markers: chunked, missing, duplicate, out-of-range and inconsistent sequence numbers, empty, foreign and short markers, saved whole, cut to 20 bytes, and not saved |
| `truncations_match_the_oracle` | every truncation of `testorig.jpg`'s header (627 lengths), and of the arithmetic progressive and both ICC variants at their first 48 and last 700 bytes and 24 points in between |
| `mutations_match_the_oracle` | 2 422 single-byte mutations of `testorig.jpg`'s 623-byte header: each byte set to 0x00, 0xFF, xor 0x55 and plus 1 |
| `suspension_matches_the_oracle` | suspending input that releases 1, 2, 3, 13, 64 or 100 000 bytes per round, with and without saved markers and ICC extraction, and input that ends early |
| `api_sequences_match_the_oracle` | `jpeg_CreateDecompress` with a wrong version or struct size, `jpeg_read_header` in a wrong state, saved-marker and processor requests for non-APPn codes, a length limit above `MAX_ALLOC_CHUNK`, empty input |

The oracle's sizes of the object (632 bytes), `my_marker_reader`,
`my_input_controller` and `my_decomp_master` are verified: the probe prints
`sizeof(struct jpeg_decompress_struct)`, and the pool records it prints after
`jpeg_create_decompress` contain the C size of each module it allocates.
Locally on macOS ARM64 (Apple Clang 21), all of this passes against the
`release` and `asan-ubsan` oracles in the dev, release and `checked`
profiles.

The probe links `tools/oracle/probes/wp2/support/wp2_support.c`, which
defines the eight functions of `jmemnobs.c` with `calloc` instead of `malloc`,
so that memory a failing or suspended call leaves unwritten is zero in C as in
the port and can be printed; the library is unmodified. Upstream's fuzz seed
corpus is not part of the pinned tree; the generated variants, streams,
truncations and mutations stand in for it, and the decompression fuzzers run
in work package 7.

**Differences by design.** With `QUANT_2PASS_SUPPORTED` treated as undefined,
`default_decompress_parms` sets `two_pass_quantize` to `FALSE` where the
oracle sets `TRUE`; the test maps that one field. Lossless files are out of
scope: with `D_LOSSLESS_SUPPORTED` treated as undefined, a 16-bit lossless
header fails with `JERR_BAD_PRECISION` where the oracle reads it, which
`lossless_files_are_out_of_scope` asserts instead of comparing.

## Work package 3

**Translated.**

| Module | Upstream | Content |
| --- | --- | --- |
| `jdhuff` | `jdhuff.h`, `jdhuff.c` | the derived tables and `jpeg_make_d_derived_tbl`, the bit-reading macros, `jpeg_fill_bit_buffer`, `jpeg_huff_decode`, `HUFF_EXTEND`, the sequential decoder with its slow and fast MCU paths, `jinit_huff_decoder` |
| `jstdhuff` | `jstdhuff.c` | `add_huff_table` and `std_huff_tables`, the decompressor's branch |
| `jdphuff` | `jdphuff.c` | all functions: `start_pass_phuff_decoder` and the four progressive MCU decoders |
| `jaricom` | `jaricom.c` | `jpeg_aritab` |
| `jdarith` | `jdarith.c` | all functions: `arith_decode`, the sequential and the four progressive MCU decoders, `start_pass`, `jinit_arith_decoder` |
| `jpegint`, `jpeglib` | `jpegint.h`, `jpeglib.h` | the rest of `jpeg_entropy_decoder` (`decode_mcu`, `insufficient_data`) and the three implementations of the `entropy` slot |

`jdinput`'s `per_scan_setup` and `latch_quant_tables` are now public, so that
the tests can start a scan with a coefficient controller of their own.

**Not translated.** `decode_mcus` and `process_restart` of
`jpeg_entropy_decoder` serve lossless mode, which is out of scope (ADR 0001),
and so are `jdlhuff.c` and `jdlossls.c`. Nothing selects a decoder yet:
`master_selection` arrives with `jdmaster.c` in work package 6, and the
coefficient controller that calls `decode_mcu` with work package 4.

**Representation.** Each decoder takes its module out of `cinfo.entropy`
for the duration of every method and puts it back on every path out. It keeps
the module across the source's `fill_input_buffer`, the error manager's
`emit_message` and the marker reader's `read_restart_marker`, so no call puts
it back early; `jpeg_fill_bit_buffer` receives the decoder's public part
(ADR 0005, refinements from work package 3). The bit-reading macros are
functions over the locals they name, and their `goto` targets are return
values; `decode_mcu_AC_refine`'s `undoit` is a labeled block. A derived
Huffman table copies its public table, the tables of an MCU's blocks are
numbers into the derived-table arrays, and the arithmetic decoder's
statistics-bin pointer is an area and an offset. The DC prediction sums of the
sequential Huffman decoder wrap, as the compiled C does where upstream exempts
them from the sanitizer. `cinfo->coef_bits` is filled with -1 for the current
scan status as in C; its second half, which C leaves uninitialized until
`start_pass` writes it, starts at zero.

**Verified against the oracle.** The probe `tools/oracle/probes/wp3/entropy`
reads a header, initializes the decoder that `master_selection` would
choose, installs a coefficient controller that mirrors `jdcoefct.c`'s
multi-scan `start_input_pass` and `consume_data` and keeps every block, and
calls `consume_input` until EOI, as buffered-image decoding does. It prints
every message, the decoder state after each scan (`insufficient_data`,
`unread_marker`, the restart and source positions, `last_good_iMCU_row`,
the progression status in `coef_bits`), every result of `consume_input`, and
an FNV-1a hash of every block row of every component. `tests/wp3_entropy.rs`
runs the same sequence on the port (`tests/support/wp3.rs`, which mirrors
`jdinput.c`'s `start_input_pass` and `consume_markers` around its own
controller, because the port's controller slot stays empty until work
package 4) and compares the outputs exactly:

| Test | Inputs |
| --- | --- |
| `test_images_match_the_oracle` | `testorig.jpg`, `testimgari.jpg`, `testimgint.jpg`, `monkey12.jpg` (12-bit) |
| `encoder_variants_match_the_oracle` | 31 `cjpeg` variants of `testorig.ppm`: sequential, optimized, progressive, arithmetic and arithmetic progressive coding, restart intervals in rows and blocks for each kind, sampling 1x1, 2x1, 1x2, 4x1, 3x2, grayscale, RGB, quality 100 and 5, 12-bit data for each kind, and `test.scan`'s scan script with Huffman and arithmetic coding; with tracing |
| `skipped_coefficients_match_the_oracle` | the variants with `DCT_scaled_size` 1 and 2 and with components not needed; the sequential ones also with `MCU_data = NULL` for the first 1, 4 and all iMCU rows |
| `suspension_matches_the_oracle` | suspending input that releases 1, 7, 64 or 1000 bytes per step, for `testorig.jpg` and the progressive, arithmetic (which cannot suspend: `JERR_CANT_SUSPEND`), restart, quality-100 progressive and scan-script variants |
| `truncations_match_the_oracle` | every variant cut at 12 points of its entropy-coded data |
| `mutations_match_the_oracle` | 992 mutations of the variants' entropy-coded data: 8 positions each, set to xor 0x55, 0xFF, 0x00 and 0xD9, every second one followed by a zero byte |
| `synthetic_streams_match_the_oracle` | 172 streams built from the variants: no DHT (the standard tables of `jstdhuff.c`), an overfull code, DC symbol 16, a 16-bit AC code, scans naming undefined or other tables, changed arithmetic conditioning (DAC), an AC scan before its DC scan, a repeated scan, a refinement before its first pass, a DC-only stream, a second scan in a single-scan stream, `Ss`, `Se`, `Ah` and `Al` out of range and inconsistent, wrong, missing and doubled restart markers and fill bytes before them; with tracing |
| `dc_overflow_matches_the_oracle` | 73 683 blocks whose DC differences are all +32767: the progressive decoder fails with `JERR_BAD_DCT_COEF`, the sequential one wraps |
| `standard_tables_reproduce_the_coefficients` | the port alone: without DHT, `testorig.jpg`'s coefficients are those of the original stream |

Together the cases reach every warning of the decoders (`JWRN_HIT_MARKER`,
`JWRN_HUFF_BAD_CODE`, `JWRN_ARITH_BAD_CODE`, `JWRN_NOT_SEQUENTIAL`,
`JWRN_BOGUS_PROGRESSION`, with `JWRN_MUST_RESYNC` and `JWRN_JPEG_EOF` from the
input side) and the errors `JERR_BAD_HUFF_TABLE`, `JERR_NO_HUFF_TABLE`,
`JERR_BAD_PROGRESSION`, `JERR_BAD_DCT_COEF` and `JERR_CANT_SUSPEND`.
`JERR_NO_ARITH_TABLE` cannot be reached from a stream: a scan names tables 0
to 15 and there are 16. Locally on macOS ARM64 (Apple Clang 21), all of this
passes against the `release` and `asan-ubsan` oracles in the dev, release and
`checked` profiles.

The probe links WP2's support library, so C's memory is zeroed as in the
port (see work package 2), and prints the memory pools after decoding with
its `Wp2PrintMemory`, which the header probe now shares. Their used sizes
contain the C size of every allocation of the decoders, so the sizes of
`huff_entropy_decoder`, `phuff_entropy_decoder`, `arith_entropy_decoder` and
`d_derived_tbl`, the statistics areas and `coef_bits` are verified.

**Differences by design.** None in the decoders' output.
