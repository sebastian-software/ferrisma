#!/usr/bin/env python3
"""Regenerate optional AV1 adapters using the complete optional C reference build."""
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import argparse

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--build", type=Path, default=ROOT / "build/baseline-libgav1-rav1e-svt-libyuv")
parser.add_argument("--clang-include", type=Path, required=True)
parser.add_argument("--c2rust", default="c2rust-transpile")
args = parser.parse_args()
entries = [entry for entry in json.loads((args.build / "compile_commands.json").read_text())
           if "avif_obj.dir" in entry["command"]]
overlay = ROOT / "build/translation-headers/libyuv"
overlay.mkdir(parents=True, exist_ok=True)
source = ROOT / "build/deps/libyuv/include"
shutil.copyfile(source / "libyuv.h", overlay.parent / "libyuv.h")
header = (source / "libyuv/row.h").read_text()
header, count = re.subn(r"^typedef \w+ __attribute__\(\(vector_size\(\d+\)\)\) \w+;\n", "", header, flags=re.M)
if count != 13:
    raise SystemExit("Unexpected libyuv vector declarations; review the pinned header")
for name in ["YuvConstants", "ArgbConstants"]:
    header = re.sub(r"struct " + name + r" \{.*?\};", "struct " + name + ";", header, flags=re.S)
(overlay / "row.h").write_text(header)
for entry in entries:
    command = shlex.split(entry["command"])
    if entry["file"].endswith("/reformat_libyuv.c"):
        command.insert(1, "-I" + str(overlay.parent))
    command += ["-isystem", str(args.clang_include.resolve())]
    entry["command"] = shlex.join(command)
database = ROOT / "build/translate-optional/compile_commands.json"
database.parent.mkdir(parents=True, exist_ok=True)
database.write_text(json.dumps(entries, indent=2) + "\n")
environment = os.environ.copy()
if environment.get("C2RUST_LIBRARY_PATH"):
    environment["DYLD_LIBRARY_PATH"] = environment["C2RUST_LIBRARY_PATH"]
raw = ROOT / "build/generated-optional"
if raw.exists():
    shutil.rmtree(raw)
result = subprocess.run([args.c2rust, str(database), "--emit-build-files", "--fail-on-error", "--output-dir", str(raw)],
                        cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, env=environment)
(ROOT / "build/transpile-optional.log").write_text(result.stdout)
if result.returncode or "error:" in result.stdout or "error generated" in result.stdout:
    raise SystemExit("Optional translation failed; inspect build/transpile-optional.log")
subprocess.run(["python3", str(ROOT / "tools/import_optional.py")], cwd=ROOT, check=True)
