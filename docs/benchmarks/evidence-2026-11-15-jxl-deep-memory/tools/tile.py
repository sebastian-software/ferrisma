"""Tile libavif's 16-bit test photograph to a size, as PPM/PAM at 16 or 8 bits.

python3 -I tile.py weld_16bit.png <width> <height> <bits 16|8> <alpha 0|1> <out>

The source is codecs/libavif-rs/upstream/libavif/tests/data/weld_16bit.png
(1024x684, 16-bit RGB). It is mirrored in both directions to fill the size,
so the content stays photographic without seams. 8 bits takes the high
byte of each sample (same content, the control). Alpha (PAM RGB_ALPHA) is
the pattern of the 2026-11-15 lossless memory evidence: column 0
transparent, then (x * 13 + y * 5) mod 256 scaled to the depth.
Standard library only.
"""
import struct
import sys
import zlib


def read_png(path):
    data = open(path, "rb").read()
    pos, idat = 8, []
    while pos < len(data):
        length, kind = struct.unpack(">I4s", data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            w, h, depth, color, _, _, interlace = struct.unpack(">IIBBBBB", body)
            assert (depth, color, interlace) == (16, 2, 0)
        elif kind == b"IDAT":
            idat.append(body)
    raw = zlib.decompress(b"".join(idat))
    bpp, stride = 6, w * 6
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(h):
        f = raw[i]
        line = bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if f == 1:
                line[x] = (line[x] + a) & 255
            elif f == 2:
                line[x] = (line[x] + b) & 255
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return w, h, rows


src, W, H, bits, alpha, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5] == "1", sys.argv[6]
w, h, rows = read_png(src)
m = (1 << bits) - 1
with open(out, "wb") as f:
    if alpha:
        f.write(f"P7\nWIDTH {W}\nHEIGHT {H}\nDEPTH 4\nMAXVAL {m}\nTUPLTYPE RGB_ALPHA\nENDHDR\n".encode())
    else:
        f.write(f"P6\n{W} {H}\n{m}\n".encode())
    for y in range(H):
        sy = y % (2 * h)
        sy = sy if sy < h else 2 * h - 1 - sy
        row = rows[sy]
        line = bytearray()
        for x in range(W):
            sx = x % (2 * w)
            sx = sx if sx < w else 2 * w - 1 - sx
            px = row[sx * 6:sx * 6 + 6]
            if bits == 16:
                line += px
            else:
                line += bytes((px[0], px[2], px[4]))
            if alpha:
                a = 0 if x == 0 else (x * 13 + y * 5) % 256
                line += struct.pack(">H", a * 257) if bits == 16 else bytes((a,))
        f.write(line)
