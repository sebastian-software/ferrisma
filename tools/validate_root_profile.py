#!/usr/bin/env python3
"""Validate native root consumers against the identical original C configuration."""
import argparse
import json
from pathlib import Path
import re
import shlex
import subprocess
import shutil
import tempfile

from check_root_profile import ROOT, digest, profile_work, validate_profile
from compare_results import results
import root_variants

SMOKE = ['avif16bittest', 'avifallocationtest', 'avifbasictest', 'avifchangesettingtest',
         'avifcodectest', 'avifdecodetest', 'avifimagetest', 'avifsize0test']
REGRESSION = ['avif16bittest', 'avifaltrtest', 'avifsampletransformtest']


def run(command):
    subprocess.run([str(value) for value in command], cwd=ROOT, check=True)


def inventory(build):
    # Even --show-only rewrites CTest's LastTest.log. Query copies of the exact
    # generated registrations, leaving existing actual execution logs intact.
    with tempfile.TemporaryDirectory(prefix='libavif-root-inventory-') as directory:
        query = Path(directory)
        for source in build.rglob('CTestTestfile.cmake'):
            destination = query / source.relative_to(build)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
        record = json.loads(subprocess.check_output(['ctest', '--test-dir', str(query), '--show-only=json-v1'], text=True))
    names = [test['name'] for test in record['tests']]
    if not names or len(names) != len(set(names)):
        raise SystemExit('Missing or duplicate original CTest registrations')
    return names


SANITIZER_FLAGS = {'address': ['-fsanitize=address', '-fno-omit-frame-pointer']}


def library_commands(build, sanitizer=None, units=28):
    """Return each original libavif unit's flags with build-local paths normalized."""
    entries = [entry for entry in json.loads((build / 'compile_commands.json').read_text())
               if 'avif_obj.dir' in entry['command']]
    commands = {}
    for entry in entries:
        flags = [flag.replace(str(build), '<build>') for flag in shlex.split(entry['command'])[1:]]
        if sanitizer:
            # The sanitized reference appends exactly its instrumentation flags
            # to the original C flags; no other preprocessing option may change.
            for flag in SANITIZER_FLAGS[sanitizer]:
                if flags.count(flag) != 1:
                    raise SystemExit(f'The original C unit lacks its exact sanitizer instrumentation: {entry["file"]}')
                flags.remove(flag)
        commands[entry['file']] = flags
    if len(commands) != units or len(entries) != units:
        raise SystemExit(f'The original C reference lacks the complete {units}-unit libavif closure')
    return commands


def symbols(path):
    # Some archive members legitimately define no symbols; inspect the listing.
    result = subprocess.run(['nm', '-A', str(path)], text=True, capture_output=True)
    if not result.stdout:
        raise SystemExit(f'Could not read the symbol table: {path}')
    return result.stdout


def instrumented(path, members=None):
    """Require ASan instrumentation in every selected archive member."""
    listing = symbols(path)
    names = set(re.findall(r'^[^\n]*?\.a:([^:\n]+\.o):', listing, re.M))
    selected = {name for name in names if members is None or name.startswith(members)}
    covered = set(re.findall(r'^[^\n]*?\.a:([^:\n]+\.o):\s+[Uu] _{2,3}asan_init$', listing, re.M))
    return bool(selected) and selected <= covered


def runtime_linked(path):
    """Require the selected runtime itself in an original consumer executable."""
    return re.search(r'\s[Tt] _{2,3}asan_init$', symbols(path), re.M) is not None


def consumer_executables(build):
    names = re.findall(r'^build ([^ :]+): C(?:XX)?_EXECUTABLE_LINKER', (build / 'build.ninja').read_text(), re.M)
    if not names:
        raise SystemExit('No original root consumer targets found')
    return [build / name for name in names]


def committed(path):
    """Report whether a regenerated record equals the checked-out commit."""
    relative = str(Path(path).resolve().relative_to(ROOT))
    result = subprocess.run(['git', 'show', f'HEAD:{relative}'], cwd=ROOT, capture_output=True)
    return result.returncode == 0 and result.stdout == Path(path).read_bytes()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--port', type=Path, required=True)
    parser.add_argument('--root-profile', type=Path, required=True)
    parser.add_argument('--aom-reference-build', type=Path, help='Read an existing matching original AOM archive without rebuilding it')
    parser.add_argument('--tier', choices=['smoke', 'regression', 'full'], required=True)
    parser.add_argument('--jobs', type=int, default=2)
    parser.add_argument('--sanitizer', choices=sorted(SANITIZER_FLAGS),
                        help='Compare separately instrumented original C and Rust consumers')
    parser.add_argument('--production', action='store_true',
                        help='Validate consumers of the production package that compiled this native profile')
    parser.add_argument('--reference-baseline', type=Path,
                        help='The ordinary original C build that generated the profile; required with --sanitizer')
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error('--jobs must be positive')
    if bool(args.sanitizer) != bool(args.reference_baseline):
        parser.error('--sanitizer and --reference-baseline must be used together')
    if args.sanitizer and args.tier != 'full':
        parser.error('Sanitizer parity requires the complete original suite')
    if args.sanitizer and args.production:
        parser.error('Production sanitizer parity is not yet part of the admission evidence')
    baseline, port = args.baseline.resolve(), args.port.resolve()
    manifest = args.root_profile.resolve()
    record = json.loads(manifest.read_text())
    validate_profile(manifest, record['generation_target'])
    if args.sanitizer and record['preprocessing_profile'] != 'release':
        parser.error('The sanitizer configuration uses original Release preprocessing')
    generating = args.reference_baseline.resolve() if args.sanitizer else baseline
    if digest(generating / 'compile_commands.json') != record['original_compile_commands_sha256']:
        raise SystemExit('The original root reference differs from the generated profile compilation database')
    if digest(generating / 'CMakeCache.txt') != record['original_cmake_cache_sha256']:
        raise SystemExit('The original root reference configuration differs from the generated profile')
    sanitizer_record = None
    if args.sanitizer:
        units = record['original_translation_units']
        if library_commands(baseline, args.sanitizer, units) != library_commands(generating, units=units):
            raise SystemExit('The sanitized original C units differ from the profile\'s original preprocessing')
        from aom_sanitizer import native_runtime
        _, runtime, _ = native_runtime()
        aom_reference = ROOT / f'build/aom-root-asan-reference-{record["generation_target"]}'
        aom_record = json.loads((aom_reference / 'build.json').read_text())
        if (aom_record['original_translation_units'] != 166 or aom_record['sanitizer'] != args.sanitizer
                or aom_record['original_sha256'] != digest(aom_reference / 'libaom.a')
                or aom_record['runtime_sha256'] != digest(runtime)):
            raise SystemExit('The sanitized C baseline lacks the complete instrumented original AOM closure')
        cache = (baseline / 'CMakeCache.txt').read_text()
        if f'LIBAVIF_RS_AOM_REFERENCE_BUILD:UNINITIALIZED={aom_reference}\n' not in cache:
            raise SystemExit('The sanitized C baseline is not linked to the instrumented original AOM closure')
        sanitizer_record = dict(sanitizer=args.sanitizer, runtime=str(runtime), runtime_sha256=digest(runtime),
                                original_aom_reference_sha256=aom_record['original_sha256'],
                                original_aom_translation_units=166,
                                original_c_flags_match_profile_preprocessing=True)
    full = inventory(baseline)
    if full != inventory(port):
        raise SystemExit('The original C/Rust CTest registration inventories differ')
    selected = REGRESSION if args.tier == 'regression' else SMOKE
    expected = full if args.tier == 'full' else selected
    if not set(expected).issubset(full):
        raise SystemExit('The original native reference is missing required smoke registrations')
    production = None
    if args.production:
        from root_production import verify_build
        production = json.loads((port / 'production-package.json').read_text())
        verify_build(production, manifest, record['generation_target'])
        proof = production
    else:
        proof = json.loads((profile_work(record, args.sanitizer) / 'summary.json').read_text())
        if proof['translation_manifest_sha256'] != digest(manifest) or proof.get('sanitizer') != args.sanitizer:
            raise SystemExit('The isolated native root build proof differs from the current translation')
        for item in proof['artifacts'].values():
            if digest(Path(item['path'])) != item['sha256']:
                raise SystemExit('The isolated native root artifact changed after its build')
    parent = port / 'root-validation'
    parent.mkdir(exist_ok=True)
    output = Path(tempfile.mkdtemp(prefix=args.tier + '-', dir=parent))
    summary = output / 'summary.json'
    feature = [] if record['variant'] == 'default' else ['--features', root_variants.cargo_features(record['variant'])]
    if not args.sanitizer:
        run(['python3', 'tools/check_abi.py', '--build', baseline,
             '--production-profile' if production else '--root-profile', manifest, *feature])
    archive = Path(proof['artifacts']['libavif.a']['path'])
    if args.sanitizer:
        c_archive = baseline / 'libavif.a'
        # Every workspace codegen unit must be instrumented; the staticlib also
        # contains the precompiled standard library and allocator shim.
        if (not instrumented(archive, tuple(f'{name}.{name}.' for name in ['avif', 'aom', 'dav1d', 'sharpyuv'])) or not instrumented(c_archive)
                or not instrumented(aom_reference / 'libaom.a')):
            raise SystemExit('A compared library lacks AddressSanitizer instrumentation')
        executables = {name: consumer_executables(build) for name, build in [('c', baseline), ('rust', port)]}
        if [path.relative_to(baseline) for path in executables['c']] != [path.relative_to(port) for path in executables['rust']]:
            raise SystemExit('The sanitized original C/Rust consumer targets differ')
        for path in executables['c'] + executables['rust']:
            if not runtime_linked(path):
                raise SystemExit(f'An original consumer does not contain the selected ASan runtime: {path}')
        sanitizer_record['instrumented_consumers_each'] = len(executables['c'])
    run(['python3', 'tools/check_aom_root_linkage.py', '--archive', archive, '--consumer-build', port,
         '--reference-build', args.aom_reference_build or ROOT / f'build/aom-native-{record["preprocessing_profile"]}',
         *(['--debug'] if record['preprocessing_profile'] == 'debug' else [])])
    executions = {}
    for implementation, build in [('c', baseline), ('rust', port)]:
        executions[implementation] = output / implementation
        run(['python3', 'tools/run_tests.py', build, '--jobs', args.jobs,
             '--evidence-dir', executions[implementation],
             *(['--tests-regex', '^(' + '|'.join(map(re.escape, selected)) + ')$'] if args.tier != 'full' else [])])
        actual = results(executions[implementation] / 'results.xml')
        if set(actual) != set(expected):
            raise SystemExit('Actual execution does not cover the selected original CTest inventory')
    run(['python3', 'tools/compare_results.py', '--baseline', executions['c'] / 'results.xml',
         '--port', executions['rust'] / 'results.xml', '--report', output / 'parity.json',
         '--baseline-log', executions['c'] / 'LastTest.log', '--port-log', executions['rust'] / 'LastTest.log'])
    # Exact bytes and the shared ABI smoke use the ordinary references; the
    # sanitized comparison executes the complete original suites only.
    for decoder in ([] if args.tier == 'regression' or args.sanitizer else ['default', 'aom']):
        run(['python3', 'tools/differential.py', '--baseline', baseline, '--port', port,
             '--output-dir', output / f'bytes-{decoder}', *(['--decoder-codec', decoder] if decoder != 'default' else [])])
    shared = next(item['path'] for name, item in proof['artifacts'].items() if name.endswith(('.so', '.dylib')))
    if args.tier != 'regression' and not args.sanitizer:
        run(['python3', 'tools/smoke_shared.py', '--library', shared])
    run(['python3', 'tools/verify_upstream.py', '--tracked'])
    if production:
        # Every test above used the recorded production artifacts unchanged.
        verify_build(production, manifest, record['generation_target'])
    snapshots = {manifest: output / 'configuration/translation.json'}
    if args.sanitizer:
        snapshots[aom_reference / 'build.json'] = output / 'configuration/aom-asan-reference.json'
        for name in ['CMakeCache.txt', 'compile_commands.json']:
            snapshots[generating / name] = output / 'configuration/reference' / name
    else:
        snapshots[ROOT / (f'build/abi-{"production-" if production else ""}{record["generation_target"]}-'
                          f'{record["preprocessing_profile"]}-{record["variant"]}/summary.json')] = output / 'configuration/abi.json'
    if production:
        snapshots[port / 'production-package.json'] = output / 'configuration/production-package.json'
    for implementation, build in [('c', baseline), ('rust', port)]:
        for name in ['CMakeCache.txt', 'compile_commands.json']:
            snapshots[build / name] = output / 'configuration' / implementation / name
    snapshots[port / 'aom-root-linkage.json'] = output / 'configuration/linkage.json'
    for source, destination in snapshots.items():
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
    evidence = [path for path in output.rglob('*') if path.is_file()]
    summary.write_text(json.dumps(dict(target=record['generation_target'], profile=record['preprocessing_profile'],
        variant=record['variant'], tier=args.tier, complete_original_root_suite=args.tier == 'full',
        **({'sanitizer_parity': sanitizer_record} if sanitizer_record else {}),
        all_features_acceptance_claimed=False, production_target_admission_claimed=False,
        **({'production_package': dict(package='libavif-rs', rust_profile=production['rust_profile'],
                                       debug_assertions=production['debug_assertions'],
                                       compiled_profile=production['root_profile'],
                                       committed_profile_manifest_reproduced=committed(manifest),
                                       committed_profile_sources_reproduced=all(
                                           committed(ROOT / name) for name in
                                           [unit['rust'] for unit in record['units']] + [record['selector']]))}
           if production else {}),
        original_ctest_registrations=len(full), executed_ctest_registrations=len(expected),
        configuration_origins={str(destination.relative_to(output)): str(source.relative_to(ROOT))
                               for source, destination in snapshots.items()},
        artifacts=proof['artifacts'], evidence={str(path.relative_to(ROOT)): digest(path) for path in evidence}), indent=2) + '\n')
    print(f'Native root {"production " if production else ""}{args.tier}{" " + args.sanitizer if args.sanitizer else ""} validation passed; {len(expected)}/{len(full)} original registrations; {summary}')


if __name__ == '__main__':
    main()
