#!/usr/bin/env python3
"""Create deterministic framed seed corpora for the sustained fuzz workflow."""
from hashlib import sha256
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
out = Path(sys.argv[1])
def read(path):
    return (root / path).read_bytes()

def seed(lane, name, selector, image):
    data = bytes([selector]) + image
    if len(data) > 512 * 1024 + 1:
        raise ValueError(f"{name} exceeds the driver's input cap")
    path = out / lane / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)

fixtures = [
    ("jpeg", "tests/fixtures/sample.jpg", 0),
    ("jxl", "tests/fixtures/sample.jxl", 1),
    ("avif", "tests/fixtures/orientation/avif-1.avif", 2),
    ("webp", "tests/fixtures/webp/reference-lossy.webp", 3),
]
for name, path, selector in fixtures:
    for output, flag in [("avif", 0), ("webp", 4)]:
        seed("transcode", f"{name}-to-{output}", selector | flag, read(path))

# A range of containers and rejection paths, not assertions of decoder support.
for name in ["poc_b_506387278.avif", "color_grid_alpha_grid_shared_tiles.avif", "paris_icc_exif_xmp.avif"]:
    seed("transcode", name, 2 | 0x18, read("codecs/libavif-rs/upstream/libavif/tests/data/" + name))
seed("transcode", "avif-alpha", 2,
     read("codecs/libavif-rs/upstream/libavif/tests/data/abc_color_irot_alpha_irot.avif"))
# The former `transcode_own_avif` lane (merged into `transcode` on
# 2026-11-15) had byte-identical copies of the AVIF-output seeds above.
# Retained regressions from earlier campaigns (inputs without their selector).
hostile_jxl = read("tests/fixtures/hostile/jxl-unknown-colour-space.jxl")
seed("transcode", "jxl-unknown-colour-space", 5, hostile_jxl)
# Smoke run 43690971661: a TOC section declaring 1 GiB (selector 1, as found).
oversized_toc = read("tests/fixtures/hostile/jxl-oversized-toc-section.jxl")
seed("transcode", "jxl-oversized-toc-section", 1, oversized_toc)
# The JPEG XL orientations, formerly of the `transcode_own_jxl` lane, which
# differed from `transcode` only by its JPEG XL reader (merged 2026-11-15;
# its other seeds were byte-identical to these lanes' own).
for orientation in range(1, 9):
    seed("transcode", f"jxl-orientation-{orientation}", 1, read(f"tests/fixtures/orientation/jxl-{orientation}.jxl"))

# JPEG XL output lane: control byte = input format | profile << 2 (0 lossy
# d1.0, 1 lossless) | policy << 3 | geometry class << 5 (fuzz/src/lib.rs).
def jxl_out(name, fmt, path, geometry, policy=0, profiles=("lossy", "lossless")):
    image = read(path)
    for profile in profiles:
        selector = fmt | (4 if profile == "lossless" else 0) | policy << 3 | geometry << 5
        seed("transcode_jxl_out", f"{name}-{profile}", selector, image)

JXL_DATA = "codecs/libjxl-rs/tests/data/"
for name, path, fmt in fixtures:
    jxl_out(f"{name}-64x64", fmt, path, 3)
jxl_out("jpeg-small", 0, "tests/fixtures/sample.jpg", 0)
jxl_out("jpeg-min-distance", 0, "tests/fixtures/sample.jpg", 6, profiles=("lossy",))
jxl_out("jpeg-max-distance", 0, "tests/fixtures/sample.jpg", 7, profiles=("lossy",))
# Alpha at the input's own size (class 4 never enlarges a small input).
jxl_out("jxl-rgba-37x19", 1, "tests/fixtures/jxl-lossless/rgba-37x19.jxl", 4)
jxl_out("jxl-rgb-alpha8", 1, JXL_DATA + "effects_rgb_alpha8.jxl", 4)
jxl_out("avif-alpha", 2, "codecs/libavif-rs/upstream/libavif/tests/data/abc_color_irot_alpha_irot.avif", 4)
# 16-bit and 12-bit gray with alpha, VarDCT and Modular. The pipeline refuses
# gray JPEG XL under every policy; these seeds start on that rejection path.
jxl_out("jxl-gray-alpha16-effects", 1, JXL_DATA + "effects_gray_alpha16.jxl", 4)
jxl_out("jxl-gray-alpha16-vardct", 1, JXL_DATA + "vardct-gray-alpha16.jxl", 4)
jxl_out("jxl-gray-alpha12-modular", 1, JXL_DATA + "modular-gray-alpha12.jxl", 4)
# High bit depth RGB: 16-bit Modular palette (17x13), 31-bit Modular (3x8),
# and a 12-bit AVIF built with a sample transform (8-bit alpha).
jxl_out("jxl-rgb16-palette-17x13", 1, JXL_DATA + "modular-palette16.jxl", 4)
jxl_out("jxl-rgb31-3x8", 1, JXL_DATA + "modular-rgb31.jxl", 4)
jxl_out("avif-12bit-sample-transform", 2,
        "codecs/libavif-rs/upstream/libavif/tests/data/weld_sato_12B_8B_q0.avif", 3)
# Gray JPEG (no ICC: assume sRGB only because it is missing) and gray JPEG XL.
jxl_out("jpeg-gray-9x7", 0, JXL_DATA + "ycbcr-jpeg-gray-9x7.jpg", 4, policy=2)
jxl_out("jxl-gray-257x3", 1, JXL_DATA + "ycbcr-jpeg-gray-257x3.jxl", 4)
# Extreme aspect ratios: wide and tall inputs to wide and tall outputs.
jxl_out("jpeg-257x3-to-8192x1", 0, JXL_DATA + "ycbcr-jpeg-444-257x3.jpg", 1, policy=2)
jxl_out("jpeg-3x257-to-1x8192", 0, JXL_DATA + "ycbcr-jpeg-444-3x257.jpg", 2, policy=2)
jxl_out("jxl-257x3-inside", 1, JXL_DATA + "ycbcr-jpeg-420-257x3.jxl", 5)
jxl_out("jxl-3x257-inside", 1, JXL_DATA + "ycbcr-jpeg-420-3x257.jxl", 5)
# More than one 256x256 group: 300x260 at its own size.
jxl_out("jxl-rgb-300x260", 1, "tests/fixtures/jxl-lossless/rgb-300x260.jxl", 5)
jxl_out("jxl-unknown-colour-space", 1, "tests/fixtures/hostile/jxl-unknown-colour-space.jxl", 3)
jxl_out("jxl-oversized-toc-section", 1, "tests/fixtures/hostile/jxl-oversized-toc-section.jxl", 3)

# JPEG APP2 carries a full ICC profile. Mutate the tag table count and the
# first tag offset while retaining the container framing and a valid profile
# baseline, so profile parsing is reached before or after mutation.
jpeg = read("tests/fixtures/sample.jpg")
profile = read("tests/fixtures/icc/DisplayP3-v4.icc")
assert jpeg[:2] == b"\xff\xd8" and len(profile) >= 144
for name, icc in [("icc-valid", profile),
                  ("icc-tag-count-overflow", profile[:128] + b"\xff" * 4 + profile[132:]),
                  ("icc-tag-offset-overflow", profile[:136] + b"\xff" * 4 + profile[140:])]:
    payload = b"ICC_PROFILE\x00\x01\x01" + icc
    app2 = b"\xff\xe2" + (len(payload) + 2).to_bytes(2, "big") + payload
    seed("transcode", name, 0x18, jpeg[:2] + app2 + jpeg[2:])

for path in sorted(out.rglob("*")):
    if path.is_file():
        print(f"{sha256(path.read_bytes()).hexdigest()}  {path.relative_to(out)}")
