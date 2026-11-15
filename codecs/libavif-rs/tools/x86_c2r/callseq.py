"""Compare, function by function, the ordered sequence of called names and
numeric constants between a C file and its Rust translation."""
import re, sys
from c2r import clean_source
IGN_RS = {'offset', 'as_mut_ptr', 'as_ptr', 'unwrap', 'Some', 'read_unaligned', 'write_unaligned', 'debug_assert',
          'size_of', 'zeroed', 'is_null', 'is_none', 'step_by', '_mm_cvtsi32_si128', 'if', 'while', 'match', 'for', 'in', 'return', 'add', 'sub', 'cast', 'copy_nonoverlapping', 'write_bytes'}
IGN_C = {'if', 'while', 'for', 'switch', 'return', 'sizeof', 'assert', 'defined', 'memcpy', 'memset', 'DECLARE_ALIGNED'}
SHIFT = {'_mm_sll_epi16': '_mm_slli_epi16', '_mm_sra_epi16': '_mm_srai_epi16', '_mm_srl_epi16': '_mm_srli_epi16',
         '_mm_sll_epi32': '_mm_slli_epi32', '_mm_sra_epi32': '_mm_srai_epi32', '_mm_srl_epi32': '_mm_srli_epi32',
         '_mm256_sll_epi16': '_mm256_slli_epi16', '_mm256_sra_epi16': '_mm256_srai_epi16',
         '_mm256_sll_epi32': '_mm256_slli_epi32', '_mm256_sra_epi32': '_mm256_srai_epi32',
         '_mm256_srl_epi16': '_mm256_srli_epi16', '_mm256_srl_epi32': '_mm256_srli_epi32'}
def bodies(text, lang):
    out = {}
    pat = (r'\n(?:static\s+)?(?:inline\s+)?[\w\s\*]*?\b(\w+)\s*\(([^;{]*?)\)\s*\{' if lang == 'c'
           else r'\n(?:pub\s+)?unsafe\s+(?:extern\s+"C-unwind"\s+)?fn\s+(\w+)\s*\(')
    for m in re.finditer(pat, text, re.S):
        name = m.group(1)
        if name in ('if', 'for', 'while', 'switch'):
            continue
        i = text.index('{', m.end() - 1)
        depth = 0
        for j in range(i, len(text)):
            if text[j] == '{': depth += 1
            elif text[j] == '}':
                depth -= 1
                if depth == 0: break
        out[name] = text[i:j]
    return out
def calls(body, lang):
    seq = []
    for m in re.finditer(r'\b([A-Za-z_]\w*)\s*(!?)\s*(::<\{?\s*([^>}]*)\}?>)?\s*\(', body):
        n = m.group(1)
        if (lang == 'c' and n in IGN_C) or (lang == 'rs' and n in IGN_RS):
            continue
        n = SHIFT.get(n, n)
        seq.append(n)
    return seq
c = clean_source(open(sys.argv[1]).read())[0]
r = open(sys.argv[2]).read()
r = re.sub(r'//[^\n]*', '', r).replace('.unwrap()(', '(')
cb, rb = bodies(c, 'c'), bodies(r, 'rs')
bad = 0
for name, body in cb.items():
    if name not in rb:
        print('missing in Rust:', name); bad += 1; continue
    a, b = calls(body, 'c'), calls(rb[name], 'rs')
    if a != b:
        bad += 1
        import difflib
        print('DIFF', name, list(difflib.unified_diff(a, b, lineterm='', n=1))[:12])
print(f'{len(cb)} C functions, {bad} differ')
