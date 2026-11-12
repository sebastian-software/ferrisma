"""Render Ferrisma's image railway with the Ferramenta studio.

A gently curved steel track carries three wagons through three stations: a source
picture under a raised guillotine, a half-size copy under a spindle press, and
the same motif as square pixels beneath an adjustable loupe. Every tool spans
the track so a wagon can pass beneath it. Run with Blender in background mode.
Family helpers are read, never modified.
"""
import argparse
import hashlib
import json
import math
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
import bmesh
import bpy
from mathutils import Vector
from studio import light, material, render, scene, studio
from hero_geometry import box, cylinder, finish, flush_screw, group, profile, ring, rod, rounded_plate, screw, surfaces, verify_geometry

CAMERA = Vector((4.8, -10, 8.3))  # Offset from the scene center.
FACING = 0  # Heading of the outer straights and of each picture's horizontal axis.

# Track cross-section, measured from the floor.
BED_WIDTH, BED_TOP = 2.3, 0.26  # One broad bed carries the rails and every tool.
GAUGE = 0.38  # Rail center offset from the track center line.
RAIL_RADIUS, RAIL_Z = 0.035, BED_TOP + 0.02
WHEEL_RADIUS = 0.085
WHEEL_Z = RAIL_Z + RAIL_RADIUS + WHEEL_RADIUS
DECK_TOP = WHEEL_Z + 0.13

# Track plan: (kind, length) or (kind, radius, turn in degrees; positive turns left).
START_HEADING = FACING


def card_angle(heading):
    """Square a picture to its wagon, turned as close to level on screen as possible."""
    return round((FACING - heading) / 90) * 90
SEGMENTS = [("line", 1.9), ("arc", 2.0, -35), ("line", 1.3), ("arc", 2.0, 35), ("line", 2.2)]

# One motif on a 4:3 field (u in 0..1, v in 0..0.75): two peaks and a sun.
ASPECT = 0.75
NEAR_PEAK = [(0, 0), (0.84, 0), (0.36, 0.54), (0, 0.135)]
FAR_PEAK = [(0.27, 0), (1, 0), (1, 0.13), (0.70, 0.42)]
SUN = ((0.75, 0.585), 0.085)


def inside(polygon, u, v):
    hit = False
    for (x1, y1), (x2, y2) in zip(polygon, polygon[1:] + polygon[:1]):
        if (y1 > v) != (y2 > v) and u < x1 + (v - y1) * (x2 - x1) / (y2 - y1):
            hit = not hit
    return hit


def motif(u, v):
    (cu, cv), r = SUN
    if (u - cu) ** 2 + (v - cv) ** 2 < r * r:
        return "sun"
    return "near" if inside(NEAR_PEAK, u, v) else "far" if inside(FAR_PEAK, u, v) else "sky"


# --- Track plan -------------------------------------------------------------------

def plan(step=0.03):
    """Sample the center line as (position, heading in radians, distance)."""
    p, h, s = Vector((0, 0)), math.radians(START_HEADING), 0.0
    samples = [(p.copy(), h, s)]
    for segment in SEGMENTS:
        if segment[0] == "line":
            length = segment[1]
            count = max(1, round(length / step))
            for i in range(1, count + 1):
                samples.append((p + Vector((math.cos(h), math.sin(h))) * length * i / count, h, s + length * i / count))
        else:
            radius, turn = segment[1], math.radians(segment[2])
            length, sign = abs(turn) * radius, 1 if turn > 0 else -1
            centre = p + Vector((-math.sin(h), math.cos(h))) * radius * sign
            count = max(2, round(length / step))
            for i in range(1, count + 1):
                hh = h + turn * i / count
                samples.append((centre + Vector((math.sin(hh), -math.cos(hh))) * radius * sign, hh, s + length * i / count))
        p, h, s = samples[-1]
    # Center the plan on the origin so the shared studio lights stay balanced.
    xs, ys = [q.x for q, _, _ in samples], [q.y for q, _, _ in samples]
    middle = Vector(((min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2))
    return [(q - middle, hh, d) for q, hh, d in samples]


def station(samples, distance):
    point, heading, _ = min(samples, key=lambda item: abs(item[2] - distance))
    return point, heading


def segment_start(index):
    total = 0.0
    for segment in SEGMENTS[:index]:
        total += segment[1] if segment[0] == "line" else abs(math.radians(segment[2])) * segment[1]
    return total


def normal(h):
    return Vector((-math.sin(h), math.cos(h)))


def closed_mesh(name, vertices, faces):
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(vertices, [], faces)
    bm = bmesh.new()
    bm.from_mesh(mesh)
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    bm.to_mesh(mesh)
    bm.free()
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.scene.collection.objects.link(obj)
    return obj


def swept_section(name, samples, section, surface, bevel=0.0):
    """Sweep a closed (lateral offset, z) section along the center line."""
    count = len(section)
    vertices, faces = [], []
    for p, h, _ in samples:
        n = normal(h)
        vertices.extend((p.x + n.x * y, p.y + n.y * y, z) for y, z in section)
    for i in range(len(samples) - 1):
        faces.extend((i * count + k, i * count + (k + 1) % count, (i + 1) * count + (k + 1) % count, (i + 1) * count + k) for k in range(count))
    last = (len(samples) - 1) * count
    faces.extend([tuple(reversed(range(count))), tuple(range(last, last + count))])
    obj = finish(closed_mesh(name, vertices, faces), surface, bevel)
    for modifier in obj.modifiers:
        if modifier.type == "BEVEL":
            # Bevel only real corners, never the facets that approximate a curve.
            modifier.limit_method = "ANGLE"
            modifier.angle_limit = math.radians(45)
    for face in obj.data.polygons:
        face.use_smooth = abs(face.normal.z) < 0.5
    return obj


def track(samples):
    w = BED_WIDTH / 2
    bed = swept_section("Curved steel track bed", samples, [(-w, 0), (w, 0), (w, BED_TOP), (-w, BED_TOP)], steel, 0.02)
    circle = [(RAIL_RADIUS * math.cos(k * math.tau / 16), RAIL_RADIUS * math.sin(k * math.tau / 16)) for k in range(16)]
    for side in [-1, 1]:
        rail = swept_section(f"Polished rail {side}", samples, [(side * GAUGE + y, RAIL_Z + z) for y, z in circle], steel)
        for face in rail.data.polygons:
            face.use_smooth = True
    # A sparse scale along the front edge, as on the family's benches.
    last, tick = samples[-1][2], -1.0
    for p, h, s in samples:
        if 0.2 < s < last - 0.2 and s - tick >= 0.12:
            tick = s
            major = round(s / 0.12) % 5 == 0
            q = p - normal(h) * (w - (0.09 if major else 0.065))
            mark = box("Track graduation", (0.014, 0.12 if major else 0.07, 0.004), (q.x, q.y, BED_TOP + 0.001), dark, 0)
            mark.rotation_euler = (0, 0, h)
    for p, h, s in [samples[0], samples[-1]]:
        inward = 1 if s == 0 else -1
        along = Vector((math.cos(h), math.sin(h))) * inward
        for side in [-1, 1]:
            q = p + normal(h) * side * GAUGE + along * 0.10
            cylinder("Rail end block", 0.10, 0.12, (q.x, q.y, BED_TOP + 0.06), steel, bevel=0.015)
            a = p + normal(h) * side * (w - 0.24) + along * 0.24
            flush_screw(bed, "Track bed anchor", a.x, a.y, BED_TOP, steel, dark, radius=0.10)


def frame(name, samples, distance, shift=0.0):
    """A local frame on the track: +x runs along the track, +y to its left."""
    p, h = station(samples, distance)
    p = p + Vector((math.cos(h), math.sin(h))) * shift
    g = group(name, (p.x, p.y, 0), (0, 0, math.degrees(h)))
    return g, math.degrees(h)


def wagon(name, g, length):
    """A flat wagon on two axles with flanged wheels; returns its deck height."""
    box(f"{name} deck", (length, 0.94, 0.07), (0, 0, DECK_TOP - 0.035), steel, 0.02, g)
    for ax in [-1, 1]:
        x = ax * (length / 2 - 0.22)
        rod(f"{name} axle", (x, -GAUGE - 0.04, WHEEL_Z), (x, GAUGE + 0.04, WHEEL_Z), 0.022, steel, g)
        for side in [-1, 1]:
            cylinder(f"{name} wheel", WHEEL_RADIUS, 0.05, (x, side * GAUGE, WHEEL_Z), steel, g, bevel=0.01).rotation_euler = (math.radians(90), 0, 0)
            cylinder(f"{name} wheel flange", WHEEL_RADIUS + 0.018, 0.012, (x, side * (GAUGE - 0.046), WHEEL_Z), steel, g, bevel=0.004).rotation_euler = (math.radians(90), 0, 0)
    return DECK_TOP


# --- Samples ----------------------------------------------------------------------

def image_card(name, width, location, parent, angle):
    """A framed ivory picture plate with relief peaks and an inlaid copper sun."""
    g = group(name, location, (0, 0, angle), parent)
    box(f"{name} steel carrier", (width + 0.10, width * ASPECT + 0.10, 0.04), (0, 0, 0.02), steel, 0.012, g)
    box(f"{name} ivory sheet", (width, width * ASPECT, 0.018), (0, 0, 0.049), ivory, 0.004, g)
    top, relief = 0.058, max(0.008, width * 0.012)

    def local(points):
        return [((u - 0.5) * width, (v - ASPECT / 2) * width) for u, v in points]

    profile(f"{name} far peak", local(FAR_PEAK), relief, (0, 0, top + relief / 2), pale, relief * 0.3, g)
    profile(f"{name} near peak", local(NEAR_PEAK), relief * 1.6, (0, 0, top + relief * 0.8), slate, relief * 0.3, g)
    (cu, cv), r = SUN
    cylinder(f"{name} copper sun", r * width, relief * 1.4, ((cu - 0.5) * width, (cv - ASPECT / 2) * width, top + relief * 0.7), copper, g, bevel=relief * 0.3)


def pixel_card(name, cols, cell, location, parent, angle):
    """The same motif as separate, clearly bounded square pixels in a graphite tray."""
    rows = round(cols * ASPECT)
    g = group(name, location, (0, 0, angle), parent)
    width, depth = cols * cell, rows * cell
    box(f"{name} graphite tray", (width + cell * 0.7, depth + cell * 0.7, 0.05), (0, 0, 0.025), graphite, 0.015, g)
    palette = {"sky": ivory, "far": pale, "near": slate, "sun": copper}
    for c in range(cols):
        for r in range(rows):
            surface = palette[motif((c + 0.5) / cols, (r + 0.5) / rows * ASPECT)]
            box(f"{name} pixel {c}-{r}", (cell * 0.84, cell * 0.84, 0.035), ((c + 0.5) * cell - width / 2, (r + 0.5) * cell - depth / 2, 0.0675), surface, cell * 0.06, g)


def helix(name, center, z0, z1, parent, radial=0.062, wire=0.016, pitch=0.075):
    """A continuous helical thread around a vertical spindle."""
    sides, per_turn = 8, 28
    steps = int((z1 - z0) / pitch * per_turn)
    vertices, faces = [], []
    for j in range(steps + 1):
        a = j * math.tau / per_turn
        c = Vector((center[0] + radial * math.cos(a), center[1] + radial * math.sin(a), z0 + j * pitch / per_turn))
        n = Vector((math.cos(a), math.sin(a), 0))
        side = Vector((-radial * math.sin(a), radial * math.cos(a), pitch / math.tau)).normalized().cross(n)
        vertices.extend(tuple(c + wire * (n * math.cos(k * math.tau / sides) + side * math.sin(k * math.tau / sides))) for k in range(sides))
    for j in range(steps):
        faces.extend((j * sides + k, j * sides + (k + 1) % sides, (j + 1) * sides + (k + 1) % sides, (j + 1) * sides + k) for k in range(sides))
    faces.extend([tuple(reversed(range(sides))), tuple(range(steps * sides, (steps + 1) * sides))])
    obj = finish(closed_mesh(name, vertices, faces), steel, 0, parent)
    for face in obj.data.polygons:
        face.use_smooth = True


def lens(name, radius, thickness, parent):
    """A biconvex lens with paraboloid faces and a short cylindrical edge."""
    rings, segments, edge = 28, 96, 0.006
    sag = thickness / 2 - edge
    vertices, faces = [(0, 0, edge + sag), (0, 0, -edge - sag)], []

    def at(side, step, k):
        return 2 + side * rings * segments + (step - 1) * segments + k % segments

    for side, sign in [(0, 1), (1, -1)]:
        for step in range(1, rings + 1):
            rr = radius * step / rings
            z = sign * (edge + sag * (1 - (rr / radius) ** 2))
            vertices.extend((rr * math.cos(k * math.tau / segments), rr * math.sin(k * math.tau / segments), z) for k in range(segments))
        faces.extend((side, at(side, 1, k), at(side, 1, k + 1)) for k in range(segments))
        for step in range(1, rings):
            faces.extend((at(side, step, k), at(side, step + 1, k), at(side, step + 1, k + 1), at(side, step, k + 1)) for k in range(segments))
    faces.extend((at(0, rings, k), at(1, rings, k), at(1, rings, k + 1), at(0, rings, k + 1)) for k in range(segments))
    obj = finish(closed_mesh(name, vertices, faces), glass, 0, parent)
    for face in obj.data.polygons:
        face.use_smooth = abs(face.normal.z) > 0.2


# --- Stations (local frame: the track runs along +x) -------------------------------

def guillotine(g, x, span=0.86, height=1.45, blade_z=1.0):
    """A gantry straddling the track with a raised, guided blade and an oxide pull handle."""
    base, top = BED_TOP, BED_TOP + 1.45
    for sy in [-1, 1]:
        box("Guillotine upright", (0.22, 0.18, height), (x, sy * span, base + height / 2), steel, 0.03, g)
        rod("Guillotine lift rod", (x, sy * 0.42, blade_z + 0.28), (x, sy * 0.42, top + 0.36), 0.03, steel, g)
        cylinder("Guillotine handle end", 0.085, 0.05, (x, sy * 0.54, top + 0.38), steel, g).rotation_euler = (math.radians(90), 0, 0)
    box("Guillotine top bridge", (0.30, span * 2 + 0.24, 0.20), (x, 0, top + 0.08), steel, 0.035, g)
    carrier = group("Guillotine blade carrier", (x, 0, blade_z), (90, 0, 90), g)
    profile("Guillotine angled blade", [(-span + 0.10, 0.20), (span - 0.10, 0.20), (span - 0.10, -0.06), (-span + 0.10, -0.18)], 0.04, (0, 0, 0), steel, 0.006, carrier)
    box("Guillotine blade back", (0.10, span * 2 - 0.12, 0.10), (x, 0, blade_z + 0.24), steel, 0.02, g)
    rod("Guillotine oxide pull handle", (x, -0.52, top + 0.38), (x, 0.52, top + 0.38), 0.075, oxide, g)


def handwheel(g, z, radius=0.46):
    ring("Press handwheel rim", radius, radius * 0.84, radius * 0.17, (0, 0, z), steel, g)
    cylinder("Press handwheel hub", radius * 0.24, radius * 0.30, (0, 0, z), steel, g, bevel=0.012)
    for j in range(3):
        a = j * math.tau / 3 + math.radians(30)
        rod("Press handwheel spoke", (radius * 0.22 * math.cos(a), radius * 0.22 * math.sin(a), z), (radius * 0.86 * math.cos(a), radius * 0.86 * math.sin(a), z), radius * 0.055, steel, g)
    a = math.radians(160)
    cylinder("Press crank grip", radius * 0.075, radius * 0.55, (radius * 0.92 * math.cos(a), radius * 0.92 * math.sin(a), z + radius * 0.32), graphite, g, bevel=0.01)
    screw("Press handwheel cap", (0, 0, z + radius * 0.15), steel, dark, radius=radius * 0.11, parent=g)


def screw_press(g, card, span=0.84, height=1.85, platen_z=1.08):
    """Guide columns on both sides of the track; the oxide crosshead spans it."""
    base = BED_TOP
    top = base + height
    for sy in [-1, 1]:
        y = sy * span
        cylinder("Press column foot", 0.17, 0.10, (0, y, base + 0.05), steel, g, bevel=0.015)
        cylinder("Press guide column", 0.075, height, (0, y, base + height / 2), steel, g, bevel=0.01)
        screw("Press column nut", (0, y, top + 0.05), steel, dark, radius=0.07, parent=g)
        cylinder("Press platen bushing", 0.115, 0.20, (0, y, platen_z), steel, g, bevel=0.015)
    box("Press oxide crosshead", (0.40, span * 2 + 0.34, 0.30), (0, 0, top - 0.10), oxide, 0.04, g)
    cylinder("Press spindle nut boss", 0.16, 0.12, (0, 0, top + 0.11), steel, g, bevel=0.015)
    cylinder("Press spindle core", 0.058, top + 0.55 - platen_z, (0, 0, (top + 0.55 + platen_z) / 2), steel, g, bevel=0.006)
    helix("Press spindle thread", (0, 0), platen_z + 0.12, top - 0.28, g)
    box("Press platen", (0.46, span * 2 + 0.26, 0.13), (0, 0, platen_z), steel, 0.03, g)
    box("Press platen face", (card + 0.08, card + 0.08, 0.04), (0, 0, platen_z - 0.08), graphite, 0.01, g)
    cylinder("Press spindle swivel", 0.10, 0.09, (0, 0, platen_z + 0.10), steel, g, bevel=0.012)
    handwheel(g, top + 0.58)


def loupe(g, heading, sample_top, post=(0.25, 0.88), aim=(0.20, 0.10), ring_r=0.40, lens_z=1.05, tilt=0.8):
    """An adjustable loupe beside the track; the lens sits on the camera's line of sight."""
    a = math.radians(-heading)
    world_drift = CAMERA.xy / CAMERA.z
    drift = Vector((world_drift.x * math.cos(a) - world_drift.y * math.sin(a), world_drift.x * math.sin(a) + world_drift.y * math.cos(a)))
    centre = Vector(aim) + drift * (lens_z - sample_top)
    lens_c = Vector((centre.x, centre.y, lens_z))
    px, py = post
    base = BED_TOP
    cylinder("Loupe post foot", 0.20, 0.08, (px, py, base + 0.04), steel, g, bevel=0.015)
    collar_z = lens_z + 0.42
    post_h = collar_z + 0.14 - base
    cylinder("Loupe post", 0.065, post_h, (px, py, base + 0.08 + post_h / 2), steel, g, bevel=0.01)
    screw("Loupe post cap", (px, py, base + 0.08 + post_h), steel, dark, radius=0.06, parent=g)
    cylinder("Loupe clamp collar", 0.11, 0.16, (px, py, collar_z), steel, g, bevel=0.015)
    toward = (Vector((px, py)) - centre).normalized()
    side = Vector((-toward.y, toward.x))
    rod("Loupe collar thumb screw", (px, py, collar_z), (px + side.x * 0.22, py + side.y * 0.22, collar_z), 0.024, steel, g)
    # The head tilts toward the viewer so the pixels are seen through the glass.
    facing = Vector((0, 0, 1)).lerp(Vector((drift.x, drift.y, 1)).normalized(), tilt).normalized()
    pivot = group("Loupe tilted head", tuple(lens_c), (0, 0, 0), g)
    pivot.rotation_mode = "QUATERNION"
    pivot.rotation_quaternion = facing.to_track_quat("Z", "Y")
    flat = Vector((toward.x, toward.y, 0))
    radial = (flat - facing * flat.dot(facing)).normalized()
    ring("Loupe oxide rim", ring_r, ring_r * 0.80, 0.12, (0, 0, 0), oxide, pivot)
    lens("Loupe crown glass lens", ring_r * 0.81, 0.09, pivot)
    cylinder("Loupe ring lug", 0.065, 0.13, tuple(pivot.rotation_quaternion.inverted() @ radial * (ring_r + 0.06)), steel, pivot, bevel=0.012)
    lug = lens_c + radial * (ring_r + 0.06)
    elbow = Vector(((px + lug.x) / 2, (py + lug.y) / 2, max(collar_z, lug.z) + 0.12))
    rod("Loupe upper arm", (px, py, collar_z), tuple(elbow), 0.045, steel, g)
    joint = cylinder("Loupe elbow joint", 0.09, 0.14, tuple(elbow), steel, g, bevel=0.015)
    joint.rotation_euler = (math.radians(90), 0, math.atan2(side.y, side.x) + math.radians(90))
    rod("Loupe fore arm", tuple(elbow), (lug.x, lug.y, lug.z + 0.06), 0.040, steel, g)


bpy.ops.object.select_all(action="SELECT")
bpy.ops.object.delete(use_global=False)
result = scene("Ferrisma image railway", draft=opts.draft)
result.render.resolution_x = 1200 if opts.draft else 2400
result.render.resolution_y = 600 if opts.draft else 1200
result.cycles.samples = 32 if opts.draft else 192
result.cycles.transmission_bounces = 12
steel, oxide, graphite, dark = surfaces()
ivory = material("Ivory sample enamel", (0.60, 0.56, 0.46), 0.42, metallic=0.08)
pale = material("Pale gray sample enamel", (0.25, 0.25, 0.24), 0.40, metallic=0.12)
slate = material("Slate gray sample enamel", (0.085, 0.085, 0.085), 0.40, metallic=0.18)
copper = material("Muted copper sun", (0.36, 0.15, 0.07), 0.34, oxide=True)
glass = bpy.data.materials.new("Loupe crown glass")
shader = glass.node_tree.nodes.get("Principled BSDF")
shader.inputs["Base Color"].default_value = (0.97, 0.98, 0.97, 1)
shader.inputs["Transmission Weight"].default_value = 1
shader.inputs["Roughness"].default_value = 0
shader.inputs["IOR"].default_value = 1.52

samples = plan()
track(samples)
# Crop: a gantry blade spans the track above the source picture's left edge.
g, heading = frame("Crop station", samples, segment_start(0) + 1.0)
deck = wagon("Crop wagon", g, 1.15)
image_card("Source picture", 1.0, (0, 0, deck), g, card_angle(heading))
guillotine(g, -0.34)
offcut = box("Cropped offcut strip", (0.75, 0.12, 0.012), (0.12, -0.80, BED_TOP + 0.006), ivory, 0.003, g)
offcut.rotation_euler = (0, 0, math.radians(5))

# Resize: on the falling straight, guide columns stand on both sides of the track.
g, heading = frame("Resize station", samples, segment_start(2) + 0.65)
deck = wagon("Resize wagon", g, 0.80)
image_card("Reduced picture", 0.5, (0, 0, deck), g, card_angle(heading))
screw_press(g, 0.5)

# Inspect: the loupe stands behind the track and leans over the pixel wagon.
g, heading = frame("Inspect station", samples, segment_start(4) + 1.05)
deck = wagon("Inspect wagon", g, 1.25)
pixel_card("Pixel sample", 10, 0.10, (0, 0, deck), g, card_angle(heading))
loupe(g, heading, deck + 0.085)

checks = verify_geometry()
target = Vector((0, 0, 0.8))
studio(tuple(target), tuple(target + CAMERA))
light("Long front reflection", (-1, -6, 6), 220, 6, 1.4, (1, 0.97, 0.93), (0, 0, 0.7))
light("Low upright-face reflection card", (-3, -5, -1.5), 140, 6, 3, (1, 0.97, 0.93), (0, 0, 1))
render(result, opts.output.resolve())
if opts.blend:
    bpy.ops.wm.save_as_mainfile(filepath=str(opts.blend.resolve()), compress=True)
if not opts.draft:
    source = Path(__file__).resolve()
    sidecar = {"product": "Ferrisma", "role": "Transparent native Blender homepage instrument", "meaning": "An image railway: wagons on a gently curved steel track carry a source picture under a guillotine, a half-size copy under a spindle press, and the motif as square pixels under a loupe. Each tool spans the track.", "renderer": f"Blender {bpy.app.version_string}, Cycles", "dimensions": "2400x1200", "samples": result.cycles.samples, "colorManagement": "AgX Medium High Contrast, sRGB alpha", "familySourceCommit": COMMIT, "familySourceHashes": EXPECTED, "constructionSha256": hashlib.sha256(source.read_bytes()).hexdigest(), "masterSha256": hashlib.sha256(opts.output.read_bytes()).hexdigest(), "geometryVerification": checks, "verificationScope": "Each evaluated mesh is closed and has positive volume; Boolean cuts remove material. Station clearances follow from the construction dimensions; assembly contacts are not generally collision-checked."}
    opts.output.with_suffix(".png.json").write_text(json.dumps(sidecar, indent=2) + "\n")
print(f"Rendered {len(checks)} verified solid bodies to {opts.output}")
