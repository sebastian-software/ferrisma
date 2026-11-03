#!/usr/bin/env python3
"""Retain all original bit-reader rows and observe the compiled native64 guard.

Source SafeReadBits documents <=24 but its DCHECK is absent in the selected
NDEBUG graph. The unchanged while/take body has a wider defined native64
bitmask/shift domain; observe it directly rather than infer equivalence from
SafeReadBits32 or discard synthetic source-defined continuation requests.
"""
import argparse
import json
from pathlib import Path
import subprocess

from brotli_bit_reader_reference import corpus as original_corpus, run
from brotli_state_reference import PIN, ROOT, compile_options, sha


def corpus():
    requests=original_corpus()
    assert len(requests)==18485
    for size in [0,1,2,3,4,5,7,8,16,32]:
        data=bytes((i*97+size*13)&255 for i in range(size)).hex() or '-'
        for bits in [0,1,7,8,16,24,25,31,32,33,48,56,63,64]:
            for full in [0,0xd3a5c197eb024f68,(1<<64)-1]:
                val=full&((1<<bits)-1)
                for width in [0,25,26,27,28,29,30,31,32]:
                    requests.append(f'3 {val} {bits} {width} {data}')
    return requests


def binding(binary,sanitized):
    commands_path=binary.parent/'compile_commands.json'
    commands=json.loads(commands_path.read_text())
    original=next(c for c in commands if c['file'].endswith('/c/dec/bit_reader.c'))
    source=Path(original['file']).resolve().parents[2]
    assert subprocess.check_output(['git','-C',source,'rev-parse','HEAD'],text=True).strip()==PIN
    assert not subprocess.check_output(['git','-C',source,'status','--porcelain','--untracked-files=no'],text=True).strip()
    units=[c for c in commands if Path(c['file']).resolve().is_relative_to(source)]
    observer=[c for c in commands if Path(c['file']).resolve()==ROOT/'tools/oracle/brotli_bit_reader.c']
    assert len(units)==35 and len(observer)==1
    for c in [*units,*observer]:compile_options(c,sanitized)
    return {'binary_sha256':sha(binary),'compile_commands_sha256':sha(commands_path),
            'configured_pristine_c_units':len(units),'actual_observer_options_checked':True,
            'source_header_sha256':sha(source/'c/dec/bit_reader.h'),
            'observer_sha256':sha(ROOT/'tools/oracle/brotli_bit_reader.c')}


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--ordinary',type=Path,required=True);p.add_argument('--sanitized',type=Path,required=True)
    p.add_argument('--rust',action='append',required=True);p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();args.output.mkdir(parents=True,exist_ok=False)
    requests=corpus();(args.output/'requests.txt').write_text('\n'.join(requests)+'\n')
    binaries=[('ordinary',args.ordinary.resolve()),('sanitized',args.sanitized.resolve())]
    binaries += [(n,Path(b).resolve()) for n,b in (s.split('=',1) for s in args.rust)]
    expected=None;report={'source_pin':PIN,'old_requests_retained':18485,'new_width_rows':len(requests)-18485,'requests':len(requests),'configurations':{}}
    for name,binary in binaries:
        source=binding(binary,name=='sanitized') if name in ['ordinary','sanitized'] else None
        rows=run(binary,requests)
        output=args.output/f'{name}.txt';output.write_text('\n'.join(rows)+'\n')
        if expected is None:expected=rows
        for i,(x,y) in enumerate(zip(rows,expected)):assert x==y,(name,i,requests[i],x,y)
        report['configurations'][name]={'source_binding':source,'binary_sha256':sha(binary),'output_sha256':sha(output)}
    report['all_exact']=True
    (args.output/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'requests':len(requests),'all_exact':True}))

if __name__=='__main__':main()
