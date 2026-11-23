#!/bin/bash
# x86_corpus.sh OUT: inside the container, after x86_inner.sh: the reader's lib tests
# (release) natively under Rosetta with the SIGSEGV trap, and the thread-identity
# corpus test under qemu-x86_64 -cpu Haswell-v4.
set -u
OUT=$1; mkdir -p "$OUT/traps"
TESTBIN=$(ls -t target/release/build/ferrisma-avif-read/*/out/ferrisma_avif_read-* target/release/deps/ferrisma_avif_read-* 2>/dev/null | grep -v '\.d$' | head -1)
echo "testbin $TESTBIN" >> "$OUT/corpus-summary.txt"
if [ -z "${QEMU_ONLY:-}" ]; then
SEGVTRAP_DIR="$OUT/traps" LD_PRELOAD="$OUT/segv_trap.so" $TESTBIN --nocapture > "$OUT/reader_rosetta.log" 2>&1
echo "reader_rosetta rc=$?" >> "$OUT/corpus-summary.txt"
fi
command -v qemu-x86_64 >/dev/null || (apt-get update -qq && apt-get install -y -qq --no-install-recommends qemu-user) > /dev/null 2>&1
timeout 21600 qemu-x86_64 -cpu Haswell-v4 $TESTBIN --exact codec_identity_tests::dav1d_threads_decode_the_avif_corpus_identically --nocapture > "$OUT/corpus_qemu.log" 2>&1
echo "corpus_qemu rc=$?" >> "$OUT/corpus-summary.txt"
echo done >> "$OUT/corpus-summary.txt"
