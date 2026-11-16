"""Uniform noise as PPM (P6) or PAM (P7 RGB_ALPHA): the content that
compresses worst (largest output, most nonzero coefficients).

python3 -I noise.py <width> <height> <bits 8|16> <alpha 0|1> <out>

Samples from Python's Mersenne Twister seeded with 2026111600 + width +
height + bits + alpha (`random.Random(seed).randbytes`), so every file is
reproducible. Alpha is noise as well. Standard library only.
"""
import random
import sys

W, H, bits, alpha, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4] == "1", sys.argv[5]
ch = 4 if alpha else 3
s = 2 if bits == 16 else 1
rng = random.Random(2026111600 + W + H + bits + int(alpha))
m = (1 << bits) - 1
with open(out, "wb") as f:
    if alpha:
        f.write(f"P7\nWIDTH {W}\nHEIGHT {H}\nDEPTH 4\nMAXVAL {m}\nTUPLTYPE RGB_ALPHA\nENDHDR\n".encode())
    else:
        f.write(f"P6\n{W} {H}\n{m}\n".encode())
    for _ in range(H):
        f.write(rng.randbytes(W * ch * s))
