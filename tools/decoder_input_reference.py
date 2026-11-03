#!/usr/bin/env python3
"""Audit actual pinned input members, callback ownership and all six graphs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import runpy
import shlex
import subprocess
from skcms_binding import live_bindings
ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def run(command, directory, name, *, data=None, cwd=ROOT):
    result = subprocess.run(command, input=data, capture_output=True, cwd=cwd)
    (directory / (name + '.stdout')).write_bytes(result.stdout)
    (directory / (name + '.stderr')).write_bytes(result.stderr)
    if result.returncode:
        raise RuntimeError(f'{name} failed: {result.returncode}')
    return result

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();output=args.output.resolve()
    output.mkdir(parents=True,exist_ok=False)
    runpy.run_path(str(ROOT/'tools/check.py'))['check_source']()
    requests=(ROOT/'tools/captures/decoder_input_requests.txt').read_bytes()
    generated=run(['python3','tools/decoder_input_corpus.py'],output,'corpus').stdout
    if generated!=requests:
        raise RuntimeError('Retained exhaustive request corpus differs from generator')
    if len(requests.splitlines())!=1192:
        raise RuntimeError('Complete input request count changed')
    native=ROOT/'tools/captures/decoder_input.tsv'
    meta=json.loads((ROOT/'tools/captures/decoder_input.json').read_text())
    if sha(native)!=meta['sha256'] or meta['requests']!=24:
        raise RuntimeError('Native original captures changed')
    native_lines=native.read_bytes().splitlines(keepends=True)
    for n in [15,22]:
        if hashlib.sha256(b''.join(native_lines[:n])).hexdigest()!=meta[f'retained_first{n}_sha256']:
            raise RuntimeError('Earlier native capture rows were altered')
    retained_initial=b'\n'.join(requests.splitlines()[24:1183])+b'\n'
    if hashlib.sha256(retained_initial).hexdigest()!=meta['initial_source_requests_sha256']:
        raise RuntimeError('Initial source-only request history changed')
    native_rows=[line.split(b'\t',1) for line in native.read_bytes().splitlines()]
    native_requests=b'\n'.join(row[0] for row in native_rows)+b'\n'
    native_expected=b'\n'.join(row[1] for row in native_rows)+b'\n'
    manifest=json.loads((ROOT/'tools/oracle/source_fragments.json').read_text())
    fragments=[f for f in manifest if f['output'].startswith('original_decoder_input_')]
    if len(fragments)!=4:
        raise RuntimeError('Complete original input member manifest missing')
    owned=['src/jxl/decoder_input.rs','src/jxl/decoder_input_cursor.rs',
           'src/jxl/padded_bytes_callbacks.rs','src/jxl/memory_manager_internal.rs',
           'src/jxl/decode.rs','tests/support/decoder_input.rs',
           'tools/decoder_input_probe.rs','tools/decoder_input_corpus.py',
           'tools/decoder_input_reference.py','tools/oracle/decoder_input.cc',
           'tools/oracle/source_fragments.json','tools/captures/decoder_input_requests.txt',
           'tools/captures/decoder_input.tsv','tools/captures/decoder_input.json']
    before={str(ROOT/p):sha(ROOT/p) for p in owned};sources={};original=None
    for graph in ['oracle','oracle-asan']:
        build=ROOT/'target'/graph;directory=output/graph;directory.mkdir()
        entries=json.loads((build/'compile_commands.json').read_text())
        adapter,=[e for e in entries if Path(e['file']).resolve()==ROOT/'tools/oracle/decoder_input.cc']
        decoder,=[e for e in entries if Path(e['file']).resolve()==ROOT/'upstream/libjxl/lib/jxl/decode.cc']
        original_units=[e for e in entries if Path(e['file']).resolve().is_relative_to(ROOT/'upstream/libjxl')
                        and Path(e['file']).suffix in ['.c','.cc','.cpp']]
        if not original_units:
            raise RuntimeError('Complete original graph compilation inventory missing')
        for unit in original_units:
            flags=unit.get('arguments') or shlex.split(unit['command'])
            expected=['-fsanitize=address,undefined','-fno-sanitize-recover=all'] if graph=='oracle-asan' else []
            if [f for f in flags if 'sanitize' in f]!=expected:
                raise RuntimeError(f"Original unit instrumentation changed: {unit['file']}")
        memory,=[e for e in entries if Path(e['file']).resolve()==ROOT/'upstream/libjxl/lib/jxl/memory_manager_internal.cc']
        for entry in [adapter,decoder,memory]:
            flags=entry.get('arguments') or shlex.split(entry['command'])
            expected=['-fsanitize=address,undefined','-fno-sanitize-recover=all'] if graph=='oracle-asan' else []
            if [f for f in flags if 'sanitize' in f]!=expected:
                raise RuntimeError('Original/adapter nonrecovering instrumentation changed')
            if [f for f in flags if f.startswith('-O')]!=['-O3'] or '-DNDEBUG' not in flags:
                raise RuntimeError('Original/adapter optimization changed')
            if any(f.startswith(('-ffast-math','-ffp-contract','-ffp-model','-funsafe-math')) for f in flags):
                raise RuntimeError('Unexpected numeric source override')
        for f in fragments:
            path=ROOT/'upstream/libjxl'/f['path'];text=path.read_text()
            if text.count(f['start'])!=1:
                raise RuntimeError('Ambiguous original input member boundary')
            start=text.index(f['start']);body=text[start:text.index(f['end'],start)]+'\n'
            if hashlib.sha256(body.encode()).hexdigest()!=f['sha256']:
                raise RuntimeError('Original input member body changed')
            line=text[:start].count('\n')+1;notice=''.join(text.splitlines(keepends=True)[:5])
            expected=notice+f'#line {line} {json.dumps(str(path))}\n'+body
            if (build/'generated'/f['output']).read_text()!=expected:
                raise RuntimeError('Compiled input fragment differs from pinned original')
        paths=[build/'jxl_oracle',build/'compile_commands.json',build/'CMakeFiles/jxl_oracle.dir/link.txt',build/'CMakeFiles/jxl_oracle.dir/decoder_input.cc.o']
        for path in paths:before[str(path)]=sha(path)
        link=shlex.split((build/'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
        binary=directory/'bound-original';mapping=directory/'original.map'
        link[link.index('-o')+1]=str(binary)
        link.append(('-Wl,-map,' if platform.system()=='Darwin' else '-Wl,-Map,')+str(mapping))
        run(link,directory,'link',cwd=build)
        bindings=live_bindings(binary,mapping,{
            'DecoderInputCommand':(r'_+Z19DecoderInputCommand[^.]*',r'(?:^|/)decoder_input\.cc\.o$'),
            'OriginalSetInput':(r'_?JxlDecoderSetInput',r'libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?decode\.cc\.o\)$'),
            'AlignedMemoryCreate':(r'_+ZN3jxl13AlignedMemory6CreateE[^.]*',r'libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?memory_manager_internal\.cc\.o\)$'),
            'AlignedMemoryDestructor':(r'_+ZN3jxl13AlignedMemoryD1Ev',r'libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?memory_manager_internal\.cc\.o\)$'),
            'OriginalJxlSignatureCheck':(r'_?JxlSignatureCheck',r'libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?decode\.cc\.o\)$'),
        })
        result=run([str(binary)],directory,'observations',data=requests)
        if result.stderr or len(result.stdout.splitlines())!=1192:
            raise RuntimeError('Unexpected original output/diagnostic count')
        if original is not None and original!=result.stdout:
            raise RuntimeError('Ordinary and instrumented source input observations differ')
        original=result.stdout
        retained=b'\n'.join(original.splitlines()[24:1183])+b'\n'
        if hashlib.sha256(retained).hexdigest()!=meta['initial_source_observations_sha256']:
            raise RuntimeError('Initial source-only observation words changed')
        captures=run([str(binary)],directory,'native-captures',data=native_requests)
        if captures.stdout!=native_expected or captures.stderr:
            raise RuntimeError('Native source captures changed')
        sources[graph]={'bindings':bindings,'compile_commands':[adapter,decoder,memory],'instrumented_original_units':len(original_units),'binary_sha256':sha(binary),'map_sha256':sha(mapping),'observation_sha256':hashlib.sha256(original).hexdigest(),'observation_bytes':len(original)}
    rust={}
    for profile,prefix,cargo_flags,rust_flags in [('debug',[],[],[]),('release',[],['--release'],['-O']),('msrv',['+1.85'],[],[])]:
        directory=output/profile;directory.mkdir()
        built=run(['cargo',*prefix,'build','--locked','--offline','--lib','--all-features','--message-format=json','--target-dir',str(directory/'build'),*cargo_flags],directory,'build')
        artifacts=[r for line in built.stdout.splitlines() if (r:=json.loads(line)).get('reason')=='compiler-artifact' and r['target']['name']=='jxl']
        lib,=[Path(f) for f in artifacts[-1]['filenames'] if f.endswith('.rlib')]
        binary=directory/'input-probe'
        run(['rustc',*prefix,'tools/decoder_input_probe.rs','--edition=2024','--extern',f'jxl={lib}','-L',f'dependency={lib.parent}',*rust_flags,'-o',str(binary)],directory,'probe-build')
        observed=run([str(binary)],directory,'observations',data=requests)
        if observed.stdout!=original or observed.stderr:
            raise RuntimeError(f'Actual Rust {profile} differs from original')
        captures=run([str(binary)],directory,'native-captures',data=native_requests)
        if captures.stdout!=native_expected or captures.stderr:
            raise RuntimeError('Actual Rust native capture mismatch')
        rust[profile]={'binary_sha256':sha(binary),'observation_sha256':hashlib.sha256(observed.stdout).hexdigest(),'native_captures':24}
    if any(sha(p)!=digest for p,digest in before.items()):
        raise RuntimeError('Source audit altered active original artifacts')
    report={'source':'a16d32d0ac113f5e54125e49bb571281c5d0b704','requests':1192,'native_captures':24,'request_sha256':hashlib.sha256(requests).hexdigest(),'source_fragments':fragments,'source_graphs':sources,'rust_graphs':rust,'all_six_exact':True,'unchanged_source_artifacts':before}
    (output/'report.json').write_text(json.dumps(report,indent=2)+'\n')
    print('All six input graphs match 1,192 exact original histories and 24 native captures')

if __name__=='__main__':main()
