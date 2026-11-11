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
        if output == "avif":
            seed("transcode_own_avif", f"{name}-to-{output}", selector, read(path))
        if name == "jxl":
            seed("transcode_own_jxl", f"{name}-to-{output}", selector | flag, read(path))

# A range of containers and rejection paths, not assertions of decoder support.
for name in ["poc_b_506387278.avif", "color_grid_alpha_grid_shared_tiles.avif", "paris_icc_exif_xmp.avif"]:
    seed("transcode", name, 2 | 0x18, read("codecs/libavif-rs/upstream/libavif/tests/data/" + name))
seed("transcode", "avif-alpha", 2,
     read("codecs/libavif-rs/upstream/libavif/tests/data/abc_color_irot_alpha_irot.avif"))
seed("transcode_own_avif", "avif-alpha", 2,
     read("codecs/libavif-rs/upstream/libavif/tests/data/abc_color_irot_alpha_irot.avif"))
for orientation in range(1, 9):
    seed("transcode_own_jxl", f"orientation-{orientation}", 1, read(f"tests/fixtures/orientation/jxl-{orientation}.jxl"))

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
