/*
 * The homepage's facts in one place. Everything the page states about the
 * ports, their originals, the gate and the measurements comes from here, and
 * every entry names the repository document it was copied from. Update the
 * document first, then this file.
 *
 * Until Ferrisma has an entry in the Ferramenta registry
 * (`ferramenta-family/registry`), its name, links and status live here too;
 * see DESIGN-NOTES.md for what moves to the registry then.
 */

export const NAME = "Ferrisma";
export const REPOSITORY = "https://github.com/sebastian-software/ferrisma";
export const FAMILY_SITE = "https://ferramenta.dev";

/**
 * A path inside the site, below Vite's base. Pages end with a slash, as GitHub
 * Pages serves them (a folder with an `index.html`); files keep their name.
 */
export const path = (route: string) => {
  const [page = "", anchor] = route.split("#");
  const slash = page === "" || page.endsWith("/") || /\.[a-z0-9]+$/i.test(page) ? "" : "/";
  return `${import.meta.env.BASE_URL}${page}${slash}${anchor === undefined ? "" : `#${anchor}`}`;
};
/** A file in the repository on GitHub, at `main`. */
export const blob = (file: string) => `${REPOSITORY}/blob/main/${file}`;

/*
 * The four codec ports. The prose names the originals; today's crate names
 * are kept only here (`crate`), because they are going to change: the ports
 * will be published as `ferrisma-libjpeg-turbo`, `ferrisma-libwebp`,
 * `ferrisma-libjxl` and `ferrisma-libavif` (`plannedCrate`). Switch `crate`
 * over when they are renamed in the repository.
 *
 * Sources: the ports' READMEs (versions; libavif's from
 * codecs/libavif-rs/UPSTREAM.json), libjxl's revision from
 * docs/benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md, libaom's from
 * docs/adr/0044-no-assembly-acceleration-through-rust-intrinsics.md.
 */
export type Port = {
  format: string;
  original: string;
  /** The pinned upstream release or revision the translation follows. */
  pinned: string;
  /** The reference build's name and version, as the drawings list it. */
  reference: string;
  /** What the original is, in the words of the port's README. */
  originalIs: string;
  /** How the engine reaches the port. */
  bridge: string;
  /** What the engine uses it for today (docs/status.md). */
  role: string;
  crate: string;
  plannedCrate: string;
  docs: string;
};

export const PORTS: Port[] = [
  {
    format: "JPEG",
    original: "libjpeg-turbo",
    pinned: "3.2.1",
    reference: "libjpeg-turbo 3.2.1",
    originalIs: "the SIMD-accelerated descendant of the Independent JPEG Group's libjpeg",
    bridge: "safe scanline API",
    role: "reads JPEG",
    crate: "libjpeg-turbo-rs",
    plannedCrate: "ferrisma-libjpeg-turbo",
    docs: "codecs/jpeg",
  },
  {
    format: "WebP",
    original: "libwebp",
    pinned: "1.6.0",
    reference: "libwebp 1.6.0",
    originalIs: "the WebP reference implementation",
    bridge: "safe decode and encode API",
    role: "reads and writes still WebP",
    crate: "libwebp-rs",
    plannedCrate: "ferrisma-libwebp",
    docs: "codecs/webp",
  },
  {
    format: "JPEG XL",
    original: "libjxl",
    pinned: "a16d32d0",
    reference: "libjxl a16d32d0",
    originalIs: "the JPEG XL reference implementation",
    bridge: "safe decoder and encoder wrappers",
    role: "reads and writes JPEG XL",
    crate: "libjxl-rs",
    plannedCrate: "ferrisma-libjxl",
    docs: "codecs/jpeg-xl",
  },
  {
    format: "AVIF",
    original: "libavif with libaom",
    pinned: "1.4.2 and 3.15.2",
    reference: "libavif 1.4.2, libaom 3.15.2",
    originalIs: "the AVIF library and the AV1 codec it encodes and decodes with",
    bridge: "safe read and write wrappers",
    role: "reads and writes AVIF",
    crate: "libavif-rs",
    plannedCrate: "ferrisma-libavif",
    docs: "codecs/avif",
  },
];

/*
 * Where each port stands against the gate is not kept here: it is generated
 * at build time from the table "Where each port stands" in
 * docs/quality-gate.md (scripts/gate-state.ts → app/gate-state.generated.ts).
 */

/*
 * Speed against the originals: port time ÷ the original's SIMD build, one
 * thread (column (b)). Above 1 the port is slower.
 *
 * Source: docs/benchmarks/v1.0-three-column-2026-11-17.md, summary table,
 * 12 MP inputs, GitHub-hosted ubuntu-24.04 (x86-64) and macos-15 (ARM64),
 * run 43978421716 on main at f4be1596.
 */
export type SpeedRow = {
  operation: string;
  x86: string;
  arm: string;
  note?: string;
};

export const SPEED_RECORD = {
  title: "Three-column record of November 17, 2026",
  href: blob("docs/benchmarks/v1.0-three-column-2026-11-17.md"),
};

/** The range and the outliers the record states, for the homepage. */
export const SPEED_SUMMARY = {
  cells: 104,
  machines: "five",
  min: "0.91",
  max: "1.63",
  atOrBelow: "1.35",
  countAtOrBelow: 91,
};

export const SPEED: SpeedRow[] = [
  { operation: "JPEG decode", x86: "1.01×", arm: "0.91×" },
  { operation: "WebP lossy decode", x86: "1.35×", arm: "1.33×" },
  { operation: "WebP lossless decode", x86: "1.28×", arm: "1.22×" },
  { operation: "WebP lossless encode", x86: "1.17×", arm: "1.08×" },
  { operation: "WebP lossy encode", x86: "1.42×", arm: "1.16×" },
  { operation: "JPEG XL decode (VarDCT)", x86: "1.23×", arm: "1.17×" },
  { operation: "JPEG XL lossless decode", x86: "1.25×", arm: "1.39×" },
  { operation: "JPEG XL lossless encode", x86: "1.13×", arm: "1.18×" },
  { operation: "JPEG XL lossy encode", x86: "1.31×", arm: "0.99×" },
  { operation: "AVIF decode through libaom", x86: "1.17×", arm: "1.33×" },
  { operation: "AVIF encode, 10-bit", x86: "1.25×", arm: "1.40×" },
];
