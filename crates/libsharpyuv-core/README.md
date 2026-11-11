# libsharpyuv-core

Shared safe numeric conversion for SharpYUV 0.4.2 from libwebp 1.6.0, pinned
at `4fa21912338357f89e4fd51cf2368325b59e9bd9`. This crate contains the
WebP port's previously tested conversion, gamma, color-matrix, and portable
DSP code. It has no codec-wide C ABI, allocator, or CPU state. Its callers
supply sample access, seven initialized scratch buffers, and a DSP choice.

The WebP crate retains its slice API, fallible Rust allocation, and SIMD
selection. The AVIF crate retains its C ABI, independent CPU callback and
init state, and C allocation policy in an audited adapter. See
[ADR-0033](../../docs/adr/0033-shared-sharpyuv-conversion-core.md) and the
[extraction evidence](../../docs/research/2026-11-10-sharpyuv-extraction-evidence.md).

The files in `LICENSES/` are unchanged copies of upstream `COPYING`,
`PATENTS`, and `AUTHORS`. Source comments retain upstream copyright notices.
The original generated AVIF translation and pinned C source remain independent
reference material for numeric and ABI verification.
