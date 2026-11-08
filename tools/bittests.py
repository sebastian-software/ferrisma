#!/usr/bin/env python3
"""Inventory and run upstream's bit tests against the oracle (ADR 0002).

Upstream's CMakeLists.txt declares its MD5 bit tests with add_bittest(PROG
NAME ARGS OUTFILE INFILE MD5SUM): a run test `<PROG>-static-<NAME>` and a
check `<PROG>-static-<NAME>-cmp`. This script reads them from the CTest suite
that tools/build_oracle.py configured in build/oracle/<variant>/libjpeg-turbo
(`ctest --show-only=json-v1`), so the inventory is upstream's, not a copy.

tools/bittests.json classifies every case: `in` for a djpeg case on the 8-bit
decompression path of ADR 0001, `out` with a reason for everything else. The
in-scope cases are the ones work package 7 runs on the port: after a
successful run, this script writes bittests-port.txt next to oracle.json, and
tests/wp7_bittests.rs runs each listed case through its Rust djpeg driver and
compares the outputs with upstream's MD5 sums.

  --check   verify that tools/bittests.json lists exactly the cases that the
            configured suite declares, with the same arguments and inputs
  --run     run the in-scope djpeg cases (run test, MD5 check and any further
            check of the same output, such as the ICC profile of rgb-islow)
            plus every test they depend on, which produces their input files
            with the oracle's cjpeg, and those producers' MD5 checks; writes
            bittests-result.json and bittests-port.txt next to oracle.json
  --all     with --run: run upstream's whole CTest suite instead, as a check of
            the oracle itself
"""

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
TESTIMAGES = ROOT / "upstream/libjpeg-turbo/testimages"
MANIFEST = ROOT / "tools/bittests.json"
PROGRAMS = ("cjpeg", "cjpeg12", "cjpeg16", "djpeg", "djpeg12", "djpeg16",
            "jpegtran", "jpegtran12")
REASONS = {
    "encoder": "cjpeg compresses; the encoder is not translated (ADR 0001)",
    "transcoding": "jpegtran transcodes; transcoding is not translated (ADR 0001)",
    "12-bit": "12-bit samples are not translated (ADR 0001)",
    "16-bit": "16-bit samples are not translated (ADR 0001)",
    "lossless": "lossless JPEG is not translated (ADR 0001)",
    "quantized": "color quantization (-colors) is not translated (ADR 0001)",
    "rgb565": "RGB565 output is not translated (ADR 0001)",
}
# Options that select an out-of-scope path of djpeg; an in-scope case must not
# use them.
OUT_OF_SCOPE_OPTIONS = {"-rgb565", "-colors", "-quantize", "-onepass", "-map", "-gif",
                        "-precision"}


def build_dir(variant):
    return ROOT / "build/oracle" / variant / "libjpeg-turbo"


def ctest_tests(build):
    if not (build / "CTestTestfile.cmake").is_file():
        raise SystemExit(f"{build} holds no CTest suite; run python3 tools/build_oracle.py")
    listing = subprocess.run(["ctest", "--show-only=json-v1"], cwd=build, check=True,
                             stdout=subprocess.PIPE, universal_newlines=True).stdout
    tests = {}
    for test in json.loads(listing)["tests"]:
        properties = {item["name"]: item["value"] for item in test.get("properties", [])}
        tests[test["name"]] = {"command": test["command"],
                               "depends": properties.get("DEPENDS") or []}
    return tests


def portable(argument, build):
    """Replace absolute paths in a test argument by stable relative ones."""
    for prefix, name in ((str(TESTIMAGES) + "/", "testimages/"), (str(build) + "/", "")):
        argument = argument.replace(prefix, name)
    return argument


def inventory(build):
    """Return the add_bittest cases of the configured suite, in suite order."""
    tests = ctest_tests(build)
    pattern = re.compile(rf"^({'|'.join(PROGRAMS)})-static-(.+)$")
    cases = []
    for name, test in tests.items():
        match = pattern.match(name)
        if not match or name.endswith("-cmp") or f"{name}-cmp" not in tests:
            continue
        arguments = [portable(argument, build) for argument in test["command"][1:]]
        outfile = arguments.index("-outfile")
        checks = [other for other, data in tests.items()
                  if other.startswith(f"{name}-") and other.endswith("-cmp")
                  and name in data["depends"]]
        md5 = tests[f"{name}-cmp"]["command"]
        cases.append({
            "program": match[1],
            "name": match[2],
            "test": name,
            "args": arguments[:outfile],
            "input": arguments[-1],
            "md5": md5[1],
            "checks": checks,
        })
    return cases, tests


def load():
    return json.loads(MANIFEST.read_text())


def verify(manifest, cases):
    problems = []
    listed = {(entry["program"], entry["name"]): entry for entry in manifest["cases"]}
    found = {(case["program"], case["name"]): case for case in cases}
    for key in sorted(set(found) - set(listed)):
        problems.append(f"bit test missing from tools/bittests.json: {key[0]} {key[1]}")
    for key in sorted(set(listed) - set(found)):
        problems.append(f"tools/bittests.json lists an unknown bit test: {key[0]} {key[1]}")
    if len(listed) != len(manifest["cases"]):
        problems.append("tools/bittests.json lists a case twice")
    for key in sorted(set(found) & set(listed)):
        entry, case = listed[key], found[key]
        label = f"{key[0]} {key[1]}"
        for field in ("args", "input"):
            if entry[field] != case[field]:
                problems.append(f"{label}: {field} {entry[field]!r} differs from upstream "
                                f"{case[field]!r}")
        if entry["scope"] == "in":
            if entry["program"] != "djpeg":
                problems.append(f"{label}: only 8-bit djpeg cases can be in scope")
            if "reason" in entry:
                problems.append(f"{label}: an in-scope case has no reason")
            if not entry.get("covers"):
                problems.append(f"{label}: an in-scope case names what it covers")
            used = OUT_OF_SCOPE_OPTIONS.intersection(entry["args"])
            if used:
                problems.append(f"{label}: in scope, but uses {sorted(used)}")
        elif entry["scope"] == "out":
            if entry.get("reason") not in REASONS:
                problems.append(f"{label}: out of scope needs a reason from {sorted(REASONS)}")
        else:
            problems.append(f"{label}: scope must be 'in' or 'out'")
    if manifest.get("reasons") != REASONS:
        problems.append("tools/bittests.json reasons differ from REASONS in tools/bittests.py")
    counts = manifest.get("counts", {})
    expected = summary(manifest)
    if counts != expected:
        problems.append(f"tools/bittests.json counts {counts} differ from the cases {expected}")
    return problems


def summary(manifest):
    counts = {"cases": len(manifest["cases"]),
              "djpeg_cases": sum(entry["program"].startswith("djpeg")
                                 for entry in manifest["cases"]),
              "in_scope": sum(entry["scope"] == "in" for entry in manifest["cases"])}
    for reason in REASONS:
        counts[f"out_{reason}"] = sum(entry.get("reason") == reason
                                      for entry in manifest["cases"])
    return counts


def closure(names, tests):
    """Return names plus every test they depend on, transitively."""
    selected, pending = set(), list(names)
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        selected.add(name)
        pending.extend(tests[name]["depends"])
    return selected


def run(variant, everything, jobs):
    build = build_dir(variant)
    manifest = load()
    cases, tests = inventory(build)
    problems = verify(manifest, cases)
    if problems:
        raise SystemExit("\n".join(problems))
    in_scope = [case for case in cases
                if next(entry for entry in manifest["cases"]
                        if (entry["program"], entry["name"]) == (case["program"], case["name"])
                        )["scope"] == "in"]
    if everything:
        selected = set(tests)
    else:
        wanted = []
        for case in in_scope:
            wanted += [case["test"], f"{case['test']}-cmp", *case["checks"]]
        selected = closure(wanted, tests)
        # The producers' own MD5 checks prove that the inputs equal upstream's.
        selected |= {f"{name}-cmp" for name in selected if f"{name}-cmp" in tests}
    for name in selected:
        if not re.fullmatch(r"[A-Za-z0-9_.-]+", name):
            raise SystemExit(f"unexpected character in test name {name!r}")
    regex = "^(" + "|".join(sorted(re.escape(name).replace("\\-", "-")
                                   for name in selected)) + ")$"
    print(f"Running {len(selected)} CTest test(s) in {build}: "
          + ("the whole upstream suite" if everything else
             f"{len(in_scope)} in-scope djpeg case(s) and their input producers"), flush=True)
    result = subprocess.run(["ctest", "--output-on-failure", "-j", str(jobs), "-R", regex],
                            cwd=build)
    record = {
        "variant": variant,
        "selection": "all" if everything else "in-scope",
        "tests_run": len(selected),
        "in_scope_cases": [case["test"] for case in in_scope],
        "passed": result.returncode == 0,
    }
    (build.parent / "bittests-result.json").write_text(json.dumps(record, indent=2) + "\n")
    if result.returncode != 0:
        raise SystemExit(f"{variant}: upstream bit tests failed")
    write_port_manifest(build, in_scope, tests)
    print(f"{variant}: {len(selected)} test(s) passed", flush=True)


def write_port_manifest(build, in_scope, tests):
    """Write bittests-port.txt: each in-scope case with its input, arguments,
    output file and MD5 checks, for tests/wp7_bittests.rs. One field per line,
    so that the Rust test needs no parser beyond splitting lines."""
    lines = ["# In-scope djpeg bit tests for tests/wp7_bittests.rs, written by "
             "tools/bittests.py --run.",
             "# Inputs are absolute; output and check files are names of the run's "
             "files in the oracle build directory."]
    for case in in_scope:
        command = [portable(argument, build) for argument in tests[case["test"]]["command"]]
        source = case["input"]
        if source.startswith("testimages/"):
            source = TESTIMAGES / source[len("testimages/"):]
        else:
            source = build / source
        lines += [f"case {case['test']}", f"input {source}"]
        lines += [f"arg {argument}" for argument in case["args"]]
        lines.append(f"outfile {command[command.index('-outfile') + 1]}")
        for check in case["checks"]:
            md5cmp = [portable(argument, build) for argument in tests[check]["command"]]
            lines.append(f"check {md5cmp[2]} {md5cmp[1]}")
        lines.append("end")
    (build.parent / "bittests-port.txt").write_text("\n".join(lines) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", default="release", help="oracle variant (default: release)")
    parser.add_argument("--check", action="store_true", help="verify tools/bittests.json")
    parser.add_argument("--run", action="store_true", help="run the in-scope cases")
    parser.add_argument("--all", action="store_true", help="with --run: run the whole suite")
    parser.add_argument("--jobs", type=int, default=4, help="parallel CTest jobs")
    parser.add_argument("--print-inventory", action="store_true",
                        help="print the configured suite's cases as JSON")
    args = parser.parse_args()
    if args.print_inventory:
        print(json.dumps(inventory(build_dir(args.variant))[0], indent=2))
        return
    if args.check or not args.run:
        problems = verify(load(), inventory(build_dir(args.variant))[0])
        for problem in problems:
            print(problem)
        if problems:
            raise SystemExit(f"{len(problems)} bit test inventory problem(s)")
        counts = summary(load())
        print(f"Bit test inventory verified: {counts['cases']} cases, "
              f"{counts['djpeg_cases']} djpeg, {counts['in_scope']} in scope", flush=True)
    if args.run:
        run(args.variant, args.all, args.jobs)


if __name__ == "__main__":
    sys.exit(main())
