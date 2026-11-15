#!/usr/bin/env bash
# Builds the pinned original libjxl (submodule codecs/libjxl-rs/upstream/libjxl)
# with cjxl, djxl and ssimulacra2, as scripts/bench/ci-perf/codec_jxl.sh builds
# it (Release, same dependency selection; clang when present), plus the
# devtools switch that provides the ssimulacra2 tool.
#   usage: build_libjxl.sh BUILD_DIR      (outside the repository; tools end up in BUILD_DIR/tools)
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
BUILD=${1:?usage: build_libjxl.sh BUILD_DIR}
JOBS=${JOBS:-4}
UPSTREAM=$ROOT/codecs/libjxl-rs/upstream/libjxl
[[ -f "$UPSTREAM/CMakeLists.txt" ]] || git -C "$ROOT" submodule update --init codecs/libjxl-rs/upstream/libjxl
for dep in highway brotli skcms; do
  [[ -n "$(ls -A "$UPSTREAM/third_party/$dep" 2>/dev/null)" ]] || git -C "$UPSTREAM" submodule update --init "third_party/$dep"
done
CC_JXL=$(command -v clang || command -v cc); CXX_JXL=$(command -v clang++ || command -v c++)
GEN=(); command -v ninja >/dev/null && GEN=(-G Ninja)
cmake -S "$UPSTREAM" -B "$BUILD" ${GEN[@]+"${GEN[@]}"} \
  -DCMAKE_C_COMPILER="$CC_JXL" -DCMAKE_CXX_COMPILER="$CXX_JXL" \
  -DCMAKE_BUILD_TYPE=Release -DCMAKE_POLICY_VERSION_MINIMUM=3.5 \
  -DBUILD_TESTING=OFF -DBUILD_SHARED_LIBS=OFF \
  -DJPEGXL_ENABLE_TOOLS=ON -DJPEGXL_ENABLE_DEVTOOLS=ON -DJPEGXL_ENABLE_BENCHMARK=OFF \
  -DJPEGXL_ENABLE_EXAMPLES=OFF -DJPEGXL_ENABLE_DOXYGEN=OFF -DJPEGXL_ENABLE_MANPAGES=OFF \
  -DJPEGXL_ENABLE_JNI=OFF -DJPEGXL_ENABLE_SJPEG=OFF -DJPEGXL_ENABLE_OPENEXR=OFF \
  -DJPEGXL_ENABLE_SKCMS=ON -DJPEGXL_ENABLE_VIEWERS=OFF -DJPEGXL_ENABLE_PLUGINS=OFF \
  -DJPEGXL_ENABLE_FUZZERS=OFF -DJPEGXL_ENABLE_TCMALLOC=OFF -DJPEGXL_BUNDLE_LIBPNG=OFF \
  -DHWY_ENABLE_CONTRIB=OFF -DHWY_ENABLE_EXAMPLES=OFF \
  -DCMAKE_DISABLE_FIND_PACKAGE_GIF=ON -DCMAKE_DISABLE_FIND_PACKAGE_JPEG=ON \
  -DCMAKE_DISABLE_FIND_PACKAGE_PNG=ON -DCMAKE_DISABLE_FIND_PACKAGE_ZLIB=ON
cmake --build "$BUILD" --parallel "$JOBS" --target cjxl djxl ssimulacra2
echo "tools in $BUILD/tools"
