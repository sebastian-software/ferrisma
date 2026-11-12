#!/bin/bash
# usage: cli_runs.sh <ferrisma-binary> ; prints report + max RSS per case (indicative)
S=$S
P=$S/jxlperf
B=$1
cd $P
COMMON=(--use-rendered-icc --budget-mb=2048 --encoder-threads=1 --transform-threads=1)
run() {
  local name=$1; shift
  out=$(/usr/bin/time -l "$B" "$@" "${COMMON[@]}" 2>&1)
  echo "$name $(echo "$out" | grep -E '^report:' | grep -oE 'decode_us=[0-9]+|total_us=[0-9]+|peak_managed_bytes=[0-9]+' | tr '\n' ' ') rss=$(echo "$out" | awk '/maximum resident/{print $1}')"
}
run xyb-full flower-d1e7.jxl o.webp 800 533
run xyb-crop flower-d1e7.jxl o.webp 567 378 --crop=567,378,1134,756
run recon-crop $S/corpus/flower-jpegrecon.jxl o.webp 567 378 --crop=567,378,1134,756
run lossless24-full $S/corpus/mountain-lossless.jxl o.webp 1600 1067
run lossless24-crop $S/corpus/mountain-lossless.jxl o.webp 1500 1000 --crop=1500,1000,3000,2000
run xyb24-full mountain-d1e7.jxl o.webp 1600 1067
