#!/usr/bin/env python3
"""Run the repository checks: provenance, skeleton, formatting, Clippy and tests.

Stages:
  static  upstream pins and license copies, module skeleton, documentation
          links, `cargo fmt --check`, Clippy with -D warnings on all targets
  test    `cargo test` in the dev, release and checked profiles
  all     both (default)

--oracle builds the C oracle (tools/build_oracle.py), sets LIBWEBP_ORACLE_DIR
and enables the `oracle` feature, so that differential tests fail instead of
skipping when they cannot run. --baseline also runs tools/oracle_baseline.py.
--msrv adds a build and test run with the toolchain named by rust-version.
"""

import argparse
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))

import build_oracle  # noqa: E402
import units  # noqa: E402
import verify_upstream  # noqa: E402

CARGO = ["--locked", "--offline"]
PROFILES = [[], ["--release"], ["--profile", "checked"]]


def run(*args, env=None):
    print("+ " + " ".join(map(str, args)), flush=True)
    subprocess.run(list(map(str, args)), cwd=ROOT, env=env, check=True)


def rust_version():
    return re.search(r'^rust-version = "([^"]+)"$', (ROOT / "Cargo.toml").read_text(), re.M)[1]


def check_provenance():
    problems = verify_upstream.verify()
    problems += units.verify(units.load())
    if problems:
        raise SystemExit("\n".join(problems))
    print("Upstream pins, license copies and module skeleton verified", flush=True)


def check_docs():
    documents = [ROOT / name for name in ("README.md", "CONTRIBUTING.md", "AGENTS.md")]
    documents.extend(sorted((ROOT / "docs").rglob("*.md")))
    for document in documents:
        for target in re.findall(r"\[[^\]]*\]\(([^)\s]+)\)", document.read_text()):
            if "://" in target or target.startswith("#"):
                continue
            path = target.split("#", 1)[0]
            if not (document.parent / path).exists():
                raise SystemExit(f"Broken local link in {document.relative_to(ROOT)}: {target}")
    print("Local documentation links verified", flush=True)


def check_static(msrv):
    check_provenance()
    check_docs()
    run("cargo", "fmt", "--all", "--", "--check")
    run("cargo", "clippy", *CARGO, "--all-targets", "--all-features", "--", "-D", "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", *CARGO, "--all-targets", "--all-features")


def check_tests(oracle_variant, baseline, msrv):
    env = dict(os.environ)
    features = []
    if oracle_variant:
        directory = build_oracle.build(oracle_variant, str(os.cpu_count() or 4), fresh=False)
        env["LIBWEBP_ORACLE_DIR"] = str(directory)
        features = ["--features", "oracle"]
        if baseline:
            run(sys.executable, ROOT / "tools/oracle_baseline.py", "--variant", oracle_variant)
    else:
        env.pop("LIBWEBP_ORACLE_DIR", None)
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
                        help="build the oracle and require it in the differential tests")
    parser.add_argument("--oracle-variant", choices=list(build_oracle.VARIANTS),
                        help="oracle variant for --oracle (default: release); implies --oracle")
    parser.add_argument("--baseline", action="store_true",
                        help="also run tools/oracle_baseline.py; implies --oracle")
    parser.add_argument("--msrv", action="store_true",
                        help="also build and test with the toolchain named by rust-version")
    args = parser.parse_args()
    oracle_variant = None
    if args.oracle or args.oracle_variant or args.baseline:
        oracle_variant = args.oracle_variant or "release"
    if args.stage in ("all", "static"):
        check_static(args.msrv)
    else:
        check_provenance()
    if args.stage in ("all", "test"):
        check_tests(oracle_variant, args.baseline, args.msrv)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
