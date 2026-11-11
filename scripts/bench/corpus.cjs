"use strict";

// Check the bytes behind a reviewable corpus inventory. This does not infer
// color, orientation, licensing, or image dimensions from file extensions.
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");

function verify(manifestPath, root) {
  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  if (manifest.schema !== 1 || !Array.isArray(manifest.images) || !manifest.images.length) {
    throw new Error("expected a nonempty schema 1 corpus");
  }
  const ids = new Set();
  const files = new Set();
  const results = [];
  for (const entry of manifest.images) {
    for (const key of ["id", "path", "sha256", "source", "license", "role", "description"]) {
      if (typeof entry[key] !== "string" || !entry[key].trim()) throw new Error(`missing ${key}`);
    }
    if (ids.has(entry.id) || files.has(entry.path)) throw new Error(`duplicate corpus entry: ${entry.id}`);
    ids.add(entry.id);
    files.add(entry.path);
    if (!/^[a-f0-9]{64}$/.test(entry.sha256)) throw new Error(`invalid SHA-256: ${entry.id}`);
    if (!new Set(["photo", "correctness-only"]).has(entry.role)) throw new Error(`invalid role: ${entry.id}`);
    const file = path.resolve(root, entry.path);
    const relative = path.relative(root, file);
    if (!relative || relative === ".." || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) {
      throw new Error(`path outside corpus root: ${entry.path}`);
    }
    const actual = crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
    if (actual !== entry.sha256) throw new Error(`SHA-256 mismatch: ${entry.id}`);
    results.push({ id: entry.id, path: entry.path, sha256: actual });
  }
  return { schema: 1, images: results };
}

if (require.main === module) {
  try {
    if (process.argv.length !== 4) throw new Error("usage: node corpus.cjs MANIFEST ROOT");
    process.stdout.write(`${JSON.stringify(verify(process.argv[2], path.resolve(process.argv[3])), null, 2)}\n`);
  } catch (error) {
    process.stderr.write(`${error.message}\n`);
    process.exitCode = 1;
  }
}
module.exports = { verify };
