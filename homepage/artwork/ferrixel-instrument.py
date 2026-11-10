"""Render Ferrixel's steel raster inspection loupe with the Ferramenta studio.

Run with Blender in background mode. Family helpers are read, never modified.
"""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

import bpy

EXPECTED = {
    'studio.py': '7b0616e4bc8899ccd98f93fb5f61337d21e21da0b8aa90df60bd36f469d824b7',
    'hero_geometry.py': '060cd7feb85d6477e6a79d97c26e41f87bdd7f6d3899a36981b8e8a89fabdd17',
}
COMMIT = '5284a1264ed78f71c72387621e63fb997b7a8842'
args = argparse.ArgumentParser()
args.add_argument('--family-source', required=True, type=Path)
args.add_argument('--output', required=True, type=Path)
args.add_argument('--draft', action='store_true')
opts = args.parse_args(sys.argv[sys.argv.index('--') + 1:] if '--' in sys.argv else [])
family = opts.family_source.resolve()
helpers = family / 'design/ferra/blender'
for name, digest in EXPECTED.items():
    actual = hashlib.sha256((helpers / name).read_bytes()).hexdigest()
    if actual != digest:
        raise SystemExit(f'Unexpected Ferramenta helper hash: {name}: {actual}')
revision = subprocess.check_output(['git', '-C', str(family), 'rev-parse', 'HEAD'], text=True).strip()
if revision != COMMIT:
    raise SystemExit(f'Unexpected Ferramenta revision: {revision}')
sys.dont_write_bytecode = True
sys.path.insert(0, str(helpers))
from studio import light, material, render, scene, studio
from hero_geometry import bench, box, cylinder, engrave_lines, ring, rod, rounded_plate, screw, surfaces, verify_geometry

bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)
s = scene('Ferrixel raster inspection loupe', draft=opts.draft)
s.render.resolution_x = 1200 if opts.draft else 2400
s.render.resolution_y = 600 if opts.draft else 1200
s.cycles.samples = 32 if opts.draft else 192
steel, oxide, graphite, dark = surfaces()
bright = material('Inspectors bright satin steel', (0.72, 0.70, 0.67), 0.23)
mid = material('Etched half-tone steel', (0.36, 0.35, 0.33), 0.44)

# Shared machined base with four physically bored, inset hex anchors.
bench('Loupe', 5.8, 3.0, steel, graphite, dark)
box('Shallow specimen stage', (4.83, 2.14, 0.09), (-0.03, 0, 0.405), bright, 0.055)
source_top = 0.505
source = rounded_plate('Fine specimen image plate', 2.28, 1.58, 0.07,
                       source_top, bright, location=(-1.30, -0.03), corner=0.08)
fine = []
for col in range(1, 9):
    fine.append((-2.31 + col*0.225, -0.03, 0.008, 1.32))
for row in range(1, 6):
    fine.append((-1.30, -0.68 + row*0.216, 1.96, 0.008))
engrave_lines(source, fine, dark, face_z=source_top)
source_cells = []
for row in range(5):
    for col in range(8):
        if (row >= 3 and col <= row+1) or (row == 1 and col == 6):
            source_cells.append((-2.20 + col*0.225, -0.55 + row*0.216, 0.15, 0.15))
engrave_lines(source, source_cells, mid, face_z=source_top)

# A visibly coarser copy sits in the aperture. It reads as inspected samples,
# while the adjacent fine plate makes the change of scale legible.
target_top = 0.557
target = rounded_plate('Enlarged sample window', 1.82, 1.82, 0.085,
                       target_top, bright, location=(1.10, -0.03), corner=0.18)
coarse = []
for col in range(1, 6):
    coarse.append((0.30 + col*0.266, -0.03, 0.012, 1.46))
for row in range(1, 6):
    coarse.append((1.10, -0.84 + row*0.266, 1.46, 0.012))
engrave_lines(target, coarse, dark, face_z=target_top)
large_cells = []
for row in range(5):
    for col in range(5):
        if (row >= 3 and col <= row) or (row == 1 and col == 4):
            large_cells.append((0.56 + col*0.266, -0.57 + row*0.266, 0.20, 0.20))
engrave_lines(target, large_cells, mid, face_z=target_top)

# Optics mount: one base-connected pivot, angled arm, fork and circular barrel.
# The open viewing ring leaves the enlarged grid genuinely visible.
cylinder('Loupe pivot ground socket', 0.27, 0.22, (2.28, 0.80, 0.54), steel)
cylinder('Loupe pivot axle', 0.115, 0.39, (2.28, 0.80, 0.76), bright)
ring('Pivot thrust washer', 0.18, 0.117, 0.04, (2.28, 0.80, 0.91), graphite)
rod('Single angled inspection arm', (2.28, 0.80, 0.88),
    (1.78, 0.62, 1.33), 0.092, steel)
box('Loupe barrel fork', (0.48, 0.25, 0.16), (1.76, 0.62, 1.31), bright, 0.038)
screw('Fork retaining socket', (1.77, 0.63, 1.39), bright, dark, radius=0.064)
ring('Magnifier bright outer barrel', 1.02, 0.82, 0.18,
     (1.10, -0.03, 1.27), bright)
ring('Magnifier inner optical rim', 0.85, 0.80, 0.055,
     (1.10, -0.03, 1.19), steel)
ring('Machined focus scale', 1.04, 1.015, 0.047,
     (1.10, -0.03, 1.365), graphite)
box('Small oxide focal index', (0.095, 0.16, 0.055),
    (2.09, -0.03, 1.385), oxide, 0.011)
# A slim under-arm stiffener lands on the base rather than floating beside it.
rod('Pivot brace', (2.27, 0.80, 0.51), (2.05, 0.80, 0.96), 0.044, bright)

checks = verify_geometry()
studio((0, 0, 0.65), (5.3, -9.8, 8.3))
light('Long front reflection', (-1, -6, 6), 220, 6, 1.4, (1, 0.97, 0.93), (0, 0, 0.7))
light('Low upright-face reflection card', (-3, -5, -1.5), 140, 6, 3, (1, 0.97, 0.93), (0, 0, 1))
render(s, opts.output.resolve())
if not opts.draft:
    source = Path(__file__).resolve()
    sidecar = {
        'product': 'Ferrixel',
        'role': 'Transparent native Blender homepage instrument',
        'meaning': 'A steel inspection ring frames an enlarged raster beside a fine source grid.',
        'renderer': f'Blender {bpy.app.version_string}, Cycles',
        'dimensions': '2400x1200',
        'samples': s.cycles.samples,
        'colorManagement': 'AgX Medium High Contrast, sRGB alpha',
        'familySourceCommit': COMMIT,
        'familySourceHashes': EXPECTED,
        'constructionSha256': hashlib.sha256(source.read_bytes()).hexdigest(),
        'masterSha256': hashlib.sha256(opts.output.read_bytes()).hexdigest(),
        'geometryVerification': checks,
        'verificationScope': 'Each evaluated mesh is closed and has positive volume; Boolean cuts remove material. Assembly contacts are not generally collision-checked.',
    }
    opts.output.with_suffix('.png.json').write_text(json.dumps(sidecar, indent=2) + '\n')
print(f'Rendered {len(checks)} verified solid bodies to {opts.output}')
