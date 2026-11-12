#!/usr/bin/env bash
# Record the runner's CPU, core count, memory and load into $OUT/host-<phase>.txt.
set -uo pipefail
phase=${1:?before|after}
: "${OUT:?set OUT}"
mkdir -p "$OUT"
{
  echo "phase: $phase"
  date -u +%Y-%m-%dT%H:%M:%SZ
  echo "runner: ${RUNNER_NAME:-} ${ImageOS:-} ${ImageVersion:-} ${RUNNER_ARCH:-}"
  uname -a
  uptime
  echo "cpus: $(getconf _NPROCESSORS_ONLN)"
  if [[ "$(uname -s)" == Linux ]]; then
    lscpu
    free -m
  else
    sysctl -n machdep.cpu.brand_string hw.model hw.ncpu hw.memsize 2>/dev/null
    sw_vers
  fi
  rustc -vV 2>/dev/null
  git rev-parse HEAD 2>/dev/null
} > "$OUT/host-$phase.txt" 2>&1
cat "$OUT/host-$phase.txt"
