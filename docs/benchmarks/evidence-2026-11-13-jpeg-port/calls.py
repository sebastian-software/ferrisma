"""List direct calls (bl/call) per function in an objdump -d listing whose name matches a pattern."""
import re
import sys
from collections import Counter

txt = open(sys.argv[1]).read()
pat = re.compile(sys.argv[2])
for f in re.split(r'\n(?=[0-9a-f]+ <)', txt):
    head = f.split('\n', 1)[0]
    m = re.match(r'[0-9a-f]+ <(.*)>:', head)
    if not m or not pat.search(m[1]):
        continue
    calls = Counter(re.sub(r'^_+R\w*?_4jpeg', '', c)
                    for c in re.findall(r'\t(?:bl|callq?)\s+0x[0-9a-f]+ <([^>+]+)', f))
    name = re.sub(r'^_+R\w*?_4jpeg', '', m[1])
    print(f"{name[-60:]:60s} insns={f.count(chr(10)):5d} calls={dict(calls)}")
