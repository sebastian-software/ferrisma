#!/usr/bin/env python3
"""Validate the pinned source, Rust port, and original C++ differential oracle."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def run(*args, env=None):
    print("+ " + " ".join(map(str, args)), flush=True)
    subprocess.run(list(map(str, args)), cwd=ROOT, env=env, check=True)


def git(*args):
    return subprocess.check_output(["git", *map(str, args)], cwd=ROOT, text=True).strip()


def check_source():
    pin = json.loads((ROOT / "upstream.json").read_text())
    source = ROOT / pin["path"]
    revision = git("-C", source, "rev-parse", "HEAD")
    if revision != pin["revision"]:
        raise SystemExit(f"Upstream revision mismatch: {revision} != {pin['revision']}")
    entry = git("ls-files", "--stage", "--", pin["path"]).split()
    if len(entry) < 3 or entry[0] != "160000" or entry[1] != pin["revision"]:
        raise SystemExit("The Git submodule entry does not match upstream.json")
    dirty = git("-C", source, "status", "--porcelain", "--untracked-files=normal", "--ignore-submodules=none")
    if dirty:
        raise SystemExit(f"Upstream source must stay unmodified:\n{dirty}")
    highway = source / "third_party/highway"
    expected = git("-C", source, "ls-tree", "HEAD", "third_party/highway").split()[2]
    if not (highway / "hwy/base.h").is_file():
        raise SystemExit("Initialize upstream's third_party/highway submodule; see CONTRIBUTING.md")
    if git("-C", highway, "rev-parse", "HEAD") != expected:
        raise SystemExit("Highway revision does not match upstream's submodule pin")
    for name in ("LICENSE", "PATENTS"):
        if (ROOT / name).read_bytes() != (source / name).read_bytes():
            raise SystemExit(f"{name} differs from the pinned upstream notice")
    print(f"Pinned source and notices verified: {revision}", flush=True)


def check_docs():
    documents = [ROOT / name for name in ("README.md", "CONTRIBUTING.md", "AGENTS.md")]
    documents.extend((ROOT / "docs").rglob("*.md"))
    for document in documents:
        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", document.read_text()):
            if "://" in target or target.startswith("#"):
                continue
            path = target.split("#", 1)[0]
            if not (document.parent / path).exists():
                raise SystemExit(f"Broken local link in {document.relative_to(ROOT)}: {target}")
    print("Local documentation links verified", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sanitizers", action="store_true", help="Instrument the original C++ oracle with ASan and UBSan")
    parser.add_argument("--msrv", action="store_true", help="Also test with the already-installed Rust 1.85 toolchain")
    args = parser.parse_args()
    check_source()
    check_docs()
    run("cargo", "fmt", "--all", "--", "--check")
    build = ROOT / "target" / ("oracle-asan" if args.sanitizers else "oracle")
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release",
        f"-DORACLE_SANITIZERS={'ON' if args.sanitizers else 'OFF'}")
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    env = dict(os.environ, JXL_CPP_ORACLE=str(build / "jxl_oracle"))
    run("cargo", "clippy", "--locked", "--offline", "--all-targets", "--all-features", "--", "-D", "warnings")
    run("cargo", "test", "--locked", "--offline", "--all-features", "--", "--nocapture", env=env)
    run("cargo", "test", "--locked", "--offline", "--release", "--all-features", "--", "--nocapture", env=env)
    if args.msrv:
        run("cargo", "+1.85", "test", "--locked", "--offline", "--all-features", "--", "--nocapture", env=env)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
