#!/usr/bin/env python3
"""Record compact verified parity evidence from the completed local test runs.

The record is written to build/validation-results.json. It is not committed;
attach it to a release or an evidence tag if it has to stay retrievable.
"""
import hashlib
import json
from pathlib import Path
import platform
import subprocess

from compare_results import results

ROOT = Path(__file__).resolve().parents[1]
profiles = [
    ("default", "", ""),
    ("rust-debug", "", "-debug"),
    ("mini", "-mini", "-mini"),
    ("extended-pixi", "-extended-pixi", "-extended-pixi"),
    ("mini-extended-pixi", "-mini-extended-pixi", "-mini-extended-pixi"),
    ("optional-av1", "-libgav1-rav1e-svt-libyuv", "-libgav1-rav1e-svt-libyuv"),
    ("avm", "-avm", "-avm"),
    ("mini-avm", "-mini-avm", "-mini-avm"),
    ("extended-pixi-avm", "-extended-pixi-avm", "-extended-pixi-avm"),
    ("mini-extended-pixi-avm", "-mini-extended-pixi-avm", "-mini-extended-pixi-avm"),
    ("all-features", "-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm", "-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm"),
    ("address-sanitizer", "-asan-llvm16", "-asan-llvm16"),
]
report = {
    "upstream_revision": json.loads((ROOT / "UPSTREAM.json").read_text())["revision"],
    "host": {"system": platform.system(), "architecture": platform.machine()},
    "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
    "profiles": [],
}
for name, baseline_suffix, port_suffix in profiles:
    baseline = ROOT / "build" / f"baseline{baseline_suffix}-results.xml"
    port = ROOT / "build" / f"port{port_suffix}-results.xml"
    left, right = results(baseline), results(port)
    if left != right or any(test["skipped"] for test in right.values()):
        raise SystemExit(f"Test inventories do not match or a registration is skipped: {name}")
    report["profiles"].append({
        "name": name,
        "ctest_registrations_each": len(right),
        "gtest_cases_each": sum(sum(test["gtest_cases"]) for test in right.values()),
        "matching_case_and_skip_inventory": True,
        "baseline_junit_sha256": hashlib.sha256(baseline.read_bytes()).hexdigest(),
        "rust_junit_sha256": hashlib.sha256(port.read_bytes()).hexdigest(),
    })
for label, path in [("abi_default", ROOT / "build/abi/summary.json"),
                    ("abi_all_features", ROOT / "build/abi-all-features-summary.json")]:
    value = json.loads(path.read_text())
    report[label] = {"named_record_instances": value["records"], "checks": value["checks"],
                     "excluded_anonymous_or_bitfield_instances": len(value["skipped"])}
scenarios = json.loads((ROOT / "build/differential/summary.json").read_text())
report["differential"] = {
    "byte_identical_avif_scenarios": len(scenarios),
    "byte_identical_png_scenarios": sum(item["png_byte_equal"] for item in scenarios),
    "confirmed_noncontracting_c_matches": sum(item["matches_noncontracting_c"] is True for item in scenarios),
    "strict_png_byte_parity": all(item["png_byte_equal"] for item in scenarios),
}
output = ROOT / "build/validation-results.json"
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(report, indent=2) + "\n")
print(f"Recorded {len(profiles)} passing C/Rust profile comparisons and explicit ABI/differential limits")
