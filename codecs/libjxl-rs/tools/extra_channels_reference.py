#!/usr/bin/env python3
"""Audit complete extra-channel source behavior, bindings and native captures.

Only genuine original encoder/header/output entry points produce fixture bytes
and expected observations. Neither active objects nor pristine sources change.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess

from extra_channel_fixtures import observe
from extra_half_boundaries import census
from spot_reference import binding as spot_binding

ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def bindings(build, directory):
    entries=json.loads((build/'compile_commands.json').read_text())
    selected,=[row for row in entries if Path(row['file']).resolve()==ROOT/'tools/oracle/extra_channels.cc']
    command=selected.get('arguments') or shlex.split(selected['command'])
    if '-O3' not in command or any(flag in command for flag in ('-O0','-O1','-fno-access-control')):
        raise RuntimeError('Complete-file adapter compilation changed')
    if ('-fsanitize=address,undefined' in command)!=(build.name=='oracle-asan'):
        raise RuntimeError('Complete-file adapter sanitizer coverage changed')
    link=shlex.split((build/'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    binary,mapping=directory/'bound',directory/'link.map'
    link[link.index('-o')+1]=str(binary)
    link.append(('-Wl,-map,' if platform.system()=='Darwin' else '-Wl,-Map,')+str(mapping))
    subprocess.run(link,cwd=build,check=True)
    expected={'EncodeFrame':'main.cc.o','WriteCodestreamHeaders':'enc_fields.cc.o',
              'WriteFrameHeader':'enc_fields.cc.o','ReadImageMetadata':'image_metadata.cc.o',
              'PreparePipeline':'dec_cache.cc.o','DecodeFrame':'dec_frame.cc.o','ImageMetadata::VisitFields':'image_metadata.cc.o',
              'ConvertToExternal':'sample_conversion.cc.o',
              'ConvertChannelsToExternal':'sample_conversion.cc.o',
              'UnpremultiplyAlpha':'alpha.cc.o','GetUpsamplingStage':'stage_upsampling.cc.o'}
    includes={'enc_frame.inc':'lib/jxl/enc_frame.cc','uint_output.inc':'lib/jxl/dec_external_image.cc'}
    for adapter,original in includes.items():
        if (ROOT/'tools/oracle'/adapter).read_text().count('#include "'+original+'"')!=1:
            raise RuntimeError('Pristine original include changed: '+adapter)
    contents=mapping.read_text();found={name:[] for name in expected}
    aliases=[]
    if platform.system()=='Darwin':
        objects=dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$',contents,re.M))
        for line in contents.splitlines():
            row=re.match(r'^0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+(\S+)$',line)
            if not row or 'FDE' in line:continue
            for name,source in expected.items():
                namespace='18PassesDecoderState' if name=='PreparePipeline' else ''
                symbol_name=name
                if name=='ImageMetadata::VisitFields':namespace,symbol_name='13ImageMetadata','VisitFields'
                if row[2].lstrip('_').startswith('ZN3jxl'+namespace+str(len(symbol_name))+symbol_name):
                    if name=='ReadImageMetadata' and row[1]=='0':
                        # ld64 coalesces identical four-byte Bundle::Read wrappers.
                        # Require the actual address-sharing original wrapper and
                        # separately bind ImageMetadata::VisitFields below.
                        peers=[peer for peer in contents.splitlines() if peer.startswith(line.split()[0]) and '15ReadFrameHeaderE' in peer]
                        if len(peers)!=1:raise RuntimeError('Unexpected metadata wrapper alias: '+line)
                        peer=re.search(r'\[\s*(\d+)\]',peers[0])
                        if not peer or 'frame_header.cc.o' not in objects.get(peer[1],''):raise RuntimeError('Metadata alias is not the original Bundle::Read wrapper')
                        aliases.append({'symbol':line,'resolved_peer':peers[0],'object':objects[peer[1]]})
                    elif source not in objects.get(row[1],''):raise RuntimeError('Unexpected source binding: '+line)
                    found[name].append({'object':objects[row[1]],'symbol':line})
    else:
        current=''
        for line in contents.splitlines():
            if re.search(r'\.o\)?\s*$',line):current=line
            if not re.match(r'^\s+0x[0-9a-f]+\s+',line):continue
            for name,source in expected.items():
                if line.rstrip().endswith(')') and re.search(r'jxl::(?:PassesDecoderState::)?'+name+r'\(',line):
                    if source not in current:raise RuntimeError('Unexpected source binding: '+line+' in '+current)
                    found[name].append({'object':current,'symbol':line})
    if any(not rows for rows in found.values()):raise RuntimeError('Missing actual original source bindings: '+str(found))
    return {'compile':command,'link':link,'symbols':found,'spot':spot_binding(contents,True),
            'wrapper_aliases':aliases,'pristine_include_observers':{name:sha(ROOT/'tools/oracle'/name) for name in includes},
            'observer_sha256':sha(ROOT/'tools/oracle/extra_channels.cc'),
            'observer_object_sha256':sha(build/'CMakeFiles/jxl_oracle.dir/extra_channels.cc.o')}

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();output=args.output.resolve();output.mkdir(parents=True,exist_ok=False)
    source=ROOT/'upstream/libjxl';manifest=json.loads((ROOT/'tests/data/extra-channels-fixtures.json').read_text())
    pins={}
    for name in ['', 'third_party/highway','third_party/brotli','third_party/skcms']:
        path=source/name
        revision=subprocess.check_output(['git','-C',str(path),'rev-parse','HEAD'],text=True).strip()
        if subprocess.check_output(['git','-C',str(path),'status','--porcelain','--ignore-submodules=none'],text=True):raise RuntimeError('Original source changed')
        expected=manifest['source_revision'] if not name else subprocess.check_output(['git','-C',str(source),'ls-tree','HEAD',name],text=True).split()[2]
        if revision!=expected:raise RuntimeError('Source pin mismatch')
        pins[name or 'libjxl']=revision
    capture=output/'requests.txt';half_capture=output/'half-requests.txt'
    environment=dict(os.environ,CARGO_BUILD_JOBS='2',RUST_TEST_THREADS='2',JXL_EXTRA_CHANNEL_CAPTURE=str(capture),JXL_EXTRA_CHANNEL_HALF_CAPTURE=str(half_capture))
    with (output/'capture.log').open('w') as log:
        subprocess.run(['cargo','test','--locked','--offline','--all-features','--test','parity','extra_channels::','--','--nocapture'],cwd=ROOT,env=environment,stdout=log,stderr=subprocess.STDOUT,check=True)
    requests=capture.read_text().splitlines();counts=Counter(q.split()[0] for q in requests)
    if counts!={'fs-decode':11280,'ec-meta':123,'ec-export':3912}:raise RuntimeError('Complete-file request inventory changed: '+str(counts))
    boundaries=census(output/'half-stride-census',captured_requests=half_capture)
    report={'half_stride_census_sha256':sha(output/'half-stride-census/report.json'),'half_stride_counts':boundaries['counts'],'pins':pins,'platform':platform.platform(),'requests':len(requests),'counts':dict(counts),'request_sha256':sha(capture),
            'manifest_sha256':sha(ROOT/'tests/data/extra-channels-fixtures.json'),'graphs':{}}
    previous=None
    for graph in ('oracle','oracle-asan'):
        build,directory=ROOT/'target'/graph,output/graph;directory.mkdir()
        binary=build/'jxl_oracle';before=sha(binary);binding=bindings(build,directory)
        writers=[];captures=[]
        for item in manifest['cases']:
            path=ROOT/'tests/data'/item['jxl']
            if sha(path)!=item['sha256']:raise RuntimeError('Genuine fixture changed')
            response,=observe(binary,[item['request']])
            if response!=item['writer_observation'] or bytes.fromhex(response.split()[0])!=path.read_bytes():raise RuntimeError('Actual original writer differs: '+item['name'])
            writers.append(response)
            # Independent original export captures retain every initialized final
            # color/EC word, or the exact decode failure before export.
            capture,=observe(binary,['ec-export '+path.read_bytes().hex()+' 32 1 3 0 0'])
            captures.append(capture)
        for item in manifest['rejected']:
            response,=observe(binary,[item['request']])
            if response!=item['mutation_observation'] or sha(ROOT/'tests/data'/item['jxl'])!=item['sha256']:raise RuntimeError('Original rejected-type mutation control differs')
            writers.append(response)
        (directory/'writer-observations.txt').write_text('\n'.join(writers)+'\n')
        with gzip.open(directory/'native-final-words.txt.gz','wt') as stream:stream.write('\n'.join(captures)+'\n')
        observations=directory/'observations.txt.gz'
        with gzip.open(observations,'wt',compresslevel=1) as stream:
            for at in range(0,len(requests),4):stream.write('\n'.join(observe(binary,requests[at:at+4]))+'\n')
        if previous is not None:
            with gzip.open(previous,'rt') as left,gzip.open(observations,'rt') as right:
                for index,request in enumerate(requests):
                    if next(left)!=next(right):raise RuntimeError(f'Original ordinary/ASan drift at {index}: {request}')
                if next(left,None) is not None or next(right,None) is not None:raise RuntimeError('Trailing original observations')
            with gzip.open(output/'oracle/native-final-words.txt.gz','rt') as left,gzip.open(directory/'native-final-words.txt.gz','rt') as right:
                if left.read()!=right.read():raise RuntimeError('Independent native final captures disagree')
        if sha(binary)!=before:raise RuntimeError('Active original binary changed')
        report['graphs'][graph]={'binary_sha256':before,'binding':binding,'observations_sha256':sha(observations),
                                 'native_final_words_sha256':sha(directory/'native-final-words.txt.gz')}
        previous=observations
    report['audit_passed']=True;(output/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print(f"{len(manifest['cases'])} genuine complete files and {len(manifest['rejected'])} actual rejected-type fixtures agree; {len(requests)} original observations agree ordinary/ASan")
    print('Source bindings and independently generated native final-word captures verified: '+str(output))

if __name__=='__main__':main()
