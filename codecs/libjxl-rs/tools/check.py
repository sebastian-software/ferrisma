#!/usr/bin/env python3
"""Validate the pinned source, Rust port, and original C++ differential oracle."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

from brotli_check_support import prepare as prepare_brotli_observers

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
    brotli = source / "third_party/brotli"
    expected = git("-C", source, "ls-tree", "HEAD", "third_party/brotli").split()[2]
    if not (brotli / "c/include/brotli/encode.h").is_file():
        raise SystemExit("Initialize upstream's third_party/brotli submodule; see CONTRIBUTING.md")
    if git("-C", brotli, "rev-parse", "HEAD") != expected:
        raise SystemExit("Brotli revision does not match upstream's submodule pin")
    skcms = source / "third_party/skcms"
    expected = git("-C", source, "ls-tree", "HEAD", "third_party/skcms").split()[2]
    if not (skcms / "skcms.h").is_file():
        raise SystemExit("Initialize upstream's third_party/skcms submodule; see CONTRIBUTING.md")
    if git("-C", skcms, "rev-parse", "HEAD") != expected:
        raise SystemExit("skcms revision does not match upstream's submodule pin")
    for name in ("LICENSE", "PATENTS"):
        if (ROOT / name).read_bytes() != (source / name).read_bytes():
            raise SystemExit(f"{name} differs from the pinned upstream notice")
    if (ROOT / "licenses/HIGHWAY-BSD-3-Clause.txt").read_bytes() != (highway / "LICENSE-BSD3").read_bytes():
        raise SystemExit("The translated Highway helper's notice differs from upstream")
    if (ROOT / "licenses/SKCMS-BSD-3-Clause.txt").read_bytes() != (skcms / "LICENSE").read_bytes():
        raise SystemExit("The translated skcms notice differs from upstream")
    if (ROOT / "licenses/BROTLI-MIT.txt").read_bytes() != (brotli / "LICENSE").read_bytes():
        raise SystemExit("The translated Brotli notice differs from upstream")
    sort_reference = ROOT / "tools/reference/libcxx"
    sort_pin = json.loads((sort_reference / "source.json").read_text())
    for name, expected in sort_pin["files"].items():
        if hashlib.sha256((sort_reference / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f"libc++ reference snapshot mismatch: {name}")
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


def rust_version():
    return re.search(r'^rust-version = "([^"]+)"$', (ROOT / "Cargo.toml").read_text(), re.M)[1]


def release_only_tests():
    # Tests compiled only without debug assertions are absent from the checked
    # profile, so their whole binaries also run in an ordinary release build.
    return sorted(path.stem for path in (ROOT / "tests").glob("*.rs")
                  if "cfg(not(debug_assertions))" in path.read_text())


def check_static(msrv):
    check_docs()
    run("cargo", "fmt", "-p", "libjxl-rs", "--", "--check")
    run("cargo", "+stable", "clippy", "--locked", "--offline", "--all-targets", "--all-features", "--", "-D", "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", "--locked", "--offline", "--all-targets", "--all-features")
    run("cargo", "build", "--locked", "--offline", "--release", "--all-targets", "--all-features")
    for name in release_only_tests():
        run("cargo", "test", "--locked", "--offline", "--release", "--all-features", "--test", name)


def build_oracle(sanitizers):
    build = ROOT / "target" / ("oracle-asan" if sanitizers else "oracle")
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release",
        f"-DORACLE_SANITIZERS={'ON' if sanitizers else 'OFF'}")
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    from skcms_fitting_reference import build_private as build_fitting_private
    from skcms_parser_reference import build_private as build_parser_private
    from skcms_transform_reference import build_private as build_transform_private
    from default_cms_reference import build_contexts as build_default_cms_contexts
    transform_helper = build_transform_private(build)
    helper = build_fitting_private(build)
    build_parser_private(build)
    default_cms_public, default_cms_private = build_default_cms_contexts(build)
    env = dict(os.environ, JXL_CPP_ORACLE=str(build / "jxl_oracle"),
               JXL_SKCMS_FITTING_ORACLE=str(helper),
               JXL_SKCMS_TRANSFORM_ORACLE=str(transform_helper),
               JXL_DEFAULT_CMS_ORACLE=str(default_cms_public),
               JXL_DEFAULT_CMS_PRIVATE_ORACLE=str(default_cms_private),
               JXL_DEFAULT_CMS_CAPTURE=str(build / "default-cms-corpus"),
               JXL_DECODER_PUBLIC_CAPTURE=str(build / "decoder-public-corpus"))
    env.update(prepare_brotli_observers(sanitizers))
    return env


def build_simd_oracle():
    # The NEON (AArch64) or AVX2 (x86-64) variant of the original for the
    # SIMD paths (ADR 0004); tests/simd_parity.rs reads JXL_CPP_ORACLE_SIMD.
    # x86-64 under Rosetta 2 needs ROSETTA_ADVERTISE_AVX=1 to report AVX2.
    import platform
    target = {"arm64": "NEON", "aarch64": "NEON", "x86_64": "AVX2", "AMD64": "AVX2"}.get(platform.machine())
    if target is None:
        raise SystemExit(f"No SIMD oracle target for {platform.machine()}")
    build = ROOT / "target" / f"oracle-{target.lower()}"
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release", f"-DORACLE_HWY_TARGET={target}")
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    return {"JXL_CPP_ORACLE_SIMD": str(build / "jxl_oracle"), "ROSETTA_ADVERTISE_AVX": "1"}


def check_tests(env, partition, filter):
    test = ["cargo", "test", "--locked", "--offline", "--profile", "checked", "--all-features"]
    if filter is not None:
        run(*test, "--", "--nocapture", filter, env=env)
        return
    if partition is None:
        run(*test, "--", "--nocapture", env=env)
        return
    index, count = partition
    # Every listed name belongs to exactly one partition. A name shared by
    # several test binaries runs in each of them within that partition.
    print(f"+ {' '.join(test)} -- --list --format terse", flush=True)
    listing = subprocess.run([*test, "--", "--list", "--format", "terse"], cwd=ROOT, env=env,
                             stdout=subprocess.PIPE, text=True, check=True).stdout
    names = sorted({line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")})
    selected = names[index - 1::count]
    print(f"Partition {index}/{count}: {len(selected)} of {len(names)} test names", flush=True)
    if selected:
        run(*test, "--", "--nocapture", "--exact", *selected, env=env)


def partition(value):
    match = re.fullmatch(r"([1-9][0-9]*)/([1-9][0-9]*)", value)
    if not match or int(match[1]) > int(match[2]):
        raise argparse.ArgumentTypeError("expected K/N with 1 <= K <= N")
    return int(match[1]), int(match[2])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sanitizers", action="store_true", help="Instrument the original C++ oracle with ASan and UBSan")
    parser.add_argument("--msrv", action="store_true",
                        help="Also compile all targets with the already-installed rust-version toolchain")
    parser.add_argument("--stage", choices=["all", "static", "oracle", "test"], default="all",
                        help="static: pins, notices, links, fmt, Clippy, MSRV, and release builds; "
                             "oracle: build the selected C++ reference only; test: oracle and checked-profile tests")
    parser.add_argument("--partition", type=partition, metavar="K/N",
                        help="Run only the K-th of N disjoint subsets of test names")
    parser.add_argument("--filter", metavar="NAME",
                        help="Run only the tests whose names contain NAME, for example to write an audit's capture")
    args = parser.parse_args()
    if args.partition is not None and args.filter is not None:
        parser.error("--partition and --filter are mutually exclusive")
    check_source()
    if args.stage in ("all", "static"):
        check_static(args.msrv)
    if args.stage in ("all", "oracle", "test"):
        env = build_oracle(args.sanitizers)
        # The SIMD variant has no sanitized twin (ADR 0004).
        env.update(build_simd_oracle())
    if args.stage in ("all", "test"):
        check_tests(env, args.partition, args.filter)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
