#!/usr/bin/env bash
# macos_pass.sh N: one macOS ARM64 pass, run while holding both gate slots.
set -euo pipefail
N=$1
R=<workspace>/.claude/worktrees/prisma-release-review-419bce
W=$R/.wt/avif-dav1d-threads
E=$R/.orch/evidence/avif-dav1d-threads
ls $R/.orch/locks > $E/macos-pass$N-locks.txt
WT=$W bash $E/scripts/bench_threads.sh $E/macos-pass$N $W/target/threads-bench/bin-macos $W/target/threads-bench/inputs "1 2 4 10" 9
