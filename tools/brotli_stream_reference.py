#!/usr/bin/env python3
"""Build and compare the actual complete pinned Brotli streaming entry.

Both pristine archive graphs retain default O3/NDEBUG flags; every ordinary
and instrumented graph uses the actual public source factories/decoder/encoder.
All initial requests/observations, including failures, remain in their original
order. This proof covers streaming, not the distinct stack-owned one-shot API.
"""
import argparse
import hashlib
import json
import os
import shutil
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify
from brotli_state_reference import ROOT, PIN
from brotli_stream_corpus import (encoder_requests, metadata_parameters,
                                 extra_encoder_requests, decoder_requests)


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


RETAINED = {
    "initial": ("cfeadfb90384f0f3b9245498fc0e38dc20ed21cbec772a3b3167a2089ebb9379",
                "89a465ec8f009a0571bdfe2242377a96e4f6403c161934ae1e4e57521c1fc8fc"),
    "extended": ("d0287a5c65c3c2a10fcf27b65e9f8321a3991fb7921bf6f1f05ea9f00b7f378d",
                 "b092ac41459798209385865bd25d46134ee3b2027c78f2739e5469fed5917af8"),
    "additional": ("49f8f509dcf9c389d5ba74809d02813ab1c20d657442dfa78fe8a564c7361906",
                   "2cfef00776c449d0f96bbb55cc0028b32aa20def782397316d6f07de9eb28a8b")}


def write_requests(path, rows):
    path.write_text("\n".join(rows) + "\n")


def execute_observer(binary, requests, output):
    with requests.open("rb") as inp, output.open("wb") as out:
        result = subprocess.run([str(binary)], stdin=inp, stdout=out, stderr=subprocess.PIPE)
    output.with_suffix(output.suffix + ".stderr").write_bytes(result.stderr)
    assert result.returncode == 0 and not result.stderr, (binary, result.returncode, result.stderr[:2000])


def segment_hashes(path, ranges):
    hashes = {name: hashlib.sha256() for name in ranges}
    counts = {name: 0 for name in ranges}
    count = 0
    with path.open("rb") as source:
        for index, line in enumerate(source):
            assert line.endswith(b"\n"), (path, index)
            for name, (start, end) in ranges.items():
                if start <= index < end:
                    hashes[name].update(line)
                    counts[name] += 1
            count += 1
    assert count == max(end for _, end in ranges.values()), (path, count)
    for name, (start, end) in ranges.items():
        assert counts[name] == end - start
    return {name: h.hexdigest() for name, h in hashes.items()}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--output", type=Path, required=True)
    p.add_argument("--build-root", type=Path)
    args = p.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    builds = args.build_root.resolve() if args.build_root else out

    def run(label, command, env=None):
        with (out / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    decoder_symbols = ["CreateInstance", "DestroyInstance", "DecompressStream", "SetParameter",
        "AttachDictionary", "GetErrorCode", "IsUsed", "IsFinished", "HasMoreOutput",
        "TakeOutput", "SetMetadataCallbacks"]
    encoder_symbols = ["Compress", "MaxCompressedSize", "CreateInstance", "DestroyInstance",
        "SetParameter", "CompressStream", "HasMoreOutput", "IsFinished",
        "PrepareDictionary", "AttachPreparedDictionary", "DestroyPreparedDictionary"]
    expected = {**{f"BrotliDecoder{name}": "libbrotlidec.a(decode.c.o)" for name in decoder_symbols},
        **{f"BrotliEncoder{name}": "libbrotlienc.a(encode.c.o)" for name in encoder_symbols},
        **{f"BrotliDecoderState{name}": "libbrotlidec.a(state.c.o)" for name in
           ["Init", "MetablockBegin", "CleanupAfterMetablock", "Cleanup"]},
        "BrotliDecoderHuffmanTreeGroupInit": "libbrotlidec.a(state.c.o)",
        "BrotliDefaultAllocFunc": "libbrotlicommon.a(platform.c.o)",
        "BrotliDefaultFreeFunc": "libbrotlicommon.a(platform.c.o)",
        "BrotliSharedDictionaryCreateInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliSharedDictionaryDestroyInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliSharedDictionaryAttach": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliGetDictionary": "libbrotlicommon.a(dictionary.c.o)",
        "BrotliGetTransforms": "libbrotlicommon.a(transform.c.o)"}
    originals = {}
    report = {"source_pin": PIN, "source": {}, "gates": {},
              "streaming_entry_implemented": True, "one_shot_stack_entry_implemented": False}
    for graph in ["ordinary", "sanitized"]:
        build = builds / f"source-{graph}"
        configure = ["cmake", "-S", ROOT / "tools/oracle/brotli_stream", "-B", build,
            "-DCMAKE_BUILD_TYPE=Release", f"-DCMAKE_C_COMPILER={shutil.which('clang')}", f"-DCMAKE_CXX_COMPILER={shutil.which('clang++')}",
            f"-DORACLE_SANITIZERS={'ON' if graph == 'sanitized' else 'OFF'}"]
        run(f"{graph}-configure", configure)
        # CMake can reset user options after a changed cached compiler. Apply
        # the exact options again after that reset; strict command verification
        # below still rejects every source unit lacking full instrumentation.
        run(f"{graph}-configure-confirm", configure)
        run(f"{graph}-build", ["cmake", "--build", build, "--target", "brotli_stream_oracle", "--parallel", "2"])
        binary = build / "brotli_stream_oracle"
        originals[graph] = binary
        report["source"][graph] = verify(binary, graph == "sanitized", "brotli_stream.map",
                                            ["brotli_stream.cc"], expected)

    encodes = encoder_requests()
    params = sorted({(first, second) for _, _, first, second in metadata_parameters()})
    extras, _ = extra_encoder_requests()
    all_encodes = encodes + [f"metadata {a} {b}" for a, b in params] + extras
    write_requests(out / "encoder-requests.txt", all_encodes)
    for graph, binary in originals.items():
        execute_observer(binary, out / "encoder-requests.txt", out / f"encoder-{graph}.txt")
    assert sha(out / "encoder-ordinary.txt") == sha(out / "encoder-sanitized.txt")
    rows = (out / "encoder-ordinary.txt").read_text().splitlines()
    assert len(rows) == len(all_encodes) and all(row.startswith("1 ") for row in rows)
    encoded = [row.split()[1] for row in rows]
    streams = encoded[:315]
    metadata_streams = dict(zip(params, encoded[315:315+len(params)]))
    extra_streams = encoded[315+len(params):]
    requests, ranges = decoder_requests(streams, metadata_streams, extra_streams)
    report.update(requests=len(requests), encoder_requests=len(all_encodes), ranges=ranges)
    request_path = out / "requests.txt"
    write_requests(request_path, requests)
    request_hashes = segment_hashes(request_path, ranges)
    for name, (expected_request, _) in RETAINED.items():
        assert request_hashes[name] == expected_request, name
    report["retained_request_segments"] = request_hashes

    capture_path = ROOT / "tools/captures/brotli_stream.tsv"
    captures = [row.split("\t", 2) for row in capture_path.read_text().splitlines()]
    write_requests(out / "capture-requests.txt", [request for _, request, _ in captures])
    expected_captures = "".join(response + "\n" for _, _, response in captures).encode()
    for graph, binary in originals.items():
        execute_observer(binary, out / "capture-requests.txt", out / f"captures-{graph}.txt")
        assert (out / f"captures-{graph}.txt").read_bytes() == expected_captures
        execute_observer(binary, request_path, out / f"{graph}.txt")
        hashes = segment_hashes(out / f"{graph}.txt", ranges)
        for name, (_, expected_output) in RETAINED.items():
            assert hashes[name] == expected_output, (graph, name)
        # All548 previous original-only callback traces are retained at the
        # end of the8,542-row extended segment; now also executed by Rust.
        start, end = ranges["extended"]
        metadata_hash = hashlib.sha256()
        with (out / f"{graph}.txt").open("rb") as source:
            for index, line in enumerate(source):
                if end-548 <= index < end:
                    metadata_hash.update(line)
        assert metadata_hash.hexdigest() == "0873040ce39ac83c74190c62461e4ef0d48e947d1c5a47d58b6f002df5383bac"
        report["source"][graph].update(observation_sha256=sha(out / f"{graph}.txt"),
            observation_bytes=(out / f"{graph}.txt").stat().st_size,
            retained_observation_segments=hashes, retained_metadata548_sha256=metadata_hash.hexdigest())
    assert sha(out / "ordinary.txt") == sha(out / "sanitized.txt")

    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.85", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings", "-C", f"opt-level={level}"]
        binary = out / f"rust-{name}"
        run(f"{name}-build", [*compiler, ROOT / "tools/brotli_stream_probe.rs", "-o", binary])
        execute_observer(binary, request_path, out / f"rust-{name}.txt")
        assert sha(out / f"rust-{name}.txt") == sha(out / "ordinary.txt"), name
        hashes = segment_hashes(out / f"rust-{name}.txt", ranges)
        for segment, (_, expected_output) in RETAINED.items():
            assert hashes[segment] == expected_output, (name, segment)
        tests = out / f"{name}-tests"
        run(f"{name}-tests-build", [*compiler, "--test", ROOT / "tests/brotli_stream.rs", "-o", tests])
        for graph in originals:
            # Explicit six terminal native test logs; original source captures
            # were independently re-observed under both full C graphs above.
            run(f"{name}-{graph}-tests", [tests, "--test-threads=2"])
            report["gates"][f"{name}-{graph}"] = {"tests": 10, "ignored": 0,
                "requests": len(requests), "source_captures": len(captures),
                "source_binary_sha256": sha(originals[graph]), "rust_binary_sha256": sha(binary),
                "observation_sha256": sha(out / f"rust-{name}.txt"),
                "retained_observation_segments": hashes}
    (out / "clippy.toml").write_text('msrv = "1.85"\n')
    run("clippy", ["clippy-driver", "--edition=2024", "--test", ROOT / "tests/brotli_stream.rs",
        "-D", "warnings", "-o", out / "clippy-tests"], dict(os.environ, CLIPPY_CONF_DIR=str(out)))
    report["files"] = {path.name: sha(path) for path in out.iterdir() if path.is_file()}
    (out / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(out / "report.json")


if __name__ == "__main__":
    main()
