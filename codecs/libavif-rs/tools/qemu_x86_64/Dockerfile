# AArch64 GNU Linux image for the local x86-64 SIMD-against-original gate
# (tools/check_aom_simd_original.py --qemu; ADR 0058, amendment of 2026-11-14)
# and for dav1d's assembly oracle (tools/build_dav1d.py --asm --cross
# x86_64-linux-gnu; ADR 0062), which needs Meson.
# It runs natively in Colima's AArch64 VM. x86-64 binaries are built with
# Debian's cross GCC and run under Debian's qemu-user (10.0 in trixie) with
# `-cpu max`, which executes AVX2, FMA and BMI2 and reports no AVX-512. The
# VM's own binfmt handler for linux/amd64 images is QEMU 7.0, which has no AVX.
#
#   docker build --platform linux/arm64 -t ferrisma-aom-x86-qemu:trixie tools/qemu_x86_64
FROM debian:trixie
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    qemu-user gcc-x86-64-linux-gnu g++-x86-64-linux-gnu libc6-dev-amd64-cross \
    cmake ninja-build meson nasm perl python3 libjpeg-turbo-progs ca-certificates file && rm -rf /var/lib/apt/lists/*
