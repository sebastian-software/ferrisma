#!/usr/bin/env python3
"""Convert an 8-bit binary PPM (P6) to PNG or 24-bit BMP without dependencies.

Usage: python3 -I ppm_to.py png|bmp IN.ppm OUT
"""

import struct
import sys
import zlib
from pathlib import Path


def read_ppm(path):
    data = Path(path).read_bytes()
    fields, position = [], 0
    while len(fields) < 4:
        while data[position:position + 1].isspace():
            position += 1
        if data[position:position + 1] == b"#":
            position = data.index(b"\n", position)
            continue
        end = position
        while not data[end:end + 1].isspace():
            end += 1
        fields.append(data[position:end])
        position = end
    if fields[0] != b"P6" or fields[3] != b"255":
        raise SystemExit(f"{path}: not an 8-bit binary PPM")
    width, height = int(fields[1]), int(fields[2])
    return width, height, data[position + 1:position + 1 + width * height * 3]


def write_png(path, width, height, pixels):
    stride = width * 3
    raw = b"".join(b"\x00" + pixels[y * stride:(y + 1) * stride] for y in range(height))

    def chunk(kind, body):
        return (struct.pack(">I", len(body)) + kind + body
                + struct.pack(">I", zlib.crc32(kind + body) & 0xFFFFFFFF))

    header = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    Path(path).write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header)
                           + chunk(b"IDAT", zlib.compress(raw, 1)) + chunk(b"IEND", b""))


def write_bmp(path, width, height, pixels):
    stride = width * 3
    padded = (stride + 3) & ~3
    pad = b"\x00" * (padded - stride)
    rows = []
    for y in range(height - 1, -1, -1):
        row = bytearray(pixels[y * stride:(y + 1) * stride])
        row[0::3], row[2::3] = row[2::3], row[0::3]  # RGB -> BGR
        rows.append(bytes(row) + pad)
    body = b"".join(rows)
    info = struct.pack("<IiiHHIIiiII", 40, width, height, 1, 24, 0, len(body), 2835, 2835, 0, 0)
    header = struct.pack("<2sIHHI", b"BM", 14 + 40 + len(body), 0, 0, 14 + 40)
    Path(path).write_bytes(header + info + body)


def main():
    kind, src, dst = sys.argv[1:4]
    width, height, pixels = read_ppm(src)
    {"png": write_png, "bmp": write_bmp}[kind](dst, width, height, pixels)


if __name__ == "__main__":
    main()
