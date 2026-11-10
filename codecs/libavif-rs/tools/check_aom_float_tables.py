#!/usr/bin/env python3
"""Compare every float bit and null slot before/after the C2Rust input workaround."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

from aom_compound_literals import name_float_tables

ROOT = Path(__file__).resolve().parents[1]


def pointer_table(text):
    start = text.index('static const float *prune_2D_adaptive_thresholds[]')
    end = text.index('\n};', start) + 3
    return text[start:end]


def main():
    source = ROOT / 'upstream/aom/av1/encoder/tx_search.c'
    original = source.read_text()
    normalized, count = name_float_tables(original)
    named_arrays = normalized.split('#line 1 ', 1)[0]
    work = ROOT / 'build/aom-float-tables'
    work.mkdir(parents=True, exist_ok=True)
    # Read all elements and every null pointer slot, retaining their original
    # ordering. Lengths come from the pinned TX_SIZE rows, not rounded values.
    driver = r'''
int main(void) {
  const unsigned lengths[] = {14,14,10,0,0,14,14,14,14,0,0,0,0,14,14,0,0,0,0};
  _Static_assert(sizeof(prune_2D_adaptive_thresholds) / sizeof(void *) == 19, "TX_SIZE rows");
  for (unsigned row = 0; row < 19; ++row) {
    const float *p = prune_2D_adaptive_thresholds[row];
    if ((!p) != (lengths[row] == 0)) return 1;
    if (fwrite(&lengths[row], sizeof(unsigned), 1, stdout) != 1) return 2;
    if (p && fwrite(p, sizeof(float), lengths[row], stdout) != lengths[row]) return 3;
  }
  return 0;
}
'''
    outputs = []
    for name, declarations in [('original', pointer_table(original)),
                               ('normalized', named_arrays + pointer_table(normalized))]:
        path = work / f'{name}.c'
        path.write_text('#include <stdio.h>\n' + declarations + '\n' + driver)
        subprocess.run([os.environ.get('CC', 'cc'), '-std=c11', '-ffp-contract=off',
                        path, '-o', work / name], check=True)
        outputs.append(subprocess.check_output([work / name]))
    if outputs[0] != outputs[1] or len(outputs[0]) != 564:
        raise SystemExit('Compound float normalization changed original table bytes')
    record = dict(named_static_arrays=count, pointer_rows=19, float_elements=122,
                  original_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                  bytes_compared=len(outputs[0]), exact_match=True,
                  output_sha256=hashlib.sha256(outputs[0]).hexdigest())
    (work / 'summary.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record, indent=2))


if __name__ == '__main__':
    main()
