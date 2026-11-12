#!/usr/bin/env python3
"""Insert an ICC profile into a PNG as its `iCCP` chunk (standard library).

Usage: python3 -I tag_png.py <input.png> <profile.icc> <output.png>
"""

import struct
import sys
import zlib


def chunk(kind, data):
    body = kind + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))


def main(source, profile, target):
    data = open(source, "rb").read()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", "PNG required"
    ihdr_end = 8 + 8 + struct.unpack(">I", data[8:12])[0] + 4
    icc = open(profile, "rb").read()
    iccp = chunk(b"iCCP", b"icc\x00\x00" + zlib.compress(icc, 9))
    open(target, "wb").write(data[:ihdr_end] + iccp + data[ihdr_end:])


if __name__ == "__main__":
    main(*sys.argv[1:])
