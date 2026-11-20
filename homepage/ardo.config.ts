import { defineArdoConfig } from "ardo/config";

export default defineArdoConfig({
  title: "Ferrisma",
  description:
    "One image pipeline for Rust, every codec included: JPEG, WebP, JPEG XL and AVIF through one API, built by Cargo alone. Experimental.",
  base: "/",
  // GitHub Pages serves each page as a folder (`/guide/status/index.html`) and
  // redirects the spelling without the slash; links, canonical URLs and the
  // sitemap use the served spelling.
  trailingSlash: true,
  siteUrl: "https://ferrisma.dev",
  icons: false,
  markdown: { headingAttributes: true },
  linkCheck: { level: "error" },
});
