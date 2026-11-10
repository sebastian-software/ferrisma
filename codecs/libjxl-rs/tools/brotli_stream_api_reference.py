#!/usr/bin/env python3
"""Actual source API pointer/alias/lifetime bindings, separately from frozen core.

No source algorithm/configuration changes. Both original graphs call actual
public APIs; out-of-range shared-dictionary enum inputs use a C bridge instead
of an undefined out-of-range C++ enum conversion. Scalar aliases name genuine
size_t members. Read-only callback handles point to the actual native state.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess

from brotli_decoder_binding import verify
from brotli_state_reference import ROOT, PIN
from brotli_stream_reference import sha, execute_observer, write_requests
from brotli_stream_corpus import encoder_requests, extra_encoder_requests
from brotli_stream_api_corpus import requests as api_requests


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

    report = {"source_pin": PIN, "source": {}, "gates": {}, "ordinary_native": "supported target only"}
    expected = {**{f"BrotliDecoder{name}": "libbrotlidec.a(decode.c.o)" for name in
        ["CreateInstance", "DestroyInstance", "DecompressStream", "AttachDictionary", "TakeOutput",
         "GetErrorCode", "IsUsed", "IsFinished", "HasMoreOutput", "SetMetadataCallbacks"]},
        "BrotliStreamAPIAttach": "brotli_stream_api_types.c.o",
        **{f"BrotliDecoderState{name}": "libbrotlidec.a(state.c.o)" for name in
           ["Init", "MetablockBegin", "CleanupAfterMetablock", "Cleanup"]},
        "BrotliDecoderHuffmanTreeGroupInit": "libbrotlidec.a(state.c.o)",
        "BrotliSharedDictionaryCreateInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliSharedDictionaryDestroyInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliSharedDictionaryAttach": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliGetDictionary": "libbrotlicommon.a(dictionary.c.o)",
        "BrotliGetTransforms": "libbrotlicommon.a(transform.c.o)"}
    seed_expected = {**{f"BrotliEncoder{name}": "libbrotlienc.a(encode.c.o)" for name in
        ["Compress", "MaxCompressedSize", "CreateInstance", "DestroyInstance", "SetParameter",
         "CompressStream", "HasMoreOutput", "IsFinished", "PrepareDictionary",
         "AttachPreparedDictionary", "DestroyPreparedDictionary"]},
        "BrotliDecoderCreateInstance": "libbrotlidec.a(decode.c.o)",
        "BrotliDecoderDecompressStream": "libbrotlidec.a(decode.c.o)"}
    originals, seeds = {}, {}
    for graph in ["ordinary", "sanitized"]:
        build = builds / f"source-{graph}"
        configure = ["cmake", "-S", ROOT / "tools/oracle/brotli_stream_api", "-B", build,
            "-DCMAKE_BUILD_TYPE=Release", f"-DCMAKE_C_COMPILER={shutil.which('clang')}",
            f"-DCMAKE_CXX_COMPILER={shutil.which('clang++')}",
            f"-DORACLE_SANITIZERS={'ON' if graph == 'sanitized' else 'OFF'}"]
        run(f"{graph}-configure", configure)
        run(f"{graph}-configure-confirm", configure)
        run(f"{graph}-build", ["cmake", "--build", build, "--target", "brotli_stream_api_oracle",
                               "brotli_stream_seed_oracle", "--parallel", "2"])
        originals[graph] = build / "brotli_stream_api_oracle"
        seeds[graph] = build / "brotli_stream_seed_oracle"
        report["source"][graph] = {
            "api": verify(originals[graph], graph == "sanitized", "brotli_stream_api.map",
                        ["brotli_stream_api.cc", "brotli_stream_api_types.c"], expected),
            "seed": verify(seeds[graph], graph == "sanitized", "brotli_stream_seed.map",
                            ["brotli_stream.cc"], seed_expected)}
    encodes = encoder_requests()
    extras, _ = extra_encoder_requests()
    encoder_rows = encodes + ["metadata 32 17"] + extras
    write_requests(out / "encoder-requests.txt", encoder_rows)
    for graph, binary in seeds.items():
        execute_observer(binary, out / "encoder-requests.txt", out / f"encoder-{graph}.txt")
    assert sha(out / "encoder-ordinary.txt") == sha(out / "encoder-sanitized.txt")
    encoded = (out / "encoder-ordinary.txt").read_text().splitlines()
    assert len(encoded) == 343 and all(row.startswith("1 ") for row in encoded)
    streams = [row.split()[1] for row in encoded[:315]]
    metadata = encoded[315].split()[1]
    extra_streams = [row.split()[1] for row in encoded[316:]]
    requests = api_requests(streams, metadata, [extra_streams[i] for i in [0, 5, 10]])
    request_path = out / "requests.txt"
    write_requests(request_path, requests)
    old_request = "\n".join(requests[:3024]) + "\n"
    import hashlib
    assert hashlib.sha256(old_request.encode()).hexdigest() == "0219cc9aad2663744bb22d363adce5346381fb0db99c6c7897ae57602f6a3a0c"
    watch_request = "\n".join(requests[3024:]) + "\n"
    assert hashlib.sha256(watch_request.encode()).hexdigest() == "1ff3448ec3b2310b0cb1ed7b29cf1b061e9d7fa4845a71118aaa0bbf8a4121ef"
    captures = [row.split("\t", 2) for row in (ROOT / "tools/captures/brotli_stream_api.tsv").read_text().splitlines()]
    write_requests(out / "capture-requests.txt", [request for _, request, _ in captures])
    capture_bytes = "".join(response + "\n" for _, _, response in captures).encode()
    def check(path):
        old, watch = hashlib.sha256(), hashlib.sha256()
        with path.open("rb") as source:
            count = 0
            for index, row in enumerate(source):
                (old if index < 3024 else watch).update(row)
                count += 1
        assert count == 3084
        assert old.hexdigest() == "ec7a6fbc25cee241b857276ff5a533494be0cf6be4de34c72e12867d633ab89f"
        assert watch.hexdigest() == "cc6dff8e88fa1ae3f422fab2f231d7e50aeadfb6c0c7da4a73322ce8d404fe40"
    for graph, binary in originals.items():
        execute_observer(binary, out / "capture-requests.txt", out / f"captures-{graph}.txt")
        assert (out / f"captures-{graph}.txt").read_bytes() == capture_bytes
        execute_observer(binary, request_path, out / f"{graph}.txt")
        check(out / f"{graph}.txt")
    assert sha(out / "ordinary.txt") == sha(out / "sanitized.txt")
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.95", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition=2024", "-D", "warnings", "-C", f"opt-level={level}"]
        binary = out / f"rust-{name}"
        run(f"{name}-build", [*compiler, ROOT / "tools/brotli_stream_api_probe.rs", "-o", binary])
        execute_observer(binary, request_path, out / f"rust-{name}.txt")
        check(out / f"rust-{name}.txt")
        assert sha(out / f"rust-{name}.txt") == sha(out / "ordinary.txt"), name
        for component in ["stream_api", "stream_api_retention"]:
            tests = out / f"{name}-{component}-tests"
            run(f"{name}-{component}-tests-build", [*compiler, "--test", ROOT / f"tests/brotli_{component}.rs", "-o", tests])
            for graph in originals:
                run(f"{name}-{graph}-{component}-tests", [tests, "--test-threads=2"])
        for graph in originals:
            report["gates"][f"{name}-{graph}"] = {"tests": 5, "api_tests": 4,
                "unchanged_retention_test_in_separate_crate": 1, "ignored": 0, "requests": 3084,
                "retained_initial_api": 3024, "callback_handle_getters": 60,
                "native_api_source_captures": len(captures), "native_frozen_stream_captures": 51,
                "source_binary_sha256": sha(originals[graph]), "rust_binary_sha256": sha(binary),
                "observation_sha256": sha(out / f"rust-{name}.txt")}
    (out / "clippy.toml").write_text('msrv = "1.95"\n')
    for component in ["stream_api", "stream_api_retention"]:
        run(f"{component}-clippy", ["clippy-driver", "--edition=2024", "--test", ROOT / f"tests/brotli_{component}.rs",
            "-D", "warnings", "-o", out / f"{component}-clippy-tests"], dict(os.environ, CLIPPY_CONF_DIR=str(out)))
    report.update(requests=3084, encoder_requests=len(encoder_rows),
                  observation_bytes=(out / "ordinary.txt").stat().st_size,
                  observation_sha256=sha(out / "ordinary.txt"))
    report["files"] = {path.name: sha(path) for path in out.iterdir() if path.is_file()}
    (out / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(out / "report.json")


if __name__ == "__main__":
    main()
