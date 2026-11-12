#!/usr/bin/env bash
# Reproducible Ferrisma single-image measurements.
# Usage: scripts/bench-m1.sh INPUT WIDTH HEIGHT [REPETITIONS]
# Set FERRISMA_PROFILE=strict|assume-srgb|rendered-icc and
# FERRISMA_FILTER=area|lanczos. No quality parity with other tools is implied.
set -euo pipefail

if [[ $# -lt 3 || $# -gt 4 ]]; then
  echo "Usage: $0 INPUT WIDTH HEIGHT [REPETITIONS]" >&2
  exit 2
fi

input=$1
width=$2
height=$3
repeats=${4:-5}
profile=${FERRISMA_PROFILE:-strict}
filter=${FERRISMA_FILTER:-lanczos}
quality=${FERRISMA_QUALITY:-80}
speed=${FERRISMA_SPEED:-6}
budget=${FERRISMA_BUDGET_MB:-512}

case "$profile" in
  strict) policy_args=() ;;
  assume-srgb) policy_args=(--assume-srgb) ;;
  rendered-icc) policy_args=(--use-rendered-icc) ;;
  *) echo "Invalid FERRISMA_PROFILE" >&2; exit 2 ;;
esac
case "$filter" in
  area) filter_args=(--area) ;;
  lanczos) filter_args=() ;;
  *) echo "Invalid FERRISMA_FILTER" >&2; exit 2 ;;
esac
for value in "$width" "$height" "$repeats" "$budget"; do
  [[ "$value" =~ ^[1-9][0-9]*$ ]] || { echo "Expected positive integer: $value" >&2; exit 2; }
done
[[ -f "$input" ]] || { echo "Input missing: $input" >&2; exit 2; }
command -v cargo >/dev/null || { echo "Cargo is required" >&2; exit 2; }
[[ -x /usr/bin/time ]] || { echo "GNU /usr/bin/time is required" >&2; exit 2; }

echo "# Ferrisma M1 performance run (not a libvips/Sharp quality parity claim)" >&2
rustc --version >&2
uname -a >&2
cargo build --release --features cli
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
echo "iteration,elapsed_seconds,peak_rss_kib,output_bytes"
for ((i=1; i<=repeats; i++)); do
  /usr/bin/time -f '%e,%M' -o "$scratch/time.txt" \
    target/release/ferrisma \
    "$input" "$scratch/output.avif" "$width" "$height" \
    "${policy_args[@]}" "${filter_args[@]}" \
    "--quality=$quality" "--speed=$speed" "--budget-mb=$budget" \
    >/dev/null
  timings=$(cat "$scratch/time.txt")
  bytes=$(wc -c < "$scratch/output.avif" | tr -d ' ')
  printf '%s,%s,%s\n' "$i" "$timings" "$bytes"
done
