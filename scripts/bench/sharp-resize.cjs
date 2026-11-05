"use strict";

// Optional comparator for run-baseline.sh. Sharp's resampler, transfer
// handling, chroma mode, threading and AV1 encoder are NOT matched to
// Ferrixel; results are labeled comparable_quality=no until SSIMULACRA2
// matching exists (M2 work package 0).
const sharp = require("sharp");

const [input, output, width, height, quality, effort] = process.argv.slice(2);
if (!input || !output || !width || !height) {
  process.stderr.write(
    "usage: node sharp-resize.cjs input output.avif width height [quality] [effort]\n",
  );
  process.exit(2);
}
sharp(input)
  .resize(Number(width), Number(height), {
    fit: "fill",
    kernel: sharp.kernel.lanczos3,
  })
  .avif({ quality: Number(quality ?? 80), effort: Number(effort ?? 4) })
  .toFile(output)
  .then(() => {})
  .catch((error) => {
    process.stderr.write(String(error) + "\n");
    process.exitCode = 1;
  });
