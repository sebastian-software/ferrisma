# ADR-0018: Feature-gated codec and CMS dependencies, MSRV and lockfile

- **Status:** Experimental
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-05
- **Supersedes:** None
- **Related:** [libavif-rs](https://github.com/sebastian-software/libavif-rs), [libjxl-rs](https://github.com/sebastian-software/libjxl-rs), [0005](0005-codec-capabilities-and-pixel-contracts.md), [0010](0010-dual-licensing.md), [0012](0012-codec-source-contract.md), [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [encoder and CMS research](../research/2026-10-31-encoder-and-cms-candidates.md), [codec adapter audit](../research/codec-adapter-audit.md), [JPEG and WebP evaluation](../research/2026-11-05-jpeg-and-webp-codec-evaluation.md), [libwebp port plan](../research/2026-11-05-libwebp-port-plan.md), [license audit](../license-audit.md)

## Context

The research notes of 2026-10-31 named candidates but chose no dependency. PR #11 then added four optional crates (`ravif`, `avif-decode`, `avif-parse`, `moxcms`), raised the package MSRV to 1.98 and left `Cargo.lock` uncommitted, without a decision record. AGENTS.md requires an ADR before new dependencies. This ADR records the current choices, the evidence behind them and the gates that remain open. It does not declare any of them release-ready.

**Strategic direction.** Ferrixel's AVIF and JPEG XL codecs are meant to come from the sister projects [libavif-rs](https://github.com/sebastian-software/libavif-rs) and [libjxl-rs](https://github.com/sebastian-software/libjxl-rs): mechanical, reference-equivalent Rust ports of libavif (with libaom, dav1d and SharpYUV) and libjxl. Their state on 2026-11-05:

- **libavif-rs:** complete port with Rust AOM, dav1d and SharpYUV; validated as a whole pipeline on macOS ARM64 and, since 2026-11-06 (`f3b2981`), on GNU Linux x86-64 for six Cargo feature sets. It exposes the original unsafe C ABI, requires nightly Rust and sets `publish = false`.
- **libjxl-rs:** complete public decoder API and decoder C ABI with exact parity against libjxl; thread runners in review; no encoder yet. Its Rust API is provisional (`jxl::jxl` module tree) and the crate is not publishable.
- **libwebp-rs:** does not exist yet. WebP input and output are planned as a third port of libwebp 1.6.0 following the libjxl-rs method ([plan](../research/2026-11-05-libwebp-port-plan.md)); the libavif-rs repository already pins the same libwebp revision for SharpYUV.

Verified facts (2026-11-05, `main` at 9291c33):

- `default = []`. Without features the crate has no dependencies and forbids `unsafe`.
- `avif-decode` 3.0.0 declares `rust-version = 1.98`. Because `rust-version` is package-wide, the codec-free core also requires 1.98.
- `avif-decode` uses `rav1d` (unsafe Rust with optional assembly; NASM is needed on x86 hosts) and `avif-parse` 2.1.0 (MPL-2.0). Ferrixel also depends on `avif-parse` **directly** for the dimension preflight.
- Neither `avif-decode` nor `avif-parse` exposes the primary item's `colr`, `irot`, `imir` or `clap` properties or the AV1 sequence-header color configuration. `avif-decode` converts YUV with the bitstream matrix and range.
- `ravif` 0.13 defaults to 10-bit AV1 output (`BitDepth::Auto`) and omits the `colr` box when the values equal its sRGB/BT.709 primaries, BT.601 matrix, full-range default.

## Decision

Keep every codec and CMS behind its own Cargo feature, off by default:

| Feature | Crate | Role | Status of the choice |
| --- | --- | --- | --- |
| `jpeg-read` | `zune-jpeg` 0.5 | JPEG input (full-frame fallback) | Experimental, ADR-0012 |
| `jpeg-xl-read` | `jxl-oxide` 0.12 | JPEG XL input with codec ROI request | **Interim** until libjxl-rs is adoptable (ADR-0012) |
| `avif-write` | `rav1e` 0.8 / `avif-serialize` 0.8 | First and only output encoder, driven directly since 2026-11-06 | **Interim choice** (permissive, Rust plus assembly); target: libavif-rs |
| `avif-read` | `avif-decode` 3.0 / `rav1d`, `avif-parse` 2.1 | AVIF input, full-frame fallback | **Interim, provisional**; target: libavif-rs |
| `icc` | `moxcms` 0.9 | Rendered-ICC → sRGB source adapter | **Chosen for M1 evaluation**; color fidelity still unverified |
| `webp-read`, `webp-write` (planned) | libwebp-rs (planned port) | WebP input and output, lossy and lossless | **Target only, no interim crate**; see the [evaluation](../research/2026-11-05-jpeg-and-webp-codec-evaluation.md) |
| JPEG output | none | — | **Not planned**; JPEG stays input-only through `jpeg-read` |

Rules that apply to all of them:

1. **Ferrixel owns color and geometry semantics.** Where a crate does not expose what the emitted samples mean, Ferrixel reads the evidence itself. For AVIF, a dependency-free, bounds-checked parser (`codecs/avif_meta.rs`) reads `colr`, `ispe`, `irot`, `imir`, `clap` and the AV1 sequence header before any pixel decode. Contradictory signals, unsupported transforms and unknown essential properties fail closed.
2. **Third-party codecs are interim.** `avif-decode`, `ravif` and `jxl-oxide` are accepted because they work today on the CI targets; adapters must stay replaceable without public API impact. The replacement target is libavif-rs for AVIF input and output, libjxl-rs for JPEG XL input and, once ported, output, and libwebp-rs for WebP input and output. A libjxl-rs encoder would also resolve the JPEG XL output license gate, since libjxl is BSD-3-Clause. `rav1d-safe` is only a fallback if the sister ports cannot meet the gates below in time.
3. **Adoption gates for the sister ports.** Ferrixel switches a codec when the port provides (a) a safe Rust API, or a small isolated adapter crate, because this package forbids `unsafe`; (b) a stable Rust toolchain; (c) a dependency Ferrixel can pin (published crate or pinned git revision); (d) evidence on Ferrixel's CI targets; and (e) exposed rendered color, orientation and alpha semantics, so that Ferrixel-side parsing such as `avif_meta.rs` becomes unnecessary or is cross-checked.
4. **License gate.** Strong copyleft stays blocked by `scripts/audit-licenses.py`. MPL-2.0 `avif-parse` is a recorded review item, not an allowlist entry. The AGPL-3.0/commercial `jxl-encoder` remains unapproved; JPEG XL output needs its own ADR. The AGPL-3.0/commercial `zenjpeg` and `zenwebp` are unapproved for the same reason.
5. **MSRV.** Declare `rust-version = 1.98` for the single crate for now, and keep the MSRV CI job. Do not split crates only to lower the core MSRV before a consumer needs it (ADR-0008).
6. **Lockfile.** Commit `Cargo.lock`. The package ships a binary, CI and benchmarks need reproducible dependency versions, and the license inventory must refer to an exact version set.

## Consequences

+ The default build stays small, safe and dependency-free; each codec can be audited, replaced or dropped independently.
+ Color correctness does not depend on a crate's undocumented conversion choices.
+ A committed lockfile makes CI, license inventory and benchmark results reproducible.
- Library users of the codec-free core still inherit Rust 1.98.
- AVIF input pulls unsafe and assembly code plus an MPL-2.0 dependency into `--features cli` builds.
- AVIF input is parsed three times (`avif-parse` preflight, Ferrixel metadata, `avif-decode`); acceptable for still images, a cost to remove when the decoder is replaced.
- The lockfile needs regular updates (Renovate or manual) to pick up fixes.

## Alternatives considered

- **`libavif` or `dav1d` through FFI:** mature and complete, but adds a native toolchain and an unsafe boundary to the core build; reconsider only if pure-Rust decoders cannot meet correctness or coverage.
- **`rav1d-safe` now:** safer than `avif-decode`, but needs its own container assembly and dependency review; kept as a fallback behind the sister ports.
- **Wait for the sister ports before any AVIF/JXL support:** would block M1 on port work (safe API, stable toolchain, encoder) that has its own schedule.
- **Accept `avif-decode`'s color handling as is:** rejected; it silently ignores container color and orientation.
- **Split into `ferrixel-core` and codec crates now:** would lower the core MSRV, but ADR-0008 asks for a demonstrated need first.
- **Keep `Cargo.lock` uncommitted:** conventional for libraries years ago, but current Cargo guidance and the binary target favor committing it.
- **`image-webp` as an interim WebP decoder:** permissive and safe, but without lossy encoding or decode-side cropping, and the libwebp-rs decoder is the first port milestone; deferred unless WebP input is needed earlier.
- **libwebp through FFI (`webpx`):** rejected for the same reasons as libavif through FFI.
- **A JPEG encoder now (`mozjpeg-rs`, BSD-3-Clause, pure Rust):** deferred; M1 output is AVIF and no consumer needs JPEG output. It is the recorded candidate if that changes.

## Validation

- Feature-gated tests per adapter, including real ravif output with and without `colr`, contradictory `nclx`, PQ/HLG classification and the strict AVIF → AVIF facade path.
- CI: all-features tests and clippy, each feature alone, MSRV job, fuzz smoke and the license metadata screen.

Before **Accepted**: complete transitive license and NOTICE review including MPL source obligations; review of native and unsafe code in `rav1d` and `rav1e`; malformed AVIF/ICC corpus; and a dated plan for replacing the interim crates with libavif-rs and libjxl-rs.

## Review when

libavif-rs or libjxl-rs meets the adoption gates (safe API, stable toolchain, pinnable release); the libjxl-rs encoder lands; `rav1d-safe` or another permissive decoder exposes planar output and container metadata; `avif-decode` lowers its MSRV or exposes color properties; a consumer needs an older MSRV for the core; a dependency changes license; the license review blocks MPL-2.0 distribution; the libwebp-rs decoder passes its parity gates; or a consumer needs JPEG output or WebP input before libwebp-rs is adoptable.

## Amendment log

- 2026-11-05 — Recorded the codec/CMS dependencies, MSRV and lockfile policy introduced with PR #11 and the AVIF color fix.
- 2026-11-05 — Made libavif-rs and libjxl-rs the replacement target for all AVIF/JPEG XL codecs, defined adoption gates and demoted `rav1d-safe` to a fallback.
- 2026-11-05 — Recorded JPEG as input-only, the planned libwebp-rs port as the WebP target with no interim WebP dependency, and `zenjpeg`/`zenwebp` as blocked by the license gate.
- 2026-11-06 — `avif-write` now enables `ravif`'s `asm` and `threading` features (M2 work package 2). `rav1e`'s assembly is bit-identical to its Rust fallbacks on all five images of the [encoder experiment](../research/encoder-experiment/results/2026-11-06-apple-m1-ultra.md) and encodes 2.1–2.4× faster on Apple Silicon. Consequences: the output encoder now links hand-written assembly (compiled with `cc` on aarch64 and NASM on x86-64, so `avif-write` builds need NASM on x86-64 like `avif-read` already did), and `rayon` becomes a runtime dependency of the encoder. No new crate enters the lockfile; `rayon`, `cc` and `nasm-rs` were already present.
- 2026-11-06 — libavif-rs admitted GNU Linux x86-64 (`f3b2981`), which covers Ferrixel's CI target. Gate (d) is now satisfiable for AVIF; gates (a) safe API and (b) stable toolchain remain open, and the libvmaf build requirement is an additional obstacle.
- 2026-11-06 — `avif-write` drives `rav1e` directly through Ferrixel's own sink and writes the container with `avif-serialize`; `ravif` remains only as a dev-dependency for one test fixture. Reason: `ravif` derives AV1 tiles from the thread count and supports neither 4:2:0 nor tile control. The sink keeps `ravif`'s quantizer curve and speed overrides (`src/av1_tuning.rs`, BSD-3-Clause notice retained) because the encoder experiment found `rav1e`'s plain presets 1.6–2× slower at equal SSIMULACRA2; with them the direct path produces byte-identical output to `ravif`.
- 2026-11-06 — `jpeg-read` enables `zune-jpeg`'s `neon` and `x86` features (ADR-0023): runtime-detected SIMD IDCT, upsampling and color conversion inside the decoder, with the decoder's own `unsafe`. Measured on the Apple M1 Ultra: the decode stage of the synthetic 24 and 48 Mpx JPEGs fell by about 20 %. The `icc` adapter now asks moxcms for linear-light sRGB output.
