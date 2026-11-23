#!/bin/bash
# x86.sh: copy the working tree (tracked files, the scratch example and the
# 16 inputs) into Colima volumes and run x86_inner.sh on linux/amd64.
# Never stops or restarts the shared VM.
set -u
ORCH=<orch>
WT=$ORCH/../.wt/avif-dav1d-threads
E=$ORCH/evidence/avif-dav1d-threads
export DOCKER_CONTEXT=colima-dav1dx86
IMG=ferrisma-dav1d-x86:trixie; V=avifdav1dthr
tag=$(git -C "$WT" rev-parse --short HEAD)-wt
D="docker run --rm --platform linux/amd64"
$D -v $V-src:/src $IMG sh -c 'find /src -mindepth 1 -maxdepth 1 ! -name target -exec rm -rf {} +'
(cd "$WT" && { git ls-files -z; printf 'codecs/libavif-rs/examples/zz_planes_threads.rs\0'; } | COPYFILE_DISABLE=1 tar --null -T - -cf -) | $D -i -v $V-src:/src $IMG tar -C /src -xf -
git -C "$WT" rev-parse HEAD | $D -i -v $V-src:/src $IMG sh -c 'cat > /src/.head'
git -C "$WT" show origin/main:crates/ferrisma-avif-read/src/lib.rs > "$E/lib.rs.main"
tar -C "$ORCH" -cf - evidence/avif-dav1d-threads/scripts evidence/dav1d-641-segv/scripts/segv_trap.c \
  | $D -i -v $V-src:/src $IMG sh -c 'mkdir -p /src/orchx && tar -C /src/orchx -xf -'
$D -i -v $V-src:/src $IMG sh -c 'cat > /src/orchx/lib.rs.main' < "$E/lib.rs.main"
(cd "$WT/target/threads-bench" && COPYFILE_DISABLE=1 tar -cf - inputs) \
  | $D -i -v $V-target:/t $IMG sh -c 'mkdir -p /t/threads-bench && tar -C /t/threads-bench -xf -'
$D --dns 1.1.1.1 -v $V-src:/src -v $V-target:/src/target -v $V-build:/src/codecs/libavif-rs/build \
  -v $V-cargo:/opt/cargo/registry -v $V-out:/out -w /src \
  $IMG bash /src/orchx/evidence/avif-dav1d-threads/scripts/x86_inner.sh /out/x86-$tag
rc=$?
$D -v $V-out:/out $IMG tar -C /out -cf - x86-$tag | tar -C "$E" -xf -
echo "x86 container rc=$rc"
