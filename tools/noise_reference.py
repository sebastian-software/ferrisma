#!/usr/bin/env python3
"""Compare the selected pristine noise stage with its default optimized object.

All compile/link arguments come from the real build. The default object removes
only the isolated stage's -O0; every sanitizer remains active. Temporary objects
and binaries never replace the active build graph. Requests mirror the Rust
row corpus; no codec algorithm is implemented here.
"""
import json
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]


def requests():
    patterns = [list(map(bits, [-2, -0.0, 0, 0.01, 0.25, 0.5, 1, 2])),
                [0, 0x80000000, 1, 0x80000001, 0x00800000, 0x80800000, 0x3f800001, 0xbf800001],
                [0x7fc12345, 0xffc98765, 0x7fa11111, 0xffa22222,
                 0x7f800000, 0xff800000, 1, 0x80000001]]
    luts = [[0] * 8, list(map(bits, [0.001, 0.0011, 0.125, 0.25, 0.5, 1, 2, -1])),
            [0x7fa12345, *map(bits, [0, 0.125, 0.5, 1, -0.0, 0.75, 0.25])]]
    cases, finite = [], []
    for kind in (0, 1):
        for first in (3, 4, 6):
            for width in (0, 1, 2, 7, 8, 9, 17):
                for left, right, origin in ((0, 0, 0), (0, 0, 5), (2, 3, 4)):
                    for values in patterns:
                        for lut in luts:
                            cases.append(" ".join(map(str, ["noise-row", kind, first, width,
                                                           left, right, origin, *lut, *values])))
                            finite.append(all(v & 0x7f800000 != 0x7f800000 for v in [*lut, *values]))
    if (len(cases), sum(finite)) != (1134, 504):
        raise RuntimeError("Noise stage corpus coverage changed")
    return cases, finite


def observe(binary, cases):
    result = subprocess.run([str(binary)], input="\n".join(cases) + "\n", text=True,
                            capture_output=True, check=True)
    lines = result.stdout.splitlines()
    if len(lines) != len(cases):
        raise RuntimeError("Noise reference response count changed")
    return lines


def default_binary(build, temporary):
    entries = json.loads((build / "compile_commands.json").read_text())
    entry, = [e for e in entries if Path(e["file"]).resolve() == ROOT / "tools/oracle/noise_stage.cc"]
    command = shlex.split(entry["command"])
    if command.count("-O0") != 1 or "-O3" not in command:
        raise RuntimeError("Unexpected scoped noise stage compiler configuration")
    command.remove("-O0")
    obj, binary = temporary / "noise_stage.cc.o", temporary / "jxl_oracle"
    command[command.index("-o") + 1] = str(obj)
    for index, arg in enumerate(command):
        if arg == "-MF":
            command[index + 1] = str(temporary / "noise_stage.cc.o.d")
        elif arg.startswith("-MF"):
            command[index] = "-MF" + str(temporary / "noise_stage.cc.o.d")
    subprocess.run(command, cwd=entry["directory"], check=True)
    link = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
    index, = [i for i, arg in enumerate(link) if arg == "CMakeFiles/jxl_oracle.dir/noise_stage.cc.o"]
    link[index] = str(obj)
    link[link.index("-o") + 1] = str(binary)
    subprocess.run(link, cwd=build, check=True)
    return binary


def audit():
    cases, finite = requests()
    with tempfile.TemporaryDirectory(prefix="jxl-noise-reference-") as directory:
        temporary = Path(directory)
        binaries = []
        for name in ("oracle", "oracle-asan"):
            target = temporary / name
            target.mkdir()
            build = ROOT / "target" / name
            binaries.append((name, build / "jxl_oracle", default_binary(build, target)))
        for start in range(0, len(cases), 8):
            chunk = cases[start:start + 8]
            previous = None
            for name, selected, default in binaries:
                actual = observe(selected, chunk)
                baseline = observe(default, chunk)
                for local, (chosen, source) in enumerate(zip(actual, baseline)):
                    if finite[start + local] and chosen != source:
                        raise RuntimeError(f"Finite noise source drift in {name}, case {start + local}")
                if previous is not None and actual != previous:
                    raise RuntimeError(f"Selected ordinary/ASan noise stages differ in cases {start}..{start + len(chunk) - 1}")
                previous = actual
    for name, _, _ in binaries:
        print(f"{name}: {sum(finite)} finite noise requests match default source", flush=True)
    print(f"Selected ordinary/ASan noise stages agree on {len(cases)} requests", flush=True)


if __name__ == "__main__":
    audit()
