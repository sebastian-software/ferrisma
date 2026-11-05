#!/usr/bin/env bash
# M2 work package 0: reproducible latency, stage timings, peak RSS and
# sequential throughput of the current Ferrixel CLI on one machine.
# Usage: scripts/bench/run-baseline.sh CORPUS_DIR OUT_DIR [RUNS]
#
# CORPUS_DIR/single/*.{jpg,jpeg,avif,jxl}: each image is resized to a width
# of BENCH_WIDTH (default 1600) with its aspect ratio preserved, RUNS times
# under hyperfine, then once more under /usr/bin/time for peak RSS and the
# CLI's stage report. CORPUS_DIR/small/*: processed one after another to a
# width of BENCH_SMALL_WIDTH (default 320) for a sequential images-per-second
# figure (one process per image; Ferrixel has no batch API yet).
#
# Comparators run only when present: vipsthumbnail on PATH, and Sharp when
# BENCH_SHARP_NODE_MODULES names a node_modules directory containing sharp.
# Their rows carry comparable_quality=no: filters, transfer intent, chroma
# mode, threading and encoders are not matched, so their time is context,
# not a result. Nothing here is a performance claim.
set -euo pipefail
export LC_ALL=C

corpus=${1:?usage: run-baseline.sh CORPUS_DIR OUT_DIR [RUNS]}
out=${2:?usage: run-baseline.sh CORPUS_DIR OUT_DIR [RUNS]}
runs=${3:-5}
width=${BENCH_WIDTH:-1600}
small_width=${BENCH_SMALL_WIDTH:-320}
quality=${BENCH_QUALITY:-80}
speed=${BENCH_SPEED:-6}
filter=${BENCH_FILTER:-lanczos}
profile=${BENCH_PROFILE:-assume-srgb}
max_input_mb=${BENCH_MAX_INPUT_MB:-512}

for tool in hyperfine python3; do
  command -v "$tool" >/dev/null || { echo "$tool is required" >&2; exit 2; }
done
[[ -x /usr/bin/time ]] || { echo "/usr/bin/time is required" >&2; exit 2; }
[[ -d "$corpus/single" ]] || { echo "missing $corpus/single" >&2; exit 2; }

case "$profile" in
  strict) policy_args=() ;;
  assume-srgb) policy_args=(--assume-srgb) ;;
  rendered-icc) policy_args=(--use-rendered-icc) ;;
  *) echo "BENCH_PROFILE must be strict, assume-srgb or rendered-icc" >&2; exit 2 ;;
esac
case "$filter" in
  lanczos) filter_args=() ;;
  area) filter_args=(--area) ;;
  *) echo "BENCH_FILTER must be lanczos or area" >&2; exit 2 ;;
esac

repo=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$out"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

(cd "$repo" && cargo build --release --features cli > "$out/build.log" 2>&1)
bin=$repo/target/release/ferrixel
# Bash 3.2 (macOS) treats an expanded empty array as unbound under set -u.
ferrixel_args=(${policy_args[@]+"${policy_args[@]}"} ${filter_args[@]+"${filter_args[@]}"}
  "--quality=$quality" "--speed=$speed" "--max-input-mb=$max_input_mb")

sharp_available=0
if [[ -n "${BENCH_SHARP_NODE_MODULES:-}" ]] \
  && NODE_PATH="$BENCH_SHARP_NODE_MODULES" node -e 'require("sharp")' 2>/dev/null; then
  sharp_available=1
fi
vips_available=0
command -v vipsthumbnail >/dev/null && vips_available=1

{
  echo "# Ferrixel M2 baseline environment ($(date -u +%Y-%m-%dT%H:%M:%SZ))"
  echo "git: $(cd "$repo" && git rev-parse --short HEAD) $(cd "$repo" && git status --short | wc -l | tr -d ' ') uncommitted paths"
  uname -a
  if [[ "$(uname)" == Darwin ]]; then
    echo "cpu: $(sysctl -n machdep.cpu.brand_string), $(sysctl -n hw.ncpu) logical cores, $(( $(sysctl -n hw.memsize) / 1024 / 1024 / 1024 )) GiB"
  else
    echo "cpu: $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //'), $(nproc) logical cores"
  fi
  rustc --version
  hyperfine --version
  for crate in zune-jpeg jxl-oxide avif-decode rav1d ravif rav1e moxcms; do
    version=$(grep -A1 "^name = \"$crate\"$" "$repo/Cargo.lock" | awk -F'"' '/version/{print $2}')
    echo "crate: $crate $version"
  done
  command -v magick >/dev/null && magick --version | head -1
  if [[ $sharp_available == 1 ]]; then
    echo "sharp: $(NODE_PATH="$BENCH_SHARP_NODE_MODULES" node -p 'require("sharp/package.json").version') on node $(node --version)"
  else
    echo "sharp: not available (set BENCH_SHARP_NODE_MODULES)"
  fi
  [[ $vips_available == 1 ]] && vipsthumbnail --version || echo "libvips: not available"
  echo "settings: width=$width small_width=$small_width quality=$quality speed=$speed filter=$filter profile=$profile runs=$runs"
} > "$out/environment.txt"

dimensions() {
  local file=$1
  if command -v magick >/dev/null; then
    magick identify -format '%w %h' "${file}[0]" 2>/dev/null && return
  fi
  if command -v sips >/dev/null; then
    sips -g pixelWidth -g pixelHeight "$file" 2>/dev/null \
      | awk '/pixelWidth/{w=$2} /pixelHeight/{h=$2} END{if (w && h) print w, h}'
    return
  fi
  echo ""
}

# Runs one command under /usr/bin/time; prints peak RSS in KiB and keeps
# stdout in $1.
peak_rss_kib() {
  local log=$1; shift
  if [[ "$(uname)" == Darwin ]]; then
    /usr/bin/time -l "$@" > "$log" 2> "$log.time"
    awk '/maximum resident set size/{printf "%d\n", $1 / 1024}' "$log.time"
  else
    /usr/bin/time -f '%M' -o "$log.time" "$@" > "$log" 2>/dev/null
    tail -n1 "$log.time"
  fi
}

# Prints mean,stddev,min,max,mean_cpu seconds; handles hyperfine's JSON
# schema 1 (aggregates) and schema 2 (per-run measurements).
hyperfine_stats() {
  python3 -I - "$1" <<'PY'
import json, statistics, sys
result = json.load(open(sys.argv[1]))["results"][0]
if "measurements" in result:
    walls = [m["time_wall_clock"]["value"] for m in result["measurements"]]
    cpus = [m["time_cpu"]["value"] for m in result["measurements"] if "time_cpu" in m]
    mean, stddev = statistics.mean(walls), (statistics.stdev(walls) if len(walls) > 1 else 0.0)
    low, high = min(walls), max(walls)
    cpu = statistics.mean(cpus) if cpus else float("nan")
else:
    mean, stddev = result["mean"], result["stddev"] or 0.0
    low, high = result["min"], result["max"]
    cpu = result["user"] + result["system"]
print(f"{mean:.4f},{stddev:.4f},{low:.4f},{high:.4f},{cpu:.4f}")
PY
}

report_field() {
  awk -v key="$2" '$1 == "report:" {for (i = 2; i <= NF; i++) {split($i, kv, "="); if (kv[1] == key) print kv[2]}}' "$1"
}

results=$out/results.csv
echo "backend,image,input_bytes,input_width,input_height,output_width,output_height,runs,mean_s,stddev_s,min_s,max_s,mean_cpu_s,peak_rss_kib,output_bytes,open_us,decode_us,transform_us,encode_us,total_us,comparable_quality" > "$results"

shopt -s nullglob
for image in "$corpus"/single/*.jpg "$corpus"/single/*.jpeg "$corpus"/single/*.avif "$corpus"/single/*.jxl; do
  name=$(basename "$image")
  read -r in_w in_h <<< "$(dimensions "$image")"
  if [[ -z "${in_w:-}" || -z "${in_h:-}" ]]; then
    echo "skip $name: cannot read dimensions (install ImageMagick)" >&2
    continue
  fi
  out_w=$width
  out_h=$(( (in_h * width + in_w / 2) / in_w ))
  input_bytes=$(wc -c < "$image" | tr -d ' ')
  echo "== $name ${in_w}x${in_h} -> ${out_w}x${out_h}" >&2

  cmd="$bin $image $tmp/ferrixel.avif $out_w $out_h ${ferrixel_args[*]}"
  if ! hyperfine --warmup 1 --runs "$runs" --shell=none \
      --export-json "$out/hyperfine-ferrixel-$name.json" "$cmd" > "$out/hyperfine-ferrixel-$name.txt" 2>&1; then
    echo "ferrixel failed on $name:" >&2
    tail -5 "$out/hyperfine-ferrixel-$name.txt" >&2
    continue
  fi
  stats=$(hyperfine_stats "$out/hyperfine-ferrixel-$name.json")
  rss=$(peak_rss_kib "$tmp/ferrixel.log" "$bin" "$image" "$tmp/ferrixel.avif" "$out_w" "$out_h" "${ferrixel_args[@]}")
  bytes=$(wc -c < "$tmp/ferrixel.avif" | tr -d ' ')
  stages=$(for key in open_us decode_us transform_us encode_us total_us; do report_field "$tmp/ferrixel.log" "$key"; done | paste -sd, -)
  echo "ferrixel,$name,$input_bytes,$in_w,$in_h,$out_w,$out_h,$runs,$stats,$rss,$bytes,$stages,n/a" >> "$results"

  if [[ $sharp_available == 1 && "$name" != *.jxl ]]; then
    cmd="node $repo/scripts/bench/sharp-resize.cjs $image $tmp/sharp.avif $out_w $out_h $quality 4"
    if NODE_PATH="$BENCH_SHARP_NODE_MODULES" hyperfine --warmup 1 --runs "$runs" --shell=none \
        --export-json "$out/hyperfine-sharp-$name.json" "$cmd" > "$out/hyperfine-sharp-$name.txt" 2>&1; then
      stats=$(hyperfine_stats "$out/hyperfine-sharp-$name.json")
      rss=$(NODE_PATH="$BENCH_SHARP_NODE_MODULES" peak_rss_kib "$tmp/sharp.log" node "$repo/scripts/bench/sharp-resize.cjs" "$image" "$tmp/sharp.avif" "$out_w" "$out_h" "$quality" 4)
      bytes=$(wc -c < "$tmp/sharp.avif" | tr -d ' ')
      echo "sharp,$name,$input_bytes,$in_w,$in_h,$out_w,$out_h,$runs,$stats,$rss,$bytes,,,,,,no" >> "$results"
    else
      echo "sharp failed on $name" >&2
    fi
  fi

  if [[ $vips_available == 1 && "$name" != *.jxl ]]; then
    cmd="vipsthumbnail $image --linear --size=${out_w}x${out_h}! --output=$tmp/vips.avif[Q=$quality,effort=4]"
    if hyperfine --warmup 1 --runs "$runs" --shell=none \
        --export-json "$out/hyperfine-vips-$name.json" "$cmd" > "$out/hyperfine-vips-$name.txt" 2>&1; then
      stats=$(hyperfine_stats "$out/hyperfine-vips-$name.json")
      rss=$(peak_rss_kib "$tmp/vips.log" vipsthumbnail "$image" --linear "--size=${out_w}x${out_h}!" "--output=$tmp/vips.avif[Q=$quality,effort=4]")
      bytes=$(wc -c < "$tmp/vips.avif" | tr -d ' ')
      echo "libvips,$name,$input_bytes,$in_w,$in_h,$out_w,$out_h,$runs,$stats,$rss,$bytes,,,,,,no" >> "$results"
    else
      echo "libvips failed on $name (AVIF output may be unsupported)" >&2
    fi
  fi
done

# Sequential many-small-images throughput: one process per image.
throughput=$out/throughput.csv
echo "backend,images,mean_s,images_per_s" > "$throughput"
small=("$corpus"/small/*)
if (( ${#small[@]} > 0 )); then
  read -r s_w s_h <<< "$(dimensions "${small[0]}")"
  if [[ -n "${s_w:-}" ]]; then
    s_out_h=$(( (s_h * small_width + s_w / 2) / s_w ))
    loop="for f in $corpus/small/*; do $bin \"\$f\" $tmp/small.avif $small_width $s_out_h ${ferrixel_args[*]} >/dev/null; done"
    hyperfine --runs 2 --shell=bash --export-json "$out/hyperfine-throughput-ferrixel.json" "$loop" > "$out/hyperfine-throughput-ferrixel.txt" 2>&1
    mean=$(hyperfine_stats "$out/hyperfine-throughput-ferrixel.json" | cut -d, -f1)
    echo "ferrixel,${#small[@]},$mean,$(python3 -I -c "print(f'{${#small[@]} / $mean:.2f}')")" >> "$throughput"
    if [[ $sharp_available == 1 ]]; then
      loop="for f in $corpus/small/*; do node $repo/scripts/bench/sharp-resize.cjs \"\$f\" $tmp/small.avif $small_width $s_out_h $quality 4; done"
      NODE_PATH="$BENCH_SHARP_NODE_MODULES" hyperfine --runs 2 --shell=bash --export-json "$out/hyperfine-throughput-sharp.json" "$loop" > "$out/hyperfine-throughput-sharp.txt" 2>&1
      mean=$(hyperfine_stats "$out/hyperfine-throughput-sharp.json" | cut -d, -f1)
      echo "sharp,${#small[@]},$mean,$(python3 -I -c "print(f'{${#small[@]} / $mean:.2f}')")" >> "$throughput"
    fi
  fi
fi

python3 -I - "$results" "$throughput" > "$out/summary.md" <<'PY'
import csv, sys

def us(value):
    return "" if value in ("", None) else f"{int(value) / 1000:.0f}"

print("| backend | image | input | output | mean s | ± s | min s | CPU/wall | peak RSS MiB | out KiB | open ms | decode ms | transform ms | encode ms | matched quality |")
print("| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |")
for row in csv.DictReader(open(sys.argv[1])):
    ratio = float(row["mean_cpu_s"]) / float(row["mean_s"]) if row["mean_cpu_s"] not in ("", "nan") else float("nan")
    print(
        f'| {row["backend"]} | {row["image"]} | {row["input_width"]}×{row["input_height"]} '
        f'| {row["output_width"]}×{row["output_height"]} | {float(row["mean_s"]):.3f} | {float(row["stddev_s"]):.3f} '
        f'| {float(row["min_s"]):.3f} | {ratio:.2f} | {int(row["peak_rss_kib"]) / 1024:.1f} | {int(row["output_bytes"]) / 1024:.0f} '
        f'| {us(row["open_us"])} | {us(row["decode_us"])} | {us(row["transform_us"])} | {us(row["encode_us"])} | {row["comparable_quality"]} |'
    )
print()
print("| backend | images | total s | images/s |")
print("| --- | ---: | ---: | ---: |")
for row in csv.DictReader(open(sys.argv[2])):
    print(f'| {row["backend"]} | {row["images"]} | {float(row["mean_s"]):.2f} | {row["images_per_s"]} |')
PY

echo "results: $results" >&2
echo "summary: $out/summary.md" >&2
cat "$out/summary.md"
