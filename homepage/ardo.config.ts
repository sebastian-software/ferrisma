import { defineArdoConfig } from "ardo/config";

export default defineArdoConfig({
  title: "Ferrixel",
  description: "A Rust image engine in development for modern web delivery.",
  base: "/ferrixel/",
  siteUrl: "https://sebastian-software.github.io",
  icons: false,
  markdown: { headingAttributes: true },
  linkCheck: { level: "error" },
});
