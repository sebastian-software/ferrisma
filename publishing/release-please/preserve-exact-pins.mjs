// Restore the exact (`=`) requirements of `ferrisma` on its internal crates
// after Release Please rewrote them (inert draft, docs/release-automation.md).
//
// `ferrisma` requires its kernels and its four bridges with `=`: they are
// internal to the engine and follow no SemVer of their own (ADR-0040 problem
// 7, amendment of 2026-11-16). Release Please's Rust updater writes the new
// version without the `=`, so the publish workflow runs this script on the
// release branch after Release Please has updated it, and pushes the
// correction when it changed anything. Precedent: sebastian-software/ferromark
// `scripts/preserve-exact-transform-pin.mjs`. The rehearsal (`rehearse.mjs`)
// applies the same function to every proposed release and checks the result.
//
// Usage, from the repository root:
//
//   node publishing/release-please/preserve-exact-pins.mjs
//
// Exit status 0: the pins are exact (the root Cargo.toml was rewritten if
// needed); 1: a pinned dependency is missing, or its version disagrees with
// the package it names.

import { readFileSync, writeFileSync } from "node:fs";
import { posix, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** The dependencies of the root `ferrisma` package that must stay exact. */
export const EXACT_PINS = {
  "ferrisma-kernels": "crates/ferrisma-kernels",
  "ferrisma-avif-read": "crates/ferrisma-avif-read",
  "ferrisma-avif-write": "crates/ferrisma-avif-write",
  "ferrisma-jxl-decoder": "crates/ferrisma-jxl-decoder",
  "ferrisma-jxl-encoder": "crates/ferrisma-jxl-encoder",
};

const escape = (text) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

function packageVersion(manifest, path) {
  const version = manifest.match(/^\[package\][\s\S]*?^version = "([^"]+)"$/m)?.[1];
  if (!version) throw new Error(`no [package] version in ${path}/Cargo.toml`);
  return version;
}

/**
 * Return the engine manifest with every pin of {@link EXACT_PINS} written as
 * `=<version of the named package>`. `read(path)` returns a manifest's text
 * relative to the repository root; `engineDir` is the engine package's
 * directory (`""`, the root, today; the rehearsal of configuration B moves it
 * to `crates/ferrisma`). Throws when a pin is missing or names a version
 * other than its package's (a stale requirement is not repaired).
 */
export function preserveExactPins(engineCargoToml, read, engineDir = "") {
  let content = engineCargoToml;
  const changed = [];
  for (const [name, path] of Object.entries(EXACT_PINS)) {
    const version = packageVersion(read(`${path}/Cargo.toml`), path);
    const relative = posix.relative(engineDir || ".", path);
    const pattern = new RegExp(
      `^(${escape(name)} = \\{ path = "${escape(relative)}", version = ")([^"]+)(")`,
      "m",
    );
    const match = content.match(pattern);
    if (!match) throw new Error(`no requirement on ${name} (path ${relative}) in the engine manifest`);
    const current = match[2];
    if (current.replace(/^=/, "") !== version) {
      throw new Error(`the engine requires ${name} ${current}, but the package is ${version}`);
    }
    if (current !== `=${version}`) {
      content = content.replace(pattern, `$1=${version}$3`);
      changed.push(name);
    }
  }
  return { content, changed };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const read = (path) => readFileSync(resolve(path), "utf8");
    const { content, changed } = preserveExactPins(read("Cargo.toml"), read);
    if (changed.length) writeFileSync(resolve("Cargo.toml"), content);
    console.log(changed.length ? `restored exact pins: ${changed.join(", ")}` : "exact pins intact");
  } catch (error) {
    console.error(`error: ${error.message}`);
    process.exit(1);
  }
}
