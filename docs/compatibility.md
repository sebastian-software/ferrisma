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
| Entry points | `transcode`, `inspect`, `Engine::transcode_batch` | `transcode_with`, `Engine::transcode_batch_with`, the `EncoderBackend`/`RowEncoder` extension traits, `cli::run`, `cli::main` |
| Options | `TransformOptions` (constructor, `with_*` methods and getters) for output, geometry (`OutputGeometry`), crop, filter, color policy, threads, and the four resource limits; `OutputSettings`; `WebpOptions` with `WebpLossy` and `WebpLossless` (libwebp quality and method, lossless level); `ResizeOptions`; `Crop`; `InspectionLimits`; `BatchItem` | `kernel_level_ceiling`, `WorkingPrecision::F64Reference`, `AvifOptions` (`OutputSettings::Avif`), `JpegXlOptions` with `JpegXlLossy` and `JpegXlLossless`, `BitDepth` |
| Results | `Output`, `ProcessingReport` field names, `ImageMetadata`, `ProcessingError::kind()` | `StageTimings` values, `decoder_backend` and `encoder_backend` strings, `KernelLevel`/`KernelLevelReason` (Ferrisma's own enums since 2026-11-16), `ColorSignal`, `RefusalReason` and `ProcessingError::refusal_reason()`, `ProcessingReport::discarded` with `Discarded` and `DiscardedSet` (ADR-0042 D2) |
| Cargo features | `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `icc`, `cli` | `jpeg-xl-write`, `avif-read`, `avif-write` (experimental since the switch to the workspace port; it builds on stable Rust 1.99 since 2026-11-15, and the tier is not decided; stable as the rav1e capability until 2026-11-15) |
| CLI | Positional arguments, color policy, `--webp-quality=`, `--webp-method=`, WebP lossless/level, crop, thread, and limit flags; exit status; `report:` keys | `--avif-quality=`, `--avif-alpha-quality=`, `--avif-speed=`, `--avif-depth=`, `--avif-chroma=`, `--reference`, `--kernel-level`; `.jxl` output with `--jxl-distance=`, `--jxl-lossless` and `--jxl-depth=` |

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
removed, and the depth was `TransformOptions::avif_depth` (`--avif-depth=`)
until the native options below.
AVIF input and output build only on native macOS ARM64 and GNU Linux
x86-64. When the switch was made, they also needed the pinned nightly; since
the same day they build on stable Rust 1.99 (libavif-rs ADR 0060, the
toolchain table below). _Working decision 2026-11-15 (delegated, revisable
by the owner):_ `avif-write` moved to the experimental tier with the switch;
its tier, and that of `avif-read`, are not decided now that the nightly
reason is gone. See the [supported matrix](supported-matrix.md).

Also since 2026-11-15 encoder settings are native per format (owner
decision; [ADR-0043](adr/0043-formats-behave-alike.md) D4, pre-1.0 breaking
change): `TransformOptions::quality`, `speed`, `chroma_subsampling` and
`avif_depth` are replaced by `TransformOptions::avif: AvifOptions` (libavif
quality, alpha quality, speed, depth and chroma at `avifenc`'s defaults 60,
60, 6, 8 bits, 4:4:4), and `WebpOptions::Lossy { speed }` by `{ method }`,
libwebp's own 0–6 (default 4). The CLI's `--quality=`, `--speed=` and
`--chroma=` are replaced by `--avif-quality=`, `--avif-alpha-quality=`,
`--avif-speed=`, `--avif-chroma=`, `--webp-quality=` and `--webp-method=`.
Default AVIF and lossy WebP output changes with the new defaults.

Since 2026-11-15 the output depth follows the input's (owner decision;
[ADR-0021](adr/0021-working-precision-policy.md) amendment, pre-1.0
breaking change): `AvifOptions::depth` is `Option<u8>` (`None`, the
default, follows the input: 8, 10 or 12; `Some(8 | 10 | 12)` explicit), and
both `JpegXlOptions` variants gain `depth: Option<u8>` (8 to 16;
`JpegXlOptions::depth()` and `with_depth()` read and set it). An 8-bit
input writes the same bytes as before. A 10- or 12-bit AVIF and a deeper
JPEG XL input now produce deep output instead of 8-bit content, and no
`PrecisionReduced`; lossless JPEG XL above 12 bits, or with alpha above 12
bits, is an ISOBMFF container, as `cjxl` writes it. `ProcessingReport` gains
`input_depth` and `output_depth` and the CLI `report:` line the keys
`input_depth=` and `depth=` (before `discarded=`, which stays last). CLI:
`--avif-depth=12` and `--jxl-depth=8..16` are accepted. The external-backend
API (experimental) changes additively: `EncodeRequest::depth`,
`EncoderBackend::row_depths` (default `[8]`, so an existing backend keeps
receiving 8-bit rows and a deeper input is reported as `PrecisionReduced`)
and `RowEncoder::write_row_u16` (default: refused).

## Constructing options

Since 2026-11-16 every option type is built one way (ADR-0040 amendment of
that date): a constructor or `Default`, then one `with_*` method per
setting, each returning the changed value; getters of the same names read
them. The fields are private and the types `#[non_exhaustive]`, so a new
setting is a new method and breaks no caller. Struct literals, including
`..TransformOptions::default()`, no longer compile outside the crate.

```rust
use ferrisma::pipeline::{
    InputProfilePolicy, OutputGeometry, OutputSettings, ResizeOptions, TransformOptions,
    WebpLossy, WebpOptions,
};

let options = TransformOptions::new(OutputGeometry::Resize(
    ResizeOptions::default().with_width(320).with_height(320),
))
.with_output(OutputSettings::WebP(WebpOptions::Lossy(
    WebpLossy::default().with_quality(80.0),
)))
.with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
```

What changed with it (pre-1.0 breaking change): `TransformOptions` has no
`Default` (its constructor takes the geometry) and is no longer `Copy`;
`output_format` and the three per-format fields `avif`, `webp` and
`jpeg_xl` are one `OutputSettings` value, `Avif(AvifOptions)`,
`WebP(WebpOptions)` or `JpegXl(JpegXlOptions)`, which selects the output,
so the settings of another format can no longer be set (they were refused
before); `output_width`/`output_height`, whose default of 1 stood for "not
set", are `OutputGeometry::Exact { width, height }`, and
`transcode_resized`/`transcode_resized_with` are `transcode`/`transcode_with`
with `OutputGeometry::Resize(ResizeOptions)`. `WebpOptions` and
`JpegXlOptions` keep their lossy and lossless variants, each now carrying a
settings struct (`WebpLossy`, `WebpLossless`, `JpegXlLossy`,
`JpegXlLossless`). The depth settings are a `BitDepth` (`FollowInput`, the
default, or `Exact(n)`) instead of `Option<u8>`. `Crop` is built with
`Crop::new(x, y, width, height)` and read with `x()`, `y()`, `width()`,
`height()`; `BatchItem` with `BatchItem::new(encoded, format, options)`;
`LibavifBackend` with `LibavifBackend::new()`. The external-backend request,
`EncodeRequest`, carries `output: OutputSettings` instead of `format`,
`avif`, `webp` and `jpeg_xl` (`EncodeRequest::format()` remains).

Thread counts are a `Threads` value (since 2026-11-16): `Threads::Automatic`
or `Threads::Count(NonZeroUsize)`, `Threads::ONE` for the sequential path,
instead of a `usize` with 0 for "automatic". `Engine::new(Threads)` returns a
`Result` and refuses a count above 256 as `InvalidOptions`, the validation
`transcode` applies; before, `Engine::new(usize)` clamped such a count to
256 silently. The command line keeps its spelling (`--transform-threads=0`
is automatic).

The experimental backend API: `RowEncoder` requires `Send` (since
2026-11-16; an encoder that keeps an `Rc` or a raw pointer must wrap it),
and `EncodeRequest::new(output, width, height)` builds a request for a
backend's own tests ([ADR-0024](adr/0024-pluggable-encoders-and-libavif-rs-backend.md)).

`ImageMetadata::rendered_cicp` is an `Option<Cicp>` (since 2026-11-16)
with the named fields `color_primaries`, `transfer_characteristics`,
`matrix_coefficients` and `full_range: bool`, instead of an `[u8; 4]`.

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
ADR-0042): a thread count above 256 in `Engine::transcode_batch` and a
setting an external backend's request does not carry. (Until 2026-11-16 also
`output_width`/`output_height` with `transcode_resized` and a setting of
another output format; the options can no longer express either.)
Each is `InvalidOptions` before decoding. Where the engine replaces a value
and reports the one it used, the replacement is documented instead: the
batch runs each image with one transform and one encoder thread, and the
`F64Reference` executor runs scalar and single-threaded.

## Panics

A panic inside Ferrisma, one of its codecs or an encoder backend does not
unwind out of the public functions (since 2026-11-16; ADR-0040 amendment of
that date, working decision, delegated, revisable by the owner):
`transcode`, `inspect`, the `_with` variants and each
item of `Engine::transcode_batch` and `transcode_batch_with` catch it and
return `ProcessingErrorKind::Internal`, whose message says that a panic was
caught and includes the panic's message when it is text. In a batch, only
the panicking item fails; the other items keep their results. A later call
starts from a fresh memory budget, so nothing of the failed call stays
reserved.

What this does not promise:

- A panic is still a defect. Please report it; the `Internal` kind is not a
  class of input to handle like a refusal, and Rust's panic hook still
  prints its message to standard error.
- With `panic = "abort"` nothing unwinds, so the process aborts. The AVIF
  features (`avif-read`, `avif-write`, and `cli`, which enables them) do not
  compile with `panic = "abort"` at all ([publishing](publishing.md)).
- A panic that would have to cross a C-ABI frame inside a codec port aborts
  the process instead of reaching the boundary.
- The state of a caller-supplied `EncoderBackend` after its own panic is the
  backend's concern; in a batch the same backend serves the other items.

## What a minor release may change

- Add enum variants. Every public enum that can grow is
  `#[non_exhaustive]`, including `ProcessingErrorKind`, so match with a
  wildcard arm.
- Add fields to `Output`, `ProcessingReport`, `StageTimings`,
  `DecoderWork` and `ImageMetadata`; these structs cannot be constructed
  outside Ferrisma. Since 2026-11-16 the report structs, the option types,
  `EncodeRequest` and `Engine` are `Clone` but not `Copy`, so a later field
  need not be `Copy`; read a report through a reference or clone it.
- Add settings to the option types: each is a new `with_*` method and
  getter (see [constructing options](#constructing-options)).
- Change error message text. Match on `ProcessingError::kind()` and
  `ProcessingError::refusal_reason()`, never on messages.
- Change the encoded bytes for the same options when an encoder improves.
  The documented meaning of the output (format, dimensions, orientation,
  color, alpha) does not change in a minor release.
- Change reported memory and timing observations.
- Raise the minimum Rust version, deliberately and in its own change with a
  changelog note, to a release that is then at least three months old.
  Patch releases never raise it.
- Change the pinned development nightly (no feature requires it since
  2026-11-15).

## Toolchains

| Feature set | Toolchain | Targets |
| --- | --- | --- |
| No features, or any of `jpeg-read`, `webp-read`, `webp-write`, `jpeg-xl-read`, `jpeg-xl-write`, `icc` | Stable Rust 1.99 or newer (`rust-version`) | Any target these crates build for; CI covers macOS ARM64 and GNU Linux x86-64 |
| `avif-read`, `avif-write`, `cli` | Stable Rust 1.99 or newer (`rust-version`; the pinned nightly before 2026-11-15) | Native macOS ARM64 and GNU Linux x86-64 only |

The minimum is one number for the whole engine (owner decision 2026-11-15,
[ADR-0040](adr/0040-stable-public-contract-and-compatibility-policy.md)).
The codec ports used on their own declare lower floors
([minimum Rust versions](supported-matrix.md#minimum-rust-versions)). The
pinned nightly in `rust-toolchain.toml` is a development tool, not a
requirement.

Builds that enable the AVIF port features as a dependency also need the
consumer profile settings listed in [publishing](publishing.md#blockers-before-the-first-functional-publication);
Cargo does not apply Ferrisma's workspace profiles to dependents.

## CLI units and exit status

- The default managed-memory budget (`max_managed_bytes`, `--budget-mb`) is
  2 GiB since 2026-11-16 (owner decision, ADR-0004 amendment; 1 GiB from
  2026-11-15, 256 MiB before): a transcode that failed with
  `ResourceExhausted` at an old default may now succeed; an operator who
  wants a former bound sets it (`--budget-mb=1024`). Under the default the
  largest JPEG XL output is now about 125.8 MP lossless opaque, 97.8 MP with
  alpha, 69.0 and 45.5 MP above 8 bits, and lossy 159.8, 123.2, 89.4 and
  54.2 MP (the [supported matrix](supported-matrix.md) has the exact
  numbers; every one extrapolates the memory model beyond its measured
  24 MP).
- Since 2026-11-16 JPEG XL output reserves its measured peak (ADR-0004 and
  ADR-0039 amendments): under the 1 GiB default of that morning, lossy JPEG
  XL above about 40.5 MP opaque, 25.5 MP with alpha, 22.3 and 11.2 MP above
  8 bits, and lossless with alpha above about 30.7 MP (13.9 MP above 8
  bits; 24.3 MP opaque above 8 bits) failed with `ResourceExhausted` where
  it passed before; the 2 GiB default of the same day admits the sizes of
  the item above, and a larger `--budget-mb` admits more.
- `--max-input-mb`, `--budget-mb`, and `--max-output-mb` count MiB
  (1,048,576 bytes); `--max-input-mpx` counts millions of pixels (10^6).
- Exit status 0 means success, 2 a usage error, 1 a processing error
  (since 2026-11-16, ADR-0040 amendment of that date; before, 2 meant only
  too few arguments and every other error was 1). Usage errors: missing
  positional arguments (the usage follows the error line), an unknown,
  repeated or contradictory flag, an unsupported input or output file
  extension, a value that does not parse, and a value the engine refuses as
  `InvalidOptions` (a setting outside its scale, a zero size or limit, a
  crop outside the image). Processing errors: unreadable input, unwritable
  output, a refused, malformed or too large input, an output above
  `--max-output-mb` (`OutputLimitExceeded` since 2026-11-16,
  `ResourceExhausted` before), an exhausted resource, a missing feature, an
  encoder failure, and `Internal`.
- An error is one line `error: <message>` on standard error (the message
  text is not stable). When the engine returned it, a second line names
  its kind and refusal reason in the form of the `report:` line:
  `failure: kind=<kind> reason=<reason>`, for example
  `failure: kind=unsupported-color reason=color-signal-missing`; the
  reason is `none` outside the refusal classes. The values are lower-case
  and hyphenated forms of `ProcessingErrorKind` and `RefusalReason`
  (`invalid-options`, `input-limit-exceeded`, `feature-disabled`,
  `malformed-input`, `unsupported-input`, `unsupported-color`,
  `resource-exhausted`, `output-limit-exceeded` (since 2026-11-16),
  `cancelled`, `encode-failed`, `internal`), written
  by the command itself, so renaming a Rust variant does not change them.
  Until 2026-11-16 Rust printed `Error: ` and the error's `Debug` form.
- `--help` or `-h` prints the usage on standard output and exits 0 (since
  2026-11-16; before, usage went to standard error with exit status 2 and
  `--help` was an unknown option). `--version` takes precedence over it.
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
  `color_signal`, `discarded`); new keys may be added. Since 2026-11-16 every
  name taken from an enum is the command's own lower-case, hyphenated
  value, so renaming a Rust variant cannot change it: `precision=` `f32` or
  `f64-reference`; `kernel_level=` `scalar`, `neon`, `avx2`,
  `avx512`, or `none` when the 1:1 sRGB identity pass-through bypassed the
  resample kernels (the level names match `--kernel-level=`; `sve2`, which no
  run reported, is no longer a value since 2026-11-16); `color_signal=` `cicp`,
  `assumed-srgb`, `rendered-icc`, `rendered-icc-with-agreeing-cicp`;
  `discarded=` a comma-separated list of `gain-map`, `exif`, `xmp`,
  `other-metadata`, `icc-profile`, `unknown-container-data`,
  `jpeg-reconstruction-data`, `preview`, `hidden-color`, `unpremultiplied`,
  `precision-reduced`, or `none`. Before, `precision=`, `color_signal=` and
  `discarded=` printed the Rust variant names (`F32`, `AssumedSrgb`,
  `IccProfile`) and `kernel_level=` the kernels crate's names. The
  `encoder=` and `decoder=` values are the report's backend strings, the
  names of the ported originals: `libjpeg-turbo`, `libwebp`, `libjxl`,
  `libavif/aom`, for AVIF input `libavif/dav1d` (the default decoder since
  2026-11-22) or `libavif/aom` (the selected reference), or an external
  backend's own `name()` (since 2026-11-16,
  owner decision; before, the package names `libjpeg-turbo-rs`,
  `libwebp-rs`, `libjxl-rs` and `libavif-rs/aom`), so renaming a package
  does not change them; experimental values. The `decoder_work:` line
  prints rectangles as `WxH+X+Y` and counts as a number or `>=` and a lower
  bound. The other output lines are for people and may change.

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
