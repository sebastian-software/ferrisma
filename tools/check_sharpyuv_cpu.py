#!/usr/bin/env python3
"""Compare the original target-specific CPU callback before initialization."""
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
work = base / 'sharpyuv-cpu'
work.mkdir(parents=True, exist_ok=True)
source = ROOT / 'upstream/libwebp'
reference = base / 'sharpyuv-reference'
target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')) / 'release'
compiler = shlex.split(os.environ.get('CC', 'cc'))
common = ['-O2', '-DHAVE_CONFIG_H', '-I' + str(source), '-I' + str(reference)]
native = ['-lm', '-lpthread'] + (['-lresolv', '-liconv'] if sys.platform == 'darwin' else ['-ldl'])
results = []
for name, libraries in [('c', [str(source / 'sharpyuv/sharpyuv_cpu.c')]),
                        ('rust', [str(target / 'libsharpyuv.a')])]:
    executable = work / name
    subprocess.run(compiler + common + [str(ROOT / 'tests/sharpyuv/cpu.c'), *libraries, *native,
                                      '-o', str(executable)], check=True)
    results.append(json.loads(subprocess.check_output([executable], text=True)))
if results[0] != results[1]:
    raise SystemExit(f'Initial CPU callback or feature queries differ: {results}')
(work / 'results.json').write_text(json.dumps(results[0], indent=2) + '\n')
print('Identical initial CPU callback and all ten original CPU-feature queries')
