#!/usr/bin/env bash
# Create the master pixel images of the v1.0 CI performance runs.
#
#   INPUTS=dir [SIZES="12 24"] [CORPUS=dir] scripts/bench/ci-perf/make_masters.sh
#
# photo12: the libavif test photograph dog_exif_extended_xmp_icc.jpg (tracked
#   in the repository), 4032x3024 in stored order, sRGB ICC, EXIF orientation 6.
# photo24: the verified 6000x4000 `mountain` photograph of
#   scripts/bench/corpus-photos.json, fetched from Wikimedia Commons unless
#   CORPUS already holds ferrisma-mountain.jpg; its SHA-256 is checked.
#
# Each master is decoded once with the system djpeg (stored order, no ICC
# transform; the masters carry no ICC or EXIF) into photoN.ppm and photoN.png.
# Every derived input of the runs is encoded from these masters, so all formats
# describe the same pixels, tagged or assumed sRGB. The originals are also
# copied as photoN-original.jpg for the "real camera file" rows.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
SIZES=${SIZES:-"12 24"}
mkdir -p "$INPUTS"
for size in $SIZES; do
  case "$size" in
    12)
      src="$ROOT/codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg"
      want=fc67748b1f04aba4ae38476adae70bd9c6157ee0d86db1e93c05aca5815b42e4 ;;
    24)
      CORPUS=${CORPUS:-$INPUTS/corpus}
      src="$CORPUS/ferrisma-mountain.jpg"
      want=e1c73ee88188b350464d0b63599301c84585fc2ca0345416d84293f0fa5ba13b
      if [[ ! -f "$src" ]]; then
        mkdir -p "$CORPUS"
        for attempt in 1 2 3 4; do
          if curl --fail --location --retry 2 --max-filesize 10000000 \
              -A 'ferrisma-bench/1.0 (https://github.com/sebastian-software/ferrisma; CI benchmark)' \
              'https://upload.wikimedia.org/wikipedia/commons/d/d2/Grandiose_Landscape_of_Mountain_He-Huan%28WDPA_9287%29_by_WEI-FU-KUO_04.jpg' \
              -o "$src"; then break; fi
          sleep $((attempt * 15))
        done
      fi ;;
    *) echo "unknown size $size" >&2; exit 1 ;;
  esac
  got=$( (sha256sum "$src" 2>/dev/null || shasum -a 256 "$src") | cut -d' ' -f1)
  [[ "$got" == "$want" ]] || { echo "$src: sha256 $got, expected $want" >&2; exit 1; }
  cp "$src" "$INPUTS/photo$size-original.jpg"
  djpeg -ppm -outfile "$INPUTS/photo$size.ppm" "$src"
  python3 -I "$ROOT/scripts/bench/ci-perf/ppm_to.py" png "$INPUTS/photo$size.ppm" "$INPUTS/photo$size.png"
done
ls -l "$INPUTS"
