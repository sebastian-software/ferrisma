"""Render Ferrixel's compact raster loupe mark with pinned Ferramenta studio helpers."""
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
parser = argparse.ArgumentParser()
parser.add_argument('--family-source', required=True, type=Path)
parser.add_argument('--output', required=True, type=Path)
parser.add_argument('--draft', action='store_true')
opts = parser.parse_args(sys.argv[sys.argv.index('--') + 1:] if '--' in sys.argv else [])
family = opts.family_source.resolve()
helpers = family / 'design/ferra/blender'
for name, expected in EXPECTED.items():
    actual = hashlib.sha256((helpers / name).read_bytes()).hexdigest()
    if actual != expected:
        raise SystemExit(f'Unexpected Ferramenta helper hash: {name}: {actual}')
revision = subprocess.check_output(['git', '-C', str(family), 'rev-parse', 'HEAD'], text=True).strip()
if revision != COMMIT:
    raise SystemExit(f'Unexpected Ferramenta revision: {revision}')
sys.dont_write_bytecode = True
sys.path.insert(0, str(helpers))
import bpy
from studio import light, material, render, scene, studio
from hero_geometry import box, cylinder, ring, rod, surfaces, verify_geometry

bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)
result = scene('Ferrixel raster loupe mark', draft=opts.draft)
result.render.resolution_x = result.render.resolution_y = 768 if opts.draft else 1600
result.cycles.samples = 48 if opts.draft else 160
steel = material('Ferra polished steel', (0.54, 0.52, 0.48), 0.18)
graphite = material('Ferra narrow graphite recess', (0.033, 0.030, 0.026), 0.38, metallic=0.55)
mid = material('Raster matte steel inlay', (0.31, 0.30, 0.28), 0.40, metallic=0.65)
for surface, roughness in [(steel, 0.18), (graphite, 0.38), (mid, 0.40)]:
    for shader in surface.node_tree.nodes:
        if shader.type == 'BUMP':
            shader.inputs['Strength'].default_value = 0.035
            shader.inputs['Distance'].default_value = 0.001
        elif shader.type == 'MAP_RANGE':
            shader.inputs['To Min'].default_value = roughness - 0.015
            shader.inputs['To Max'].default_value = roughness + 0.015

_, rust, _, _ = surfaces()
# Weathered iron oxide occupies the broad ring face while polished steel lips
# keep both boundaries crisp at icon size. A shallow plate supports the cells.
ring('Graphite lens depth', 1.015, 0.650, 0.105, (0, 0, -0.10), graphite)
ring('Weathered oxide loupe face', 1.010, 0.650, 0.185, (0, 0, 0.025), rust)
for name, outer, inner in [('Bright outer steel lip', 1.018, 0.938),
                           ('Bright inner steel lip', 0.725, 0.645)]:
    lip = ring(name, outer, inner, 0.22, (0, 0, 0.027), steel)
    for modifier in lip.modifiers:
        if modifier.type == 'BEVEL':
            modifier.width = 0.023
            modifier.segments = 6
cylinder('Recessed steel lens ground', 0.642, 0.065, (0, 0, -0.104), mid, bevel=0.009)
# Seven broad cells give the lens an unmistakable pixel reading at 32px.
for row, col in [(0,0),(0,1),(1,0),(1,1),(1,2),(2,1),(2,2)]:
    x, y = (col-1)*0.31, (1-row)*0.31
    box(f'Raised raster cell {row}-{col}', (0.287, 0.287, 0.085),
        (x, y, -0.023), steel, 0.018)
# A short, coherent handle grows from the ring. The rust grip is substantial,
# with a visible steel ferrule and end to retain the family material contrast.
rod('Steel loupe handle', (0.62, -0.62, -0.04), (1.35, -1.35, -0.04), 0.145, steel)
rod('Weathered oxide handle grip', (0.80, -0.80, -0.04), (1.20, -1.20, -0.04), 0.171, rust)
checks = verify_geometry()
studio((0.10, -0.10, 0), (3.1, -4.1, 12), up=(0, 1, 0))
light('Rolling face reflection', (-5.5, 2.5, 8), 160, 1, 6, (1, 0.97, 0.93), (0, 0, 0))
render(result, opts.output.resolve())
if not opts.draft:
    source = Path(__file__).resolve()
    record = {
        'product': 'Ferrixel',
        'role': 'Transparent native Blender raster loupe product mark master',
        'meaning': 'A weathered iron-oxide loupe with polished steel lips frames seven raised raster cells; a patinated grip completes the handle.',
        'renderer': f'Blender {bpy.app.version_string}, Cycles',
        'dimensions': '1600x1600', 'samples': result.cycles.samples,
        'colorManagement': 'AgX Medium High Contrast, sRGB alpha',
        'familySourceCommit': COMMIT, 'familySourceHashes': EXPECTED,
        'constructionSha256': hashlib.sha256(source.read_bytes()).hexdigest(),
        'masterSha256': hashlib.sha256(opts.output.read_bytes()).hexdigest(),
        'geometryVerification': checks,
        'verificationScope': 'Each evaluated mesh is closed and has positive volume; assembly contacts are not generally collision-checked.',
    }
    opts.output.with_suffix('.png.json').write_text(json.dumps(record, indent=2) + '\n')
print(f'Rendered {len(checks)} verified solids to {opts.output}')
