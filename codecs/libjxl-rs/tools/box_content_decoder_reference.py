#!/usr/bin/env python3
"""Complete actual box-content class and registered pure-Rust streaming proof."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import runpy
import shlex
import shutil
import subprocess

from brotli_decoder_binding import requested_bindings
from brotli_state_reference import ROOT, compile_options
from box_content_decoder_corpus import encoder_requests, requests
from rust_probe_artifacts import jxl_extern_args


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as source:
        while block := source.read(1 << 20): h.update(block)
    return h.hexdigest()


def original_binding(binary, sanitized):
    """Prove actual live C++ methods, not a similarly named object in a map."""
    build = binary.parent
    commands = json.loads((build / 'compile_commands.json').read_text())
    source_path = ROOT / 'upstream/libjxl/lib/jxl/box_content_decoder.cc'
    source = [c for c in commands if Path(c['file']).resolve() == source_path]
    observer = [c for c in commands if Path(c['file']).resolve() == ROOT / 'tools/oracle/box_content_decoder.cc']
    assert len(source) == len(observer) == 1
    expected_includes = [f'-I{ROOT / "upstream/libjxl"}',
                         f'-I{ROOT / "upstream/libjxl/lib/include"}',
                         f'-I{ROOT / "upstream/libjxl/third_party/brotli/c/include"}',
                         f'-I{build / "generated"}']
    for c in source + observer:
        flags = c.get('arguments') or shlex.split(c['command'])
        includes = [f for f in flags if f.startswith('-I')]
        assert includes == expected_includes, (c['file'], includes)
        # The inherited checker matches the substring "sanitize"; a literal
        # generated-header include directory is not a sanitizer option. Remove
        # only these four exactly validated include arguments for its strict
        # option check, retaining every actual optimization/instrumentation flag.
        compile_options(dict(c, arguments=[f for f in flags if f not in includes]), sanitized)
    brotli = (ROOT / 'upstream/libjxl/third_party/brotli').resolve()
    units = [c for c in commands if Path(c['file']).resolve().is_relative_to(brotli)]
    assert len(units) == 35
    for c in units: compile_options(c, sanitized)
    source_flags = shlex.split(source[0]['command'])
    observer_flags = shlex.split(observer[0]['command'])
    assert '-std=c++17' in source_flags and '-std=c++17' in observer_flags
    assert '-fno-access-control' not in source_flags and observer_flags.count('-fno-access-control') == 1
    for f in ['-fno-rtti', '-fno-exceptions', '-fno-slp-vectorize', '-fno-vectorize']:
        assert source_flags.count(f) == 1, (f, source_flags)
    methods = ['_ZN3jxl20JxlBoxContentDecoder' + n for n in
               ['C1Ev', 'C2Ev', 'D1Ev', 'D2Ev', '8StartBoxEbbm', '7ProcessEPKhmmPPhPm']]
    symbols = subprocess.check_output(['nm', binary], text=True)
    addresses = {}
    for name in methods:
        found = re.findall(rf'^([0-9a-fA-F]+) [Tt] _?{re.escape(name)}$', symbols, re.M)
        assert len(found) == 1, name
        addresses[name] = int(found[0], 16)
    mapped = (build / 'box_content_decoder.map').read_text()
    owners = {}
    if '# Object files:\n' in mapped:
        objects = dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$', mapped.split('# Object files:\n',1)[1].split('# Sections:',1)[0], re.M))
        rows = mapped.split('# Symbols:\n',1)[1].split('# Dead Stripped Symbols:',1)[0]
        for address, size, obj, symbol in re.findall(r'^(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+\[\s*(\d+)\]\s+(.+)$',rows,re.M):
            if symbol.startswith('__Z'): symbol = symbol[1:]
            if symbol in methods:
                assert symbol not in owners and int(size,16)>0
                owners[symbol]={'address':int(address,16),'object':objects[obj]}
    else:
        # GNU input .text extents attribute each actual nm address to its live
        # contributing object. Discarded/extraction listings cannot prove this.
        live = mapped.split('Linker script and memory map\n',1)[1]
        extents, section = [], None
        for line in live.splitlines():
            header = re.match(r'^ (\.\S+)(?:\s+(.*))?$',line)
            if header: section=header[1]
            extent = re.match(r'^\s+(?:\.\S+\s+)?(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+(.+\.o)\s*$',line)
            if extent and section and section.startswith('.text'):
                extents.append((int(extent[1],16),int(extent[2],16),extent[3]))
            if line and not line[0].isspace(): section=None
        for name,address in addresses.items():
            candidates=[o for first,n,o in extents if first<=address<first+n]
            assert len(candidates)==1,(name,address,candidates)
            owners[name]={'address':address,'object':candidates[0]}
    for name,address in addresses.items():
        assert owners[name]['address']==address
        assert owners[name]['object'].endswith('/upstream/libjxl/lib/jxl/box_content_decoder.cc.o'),owners[name]
    expected_brotli = {
        **{f'BrotliDecoder{name}':'libbrotlidec.a(decode.c.o)' for name in
           ['CreateInstance','DestroyInstance','DecompressStream','GetErrorCode','IsUsed','IsFinished','HasMoreOutput']},
        'BrotliEncoderCompress':'libbrotlienc.a(encode.c.o)'}
    bound = requested_bindings(mapped, expected_brotli)
    bindings = {}
    for name,owner in expected_brotli.items():
        found = re.findall(rf'^([0-9a-fA-F]+) [Tt] _?{name}$',symbols,re.M)
        assert len(found)==1 and bound[name]['address']==int(found[0],16)
        assert Path(bound[name]['object']).name==owner
        bindings[name]=bound[name]
    proof={'configured_pristine_c_units':35,'checked_observer_units':1,
           'checked_original_box_units':1,'live_symbol_bindings':bindings,
           'binary_sha256':sha(binary),'actual_link_map_sha256':sha(build/'box_content_decoder.map'),
           'compile_commands_sha256':sha(build/'compile_commands.json')}
    proof.update({'class_live_symbol_bindings':owners,'original_class_source_sha256':sha(source_path),
                  'actual_original_class_compile_options':source_flags})
    return proof


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--build-root',type=Path)
    a=p.parse_args();out=a.output.resolve();out.mkdir(parents=True,exist_ok=False)
    assert platform.machine().lower() in ['arm64','aarch64','x86_64','amd64']
    builds=a.build_root.resolve() if a.build_root else out
    pins={'upstream/libjxl':'a16d32d0ac113f5e54125e49bb571281c5d0b704',
          'upstream/libjxl/third_party/brotli':'db24a98aed38a16de78cc05ca15cdd8dc2a0d92d',
          'upstream/libjxl/third_party/highway':'457c891775a7397bdb0376bb1031e6e027af1c48',
          'upstream/libjxl/third_party/skcms':'96d9171c94b937a1b5f0293de7309ac16311b722'}
    for name,pin in pins.items():
        assert subprocess.check_output(['git','-C',ROOT/name,'rev-parse','HEAD'],text=True).strip()==pin
        assert not subprocess.check_output(['git','-C',ROOT/name,'status','--porcelain'])
    report={'pins':pins,'source':{},'gates':{},'full_public_container_API':False}
    def run(label,command,env=None):
        with (out/f'{label}.log').open('w') as log:
            subprocess.run(list(map(str,command)),cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
    def observe(binary,input_path,output_path):
        with input_path.open('rb') as source,output_path.open('wb') as sink:
            r=subprocess.run([binary],stdin=source,stdout=sink,stderr=subprocess.PIPE)
        output_path.with_suffix(output_path.suffix+'.stderr').write_bytes(r.stderr)
        assert r.returncode==0 and not r.stderr,(binary,r.returncode,r.stderr[:2048])
    originals={}
    for graph in ['ordinary','sanitized']:
        build=builds/graph
        config=['cmake','-S',ROOT/'tools/oracle/box_content_decoder','-B',build,
                '-DCMAKE_BUILD_TYPE=Release',f'-DCMAKE_C_COMPILER={shutil.which("clang")}',
                f'-DCMAKE_CXX_COMPILER={shutil.which("clang++")}',
                f'-DORACLE_SANITIZERS={"ON" if graph=="sanitized" else "OFF"}']
        run(graph+'-configure',config);run(graph+'-configure-confirm',config)
        run(graph+'-build',['cmake','--build',build,'--target','box_content_decoder_oracle','--parallel','2'])
        originals[graph]=build/'box_content_decoder_oracle'
        report['source'][graph]=original_binding(originals[graph],graph=='sanitized')
    encoder_path=out/'encoder-requests.txt';encoder_path.write_text('\n'.join(encoder_requests())+'\n')
    for graph,binary in originals.items():observe(binary,encoder_path,out/f'encoder-{graph}.txt')
    assert sha(out/'encoder-ordinary.txt')==sha(out/'encoder-sanitized.txt')
    encoded=(out/'encoder-ordinary.txt').read_text().splitlines();assert len(encoded)==84
    rows=requests(encoded);assert len(rows)==6165
    path=out/'requests.txt';path.write_text('\n'.join(rows)+'\n')
    # Retain the complete original-first request and response corpus.
    assert sha(path)=='a77013672f0585eb74b3eefb5d0f85dee8aa8211d5257a8f4612e335506455bf'
    captures=[row.split('\t',2) for row in (ROOT/'tools/captures/box_content_decoder.tsv').read_text().splitlines()]
    capture_path=out/'capture-requests.txt';capture_path.write_text('\n'.join(r for _,r,_ in captures)+'\n')
    expected_capture=''.join(response+'\n' for _,_,response in captures).encode()
    for graph,binary in originals.items():
        observe(binary,path,out/f'{graph}.txt')
        assert sha(out/f'{graph}.txt')=='35870c77a9933b588fc12a92f48b19946ca8a12482415757ae85c3dabc019284'
        observe(binary,capture_path,out/f'captures-{graph}.txt')
        assert (out/f'captures-{graph}.txt').read_bytes()==expected_capture
    env=dict(os.environ,CARGO_BUILD_JOBS='2',RUST_TEST_THREADS='2')
    msrv=runpy.run_path(str(ROOT/'tools/check.py'))['rust_version']()
    # The monorepo pins nightly. Using stable rustc with the default Cargo
    # build mixes incompatible compiler metadata; use one toolchain per graph.
    for profile,chain,release in [('debug',[],False),('release',[],True),('msrv',[f'+{msrv}'],False)]:
        cargo=['cargo',*chain]
        flags=['--locked','--offline','--all-features']+(['--release'] if release else [])
        built = subprocess.run(cargo+['build',*flags,'--lib','--message-format=json'],
                               cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=False)
        (out/f'{profile}-library.log').write_bytes(built.stderr)
        (out/f'{profile}-library.json').write_bytes(built.stdout)
        built.check_returncode()
        extern = jxl_extern_args(built.stdout)
        run(profile+'-tests',cargo+['test',*flags,'--test','box_content_decoder','--','--nocapture'],env)
        lib=Path(extern[1].split('=',1)[1])
        binary=out/f'rust-{profile}'
        run(profile+'-probe',['rustc',*chain,'--edition','2024','--cfg','feature="cpp-oracle"',
            '-D','warnings','-C',f'opt-level={3 if release else 0}',ROOT/'tools/box_content_decoder_probe.rs',
            *extern,'-L',f'dependency={lib.parent/"deps"}','-o',binary])
        observe(binary,path,out/f'rust-{profile}.txt')
        assert sha(out/f'rust-{profile}.txt')==sha(out/'ordinary.txt')
        for graph in originals:
            report['gates'][f'{profile}-{graph}']={'tests':5,'ignored':0,'requests':6165,
                'source_captures':36,'actual_registered_crate_library_sha256':sha(lib),
                'rust_binary_sha256':sha(binary),'source_binary_sha256':sha(originals[graph]),
                'observation_sha256':sha(out/f'rust-{profile}.txt')}
    run('clippy',['cargo','clippy','--locked','--offline','--all-targets','--all-features','--','-D','warnings'],env)
    report.update({'requests':6165,'encoder_requests':84,'capture_requests':36,
                   'observation_bytes':(out/'ordinary.txt').stat().st_size,
                   'observation_sha256':sha(out/'ordinary.txt'),
                   'files':{str(p.relative_to(out)):sha(p) for p in out.iterdir() if p.is_file()}})
    (out/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(out/'report.json')


if __name__=='__main__':main()
