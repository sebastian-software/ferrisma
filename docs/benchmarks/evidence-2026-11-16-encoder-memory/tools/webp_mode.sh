#!/bin/sh
# measure_serial.sh adapter for webp_memory_probe: one mode token
# (lossy-<quality>-<method> or lossless-<level>) becomes its arguments; exec
# keeps the probe the measured process. webp_mode.sh <input> <mode> [ignored]
PROBE=$(dirname "$0")/webp_memory_probe
IFS=-; set -- "$1" $2; IFS=' '
exec "$PROBE" "$@"
