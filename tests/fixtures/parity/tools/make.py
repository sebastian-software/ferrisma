#!/usr/bin/env python3
"""Inputs for the format-parity fixtures (standard library only).

Writes into the directory given as the first argument:

- ``gray-gamma22.icc``: an ICC v2.1 monitor profile, color space GRAY,
  PCS XYZ, with a D50 media white point and one ``kTRC`` gamma curve
  (``curv`` with one entry, 2.19921875 = 0x0233 in u8Fixed8);
- ``gray-16x8.raw``: 16 x 8 8-bit gray samples, ``(16 * x + 8 * y) % 256``,
  row major;
- ``rgb-16x8.raw``: 16 x 8 8-bit RGB samples, r = 16 * x, g = 32 * y,
  b = 255 - 16 * x, row major.

The cjxl build of the README reads PNM only, so the samples are raw and the
profiles are passed with ``-x icc_pathname=``.

Usage: python3 -I make.py <out-dir>
"""

import struct
import sys
from pathlib import Path


def s15f16(value):
    return struct.pack(">i", round(value * 65536))


def gray_profile():
    def text_description(text):
        ascii_ = text.encode("ascii") + b"\0"
        return (b"desc" + b"\0" * 4 + struct.pack(">I", len(ascii_)) + ascii_
                + b"\0" * 4 + b"\0" * 4 + b"\0" * 2 + b"\0" + b"\0" * 67)

    tags = [
        (b"desc", text_description("Gray gamma 2.2 (Ferrisma parity fixture)")),
        (b"cprt", b"text" + b"\0" * 4 + b"CC0\0"),
        (b"wtpt", b"XYZ " + b"\0" * 4 + s15f16(0.9642) + s15f16(1.0) + s15f16(0.8249)),
        (b"kTRC", b"curv" + b"\0" * 4 + struct.pack(">I", 1) + struct.pack(">H", 0x0233)),
    ]
    table_size = 4 + 12 * len(tags)
    offset = 128 + table_size
    entries, data = b"", b""
    for signature, body in tags:
        while (offset + len(data)) % 4:
            data += b"\0"
        entries += signature + struct.pack(">II", offset + len(data), len(body))
        data += body
    while (offset + len(data)) % 4:
        data += b"\0"
    size = offset + len(data)
    header = (struct.pack(">I", size) + b"\0" * 4 + struct.pack(">I", 0x02100000)
              + b"mntr" + b"GRAY" + b"XYZ "
              + struct.pack(">6H", 2026, 11, 15, 0, 0, 0)
              + b"acsp" + b"\0" * 4 + b"\0" * 4 + b"\0" * 4 + b"\0" * 4
              + b"\0" * 8 + b"\0" * 4
              + s15f16(0.9642) + s15f16(1.0) + s15f16(0.8249)
              + b"\0" * 4 + b"\0" * 16 + b"\0" * 28)
    assert len(header) == 128, len(header)
    return header + struct.pack(">I", len(tags)) + entries + data


def main():
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    icc = gray_profile()
    (out / "gray-gamma22.icc").write_bytes(icc)
    gray = [(16 * x + 8 * y) % 256 for y in range(8) for x in range(16)]
    (out / "gray-16x8.raw").write_bytes(bytes(gray))
    rgb = [v for y in range(8) for x in range(16) for v in (16 * x, 32 * y, 255 - 16 * x)]
    (out / "rgb-16x8.raw").write_bytes(bytes(rgb))


if __name__ == "__main__":
    main()
