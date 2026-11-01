#!/usr/bin/env python3
"""Build the unchanged upstream consumers against C or the Rust archive."""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
DEPENDENCIES = {
    "googletest": ("https://github.com/google/googletest.git", "52eb8108c5bdec04579160ae17225d66034bd723"),
    "libargparse": ("https://github.com/kmurray/libargparse.git", "ee74d1b53bd680748af14e737378de57e2a0a954"),
}


def run(command, **kwargs):
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
    parser.add_argument("--mini", action="store_true", help="Enable upstream experimental reduced headers")
    parser.add_argument("--extended-pixi", action="store_true", help="Enable upstream experimental extended pixel information")
    for feature in ["libgav1", "rav1e", "svt", "libyuv"]:
        parser.add_argument("--" + feature, action="store_true")
    args = parser.parse_args()
    run(["python3", ROOT / "tools/verify_upstream.py"])
    dependencies(args.fetch_deps)
    features = [name for name, enabled in [("mini", args.mini), ("extended-pixi", args.extended_pixi),
                ("libgav1", args.libgav1), ("rav1e", args.rav1e), ("svt", args.svt), ("libyuv", args.libyuv)] if enabled]
    suffix = ("-" + "-".join(features) if features else "") + ("-debug" if args.debug else "")
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
        f"-DCMAKE_PROJECT_libavif_INCLUDE={ROOT}/tools/local-dependencies.cmake",
    ]
    if args.implementation == "port":
        cargo = ["cargo", "build", "--locked"]
        if not args.debug:
            cargo.append("--release")
        if features:
            cargo += ["--features", ",".join(features)]
        run(cargo)
        archive = ROOT / "target" / ("debug" if args.debug else "release") / "libavif.a"
        command += [f"-DLIBAWFRS_ARCHIVE={archive}"]
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
