"""Fit the engine's JPEG XL peak-memory model and compare it with the
measurements (2026-11-16).

python3 -I fit.py <results>... [--old-lossless F] [--old-deep F] [--table]

<results>: measure_serial.sh lines (input mode run max_rss peak_footprint
  heap_peak bytes sha; lines starting with # are comments). The model's
  basis is the maximum resident set size, as for the 2026-11-15 evidence;
  the peak footprint of a rejected run counts as a sample as well.
--old-lossless: the 2026-11-15 lossless-memory results.txt (probe holding
  the input once, 3 or 4 B/px): its `port-api-default` rows are compared at
  the engine's 4 B/px frame (+1 B/px for RGB).
--old-deep: the 2026-11-15 deep-memory results.txt (encode_pnm, which held
  the file and a converted copy): its `port` rows are compared at the
  engine's frame (minus the extra copy: 2, 4, 4 or 8 B/px for RGB8, RGBA8,
  RGB16, RGBA16).

Model, per mode (lossless, lossy), alpha and depth class (8 bits, above):
  one-shot (at most 8 groups of 256x256):  C + O * px
  streaming (more than 8 groups):          C + A * px + B * min(px, D)
                                             + R * min(max(px - D, 0), D)
with D = 2048 * 2048 (one DC group). Every case must satisfy
model >= (1 + MARGIN) * the largest peak on record. Fitting order:
  A: the steepest least-squares slope of one content (of its peaks, or of
     its live-heap peaks), through the streaming cases of at least two DC
     groups: the buffers that scale with the whole frame;
  B: the least value for the streaming cases of at most one DC group;
  R: the least value for the cases above one DC group (the second DC
     group's working set that the allocator keeps);
  O: the least value for the one-shot cases.
C is fixed: 12 MiB lossless, 20 MiB lossy. Finally each term of an alpha or
a deep model is raised to at least that term of the opaque or 8-bit model.
"""
import math
import re
import sys

MIB = 1 << 20
GIB = 1 << 30
D = 2048 * 2048
MARGIN = 0.10
FOOT = {}


def groups(w, h):
    return -(-w // 256) * -(-h // 256)


def parse(argv):
    files, old_ll, old_deep, table = [], None, None, False
    global RUST
    RUST = "--rust" in argv
    argv = [a for a in argv if a != "--rust"]
    i = 0
    while i < len(argv):
        if argv[i] == "--old-lossless":
            old_ll = argv[i + 1]; i += 2
        elif argv[i] == "--old-deep":
            old_deep = argv[i + 1]; i += 2
        elif argv[i] == "--table":
            table = True; i += 1
        else:
            files.append(argv[i]); i += 1
    return files, old_ll, old_deep, table


def collect(files, old_ll, old_deep):
    """{(mode, alpha, deep): {(w, h, content): [rss...]}} and heap peaks."""
    rss, heap = {}, {}
    def add(mode, a, deep, w, h, content, v, hp=None):
        key = (mode, a, deep)
        rss.setdefault(key, {}).setdefault((w, h, content), []).append(v)
        if hp:
            heap.setdefault(key, {})[(w, h, content)] = hp
    for f in files:
        for line in open(f):
            p = line.split()
            # A rejected run's footprint (resident plus compressed and
            # swapped pages) is a lower bound of its undisturbed peak: it
            # enters as a sample too, which can only raise the model.
            r = re.match(r"# REJECTED (\S+) (\S+) rss=(\d+) footprint=(\d+)", line)
            if r:
                p = [r.group(1), r.group(2), "0", r.group(4), r.group(4), "-", "0", "-"]
            if len(p) != 8 or not p[3].isdigit():
                continue
            m = re.match(r"(p|w8|w16|n8|n16)-(\d+)x(\d+)(a?)$", p[0])
            mode = "lossless" if p[1] == "lossless" else "lossy"
            # measure_serial.sh: rss, footprint, heap. The model's basis is
            # the maximum resident set size, as in the 2026-11-15 evidence;
            # the footprint is reported beside it.
            peak, hp = int(p[3]), p[5]
            FOOT[(p[0], mode)] = max(FOOT.get((p[0], mode), 0), int(p[4]))
            add(mode, m.group(4) == "a", m.group(1) in ("w16", "n16"), int(m.group(2)), int(m.group(3)),
                m.group(1) + ("" if mode == "lossless" else "@" + p[1]), peak,
                int(hp) if hp.isdigit() else None)
    if old_ll:
        sizes = {"photo12": (4032, 3024), "photo24": (6000, 4000)}
        for line in open(old_ll):
            m = re.match(r"(\w+?)(a?)-port-api-default-\d status=0 max_rss=(\d+)", line)
            if not m:
                continue
            name, a = m.group(1), m.group(2) == "a"
            w, h = sizes.get(name) or tuple(map(int, name[1:].split("x")))
            v = int(m.group(3)) + (0 if a else w * h)
            add("lossless", a, False, w, h, "2026-11-15 " + name, v)
    if old_deep:
        for line in open(old_deep):
            p = line.split()
            if len(p) < 3 or p[1] != "port":
                continue
            m = re.match(r"c(\d+)x(\d+)-(8|16)(a?)$", p[0])
            w, h, deep, a = int(m.group(1)), int(m.group(2)), m.group(3) == "16", m.group(4) == "a"
            extra = {(False, False): 2, (False, True): 4, (True, False): 4, (True, True): 8}[(deep, a)]
            add("lossless", a, deep, w, h, "2026-11-15 weld" + m.group(3), int(p[2]) - extra * w * h)
    return rss, heap


def slope(points):
    if len(points) < 2:
        return 0.0
    mx = sum(x for x, _ in points) / len(points)
    my = sum(y for _, y in points) / len(points)
    den = sum((x - mx) ** 2 for x, _ in points)
    return sum((x - mx) * (y - my) for x, y in points) / den if den else 0.0


def fit(key, cases, heaps):
    mode = key[0]
    C = 12 * MIB if mode == "lossless" else 20 * MIB
    need = {c: (1 + MARGIN) * max(v) for c, v in cases.items()}
    streaming = {c: n for c, n in need.items() if groups(c[0], c[1]) > 8}
    one_shot = {c: n for c, n in need.items() if groups(c[0], c[1]) <= 8}
    by_content = {}
    for (w, h, content), v in cases.items():
        if groups(w, h) > 8 and w * h >= 2 * D:
            by_content.setdefault(content, []).append((w * h, max(v)))
    heap_by_content = {}
    for (w, h, content), hp in heaps.get(key, {}).items():
        if groups(w, h) > 8 and w * h >= 2 * D:
            heap_by_content.setdefault(content, []).append((w * h, hp))
    A = math.ceil(max(slope(v) for v in list(by_content.values()) + list(heap_by_content.values())))
    B = max((math.ceil((n - C) / (w * h)) - A for (w, h, _), n in streaming.items() if w * h <= D), default=0)
    R = 0
    for (w, h, _), n in streaming.items():
        px = w * h
        if px > D:
            e = min(px - D, D)
            base = C + A * px + B * D
            R = max(R, math.ceil((n - base) / e))
    O = max((math.ceil((n - C) / (w * h)) for (w, h, _), n in one_shot.items()), default=0)
    return dict(C=C, O=O, A=A, B=B, R=R)


def total(m, w, h):
    px = w * h
    if groups(w, h) <= 8:
        return m["C"] + m["O"] * px
    return m["C"] + m["A"] * px + m["B"] * min(px, D) + m["R"] * min(max(px - D, 0), D)


def before(key, w, h):
    mode, a, deep = key
    px, i, s = w * h, int(a), 2 if deep else 1
    if mode == "lossless":
        one, ch, dc = ([88, 107], [25, 32], [71, 99]) if deep else ([76, 97], [16, 20], [53, 70])
        streams = groups(w, h) > 8
        peak = (px * ch[i] + min(px, D) * dc[i] if streams else px * one[i]) + 12 * MIB
        res = max(peak - px * (3 + i) * s, 0)
        if not streams:
            pad = -(-w // 8) * 8 * (-(-h // 8) * 8)
            res = max(res, pad * 12 + px * i * 4 + px * (3 + i) * 4)
        return px * 4 * s + res
    b8 = lambda n: -(-n // 8) * 8
    planes = lambda x, y: b8(x) * b8(y) * 28
    extra = 12 if a else 0
    if groups(w, h) > 8:
        border = 256 + 16
        codec = planes(border, border) + 3 * 256 * 256 * 4 + (-(-w // 8)) * (-(-h // 8)) * 28 + border * border * extra
    else:
        codec = planes(w, h) + groups(w, h) * 3 * 256 * 256 * 4 + px * extra
    return px * 4 * s + px * (3 + i) * s + codec


def limit(m, budget=GIB):
    """Largest pixel count whose model fits `budget` (streaming region)."""
    lo, hi = 1, 1 << 28
    while lo < hi:
        mid = (lo + hi + 1) // 2
        w = 4096
        h = -(-mid // w)
        ok = m["C"] + m["A"] * mid + m["B"] * min(mid, D) + m["R"] * min(max(mid - D, 0), D) <= budget
        lo, hi = (mid, hi) if ok else (lo, mid - 1)
    return lo


def rust_rows(rss):
    """The recorded peaks as rows of `jxl_output_tests`: the largest per
    content, size, mode, alpha and depth (lossy over every distance)."""
    merged = {}
    for (mode, a, deep), cases in rss.items():
        for (w, h, content), v in cases.items():
            key = (mode != "lossless", a, deep, w * h, w, h, content.split("@")[0])
            merged[key] = max(merged.get(key, 0), max(v))
    for (lossy, a, deep, _, w, h, label), peak in sorted(merged.items()):
        depth = 16 if deep else 8
        print(f'        ("{label}", {w}, {h}, {str(not lossy).lower()}, {str(a).lower()}, {depth}, {peak:_}),')


def main():
    files, old_ll, old_deep, table = parse(sys.argv[1:])
    rss, heap = collect(files, old_ll, old_deep)
    if RUST:
        rust_rows(rss)
        return
    models = {}
    # 8 bits before deep, opaque before alpha: each term of a deeper or an
    # alpha model is at least that term of the shallower or opaque model of
    # the same mode (wider samples and an extra channel add buffers, never
    # remove them).
    for key in sorted(rss, key=lambda k: (k[0], k[2], k[1])):
        m = fit(key, rss[key], heap)
        for lower in ((key[0], False, key[2]), (key[0], key[1], False)):
            if lower != key and lower in models:
                for term in ("O", "A", "B", "R"):
                    m[term] = max(m[term], models[lower][term])
        models[key] = m
    for key in sorted(rss):
        m = models[key]
        name = f"{key[0]} {'RGBA' if key[1] else 'RGB'} {'above 8 bits' if key[2] else '8 bits'}"
        print(f"## {name}: C={m['C'] // MIB} MiB O={m['O']} A={m['A']} B={m['B']} R={m['R']}; "
              f"1 GiB admits {limit(m):,} px")
        if not table:
            continue
        print("| Case | Pixels | Runs | Measured MB | B/px | Before B/px | Margin | After B/px | Margin |")
        print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
        for (w, h, c), v in sorted(rss[key].items(), key=lambda kv: (kv[0][0] * kv[0][1], kv[0][2])):
            px, r = w * h, max(v)
            b, t = before(key, w, h), total(m, w, h)
            flag = " **under**" if b < r else ""
            print(f"| {c} {w}x{h} | {px:,} | {len(v)} | {r / 1e6:,.1f} | {r / px:.1f} | {b / px:.1f} | "
                  f"{(b / r - 1) * 100:+.1f} %{flag} | {t / px:.1f} | {(t / r - 1) * 100:+.1f} % |")
        print()


main()
