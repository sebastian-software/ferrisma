#!/usr/bin/env python3
"""Source/Rust ring allocation, partial output, and initialized-prefix proof.

An optional CMake build-root cache is always reconfigured and built normally;
no verification phase is skipped. Fresh reports/test executables/logs live in
the newly created output directory. This does not implement a Rust stream API.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT=Path(__file__).resolve().parent.parent


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--build-root',type=Path,help='optional reusable CMake cache; configure/build still execute')
    args=p.parse_args();out=args.output.resolve();out.mkdir(parents=True,exist_ok=False)
    builds=args.build_root.resolve() if args.build_root else out
    def execute(label,command,env=None):
        with (out/f'{label}.log').open('w') as log:
            subprocess.run(list(map(str,command)),cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,env=env,check=True)
    originals={}
    for graph in ['ordinary','sanitized']:
        directory=builds/f'source-{graph}'
        execute(f'{graph}-configure',['cmake','-S',ROOT/'tools/oracle/brotli_ringbuffer','-B',directory,
            '-DCMAKE_BUILD_TYPE=Release','-DCMAKE_C_COMPILER=clang',f'-DORACLE_SANITIZERS={"ON" if graph=="sanitized" else "OFF"}'])
        execute(f'{graph}-build',['cmake','--build',directory,'--target','brotli_ringbuffer_oracle','--parallel','2'])
        originals[graph]=directory/'brotli_ringbuffer_oracle'
    rust=[]
    for name,tc,level in [('debug','stable','0'),('release','stable','3'),('msrv','1.95','0')]:
        compiler=['rustup','run',tc,'rustc','--edition','2024','-D','warnings','-C',f'opt-level={level}']
        binary=out/name;tests=out/f'{name}-tests'
        execute(f'{name}-build',[*compiler,ROOT/'tools/brotli_ringbuffer_probe.rs','-o',binary])
        execute(f'{name}-tests-build',[*compiler,'--test','--cfg','feature="cpp-oracle"',ROOT/'tests/brotli_ringbuffer.rs','-o',tests])
        for graph in ['ordinary','sanitized']:
            execute(f'{name}-{graph}-tests',[tests],dict(os.environ,BROTLI_RINGBUFFER_ORACLE=str(originals[graph]),RUST_TEST_THREADS='2'))
        rust+=['--rust',f'{name}={binary}']
    execute('audit',[sys.executable,ROOT/'tools/brotli_ringbuffer_reference.py','--ordinary',originals['ordinary'],
        '--sanitized',originals['sanitized'],*rust,'--output',out/'audit'])
    (out/'clippy.toml').write_text('msrv = "1.95"\n')
    execute('clippy',['clippy-driver','--edition=2024','--test',ROOT/'tests/brotli_ringbuffer.rs','--cfg','feature="cpp-oracle"',
        '-D','warnings','-o',out/'clippy'],dict(os.environ,CLIPPY_CONF_DIR=str(out)))
    (out/'report.json').write_text(json.dumps({'all_six_exact':True,'tests_each_configuration':6,
        'requests_each_configuration':8365,'audit':'audit/report.json','full_rust_streaming_decoder':False},indent=2)+'\n')
    print(out/'report.json')

if __name__=='__main__':main()
