import re, sys
from c2r import clean_source
for f in sys.argv[1:]:
    c = clean_source(open(f).read())[0]
    for m in re.finditer(r'static\s+(?:inline\s+|AOM_FORCE_INLINE\s+)?([\w\s\*]+?)\b(\w+)\s*\(([^;{]*?)\)\s*\{', c, re.S):
        print(f'{" ".join(m.group(1).split())} {m.group(2)}({" ".join(m.group(3).split())});')
