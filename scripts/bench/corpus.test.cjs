"use strict";
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { verify } = require("./corpus.cjs");

test("corpus verification rejects changed, missing, duplicate and escaping files", (t) => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "ferrixel-corpus-"));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));
  fs.writeFileSync(path.join(root, "image"), "abc");
  const entry = { id: "test", path: "image", sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", source: "source", license: "license", role: "photo", description: "test" };
  const manifest = path.join(root, "manifest.json");
  const write = (images) => fs.writeFileSync(manifest, JSON.stringify({ schema: 1, images }));
  write([entry]);
  assert.equal(verify(manifest, root).images.length, 1);
  fs.writeFileSync(path.join(root, "image"), "changed");
  assert.throws(() => verify(manifest, root), /SHA-256 mismatch/);
  fs.unlinkSync(path.join(root, "image"));
  assert.throws(() => verify(manifest, root), /ENOENT/);
  write([entry, entry]);
  fs.writeFileSync(path.join(root, "image"), "abc");
  assert.throws(() => verify(manifest, root), /duplicate/);
  write([{ ...entry, path: "../outside" }]);
  assert.throws(() => verify(manifest, root), /outside corpus root/);
});
