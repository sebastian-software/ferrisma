# x86-64 GNU Linux image for the local x86-64 gates of the dav1d kernels
# (ADR 0062): the original with its assembly, the port, checkasm and Valgrind,
# all as x86-64 processes that Rosetta executes in a Linux VM on the
# development Mac. Rosetta reports and executes SSE2 to AVX2 there (CPUID
# leaf 7 and XCR0; no AVX-512), so dav1d's own detection selects the AVX2
# kernels without any setting. Test tooling only; nothing of it ships.
#
# Once, a Colima VM with Rosetta beside the default one (it does not become
# the active Docker context), with the checkout mounted writable:
#
#   colima start --profile dav1dx86 --activate=false --vm-type vz --vz-rosetta \
#     --cpus 4 --memory 8 --disk 40 --mount-type virtiofs --mount "<checkout>:w"
#   DOCKER_CONTEXT=colima-dav1dx86 docker build --platform linux/amd64 \
#     -t ferrisma-dav1d-x86:trixie tools/rosetta_x86_64
#
# Then `tools/rosetta_x86_64/run.sh COMMAND...` runs COMMAND in the port
# directory inside the container (see the script for the volumes), for
# example the gates of ADR 0062 at the AVX2 level:
#
#   tools/rosetta_x86_64/run.sh sh -c 'cp -R /hostdeps build/deps &&
#     python3 tools/build_dav1d.py && python3 tools/build_dav1d.py --debug &&
#     python3 tools/build_dav1d.py --asm'                       # once
#   tools/rosetta_x86_64/run.sh python3 tools/build_dav1d_consumers.py
#   tools/rosetta_x86_64/run.sh python3 tools/check_dav1d_dsp.py --simd avx2
#   tools/rosetta_x86_64/run.sh python3 tools/check_dav1d_simd_original.py --level avx2
#
# `cc` is Clang, as on the hosted Linux runner (`CC=clang-16` there):
# tools/check_dav1d_abi.py reads Clang's AST of the original.
#
# Rosetta is an emulator: it does not fault on a misaligned aligned move, and
# no timing taken under it is a measurement of x86-64 hardware (engine
# ADR-0041 section 5). Valgrind does fault on the former, and its Callgrind
# counts the x86-64 instructions a process executes
# (tools/measure_dav1d_kernels.py).
FROM debian:trixie
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    gcc g++ clang libc6-dev binutils make cmake ninja-build meson nasm perl python3 pkg-config git curl \
    ca-certificates file procps time valgrind gdb && rm -rf /var/lib/apt/lists/* \
    && update-alternatives --install /usr/bin/cc cc /usr/bin/clang 100
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain nightly-2026-09-07 \
    --component rustfmt --component clippy && rustc -vV && cargo -V
