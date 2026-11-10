#!/usr/bin/env python3
"""Fail on missing tests, failures, changed skip counts, or mismatched case counts."""
import argparse
import hashlib
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
            # Compare skipped case names, not their nondeterministic elapsed time.
            "gtest_skipped": re.findall(r"^\[  SKIPPED \] (.+?)(?: \([\d.]+ ms\))?$", output, re.M),
        }
    return records


def case_identities(path, registrations):
    # LastTest.log retains complete output, including deliberately invalid
    # UTF-8 diagnostics that cannot be decoded as one text document. JUnit
    # output may truncate passing tests after 1,024 bytes.
    data = path.read_bytes()
    blocks = re.findall(rb"^\d+/\d+ Testing: ([^\r\n]+)\r?\n(.*?)(?=^\d+/\d+ Testing: |\Z)",
                        data, re.M | re.S)
    identities = {}
    for raw_name, output in blocks:
        name = raw_name.decode("utf-8")
        if name in identities or name not in registrations:
            raise SystemExit(f"Duplicate or unexpected CTest log registration: {name}")
        counts = [int(value) for value in re.findall(rb"\[==========\] Running (\d+) tests? from", output)]
        names = re.findall(rb"^\[ RUN      \] ([^\r\n]+)\r?$", output, re.M)
        if counts != registrations[name]["gtest_cases"] or len(names) != sum(counts):
            raise SystemExit(f"Incomplete GoogleTest case identities in {path}: {name}")
        identities[name] = [value.decode("utf-8") for value in names]
    if identities.keys() != registrations.keys():
        raise SystemExit(f"Missing CTest log registrations in {path}")
    return identities


def case_outcomes(path, registrations):
    """Read every case's terminal result from complete output, not truncated XML."""
    identities = case_identities(path, registrations)
    blocks = re.findall(rb"^\d+/\d+ Testing: ([^\r\n]+)\r?\n(.*?)(?=^\d+/\d+ Testing: |\Z)",
                        path.read_bytes(), re.M | re.S)
    outcomes = {}
    for raw_name, output in blocks:
        name = raw_name.decode('utf-8')
        active, cases = None, []
        for line in output.splitlines():
            start = re.fullmatch(rb'\[ RUN      \] (.+)', line)
            if start:
                if active is not None:
                    raise SystemExit(f'Missing original case result in {path}: {active}')
                active = start[1].decode('utf-8')
            finish = re.fullmatch(rb'\[ *(OK|SKIPPED|FAILED) \] (.+) \([\d.]+ ms\)', line)
            if finish:
                if active is None or finish[2].decode('utf-8') != active:
                    raise SystemExit(f'Unexpected original case result in {path}: {line!r}')
                if finish[1] == b'FAILED':
                    raise SystemExit(f'Failed original case in {path}: {active}')
                cases.append(dict(name=active, status=finish[1].decode('ascii')))
                active = None
        if active is not None or [case['name'] for case in cases] != identities[name]:
            raise SystemExit(f'Incomplete original case outcomes in {path}: {name}')
        outcomes[name] = cases
    return outcomes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=ROOT / "build/baseline-results.xml")
    parser.add_argument("--port", type=Path, default=ROOT / "build/port-results.xml")
    parser.add_argument("--report", type=Path, default=ROOT / "build/parity-summary.json")
    parser.add_argument("--baseline-log", type=Path, help="Complete original CTest LastTest.log")
    parser.add_argument("--port-log", type=Path, help="Complete Rust CTest LastTest.log")
    args = parser.parse_args()
    if bool(args.baseline_log) != bool(args.port_log):
        parser.error("Provide both complete CTest logs to compare original case identities")
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
    if args.baseline_log:
        original = case_identities(args.baseline_log, baseline)
        candidate = case_identities(args.port_log, port)
        if original != candidate:
            changed = sorted(name for name in original if original[name] != candidate[name])
            raise SystemExit("Original ordered GoogleTest case names differ: " + ", ".join(changed))
        original_outcomes = case_outcomes(args.baseline_log, baseline)
        candidate_outcomes = case_outcomes(args.port_log, port)
        if original_outcomes != candidate_outcomes:
            changed = sorted(name for name in original_outcomes
                             if original_outcomes[name] != candidate_outcomes[name])
            raise SystemExit('Original ordered GoogleTest outcomes or dynamic skips differ: ' + ', '.join(changed))
        summary["gtest_case_identity"] = {
            "exact_ordered_case_names_match": True,
            "executed_cases": sum(len(names) for names in candidate.values()),
            "case_names_sha256_by_ctest_registration": {
                name: hashlib.sha256(json.dumps(names).encode()).hexdigest()
                for name, names in sorted(candidate.items())
            },
            "baseline_complete_log_sha256": hashlib.sha256(args.baseline_log.read_bytes()).hexdigest(),
            "port_complete_log_sha256": hashlib.sha256(args.port_log.read_bytes()).hexdigest(),
        }
        summary['gtest_case_outcomes'] = dict(exact_ordered_outcomes_and_dynamic_skips_match=True,
            skipped_cases_by_ctest_registration={name: [case['name'] for case in cases if case['status'] == 'SKIPPED']
                                                for name, cases in candidate_outcomes.items()},
            outcomes_sha256_by_ctest_registration={name: hashlib.sha256(json.dumps(cases).encode()).hexdigest()
                                                  for name, cases in candidate_outcomes.items()})
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(summary, indent=2) + "\n")
    print(f"Identical passing inventory: {len(port)} CTest registrations, {summary['gtest_cases']} GoogleTest cases")


if __name__ == "__main__":
    main()
