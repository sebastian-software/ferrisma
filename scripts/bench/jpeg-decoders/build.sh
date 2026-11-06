#!/usr/bin/env bash
# Builds the ADR-0025 JPEG decoder comparison: pinned libjpeg-turbo and jpegli
# from source, the shared jpegdec.c harness against each, and the zune-jpeg
# harness. Needs git, cmake, ninja, a C/C++ compiler and cargo.
#
# usage: scripts/bench/jpeg-decoders/build.sh WORK_DIR
# Produces WORK_DIR/bin/{jpegdec-turbo,jpegdec-jpegli,zune,cjpeg-turbo}.
set -euo pipefail

TURBO_TAG=3.2.1
TURBO_REV=7b4993786d856cbccabfc0ed2f923ba000e326cc
JPEGLI_REV=031a0077f5799a6041004267fc12b956c1f52a20

here=$(cd "$(dirname "$0")" && pwd)
work=$(mkdir -p "${1:?usage: build.sh WORK_DIR}" && cd "$1" && pwd)
src=$work/src build=$work/build bin=$work/bin
mkdir -p "$src" "$build" "$bin"

if [[ ! -d $src/turbo ]]; then
  git clone -q --depth 1 --branch "$TURBO_TAG" https://github.com/libjpeg-turbo/libjpeg-turbo.git "$src/turbo"
fi
[[ $(git -C "$src/turbo" rev-parse HEAD) == "$TURBO_REV" ]] || { echo "libjpeg-turbo revision mismatch" >&2; exit 1; }
if [[ ! -d $src/jpegli ]]; then
  git clone -q https://github.com/google/jpegli.git "$src/jpegli"
  git -C "$src/jpegli" checkout -q "$JPEGLI_REV"
  git -C "$src/jpegli" submodule update -q --init --depth 1 \
    third_party/highway third_party/libjpeg-turbo third_party/skcms
fi
[[ $(git -C "$src/jpegli" rev-parse HEAD) == "$JPEGLI_REV" ]] || { echo "jpegli revision mismatch" >&2; exit 1; }

cmake -S "$src/turbo" -B "$build/turbo" -G Ninja -DCMAKE_BUILD_TYPE=Release \
  -DENABLE_SHARED=OFF -DWITH_TURBOJPEG=OFF -DWITH_TESTS=OFF >/dev/null
ninja -C "$build/turbo" jpeg-static cjpeg-static >/dev/null

cmake -S "$src/jpegli" -B "$build/jpegli" -G Ninja -DCMAKE_BUILD_TYPE=Release \
  -DBUILD_SHARED_LIBS=OFF -DBUILD_TESTING=OFF -DJPEGLI_ENABLE_TOOLS=OFF \
  -DJPEGLI_ENABLE_BENCHMARK=OFF -DJPEGLI_ENABLE_DOXYGEN=OFF -DJPEGLI_ENABLE_MANPAGES=OFF \
  -DJPEGLI_ENABLE_JNI=OFF -DJPEGLI_ENABLE_SJPEG=OFF -DJPEGLI_ENABLE_OPENEXR=OFF \
  -DJPEGLI_ENABLE_SKCMS=ON -DJPEGLI_ENABLE_FUZZERS=OFF >/dev/null
ninja -C "$build/jpegli" jpegli-static >/dev/null

cc -O2 -o "$bin/jpegdec-turbo" "$here/jpegdec.c" \
  -I"$src/turbo/src" -I"$build/turbo" "$build/turbo/libjpeg.a"
c++ -O2 -x c++ -DJPEGLI -c -o "$build/jpegdec-jpegli.o" "$here/jpegdec.c" \
  -I"$src/jpegli" -I"$build/jpegli/lib/include/jpegli"
c++ -o "$bin/jpegdec-jpegli" "$build/jpegdec-jpegli.o" \
  "$build/jpegli/lib/libjpegli-static.a" "$build/jpegli/third_party/highway/libhwy.a" -lpthread
cp "$build/turbo/cjpeg-static" "$bin/cjpeg-turbo"

cargo build -q --release --manifest-path "$here/zune/Cargo.toml" --target-dir "$build/zune"
cp "$build/zune/release/ferrixel-jpeg-decoders-zune" "$bin/zune"
echo "built: $(ls "$bin" | tr '\n' ' ')" >&2
