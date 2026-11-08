# Port status

| WP | State | Evidence |
| --- | --- | --- |
| 0 Tooling and representation | done | Oracle, probes, bit-test inventory, unit map, check script and CI below; [ADR 0005](adr/0005-rust-representation.md) accepted |
| 1 Common infrastructure | done | 11 modules translated; message, memory and utility probes below |
| 2 Input and markers | done | `jpeg_read_header` and the input side translated; header probe below |
| 3 Entropy decoding | done | Huffman, progressive Huffman and arithmetic decoders translated; entropy probe below |
| 4 Coefficients and IDCT | done | Coefficient controller, IDCT manager and all IDCT routines translated; IDCT and pipeline probes below |
| 5 Upsampling, color, buffers | done | Upsamplers, color conversion, main and postprocessing controllers translated; decode probe below |
| 6 Control and API | done | Master control and the decompression API translated; API probe below |
| 7 Upstream bit tests | in review | The 30 in-scope `djpeg` bit tests produce upstream's MD5 sums on the port; driver below |
| 8–11 | not started | — |

Work packages 1 to 6 are translated: all 37 modules under `src/`
([tools/units.json](../tools/units.json)).

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

## Work package 4

**Translated.**

| Module | Upstream | Content |
| --- | --- | --- |
| `jdcoefct` | `jdcoefct.h`, `jdcoefct.c` | all functions: `start_iMCU_row`, the input and output pass setup, `decompress_onepass`, `dummy_consume_data`, `consume_data`, `decompress_data`, block smoothing (`smoothing_ok`, `decompress_smooth_data`) and `jinit_d_coef_controller` |
| `jddctmgr` | `jddctmgr.c` | `my_idct_controller`, `multiplier_table`, `start_pass` with the islow, ifast and float multiplier tables, `jinit_inverse_dct` |
| `jdct` | `jdct.h` | the inverse-DCT types and macros (`DCTELEM`, the multiplier types, `IDCT_range_limit`, `RANGE_MASK`, `FIX`, `DESCALE`, `MULTIPLY16C16`, `MULTIPLY16V16`) |
| `jidctint` | `jidctint.c` | `jpeg_idct_islow` and the scaled IDCTs 3x3, 5x5, 6x6, 7x7 and 9x9 to 16x16 |
| `jidctfst` | `jidctfst.c` | `jpeg_idct_ifast` |
| `jidctflt` | `jidctflt.c` | `jpeg_idct_float` |
| `jidctred` | `jidctred.c` | `jpeg_idct_4x4`, `jpeg_idct_2x2`, `jpeg_idct_1x1` |
| `jpegint`, `jpeglib` | `jpegint.h`, `jpeglib.h` | the rest of `jpeg_d_coef_controller`, `jpeg_inverse_dct` and `inverse_DCT_method_ptr`, the `idct` slot, `jpeg_component_info::dct_table` and `sample_range_limit` |

The `coef` slot holds `my_coef_controller`, its one implementation in scope,
and `free_pool(JPOOL_IMAGE)` now also resets `idct` and `sample_range_limit`
(`tools/pools.json` lists `sample_range_limit` under work package 4, which
adds it, although `jdmaster.c` fills it in work package 6).

**Not translated.** `decompress_data_12`, `decompress_data_16`,
`inverse_DCT_12` and the SIMD IDCT slots serve other precisions, lossless
mode and SIMD (ADR 0001, ADR 0004), and so does `jddiffct.c`; the
forward-DCT declarations of `jdct.h` serve the encoder. `WITH_PROFILE` is
off. Nothing calls `jinit_d_coef_controller` or `jinit_inverse_dct` yet:
`master_selection` arrives with `jdmaster.c` in work package 6, and the main
controller that calls `decompress_data` with work package 5.

**Representation.** The controller's methods take its module out of
`cinfo.coef`; `decompress_data` and `decompress_smooth_data` run their input
loop, which reaches `consume_data` through `cinfo`, before they take it. In
single-pass mode the module owns the one-MCU buffer and lends all ten blocks
to `decode_mcu`. In multi-pass mode `consume_data` takes the memory manager's
virtual arrays out of `cinfo.mem` while it decodes an iMCU row and lends each
MCU's blocks to `decode_mcu` from them; `whole_image` holds `None` where C's
`memset` leaves null pointers, and `coef_arrays` is a copy of it. The IDCT
routines take `cinfo` shared beside `compptr` and the block, which may lie in
a virtual array; their output is a row list of the caller's sample array
(`JSAMPROWS_MUT`), and C's pointer walks over the block, the multiplier table
and the workspace are indexes. The multiplier table keeps the bits of C's
union in one array of `int`s. `start_output_pass` returns a `Result` where C
returns nothing, because `smoothing_ok` allocates the latch. `sample_range_limit`
is the whole allocation of `prepare_range_limit_table`, and
`jpeg_decompress_struct::sample_range_limit` returns it from C's pointer on
(ADR 0005, refinements from work package 4).

**Verified against the oracle.** Two probes cover the package, and
`tests/support/wp4.rs` runs the same sequences on the port, with copies of
`jdmaster.c`'s `prepare_range_limit_table` and of the parts of
`master_selection` and `prepare_for_output_pass` that set up these modules.
Both print the memory pools, so the C sizes of `my_coef_controller`, its
virtual arrays, the one-MCU buffer, the workspace, the smoothing latch,
`my_idct_controller`, `multiplier_table` and the range-limit table are
verified.

`tools/oracle/probes/wp4/idct` gives one component a random quantization
table, lets `jinit_inverse_dct` and `start_pass` select the routine for a
method and a scaled size, and hashes the output of every transformed block
(`tests/wp4_idct.rs`):

| Test | Inputs |
| --- | --- |
| `scaled_sizes` | every scaled size from 1 to 16 with the accurate integer method, 400 image-like, DC-only, single-coefficient and dense blocks for each of 3 seeds, and 400 blocks with arbitrary coefficients (including -32768 and 32767) and quantizers up to 65535 |
| `methods` | the islow, ifast and float methods at full size, 1000 blocks for each of 5 seeds and quantizers up to 1, 16 and 255 |
| `start_pass_errors` | an unknown method at full size (`JERR_NOT_COMPILED`), scaled sizes 0, 17 and -1 (`JERR_BAD_DCTSIZE`), and an unknown method at a scaled size, which is accepted |

`tools/oracle/probes/wp4/coefidct` reads a header, sets up the IDCT, the
entropy decoder and the coefficient controller as `master_selection` does,
and runs output passes as `jpeg_start_decompress` and
`jpeg_finish_decompress` do, or as buffered-image mode's progressive-display
loop of `jpeg_start_output` and `jpeg_finish_output` does. It hashes all
output rows after each iMCU row of every pass and prints every result of
`consume_input` and `decompress_data`, the final input and output positions
and `last_good_iMCU_row` (`tests/wp4_coefidct.rs`):

| Test | Inputs |
| --- | --- |
| `test_images_match_the_oracle` | `testorig.jpg`, `testimgari.jpg`, `testimgint.jpg` in six modes: single-pass or full-image decoding, buffered-image mode with and without block smoothing, the ifast and float methods, and a buffered run that changes from float to ifast after the first pass; `monkey12.jpg` fails with `JERR_BAD_PRECISION` |
| `encoder_variants_match_the_oracle` | WP3's 31 `cjpeg` variants in the same six modes |
| `scaled_sizes_match_the_oracle` | the 27 8-bit variants at every scaled size from 1 to 16, the 12 multi-scan ones also in buffered mode, and four mixes of per-component sizes and methods |
| `unneeded_components_and_cropping_match_the_oracle` | the 8-bit variants with components not needed, and with the first and last iMCU and MCU columns set as `jpeg_crop_scanline` sets them, in both modes and with scaling |
| `errors_match_the_oracle` | a scaled size of 17 or 18 (`JERR_BAD_DCTSIZE`) and an unknown method (`JERR_NOT_COMPILED`) in both modes, and a negative size |
| `suspension_matches_the_oracle` | suspending input that releases 1, 7, 64 or 1000 bytes per step, in both modes, for `testorig.jpg` and the progressive, arithmetic, restart, arithmetic progressive and scan-script variants |
| `truncations_match_the_oracle` | the 8-bit variants cut at 12 points of their entropy-coded data, in both modes |
| `mutations_match_the_oracle` | 864 mutations of the 8-bit variants' entropy-coded data as in WP3, in both modes |

Block smoothing changes the output of 9 of the 10 passes of the progressive
variant, so `decompress_smooth_data` and `smoothing_ok` are compared on
partial and complete progression states, DC-only scans included, and the
truncations compare passes over incomplete scans. The
fast integer method computes in `int`, and the float method converts to
`int`; arbitrary coefficients and 16-bit quantizers overflow both in C, so
those methods are compared on image-like blocks and real streams, and the
damaged streams use the accurate method. Locally on macOS ARM64 (Apple
Clang 21), all of this passes against the `release` and `asan-ubsan` oracles
in the dev, release and `checked` profiles.

**Differences by design.** None in the output. A float IDCT output outside
the range of `int`, which is undefined in C, saturates in the port (ADR
0005, refinements from work package 4).

## Work package 5

**Translated.**

| Module | Upstream | Content |
| --- | --- | --- |
| `jdcolor` | `jdcolor.c` | `my_color_deconverter`, the conversion tables, `rgb_gray_convert`, `null_convert`, `grayscale_convert`, `ycck_cmyk_convert`, the 21 instances of `jdcolext.c`'s routines, `start_pass_dcolor`, `jinit_color_deconverter` |
| `jdcolext` | `jdcolext.c` | `ycc_rgb_convert`, `gray_rgb_convert`, `rgb_rgb_convert`, generic over the pixel layout |
| `jdsample` | `jdsample.h`, `jdsample.c` | `my_upsampler`, `sep_upsample`, the full-size, no-op, integral, h2v1, h2v2 and fancy h2v1, h1v2 and h2v2 upsamplers, `jinit_upsampler` |
| `jdmerge` | `jdmerge.h`, `jdmerge.c` | `my_merged_upsampler`, its table, the 1:1 and 2:1 vertical control routines, the 14 instances of `jdmrgext.c`'s routines, `jinit_merged_upsampler` |
| `jdmrgext` | `jdmrgext.c` | `h2v1_merged_upsample`, `h2v2_merged_upsample`, generic over the pixel layout |
| `jdpostct` | `jdpostct.c` | `my_post_controller`, `start_pass_dpost`, `jinit_d_post_controller` |
| `jdmainct` | `jdmainct.h`, `jdmainct.c` | `my_main_controller`, the funny pointer lists (`alloc_funny_pointers`, `make_funny_pointers`, `set_wraparound_pointers`, `set_bottom_pointers`), `start_pass_main`, `process_data_simple_main`, `process_data_context_main`, `jinit_d_main_controller` |
| `jpegint`, `jpeglib` | `jpegint.h`, `jpeglib.h` | `jpeg_d_main_controller`, `jpeg_d_post_controller`, `jpeg_upsampler`, `jpeg_color_deconverter` and the slots `main`, `post`, `upsample` (with its two implementations) and `cconvert` |

`free_pool(JPOOL_IMAGE)` now also resets the four new slots.

**Not translated.** Color quantization is out of scope (ADR 0001): the
`cquantize` slot, `colormap` and `jdpostct.c`'s `post_process_1pass`,
`post_process_prepass` and `post_process_2pass` are absent. With quantization
`start_pass_dpost` fails with `JERR_NOT_COMPILED`, and a full-image buffer
with `JERR_BAD_BUFFER_MODE` as upstream's `#else` does; `jdmainct.c`'s
`process_data_crank_post` is not compiled without `QUANT_2PASS_SUPPORTED`.
RGB565 output (`jdcol565.c`, `jdmrg565.c`) fails with
`JERR_CONVERSION_NOTIMPL` in `jinit_color_deconverter` and
`jinit_merged_upsampler` (ADR 0005). The 12-bit and 16-bit methods and the
SIMD slots of the four interfaces are absent. Work package 6 adds
`master_selection`, which initializes the modules, and `jpeg_read_scanlines`,
which calls `process_data`.

**Representation.** The main controller keeps one sample array per component
and, in the context case, one block of both pointer lists per component as
row offsets with the positions where C's `xbuffer[0][ci]` and
`xbuffer[1][ci]` point; it hands the lists to the coefficient controller as
row lists and to the postprocessor as `JSAMPROWS_AT`, which can reach the
context row group before row 0. `color_buf[ci]` is the upsampler's own array,
the input rows of a full-size component, or nothing. The routines included
once per pixel layout (`jdcolext.c`, `jdmrgext.c`) are generic functions,
and their C instances are named constants. The color converters only read
the object and take it shared; they index `sample_range_limit` below zero
through `sample_range_limit_signed`. The merged upsampler lends its spare row
as the second output row. Method pointers that C leaves unset until a
selection (`process_data`, `post_process_data`, `color_convert`, `upmethod`)
are `None` until then. The methods take their module out of its slot; the
main controller keeps it across the coefficient controller and the
postprocessor, and the upsamplers across the color converter, none of which
reach it (ADR 0005, refinements from work package 5).

**Verified against the oracle.** The probe `tools/oracle/probes/wp5/decode`
reads a header, calls `jpeg_calc_output_dimensions`, sets up the decoder as
`master_selection` does without color quantization (with copies of
`prepare_range_limit_table` and `use_merged_upsample`), starts the output
pass as `jpeg_start_decompress` does and reads every scanline through the
main controller, a given number of rows per call, as `jpeg_read_scanlines`
does. It prints the output dimensions, the choice of the merged upsampler and
of context rows, each component's scaled and downsampled size, a hash of every
output row, every row count `process_data` returned, the final state, the
memory pools (so the C sizes of the four modules, their tables and buffers,
the pointer lists and the spare row are verified) and the error. For the
upstream test images its rows equal `djpeg`'s output with fancy and plain
upsampling, the float method, scaling and grayscale. `tests/support/wp5.rs`
runs the same sequence on the port, with the port's
`jpeg_calc_output_dimensions` (a copy until work package 6) and a copy of
`use_merged_upsample`, and `tests/wp5_decode.rs` compares the outputs
exactly:

| Test | Inputs |
| --- | --- |
| `test_images_match_the_oracle` | `testorig.jpg`, `testimgari.jpg`, `testimgint.jpg` in nine modes: default, plain upsampling (the merged upsampler for 2x2 and 2x1 YCbCr), ifast, float, scales 1/8, 3/8 and 13/8, grayscale output, and 3 and 16 rows per call; `monkey12.jpg` fails with `JERR_BAD_PRECISION` |
| `encoder_variants_match_the_oracle` | the 27 8-bit `cjpeg` variants of the earlier packages in the same nine modes |
| `color_spaces_match_the_oracle` | every in-scope output color space (0 to 15) from YCbCr 2x2, 2x1 and 1x1, grayscale, RGB, progressive, CMYK, YCCK and subsampled YCCK input (the last three encoded by the probe `wp5/cmyk`), with fancy and plain upsampling and at half size |
| `scaling_and_sampling_match_the_oracle` | the default, sampling (1x1, 2x1, 1x2, 4x1, 3x2) and grayscale variants at every scale from 1/8 to 16/8, with fancy and plain upsampling and to RGBA |
| `rows_per_call_match_the_oracle` | every 8-bit variant read 1, 2, 3, 5, 16 and 200 rows per call, with fancy and plain upsampling |
| `errors_match_the_oracle` | `CCIR601_sampling` (`JERR_CCIR601_NOTIMPL`), unsupported conversions (`JERR_CONVERSION_NOTIMPL`) and fractional sampling factors (`JERR_FRACT_SAMPLE_NOTIMPL`) |
| `suspension_matches_the_oracle` | suspending input that releases 1, 7, 64 or 1000 bytes per step, for `testorig.jpg` and the progressive, arithmetic, restart and 1x2 progressive variants, with fancy and plain upsampling |
| `damaged_streams_match_the_oracle` | the 8-bit variants cut at 6 points and mutated at 4 positions, with fancy and plain upsampling |
| `rgb565_is_out_of_scope` | the port alone: RGB565 output fails with `JERR_CONVERSION_NOTIMPL` through the color deconverter and through the merged upsampler |

The cases reach every upsampling method (full-size, integral, h2v1, h2v2,
fancy h2v1, h1v2 and h2v2, both merged routines), every color conversion
routine and pixel layout (of `null_convert`, the three- and four-component
loops; its general loop needs a stream with another component count in an
unknown color space, which no input here has), both main controller modes with their bottom and
wraparound pointer handling, and the merged upsampler's spare row. Locally on
macOS ARM64 (Apple Clang 21), all of this passes against the `release` and
`asan-ubsan` oracles in the dev, release and `checked` profiles.

**Differences by design.** RGB565 output, which the oracle produces, fails
in the port (ADR 0005).

## Work package 6

**Translated.**

| Module | Upstream | Content |
| --- | --- | --- |
| `jdmaster` | `jdmaster.c` | `use_merged_upsample`, `jpeg_core_output_dimensions`, `jpeg_calc_output_dimensions`, `prepare_range_limit_table`, `master_selection`, `prepare_for_output_pass`, `finish_output_pass`, `jpeg_new_colormap`, `jinit_master_decompress` |
| `jdapistd` | `jdapistd.c` | `jpeg_start_decompress`, `output_pass_setup`, `jpeg_crop_scanline`, `jpeg_read_scanlines`, `noop_convert`, `read_and_discard_scanlines`, `increment_simple_rowgroup_ctr`, `jpeg_skip_scanlines`, `jpeg_read_raw_data`, `jpeg_start_output`, `jpeg_finish_output` |
| `jdapimin` | `jdapimin.c` | `jpeg_finish_decompress` |
| `jpegint`, `jpeglib` | `jpegint.h`, `jpeglib.h` | the methods `prepare_for_output_pass` and `finish_output_pass` of `jpeg_decomp_master`, and the call of the application's `progress_monitor` |

`jdapistd.c` calls two inline functions of the module headers,
`jdmainct.h`'s `set_wraparound_pointers` and `jdcoefct.h`'s
`start_iMCU_row`, which are now `pub`.

**Not translated.** Color quantization is out of scope (ADR 0001):
`noop_quantize` and the quantizer selection are absent, `master_selection`
fails with `JERR_NOT_COMPILED` for quantization, and a dummy pass of two-pass
quantization fails the same way, as upstream's `#else` does. 12-bit images
and a 12-bit output precision fail with `JERR_BAD_PRECISION` where upstream
starts its 12-bit modules, and lossless images with `JERR_NOT_COMPILED`
(ADR 0005). `WITH_PROFILE` is off.

**Representation.** The public functions return `Result`, with C's
`boolean` and `JDIMENSION` results inside. `jpeg_crop_scanline` takes C's
pointers as `Option<&mut JDIMENSION>`, so a null pointer still fails with
`JERR_BAD_CROP_SPEC`. The application's rows are `JSAMPROWS_MUT`, and raw
data is a row list per component. The progress monitor's methods are taken
out of `jpeg_progress_mgr` for the call. `prepare_range_limit_table` keeps
the whole table, as earlier packages read it. To discard lines of the merged
2:1 upsampler, the port reads them into a row of the spare row's length
instead of the spare row itself. Where C would call the color quantizer that
an application enabled after `jpeg_start_decompress`, the port fails with
`JERR_NOT_COMPILED` (ADR 0005, refinements from work package 6).

**Verified against the oracle.** The probe `tools/oracle/probes/wp6/api`
decompresses through the public functions as an application does:
`jpeg_read_header`, the options, `jpeg_start_decompress`, an output pass,
and `jpeg_finish_decompress`. The output pass follows a plan of
`jpeg_read_scanlines`, `jpeg_skip_scanlines` and `jpeg_crop_scanline` calls,
calls at the wrong time and an early end, and then reads the rest a given
number of rows per call. With options it reads raw data instead, or runs
every output pass of buffered-image mode. It repeats any call that the
suspending source makes return early, installs a progress monitor, and can
decompress a second time with the same object. It prints every message, the
output dimensions and state, every crop, a hash of every output row, every
count the calls returned, the progress monitor's calls and a hash of its
fields, the final state, the memory pools before `jpeg_finish_decompress`
and the error. For the upstream test images its rows equal `djpeg`'s output
with fancy and plain upsampling, scaling, grayscale output, the float
method, and `djpeg -crop`, which crops and skips. `tests/support/wp6.rs`
runs the same sequence on the port, and `tests/wp6_api.rs` compares the
outputs exactly:

| Test | Inputs |
| --- | --- |
| `test_images_match_the_oracle` | `testorig.jpg`, `testimgari.jpg`, `testimgint.jpg` in ten modes: default, plain upsampling, ifast and float with several rows per call, scales 1/8, 3/8 (after `jpeg_calc_output_dimensions`) and 13/8, grayscale and BGRX output, the progress monitor, one more read and skip after the last row, and a second decompression |
| `encoder_variants_match_the_oracle` | the 27 8-bit `cjpeg` variants of the earlier packages in the same ten modes |
| `crops_match_the_oracle` | the test images and six sampling, grayscale and progressive variants, cropped at ten offsets and widths (down to one sample, which reinitializes the upsampler) at four scales, with fancy and plain upsampling |
| `skips_match_the_oracle` | the test images and seven variants, with 26 plans of skips at every position relative to the iMCU rows, after reads and crops, with context rows, the simple main controller and the merged upsampler, at full and half size |
| `skips_near_the_end_match_the_oracle` | four inputs: reads of 0 to 33 rows, then skips that end 1 to 24 rows before the last |
| `scales_match_the_oracle` | six inputs at every scale from 1/8 to 16/8, with crops, skips and raw data |
| `raw_data_matches_the_oracle` | every 8-bit variant as raw data at three scales and with a longer buffer; a short buffer fails with `JERR_BUFFER_SIZE` |
| `buffered_image_matches_the_oracle` | every 8-bit variant in buffered-image mode: all passes, one or two passes, scan numbers out of range, skips and crops within a pass, the progress monitor, and a second decompression |
| `sequential_multiscan_matches_the_oracle` | a sequential file with one scan per component, with skips, buffered-image mode, raw data and suspension |
| `color_spaces_match_the_oracle` | every in-scope output color space from YCbCr, CMYK, YCCK, RGB and grayscale input, also cropped and skipped |
| `errors_match_the_oracle` | 36 calls at the wrong time and invalid requests: `JERR_BAD_STATE`, `JERR_BAD_CROP_SPEC`, `JERR_WIDTH_OVERFLOW`, `JERR_TOO_LITTLE_DATA`, `JERR_NOTIMPL`, `JERR_BAD_PRECISION`, `JERR_MODE_CHANGE`, `JERR_CCIR601_NOTIMPL`, `JWRN_TOO_MUCH_DATA`, extreme scale factors |
| `suspension_matches_the_oracle` | suspending input that releases 1, 7, 64 or 1000 bytes per step, for the test images and three variants, through scanlines, skips, raw data and buffered-image mode, twice per object |
| `damaged_streams_match_the_oracle` | the 8-bit variants cut at 5 points and mutated at 3 positions, read whole, with skips and in buffered-image mode |
| `out_of_scope_requests_fail` | the port alone: 12-bit images and a 12-bit output precision fail with `JERR_BAD_PRECISION`, color quantization and a lossless image with `JERR_NOT_COMPILED` |

The cases reach every branch of `jpeg_skip_scanlines`,
`jpeg_crop_scanline` and the master selection that 8-bit lossy decoding can
take, with these exceptions. `jpeg_skip_scanlines` calls
`finish_input_pass` when its iMCU loop reaches the last iMCU row; no test
reaches it, and neither does a sweep of about 39,000 reads and skips over four
image heights, because a skip that far is cut at the end of the image
first. `read_and_discard_scanlines` passes C's null row list only for the
merged 1:1 upsampler, which never reads there. The branches for other
precisions, lossless mode and color quantization fail as described above.
Locally on macOS ARM64 (Apple Clang 21), all of this passes against the
`release` and `asan-ubsan` oracles in the dev, release and `checked`
profiles.

**Differences by design.** 12-bit and lossless images, a 12-bit output
precision and color quantization, which the oracle decodes, fail in the
port (ADR 0005).

## Work package 7

**Scope change.** The port plan first had work package 7 export a C ABI, so
that the unmodified `djpeg` and upstream's fuzzers could link the port. The
port serves Rust programs, and C compatibility is not a goal, so it exports
no C ABI and stays free of `unsafe`
([ADR 0003](adr/0003-toolchain-and-unsafe-policy.md)). Upstream's bit tests
run on the port through a Rust driver instead
([ADR 0002](adr/0002-differential-oracle.md)).

**The driver.** `tests/support/djpeg.rs` translates, as test code, the
library calls of `djpeg.c`'s `main` and `parse_switches` (with `keymatch`
and the `print_text_marker` processor for COM and APP12) and the 8-bit
PPM/PGM writer of `wrppm.c` with `cmyk_to_rgb`, as upstream builds them
(`DJPEG_MULTILINE` undefined). Switches that select another output format,
color quantization, RGB565, another precision, a progress monitor, a memory
limit or `-strict` are outside it and fail with a message. The input is
attached with `jpeg_mem_src`, because the stdio source is out of scope.
`tests/support/md5.rs` is MD5 after RFC 1321, checked with the RFC's test
suite.

**The bit tests.** `tools/bittests.py --run` runs the 30 in-scope cases of
[tools/bittests.json](../tools/bittests.json) against the oracle, produces
their inputs with the oracle's `cjpeg` and checks those inputs' MD5 sums, and
writes `bittests-port.txt` next to `oracle.json`: each case's input,
switches, output file and MD5 checks, taken from upstream's CTest suite.
`tests/wp7_bittests.rs` reads it and runs every case on the port:

| Test | What it checks |
| --- | --- |
| `upstream_bit_tests_pass_on_the_port` | all 30 in-scope cases: their PPM outputs and the ICC profile of `rgb-islow` have upstream's MD5 sums, without warnings. They cover the DCT methods, fancy, plain and merged upsampling, progressive and arithmetic files, every scale of `-scale`, `-skip` and `-crop` |
| `driver_matches_the_oracle_djpeg` | the driver itself: its output and ICC file equal the oracle's `djpeg`'s byte for byte for the three test images, CMYK, YCCK, progressive and grayscale input with 14 switch sets (PPM, PGM, `-rgb`, `-fast`, scaling, the float method, skips, crops and `-icc`); 80 runs compare bytes, and 18 fail in both, for crops and skips beyond the image and conversions that four-component input does not support |

Locally on macOS ARM64 (Apple Clang 21), all of this passes against the
`release` and `asan-ubsan` oracles in the dev, release and `checked`
profiles.

**Not covered.** Upstream's fuzzers are C programs and would need a C ABI.
The damaged-stream tests of work packages 2 to 6 cover corrupt input against
the oracle; coverage-guided fuzzing in Rust needs a development dependency
and is a separate decision (ADR 0002, ADR 0003).
