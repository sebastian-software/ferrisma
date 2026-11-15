"""Writes the `rows()` test pattern of src/jxl_output_tests.rs as PPM (opaque)
or PAM with straight alpha, the input of the cjxl references (README.md).

python3 -I rows.py <width> <height> <alpha 0|1> <out>
"""
import sys

w, h, alpha, out = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3] == "1", sys.argv[4]
px = bytearray()
for y in range(h):
    for x in range(w):
        hsh = ((x * 2654435761) & 0xFFFFFFFF) ^ ((y * 40503) & 0xFFFFFFFF)
        noise = (hsh >> 13) & 0x1F
        r = (((x * 255) // max(w, 2)) & 0xFF) ^ noise
        g = ((y * 255) // max(h, 2)) & 0xFF
        b = (((x + y) * 7) & 0xFF) ^ ((noise << 2) & 0xFF)
        a = 255 if not alpha else (0 if x == 0 else (x * 13 + y * 5) % 256)
        px += bytes([r, g, b, a] if alpha else [r, g, b])
if alpha:
    head = f"P7\nWIDTH {w}\nHEIGHT {h}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n"
else:
    head = f"P6\n{w} {h}\n255\n"
open(out, "wb").write(head.encode() + bytes(px))
