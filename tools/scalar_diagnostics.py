#!/usr/bin/env python3
"""Record native scalar instructions when comparing floating-point payloads."""

from pathlib import Path
import os
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def run(*args, **kwargs):
    print("+ " + " ".join(map(str, args)), flush=True)
    return subprocess.run(list(map(str, args)), cwd=ROOT, check=True, **kwargs)


def instructions(path, names):
    if not path.is_file():
        print(f"No compiled object: {path}", flush=True)
        return
    symbols = subprocess.check_output(
        ["nm", "--defined-only", str(path)], text=True
    )
    for line in symbols.splitlines():
        fields = line.split()
        if len(fields) < 3 or fields[-2] not in ("t", "T", "w", "W"):
            continue
        symbol = fields[-1]
        if any(name in symbol for name in names):
            run("objdump", "-d", "--disassemble=" + symbol, path)


def main():
    run("rustc", "-vV")
    compiler = os.environ.get("CXX", "clang++")
    run(compiler, "--version")
    cpu = Path("/proc/cpuinfo")
    if cpu.is_file():
        seen = set()
        for line in cpu.read_text().splitlines():
            key = line.split(":", 1)[0].strip()
            if key in ("vendor_id", "model name", "flags") and key not in seen:
                print(line, flush=True)
                seen.add(key)
    probe = ROOT / "target/scalar-diagnostics"
    run(compiler, "-std=c++17", "-O3", "-fno-vectorize", "-fno-slp-vectorize",
        "-stdlib=libc++", "-DHWY_COMPILE_ONLY_SCALAR=1",
        "-Iupstream/libjxl", "-Iupstream/libjxl/third_party/highway",
        "-Iupstream/libjxl/lib/include", "tools/oracle/scalar_diagnostics.cc",
        "-o", probe)
    run(probe, "2147483648", "2141266757", "2143913829", "1046066128",
        "1060575065", "1033100696", "0")
    instructions(probe, ("AddProbe", "MulProbe", "OriginalGamutProbe"))
    for build in ("oracle", "oracle-asan"):
        directory = ROOT / "target" / build
        original = directory / "jxl_oracle"
        if original.is_file():
            run(original, input=(
                "tm-color 4 0 1148846080 0 1132396544 1046066128 1060575065 "
                "1033100696 2147483648 2141266757 2143913829 0\n"
            ), text=True)
        instructions(directory / "CMakeFiles/jxl_oracle.dir/sample_conversion.cc.o",
                     ("GamutMap",))
    for profile in ("debug", "release"):
        for candidate in sorted((ROOT / "target" / profile / "deps").glob("parity-*")):
            if candidate.is_file() and candidate.open("rb").read(4) == b"\x7fELF":
                instructions(candidate, ("add_gamut", "gamut_map"))


if __name__ == "__main__":
    main()
