#!/bin/bash
# The libjpeg-turbo-rs gates of tools/check.py's test stage, per oracle,
# SIMD mode and profile, without the T.83 conformance target (its ITU data
# is not downloaded on this host). Usage: gates.sh <logdir> <arch: native|x86_64> [profiles]
set -u
LOG=$1; ARCH=$2; PROFILES=${3:-"dev release checked"}
C=$(cd "$(dirname "$0")/../../../codecs/libjpeg-turbo-rs" && pwd)
cd "$C" || exit 1
mkdir -p "$LOG"
export CARGO_BUILD_JOBS=5
unset JSIMD_FORCENONE JSIMD_FORCESSE2 LIBJPEG_EXPECT_SIMD LIBJPEG_T83_DIR
TESTS=""
for t in tests/*.rs; do n=$(basename "$t" .rs); [ "$n" = wp11_conformance ] || TESTS="$TESTS --test $n"; done
if [ "$ARCH" = x86_64 ]; then
  TARGET="--target x86_64-apple-darwin"; SUFFIX="-x86_64"; BTARCH="--arch x86_64"
  export ROSETTA_ADVERTISE_AVX=1
  MODES="avx2: sse2:JSIMD_FORCESSE2=1 none:JSIMD_FORCENONE=1"
else
  TARGET=""; SUFFIX=""; BTARCH=""
  MODES="neon: none:JSIMD_FORCENONE=1"
fi
status=0
step() { # name, command...
  local name=$1; shift
  local t0=$(date +%s)
  if "$@" > "$LOG/$name.log" 2>&1; then r=PASS; else r=FAIL; status=1; fi
  local summary=$(grep -E "^test result:" "$LOG/$name.log" | awk '{p+=$4; f+=$6; i+=$8} END {print p" passed, "f" failed, "i" ignored"}')
  echo "$r $name ($(( $(date +%s) - t0 ))s; $summary; load $(sysctl -n vm.loadavg | awk '{print $2}'))" | tee -a "$LOG/summary.txt"
}
prof() { case $1 in dev) echo "";; release) echo "--release";; checked) echo "--profile checked";; esac; }
for v in release simd; do
  step "bittests-$v$SUFFIX" python3 tools/bittests.py --check --run --variant $v --jobs 5 $BTARCH
done
for p in $PROFILES; do
  step "release$SUFFIX-$p" env LIBJPEG_ORACLE_DIR=$C/build/oracle/release$SUFFIX \
    cargo test --locked --offline $TARGET $(prof $p) --no-default-features --features oracle --lib $TESTS
  for m in $MODES; do
    name=${m%%:*}; var=${m#*:}
    step "simd$SUFFIX-$name-$p" env LIBJPEG_ORACLE_DIR=$C/build/oracle/simd$SUFFIX LIBJPEG_EXPECT_SIMD=$name $var \
      cargo test --locked --offline $TARGET $(prof $p) --features oracle --lib $TESTS
  done
done
exit $status
