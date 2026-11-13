#!/bin/sh
# C compiler and linker driver for x86_64-unknown-linux-gnu that runs GCC in an
# x86-64 GNU Linux container (Colima with Rosetta 2 on macOS ARM64). It lets
# the host's Rust toolchain build the port's GNU Linux x86-64 profile without
# compiling inside the small Docker VM: rustc runs natively and only the final
# link runs in the container.
#
#   export AOM_LINUX_X86_64_IMAGE=<image with gcc and glibc development files>
#   export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=$PWD/tools/docker_x86_64_linux_cc.sh
#   export RUSTFLAGS='-Clinker-features=-lld -Clink-self-contained=-linker'
#   export TMPDIR=<a directory under $HOME>   # rustc's link inputs must be visible
#   cargo build --release --target x86_64-unknown-linux-gnu --example encode_still
#
# Colima shares $HOME with the VM at the same path; everything the link reads
# or writes (the target directory, the rustup toolchain, TMPDIR) must be there.
set -eu
: "${AOM_LINUX_X86_64_IMAGE:?set AOM_LINUX_X86_64_IMAGE}"
exec docker run --rm --platform linux/amd64 -v "$HOME:$HOME" -w "$PWD" "$AOM_LINUX_X86_64_IMAGE" cc "$@"
