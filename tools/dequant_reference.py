#!/usr/bin/env python3
"""Audit the isolated Linux dequant observer against its default source objects.

Only protocol inputs and process orchestration live here. Both executables call
identical hash-checked original bodies. Baseline objects retain all compiler and
sanitizer flags except the new observer's scoped -O0 configuration.
"""

import json
import re
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]


def geometry():
    # Read the pinned lookup values only to construct the existing block inputs.
    # Neither adapter nor reference algorithm is supplied by this generator.
    source = (ROOT / "upstream/libjxl/lib/jxl/ac_strategy.h").read_text()
    result = []
    for axis in ("x", "y"):
        expression = (r"covered_blocks_" + axis + r"\(\) const\s*\{\s*"
                      r"static constexpr uint8_t kLut\[\] = \{([^}]+)\};")
        match, = re.findall(expression, source)
        values = [int(value) for value in match.split(",") if value.strip()]
        if len(values) != 27 or any(value not in (1, 2, 4, 8, 16, 32) for value in values):
            raise RuntimeError("Unexpected pinned AC strategy geometry")
        result.append(values)
    return list(zip(*result))


def block_request(raw, width, mode, xblocks, yblocks):
    size = xblocks * yblocks * 64
    values = ["dq-block", raw, width, size,
              bits([0.125, 3.75, 1 / 65536, 128][mode]),
              [1, 3, 256, 65536][mode], bits(0.8), bits(1.25),
              bits(-0.125), bits(0.375),
              1064436340, 1064177896, 1064515444, 1041529569]
    for c in range(3):
        for k in range(size):
            if mode == 0:
                value = [0, 1, -1, 2, -2, 3, -3, 32767, -32768][(k + c) % 9]
            elif mode == 1:
                value = (k * 173 + c * 31) % 65536 - 32768
            elif mode == 2:
                value = -(1 << 31) + c if k == size // 3 else 0
            else:
                value = (1 << 31) - 1 - c if k % 11 == 0 else k % 193 - 96
            values.append(value)
    stride = xblocks + 3
    values.extend((stride, 1, 2, 0))
    for c in range(3):
        values.extend(bits(((k + c * 3) % 37) / 32 - 0.5)
                      for k in range(yblocks * stride + 4))
    return " ".join(map(str, values))


def requests():
    special = [0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff,
               0xff7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc98765,
               0x7fa12345, 0xffa98765]
    bias = [1064436340, 1064177896, 1064515444, 1041529569]
    cases, finite = [], []
    # The same 34 existing exceptional-lane requests as tests/parity/dequant.rs.
    for width in range(2):
        for mode in range(17):
            values = ["dq-lane", 0, width, 64,
                      bits([0.125, 3.75, 1 / 65536, 128][mode % 4]),
                      [1, 3, 256, 65536][mode % 4],
                      bits(0.8), bits(1.25), bits(-0.125), bits(0.375), *bias]
            for c in range(3):
                for k in range(64):
                    values.append(special[(c + k + mode) % len(special)]
                                  if mode >= 4 and k % 7 == 0
                                  else bits((k + c * 3) / 16 + 0.125))
            for c in range(3):
                for k in range(64):
                    if mode % 4 == 0:
                        v = [0, 1, -1, 2, -2, 3, -3, 32767, -32768][(k + c) % 9]
                    elif mode % 4 == 1:
                        v = (k * 173 + c * 31) % 65536 - 32768
                    elif mode % 4 == 2:
                        v = -(1 << 31) + c if k == 64 // 3 else 0
                    else:
                        v = (1 << 31) - 1 - c if k % 11 == 0 else k % 193 - 96
                    values.append(v)
            finite.append(mode < 4)
            cases.append(" ".join(map(str, values)))
    # Additional finite inputs cover both widths, signs, zeros and subnormals.
    for case in range(128):
        values = ["dq-lane", 0, case % 2, 64, bits([0.125, 3.75, 128][case % 3]),
                  [1, 3, 65536][case % 3], bits(0.8), bits(1.25),
                  bits(-0.125), bits(0.375), *bias]
        for c in range(3):
            for k in range(64):
                v = bits(((case * 17 + c * 13 + k * 7) % 1025 - 512) / 16)
                if k % 17 == 0:
                    v = [0, 0x80000000, 1, 0x80000001, 0x00800000][(case + c) % 5]
                values.append(v)
        values.extend(((case * 257 + c * 193 + k * 173) % 65536 - 32768)
                      for c in range(3) for k in range(64))
        cases.append(" ".join(map(str, values)))
        finite.append(True)
    # The unchanged bias test's 168 protocol requests, including its complete
    # integer-boundary vector and all four independently varied bias slots.
    integers = [0, 1, -1, 2, -2, -(1 << 31), (1 << 31) - 1,
                16777215, 16777216, 16777217, -16777217]
    for exponent in range(1, 31):
        for offset in (-1, 0, 1):
            integers.extend(((1 << exponent) + offset, -(1 << exponent) + offset))
    words = [0, 0x80000000, 1, 0x007fffff, 0x00800000, 0x3f000000,
             0x3f800000, 0x7f7fffff, 0x7f800000, 0xff800000,
             0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765]
    for channel in range(3):
        for index in range(4):
            for word in words:
                biases = bias.copy()
                biases[index] = word
                cases.append(" ".join(map(str, ["dq-bias", channel, len(integers),
                                                *biases, *integers])))
                finite.append(word & 0x7f800000 != 0x7f800000)
    # All 216 existing finite DequantBlock inputs, retaining LLF/DC geometry,
    # both coefficient widths and all four integer/scale modes.
    for raw, (xblocks, yblocks) in enumerate(geometry()):
        for width in range(2):
            for mode in range(4):
                cases.append(block_request(raw, width, mode, xblocks, yblocks))
                finite.append(True)
    if (len(cases), sum(finite)) != (546, 448):
        raise RuntimeError("Dequant reference corpus coverage changed")
    return cases, finite


def observe(binary, cases):
    result = subprocess.run([str(binary)], input="\n".join(cases) + "\n",
                            text=True, capture_output=True, check=True)
    lines = result.stdout.splitlines()
    if len(lines) != len(cases):
        raise RuntimeError("Dequant reference response count changed")
    return lines


def default_binary(build, temporary):
    entries = json.loads((build / "compile_commands.json").read_text())
    entry, = [e for e in entries if Path(e["file"]).resolve() ==
              ROOT / "tools/oracle/dequant.cc"]
    command = shlex.split(entry["command"])
    if command.count("-O0") != 1 or "-O3" not in command:
        raise RuntimeError("Unexpected scoped dequant compiler configuration")
    # Retain every original compiler/sanitizer option except the scoped -O0.
    command.remove("-O0")
    obj, binary = temporary / "dequant.cc.o", temporary / "jxl_oracle"
    command[command.index("-o") + 1] = str(obj)
    for index, argument in enumerate(command):
        if argument == "-MF":
            command[index + 1] = str(temporary / "dequant.cc.o.d")
        elif argument.startswith("-MF"):
            command[index] = "-MF" + str(temporary / "dequant.cc.o.d")
    subprocess.run(command, cwd=entry["directory"], check=True)
    link = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
    index, = [i for i, arg in enumerate(link)
              if arg == "CMakeFiles/jxl_oracle.dir/dequant.cc.o"]
    link[index] = str(obj)
    link[link.index("-o") + 1] = str(binary)
    subprocess.run(link, cwd=build, check=True)
    return binary


def audit():
    cases, finite = requests()
    names = ("oracle", "oracle-asan")
    with tempfile.TemporaryDirectory(prefix="jxl-dequant-reference-") as temporary:
        temporary = Path(temporary)
        originals, defaults = [], []
        # Both default objects/executables live in this one temporary owner;
        # no active build object, dependency file or executable is replaced.
        for name in names:
            directory = temporary / name
            directory.mkdir()
            build = ROOT / "target" / name
            originals.append(build / "jxl_oracle")
            defaults.append(default_binary(build, directory))
        # A largest-strategy response contains hundreds of thousands of float
        # words. Compare four requests at a time and release output each chunk.
        for start in range(0, len(cases), 4):
            chunk = cases[start:start + 4]
            previous = None
            for name, original, default in zip(names, originals, defaults):
                actual = observe(original, chunk)
                baseline = observe(default, chunk)
                for local, (chosen, source) in enumerate(zip(actual, baseline)):
                    index = start + local
                    if finite[index] and chosen != source:
                        raise RuntimeError(f"Finite dequant source drift in {name}, case {index}")
                    if index == 4:
                        source_words, chosen_words = source.split(), chosen.split()
                        print(f"{name}: mode-4 default X/B words="
                              f"{source_words[9]}/{source_words[186]}, selected="
                              f"{chosen_words[9]}/{chosen_words[186]}", flush=True)
                del baseline
                if previous is not None and actual != previous:
                    raise RuntimeError(f"Selected ordinary/ASan dequant source observations differ "
                                       f"in cases {start}..{start + len(chunk) - 1}")
                previous = actual
            del actual, previous
    for name in names:
        print(f"{name}: {sum(finite)} finite dequant requests match default source", flush=True)
    print(f"Selected ordinary/ASan dequant objects agree on {len(cases)} requests", flush=True)


if __name__ == "__main__":
    audit()
