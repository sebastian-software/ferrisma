#!/usr/bin/env bash
# rav1e (the `ferrixel` CLI) against libavif-rs (the `ferrixel-libavif` CLI)
# at matched SSIMULACRA2 (ADR-0024). Reuses the corpus and references that
# `compare-sharp.py prepare` writes, and the target-A scores of its
# `anchors.tsv`.
#
# usage: scripts/bench/compare-encoders.sh OUT_DIR [match|time|all]
#   OUT_DIR  the compare-sharp.py output directory (corpus/, refs/, anchors.tsv)
#   match    bisect libavif-rs quality to target A at speeds 6 and 7
#   time     best-of-five stage timings at the matched settings; waits until
#            the 1-minute load average is below COMPARE_MAX_LOAD (default 4)
set -euo pipefail

out=${1:?usage: compare-encoders.sh OUT_DIR [match|time|all]}
phase=${2:-all}
repo=$(cd "$(dirname "$0")/../.." && pwd)
rav1e=$repo/target/release/ferrixel
libavif=$repo/crates/ferrixel-libavif/target/release/ferrixel-libavif
qm=$repo/scripts/bench/quality-match/target/release/ferrixel-quality-match
(cd "$repo" && cargo build --release --features cli --bin ferrixel >/dev/null)
(cd "$repo/crates/ferrixel-libavif" && cargo build --release --features cli >/dev/null)
(cd "$repo/scripts/bench/quality-match" && cargo build --release >/dev/null)
mkdir -p "$out/encoders"

# name|file|width|height|profile flag
images=(
  "dog|dog.jpg|1600|2133|--use-rendered-icc"
  "flower|flower.jpg|1600|1067|--assume-srgb"
  "kodim03|kodim03.jpg|512|341|--assume-srgb"
  "kodim23|kodim23.jpg|512|341|--assume-srgb"
)
target() { awk -v n="$1" '$1 == n && $2 == "ferrixel-q80" { print $4 }' "$out/anchors.tsv"; }

match() {
  printf 'image\tspeed\tstatus\ttarget\tq_below\tscore_below\tbytes_below\tq_above\tscore_above\tbytes_above\tbytes_at_target\tq_nearest\tscore_nearest\tencodes\n'
  for entry in "${images[@]}"; do
    IFS='|' read -r name file w h flag <<< "$entry"
    for speed in 6 7; do
      result=$("$qm" bisect "$out/refs/$name.ref" "$(target "$name")" 30 95 1 "$out/encoders/$name-s$speed" -- \
        "$libavif" "$out/corpus/$file" '{out}' "$w" "$h" "$flag" '--quality={q}' "--speed=$speed" \
        --encoder-threads=0 --transform-threads=0 2>/dev/null)
      printf '%s\t%s\t%s\n' "$name" "$speed" "$result"
    done
  done
}

load() {
  if [[ "$(uname)" == Darwin ]]; then sysctl -n vm.loadavg | awk '{print $2}'; else cut -d' ' -f1 /proc/loadavg; fi
}

timing() {
  local limit=${COMPARE_MAX_LOAD:-4}
  while awk -v l="$(load)" -v m="$limit" 'BEGIN { exit !(l >= m) }'; do sleep 30; done
  echo "| image | encoder | threads | quality | decode ms | transform ms | encode ms | total ms | bytes | load |"
  echo "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
  for entry in "${images[@]}"; do
    IFS='|' read -r name file w h flag <<< "$entry"
    q6=$(awk -v n="$name" '$1 == n && $2 == 6 { print $12 }' "$out/encoders/matched.tsv")
    q7=$(awk -v n="$name" '$1 == n && $2 == 7 { print $12 }' "$out/encoders/matched.tsv")
    # rav1e at its quality-80 anchor, which defines target A.
    for run in "rav1e s6|$rav1e|80|6|0" "libavif s6|$libavif|$q6|6|0" "libavif s7|$libavif|$q7|7|0" \
               "rav1e s6|$rav1e|80|6|1" "libavif s6|$libavif|$q6|6|1"; do
      IFS='|' read -r label bin q speed threads <<< "$run"
      best=999999999 line=""
      for _ in 1 2 3 4 5; do
        report=$("$bin" "$out/corpus/$file" "$out/encoders/t.avif" "$w" "$h" "$flag" "--quality=$q" \
          "--speed=$speed" "--encoder-threads=$threads" "--transform-threads=$threads" 2>/dev/null | grep '^report:')
        total=$(sed -E 's/.*total_us=([0-9]+).*/\1/' <<< "$report")
        if (( total < best )); then best=$total line=$report; fi
      done
      field() { sed -E "s/.* $1=([0-9]+).*/\1/" <<< "$line"; }
      printf '| %s | %s | %s | %s | %d | %d | %d | %d | %d | %s |\n' "$name" "$label" \
        "$([[ $threads == 0 ]] && echo all || echo 1)" "$q" $(( $(field decode_us) / 1000 )) \
        $(( $(field transform_us) / 1000 )) $(( $(field encode_us) / 1000 )) $(( best / 1000 )) \
        "$(field output_bytes)" "$(load)"
    done
  done
}

case $phase in
  match) match | tee "$out/encoders/matched.tsv" ;;
  time) timing | tee "$out/encoders/time.md" ;;
  all) match | tee "$out/encoders/matched.tsv"; timing | tee "$out/encoders/time.md" ;;
  *) echo "unknown phase $phase" >&2; exit 2 ;;
esac
