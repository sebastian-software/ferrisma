#!/bin/sh
# run.sh COMMAND...: run COMMAND in the x86-64 GNU Linux container of this
# directory's Dockerfile (Rosetta, Colima profile dav1dx86), in the port
# directory (codecs/libavif-rs) of this checkout.
#
# The checkout is mounted at its host path. Its `codecs/libavif-rs/build` and
# `target` directories are container volumes: the host's hold macOS ARM64
# builds and stay untouched, and the x86-64 builds live on the VM's disk. The
# cargo registry is a volume as well. The host's `build/deps` (the fetched
# checkasm and test data, tools/import_dav1d.py --fetch) is visible read-only
# as /hostdeps, to be copied into the build volume once.
# DAV1D_X86_VOLUMES names the volumes' prefix (default: dav1dx86), so that two
# checkouts do not share them; DAV1D_X86_DOCKER_ARGS adds `docker run`
# arguments (further mounts, a working directory).
set -eu
PORT=$(cd "$(dirname "$0")/../.." && pwd)
CHECKOUT=$(cd "$PORT/../.." && pwd)
V=${DAV1D_X86_VOLUMES:-dav1dx86}
export DOCKER_CONTEXT=colima-dav1dx86
exec docker run --rm --platform linux/amd64 \
  -v "$CHECKOUT:$CHECKOUT" -v "$V-build:$PORT/build" -v "$V-target:$CHECKOUT/target" \
  -v "$V-cargo:/opt/cargo/registry" -v "$PORT/build/deps:/hostdeps:ro" -w "$PORT" \
  -e CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}" -e DAV1D_NINJA_JOBS="${DAV1D_NINJA_JOBS:-4}" \
  ${DAV1D_X86_DOCKER_ARGS:-} ferrisma-dav1d-x86:trixie "$@"
