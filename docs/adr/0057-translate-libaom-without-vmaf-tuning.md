# Translate libaom without VMAF tuning

This supersedes [ADR 0035](0035-retain-libaom-vmaf-modes-during-the-codec-port.md)
and [ADR 0042](0042-preserve-the-original-vmaf-float-model-rejection.md).

## Context

ADR 0035 kept libaom's complete generic configuration, including
`CONFIG_TUNE_VMAF=1`, so that the port would not shrink the behavior boundary of
the native engine it replaces. That configuration links native libvmaf and the
C++ standard library into every consumer, and its two VMAF units
(`av1/encoder/tune_vmaf.c`, `aom_dsp/vmaf.c`) are about 9,700 translated lines
per profile, roughly 40,000 across the four committed profiles, that every
upstream update must regenerate and re-verify.

The port's consumer is Ferrixel, a still-image pipeline. Ferrixel's codec policy
([ADR-0025](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0025-codec-selection-reference-conformance-and-image-scope.md))
limits its ports to the still-image scope it uses, reduced through upstream's
own configuration switches before translation and never by editing generated
code. Ferrixel never selects a VMAF tuning mode, and its adoption rules
([ADR-0018](https://github.com/sebastian-software/ferrixel/blob/main/docs/adr/0018-optional-codec-and-cms-dependencies.md))
count the libvmaf requirement as an open gate.

No original libaom test depends on `CONFIG_TUNE_VMAF`: `test/test.cmake` has no
VMAF condition, and the `VmafPreprocessing` cases in `av1_convolve_test.cc`
exercise convolution kernels that are compiled in both configurations. Every
run-time use of the VMAF units is guarded by a tuning value of 4 to 7
(`AOM_TUNE_VMAF_*`); PSNR, SSIM, IQ and SSIMULACRA2 tuning do not read them.

## Decision

Build the C reference and the translation with upstream's
`-DCONFIG_TUNE_VMAF=0`. Everything else in ADR 0035's configuration stays:
generic target, encoder and decoder, high bit depth, multithreading, denoising,
quantization matrices, no real-time-only build. The archive closure becomes 166
translation units instead of 168.

Requests for a VMAF tuning mode take upstream's own path for this
configuration: `validate_config` in `av1/av1_cx_iface.c` rejects them with
`AOM_CODEC_INVALID_PARAM` and its "set -DCONFIG_TUNE_VMAF=1" detail, and
libavif reports `AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION`. `vmaf-model-path`
becomes an unknown option, as upstream builds it. This is upstream behavior,
not a stub or a fallback.

Remove the libvmaf bootstrap, link flags and checks (`tools/aom_vmaf.py`,
`tools/build_aom_vmaf.py`, `tools/check_aom_vmaf.py`,
`tests/ports/aom_vmaf_model.c`) and the C++ standard library link that only
libvmaf needed. ADR 0042 has no subject left.

Keep `CONFIG_AV1_DECODER=1`. Without the decoder, upstream builds neither
`test/decode_test_driver.cc`, which the encoder end-to-end tests use to compare
the encoder's reconstruction with a decode of its stream, nor about 30
encoder-relevant test sources such as `lossless_test.cc`,
`still_picture_test.cc`, `quant_test.cc`, `ethread_test.cc` and
`tile_independence_test.cc`. Those tests are the strongest evidence for the
encoder while its SIMD and later optimizations land, so the decoder stays until
a separate decision replaces that evidence.

## Consequences

Every translation unit is regenerated, so the change carries the acceptance of
[ADR 0048](0048-require-complete-release-and-focused-debug-aom-acceptance.md)
under the gating of [ADR 0055](0055-gate-pull-requests-on-linux-ci-and-local-macos-checks.md),
and the macOS Neon dispatch of
[ADR 0056](0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md)
must keep its byte identity. The original test inventories do not change; the
records of the configuration, the closure, the ABI and the allocation-failure
enumeration do (`av1_create_compressor` no longer allocates `vmaf_info`).
Encoder output for non-VMAF tuning is unchanged. The internal layout of
`AV1_COMP` and the string of `aom_codec_build_config()` change.

The port no longer claims to replace a libaom built with VMAF tuning. A consumer
that needs VMAF tuning needs a separate decision, ideally with a VMAF port
instead of a native dependency.
