#!/bin/sh
# usage: hashes.sh <probe> <label>  -> prints FNV per input and encoder output sha256
S="${S:?scratch directory}"
P=$1; L=$2
for f in $S/corpus/flower-jpegrecon.jxl $S/jxlperf/flower-d1e7.jxl $S/jxlperf/mountain-d1e7.jxl $S/corpus/flower-lossless.jxl; do
  echo "$(basename $f) $(JXL_PROBE_HASH=1 $P decode $f 1 | grep fnv)"
done
$P lossy $S/jxlperf/flower.ppm 7 1.0 1 $S/r3/enc-$L.jxl | grep bytes
shasum -a 256 $S/r3/enc-$L.jxl
