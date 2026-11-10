# SharpYUV extraction: first boundary evidence

**Date:** 2026-11-10. **Base:** `65aeca3aa7e49efccf44094a93aae95e01530d69`.
This is evidence for [ADR-0026](../adr/0026-codecs-in-one-repository-and-workspace.md),
not an extraction or an adoption decision.

## Boundary inventory

The existing `cross_port` test compares 2,880 conversion combinations in the
same process. This package adds 16 signed-layout combinations (planar or
interleaved, forward or reverse rows, forward or reverse pixels, and 8- or
10-bit output), each called three times, plus ten defined early rejections.
The tests compare whole output allocations including padding, input
immutability, return values, and unchanged output on rejection. Sixteen-bit
input values exercise the existing truncation path; they do not redefine it.
The new cases use the public WebP safe function and AVIF C ABI function.
All 16 signed-layout outputs also match fixed FNV-64 hashes from the pinned
SharpYUV 0.4.2 C reference. The small C probe is retained at
[`extraction_boundaries.c`](../../codecs/libavif-rs/tests/sharpyuv/extraction_boundaries.c).

Existing independent evidence remains necessary: the AVIF C oracle's 1,271
API/corpus groups include callback reinitialization and concurrent cold
conversions; its allocation probe injects each of seven allocation failures
and checks cleanup. The WebP oracle includes 64 flipped groups and its own
concurrent calls. This package does not duplicate those mechanisms or alter
their pinned sources.

## Reproduce the local checks

From the repository root:

```sh
cargo fmt --all --check
cargo test -p libavif-sharpyuv --test cross_port --profile checked --locked
cargo test -p libavif-sharpyuv --test cross_port --release --locked
python3 codecs/libavif-rs/tools/build_sharpyuv.py
cc -std=c11 -O2 -I codecs/libavif-rs/upstream/libwebp \
  codecs/libavif-rs/tests/sharpyuv/extraction_boundaries.c \
  -L codecs/libavif-rs/build/sharpyuv-reference -lsharpyuv \
  -Wl,-rpath,"$PWD/codecs/libavif-rs/build/sharpyuv-reference" \
  -o codecs/libavif-rs/build/sharpyuv-extraction-c
codecs/libavif-rs/build/sharpyuv-extraction-c
cargo build -p libavif-sharpyuv --example conversion_baseline --release --locked
target/release/examples/conversion_baseline avif
target/release/examples/conversion_baseline webp
```

The `cross_port` test passed with three tests in both profiles on macOS ARM64.
The local C build verified its upstream libwebp source at
`4fa21912338357f89e4fd51cf2368325b59e9bd9` and recorded the required
`-ffp-contract=off` acceptance reference. Its 16 printed hashes match the
constants asserted by the Rust test. The reference and test also pass on this
host's debug Rust profile; this is not native Linux evidence.
For the baseline, run each port in a fresh process, alternate order, and
repeat. Both paths use one 512 × 512 interleaved RGB8 source, WebP matrix,
sRGB transfer, YUV420P8 output, three warmups, and 30 timed conversions.
The example reports elapsed time for those 30 calls, a checksum, and process
peak RSS from `getrusage`. The peak includes Rust runtime and allocator state;
it is not an isolated conversion allocation count or a memory budget.

Local baseline: Apple M1 Ultra, macOS 27.0.1 ARM64, Rust 1.100.0-nightly
(`5a2be9f5f`, LLVM 23.1.1), release profile. Each process reported checksum
`51556684`:

| Boundary | Mean ms per call, three runs | Peak process RSS, three runs |
| --- | --- | --- |
| AVIF C ABI | 10.492, 10.388, 10.283 | 7,045,120; 7,028,736; 7,045,120 bytes |
| WebP safe API | 15.232, 15.459, 15.074 | 5,210,112; 8,912,896; 7,061,504 bytes |

These are pre-extraction observations of two different APIs and memory
management paths, not a speed or memory prediction for a shared core. Repeat
the same workload after extraction on the same hosts, and add representative
depths, layouts, and dimensions before selecting the implementation.

## Remaining adoption gates

- Run the existing independent C API, ABI, allocation, CPU callback, linkage,
  and original consumer checks on native macOS ARM64 and GNU Linux x86-64.
  A cross-compiled binary does not establish native Linux behavior.
- Run the new pinned-C signed-layout probe on native GNU Linux x86-64 and
  require the same 16 hashes. Defined rejection behavior already has the
  independent AVIF C oracle's separate corpus, but the new cross-port cases
  do not include null options because AVIF dereferences options before its
  argument checks; retain only cases with documented C behavior.
- After a small core is extracted, keep the AVIF ABI and independent CPU
  callback/initialization and allocation behavior, rerun the complete numeric
  suites, and record the same runtime and peak-RSS workload before and after.
- Keep the AVIF production package independent of the entire WebP codec. The
  WebP dependency used by `cross_port` and this example is development-only.
