#!/usr/bin/env bash
# Hotspot profile of one Ferrisma run (M2 work package 0).
# Usage: scripts/bench/profile.sh INPUT WIDTH HEIGHT OUT_DIR [extra ferrisma args]
#
# macOS: /usr/bin/sample attaches to the running process at a 1 ms interval
# and writes a call tree; its "Sort by top of stack" section is the
# self-time list saved as hotspots.txt. Linux: perf record -g, then
# perf report --stdio --no-children. Rust symbols are demangled through
# rustfilt when it is installed. For an interactive flame graph use
# `samply record target/release/ferrisma ...` instead.
# Release builds keep symbol names but no line tables, so inlined functions
# are attributed to their caller.
set -euo pipefail

input=${1:?usage: profile.sh INPUT WIDTH HEIGHT OUT_DIR [extra args]}
width=${2:?}
height=${3:?}
out=${4:?}
shift 4

repo=$(cd "$(dirname "$0")/../.." && pwd)
(cd "$repo" && cargo build --release --features cli >/dev/null)
bin=$repo/target/release/ferrisma
mkdir -p "$out"
args=("$input" "$out/profile-output.avif" "$width" "$height" --assume-srgb --quality=80 --speed=6 --max-input-mb=512 "$@")

demangle() {
  if command -v rustfilt >/dev/null; then rustfilt; else cat; fi
}

if [[ "$(uname)" == Darwin ]]; then
  "$bin" "${args[@]}" > "$out/profile-run.log" &
  pid=$!
  sample "$pid" 600 1 -mayDie -file "$out/sample-raw.txt" >/dev/null 2>&1 || true
  wait "$pid"
  demangle < "$out/sample-raw.txt" > "$out/sample.txt"
  awk '/Sort by top of stack/{flag=1} flag' "$out/sample.txt" | head -80 > "$out/hotspots.txt"
else
  command -v perf >/dev/null || { echo "perf is required on Linux" >&2; exit 2; }
  perf record -g -o "$out/perf.data" -- "$bin" "${args[@]}" > "$out/profile-run.log"
  perf report -i "$out/perf.data" --stdio --no-children 2>/dev/null | demangle | head -150 > "$out/hotspots.txt"
fi

cat "$out/profile-run.log"
echo
echo "hotspots: $out/hotspots.txt"
head -60 "$out/hotspots.txt"
