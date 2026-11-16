"""Writes the deep test pattern of crates/ferrisma-avif-write's
`deep_output_equals_the_c_original` as little-endian uint16 RGBA, the input of
encode_deep.c (README.md).

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
        r = ((x * m) // (w - 1)) ^ noise
        g = (y * m) // (h - 1)
        b = ((x + y) * 37) & m
        a = m if not alpha else (0 if x == 0 else (x * 13 * (m // 255) + y * 5) % (m + 1))
        px += [r, g, b, a]
open(out, "wb").write(struct.pack("<%dH" % len(px), *px))
