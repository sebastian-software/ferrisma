#!/usr/bin/env python3
"""Generate genuine complete files using only the pinned original encoder."""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess

ROOT = Path(__file__).resolve().parent.parent

def word(v):
    return struct.unpack('<I', struct.pack('<f', v))[0]

def ec(kind=0, bits=8, exponent=0, shift=0, associated=0, index=0):
    color = (0.25, 0.5, 1.0, 0.5) if index % 2 == 0 else (1.0, 0.25, 0.5, 0.75)
    return [kind, int(exponent != 0), bits, exponent, shift, associated,
            *map(word, color), 19 + index, ('channel-' + str(index) + '-type-' + str(kind)).encode().hex()]

def frame(w=9, h=7, modular=1, last=1, save=0, source=1, before=0,
          blend=0, mode=2, clamp=1, x=0, y=0, seed=1, up=1, ecup=1, group=0, features=0):
    return [modular, last, save, source, before, blend, mode, clamp, x, y,
            w, h, seed, up, ecup, group, features]

def requests():
    cases = []
    def add(name, channels, w=9, h=7, gray=0, xyb=0, bits=8, exponent=0, frames=None):
        fields = [w, h, gray, xyb, int(exponent != 0), bits, exponent, len(channels)]
        for channel in channels:
            fields.extend(channel)
        frames = frames or [frame(w, h)]
        fields.append(len(frames))
        for row in frames:
            fields.extend(row)
        cases.append((name, 'ec-encode ' + ' '.join(map(str, fields))))
    for kind in [0, 1, 2, 3, 4, 5, 6, 16]:
        add('type-' + str(kind), [ec(kind, index=kind)])
    for associated in [0, 1]:
        for up in [1, 2, 4, 8]:
            for shift in range(up.bit_length()):
                add(f'alpha-{associated}-up-{up}-shift-{shift}', [ec(shift=shift, associated=associated)],
                    frames=[frame(ecup=up)])
    for modular in [0,1]:
        for associated in [0,1]:
            for up in [1,2,4,8]:
                for shift in range(up.bit_length()):
                    add(f'alpha-xyb-{modular}-{associated}-up-{up}-shift-{shift}', [ec(shift=shift,associated=associated)],xyb=1,
                        frames=[frame(modular=modular,ecup=up)])
    for bits, exponent in [(1, 0), (10, 0), (12, 0), (16, 0), (24, 0), (31, 0), (16, 5), (24, 7), (32, 8)]:
        add(f'alpha-depth-{bits}-{exponent}', [ec(bits=bits, exponent=exponent, associated=1)])
    add('gray-independent-float', [ec(1,16,5,index=0), ec(0,12,associated=1,index=1), ec(2,24,7,index=2)], gray=1, bits=16, exponent=5)
    all_types = [ec(k, bits=12 if i%2 else 8, associated=int(k==0), index=i) for i,k in enumerate([1,0,2,3,4,5,6,16,0,2])]
    add('all-types', all_types)
    add('all-types-dyadic', [ec(k,32,8,associated=int(k==0),index=i) for i,k in enumerate([1,0,2,3,4,5,6,16,0,2])],bits=32,exponent=8)
    add('all-types-tiny', all_types, w=1, h=1)
    add('all-types-tiny-xyb', all_types, w=1, h=1, xyb=1, frames=[frame(w=1,h=1,modular=0)])
    add('all-types-horizontal-groups', all_types, w=129, h=3)
    add('all-types-vertical-groups', all_types, w=3, h=129)
    add('alpha-first-unassociated', [ec(1,index=0),ec(0,12,index=1),ec(0,16,associated=1,index=2)])
    extreme=ec(2,index=0);extreme[6:10]=list(map(word,[-65504.0,0.0,65504.0,-0.5]));extreme[-1]='00ff807f'
    add('spot-signed-extreme-binary-name',[extreme,ec(5,index=1)])
    add('multiple-alpha-spots-float', [ec(1,24,7),ec(2,32,8,index=1),ec(0,16,5,associated=1,index=2),ec(0,32,8,index=3),ec(2,16,5,index=4)], bits=32, exponent=8)
    for modular in [0,1]:
        for up,ecup in [(2,2),(2,4),(4,4),(4,8),(8,8)]:
            add(f'xyb-{modular}-up-{up}-{ecup}',[ec(1,16,index=0),ec(0,12,associated=1,index=1),ec(2,index=2)], xyb=1,
                frames=[frame(modular=modular,up=up,ecup=ecup)])
    add('xyb-filters',all_types,xyb=1,frames=[frame(modular=0,features=1)])
    add('patches',all_types,w=65,h=49,frames=[frame(w=65,h=49,features=2)])
    for before in [0,1]:
        for mode in range(5):
            add(f'reference-{before}-{mode}',all_types,
                frames=[frame(last=0,save=1,before=before),frame(w=7,h=5,blend=1,mode=mode,x=1,y=1,seed=7)])
    return cases

def observe(binary, cases):
    process = subprocess.run([str(binary)], input='\n'.join(cases)+'\n', text=True, capture_output=True, check=True)
    if process.stderr or len(process.stdout.splitlines()) != len(cases):
        raise RuntimeError(process.stderr or 'Missing actual original observation')
    return process.stdout.splitlines()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--oracle', type=Path, default=ROOT/'target/oracle/jxl_oracle')
    args = parser.parse_args()
    source = ROOT/'upstream/libjxl'
    revision = subprocess.check_output(['git','-C',str(source),'rev-parse','HEAD'],text=True).strip()
    if revision != 'a16d32d0ac113f5e54125e49bb571281c5d0b704' or subprocess.check_output(['git','-C',str(source),'status','--porcelain','--ignore-submodules=none'],text=True):
        raise RuntimeError('Original source/dependency pins changed')
    rows=requests()
    manifest={'source_revision':revision,'cases':[]}
    for name,request in rows:
        response,=observe(args.oracle,[request]);fields=response.split();data=bytes.fromhex(fields[0])
        target=ROOT/'tests/data'/f'extra-channel-{name}.jxl'
        if target.exists() and target.read_bytes()!=data:
            raise RuntimeError('Existing genuine fixture changed: '+name)
        target.write_bytes(data)
        decoded,=observe(args.oracle,['fs-decode '+data.hex()]); words=decoded.split(); end=max(i for i,v in enumerate(words) if v=='E'); decode_status=int(words[end+1])
        if name=='all-types-tiny-xyb':
            rows.append(('mixed-sampling-xyb','ec-mixed-ups '+data.hex()+' 10 1 2 4 8 1 2 4 8 1 2'))
        if name=='all-types-tiny':
            rows.append(('mixed-sampling-modular-source-error','ec-mixed-ups '+data.hex()+' 10 1 2 4 8 1 2 4 8 1 2'))
        manifest['cases'].append({'decode_status':decode_status,'name':name,'request':request,'jxl':target.name,'sha256':hashlib.sha256(data).hexdigest(),'writer_observation':response})
    base=(ROOT/'tests/data/extra-channel-type-16.jxl').read_bytes().hex()
    rejected=[]
    for kind in [7,8,9,10,11,12,13,14,15,17,31,63]:
        request='ec-mutate-type '+base+' '+str(kind)
        response,=observe(args.oracle,[request]);data=bytes.fromhex(response.split()[0]);target=ROOT/'tests/data'/f'extra-channel-rejected-{kind}.jxl'
        if target.exists() and target.read_bytes()!=data:raise RuntimeError('Existing malformed source fixture changed')
        target.write_bytes(data)
        rejected.append({'type':kind,'request':request,'jxl':target.name,'sha256':hashlib.sha256(data).hexdigest(),'mutation_observation':response})
    manifest['rejected']=rejected
    (ROOT/'tests/data/extra-channels-rejections.requests').write_text(''.join(str(item['type'])+'\t'+item['request']+'\n' for item in rejected))
    captured,=observe(args.oracle,['ec-export '+(ROOT/'tests/data/extra-channel-all-types-dyadic.jxl').read_bytes().hex()+' 32 1 3 0 0'])
    values=captured.split();assert values[0]=='0' and values[4]=='X';pixels=bytes.fromhex(values[3])[:-7]
    for i in range(int(values[5])):
        assert values[6+2*i]=='0';pixels+=bytes.fromhex(values[7+2*i])[:-7]
    capture=ROOT/'tests/data/extra-channel-all-types-dyadic.float-pixels'
    if capture.exists() and capture.read_bytes()!=pixels:raise RuntimeError('Independent native source capture changed')
    capture.write_bytes(pixels);manifest['native_dyadic_capture']={'file':capture.name,'sha256':hashlib.sha256(pixels).hexdigest(),'source_export':captured}
    (ROOT/'tests/data/extra-channels.decode-statuses').write_text(''.join(str(c['decode_status'])+'\n' for c in manifest['cases']))
    (ROOT/'tests/data/extra-channels.requests').write_text(''.join(name+'\t'+q+'\n' for name,q in rows))
    (ROOT/'tests/data/extra-channels-fixtures.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(len(rows),'genuine complete original extra-channel fixtures generated')

if __name__=='__main__':
    main()
