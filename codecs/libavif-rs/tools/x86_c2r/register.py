#!/usr/bin/env python3
"""Add x86 kernel files to crates/aom/src/simd/mod.rs in sorted order.

Usage: python3 tools/x86_c2r/register.py aom_dsp/x86/sse_avx2.rs ...
"""
import re
import sys
from pathlib import Path

MOD = Path(__file__).resolve().parents[2] / 'crates/aom/src/simd/mod.rs'
PAT = re.compile(r'#\[cfg\(target_arch = "x86_64"\)\]\n#\[path = "([^"]+)"\]\npub mod (\w+);\n\n?')

s = MOD.read_text()
ms = list(PAT.finditer(s))
first, last = ms[0].start(), ms[-1].end()
items = {m.group(1): m.group(2) for m in ms}
for path in sys.argv[1:]:
    items.setdefault(path, path[:-3].replace('/', '__'))
block = '\n'.join(f'#[cfg(target_arch = "x86_64")]\n#[path = "{p}"]\npub mod {m};\n' for p, m in sorted(items.items()))
MOD.write_text(s[:first] + block + ('\n' if s[last:].strip() else '') + s[last:])
