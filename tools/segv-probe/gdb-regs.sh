#!/usr/bin/env bash
# Throwaway: runs the dav1d lib tests under gdb (as CI, in parallel) until a SIGSEGV and dumps the vector state.
set -u
grep -m1 'model name' /proc/cpuinfo
BIN=$(cargo test --locked --offline --release -p ferrisma-dav1d --lib --no-run --message-format=json \
  | jq -r 'select(.reason == "compiler-artifact" and .executable != null and .profile.test) | .executable' | tail -1)
for i in $(seq 1 60); do
  gdb -q -batch -ex 'set pagination off' -ex run \
    -ex 'p $pc' -ex 'p/x $rcx' -ex 'p/x $rsp' -ex 'p $_siginfo._sifields._sigfault.si_addr' \
    -ex 'p/x $ymm3.v8_int32' -ex 'p/x $ymm4.v8_int32' -ex 'p/x $ymm11.v4_int64' -ex 'p/x $ymm2.v4_int64' -ex 'p/x $ymm7.v16_int16' \
    -ex 'x/60i $pc-200' -ex 'x/64xb $rcx-32' --args "$BIN" > gdb-$i.txt 2>&1
  if grep -q 'SIGSEGV' gdb-$i.txt; then echo "attempt $i: SIGSEGV"; grep -v '^\[' gdb-$i.txt | grep -v '^test ' | tail -120; exit 0; fi
done
echo "no SIGSEGV in 60 attempts"
