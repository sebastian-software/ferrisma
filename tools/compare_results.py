#!/usr/bin/env python3
"""Fail on missing tests, failures, changed skip counts, or mismatched case counts."""
import argparse
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]


def results(path):
    root = ET.parse(path).getroot()
    cases = list(root.iter("testcase"))
    if not cases:
        raise SystemExit(f"No tests executed in {path}")
    records = {}
    for case in cases:
        if case.find("failure") is not None or case.find("error") is not None:
            raise SystemExit(f"Failed test in {path}: {case.attrib['name']}")
        name = case.attrib["name"]
        if name in records:
            raise SystemExit(f"Duplicate test registration: {name}")
        output = case.findtext("system-out", "")
        counts = re.findall(r"\[==========\] Running (\d+) tests? from", output)
        records[name] = {
            "status": case.attrib.get("status"),
            "skipped": case.find("skipped") is not None,
            "gtest_cases": [int(count) for count in counts],
            "gtest_skipped": re.findall(r"\[  SKIPPED \] (.+)", output),
        }
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=ROOT / "build/baseline-results.xml")
    parser.add_argument("--port", type=Path, default=ROOT / "build/port-results.xml")
    parser.add_argument("--report", type=Path, default=ROOT / "build/parity-summary.json")
    args = parser.parse_args()
    baseline, port = results(args.baseline), results(args.port)
    if baseline != port:
        changed = [name for name in baseline.keys() | port.keys() if baseline.get(name) != port.get(name)]
        raise SystemExit("Test inventory, case counts, or skips differ: " + ", ".join(sorted(changed)))
    if any(case["skipped"] for case in port.values()):
        raise SystemExit("A CTest registration was skipped")
    summary = {
        "ctest_registrations": len(port),
        "gtest_cases": sum(sum(case["gtest_cases"]) for case in port.values()),
        "tests": port,
    }
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(summary, indent=2) + "\n")
    print(f"Identical passing inventory: {len(port)} CTest registrations, {summary['gtest_cases']} GoogleTest cases")


if __name__ == "__main__":
    main()
