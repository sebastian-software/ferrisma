#!/bin/bash
# Final interleaved before/after/C runs (indicative; loaded host; x86-64 emulated by Rosetta 2).
# W holds jpeg_ab2.py and bin/ with the port's examples/bench builds:
#   {arm,x86}-{simd,portable}-base   at e4038a4a (before)
#   {arm,x86}-{simd,portable}-f3     with the three changes (after)
W=${W:-$(pwd)}
C=$(cd "$(dirname "$0")/../../../codecs/libjpeg-turbo-rs" && pwd)
O=$C/build/oracle
cd "$W" || exit 1
{
echo "## arm64 simd"; python3 -I jpeg_ab2.py $C $O/simd final3-arm-simd.json 7 before=bin/arm-simd-base after=bin/arm-simd-f3
echo "## arm64 portable"; python3 -I jpeg_ab2.py $C $O/release final3-arm-portable.json 7 before=bin/arm-portable-base after=bin/arm-portable-f3
echo "## x86-64 emulated avx2"; python3 -I jpeg_ab2.py $C $O/simd-x86_64 final3-x86-avx2.json 3 --env ROSETTA_ADVERTISE_AVX=1 before=bin/x86-simd-base after=bin/x86-simd-f3
echo "## x86-64 emulated sse2"; python3 -I jpeg_ab2.py $C $O/simd-x86_64 final3-x86-sse2.json 3 --env ROSETTA_ADVERTISE_AVX=1 --env JSIMD_FORCESSE2=1 before=bin/x86-simd-base after=bin/x86-simd-f3
echo "## x86-64 emulated portable"; python3 -I jpeg_ab2.py $C $O/release-x86_64 final3-x86-portable.json 3 before=bin/x86-portable-base after=bin/x86-portable-f3
} > final-tables.md 2>&1
echo done
