# ADR-0045: Recipe API: one source, several crops, several outputs

- **Status:** Proposed (owner design session, 2026-11-24 to 2026-11-26)
- **Date:** 2026-11-26
- **Last reviewed:** 2026-11-26
- **Supersedes:** None yet. If accepted, it replaces the call shape of [0017](0017-minimal-public-facade-and-cli.md) (`transcode(input, options)`) and the API inventory of [0040](0040-stable-public-contract-and-compatibility-policy.md) for the items it covers.
- **Related:** [0012](0012-codec-source-contract.md), [0022](0022-parallel-row-pipeline-and-engine.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0034](0034-thumbnail-geometry.md), [0040](0040-stable-public-contract-and-compatibility-policy.md), tracker #271, [research notes](../research/2026-11-quality-and-recipe-api.md)

## Context

Ferrisma ships 0.x pre-releases first (owner, 2026-11-24); the API may still change between them.
This record describes the API the owner wants, not the current one.

Who calls Ferrisma (owner):

- build tools that render and cache images ahead of time (the static-site generator Ardo, a Vite plugin through the npm package);
- image servers behind a CDN that render thumbnails and crops on request;
- a command line for scripts.

All three matter equally. A typical build-tool request is responsive images. One photo is cut to 16:9 and to 4:3 around a face. Each crop is then needed in several widths and formats for `srcset`.

Constraints the owner set:

- The work is described first and run at the end, as in sharp. That is the only way the engine can move data between its stages without handing intermediate copies back to the caller.
- The default interface uses simple, understandable settings. Every encoder-specific setting stays reachable, but through configuration, not through CLI flags or wrapper options.
- Streaming is not needed now. Nothing in the contract may block it later.
- Face and object detection are not Ferrisma's job. Focus regions come from the caller or from the image's metadata.
- A server and its URL scheme are not in scope. They sit on top.

## Decision

### 1. The recipe is the contract

A **recipe** is a serializable value (serde): a tree with one source, one or more **variants** (crops), and one or more **targets** per variant (width × format × quality).
`engine.run(source, &recipe)` executes it and returns one result per target.

```rust
let recipe = Recipe::new()
    .variant(Crop::aspect(16, 9).focus(Focus::Metadata))
        .layout(Layout::css(480).densities([1, 2, 3]))
        .formats([Format::Avif, Format::Webp])
        .quality(Quality::High)
    .variant(Crop::aspect(4, 3).focus(Focus::Metadata))
        .widths([320, 640, 960])
        .formats([Format::Avif, Format::Webp]);
let outputs = engine.run(Source::path("hero.jpg"), &recipe)?;
```

The Rust methods above build that recipe; they are sugar, not a second API. The Node binding, the CLI and configuration files map onto the same recipe:

```js
await ferrisma("hero.jpg").resize(800).avif({ quality: "high" }).toFile("hero.avif");
```

### 2. The planner owns execution

The planner sees all targets at once.

- It decodes the source **once**, and only the region the variants need together (ADR-0012).
- Each target is resized **from the source** by default. Cascading from a larger intermediate is faster but changes the output slightly, so it is an opt-in.
- How data moves between stages is the engine's business, not the API's: no intermediate result is returned to the caller.

### 3. Sources and sinks

- `Source` today: borrowed bytes (`&[u8]`) or a path. A path may be memory-mapped.
- Each target writes into a `Sink`: a `Vec<u8>`, a file, or any `Write`.
- Streaming arrives later as further `Source` and `Sink` kinds without changing a signature.
- No stable report field, error kind or guarantee may assume a whole-frame decoder (ADR-0040 problem 16 applies unchanged).

### 4. Focus

`Crop::aspect(w, h)` computes the largest rectangle of that ratio that contains the focus. The focus is chosen in this order:

1. an explicit point or box in the recipe;
2. `Focus::Metadata`: an IPTC Image Region or an MWG region (face, focus) read from the image's XMP;
3. the center.

There is no sidecar format (owner, 2026-11-26). A separate tool, for example a face or object detector, writes the region into the image's XMP, and Ferrisma reads it from there.
The recipe records which focus was used in the report.

### 5. One quality scale

`Quality` is common to all output formats: `Low`, `Medium`, `High`, `Lossless`, and possibly a number 0–100.

- **Meaning.** Each level is a **target SSIMULACRA 2 score**. The working assumption follows the metric's own bands: about 90 for visually lossless, 70 for high, 50 for medium, 30 for low.
- **Mapping.** A table maps each level to native encoder settings per encoder. It is measured once on a corpus with separate image classes (photos, graphics, text, skin, gradients) and committed together with its measurement script.
- **Determinism.** The table is fixed, so the same recipe gives the same bytes on every platform.
- **Precedent.** No mainstream tool offers such a scale: sharp, imgproxy and libvips take one number per encoder. A study by Wagtail on CID22 shows that the offsets between formats are not constant, so the table is measured per class, not derived.
- **Later, opt-in.** `Quality::Auto(level)` searches the target score per image, as imgproxy's autoquality and Cloudinary's `q_auto` do. It costs several encodes per image.

### 6. Quality per size and screen density

A larger image for a denser screen can use a lower encoder quality ("compressive images": Filament Group 2012, web.dev, Jake Archibald 2021). The recipe offers two levels:

- **Explicit.** `quality_for(width, Quality)` overrides the quality of one target.
- **By intent.** `Layout::css(480).densities([1, 2, 3])` declares the displayed width and the densities, and produces 480, 960 and 1440 pixel targets. `Quality` then means the quality **as displayed**. For a 2x or 3x target the engine chooses the encoder setting whose SSIMULACRA 2 score, measured after downscaling to the displayed size, reaches the level. This mapping is part of the measured table of decision 5.

The intent matters because a 1440-pixel image may be meant for a dense phone screen or for a large monitor at 1x. Only the caller knows which, and the recipe says it. Plain `widths([...])` keeps the quality as stated.

### 7. Encoder-specific settings

- Every native setting of the ported encoders is reachable as a typed struct per encoder: `[encoder.avif]`, `[encoder.webp]`, `[encoder.jxl]` in a configuration file, or the same structs in Rust.
- These settings override what `Quality` chose and have their own, experimental tier.
- The CLI takes them only through `--config <file>`, never as flags.

### 8. Recipes from outside

The engine provides what a server, and Ardo's cache, need without becoming a server:

- **A canonical form and a stable hash.** The same recipe always gives the same key, for caches and CDNs.
- **Validation before any work.** A recipe is checked against limits (number of targets, output pixels, memory budget) and rejected before it costs anything.
- **Serde formats.** A URL scheme can be built on top without touching the engine.

## Consequences

+ One contract for Rust, Node, the CLI and configuration files. A server can store recipes as data.
+ Decoding once for many targets is the largest saving for responsive images.
+ Stating quality as displayed makes high-density output smaller without the caller tuning numbers.
- The quality table is work, and it must be measured again when an encoder port changes its defaults.
- The current `transcode`/`TransformOptions` facade, CLI flags and tests are reshaped. 0.x allows this; the migration is planned per pre-release.
- Calibrating against SSIMULACRA 2 ties the meaning of `Quality` to that metric. The metric and the CID22 dataset both come from Cloudinary, and CID22 covers medium to near-lossless quality only.

## Alternatives considered

- **A fluent pipeline only, as in sharp.** Familiar, but each binding rebuilds the chain, and one source with many crops and outputs does not fit a linear chain. It is kept as sugar over the recipe.
- **A quality scale per format (sharp, imgproxy, libvips).** Honest about the codecs, but the caller must learn and tune every scale, and the owner wants one scale. The native scales remain reachable through decision 7.
- **DSSIM or Butteraugli as the calibration metric.** DSSIM is AGPL or commercial, and Butteraugli's repository is archived. On CID22 both correlate slightly worse with human ratings than SSIMULACRA 2. Either may still be reported.
- **Face detection in the engine.** Rejected: it is persistent knowledge about the picture and belongs in its metadata.
- **A URL scheme in the engine.** Deferred to whoever builds a server; decision 8 is the base for it.

## Validation

- A prototype runs the example of decision 1 and decodes the source exactly once (decoder work counted in the report). Twelve targets match twelve separate runs bit for bit when cascading is off.
- The quality table is measured with a committed script on a committed corpus. Each level reaches its SSIMULACRA 2 target within a stated spread per image class.
- For densities 2 and 3, the downscaled score reaches the level, and the file is smaller than the 1x target at the same level. Otherwise decision 6 is revised.
- A recipe's hash is stable across platforms and across equal recipes written in different field orders.

## Review when

- After the first pre-release that ships the recipe, with feedback from Ardo and the Vite plugin.
- When an encoder port changes its defaults (remeasure the table).
- When streaming (#197, #198) or ROI decode (#60) is scheduled.

## Amendment log

- 2026-11-26 — Initial proposal from the owner's design session.
- 2026-11-26 — Owner review: no JSON sidecar for focus, regions live in XMP only; SSIMULACRA 2 as the calibration metric and quality as displayed for dense screens (decisions 5 and 6) confirmed.
