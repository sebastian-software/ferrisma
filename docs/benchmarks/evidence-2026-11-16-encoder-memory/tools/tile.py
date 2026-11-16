"""Mirror-tile a photograph to a size, as PPM (P6) or PAM (P7 RGB_ALPHA).

python3 -I tile.py <source> <width> <height> <bits 16|8> <alpha 0|1> <out>

<source> is a 16-bit RGB PNG (libavif's tests/data/weld_16bit.png) or an
8-bit P6 PPM (photo24.ppm of the 2026-11-15 lossless evidence). The source
is mirrored in both directions to fill the size (a crop where it is large
enough), so the content stays photographic without seams. For the PNG,
8 bits take the high byte of each sample; a PPM source allows 8 bits only.
Alpha keeps the pattern of the 2026-11-15 evidence for each source, column 0
transparent: for the PNG that of the deep-memory tools/tile.py,
(13 * x + 5 * y) mod 256 (scaled by 257 at 16 bits); for the PPM that of the
lossless-memory tools/rgba.py, which computes (13 * (x + (5 * y mod 256)))
mod 256. Sizes the 2026-11-15 records also measured give the same files
byte for byte (inputs.sha256).
Standard library only.
"""
import struct
import sys
import zlib


def read_png16(path):
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
    return w, h, rows, 6


def read_ppm8(path):
    data = open(path, "rb").read()
    parts = data.split(b"\n", 3)
    assert parts[0] == b"P6" and parts[2] == b"255"
    w, h = map(int, parts[1].split())
    rgb = parts[3]
    return w, h, [rgb[y * w * 3:(y + 1) * w * 3] for y in range(h)], 3


def mirrored(row, w, bpp, width):
    """The row extended to `width` pixels by mirroring at both edges."""
    rev = b"".join(row[x * bpp:(x + 1) * bpp] for x in range(w - 1, -1, -1))
    period = row + rev
    n = width * bpp
    reps = n // len(period) + 1
    return (period * reps)[:n]


src, W, H, bits, alpha, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5] == "1", sys.argv[6]
if src.endswith(".png"):
    w, h, rows, bpp = read_png16(src)
else:
    w, h, rows, bpp = read_ppm8(src)
    assert bits == 8
m = (1 << bits) - 1
s = 2 if bits == 16 else 1
cache = {}
alpha_base = bytes((13 * i) % 256 for i in range(W + 256))
png = src.endswith(".png")
with open(out, "wb") as f:
    if alpha:
        f.write(f"P7\nWIDTH {W}\nHEIGHT {H}\nDEPTH 4\nMAXVAL {m}\nTUPLTYPE RGB_ALPHA\nENDHDR\n".encode())
    else:
        f.write(f"P6\n{W} {H}\n{m}\n".encode())
    for y in range(H):
        sy = y % (2 * h)
        sy = sy if sy < h else 2 * h - 1 - sy
        if sy not in cache:
            line = mirrored(rows[sy], w, bpp, W)
            if bpp == 6 and bits == 8:
                line = line[0::2]  # high bytes
            cache[sy] = line
        rgb = cache[sy]
        if not alpha:
            f.write(rgb)
            continue
        if png:
            a = bytearray((13 * x + 5 * y) % 256 for x in range(W))
        else:
            a = bytearray(alpha_base[(5 * y) % 256:(5 * y) % 256 + W])
        a[0] = 0
        buf = bytearray(W * 4 * s)
        for c in range(3):
            for k in range(s):
                buf[c * s + k::4 * s] = rgb[c * s + k::3 * s]
        buf[3 * s::4 * s] = a
        if s == 2:
            buf[3 * s + 1::4 * s] = a  # a * 257, big-endian
        f.write(buf)
