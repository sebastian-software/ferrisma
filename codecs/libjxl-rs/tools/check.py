#!/usr/bin/env python3
"""Validate the pinned source, Rust port, and original C++ differential oracle."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

from brotli_check_support import prepare as prepare_brotli_observers

ROOT = Path(__file__).resolve().parent.parent


def run(*args, env=None):
    print("+ " + " ".join(map(str, args)), flush=True)
    subprocess.run(list(map(str, args)), cwd=ROOT, env=env, check=True)


def git(*args):
    return subprocess.check_output(["git", *map(str, args)], cwd=ROOT, text=True).strip()


def check_codegen():
    # The gate builds what ships (ADR 0002, 2026-11-16): the engine and every
    # dependent build this crate from the workspace root, where a Cargo
    # configuration in this directory does not apply. Rustflags here would
    # make the gate compile code that nobody ships.
    for name in ("config.toml", "config"):
        if (ROOT / ".cargo" / name).exists():
            raise SystemExit(f".cargo/{name}: the port takes no Cargo configuration of its own; "
                             "the gate must build with the code generation that ships")
    for name in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS"):
        if os.environ.get(name):
            print(f"Note: {name} is set; this run does not build what ships", flush=True)


def check_source():
    check_codegen()
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


def lint_toolchain():
    # The one Rust version Clippy runs at, in CI and locally: tools/lint-toolchain
    # at the repository root (ADR-0027, amendment 2026-11-15). Raise it only as a
    # deliberate change together with the lint fixes it needs.
    return (ROOT.parent.parent / "tools" / "lint-toolchain").read_text().strip()


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
    run("cargo", f"+{lint_toolchain()}", "clippy", "--locked", "--offline", "--all-targets", "--all-features", "--", "-D", "warnings")
    if msrv:
        run("cargo", f"+{rust_version()}", "check", "--locked", "--offline", "--all-targets", "--all-features")
    run("cargo", "build", "--locked", "--offline", "--release", "--all-targets", "--all-features")
    for name in release_only_tests():
        run("cargo", "test", "--locked", "--offline", "--release", "--all-features", "--test", name)
    # codegen check
    check_attribute_context()
    # end codegen check


# codegen check: one attribute context per kernel on x86-64 (ADR 0004).
# Port functions that call a core::arch x86 intrinsic out of line (or libm's
# fmaf) run their vector operations outside the AVX2 attribute context. The
# port's skcms engines (jxl::skcms) are held to zero too since the HSW
# helpers were brought into the engine's context (ADR 0004, "The skcms HSW
# engine in its attribute context").
CODEGEN_EXAMPLES = ["encode_lossy", "encode_lossless", "conformance_decode"]


def check_attribute_context():
    import platform
    machine = platform.machine()
    if machine in ("x86_64", "AMD64"):
        target = None
    elif platform.system() == "Darwin":
        target = "x86_64-apple-darwin"
    else:
        print("Attribute-context check skipped: no x86-64 build on this host", flush=True)
        return
    build = ["cargo", "build", "--locked", "--offline", "--release", "-p", "libjxl-rs"]
    for name in CODEGEN_EXAMPLES:
        build += ["--example", name]
    if target is not None:
        build += ["--target", target]
    run(*build)
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps", "--offline"], cwd=ROOT, text=True))
    out = Path(metadata["target_directory"]) / (target or "") / "release" / "examples"
    failures = []
    for name in CODEGEN_EXAMPLES:
        disassembly = subprocess.check_output(
            ["objdump", "-d", "--no-show-raw-insn", "-C", str(out / name)], text=True)
        counts = {}
        function = ""
        for line in disassembly.splitlines():
            header = re.match(r"^[0-9a-f]+ <(.*)>:$", line)
            if header:
                function = header[1]
                continue
            if not re.search(r"\bcall", line):
                continue
            if "core_arch::x86::" in line or re.search(r"\bfmaf\b", line):
                module = "::".join(function.lstrip("<").split("::")[:3])
                counts[module] = counts.get(module, 0) + 1
        port = {m: n for m, n in counts.items() if m.startswith(("jxl::jxl::", "jxl::skcms::"))}
        print(f"{name}: out-of-line x86 intrinsic and fmaf calls per module: {counts or 'none'}", flush=True)
        if port:
            failures.append(f"{name}: {port}")
    if failures:
        raise SystemExit("Kernel code outside the AVX2 attribute context (port ADR 0004, "
                         "\"One attribute context per kernel on x86-64\"): " + "; ".join(failures))
# end codegen check


def cross_processor(arch, build):
    """CMake arguments that make an Apple cross build see the target processor.

    O9 (owner decision 2026-11-15, port ADR 0004): upstream's
    third_party/skcms.cmake compiles skcms's HSW (and SKX) engine only when
    CMAKE_SYSTEM_PROCESSOR is x86_64. On an ARM64 Mac CMake sets it to the
    host's arm64 and ignores a -DCMAKE_SYSTEM_PROCESSOR given without a
    CMAKE_SYSTEM_NAME (it is a cross-compilation setting), so the cross
    build left both engines out. CMAKE_APPLE_SILICON_PROCESSOR is CMake's
    own switch for this case: it makes the host processor, and therefore
    CMAKE_SYSTEM_PROCESSOR, x86_64 without entering cross-compiling mode,
    as on a native x86-64 Mac. The original's source is untouched. CMake
    records the processor once per build directory, so a directory
    configured before is configured afresh.
    """
    for recorded in build.glob("CMakeFiles/*/CMakeSystem.cmake"):
        if f'set(CMAKE_SYSTEM_PROCESSOR "{arch}")' not in recorded.read_text():
            shutil.rmtree(build / "CMakeFiles")
            (build / "CMakeCache.txt").unlink(missing_ok=True)
            break
    return [f"-DCMAKE_APPLE_SILICON_PROCESSOR={arch}"]


# --toolchain: the Rust toolchain of the oracle and test stages (port ADR 0002,
# "Results that depend on the Rust compiler", 2026-11-15). None keeps
# rust-toolchain.toml's pinned nightly and Cargo's default target directory.
# Another toolchain runs every Cargo command of those stages, including the
# Rust library that the oracle's jxl_abi_rust links, with RUSTUP_TOOLCHAIN in
# target/rust-<toolchain>, so its builds never replace the pinned nightly's.
RUST_TOOLCHAIN = None


def rust_target_dir():
    return ROOT / "target" / f"rust-{RUST_TOOLCHAIN}" if RUST_TOOLCHAIN else ROOT / "target"


def use_toolchain(name):
    global RUST_TOOLCHAIN
    RUST_TOOLCHAIN = name
    os.environ["RUSTUP_TOOLCHAIN"] = name
    os.environ["CARGO_TARGET_DIR"] = str(rust_target_dir())
    version = subprocess.check_output(["rustc", "-vV"], text=True)
    print(f"Rust toolchain {name}: " + "; ".join(line for line in version.splitlines()
                                                 if line.startswith(("release:", "LLVM version:"))), flush=True)


def cross_arguments(target, build):
    """CMake arguments for an oracle of another Apple architecture (run under Rosetta)."""
    if target is None:
        return []
    arch = {"x86_64-apple-darwin": "x86_64"}[target]
    # Compiler wrappers also carry -arch into the helper links that
    # tools/*_reference.py derive from compile_commands.json.
    build.mkdir(parents=True, exist_ok=True)
    wrappers = []
    for name in ("clang", "clang++"):
        wrapper = build / f"{arch}-{name}"
        compiler = subprocess.check_output(["xcrun", "-f", name], text=True).strip()
        wrapper.write_text(f'#!/bin/sh\ncase " $* " in *" -arch "*) exec "{compiler}" "$@";; esac\n'
                           f'exec "{compiler}" -arch {arch} "$@"\n')
        wrapper.chmod(0o755)
        wrappers.append(wrapper)
    return [f"-DCMAKE_OSX_ARCHITECTURES={arch}", *cross_processor(arch, build), f"-DORACLE_RUST_TARGET={target}",
            f"-DCMAKE_C_COMPILER={wrappers[0]}", f"-DCMAKE_CXX_COMPILER={wrappers[1]}"]


def build_reference_variant(target):
    """The second scalar reference of D8/O1: the oracle without its per-TU numeric pins."""
    suffix = "" if target is None else "-" + target.split("-")[0]
    build = ROOT / "target" / f"oracle-o3{suffix}"
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release",
        "-DORACLE_SANITIZERS=OFF", "-DORACLE_REFERENCE_PINS=OFF", *cross_arguments(target, build),
        f"-DORACLE_RUST_TARGET_DIR={rust_target_dir()}")
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    return build / "jxl_oracle"


def build_oracle(sanitizers, target=None):
    suffix = "" if target is None else "-" + target.split("-")[0]
    build = ROOT / "target" / (("oracle-asan" if sanitizers else "oracle") + suffix)
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release",
        f"-DORACLE_SANITIZERS={'ON' if sanitizers else 'OFF'}", "-DORACLE_REFERENCE_PINS=ON",
        *cross_arguments(target, build), f"-DORACLE_RUST_TARGET_DIR={rust_target_dir()}")
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    variant = build_reference_variant(target)
    from skcms_fitting_reference import build_private as build_fitting_private
    from skcms_parser_reference import build_private as build_parser_private
    from skcms_transform_reference import build_private as build_transform_private
    from default_cms_reference import build_contexts as build_default_cms_contexts
    transform_helper = build_transform_private(build)
    helper = build_fitting_private(build)
    build_parser_private(build)
    default_cms_public, default_cms_private = build_default_cms_contexts(build)
    env = dict(os.environ, JXL_CPP_ORACLE=str(build / "jxl_oracle"),
               JXL_CPP_ORACLE_O3=str(variant),
               JXL_SKCMS_FITTING_ORACLE=str(helper),
               JXL_SKCMS_TRANSFORM_ORACLE=str(transform_helper),
               JXL_DEFAULT_CMS_ORACLE=str(default_cms_public),
               JXL_DEFAULT_CMS_PRIVATE_ORACLE=str(default_cms_private),
               JXL_DEFAULT_CMS_CAPTURE=str(build / "default-cms-corpus"),
               JXL_DECODER_PUBLIC_CAPTURE=str(build / "decoder-public-corpus"))
    env.update(prepare_brotli_observers(sanitizers, None if target is None else target.split("-")[0]))
    return env


def build_simd_oracle(rust_target=None):
    # The NEON (AArch64) or AVX2 (x86-64) variant of the original for the
    # SIMD paths (ADR 0004); the SIMD_TESTS binaries read JXL_CPP_ORACLE_SIMD.
    # x86-64 under Rosetta 2 reports AVX2 only with ROSETTA_ADVERTISE_AVX=1;
    # check_tests sets it for the SIMD_TESTS binaries alone.
    import platform
    machine = "x86_64" if rust_target and rust_target.startswith("x86_64") else platform.machine()
    target = {"arm64": "NEON", "aarch64": "NEON", "x86_64": "AVX2", "AMD64": "AVX2"}.get(machine)
    if target is None:
        raise SystemExit(f"No SIMD oracle target for {machine}")
    build = ROOT / "target" / f"oracle-{target.lower()}"
    cross = (["-DCMAKE_OSX_ARCHITECTURES=x86_64", *cross_processor("x86_64", build)]
             if machine == "x86_64" and platform.machine() != "x86_64" else [])
    run("cmake", "-S", "tools/oracle", "-B", build, "-DCMAKE_BUILD_TYPE=Release", f"-DORACLE_HWY_TARGET={target}", *cross)
    run("cmake", "--build", build, "--target", "jxl_oracle", "--parallel", "4")
    return {"JXL_CPP_ORACLE_SIMD": str(build / "jxl_oracle")}


# The SIMD parity binaries (ADR 0004), which need the SIMD oracle and, under
# Rosetta 2, ROSETTA_ADVERTISE_AVX.
SIMD_TESTS = ["simd_parity", "simd_render", "simd_vardct"]
# public gate: whole decodes through the public decoder (ADR 0004 condition (b)).
SIMD_TESTS += ["simd_public"]
# end public gate
# modular
SIMD_TESTS += ["simd_modular"]
# end modular
# stages2
SIMD_TESTS += ["simd_stages2"]
# end stages2
# color
SIMD_TESTS += ["simd_color"]
# end color
# threads: worker-count independence of whole public decodes (ADR 0003).
SIMD_TESTS += ["simd_threads"]
# end threads
# encoder simd: the encoder's SIMD kernels (ADR 0004, "Encoder path").
SIMD_TESTS += ["simd_encoder"]
# end encoder simd
# encoder acs: the encoder's AC strategy search (ADR 0004, "Encoder path").
SIMD_TESTS += ["simd_encoder_acs"]
# end encoder acs
# encoder cfl: the encoder's chroma-from-luma SIMD kernels (ADR 0004, "Encoder path").
SIMD_TESTS += ["simd_encoder_cfl"]
# end encoder cfl
# encoder entropy: the encoder's smaller SIMD exports (ADR 0004, "Encoder path").
SIMD_TESTS += ["simd_encoder_entropy"]
# end encoder entropy
# encoder aq: the SIMD AdaptiveQuantizationMap (ADR 0004, "Encoder path").
SIMD_TESTS += ["simd_encoder_aq"]
# end encoder aq
# encoder selection: whole encodes with the encoder's SIMD target forced (ADR 0004, "Encoder selection").
SIMD_TESTS += ["simd_encoder_public"]
# end encoder selection
SIMD_TEST_ARGS = [argument for name in SIMD_TESTS for argument in ("--test", name)]


def check_tests(env, partition, filter, target=None, keep_going=False):
    # public gate: ADR 0004 "Decoder path" condition (c). The scalar gate
    # forces the scalar decoder path, whatever hwy::DECODER_SELECTS_SIMD
    # says; the oracle test builds read it (hwy::scalar_forced_by_environment)
    # and only an explicit set_decoder_simd_for_test, as in the SIMD
    # binaries, selects a SIMD target.
    env = dict(env, JXL_DECODER_TARGET="scalar")
    # end public gate
    # encoder selection: ADR 0004 "Encoder path" condition (d). The scalar
    # gate forces the scalar encoder path, whatever hwy::ENCODER_SELECTS_SIMD
    # says (hwy::encoder_scalar_forced_by_environment); the SIMD binaries
    # select their target with set_encoder_simd_for_test, which wins.
    env = dict(env, JXL_ENCODER_TARGET="scalar")
    # end encoder selection
    test = ["cargo", "test", "--locked", "--offline", "--profile", "checked", "--all-features"]
    if target is not None:
        test += ["--target", target]
    if keep_going:
        test.append("--no-fail-fast")

    def listed(*selection):
        print(f"+ {' '.join([*test, *selection])} -- --list --format terse", flush=True)
        listing = subprocess.run([*test, *selection, "--", "--list", "--format", "terse"], cwd=ROOT, env=env,
                                 stdout=subprocess.PIPE, text=True, check=True).stdout
        return sorted({line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")})

    # The SIMD parity binaries (SIMD_TESTS) run as their own invocation, with
    # --no-fail-fast, after the other tests and even when those fail: cargo
    # otherwise stops at the first failing test binary, and the later SIMD
    # binaries would report nothing (hosted AVX2 run of 2026-11-14). The
    # stage still fails when any test fails. A test of the same name in
    # another binary would move with the SIMD group.
    # Under Rosetta 2 the SIMD parity tests also need AVX2 reported to their
    # own process; every other test runs without it, so port and original
    # both select skcms's baseline engine there. The cross-built references
    # contain the HSW engine since O9 (cross_processor), so setting
    # ROSETTA_ADVERTISE_AVX=1 for the whole stage compares HSW with HSW as a
    # native x86-64 run does; it is not the default because Rosetta 2
    # mis-executes some AVX code (port ADR 0004, "The skcms HSW engine in its
    # attribute context").
    simd = listed(*SIMD_TEST_ARGS)
    simd_env = dict(env, ROSETTA_ADVERTISE_AVX="1")
    simd_test = test if keep_going else [*test, "--no-fail-fast"]
    skip = [argument for name in simd for argument in ("--skip", name)]
    failures = []

    def attempt(*args, env):
        try:
            run(*args, env=env)
        except subprocess.CalledProcessError as error:
            failures.append(error)

    if filter is not None:
        attempt(*test, "--", "--nocapture", filter, *skip, env=env)
        if any(filter in name for name in simd):
            attempt(*simd_test, *SIMD_TEST_ARGS, "--", "--nocapture", filter, env=simd_env)
    elif partition is None:
        # --exact makes each --skip match one whole test name.
        attempt(*test, "--", "--nocapture", "--exact", *skip, env=env)
        if simd:
            attempt(*simd_test, *SIMD_TEST_ARGS, "--", "--nocapture", env=simd_env)
    else:
        index, count = partition
        # Every listed name belongs to exactly one partition. A name shared by
        # several test binaries runs in each of them within that partition.
        names = listed()
        selected = partition_names(names, index, count)
        print(f"Partition {index}/{count}: {len(selected)} of {len(names)} test names", flush=True)
        scalar_selected = [name for name in selected if name not in simd]
        simd_selected = [name for name in selected if name in simd]
        if scalar_selected:
            attempt(*test, "--", "--nocapture", "--exact", *scalar_selected, env=env)
        if simd_selected:
            attempt(*simd_test, *SIMD_TEST_ARGS, "--", "--nocapture", "--exact", *simd_selected, env=simd_env)
    if failures:
        raise SystemExit(f"{len(failures)} test invocation(s) failed: "
                         + "; ".join(f"exit {error.returncode}" for error in failures))


# The tests that take most of a shard's time: whole encodes of large images
# against the original (the encoder's gate sizes up to 4000x3000 and their
# streaming, output-processor and container variants), and the SIMD
# whole-encode gates, whose binary runs its tests one at a time. Heaviest
# first, from the hosted runs of 2026-11-16 (ordinary and sanitized oracle).
# They are dealt out before the other tests, so that each partition gets its
# share: a round robin over the sorted names put three of the largest into
# one shard, which then ran past the job's time limit. The list only orders
# the partitions; every listed test still runs in exactly one of them, and a
# name that no test has any more fails the stage, so the list stays current.
HEAVY_TESTS = [
    "enc_public_streaming_gate_4000x3000_matches_the_original_target",
    "encoder_public::encoder_container::public_encoder_output_mode_2_gate_4000x3000_matches_original",
    "enc_public_lossy_small_large_and_settings_match_the_original_target",
    "encoder_public::public_encoder_streaming_gate_4000x3000_d1_matches_original",
    "encoder_public::public_encoder_streaming_gate_4000x3000_d2_matches_original",
    "encoder_public::encoder_container::public_encoder_output_mode_2_gate_2049_matches_original",
    "encoder_public::public_encoder_streaming_gate_2048x1536_matches_original",
    "encoder_public::encoder_container::public_encoder_output_mode_2_gate_2048x1536_matches_original",
    "encoder_public::public_encoder_output_processor_matches_original",
    "encoder_public::public_encoder_streaming_gate_2049_d1_matches_original",
    "encoder_public::public_encoder_streaming_gate_2049_d2_matches_original",
    "encoder_public::public_encoder_lossy_vardct_streaming_matches_original",
    "encoder_public::encoder_container::public_encoder_streaming_suite_container_instances_match_original",
    "encoder_public::public_encoder_streaming_gate_1024_matches_original",
    "enc_public_streaming_gate_sizes_match_the_original_target",
    "encoder_public::encoder_container::public_encoder_container_output_processor_matches_original",
    "encoder_public::encoder_lossless::public_encoder_lossless_gate_4000x3000_matches_original",
    "encoder_public::encoder_container::public_encoder_output_mode_2_gate_1024_matches_original",
    "encoder_public::public_encoder_lossy_vardct_large_sizes_match_original",
    "encoder_public::public_encoder_streaming_gate_768_matches_original",
]


def partition_names(names, index, count):
    """The names of partition `index` of `count`: HEAVY_TESTS in their order,
    then the other names, dealt out in alternating direction (1..count,
    count..1, ...)."""
    missing = [name for name in HEAVY_TESTS if name not in names]
    if missing:
        raise SystemExit(f"HEAVY_TESTS names tests that do not exist: {missing}")
    order = [*HEAVY_TESTS, *(name for name in names if name not in HEAVY_TESTS)]
    selected = []
    for position, name in enumerate(order):
        slot = position % count
        if (position // count) % 2:
            slot = count - 1 - slot
        if slot == index - 1:
            selected.append(name)
    return selected


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
    parser.add_argument("--target", choices=["x86_64-apple-darwin"],
                        help="oracle and test stages only: build both scalar references for this Apple "
                             "architecture, with the Brotli observers, and run the tests for it "
                             "(under Rosetta 2 on an ARM64 host)")
    parser.add_argument("--toolchain", metavar="NAME",
                        help="oracle and test stages only: build and run the Rust side with this installed "
                             "rustup toolchain (for example 1.100.0 or stable) instead of the pinned nightly, "
                             "in target/rust-NAME; the C++ references do not depend on it")
    parser.add_argument("--no-fail-fast", action="store_true",
                        help="test stage: run every test binary even after a failure (diagnosis)")
    args = parser.parse_args()
    if args.partition is not None and args.filter is not None:
        parser.error("--partition and --filter are mutually exclusive")
    if args.target is not None and args.stage in ("all", "static"):
        parser.error("--target applies to the oracle and test stages")
    if args.toolchain is not None and args.stage in ("all", "static"):
        parser.error("--toolchain applies to the oracle and test stages")
    check_source()
    if args.toolchain is not None:
        use_toolchain(args.toolchain)
    if args.stage in ("all", "static"):
        check_static(args.msrv)
    if args.stage in ("all", "oracle", "test"):
        env = build_oracle(args.sanitizers, args.target)
        # The SIMD variant has no sanitized twin (ADR 0004).
        env.update(build_simd_oracle(args.target))
    if args.stage in ("all", "test"):
        check_tests(env, args.partition, args.filter, args.target, args.no_fail_fast)
    run("git", "diff", "--check")
    run("git", "diff", "--cached", "--check")


if __name__ == "__main__":
    main()
