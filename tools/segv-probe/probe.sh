#!/usr/bin/env bash
# Throwaway (v1/dav1d-x86-segv): runs the ferrisma-dav1d lib tests many times on this runner and records
# the CPU, every SIGSEGV's PC (resolved to a symbol) and a gdb backtrace. Not for main.
set -u
OUT=${OUT:-probe-out}; RUNS=${RUNS:-30}
mkdir -p "$OUT/traps"
{ grep -m1 'model name' /proc/cpuinfo; grep -m1 '^flags' /proc/cpuinfo; nproc; uname -a; } > "$OUT/cpu.txt"
cat "$OUT/cpu.txt" | head -1
cc -O1 -shared -fPIC -o "$OUT/segv_trap.so" tools/segv-probe/segv_trap.c
BIN=$(cargo test --locked --offline --release -p ferrisma-dav1d --lib --no-run --message-format=json 2>/dev/null \
  | python3 -c 'import json,sys
for l in sys.stdin:
    m=json.loads(l)
    if m.get("reason")=="compiler-artifact" and m.get("executable") and m["target"]["name"]=="dav1d" and m["profile"]["test"]: print(m["executable"])' | tail -1)
echo "BIN=$BIN" | tee -a "$OUT/cpu.txt"
"$BIN" --list > "$OUT/list.txt" 2>&1
fails=0
run() { # run MODE I ARGS...
  local mode=$1 i=$2; shift 2
  SEGVTRAP_DIR="$OUT/traps" LD_PRELOAD="$OUT/segv_trap.so" "$BIN" "$@" > "$OUT/$mode-$i.log" 2>&1
  local rc=$?
  echo "$mode $i rc=$rc" >> "$OUT/summary.txt"
  if [ $rc -ne 0 ]; then fails=$((fails+1)); tail -25 "$OUT/$mode-$i.log"; fi
}
for i in $(seq 1 "$RUNS"); do run par "$i"; done
for i in $(seq 1 5); do run serial "$i" --test-threads=1; done
# Every test alone, 3 times each.
while read -r t; do
  for i in 1 2 3; do run "one-${t//:/_}" "$i" --exact "$t" --test-threads=1; done
done < <(sed -n 's/: test$//p' "$OUT/list.txt")
# Resolve trap PCs against the binary's mapping.
for f in "$OUT"/traps/trap.*.txt; do
  [ -e "$f" ] || continue
  echo "== $f"; head -3 "$f"
  python3 - "$f" "$BIN" <<'PY'
import re, subprocess, sys
f, binp = sys.argv[1], sys.argv[2]
txt = open(f).read()
base = None
real = subprocess.run(["readlink", "-f", binp], capture_output=True, text=True).stdout.strip()
for line in txt.splitlines():
    m = re.match(r"([0-9a-f]+)-[0-9a-f]+ r.xp ([0-9a-f]+) \S+ \d+\s+(\S+)", line)
    if m and m.group(3) == real and base is None:
        base = int(m.group(1), 16) - int(m.group(2), 16)
    if m and m.group(3) == real and line.split()[1] == "r--p" and base is None:
        base = int(m.group(1), 16)
# first mapping of the binary is the load base for PIE
for line in txt.splitlines():
    parts = line.split()
    if len(parts) >= 6 and parts[-1] == real:
        base = int(parts[0].split("-")[0], 16); break
addrs = re.findall(r"(?:pc=|bt )(0x[0-9a-f]+)", txt)
offs = [hex(int(a, 16) - base) for a in addrs if base and int(a, 16) >= base]
if offs:
    out = subprocess.run(["addr2line", "-f", "-C", "-i", "-e", binp] + offs, capture_output=True, text=True).stdout
    print(out)
PY
done
# gdb: parallel runs (as CI) until the first SIGSEGV, then one serial run.
for i in $(seq 1 10); do
  gdb -q -batch -ex 'set pagination off' -ex run -ex bt -ex 'x/i $pc' -ex 'info registers' \
    -ex 'thread apply all bt 12' --args "$BIN" > "$OUT/gdb-par-$i.txt" 2>&1
  if grep -q SIGSEGV "$OUT/gdb-par-$i.txt"; then echo "gdb par $i: SIGSEGV"; grep -v '^\[' "$OUT/gdb-par-$i.txt" | tail -120; break; fi
done
# gdb: one serial run that stops at the first SIGSEGV.
gdb -q -batch -ex 'set pagination off' -ex run -ex bt -ex 'x/6i $pc-12' -ex 'x/i $pc' -ex 'info registers' \
  -ex 'thread apply all bt 8' --args "$BIN" --test-threads=1 > "$OUT/gdb-serial.txt" 2>&1
tail -60 "$OUT/gdb-serial.txt"
echo "failures: $fails"; grep -v 'rc=0' "$OUT/summary.txt" || true
