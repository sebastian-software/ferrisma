"""Tile a binary PPM to WIDTHxHEIGHT. Usage: python3 -I mkppm.py IN.ppm OUT.ppm W H"""
import sys

data = open(sys.argv[1], "rb").read()
fields, pos = [], 0
while len(fields) < 4:
    while data[pos:pos + 1].isspace():
        pos += 1
    if data[pos:pos + 1] == b"#":
        pos = data.index(b"\n", pos)
        continue
    end = pos
    while not data[end:end + 1].isspace():
        end += 1
    fields.append(data[pos:end])
    pos = end
w, h = int(fields[1]), int(fields[2])
px = data[pos + 1:pos + 1 + w * h * 3]
W, H = int(sys.argv[3]), int(sys.argv[4])
rows = []
for y in range(H):
    src = px[(y % h) * w * 3:(y % h + 1) * w * 3]
    rows.append((src * (W // w + 1))[:W * 3])
open(sys.argv[2], "wb").write(b"P6\n%d %d\n255\n" % (W, H) + b"".join(rows))
