import { defineArdoConfig } from "ardo/config";

export default defineArdoConfig({
  title: "Ferrisma",
  description: "A Rust image engine in development for modern web delivery.",
  base: "/",
  siteUrl: "https://ferrisma.dev",
  icons: false,
  markdown: { headingAttributes: true },
  linkCheck: { level: "error" },
});
