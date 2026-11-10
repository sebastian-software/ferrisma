#!/usr/bin/env python3
"""Pinned ring/output helpers at genuine allocation and partial-state boundaries.

Fixture allocators initialize only their own observed backing, and quarantine
logical frees to retain unique original owner identities. Production code does
not seed/zero complete ring capacity. All source bodies come from one pristine
include with unchanged default configuration; public APIs bind that same TU.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify
from brotli_state_reference import PIN, ROOT, sha


def corpus():
    requests = []
    def add(op, seed=165, fail=0, window=10, canny=1, old=1024, new=2048,
            pos=17, remaining=100, rounds=0, partial=0, capacity=32, force=0,
            target=0, state=0, error=0, wrap=0, consumed=0):
        requests.append(' '.join(map(str, [op,seed,fail,window,canny,old,new,pos,remaining,
                                         rounds,partial,capacity,force,target,state,error,wrap,consumed])))
    for window in [10,11,16,24,30]:
        for old in [0,1024,2048,1<<window]:
            # Large rings are valid sizing inputs, but need not be allocated
            # merely to observe a pure sizing branch. The observer's fixture
            # avoids gigantic payloads; include such selected windows with
            # actual existing small owners instead of fabricated huge blocks.
            if old > 2048: continue
            for remaining in [0,1,1023,1024,1025,2048,65536]:
                for canny in [0,1]:
                    for metadata in [0,1]:
                        add(0,window=window,old=old,remaining=remaining,canny=canny,force=metadata)
    add(1,old=0,new=0,pos=0)
    for seed in [0,90,165,255]:
        for old in [0,2,8,64,1024]:
            for new in [2,8,64,1024,2048]:
                # Ensure copies a live ring; select source-valid old copy
                # extents in both actual allocations, including unknown tails.
                for pos in [0,min(old,new,17)]:
                    for fail in [0,3,4]:
                        add(1,seed=seed,old=old,new=new,pos=pos,fail=fail)
    for old in [2,8,64,1024]:
        for pos in [0,1,old-1,old,old+min(old,542)]:
            for rounds in [0,1,7]:
                for consumed in [0,min(pos,old)//2,min(pos,old)]:
                    for wrap in [0,1]:
                        add(2,old=old,pos=pos,rounds=rounds,partial=rounds*old+consumed,wrap=wrap)
    for old in [8,64,1024]:
        for pos in [0,1,old-1,old,old+min(old,542)]:
            for partial in [0,min(pos,old)//2,min(pos,old)]:
                for capacity in [0,1,7,32,2048]:
                    for force in [0,1]:
                        for target in [0,1,2]:
                            for remaining in [-1,0,123]:
                                add(3,old=old,pos=pos,partial=partial,capacity=capacity,force=force,target=target,remaining=remaining)
    for old in [8,64,1024]:
        for pos in [0,1,min(old,542)]:
            for wrap in [0,1]:
                for seed in [0,165,255]:
                    add(4,old=old,pos=pos,wrap=wrap,seed=seed)
    for old in [0,8,64,1024]:
        for pos in [0,1,min(old,542)]:
            for partial in [0,min(pos,old)]:
                for capacity in [0,1,32,2048]:
                    for remaining in [-1,0,100]:
                        for error in [-31,0,1,2,3]:
                            for wrap in [0,1]:
                                if not old and wrap: continue
                                add(5,old=old,pos=pos,partial=partial,capacity=capacity,remaining=remaining,error=error,wrap=wrap)
    for old in [0,64]:
        for state in [0,1,26,0xffffffff]:
            for error in [-31,-1,0,1,2,3]:
                for pos in [0,1,64]:
                    add(6,old=old,pos=pos,state=state,error=error)
    for error in [-31,-9,-1,0,1,2,3,42]:
        for wrap in [0,1]:
            for target in [0,1]:
                for consumed in [0,1,2,3,(1<<64)-1]:
                    add(7,old=0,error=error,wrap=wrap,target=target,consumed=consumed)
    return requests


def binding(binary, sanitized):
    expected = {name:'brotli_ringbuffer.c.o' for name in [
        'BrotliDecoderCreateInstance','BrotliDecoderDestroyInstance',
        'BrotliDecoderHasMoreOutput','BrotliDecoderTakeOutput','BrotliDecoderIsFinished']}
    expected.update({'BrotliDecoderStateInit':'libbrotlidec.a(state.c.o)',
                     'BrotliSharedDictionaryCreateInstance':'libbrotlicommon.a(shared_dictionary.c.o)'})
    proof = verify(binary,sanitized,'brotli_ringbuffer.map',['brotli_ringbuffer.c'],expected)
    wrapper = ROOT/'tools/oracle/brotli_ringbuffer.c'
    assert wrapper.read_text().count('#include "c/dec/decode.c"') == 1
    proof['pristine_include_adapter_sha256'] = sha(wrapper)
    return proof


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--ordinary',type=Path,required=True)
    p.add_argument('--sanitized',type=Path,required=True)
    p.add_argument('--rust',action='append',required=True)
    p.add_argument('--output',type=Path,required=True)
    args=p.parse_args();args.output.mkdir(parents=True,exist_ok=False)
    requests=corpus();payload=('\n'.join(requests)+'\n').encode()
    (args.output/'requests.txt').write_bytes(payload)
    binaries=[('ordinary',args.ordinary.resolve()),('sanitized',args.sanitized.resolve())]
    binaries += [(name,Path(path).resolve()) for name,path in (r.split('=',1) for r in args.rust)]
    report={'source_pin':PIN,'requests':len(requests),'configurations':{},'full_Rust_streaming_decoder':False}
    expected=None
    for name,binary in binaries:
        proof=binding(binary,name=='sanitized') if name in ['ordinary','sanitized'] else None
        output=args.output/f'{name}.txt'
        with output.open('wb') as stream:
            r=subprocess.run([binary],input=payload,stdout=stream,stderr=subprocess.PIPE)
        (args.output/f'{name}.stderr').write_bytes(r.stderr)
        assert r.returncode==0 and not r.stderr,(name,r.returncode,r.stderr[:2048])
        records=output.read_bytes().splitlines()
        assert len(records)==len(requests),(name,len(records))
        if expected is None:expected=records
        for index,(actual,wanted) in enumerate(zip(records,expected)):
            assert actual==wanted,(name,index,requests[index],actual[:300],wanted[:300])
        report['configurations'][name]={'source_binding':proof,'binary_sha256':sha(binary),'output_sha256':sha(output),'output_bytes':output.stat().st_size}
    report['all_exact']=True
    (args.output/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'requests':len(requests),'all_exact':True}))

if __name__=='__main__':main()
