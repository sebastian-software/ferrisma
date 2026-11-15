#!/usr/bin/env python3
"""Deterministic corpus for the JPEG XL patches-cost measurement (issue #236).

Usage: make_corpus.py OUT_DIR

Writes binary PPM (P6) files into OUT_DIR: eighteen generated text-like images
(desktop UI, web page, terminal/code, document page, slide, chart; each at
about 1 MP and about 4 MP, plus one small image of at most 8 groups of
256x256 pixels per kind) and four photographs taken from fixtures that
already live in the repository. Writes OUT_DIR/corpus.json with the kind,
description and SHA-256 of every file.

Requires Python 3 with Pillow (with FreeType) and NumPy:
  python3 -m venv VENV && VENV/bin/pip install pillow numpy
Text is set in the font bundled with Pillow (ImageFont.load_default(size));
nothing is read from the system's fonts, so the images depend only on the
Pillow version (recorded in corpus.json). The generated images carry no
license question: they contain only drawn shapes and pseudo-random words.
Photographs are decoded from the repository's fixtures; the three JPEG-origin
photographs depend on Pillow's libjpeg build for bit-exact pixels.
"""
import hashlib
import json
import random
import sys
from pathlib import Path

import numpy as np
import PIL
from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parents[3]

WORDS = (
    "alpha beta gamma delta render pixel buffer stream encode decode filter kernel "
    "window panel select option layout margin border column table value result "
    "report config update status device memory thread handle record export import "
    "network gateway session service module package version channel profile "
    "balance quality section summary overview details settings storage archive "
    "request response timeout payload message content preview download upload"
).split()

_font_cache = {}


def font(size):
    size = max(6, int(round(size)))
    if size not in _font_cache:
        _font_cache[size] = ImageFont.load_default(size=size)
    return _font_cache[size]


class Canvas:
    def __init__(self, w, h, bg, aa, s):
        self.img = Image.new("RGB", (w, h), bg)
        self.d = ImageDraw.Draw(self.img)
        self.d.fontmode = "L" if aa else "1"
        self.aa = aa
        self.s = s
        self.w, self.h = w, h

    def text(self, xy, s, size, fill, bold=False):
        x, y = xy[0] * self.s, xy[1] * self.s
        self.d.text((x, y), s, font=font(size * self.s), fill=fill,
                    stroke_width=1 if bold and self.s > 1 else 0,
                    stroke_fill=fill)

    def width(self, s, size):
        return self.d.textlength(s, font=font(size * self.s))

    def mono(self, xy, s, size, fill, advance):
        # fixed-advance cells, as a terminal grid draws them
        x, y = xy
        for i, ch in enumerate(s):
            if ch != " ":
                self.d.text((x + i * advance, y), ch, font=font(size * self.s), fill=fill)

    def rect(self, box, fill=None, outline=None, width=1):
        s = self.s
        b = [v * s for v in box]
        self.d.rectangle(b, fill=fill, outline=outline, width=max(1, width * s if width > 1 else 1))

    def line(self, pts, fill, width=1):
        self.d.line([v * self.s for v in pts], fill=fill, width=max(1, int(width * self.s)))

    def ellipse(self, box, fill=None, outline=None):
        self.d.ellipse([v * self.s for v in box], fill=fill, outline=outline)


def words(rng, n):
    return " ".join(rng.choice(WORDS) for _ in range(n))


def sentence(rng, n):
    t = words(rng, n)
    return t[0].upper() + t[1:] + "."


def theme(dark):
    if dark:
        return dict(bg=(30, 31, 34), panel=(43, 45, 49), bar=(24, 25, 28), ink=(219, 222, 225),
                    dim=(148, 155, 164), accent=(88, 101, 242), line=(63, 65, 70), ok=(87, 200, 120))
    return dict(bg=(255, 255, 255), panel=(243, 244, 246), bar=(226, 228, 232), ink=(24, 26, 30),
                dim=(100, 108, 118), accent=(37, 99, 235), line=(209, 213, 219), ok=(22, 130, 60))


def desktop_ui(w, h, s, dark, aa, seed):
    rng = random.Random(seed)
    t = theme(dark)
    c = Canvas(w, h, t["bg"], aa, s)
    W, H = w // s, h // s
    c.rect((0, 0, W, 28), fill=t["bar"])
    for i, label in enumerate(["File", "Edit", "View", "Go", "Window", "Help"]):
        c.text((12 + i * 56, 7), label, 13, t["ink"])
    c.text((W - 70, 7), "10:42", 13, t["ink"])
    c.rect((0, 28, 230, H), fill=t["panel"])
    for i in range(18):
        y = 44 + i * 30
        if i == 3:
            c.rect((8, y - 5, 222, y + 21), fill=t["accent"])
        c.rect((16, y, 32, y + 16), outline=t["dim"])
        c.text((42, y), words(rng, 2), 14, t["ink"] if i != 3 else (255, 255, 255))
    c.text((250, 44), "Project settings", 22, t["ink"], bold=True)
    for i in range(14):
        y = 92 + i * 34
        c.text((250, y), words(rng, 2).title(), 14, t["ink"])
        c.rect((420, y - 4, 640, y + 20), outline=t["line"])
        c.text((428, y), words(rng, 3), 13, t["dim"])
        c.rect((660, y - 2, 680, y + 18), outline=t["dim"])
        if i % 3 == 0:
            c.line((664, y + 8, 669, y + 14, 678, y + 2), t["ok"], 2)
    # table
    x0, y0 = 720, 92
    for col, label in enumerate(["Name", "Size", "Modified", "Kind"]):
        c.text((x0 + col * 150, y0), label, 13, t["dim"], bold=True)
    c.line((x0, y0 + 22, x0 + 600, y0 + 22), t["line"])
    for r in range(min(18, (H - y0 - 40) // 26 - 1)):
        y = y0 + 32 + r * 26
        c.text((x0, y), rng.choice(WORDS) + "." + rng.choice(["rs", "toml", "md", "json"]), 13, t["ink"])
        c.text((x0 + 150, y), f"{rng.randint(1, 999)} KB", 13, t["ink"])
        c.text((x0 + 300, y), f"2026-11-{rng.randint(1, 15):02d}", 13, t["ink"])
        c.text((x0 + 450, y), rng.choice(["Source", "Document", "Data"]), 13, t["ink"])
    for i, label in enumerate(["Cancel", "Apply", "OK"]):
        bx = W - 260 + i * 84
        c.rect((bx, H - 52, bx + 72, H - 24), fill=t["accent"] if i == 2 else t["panel"], outline=t["line"])
        c.text((bx + 14, H - 45), label, 14, (255, 255, 255) if i == 2 else t["ink"])
    return c.img


def web_page(w, h, s, dark, aa, seed):
    rng = random.Random(seed)
    t = theme(dark)
    c = Canvas(w, h, t["bg"], aa, s)
    W, H = w // s, h // s
    c.rect((0, 0, W, 56), fill=t["bar"])
    c.text((24, 16), "Example Journal", 22, t["ink"], bold=True)
    for i, label in enumerate(["Home", "Articles", "Archive", "About", "Subscribe"]):
        c.text((W - 460 + i * 90, 20), label, 14, t["accent"] if i == 1 else t["ink"])
    col_w = min(760, W - 360)
    x = (W - col_w - 300) // 2 + 20
    c.text((x, 90), sentence(rng, 5)[:-1].title(), 34, t["ink"], bold=True)
    c.text((x, 140), "By " + rng.choice(WORDS).title() + " on 2026-11-1" + str(rng.randint(0, 5)), 13, t["dim"])
    y = 180
    while y < H - 60:
        line = " ".join(sentence(rng, rng.randint(8, 14)) for _ in range(rng.randint(3, 6)))
        # wrap
        cur = ""
        for word in line.split():
            if c.width(cur + " " + word, 16) / s > col_w:
                c.text((x, y), cur, 16, t["ink"])
                y += 26
                cur = word
            else:
                cur = (cur + " " + word).strip()
            if y > H - 60:
                break
        y += 18
        if rng.random() < 0.3 and y < H - 120:
            c.text((x, y), words(rng, 3).title(), 22, t["ink"], bold=True)
            y += 40
    sx = x + col_w + 40
    for i in range(8):
        y = 100 + i * 92
        c.rect((sx, y, sx + 260, y + 76), fill=t["panel"], outline=t["line"])
        c.text((sx + 12, y + 10), words(rng, 3).title(), 14, t["ink"], bold=True)
        c.text((sx + 12, y + 36), words(rng, 4), 13, t["dim"])
    return c.img


def terminal(w, h, s, dark, aa, seed):
    rng = random.Random(seed)
    bg = (24, 24, 27) if dark else (253, 246, 227)
    ink = (212, 212, 216) if dark else (88, 110, 117)
    green = (95, 200, 120) if dark else (66, 130, 0)
    blue = (110, 160, 255) if dark else (38, 139, 210)
    red = (240, 110, 110) if dark else (200, 40, 40)
    c = Canvas(w, h, bg, aa, s)
    size, adv = 14, 8.0 * s
    lh = 20 * s
    cols = int((w - 24 * s) // adv)
    rows = int((h - 24 * s) // lh)
    c.rect((0, 0, w // s, 24), fill=(50, 50, 56) if dark else (220, 214, 196))
    for i, col in enumerate([(255, 95, 86), (255, 189, 46), (39, 201, 63)]):
        c.ellipse((10 + i * 20, 6, 22 + i * 20, 18), fill=col)
    snippets = [
        ("$ ", "cargo test --workspace", None),
        ("   Compiling ", "ferrisma-core v0.9.0 (/work/ferrisma)", green),
        ("warning: ", "unused variable: `buffer`", (230, 190, 60)),
        ("error[E0308]: ", "mismatched types", red),
        ("  --> ", "src/pipeline/encode.rs:" , blue),
    ]
    templates = [
        "fn {a}(&self, {b}: &{C}, opts: &Options) -> Result<{C}, Error> {{",
        "    let mut {a} = Vec::with_capacity({b}.len() / {n});",
        "    for ({a}, {b}) in self.{c}.iter().zip(other.{d}.iter_mut()) {{",
        "        {b}.push_{a}({c}, opts.{d})?;",
        "    }}",
        "    if {a}.{b}() > {n} {{ return Err(Error::{C}); }}",
        "    self.{a} = self.{b}.wrapping_add({n}) & 0x{n:x};",
        "    println!(\"{a} = {{}}, {b} = {{:?}}\", {c}, {d});",
        "    Ok({a})",
        "}}",
        "impl {C} for {D} {{ fn {a}(&self) -> usize {{ {n} }} }}",
    ]
    def code_line():
        t = rng.choice(templates)
        return t.format(a=rng.choice(WORDS), b=rng.choice(WORDS), c=rng.choice(WORDS),
                        d=rng.choice(WORDS), C=rng.choice(WORDS).title(),
                        D=rng.choice(WORDS).title(), n=rng.randint(2, 4096))
    y = 34 * s
    r = 0
    while r < rows - 1:
        kind = rng.random()
        if kind < 0.45:
            for _ in range(rng.randint(4, 9)):
                if r >= rows - 1:
                    break
                ln = code_line()
                c.mono((12 * s, y), ln[:cols], size, ink if "fn" not in ln else blue, adv)
                y += lh
                r += 1
        else:
            pre, body, col = rng.choice(snippets)
            line = pre + body + (str(rng.randint(10, 400)) + ":" + str(rng.randint(1, 80)) if body.endswith(":") else "")
            c.mono((12 * s, y), line[:cols], size, col or ink, adv)
            y += lh
            r += 1
    return c.img


def document(w, h, s, dark, aa, seed, scan=False):
    rng = random.Random(seed)
    c = Canvas(w, h, (255, 255, 255), aa, s)
    W, H = w // s, h // s
    mx = 80
    c.text((W // 2 - 140, 70), "Technical Report " + str(rng.randint(10, 99)), 26, (0, 0, 0), bold=True)
    c.text((W // 2 - 90, 108), "Revision 2026-11-15", 13, (60, 60, 60))
    y = 150
    page = 1
    while y < H - 90:
        if rng.random() < 0.25:
            c.text((mx, y), f"{rng.randint(1, 9)}. " + words(rng, 3).title(), 18, (0, 0, 0), bold=True)
            y += 36
        para = " ".join(sentence(rng, rng.randint(8, 14)) for _ in range(rng.randint(3, 5)))
        cur = ""
        for word in para.split():
            if c.width(cur + " " + word, 12.5) / s > W - 2 * mx:
                c.text((mx, y), cur, 12.5, (20, 20, 20))
                y += 19
                cur = word
                if y > H - 90:
                    break
            else:
                cur = (cur + " " + word).strip()
        y += 14
    c.text((W // 2 - 10, H - 50), str(page), 12, (60, 60, 60))
    img = c.img
    if scan:
        img = img.rotate(0.35, resample=Image.BICUBIC, fillcolor=(236, 232, 224))
        img = img.filter(ImageFilter.GaussianBlur(0.7))
        a = np.asarray(img).astype(np.float32)
        nr = np.random.default_rng(seed)
        a = a * 0.93 + 6 + nr.normal(0, 5.0, a.shape[:2])[..., None]
        img = Image.fromarray(np.clip(a, 0, 255).astype(np.uint8))
    return img


def slide(w, h, s, dark, aa, seed):
    rng = random.Random(seed)
    bg = (16, 24, 48) if dark else (250, 250, 247)
    ink = (245, 245, 245) if dark else (20, 28, 50)
    acc = (255, 176, 59) if dark else (200, 60, 30)
    c = Canvas(w, h, bg, aa, s)
    W, H = w // s, h // s
    c.rect((0, 0, 14, H), fill=acc)
    c.text((70, 54), words(rng, 3).title(), 46, ink, bold=True)
    c.line((70, 118, 420, 118), acc, 4)
    for i in range(5):
        y = 160 + i * 74
        c.ellipse((74, y + 10, 90, y + 26), fill=acc)
        c.text((110, y), sentence(rng, 6)[:-1], 28, ink)
        c.text((110, y + 36), words(rng, 7), 16, (150, 160, 180) if dark else (90, 100, 120))
    # side figure: three labeled boxes
    for i, label in enumerate(["Input", "Process", "Output"]):
        bx = 780 + 0
        by = 160 + i * 130
        c.rect((bx, by, bx + 360, by + 90), fill=(36, 52, 96) if dark else (232, 236, 245), outline=acc)
        c.text((bx + 120, by + 28), label, 26, ink, bold=True)
        if i < 2:
            c.line((bx + 180, by + 90, bx + 180, by + 130), acc, 3)
    c.text((W - 120, H - 36), f"{rng.randint(2, 30)} / 32", 14, (150, 160, 180))
    return c.img


def chart(w, h, s, dark, aa, seed):
    rng = random.Random(seed)
    t = theme(dark)
    c = Canvas(w, h, t["bg"], aa, s)
    W, H = w // s, h // s
    c.text((60, 24), "Monthly throughput by region", 24, t["ink"], bold=True)
    c.text((60, 58), "Requests per second, 2026", 14, t["dim"])
    left, top, right, bottom = 90, 110, W - 60, H - 90
    for i in range(6):
        y = bottom - (bottom - top) * i / 5
        c.line((left, y, right, y), t["line"], 1)
        c.text((left - 56, y - 8), f"{i * 200}", 14, t["dim"])
    n = 12
    palette = [(66, 133, 244), (234, 67, 53), (251, 188, 5), (52, 168, 83)]
    bw = (right - left) / n
    series = [[rng.randint(120, 900) for _ in range(n)] for _ in range(3)]
    for m in range(n):
        for k in range(3):
            v = series[k][m]
            x0 = left + m * bw + 8 + k * (bw - 16) / 3
            x1 = x0 + (bw - 16) / 3 - 2
            c.rect((x0, bottom - (bottom - top) * v / 1000, x1, bottom), fill=palette[k])
        c.text((left + m * bw + bw / 2 - 12, bottom + 10), ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                                                             "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"][m], 14, t["ink"])
    for k, name in enumerate(["EMEA", "APAC", "AMER"]):
        lx = right - 330 + k * 110
        c.rect((lx, 78, lx + 14, 92), fill=palette[k])
        c.text((lx + 20, 76), name, 14, t["ink"])
    c.line((left, top, left, bottom), t["ink"], 2)
    c.line((left, bottom, right, bottom), t["ink"], 2)
    return c.img


# (id, kind, description, builder, size, extra) -- two sizes per kind
def generated():
    # (w, h, s): about 1 MP at s=1, about 4 MP at s=2
    small, large = (1280, 800, 1), (2560, 1600, 2)
    return [
        ("desktop-ui-light-1mp", "desktop UI", "settings window, light theme, antialiased text", desktop_ui, small, dict(dark=False, aa=True, seed=11)),
        ("desktop-ui-dark-4mp", "desktop UI", "settings window, dark theme, 2x scale, text without antialiasing", desktop_ui, large, dict(dark=True, aa=False, seed=12)),
        ("web-light-1mp", "web page", "article page with sidebar cards, light theme, antialiased", web_page, small, dict(dark=False, aa=True, seed=21)),
        ("web-dark-4mp", "web page", "article page with sidebar cards, dark theme, 2x scale, antialiased", web_page, large, dict(dark=True, aa=True, seed=22)),
        ("terminal-dark-1mp", "terminal/code", "dark terminal grid, code and compiler output, antialiased", terminal, small, dict(dark=True, aa=True, seed=31)),
        ("terminal-light-4mp", "terminal/code", "light terminal grid, 2x scale, text without antialiasing", terminal, large, dict(dark=False, aa=False, seed=32)),
        ("document-scan-1mp", "document page", "rendered page with scan simulation (rotation 0.35 deg, blur, grain)", lambda w, h, s, **k: document(w, h, s, scan=True, **k), (850, 1100, 1), dict(dark=False, aa=True, seed=41)),
        ("document-clean-4mp", "document page", "rendered page, 2x scale, clean, antialiased", document, (1700, 2200, 2), dict(dark=False, aa=True, seed=42)),
        ("slide-dark-1mp", "slide", "dark title slide with bullets and boxes, antialiased", slide, (1280, 720, 1), dict(dark=True, aa=True, seed=51)),
        ("slide-light-4mp", "slide", "light slide, 2x scale, text without antialiasing", slide, (2560, 1440, 2), dict(dark=False, aa=False, seed=52)),
        ("chart-light-1mp", "chart", "grouped bar chart with labels, light theme, antialiased", chart, (1200, 900, 1), dict(dark=False, aa=True, seed=61)),
        ("chart-dark-4mp", "chart", "grouped bar chart with labels, dark theme, 2x scale, antialiased", chart, (2400, 1800, 2), dict(dark=True, aa=True, seed=62)),
        # Small images, at most 8 groups of 256x256 pixels: cjxl's default
        # (non-streaming) path, where its patch heuristic runs at effort 7.
        ("desktop-ui-light-0.5mp", "desktop UI", "small settings window, light theme, antialiased (8 groups)", desktop_ui, (1024, 512, 1), dict(dark=False, aa=True, seed=13)),
        ("web-light-0.5mp", "web page", "small article page, light theme, text without antialiasing (8 groups)", web_page, (1024, 512, 1), dict(dark=False, aa=False, seed=23)),
        ("terminal-dark-0.4mp", "terminal/code", "small dark terminal grid, antialiased (6 groups)", terminal, (768, 512, 1), dict(dark=True, aa=True, seed=33)),
        ("document-clean-0.4mp", "document page", "small rendered page, clean, antialiased (6 groups)", document, (512, 768, 1), dict(dark=False, aa=True, seed=43)),
        ("slide-dark-0.5mp", "slide", "small dark slide, antialiased (8 groups)", slide, (912, 512, 1), dict(dark=True, aa=True, seed=53)),
        ("chart-light-0.5mp", "chart", "small bar chart with labels, light theme, antialiased (8 groups)", chart, (1024, 512, 1), dict(dark=False, aa=True, seed=63)),
    ]


PHOTOS = [
    ("photo-paris-png", "codecs/libavif-rs/upstream/libavif/tests/data/paris_icc_exif_xmp.png"),
    ("photo-flower-jpeg", "codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg"),
    ("photo-dog-jpeg", "codecs/libavif-rs/upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg"),
    ("photo-seine-jpeg", "codecs/libavif-rs/upstream/libavif/tests/data/seine_sdr_gainmap_srgb.jpg"),
]


def write_ppm(path, img):
    img = img.convert("RGB")
    with open(path, "wb") as f:
        f.write(b"P6\n%d %d\n255\n" % img.size)
        f.write(img.tobytes())


def main():
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    entries = []
    for name, kind, desc, fn, (w, h, s), kw in generated():
        img = fn(w, h, s, **kw)
        assert img.size == (w, h), (name, img.size)
        write_ppm(out / f"{name}.ppm", img)
        entries.append(dict(name=name, kind=kind, description=desc, width=w, height=h, source="generated"))
    for name, rel in PHOTOS:
        src = ROOT / rel
        img = Image.open(src)
        img.load()
        write_ppm(out / f"{name}.ppm", img)
        entries.append(dict(name=name, kind="photograph", description="repository fixture, decoded to 8-bit RGB (embedded ICC ignored)",
                            width=img.size[0], height=img.size[1], source=rel))
    for e in entries:
        e["sha256"] = hashlib.sha256((out / f"{e['name']}.ppm").read_bytes()).hexdigest()
    (out / "corpus.json").write_text(json.dumps(dict(pillow=PIL.__version__, numpy=np.__version__, images=entries), indent=2) + "\n")
    for e in entries:
        print(f"{e['name']:24} {e['kind']:14} {e['width']}x{e['height']}  {e['sha256'][:12]}")


if __name__ == "__main__":
    main()
