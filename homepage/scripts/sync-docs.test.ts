import { test } from "node:test";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { render, rewriteTarget } from "./sync-docs.ts";

const source = resolve(import.meta.dirname, "../../docs/getting-started.md");
const github = "https://github.com/sebastian-software/ferrixel/blob/main/";

test("rewrites code-formatted link labels, images and references", () => {
  const input = '[`ferrixel`](../Cargo.toml) ![diagram](../README.md#part)\n\n[details][ref]\n\n[ref]: architecture.md#memory-and-scheduling\n';
  const output = render(source, input);
  assert.match(output, new RegExp(`\\[\\x60ferrixel\\x60\\]\\(${github}Cargo\\.toml\\)`));
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
