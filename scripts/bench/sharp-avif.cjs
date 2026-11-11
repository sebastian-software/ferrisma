"use strict";

// Sharp comparator for the quality-matched comparison (M2 work package 0,
// docs/benchmarks/M2-sharp-comparison.md). One image: auto-orient, resize to
// exactly WIDTH x HEIGHT with Lanczos3, convert to sRGB, write AVIF (or a
// lossless PNG when OUTPUT ends in .png, to measure the resize alone).
//
// usage: node sharp-avif.cjs INPUT OUTPUT WIDTH HEIGHT [key=value ...]
//   quality=50      AVIF quality (Sharp's default is 50)
//   effort=4        AVIF effort 0-9 (Sharp's default is 4)
//   chroma=444      444 (Sharp's default) or 420
//   bitdepth=8      8 (default), 10 or 12
//   space=linear    linear: resize in linear light (pipelineColourspace scrgb,
//                   the closest match to Ferrixel); srgb: Sharp's default,
//                   resize on gamma-encoded values; gamma22: Sharp's
//                   .gamma(2.2) approximation
//   icc=convert     convert: Sharp's default, apply an embedded ICC profile;
//                   ignore: ignoreIcc. Sharp 0.34.5 returns an all-black image
//                   for an ICC-tagged JPEG with space=linear, so the
//                   comparison uses ignore for its sRGB-tagged photo
//   fastshrink=1    Sharp's fastShrinkOnLoad (default true); 0 disables it.
//                   Sharp skips shrink-on-load whenever it rotates before the
//                   resize, applies gamma or sets a pipeline colourspace
//   crop=x,y,w,h  crop upright source pixels before resize (optional)
//   threads=0       sharp.concurrency(); 0 keeps Sharp's default (all cores)
//   repeat=1        run the whole pipeline N times in this process and print
//                   per-run milliseconds (excludes Node and module start-up)
// Sharp's operation cache is disabled so repeated runs do real work.
const path = require("node:path");
const sharp = require("sharp");

const [input, output, width, height, ...rest] = process.argv.slice(2);
if (!input || !output || !width || !height) {
  process.stderr.write("usage: node sharp-avif.cjs INPUT OUTPUT WIDTH HEIGHT [key=value ...]\n");
  process.exit(2);
}
const options = {
  quality: 50, effort: 4, chroma: "444", bitdepth: 8, space: "linear",
  icc: "convert", fastshrink: 1, threads: 0, repeat: 1, crop: "",
};
const strings = new Set(["chroma", "space", "icc", "crop"]);
for (const arg of rest) {
  const [key, value] = arg.split("=");
  if (!(key in options) || value === undefined) {
    process.stderr.write(`unknown option ${arg}\n`);
    process.exit(2);
  }
  options[key] = strings.has(key) ? value : Number(value);
}

sharp.cache(false);
if (options.threads > 0) sharp.concurrency(options.threads);

function pipeline() {
  let image = sharp(input, { failOn: "error", ignoreIcc: options.icc === "ignore" }).autoOrient();
  if (options.crop) {
    const parts = options.crop.split(",");
    if (parts.length !== 4 || parts.some((v) => !/^\d+$/.test(v)) || Number(parts[2]) === 0 || Number(parts[3]) === 0) {
      throw new Error("crop must be x,y,width,height in upright pixels");
    }
    const [left, top, width, height] = parts.map(Number);
    image = image.extract({ left, top, width, height });
  }
  if (options.space === "linear") image = image.pipelineColourspace("scrgb");
  if (options.space === "gamma22") image = image.gamma(2.2);
  image = image
    .resize(Number(width), Number(height), {
      fit: "fill",
      kernel: sharp.kernel.lanczos3,
      fastShrinkOnLoad: options.fastshrink !== 0,
    })
    .toColourspace("srgb");
  if (path.extname(output).toLowerCase() === ".png") {
    return image.png({ compressionLevel: 1 }).toFile(output);
  }
  return image
    .avif({
      quality: options.quality,
      effort: options.effort,
      chromaSubsampling: options.chroma === "420" ? "4:2:0" : "4:4:4",
      bitdepth: options.bitdepth,
    })
    .toFile(output);
}

(async () => {
  const times = [];
  for (let i = 0; i < options.repeat; i++) {
    const started = process.hrtime.bigint();
    await pipeline();
    times.push(Number(process.hrtime.bigint() - started) / 1e6);
  }
  if (options.repeat > 1) {
    const sorted = [...times].sort((a, b) => a - b);
    process.stdout.write(
      `sharp-in-process: runs=${times.length} median_ms=${sorted[Math.floor(sorted.length / 2)].toFixed(1)} ` +
        `min_ms=${sorted[0].toFixed(1)} all_ms=${times.map((t) => t.toFixed(1)).join(",")}\n`,
    );
  }
})().catch((error) => {
  process.stderr.write(String(error) + "\n");
  process.exitCode = 1;
});
