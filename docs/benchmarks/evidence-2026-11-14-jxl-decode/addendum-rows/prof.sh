#!/bin/bash
# prof.sh BIN OUT : sample the port harness decoding photo24 8 times
cd /Users/sebastian/Workspace/ferrisma/.claude/worktrees/prisma-release-review-419bce/.wt/jxl-modular/target/ci-perf/rows-work
JXL_BENCH_REPEAT=8 $1 photo24-e7d1.jxl 0 default > /dev/null &
P=$!; sleep 0.3
/usr/bin/sample $P 30 1 -mayDie -file $2 >/dev/null 2>&1
wait $P
