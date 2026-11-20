# Homepage design notes

The first complete version of an overview page for Ferrisma, on the
Ferramenta family's page system. It is a version for the owner to react to,
not a final one. This note covers the arc, what each drawing is for, the copy
decisions, what is provisional, and the open questions.

## The design system: Monografie, not the 2.x landing kit

The brief for this page named the family's 2.x landing kit (`ProjectHero`,
`Plate`, steel, oak and rust, `tokens.css` … `chrome.css`). That kit is gone:
ferramenta-family 3.0 removed it, ADR-0012 in the Ferramenta repository makes
Monografie the one page system for every engine homepage and its
documentation, and the package this site already pins
(`70d696962356af5a100c970f0f4b0187fdafcccf`) carries only Monografie. The page
is therefore built on Monografie, as the kit's sample engine page shows it:
the instrument as the opening plate, one reading column, margin notes, wide
specimens on light sheets, oxide as the one accent, rust only on the primary
action. The lockfile and the pin are unchanged.

## The arc

1. **Opening.** The instrument, then what Ferrisma is and does in plain
   words: it reads four formats, turns, crops and resizes in linear light, and
   writes three; its codecs are translations of the originals. Who it is for.
   The facts row names inputs, outputs, the standard (bit for bit) and the
   status (experimental, no release).
2. **The pipeline** — *One pass from file to file.* How one image travels,
   with the report beside the file. Four short terms: depth follows the input,
   the memory budget, panics as errors, parallel and batch work.
3. **The ports** — *Four codecs, translated from the originals.* The lineage
   in ADR-0004's register: honor the originals, state that each port is a
   translation at a pinned version, give the why (memory safety, one
   toolchain, no C compiler). Then the anatomy drawing, and the plain truth on
   assembly (the rule, the scalar fallback, the six remaining sites).
4. **The gate** — *The same bytes as the original, or it is not done.* What
   bit-identical means and how it is proven, the gate drawing, and where each
   port stands, dated, with the honest sentence that no port meets the whole
   gate on both targets yet.
5. **Refuse or report** — *Nothing disappears quietly.* The three outcomes,
   the decision drawing, and the rule that formats behave alike.
6. **Speed** — *Measured against the originals, gaps included.* A table of
   port time against the original's SIMD build, one thread, with the two far
   gaps kept in and the AVIF/dav1d gap said in words.
7. **Status** — *Experimental, and plain about it.* What works today, what is
   not there yet, the first command, what comes next.
8. Colophon (the family, licenses, the instrument) and the closing: *Build it.
   Check it. Tell us what breaks.*

The order moves from what it does (pipeline), to what it is made of (ports),
to why you can trust it (gate, refuse or report), to what it costs (speed),
to what you can do today (status). The gate is the story; it sits in the
middle, where the first two sections have earned the reader's attention.

## The drawings

All four are inline SVG components in `app/drawings/`, drawn from data, in
two layouts each: a wide one (from 1000px) and a narrow one, so labels stay
13–17px instead of shrinking with the viewBox. Every color is a `--mono-`
token (`drawings.css`); oxide marks the one element each drawing is about.
Nothing moves. No drawing shows a figure that is not in a committed record,
and none draws the family's members.

- **The pipeline in cross-section** (`Pipeline.tsx`). Four files enter one
  channel; five stations (decode, linear light, geometry, quantize, encode)
  under one memory-budget bracket; the image and the report (oxide) leave
  side by side. It shows what text cannot: that the route is the same for
  every format and that the report is a product of the run, not a log.
- **The anatomy** (`Anatomy.tsx`). Your code, the engine (no unsafe code)
  with its kernels, the safe bridges, the four ports with what each
  translates, the shared SharpYUV core, the originals standing outside as
  references only, and a dashed box of what is absent by design (a C compiler
  in the build, assembly kernels, the originals' code in a package).
- **The quality gate** (`Gate.tsx`). One input into both sides, per target,
  build kind against build kind (scalar/scalar, NEON/NEON, AVX2/AVX2), each
  pair meeting in an oxide ring. Under it, the comparison rules and "no speed
  gate".
- **Refuse or report** (`RefuseOrReport.tsx`). Two questions, three outcomes,
  the report outcome in oxide, and "There is no fourth outcome."

The speed comparison is a table, not a drawing: the figures carry the
argument, and a table keeps them exact and accessible.

## Copy decisions

- Say what it is first: the h1 names the job ("Read, crop, resize and write
  images in Rust"); the italic second line names the difference ("on codecs
  translated from the originals, and held to them bit for bit").
- The originals are named in prose (libjpeg-turbo, libwebp, libjxl, libavif
  with libaom); today's crate names appear nowhere in the copy. All port facts,
  including the planned package names, live in `app/ferrisma.ts`.
- "Held to them bit for bit" describes the standard, not a finished result;
  the gate section says plainly that no port meets the whole gate yet.
- Figures appear only in the speed table and come from two committed records,
  named and linked with their dates. The AVIF decode gap against libavif with
  dav1d is said in words only: its factor is in `docs/status.md` and the ADRs,
  but not in a record under `docs/benchmarks/`.
- The brief's "roughly 1.0 to 1.3 times in most cells" is not what the record
  says; the page says "between one and one and a half times" for most
  operations, which the table supports, and names the two far gaps.
- No social proof, no superlatives, no "fast". The family register: honor the
  original, state the succession, give the why.

## Chrome without a registry entry

`MonoHeader`, `MonoFooter`, `MonoOpening` and `MonoArdoSite` take a
registered identity (`product`), and the pinned package has no `ferrisma`
identity. `app/site/chrome.tsx` renders the same markup and classes as the
package's header and footer, with the menu behavior copied from
`MonoChrome.tsx`, and the site-local mark (`.fx-mark`). The family link uses
the registered `ferramenta` mark. The opening uses Monografie's markup with
`public/ferrisma-instrument.webp`. `root.tsx` follows `MonoArdoSite`'s
structure: the header in Ardo's header slot, Ardo's menu and search on
documentation pages (with the header's links in the menu), Monografie's prose
as the MDX wrapper, the footer outside Ardo's main.

What switches over when the registry has a `ferrisma` entry with its mark and
instrument (and the pin is bumped to a commit that has it):

- `root.tsx` becomes `MonoArdoLayout` + `MonoArdoSite product="ferrisma"`
  with the same links and sections; `app/site/chrome.tsx` and the `.fx-mark`
  rule are deleted.
- The opening becomes `MonoOpening product="ferrisma"`; the local
  `public/ferrisma-instrument.webp` and the favicons in `app/assets/` give way
  to the package's `ferra-assets/ferrisma/*`.
- `NAME`, `REPOSITORY` and the status line can be read from the registry.

Note for the owner: the Ferramenta repository's `main` already registers this
project under its former name, `ferrixel` (status `early`, version `0.1.0`,
with a mark and an instrument), in a commit after the pinned one. It will need
the rename before this site can switch over.

## What is provisional

- **No member icon, no registered mark.** The header mark and the instrument
  are the site-local renders in `artwork/`.
- **No registry entry** (see above).
- **Figures that will move.** The speed table is a snapshot of the records of
  November 13 and 14, 2026; the JPEG XL rows predate the encoder's SIMD work;
  there is no x86-64 JPEG XL decode figure in a record yet, and the JPEG XL
  lossy encode row predates the encoder's SIMD default of November 16.
- **The gate state** is generated at build time from the table "Where each
  port stands (date)" in `docs/quality-gate.md` (its column "In short"),
  by `scripts/gate-state.ts`; the build fails if the table is missing or
  changes shape. Update the guide, and the homepage follows.
- **Crate names** of the ports will change (`plannedCrate` in
  `app/ferrisma.ts`).
- The ferrisma.dev domain has served this site since 2026-11-20 (before that it
  answered with a GitHub Pages page that was not ours, #251). The Homepage
  workflow deploys `main` to GitHub Pages; the custom domain is set in the
  repository's Pages settings.

## Two guide pages added

`docs/quality-gate.md` and `docs/supported-matrix.md` are now synchronized
into the guide (`/guide/quality-gate`, `/guide/supported-matrix`), so the
homepage can link its central claims to pages on the site. `sync-docs.ts`
now rewrites a GitHub-style anchor into a guide page to the anchor Ardo
renders (`#…-v10` becomes `#…-v1-0`); a test covers it.

## Open questions for the owner

1. **The headline.** "Read, crop, resize and write images in Rust" is plain
   and says the job; the gate lives in the second line. Should the gate lead
   instead ("Image codecs that are the originals, in Rust")?
2. **How loud the experimental status is.** Today: the header tag, the last
   fact in the opening, and a whole status section. Enough, or a short notice
   above the fold?
3. **Which figures to show.** All ten rows, or only the decoders and the
   lossless paths, with the encoder gaps in words? Should the AVIF/dav1d
   factor get a record under `docs/benchmarks/` so the page can print it?
4. **The gate state per port.** Keep a dated table on the homepage, or link
   to the quality gate guide only, so the homepage cannot go stale?
5. **The six inline assembly sites.** The page names them. Keep that candor on
   the homepage, or leave it to ADR-0044?

## Decisions of round two (2026-11-16)

The owner approved the direction. Working answers to the questions above:
the headline stays; "experimental" stays as prominent as it is until a 0.x
exists; the speed table shows every operation, gaps included, and no dav1d
comparison until its kernels are complete on both targets; the dated
per-port gate state stays, with the link to the guide; the sentence about the
remaining assembly stays as long as it is true.

## Round three (2026-11-16)

- **How one port is made** (`/how-a-port-is-made`, `app/routes/how-a-port-is-made.tsx`):
  one page and one drawing (`PortMaking.tsx`, two lanes, the original above
  and the port below, six steps), with the JPEG XL decoder as the worked
  example. Every statement comes from libjxl-rs's `upstream.json`,
  `PROVENANCE.md`, ADR 0001 and ADR 0004, and the quality gate guide. It is
  linked from the ports section and the guide navigation. Who does the
  translating (people, tools, agents) is not documented in the repository,
  so the page does not say.
- **Gate state from data**: see "What is provisional" above.

## Round six (2026-11-17): the new speed record

The speed summary on the homepage and the full table on `/how-it-is-built`
now come from `docs/benchmarks/v1.0-three-column-2026-11-17.md` (run
43978421716, `main` at `f4be1596`): column (b) 0.91 to 1.63 over 104 cells,
91 at or below 1.35; the slowest WebP lossy encode on x86-64 and JPEG XL
lossless decode on Linux ARM64. The dav1d context columns stay in the record
only.

## Round five (2026-11-17): the headline

The owner chose candidate B: "One image pipeline for Rust, every codec
included", second line "JPEG, WebP, JPEG XL and AVIF through one API, built
by Cargo alone". It is in the page title, the meta description and the site
description in `ardo.config.ts`, which Ardo also writes into `llms.txt`. The
site has no Open Graph tags. The lead no longer repeats the formats and "one
API": it says what the headline does not (crop and resize, the sharp
comparison, no C library underneath, the originals' pictures), and the first
section's standfirst starts from the outputs.

## Round four (2026-11-16): the benefit first

Owner feedback: the port is a good technical story but belongs on a sub-page;
the homepage leads with what a user gets. The homepage now follows his order:

1. **Opening.** "Image processing for Rust, written in Rust", second line
   "down to the codecs, with the originals' pictures". The lead names the
   job and compares with sharp only in what is true of both (one interface for
   reading, resizing and writing the web's formats); it says there is no C
   library underneath, and the status block says there is no Node.js binding.
2. **One pipeline for every format** (`#use`): the thumbnail call from
   `examples/thumbnail.rs` (verbatim minus argument parsing; CI compiles every
   example with `--all-targets --all-features`), the CLI line from
   `docs/getting-started.md`, then the pipeline drawing.
3. **All Rust, one build** (`#rust`) with a new drawing (`OneBuild.tsx`):
   binding the C and C++ originals (compilers, CMake, NASM) beside Ferrisma
   (Cargo). Targets verified today: macOS ARM64 and GNU Linux x86-64.
4. **Memory safety where the files come in** (`#safety`): exactly what
   `docs/unsafe-review.md` backs, including that the JPEG XL and AVIF ports
   still carry much unsafe code behind documented bridges. No claim that the
   stack is memory-safe; no statistics.
5. **The same pictures as the originals** (`#originals`): the port work as a
   benefit, three sentences and links.
6. **You know what happened to every image** (`#report`): four terms.
7. **The cost: close to the originals' speed** (`#speed`): the range of the
   committed record and the named outliers, with a link to the full table.
   The owner's "10 to 20 percent" is an expectation, not a measurement, and is
   not printed.
8. Status, colophon, closing.

The depth moved to `/how-it-is-built` (the anatomy with the originals, the
gate drawing and per-port state, refuse or report, the full speed table) and
`/how-a-port-is-made`. The header's "Quality gate" link became "How it is
built"; the guide navigation lists both pages.

Headline candidates considered:
- "Image processing for Rust, written in Rust" / "down to the codecs, with the
  originals' pictures" (built).
- "One image pipeline for Rust, every codec included" / "JPEG, WebP, JPEG XL
  and AVIF through one API, built by Cargo alone".
- "Read, resize and write images in pure Rust" / "the originals' codecs,
  translated and held to them bit for bit".
