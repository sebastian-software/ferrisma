#!/usr/bin/env python3
"""Build the C differential oracle from the pinned libjpeg-turbo (ADR 0002).

Builds the unmodified upstream/libjpeg-turbo through its own CMake project in
the reference configuration of ADR 0002, then configures tools/oracle, which
compiles every probe under tools/oracle/probes against the static library of
that build. Each variant lands in build/oracle/<variant>/:

  libjpeg-turbo/      upstream build tree: libjpeg.a, djpeg-static,
                      cjpeg-static, jpegtran-static, md5cmp, jconfig.h and
                      upstream's CTest suite (tools/bittests.py runs it)
  probes-build/       CMake tree of tools/oracle
  probes/<wp>/<name>  probe executables
  oracle.json         compiler, flags, configuration defines and paths

Point LIBJPEG_ORACLE_DIR at a variant directory to run the differential tests
against it. Variants:

  release     the reference oracle (ADR 0002)
  asan-ubsan  the same configuration, library, tools and probes instrumented
              with AddressSanitizer and UndefinedBehaviorSanitizer
  simd        WITH_SIMD=1, diagnostic only, for the SIMD packages of ADR 0004;
              on x86-64 it needs NASM or Yasm

upstream/libjpeg-turbo is never modified: CMake writes only below build/.
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
UPSTREAM = ROOT / "upstream/libjpeg-turbo"
PROBES = ROOT / "tools/oracle"

# The reference configuration of ADR 0002, in the order it lists them, without
# CMAKE_BUILD_TYPE and CMAKE_C_FLAGS, which every variant sets itself.
REFERENCE = [
    "-DWITH_SIMD=0", "-DWITH_ARITH_DEC=1", "-DWITH_ARITH_ENC=1",
    "-DWITH_TURBOJPEG=1", "-DENABLE_SHARED=0", "-DENABLE_STATIC=1",
]
REFERENCE_C_FLAGS = ["-ffp-contract=off"]
SANITIZER_FLAGS = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all",
                   "-fno-omit-frame-pointer"]
VARIANTS = {
    "release": {"simd": False, "sanitizers": False, "reference": True},
    "asan-ubsan": {"simd": False, "sanitizers": True, "reference": True},
    "simd": {"simd": True, "sanitizers": False, "reference": False},
}
# jconfig.h defines that the 8-bit decoder's configuration depends on.
REQUIRED_DEFINES = {"C_ARITH_CODING_SUPPORTED": "1", "D_ARITH_CODING_SUPPORTED": "1",
                    "MEM_SRCDST_SUPPORTED": "1", "BITS_IN_JSAMPLE": "8"}


def run(*args, cwd=ROOT):
    print("+ " + " ".join(shlex.quote(str(arg)) for arg in args), flush=True)
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def output(*args):
    return subprocess.run([str(arg) for arg in args], cwd=ROOT, check=True,
                          stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                          universal_newlines=True).stdout


def variant_dir(variant):
    return ROOT / "build/oracle" / variant


def cache_value(build, name):
    match = re.search(rf"^{re.escape(name)}:[A-Z]+=(.*)$",
                      (build / "CMakeCache.txt").read_text(), re.M)
    return match[1] if match else None


def defines(compiler, c_flags, header):
    """Return the macros that a generated header defines on this host.

    The compiler evaluates the header (`-dM -E`), so conditional branches such
    as jconfig.h's _WIN32 block count only where they apply; macros that the
    compiler predefines without the header are left out.
    """
    def macros(source):
        listing = subprocess.run([compiler, *shlex.split(c_flags), "-dM", "-E", "-x", "c", "-"],
                                 input=source, check=True, stdout=subprocess.PIPE,
                                 universal_newlines=True).stdout
        return dict(re.findall(r"^#define (\w+)(?: (.*))?$", listing, re.M))

    builtin = macros("")
    found = macros(f'#include "{header}"\n')
    return {name: value for name, value in sorted(found.items())
            if builtin.get(name) != value}


def is_x86_64():
    return platform.machine().lower() in ("x86_64", "amd64")


def configuration(variant):
    settings = VARIANTS[variant]
    c_flags = list(REFERENCE_C_FLAGS)
    linker_flags = []
    if settings["sanitizers"]:
        c_flags += SANITIZER_FLAGS
        linker_flags += SANITIZER_FLAGS[:1]
    options = list(REFERENCE)
    if settings["simd"]:
        options[0] = "-DWITH_SIMD=1"
        options.append("-DREQUIRE_SIMD=1")
    # Upstream picks the expected floating-point IDCT hashes from the compiler
    # (FLOATTEST8, FLOATTEST12). The oracle is built with -ffp-contract=off, so
    # its C float IDCT matches the no-fp-contract hashes; only the x86-64 SIMD
    # build replaces that IDCT with the SSE code.
    float_test = "sse" if settings["simd"] and is_x86_64() else "no-fp-contract"
    options += [f"-DFLOATTEST8={float_test}", "-DFLOATTEST12=no-fp-contract"]
    return {
        "options": options,
        "c_flags": " ".join(c_flags),
        "linker_flags": " ".join(linker_flags),
        "float_test": float_test,
    }


def build(variant, jobs, fresh):
    if not (UPSTREAM / "src/jpeglib.h").is_file():
        raise SystemExit("Initialize upstream/libjpeg-turbo first: git submodule update --init")
    settings = VARIANTS[variant]
    config = configuration(variant)
    directory = variant_dir(variant)
    if fresh and directory.exists():
        shutil.rmtree(directory)
    upstream_build = directory / "libjpeg-turbo"
    probes_build = directory / "probes-build"

    flags = [f"-DCMAKE_C_FLAGS={config['c_flags']}"]
    if config["linker_flags"]:
        flags.append(f"-DCMAKE_EXE_LINKER_FLAGS={config['linker_flags']}")
    run("cmake", "-S", UPSTREAM, "-B", upstream_build, "-DCMAKE_BUILD_TYPE=Release",
        *config["options"], *flags)
    run("cmake", "--build", upstream_build, "--parallel", jobs)

    run("cmake", "-S", PROBES, "-B", probes_build, "-DCMAKE_BUILD_TYPE=Release",
        f"-DORACLE_UPSTREAM_BUILD={upstream_build}", f"-DORACLE_PROBE_DIR={directory / 'probes'}",
        *flags)
    run("cmake", "--build", probes_build, "--parallel", jobs)

    compiler = cache_value(upstream_build, "CMAKE_C_COMPILER")
    jconfig = defines(compiler, config["c_flags"], upstream_build / "jconfig.h")
    jconfigint = defines(compiler, config["c_flags"], upstream_build / "jconfigint.h")
    for name, value in REQUIRED_DEFINES.items():
        if jconfig.get(name) != value:
            raise SystemExit(f"{variant}: jconfig.h defines {name}={jconfig.get(name)!r}, "
                             f"expected {value}")
    if ("WITH_SIMD" in jconfig) != settings["simd"]:
        raise SystemExit(f"{variant}: jconfig.h WITH_SIMD is {jconfig.get('WITH_SIMD')!r}, "
                         f"expected {'1' if settings['simd'] else 'undefined'}")

    probes = sorted(str(path.relative_to(directory / "probes"))
                    for path in (directory / "probes").glob("*/*")
                    if path.is_file() and os.access(path, os.X_OK))
    manifest = {
        "variant": variant,
        "reference": settings["reference"],
        "upstream_revision": output("git", "-C", UPSTREAM, "rev-parse", "HEAD").strip(),
        "host": f"{platform.system()} {platform.release()} {platform.machine()}",
        "cmake": output("cmake", "--version").splitlines()[0],
        "c_compiler": compiler,
        "c_compiler_version": output(compiler, "--version").splitlines()[0],
        "build_type": "Release",
        "cmake_options": ["-DCMAKE_BUILD_TYPE=Release", *config["options"], *flags],
        "c_flags": cache_value(upstream_build, "CMAKE_C_FLAGS"),
        "c_flags_release": cache_value(upstream_build, "CMAKE_C_FLAGS_RELEASE"),
        "exe_linker_flags": cache_value(upstream_build, "CMAKE_EXE_LINKER_FLAGS"),
        "sanitizers": "address,undefined" if settings["sanitizers"] else None,
        "simd": settings["simd"],
        "floattest8": cache_value(upstream_build, "FLOATTEST8"),
        "floattest12": cache_value(upstream_build, "FLOATTEST12"),
        "jconfig_h": jconfig,
        "jconfigint_h": jconfigint,
        "library": str(upstream_build / "libjpeg.a"),
        "tools": {name: str(upstream_build / f"{name}-static")
                  for name in ("djpeg", "cjpeg", "jpegtran")},
        "probes": probes,
    }
    (directory / "oracle.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Oracle {variant}: {len(probes)} probe(s); LIBJPEG_ORACLE_DIR={directory}", flush=True)
    return directory


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", choices=[*VARIANTS, "reference"], action="append",
                        help="variant to build (repeatable; default: release; "
                             "'reference' builds release and asan-ubsan)")
    parser.add_argument("--jobs", default=str(os.cpu_count() or 4), help="parallel build jobs")
    parser.add_argument("--fresh", action="store_true", help="delete the variant directory first")
    args = parser.parse_args()
    variants = []
    for variant in args.variant or ["release"]:
        names = [name for name, settings in VARIANTS.items() if settings["reference"]] \
            if variant == "reference" else [variant]
        variants += [name for name in names if name not in variants]
    for variant in variants:
        build(variant, args.jobs, args.fresh)


if __name__ == "__main__":
    sys.exit(main())
