#!/usr/bin/env python3
"""Byte patches that derive parity fixtures from tracked files (standard
library only). Run from the repository root:

    python3 -I tests/fixtures/parity/tools/patch.py

- ``gray-gray-icc.jpg``: libjxl's ``ycbcr-jpeg-gray.jpg`` with
  ``gray-gamma22.icc`` inserted as one ``APP2 ICC_PROFILE`` segment right
  after SOI; no other byte changes.
- ``gray-rgb-icc.jpg``: the same gray JPEG with ``../icc/paris-srgb.icc``
  (an RGB profile) inserted the same way.
- ``rgb-gray-icc.jpg``: the repository's ``sample.jpg`` (RGB) with
  ``gray-gamma22.icc`` inserted the same way.
- ``premultiplied-alpha.avif``: libavif's ``abc_color_irot_alpha_irot.avif``
  with a ``prem`` reference (color item 1 to alpha item 2) appended to its
  ``iref``; ``iref`` and ``meta`` grow by 14 bytes and the two ``iloc``
  extent offsets move by 14 (the helper in ``tests/discarded_report.rs``
  does the same in memory).
- ``rgb-icc-cicp-p3.jpg``: the repository's ``sample.jpg`` with
  ``../icc/paris-srgb.icc`` extended by a ``cicp`` tag (type ``cicp``,
  code points 12/13/0/1, Display P3, full range) and inserted as one
  ``APP2`` segment: an sRGB profile whose own ``cicp`` tag contradicts it.
  The tag table grows by one entry (12 bytes), every tag offset moves by
  12, the 12-byte tag is appended and the header size updated.
- ``icc-nclx-12-13-6.avif``: libavif's ``paris_icc_exif_xmp.avif`` (written
  by avifenc: sRGB ``prof`` plus ``nclx`` 2/2/6) with the ``nclx`` primaries
  and transfer rewritten to 12/13 (Display P3); the AV1 sequence header
  keeps 2/2, which does not contradict the container.
"""

import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / "tests/fixtures/parity"
JXL_DATA = ROOT / "codecs/libjxl-rs/tests/data"
AVIF_DATA = ROOT / "codecs/libavif-rs/upstream/libavif/tests/data"


def with_icc(jpeg, icc):
    assert jpeg[:2] == b"\xff\xd8"
    payload = b"ICC_PROFILE\x00\x01\x01" + icc
    app2 = b"\xff\xe2" + struct.pack(">H", len(payload) + 2) + payload
    return jpeg[:2] + app2 + jpeg[2:]


def with_cicp_tag(icc, cicp):
    size, = struct.unpack(">I", icc[:4])
    assert size == len(icc)
    count, = struct.unpack(">I", icc[128:132])
    entries = []
    for index in range(count):
        at = 132 + 12 * index
        signature, offset, length = struct.unpack(">4sII", icc[at:at + 12])
        assert signature != b"cicp"
        entries.append((signature, offset + 12, length))
    body = icc[132 + 12 * count:]
    while (len(icc) + 12) % 4:
        body += b"\0"
    tag_offset = 132 + 12 * (count + 1) + len(body)
    tag = b"cicp" + b"\0" * 4 + bytes(cicp)
    entries.append((b"cicp", tag_offset, len(tag)))
    table = struct.pack(">I", count + 1) + b"".join(struct.pack(">4sII", *e) for e in entries)
    out = bytearray(icc[:128] + table + body + tag)
    out[:4] = struct.pack(">I", len(out))
    return bytes(out)


GRAY_JPEG = JXL_DATA / "ycbcr-jpeg-gray.jpg"
GRAY_ICC = OUT / "gray-gamma22.icc"
RGB_ICC = ROOT / "tests/fixtures/icc/paris-srgb.icc"


def premultiplied_avif():
    data = bytearray((AVIF_DATA / "abc_color_irot_alpha_irot.avif").read_bytes())

    def find(kind):
        return data.index(kind) - 4

    def u32(at):
        return struct.unpack(">I", data[at:at + 4])[0]

    meta, iref, iloc = find(b"meta"), find(b"iref"), find(b"iloc")
    assert meta < iloc < iref
    assert data[iloc + 8:iloc + 16] == bytes([0, 0, 0, 0, 0x44, 0, 0, 2])
    for item in range(2):
        entry = iloc + 16 + item * 14
        assert data[entry + 4:entry + 6] == b"\0\x01"
        data[entry + 6:entry + 10] = struct.pack(">I", u32(entry + 6) + 14)
    prem = struct.pack(">I", 14) + b"prem" + struct.pack(">HHH", 1, 1, 2)
    end = iref + u32(iref)
    data[end:end] = prem
    data[iref:iref + 4] = struct.pack(">I", u32(iref) + 14)
    data[meta:meta + 4] = struct.pack(">I", u32(meta) + 14)
    return bytes(data)


def p3_nclx_beside_icc():
    data = bytearray((AVIF_DATA / "paris_icc_exif_xmp.avif").read_bytes())
    at = data.index(b"colrnclx") + 8
    assert struct.unpack(">HHH", data[at:at + 6]) == (2, 2, 6)
    assert data.count(b"colrnclx") == 1
    data[at:at + 4] = struct.pack(">HH", 12, 13)
    return bytes(data)


def main():
    (OUT / "gray-gray-icc.jpg").write_bytes(with_icc(GRAY_JPEG.read_bytes(), GRAY_ICC.read_bytes()))
    (OUT / "gray-rgb-icc.jpg").write_bytes(with_icc(GRAY_JPEG.read_bytes(), RGB_ICC.read_bytes()))
    (OUT / "rgb-gray-icc.jpg").write_bytes(
        with_icc((ROOT / "tests/fixtures/sample.jpg").read_bytes(), GRAY_ICC.read_bytes()))
    (OUT / "rgb-icc-cicp-p3.jpg").write_bytes(with_icc(
        (ROOT / "tests/fixtures/sample.jpg").read_bytes(),
        with_cicp_tag(RGB_ICC.read_bytes(), [12, 13, 0, 1])))
    (OUT / "premultiplied-alpha.avif").write_bytes(premultiplied_avif())
    (OUT / "icc-nclx-12-13-6.avif").write_bytes(p3_nclx_beside_icc())


if __name__ == "__main__":
    main()
