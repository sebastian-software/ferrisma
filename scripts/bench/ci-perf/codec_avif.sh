#!/usr/bin/env bash
# CI performance comparison: libavif-rs (the Rust port, packages libavif-rs
# [lib `avif`], libavif-aom, libavif-dav1d) versus the pinned originals it
# translates: libavif 1.4.2, libaom 3.15.2 and dav1d 1.5.4.
#
#   INPUTS=dir [SIZES="12 24"] [ENCODE_SIZES="12"] [WORK=dir] [OUT=dir] \
#     [REPS=7] [WARMUPS=1] [JOBS=n] bash scripts/bench/ci-perf/codec_avif.sh
#
# Platforms: the port compiles only on macOS ARM64 and GNU Linux x86-64 with
# the pinned nightly (rust-toolchain.toml). On any other platform (notably
# Linux ARM64) the script writes $OUT/avif-skipped.json and exits 0.
#
# Operations (C and port in ONE interleaved plan per size and thread mode):
#   encode  still encode at the engine's libavif defaults (src/pipeline.rs,
#           src/own_avif.rs, crates/ferrisma-avif-write): 10-bit, 4:4:4,
#           quality 75, speed 6, BT.709/sRGB/BT.601 full range, all-intra
#           still, maxThreads=threads and autoTiling when threads != 1.
#           C: codecs/libavif-rs/tools/encode_still.c (copied to $WORK and
#           given an `encode_us=` timer around avifEncoderWrite) linked to the
#           release libavif+libaom built here; port:
#           codecs/libavif-rs/examples/encode_still.rs (same calls, same
#           `encode_us=` region). Input photoN.bmp (ppm_to.py bmp). t1 and tN
#           (all cores). Only the sizes in ENCODE_SIZES (default: the first
#           of SIZES) are encoded, to keep the job inside its time budget.
#   decode  AV1 still decode to 8-bit RGB with nearest chroma upsampling, the
#           way the engine reads AVIF (crates/ferrisma-avif-read: libavif
#           port with its AOM decoder, libavif's default maxThreads=1).
#           C: a small libavif driver written to $WORK (decode_still.c below)
#           with `-c aom` and `-c dav1d`; port:
#           crates/ferrisma-avif-read/examples/decode_still.rs. Both print
#           `elapsed_ms=` for decoder create + parse + decode + YUV->RGB and
#           write the RGB as PPM outside the timer. t1 compares all three;
#           tN runs only the C decoders (the safe reader has no thread knob,
#           the engine decodes AVIF on one thread), as a reference.
# Decode inputs: photoN-q60.avif made by the ORIGINAL (C encode_still,
# 8-bit 4:2:0, quality 60, speed 6, one thread, no autotiling: one tile, so
# the bitstream does not depend on the runner's core count).
#
# Encoder byte identity: the port's AOM SIMD kernels are byte-identical to
# libaom's `_c` paths (ADR 0056/0058), while the upstream release C build
# uses its own SIMD (on ARM64 also dotprod/i8mm/SVE kernels) and the
# compiler's default FP contraction, so C and port AVIF bytes are expected to
# differ slightly; the difference is recorded, not treated as a failure.
#
# Outputs in $OUT: avif-build.json (cmake/meson flags, compiler, versions,
# SIMD/asm configuration of both sides), avif-inputs.json,
# avif-<op>-<size>-<t1|tN>.json (run_interleaved.py) and avif-checks.json
# (C/port encoder byte identity per size and thread mode; decoded RGB
# identity between C aom, C dav1d and the port).
#
# System packages:
#   Linux x86-64 (apt): cmake ninja-build meson nasm pkg-config
#     build-essential python3 time
#   macOS ARM64 (brew): cmake ninja meson pkg-config (nasm not needed)
#   plus rustup (the repository's pinned nightly is installed from
#   rust-toolchain.toml).
# Submodules: none; libavif, aom and dav1d are tracked snapshots under
# codecs/libavif-rs/upstream.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
SIZES=${SIZES:-"12 24"}
read -r -a SIZE_LIST <<<"$SIZES"
ENCODE_SIZES=${ENCODE_SIZES:-${SIZE_LIST[0]}}
WORK=${WORK:-$ROOT/target/ci-perf/avif-work}
OUT=${OUT:-$ROOT/target/ci-perf/avif-out}
REPS=${REPS:-7}
WARMUPS=${WARMUPS:-1}
JOBS=${JOBS:-$(getconf _NPROCESSORS_ONLN)}
NCPU=$(getconf _NPROCESSORS_ONLN)
mkdir -p "$WORK" "$OUT"
WORK=$(cd "$WORK" && pwd)
OUT=$(cd "$OUT" && pwd)
INPUTS=$(cd "$INPUTS" && pwd)

SYSTEM=$(uname -s)
MACHINE=$(uname -m)
case "$SYSTEM/$MACHINE" in
  Darwin/arm64 | Linux/x86_64) ;;
  *)
    printf '{"codec": "avif", "skipped": true, "platform": "%s/%s", "reason": "%s"}\n' \
      "$SYSTEM" "$MACHINE" \
      "libavif-rs compiles only on macOS ARM64 and GNU Linux x86-64 (ADR-0026, codecs/libavif-rs README)" \
      >"$OUT/avif-skipped.json"
    echo "avif: skipped on $SYSTEM/$MACHINE" >&2
    exit 0
    ;;
esac

UPSTREAM=$ROOT/codecs/libavif-rs/upstream
PREFIX=$WORK/c-prefix
export PKG_CONFIG_PATH=$PREFIX/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}

# 1. The originals with their upstream release configurations, upstream
# default SIMD/RTCD (libaom; nasm on x86-64) and assembly (dav1d). Only
# targets are trimmed: no tests, examples, docs, tools or apps.
AOM_FLAGS=(-G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX="$PREFIX"
  -DCMAKE_INSTALL_LIBDIR=lib -DENABLE_TESTS=0 -DENABLE_TESTDATA=0
  -DENABLE_EXAMPLES=0 -DENABLE_DOCS=0 -DENABLE_TOOLS=0 -DENABLE_APPS=0)
cmake -S "$UPSTREAM/aom" -B "$WORK/aom-release" "${AOM_FLAGS[@]}" >"$WORK/aom-configure.log"
cmake --build "$WORK/aom-release" --parallel "$JOBS" --target install >"$WORK/aom-build.log"

DAV1D_FLAGS=(--buildtype=release --default-library=static --prefix="$PREFIX" --libdir=lib
  -Denable_tools=false -Denable_tests=false -Denable_examples=false)
if [[ ! -f "$WORK/dav1d-release/build.ninja" ]]; then
  meson setup "$WORK/dav1d-release" "$UPSTREAM/dav1d" "${DAV1D_FLAGS[@]}" >"$WORK/dav1d-configure.log"
fi
ninja -C "$WORK/dav1d-release" -j "$JOBS" install >"$WORK/dav1d-build.log"

AVIF_FLAGS=(-G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX="$PREFIX"
  -DCMAKE_INSTALL_LIBDIR=lib -DCMAKE_PREFIX_PATH="$PREFIX" -DBUILD_SHARED_LIBS=OFF
  -DAVIF_CODEC_AOM=SYSTEM -DAVIF_CODEC_DAV1D=SYSTEM -DAVIF_LIBYUV=OFF
  -DAVIF_LIBSHARPYUV=OFF -DAVIF_BUILD_APPS=OFF -DAVIF_BUILD_TESTS=OFF
  -DAVIF_BUILD_EXAMPLES=OFF)
cmake -S "$UPSTREAM/libavif" -B "$WORK/libavif-release" "${AVIF_FLAGS[@]}" >"$WORK/libavif-configure.log"
cmake --build "$WORK/libavif-release" --parallel "$JOBS" --target install >"$WORK/libavif-build.log"

# The C drivers. encode_still.c is the repository's own C counterpart of the
# port example; the copy only adds the encode_us timer and corrects the RTCD
# note (this libaom is a release build with its own SIMD dispatch).
mkdir -p "$WORK/c-bin"
python3 -I - "$ROOT/codecs/libavif-rs/tools/encode_still.c" "$WORK/c-bin/encode_still.c" <<'EOF'
import sys
from pathlib import Path
source = Path(sys.argv[1]).read_text()
call = "        if (avifEncoderWrite(encoder, image, &output) != AVIF_RESULT_OK) {\n"
size = '        printf("bytes=%zu\\n", output.size);\n'
note = 'printf("rtcd=c(reference)\\n");'
assert source.count(call) == 1 and source.count(size) == 1 and source.count(note) == 1
source = source.replace("#include <unistd.h>\n", "#include <time.h>\n#include <unistd.h>\n", 1)
source = source.replace(call, "        struct timespec start, stop;\n"
                        "        clock_gettime(CLOCK_MONOTONIC, &start);\n"
                        "        avifResult written = avifEncoderWrite(encoder, image, &output);\n"
                        "        clock_gettime(CLOCK_MONOTONIC, &stop);\n"
                        "        if (written != AVIF_RESULT_OK) {\n")
source = source.replace(size, '        printf("encode_us=%lld bytes=%zu\\n",\n'
                        "               (long long)(stop.tv_sec - start.tv_sec) * 1000000 +\n"
                        "                   (stop.tv_nsec - start.tv_nsec) / 1000,\n"
                        "               output.size);\n")
source = source.replace(note, 'printf("rtcd=upstream-release(runtime-cpu-detect)\\n");')
Path(sys.argv[2]).write_text(source)
EOF
cat >"$WORK/c-bin/decode_still.c" <<'EOF'
// C counterpart of crates/ferrisma-avif-read/examples/decode_still.rs:
//   decode_still aom|dav1d THREADS input.avif output.ppm
// Same libavif settings as the safe reader (primary item, one image, nearest
// chroma upsampling to 8-bit RGB). elapsed_ms covers decoder creation, parse,
// decode and YUV->RGB; reading the input and writing the PPM are outside.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "avif/avif.h"

static void fail(const char * message)
{
    fprintf(stderr, "decode_still: %s\n", message);
    exit(1);
}

int main(int argc, char ** argv)
{
    if (argc != 5) {
        fail("usage: decode_still aom|dav1d THREADS input.avif output.ppm");
    }
    avifCodecChoice choice = !strcmp(argv[1], "aom") ? AVIF_CODEC_CHOICE_AOM
                             : !strcmp(argv[1], "dav1d") ? AVIF_CODEC_CHOICE_DAV1D
                                                         : AVIF_CODEC_CHOICE_AUTO;
    if (choice == AVIF_CODEC_CHOICE_AUTO) {
        fail("codec must be aom or dav1d");
    }
    int threads = atoi(argv[2]);
    FILE * file = fopen(argv[3], "rb");
    if (!file) {
        fail("read input");
    }
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    uint8_t * data = malloc((size_t)size);
    if (!data || fread(data, 1, (size_t)size, file) != (size_t)size) {
        fail("read input");
    }
    fclose(file);

    struct timespec start, stop;
    clock_gettime(CLOCK_MONOTONIC, &start);
    avifDecoder * decoder = avifDecoderCreate();
    if (!decoder) {
        fail("avifDecoderCreate");
    }
    decoder->codecChoice = choice;
    decoder->maxThreads = threads;
    decoder->requestedSource = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
    decoder->imageSizeLimit = 0xFFFFFFFFu;
    decoder->imageCountLimit = 1;
    if (avifDecoderSetIOMemory(decoder, data, (size_t)size) != AVIF_RESULT_OK ||
        avifDecoderParse(decoder) != AVIF_RESULT_OK) {
        fail("parse");
    }
    if (avifDecoderNthImage(decoder, 0) != AVIF_RESULT_OK) {
        fail("decode");
    }
    avifRGBImage rgb;
    avifRGBImageSetDefaults(&rgb, decoder->image);
    rgb.depth = 8;
    rgb.format = AVIF_RGB_FORMAT_RGB;
    rgb.alphaPremultiplied = AVIF_FALSE;
    rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
    if (avifRGBImageAllocatePixels(&rgb) != AVIF_RESULT_OK || avifImageYUVToRGB(decoder->image, &rgb) != AVIF_RESULT_OK) {
        fail("YUV to RGB");
    }
    clock_gettime(CLOCK_MONOTONIC, &stop);

    FILE * out = fopen(argv[4], "wb");
    if (!out) {
        fail("write output");
    }
    fprintf(out, "P6\n%u %u\n255\n", rgb.width, rgb.height);
    for (uint32_t y = 0; y < rgb.height; ++y) {
        if (fwrite(rgb.pixels + (size_t)y * rgb.rowBytes, 1, (size_t)rgb.width * 3, out) != (size_t)rgb.width * 3) {
            fail("write output");
        }
    }
    if (fclose(out) != 0) {
        fail("write output");
    }
    printf("elapsed_ms=%.3f width=%u height=%u codec=%s threads=%d\n",
           (double)(stop.tv_sec - start.tv_sec) * 1000.0 + (double)(stop.tv_nsec - start.tv_nsec) / 1e6,
           rgb.width, rgb.height, argv[1], threads);
    avifRGBImageFreePixels(&rgb);
    avifDecoderDestroy(decoder);
    free(data);
    return 0;
}
EOF
CC=${CC:-cc}
C_LIBS=("$PREFIX/lib/libavif.a" "$PREFIX/lib/libaom.a" "$PREFIX/lib/libdav1d.a" -lpthread -lm)
[[ "$SYSTEM" == Linux ]] && C_LIBS+=(-ldl)
for driver in encode_still decode_still; do
  "$CC" -O2 -I"$PREFIX/include" -o "$WORK/c-bin/$driver" "$WORK/c-bin/$driver.c" "${C_LIBS[@]}"
done
C_ENC=$WORK/c-bin/encode_still
C_DEC=$WORK/c-bin/decode_still

# 2. The port: the repository release profile, no RUSTFLAGS.
(cd "$ROOT" && cargo build --release --locked -p libavif-rs --example encode_still)
(cd "$ROOT" && cargo build --release --locked -p ferrisma-avif-read --example decode_still)
EXAMPLES=${CARGO_TARGET_DIR:-$ROOT/target}/release/examples
[[ "$EXAMPLES" = /* ]] || EXAMPLES=$ROOT/$EXAMPLES
P_ENC=$EXAMPLES/encode_still
P_DEC=$EXAMPLES/decode_still

# RTCD selection of both encoders, on a tiny image (cheap).
python3 -I - "$WORK/tiny.bmp" <<'EOF'
import struct, sys
w = h = 64
pixels = b"".join(bytes(((x * 4) % 256, (y * 4) % 256, ((x + y) * 2) % 256)) for y in range(h) for x in range(w))
header = struct.pack("<2sIHHI", b"BM", 54 + len(pixels), 0, 0, 54)
info = struct.pack("<IiiHHIIiiII", 40, w, -h, 1, 24, 0, len(pixels), 2835, 2835, 0, 0)
open(sys.argv[1], "wb").write(header + info + pixels)
EOF
RTCD_C=$("$C_ENC" "$WORK/tiny.bmp" "$WORK/tiny-c.avif" --report-rtcd | sed -n 's/^rtcd=//p')
RTCD_PORT=$("$P_ENC" "$WORK/tiny.bmp" "$WORK/tiny-port.avif" --report-rtcd | sed -n 's/^rtcd=//p')

CC_VERSION=$("$CC" --version | head -n 1)
python3 -I - "$OUT/avif-build.json" "$WORK" "$UPSTREAM" "$ROOT" "$CC_VERSION" "$RTCD_C" "$RTCD_PORT" \
  "${AOM_FLAGS[*]}" "${DAV1D_FLAGS[*]}" "${AVIF_FLAGS[*]}" <<'EOF'
import json, re, subprocess, sys
from pathlib import Path
out, work, upstream, root, cc, rtcd_c, rtcd_port, aom, dav1d, avif = sys.argv[1:]
work, upstream, root = Path(work), Path(upstream), Path(root)
def run(*cmd):
    try:
        return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout.strip()
    except Exception as error:
        return f"unavailable: {error}"
def defines(path, pattern):
    try:
        text = Path(path).read_text()
    except OSError:
        return {}
    return {m.group(1): m.group(2) for m in re.finditer(r"#define (" + pattern + r")\s+(\S+)", text)}
provenance = root / "codecs/libavif-rs/provenance"
upstream_json = json.loads((root / "codecs/libavif-rs/UPSTREAM.json").read_text())
record = {
    "codec": "avif",
    "original": {
        "libavif": {"version": upstream_json["version"], "revision": upstream_json["revision"],
                    "source": "codecs/libavif-rs/upstream/libavif (tracked snapshot)",
                    "cmake_flags": avif},
        "libaom": {**{k: json.loads((provenance / "aom-source.json").read_text())[k] for k in ("component", "revision", "tag")},
                   "source": "codecs/libavif-rs/upstream/aom (tracked snapshot)",
                   "cmake_flags": aom,
                   "config_simd": defines(work / "aom-release/config/aom_config.h",
                                          r"(?:HAVE_(?:NEON\w*|SVE\w*|ARM_CRC32|MMX|SSE\w*|SSSE3|AVX\w*)|CONFIG_RUNTIME_CPU_DETECT|AOM_ARCH_\w+|CONFIG_REALTIME_ONLY|CONFIG_AV1_HIGHBITDEPTH|CONFIG_MULTITHREAD)"),
                   "rtcd_selection": rtcd_c},
        "dav1d": {**{k: json.loads((provenance / "dav1d-source.json").read_text())[k] for k in ("component", "revision", "tag")},
                  "source": "codecs/libavif-rs/upstream/dav1d (tracked snapshot)",
                  "meson_flags": dav1d,
                  "config": defines(work / "dav1d-release/config.h", r"(?:HAVE_ASM|ARCH_\w+|CONFIG_\w+|HAVE_AVX512ICL|HAVE_DOTPROD|HAVE_I8MM|HAVE_SVE2?)")},
        "repo_tooling": ("not reused: tools/build_aom.py and tools/build.py build the portable "
                         "AOM_TARGET_CPU=generic, -ffp-contract=off oracle (no SIMD); this script "
                         "builds the upstream release configuration instead"),
        "compiler": cc,
        "cmake": run("cmake", "--version").splitlines()[0],
        "meson": run("meson", "--version"),
        "drivers": {"encode": "tools/encode_still.c + encode_us timer (copied)",
                    "decode": "decode_still.c written by codec_avif.sh", "cflags": "-O2"},
    },
    "port": {
        "packages": ["libavif-rs (lib avif)", "libavif-aom", "libavif-dav1d", "ferrisma-avif-read"],
        "build": "cargo build --release --locked -p libavif-rs --example encode_still; "
                 "cargo build --release --locked -p ferrisma-avif-read --example decode_still",
        "rustc": run("rustc", "-Vv"),
        "simd": ("AOM encoder: hand-translated Neon (macOS ARM64) or SSE2/SSE4.1/AVX2 "
                 "(x86-64 Linux) kernels through translated RTCD (ADR 0056/0058); dav1d "
                 "port and the AOM decoder paths without kernels are portable C translations "
                 "(no assembly)"),
        "rtcd_selection": rtcd_port,
        "decoder_backend": "ferrisma-avif-read pins AVIF_CODEC_CHOICE_AOM (the AOM decoder port)",
    },
}
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

# 3. Inputs: BMP masters for the encoders, reference AVIF from the original.
for size in "${SIZE_LIST[@]}"; do
  python3 -I "$ROOT/scripts/bench/ci-perf/ppm_to.py" bmp "$INPUTS/photo$size.ppm" "$WORK/photo$size.bmp"
  "$C_ENC" "$WORK/photo$size.bmp" "$WORK/photo$size-q60.avif" --quality=60 --speed=6 --depth=8 \
    --threads=1 >/dev/null
done
python3 -I - "$OUT/avif-inputs.json" "$WORK" "$SIZES" <<'EOF'
import hashlib, json, sys
from pathlib import Path
out, work, sizes = sys.argv[1], Path(sys.argv[2]), sys.argv[3].split()
record = {"made_with": "C libavif+libaom release, encode_still.c --quality=60 --speed=6 --depth=8 "
                       "--threads=1 (4:2:0, one tile, BT.709/sRGB/BT.601 full range)", "files": {}}
for size in sizes:
    for name in (f"photo{size}.bmp", f"photo{size}-q60.avif"):
        data = (work / name).read_bytes()
        record["files"][name] = {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

# 4. Interleaved plans.
plan() { # plan FILE GROUP CANDIDATE_JSON...
  local file=$1 group=$2
  shift 2
  python3 -I - "$file" "$REPS" "$WARMUPS" "$group" "$@" <<'EOF'
import json, sys
out, reps, warmups, group, *candidates = sys.argv[1:]
plan = {"reps": int(reps), "warmups": int(warmups), "group": group,
        "candidates": [json.loads(c) for c in candidates]}
open(out, "w").write(json.dumps(plan, indent=2) + "\n")
EOF
}
candidate() { # candidate ID OUTPUT META_JSON CMD...
  local id=$1 output=$2 meta=$3
  shift 3
  python3 -I -c 'import json, sys; print(json.dumps({"id": sys.argv[1], "cmd": sys.argv[4:], "env": {}, "output": sys.argv[2], "meta": json.loads(sys.argv[3])}))' \
    "$id" "$output" "$meta" "$@"
}

ENC_NOTE="encode_us = avifEncoderWrite only (RGB->YUV and BMP read outside); one encode per process; 10-bit 4:4:4 q75 s6, autoTiling when threads != 1"
for size in $ENCODE_SIZES; do
  for mode in t1 tN; do
    threads=1
    [[ $mode == tN ]] && threads=$NCPU
    args=(--quality=75 --speed=6 --depth=10 --yuv444 --threads=$threads)
    meta() { printf '{"codec": "avif", "op": "encode", "input": "photo%s.bmp", "impl": "%s", "threads": %s, "notes": "%s"}' "$size" "$1" "$threads" "$ENC_NOTE"; }
    plan "$WORK/plan-encode-$size-$mode.json" "avif/encode/photo$size/$mode" \
      "$(candidate "c:$mode" "$WORK/enc-c-$size-$mode.avif" "$(meta c)" \
        "$C_ENC" "$WORK/photo$size.bmp" "$WORK/enc-c-$size-$mode.avif" "${args[@]}")" \
      "$(candidate "port:$mode" "$WORK/enc-port-$size-$mode.avif" "$(meta port)" \
        "$P_ENC" "$WORK/photo$size.bmp" "$WORK/enc-port-$size-$mode.avif" "${args[@]}")"
    python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$WORK/plan-encode-$size-$mode.json" \
      "$OUT/avif-encode-$size-$mode.json"
  done
done

DEC_NOTE="elapsed_ms = decoder create + parse + AV1 decode + YUV->RGB8 (nearest upsampling); file read and PPM write outside"
for size in "${SIZE_LIST[@]}"; do
  input=$WORK/photo$size-q60.avif
  for mode in t1 tN; do
    threads=1
    [[ $mode == tN ]] && threads=$NCPU
    meta() { printf '{"codec": "avif", "op": "decode", "input": "photo%s-q60.avif", "impl": "%s", "threads": %s, "notes": "%s"}' "$size" "$1" "$threads" "$DEC_NOTE${2:-}"; }
    candidates=(
      "$(candidate "c-aom:$mode" "$WORK/dec-c-aom-$size-$mode.ppm" "$(meta c-aom)" \
        "$C_DEC" aom "$threads" "$input" "$WORK/dec-c-aom-$size-$mode.ppm")"
      "$(candidate "c-dav1d:$mode" "$WORK/dec-c-dav1d-$size-$mode.ppm" "$(meta c-dav1d)" \
        "$C_DEC" dav1d "$threads" "$input" "$WORK/dec-c-dav1d-$size-$mode.ppm")"
    )
    if [[ $mode == t1 ]]; then
      candidates+=("$(candidate "port:t1" "$WORK/dec-port-$size-t1.ppm" \
        "$(meta port "; ferrisma-avif-read safe API, AOM decoder port, libavif default maxThreads=1 (the engine's path); also scans the AV1 sequence headers during parse")" \
        "$P_DEC" "$input" "$WORK/dec-port-$size-t1.ppm")")
    fi
    plan "$WORK/plan-decode-$size-$mode.json" "avif/decode/photo$size/$mode" "${candidates[@]}"
    python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$WORK/plan-decode-$size-$mode.json" \
      "$OUT/avif-decode-$size-$mode.json"
  done
done

# 5. Correctness side-checks (recorded, never fatal).
python3 -I - "$OUT/avif-checks.json" "$WORK" "$SIZES" "$ENCODE_SIZES" <<'EOF'
import hashlib, json, sys
from pathlib import Path
out, work, sizes, encode_sizes = sys.argv[1], Path(sys.argv[2]), sys.argv[3].split(), sys.argv[4].split()
def digest(name):
    path = work / name
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None
checks = {"encode_byte_identity": {}, "decode_rgb_identity": {}}
for size in encode_sizes:
    for mode in ("t1", "tN"):
        c, port = digest(f"enc-c-{size}-{mode}.avif"), digest(f"enc-port-{size}-{mode}.avif")
        checks["encode_byte_identity"][f"photo{size}/{mode}"] = {
            "c_sha256": c, "port_sha256": port,
            "c_bytes": (work / f"enc-c-{size}-{mode}.avif").stat().st_size if c else None,
            "port_bytes": (work / f"enc-port-{size}-{mode}.avif").stat().st_size if port else None,
            "identical": c is not None and c == port}
for size in sizes:
    hashes = {name: digest(f"dec-{name}-{size}-t1.ppm") for name in ("c-aom", "c-dav1d", "port")}
    hashes.update({f"{name}:tN": digest(f"dec-{name}-{size}-tN.ppm") for name in ("c-aom", "c-dav1d")})
    present = [h for h in hashes.values() if h]
    checks["decode_rgb_identity"][f"photo{size}"] = {
        "sha256": hashes, "all_identical": len(present) == len(hashes) and len(set(present)) == 1}
Path(out).write_text(json.dumps(checks, indent=2) + "\n")
print(json.dumps(checks, indent=2))
EOF
