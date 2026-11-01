#!/usr/bin/env python3
"""Regenerate Rust from the reference build's actual C compilation database."""
import argparse
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, default=ROOT / "build/baseline")
    parser.add_argument("--c2rust", default="c2rust-transpile")
    parser.add_argument("--clang-include", type=Path, required=True,
                        help="LLVM 16's lib/clang/16/include directory")
    parser.add_argument("--check", action="store_true", help="Regenerate and compare without importing")
    args = parser.parse_args()
    if not (args.clang_include / "stdarg.h").exists():
        raise SystemExit("--clang-include must contain LLVM 16's standard headers")
    subprocess.run(["python3", str(ROOT / "tools/verify_upstream.py")], check=True)
    entries = [entry for entry in json.loads((args.build / "compile_commands.json").read_text())
               if "avif_obj.dir" in entry.get("command", "")]
    if not entries or any("rust_archive_anchor" in entry["file"] for entry in entries):
        raise SystemExit("Use the C reference compilation database, not the Rust test build")
    database = ROOT / "build/translate/compile_commands.json"
    database.parent.mkdir(parents=True, exist_ok=True)
    for entry in entries:
        entry["command"] += " -isystem " + shlex.quote(str(args.clang_include.resolve()))
    database.write_text(json.dumps(entries, indent=2) + "\n")
    raw = ROOT / "build/generated"
    if raw.exists():
        shutil.rmtree(raw)
    command = [args.c2rust, str(database), "--emit-build-files", "--fail-on-error",
               "--output-dir", str(raw)]
    result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT)
    log = ROOT / "build/transpile.log"
    log.write_text(result.stdout)
    # Some C2Rust/Clang parse failures can leave output and return success.
    if result.returncode or "error:" in result.stdout or "error generated" in result.stdout:
        raise SystemExit(f"Translation failed; inspect {log}")
    importer = ["python3", str(ROOT / "tools/import_translation.py"),
                "--raw", str(raw), "--database", str(database)]
    if args.check:
        importer.append("--check")
    subprocess.run(importer, cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
