#!/bin/bash
# usage: ab_crop.sh <before-bin> <after-bin> <reps> ; interleaved before/after,
# fresh process per sample, order alternating per repetition (indicative).
S=${S:?set S to the work directory with corpus/ and crop5/}
C=$S/corpus
OUT=$S/crop5/out
mkdir -p "$OUT"
BEFORE=$1; AFTER=$2; REPS=${3:-3}
COMMON=(--use-rendered-icc --budget-mb=2048 --encoder-threads=1 --transform-threads=1)
cases=(
  "flower-full|$C/flower-lossless.jxl|800 533|"
  "flower-crop|$C/flower-lossless.jxl|567 378|--crop=567,378,1134,756"
  "dog-full|$C/dog-lossless.jxl|1200 1600|"
  "dog-crop|$C/dog-lossless.jxl|756 1008|--crop=756,1008,1512,2016"
  "mountain-full|$C/mountain-lossless.jxl|1600 1067|"
  "mountain-crop|$C/mountain-lossless.jxl|1500 1000|--crop=1500,1000,3000,2000"
)
one() {
  local label=$1 bin=$2 input=$3 dims=$4 extra=$5
  # shellcheck disable=SC2086
  out=$(/usr/bin/time -l "$bin" "$input" "$OUT/o.webp" $dims $extra "${COMMON[@]}" 2>&1)
  load=$(sysctl -n vm.loadavg | awk '{print $2}')
  echo "$label $(echo "$out" | grep -E '^report:' | grep -oE 'decode_us=[0-9]+|total_us=[0-9]+|peak_managed_bytes=[0-9]+|decoder_full_frames=[0-9]+' | tr '\n' ' ') rss=$(echo "$out" | awk '/maximum resident/{print $1}') load=$load"
}
for rep in $(seq 0 "$REPS"); do
  for c in "${cases[@]}"; do
    IFS='|' read -r name input dims extra <<<"$c"
    if (( rep % 2 == 0 )); then order=(before after); else order=(after before); fi
    for which in "${order[@]}"; do
      bin=$BEFORE; [ "$which" = after ] && bin=$AFTER
      tag=$([ "$rep" = 0 ] && echo warm || echo "rep$rep")
      one "$tag $name $which" "$bin" "$input" "$dims" "$extra"
    done
  done
done
