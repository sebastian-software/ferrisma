#!/usr/bin/env python3
"""Positively census original Float16 odd-stride stores without defining UB."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parent.parent

def requests():
    manifest=json.loads((ROOT/'tests/data/extra-channels-fixtures.json').read_text())
    return [(item['name'],channels,unpremul,'ec-export '+(ROOT/'tests/data'/item['jxl']).read_bytes().hex()+f' 16 1 {channels} {unpremul} 3')
            for item in manifest['cases'] for channels in range(1,5) for unpremul in range(2)]

def exact_misalignment(stderr):
    return ('span.h:59:' in stderr and 'runtime error: reference binding to misaligned address ' in stderr
            and "for type 'hwy::float16_t', which requires 2 byte alignment" in stderr
            and 'SUMMARY: UndefinedBehaviorSanitizer: undefined-behavior ' in stderr
            and stderr.rstrip().endswith('span.h:59:12'))

def census(output,write_classification=False,captured_requests=None):
    output.mkdir(parents=True,exist_ok=False);rows=requests();report={'requests':len(rows),'cases':[],'graphs':{}}
    if captured_requests is not None and captured_requests.read_text().splitlines()!=[row[3] for row in rows]:
        raise RuntimeError('Actual Rust original-only half-stride inventory changed')
    for name,n,unpremul,request in rows:
        report['cases'].append({'name':name,'channels':n,'unpremul':unpremul,'request':request,'observations':{}})
    for graph in ['oracle','oracle-asan']:
        binary=ROOT/'target'/graph/'jxl_oracle';before=hashlib.sha256(binary.read_bytes()).hexdigest();diagnosed=[]
        for index,row in enumerate(report['cases']):
            p=subprocess.run([str(binary)],input=row['request']+'\n',text=True,capture_output=True)
            observation={'status':p.returncode,'stdout':p.stdout,'stderr':p.stderr};row['observations'][graph]=observation
            misaligned=exact_misalignment(p.stderr)
            if misaligned:
                if graph!='oracle-asan' or p.returncode==0:raise RuntimeError('Nonrecovering source sanitizer contract changed')
                row['classification']='source-undefined';diagnosed.append(index)
            elif p.returncode!=0 or p.stderr or len(p.stdout.splitlines())!=1:
                raise RuntimeError(f'Unexpected original half-stride probe failure at {index}: {observation}')
            elif graph=='oracle-asan':
                row['classification']='defined'
                if observation!=row['observations']['oracle']:raise RuntimeError('Defined ordinary/ASan half-stride words differ')
        if hashlib.sha256(binary.read_bytes()).hexdigest()!=before:raise RuntimeError('Active source binary changed')
        report['graphs'][graph]={'binary_sha256':before,'diagnosed':diagnosed}
    counts=Counter(row['classification'] for row in report['cases']);report['counts']=dict(counts)
    if counts!={'source-undefined':528,'defined':360}:raise RuntimeError('Actual source half-stride boundary set changed: '+str(counts))
    classification=''.join(f"{row['name']}\t{row['channels']}\t{row['unpremul']}\t{row['classification']}\n" for row in report['cases'])
    path=ROOT/'tests/data/extra-channels-half-strides.tsv'
    if write_classification:
        if path.exists() and path.read_text()!=classification:raise RuntimeError('Existing actual source classification changed')
        path.write_text(classification)
    elif path.read_text()!=classification:raise RuntimeError('Actual source classification no longer matches frozen positive evidence')
    report['classification_sha256']=hashlib.sha256(classification.encode()).hexdigest();report['audit_passed']=True
    (output/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print('888 original odd-stride Float16 requests retained: 528 exact nonrecovering span.h:59 diagnostics, 360 defined observations agree',flush=True)
    return report

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--output',type=Path,required=True);parser.add_argument('--write-classification',action='store_true');args=parser.parse_args()
    census(args.output.resolve(),args.write_classification)

if __name__=='__main__':main()
