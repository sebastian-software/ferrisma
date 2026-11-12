#!/usr/bin/env python3
"""Write a copy of a JPEG with one minimal EXIF APP1 (IFD0 Orientation SHORT).

Usage: python3 -I set_orientation.py IN.jpg OUT.jpg ORIENTATION

Any existing APP1 Exif segment is removed; every other segment and the
entropy-coded data are copied unchanged. The inserted segment is big-endian
TIFF with one IFD0 entry (tag 0x0112, type 3, count 1).
"""

import struct
import sys


def main():
    data = open(sys.argv[1], "rb").read()
    value = int(sys.argv[3])
    assert data[:2] == b"\xff\xd8"
    tiff = b"MM\x00\x2a" + struct.pack(">I", 8) + struct.pack(">H", 1)
    tiff += struct.pack(">HHIHH", 0x0112, 3, 1, value, 0) + struct.pack(">I", 0)
    payload = b"Exif\x00\x00" + tiff
    out = bytearray(b"\xff\xd8\xff\xe1" + struct.pack(">H", len(payload) + 2) + payload)
    i = 2
    while True:
        marker = data[i + 1]
        if marker == 0xDA:
            out += data[i:]
            break
        length = int.from_bytes(data[i + 2:i + 4], "big")
        if not (marker == 0xE1 and data[i + 4:i + 10] == b"Exif\x00\x00"):
            out += data[i:i + 2 + length]
        i += 2 + length
    open(sys.argv[2], "wb").write(out)


if __name__ == "__main__":
    main()
