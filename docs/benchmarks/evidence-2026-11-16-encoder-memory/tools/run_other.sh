#!/bin/sh
# Peak RSS and heap peak of the AVIF and WebP probes, one fresh process per
# run, two runs per case (2026-11-16).
# run_other.sh <probe> <out dir> <label> "<probe args after the input>" <inputs...>
# Prints: input label run max_rss heap_peak reserved output_bytes
PROBE=$1; OUT=$2; LABEL=$3; ARGS=$4; shift 4; mkdir -p "$OUT"
for f in "$@"; do
  n=$(basename "$f"); n=${n%.*}
  for run in 1 2; do
    # shellcheck disable=SC2086
    /usr/bin/time -l "$PROBE" "$f" $ARGS > "$OUT/$n.$LABEL.out" 2> "$OUT/$n.$LABEL.time" || echo "FAILED $n $LABEL"
    rss=$(awk '/maximum resident set size/ {print $1}' "$OUT/$n.$LABEL.time")
    heap=$(sed -n 's/^heap_peak_bytes=//p' "$OUT/$n.$LABEL.out")
    reserved=$(sed -n 's/.*reserved=\([0-9]*\).*/\1/p' "$OUT/$n.$LABEL.out")
    bytes=$(sed -n 's/.* bytes=\([0-9]*\)$/\1/p' "$OUT/$n.$LABEL.out")
    echo "$n $LABEL $run $rss $heap $reserved $bytes"
  done
done
