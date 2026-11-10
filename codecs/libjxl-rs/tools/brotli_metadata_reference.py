#!/usr/bin/env python3
"""Actual original callback replacement during genuine metadata decoding.

This establishes the source callback/reload contract. It deliberately makes no
claim that the Rust streaming decoder is complete. The Rust allocation-relative
setter and its callable fields have separate native/differential API tests.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify

PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def binding(binary, sanitized):
    expected = {name: "libbrotlidec.a(decode.c.o)" for name in [
        "BrotliDecoderCreateInstance", "BrotliDecoderDestroyInstance",
        "BrotliDecoderSetMetadataCallbacks", "BrotliDecoderDecompressStream",
        "BrotliDecoderGetErrorCode", "BrotliDecoderIsFinished"]}
    expected.update({"BrotliDecoderStateInit": "libbrotlidec.a(state.c.o)",
                     "BrotliEncoderCreateInstance": "libbrotlienc.a(encode.c.o)",
                     "BrotliEncoderDestroyInstance": "libbrotlienc.a(encode.c.o)",
                     "BrotliEncoderCompressStream": "libbrotlienc.a(encode.c.o)"})
    return verify(binary, sanitized, "brotli_metadata.map", ["brotli_metadata.cc"], expected)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", type=Path, required=True)
    p.add_argument("--sanitized", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    # Preserve the first eight independently captured requests in order.
    requests = ["0 0 32 17", "1 0 32 17", "2 0 32 17", "3 0 32 17",
                "4 0 32 17", "5 0 32 17", "1 1 32 17", "2 7 32 17"]
    for mode in range(6):
        for chunk in [0, 1, 2, 3, 7, 8, 16, 31, 64]:
            for first, second in [(0, 0), (1, 0), (0, 1), (1, 1), (3, 2),
                                  (32, 17), (63, 65), (255, 256), (4096, 17), (16384, 3)]:
                requests.append(f"{mode} {chunk} {first} {second}")
    (args.output / "requests.txt").write_text("\n".join(requests) + "\n")
    report = {"source_pin": PIN, "requests": len(requests),
              "source_only_callback_flow": True, "complete_rust_streaming_decoder": False,
              "source_setter_order": ["metadata_start_func", "metadata_chunk_func", "metadata_callback_opaque"],
              "configurations": {}}
    expected = None
    for name, binary, sanitized in [("ordinary", args.ordinary.resolve(), False),
                                     ("sanitized", args.sanitized.resolve(), True)]:
        source = binding(binary, sanitized)
        path = args.output / f"{name}.txt"
        with path.open("wb") as output:
            result = subprocess.run([binary], input=("\n".join(requests) + "\n").encode(),
                                    stdout=output, stderr=subprocess.PIPE)
        assert result.returncode == 0 and not result.stderr, (name, result.returncode, result.stderr)
        records = path.read_bytes().splitlines()
        assert len(records) == len(requests)
        if expected is None:
            expected = records
        assert records == expected, name
        for record in records:
            assert b" Z1:" in record, record[-120:]
            for event in record.split():
                if event.startswith(b"C"):
                    fields = event.split(b":")
                    assert len(fields) == 6 and len(fields[5]) == int(fields[1]) * 2
                if event.startswith(b"S"):
                    assert event == b"S111"
            # Actual metadata-only codestreams produce no ordinary output.
            assert record.split()[-1].split(b":")[-1] == b"0"
        report["configurations"][name] = {"source_binding": source, "output_sha256": sha(path),
                                            "output_bytes": path.stat().st_size}
    report["all_source_exact"] = True
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"source_requests": len(requests), "all_source_exact": True}))


if __name__ == "__main__":
    main()
