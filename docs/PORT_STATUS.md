# Port status

This repository contains bitstream/header and ANS-table slices. It cannot encode or
decode JPEG XL images. There are no C++ runtime fallbacks and no placeholder
codec APIs. The source revision is pinned in [upstream.json](../upstream.json).

## Source mapping

| Upstream source under `lib/jxl/` | Rust module under `src/jxl/` | Boundary |
| --- | --- | --- |
| `dec_bit_reader.h`, `.cc` | `dec_bit_reader.rs` | Reader, checkpoints, native moves, scoped closer; unused private aligned-word helper omitted |
| `enc_bit_writer.h`, `.cc` | `enc_bit_writer.rs` | Bit writes, byte/unaligned appends, nested allotments and charging; custom allocator and padded-buffer transfer unported |
| `field_encodings.h` | `field_encodings.rs` | Fields trait, U32 distributions, raw enum enumeration and validation |
| `fields.h`, `.cc` | `fields.rs` | Bits/U32/U64/F16 coders, bundle visitors, and enum visitor helper |
| `base/bits.h` | `base/bits.rs` | Unsigned 32/64-bit and pointer-width zero counts and logarithms |
| `base/byte_order.h` | `base/byte_order.rs` | Endian loads/stores, float-bit conversions, and order-tag helpers |
| `../include/jxl/types.h` | `types.rs` | Endianness only; other public API types unported |
| `ans_params.h` | `ans_params.rs` | ANS and prefix coding constants |
| `ans_common.h`, `.cc` | `ans_common.rs` | Alias table construction/lookup, flat histograms, count precision; prefetch unported |
| `enc_fields.cc` | `enc_fields.rs` | Primitive writes, Bundle::Write and size-header wrapper; full codestream/frame/quantizer writes unported |
| `headers.h`, `.cc` | `headers.rs` | Size, preview, animation, and size-header read wrapper |
| `base/status.h` | `base/status.rs` | Status-code/Result mapping; diagnostics and crash instrumentation unported |
| `padded_bytes.h` | `padded_bytes.rs` | Writer storage subset; other container operations unported |
| `memory_manager_internal.h`, `.cc` | `memory_manager_internal.rs` | Aligned allocation subset using Rust allocator; custom callbacks and image-row sizing unported |
| `enc_aux_out.h`, `.cc` | `enc_aux_out.rs` | Layer totals and writer charging; image statistics and diagnostic output unported |

Internal storage mappings are specified in the
[representation ADR](adr/0003-rust-representation.md). This table counts
translated boundaries, not whole-file or whole-library completion.

## Evidence

Validated on 2026-10-31 on macOS arm64 with Rust 1.99.0 (stable), Rust 1.85.1
(minimum-version series), and Apple Clang 21.0.0. The C++ oracle is built from
the unmodified pinned source with scalar Highway and release status behavior.

Command: `python3 tools/check.py --sanitizers --msrv`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Source revision, Highway pin, notices, local doc links | PASSED | Checked against `upstream.json` and the Git submodule entries |
| Rust formatting and Clippy | PASSED | All targets and features, warnings treated as errors |
| Rust debug | PASSED | 44 tests, including 14 differential groups; zero ignored cases |
| Rust release | PASSED | Same 44 tests and corpus |
| Rust 1.85.1 | PASSED | Same 44 tests and corpus |
| C++ ASan and UBSan | PASSED | Fail-fast instrumentation during all three Rust test configurations |

Each differential run compares **224,790 requests** against C++. These include
all 65,536 binary16 input patterns; primitive encodings; header limits, aspect
ratios and truncations; reader refills and checkpoints; nested accounting;
appends; extension compatibility; default/nested bundle control flow; raw enum
validation; unsigned word/byte helpers; and ANS alias tables. Alias comparisons
include all 4,096 lookups for every successful generated table.
The native writer test additionally round-trips 1,048,576 generated bit patches.

The default-bundle test first failed on an early Rust return after a truncated
`AllDefault` predicate: C++ consumed five bits and updated the next field, while
Rust consumed one bit and left that field unchanged. The corrected predicate
mapping passes that regression; see
[ADR 0003](adr/0003-rust-representation.md).

The pinned Highway CMake file emits a CMP0111 deprecation warning. It does not
fail the build; the reference source has not been edited to suppress it.

Not yet established: complete image decoding/encoding, public C ABI compatibility,
thread runners, SIMD parity, full upstream test inventory, external conformance
corpus, cross-platform CI results, fuzzing, Rust sanitizer/Miri results, or
performance/memory parity. Test passing here does not establish any of those.
The Linux/macOS workflow is configured but has not run on a hosted service.

## Next dependency boundaries

1. Port Huffman trees, code-length serialization, and prefix decoding with C++ comparisons.
2. Build ANS stream decoding/encoding on the translated alias tables.
3. Port image metadata and frame headers, then the first actual frame path.
4. Expand through modular and VarDCT decoding and encoding before introducing
   the C ABI and optimized target-specific kernels.

Preserve the upstream test inventory as that scope grows. Adapted tests in this
first slice do not replace the unported image, threading, allocation-failure,
platform, or codec test suites.
