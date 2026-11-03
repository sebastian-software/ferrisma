#!/usr/bin/env python3
"""Audit actual pristine skcms transform, baseline engine and private helpers."""
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
from skcms_binding import engine_configuration, live_bindings
from skcms_fitting_reference import source_command
ROOT=Path(__file__).resolve().parent.parent

def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def inventory():
    """Compare all format/stage names against the actual pinned headers."""
    source=ROOT/'upstream/libjxl/third_party/skcms/src'
    public=(source/'skcms_public.h').read_text()
    rust=(ROOT/'src/skcms/transform_types.rs').read_text()
    result={}
    for kind,carrier in [('PixelFormat','PixelFormat'),('AlphaFormat','AlphaFormat')]:
        body=public.split('typedef enum skcms_'+kind+' {',1)[1].split('}',1)[0]
        names=re.findall(r'\bskcms_'+kind+r'_(\w+)\s*,',body)
        body=rust.split('impl '+carrier+' {',1)[1].split('\n}',1)[0]
        actual=re.findall(r'pub const (\w+): Self = Self\((\d+)\);',body)
        assert actual==[(name,str(i))for i,name in enumerate(names)],kind
        result[kind]=names
    header=(source/'skcms_Transform.h').read_text().split('enum class Op',1)[0]
    names=re.findall(r'\bM\((\w+)\)',header)
    body=rust.split('pub(crate) enum Op {',1)[1].split('}',1)[0]
    assert re.findall(r'^\s*(\w+),',body,re.M)==names
    assert (ROOT/'tests/data/skcms-transform-ops.txt').read_text().splitlines()==names
    assert len(names)==79 and len(result['PixelFormat'])==42
    result['Op']=names
    return result

def object_evidence(entry,directory,engine='baseline'):
    """Retain disassembly from the object actually linked into the CMS archive."""
    command=entry.get('arguments')or shlex.split(entry['command'])
    obj=Path(command[command.index('-o')+1])
    if not obj.is_absolute():obj=Path(entry['directory'])/obj
    destination=directory/(engine+'-object-disassembly.txt')
    disassemble=['otool','-tvV',str(obj)]if platform.system()=='Darwin'else['objdump','-dr',str(obj)]
    with destination.open('w')as stream:
        subprocess.run(disassemble,stdout=stream,stderr=subprocess.STDOUT,check=True)
    assert destination.stat().st_size>1024
    return {'command':disassemble,'object':str(obj),'object_sha256':sha(obj),'disassembly_sha256':sha(destination)}

def build_private(build):
    build=Path(build).resolve();entry,original=source_command(build)
    obj=build/'skcms_transform_private.o';binary=build/'skcms_transform_private'
    command=original.copy();command[command.index('-o')+1]=str(obj);command[command.index('-c')+1]=str(ROOT/'tools/oracle/skcms_transform_private.cc')
    with (build/'skcms_transform_private-compile.log').open('w') as log:subprocess.run(command,cwd=entry['directory'],stdout=log,stderr=subprocess.STDOUT,check=True)
    adapter=build/'CMakeFiles/jxl_oracle.dir/skcms_parser.cc.o'
    link=[command[0],str(obj),str(adapter),str(build/'upstream/lib/libjxl_cms.a'),'-o',str(binary)]
    for flag in command[1:]:
        if flag.startswith(('-fsanitize=','-fno-sanitize-recover=','-stdlib=','--target=','-mmacosx-version-min=')):link.append(flag)
    if '-target' in command:
        i=command.index('-target');link+=command[i:i+2]
    mapping=build/'skcms_transform_private.map';link+=[('-Wl,-map,' if platform.system()=='Darwin' else '-Wl,-Map,')+str(mapping)]
    if platform.system()=='Linux':link+=['-lm']
    with (build/'skcms_transform_private-link.log').open('w') as log:subprocess.run(link,cwd=build,stdout=log,stderr=subprocess.STDOUT,check=True)
    receipt={'original':entry,'compile':command,'link':link,'compiler':subprocess.check_output([command[0],'--version'],text=True),'sha256':{str(p):sha(p)for p in [Path(entry['file']),ROOT/'tools/oracle/skcms_transform_private.cc',ROOT/'tools/oracle/skcms_transform.cc',ROOT/'tools/oracle/skcms_parser.cc',obj,adapter,build/'upstream/lib/libjxl_cms.a',binary,mapping]}}
    (build/'skcms_transform_private.json').write_text(json.dumps(receipt,indent=2)+'\n');return binary

def binding(build,directory):
    entry,original=source_command(build);receipt=json.loads((build/'skcms_transform_private.json').read_text())
    expected=original.copy();expected[expected.index('-o')+1]=str(build/'skcms_transform_private.o');expected[expected.index('-c')+1]=str(ROOT/'tools/oracle/skcms_transform_private.cc')
    assert receipt['original']==entry and receipt['compile']==expected
    for path,wanted in receipt['sha256'].items():assert sha(path)==wanted,path
    commands=json.loads((build/'compile_commands.json').read_text());baseline,=[e for e in commands if Path(e['file']).name=='skcms_TransformBaseline.cc']
    baseline_command=baseline.get('arguments')or shlex.split(baseline['command'])
    normalized=baseline_command.copy();normalized[normalized.index('-o')+1]=original[original.index('-o')+1];normalized[normalized.index('-c')+1]=original[original.index('-c')+1]
    assert normalized==original,'Baseline source must inherit the identical original configuration'
    assert not any('SKCMS_PORTABLE' in a for a in original)
    engines,engine_specs=engine_configuration(commands,original)
    link=shlex.split((build/'CMakeFiles/jxl_oracle.dir/link.txt').read_text());bound=directory/'public-bound';mapping=directory/'public.map'
    link[link.index('-o')+1]=str(bound);link+=[('-Wl,-map,' if platform.system()=='Darwin' else '-Wl,-Map,')+str(mapping)]
    with (directory/'public-link.log').open('w')as log:subprocess.run(link,cwd=build,stdout=log,stderr=subprocess.STDOUT,check=True)
    public=['skcms_Transform','skcms_MakeUsableAsDestination','skcms_MakeUsableAsDestinationWithSingleCurve','skcms_ApproximatelyEqualProfiles','skcms_TRCs_AreApproximateInverse','skcms_ParseWithA2BPriority']
    specs={name:(r'_?'+name,r'libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms\.cc\.o\)$')for name in public}
    baseline_symbol=r'_?_*ZN13skcms_private8baseline11run_programE.*';baseline_owner=r'libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms_TransformBaseline\.cc\.o\)$'
    specs['baseline::run_program']=(baseline_symbol,baseline_owner);specs.update(engine_specs)
    public_bound=live_bindings(bound,mapping,specs)
    private_map=build/'skcms_transform_private.map';text=private_map.read_text()
    # A pristine source include supplies the helper's public definitions. Only
    # the unchanged original stage engines may be pulled from the CMS archive.
    assert not re.search(r'libjxl_cms\.a[^\n]*\bskcms\.cc\.o',text)
    names=['select_curve_op','select_curve_ops','bytes_per_pixel','set_reference_pq_ish_trc','set_sdr_hlg_ish_trc','prep_for_destination']
    private_specs={name:(r'_+ZL'+str(len(name))+name+r'.*',r'(?:^|/)skcms_transform_private\.o$')for name in names}
    private_specs['baseline::run_program']=(baseline_symbol,baseline_owner)
    private_bound=live_bindings(build/'skcms_transform_private',private_map,private_specs)
    adapters={}
    for name in ['skcms_transform.cc','skcms_parser.cc']:
        adapter,=[e for e in commands if Path(e['file']).name==name]
        argv=adapter.get('arguments')or shlex.split(adapter['command'])
        wanted=['-fsanitize=address,undefined','-fno-sanitize-recover=all']if build.name=='oracle-asan'else[]
        assert [a for a in argv if 'sanitize' in a]==wanted,(name,argv)
        adapters[name]=adapter
    return {'public':public_bound,'private':private_bound,'baseline_command':baseline,'baseline_object':object_evidence(baseline,directory),'engine_commands':engines,'engine_objects':{name:object_evidence(entry,directory,name)for name,entry in engines.items()},'adapter_commands':adapters,'private_receipt':receipt}

def observe(binary,requests):
    p=subprocess.run([str(binary)],input='\n'.join(requests)+'\n',text=True,capture_output=True)
    assert p.returncode==0 and not p.stderr,(binary,p.returncode,p.stderr[:3000])
    rows=p.stdout.splitlines();assert len(rows)==len(requests);return rows

def retained_preflight(requests):
    fixture=ROOT/'tests/data/skcms-transform-preflight.requests.gz'
    manifest=json.loads((ROOT/'tests/data/skcms-transform-preflight.json').read_text())
    assert sha(fixture)==manifest['gzip_sha256']
    raw=gzip.decompress(fixture.read_bytes())
    assert hashlib.sha256(raw).hexdigest()==manifest['requests_sha256']
    original=raw.decode().splitlines()
    assert len(original)==manifest['requests']==11448
    assert not set(original)-set(requests),'Initial transform requests were dropped'
    indices=manifest['initial_failure_indices']
    assert len(indices)==manifest['initial_failures']==148
    assert [hashlib.sha256(original[i].encode()).hexdigest()for i in indices]==manifest['initial_failure_requests_sha256']
    return original,manifest

def source_count_probes(output):
    directory=output/'source-only-channel-count';directory.mkdir()
    requests=['skcms-transform-private-channel-count -2147483648','skcms-transform-unknown 16','skcms-transform-unknown 17']
    report=[]
    for i,request in enumerate(requests):
        row={'request':request,'observations':{}}
        for graph in ['oracle','oracle-asan']:
            binary=ROOT/'target'/graph/('skcms_transform_private'if '-private-'in request else'jxl_oracle')
            p=subprocess.run([str(binary)],input=request+'\n',text=True,capture_output=True)
            prefix=directory/(str(i)+'-'+graph)
            prefix.with_suffix('.stdout').write_text(p.stdout);prefix.with_suffix('.stderr').write_text(p.stderr)
            row['observations'][graph]={'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
            if graph=='oracle-asan':
                assert p.returncode!=0 and 'skcms.cc:2536:'in p.stderr and 'signed integer overflow'in p.stderr and '-2147483648 - 1'in p.stderr
        report.append(row)
    return report

def audit(output,record_native):
    output.mkdir(parents=True,exist_ok=False);report={'platform':platform.platform(),'pins':{},'inventory':inventory(),'graphs':{}}
    source=ROOT/'upstream/libjxl'
    for name in ['', 'third_party/highway','third_party/brotli','third_party/skcms']:
        path=source/name;actual=subprocess.check_output(['git','-C',str(path),'rev-parse','HEAD'],text=True).strip()
        expected=json.loads((ROOT/'upstream.json').read_text())['revision']if not name else subprocess.check_output(['git','-C',str(source),'ls-tree','HEAD',name],text=True).split()[2]
        assert actual==expected and not subprocess.check_output(['git','-C',str(path),'status','--porcelain','--ignore-submodules=none'],text=True).strip();report['pins'][name or'libjxl']=actual
    capture=output/'requests.txt';env=dict(os.environ,JXL_SKCMS_TRANSFORM_CAPTURE=str(capture),JXL_CPP_ORACLE=str(ROOT/'target/oracle/jxl_oracle'),JXL_SKCMS_TRANSFORM_ORACLE=str(ROOT/'target/oracle/skcms_transform_private'),RUST_TEST_THREADS='2')
    with (output/'capture.log').open('w')as log:subprocess.run(['cargo','test','--locked','--offline','--all-features','--test','parity','skcms_transform::','--','--nocapture'],cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
    requests=capture.read_text().splitlines();report.update(requests=len(requests),counts=dict(Counter(q.split()[0]for q in requests)),request_sha256=sha(capture))
    preflight,manifest=retained_preflight(requests);preflight_set=set(preflight)
    report['retained_preflight']={'requests':len(preflight),'initial_failures':manifest['initial_failures'],'manifest':manifest}
    previous={};native=[]
    # One independently captured input from each public family, followed by
    # source gamma/PQ/HLG and CLUT contexts with genuinely unknown inactive words.
    prefixes=['skcms-transform ','skcms-transform-usable ','skcms-transform-single ','skcms-transform-equal ','skcms-transform-inverse ','skcms-transform-unknown ']
    chosen=[next(q for q in sorted(requests)if q.startswith(prefix))for prefix in prefixes]
    chosen+=sorted(q for q in requests if q.startswith('skcms-transform-private-partial-select '))[:4]
    chosen+=[next(q for q in sorted(requests)if q.startswith('skcms-transform-private-partial-stage '+family+'_'))for family in ['gamma','pq','hlg','hlginv']]
    chosen+=sorted(q for q in requests if q.startswith('skcms-transform-private-partial-grid '))[:2]
    assert len(chosen)==16
    for graph in ['oracle','oracle-asan']:
        build=ROOT/'target'/graph;directory=output/graph;directory.mkdir();stable=[build/'jxl_oracle',build/'skcms_transform_private',build/'skcms_transform_private.o',build/'upstream/lib/libjxl_cms.a'];before={str(p):sha(p)for p in stable}
        bindings=binding(build,directory)
        for kind in ['public','private']:
            selected=[q for q in requests if ('-private-'in q)==(kind=='private')];binary=build/('jxl_oracle'if kind=='public'else'skcms_transform_private');target=directory/(kind+'.txt.gz')
            retained={}
            with gzip.open(target,'wt',compresslevel=1)as stream:
                for at in range(0,len(selected),64):
                    batch=selected[at:at+64];rows=observe(binary,batch)
                    if kind=='public':assert rows==observe(build/'skcms_transform_private',batch),'Pristine helper context changed public output'
                    retained.update((q,row)for q,row in zip(batch,rows)if q in preflight_set)
                    stream.write('\n'.join(rows)+'\n')
            if kind=='public':
                assert set(retained)==preflight_set
                digest=hashlib.sha256()
                for q in preflight:digest.update((retained[q]+'\n').encode())
                report['retained_preflight'][graph+'_observations_sha256']=digest.hexdigest()
                if platform.platform()==manifest['capture_platform']:
                    assert digest.hexdigest()==manifest['original_observations_sha256'],'Initial original transform words changed'
            if kind in previous:
                with gzip.open(previous[kind],'rt')as left,gzip.open(target,'rt')as right:
                    for i,q in enumerate(selected):assert next(left)==next(right),(graph,kind,i,q[:200])
                    assert next(left,None)is None and next(right,None)is None
            previous[kind]=target
        after={str(p):sha(p)for p in stable};assert before==after
        report['graphs'][graph]={'bindings':bindings,'stable_sha256':after}
        captures=[(q,observe(build/('skcms_transform_private'if '-private-'in q else'jxl_oracle'),[q])[0])for q in chosen]
        assert not native or native==captures,'Independent native captures changed between source graphs'
        native=captures
    report['source_only_channel_count']=source_count_probes(output)
    captures=output/'native.tsv';captures.write_text('\n'.join(q+'\t'+row for q,row in native)+'\n')
    report['native_captures']={'count':len(native),'sha256':sha(captures),'architecture':platform.machine()}
    if record_native:
        assert platform.system()=='Darwin'and platform.machine()=='arm64','The retained fixture is explicitly the macOS ARM baseline'
        fixture=ROOT/'tests/data/skcms-transform-native-arm64.tsv';fixture.write_bytes(captures.read_bytes())
        (ROOT/'tests/data/skcms-transform-native-arm64.json').write_text(json.dumps({'pins':report['pins'],'platform':report['platform'],'architecture':platform.machine(),'compiler':report['graphs']['oracle']['bindings']['private_receipt']['compiler'],'captures':len(native),'tsv_sha256':sha(fixture)},indent=2)+'\n')
    (output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'requests':len(requests),'report_sha256':sha(output/'report.json')}),flush=True)

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--build-private',type=Path);p.add_argument('--output',type=Path);p.add_argument('--record-native',action='store_true');args=p.parse_args()
    if args.build_private:build_private(args.build_private.resolve())
    elif args.output:audit(args.output.resolve(),args.record_native)
    else:p.error('Choose --build-private or --output')
if __name__=='__main__':main()
