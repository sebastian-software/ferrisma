#!/bin/bash
# x86_inner.sh OUT: runs inside the Colima linux/amd64 container from /src.
# 1. builds decode_still (this branch, and with origin/main's reader lib.rs),
#    zz_planes_threads and the reader's test binary (release);
# 2. bench_threads.sh at threads 1 2 4 (4 vCPUs) under the SIGSEGV trap;
# 3. QEMU Haswell-v4 cross-check (no Rosetta): zz_planes_threads dav1d t1/2/4
#    on all 16 inputs, and the reader's thread-identity corpus test;
# 4. the reader's thread-identity corpus test natively (Rosetta).
set -u
OUT=$1; mkdir -p "$OUT/traps"
export CARGO_BUILD_JOBS=4
cc -O1 -shared -fPIC -o "$OUT/segv_trap.so" /src/orchx/evidence/dav1d-641-segv/scripts/segv_trap.c
TRAP="$OUT/segv_trap.so"
cat /src/.head > "$OUT/head.txt"; uname -a > "$OUT/host.txt"; grep -m1 "model name" /proc/cpuinfo >> "$OUT/host.txt"
B=/src/target/threads-bench/bin-x86; mkdir -p $B
{
  set -e
  cargo build --release --locked -p ferrisma-avif-read --example decode_still
  cp target/release/examples/decode_still $B/decode_still-new
  cp crates/ferrisma-avif-read/src/lib.rs /tmp/lib.rs.new
  cp /src/orchx/lib.rs.main crates/ferrisma-avif-read/src/lib.rs
  cargo build --release --locked -p ferrisma-avif-read --example decode_still
  cp target/release/examples/decode_still $B/decode_still-main
  cp /tmp/lib.rs.new crates/ferrisma-avif-read/src/lib.rs
  cargo build --release --locked -p ferrisma-libavif --example zz_planes_threads
  cp target/release/examples/zz_planes_threads $B/
  cargo test --release --locked -p ferrisma-avif-read --lib --no-run 2>&1 | tee /tmp/norun.txt
} > "$OUT/build.log" 2>&1
echo "build rc=$?" >> "$OUT/summary.txt"
TESTBIN=$(ls -t target/release/deps/ferrisma_avif_read-* | grep -v '\.d$' | head -1)
echo "testbin $TESTBIN" >> "$OUT/summary.txt"
# 2. timing (Rosetta, AVX2 real), trap preloaded
SEGVTRAP_DIR="$OUT/traps" LD_PRELOAD="$TRAP" WT=/src RUNNER=/src/scripts/bench/ci-perf/run_interleaved.py \
  bash /src/orchx/evidence/avif-dav1d-threads/scripts/bench_threads.sh "$OUT/bench" $B /src/target/threads-bench/inputs "1 2 4" 9 \
  > "$OUT/bench.log" 2>&1
echo "bench rc=$?" >> "$OUT/summary.txt"
# 4. native corpus thread test
SEGVTRAP_DIR="$OUT/traps" LD_PRELOAD="$TRAP" $TESTBIN --exact codec_identity_tests::dav1d_threads_decode_the_avif_corpus_identically --nocapture > "$OUT/corpus_rosetta.log" 2>&1
echo "corpus_rosetta rc=$?" >> "$OUT/summary.txt"
# 3. QEMU Haswell-v4
(apt-get update -qq && apt-get install -y -qq --no-install-recommends qemu-user) > "$OUT/qemu-install.log" 2>&1
Q="qemu-x86_64 -cpu Haswell-v4"
: > "$OUT/qemu_planes.txt"
for f in /src/target/threads-bench/inputs/photo*.avif; do
  name=$(basename "$f" .avif)
  for t in 1 2 4; do
    $Q $B/zz_planes_threads "$f" /tmp/p.raw /tmp/r.raw dav1d $t > /dev/null 2>> "$OUT/qemu_planes.err"
    echo "$name dav1d t$t rc=$? planes=$(sha256sum /tmp/p.raw | cut -c1-64) rgb=$(sha256sum /tmp/r.raw | cut -c1-64)" >> "$OUT/qemu_planes.txt"
  done
done
echo "qemu_planes done" >> "$OUT/summary.txt"
timeout 14400 $Q $TESTBIN --exact codec_identity_tests::dav1d_threads_decode_the_avif_corpus_identically --nocapture > "$OUT/corpus_qemu.log" 2>&1
echo "corpus_qemu rc=$?" >> "$OUT/summary.txt"
echo done >> "$OUT/summary.txt"
