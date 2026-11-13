#!/usr/bin/env python3
"""Run the repository checks: provenance, skeleton, formatting, Clippy and tests.

Stages:
  static  upstream pins and license copies, module skeleton, documentation
          links, `cargo fmt --check`, Clippy with -D warnings on all targets
  test    `cargo test` in the dev, release and checked profiles
  all     both (default)

--oracle runs the quality gate of ADR 0008 (Ferrisma decision D4): the port's
scalar build against the scalar original and the port's SIMD build against the
original's SIMD build at the same instruction-set level. It builds the C
oracle variants release and release-simd (tools/build_oracle.py, with the
consumer builds against the Rust static library), checks that the declared
package dependencies cover the oracle's cross-unit references
(tools/unit_dependencies.py --check), sets LIBWEBP_ORACLE_DIR and enables the
`oracle` feature, so that differential tests fail instead of skipping when
they cannot run. The crate's features follow the variant:

  release, asan-ubsan, debug   --no-default-features: the scalar translation
  release-simd                 the default `simd` feature, once per SIMD mode
                               of the architecture, with LIBWEBP_SIMD_LEVEL set
                               for the port and the oracle alike:
                                 aarch64  neon
                                 x86_64   avx2, sse41, sse2

LIBWEBP_SIMD_LEVEL caps the CPU query on both sides (src/dsp/cpu.rs with the
`oracle` feature, tools/oracle/probe_support/simd_level.c), so avx2 is what an
AVX2 machine selects, including the SSE2 and SSE4.1 kernels it falls back to,
and sse41 and sse2 are what machines without AVX2 or SSE4.1 select.
tests/dispatch_features.rs fails if the level the host reports is lower than
the mode asks for. --oracle-variant runs one variant; --no-simd with --oracle
runs only the scalar variant. --baseline also runs tools/oracle_baseline.py
for the scalar variant.

--arch x86_64 checks the x86-64 build on an ARM64 Mac through Rosetta 2: Cargo
builds for x86_64-apple-darwin, the oracle is tools/build_oracle.py's --arch
x86_64 variant, and ROSETTA_ADVERTISE_AVX=1 lets Rosetta report AVX2.
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
# The SIMD modes of each architecture: the values of LIBWEBP_SIMD_LEVEL that
# the release-simd variant is compared in (ADR 0008).
SIMD_MODES = {"aarch64": ["neon"], "x86_64": ["avx2", "sse41", "sse2"]}
SIMD_VARIABLES = ["LIBWEBP_SIMD_LEVEL"]


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


def check_static(msrv, arch=None):
    check_provenance()
    check_docs()
    target = build_oracle.cargo_target(arch)
    run("cargo", "fmt", "-p", "libwebp-rs", "--", "--check")
    run("cargo", "+stable", "clippy", *CARGO, *target, "--all-targets", "--all-features", "--", "-D",
        "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", *CARGO, *target, "--all-targets",
            "--all-features")


def check_tests(oracle_variants, baseline, msrv, no_simd=False, arch=None):
    env = dict(os.environ)
    for name in SIMD_VARIABLES:
        env.pop(name, None)
    target = build_oracle.cargo_target(arch)
    if arch is not None:
        env["ROSETTA_ADVERTISE_AVX"] = "1"
    # Runs: (description, features, environment). Without an oracle, --no-simd
    # tests the scalar translation (ADR 0008), and the default the SIMD build
    # at the level the host reports.
    runs = []
    if oracle_variants:
        jobs = str(os.cpu_count() or 4)
        for variant in oracle_variants:
            simd = build_oracle.VARIANTS[variant].get("simd", False)
            rust_lib = build_oracle.rust_staticlib(arch, simd=simd)
            directory = build_oracle.build(variant, jobs, fresh=False, rust_lib=rust_lib,
                                           arch=arch)
            run(sys.executable, ROOT / "tools/unit_dependencies.py", "--check", "--variant",
                directory.name)
            if baseline and not simd:
                run(sys.executable, ROOT / "tools/oracle_baseline.py", "--variant", directory.name)
            variant_env = {**env, "LIBWEBP_ORACLE_DIR": str(directory)}
            if simd:
                for level in SIMD_MODES[build_oracle.target_arch(arch)]:
                    runs.append((f"{variant}, SIMD mode LIBWEBP_SIMD_LEVEL={level}",
                                 ["--features", "oracle"],
                                 {**variant_env, "LIBWEBP_SIMD_LEVEL": level}))
            else:
                runs.append((f"{variant}, scalar build",
                             ["--no-default-features", "--features", "oracle"], variant_env))
    else:
        env.pop("LIBWEBP_ORACLE_DIR", None)
        runs.append(("no oracle", ["--no-default-features"] if no_simd else [], env))
    for description, features, run_env in runs:
        print(f"+ Gate run: {description}", flush=True)
        for profile in PROFILES:
            run("cargo", "test", *CARGO, *target, *profile, *features, env=run_env)
    if msrv:
        toolchain = f"+{rust_version()}"
        _, features, run_env = runs[0]
        run("cargo", toolchain, "build", *CARGO, *target, "--all-targets", *features)
        run("cargo", toolchain, "test", *CARGO, *target, *features, env=run_env)


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", choices=["all", "static", "test"], default="all")
    parser.add_argument("--oracle", action="store_true",
                        help="build the oracle and require it in the differential tests")
    parser.add_argument("--oracle-variant", choices=list(build_oracle.VARIANTS),
                        help="run only this oracle variant (default for --oracle: release and "
                             "release-simd); implies --oracle")
    parser.add_argument("--baseline", action="store_true",
                        help="also run tools/oracle_baseline.py; implies --oracle")
    parser.add_argument("--arch", choices=list(build_oracle.CROSS_ARCHES),
                        help="check this architecture on an ARM64 Mac (Rosetta 2)")
    parser.add_argument("--msrv", action="store_true",
                        help="also build and test with the toolchain named by rust-version")
    parser.add_argument("--no-simd", action="store_true",
                        help="test only the scalar translation without the SIMD modules "
                             "(--no-default-features, ADR 0008); with --oracle, only the "
                             "scalar variant")
    args = parser.parse_args()
    oracle_variants = []
    if args.oracle_variant:
        oracle_variants = [args.oracle_variant]
    elif args.oracle or args.baseline:
        oracle_variants = ["release"] if args.no_simd else ["release", "release-simd"]
    if args.no_simd and any(build_oracle.VARIANTS[v].get("simd") for v in oracle_variants):
        raise SystemExit("--no-simd cannot run a SIMD oracle variant")
    if args.stage in ("all", "static"):
        check_static(args.msrv, args.arch)
    else:
        check_provenance()
    if args.stage in ("all", "test"):
        check_tests(oracle_variants, args.baseline, args.msrv, args.no_simd, args.arch)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
