"""Writes the deep test pattern of src/jxl_output_tests.rs (`deep_rows`) as a
binary PPM (opaque) or PAM with straight alpha at MAXVAL 2^depth - 1, the
input of the cjxl references (README.md).

python3 -I rows16.py <width> <height> <depth> <alpha 0|1> <out>
"""
import struct
import sys

w, h, depth, alpha, out = (int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]),
                           sys.argv[4] == "1", sys.argv[5])
m = (1 << depth) - 1
px = []
for y in range(h):
    for x in range(w):
        hsh = ((x * 2654435761) & 0xFFFFFFFF) ^ ((y * 40503) & 0xFFFFFFFF)
        noise = (hsh >> 13) & 0x3F
        r = ((x * m) // max(w - 1, 1)) ^ noise
        g = (y * m) // max(h - 1, 1)
        b = ((x + y) * 37) & m
        a = m if not alpha else (0 if x == 0 else (x * 13 * (m // 255) + y * 5) % (m + 1))
        px += [r, g, b, a] if alpha else [r, g, b]
fmt = ">%dH" if depth > 8 else ">%dB"
if alpha:
    head = f"P7\nWIDTH {w}\nHEIGHT {h}\nDEPTH 4\nMAXVAL {m}\nTUPLTYPE RGB_ALPHA\nENDHDR\n"
else:
    head = f"P6\n{w} {h}\n{m}\n"
open(out, "wb").write(head.encode() + struct.pack(fmt % len(px), *px))
