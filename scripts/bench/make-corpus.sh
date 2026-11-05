#!/usr/bin/env bash
# Synthetic fallback corpus for the M2 baseline harness (work package 0).
# Usage: scripts/bench/make-corpus.sh OUT_DIR
#
# Writes deterministic plasma images at 1, 12, 24 and 48 megapixels as JPEG
# (and one AVIF when ImageMagick has libheif) into OUT_DIR/single, plus 100
# small 640x427 JPEGs into OUT_DIR/small for sequential throughput runs.
# Plasma noise is NOT a photograph: it decodes and compresses differently.
# Use a real-photo corpus for any quality-matched comparison; run-baseline.sh
# accepts any directory with the same layout.
set -euo pipefail

out=${1:?usage: make-corpus.sh OUT_DIR}
command -v magick >/dev/null || { echo "ImageMagick (magick) is required" >&2; exit 2; }
mkdir -p "$out/single" "$out/small"

generate() {
  local name=$1 width=$2 height=$3 target="$out/single/$1.jpg"
  if [[ -f "$target" ]]; then
    echo "exists: $target" >&2
    return
  fi
  magick -size "${width}x${height}" -seed 7 plasma:fractal -blur 0x1.5 \
    -sampling-factor 2x2 -quality 88 "$target"
  echo "wrote: $target ($(wc -c < "$target" | tr -d ' ') bytes)" >&2
}

generate synthetic-1mpx-1280x853 1280 853
generate synthetic-12mpx-4000x3000 4000 3000
generate synthetic-24mpx-6000x4000 6000 4000
generate synthetic-48mpx-8000x6000 8000 6000

avif="$out/single/synthetic-12mpx-4000x3000.avif"
if magick -list format 2>/dev/null | grep -q '^ *AVIF' && [[ ! -f "$avif" ]]; then
  magick "$out/single/synthetic-12mpx-4000x3000.jpg" -quality 60 "$avif"
  echo "wrote: $avif ($(wc -c < "$avif" | tr -d ' ') bytes)" >&2
fi

for index in $(seq -f '%03g' 1 100); do
  target="$out/small/small-$index.jpg"
  [[ -f "$target" ]] && continue
  magick -size 640x427 -seed "$index" plasma:fractal -blur 0x1 \
    -sampling-factor 2x2 -quality 85 "$target"
done
echo "small images: $(ls "$out/small" | wc -l | tr -d ' ')" >&2
