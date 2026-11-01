#!/usr/bin/env python3
"""Execute every unmodified Meson test command after explicit decoder replacement."""
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import json
import math
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def run_one(index, test, output, multiplier):
    if test['protocol'] != 'exitcode' or not test['is_parallel']:
        raise SystemExit(f'Unsupported original test protocol: {test}')
    environment = os.environ.copy()
    environment.update(test['env'])
    start = time.monotonic()
    try:
        result = subprocess.run(test['cmd'], cwd=test['workdir'] or ROOT, env=environment,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=test['timeout'] * multiplier)
        status, text = ('OK' if result.returncode == 0 else 'FAIL'), result.stdout
        code = result.returncode
    except subprocess.TimeoutExpired as failure:
        status, text, code = 'TIMEOUT', failure.stdout or b'', None
    (output / f'{index:04d}.log').write_bytes(text)
    return {'index': index, 'name': test['name'], 'suite': test['suite'],
            'result': status, 'returncode': code, 'seconds': time.monotonic() - start,
            'command': test['cmd'], 'original_timeout': test['timeout'],
            'timeout': test['timeout'] * multiplier, 'timeout_multiplier': multiplier}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build', type=Path)
    parser.add_argument('--jobs', type=int, default=4)
    parser.add_argument('--timeout-multiplier', type=float, default=1)
    args = parser.parse_args()
    if not math.isfinite(args.timeout_multiplier) or args.timeout_multiplier <= 0 or args.jobs <= 0:
        parser.error('Jobs and timeout multiplier must be positive and finite')
    build = args.build.resolve()
    tests = json.loads((build / 'original-tests.json').read_text())
    output = build / 'original-test-logs'
    output.mkdir(exist_ok=True)
    records = []
    with ThreadPoolExecutor(max_workers=args.jobs) as executor:
        futures = [executor.submit(run_one, index, test, output, args.timeout_multiplier) for index, test in enumerate(tests)]
        for future in as_completed(futures):
            record = future.result()
            records.append(record)
            if record['result'] != 'OK' or len(records) % 50 == 0:
                print(f"{len(records)}/{len(tests)}: {record['result']} {record['name']}", flush=True)
    records.sort(key=lambda record: record['index'])
    (build / 'original-results.json').write_text(json.dumps(records, indent=2) + '\n')
    failures = [record for record in records if record['result'] != 'OK']
    print(f'{len(records) - len(failures)}/{len(tests)} original tests passed; {len(failures)} failures')
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
