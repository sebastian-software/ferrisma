#!/usr/bin/env bash
# Install the tools one v1.0 CI performance job needs.
#   bash scripts/bench/ci-perf/setup.sh engine|jpeg|webp|jxl|avif
# Linux uses apt (GNU time is required by run_interleaved.py), macOS brew.
# The jpeg-turbo keg (djpeg/cjpeg for the masters) is added to GITHUB_PATH.
set -euo pipefail
job=${1:?job}
if [[ "$(uname -s)" == Linux ]]; then
  packages=(time libjpeg-turbo-progs cmake build-essential pkg-config)
  case "$job" in
    engine) packages+=(webp libjxl-tools libavif-bin nasm) ;;
    jpeg) packages+=(nasm) ;;
    webp) ;;
    jxl) packages+=(ninja-build clang libc++-dev libc++abi-dev) ;;
    avif) packages+=(ninja-build meson nasm) ;;
  esac
  sudo apt-get update -qq
  sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq "${packages[@]}"
else
  packages=(jpeg-turbo)
  case "$job" in
    engine) packages+=(webp jpeg-xl libavif) ;;
    jxl) packages+=(ninja) ;;
    avif) packages+=(ninja meson pkg-config) ;;
  esac
  brew install --quiet "${packages[@]}" || brew install "${packages[@]}"
  prefix=$(brew --prefix jpeg-turbo)
  echo "$prefix/bin" >> "${GITHUB_PATH:-/dev/null}"
fi
rustup show active-toolchain
