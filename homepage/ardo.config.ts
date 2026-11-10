import { defineArdoConfig } from "ardo/config";

export default defineArdoConfig({
  title: "Ferrixel",
  description: "An experimental image pipeline in Rust.",
  base: "/ferrixel/",
  siteUrl: "https://sebastian-software.github.io",
  icons: false,
  markdown: { headingAttributes: true },
  linkCheck: { level: "error" },
});
