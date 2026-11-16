import { test } from "node:test";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { ardoSlug, githubSlug, render, rewriteTarget } from "./sync-docs.ts";

const source = resolve(import.meta.dirname, "../../docs/getting-started.md");
const github = "https://github.com/sebastian-software/ferrisma/blob/main/";

test("rewrites code-formatted link labels, images and references", () => {
  const input = '[`ferrisma`](../Cargo.toml) ![diagram](../README.md#part)\n\n[details][ref]\n\n[ref]: architecture.md#memory-and-scheduling\n';
  const output = render(source, input);
  assert.match(output, new RegExp(`\\[\\x60ferrisma\\x60\\]\\(${github}Cargo\\.toml\\)`));
  assert.ok(output.includes(`![diagram](${github}README.md#part)`));
  assert.ok(output.includes('[ref]: /guide/architecture#memory-and-scheduling'));
});

test("preserves fenced and inline code, anchors and external URLs", () => {
  const input = '`[code](missing.md)` [section](#part) [external](https://example.org/x)\n\n```md\n[example](missing.md)\n```\n';
  assert.equal(render(source, input), input);
});

test("fails on missing targets and sources outside the repository", () => {
  assert.throws(() => rewriteTarget(source, 'no-such-file.md'));
  assert.throws(() => rewriteTarget(source, '../../outside.md'));
});

test("rewrites GitHub anchors into guide pages to Ardo's anchors", () => {
  assert.equal(githubSlug("SIMD instruction-set levels (v1.0)"), "simd-instruction-set-levels-v10");
  assert.equal(ardoSlug("SIMD instruction-set levels (v1.0)"), "simd-instruction-set-levels-v1-0");
  assert.equal(
    rewriteTarget(source, "supported-matrix.md#simd-instruction-set-levels-v10"),
    "/guide/supported-matrix#simd-instruction-set-levels-v1-0",
  );
  assert.equal(
    rewriteTarget(source, "supported-matrix.md#minimum-rust-versions"),
    "/guide/supported-matrix#minimum-rust-versions",
  );
});

test("reads the homepage's gate state from the quality gate guide", async () => {
  const { parseGateState } = await import("./gate-state.ts");
  const { readFileSync } = await import("node:fs");
  const guide = readFileSync(resolve(import.meta.dirname, "../../docs/quality-gate.md"), "utf8");
  const state = parseGateState(guide);
  assert.match(state.asOf, /^[A-Z][a-z]+ \d{1,2}, \d{4}$/);
  assert.deepEqual(
    state.ports.map((entry) => entry.port),
    ["JPEG", "WebP", "JPEG XL", "AVIF"],
  );
  for (const entry of state.ports) assert.doesNotMatch(entry.state, /[`[\]]|\d{4}-\d{2}-\d{2}/);
  assert.throws(() => parseGateState(guide.replace("## Where each port stands", "## Ports")));
  assert.throws(() => parseGateState(guide.replace("| In short |", "| Summary |")));
  assert.throws(() => parseGateState(guide.replace("| libwebp-rs |", "| libwebp |")));
});
