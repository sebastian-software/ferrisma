"use strict";

// Sharp side of the batch-throughput comparison (M2 work package 0,
// docs/benchmarks/M2-sharp-comparison.md); the Ferrixel side is
// `examples/batch.rs`. Reads every .jpg in DIR into memory, then transcodes
// all of them buffer to buffer (auto-orient, Lanczos3 resize in linear light
// to WIDTH x HEIGHT, sRGB, AVIF) with PARALLEL images in flight, and prints
// images per second. Nothing is written unless OUT_DIR is given.
//
// usage: node sharp-batch.cjs DIR WIDTH HEIGHT [key=value ...]
//   quality=50 effort=4 chroma=444   AVIF settings as in sharp-avif.cjs
//   threads=0      sharp.concurrency() per image; 0 keeps Sharp's default
//   parallel=4     images in flight; also sets UV_THREADPOOL_SIZE, which caps
//                  how many images libuv runs at once (Sharp's default is 4)
//   out=DIR        write each output as DIR/<name>.avif (for scoring)
const fs = require("node:fs");
const path = require("node:path");

const [dir, width, height, ...rest] = process.argv.slice(2);
if (!dir || !width || !height) {
  process.stderr.write("usage: node sharp-batch.cjs DIR WIDTH HEIGHT [key=value ...]\n");
  process.exit(2);
}
const options = { quality: 50, effort: 4, chroma: "444", threads: 0, parallel: 4, out: "" };
for (const arg of rest) {
  const [key, value] = arg.split("=");
  if (!(key in options) || value === undefined) {
    process.stderr.write(`unknown option ${arg}\n`);
    process.exit(2);
  }
  options[key] = key === "chroma" || key === "out" ? value : Number(value);
}
// Must be set before libuv starts its pool, that is before the first
// asynchronous operation and before loading sharp.
process.env.UV_THREADPOOL_SIZE = String(options.parallel);
const sharp = require("sharp");
sharp.cache(false);
if (options.threads > 0) sharp.concurrency(options.threads);

const files = fs.readdirSync(dir).filter((f) => f.toLowerCase().endsWith(".jpg")).sort();
const inputs = files.map((f) => fs.readFileSync(path.join(dir, f)));

function transcode(buffer) {
  return sharp(buffer, { failOn: "error" })
    .autoOrient()
    .pipelineColourspace("scrgb")
    .resize(Number(width), Number(height), { fit: "fill", kernel: sharp.kernel.lanczos3 })
    .toColourspace("srgb")
    .avif({
      quality: options.quality,
      effort: options.effort,
      chromaSubsampling: options.chroma === "420" ? "4:2:0" : "4:4:4",
    })
    .toBuffer();
}

(async () => {
  const outputs = new Array(inputs.length);
  let next = 0;
  let failed = 0;
  const started = process.hrtime.bigint();
  async function worker() {
    for (;;) {
      const index = next++;
      if (index >= inputs.length) return;
      try {
        outputs[index] = await transcode(inputs[index]);
      } catch {
        failed++;
      }
    }
  }
  await Promise.all(Array.from({ length: options.parallel }, worker));
  const seconds = Number(process.hrtime.bigint() - started) / 1e9;
  const bytes = outputs.reduce((sum, b) => sum + (b ? b.length : 0), 0);
  if (options.out) {
    fs.mkdirSync(options.out, { recursive: true });
    outputs.forEach((b, i) => b && fs.writeFileSync(path.join(options.out, files[i].replace(/\.jpg$/i, ".avif")), b));
  }
  process.stdout.write(
    `images: ${inputs.length}, failed: ${failed}, output ${width}x${height}, parallel: ${options.parallel}, ` +
      `threads per image: ${sharp.concurrency()}, output bytes: ${bytes}\n` +
      `sharp batch: ${seconds.toFixed(2)} s, ${(inputs.length / seconds).toFixed(1)} images/s\n`,
  );
})();
