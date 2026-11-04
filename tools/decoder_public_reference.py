#!/usr/bin/env python3
"""Audit the public decoder against the original public C API in both graphs.

Run `python3 tools/check.py --stage test --filter decoder_public::` for the
ordinary graph and again with `--sanitizers` first: the parity tests capture
their exact request corpus, whose inputs come from the original encoder. This
audit binds the original public decoder and encoder entry points to the
complete archive, checks the reference compile configuration, replays every
request on both original graphs, through the C ABI drivers of both graphs
linked to the original and to the Rust library, and on the actual Rust library
in debug, release and minimum-version builds, and requires identical
observations.
"""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import runpy
import shlex
import subprocess
from skcms_binding import live_bindings

ROOT = Path(__file__).resolve().parent.parent
FAMILIES = ["events", "box-buffers", "truncated", "misuse", "rewind", "allocations", "jpeg", "color", "frames", "frame-mutations", "jpeg-output",
            "custom-manager"]
ARCHIVE = r"libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?{}\.o\)$"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def run(command, directory, name, *, data=None, cwd=ROOT):
    result = subprocess.run(command, input=data, capture_output=True, cwd=cwd)
    (directory / (name + ".stdout")).write_bytes(result.stdout)
    (directory / (name + ".stderr")).write_bytes(result.stderr)
    if result.returncode:
        raise RuntimeError(f"{name} failed: {result.returncode}")
    return result


def flags_of(entry):
    return entry.get("arguments") or shlex.split(entry["command"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    check = runpy.run_path(str(ROOT / "tools/check.py"))
    check["check_source"]()
    msrv = check["rust_version"]()

    corpora = {}
    undefined_corpora = {}
    for graph in ["oracle", "oracle-asan"]:
        captured = {}
        undefined = {}
        for family in FAMILIES:
            base = ROOT / "target" / graph / "decoder-public-corpus"
            captured[family] = (base / f"{family}.requests").read_bytes()
            undefined[family] = (base / f"{family}.undefined").read_bytes()
            if not captured[family]:
                raise RuntimeError(f"Empty captured family {family} in {graph}")
        corpora[graph] = captured
        undefined_corpora[graph] = undefined
    if corpora["oracle"] != corpora["oracle-asan"] or undefined_corpora["oracle"] != undefined_corpora["oracle-asan"]:
        raise RuntimeError("Captured request corpora differ between the original graphs")
    requests = b"".join(corpora["oracle"][family] for family in FAMILIES)
    count = len(requests.splitlines())
    # Histories reaching the original's raw-box memcpy with a null input are
    # undefined in the source and run original-only (see ADR 0003).
    undefined_requests = [line for family in FAMILIES
                          for line in undefined_corpora["oracle"][family].splitlines()]
    null_copy = (b"box_content_decoder.cc:110:23: runtime error: null pointer passed as argument 2, "
                 b"which is declared to never be null")

    owned = ["src/jxl/decode.rs", "src/jxl/codestream_header.rs", "src/jxl/base/data_parallel.rs",
             "src/jxl/decoder_input.rs", "src/jxl/decoder_input_cursor.rs",
             "tests/support/decoder_public.rs", "tests/parity/decoder_public.rs",
             "tools/decoder_public_probe.rs", "tools/decoder_public_reference.py",
             "tools/decoder_public_captures.py", "tools/captures/decoder_public.tsv",
             "tests/decoder_public.rs", "tools/oracle/decoder_public.cc",
             "tools/oracle/decoder_public_driver.h", "tools/oracle/decoder_abi.cc",
             "src/jxl/decode_abi.rs"]
    before = {str(ROOT / p): sha((ROOT / p).read_bytes()) for p in owned}
    sources = {}
    original = None
    for graph in ["oracle", "oracle-asan"]:
        build = ROOT / "target" / graph
        directory = output / graph
        directory.mkdir()
        entries = json.loads((build / "compile_commands.json").read_text())
        adapter, = [e for e in entries if Path(e["file"]).resolve() == ROOT / "tools/oracle/decoder_public.cc"]
        units = {name: [e for e in entries if Path(e["file"]).resolve() == ROOT / f"upstream/libjxl/lib/jxl/{name}.cc"]
                 for name in ["decode", "encode", "box_content_decoder", "icc_codec", "decode_to_jpeg"]}
        for name, matches in units.items():
            if len(matches) != 1:
                raise RuntimeError(f"Original {name}.cc compile entry missing")
        expected = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"] if graph == "oracle-asan" else []
        for entry in [adapter, *(m[0] for m in units.values())]:
            flags = flags_of(entry)
            if [f for f in flags if "sanitize" in f] != expected:
                raise RuntimeError(f"Instrumentation changed: {entry['file']}")
            if [f for f in flags if f.startswith("-O")] != ["-O3"] or "-DNDEBUG" not in flags:
                raise RuntimeError(f"Optimization changed: {entry['file']}")
            if any(f.startswith(("-ffast-math", "-ffp-contract", "-ffp-model", "-funsafe-math")) for f in flags):
                raise RuntimeError(f"Unexpected numeric override: {entry['file']}")
        original_units = [e for e in entries if Path(e["file"]).resolve().is_relative_to(ROOT / "upstream/libjxl")
                          and Path(e["file"]).suffix in [".c", ".cc", ".cpp"]]
        for unit in original_units:
            if [f for f in flags_of(unit) if "sanitize" in f] != expected:
                raise RuntimeError(f"Original unit instrumentation changed: {unit['file']}")
        paths = [build / "jxl_oracle", build / "compile_commands.json",
                 build / "CMakeFiles/jxl_oracle.dir/link.txt",
                 build / "CMakeFiles/jxl_oracle.dir/decoder_public.cc.o"]
        for path in paths:
            before[str(path)] = sha(path.read_bytes())
        link = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
        binary = directory / "bound-original"
        mapping = directory / "original.map"
        link[link.index("-o") + 1] = str(binary)
        link.append(("-Wl,-map," if platform.system() == "Darwin" else "-Wl,-Map,") + str(mapping))
        run(link, directory, "link", cwd=build)
        public = {f"Original{name}": (rf"_?JxlDecoder{name}", ARCHIVE.format("decode\\.cc"))
                  for name in ["Create", "Destroy", "Reset", "Rewind", "ProcessInput", "SetInput",
                               "ReleaseInput", "CloseInput", "SubscribeEvents", "GetBasicInfo",
                               "GetExtraChannelInfo", "GetExtraChannelName", "GetColorAsEncodedProfile",
                               "GetICCProfileSize", "GetColorAsICCProfile", "SetBoxBuffer",
                               "ReleaseBoxBuffer", "GetBoxType", "GetBoxSizeRaw", "GetBoxSizeContents",
                               "SetDecompressBoxes", "SizeHintBasicInfo"]}
        public["OriginalEncoderCreate"] = (r"_?JxlEncoderCreate", ARCHIVE.format("encode\\.cc"))
        public["OriginalEncoderAddBox"] = (r"_?JxlEncoderAddBox", ARCHIVE.format("encode\\.cc"))
        public["DecoderPublicCommand"] = (r"_+Z20DecoderPublicCommand[^.]*", r"(?:^|/)decoder_public\.cc\.o$")
        bindings = live_bindings(binary, mapping, public)
        result = run([str(binary)], directory, "observations", data=requests)
        if result.stderr or len(result.stdout.splitlines()) != count:
            raise RuntimeError("Unexpected original output or diagnostic count")
        if original is not None and original != result.stdout:
            raise RuntimeError("Ordinary and instrumented original observations differ")
        original = result.stdout
        run(["python3", "tools/decoder_public_captures.py", "--check", "--oracle", str(binary)],
            directory, "standalone-captures")
        # The same histories through the exported C ABI of either library.
        abi = {}
        for variant in ["original", "rust"]:
            driver = build / f"jxl_abi_{variant}"
            observed = run([str(driver)], directory, f"abi-{variant}-observations", data=requests)
            if observed.stdout != original or observed.stderr:
                raise RuntimeError(f"The {variant} C ABI differs from the original public API")
            abi[variant] = {"binary_sha256": sha(driver.read_bytes()),
                            "observation_sha256": sha(observed.stdout)}
        detects = graph == "oracle-asan" and platform.system() == "Linux"
        diagnosed = 0
        for request in undefined_requests:
            probe = subprocess.run([str(binary)], input=request + b"\n", capture_output=True)
            if detects:
                if (probe.returncode == 0 or null_copy not in probe.stderr
                        or probe.stderr.count(b"runtime error:") != 1):
                    raise RuntimeError("Instrumented original must diagnose the null raw-box copy")
                diagnosed += 1
            elif probe.returncode or probe.stderr:
                raise RuntimeError("Original null raw-box copy probe must complete on this graph")
        sources[graph] = {"bindings": bindings, "compile_commands": [adapter, *(m[0] for m in units.values())],
                          "c_abi_drivers": abi,
                          "instrumented_original_units": len(original_units),
                          "binary_sha256": sha(binary.read_bytes()),
                          "observation_sha256": sha(original),
                          "undefined_null_copy_probes": len(undefined_requests),
                          "undefined_null_copy_diagnosed": diagnosed}

    rust = {}
    for profile, prefix, cargo_flags, rust_flags in [("debug", [], [], []),
                                                     ("release", [], ["--release"], ["-O"]),
                                                     ("msrv", [f"+{msrv}"], [], [])]:
        directory = output / profile
        directory.mkdir()
        built = run(["cargo", *prefix, "build", "--locked", "--offline", "--lib", "--all-features",
                     "--message-format=json", "--target-dir", str(directory / "build"), *cargo_flags],
                    directory, "build")
        artifacts = [r for line in built.stdout.splitlines()
                     if (r := json.loads(line)).get("reason") == "compiler-artifact" and r["target"]["name"] == "jxl"]
        lib, = [Path(f) for f in artifacts[-1]["filenames"] if f.endswith(".rlib")]
        binary = directory / "public-probe"
        run(["rustc", *prefix, "tools/decoder_public_probe.rs", "--edition=2024", "--extern", f"jxl={lib}",
             "-L", f"dependency={lib.parent}", *rust_flags, "-o", str(binary)], directory, "probe-build")
        observed = run([str(binary)], directory, "observations", data=requests)
        if observed.stdout != original or observed.stderr:
            raise RuntimeError(f"Actual Rust {profile} differs from the original public API")
        rust[profile] = {"binary_sha256": sha(binary.read_bytes()), "observation_sha256": sha(observed.stdout)}

    after = {}
    for path in before:
        after[path] = sha(Path(path).read_bytes())
    if after != before:
        raise RuntimeError("The audit altered active sources or original artifacts")
    report = {"source": "a16d32d0ac113f5e54125e49bb571281c5d0b704", "requests": count,
              "families": {family: len(corpora["oracle"][family].splitlines()) for family in FAMILIES},
              "request_sha256": sha(requests), "source_graphs": sources, "rust_graphs": rust,
              "all_five_exact": True, "unchanged_artifacts": before}
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"Both original graphs, their four C ABI drivers and three Rust builds match "
          f"{count:,} public decoder histories")


if __name__ == "__main__":
    main()
