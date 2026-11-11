"use strict";
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const sharp = require("sharp");

test("comparator extracts the requested upright pixels before resizing", async (t) => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "ferrixel-sharp-"));
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const input = path.join(dir, "input.png");
  const output = path.join(dir, "output.png");
  const pixels = Buffer.alloc(8 * 8 * 3);
  for (let y = 0; y < 8; y++) for (let x = 0; x < 8; x++) {
    const i = (y * 8 + x) * 3;
    pixels[i] = x * 24;
    pixels[i + 1] = y * 24;
    pixels[i + 2] = (x + y) * 12;
  }
  await sharp(pixels, { raw: { width: 8, height: 8, channels: 3 } }).png().toFile(input);
  const script = path.join(__dirname, "sharp-avif.cjs");
  const result = spawnSync(process.execPath, [script, input, output, "4", "4", "space=srgb", "crop=2,1,4,4"], { encoding: "utf8" });
  assert.equal(result.status, 0, result.stderr);
  const actual = await sharp(output).raw().toBuffer();
  const expected = await sharp(input).extract({ left: 2, top: 1, width: 4, height: 4 }).raw().toBuffer();
  assert.deepEqual(actual, expected);
  const bad = spawnSync(process.execPath, [script, input, output, "4", "4", "crop=1,2,0,4"], { encoding: "utf8" });
  assert.notEqual(bad.status, 0);
});
