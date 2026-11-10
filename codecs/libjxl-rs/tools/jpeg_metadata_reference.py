#!/usr/bin/env python3
"""Capture the genuine pinned JPEGData bundle, without a replacement codec."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
EMPTY = "0 0 0 0 0 0 0 0 0 0 - 0 - 0 -"
PINS = {"": "a16d32d0ac113f5e54125e49bb571281c5d0b704",
        "third_party/highway": "457c891775a7397bdb0376bb1031e6e027af1c48",
        "third_party/brotli": "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d",
        "third_party/skcms": "96d9171c94b937a1b5f0293de7309ac16311b722"}

def digest(data):
    return hashlib.sha256(data).hexdigest()

def capture(exe, requests):
    p = subprocess.run([str(exe)], input="\n".join(requests)+"\n",
                       text=True, capture_output=True, check=True)
    assert not p.stderr, p.stderr
    lines = p.stdout.splitlines()
    assert len(lines) == len(requests)
    return lines

def pinned():
    for path, pin in PINS.items():
        root = ROOT / "upstream/libjxl" / path
        assert subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip() == pin
        assert not subprocess.check_output(["git", "-C", str(root), "status", "--porcelain", "--ignore-submodules=none"], text=True).strip()

def fields_request(fields, op):
    return f"jm-ops {fields} 1 {op}"

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", type=Path, default=ROOT/"target/oracle/jxl_oracle")
    p.add_argument("--sanitizers", type=Path, default=ROOT/"target/oracle-asan/jxl_oracle")
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    pinned()
    manifest = json.loads((ROOT/"tests/data/jpeg-metadata/provenance.json").read_text())
    graph_files = {}
    for label, exe in [("ordinary", args.ordinary), ("sanitizers", args.sanitizers)]:
        assert exe.is_file()
        commands = json.loads((exe.parent/"compile_commands.json").read_text())
        selected = [x for x in commands if x["file"].endswith("/lib/jxl/jpeg/jpeg_data.cc")]
        assert len(selected) == 1
        assert ("-fsanitize=address,undefined" in selected[0]["command"]) == (label == "sanitizers")
        if label == "sanitizers":
            assert "-fno-sanitize-recover=all" in selected[0]["command"]
        symbols = subprocess.check_output(["nm", "-C", str(exe)], text=True)
        required = ["jxl::jpeg::JPEGData::VisitFields(jxl::Visitor*)", "jxl::jpeg::SetJPEGDataFromICC(", "jxl::jpeg::ReadJpeg("]
        for name in required:
            assert any(name in line and re.search(r"\b[Tt] ", line) for line in symbols.splitlines()), name
        (args.output/f"{label}-symbols.txt").write_text(symbols)
        graph_files[label] = {"binary_sha256": digest(exe.read_bytes()), "source_compile": selected[0], "bound_symbols": required}
    fixtures = {}
    requests = []
    for name, info in manifest["files"].items():
        raw = (ROOT/"tests/data/jpeg-metadata"/name).read_bytes()
        assert digest(raw) == info["sha256"]
        fixture_request = "jm-fixture " + raw.hex()
        fields = capture(args.ordinary, [fixture_request])[0].strip()
        assert fields == capture(args.sanitizers, [fixture_request])[0].strip()
        (args.output/(name+".fields.txt")).write_text(fields+"\n")
        fixtures[name] = {"jpeg_sha256": digest(raw), "fields_sha256": digest((fields+"\n").encode())}
        if name == "gray.jpg":
            assert fields+"\n" == (ROOT/"tests/data/jpeg-metadata/gray.fields.txt").read_text()
        for offset in range(8):
            req = fields_request(fields, f"W {offset}")
            write = capture(args.ordinary, [req])[0]
            assert write == capture(args.sanitizers, [req])[0]
            prefix = write.split(" S", 1)[0].split()
            assert prefix[:2] == ["W", "0"]
            encoded = bytes.fromhex(prefix[3])
            requests.append(req)
            requests.extend(fields_request(EMPTY, f"D {offset} {encoded[:length].hex() or '-'}") for length in range(len(encoded)+1))
            if name == "gray.jpg" and offset == 3:
                assert int(prefix[2]) == 1844
                assert encoded.hex()+"\n" == (ROOT/"tests/data/jpeg-metadata/gray.bundle.hex").read_text()
                (args.output/"gray-source-write.txt").write_text(write+"\n")
    icc_fields = "0 0 0 2 "+("a5"*18)+" "+("5a"*20)+" 2 1 1 0 0 0 0 0 - 0 - 0 -"
    requests.append(fields_request(icc_fields, "I 0708"))
    ordinary = capture(args.ordinary, requests)
    sanitized = capture(args.sanitizers, requests)
    assert ordinary == sanitized
    for label, lines in [("ordinary", ordinary), ("sanitizers", sanitized)]:
        (args.output/f"{label}-observations.txt").write_text("\n".join(lines)+"\n")
    (args.output/"requests.txt").write_text("\n".join(requests)+"\n")
    # A source-undefined struct input must not acquire invented Rust semantics.
    # CanEncode accumulates the out-of-range two-bit error, but the source
    # accesses bool[4] first during its final table-order validation.
    baseline = (args.output/"baseline.jpg.fields.txt").read_text().split()
    i = 3
    for _ in range(3):
        n = int(baseline[i]); i += 1+n
    n = int(baseline[i]); i += 1+n*67
    assert int(baseline[i]) == 4
    assert baseline[i+1] == "0"
    # Two complete defined calls and their exact owner snapshots precede the
    # reached out-of-bounds call. H is only a data-fixture owner assignment.
    prefix_request = f"jm-ops {' '.join(baseline)} 2 C H 0 4"
    expected_prefix = capture(args.ordinary, [prefix_request])[0]
    assert expected_prefix == capture(args.sanitizers, [prefix_request])[0]
    probe = f"jm-ops {' '.join(baseline)} 3 C H 0 4 C\n"
    diagnosed = subprocess.run([str(args.sanitizers)], input=probe, text=True, capture_output=True)
    assert diagnosed.returncode != 0
    assert diagnosed.stdout == expected_prefix
    assert re.search(r"jpeg_data\.cc:415:13: runtime error: index 4 out of bounds for type 'bool\[4\]'", diagnosed.stderr), diagnosed.stderr
    (args.output/"invalid-slot-request.txt").write_text(probe)
    (args.output/"invalid-slot-defined-prefix.txt").write_text(expected_prefix)
    (args.output/"invalid-slot-diagnostic.txt").write_text(diagnosed.stderr)
    pinned()
    report = {"pins": PINS, "graphs": graph_files, "fixtures": fixtures,
              "defined_requests": len(requests), "source_errors": sum(x.startswith(" D ") and x.split()[1] != "0" for x in ordinary),
              "source_undefined_probe": {"returncode": diagnosed.returncode, "predicted_index": 4, "reached_array": "dc_ok[4] at first DHT marker with huffman_code[0].slot_id=4", "defined_prefix_sha256": digest(expected_prefix.encode()), "request_sha256": digest(probe.encode()), "diagnostic_sha256": digest(diagnosed.stderr.encode())},
              "captures": {x.name: digest(x.read_bytes()) for x in args.output.iterdir() if x.is_file()},
              "scope": "JPEGData metadata bundle and initialized field/owner snapshots; Brotli DecodeJPEGData and full reconstruction absent"}
    (args.output/"report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({"defined_requests": len(requests), "source_errors": report["source_errors"], "report": str(args.output/"report.json")}))

if __name__ == "__main__":
    main()
