"""Top-left crop of a P6 PPM: crop.py in.ppm width height out.ppm"""
import sys
data = open(sys.argv[1], "rb").read()
parts = data.split(b"\n", 3)
w, h = map(int, parts[1].split())
cw, ch = int(sys.argv[2]), int(sys.argv[3])
rgb = parts[3]
rows = b"".join(rgb[y * w * 3:y * w * 3 + cw * 3] for y in range(ch))
open(sys.argv[4], "wb").write(f"P6\n{cw} {ch}\n255\n".encode() + rows)
