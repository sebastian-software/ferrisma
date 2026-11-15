#!/bin/bash
# Peak memory of the lossless effort-2 encode, port and original, one fresh
# process per run (issue #223, 2026-11-15). Memory, not timing.
# Usage: run_memory.sh <probe> <cjxl> <input-dir> <output-dir>
#   probe: lossless_memory_probe built from tools/ (README.md)
#   cjxl:  the pinned original, scalar configuration (README.md)
set -u
PROBE=$1; CJXL=$2; IN=$3; OUT=$4
mkdir -p "$OUT"
rss() { # label command...: prints label, max RSS bytes, heap peak, bytes, sha
  local label=$1; shift
  /usr/bin/time -l "$@" > "$OUT/$label.stdout" 2> "$OUT/$label.time"
  local status=$?
  local max; max=$(awk '/maximum resident set size/ {print $1}' "$OUT/$label.time")
  local heap; heap=$(sed -n 's/^heap_peak_bytes=//p' "$OUT/$label.stdout")
  echo "$label status=$status max_rss=$max heap_peak=${heap:--}"
}
for image in c700x700.ppm c700x700a.pam c1024x1024.ppm c1024x1024a.pam \
    c2048x2048.ppm c2048x2048a.pam c3000x2000.ppm c3000x2000a.pam \
    photo12.ppm photo12a.pam photo24.ppm photo24a.pam; do
  base=${image%.*}
  for rep in 1 2; do
    rss "$base-port-fixed-$rep" "$PROBE" fixed "$IN/$image" "$OUT/$base-port-fixed.jxl"
    rss "$base-port-api0-$rep" "$PROBE" api "$IN/$image" 0 "$OUT/$base-port-api0.jxl"
    rss "$base-port-api-default-$rep" "$PROBE" api "$IN/$image" -1 "$OUT/$base-port-api-default.jxl"
    rss "$base-cjxl-buffering0-$rep" "$CJXL" "$IN/$image" "$OUT/$base-cjxl-buffering0.jxl" \
      -d 0 -e 2 --buffering=0 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG
    rss "$base-cjxl-default-$rep" "$CJXL" "$IN/$image" "$OUT/$base-cjxl-default.jxl" \
      -d 0 -e 2 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG
  done
done
(cd "$OUT" && shasum -a 256 ./*.jxl)
