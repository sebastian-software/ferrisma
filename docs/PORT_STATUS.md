# Port status

| WP | State | Evidence |
| --- | --- | --- |
| 0 Tooling and representation | done | Oracle, probes, bit-test inventory, unit map, check script and CI below; [ADR 0005](adr/0005-rust-representation.md) accepted |
| 1 Common infrastructure | in review | 11 modules translated; message, memory and utility probes below |
| 2–11 | not started | — |

Work package 1's 11 modules are translated. The other 26 modules under `src/`
hold only their provenance headers ([tools/units.json](../tools/units.json)).

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
