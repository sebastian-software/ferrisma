#!/usr/bin/env python3
"""Run the repository checks: provenance, skeleton, links, formatting, Clippy, tests.

Stages:
  static  upstream pin, module skeleton (tools/units.py), pool inventory
          (tools/pools.py), documentation links, `cargo fmt --check`, Clippy
          with -D warnings on all targets, with all features and without the
          default ones
  test    `cargo test` in the dev, release and checked profiles
  all     both (default)

--oracle builds the C oracle (tools/build_oracle.py), verifies the bit-test
inventory and runs the in-scope djpeg bit tests against it
(tools/bittests.py), fetches the ISO/IEC 10918-2 compliance data
(tools/fetch_t83.py, which reuses a verified zip in build/t83), then sets
LIBJPEG_ORACLE_DIR and LIBJPEG_T83_DIR and enables the `oracle` Cargo
feature, so that differential and conformance tests fail instead of skipping
when they cannot run. --oracle-variant picks the variant (default: release)
and implies --oracle. --msrv adds a build and test run with the toolchain named by
rust-version.

The crate's features follow the oracle (ADR 0007): the reference variants
test the crate without default features, the reference configuration; the
simd variant tests it with the default `simd` feature in each SIMD mode of
the architecture (ADR 0008), with the same environment for the port and the
oracle, and sets LIBJPEG_EXPECT_SIMD to the instruction set that the dispatch
test must find selected:

  aarch64  neon; none (JSIMD_FORCENONE=1)
  x86_64   avx2; sse2 (JSIMD_FORCESSE2=1); none (JSIMD_FORCENONE=1)

Without an oracle, the tests run with the default features, plus once without
them in the dev profile; the conformance tests run if build/t83 holds the data
and skip otherwise.

--arch x86_64 checks the x86-64 build on an ARM64 Mac through Rosetta 2: Cargo
builds for x86_64-apple-darwin, the oracle is tools/build_oracle.py's
--arch x86_64 variant, and ROSETTA_ADVERTISE_AVX=1 lets Rosetta report AVX2.
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
import fetch_t83  # noqa: E402
import pools  # noqa: E402
import units  # noqa: E402

CARGO = ["--locked", "--offline"]
PROFILES = [[], ["--release"], ["--profile", "checked"]]
# The SIMD modes of each architecture: the value of LIBJPEG_EXPECT_SIMD and
# the variables that select the mode in the port and the oracle.
SIMD_MODES = {
    "aarch64": [("neon", {}), ("none", {"JSIMD_FORCENONE": "1"})],
    "x86_64": [("avx2", {}), ("sse2", {"JSIMD_FORCESSE2": "1"}),
               ("none", {"JSIMD_FORCENONE": "1"})],
}
SIMD_VARIABLES = ["JSIMD_FORCENONE", "JSIMD_FORCESSE2", "LIBJPEG_EXPECT_SIMD"]


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
    gitlink = output("git", "ls-files", "--stage", "--", pin["path"]).split()
    if len(gitlink) < 3 or gitlink[0] != "160000" or gitlink[1] != pin["revision"]:
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


def check_pools():
    problems, count = pools.verify()
    if problems:
        raise SystemExit("\n".join(problems))
    print(f"Pool inventory verified: {count} allocation calls", flush=True)


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


def cargo_target(arch):
    """Cargo's --target for tools/build_oracle.py's --arch, if any."""
    if arch is None:
        return []
    return ["--target", build_oracle.CROSS_ARCHES[arch]["rust_target"]]


def check_static(msrv, arch):
    check_upstream()
    check_skeleton()
    check_pools()
    check_docs()
    target = cargo_target(arch)
    run("cargo", "fmt", "-p", "libjpeg-turbo-rs", "--", "--check")
    run("cargo", "+stable", "clippy", *CARGO, *target, "--all-targets", "--all-features", "--", "-D",
        "warnings")
    run("cargo", "+stable", "clippy", *CARGO, *target, "--all-targets", "--no-default-features",
        "--features", "oracle", "--", "-D", "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", *CARGO, *target, "--all-targets",
            "--all-features")


def check_tests(oracle_variant, msrv, arch):
    env = dict(os.environ)
    for name in SIMD_VARIABLES:
        env.pop(name, None)
    target = cargo_target(arch)
    if arch is not None:
        env["ROSETTA_ADVERTISE_AVX"] = "1"
    features = []
    modes = [(None, env)]
    if oracle_variant:
        jobs = str(os.cpu_count() or 4)
        directory = build_oracle.build(oracle_variant, jobs, fresh=False, arch=arch)
        run(sys.executable, ROOT / "tools/bittests.py", "--check", "--run",
            "--variant", oracle_variant, "--jobs", jobs,
            *(["--arch", arch] if arch is not None else []), env=env)
        env["LIBJPEG_ORACLE_DIR"] = str(directory)
        env["LIBJPEG_T83_DIR"] = str(fetch_t83.fetch(fetch_t83.DEFAULT_DIR))
        if build_oracle.VARIANTS[oracle_variant]["simd"]:
            features = ["--features", "oracle"]
            modes = [(expected, {**env, **variables, "LIBJPEG_EXPECT_SIMD": expected})
                     for expected, variables in SIMD_MODES[build_oracle.target_arch(arch)]]
        else:
            features = ["--no-default-features", "--features", "oracle"]
    else:
        env.pop("LIBJPEG_ORACLE_DIR", None)
        env.pop("LIBJPEG_T83_DIR", None)
    for expected, mode in modes:
        if expected is not None:
            settings = [f"{name}={mode[name]}" for name in SIMD_VARIABLES if name in mode]
            print("+ SIMD mode: " + " ".join(settings), flush=True)
        for profile in PROFILES:
            run("cargo", "test", *CARGO, *target, *profile, *features, env=mode)
    if not oracle_variant:
        run("cargo", "test", *CARGO, *target, "--no-default-features", env=env)
    if msrv:
        toolchain = f"+{rust_version()}"
        run("cargo", toolchain, "build", *CARGO, *target, "--all-targets", *features)
        run("cargo", toolchain, "test", *CARGO, *target, *features, env=modes[0][1])


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
    parser.add_argument("--arch", choices=list(build_oracle.CROSS_ARCHES),
                        help="check this architecture on an ARM64 Mac (Rosetta 2)")
    args = parser.parse_args()
    oracle_variant = None
    if args.oracle or args.oracle_variant:
        oracle_variant = args.oracle_variant or "release"
    if args.stage in ("all", "static"):
        check_static(args.msrv, args.arch)
    else:
        check_upstream()
        check_skeleton()
    if args.stage in ("all", "test"):
        check_tests(oracle_variant, args.msrv, args.arch)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
