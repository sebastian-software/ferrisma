# Shared SharpYUV core: local extraction evidence

**Date:** 2026-11-11. **Base:** `cac086a4b469c368edbaaca9ca14f41eb5b54505`
plus the two PR #176 evidence commits, before this extraction. This record
supports [ADR-0033](../adr/0033-shared-sharpyuv-conversion-core.md).

## Correctness and boundaries

On macOS ARM64, `cross_port` passed all three tests in `checked` and `release`:
2,880 RGB/YUV/matrix/transfer/dimension combinations, 16 signed-layout
combinations repeated three times, and ten defined rejections. The 16
signed-layout output hashes matched the pinned SharpYUV 0.4.2 C fixture.
The independent C API comparison passed all 1,271 groups in debug and release;
the ABI comparison passed 23 checks in both profiles. Allocation fault
injection retained all seven ordered failure/cleanup boundaries and successful
conversion. Initial CPU callback and ten feature queries, archive symbols,
and consumer linkage passed. The generated AVIF translation checker verified
all seven SharpYUV units with unchanged source hashes. WebP's focused
conversion, matrix, gamma, CPU, portable DSP, and SIMD tests passed; scalar
conversion passed with `--no-default-features`. Stable Clippy with warnings
denied passed for the shared core and WebP port.

A separate single-test process also releases eight threads together into the
AVIF and WebP public boundaries without a sequential warmup. Repeated calls
preserve input buffers and match each other, including output padding, and
the pinned C hash. This passes locally in `checked` and `release`; native
Linux and macOS PR jobs run it in both profiles. It is a concurrency
regression check, not a thread-sanitizer result.

The original full WebP consumer corpus and GNU Linux x86-64 native run remain
CI/adoption gates. This local worktree lacks the pinned FuzzTest checkout and
WebP test-data submodule, so the bounded local checks do not claim those gates.

The follow-up [full native run](https://github.com/sebastian-software/ferrisma/actions/runs/42994552674)
builds the unchanged pinned C and Rust-linked original consumers on GNU Linux
x86-64 and macOS ARM64. Acceptance requires both jobs to pass, including the
complete ordered test-case outcomes and skips from their CTest logs. The
final PR records the run's completed result separately from these local checks.

## Reproduce the numeric and ABI checks

From the repository root after initializing the documented codec references:

```sh
cargo fmt --all --check
cargo test -p libavif-sharpyuv --test cross_port --profile checked --locked
cargo test -p libavif-sharpyuv --test cross_port --release --locked
cargo test -p libavif-sharpyuv --test cross_port_cold_threads --profile checked --locked
cargo test -p libavif-sharpyuv --test cross_port_cold_threads --release --locked
cargo test -p libwebp-rs --test wp5_convert --test wp5_matrix --test wp5_gamma --test wp5_dsp --test wp5_cpu --test wp20_sharpyuv --locked
cargo test -p libwebp-rs --no-default-features --test wp5_convert --locked
python3 codecs/libavif-rs/tools/check_translation.py
```

From `codecs/libavif-rs`:

```sh
python3 tools/build_sharpyuv.py
python3 tools/build_sharpyuv.py --debug
cargo build -p libavif-sharpyuv --release --locked
cargo build -p libavif-sharpyuv --locked
python3 tools/check_sharpyuv.py
python3 tools/check_sharpyuv.py --debug
python3 tools/check_sharpyuv_abi.py
python3 tools/check_sharpyuv_abi.py --debug
python3 tools/check_sharpyuv_alloc.py
python3 tools/check_sharpyuv_cpu.py
python3 tools/check_sharpyuv_linkage.py
```

## Matched conversion workload

The original PR #176 `conversion_baseline` harness runs one boundary per fresh
release-profile process, with a 512 × 512 interleaved RGB8 source, WebP matrix,
sRGB transfer, YUV420P8 output, three warmups, and 30 timed conversions. A
second, identical source file in both trees, `conversion_depth_layout.rs`
(SHA-256 `d6449070804f01743ae52b1a92a72b599febe6bdf85899f03ef2ca35c7a17627`),
uses 256 × 256 RGB16/YUV420P10 with negative row stride and pixel step. Its
input and distinct output planes have guard rows for the C implementation's
formed but unused pointer advances. Both benchmarks use the same WebP matrix,
sRGB transfer, warmups, and timed call count.

The *before* binary was built from the detached PR #176 commit
`6ac2c331acd75765a0d0621ede9c57731ceedecb` into a separate target
directory. The *after* binary was built from this extraction tree. All runs
were made on the same Apple M1 Ultra host in one CPU-idle window, alternating
old/new and WebP/AVIF order. The 512-pixel benchmark checksum was `51556684`
and the signed-layout checksum was `166660870` in every run.

| Workload and boundary | Before, ms/call (three runs) | After, ms/call (three runs) |
| --- | --- | --- |
| 512² RGB8, AVIF C ABI | 11.065, 10.993, 11.073 | 13.003, 13.247, 12.946 |
| 512² RGB8, WebP safe API | 16.229, 16.019, 16.189 | 14.073, 14.008, 14.235 |
| 256² signed RGB16, AVIF C ABI | 4.591, 4.588, 4.569 | 5.396, 5.385, 5.294 |
| 256² signed RGB16, WebP safe API | 6.788, 6.824, 6.642 | 5.759, 5.704, 5.795 |

| Workload and boundary | Before peak process RSS, bytes | After peak process RSS, bytes |
| --- | --- | --- |
| 512² RGB8, AVIF C ABI | 10,731,520; 10,747,904; 10,747,904 | 7,028,736; 10,747,904; 8,896,512 |
| 512² RGB8, WebP safe API | 10,240,000; 10,780,672; 10,780,672 | 10,764,288; 8,912,896; 10,780,672 |
| 256² signed RGB16, AVIF C ABI | 4,227,072; 4,227,072; 4,702,208 | 4,734,976; 4,702,208; 4,718,592 |
| 256² signed RGB16, WebP safe API | 4,259,840; 4,423,680; 4,734,976 | 4,259,840; 4,734,976; 4,390,912 |

The AVIF boundary slowed by roughly 16–21% in these two workloads, while WebP
became roughly 11–16% faster. An earlier extracted-core trial without the
per-conversion `GammaContext` took 15.604–15.617 ms (AVIF) and
16.457–16.762 ms (WebP) in RGB8, and 6.338–6.411 ms (AVIF) and
6.712–6.790 ms (WebP) in the signed RGB16 workload. That trial supported the
specific hot-loop hypothesis: borrowing already initialized sRGB tables once
per conversion avoids repeated `LazyLock` dereferencing per sample. Its
numeric and C checks passed too. The retained context shares the same tables
and arithmetic as the original safe WebP functions. The remaining AVIF cost
is consistent with moving from the generated AVIF loop to the selected safe
WebP loop, but these aggregate timings do not isolate individual functions.
The adapter also initializes C-allocated scratch before constructing Rust
slices; these measurements do not isolate that cost. Peak RSS includes
runtime and allocator state and varies across fresh processes; it does not
measure the converter's seven scratch buffers alone. These two workloads are
an explicit adoption limit, not a general speed or memory prediction.

Reproduce the matched workload from the repository root:

```sh
cargo build -p libavif-sharpyuv --example conversion_baseline --release --locked
target/release/examples/conversion_baseline avif
target/release/examples/conversion_baseline webp
cargo build -p libavif-sharpyuv --example conversion_depth_layout --release --locked
target/release/examples/conversion_depth_layout avif
target/release/examples/conversion_depth_layout webp
```
