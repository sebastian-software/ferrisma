#!/usr/bin/env python3
"""Record compiler-default differences separately from exact acceptance gates."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
subprocess.run([sys.executable, ROOT / 'tools/build_sharpyuv.py', '--compiler-default-diagnostic'], check=True)
suffix = '.dylib' if sys.platform == 'darwin' else '.so'
reference = base / 'sharpyuv-compiler-default' / ('libsharpyuv' + suffix)
report_path = base / 'sharpyuv-parity-compiler-default/summary.json'
if report_path.exists():
    report_path.unlink()
result = subprocess.run([sys.executable, ROOT / 'tools/check_sharpyuv.py', '--reference', reference,
                         '--label', 'compiler-default'], cwd=ROOT, text=True,
                        stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
if not report_path.exists():
    raise SystemExit('Diagnostic comparison could not execute: ' + result.stdout)
report = json.loads(report_path.read_text())
if report['cases'] != 1271 or report['reference_sha256'] != hashlib.sha256(reference.read_bytes()).hexdigest():
    raise SystemExit('Incomplete diagnostic result')
expected_status = 1 if report['differences'] else 0
if result.returncode != expected_status:
    raise SystemExit('Unexpected diagnostic failure: ' + result.stdout)
report['acceptance_reference'] = False
report['purpose'] = 'Compiler-default diagnostic; primary C reference uses -ffp-contract=off'
report_path.write_text(json.dumps(report, indent=2) + '\n')
print(f"Compiler-default diagnostic: {len(report['differences'])}/{report['cases']} differing groups; not an acceptance gate")
