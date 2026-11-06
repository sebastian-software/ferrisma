# WP2: bit I/O

- Status: every unit of the WP2 inventory is translated, except
  `VP8RemapBitReader`. That function is an open design question (below), so it
  stays absent until the integrator answers it. Everything else is verified
  against the release oracle, and the sanitizer and debug variants agree.
- Sub-wave: 1b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1
- Probes: `tools/oracle/probes/wp2/`; tests: `tests/wp2_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Open question: `VP8RemapBitReader`

`VP8RemapBitReader` (`src/utils/bit_reader_utils.c`) moves a reader's pointers
by an offset. The incremental decoder calls it in `DoRemap`
(`src/dec/idec_dec.c`) when its input memory moves: in `MEM_MODE_MAP`, the
readers of the partitions and the first partition are moved, and
`VP8LBitReaderSetBuffer` re-points the lossless reader at the moved memory.

ADR 0003 turns a pointer into a buffer into a slice plus an offset, so each
reader borrows its input (`VP8BitReader<'a>` holds `&'a [u8]`). The incremental
decoder owns that memory and holds its readers at the same time. Safe Rust does
not allow a struct to own a buffer and borrow from it, so the representation
and the relocation conflict. The ADRs do not settle this. Options for the
integrator:

1. Readers hold no reference to their input, and each reader call takes the
   slice. This changes the signatures that WP6, WP7 and WP12 call.
2. Each reader is split into its state (register, range, bit count, offsets,
   end flag) and a borrowed input that the owner supplies. The owner moves the
   state with its memory, and the remap becomes "replace the input, keep the
   state". This keeps the C shape of the calls.
3. An ADR amendment that gives the incremental decoder another ownership form.

Until this is decided, the reader representation of this package is the one
ADR 0003 gives. `VP8RemapBitReader` is not translated, and WP12's incremental
decoder depends on the answer.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/utils/bit_reader_utils.rs` | `src/utils/bit_reader_utils.c`, `src/utils/bit_reader_utils.h` | partly: everything except `VP8RemapBitReader` (open question). Translated: `BITS` (chosen by target, deviation 6), `bit_t`, `range_t`, `VP8BitReader`, the tables `kVP8Log2Range` (`VP8_LOG2_RANGE`) and `kVP8NewRange` (`VP8_NEW_RANGE`), `VP8BitReaderSetBuffer`, `VP8InitBitReader`, `VP8LoadFinalBytes`, `VP8GetValue`, `VP8GetSignedValue`, `VP8Get`, `VP8LBitReader`, `vp8l_val_t`, `VP8L_LBITS`, `VP8L_WBITS`, `VP8L_MAX_NUM_BIT_READ`, `VP8L_USE_FAST_LOAD`, `kBitMask` (`BIT_MASK`), `VP8LInitBitReader`, `VP8LBitReaderSetBuffer`, `VP8LSetEndOfStream`, `ShiftBytes`, `VP8LReadBits`, `VP8LDoFillBitWindow`, and the header's inline functions `VP8LPrefetchBits`, `VP8LIsEndOfStream`, `VP8LSetBitPos` and `VP8LFillBitWindow`. |
| `src/utils/bit_reader_inl_utils.rs` | `src/utils/bit_reader_inl_utils.h` | yes: `lbit_t`, `VP8LoadNewBytes`, `VP8GetBit`, `VP8GetSigned` and `VP8GetBitAlt`; the helper `low_range` stands for the `(range_t)` cast (deviation 4). |
| `src/utils/bit_writer_utils.rs` | `src/utils/bit_writer_utils.c`, `src/utils/bit_writer_utils.h` | yes: `VP8BitWriter` with `VP8PutBit`, `VP8PutBitUniform`, `VP8PutBits`, `VP8PutSignedBits`, `VP8BitWriterInit`, `VP8BitWriterFinish`, `VP8BitWriterAppend`, `VP8BitWriterWipeOut`, `VP8BitWriterPos`, `VP8BitWriterBuf` and `VP8BitWriterSize`, with the internal `BitWriterResize` and `Flush`; `kNorm` (`NORM`), `kNewRange` (`NEW_RANGE`), `MIN_EXTRA_SIZE`; the `VP8L_WRITER_*` constants and the types `vp8l_atype_t` and `vp8l_wtype_t`, chosen by target (deviation 9); `VP8LBitWriter` with `VP8LBitWriterNumBytes`, `VP8LBitWriterInit`, `VP8LBitWriterClone`, `VP8LBitWriterWipeOut`, `VP8LBitWriterReset`, `VP8LBitWriterSwap`, `VP8LBitWriterFinish`, `VP8LPutBitsFlushBits`, `VP8LPutBitsInternal` and the inline `VP8LPutBits`. |

Every translated item carries an `// upstream:` marker. `#![deny(unsafe_code)]`
holds: none of the three modules uses `unsafe`.

## Evidence

Each test runs the probe in a separate process against the oracle built by
`python3 tools/build_oracle.py`. The probes print one `key=value` line per
observation, after every command, and the tests compare the whole output with the
Rust results. A probe run is compared field by field, so a difference in any
bit, offset, flag or byte fails the test. The counts are the observations
compared in one run, on this host (macOS ARM64, release oracle).

| Test file | Test | Observations | What it compares |
| --- | --- | ---: | --- |
| `tests/wp2_config.rs` | `target_configuration_matches_the_c_build` | 9 | `BITS`, the sizes of `bit_t`, `lbit_t`, `vp8l_atype_t` and `vp8l_wtype_t`, `VP8L_WRITER_BYTES`, `VP8L_WRITER_BITS`, `VP8L_WRITER_MAX_BITS`, and the fast-load mode, which the probe observes rather than reads. |
| `tests/wp2_config.rs` | `widths_follow_the_target_choice`, `lossless_reader_constants_match_the_header` | (no oracle) | Consistency of the configuration and of the header constants. |
| `tests/wp2_bit_reader.rs` | `bit_reader_states_match_the_c_reader` | 246,049 | About 30,000 commands (`VP8GetBit`, `VP8GetBitAlt`, `VP8GetValue`, `VP8Get`, `VP8GetSignedValue`, `VP8GetSigned`) on 55 streams: every length from 0 to 40 bytes, the lengths around the seven-byte loads, 255 to 4,096 bytes, constant and alternating bytes. After every command: the result and the register, range, bit count, the offsets of the next byte and of the limit, and the end flag. |
| `tests/wp2_bit_reader.rs` | `truncated_streams_set_eof_and_shift_zeros` | 14,442 | Streams of 0, 1, 2, 7, 8 and 9 bytes read for 300 bits past their end: the end flag and the zero shifts after it. |
| `tests/wp2_bit_reader.rs` | `signed_and_value_reads_match_the_c_reader` | 68,680 | 24 seeded streams of up to 200 bytes, with `VP8GetValue`, `VP8GetSignedValue`, `VP8Get` and `VP8GetSigned` interleaved. |
| `tests/wp2_vp8l_bit_reader.rs` | `lossless_reader_states_match_the_c_reader` | 180,939 | Random commands (`VP8LReadBits` up to 32 bits, `VP8LPrefetchBits`, `VP8LSetBitPos`, `VP8LFillBitWindow`, `VP8LDoFillBitWindow`, `VP8LIsEndOfStream`, `VP8LBitReaderSetBuffer`) on 48 buffers from 0 to 1,000 bytes. After every command: the window, the position, the length, the bit position and the end flag. |
| `tests/wp2_vp8l_bit_reader.rs` | `prefetch_window_refills_match_the_c_reader` | 18,204 | Every length from 0 to 40 bytes, read with 24-bit reads, fills and bit positions near the 64-bit window and the four-byte refill. |
| `tests/wp2_vp8l_bit_reader.rs` | `set_buffer_keeps_the_position_and_recomputes_eos` | 4,129 | Moves to other buffers, shorter and longer, keeping the position and recomputing the end flag. |
| `tests/wp2_bit_writer.rs` | `bit_writer_states_and_bytes_match_the_c_writer` | 683,308 | 20 sequences from 0 to 70,000 expected bytes, with bits, uniform bits, signed values, appends, finishes and byte reads, including carries through `0xff` runs. After every command: the state, the allocated size (the growth policy) and the error flag. The bytes at each read and at the finish. |
| `tests/wp2_bit_writer.rs` | `refused_allocation_sets_the_error_and_writing_continues` | 31,595 | An expected size of 2^34 + 1 bytes, above `WEBP_MAX_ALLOCABLE_MEMORY`, is refused before any memory is reserved; the error flag is set, and the encoder keeps writing. |
| `tests/wp2_vp8l_bit_writer.rs` | `lossless_writer_states_and_bytes_match_the_c_writer` | 544,985 | Six sequences of `VP8LPutBits` with widths from 0 to 32, with `Clone`, `Reset` (after a clone of the same source), `Swap`, `Init` from 0 to 40,000 bytes, and at the end `Finish`, `Written` and `WipeOut`. |
| `tests/wp2_vp8l_bit_writer.rs` | `refused_allocation_sets_the_error_and_writing_continues` | 22,359 | A refused writer, then a finish that allocates it, and a clone that carries the error flag to the other writer. |

The generators of the two lossless tests run the Rust reader or writer in
lockstep, because some calls are undefined in C and the tests must not issue
them (deviation 13). The tables are also compared with the C source text: the five
tables `kVP8Log2Range`, `kVP8NewRange`, `kBitMask`, `kNorm` and `kNewRange` have
the same 128 or 25 values in both.

Local results, on this branch:

- `python3 tools/check.py --oracle` passes: formatting, Clippy with
  `-D warnings`, provenance and the module skeleton, documentation links,
  `tools/unit_dependencies.py --check` (no undeclared references), and the tests
  of the dev, release and `checked` profiles, with the oracle feature (no skips).
  Each profile runs 26 tests, 13 of them WP2, and none failed. The same 26 pass
  with the Rust 1.95 toolchain (`--msrv`), which the check also runs.
- `python3 tools/check.py --oracle-variant asan-ubsan --stage test` passes: 26
  tests in each profile, none failed, and no sanitizer report from the probes.
- The WP2 test binaries pass against the `debug` oracle, which keeps upstream's
  assertions (13 tests). `tools/check.py --oracle-variant debug` stops earlier, in
  the WP1 probe `safe_alloc`, which aborts on `assert(nmemb * size > 0)` for zero
  counts (`src/utils/utils.c`). CI runs only the baseline for the debug variant,
  so this is not a WP2 result; the WP1 owner should see it.
- Three mutations of the Rust code were each caught by the tests: the fast-load
  path disabled (3 failing tests), the carry of the byte writer removed (2), and
  the bit-0 branch of `VP8GetBit` changed (3). The files were restored afterwards.

## Not-applicable checks

These checks do not apply to WP2, or cannot be made here, with the reason.

1. Layout of `repr(C)` mirrors. None of the four structs is exported by a public
   header; the C declarations are internal to `bit_reader_utils.h` and
   `bit_writer_utils.h`. ADR 0004 gives no ABI mirror to them.
2. The bit-trace tool (`BITTRACE`, `BitTrace`, `PrintBitTraces`) is off in the
   reference configuration (ADR 0002). It is not translated, and the labels that it
   adds to the reader functions are removed, as the header's macros do.
3. The MIPS32 branch of `VP8LoadNewBytes` (`WEBP_USE_MIPS32`) is excluded with the
   MIPS sources (`tools/units.json`). Its unaligned load gives the same value as the
   `memcpy` path that is translated.
4. The big-endian branch of `VP8LoadNewBytes` is not separately compiled or tested.
   The translated formula, the big-endian value of the bytes read, gives the same
   bits on every byte order, and the host is little-endian.
5. The other `BITS` values of the header (8, 16, 32 and 48) are selected by no
   target that upstream names, so they are not translated. `bit_t` and `lbit_t`
   follow the two selected values.
6. The sanitizer attributes (`WEBP_UBSAN_IGNORE_UNDEF`,
   `WEBP_UBSAN_IGNORE_UNSIGNED_OVERFLOW`) have no counterpart. The wrapping
   operations of `VP8GetSigned` state the unsigned arithmetic that they permit.
7. The C assertion `sizeof(vp8l_wtype_t) == 2` of `VP8LPutBitsInternal` holds on
   every path that calls the function, the 16-bit accumulator path, so it is not
   checked at run time.
8. The failure branch of `VP8LPutBitsFlushBits` (the cursor returns to the start,
   and the error flag is set) is reached only when the growth would exceed
   `WEBP_MAX_ALLOCABLE_MEMORY`, which the tests cannot reach without reserving
   16 GiB. The arithmetic is the same as the refused `Init`, which is tested.
9. `VP8BitWriterFinish` and `VP8LBitWriterFinish` return the buffer as a borrow,
   and the C callers free it through `WipeOut`. The translation returns the whole
   allocation, as C returns the pointer; the callers take the size from
   `VP8BitWriterSize` or `VP8LBitWriterNumBytes`.
10. The host-specific writer state. On this host (macOS ARM64, not x86-64) the
    lossless writer uses the 32-bit accumulator and 16-bit write words; x86-64 uses
    64 and 32 bits. The tests compare the internal state, so they check the path of
    the host they run on. `VP8LPutBitsInternal` is reached only by the 16-bit words,
    so the x86-64 CI host exercises the 32-bit path alone, and the macOS ARM64 host
    the 16-bit path.

## Recorded deviations

Deviations are changes of representation that ADR 0003 allows, or behavior that
the Rust side cannot reproduce exactly. Each records the reason.

1. **Borrowed input.** `VP8BitReader<'a>` and `VP8LBitReader<'a>` hold their input
   as `&'a [u8]`, with `usize` offsets for the pointers (ADR 0003). The reader
   functions take the slice given to `init` or `set_buffer`. This is the open
   question above.
2. **Public fields.** The fields of the four structs are `pub`, as the C structs are
   public in their headers, so that the tests can read the state. The C fields
   `buf_end` and `len` are not stored: they are the slice length.
3. **Writer buffers.** `VP8BitWriter::buf` and `VP8LBitWriter::buf` are `Vec<u8>`.
   The length of the vector is the allocated size (`max_pos`, or `end - buf`),
   and `cur` is an offset. Allocations use `webp_safe_calloc::<u8>(n, 1)`, which
   applies the size test of `WebPSafeMalloc(1, n)` with the same decision (`n`
   and `1` in place of `1` and `n`), and it zero-fills the buffer, as ADR 0003
   requires. C's `malloc` does not zero-fill.
4. **`(range_t)` cast.** `low_range` keeps the low 32 bits of the register window,
   as the C cast does. A corrupt stream can make the window wider than 32 bits; a
   checked conversion would panic where C continues. The tests found this case.
5. **Probabilities and signed reads.** `prob` is an `i32` as in C. Its unsigned
   conversion wraps, as C's, so a negative probability gives the same result.
   `VP8PutSignedBits` takes the magnitude with `unsigned_abs`, which defines the
   case of `INT_MIN`, where C negates an `int` and is undefined.
6. **`BITS` and `bit_t`.** `BITS` is 56 on x86-64, AArch64 and WebAssembly and 24 on
   every other target, as upstream selects them; `bit_t` is 64 bits when `BITS > 24`.
   The oracle reports the same values on this host (`tests/wp2_config.rs`).
7. **Labels removed.** The `label` parameters of the reader functions are removed
   (`BITTRACE` is off), so `VP8GetValue(br, n)` and `VP8GetBit(br, prob)` are the
   Rust signatures.
8. **Lengths of the input.** `VP8InitBitReader` and `VP8LInitBitReader` take the
   slice, so the C size argument is the slice length. The assertion `size < 1 << 31`
   is a debug assertion.
9. **Writer constants.** `VP8L_WRITER_BYTES`, `VP8L_WRITER_BITS` and
   `VP8L_WRITER_MAX_BITS` are chosen by target: 4, 32 and 64 on x86-64, and 2, 16
   and 32 elsewhere. The types `vp8l_atype_t` and `vp8l_wtype_t` follow them. The
   output bytes are the same for both choices; the internal state is not.
10. **Bit values.** `VP8PutBit` and `VP8PutBitUniform` take and return a `bool`,
    because the C `int` is used only as a predicate. `VP8GetBit`, `VP8GetSigned`
    and `VP8GetBitAlt` return an `i32`, because the decoder uses the bit as a value.
11. **Writer lifecycle.** `VP8BitWriterWipeOut` and `VP8LBitWriterWipeOut` reset the
    struct to `Default`, which is the zero state that C's `memset` leaves.
    `VP8LBitWriterReset` copies the cursor and the bits but no bytes, as C does.
12. **Errors.** The writers keep `error` as a `bool`. A refused allocation sets it,
    and it stays set until `Init` or `WipeOut` clears it, as in C.
13. **Undefined behavior of the C code, excluded.** The asan-ubsan oracle found
    these, and the generators keep the tests away from them. Each is recorded here
    as an input that C does not define, and no test compares it.
    - `VP8LPutBitsInternal` with 32 bits at `used == 0` on the 16-bit path shifts a
      `uint32_t` by 32. The Rust `>>=` panics in dev and `checked` builds. The
      generator narrows such a call to 31 bits.
    - Writes after `VP8LBitWriterFinish`, whose cursor may be odd, store a 16-bit or
      32-bit word at an odd address (`*(vp8l_wtype_t*)bw->cur`). The sequences
      finish and wipe a writer only at their end.
    - A write into a writer without a buffer (after a refused `Init`) adds the cursor
      offset to a null pointer. The Rust version resizes the empty vector and writes.
      The refused-allocation test issues only writes that do not flush, then a finish.
    - `VP8LBitWriterReset` to a cursor whose bytes the destination does not hold:
      a resize copies only the bytes before the cursor, and C then reads memory that
      it never wrote. The tests reset only after a clone of the same source.
    - `VP8LBitWriterClone` from a writer without a buffer passes a null pointer to
      `memcpy`. No sequence wipes a writer before a clone.
    - `VP8LBitReaderSetBuffer` with a buffer shorter than the reader's position is a
      parameter error (the comment of the function says so), and upstream's own
      debug assertion in `VP8LIsEndOfStream` fires there. The Rust debug assertion
      mirrors it. The tests substitute `fill` for such a call.
    - `VP8LDoFillBitWindow` requires `bit_pos >= VP8L_WBITS` (debug assertion). The
      tests call it only there.
    - `VP8LReadBits` with a negative count indexes `kBitMask` out of range. The
      Rust index panics in every profile, after the debug assertion; C reads outside
      the table. No test issues it.
