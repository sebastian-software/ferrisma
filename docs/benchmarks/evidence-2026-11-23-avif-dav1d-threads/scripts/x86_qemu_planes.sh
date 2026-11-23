#!/bin/bash
# x86_qemu_planes.sh OUT: zz_planes_threads dav1d t1/2/4 on the 16 inputs under qemu-x86_64 -cpu Haswell-v4.
set -u
OUT=$1
B=/src/target/threads-bench/bin-x86
command -v qemu-x86_64 >/dev/null || (apt-get update -qq && apt-get install -y -qq --no-install-recommends qemu-user) > /dev/null 2>&1
: > "$OUT/qemu_planes.txt"
for f in /src/target/threads-bench/inputs/photo*.avif; do
  name=$(basename "$f" .avif)
  for t in 1 2 4; do
    qemu-x86_64 -cpu Haswell-v4 $B/zz_planes_threads "$f" /tmp/p.raw /tmp/r.raw dav1d $t > /dev/null 2>> "$OUT/qemu_planes.err"
    echo "$name dav1d t$t rc=$? planes=$(sha256sum /tmp/p.raw | cut -c1-64) rgb=$(sha256sum /tmp/r.raw | cut -c1-64)" >> "$OUT/qemu_planes.txt"
    rm -f /tmp/p.raw /tmp/r.raw
  done
done
echo done >> "$OUT/qemu_planes.txt.done"
