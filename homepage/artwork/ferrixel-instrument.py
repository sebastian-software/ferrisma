"""Render Ferrixel's image trimming, resizing, and inspection workbench.

Run with Blender in background mode. Family helpers are read, never modified.
"""
import argparse
import hashlib
import json
import math
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
bright = material('Warm satin tool steel', (0.69, 0.67, 0.63), 0.25)
paper = material('Warm ivory photo paper', (0.76, 0.73, 0.65), 0.6, metallic=0.04)
sky = material('Photo pale sky', (0.80, 0.78, 0.72), 0.65, metallic=0.02)
sun = material('Photo copper sun', (0.36, 0.14, 0.055), 0.82, metallic=0.0)
snow = material('Photo mountain highlights', (0.37, 0.37, 0.35), 0.82, metallic=0.0)
ridge = material('Photo stone ridge', (0.052, 0.055, 0.056), 0.87, metallic=0.0)
shadow = material('Photo dark mountain', (0.012, 0.015, 0.018), 0.90, metallic=0.0)
ground = material('Photo foreground', (0.075, 0.080, 0.076), 0.87, metallic=0.0)

# All three stations attach to one bed and share the same fore-and-aft rails.
bench('Image workbench', 8.35, 3.2, steel, graphite, dark)
box('Continuous specimen rail', (7.52, 0.12, 0.13), (0, 1.12, 0.415), steel, 0.023)
box('Continuous front registration rail', (7.52, 0.11, 0.10), (0, -1.13, 0.40), bright, 0.018)
for x in (-3.75, -1.52, 1.28, 3.75):
    screw('Rail fixing', (x, 1.12, 0.48), steel, dark, radius=0.057)
for x in (-1.48, 1.32):
    box('Station joint', (0.055, 2.30, 0.035), (x, 0, 0.347), graphite, 0.01)

def landscape_tone(col, row, cols, rows):
    """One two-peak landscape, sampled on three physical photo specimens."""
    x = (col + 0.5) / cols
    y = (row + 0.5) / rows
    first = 0.43 + 0.34 * max(0, 1 - abs(x - 0.28) / 0.31)
    second = 0.41 + 0.31 * max(0, 1 - abs(x - 0.72) / 0.29)
    skyline = max(first, second)
    if (x - 0.68) ** 2 + (y - 0.80) ** 2 < 0.17 ** 2:
        return sun
    if y > skyline:
        return sky
    if y > skyline - 0.11:
        return snow
    if y > 0.28:
        return ridge if x < 0.51 else shadow
    return ground

def specimen(name, center, size, cells, top, tile_gap):
    """A paper-backed photo whose filled samples form the same landscape."""
    cx, cy = center
    width, height = size
    cols, rows = cells
    rounded_plate(f'{name} paper border', width + 0.17, height + 0.17,
                  0.043, top, paper, location=center, corner=0.035)
    cell_w, cell_h = width / cols, height / rows
    for row in range(rows):
        for col in range(cols):
            x = cx - width / 2 + (col + 0.5) * cell_w
            y = cy - height / 2 + (row + 0.5) * cell_h
            box(f'{name} pixel {col}-{row}',
                (cell_w - tile_gap, cell_h - tile_gap, 0.012),
                (x, y, top + 0.009), landscape_tone(col, row, cols, rows),
                min(0.008, cell_w / 10))

# Left: a trim board with a registration fence, measured guide and lifted blade.
rounded_plate('Photo trimming board', 2.38, 2.24, 0.12, 0.46,
              bright, location=(-2.72, 0), corner=0.10)
box('Trimmer dark cutting slot', (0.045, 1.72, 0.014),
    (-1.62, -0.03, 0.527), graphite, 0.005)
box('Trimmer rear paper fence', (2.23, 0.10, 0.24),
    (-2.73, 0.99, 0.59), steel, 0.022)
box('Trimmer left square', (0.09, 1.62, 0.16),
    (-3.78, -0.02, 0.56), steel, 0.012)
specimen('Source photo', (-2.79, -0.04), (1.50, 1.31), (11, 8), 0.555, 0.006)
for index in range(12):
    x = -3.70 + index * 0.174
    box(f'Trimmer ruler tick {index}',
        (0.012, 0.12 if index % 3 == 0 else 0.065, 0.007),
        (x, 1.005, 0.715), graphite, 0.002)
cylinder('Blade hinge bearing', 0.15, 0.18, (-1.64, 0.94, 0.69), steel)
rod('Lifted oxide cutting lever', (-1.64, 0.91, 0.75),
    (-1.54, -0.89, 1.08), 0.090, oxide)
rod('Inset polished cutting edge', (-1.62, 0.85, 0.72),
    (-1.52, -0.84, 1.03), 0.022, bright)
blade = box('Raised flat steel shear blade', (0.11, 1.80, 0.045),
            (-1.66, 0.01, 0.91), bright, 0.007)
blade.rotation_euler[0] = -0.18
cylinder('Blade rear pivot cap', 0.11, 0.08, (-1.64, 0.94, 0.79), oxide)
screw('Blade hinge fastener', (-1.64, 0.94, 0.84), steel, dark, radius=0.055)
box('Photo retention clamp', (0.43, 0.13, 0.10),
    (-3.45, -0.86, 0.635), oxide, 0.016)
screw('Clamp adjustment', (-3.45, -0.86, 0.70), bright, dark, radius=0.047)

# Middle: a screw-operated compression carriage with a smaller copy of the photo.
rounded_plate('Resizing press tray', 2.55, 2.24, 0.13, 0.47,
              steel, location=(-0.06, 0), corner=0.12)
box('Press lower die', (1.88, 1.61, 0.085), (-0.06, -0.12, 0.54), graphite, 0.028)
specimen('Reduced photo', (-0.06, -0.21), (1.22, 1.07), (8, 6), 0.605, 0.010)
for x in (-1.06, 0.94):
    cylinder('Press foot bearing', 0.19, 0.13, (x, 0.83, 0.56), steel)
    cylinder('Press guide post', 0.10, 0.95, (x, 0.83, 1.08), bright)
    ring('Press post collar', 0.15, 0.104, 0.075, (x, 0.83, 1.48), graphite)
box('Weathered resizing yoke', (2.27, 0.32, 0.30),
    (-0.06, 0.83, 1.53), oxide, 0.055)
box('Moving press platen', (1.78, 0.28, 0.18),
    (-0.06, 0.57, 0.91), bright, 0.035)
box('Depth-extending moving die face', (1.54, 0.76, 0.12),
    (-0.06, 0.47, 0.85), steel, 0.025)
cylinder('Lead screw to platen coupling', 0.097, 0.57,
         (-0.06, 0.73, 1.12), graphite)
for x in (-0.74, 0.62):
    rod('Platen hanger', (x, 0.73, 1.32), (x, 0.57, 0.96), 0.055, steel)
cylinder('Press lead shaft', 0.087, 0.59, (-0.06, 0.83, 1.83), steel)
for index in range(12):
    ring(f'Lead screw thread {index}', 0.100, 0.087, 0.014,
         (-0.06, 0.83, 1.59 + index * 0.043), bright)
ring('Press handwheel rim', 0.35, 0.28, 0.085,
     (-0.06, 0.83, 2.11), oxide)
for angle in (0, math.pi / 2, math.pi, 3 * math.pi / 2):
    rod('Handwheel spoke', (-0.06, 0.83, 2.11),
        (-0.06 + 0.32 * math.cos(angle), 0.83 + 0.32 * math.sin(angle), 2.11),
        0.032, bright)
cylinder('Handwheel hub', 0.13, 0.13, (-0.06, 0.83, 2.11), oxide)
screw('Press spindle keeper', (-0.06, 0.83, 2.18), bright, dark, radius=0.06)
for x in (-1.08, 0.96):
    screw('Press tray keeper', (x, -0.94, 0.48), steel, dark, radius=0.07)

# Right: a mounted loupe frames coarse, filled cells of the same photograph.
rounded_plate('Inspection window carrier', 2.30, 2.24, 0.12, 0.46,
              bright, location=(2.72, 0), corner=0.12)
box('Inset photo well', (1.93, 1.77, 0.055), (2.53, -0.08, 0.53), graphite, 0.04)
specimen('Magnified mountain photo', (2.50, 0.13),
         (1.68, 1.68), (7, 7), 0.585, 0.010)
cylinder('Loupe bearing pedestal', 0.25, 0.25, (3.64, 0.80, 0.59), steel)
cylinder('Loupe upright spindle', 0.12, 0.62, (3.64, 0.80, 0.97), oxide)
ring('Loupe thrust bearing', 0.18, 0.12, 0.052,
     (3.64, 0.80, 1.22), graphite)
rod('Weathered articulated inspection arm', (3.64, 0.80, 1.15),
    (3.03, 0.51, 1.08), 0.11, oxide)
box('Steel optical barrel fork', (0.48, 0.27, 0.17),
    (3.02, 0.51, 1.08), bright, 0.033)
screw('Loupe fork keeper', (3.02, 0.51, 1.17), bright, dark, radius=0.06)
ring('Weathered loupe barrel', 0.91, 0.76, 0.16,
     (2.49, -0.09, 1.03), oxide)
ring('Polished loupe front bevel', 0.93, 0.855, 0.055,
     (2.49, -0.09, 1.12), bright)
ring('Polished loupe inner rim', 0.78, 0.745, 0.055,
     (2.49, -0.09, 0.96), steel)
ring('Loupe focusing track', 0.94, 0.925, 0.037,
     (2.49, -0.09, 1.145), graphite)
box('Focus witness mark', (0.11, 0.13, 0.05),
    (3.41, -0.09, 1.17), bright, 0.01)
for y in (-0.95, 0.97):
    screw('Inspection carrier keeper', (3.68, y, 0.48), steel, dark, radius=0.065)

checks = verify_geometry()
studio((0, 0, 0.75), (5.3, -9.8, 8.3))
light('Long front reflection', (-1, -6, 6), 220, 6, 1.4, (1, 0.97, 0.93), (0, 0, 0.7))
light('Low upright-face reflection card', (-3, -5, -1.5), 140, 6, 3, (1, 0.97, 0.93), (0, 0, 1))
render(s, opts.output.resolve())
if not opts.draft:
    source = Path(__file__).resolve()
    sidecar = {
        'product': 'Ferrixel',
        'role': 'Transparent native Blender homepage instrument',
        'meaning': 'One steel workbench trims, presses, and inspects three scales of the same mountain photograph.',
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
