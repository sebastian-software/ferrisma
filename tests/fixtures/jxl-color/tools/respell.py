#!/usr/bin/env python3
"""Rewrite only the enum color encoding of a bare JPEG XL codestream.

Standard library only. No JPEG XL encoder is available on the development
host, so color-signalling fixtures for issue #41 are derived from existing,
independently encoded files: this tool parses the ImageHeader (ISO/IEC
18181-1 Annex A: SizeHeader, ImageMetadata) up to and including
`colour_encoding`, replaces that bundle's bits with a new enum encoding, and
copies every later bit unchanged. The frame data are byte-aligned after the
header, so the image samples stay bit-identical; only their declared color
meaning changes. The source header must not be `all_default` and must not
carry an ICC profile.

Usage, from the repository root:

    python3 -I tests/fixtures/jxl-color/tools/respell.py
"""

import hashlib
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.dirname(os.path.dirname(os.path.dirname(ROOT)))


class Reader:
    def __init__(self, data):
        self.data = data
        self.pos = 0

    def bits(self, n):
        value = 0
        for i in range(n):
            byte = self.data[(self.pos + i) // 8]
            value |= ((byte >> ((self.pos + i) % 8)) & 1) << i
        self.pos += n
        return value

    def bool(self):
        return self.bits(1) == 1

    def u32(self, dists):
        kind, a, b = dists[self.bits(2)]
        if kind == "val":
            return a
        return a + self.bits(b)

    def enum(self):
        return self.u32(ENUM)


class Writer:
    def __init__(self):
        self.out = []

    def bits(self, n, value):
        for i in range(n):
            self.out.append((value >> i) & 1)

    def enum(self, value):
        if value in (0, 1):
            self.bits(2, value)
        elif 2 <= value < 18:
            self.bits(2, 2)
            self.bits(4, value - 2)
        else:
            self.bits(2, 3)
            self.bits(6, value - 18)


ENUM = [("val", 0, 0), ("val", 1, 0), ("bits", 2, 4), ("bits", 18, 6)]
SIZE = [("bits", 1, 9), ("bits", 1, 13), ("bits", 1, 18), ("bits", 1, 30)]


def skip_size_header(r):
    div8 = r.bool()
    if div8:
        r.bits(5)
    else:
        r.u32(SIZE)
    ratio = r.bits(3)
    if ratio == 0:
        if div8:
            r.bits(5)
        else:
            r.u32(SIZE)


def find_colour_encoding(data):
    """Bit range [start, end) of ImageMetadata.colour_encoding."""
    assert data[:2] == b"\xff\x0a", "bare codestream required"
    r = Reader(data)
    r.pos = 16
    skip_size_header(r)
    assert not r.bool(), "all_default metadata cannot be respelled"
    extra_fields = r.bool()
    if extra_fields:
        r.bits(3)  # orientation - 1
        assert not r.bool(), "intrinsic size unsupported"
        assert not r.bool(), "preview unsupported"
        assert not r.bool(), "animation unsupported"
    # BitDepth
    if r.bool():  # float_sample
        r.u32([("val", 32, 0), ("val", 16, 0), ("val", 24, 0), ("bits", 1, 6)])
        r.bits(4)
    else:
        r.u32([("val", 8, 0), ("val", 10, 0), ("val", 12, 0), ("bits", 1, 6)])
    r.bool()  # modular_16bit_buffers
    num_extra = r.u32([("val", 0, 0), ("val", 1, 0), ("bits", 2, 4), ("bits", 1, 12)])
    assert num_extra == 0, "extra channels unsupported"
    xyb = r.bool()
    start = r.pos
    if not r.bool():  # colour_encoding.all_default
        assert not r.bool(), "ICC-tagged source unsupported"
        space = r.enum()
        if space != 3:  # not XYB
            if r.enum() == 2:
                raise AssertionError("custom white point unsupported")
        if space not in (1, 3):  # not grey, not XYB
            if r.enum() == 2:
                raise AssertionError("custom primaries unsupported")
        if r.bool():
            r.bits(24)
        else:
            r.enum()
        r.enum()  # rendering intent
    return start, r.pos, xyb


def encode_colour(space, white, primaries, transfer, intent):
    w = Writer()
    w.bits(1, 0)  # not all_default
    w.bits(1, 0)  # want_icc = false
    w.enum(space)
    w.enum(white)
    w.enum(primaries)
    w.bits(1, 0)  # have_gamma = false
    w.enum(transfer)
    w.enum(intent)
    return w.out


def respell(data, colour_bits):
    start, end, xyb = find_colour_encoding(data)
    total = len(data) * 8
    bits = [(data[i // 8] >> (i % 8)) & 1 for i in range(total)]
    # Find the byte-aligned end of the image header: the remaining metadata
    # fields are copied verbatim, so the aligned frame data must follow the
    # same number of header bits. Pad the header to a byte boundary again.
    header_end = header_length(data, end, xyb)
    new = bits[:start] + colour_bits + bits[end:header_end]
    while len(new) % 8:
        new.append(0)
    out = bytearray()
    for i in range(0, len(new), 8):
        out.append(sum(bit << j for j, bit in enumerate(new[i:i + 8])))
    aligned = (header_end + 7) // 8
    return bytes(out) + data[aligned:]


def header_length(data, colour_end, xyb):
    """Bit position at the end of ImageMetadata (before zero padding)."""
    r = Reader(data)
    r.pos = 16
    skip_size_header(r)
    r.bool()
    extra_fields = r.bool()
    r.pos = colour_end
    if extra_fields:
        if not r.bool():  # tone_mapping.all_default
            r.bits(16)  # intensity_target
            r.bits(16)  # min_nits
            r.bits(1)  # relative_to_max_display
            r.bits(16)  # linear_below
    # Extensions: U64 extension bits; only zero supported.
    selector = r.bits(2)
    assert selector == 0, "extensions unsupported"
    default_m = r.bool()
    assert default_m, "custom opsin/upsampling weights unsupported"
    return r.pos


# (output name, source, colour space, white point, primaries, transfer, intent)
# Enum values (ISO/IEC 18181-1): space RGB 0; white D65 1, DCI 11;
# primaries sRGB 1, BT.2100 9, P3 11; transfer BT.709 1, linear 8, sRGB 13,
# PQ 16, DCI 17, HLG 18; intent perceptual 0, relative 1.
VARIANTS = [
    ("lossless-p3", "tests/fixtures/orientation/jxl-1.jxl", 0, 1, 11, 13, 1),
    ("lossless-pq", "tests/fixtures/orientation/jxl-1.jxl", 0, 1, 9, 16, 1),
    ("lossless-hlg", "tests/fixtures/orientation/jxl-1.jxl", 0, 1, 9, 18, 1),
    ("lossless-linear-srgb", "tests/fixtures/orientation/jxl-1.jxl", 0, 1, 1, 8, 1),
    ("lossless-dci-p3", "tests/fixtures/orientation/jxl-1.jxl", 0, 11, 11, 17, 1),
    ("xyb-p3", "tests/fixtures/sample.jxl", 0, 1, 11, 13, 1),
    ("xyb-pq", "tests/fixtures/sample.jxl", 0, 1, 9, 16, 1),
]


def main():
    for name, source, *colour in VARIANTS:
        with open(os.path.join(REPO, source), "rb") as f:
            data = f.read()
        out = respell(data, encode_colour(*colour))
        target = os.path.join(ROOT, name + ".jxl")
        with open(target, "wb") as f:
            f.write(out)
        digest = hashlib.sha256(out).hexdigest()
        print(f"{name}.jxl from {source}: {len(out)} bytes, sha256 {digest}")


if __name__ == "__main__":
    main()
