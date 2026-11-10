#!/usr/bin/env python3
"""Real pinned window/varlen/metablock readers, resumed at actual checkpoints.

The private observation TU includes pristine decode.c exactly once with its
default configuration. An independent public-only target binds the unchanged
archive. Actual encoder-produced normal and metadata streams prove that public
decoder bindings in both targets agree before any private-helper comparison.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify

PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"
ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def capture(binary, requests, path):
    payload = ("\n".join(requests) + "\n").encode()
    with path.open("wb") as output:
        result = subprocess.run([binary], input=payload, stdout=output, stderr=subprocess.PIPE)
    assert result.returncode == 0 and not result.stderr, (binary, result.returncode, result.stderr)
    records = path.read_bytes().splitlines()
    assert len(records) == len(requests), (binary, len(records), len(requests))
    return records


def binding(binary, sanitized, private):
    decoder_object = "brotli_metablock.c.o" if private else "libbrotlidec.a(decode.c.o)"
    expected = {name: decoder_object for name in [
        "BrotliDecoderCreateInstance", "BrotliDecoderDestroyInstance",
        "BrotliDecoderDecompress", "BrotliDecoderDecompressStream"]}
    expected.update({"BrotliDecoderStateInit": "libbrotlidec.a(state.c.o)",
                     "BrotliEncoderCompress": "libbrotlienc.a(encode.c.o)",
                     "BrotliEncoderCompressStream": "libbrotlienc.a(encode.c.o)"})
    result = verify(binary, sanitized, f"{binary.name}.map", ["brotli_metablock.c"], expected)
    commands = json.loads((binary.parent / "compile_commands.json").read_text())
    adapter = [c for c in commands if c["file"].endswith("/tools/oracle/brotli_metablock.c")
               and f"CMakeFiles/{binary.name}.dir/" in c["command"]]
    assert len(adapter) == 1
    assert ("-DORACLE_DEFAULT_ARCHIVE=1" in adapter[0]["command"]) == (not private)
    source = Path(next(c["file"] for c in commands if c["file"].endswith("/c/dec/decode.c"))).resolve().parents[2]
    wrapper = ROOT / "tools/oracle/brotli_metablock.c"
    assert wrapper.read_text().count('#include "c/dec/decode.c"') == 1
    result.update(pristine_decode_sha256=sha(source / "c/dec/decode.c"),
                  private_pristine_include=private, observation_adapter_sha256=sha(wrapper))
    return result


def base_corpus():
    requests = []
    for seed in [0, 90, 165, 255]:
        for large in [0, 1]:
            for byte in range(256):
                requests.append(f"0 {seed} 0 {byte} 8 {large} 0 -")
    for number in range(65536):
        data = number.to_bytes(2, "little").hex()
        requests.append(f"1 165 0 0 0 0 1 {data}")
    for bits in range(8):
        for accumulator in range(1 << bits):
            for data in ["-", "00", "ff", "55aa"]:
                requests.append(f"1 90 0 {accumulator} {bits} 0 1 {data}")
    for seed in [0, 165]:
        for number in range(4096):
            data = bytes([number & 255, number >> 8, (number * 97) & 255,
                          (number * 31) & 255, 0, 0, 0, 0])
            for chunk in [0, 1, 2, 3]:
                requests.append(f"2 {seed} 0 0 0 0 {chunk} {data.hex()}")
    for byte in range(256):
        for size in [0, 1, 2, 3, 4, 5, 6]:
            data = bytes([byte]) * size
            requests.append(f"2 255 0 0 0 0 1 {data.hex() or '-'}")
    # Default discriminants return the actual original UNREACHABLE status;
    # no field requiring a valid resume history is dereferenced in that path.
    for op in [1, 2]:
        for state in [99, 0xffffffff]:
            requests.append(f"{op} 165 {state} 0 0 0 1 -")
    assert len(requests) == 103168
    return requests


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", type=Path, required=True)
    p.add_argument("--sanitized", type=Path, required=True)
    p.add_argument("--default-ordinary", type=Path, required=True)
    p.add_argument("--default-sanitized", type=Path, required=True)
    p.add_argument("--rust", action="append", required=True)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    binaries = [("ordinary", args.ordinary.resolve(), False, True),
                ("sanitized", args.sanitized.resolve(), True, True),
                ("default-ordinary", args.default_ordinary.resolve(), False, False),
                ("default-sanitized", args.default_sanitized.resolve(), True, False)]
    report = {"source_pin": PIN, "source_bindings": {name: binding(binary, san, private)
              for name, binary, san, private in binaries}}
    payloads = [b"", b"x", bytes(range(256)), b"dictionary time down life left back code data " * 40,
                bytes((i * 97 + i // 13) & 255 for i in range(4096))]
    encoders = []
    for operation in [4, 5]:
        for quality in [0, 4, 11]:
            for window in [10, 16, 24]:
                for data in payloads:
                    encoders.append(f"{operation} {quality} {window} 0 0 0 0 {data.hex() or '-'}")
    encoded = None
    for name, binary, _, _ in binaries:
        records = capture(binary, encoders, args.output / f"encode-{name}.txt")
        if encoded is None:
            encoded = records
        assert records == encoded, name
    streams = []
    for record in encoded:
        ok, size, encoded_hex = record.split()
        assert ok == b"1" and len(encoded_hex) == int(size) * 2
        streams.append(encoded_hex.decode())
    decode = [f"3 0 0 0 0 0 0 {stream}" for stream in streams]
    decoded = None
    for name, binary, _, _ in binaries:
        records = capture(binary, decode, args.output / f"decode-{name}.txt")
        if decoded is None:
            decoded = records
        assert records == decoded and all(r.startswith(b"1 ") for r in records), name
    for index, record in enumerate(decoded):
        expected = payloads[index % len(payloads)] if index < len(streams) // 2 else b""
        fields = record.split()
        actual = bytes.fromhex(fields[2].decode()) if len(fields) == 3 else b""
        assert int(fields[1]) == len(expected) and actual == expected
    requests = base_corpus()
    # Actual encoder-produced streams also reach the helpers at the real
    # window→metablock checkpoint, including genuine metadata headers.
    windows = [f"0 165 0 {bytes.fromhex(s)[0]} 8 0 0 -" for s in streams]
    controls = capture(args.ordinary.resolve(), windows, args.output / "genuine-window.txt")
    requests += windows
    for stream, control in zip(streams, controls):
        fields = control.split(b":")
        assert fields[0] == b"1"
        accumulator, bits = int(fields[2]), int(fields[3])
        rest = bytes.fromhex(stream)[1:]
        for length in range(min(len(rest), 6) + 1):
            for chunk in [0, 1, 2, 3]:
                requests.append(f"2 165 0 {accumulator} {bits} 0 {chunk} {rest[:length].hex() or '-'}")
    (args.output / "requests.txt").write_text("\n".join(requests) + "\n")
    expected = None
    configurations = [("ordinary", args.ordinary.resolve()), ("sanitized", args.sanitized.resolve())]
    configurations += [(name, Path(path).resolve()) for name, path in (x.split("=", 1) for x in args.rust)]
    assert len({name for name, _ in configurations}) == len(configurations)
    report["configurations"] = {}
    for name, binary in configurations:
        path = args.output / f"{name}.txt"
        records = capture(binary, requests, path)
        if expected is None:
            expected = records
        assert records == expected, next((i, x[:120], y[:120]) for i, (x, y) in enumerate(zip(records, expected)) if x != y)
        report["configurations"][name] = {"output_sha256": sha(path), "binary_sha256": sha(binary)}
    report.update(requests=len(requests), genuine_complete_streams=len(streams), all_exact=True)
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"requests": len(requests), "genuine_complete_streams": len(streams), "all_exact": True}))


if __name__ == "__main__":
    main()
