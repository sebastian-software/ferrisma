#!/usr/bin/env python3
"""Prove packed-field bytes against actual pinned C declarations/assignments."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex
import subprocess

ROOT = Path(__file__).resolve().parent.parent
PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def invoke(command, requests):
    p = subprocess.run(list(map(str, command)), input="\n".join(requests)+"\n",
                       text=True, capture_output=True, check=True)
    assert not p.stderr, p.stderr
    lines = p.stdout.splitlines()
    assert len(lines) == len(requests)
    return lines


def corpus():
    cases = []
    widths = [1, 1, 1, 1, 1, 1, 6, 8]
    for seed in range(256):
        for field, width in enumerate(widths):
            for value in [0, (1 << width)-1, 1 << width, 0xffffffff]:
                cases.append((seed, [(field, value)]))
    for seed in [0, 0xff, 0xa5, 0x5a]:
        for field in range(8):
            for value in [*range(256), 256, 65535, 0x80000000, 0xffffffff]:
                cases.append((seed, [(field, value)]))
    # Actual StateInit assignment order: large_window is assigned earlier than
    # the other flags; size_nibbles is deliberately not initialized there.
    init = [(5, 0), (0, 0), (1, 0), (2, 0), (3, 0), (4, 1), (6, 0)]
    for seed in range(256):
        cases.append((seed, init))
    state = 0x9e3779b9
    for seed in range(64):
        steps = []
        for _ in range(64):
            state ^= (state << 13) & 0xffffffff
            state ^= state >> 17
            state ^= (state << 5) & 0xffffffff
            steps.append((state % 8, state))
        cases.append((seed, steps))
    return [f"{seed} {len(steps)} " + " ".join(f"{f} {v}" for f,v in steps)
            for seed, steps in cases]


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", type=Path, required=True)
    p.add_argument("--sanitized", type=Path, required=True)
    p.add_argument("--rust", action="append", required=True, help="configuration=executable")
    p.add_argument("--rust-asm", action="append", required=True, help="configuration=assembly")
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--decoder-report", type=Path, required=True,
                   help="fresh complete original constructor allocation audit")
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    source = ROOT / "upstream/libjxl/third_party/brotli"
    assert subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip() == PIN
    assert not subprocess.check_output(["git", "-C", str(source), "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    header = (source / "c/dec/state.h").read_text()
    fields = re.findall(r"unsigned int (\w+) : (\d+);", header)
    assert fields == list(zip(["is_last_metablock", "is_uncompressed", "is_metadata", "should_wrap_ringbuffer",
                              "canny_ringbuffer_allocation", "large_window", "window_bits", "size_nibbles"],
                             ["1", "1", "1", "1", "1", "1", "6", "8"]))
    requests = corpus()
    (args.output / "requests.txt").write_text("\n".join(requests)+"\n")
    outputs = {}
    original = {}
    for name, binary, sanitized in [("ordinary", args.ordinary.resolve(), False),
                                    ("sanitized", args.sanitized.resolve(), True)]:
        commands = json.loads((binary.parent / "compile_commands.json").read_text())
        assert len(commands) == 1
        command = commands[0]
        assert Path(command["file"]).resolve() == ROOT / "tools/oracle/brotli_bitfield.c"
        flags = command["command"]
        assert ("-fsanitize=address,undefined" in flags) == sanitized
        if sanitized: assert "-fno-sanitize-recover=all" in flags
        assert "-O3" in flags and "-DNDEBUG" in flags
        assert "BROTLI_" not in flags
        assembly = args.output / f"{name}.s"
        cmd = shlex.split(flags)
        cmd[cmd.index("-c")] = "-S"
        cmd[cmd.index("-o")+1] = str(assembly)
        subprocess.run(cmd, cwd=command["directory"], check=True, capture_output=True)
        for uninitialized in [False, True]:
            label = name + ("-uninitialized" if uninitialized else "")
            result = invoke([binary, *(["--uninitialized"] if uninitialized else [])], requests)
            (args.output / f"{label}.txt").write_text("\n".join(result)+"\n")
            outputs[label] = result
        original[name] = {"binary": str(binary), "binary_sha256": sha(binary),
                          "assembly_sha256": sha(assembly),
                          "compile_commands_sha256": sha(binary.parent / "compile_commands.json")}
    assert outputs["ordinary"] == outputs["sanitized"]
    assert outputs["ordinary-uninitialized"] == outputs["sanitized-uninitialized"]
    size, offset = map(int, outputs["ordinary"][0].split()[:2])
    decoder_report = json.loads(args.decoder_report.read_text())
    assert decoder_report["pin"] == PIN and decoder_report["ordinary_sanitized_mismatches"] == 0
    init_start = 256*8*4 + 4*8*260
    for seed in range(256):
        init_word = outputs["ordinary"][init_start+seed].split()[-1].split(':')[1]
        assert decoder_report["source_initial_flags"][str(seed)] == f"{size}:{offset}:{init_word}"
    binaries = dict(s.split("=", 1) for s in args.rust)
    assemblies = dict(s.split("=", 1) for s in args.rust_asm)
    assert set(binaries) == set(assemblies)
    configs = {}
    for name, binary in binaries.items():
        for uninitialized in [False, True]:
            label = name + ("-uninitialized" if uninitialized else "")
            result = invoke([binary, size, offset, *(["--uninitialized"] if uninitialized else [])], requests)
            (args.output / f"{label}.txt").write_text("\n".join(result)+"\n")
            assert result == outputs["ordinary" + ("-uninitialized" if uninitialized else "")], label
        configs[name] = {"binary": binary, "binary_sha256": sha(binary),
                         "assembly": assemblies[name], "assembly_sha256": sha(assemblies[name]),
                         "ordinary_requests": len(requests)*2, "sanitized_requests": len(requests)*2}
    report = {"pin": PIN, "state_size": size, "word_offset": offset,
              "source_fields": fields, "requests_per_initialization_domain": len(requests),
              "initialized_and_partial_requests": len(requests)*2,
              "source_graphs": original, "rust_configurations": configs,
              "mismatches": 0, "other_state_bytes_unchanged": True,
              "actual_constructor_all_256_seed_words_exact": True,
              "decoder_report": str(args.decoder_report), "decoder_report_sha256": sha(args.decoder_report),
              "source_state_header_sha256": sha(source / "c/dec/state.h"),
              "files": {f.name: sha(f) for f in args.output.iterdir() if f.is_file()}}
    (args.output / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({"requests": len(requests)*2, "configurations": len(configs), "mismatches": 0}))


if __name__ == "__main__":
    main()
