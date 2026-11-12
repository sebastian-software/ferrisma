"""Render Ferrisma's compact mark with the Ferramenta family mark helpers.

A polished steel picture plate has a stepped pixel mountain cut through its
face into a dark graphite bed. A crowned copper sun sits in its sky; a graphite registration corner holds the plate. Run with Blender in
background mode. Family helpers are read, never modified.
"""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

EXPECTED = {
    "studio.py": "7b0616e4bc8899ccd98f93fb5f61337d21e21da0b8aa90df60bd36f469d824b7",
    "family_marks.py": "0988b720737796570a01bbb413e264909f449db61672bfd12555876ae722a891",
}
COMMIT = "5284a1264ed78f71c72387621e63fb997b7a8842"
args = argparse.ArgumentParser()
args.add_argument("--family-source", required=True, type=Path)
args.add_argument("--output", required=True, type=Path)
args.add_argument("--draft", action="store_true")
args.add_argument("--blend", type=Path, help="Optionally save the constructed scene.")
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
from family_marks import corner, cut, group, materials, polygon, rounded_outline, tile, verify_geometry

# Column heights of the stepped mountain, in pixel cells: a high peak and a lower one.
HEIGHTS = [2, 3, 4, 5, 4, 3, 4, 3]
CELL, BASE = 0.165, -0.58


def rebuild(obj, surface, radius):
    """Drop interpolated split normals after a boolean and add one fine edge radius."""
    mesh = bpy.data.meshes.new(f"Clean {obj.name}")
    mesh.from_pydata([tuple(v.co) for v in obj.data.vertices], [], [tuple(p.vertices) for p in obj.data.polygons])
    mesh.update()
    mesh.materials.append(surface)
    obj.data = mesh
    for part in mesh.polygons:
        part.use_smooth = abs(part.normal.z) < 0.999
    bevel = obj.modifiers.new("Fine machined cut radius", "BEVEL")
    bevel.width, bevel.segments = radius, 4
    bevel.harden_normals = True


bpy.ops.object.select_all(action="SELECT")
bpy.ops.object.delete(use_global=False)
result = scene("Ferrisma mark", draft=opts.draft)
result.render.resolution_x = result.render.resolution_y = 768 if opts.draft else 1600
steel, copper, graphite = materials()
well = material("Ferra dark picture recess", (0.013, 0.012, 0.010), 0.62, metallic=0.15)

carrier = group("Pixel picture plate", angles=(5, -9, -3))
corner("Graphite picture registration corner", 2.10, 2.02, 0.26, graphite, carrier, (-0.06, -0.10, -0.23))
tile("Recessed graphite picture bed", 1.80, 1.66, 0.10, 0.12, well, carrier, (0, 0.05, -0.12), crown=0)
plate = polygon("Polished steel picture plate", rounded_outline(1.80, 1.66, 0.12), 0.25, steel, carrier, (0, 0.05, 0.05), bevel=0.04)

# One through-cut outline: each column rises or falls by whole pixel cells.
x0 = -len(HEIGHTS) * CELL / 2
outline = [(x0, BASE)]
for i, height in enumerate(HEIGHTS):
    outline += [(x0 + i * CELL, BASE + height * CELL), (x0 + (i + 1) * CELL, BASE + height * CELL)]
outline.append((x0 + len(HEIGHTS) * CELL, BASE))
cut(plate, polygon("Temporary pixel mountain cutter", outline, 0.9, steel, carrier, (0, 0, 0.05), bevel=0))
rebuild(plate, steel, 0.012)

# A crowned copper sun in the open sky, as on the instrument's picture samples.
tile("Crowned copper sun", 0.46, 0.46, 0.18, 0.2299, copper, carrier, (0.42, 0.50, 0.24), crown=0.03)

checks = verify_geometry()
studio((0, 0, 0), (3.1, -4.1, 12), up=(0, 1, 0))
light("Rolling face reflection", (-5.5, 2.5, 8), 160, 1, 6, (1, 0.97, 0.93), (0, 0, 0))
render(result, opts.output.resolve())
if opts.blend:
    bpy.ops.wm.save_as_mainfile(filepath=str(opts.blend.resolve()), compress=True)
if not opts.draft:
    source = Path(__file__).resolve()
    sidecar = {"product": "Ferrisma", "role": "Transparent native Blender product mark master", "meaning": "A polished steel picture plate with a stepped pixel mountain cut through into a graphite bed, a crowned copper sun, and a graphite registration corner.", "renderer": f"Blender {bpy.app.version_string}, Cycles", "dimensions": "1600x1600", "samples": result.cycles.samples, "colorManagement": "AgX Medium High Contrast, sRGB alpha", "familySourceCommit": COMMIT, "familySourceHashes": EXPECTED, "constructionSha256": hashlib.sha256(source.read_bytes()).hexdigest(), "masterSha256": hashlib.sha256(opts.output.read_bytes()).hexdigest(), "geometryVerification": checks}
    opts.output.with_suffix(".png.json").write_text(json.dumps(sidecar, indent=2) + "\n")
print(f"Rendered {len(checks)} verified solid bodies to {opts.output}")
