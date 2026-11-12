#!/usr/bin/env python3
"""List the `colr` properties of an AVIF file (standard library).

Prints one line per `colr` box in `meta/iprp/ipco`: the colour type and,
for `nclx`, primaries/transfer/matrix/full-range; for `prof`/`rICC`, the
profile size and SHA-256. Usage: python3 -I colr.py <file.avif>...
"""

import hashlib
import struct
import sys


def boxes(data, start, end):
    pos = start
    while pos + 8 <= end:
        size, kind = struct.unpack(">I4s", data[pos:pos + 8])
        header = 8
        if size == 1:
            size = struct.unpack(">Q", data[pos + 8:pos + 16])[0]
            header = 16
        elif size == 0:
            size = end - pos
        yield kind, pos + header, pos + size
        pos += size


def find(data, start, end, path):
    for kind, body, stop in boxes(data, start, end):
        if kind == path[0]:
            if len(path) == 1:
                return body, stop
            # `meta` is a full box: skip version and flags.
            inner = body + 4 if kind == b"meta" else body
            return find(data, inner, stop, path[1:])
    raise AssertionError(f"missing {path}")


def colr(data):
    start, end = find(data, 0, len(data), [b"meta", b"iprp", b"ipco"])
    for kind, body, stop in boxes(data, start, end):
        if kind != b"colr":
            continue
        colour_type = data[body:body + 4]
        if colour_type == b"nclx":
            p, t, m = struct.unpack(">HHH", data[body + 4:body + 10])
            full = data[body + 10] >> 7
            yield f"nclx {p}/{t}/{m} full_range={full}"
        else:
            profile = data[body + 4:stop]
            digest = hashlib.sha256(profile).hexdigest()
            yield f"{colour_type.decode()} {len(profile)} bytes sha256 {digest}"


if __name__ == "__main__":
    for name in sys.argv[1:]:
        for line in colr(open(name, "rb").read()):
            print(f"{name}: {line}")
