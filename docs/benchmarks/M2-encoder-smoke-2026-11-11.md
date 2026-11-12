# AVIF encoder adoption smoke — 2026-11-11

**Scope:** one photograph, one anchor quality, one x86-64 host. This is a
completed procedure check and a dated measurement, not final backend selection
or evidence of general Sharp parity. Rav1e remains the default.

## Source and workload

[Run 43075697017](https://github.com/sebastian-software/ferrisma/actions/runs/43075697017)
measured revision `efe799b948022ca9ba0ce4bfd71ebdbe57a21704` using the
[encoder comparison procedure](encoder-adoption.md). Both measured CLIs use
Ferrisma's decode/transform path. The libavif compatibility CLI uses the shared
workspace AOM writer; the normal facade selector was not timed separately.
This snapshot includes the writer extraction and resource admission changes,
and precedes the later worker-startup hardening.

The input is the attributed `dog` photograph in the [corpus](corpus-and-comparison.md):
4032×3024 JPEG, EXIF orientation 6, sRGB ICC. The output is 1600×2133,
10-bit 4:4:4 AVIF after the default linear-light Lanczos3/f32 transform and
explicit rendered-ICC policy. Sharp 0.34.5/libvips 8.17.3 supply independent
reference preparation; Sharp is not a timed encoder in this run. The corpus
guide's JPEG-decoder, ICC, RGB-only scoring and reference limitations apply.

The runner exposed four logical/affinity CPUs, identified as AMD EPYC 9V74
80-Core Processor. This does not mean 80 cores were available to the job.
The workflow uses Ubuntu 24.04; recorded platform is
`Linux-7.0.0-1017-azure-x86_64-with-glibc2.39`. The root toolchain is
`nightly-2026-09-07`; rustc reports `1.100.0-nightly (5a2be9f5f 2026-09-06)`
with LLVM 23.1.1. No RUSTFLAGS override was recorded, and the tracked source
patch was empty.

## Quality and normal AOM dispatch

Rav1e speed 6/quality 80 defines separate targets for four and one resolved
threads. AOM speeds 6 and 7 use independently matched integer qualities.
Every measured output met the 0.5 SSIMULACRA2 target tolerance and 0.1 replay
tolerance. The largest target difference was 0.456; the candidates are close
under that rule, not identical in quality.

Process wall time includes startup and I/O; RSS belongs to that same process.
Each row has one warmup and five measured samples. Nearest-rank p95 is the
maximum of five samples, not an established tail-latency distribution.

| Threads | Encoder | Quality | SSIMULACRA2 | Actual bytes | Median ms | p95 ms | Max RSS MiB |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 4 | Rav1e s6 | 80 | 73.467 | 261,883 | 2,623.7 | 2,634.3 | 142.3 |
| 4 | AOM s6 | 55 | 73.076 | 228,547 | 1,466.2 | 1,473.7 | 144.3 |
| 4 | AOM s7 | 56 | 73.890 | 252,582 | 1,040.5 | 1,041.9 | 143.6 |
| 1 | Rav1e s6 | 80 | 73.518 | 252,291 | 5,536.2 | 5,542.7 | 143.8 |
| 1 | AOM s6 | 55 | 73.062 | 227,152 | 3,182.8 | 3,213.7 | 138.0 |
| 1 | AOM s7 | 56 | 73.833 | 249,732 | 2,128.1 | 2,190.1 | 137.5 |

AOM s6 had lower median wall time in this case, while its integer-quality
output scored below the corresponding Rav1e anchor. Bracket interpolation
estimates AOM bytes at the exact anchor as 233,127/247,549 for speeds 6/7 on
four threads, and 232,276/246,113 on one thread. These are estimates, not
sizes of actual equally scored files.

## AOM runtime SIMD mask zero

The second timing pass sets `AOM_SIMD_CAPS_MASK=0`. Ferrisma still reports
its AVX-512 transform kernel in both passes. This is not a scalar-engine
measurement and does not rule out compiler auto-vectorization. The six final
bitstreams match the corresponding first-pass hashes; scores and sizes above
therefore apply to both passes.

| Threads | Encoder | Median ms | p95 ms | Max RSS MiB |
| ---: | --- | ---: | ---: | ---: |
| 4 | Rav1e s6 | 2,626.6 | 2,641.9 | 142.1 |
| 4 | AOM s6 | 2,726.0 | 2,800.6 | 143.9 |
| 4 | AOM s7 | 1,468.4 | 1,475.6 | 143.4 |
| 1 | Rav1e s6 | 5,524.2 | 5,562.0 | 143.9 |
| 1 | AOM s6 | 6,140.3 | 6,143.3 | 137.8 |
| 1 | AOM s7 | 3,138.0 | 3,157.4 | 137.4 |

Retained CLI binary sizes were 18,271,848 bytes for `ferrisma` and 19,296,552
for `ferrisma-libavif`. These are complete linked CLI builds, not isolated
encoder-library sizes. The maximum sampled before/after load was about 3.27;
the load gate does not establish physical-host isolation.

## Evidence and remaining gates

The [artifact](https://github.com/sebastian-software/ferrisma/actions/runs/43075697017/artifacts/13881268261)
has ZIP SHA-256 `8218efc7042dbc16dc0302d58d45e6d398a4ce84516e20e5041bba75f791403e`.
It retains references, all matching outputs, 60 measured samples, 12 summaries,
final outputs, source/build receipts and completion checksums. Independent
audits recomputed the summaries and checked the declared output/reference
hashes, counts and score gates. All five samples within each configuration
had the same output hash. AVIF configuration boxes indicate 10-bit 4:4:4.
The original JPEG is reproducible from the pinned corpus fixture, but it and
the encoder/scorer binaries are not included in the artifact; their recorded
hashes alone do not make the archive a self-contained build reproduction.

The earlier [run 43073087629](https://github.com/sebastian-software/ferrisma/actions/runs/43073087629)
failed its quality gate at anchor quality 60: the nearest AOM integers were
1.379–1.619 SSIMULACRA2 away. Its partial timing is invalid. The procedure
now rejects such integer-quality gaps before warmup; the tolerance was not
relaxed to obtain this result.

[ADR-0025](../adr/0025-codec-selection-reference-conformance-and-image-scope.md)
still requires Arm and x86-64, Kodak plus multiple real 12–48 MP photographs,
several quality levels, alpha/depth/chroma coverage and broader resource and
maintenance evidence. This one-level RGB photograph does not close those
gates or change the default encoder.
