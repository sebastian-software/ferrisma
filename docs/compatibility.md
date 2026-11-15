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
| Options | `TransformOptions` fields for format, geometry, crop, filter, color policy, quality, speed, chroma, threads, and the four resource limits; `ResizeOptions`; `InspectionLimits` | `kernel_level_ceiling`, `WorkingPrecision::F64Reference`, `avif_depth`, `JpegXlOptions` |
| Results | `Output`, `ProcessingReport` field names, `ImageMetadata`, `ProcessingError::kind()` | `StageTimings` values, `decoder_backend` and `encoder_backend` strings, `KernelLevel`/`KernelLevelReason`, `ColorSignal`, `RefusalReason` and `ProcessingError::refusal_reason()`, `ProcessingReport::discarded` with `Discarded` and `DiscardedSet` (ADR-0042 D2) |
| Cargo features | `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `icc`, `cli` | `jpeg-xl-write`, `avif-read`, `avif-write` (experimental until it builds on stable Rust; stable as the rav1e capability until 2026-11-15) |
| CLI | Positional arguments, color policy, quality, speed, chroma, WebP lossless/level, crop, thread, and limit flags; exit status; `report:` keys | `--avif-depth`, `--reference`, `--kernel-level`; `.jxl` output with `--jxl-distance=` and `--jxl-lossless` |

`jpeg-xl-read` promises the *capability* (JPEG XL input), not a particular
backend: ADR-0025 may switch the backend behind it in a minor release when its
evidence gates are met.

Since 2026-11-15 `jpeg-xl-read` is the workspace libjxl-rs reader, the only
JPEG XL reader (owner decision; [ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)).
The feature of that name used to select jxl-oxide, and the workspace reader
was the experimental `jpeg-xl-own-read`, which is gone; `JpegXlDecoder`,
`TransformOptions::jxl_decoder` and `--jxl-decoder=` are removed (pre-1.0
breaking change). Likewise since 2026-11-15 `avif-write` is the workspace
libavif/AOM writer, the only AVIF writer (owner decision;
[ADR-0037](adr/0037-selectable-workspace-avif-encoder.md)): the name used to
select rav1e, and the workspace writer was `avif-own-write`, which is gone;
`AvifEncoder`, `TransformOptions::avif_encoder` and `--avif-encoder=` are
removed, and the depth is `TransformOptions::avif_depth` (`--avif-depth=`).
The known cost: AVIF output now needs the pinned nightly on macOS ARM64 or
GNU Linux x86-64, as AVIF input already did; a stable-toolchain build can
neither read nor write AVIF until the workspace AVIF port builds on stable
Rust. _Working decision 2026-11-15 (delegated, revisable by the owner):_
`avif-write` therefore moves to the experimental tier until then. AVIF
`quality` and `speed` are libavif's scales and are not remapped. See the
[supported matrix](supported-matrix.md).

## Refusal reasons

`ProcessingError::refusal_reason()` names the class of a refusal of
well-formed but unsupported input, so a caller can decide without parsing the
message (issue #212; experimental, working decisions 2026-11-14 and
2026-11-15, delegated, revisable by the owner). Every other error, including
malformed data, limits and invalid options, returns `None`. Where an input
meets several classes, the reason names the check that refused it first.

| `RefusalReason` | Kind | Refused input | What a caller can do |
| --- | --- | --- | --- |
| `Animation` | `UnsupportedInput` | animated WebP, JPEG XL, AVIF sequences; no first-frame fallback | extract one frame first |
| `ColorSignalMissing` | `UnsupportedColor` | neither ICC nor CICP under `Strict` | `AssumeSrgbIfMissing` if sRGB is known |
| `RenderedIccRequiresOptIn` | `UnsupportedColor` | ICC-described output under `Strict`/`AssumeSrgbIfMissing` (ICC-tagged JPEG, WebP, JPEG XL and AVIF; until 2026-11-15 every JPEG XL input, whose enum encoding is now a typed signal, #222) | `UseRenderedIcc` (`icc` feature) |
| `RenderedIccMissing` | `UnsupportedColor` | `UseRenderedIcc`, but no ICC: untagged input, or a typed signal (AVIF `nclx`, JPEG XL enum) outside sRGB, Display P3 and linear sRGB; those three pass this policy since 2026-11-15 | another policy |
| `CicpBesideIcc` | `UnsupportedColor` | under `UseRenderedIcc`, a `cicp` tag inside the profile (or a decoder's CICP beside it) is HDR, uncheckable or disagrees; AVIF `nclx` beside a profile no longer is (MIAF, since 2026-11-15) | resolve the conflict upstream |
| `HdrTransfer` | `UnsupportedColor` | PQ or HLG (JPEG XL, AVIF), under every policy | tone-map to SDR elsewhere |
| `UnsupportedColorEncoding` | `UnsupportedColor` | SDR CICP (AVIF `nclx`, and since 2026-11-15 a JPEG XL enum encoding) outside sRGB, Display P3, linear sRGB without ICC (for example BT.2020 SDR, DCI-P3). (Until 2026-11-15 also a JPEG XL colour encoding jxl-oxide could not render; the workspace reader, like libjxl, rejects that file as `MalformedInput`.) | convert elsewhere |
| `ContradictoryColorSignals` | `UnsupportedColor` | AVIF container `nclx`/ICC contradicting the AV1 sequence header | repair the file |
| `UnsupportedLayout` | `UnsupportedInput` | CMYK/YCCK JPEG, constant-luminance BT.2020, CMYK/float/extra-channel JPEG XL (grayscale JPEG XL and monochrome AVIF are accepted since 2026-11-15) | convert to RGB(A) first |
| `AlphaAssociation` | `UnsupportedInput` | alpha of an association the reader did not convert; no built-in reader produces it since 2026-11-15 (premultiplied JPEG XL and AVIF alpha are converted and reported as `Discarded::Unpremultiplied`); kept for a future reader | convert to straight alpha first |
| `GeometricTransform` | `UnsupportedInput` | AVIF `clap`, `imir` before `irot`, EXIF orientation outside 1 to 8 | remove the transform first |
| `IccColorSpaceMismatch` | `UnsupportedColor` | under `UseRenderedIcc`, an RGB profile beside gray samples or a gray profile beside RGB samples (since 2026-11-15) | repair the profile |
| `UnsupportedEssentialProperty` | `UnsupportedInput` | an AVIF item the reader uses (primary, grid cells, alpha) cites a property marked essential that the engine does not understand (alpha and cells since 2026-11-15, ADR-0042 D1) (since 2026-11-15; before, `MalformedInput` without a reason) | remove or convert that property first |

New reasons may be added in a minor release (match with a wildcard arm); a
documented reason keeps its meaning.

## Discarded information

`ProcessingReport::discarded` names each class of input information a
successful transcode left out of a correct picture ([ADR-0042](adr/0042-no-silent-loss-refuse-or-report.md)
D2; experimental, working decision 2026-11-15, delegated, revisable by the
owner). A caller that must not lose anything can reject a result whose set
is not empty; the classes and when each applies are in the
[supported matrix](supported-matrix.md#discarded-report). New classes may
be added in a minor release; a class keeps its meaning. The CLI prints the
set as the `discarded=` key of its `report:` line (`none` when empty).

## Options without effect

An option that would have no effect is refused, not ignored, when it is
set to a value other than its default (owner decision of 2026-11-15,
ADR-0042): `output_width`/`output_height` with `transcode_resized`, a
thread count above 256 in `Engine::transcode_batch`, a setting of another
output format, and a setting an external backend's request does not carry.
Each is `InvalidOptions` before decoding. Where the engine replaces a value
and reports the one it used, the replacement is documented instead: the
batch runs each image with one transform and one encoder thread, and the
`F64Reference` executor runs scalar and single-threaded.

## What a minor release may change

- Add enum variants. Every public enum that can grow is
  `#[non_exhaustive]`, including `ProcessingErrorKind`, so match with a
  wildcard arm.
- Add fields to `Output`, `ProcessingReport`, `StageTimings`, and
  `ImageMetadata`; these structs cannot be constructed outside Ferrisma.
- Change error message text. Match on `ProcessingError::kind()` and
  `ProcessingError::refusal_reason()`, never on messages.
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
| No features, or any of `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `jpeg-xl-write`, `icc` | Stable Rust 1.98 or newer (`rust-version`) | Any target these crates build for; CI covers macOS ARM64 and GNU Linux x86-64 |
| `avif-read`, `avif-write`, `cli` | The pinned nightly in `rust-toolchain.toml` (`nightly-2026-09-07` today) | Native macOS ARM64 and GNU Linux x86-64 only |

Builds that enable the AVIF port features as a dependency also need the
consumer profile settings listed in [publishing](publishing.md#blockers-before-the-first-functional-publication);
Cargo does not apply Ferrisma's workspace profiles to dependents.

## CLI units and exit status

- `--max-input-mb`, `--budget-mb`, and `--max-output-mb` count MiB
  (1,048,576 bytes); `--max-input-mpx` counts millions of pixels (10^6).
- Exit status 0 means success; 2 means too few arguments (usage is printed);
  1 means any other error, with `Error: …` on standard error.
- Every flag may be given once, and contradictory flags are refused with
  both names (`--assume-srgb` with `--use-rendered-icc`, `--jxl-lossless`
  with `--jxl-distance=`, `--webp-lossless` or `--webp-level=` with
  lossy-only settings); neither takes precedence (owner decision of
  2026-11-15). Until then the last value of a repeated flag won silently.
- `--version` or `-V`, anywhere after the program name, prints four lines
  (package version, target, compiled engine features, build identity) and
  exits 0 without reading input (since 2026-11-13, #237). ADR-0040 does not
  yet assign the flag or its output to a stability tier.
- The `report:` line on standard output has stable keys with integer
  values, except the names (precision, kernel level, backends,
  `color_signal`, `discarded`); new keys may be added. The other output lines are for people and
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
