#!/usr/bin/env python3
"""Build the unchanged upstream consumers against C or the Rust archive."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import shlex

ROOT = Path(__file__).resolve().parents[1]
DEPENDENCIES = {
    "googletest": ("https://github.com/google/googletest.git", "52eb8108c5bdec04579160ae17225d66034bd723"),
    "libargparse": ("https://github.com/kmurray/libargparse.git", "ee74d1b53bd680748af14e737378de57e2a0a954"),
}


def run(command, **kwargs):
    environment = kwargs.setdefault("env", os.environ.copy())
    local = ROOT / "build/deps/pkgconfig"
    if local.is_dir():
        existing = environment.get("PKG_CONFIG_PATH", "")
        environment["PKG_CONFIG_PATH"] = existing + (":" if existing else "") + str(local)
    subprocess.run([str(arg) for arg in command], cwd=ROOT, check=True, **kwargs)


def dependencies(fetch):
    for name, (url, revision) in DEPENDENCIES.items():
        path = ROOT / "build/deps" / name
        if not path.exists():
            if not fetch:
                raise SystemExit(f"Missing {name}; run with --fetch-deps to download the pinned test dependencies")
            path.parent.mkdir(parents=True, exist_ok=True)
            run(["git", "clone", "--no-checkout", url, path])
            run(["git", "-C", path, "checkout", "--detach", revision])
        actual = subprocess.check_output(["git", "-C", str(path), "rev-parse", "HEAD"], text=True).strip()
        if actual != revision:
            raise SystemExit(f"Unexpected {name} revision: {actual}; expected {revision}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("implementation", choices=["baseline", "port"])
    parser.add_argument("--fetch-deps", action="store_true")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--debug", action="store_true", help="Test unoptimized Rust with the same upstream Release configuration")
    parser.add_argument("--asan", action="store_true", help="Instrument the library and original test consumers with AddressSanitizer")
    parser.add_argument("--mini", action="store_true", help="Enable upstream experimental reduced headers")
    parser.add_argument("--extended-pixi", action="store_true", help="Enable upstream experimental extended pixel information")
    for feature in ["libgav1", "rav1e", "svt", "libyuv", "avm"]:
        parser.add_argument("--" + feature, action="store_true")
    args = parser.parse_args()
    run(["python3", ROOT / "tools/verify_upstream.py"])
    dependencies(args.fetch_deps)
    features = [name for name, enabled in [("mini", args.mini), ("extended-pixi", args.extended_pixi),
                ("libgav1", args.libgav1), ("rav1e", args.rav1e), ("svt", args.svt), ("libyuv", args.libyuv), ("avm", args.avm)] if enabled]
    suffix = ("-" + "-".join(features) if features else "") + ("-debug" if args.debug else "") + ("-asan-llvm16" if args.asan else "")
    build = ROOT / "build" / (args.implementation + suffix)
    command = [
        "cmake", "-S", ROOT / "upstream/libavif", "-B", build, "-G", "Ninja",
        "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
        "-DBUILD_SHARED_LIBS=OFF", "-DAVIF_BUILD_TESTS=ON", "-DAVIF_BUILD_APPS=ON",
        "-DAVIF_CODEC_AOM=SYSTEM", "-DAVIF_CODEC_DAV1D=SYSTEM", f"-DAVIF_LIBYUV={'SYSTEM' if args.libyuv else 'OFF'}",
        "-DAVIF_LIBSHARPYUV=SYSTEM", "-DAVIF_LIBXML2=SYSTEM", "-DAVIF_GTEST=LOCAL",
        f"-DFETCHCONTENT_SOURCE_DIR_GOOGLETEST={ROOT}/build/deps/googletest",
        f"-DFETCHCONTENT_SOURCE_DIR_LIBARGPARSE={ROOT}/build/deps/libargparse",
        f"-DAVIF_ENABLE_EXPERIMENTAL_MINI={'ON' if args.mini else 'OFF'}",
        f"-DAVIF_ENABLE_EXPERIMENTAL_EXTENDED_PIXI={'ON' if args.extended_pixi else 'OFF'}",
        f"-DAVIF_CODEC_LIBGAV1={'SYSTEM' if args.libgav1 else 'OFF'}",
        f"-DAVIF_CODEC_RAV1E={'SYSTEM' if args.rav1e else 'OFF'}",
        f"-DAVIF_CODEC_SVT={'SYSTEM' if args.svt else 'OFF'}",
        f"-DAVIF_CODEC_AVM={'SYSTEM' if args.avm else 'OFF'}",
        f"-DCMAKE_PROJECT_libavif_INCLUDE={ROOT}/tools/local-dependencies.cmake",
    ]
    if args.asan:
        prefix = Path(os.environ.get("LIBAVIF_RS_LLVM_PREFIX", str(ROOT / "build/toolchain/llvm@16/16.0.6_1")))
        if not (prefix / "bin/clang").is_file():
            raise SystemExit("ASan parity requires LLVM 16; set LIBAVIF_RS_LLVM_PREFIX to its installation prefix")
        wrappers = ROOT / "build/toolchain/wrappers"
        wrappers.mkdir(parents=True, exist_ok=True)
        for compiler in ["clang", "clang++"]:
            wrapper = wrappers / compiler
            wrapper.write_text("#!/bin/sh\nexport DYLD_LIBRARY_PATH=" + shlex.quote(str(prefix / "lib") + ":/opt/homebrew/lib") +
                               "\nexec " + shlex.quote(str(prefix / "bin" / compiler)) + ' "$@"\n')
            wrapper.chmod(0o755)
        sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip())
        runtime_path = sysroot / "lib/rustlib/aarch64-apple-darwin/lib"
        if not (runtime_path / "librustc-nightly_rt.asan.dylib").is_file():
            raise SystemExit("The selected nightly toolchain has no matching macOS ASan runtime")
        command += ["-DCMAKE_C_FLAGS=-fsanitize=address -fno-omit-frame-pointer",
                    "-DCMAKE_CXX_FLAGS=-fsanitize=address -fno-omit-frame-pointer",
                    f"-DCMAKE_C_COMPILER={wrappers}/clang", f"-DCMAKE_CXX_COMPILER={wrappers}/clang++",
                    "-DCMAKE_EXE_LINKER_FLAGS=-fno-sanitize=address " + shlex.join([
                        "-L" + str(runtime_path), "-lrustc-nightly_rt.asan", "-Wl,-rpath," + str(runtime_path)])]
    if args.implementation == "port":
        cargo = ["cargo", "build", "--locked"]
        if not args.debug:
            cargo.append("--release")
        if features:
            cargo += ["--features", ",".join(features)]
        environment = os.environ.copy()
        target = ROOT / "target"
        if args.asan:
            target = target / "asan"
            environment["CARGO_TARGET_DIR"] = str(target)
            flags = environment.get("CARGO_ENCODED_RUSTFLAGS", "").split("\x1f") if environment.get("CARGO_ENCODED_RUSTFLAGS") else shlex.split(environment.get("RUSTFLAGS", ""))
            flags += ["-Cllvm-args=-fp-contract=fast", "-Zsanitizer=address", "-Zexternal-clangrt", "-Cforce-frame-pointers=yes",
                      "--cfg", "libavif_rs_asan", "-Lnative=" + str(runtime_path),
                      "-ldylib=rustc-nightly_rt.asan", "-Clink-arg=-Wl,-rpath," + str(runtime_path)]
            environment["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(flags)
            environment.pop("RUSTFLAGS", None)
            cargo += ["--target", "aarch64-apple-darwin"]
            target = target / "aarch64-apple-darwin"
        run(cargo, env=environment)
        archive = target / ("debug" if args.debug else "release") / "libavif.a"
        command += [f"-DLIBAVIF_RS_ARCHIVE={archive}"]
    run(command)
    run(["cmake", "--build", build, "-j", args.jobs])
    if args.implementation == "port":
        entries = json.loads((build / "compile_commands.json").read_text())
        library = [entry["file"] for entry in entries if "avif_obj.dir" in entry["command"]]
        if len(library) != 1 or not library[0].endswith("rust_archive_anchor.c"):
            raise SystemExit(f"Unexpected C library implementations linked into Rust tests: {library}")
        print("Confirmed: upstream tests link Rust; no C libavif implementation is compiled")


if __name__ == "__main__":
    main()
