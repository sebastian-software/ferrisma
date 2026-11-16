#!/usr/bin/env python3
"""Translate the original codec registry with every optional upstream codec guard enabled.

The root is upstream libavif with AVIF_CODEC_DAV1D=OFF (port ADR 0061): the C
reference must not define AVIF_CODEC_DAV1D, and the registry gains only the
optional guards that the root's Cargo features select (libgav1, rav1e, svt,
avm). `tools/import_optional.py` corrects the output and records it in
`translations/codec-registry.json`; its `--check` mode compares it.
"""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]
GUARDS = ["-DAVIF_CODEC_LIBGAV1=1", "-DAVIF_CODEC_RAV1E=1", "-DAVIF_CODEC_SVT=1", "-DAVIF_CODEC_AVM=1"]


def registry_command(command):
    """Return the original avif.c command with the optional guards; reject a dav1d configuration."""
    if any(flag.startswith("-DAVIF_CODEC_DAV1D") for flag in shlex.split(command)):
        raise SystemExit("The root C reference must be configured with AVIF_CODEC_DAV1D=OFF (port ADR 0061)")
    return command + " " + " ".join(GUARDS)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, default=ROOT / "build/baseline")
    parser.add_argument("--clang-include", type=Path, required=True)
    parser.add_argument("--c2rust", default="c2rust-transpile")
    args = parser.parse_args()
    entry = next(entry for entry in json.loads((args.build / "compile_commands.json").read_text())
                 if entry["file"].endswith("/src/avif.c"))
    entry["command"] = registry_command(entry["command"])
    entry["command"] += " -isystem " + shlex.quote(str(args.clang_include.resolve()))
    database = ROOT / "build/translate-avm-registry/compile_commands.json"
    database.parent.mkdir(parents=True, exist_ok=True)
    database.write_text(json.dumps([entry], indent=2) + "\n")
    environment = os.environ.copy()
    if environment.get("C2RUST_LIBRARY_PATH"):
        environment["DYLD_LIBRARY_PATH"] = environment["C2RUST_LIBRARY_PATH"]
    result = subprocess.run([args.c2rust, str(database), "--emit-build-files", "--fail-on-error", "--overwrite-existing",
                             "--output-dir", str(ROOT / "build/generated-avm-registry")], cwd=ROOT,
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, env=environment)
    (ROOT / "build/transpile-avm-registry.log").write_text(result.stdout)
    if result.returncode or "error:" in result.stdout or "error generated" in result.stdout:
        raise SystemExit("Codec registry translation failed; inspect build/transpile-avm-registry.log")
    print("Translated all original codec registry entries")


if __name__ == "__main__":
    main()
