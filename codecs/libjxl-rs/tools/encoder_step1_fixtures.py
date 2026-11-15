#!/usr/bin/env python3
"""Raw pixel fixtures for the upstream test cases of encoder step 1 (#118).

The upstream jxl_test.cc and speed_tier_test.cc cases read PNG files of the
pinned testdata submodule through the extras PNG decoder, which the oracle does
not build (port docs/ENCODER_EXPANSION.md, "Upstream tests"). This independent
tool decodes those PNG files once with the Python standard library (zlib and
the PNG filter definitions of the PNG specification, section 9), keeps the
top-left crop that each test's TestImage::SetDimensions or ShrinkTo leaves
(CropImage keeps each row's prefix), and writes the 8-bit interleaved samples
and the decompressed iCCP profile, which the extras decoder makes the primary
color representation. Both encoders then read the same bytes.

Usage: encoder_step1_fixtures.py TESTDATA_DIR OUTPUT_DIR [--check]
"""
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path

# (fixture name, testdata path, crop width, crop height). The crops are the
# largest region a test reads.
FIXTURES = [
    # speed_tier_test (full size), jxl_test RoundtripSmallD1 (/8),
    # RoundtripUnalignedD2 (/12, /7); every other read is a smaller crop.
    ("bliznaca", "external/wesaturate/500px/u76c0g_bliznaca_srgb8.png", 500, 500),
    # jxl_test RoundtripD2Consistent: SetDimensions(48..41, 15).
    ("flower", "jxl/flower/flower.png", 48, 15),
]


def chunks(data):
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit("not a PNG file")
    i = 8
    while i < len(data):
        (n,) = struct.unpack(">I", data[i : i + 4])
        yield data[i + 4 : i + 8], data[i + 8 : i + 8 + n]
        i += 12 + n


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    return b if pb <= pc else c


def decode(data):
    idat = b""
    icc = None
    for kind, payload in chunks(data):
        if kind == b"IHDR":
            width, height, depth, color, compression, filtering, interlace = struct.unpack(
                ">IIBBBBB", payload
            )
        elif kind == b"IDAT":
            idat += payload
        elif kind == b"iCCP":
            name_end = payload.index(b"\0")
            if payload[name_end + 1] != 0:
                raise SystemExit("unknown iCCP compression")
            icc = zlib.decompress(payload[name_end + 2 :])
    if depth != 8 or color not in (0, 2) or compression or filtering or interlace:
        raise SystemExit("only non-interlaced 8-bit gray or RGB is needed")
    channels = 1 if color == 0 else 3
    stride = width * channels
    raw = zlib.decompress(idat)
    rows = []
    prev = bytearray(stride)
    pos = 0
    for _ in range(height):
        kind = raw[pos]
        line = bytearray(raw[pos + 1 : pos + 1 + stride])
        pos += 1 + stride
        for x in range(stride):
            a = line[x - channels] if x >= channels else 0
            b = prev[x]
            c = prev[x - channels] if x >= channels else 0
            if kind == 1:
                line[x] = (line[x] + a) & 255
            elif kind == 2:
                line[x] = (line[x] + b) & 255
            elif kind == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif kind == 4:
                line[x] = (line[x] + paeth(a, b, c)) & 255
            elif kind != 0:
                raise SystemExit("unknown PNG filter")
        rows.append(bytes(line))
        prev = line
    return width, height, channels, rows, icc


def main():
    testdata = Path(sys.argv[1])
    output = Path(sys.argv[2])
    check = "--check" in sys.argv[3:]
    output.mkdir(parents=True, exist_ok=True)
    provenance = {"testdata_revision": "73695d303670c90e4d506ea89d9901b081385089", "fixtures": []}
    for name, path, crop_x, crop_y in FIXTURES:
        data = (testdata / path).read_bytes()
        width, height, channels, rows, icc = decode(data)
        pixels = b"".join(row[: crop_x * channels] for row in rows[:crop_y])
        files = {f"{name}.u8": pixels}
        if icc is not None:
            files[f"{name}.icc"] = icc
        entry = {
            "name": name,
            "source": path,
            "source_sha256": hashlib.sha256(data).hexdigest(),
            "source_size": [width, height],
            "channels": channels,
            "crop": [crop_x, crop_y],
            "files": {},
        }
        for file, content in files.items():
            entry["files"][file] = hashlib.sha256(content).hexdigest()
            target = output / file
            if check:
                if target.read_bytes() != content:
                    raise SystemExit(f"fixture differs: {target}")
            else:
                target.write_bytes(content)
        provenance["fixtures"].append(entry)
    text = json.dumps(provenance, indent=2) + "\n"
    target = output / "fixtures.json"
    if check:
        if target.read_text() != text:
            raise SystemExit("provenance differs")
    else:
        target.write_text(text)


if __name__ == "__main__":
    main()
