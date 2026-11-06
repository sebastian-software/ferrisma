#!/usr/bin/env python3
"""Run the repository checks: provenance, skeleton, links, formatting, Clippy, tests.

Stages:
  static  upstream pin, module skeleton (tools/units.py), documentation links,
          `cargo fmt --check`, Clippy with -D warnings on all targets
  test    `cargo test` in the dev, release and checked profiles
  all     both (default)

--oracle builds the C oracle (tools/build_oracle.py), verifies the bit-test
inventory and runs the in-scope djpeg bit tests against it
(tools/bittests.py), then sets LIBJPEG_ORACLE_DIR and enables the `oracle`
Cargo feature, so that differential tests fail instead of skipping when they
cannot run. --oracle-variant picks the variant (default: release) and implies
--oracle. --msrv adds a build and test run with the toolchain named by
rust-version.
"""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))

import build_oracle  # noqa: E402
import units  # noqa: E402

CARGO = ["--locked", "--offline"]
PROFILES = [[], ["--release"], ["--profile", "checked"]]


def run(*args, env=None):
    print("+ " + " ".join(map(str, args)), flush=True)
    subprocess.run(list(map(str, args)), cwd=ROOT, env=env, check=True)


def output(*args):
    return subprocess.run(list(map(str, args)), cwd=ROOT, check=True,
                          stdout=subprocess.PIPE, universal_newlines=True).stdout


def rust_version():
    return re.search(r'^rust-version = "([^"]+)"$', (ROOT / "Cargo.toml").read_text(), re.M)[1]


def check_upstream():
    """The submodule sits at the pinned commit and is unmodified."""
    pin = json.loads((ROOT / "upstream.json").read_text())["libjpeg-turbo"]
    path = ROOT / pin["path"]
    if not (path / "src/jpeglib.h").is_file():
        raise SystemExit(f"{pin['path']} is missing; run git submodule update --init")
    head = output("git", "-C", path, "rev-parse", "HEAD").strip()
    if head != pin["revision"]:
        raise SystemExit(f"{pin['path']} is at {head}, upstream.json pins {pin['revision']}")
    gitlink = output("git", "ls-tree", "HEAD", pin["path"]).split()
    if len(gitlink) < 3 or gitlink[2] != pin["revision"]:
        raise SystemExit(f"the {pin['path']} gitlink differs from upstream.json")
    status = output("git", "-C", path, "status", "--porcelain", "--untracked-files=all")
    if status.strip():
        raise SystemExit(f"{pin['path']} is modified:\n{status}")
    print(f"Upstream pin verified: {pin['tag']} at {pin['revision']}, unmodified", flush=True)


def check_skeleton():
    problems = units.verify(units.load())
    if problems:
        raise SystemExit("\n".join(problems))
    print("Module skeleton verified", flush=True)


def check_docs():
    documents = [ROOT / name for name in ("README.md", "AGENTS.md")]
    documents.extend(sorted((ROOT / "docs").rglob("*.md")))
    for document in documents:
        for target in re.findall(r"\[[^\]]*\]\(([^)\s]+)\)", document.read_text()):
            if "://" in target or target.startswith(("#", "mailto:")):
                continue
            path = target.split("#", 1)[0]
            if not (document.parent / path).exists():
                raise SystemExit(f"Broken local link in {document.relative_to(ROOT)}: {target}")
    print("Local documentation links verified", flush=True)


def check_static(msrv):
    check_upstream()
    check_skeleton()
    check_docs()
    run("cargo", "fmt", "--all", "--", "--check")
    run("cargo", "clippy", *CARGO, "--all-targets", "--all-features", "--", "-D", "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", *CARGO, "--all-targets", "--all-features")


def check_tests(oracle_variant, msrv):
    env = dict(os.environ)
    features = []
    if oracle_variant:
        directory = build_oracle.build(oracle_variant, str(os.cpu_count() or 4), fresh=False)
        run(sys.executable, ROOT / "tools/bittests.py", "--check", "--run",
            "--variant", oracle_variant, "--jobs", str(os.cpu_count() or 4))
        env["LIBJPEG_ORACLE_DIR"] = str(directory)
        features = ["--features", "oracle"]
    else:
        env.pop("LIBJPEG_ORACLE_DIR", None)
    for profile in PROFILES:
        run("cargo", "test", *CARGO, *profile, *features, env=env)
    if msrv:
        toolchain = f"+{rust_version()}"
        run("cargo", toolchain, "build", *CARGO, "--all-targets", *features)
        run("cargo", toolchain, "test", *CARGO, *features, env=env)


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", choices=["all", "static", "test"], default="all")
    parser.add_argument("--oracle", action="store_true",
                        help="build the oracle, run the bit tests and require the oracle in tests")
    parser.add_argument("--oracle-variant", choices=list(build_oracle.VARIANTS),
                        help="oracle variant for --oracle (default: release); implies --oracle")
    parser.add_argument("--msrv", action="store_true",
                        help="also build and test with the toolchain named by rust-version")
    args = parser.parse_args()
    oracle_variant = None
    if args.oracle or args.oracle_variant:
        oracle_variant = args.oracle_variant or "release"
    if args.stage in ("all", "static"):
        check_static(args.msrv)
    else:
        check_upstream()
        check_skeleton()
    if args.stage in ("all", "test"):
        check_tests(oracle_variant, args.msrv)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
