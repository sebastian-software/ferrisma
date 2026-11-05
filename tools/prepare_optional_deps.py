#!/usr/bin/env python3
"""Build the pinned optional native dependencies outside the upstream snapshot."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
DEPS = ROOT / "build/deps"
# rav1e's C API is built with cargo-c from the pinned source revision.
CARGO_C_VERSION = "0.10.25+cargo-0.99.0"


def run(command, **kwargs):
    subprocess.run([str(arg) for arg in command], cwd=kwargs.pop("cwd", ROOT), check=True, **kwargs)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def checkout(name, url, revision, tag=None):
    path = DEPS / name
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


def install_pkgconfig(prefix, name):
    """Expose a task-local installation through build/deps/pkgconfig only."""
    source = prefix / "lib/pkgconfig" / f"{name}.pc"
    destination = DEPS / "pkgconfig" / f"{name}.pc"
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, destination)
    return destination


def build_rav1e(jobs):
    source = checkout("rav1e", "https://github.com/xiph/rav1e.git",
                      "1fe82de02510767539e89b2ee6fa846920ae2686", "v0.8.1")
    tools = DEPS / "cargo-c"
    if not (tools / "bin/cargo-cinstall").is_file():
        run(["cargo", "install", "cargo-c", "--version", CARGO_C_VERSION, "--locked", "--root", tools, "-j", jobs])
    prefix = DEPS / "rav1e-install"
    environment = os.environ.copy()
    environment["PATH"] = str(tools / "bin") + os.pathsep + environment["PATH"]
    # The default rav1e feature set, including its original assembly, is the
    # external engine; only the C API's library type is selected here.
    run(["cargo", "cinstall", "--release", "--locked", "--library-type", "cdylib", "--prefix", prefix,
         "--libdir", "lib", "-j", jobs], cwd=source, env=environment)
    return dict(source="https://github.com/xiph/rav1e.git", tag="v0.8.1",
                revision="1fe82de02510767539e89b2ee6fa846920ae2686", cargo_c=CARGO_C_VERSION,
                library_type="shared", prefix=prefix, pkgconfig=install_pkgconfig(prefix, "rav1e"))


def build_svt(jobs):
    source = checkout("svt-av1", "https://gitlab.com/AOMediaCodec/SVT-AV1.git",
                      "9292ec8e32bce26f781f277ec8739b53426c4300", "v4.2.0")
    build, prefix = DEPS / "svt-av1-build", DEPS / "svt-av1-install"
    run(["cmake", "-S", source, "-B", build, "-G", "Ninja", "-DCMAKE_BUILD_TYPE=Release",
         "-DBUILD_SHARED_LIBS=ON", "-DBUILD_APPS=OFF", "-DBUILD_TESTING=OFF",
         f"-DCMAKE_INSTALL_PREFIX={prefix}", "-DCMAKE_INSTALL_LIBDIR=lib"])
    run(["cmake", "--build", build, "-j", jobs])
    run(["cmake", "--install", build])
    return dict(source="https://gitlab.com/AOMediaCodec/SVT-AV1.git", tag="v4.2.0",
                revision="9292ec8e32bce26f781f277ec8739b53426c4300", library_type="shared",
                prefix=prefix, pkgconfig=install_pkgconfig(prefix, "SvtAv1Enc"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--avm", action="store_true", help="Also build the experimental AV2 codec")
    for name in ["libgav1", "libyuv", "rav1e", "svt"]:
        parser.add_argument("--" + name, action="store_true", help=f"Build only the selected engines, including {name}")
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    selected = [name for name in ["libgav1", "libyuv", "rav1e", "svt"] if getattr(args, name)]
    if not selected:
        # The historical default prepares libgav1 and libyuv; rav1e and
        # SVT-AV1 come from pkg-config unless explicitly selected.
        selected = ["libgav1", "libyuv"]
    configurations, records = [], {}
    if "libgav1" in selected:
        gav1 = checkout("libgav1", "https://chromium.googlesource.com/codecs/libgav1",
                        "c05bf9be660cf170d7c26bd06bb42b3322180e58", "v0.20.0")
        configurations.append(("libgav1", gav1, ["-DLIBGAV1_THREADPOOL_USE_STD_MUTEX=1", "-DLIBGAV1_ENABLE_EXAMPLES=OFF",
                               "-DLIBGAV1_ENABLE_TESTS=OFF", "-DLIBGAV1_MAX_BITDEPTH=12"], None, "libgav1.a"))
    if "libyuv" in selected:
        yuv = checkout("libyuv", "https://chromium.googlesource.com/libyuv/libyuv",
                       "26e56be0f984af3dae4d0c0ab7a0bac8ac20e1b0")
        configurations.append(("libyuv", yuv, ["-DCMAKE_DISABLE_FIND_PACKAGE_JPEG=TRUE"], "yuv", "libyuv.a"))
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
        configurations.append(("avm", avm, ["-DBUILD_SHARED_LIBS=ON", "-DCONFIG_PIC=1", "-DCONFIG_LIBYUV=0",
                                            "-DCONFIG_WEBM_IO=0", "-DENABLE_DOCS=0", "-DENABLE_EXAMPLES=0",
                                            "-DENABLE_TESTDATA=0", "-DENABLE_TESTS=0", "-DENABLE_TOOLS=0"], "avm", None))
    for name, source, flags, target, library in configurations:
        build = source.parent / (source.name + "-build")
        run(["cmake", "-S", source, "-B", build, "-G", "Ninja", "-DCMAKE_BUILD_TYPE=Release",
             "-DCMAKE_POSITION_INDEPENDENT_CODE=ON", *flags])
        command = ["cmake", "--build", build, "-j", args.jobs]
        if target:
            command += ["--target", target]
        run(command)
        if library:
            records[name] = dict(revision=subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"],
                                                                  text=True).strip(),
                                 library_type="static", library=build / library)
    if "rav1e" in selected:
        records["rav1e"] = build_rav1e(args.jobs)
    if "svt" in selected:
        records["svt"] = build_svt(args.jobs)
    # Record the actual engine identities the original C and Rust consumers link.
    for record in records.values():
        if "prefix" in record:
            libraries = sorted(path for path in (record["prefix"] / "lib").iterdir()
                               if path.is_file() and not path.is_symlink() and (".so" in path.name or path.suffix == ".dylib"))
            record["libraries"] = {str(path.relative_to(ROOT)): digest(path) for path in libraries}
            record["pkgconfig_sha256"] = digest(record["pkgconfig"])
            record["prefix"], record["pkgconfig"] = str(record["prefix"].relative_to(ROOT)), str(record["pkgconfig"].relative_to(ROOT))
        else:
            record["library_sha256"] = digest(record["library"])
            record["library"] = str(record["library"].relative_to(ROOT))
    manifest = DEPS / "optional-engines.json"
    existing = json.loads(manifest.read_text()) if manifest.is_file() else {}
    existing.update(records)
    manifest.write_text(json.dumps(existing, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
