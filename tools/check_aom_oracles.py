#!/usr/bin/env python3
"""Preserve original assertions with separate C oracles and Rust subjects."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import xml.etree.ElementTree as ET

from aom_reference import PREFIX, reference_archive
from build_aom import library_entries, test_names
from check_abi import compiler_flags
from check_aom_linkage import consumer_owners
from dav1d_storage import tokens

ROOT = Path(__file__).resolve().parents[1]
SOURCE = 'test/av1_convolve_test.cc'
# Reviewed oracle calls only. Parameter initializers and indirect subjects keep
# their original names. Include disabled speed-call sites without enabling them.
SITES = {
    'av1_convolve_x_sr_c': [361, 388],
    'av1_highbd_convolve_x_sr_c': [577, 604],
    'av1_convolve_y_sr_c': [794, 815],
    'av1_highbd_convolve_y_sr_c': [991, 1012],
    'aom_convolve_copy_c': [1166],
    'aom_highbd_convolve_copy_c': [1212],
}
FILTER = ':'.join(f'C/AV1Convolve{name}Test.RunTest/*' for name in
                  ['X', 'XHighbd', 'Y', 'YHighbd', 'Copy', 'CopyHighbd'])
SUITES = {
    'convolution': dict(source=SOURCE, sites=SITES, subjects=list(SITES), filter=FILTER, cases=243,
                        headers=['config/av1_rtcd.h', 'config/aom_dsp_rtcd.h']),
    # RunSoftmaxTest's reference stays C. AnchorSoftmax16Fn and the fast
    # parameter remain Rust subjects, including the original anchor case.
    'softmax': dict(source='test/av1_softmax_test.cc', sites={'av1_nn_softmax': [64]},
                    subjects=['av1_nn_softmax', 'av1_nn_fast_softmax_16_c'],
                    filter='C/FastSoftmaxTest.RandomValues/*', cases=2,
                    headers=['av1/encoder/ml.h']),
}
INTRABC_SITES = {
    'av1_convolve_x_sr_intrabc_c': [467, 495],
    'av1_highbd_convolve_x_sr_intrabc_c': [681, 710],
    'av1_convolve_y_sr_intrabc_c': [890, 913],
    'av1_highbd_convolve_y_sr_intrabc_c': [1084, 1109],
    'av1_convolve_2d_sr_intrabc_c': [1514, 1544],
    'av1_highbd_convolve_2d_sr_intrabc_c': [1801, 1835],
}
SUITES['intrabc'] = dict(source=SOURCE, sites=INTRABC_SITES, subjects=list(INTRABC_SITES),
    filter='C/AV1Convolve*IntraBCTest.RunTest/*', cases=243,
    headers=['config/av1_rtcd.h', 'config/aom_dsp_rtcd.h'])
CONVOLUTION_2D_SITES = {
    'av1_convolve_2d_sr_c': [1313, 1348, 1405],
    'av1_highbd_convolve_2d_sr_c': [1643, 1671, 1715],
}
SUITES['convolution-2d'] = dict(source=SOURCE, sites=CONVOLUTION_2D_SITES,
    subjects=list(CONVOLUTION_2D_SITES), cases=162,
    filter='C/AV1Convolve2DTest.RunTest/*:C/AV1Convolve2DTest.VmafPreprocessing/*:'
           'C/AV1Convolve2DHighbdTest.RunTest/*:C/AV1Convolve2DHighbdTest.VmafPreprocessing/*',
    headers=['config/av1_rtcd.h', 'config/aom_dsp_rtcd.h'])
# ReferenceFunc returns and Convolve's first argument select C oracles;
# registration initializers continue to select the original Rust subjects.
COMPOUND_SITES = {
    'av1_dist_wtd_convolve_x_c': [2033],
    'av1_highbd_dist_wtd_convolve_x_c': [2141],
    'av1_dist_wtd_convolve_y_c': [2238],
    'av1_highbd_dist_wtd_convolve_y_c': [2287],
    'av1_dist_wtd_convolve_2d_copy_c': [2365, 2396, 2398],
    'av1_highbd_dist_wtd_convolve_2d_copy_c': [2489],
    'av1_dist_wtd_convolve_2d_c': [2589],
    'av1_highbd_dist_wtd_convolve_2d_c': [2700],
}
SUITES['compound'] = dict(source=SOURCE, sites=COMPOUND_SITES,
    subjects=list(COMPOUND_SITES), filter='C/AV1Convolve*CompoundTest.RunTest/*', cases=264,
    headers=['config/av1_rtcd.h', 'config/aom_dsp_rtcd.h'], identifier_oracles=True)
SUITES['kmeans'] = dict(source='test/av1_k_means_test.cc',
    sites={'av1_calc_indices_dim1_c': [94, 109], 'av1_calc_indices_dim2_c': [200, 215]},
    subjects=['av1_calc_indices_dim1_c', 'av1_calc_indices_dim2_c'],
    filter='PortScalar/AV1KmeansTest*.CheckOutput/*', cases=28,
    headers=['config/av1_rtcd.h'], supplementary=True, identifier_oracles=True,
    added_registrations=56, added_disabled=28)
SUITES['nn-predict'] = dict(source='test/av1_nn_predict_test.cc',
    sites={'av1_nn_predict_c': [118, 155]}, subjects=['av1_nn_predict_c'],
    filter='PortScalar/NnPredictTest.RandomValues/*', cases=1,
    headers=['config/av1_rtcd.h'], supplementary=True,
    added_registrations=2, added_disabled=1)
DR_PREDICTION_SUBJECTS = [f'av1_{depth}dr_prediction_z{zone}_c'
                          for depth in ['', 'highbd_'] for zone in [1, 2, 3]]
SUITES['directional-prediction'] = dict(source='test/dr_prediction_test.cc',
    sites={name: [] for name in DR_PREDICTION_SUBJECTS}, subjects=DR_PREDICTION_SUBJECTS,
    filter='PortScalar/*DrPredTest.SaturatedValues/*:PortScalar/*DrPredTest.OperationCheck/*', cases=24,
    headers=['config/av1_rtcd.h', 'av1/common/reconintra.h'], supplementary=True,
    added_registrations=36, added_disabled=12)
SUITES['round-shift-array'] = dict(source='test/av1_round_shift_array_test.cc',
    sites={'av1_round_shift_array_c': [70, 89]}, subjects=['av1_round_shift_array_c'],
    filter='PortScalar/AV1CompRoundShiftTest.CheckOutput/*', cases=171,
    headers=['config/av1_rtcd.h'], supplementary=True, identifier_oracles=True,
    added_registrations=342, added_disabled=171)
SUITES['cubic-rd'] = dict(source='test/model_rd_test.cc',
    sites={'av1_interp_cubic_rate_dist_c': [70, 95]}, subjects=['av1_interp_cubic_rate_dist_c'],
    filter='PortScalar/InterpCubicTest.CheckOutput/*', cases=1,
    headers=['config/av1_rtcd.h'], supplementary=True,
    added_registrations=2, added_disabled=1)
SUITES['optical-flow'] = dict(source='test/disflow_test.cc',
    sites={'aom_compute_flow_at_point_c': [77, 85]}, subjects=['aom_compute_flow_at_point_c'],
    filter='PortScalar/ComputeFlowTest.CheckOutput/*', cases=1,
    headers=['config/aom_dsp_rtcd.h'], supplementary=True,
    added_registrations=2, added_disabled=1, fixtures=['bus_352x288_420_f20_b8.yuv'])
SUITES['scaled-convolution'] = dict(source='test/convolve_test.cc',
    sites={'aom_scaled_2d_c': [1117]}, subjects=['aom_scaled_2d_c'],
    filter='C/ConvolveScale2DTest.Correctness/*', cases=13,
    headers=['config/aom_dsp_rtcd.h'])
SUITES['blend-mask-1d'] = dict(source='test/blend_a64_mask_1d_test.cc',
    sites={'aom_blend_a64_mask_c': [177, 192], 'aom_highbd_blend_a64_mask_c': [296, 311]},
    subjects=[f'aom_{depth}blend_a64_{direction}mask_c'
              for depth in ['', 'highbd_'] for direction in ['h', 'v']],
    filter='C/BlendA64Mask1DTest8B.RandomValues/*:C/BlendA64Mask1DTest8B.ExtremeValues/*:'
           'C/BlendA64Mask1DTestHBD.RandomValues/*:C/BlendA64Mask1DTestHBD.ExtremeValues/*',
    cases=8, headers=['config/aom_dsp_rtcd.h'])
SUITES['warp-affine'] = dict(source='test/warp_filter_test_util.cc',
    sites={'av1_warp_affine_c': [265]}, subjects=['av1_warp_affine_c'],
    subject_source='test/warp_filter_test.cc',
    filter='C/AV1WarpFilterTest.CheckOutput/*', cases=80,
    headers=['config/av1_rtcd.h'])


def supplementary_registrations(original, suite, reference_prefix=''):
    """Reuse original parameter matrices and bodies for a separate scalar comparison."""
    if suite == 'optical-flow':
        return ('\nnamespace {\nINSTANTIATE_TEST_SUITE_P(PortScalar, ComputeFlowTest,\n'
                '  ::testing::Values(aom_compute_flow_at_point_c));\n}\n')
    if suite == 'round-shift-array':
        bits = re.search(r'const int kValidBitCheck\[\] = \{(.*?)\};', original, re.S)
        if not bits or list(map(int, re.findall(r'-?\d+', bits[1]))) != list(range(-4, 5)):
            raise SystemExit('Original round-shift bit matrix changed')
        return ('\nnamespace AV1CompRoundShift {\nconst int kPortScalarBitCheck[] = {' + bits[1] + '};\n'
                'INSTANTIATE_TEST_SUITE_P(PortScalar, AV1CompRoundShiftTest,\n'
                '  ::testing::Combine(::testing::Values(&av1_round_shift_array_c),\n'
                '    ::testing::ValuesIn(txsize_to_bsize), ::testing::ValuesIn(kPortScalarBitCheck)));\n}\n')
    if suite == 'cubic-rd':
        return ('\nnamespace {\nINSTANTIATE_TEST_SUITE_P(PortScalar, InterpCubicTest,\n'
                '  ::testing::Values(av1_interp_cubic_rate_dist_c));\n}\n')
    if suite == 'directional-prediction':
        result = '\nnamespace {\n'
        for name, expected in [('LowbdDrPredTest', 3), ('HighbdDrPredTest', 9)]:
            # The original C matrix leaves tst_fn null. Copy its exact angles,
            # bit depths, and wrappers, adding a separate scalar subject slot.
            match = re.search(r'INSTANTIATE_TEST_SUITE_P\(\s*C,\s*' + name + r',.*?\);', original, re.S)
            if not match:
                raise SystemExit(f'Original directional prediction matrix changed: {name}')
            registration = re.sub(r'(INSTANTIATE_TEST_SUITE_P\(\s*)C,', r'\1PortScalar,', match[0], count=1)
            def bind(match):
                wrapper, function = match.groups()
                return f'&{wrapper}<{reference_prefix}{function}>, &{wrapper}<{function}>'
            registration, count = re.subn(
                r'&(z[123]_wrapper(?:_hbd)?)<(av1_(?:highbd_)?dr_prediction_z[123]_c)>,\s*nullptr',
                bind, registration)
            if count != expected:
                raise SystemExit(f'Original directional prediction parameters changed: {name}: {count}')
            result += registration + '\n'
        return result + '}\n'
    if suite == 'nn-predict':
        return ('\nnamespace {\nINSTANTIATE_TEST_SUITE_P(PortScalar, NnPredictTest,\n'
                '  ::testing::Values(av1_nn_predict_c));\n}\n')
    if suite != 'kmeans':
        raise SystemExit(f'Unreviewed supplementary scalar matrix: {suite}')
    matrix = re.search(r'const BLOCK_SIZE kValidBlockSize\[\] = \{(.*?)\};', original, re.S)
    if not matrix or len(re.findall(r'\bBLOCK_\w+', matrix[1])) != 14:
        raise SystemExit('Original k-means block-size matrix changed')
    result = '\nnamespace AV1Kmeans {\nconst BLOCK_SIZE kPortScalarBlockSize[] = {' + matrix[1] + '};\n'
    for dimension in [1, 2]:
        result += (f'INSTANTIATE_TEST_SUITE_P(PortScalar, AV1KmeansTest{dimension},\n'
                   f'  ::testing::Combine(::testing::Values(&av1_calc_indices_dim{dimension}_c),\n'
                   '    ::testing::ValuesIn(kPortScalarBlockSize)));\n')
    return result + '}\n'


def compilation_copy(text, source_name, augmentation):
    # A UTF-8 signature must remain at byte zero when inserting diagnostics.
    signature = '\ufeff' if text.startswith('\ufeff') else ''
    return signature + f'#line 1 "{source_name}"\n' + text[len(signature):] + augmentation


def audit_map(path, symbols, rust_archive, oracle_archive, subjects, oracles):
    loaded, oracle_loaded = set(), set()
    owners, _ = consumer_owners(path)
    for name, owner in owners.items():
        if name in symbols:
            if not owner.startswith(str(rust_archive) + '(aom.') or not owner.endswith('.rcgu.o)'):
                raise SystemExit(f'Original subject global supplied outside Rust: {name}: {owner}')
            loaded.add(name)
        if name.startswith(PREFIX) and name[len(PREFIX):] in symbols:
            if not owner.startswith(str(oracle_archive) + '('):
                raise SystemExit(f'Oracle global supplied outside isolated C: {name}: {owner}')
            oracle_loaded.add(name[len(PREFIX):])
    if not set(subjects) <= loaded or not set(oracles) <= oracle_loaded:
        raise SystemExit('The original subject and independent C oracle are not both loaded')
    return dict(rust_original_globals=len(loaded), isolated_c_oracle_globals=len(oracle_loaded),
                reviewed_rust_subjects=sorted(subjects), reviewed_c_oracles=sorted(oracles),
                map_sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--suite', choices=SUITES, default='convolution')
    parser.add_argument('--reference-build', type=Path, help='Read an existing native reference without modifying it')
    parser.add_argument('--rust-archive', type=Path, help='Read an existing candidate Rust archive')
    parser.add_argument('--fixtures', type=Path, help='Read an existing original fixture directory')
    args = parser.parse_args()
    suite = SUITES[args.suite]
    source_name, oracle_sites = suite['source'], suite['sites']
    profile = 'debug' if args.debug else 'release'
    build = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    suffix = '' if args.suite == 'convolution' else '-' + args.suite
    work = ROOT / f'build/aom-oracles-{profile}{suffix}'
    work.mkdir(parents=True, exist_ok=True)
    (work / 'summary.json').unlink(missing_ok=True)
    archive = (args.rust_archive or ROOT / f'target/{profile}/libaom.a').resolve()
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest = ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json'
    translation = json.loads(manifest.read_text())
    reference = json.loads((build / 'reference-profile.json').read_text())
    if reference['profile'] != profile or translation['preprocessing_profile'] != profile:
        raise SystemExit('The native reference and Rust profile differ')
    for name, expected in translation['generated_reference_headers'].items():
        if hashlib.sha256((build / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f'The native reference and Rust preprocessing differ: {name}')
    fixtures = (args.fixtures or ROOT / 'build/deps/aom-test-data').resolve()
    fixture_records = {}
    if suite.get('fixtures'):
        selected = json.loads((build / 'test-data.json').read_text())
        upstream = dict(line.split(' *')[::-1] for line in
                        (ROOT / 'upstream/aom/test/test-data.sha1').read_text().splitlines())
        for name in suite['fixtures']:
            if Path(name).name != name or name not in selected or selected[name] != upstream.get(name):
                raise SystemExit(f'Fixture is not in the original selected manifest: {name}')
            path = fixtures / name
            if not path.is_file():
                raise SystemExit(f'Missing original fixture: {path}')
            content = path.read_bytes()
            if hashlib.sha1(content).hexdigest() != selected[name]:
                raise SystemExit(f'Original fixture checksum mismatch: {name}')
            fixture_records[name] = dict(sha1=selected[name], size=len(content),
                                         sha256=hashlib.sha256(content).hexdigest())
    oracle, symbols, oracle_record = reference_archive(profile, build)
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry = next(value for value in entries if Path(value['file']).as_posix().endswith('/' + source_name))
    source = Path(entry['file'])
    source_root = source.parents[1]
    source_hash = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files'][source_name]
    if hashlib.sha256(source.read_bytes()).hexdigest() != source_hash:
        raise SystemExit(f'Modified original test source: {source_name}')
    original = source.read_text()
    augmentation = supplementary_registrations(original, args.suite, PREFIX) if suite.get('supplementary') else ''
    baseline_augmentation = supplementary_registrations(original, args.suite) if augmentation else ''
    lexemes, replacements, sites = tokens(original), [], {name: [] for name in oracle_sites}
    for index, token in enumerate(lexemes[:-1]):
        line = original.count('\n', 0, token.start()) + 1 if token[0] in oracle_sites else 0
        if (token[0] in oracle_sites and line in oracle_sites[token[0]]
                and (lexemes[index + 1][0] == '(' or suite.get('identifier_oracles'))):
            sites[token[0]].append(line)
            replacements.append((token.start(), token.end(), PREFIX + token[0]))
    if sites != oracle_sites:
        raise SystemExit(f'Original oracle call sites changed: {sites}')
    text = original
    for start, end, name in reversed(replacements):
        text = text[:start] + name + text[end:]
    include = work / 'oracles.h'
    include.write_text(''.join(f'#include "{name}"\n' for name in suite['headers']) + 'extern "C" {\n'
                       + ''.join(f'decltype({name}) {PREFIX}{name};\n' for name in oracle_sites) + '}\n')
    copied = work / source.name
    copied.write_text(compilation_copy(text, source_name, augmentation))
    output = work / (source.name + '.o')
    flags = compiler_flags(entry)
    if any(flag in ['-MD', '-MMD', '-MP', '-MG'] or flag.startswith(('-MF', '-MT', '-MQ', '-MJ'))
           for flag in flags):
        raise SystemExit('The read-only reference flags request dependency output files')
    subprocess.run([*flags, '-include', include, '-c', copied, '-o', output],
                   cwd=entry['directory'], check=True)
    undefined = {line.split()[-1] for line in subprocess.check_output(['nm', '-u', output], text=True).splitlines() if line.split()}
    subject_binding = None
    if suite.get('subject_source'):
        # The original parameter matrix may live in a separate unchanged C++
        # object from its reference body. Require its actual external subject
        # reference instead of inferring that reference from the link alone.
        subject_entry = next(value for value in entries
                             if Path(value['file']).as_posix().endswith('/' + suite['subject_source']))
        subject_source = Path(subject_entry['file'])
        expected_hash = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files'][suite['subject_source']]
        if hashlib.sha256(subject_source.read_bytes()).hexdigest() != expected_hash:
            raise SystemExit('Modified original indirect subject registration source')
        subject_object = Path(subject_entry['directory']) / subject_entry['output']
        subject_undefined = {line.split()[-1] for line in
                            subprocess.check_output(['nm', '-u', subject_object], text=True).splitlines() if line.split()}
        decoration = '_' if sys.platform == 'darwin' else ''
        if not {decoration + name for name in suite['subjects']} <= subject_undefined:
            raise SystemExit('Original separate parameter matrix does not reference its external Rust subjects')
        undefined.update(subject_undefined)
        subject_binding = dict(source=suite['subject_source'], source_sha256=expected_hash,
                               original_object_sha256=hashlib.sha256(subject_object.read_bytes()).hexdigest(),
                               unchanged_original_registration_object=True)
    decoration = '_' if sys.platform == 'darwin' else ''
    expected = {decoration + name for name in suite['subjects']}
    expected.update(decoration + PREFIX + name for name in oracle_sites)
    if not expected <= undefined:
        raise SystemExit('Original parameterized subjects and isolated C oracles must both remain external')
    commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'test_libaom'], text=True)
    command = shlex.split(commands.splitlines()[-1])
    if command[:2] == [':', '&&'] and command[-2:] == ['&&', ':']:
        command = command[2:-2]
    if any(token in [':', '&&', '||', ';'] for token in command):
        raise SystemExit('Unmapped original test link command')
    library_sources = {Path(value['file']) for value in library_entries(build)}
    objects = {str((Path(value['directory']) / value['output']).relative_to(build)): Path(value['file']) for value in entries}
    for token in list(command):
        if objects.get(token) in library_sources:
            if objects[token].relative_to(source_root).as_posix() not in ['common/args_helper.c', 'av1/arg_defs.c']:
                raise SystemExit(f'Unreviewed original test/codec overlap: {token}')
            command.remove(token)
    original_link = list(command)
    old = str((Path(entry['directory']) / entry['output']).relative_to(build))
    if command.count(old) != 1 or command.count('libaom.a') != 1:
        raise SystemExit('Expected one original test object and one codec archive')
    command[command.index(old)] = str(output)
    index = command.index('libaom.a')
    command[index:index + 1] = [str(archive), str(oracle)]
    executable = work / 'test_libaom'
    command[command.index('-o') + 1] = str(executable)
    link_map = work / 'test_libaom.map'
    command.append(f'-Wl,-map,{link_map}' if sys.platform == 'darwin' else f'-Wl,-Map,{link_map},--cref')
    subprocess.run(command, cwd=build, check=True)
    ownership = audit_map(link_map, symbols, archive, oracle, suite['subjects'], oracle_sites)
    names = test_names((build / 'test_libaom-inventory.txt').read_text())
    registered = test_names(subprocess.check_output([executable, '--gtest_list_tests'], text=True))
    added = [name for name in registered if name.startswith('PortScalar/')] if augmentation else []
    if [name for name in registered if name not in added] != names:
        raise SystemExit('Independent oracle binding changed original registrations, order, or parameters')
    if augmentation and (len(added) != suite['added_registrations']
                         or sum('DISABLED_' in name for name in added) != suite['added_disabled']):
        raise SystemExit('Supplementary scalar registrations do not match the original matrix/disabled flags')
    baseline = build / 'test_libaom'
    if augmentation:
        baseline_source = work / ('c-' + source.name)
        baseline_source.write_text(compilation_copy(original, source_name, baseline_augmentation))
        baseline_object = work / ('c-' + source.name + '.o')
        subprocess.run([*flags, '-c', baseline_source, '-o', baseline_object],
                       cwd=entry['directory'], check=True)
        baseline = work / 'test_libaom-c'
        original_link[original_link.index(old)] = str(baseline_object)
        original_link[original_link.index('-o') + 1] = str(baseline)
        subprocess.run(original_link, cwd=build, check=True)
        if test_names(subprocess.check_output([baseline, '--gtest_list_tests'], text=True)) != registered:
            raise SystemExit('Supplementary C/Rust registrations differ')
    results = []
    environment = dict(os.environ, LIBAOM_TEST_DATA_PATH=str(fixtures))
    for label, binary in [('c', baseline), ('rust', executable)]:
        xml = work / f'{label}.xml'
        xml.unlink(missing_ok=True)
        with (work / f'{label}.log').open('w') as log:
            subprocess.run([binary, f'--gtest_filter={suite["filter"]}', f'--gtest_output=xml:{xml}'],
                           cwd=build, env=environment, stdout=log, stderr=subprocess.STDOUT, check=True)
        cases = [dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                      failed=bool(case.findall('failure')), skipped=bool(case.findall('skipped')))
                 for case in ET.parse(xml).getroot().iter('testcase') if case.attrib.get('status') == 'run']
        if any(case['failed'] or case['skipped'] for case in cases):
            raise SystemExit('An original independent-oracle assertion failed or skipped')
        results.append(cases)
    if results[0] != results[1] or len(results[1]) != suite['cases']:
        raise SystemExit('Original independent-oracle execution inventories differ')
    record = dict(profile=profile, original_registrations=len(names),
                  executed_original_cases=0 if augmentation else len(results[1]),
                  executed_supplementary_cases=len(results[1]) if augmentation else 0,
                  supplementary_registrations=added,
                  augmentation_sha256=hashlib.sha256(augmentation.encode()).hexdigest() if augmentation else None,
                  baseline_augmentation_sha256=hashlib.sha256(baseline_augmentation.encode()).hexdigest() if augmentation else None,
                  original_assertions_parameters_and_disabled_flags_unchanged=True,
                  original_source=source_name, original_source_sha256=source_hash, oracle_sites=oracle_sites,
                  binding_copy_sha256=hashlib.sha256(copied.read_bytes()).hexdigest(),
                  complete_dsp_role_review=False, reference_archive=oracle_record, ownership=ownership,
                  translation_manifest_sha256=hashlib.sha256(manifest.read_bytes()).hexdigest(),
                  rust_archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(), cases=results[1])
    if fixture_records:
        record['original_fixtures'] = fixture_records
    if subject_binding:
        record['separate_subject_binding'] = subject_binding
    (work / 'summary.json').write_text(json.dumps(record, indent=2) + '\n')
    origin = 'supplementary' if augmentation else 'original'
    print(f'{len(results[1])} {origin} {args.suite} cases compare {len(suite["subjects"])} Rust subjects with isolated C oracles')


if __name__ == '__main__':
    main()
