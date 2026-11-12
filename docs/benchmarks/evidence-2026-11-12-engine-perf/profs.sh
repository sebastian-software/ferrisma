#!/bin/bash
# Record samply profiles of release CLI runs (indicative; loaded host).
cd "$(dirname "$0")"
B=${B:-./ferrisma-base}; I=../in19; E=../enc-elcito/corpus
P="--budget-mb=4096 --max-output-mb=1024"
rec(){ name=$1; shift; echo "$name load=$(sysctl -n vm.loadavg)" >> prof/loads.txt; samply record -s --unstable-presymbolicate -r 2000 -o prof/$name.json.gz -- "$@" > prof/$name.log 2>&1; }
SET=${SET:-main}
if [ "$SET" = main ]; then
rec jpeg24-avif-down-t1 $B $I/jpeg24-o1.jpg out/x.avif 1600 1067 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec jpeg24-webp-down-t1 $B $I/jpeg24-o1.jpg out/x.webp 1600 1067 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec jpeg24-webp-none-t1 $B $I/jpeg24-o1.jpg out/x.webp 6000 4000 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec webp24-avif-down-t1 $B $I/webp24.webp out/x.avif 1600 1067 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec avif24-webp-down-t1 $B $I/avif24.avif out/x.webp 1600 1067 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec avif24-avif-down-t1 $B $I/avif24.avif out/x.avif 1600 1067 --assume-srgb $P --transform-threads=1 --encoder-threads=1
rec jpeg12o6-webp-down-t1 $B $I/jpeg12-o6.jpg out/x.webp 1200 1600 --use-rendered-icc $P --transform-threads=1 --encoder-threads=1
rec jpeg24-avif-down-t0 $B $I/jpeg24-o1.jpg out/x.avif 1600 1067 --assume-srgb $P --transform-threads=0 --encoder-threads=0
rec jpeg24-avif-none-t0 $B $I/jpeg24-o1.jpg out/x.avif 6000 4000 --assume-srgb $P --transform-threads=0 --encoder-threads=0
rec avif24-webp-none-t0 $B $I/avif24.avif out/x.webp 6000 4000 --assume-srgb $P --transform-threads=0 --encoder-threads=0
rec elcito-aom-q82-t1 $B $E/elcito.jpg out/x.avif 1600 1067 --assume-srgb $P --avif-encoder=libavif --quality=82 --speed=6 --chroma=444 --transform-threads=1 --encoder-threads=1
rec elcito-rav1e-q90-t1 $B $E/elcito.jpg out/x.avif 1600 1067 --assume-srgb $P --avif-encoder=rav1e --quality=90 --speed=6 --chroma=444 --transform-threads=1 --encoder-threads=1
fi
echo ALLDONE
