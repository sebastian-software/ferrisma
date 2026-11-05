#!/usr/bin/env python3
"""Build the C differential oracle from the pinned libwebp (ADR 0002).

Configures tools/oracle/CMakeLists.txt, which builds upstream/libwebp through
its own CMake project in the reference configuration of docs/PORTING.md, the
upstream tools dwebp, cwebp and webpinfo, and every probe under
tools/oracle/probes. Each variant lands in build/oracle/<variant>/:

  libwebp/                 upstream build tree (libraries, dwebp, cwebp, webpinfo)
  probes/<wp>/<name>       probe executables
  oracle.json              compiler, flags, configuration defines and paths

Point LIBWEBP_ORACLE_DIR at a variant directory to run the differential tests
against it. Variants: release (the reference), asan-ubsan (the same build
instrumented with AddressSanitizer and UndefinedBehaviorSanitizer) and debug
(assertions enabled).
"""

import argparse
import json
import os
from pathlib import Path
import platform
import re
import shlex
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "tools/oracle"
VARIANTS = {
    "release": {"build_type": "Release", "sanitizers": False},
    "asan-ubsan": {"build_type": "Release", "sanitizers": True},
    "debug": {"build_type": "Debug", "sanitizers": False},
}
# The reference configuration from the port plan, in the order it lists them.
REFERENCE = [
    "-DWEBP_ENABLE_SIMD=OFF", "-DWEBP_USE_THREAD=ON", "-DWEBP_NEAR_LOSSLESS=ON",
    "-DWEBP_ENABLE_SWAP_16BIT_CSP=OFF", "-DWEBP_BUILD_DWEBP=ON", "-DWEBP_BUILD_CWEBP=ON",
    "-DWEBP_BUILD_WEBPINFO=ON", "-DWEBP_BUILD_LIBWEBPMUX=ON", "-DWEBP_BUILD_ANIM_UTILS=OFF",
    "-DWEBP_BUILD_GIF2WEBP=OFF", "-DWEBP_BUILD_IMG2WEBP=OFF", "-DWEBP_BUILD_VWEBP=OFF",
    "-DWEBP_BUILD_WEBPMUX=OFF", "-DWEBP_BUILD_EXTRAS=OFF", "-DCMAKE_C_FLAGS=-ffp-contract=off",
]
# config.h defines that would mean SIMD code was compiled into the oracle.
SIMD_DEFINES = re.compile(r"#define WEBP_HAVE_(SSE2|SSE41|AVX2|NEON|NEON_RTCD|MIPS32|MIPS_DSP_R2|MSA)\b")


def run(*args, cwd=ROOT):
    print("+ " + " ".join(shlex.quote(str(arg)) for arg in args), flush=True)
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def output(*args):
    return subprocess.run([str(arg) for arg in args], cwd=ROOT, check=True,
                          stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True).stdout


def variant_dir(variant):
    return ROOT / "build/oracle" / variant


def cache_value(build, name):
    match = re.search(rf"^{name}:[A-Z]+=(.*)$", (build / "CMakeCache.txt").read_text(), re.M)
    return match[1] if match else None


def build(variant, jobs, fresh):
    settings = VARIANTS[variant]
    directory = variant_dir(variant)
    if fresh and directory.exists():
        shutil.rmtree(directory)
    configure = ["cmake", "-S", SOURCE, "-B", directory,
                 f"-DCMAKE_BUILD_TYPE={settings['build_type']}", *REFERENCE,
                 f"-DORACLE_SANITIZERS={'ON' if settings['sanitizers'] else 'OFF'}"]
    run(*configure)
    run("cmake", "--build", directory, "--target", "oracle_tools", "oracle_probes",
        "--parallel", jobs)

    config = (directory / "libwebp/src/webp/config.h").read_text()
    defines = sorted(set(re.findall(r"^#define (\w+)", config, re.M)))
    simd = SIMD_DEFINES.findall(config)
    if simd:
        raise SystemExit(f"{variant}: the oracle was built with SIMD enabled: {simd}")
    flags = dict(line.split("=", 1) for line in
                 (directory / "upstream-flags.txt").read_text().splitlines() if "=" in line)
    probes = sorted(str(path.relative_to(directory / "probes"))
                    for path in (directory / "probes").glob("*/*") if os.access(path, os.X_OK))
    compiler = cache_value(directory, "CMAKE_C_COMPILER")
    manifest = {
        "variant": variant,
        "build_type": settings["build_type"],
        "sanitizers": "address,undefined" if settings["sanitizers"] else None,
        "upstream_revision": output("git", "-C", "upstream/libwebp", "rev-parse", "HEAD").strip(),
        "host": f"{platform.system()} {platform.release()} {platform.machine()}",
        "cmake": output("cmake", "--version").splitlines()[0],
        "c_compiler": compiler,
        "c_compiler_version": output(compiler, "--version").splitlines()[0],
        "reference_options": REFERENCE,
        "upstream_c_flags": flags.get("c_flags", "").strip(),
        "upstream_c_flags_build_type": cache_value(directory, f"CMAKE_C_FLAGS_{settings['build_type'].upper()}"),
        "upstream_definitions": flags.get("definitions", ""),
        "config_h_defines": defines,
        "tools": {name: str(directory / "libwebp" / name) for name in ("dwebp", "cwebp", "webpinfo")},
        "probes": probes,
    }
    (directory / "oracle.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Oracle {variant}: {len(probes)} probe(s); LIBWEBP_ORACLE_DIR={directory}", flush=True)
    return directory


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", choices=[*VARIANTS, "all"], action="append",
                        help="variant to build (repeatable; default: release)")
    parser.add_argument("--jobs", default=str(os.cpu_count() or 4), help="parallel build jobs")
    parser.add_argument("--fresh", action="store_true", help="delete the variant directory first")
    args = parser.parse_args()
    variants = args.variant or ["release"]
    if "all" in variants:
        variants = list(VARIANTS)
    for variant in variants:
        build(variant, args.jobs, args.fresh)


if __name__ == "__main__":
    sys.exit(main())
