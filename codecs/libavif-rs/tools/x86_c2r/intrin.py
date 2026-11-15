"""Signatures of x86 intrinsics from clang's headers: name -> (ret, [param types], macro?)."""
import re, json, glob
import os
INC = os.environ.get('CLANG_INCLUDE') or sorted(glob.glob('/Library/Developer/CommandLineTools/usr/lib/clang/*/include'))[0]
FILES = ['mmintrin.h', 'xmmintrin.h', 'emmintrin.h', 'pmmintrin.h', 'tmmintrin.h', 'smmintrin.h',
         'avxintrin.h', 'avx2intrin.h']
def norm(t):
    t = re.sub(r'\b(const|__restrict|volatile)\b', ' ', t)
    t = re.sub(r'\b__m128i_u\b', '__m128i', t); t = re.sub(r'\b__m256i_u\b', '__m256i', t)
    t = re.sub(r'\b__m128_u\b', '__m128', t); t = re.sub(r'\b__m128d_u\b', '__m128d', t)
    t = re.sub(r'\s+', ' ', t).strip()
    t = t.replace(' *', '*')
    return t
def load():
    table = {}
    for f in FILES:
        text = open(f'{INC}/{f}').read()
        text = re.sub(r'/\*.*?\*/', ' ', text, flags=re.S)
        text = re.sub(r'//[^\n]*', '', text)
        for m in re.finditer(r'#define\s+(_mm\w*)\(([^)]*)\)', text):
            table[m.group(1)] = dict(macro=True, nparams=len([p for p in m.group(2).split(',') if p.strip()]))
        flat = re.sub(r'\\\n', ' ', text)
        for m in re.finditer(r'static\s+__inline(?:__)?\s+(.*?)\b(_mm\w*)\s*\(([^)]*)\)\s*\{', flat, re.S):
            ret = re.sub(r'__DEFAULT_FN_ATTRS\w*|__attribute__\s*\(\(.*?\)\)|__inline__|__inline', ' ', m.group(1))
            params = []
            for p in m.group(3).split(','):
                p = p.strip()
                if not p or p == 'void': continue
                pm = re.match(r'(.*?)(\w+)$', p)
                params.append(norm(pm.group(1)))
            table.setdefault(m.group(2), dict(macro=False, ret=norm(ret), params=params))
    return table
if __name__ == '__main__':
    t = load(); print(len(t)); 
    for n in ['_mm_set1_epi16','_mm_srli_si128','_mm_slli_epi16','_mm_storel_epi64','_mm_cvtsi128_si32','_mm256_permute4x64_epi64','_mm_loadl_epi64','_mm256_extracti128_si256','_mm_packus_epi16','_mm256_set_m128i']:
        print(n, t.get(n))
