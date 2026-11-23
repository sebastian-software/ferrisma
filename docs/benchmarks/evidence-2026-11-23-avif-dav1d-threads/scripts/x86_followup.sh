#!/bin/bash
# x86_followup.sh: copy the follow-up scripts in and run the corpus tests and the QEMU plane pass.
set -u
ORCH=<orch>
E=$ORCH/evidence/avif-dav1d-threads
export DOCKER_CONTEXT=colima-dav1dx86
IMG=ferrisma-dav1d-x86:trixie; V=avifdav1dthr; O=/out/x86-d886949e-wt
D="docker run --rm --platform linux/amd64"
tar -C "$ORCH" -cf - evidence/avif-dav1d-threads/scripts | $D -i -v $V-src:/src $IMG sh -c 'tar -C /src/orchx -xf -'
$D --dns 1.1.1.1 -v $V-src:/src -v $V-target:/src/target -v $V-build:/src/codecs/libavif-rs/build -v $V-cargo:/opt/cargo/registry -v $V-out:/out -w /src \
  $IMG bash -c "bash /src/orchx/evidence/avif-dav1d-threads/scripts/x86_qemu_planes.sh $O; QEMU_ONLY=1 bash /src/orchx/evidence/avif-dav1d-threads/scripts/x86_corpus.sh $O"
rc=$?
$D -v $V-out:/out $IMG tar -C /out -cf - x86-d886949e-wt | tar -C "$E" -xf -
echo "followup rc=$rc"
