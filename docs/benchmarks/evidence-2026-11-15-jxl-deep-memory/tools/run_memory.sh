#!/bin/sh
# Peak RSS of a fresh process for the bridge's lossless encode (the engine's
# JPEG XL sink path) and the pinned cjxl, two runs per case, larger kept.
# run_memory.sh <encode_pnm> <cjxl> <inputs dir> <out dir>
EX=$1; CJXL=$2; IN=$3; OUT=$4; mkdir -p "$OUT"
for f in "$IN"/*.ppm "$IN"/*.pam; do
  n=$(basename "$f"); n=${n%.*}
  for tool in port cjxl; do
    best=0
    for run in 1 2; do
      if [ $tool = port ]; then
        /usr/bin/time -l "$EX" "$f" "$OUT/$n.$tool.jxl" lossless > /dev/null 2> "$OUT/$n.$tool.time"
      else
        /usr/bin/time -l "$CJXL" "$f" "$OUT/$n.$tool.jxl" -d 0 -e 2 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG --quiet > /dev/null 2> "$OUT/$n.$tool.time"
      fi
      rss=$(awk '/maximum resident set size/ {print $1}' "$OUT/$n.$tool.time")
      [ "$rss" -gt "$best" ] && best=$rss
    done
    echo "$n $tool $best $(shasum -a 256 "$OUT/$n.$tool.jxl" | cut -c1-16)"
  done
done
