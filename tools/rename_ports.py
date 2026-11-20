#!/usr/bin/env python3
"""Rename the port packages to their decided crates.io names (#207, owner decision O5).

The owner decided on 2026-11-15 that the four ports take the `ferrisma-` prefix with
the originals' exact names (docs/publishing.md, section 1). Other branches touch the
same manifests, so the rename is not a hand edit: this script makes it, and it is
re-run on top of `main` after every rebase instead of cherry-picking the old commit.

What it changes, and nothing else:

* every tracked `Cargo.toml`: the `[package] name`, dependency keys and
  `package = "..."` values, `[...dependencies.<name>]` and
  `[profile.*.package.<name>]` headers, and `dep:<name>` / `<name>/feature`
  entries in feature lists;
* every tracked `Cargo.lock` that names a renamed package (the names are
  replaced, then Cargo re-sorts the file offline; no version changes);
* tooling that passes package names to Cargo or to the release drafts: `-p
  <name>` / `--package <name>`, exact string literals equal to a package name
  in Python, shell, YAML, JSON, JavaScript, CMake and the xtask, and the
  one-name-per-line crate list of the publish draft.

What it leaves alone on purpose: directory names (`codecs/libavif-rs` stays),
library target names (`jpeg`, `webp`, `jxl`, `avif`, so no Rust `use` changes),
prose and comments (they name the port, not the package), the engine's
user-visible decoder identifiers (`--jxl-decoder=libjxl-rs`, `src/`, `tests/`,
`scripts/bench/ci-perf/engine.py`), the homepage, and dated evidence under
`docs/`. `--check` lists the exact literals that remain, so a reviewer sees
every place the rules did not touch.

Usage (from the repository root, on a clean tree):

    python3 tools/rename_ports.py            # rename the four ports
    python3 tools/rename_ports.py --sub-crates   # also the sub-crates (not decided yet)
    python3 tools/rename_ports.py --check    # report only, change nothing

The internal ADR-0024 adapter `crates/ferrisma-libavif` holds the name the
libavif port takes; it is renamed first, in the same pass (its new name is
not decided by the owner; INTERNAL_ADAPTER below is the proposal).
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# The internal ADR-0024 adapter must leave the name `ferrisma-libavif` first.
INTERNAL_ADAPTER = ("crates/ferrisma-libavif", "ferrisma-libavif", "ferrisma-avif-adapter")

# Decided (owner, 2026-11-15, B1/O5): manifest directory, old name, new name.
PORTS = [
    ("codecs/libjpeg-turbo-rs", "libjpeg-turbo-rs", "ferrisma-libjpeg-turbo"),
    ("codecs/libwebp-rs", "libwebp-rs", "ferrisma-libwebp"),
    ("codecs/libjxl-rs", "libjxl-rs", "ferrisma-libjxl"),
    ("codecs/libavif-rs", "libavif-rs", "ferrisma-libavif"),
]

# Proposed in the same scheme, NOT decided: applied only with --sub-crates.
SUB_CRATES = [
    ("codecs/libavif-rs/crates/aom", "libavif-aom", "ferrisma-libaom"),
    ("codecs/libavif-rs/crates/dav1d", "libavif-dav1d", "ferrisma-dav1d"),
    ("codecs/libavif-rs/crates/dav1d-kernels", "libavif-dav1d-kernels", "ferrisma-dav1d-kernels"),
    ("codecs/libavif-rs/crates/sharpyuv", "libavif-sharpyuv", "ferrisma-libavif-sharpyuv"),
    ("crates/libsharpyuv-core", "libsharpyuv-core", "ferrisma-sharpyuv"),
]

CODE_SUFFIXES = {".py", ".sh", ".yml", ".yaml", ".json", ".mjs", ".js", ".cmake", ".draft"}
CODE_FILES = {"tools/xtask/src/main.rs"}
# Literals that are not package names: user-visible decoder identifiers, the
# homepage's prose data, dated evidence, and this script.
SKIP_PREFIXES = ("docs/", "homepage/", "src/", "tests/", "examples/", "fuzz/fuzz_targets/")
SKIP_FILES = {"tools/rename_ports.py", "scripts/bench/ci-perf/engine.py"}
# xtask maps a codec to its *directory* in this block; directories keep their names.
# Manual additions (documented here, applied by every run): places that build a
# package name from parts, which no generic rule can see. Each entry applies
# only when all the old names it needs are being renamed in this run.
# (file, needed old names, old text, new text)
FIXUPS = [
    # The root-profile probe writes its own manifest with the three AVIF
    # sub-crates as dependencies, naming them `libavif-{name}` (found by the
    # AVIF root workflow on v1/rename-ports, 2026-11-20).
    ("codecs/libavif-rs/tools/check_root_profile.py",
     {"libavif-aom", "libavif-dav1d", "libavif-sharpyuv"},
     r"""    for alias, name in [('aom', 'aom'), ('dav1d', 'dav1d'), ('sharpyuv', 'sharpyuv')]:
        manifest += f'{alias}={{package="libavif-{name}",path={json.dumps(str(ROOT / "crates" / name))}}}\n'
""",
     r"""    for alias, name, package in [('aom', 'aom', 'ferrisma-libaom'), ('dav1d', 'dav1d', 'ferrisma-dav1d'),
                                 ('sharpyuv', 'sharpyuv', 'ferrisma-libavif-sharpyuv')]:
        manifest += f'{alias}={{package="{package}",path={json.dumps(str(ROOT / "crates" / name))}}}\n'
"""),
]

XTASK_DIR_BLOCK = re.compile(r'root\(\)\.join\("codecs"\)\.join\(match codec\.as_str\(\) \{.*?\}\);', re.S)


def tracked(pattern: str) -> list[Path]:
    out = subprocess.run(["git", "ls-files", "-z", "--", pattern], cwd=ROOT, check=True,
                         capture_output=True).stdout.decode()
    return [ROOT / p for p in out.split("\0") if p]


def package_name(manifest: Path) -> str | None:
    section = None
    for line in manifest.read_text().splitlines():
        s = line.strip()
        if s.startswith("["):
            section = s
        elif section == "[package]":
            m = re.match(r'name\s*=\s*"([^"]+)"', s)
            if m:
                return m.group(1)
    return None


def active_mapping(sub_crates: bool) -> dict[str, str]:
    """Only rename a package that still carries its old name in its own manifest.

    This keeps a re-run harmless: after the rename `ferrisma-libavif` is the
    libavif port and must not be taken for the internal adapter again.
    """
    rows = [INTERNAL_ADAPTER, *PORTS, *(SUB_CRATES if sub_crates else [])]
    mapping = {}
    for directory, old, new in rows:
        manifest = ROOT / directory / "Cargo.toml"
        if manifest.exists() and package_name(manifest) == old:
            mapping[old] = new
    # Every new name must be free in the workspace after the rename.
    existing = {package_name(m) for m in tracked("*Cargo.toml")} - set(mapping)
    clash = sorted(n for n in mapping.values() if n in existing)
    if clash:
        sys.exit(f"rename_ports: new names already used by other packages: {clash}")
    return mapping


def name_re(mapping: dict[str, str]) -> str:
    # Longest first, so `ferrisma-libavif` never matches inside a longer name.
    return "|".join(re.escape(n) for n in sorted(mapping, key=len, reverse=True))


def rename_manifest(text: str, mapping: dict[str, str]) -> str:
    names = name_re(mapping)
    sub = lambda m: m.group(1) + mapping[m.group(2)] + m.group(3)  # noqa: E731
    out, section = [], ""
    for line in text.splitlines(keepends=True):
        code, hash_, comment = line.partition("#") if not line.lstrip().startswith("[") else (line, "", "")
        stripped = code.strip()
        if stripped.startswith("["):
            section = stripped
            code = re.sub(rf'(\[(?:[\w.\'"()-]*\.)?(?:dependencies|package)\.)({names})(\])', sub, code)
        elif section == "[package]":
            code = re.sub(rf'^(\s*name\s*=\s*")({names})(")', sub, code)
        elif section.endswith("dependencies]"):
            code = re.sub(rf'^(\s*)({names})(\s*=|\.)', sub, code)
            code = re.sub(rf'(package\s*=\s*")({names})(")', sub, code)
        elif section == "[features]":
            code = re.sub(rf'("(?:dep:)?)({names})(["/?])', sub, code)
        out.append(code + hash_ + comment)
    return "".join(out)


def keep_lib_name(path: Path, text: str, mapping: dict[str, str]) -> str:
    """Pin the library target name of a renamed package that has none.

    Cargo derives the library name from the package name, so without this the
    adapter's `ferrisma_libavif::` paths would break. The ports already name
    their libraries (`jpeg`, `webp`, `jxl`, `avif`).
    """
    old = package_name(path)
    if old not in mapping or re.search(r"^\[lib\]", text, re.M) or not (path.parent / "src/lib.rs").exists():
        return text
    lib = f'\n[lib]\n# The library keeps its name across the package rename (#207).\nname = "{old.replace("-", "_")}"\n'
    return text.rstrip("\n") + "\n" + lib


def rename_lock(text: str, mapping: dict[str, str]) -> str:
    names = name_re(mapping)
    text = re.sub(rf'^(name = ")({names})(")$', lambda m: m.group(1) + mapping[m.group(2)] + m.group(3),
                  text, flags=re.M)
    return re.sub(rf'^(\s*")({names})((?: [^"]*)?",?)$',
                  lambda m: m.group(1) + mapping[m.group(2)] + m.group(3), text, flags=re.M)


def rename_code(rel: str, text: str, mapping: dict[str, str]) -> str:
    names = name_re(mapping)
    keep = []
    if rel == "tools/xtask/src/main.rs":
        keep = [m.span() for m in XTASK_DIR_BLOCK.finditer(text)]

    def outside(pos: int) -> bool:
        return not any(a <= pos < b for a, b in keep)

    def apply(pattern: str, s: str, flags: int = 0) -> str:
        def repl(m: re.Match) -> str:
            if not outside(m.start()):
                return m.group(0)
            return m.group(1) + mapping[m.group(2)] + m.group(3)
        return re.sub(pattern, repl, s, flags=flags)

    # Exact string literals, `-p name`, `--package name`, and bare list lines.
    if rel.endswith(".rs") and rel not in CODE_FILES:
        # Rust sources elsewhere: only command lines in doc comments.
        return apply(rf'((?<![\w-])(?:-p|--package)[ =])({names})((?![\w/-]))', text)
    text = apply(rf'''(["'])({names})(\1)''', text)
    text = apply(rf'((?<![\w-])(?:-p|--package)[ =])({names})((?![\w/-]))', text)
    # Manifest fragments that tools write themselves (profile headers and
    # `package = "..."` inside Python or shell strings).
    text = apply(rf'(package\.)({names})(\])', text)
    text = apply(rf'(package\s*=\s*\\?")({names})(\\?")', text)
    if rel.endswith(".draft"):
        text = apply(rf'^(\s+)({names})(\s*)$', text, re.M)
    return text


def remaining_literals(mapping: dict[str, str]) -> list[str]:
    names = name_re(mapping)
    pattern = rf'''(["'])({names})\1|(?<![\w-])(?:-p|--package)[ =]({names})(?![\w/-])'''
    out = subprocess.run(["git", "grep", "-nP", pattern, "--", ":!*.md", ":!docs/**"], cwd=ROOT,
                         capture_output=True, text=True).stdout
    return out.splitlines()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--sub-crates", action="store_true", help="also rename the (undecided) sub-crates")
    parser.add_argument("--check", action="store_true", help="report only")
    args = parser.parse_args()

    mapping = active_mapping(args.sub_crates)
    if not mapping:
        print("rename_ports: nothing to rename (already done)")
        return
    for old, new in mapping.items():
        print(f"rename_ports: {old} -> {new}")

    changed: list[str] = []
    locks: list[Path] = []
    for path in tracked("*"):
        rel = path.relative_to(ROOT).as_posix()
        if not path.is_file() or rel in SKIP_FILES:
            continue
        if path.name == "Cargo.toml" and not rel.startswith("docs/"):
            new_text = rename_manifest(keep_lib_name(path, path.read_text(), mapping), mapping)
        elif path.name == "Cargo.lock" and not rel.startswith("docs/"):
            new_text = rename_lock(path.read_text(), mapping)
        elif (path.suffix in CODE_SUFFIXES or path.suffix == ".rs" or rel in CODE_FILES) \
                and not rel.startswith(SKIP_PREFIXES):
            new_text = rename_code(rel, path.read_text(), mapping)
        else:
            continue
        for fixed, needed, old_text, new_text_part in FIXUPS:
            if rel == fixed and needed <= set(mapping):
                new_text = new_text.replace(old_text, new_text_part)
        if new_text != path.read_text():
            changed.append(rel)
            if not args.check:
                path.write_text(new_text)
                if path.name == "Cargo.lock":
                    locks.append(path)

    for rel in changed:
        print(f"rename_ports: {'would change' if args.check else 'changed'} {rel}")
    if args.check:
        return
    # Cargo sorts lockfile entries by name; let it rewrite the renamed files
    # offline (no resolution changes), then prove each one is current.
    for lock in locks:
        manifest = lock.with_name("Cargo.toml")
        for extra in ([], ["--locked"]):
            subprocess.run(["cargo", "metadata", "--offline", "--format-version", "1", *extra,
                            "--manifest-path", str(manifest)], cwd=ROOT, check=True,
                           stdout=subprocess.DEVNULL)
    # A name that is also a new name (`ferrisma-libavif`) is not "left over".
    left = remaining_literals({old: new for old, new in mapping.items() if old not in mapping.values()})
    if left:
        print("rename_ports: literals of the old names left (by design or new; review):")
        for line in left:
            print("  " + line)


if __name__ == "__main__":
    main()
