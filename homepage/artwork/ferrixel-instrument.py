"""Render Ferrixel's optical registration bench with the Ferramenta studio.

Run with Blender in background mode. Family helpers are read, never modified.
"""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

EXPECTED = {
    "studio.py": "7b0616e4bc8899ccd98f93fb5f61337d21e21da0b8aa90df60bd36f469d824b7",
    "hero_geometry.py": "060cd7feb85d6477e6a79d97c26e41f87bdd7f6d3899a36981b8e8a89fabdd17",
}
COMMIT = "5284a1264ed78f71c72387621e63fb997b7a8842"
args = argparse.ArgumentParser()
args.add_argument("--family-source", required=True, type=Path)
args.add_argument("--output", required=True, type=Path)
args.add_argument("--draft", action="store_true")
opts = args.parse_args(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
family = opts.family_source.resolve()
helpers = family / "design/ferra/blender"
for name, expected in EXPECTED.items():
    actual = hashlib.sha256((helpers / name).read_bytes()).hexdigest()
    if actual != expected:
        raise SystemExit(f"Unexpected Ferramenta helper hash: {name}: {actual}")
revision = subprocess.check_output(["git", "-C", str(family), "rev-parse", "HEAD"], text=True).strip()
if revision != COMMIT:
    raise SystemExit(f"Unexpected Ferramenta revision: {revision}")
sys.dont_write_bytecode = True
sys.path.insert(0, str(helpers))
import bpy
from studio import light, material, render, scene, studio
from hero_geometry import bench, box, cylinder, knob, rod, screw, surfaces, verify_geometry

bpy.ops.object.select_all(action="SELECT")
bpy.ops.object.delete(use_global=False)
result = scene("Ferrixel optical registration bench", draft=opts.draft)
result.render.resolution_x = 1200 if opts.draft else 2400
result.render.resolution_y = 600 if opts.draft else 1200
result.cycles.samples = 32 if opts.draft else 192
steel, oxide, graphite, dark = surfaces()
blue = material("Muted blue enamel", (0.10, 0.19, 0.21), 0.36, metallic=0.25)
cream = material("Warm ivory enamel", (0.66, 0.61, 0.49), 0.4, metallic=0.15)
bench("Image registration bench", 7.1, 2.4, steel, graphite, dark)
# Two trays display a large source raster and a smaller output raster.
for name, x, cols, rows, step in [("Source", -2.05, 6, 4, 0.22), ("Output", 2.18, 3, 2, 0.25)]:
    width, depth = cols * step + 0.20, rows * step + 0.20
    box(f"{name} tray", (width, depth, 0.14), (x, -0.15, 0.41), graphite, 0.045)
    for col in range(cols):
        for row in range(rows):
            palette = [steel, cream, blue, oxide]
            surface = palette[(col + row * 2 + (col * row) % 3) % len(palette)]
            px = x + (col - (cols - 1) / 2) * step
            py = -0.15 + (row - (rows - 1) / 2) * step
            box(f"{name} sample {col}-{row}", (step * 0.83, step * 0.83, 0.055), (px, py, 0.51), surface, 0.012)
# An elevated open aperture registers a region of the larger mosaic.
for x in [-2.53, -1.56]:
    box("Crop aperture side", (0.07, 0.83, 0.07), (x, -0.15, 0.67), oxide, 0.012)
for y in [-0.56, 0.26]:
    box("Crop aperture crossbar", (1.04, 0.07, 0.07), (-2.045, y, 0.67), oxide, 0.012)
for x in [-2.53, -1.56]:
    for y in [-0.56, 0.26]:
        cylinder("Aperture standoff", 0.035, 0.16, (x, y, 0.59), steel, bevel=0.006)
# Parallel rails and a retained carriage make the transformation legible.
for y in [-0.94, 0.73]:
    rod("Polished guide", (-2.9, y, 0.56), (2.95, y, 0.56), 0.045, steel)
    for x in [-2.9, 2.95]:
        cylinder("Guide support", 0.11, 0.25, (x, y, 0.43), steel)
box("Filter carriage", (0.95, 1.47, 0.25), (0, -0.1, 0.64), steel, 0.06)
box("Carriage inset", (0.69, 1.05, 0.07), (0, -0.1, 0.80), graphite, 0.025)
for offset, surface in [(-0.20, steel), (0, oxide), (0.20, steel)]:
    box("Filter layer", (0.13, 0.68, 0.16), (offset, -0.1, 0.91), surface, 0.02)
for y in [-0.86, 0.65]:
    screw("Carriage retainer", (0, y, 0.77), steel, dark, radius=0.07)
knob("Registration adjuster", (0.65, 0.68, 0.44), steel, oxide, dark, radius=0.16)
checks = verify_geometry()
studio((0, 0, 0.6), (4.8, -10, 8.3))
light("Long front reflection", (-1, -6, 6), 220, 6, 1.4, (1, 0.97, 0.93), (0, 0, 0.7))
render(result, opts.output.resolve())
if not opts.draft:
    source = Path(__file__).resolve()
    sidecar = {"product": "Ferrixel", "role": "Transparent native Blender homepage instrument", "meaning": "An image raster, a registered crop aperture, a filter carriage, and a smaller output raster.", "renderer": f"Blender {bpy.app.version_string}, Cycles", "dimensions": "2400x1200", "samples": result.cycles.samples, "colorManagement": "AgX Medium High Contrast, sRGB alpha", "familySourceCommit": COMMIT, "familySourceHashes": EXPECTED, "constructionSha256": hashlib.sha256(source.read_bytes()).hexdigest(), "masterSha256": hashlib.sha256(opts.output.read_bytes()).hexdigest(), "geometryVerification": checks, "verificationScope": "Each evaluated mesh is closed and has positive volume; Boolean cuts remove material. Assembly contacts are not generally collision-checked."}
    opts.output.with_suffix(".png.json").write_text(json.dumps(sidecar, indent=2) + "\n")
print(f"Rendered {len(checks)} verified solid bodies to {opts.output}")
