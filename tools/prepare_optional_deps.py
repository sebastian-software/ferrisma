#!/usr/bin/env python3
"""Build the pinned optional native dependencies outside the upstream snapshot."""
import argparse
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def run(command, **kwargs):
    subprocess.run([str(arg) for arg in command], cwd=ROOT, check=True, **kwargs)


def checkout(name, url, revision, tag=None):
    path = ROOT / "build/deps" / name
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        if tag:
            run(["git", "clone", "--depth", "1", "--branch", tag, url, path])
        else:
            run(["git", "init", path])
            run(["git", "-C", path, "remote", "add", "origin", url])
            run(["git", "-C", path, "fetch", "--depth", "1", "origin", revision])
            run(["git", "-C", path, "checkout", "--detach", revision])
    actual = subprocess.check_output(["git", "-C", str(path), "rev-parse", "HEAD"], text=True).strip()
    if actual != revision:
        raise SystemExit(f"Unexpected {name} revision {actual}; expected {revision}")
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--avm", action="store_true", help="Also build the experimental AV2 codec")
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    gav1 = checkout("libgav1", "https://chromium.googlesource.com/codecs/libgav1",
                    "c05bf9be660cf170d7c26bd06bb42b3322180e58", "v0.20.0")
    yuv = checkout("libyuv", "https://chromium.googlesource.com/libyuv/libyuv",
                   "26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0")
    configurations = [
        (gav1, ["-DLIBGAV1_THREADPOOL_USE_STD_MUTEX=1", "-DLIBGAV1_ENABLE_EXAMPLES=OFF",
                "-DLIBGAV1_ENABLE_TESTS=OFF", "-DLIBGAV1_MAX_BITDEPTH=12"], None),
        (yuv, ["-DCMAKE_DISABLE_FIND_PACKAGE_JPEG=TRUE"], "yuv"),
    ]
    if args.avm:
        avm = checkout("avm", "https://github.com/AOMediaCodec/avm.git",
                       "966a7d7cd6fcf60360caf5dc413b2aeeb65e144d", "v1.0.0")
        # AVM v1.0.0's macOS export list still names a removed function. Fix the
        # downloaded dependency's list; do not add a stub or alter libavif.
        exports = avm / "avm/exports_dec"
        original = subprocess.check_output(["git", "-C", str(avm), "show", "HEAD:avm/exports_dec"], text=True)
        corrected = original.replace("text avm_read_obu_header\n", "")
        if exports.read_text() not in (original, corrected):
            raise SystemExit("Unexpected edits to the local AVM export list; preserve and review them")
        exports.write_text(corrected)
        configurations.append((avm, ["-DBUILD_SHARED_LIBS=ON", "-DCONFIG_PIC=1", "-DCONFIG_LIBYUV=0",
                                   "-DCONFIG_WEBM_IO=0", "-DENABLE_DOCS=0", "-DENABLE_EXAMPLES=0",
                                   "-DENABLE_TESTDATA=0", "-DENABLE_TESTS=0", "-DENABLE_TOOLS=0"], "avm"))
    for source, flags, target in configurations:
        build = source.parent / (source.name + "-build")
        run(["cmake", "-S", source, "-B", build, "-G", "Ninja", "-DCMAKE_BUILD_TYPE=Release",
             "-DCMAKE_POSITION_INDEPENDENT_CODE=ON", *flags])
        command = ["cmake", "--build", build, "-j", args.jobs]
        if target:
            command += ["--target", target]
        run(command)


if __name__ == "__main__":
    main()
