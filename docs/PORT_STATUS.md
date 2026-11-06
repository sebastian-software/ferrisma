# Port status

| WP | State | Evidence |
| --- | --- | --- |
| 0 Tooling and representation | in review | Oracle, probes, bit-test inventory, unit map, check script and CI below; [ADR 0005](adr/0005-rust-representation.md) accepted |
| 1–11 | not started | — |

No upstream unit is translated. The 37 modules under `src/` hold only their
provenance headers ([tools/units.json](../tools/units.json)).

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
