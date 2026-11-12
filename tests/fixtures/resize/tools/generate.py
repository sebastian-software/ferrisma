#!/usr/bin/env python3
"""Regenerate the ImageMagick resize references in tests/fixtures/resize.

Standard library only. Requires ImageMagick 7 with HDRI (`magick`). The input
images are synthesized here and, bit for bit, by `tests/support/resize_oracle.rs`
(`pattern`). The FNV-1a 64 hash of every input is printed; the Rust test
asserts the same hashes, so the two generators cannot drift apart silently.

Each reference is ImageMagick's own linear-light result, written as
little-endian float32 RGBA with straight (unassociated) alpha in [0, 1]
before any transfer encoding or 8-bit quantization. The Rust test applies the
sRGB encoding and rounding itself, so final quantization stays a separate,
explicit step on both sides.

Usage, from the repository root:

    python3 -I tests/fixtures/resize/tools/generate.py
"""

import os
import subprocess
import sys
import tempfile

MASK = (1 << 64) - 1
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def pattern(width, height, alpha, seed):
    """Deterministic RGBA8 test image; mirrored by `pattern` in Rust."""
    state = seed & MASK
    out = bytearray()
    for y in range(height):
        for x in range(width):
            draws = []
            for _ in range(4):
                state = (state * 6364136223846793005 + 1442695040888963407) & MASK
                draws.append(state >> 56)
            region = (x * 4) // width
            if region == 0:
                rgb = draws[0:3]
            elif region == 1:
                v = 255 * ((x + y) & 1)
                rgb = [v, v, v]
            elif region == 2:
                rgb = [255, 0, 0] if (x * 8) // width < 5 else [0, 0, 255]
            else:
                rgb = [
                    (x * 255) // (width - 1),
                    (y * 255) // (height - 1),
                    (draws[0] & 1) * 255,
                ]
            a = 255
            if alpha:
                quadrant = (x * 2) // width + 2 * ((y * 2) // height)
                if quadrant == 1:
                    # Fully transparent, with saturated "garbage" color that
                    # must never leak into visible output.
                    rgb = [0, 255, 0]
                    a = 0
                elif quadrant == 2:
                    a = 255 if (x // 2) & 1 else 0
                elif quadrant == 3:
                    a = draws[3]
            out.extend(bytes([rgb[0], rgb[1], rgb[2], a]))
    return bytes(out)


def fnv1a64(data):
    h = 0xCBF29CE484222325
    for b in data:
        h ^= b
        h = (h * 0x100000001B3) & MASK
    return h


# (name, input (w, h, alpha, seed), crop (x, y, w, h) or None, output (w, h), operations)
# Operations are ImageMagick resize steps applied in linear light. ImageMagick
# has no single filter equal to Ferrisma's Area at every ratio, so Area is
# expressed per axis: `-scale` (pixel-area averaging) on downscaled axes and
# `-filter Triangle -resize` (the tent that a unit box over unit pixels gives)
# on upscaled axes. See the README for the derivation.
INPUTS = {
    "opaque": (48, 36, False, 0x5EED0001),
    "alpha": (40, 30, True, 0x5EED0002),
    "large_opaque": (240, 160, False, 0x5EED0003),
    "large_alpha": (240, 160, True, 0x5EED0004),
}

CASES = [
    ("opaque-up-lanczos3", "opaque", None, (101, 77), "lanczos"),
    ("opaque-up-area", "opaque", None, (101, 77), "area"),
    ("alpha-up-lanczos3", "alpha", None, (87, 61), "lanczos"),
    ("alpha-up-area", "alpha", None, (87, 61), "area"),
    ("large-opaque-extreme-lanczos3", "large_opaque", None, (3, 2), "lanczos"),
    ("large-opaque-extreme-area", "large_opaque", None, (3, 2), "area"),
    ("large-alpha-extreme-lanczos3", "large_alpha", None, (4, 3), "lanczos"),
    ("large-alpha-extreme-area", "large_alpha", None, (4, 3), "area"),
    ("opaque-crop-mixed-lanczos3", "opaque", (7, 5, 29, 23), (11, 40), "lanczos"),
    ("opaque-crop-mixed-area", "opaque", (7, 5, 29, 23), (11, 40), "area"),
    ("alpha-crop-odd-lanczos3", "alpha", (3, 1, 31, 27), (9, 8), "lanczos"),
    ("alpha-crop-odd-area", "alpha", (3, 1, 31, 27), (9, 8), "area"),
    ("alpha-column-lanczos3", "alpha", (13, 0, 1, 30), (2, 7), "lanczos"),
    ("large-alpha-mixed-lanczos3", "large_alpha", (1, 3, 237, 9), (5, 31), "lanczos"),
    ("large-alpha-mixed-area", "large_alpha", (1, 3, 237, 9), (5, 31), "area"),
]


def resize_ops(kind, src, dst):
    sw, sh = src
    dw, dh = dst
    if kind == "lanczos":
        # ImageMagick's Lanczos is the 3-lobe sinc-windowed sinc; force the
        # exact Sinc instead of its SincFast polynomial approximation.
        return [
            "-filter", "Lanczos",
            "-define", "filter:filter=Sinc",
            "-define", "filter:window=Sinc",
            "-define", "filter:lobes=3",
            "-resize", f"{dw}x{dh}!",
        ]
    ops = []
    # Horizontal axis first, then vertical; both are separable in float.
    if dw < sw:
        ops += ["-scale", f"{dw}x{sh}!"]
    elif dw > sw:
        ops += ["-filter", "Triangle", "-resize", f"{dw}x{sh}!"]
    if dh < sh:
        ops += ["-scale", f"{dw}x{dh}!"]
    elif dh > sh:
        ops += ["-filter", "Triangle", "-resize", f"{dw}x{dh}!"]
    return ops


def main():
    version = subprocess.run(
        ["magick", "-version"], check=True, capture_output=True, text=True
    ).stdout.splitlines()[0]
    print(version)
    if "HDRI" not in version:
        sys.exit("ImageMagick must be built with HDRI so negative lobes survive")
    with tempfile.TemporaryDirectory() as tmp:
        hashes = {}
        for name, (w, h, alpha, seed) in INPUTS.items():
            data = pattern(w, h, alpha, seed)
            hashes[name] = fnv1a64(data)
            with open(os.path.join(tmp, name + ".rgba"), "wb") as f:
                f.write(data)
            print(f"input {name} {w}x{h} alpha={alpha} fnv1a64={hashes[name]:016x}")
        for name, input_name, crop, out, kind in CASES:
            w, h, _, _ = INPUTS[input_name]
            cmd = [
                "magick",
                "-size", f"{w}x{h}",
                "-depth", "8",
                f"rgba:{os.path.join(tmp, input_name + '.rgba')}",
                "-set", "colorspace", "sRGB",
            ]
            src = (w, h)
            if crop:
                cx, cy, cw, ch = crop
                cmd += ["-crop", f"{cw}x{ch}+{cx}+{cy}", "+repage"]
                src = (cw, ch)
            cmd += ["-colorspace", "RGB"]
            cmd += resize_ops(kind, src, out)
            target = os.path.join(ROOT, name + ".f32")
            cmd += [
                "-depth", "32",
                "-define", "quantum:format=floating-point",
                "-endian", "LSB",
                f"rgba:{target}",
            ]
            subprocess.run(cmd, check=True)
            size = os.path.getsize(target)
            expected = out[0] * out[1] * 16
            if size != expected:
                sys.exit(f"{name}: {size} bytes, expected {expected}")
            print(name, " ".join(cmd[1:]).replace(tmp + os.sep, "").replace(ROOT + os.sep, ""))


if __name__ == "__main__":
    main()
