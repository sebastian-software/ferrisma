#!/usr/bin/env python3
"""Build and audit unchanged original skcms fitting entry points and helpers.

Public calls stay in the complete original archive graph. A separate helper
executable includes pristine skcms.cc and inherits its exact actual compiler
command, changing only the observation TU/output and two adapter defines.
"""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import re
import shlex
import struct
import subprocess
from skcms_binding import live_bindings

ROOT = Path(__file__).resolve().parent.parent
COMMANDS = ["skcms-fit-approx", "skcms-fit-roundtrip", "skcms-fit-gradient",
            "skcms-fit-step", "skcms-fit-nonlinear", "skcms-fit-checked"]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def source_command(build):
    entries = json.loads((build / "compile_commands.json").read_text())
    entry, = [e for e in entries if Path(e["file"]).name == "skcms.cc"]
    command = entry.get("arguments") or shlex.split(entry["command"])
    assert [x for x in command if x.startswith("-O")] == ["-O3"]
    assert "-DNDEBUG" in command
    assert not any(x.startswith(("-ffast-math", "-ffp-contract", "-ffp-model",
                                 "-funsafe-math", "-fno-signed-zeros")) for x in command)
    wanted = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"] if build.name == "oracle-asan" else []
    assert [x for x in command if "sanitize" in x] == wanted
    source = Path(entry["file"]).resolve().parent
    assert subprocess.check_output(["git", "-C", source, "rev-parse", "HEAD"], text=True).strip() == "96d9171c94b937a1b5f0293de7309ac16311b722"
    assert not subprocess.check_output(["git", "-C", source, "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    return entry, command


def build_private(build):
    build = Path(build).resolve()
    entry, original = source_command(build)
    obj, binary = build / "skcms_fitting_private.o", build / "skcms_fitting_private"
    command = original.copy()
    command[command.index("-o") + 1] = str(obj)
    command[command.index("-c") + 1] = str(ROOT / "tools/oracle/skcms_fitting.cc")
    command += ["-DSKCMS_FIT_PRIVATE", "-DSKCMS_FIT_STANDALONE"]
    with (build / "skcms_fitting_private-compile.log").open("w") as log:
        subprocess.run(command, cwd=entry["directory"], stdout=log, stderr=subprocess.STDOUT, check=True)
    link = [command[0], str(obj), str(build / "upstream/lib/libjxl_cms.a"), "-o", str(binary)]
    if platform.system() == "Linux":
        link += ["-stdlib=libc++"]
    if build.name == "oracle-asan":
        link += ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"]
    mapping = build / "skcms_fitting_private.map"
    link += [("-Wl,-map," if platform.system() == "Darwin" else "-Wl,-Map,") + str(mapping)]
    with (build / "skcms_fitting_private-link.log").open("w") as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    receipt = {"original": entry, "compile": command, "link": link,
               "source_sha256": sha(Path(entry["file"])), "observer_sha256": sha(ROOT / "tools/oracle/skcms_fitting.cc"),
               "object_sha256": sha(obj), "archive_sha256": sha(build / "upstream/lib/libjxl_cms.a"),
               "binary_sha256": sha(binary), "map_sha256": sha(mapping)}
    (build / "skcms_fitting_private.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return binary


def retained_requests():
    path = ROOT / "tests/data/skcms-fitting-requests.bin"
    manifest = json.loads((ROOT / "tests/data/skcms-fitting-requests.json").read_text())
    assert sha(path) == manifest["inventory"]["binary_sha256"]
    data, offset, rows = path.read_bytes(), 0, []
    while offset < len(data):
        tag = data[offset]
        count, = struct.unpack_from("<I", data, offset + 1)
        words = struct.unpack_from("<" + "I" * count, data, offset + 5)
        args = [word - (1 << 32) if i in (4, 7, 8) and word >= 1 << 31 else word for i, word in enumerate(words)]
        rows.append(COMMANDS[tag] + " " + " ".join(map(str, args)))
        offset += 5 + 4 * count
    assert offset == len(data) and len(rows) == 11432
    assert hashlib.sha256(("\n".join(rows) + "\n").encode()).hexdigest() == manifest["inventory"]["text_sha256"]
    return rows


def observe(binary, requests):
    result = subprocess.run([str(binary)], input="\n".join(requests) + "\n", text=True, capture_output=True)
    assert result.returncode == 0 and not result.stderr, (binary, result.returncode, result.stderr[:1500])
    rows = result.stdout.splitlines()
    assert len(rows) == len(requests)
    return rows


def graph_binding(build, directory):
    entry, original = source_command(build)
    receipt = json.loads((build / "skcms_fitting_private.json").read_text())
    expected = original.copy()
    expected[expected.index("-o") + 1] = str(build / "skcms_fitting_private.o")
    expected[expected.index("-c") + 1] = str(ROOT / "tools/oracle/skcms_fitting.cc")
    expected += ["-DSKCMS_FIT_PRIVATE", "-DSKCMS_FIT_STANDALONE"]
    assert receipt["original"] == entry and receipt["compile"] == expected
    for key, path in {"source_sha256": Path(entry["file"]),
                      "observer_sha256": ROOT / "tools/oracle/skcms_fitting.cc",
                      "object_sha256": build / "skcms_fitting_private.o",
                      "archive_sha256": build / "upstream/lib/libjxl_cms.a",
                      "binary_sha256": build / "skcms_fitting_private",
                      "map_sha256": build / "skcms_fitting_private.map"}.items():
        assert receipt[key] == sha(path), key
    link = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
    bound, mapping = directory / "public-bound", directory / "public.map"
    link[link.index("-o") + 1] = str(bound)
    link.append(("-Wl,-map," if platform.system() == "Darwin" else "-Wl,-Map,") + str(mapping))
    with (directory / "public-link.log").open("w") as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    public = ["skcms_ApproximateCurve", "skcms_MaxRoundtripError", "skcms_AreApproximateInverses",
              "skcms_TransferFunction_eval", "skcms_TransferFunction_invert", "skcms_Matrix3x3_invert"]
    public_bindings = live_bindings(bound, mapping, {
        name: (r"_?" + name, r"libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms\.cc\.o\)$") for name in public})
    private = ["rg_nonlinear", "gauss_newton_step", "max_roundtrip_error_checked", "fit_nonlinear"]
    private_bindings = live_bindings(build / "skcms_fitting_private", build / "skcms_fitting_private.map", {
        name: (r"_+ZL" + str(len(name)) + name + r".*", r"(?:^|/)skcms_fitting_private\.o$") for name in private})
    helper_map = (build / "skcms_fitting_private.map").read_text()
    assert not re.search(r"libjxl_cms\.a[^\n]*skcms\.cc\.o", helper_map), "duplicate original public skcms object"
    assert re.search(r"libjxl_cms\.a[^\n]*skcms_TransformBaseline\.cc\.o", helper_map), "missing original baseline transform dependency"
    return {"private_command_receipt": receipt, "public_bindings": public_bindings,
            "private_bindings": private_bindings, "public_map_sha256": sha(mapping)}


def audit(output):
    from check import check_source
    check_source()
    output.mkdir(parents=True, exist_ok=False)
    requests = retained_requests()
    public = [q for q in requests if q.split()[0] in COMMANDS[:2]]
    report = {"source_pin": "96d9171c94b937a1b5f0293de7309ac16311b722", "requests": len(requests),
              "public_requests": len(public), "graphs": {}}
    previous = None
    for graph in ["oracle", "oracle-asan"]:
        build = ROOT / "target" / graph
        directory = output / graph
        directory.mkdir()
        active = [build / "jxl_oracle", build / "compile_commands.json",
                  build / "CMakeFiles/jxl_oracle.dir/link.txt", build / "upstream/lib/libjxl_cms.a",
                  build / "skcms_fitting_private", build / "skcms_fitting_private.o",
                  build / "skcms_fitting_private.json", build / "skcms_fitting_private.map"]
        before = {str(p): sha(p) for p in active}
        binding = graph_binding(build, directory)
        observations = observe(build / "skcms_fitting_private", requests)
        direct = observe(build / "jxl_oracle", public)
        expected = [value for request, value in zip(requests, observations) if request.split()[0] in COMMANDS[:2]]
        assert direct == expected, "public archive/private-include compilation-context difference"
        if previous is not None:
            assert observations == previous, "ordinary/nonrecovering sanitizer original drift"
        previous = observations
        path = directory / "observations.txt"
        path.write_text("\n".join(observations) + "\n")
        (directory / "public.txt").write_text("\n".join(direct) + "\n")
        assert before == {str(p): sha(p) for p in active}, "active graph changed during audit"
        report["graphs"][graph] = {"bindings": binding, "active_sha256": before,
                                    "observations_sha256": sha(path)}
    report["all_original_exact"] = True
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"requests": len(requests), "public_requests": len(public),
                      "public_bindings_each": 6, "private_bindings_each": 4, "all_original_exact": True}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-private", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    if args.build_private:
        if args.output:
            parser.error("choose build or audit")
        print(build_private(args.build_private))
        return
    if args.output:
        audit(args.output.resolve())
        return
    parser.error("provide --build-private or --output")


if __name__ == "__main__":
    main()
