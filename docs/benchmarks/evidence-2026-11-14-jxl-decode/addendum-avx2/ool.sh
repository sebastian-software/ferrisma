#!/bin/sh
# Usage: ool.sh <disassembly> : out-of-line core::arch x86 intrinsic calls per module
awk '/^[0-9a-f]+ <.*>:$/{fn=$0} /call.*core_arch::x86::/{n=fn; sub(/^[0-9a-f]+ <+/,"",n); split(n,p,"::"); k=p[1]"::"p[2]"::"p[3]; c[k]++; t++; f[fn]=1} END{for(x in c) print c[x], x; print t+0, "TOTAL calls in", length(f), "functions"}' "$1" | sort -rn
