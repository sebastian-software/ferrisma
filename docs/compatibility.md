# Compatibility and stability

Ferrisma has not made a 1.0 release. This page describes the compatibility
policy proposed in [ADR-0040](adr/0040-stable-public-contract-and-compatibility-policy.md);
it becomes binding when the maintainer accepts that ADR and publishes 1.0.0.
Until then, any 0.x minor release may break the API, as Cargo's SemVer rules
already assume for 0.x versions.

## What is stable

The stable contract is the documented `ferrisma::pipeline` facade, the
`ferrisma` command line, and the stable Cargo features below. Everything
else, including every private module, is outside the contract.

| Area | Stable | Experimental (may change in a minor release) |
| --- | --- | --- |
| Entry points | `transcode`, `transcode_resized`, `inspect`, `Engine::transcode_batch` | `transcode_with`, `transcode_resized_with`, `Engine::transcode_batch_with`, the `EncoderBackend`/`RowEncoder` extension traits, `cli::run` |
| Options | `TransformOptions` fields for format, geometry, crop, filter, color policy, quality, speed, chroma, threads, and the four resource limits; `ResizeOptions`; `InspectionLimits` | `kernel_level_ceiling`, `WorkingPrecision::F64Reference`, `AvifEncoder::Libavif`, `JpegXlDecoder::LibjxlRs`, `JpegXlOptions` |
| Results | `Output`, `ProcessingReport` field names, `ImageMetadata`, `ProcessingError::kind()` | `StageTimings` values, `encoder_backend` strings, `KernelLevel`/`KernelLevelReason` |
| Cargo features | `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `avif-write`, `icc`, `cli` | `jpeg-xl-own-read`, `jpeg-xl-write`, `avif-read`, `avif-own-write` |
| CLI | Positional arguments, color policy, quality, speed, chroma, WebP lossless/level, crop, thread, and limit flags; exit status; `report:` keys | `--avif-encoder`, `--avif-depth`, `--jxl-decoder`, `--reference`, `--kernel-level`; `.jxl` output with `--jxl-distance=` and `--jxl-lossless` |

`jpeg-xl-read` and `avif-write` promise the *capability* (JPEG XL input, AVIF
output), not a particular backend: ADR-0025 may switch the backend behind them
in a minor release when its evidence gates are met.

Since 2026-11-12 (confirmed by the owner on 2026-11-13, ADR-0041 D1), the default backend follows the
compiled features: the workspace libjxl-rs reader whenever `jpeg-xl-own-read`
is enabled, and the workspace libavif/AOM writer (10-bit) whenever
`avif-own-write` is enabled. `cli` enables both, so it runs these experimental
backends by default; `--jxl-decoder=jxl-oxide` and `--avif-encoder=rav1e`
select the others. AVIF `quality` and `speed` are interpreted on the selected
encoder's own scale and are not remapped. Select a backend explicitly when the
result must not depend on Cargo feature unification. See the
[supported matrix](supported-matrix.md).

## What a minor release may change

- Add enum variants. Every public enum that can grow is
  `#[non_exhaustive]`, including `ProcessingErrorKind`, so match with a
  wildcard arm.
- Add fields to `Output`, `ProcessingReport`, `StageTimings`, and
  `ImageMetadata`; these structs cannot be constructed outside Ferrisma.
- Change error message text. Match on `ProcessingError::kind()`, never on
  messages.
- Change the encoded bytes for the same options when an encoder improves.
  The documented meaning of the output (format, dimensions, orientation,
  color, alpha) does not change in a minor release.
- Change reported memory and timing observations.
- Raise the minimum Rust version for stable features, with a changelog note.
  Patch releases never raise it.
- Change the pinned nightly required by the experimental AVIF features.

## Toolchains

| Feature set | Toolchain | Targets |
| --- | --- | --- |
| No features, or any of `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `jpeg-xl-own-read`, `avif-write`, `icc` | Stable Rust 1.98 or newer (`rust-version`); NASM on x86-64 for `avif-write` | Any target these crates build for; CI covers macOS ARM64 and GNU Linux x86-64 |
| `avif-read`, `avif-own-write`, `cli` | The pinned nightly in `rust-toolchain.toml` (`nightly-2026-09-07` today) | Native macOS ARM64 and GNU Linux x86-64 only |

Builds that enable the AVIF port features as a dependency also need the
consumer profile settings listed in [publishing](publishing.md#blockers-before-the-first-functional-publication);
Cargo does not apply Ferrisma's workspace profiles to dependents.

## CLI units and exit status

- `--max-input-mb`, `--budget-mb`, and `--max-output-mb` count MiB
  (1,048,576 bytes); `--max-input-mpx` counts millions of pixels (10^6).
- Exit status 0 means success; 2 means too few arguments (usage is printed);
  1 means any other error, with `Error: …` on standard error.
- The `report:` line on standard output has stable keys with integer
  values; new keys may be added. The other output lines are for people and
  may change.

## Deprecation and upgrades

A stable item is deprecated for at least one minor release, with a note
naming its replacement, before a major release removes it. Each release's
changelog separates *Breaking*, *Added*, *Changed*, *Output changes*, and
*Toolchain* entries. Major releases add an upgrade section to this page.

## A complete consumer

[`examples/consumer.rs`](../examples/consumer.rs) shows the intended use of
the stable surface: bounded reads, header inspection, an explicit color
policy, explicit resource limits, and a disposition for every error kind.
Run its self-check with:

```sh
cargo run --example consumer --features jpeg-read,webp-read,webp-write -- --self-check
```
