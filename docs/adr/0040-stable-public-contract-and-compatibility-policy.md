# ADR-0040: Stable 1.0 public contract, compatibility, versioning, and upgrade policy

- **Status:** Proposed
- **Date:** 2026-11-12
- **Last reviewed:** 2026-11-16
- **Supersedes:** None. Once accepted, it replaces decision 6 ("no stability promise") of [ADR-0017](0017-minimal-public-facade-and-cli.md) for the items it marks stable.
- **Superseded in part by:** [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (when the version 1.0 may be used, and the release tooling)
- **Related:** [0010](0010-dual-licensing.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0034](0034-thumbnail-geometry.md), [0037](0037-selectable-workspace-avif-encoder.md), [publishing](../publishing.md), [release automation](../release-automation.md), [compatibility guide](../compatibility.md), tracker #196

## Context

The v1.0 tracker (#196) asks for a reviewed public API and CLI contract,
feature stability boundaries, error semantics, a compatibility and
versioning policy, and upgrade guidance before stability is promised.
ADR-0017 records a deliberately small facade, but its decision 6 says any
part may change without deprecation. Nothing functional is published yet:
crates.io and npm hold only `ferrisma` 0.0.0 placeholders (ADR-0038), and the
workspace members are at 0.1.0 except `ferrisma-avif-read`,
`ferrisma-avif-write` and `ferrisma-libavif` (0.0.0), with `publish = false`
everywhere.

Verified facts on 2026-11-12 (branch on top of `90b638dd`):

- The library exposes two modules, `pipeline` and `cli`. Graph, planner,
  executor, codec adapters, color contract, and memory types are private.
  The engine crate forbids `unsafe`; codec types never appear in public
  signatures.
- `ferrisma::pipeline::{KernelLevel, KernelLevelReason}` are re-exports of
  `ferrisma_kernels::{IsaLevel, LevelReason}`, so `ferrisma-kernels` is a
  public dependency.
- `examples/consumer.rs` exercises the facade outside the test suite,
  including every documented failure category and resource limit; its
  `--self-check` passes (10 scenarios) and runs in CI.
- The `avif-read`, `avif-own-write`, and `cli` features require the pinned
  nightly and native macOS ARM64 or GNU Linux x86-64; everything else
  builds on stable Rust 1.98 (CI MSRV job). Since 2026-11-15 the AVIF features (`avif-read`,
  `avif-write`, `cli`) build on stable Rust 1.99 instead (libavif-rs ADR 0060),
  and since the owner's MSRV decision of the same day the engine declares
  1.99 for every feature (amendment log, 2026-11-15). _Correction
  2026-11-16:_ `avif-own-write` no longer exists (its writer is `avif-write`
  since 2026-11-15), and no feature needs the nightly; only the native
  targets remain.
- On this branch, every public facade item is documented
  (`#![warn(missing_docs)]`), and the growable enums and output-only structs
  are `#[non_exhaustive]` (see the inventory below).

## Public surface inventory

### Rust API (`ferrisma::pipeline`)

| Item | Kind | Proposed 1.0 tier | Notes |
| --- | --- | --- | --- |
| `transcode`, `transcode_resized` | fn | Stable | Exact and aspect-aware geometry (ADR-0034) |
| `inspect`, `InspectionLimits`, `ImageMetadata` | fn, struct, struct | Stable | JPEG and WebP only; `ImageMetadata` is non-exhaustive |
| `TransformOptions` | struct, public fields, `Copy`, `Default` | Stable, see problems 1–3 | Field docs state each field's scale and limit; since 2026-11-16 the rustdoc of `max_output_bytes` describes when the limit is checked as this release's behavior, not a promise (problem 16) |
| `Crop` | struct, public fields | Stable | Deliberately exhaustive plain rectangle |
| `ResizeOptions`, `ResizeFit` | struct, enum | Stable | `ResizeFit` non-exhaustive |
| `ResizeFilter` (`Area`, `Lanczos3`) | enum | Stable | Non-exhaustive |
| `InputProfilePolicy` | enum | Stable | Non-exhaustive; `Strict` default |
| `ImageFormat`, `OutputFormat` | enum | Stable | Non-exhaustive |
| `WebpOptions` (`Lossy { quality, method }`, `Lossless { level }`) | enum | Stable | Non-exhaustive; libwebp's own scales; `speed` replaced by `method` on 2026-11-15 |
| `WebpOptions::DEFAULT_METHOD`, `WebpOptions::MAX_METHOD` | consts | Tier not decided | libwebp's default and largest method (4, 6) |
| `JpegXlOptions` (`Lossy { distance, depth }`, `Lossless { effort, depth }`) | enum | Experimental (ADR-0039); default lossy d1.0, patches off (owner decision 2026-11-15) | Non-exhaustive; `PartialEq` only; `depth: Option<u8>` in both variants since 2026-11-15 (`None` follows the input) |
| `JpegXlOptions::MIN_LOSSY_DISTANCE`, `MAX_LOSSY_DISTANCE`, `depth()`, `with_depth()` | consts, methods | Tier not decided | The admitted distance range (0.05, exclusive 3.0) and the depth of either variant |
| `ChromaSubsampling` | enum | Stable | Non-exhaustive |
| `AvifOptions` (`quality`, `alpha_quality`, `speed`, `depth`, `chroma`), `TransformOptions::avif` | struct, field | Experimental (owner decision 2026-11-15, ADR-0037; replaces `avif_depth`, `quality`, `speed`, `chroma_subsampling`) | libavif's scales; `avifenc` defaults 60, 60, 6, depth `None` (follows the input: 8, 10 or 12; a fixed 8 until the depth decision of 2026-11-15), 4:4:4; `depth: Option<u8>`; public fields, not non-exhaustive, like `TransformOptions` (problem 1) |
| `LibavifBackend` | unit struct | Experimental (`avif-write`; stable Rust 1.99 since 2026-11-15, pinned nightly before) | ADR-0037; the built-in AVIF writer; reads every setting from the request since 2026-11-15 |
| `WorkingPrecision` | enum | `F32` stable; `F64Reference` experimental (verification only) | Non-exhaustive |
| `Output` | struct | Stable | Non-exhaustive; `format`, `bytes`, `width`, `height`, `report` |
| `ProcessingReport`, `SourceAccess` | struct, enum | Stable field names; values are observations | Non-exhaustive; new fields are additive. `decoder_backend` and `encoder_backend` strings are experimental values (2026-11-14, #217) |
| `StageTimings` | struct | Experimental values | Wall-clock observations, never a contract; since 2026-11-16 "`total` is at least the sum of the stages" is described as this release's behavior, not promised (problem 16) |
| `ProcessingReport::decoder_work`, `DecoderWork` (`crop`, `requested`, `codec_request`, `codec_output`, `overfetch_pixels`, `reconstructed_pixels`, `entropy_rows`, `delivered_rows`, `wrapper_copied_bytes`) | field, struct | Tier not decided | Non-exhaustive, `Copy`; ADR-0012; since 2026-11-16 "`requested` equals `crop`" is described as this release's behavior, not promised (problem 16) |
| `WorkCount` (`Exact`, `AtLeast`), `WorkCount::at_least` | enum, method | Tier not decided | Non-exhaustive; only `Exact` bounds the work from above |
| `ProcessingReport::input_depth`, `output_depth` | fields | Tier not decided | Bits per sample read and written (since 2026-11-15, ADR-0021 amendment) |
| `ColorSignal`, `ProcessingReport::color_signal` | enum, field | Experimental (working decision 2026-11-14, #212) | Non-exhaustive; reports the signal used, never a precedence |
| `KernelLevel`, `KernelLevelReason`, `TransformOptions::kernel_level_ceiling` | re-exported enums, field | Experimental (diagnostics) | Couples to `ferrisma-kernels`; see problem 7 |
| `ProcessingError`, `ProcessingErrorKind` | struct, enum | Stable | Kind is non-exhaustive; message text is not stable |
| `RefusalReason`, `ProcessingError::refusal_reason` | enum, method | Experimental (working decisions 2026-11-14 and 2026-11-15, #212) | Non-exhaustive closed vocabulary by refusal class (thirteen variants since 2026-11-15, `IccColorSpaceMismatch` added with gray input); `None` outside it |
| `ProcessingReport::discarded`, `Discarded`, `DiscardedSet` | field, enum, set | Experimental (working decision 2026-11-15, ADR-0042 D2) | Non-exhaustive classes of information left out of a correct picture; CLI `discarded=` |
| `Engine`, `BatchItem` | struct, struct | Stable | ADR-0022 |
| `transcode_with`, `transcode_resized_with`, `Engine::transcode_batch_with` | fn | Experimental | ADR-0024 backend extension point |
| `EncoderBackend`, `RowEncoder`, `EncodeRequest`, `EncodedImage`, `EncodeError`, `EncodeErrorKind` | traits, structs, enum | Experimental | Implementors break if a required method is added; the structs and kind are already non-exhaustive |

### Rust API (`ferrisma::cli`)

`cli::run(args, backend)` is public so adapter crates can ship the same
command, and since 2026-11-16 `cli::main(args, backend) -> ExitCode` runs it
and prints the error and chooses the exit status as the binary does (the
binary and `ferrisma-libavif` call it). Proposed tier for both:
experimental and `#[doc(hidden)]` candidates (problem 12); the binary's
command line, not these functions, is the stable CLI contract.

### Cargo features

| Feature | Proposed 1.0 tier | Toolchain |
| --- | --- | --- |
| `jpeg-read`, `webp-read`, `webp-write`, `icc` | Stable | Stable 1.99 (1.98 before 2026-11-15) |
| `jpeg-xl-read` | Stable as "JPEG XL input"; since 2026-11-15 the workspace libjxl-rs reader, the only one (ADR-0032) | Stable 1.99 (1.98 before 2026-11-15) |
| `jpeg-xl-write` (in `cli` since 2026-11-12, ADR-0039) | Experimental | Stable 1.99 (1.98 before 2026-11-15) |
| `avif-read`, `avif-write` | Experimental; tier not decided (`avif-write` was the stable rav1e capability until 2026-11-15; the "until they build on stable" condition is met since 2026-11-15, so only the owner's choice remains) | Stable 1.99 (pinned nightly before 2026-11-15), native targets only |
| `cli` | Stable command line; inherits the 1.99 floor through `avif-read`/`avif-write` (the nightly requirement until 2026-11-15) | Stable 1.99 |

### Command line (`ferrisma` binary, `cli` feature)

| Element | Proposed tier |
| --- | --- |
| Positional `INPUT OUTPUT WIDTH HEIGHT`; input format from the extension (`jpg`, `jpeg`, `webp`, `jxl`, `avif`); output from `.avif` or `.webp` | Stable |
| `--assume-srgb`, `--use-rendered-icc` | Stable |
| `--webp-quality=`, `--webp-method=`, `--webp-lossless`, `--webp-level=` | Stable (`--quality=`, `--speed=`, `--chroma=` removed on 2026-11-15) |
| `--crop=x,y,w,h` | Stable |
| `--max-input-mb=`, `--max-input-mpx=`, `--budget-mb=`, `--max-output-mb=` | Stable |
| `--transform-threads=`, `--encoder-threads=` | Stable |
| `--area` | Stable, see problem 15 |
| `--avif-quality=`, `--avif-alpha-quality=`, `--avif-speed=`, `--avif-depth=8\|10\|12` (12 and "omitted follows the input" since 2026-11-15), `--avif-chroma=444\|420` | Experimental, as `AvifOptions` |
| `--reference`, `--kernel-level=scalar\|neon\|avx2\|avx512` | Experimental (verification and diagnostics) |
| `.jxl` output, `--jxl-distance=`, `--jxl-lossless` (ADR-0039) | Experimental |
| `--jxl-depth=8..16` (since 2026-11-15; omitted follows the input) | Tier not decided |
| Exit status 0 success, 2 usage error (missing positionals, unknown, repeated or contradictory flag, unsupported extension, a value that does not parse, `InvalidOptions`), 1 processing error (since 2026-11-16; before: 2 only for fewer than four positionals, 1 for everything else) | Stable |
| Standard error: one line `error: <message>` (message text not stable); for an error the engine returned, a second line `failure: kind=<kind> reason=<reason or none>` with lower-case hyphenated names owned by the CLI (since 2026-11-16; before, Rust's `Error: ` and the derived `Debug`) | Tier not decided |
| `--help`, `-h`: usage on stdout, exit 0 (since 2026-11-16) | Tier not decided |
| `--version`, `-V`: four lines on stdout (version, target, features, build), exit 0, precedence over every other argument | Tier not decided |
| Each flag at most once; mutually exclusive pairs refused with both names (owner decision 2026-11-15) | Stable |
| `report:` key=value line on stdout | Stable keys; new keys are additive; integer values except `precision`, `kernel_level`, `encoder`, `decoder` (appended 2026-11-14, #217), `color_signal` and `discarded`; the names taken from enums are the CLI's own lower-case hyphenated values since 2026-11-16 (tier of the values not decided) |
| `decoder_work:` line on stdout (rectangles `WxH+X+Y`, counts with `>=` for a lower bound) | Tier not decided |
| Human-readable `Encoded …`, `Timings: …` lines and the stderr memory note | Not stable |

## Decision

Proposed, pending maintainer review:

1. **Tiers.** Items marked stable above are covered by SemVer from 1.0.0 on.
   Experimental items are documented as such in rustdoc and in the
   [compatibility guide](../compatibility.md), may change or disappear in any
   minor release, and must not be required to use a stable feature. Private
   items have no compatibility promise.
2. **Semantic versioning** follows Cargo's SemVer rules for every published
   package. Breaking changes for `ferrisma` include: removing or renaming a
   stable item, feature, or CLI flag; changing a signature; adding a public
   field to a struct that is not `#[non_exhaustive]`; adding a required method
   to a stable trait; changing a default that changes output semantics
   (dimensions, orientation, color meaning, alpha, the strict color policy);
   or making a stable feature require a different toolchain. Not breaking:
   new variants of non-exhaustive enums (including new
   `ProcessingErrorKind` categories), new report fields, new optional
   features, error message text, timings, memory observations, and changed
   encoded bytes at the same documented semantics (encoder improvements).
3. **Output compatibility.** Ferrisma promises the documented meaning of the
   output (format, dimensions, color, alpha, orientation), not byte-identical
   files across versions. Byte-level changes are listed in the changelog.
4. **Rust version policy.** The stable feature set declares
   `rust-version` and keeps a CI job at that version. Raising it is allowed
   in a minor release, never in a patch release, and is announced in the
   changelog. Nightly-only experimental features name the exact pinned
   nightly in the release notes; changing that pin is a minor-release change.
   Amended 2026-11-15 (owner decision, amendment log): the product has one
   minimum, Rust 1.99, for every feature; the codec ports published on their
   own declare their measured floors; a raise is deliberate, in its own
   change, to a release then at least three months old; and a CI job builds
   and tests each package group at exactly its declared version.
5. **Dependencies.** Codec and CMS crates stay out of public signatures
   (ADR-0017), so their major versions are internal. `ferrisma-kernels` is a
   public dependency while `KernelLevel` is re-exported; a breaking change to
   those two types is a breaking change of `ferrisma` (problem 7). Since
   2026-11-16 `ferrisma` requires the kernels and its four bridges with `=`
   (amendment of that date).
6. **Deprecation and upgrade guidance.** Stable items are deprecated with
   `#[deprecated(since, note)]` for at least one minor release before removal
   in the next major release. Each release has a changelog entry with
   *Breaking*, *Added*, *Changed*, *Output changes*, and *Toolchain* sections;
   each major release adds an upgrade guide to the compatibility document.
7. **Applied now (uncontroversial):** documentation for every public item and
   `#[non_exhaustive]` on the growable enums (`ImageFormat`, `OutputFormat`,
   `WebpOptions`, `AvifEncoder` and `JpegXlDecoder` (both removed on
   2026-11-15, see the amendment log), `InputProfilePolicy`,
   `WorkingPrecision`, `ChromaSubsampling`, `SourceAccess`, `ResizeFit`,
   `ResizeFilter`, `KernelLevel`, `KernelLevelReason`) and the output-only
   structs (`Output`, `ProcessingReport`, `StageTimings`, `ImageMetadata`).
   `ProcessingErrorKind`, `EncodeErrorKind`, `EncodeRequest`, and
   `EncodedImage` already were. Consumers that matched these enums
   exhaustively need a wildcard arm; nothing in the workspace did.

## Problems to resolve before 1.0 (recommendations, not applied)

These change signatures or behavior and therefore need a maintainer
decision. Items 1, 2, 7, and 10 should be settled before 1.0.0 because they
cannot be fixed later without a major release. Item 16 is a check the
owner requires of the API review before 1.0.

1. **Input option structs cannot grow.** `TransformOptions`,
   `ResizeOptions`, `InspectionLimits`, `BatchItem`, and `LibavifBackend` have
   public fields and no `#[non_exhaustive]`, and the documented usage is a
   struct literal with `..TransformOptions::default()`. Adding any option is
   then a breaking change. Making them non-exhaustive forbids that literal
   (including the functional-update form) outside the crate; callers would
   use `let mut o = TransformOptions::default(); o.quality = 80.0;` or a
   builder. Recommendation: mark them `#[non_exhaustive]` before 1.0 and
   update the examples and documentation in the same change.
2. **`TransformOptions` mixes formats.** `quality`, `speed`,
   `chroma_subsampling`, and `avif_encoder` apply only to AVIF, while WebP uses
   `webp`; `quality`/`speed` change scale with the selected AVIF encoder.
   Recommendation: an `AvifOptions` value parallel to `WebpOptions`, or an
   output enum carrying per-format settings. _Done on 2026-11-15 (owner
   decision, ADR-0037, ADR-0043 D4):_ `AvifOptions`, and the mixed fields
   are removed.
3. **Two geometry entry points.** `output_width`/`output_height` are ignored
   by `transcode_resized`. Recommendation: one geometry enum (exact or
   `ResizeOptions`) inside the options, or keep both functions and document
   the ignored fields (done in rustdoc). _Since 2026-11-15 (owner decision,
   ADR-0042):_ not ignored but refused when they differ from the default;
   the geometry enum stays a recommendation.
4. **Backend variants double the API.** Each entry point has a `_with`
   twin. Recommendation: keep the twins experimental for 1.0, or fold the
   backend into an options value later.
5. **Output-limit errors are misreported.** Exceeding `max_output_bytes`
   returns `ResourceExhausted`, and the WebP writer's message says "managed
   buffer needs 12 bytes with 64 bytes available", which describes the
   pixel budget. Recommendation: an additive `OutputLimitExceeded` kind and
   an accurate message.
6. **No structured error details.** `ProcessingError` exposes `kind()`,
   the message, and `refusal_reason()` (#212): since 2026-11-14 for four
   refusals, since 2026-11-15 for every documented refusal class of the
   [supported matrix](../supported-matrix.md) (twelve reasons). Additive accessors (which limit, requested and allowed
   values) and `source()` can follow after 1.0.
7. **`KernelLevel` couples SemVer to `ferrisma-kernels`.** Recommendation:
   define the two diagnostic enums in `ferrisma` and map them, or keep them
   experimental and version both crates together. _Since 2026-11-15 (owner
   decision O3, ADR-0041):_ all packages share one version, which
   [release automation](../release-automation.md) noted makes this problem
   moot; the recommendation to define the enums in `ferrisma` stays open.
   _Not moot (API review of 2026-11-16):_ one shared version does not stop
   a caret requirement from resolving a published `ferrisma` to a later
   kernels or bridge release whose API changed, and the bridges follow no
   SemVer of their own. Since 2026-11-16 `ferrisma` requires the kernels and
   the four bridges with `=` (amendment of that date, item 4); the
   kernel-level enums are still foreign types in `ferrisma`'s API, so the
   recommendation to define them in `ferrisma` stays open.
8. **Raw CICP.** `ImageMetadata::rendered_cicp` is `Option<[u8; 4]>`.
   Recommendation: a named `Cicp` struct with primaries, transfer, matrix,
   and full-range fields.
9. **Thread counts use `0` as "automatic".** `transform_threads`,
   `encoder_threads`, and `Engine::new` overload `usize`. Recommendation:
   `Option<NonZeroUsize>` or a small `Threads` enum.
10. **`Copy` on `TransformOptions`.** Any future non-`Copy` option (for
    example a caller-supplied ICC profile) would break it. Recommendation:
    drop `Copy` before 1.0 if such options are plausible.
11. **`AvifEncoder::Libavif { depth }` and `LibavifBackend { depth }`
    duplicate the same configuration.** Recommendation: one type. _Done on
    2026-11-15:_ the depth is `AvifOptions::depth` only; `LibavifBackend`
    reads it from the request.
12. **`cli::run` is a public library function** returning
    `Box<dyn Error>`. Recommendation: `#[doc(hidden)]` or move it to the
    binary's crate if adapters no longer need it.
13. **CLI help.** _Since 2026-11-16 (amendment of that date):_ `--help`
    and `-h` print the usage on standard output and exit 0, and an unknown
    flag exits 2. Before, there was no `--help`; usage was printed only for
    too few arguments, and an unknown flag exited 1. `--version` (and `-V`) exists
    since 2026-11-13 (#237, [compatibility guide](../compatibility.md)) but
    is not yet assigned to a tier. Recommendation: add `--help` and assign
    both flags a tier before 1.0.
14. **CLI units differ.** `--max-input-mb`, `--budget-mb`, and
    `--max-output-mb` use MiB, while `--max-input-mpx` uses 10^6 pixels.
    Recommendation: document the units (now in the compatibility guide) or
    rename with explicit units.
15. **CLI filter and fit flags.** `--area` is a lone switch, the CLI cannot
    express `ResizeFit`, and `--kernel-level` lacks `avx512` (added 2026-11-13
    as the AVX-512 opt-in, ADR-0020 decision D6).
    Recommendation: `--filter=area|lanczos3` and `--fit=inside|cover|fill`
    before 1.0, keeping `--area` as a deprecated alias.
16. **Compatibility with later streaming** (owner constraint of
    2026-11-15). Row-wise or streaming reading and writing at both ends
    (#197, #198) and ROI decode (#60) stay after 1.0; the scope is not
    widened for them now. But the 1.0 contract must not fix anything that
    those improvements would have to break. The API review before 1.0
    checks every stable item for such an assumption, for example a
    signature that requires the whole input or output in memory, a report
    field or error kind that only a whole-frame decoder can fill, or a
    guarantee about when work happens. _Partly done on 2026-11-16_ (API
    review of that date, amendment item 5): the review found the reading
    and the writing side compatible after pre-1.0 changes. Four rustdoc
    guarantees that streaming would have to break were reworded into
    descriptions of today's behavior, without a behavior change: the
    `transcode` rustdoc ("only one decoder region is requested"),
    `DecoderWork::requested` ("equals `crop`"), `StageTimings::total` ("at
    least the sum of the other stages"), and `TransformOptions::max_output_bytes`
    ("checked after their encoder returns"). What remains, all owner
    decisions: the output-limit kind (`ResourceExhausted` today, problem 5),
    `Copy` on the report types and `TransformOptions` (problem 10),
    exhaustive `TransformOptions` and `InspectionLimits` (problem 1), `Send`
    on `RowEncoder`, the CLI's output-file atomicity (the file is written
    only after success today, not promised), and the reader and writer entry
    points themselves, which can be added later.

## Consequences

+ Callers can rely on a documented subset, and maintainers know which
  changes need a major release.
+ Non-exhaustive enums and output structs let Ferrisma add formats, policies,
  error kinds, and report fields in minor releases.
- Exhaustive matches on the affected enums now need a wildcard arm, and
  output structs can no longer be constructed outside the crate (no
  workspace code did either).
- Until problems 1, 2, 7, and 10 are decided, a 1.0.0 release would freeze
  shapes that the review identifies as hard to evolve.
- Nightly-only features stay experimental, so the stable promise covers
  fewer formats than the full CLI.

## Alternatives considered

- **Declare the whole current surface stable.** Freezes experimental
  backends, diagnostic types, and option shapes with known problems.
- **Stay on 0.x indefinitely.** Honest about churn, but gives users no
  upgrade rules; Cargo already treats 0.x minor versions as breaking, which
  this policy keeps until the maintainer accepts 1.0.
- **Separate `ferrisma-cli` crate now.** Cleaner library surface, but ADR-0008
  asks for a demonstrated need; deferring keeps the published chain shorter.
- **Builder-only API.** Avoids public fields entirely, at the cost of more
  surface; problem 1 lists it as one option.

## Validation

- `cargo doc` without missing-documentation warnings for the engine crate
  (`#![warn(missing_docs)]`, denied in CI through `-D warnings`).
- `examples/consumer.rs --self-check` in CI asserts the stable failure
  categories and limits through the public API.
- Before acceptance: build a consumer against the packaged crates outside
  the workspace (see [publishing](../publishing.md)), and run a SemVer checker
  such as `cargo-semver-checks` against the previous release for every
  release after 1.0.0.

## Review when

The maintainer accepts or rejects the recommendations above; the AVIF
features build on stable Rust (met since 2026-11-15; their tier is still
open); a new output format, color policy, or backend
is added; the first functional crates.io release is prepared; or a consumer
reports that a stable item cannot express its use case.

## Amendment log

- 2026-11-12 — Proposed with the surface inventory, tiers, SemVer, Rust
  version, deprecation and upgrade policy, the applied `#[non_exhaustive]` and
  documentation changes, and fifteen recommendations.
- 2026-11-12 — Default switch interaction (ADR-0032, ADR-0037, pending owner
  confirmation): `JpegXlDecoder::default()` is now `LibjxlRs` whenever
  `jpeg-xl-own-read` is compiled in (including `cli`), and
  `AvifEncoder::default()` is `Libavif { depth: 10 }` whenever
  `avif-own-write` is compiled in (including `cli`). The inventory above still
  lists those variants and features as experimental and `JxlOxide`/`Rav1e` as
  stable, so the stable `cli` feature now runs experimental backends by
  default, and an unchanged AVIF `quality` value means a different encoder
  scale. Before 1.0 the maintainer needs to either promote the two workspace
  backends (with their nightly/native-target boundary) or state that the
  stable promise for `cli` covers the capability, not the backend, as it
  already does for `jpeg-xl-read` and `avif-write`. Under decision 2, a
  default change that changes output semantics is breaking after 1.0; this
  change happens before any functional release.
- 2026-11-13 — Owner decision D10, recorded in [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) section 8: pre-releases are published from `main` as 0.x; the version **1.0 is reserved** for the state that meets ADR-0041's quality gate completely (all four ports, AArch64 and x86-64, scalar and the v1.0 SIMD levels). Releases use Release Please as in the organization's other packages (`sebastian-software/ferromark` ADR-0020, `sebastian-software/ferrugo`). Whether all crates share one version is open (ADR-0041 O3). This adds a release condition; it does not accept this ADR's proposed tiers or decide its fifteen recommendations, which remain pending maintainer review. Under decision 2, 0.x minor releases may break, as Cargo treats them. The alternative "stay on 0.x indefinitely" is replaced by "stay on 0.x until the gate is met".
- 2026-11-13 — Release automation (owner decision D10: 0.x pre-releases from
  `main`, 1.0 reserved for the state that meets the quality gate, Release
  Please as in the organization's other packages). The inert draft and its
  evidence are in [release automation](../release-automation.md): the
  organization conventions with sources, two configurations (one shared
  version, or engine group plus per-port versions; open question O3), a local
  rehearsal with release-please 17.6.0, and the blockers before a first 0.1.0.
  Relevant here: `bump-minor-pre-major` keeps breaking changes in 0.x minors,
  so 1.0.0 only comes from an explicit `Release-As: 1.0.0`; Release Please's
  default changelog sections do not match decision 6's *Output changes* and
  *Toolchain* sections without extra commit types or curated notes; and with a
  shared version problem 7 disappears. Nothing was activated or published.
- 2026-11-13 — ADR-0020 decision D6: `kernel_level_ceiling: None` now stops
  at AVX2 on x86-64 and NEON on AArch64; `KernelLevelReason` gains
  `DefaultCeiling` (the enum is `#[non_exhaustive]`), and
  `--kernel-level=avx512` is the opt-in. Both stay experimental.
- 2026-11-13 — Corrections, no decision change: the version statement in the
  context now names the three members at 0.0.0 (`ferrisma-avif-read`,
  `ferrisma-avif-write`, `ferrisma-libavif`; their manifests), and a
  duplicated "Related" line is removed. The default switch of 2026-11-12 is
  confirmed by owner decision D1 ([ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md)
  section 7); the tier question it raised for `cli` stays open with this
  record's other recommendations.
- 2026-11-14 — Decoder backend in the report (issue #217; design in
  [ADR-0017](0017-minimal-public-facade-and-cli.md), amendment of the same
  date): `ProcessingReport::decoder_backend: &'static str` is an additive
  field of the non-exhaustive report (decision 2), and the `report:` line
  appends `decoder=`. Working decision 2026-11-14 (delegated, revisable by
  the owner): its string values share the experimental tier of the
  `encoder_backend` strings; the field name follows the report's stable
  field-name rule. Correction, no decision change: problem 13 said there is
  no `--version`; the flag exists since 2026-11-13 (`src/cli.rs`, #237), so
  the problem now covers `--help` and the tier of both flags.
- 2026-11-14 — Color signal and refusal reasons (issue #212; design in
  [ADR-0017](0017-minimal-public-facade-and-cli.md), amendment of the same
  date): `ColorSignal` with `ProcessingReport::color_signal`, and
  `RefusalReason` with `ProcessingError::refusal_reason()`, both additive and
  `#[non_exhaustive]` (decision 2: new variants are not breaking). The CLI
  `report:` line appends `color_signal=`. Working decision 2026-11-14
  (delegated, revisable by the owner): both enter the inventory as
  experimental, and a reason, once documented, keeps its meaning; making
  them stable is the owner's choice. Problem 6 is narrowed accordingly; which
  limit was exceeded and `source()` remain open.
- 2026-11-15 — Refusal reasons for every documented refusal class (issue
  #212; design in [ADR-0017](0017-minimal-public-facade-and-cli.md),
  amendment of the same date): seven additive `RefusalReason` variants
  (`HdrTransfer`, `UnsupportedColorEncoding`, `RenderedIccMissing`,
  `ContradictoryColorSignals`, `UnsupportedLayout`, `AlphaAssociation`,
  `GeometricTransform`), allowed by decision 2. No `ProcessingErrorKind` and
  no message text changed. Inventory row and problem 6 updated. Working
  decision 2026-11-15 (delegated, revisable by the owner): the enum stays
  experimental; the names and class boundaries are the owner's to confirm.
  Not decided here, and described as today's behavior: whether grayscale or
  premultiplied JPEG XL should be accepted, the strict color policy (#222),
  and the kind of an unknown essential AVIF property (`MalformedInput`,
  #67), which keeps no reason.
- 2026-11-15 — Correction of the entry above (issue #67, approved by the
  owner through the v1.0 coordinator's brief): an unknown essential AVIF
  property is `UnsupportedInput` with the additive
  `RefusalReason::UnsupportedEssentialProperty` (twelve variants), no
  longer `MalformedInput`; a malformed property box stays `MalformedInput`.
  A changed kind for one input class, allowed in 0.x and recorded in
  [ADR-0017](0017-minimal-public-facade-and-cli.md). Working decision
  2026-11-15 (delegated, revisable by the owner): the variant name.
- 2026-11-15 — `ProcessingReport::discarded` (`Discarded`, `DiscardedSet`)
  and the CLI key `discarded=` enter the inventory as experimental
  (additive, decision 2); the rule is [ADR-0042](0042-no-silent-loss-refuse-or-report.md)
  D2, the design the [ADR-0017](0017-minimal-public-facade-and-cli.md)
  amendment of the same date.
- 2026-11-15 — Removed items (owner decision, ADR-0032; pre-1.0 breaking
  change of the inventory): `JpegXlDecoder` with `JxlOxide` and
  `LibjxlRs`, `TransformOptions::jxl_decoder`, the CLI flag
  `--jxl-decoder=`, the `decoder_backend` value `"jxl-oxide"`, and the
  feature `jpeg-xl-own-read`. `jpeg-xl-read` keeps its tier ("JPEG XL
  input") and now compiles the workspace reader, which was
  `jpeg-xl-own-read` (working decision 2026-11-15, delegated, revisable by
  the owner). A build that enabled `jpeg-xl-read` keeps JPEG XL input; one
  that enabled `jpeg-xl-own-read` must rename it; code that set
  `jxl_decoder` must drop the field.
- 2026-11-15 — Removed items (owner decision, ADR-0037; pre-1.0 breaking
  change of the inventory): `AvifEncoder` with `Rav1e` (stable) and
  `Libavif { depth }`, `TransformOptions::avif_encoder`, the CLI flag
  `--avif-encoder=`, the `encoder_backend` value `"rav1e"`, the feature
  `avif-own-write`, and the stable feature `avif-write` as the rav1e
  capability. Added, experimental: `TransformOptions::avif_depth`. The name
  `avif-write` now compiles the workspace AOM writer, which needs the pinned
  nightly, so it moves to the experimental tier until it builds on stable
  (working decisions 2026-11-15, delegated, revisable by the owner). A
  stable-toolchain consumer of the old `avif-write` loses AVIF output.
- 2026-11-15 — Owner decision on repeated and contradictory command-line
  flags (ADR-0042): every flag may be given once, and a pair of mutually
  exclusive flags (`--assume-srgb`/`--use-rendered-icc`,
  `--jxl-lossless`/`--jxl-distance=`, `--webp-lossless` or `--webp-level=`
  with lossy-only settings) is refused with both names, exit status 1,
  before any input is read. A script that relied on "the last one wins"
  breaks; the old result was a silent choice between two instructions.
  Added to the command-line inventory as stable.
- 2026-11-15 — Owner decision on options without effect (ADR-0042):
  behavior of stable items changes, no signature. `transcode_resized` and
  `transcode_resized_with` refuse an `output_width`/`output_height` other
  than the default (problem 3), and `Engine::transcode_batch` refuses the
  thread counts that `transcode` refuses before it replaces valid ones
  with 1, as `InvalidOptions`. A caller that relied on the ignored values
  gets an error instead of a silent choice.
- 2026-11-15 — Native encoder options (owner decision, ADR-0043 D4,
  ADR-0037): `AvifOptions` and `TransformOptions::avif` replace
  `avif_depth`, `quality`, `speed` and `chroma_subsampling` (problem 2
  done); `WebpOptions::Lossy { speed }` becomes `{ method }` on libwebp's
  scale; `EncodeRequest` carries `avif` instead of `quality`, `speed` and
  `chroma`; `LibavifBackend` loses its depth (problem 11 done). CLI:
  `--quality=`, `--speed=` and `--chroma=` removed; `--avif-quality=`,
  `--avif-alpha-quality=`, `--avif-speed=`, `--avif-chroma=`,
  `--webp-quality=` and `--webp-method=` added. The AVIF defaults follow
  `avifenc` (8 bits, quality 60), and the WebP lossy default is libwebp's
  method 4, so default output bytes change. Breaking changes before 1.0,
  intended. _Working decision 2026-11-15 (delegated, revisable by the
  owner):_ `AvifOptions` is a plain struct with public fields and no
  `#[non_exhaustive]`, as `TransformOptions` is until problem 1 is decided;
  the CLI names; the WebP lossy default method 4.
- 2026-11-15 — **Minimum supported Rust versions** (owner decision
  2026-11-15; amends decision 4 and the toolchain column above).
  1. **The product's minimum Rust version is 1.99**, and it is the headline
     everywhere a reader looks first (README, getting started, the supported
     matrix). It applies to every feature: `ferrisma`, `ferrisma-kernels`,
     the bridges `ferrisma-jxl-decoder`, `ferrisma-jxl-encoder`,
     `ferrisma-avif-read`, `ferrisma-avif-write`, and `ferrisma-xtask`
     declare `rust-version = "1.99"` (1.98 for the engine, the kernels and
     the tool, 1.95 for the JPEG XL bridges before); `libavif-rs`,
     `libavif-aom` and `libavif-dav1d` keep 1.99. Reason: AVIF is the
     preferred output format, and the AVIF port defines C-variadic
     functions, stable since Rust 1.99 (libavif-rs ADR 0060), so a number
     that excludes AVIF would mislead. The JPEG XL bridges and the kernels
     carry the product's number because they are used only through the
     engine (both bridge READMEs say so); on their own they would need 1.88
     (bridges) and 1.93 (kernels on x86-64: AVX-512 target features and
     `stdarch_x86_avx512`, `E0658` on 1.88, stabilized in 1.89). The engine
     without AVIF would need 1.89 on its own: `moxcms` 0.9.1 declares
     `rust-version = "1.89"`; the earlier 1.88 measurement passed
     `--ignore-rust-version`, which hides that.
  2. **The codec ports published as standalone crates declare their true
     floor**, because someone may use one without the engine:
     `libjxl-rs` 1.88, `libsharpyuv-core` 1.88, `libjpeg-turbo-rs` 1.93,
     `libwebp-rs` 1.95 (all four 1.95 before). `libavif-sharpyuv` keeps
     1.95 with the AVIF port. What forces each: edition 2024 sets 1.85 as
     the lower bound for all; JPEG's x86 SIMD calls `_mm_sfence` outside an
     `unsafe` block, rejected with `E0133` by 1.88 and accepted by 1.93;
     WebP's x86 CPU query also calls `__cpuid_count` outside an `unsafe`
     block, rejected with `E0133` by 1.93 and accepted by 1.95; nothing in
     `libjxl-rs` or `libsharpyuv-core` needs more than the lowest installed
     toolchain, 1.88. Measured on 2026-11-15 on macOS ARM64 with the
     installed toolchains 1.88, 1.93 and 1.95 (1.85 to 1.87, 1.89 to 1.92
     and 1.94 were not installed or tested, so each declared floor is a
     version that passed fully): `cargo +<v> check --locked -p <package>
     --all-targets --all-features` passes at the declared version for
     `aarch64-apple-darwin`, `x86_64-apple-darwin`,
     `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`;
     `cargo +<v> test --locked -p <package> --profile checked` without
     the C/C++ oracle passes natively and for x86-64 macOS under Rosetta 2
     for JPEG (with and without default features), WebP (both) and
     `libjxl-rs` x86-64; `libjxl-rs` on AArch64 passes 444 tests and fails
     one (`patches::selected_original_alpha_max_retains_comparison_result`,
     a signaling-NaN alpha) identically on 1.88, 1.93, 1.95 and stable
     1.98.1, while the pinned nightly passes it: a stable-versus-nightly
     code generation difference, not a floor (finding; details in the
     supported matrix). The x86-64 Linux and ARM64 Linux results are checks
     only, not runs.
  3. **Rule for the future:** a minimum is raised only deliberately, in its
     own change with a changelog entry (decision 6's *Toolchain* section;
     until Release Please is active, the body of that change's commit),
     never in a patch release, and the new minimum is then at least three
     months old. 1.99 is younger than that today; the owner set it with this
     decision, and the rule governs later raises. A CI job builds and tests
     each package group at exactly its declared version, so the declaration
     cannot go stale: `minimum-rust` (`ci.yml`, GNU Linux x86-64) checks and
     tests `ferrisma` and `ferrisma-kernels` with `cli` and checks the
     non-AVIF set, the four bridges and `ferrisma-xtask` at `1.99.0`; each
     port's static stage (`check.py --msrv`, run by the codec PR job and the
     port workflows) checks at the version its `Cargo.toml` names, and the
     codec PR job installs that version; the `workspace` job checks and lints
     `libsharpyuv-core` at its declared version. The 1.98 steps are gone.
  - **Rejected:** lowering AVIF below 1.99 by rewriting the C-variadic
    definitions (libavif-rs ADR 0060, "Rejected: typed error and control
    entry points"): several days of work with risk at the C ABI, for a gain
    that shrinks every month.
  - Not decided here: whether the static stage's stable Clippy is pinned or
    floats (#235). The pinned nightly in `rust-toolchain.toml` stays the
    development default for formatting, sanitizers and fuzzing; no user
    needs it.
- 2026-11-15 — Owner decisions of 2026-11-15 that touch this record:
  (1) **Clippy (#235): pin.** The static stages' Clippy runs at one
  explicitly named stable version, raised deliberately in its own change;
  recorded in [ADR-0027](0027-tiered-validation-and-rust-test-orchestration.md),
  amendment of 2026-11-15 (on branch `v1/clippy-pin` when this entry was
  written). This answers the "Not decided here" sentence of the MSRV entry
  above; the minimum versions are unchanged. (2) **One shared version for
  all packages** (O3, ADR-0041); problem 7 is annotated. (3) **The JPEG XL
  default** lossy d1.0 with patches off is decided (ADR-0039); the
  inventory row no longer says "pending owner confirmation". (4) **Streaming
  compatibility:** new problem 16, a check the API review must make before
  1.0. This record stays Proposed; its tiers and the other recommendations
  are not decided by these entries.
- 2026-11-15 — Output depth follows the input (owner decision, ADR-0021
  amendment): `AvifOptions::depth` becomes `Option<u8>` and both
  `JpegXlOptions` variants gain `depth: Option<u8>` (breaking before 1.0,
  intended); `ProcessingReport::input_depth` and `output_depth` and the CLI
  keys `input_depth=` and `depth=` are additive (inserted before
  `discarded=`, which stays the last key, ADR-0042); the experimental
  backend API gains `EncodeRequest::depth`, `EncoderBackend::row_depths`
  and `RowEncoder::write_row_u16` with defaults, so existing backends need
  no change.
- 2026-11-16 — **API hardening from the API review of 2026-11-16**
  (working decisions, delegated, revisable by the owner; the record stays
  Proposed). Changes that need no API-shape decision:
  1. **Panic policy.** The public entry points (`transcode`,
     `transcode_resized`, `inspect`, the `_with` variants, and each item of
     `Engine::transcode_batch` and `transcode_batch_with`) catch a panic
     with `std::panic::catch_unwind` and return
     `ProcessingErrorKind::Internal` with a message that says a panic was
     caught (and the panic's text, when it is a `&str` or `String`). Before,
     a panic in a codec port or a backend unwound into the caller, and one
     panicking batch item re-panicked `std::thread::scope` after the join,
     so every result of the batch was lost; now only that item fails.
     Promised: no unwinding out of those functions, kind `Internal`, and a
     later call is unaffected (the memory budget is per call and released
     while unwinding; the argument is at `contain_panic` in
     `src/pipeline.rs`). Not promised: a panic stays a defect, not an input
     class; with `panic = "abort"` the process aborts (the AVIF features do
     not compile with it); a panic that has to cross a C-ABI frame inside a
     codec port aborts; the state of a caller's `EncoderBackend` after its
     own panic is the backend's. Evidence: `tests/panic_containment.rs`
     provokes a panic through the public backend API in
     `required_frame_bytes`, `begin`, `write_row` and `finish`, with one and
     four transform threads and in a batch.
  2. **CLI error output and exit status** (review sections 2.8 and 2.10).
     The binary prints an error with `Display`, not the derived `Debug`
     (`main` returned `Box<dyn Error>`, so Rust printed
     `Error: ProcessingError { kind: …, reason: …, message: "…" }` or
     `Error: "…"`): one line `error: <message>` on standard error, and for an
     error the engine returned a second line
     `failure: kind=<kind> reason=<reason or none>` shaped like the
     `report:` line, with lower-case hyphenated names written by the CLI
     (`kind_name`, `reason_name` in `src/cli.rs`). Exit status 0 success;
     `--help`/`-h` print the usage on standard output and exit 0 (before:
     unknown option, exit 1; too few arguments printed the usage to
     standard error and exit 2, which stays, now after an `error:` line);
     `--version` unchanged; 2 for every usage error (missing positionals,
     unknown, repeated or contradictory flag, unsupported extension, a value
     that does not parse, and a value the engine refuses as
     `InvalidOptions`, so a range check moved into the engine stays a usage
     error); 1 for every processing error (unreadable input, unwritable
     output, refused, malformed, input limit, resource exhausted, feature
     disabled, encoder failure, internal). The usage text now lists
     `--help` and `--kernel-level=`. Added, experimental like `cli::run`:
     `cli::main(args, backend) -> ExitCode`, which the binary and
     `ferrisma-libavif` call, so an adapter's command behaves the same.
     Command-line inventory rows updated; problem 13 is answered for
     `--help`, the tiers of `--help`, `--version` and the error lines are
     not decided. Evidence: `tests/cli_exit_status.rs` through the built
     binary, `cli::tests::exit_status_separates_usage_from_processing_errors`
     (including `Internal` from a caught panic).
  3. **Report value names** (review section 2.8). Every value of the
     `report:` and `decoder_work:` lines that comes from an enum is written
     by a name function of `src/cli.rs`, lower-case and hyphenated, instead
     of `Debug` (`precision`, `color_signal`, `discarded`) or the kernels
     crate's `Display` (`kernel_level`), so renaming a Rust variant or a
     kernels release cannot change the command's output. Changed values:
     `precision=F32` → `f32`, `F64Reference` → `f64-reference`;
     `color_signal=Cicp` → `cicp`, `AssumedSrgb` → `assumed-srgb`,
     `RenderedIcc` → `rendered-icc`, `RenderedIccWithAgreeingCicp` →
     `rendered-icc-with-agreeing-cicp`; `discarded=` classes `GainMap` →
     `gain-map`, `Exif` → `exif`, `Xmp` → `xmp`, `OtherMetadata` →
     `other-metadata`, `IccProfile` → `icc-profile`,
     `UnknownContainerData` → `unknown-container-data`,
     `JpegReconstructionData` → `jpeg-reconstruction-data`, `Preview` →
     `preview`, `HiddenColor` → `hidden-color`, `Unpremultiplied` →
     `unpremultiplied`, `PrecisionReduced` → `precision-reduced` (`none`
     unchanged). Unchanged: the keys and their order, `kernel_level=`
     (`scalar`, `neon`, `sve2`, `avx2`, `avx512`, now owned by the CLI), the
     `decoder_work:` counts (`7`, `>=7`), and the `encoder=`/`decoder=`
     backend strings, which stay the report's strings
     (`libavif-rs/aom`, `libwebp-rs`, `libjxl-rs`, `libjpeg-turbo-rs`, or an
     external backend's `name()`): whether they name the original codec or
     the package is the owner's decision before the rename (review 2.7).
     Evidence: `cli::tests::report_value_names_are_owned_by_the_cli`,
     `cli::tests::report_line_names_the_decoder_and_encoder_backends`,
     `discarded_report::the_cli_report_line_names_the_discarded_classes`.
  4. **Exact pins on the internal crates** (review section 2.2, problem 7).
     `ferrisma` requires `ferrisma-kernels`, `ferrisma-avif-read`,
     `ferrisma-avif-write`, `ferrisma-jxl-decoder` and
     `ferrisma-jxl-encoder` with `=` instead of a caret requirement, and the
     four bridges' READMEs and crate docs say that they are internal to
     `ferrisma`, follow no SemVer of their own and are not for direct use.
     Reason: with caret requirements a published `ferrisma` would resolve to
     a later bridge release whose API changed, although every package shares
     one version. The ports keep caret requirements (they are published as
     standalone crates with their own SemVer). Release Please drops `=`, so
     the inert publish draft runs `publishing/release-please/preserve-exact-pins.mjs`
     on the release branch (ferromark's precedent), and the rehearsal
     applies the same step and fails on a pin without `=`; rehearsed with
     release-please 17.6.0: A and B clean, the negative control 20 findings
     each ([release automation](../release-automation.md)). Not decided:
     whether `ferrisma-kernels` should say the same of itself while its
     enums are re-exported.
  5. **Stale statements** (review section 5). Corrected without a behavior
     change: the crate documentation (`src/lib.rs` said there is
     "intentionally no stable public processing API"; it now names the
     `pipeline` facade and points to the proposed tiers), the `transcode`
     rustdoc ("to SDR AVIF"; it writes AVIF, WebP or JPEG XL), the
     `pipeline` module documentation ("lossless JPEG XL"), the
     `JpegXlOptions::default` rustdoc ("pending owner confirmation"; decided
     2026-11-15), `Discarded::PrecisionReduced` ("8-bit rows"; since
     2026-11-15 it means an output depth below the input's), the
     compatibility guide's paragraph that AVIF output needs the nightly, and
     the two AVIF bridge READMEs ("needs the nightly"). In this record: the
     context's `avif-own-write` and nightly statement (annotated), decision
     7's removed `AvifEncoder` and `JpegXlDecoder` (annotated), the
     `AvifOptions` depth default (`None`, follows the input), the
     `JpegXlOptions` depth, the AVIF feature rows ("experimental until they
     build on stable", a condition met on 2026-11-15; tier not decided),
     `--avif-depth=12`, and new rows, each "tier not decided": `DecoderWork`
     with `ProcessingReport::decoder_work`, `WorkCount`, `input_depth` and
     `output_depth`, the constants and methods of `WebpOptions` and
     `JpegXlOptions`, `--jxl-depth=`, `--version`, `--help`, the error lines
     and the `decoder_work:` line. The four streaming-blocking guarantees
     are reworded (problem 16).
- 2026-11-16 — **API shape before the freeze** (owner decision of
  2026-11-16: the redesign of the API review's priorities 2 and 7 to 13 is
  accepted; the record stays Proposed for its tiers). Each item names the
  problem it closes.
  1. **One construction style for every option type** (problems 1, 2, 3).
     `TransformOptions`, `ResizeOptions`, `Crop`, `InspectionLimits`,
     `BatchItem`, `AvifOptions` and the per-profile settings of WebP and
     JPEG XL have private fields and are `#[non_exhaustive]`; a value comes
     from a constructor (`TransformOptions::new(geometry)`, `Crop::new`,
     `BatchItem::new`, `WebpLossless::new(level)`) or `Default`, and each
     setting has a `with_*` method that returns the changed value and a
     getter of the same name. _Why this style and not public fields that
     stay assignable after construction_ (the other form the review
     compared, `image`'s `Limits`; `zune-image`'s `DecoderOptions` is the
     precedent for methods on private fields): with methods, a setting's
     storage can change without a breaking change (a field's type and name
     are API, a method's signature only), a group of settings can later move
     into its own type behind forwarding methods, and a value is built in
     one expression wherever one is needed (batch items, nested settings,
     tests), where public fields need a `let mut` and a statement per field.
     The cost is two methods per setting, generated by one macro in
     `src/pipeline.rs`. A struct literal, including the functional-update
     form, does not compile outside the crate, and no documentation shows
     one. Report types keep public fields (read access that survives added
     fields, item 2).
     - `output_format` and the three per-format fields are one
       `OutputSettings` (`Avif(AvifOptions)`, `WebP(WebpOptions)`,
       `JpegXl(JpegXlOptions)`, `#[non_exhaustive]`) in `TransformOptions`
       and in `EncodeRequest`; the selected settings decide the output
       format. "The other formats' settings must equal their defaults", and
       with it the refusal "encoder options do not match the selected output
       format", are gone with the state that could express them: nothing
       accepted is ignored (ADR-0042).
     - `WebpOptions::{Lossy, Lossless}` and `JpegXlOptions::{Lossy,
       Lossless}` carry `WebpLossy`, `WebpLossless`, `JpegXlLossy` and
       `JpegXlLossless`, so a profile can gain a setting; callers still
       match lossy against lossless.
     - The geometry is `OutputGeometry::{Exact { width, height },
       Resize(ResizeOptions)}`, `#[non_exhaustive]`, the constructor's
       argument; the `1` sentinel of `output_width`/`output_height`, the
       refusal of an exact size beside `ResizeOptions`, and the entry points
       `transcode_resized` and `transcode_resized_with` are gone (problem 3;
       `transcode` and `transcode_with` take either geometry, which also
       halves the `_with` twins of problem 4).
     - The depth settings are `BitDepth::{FollowInput, Exact(u8)}`,
       `#[non_exhaustive]`, so float output or "follow the input, at most N"
       can be added as variants.
     - `LibavifBackend` is a `#[non_exhaustive]` unit struct built with
       `LibavifBackend::new()` or `Default`.
     - Option types are `Clone`, not `Copy` (item 2).
  2. **No `Copy` on types that may gain fields** (problem 10, review 2.1,
     priority 7). `ProcessingReport`, `DecoderWork`, `StageTimings`,
     `EncodeRequest`, `TransformOptions`, `BatchItem` and `Engine` are
     `Clone` but not `Copy` (the option types since item 1), so that a later
     field that is not `Copy` (a list of regions or tiles, an owned name, a
     caller-supplied profile, a worker pool) is additive. Every report
     struct (`Output`, `ProcessingReport`, `StageTimings`, `DecoderWork`,
     `ImageMetadata`, `EncodedImage`) is `#[non_exhaustive]` with public
     fields: read access that survives added fields, no construction or
     exhaustive destructuring outside the crate. Small value types keep
     `Copy` (`Crop`, `BitDepth`, the enums without data, `DiscardedSet`,
     `WorkCount`).
  3. **`OutputLimitExceeded`** (problem 5, review 2.8; a behavior change of
     the error kind). An encoded output above `max_output_bytes` is the new
     `ProcessingErrorKind::OutputLimitExceeded`, no longer
     `ResourceExhausted`, with one message for every path ("encoded output
     exceeds max_output_bytes: it needs at least N bytes, and the limit is
     M"; the WebP and lossy JPEG XL writers said "managed buffer needs …",
     the memory budget's message). The CLI names it
     `failure: kind=output-limit-exceeded` and exits 1, as for every
     processing error. When the limit is checked (while writing or after
     the encoder returns) stays a description of today's behavior, so a
     streaming writer can stop earlier with the same kind (problem 16).
