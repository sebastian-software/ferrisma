#!/bin/sh
# Strictly serial memory runs with an acceptance check (2026-11-16).
# measure_serial.sh <probe> <out dir> <runs> <case file>
#   case file: lines "<input path> <mode>" (mode: lossless or a distance)
# For every case, `runs` accepted fresh processes. A run is accepted when
#   - free memory (memory_pressure) was at least 20 % before it,
#   - swap use (vm.swapusage) grew by at most 32 MiB during it, and
#   - its peak memory footprint is at most 2 % above its maximum resident
#     set size (footprint counts compressed and swapped pages, RSS does
#     not: a larger footprint means pages left memory during the run).
#     RSS above the footprint is accepted: it is memory the allocator freed
#     and marked reusable, still resident (and counted by the 2026-11-15
#     evidence), but no longer in the footprint;
# otherwise it is logged as REJECTED and repeated (at most 6 attempts).
# Prints: input mode run max_rss peak_footprint heap_peak output_bytes sha256[0..16]
# and, per batch, the environment lines "# env ...".
PROBE=$1; OUT=$2; RUNS=$3; CASES=$4; mkdir -p "$OUT"
swap() { sysctl -n vm.swapusage | sed 's/.*used = \([0-9.]*\)M.*/\1/'; }
free() { memory_pressure | tail -1 | sed 's/.*: \([0-9]*\)%.*/\1/'; }
echo "# env $(date +%FT%T) $(sysctl -n vm.swapusage) | $(memory_pressure | tail -1) | $(uptime | sed 's/.*load/load/')"
while read -r f mode; do
  [ -n "$f" ] || continue
  n=$(basename "$f"); n=${n%.*}
  ok=0; tries=0
  while [ $ok -lt "$RUNS" ] && [ $tries -lt 6 ]; do
    tries=$((tries + 1))
    fr=$(free); s0=$(swap)
    /usr/bin/time -l "$PROBE" "$f" "$mode" "$OUT/$n.$mode.jxl" > "$OUT/$n.$mode.out" 2> "$OUT/$n.$mode.time" || echo "# FAILED $n $mode"
    s1=$(swap)
    rss=$(awk '/maximum resident set size/ {print $1}' "$OUT/$n.$mode.time")
    fp=$(awk '/peak memory footprint/ {print $1}' "$OUT/$n.$mode.time")
    heap=$(sed -n 's/^heap_peak_bytes=//p' "$OUT/$n.$mode.out")
    bytes=$(sed -n 's/.*bytes=\([0-9]*\)$/\1/p' "$OUT/$n.$mode.out" | head -1)
    sha=$(shasum -a 256 "$OUT/$n.$mode.jxl" | cut -c1-16)
    verdict=$(awk -v r="$rss" -v p="$fp" -v a="$s0" -v b="$s1" -v fr="$fr" 'BEGIN {
      print (fr >= 20 && b - a <= 32 && p <= 1.02 * r) ? "ok" : "rejected" }')
    if [ "$verdict" = ok ]; then
      ok=$((ok + 1))
      echo "$n $mode $ok $rss $fp $heap $bytes $sha"
    else
      echo "# REJECTED $n $mode rss=$rss footprint=$fp free=$fr% swap=${s0}M->${s1}M"
    fi
  done
done < "$CASES"
echo "# env $(date +%FT%T) $(sysctl -n vm.swapusage) | $(memory_pressure | tail -1) | $(uptime | sed 's/.*load/load/')"
