import { defineArdoConfig } from "ardo/config";

export default defineArdoConfig({
  title: "Ferrisma",
  description:
    "One image pipeline for Rust, every codec included: JPEG, WebP, JPEG XL and AVIF through one API, built by Cargo alone. Experimental.",
  base: "/",
  siteUrl: "https://ferrisma.dev",
  icons: false,
  markdown: { headingAttributes: true },
  linkCheck: { level: "error" },
});
