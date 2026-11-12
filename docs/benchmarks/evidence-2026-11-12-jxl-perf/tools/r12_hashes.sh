#!/bin/bash
# usage: hashes.sh <probe-name>... ; FNV-1a of every decoded F32 sample per input
for f in $S/corpus/flower-jpegrecon.jxl $S/jxlperf/flower-d1e7.jxl $S/corpus/flower-lossless.jxl $S/jxlperf/mountain-d1e7.jxl $S/corpus/mountain-lossless.jxl; do
  for b in "$@"; do
    echo "$(basename $f) $b $(JXL_PROBE_HASH=1 $S/jxlperf/bin/$b decode $f 1 2>&1 | grep -oE 'fnv[^ ]*|hash[^ ]*' | tr '\n' ' ')"
  done
done
