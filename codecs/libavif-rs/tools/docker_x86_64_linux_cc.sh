#!/bin/sh
# C compiler and linker driver for x86_64-unknown-linux-gnu that runs GCC in a
# container on macOS ARM64 (Colima). It lets the host's Rust toolchain build
# the port's GNU Linux x86-64 profile without compiling inside the small Docker
# VM: rustc runs natively and only the final link runs in the container.
#
# Two images work:
# - the QEMU image of tools/qemu_x86_64/Dockerfile (an AArch64 container with
#   Debian's x86-64 cross GCC; the default route of
#   `check_aom_simd_original.py --qemu`, ADR 0058 amendment of 2026-11-14):
#     export AOM_LINUX_X86_64_IMAGE=ferrisma-aom-x86-qemu:trixie
#     export AOM_LINUX_X86_64_PLATFORM=linux/arm64
#     export AOM_LINUX_X86_64_CC=x86_64-linux-gnu-gcc
# - an x86-64 image with gcc and glibc development files (the defaults,
#   AOM_LINUX_X86_64_PLATFORM=linux/amd64 and AOM_LINUX_X86_64_CC=cc; the
#   linker then runs emulated).
#
#   export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=$PWD/tools/docker_x86_64_linux_cc.sh
#   export RUSTFLAGS='-Clinker-features=-lld -Clink-self-contained=-linker'
#   export TMPDIR=<a directory under $HOME>   # rustc's link inputs must be visible
#   cargo build --release --target x86_64-unknown-linux-gnu --example encode_still
#
# Colima shares $HOME with the VM at the same path; everything the link reads
# or writes (the target directory, the rustup toolchain, TMPDIR) must be there.
set -eu
: "${AOM_LINUX_X86_64_IMAGE:?set AOM_LINUX_X86_64_IMAGE}"
exec docker run --rm --platform "${AOM_LINUX_X86_64_PLATFORM:-linux/amd64}" -v "$HOME:$HOME" -w "$PWD" \
  "$AOM_LINUX_X86_64_IMAGE" "${AOM_LINUX_X86_64_CC:-cc}" "$@"
