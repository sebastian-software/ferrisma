#!/usr/bin/env python3
"""Diagnose original skcms numeric configurations without changing active graphs.

Every baseline and candidate word is retained. A matching candidate is evidence
for review, never an automatically selected reference configuration or parity.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
import platform
from pathlib import Path
import shlex
import shutil
import subprocess
import sys


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def run(command, directory, log):
    with log.open('w') as stream:
        subprocess.run(command, cwd=directory, stdout=stream,
                       stderr=subprocess.STDOUT, check=True)


def observe(binary, payload, path):
    with path.open('wb') as output:
        result = subprocess.run([str(binary)], input=payload, stdout=output,
                                stderr=subprocess.PIPE)
    path.with_suffix('.stderr').write_bytes(result.stderr)
    assert result.returncode == 0 and not result.stderr, (result.returncode, str(path))
    rows = path.read_text().splitlines()
    assert len(rows) == len(payload.splitlines()), (binary, len(rows))
    return rows


def differences(requests, first, second, path):
    assert len(requests) == len(first) == len(second)
    rows = [{'index': index, 'request': request, 'baseline': a, 'candidate': b}
            for index, (request, a, b) in enumerate(zip(requests, first, second)) if a != b]
    with gzip.open(path, 'wt') as output:
        json.dump(rows, output, indent=2)
        output.write('\n')
    return len(rows)


def member_hashes(archive):
    members = subprocess.check_output(['ar', 't', str(archive)], text=True).splitlines()
    assert len(members) == len(set(members)), 'Duplicate source archive members'
    return {member: hashlib.sha256(subprocess.check_output(
            ['ar', 'p', str(archive), member])).hexdigest() for member in members}


def redirect_dependencies(original, command, directory, obj):
    """Keep copied driver dependency output entirely inside the candidate."""
    depfile, target = str(directory / 'skcms.cc.d'), str(obj)
    changes, has_depfile = [], False
    index = 0
    while index < len(original):
        arg = original[index]
        assert arg != '-dependency-file' and not arg.startswith('-MJ'), 'Unsupported extra compiler output: ' + arg
        assert not (arg.startswith('-Wp,') and any(x in arg for x in (',-MD', ',-MMD', ',-MF'))), 'Unsupported forwarded dependency output'
        for option in ('-MF', '-MT', '-MQ'):
            value = depfile if option == '-MF' else target
            if arg == option:
                assert index + 1 < len(original)
                command[index + 1] = value
                changes.append({'option': option, 'original': original[index + 1], 'candidate': value})
                has_depfile |= option == '-MF'
                index += 2
                break
            if arg.startswith(option) and len(arg) > len(option):
                command[index] = option + value
                changes.append({'option': option, 'original': arg, 'candidate': command[index]})
                has_depfile |= option == '-MF'
                index += 1
                break
        else:
            index += 1
    if any(arg in ('-MD', '-MMD') for arg in original) and not has_depfile:
        command.extend(['-MF', depfile])
        changes.append({'option': '-MF', 'original': None, 'candidate': depfile})
    return changes


def candidate(root, build, directory, options, bindings):
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry, = [item for item in entries if Path(item['file']).name == 'skcms.cc']
    expected_source = (root / 'upstream/libjxl/third_party/skcms/skcms.cc').resolve()
    assert Path(entry['file']).resolve() == expected_source
    original = entry.get('arguments') or shlex.split(entry['command'])
    command_source = Path(original[original.index('-c') + 1])
    if not command_source.is_absolute():
        command_source = Path(entry['directory']) / command_source
    assert command_source.resolve() == expected_source
    command = list(original)
    original_object = Path(command[command.index('-o') + 1])
    if not original_object.is_absolute():
        original_object = Path(entry['directory']) / original_object
    obj = directory / 'skcms.cc.o'
    command[command.index('-o') + 1] = str(obj)
    dependency_changes = redirect_dependencies(original, command, directory, obj)
    command.extend(options)
    command_changes = [{'index': i, 'original': original[i] if i < len(original) else None,
                        'candidate': command[i] if i < len(command) else None}
                       for i in range(max(len(original), len(command)))
                       if (original[i] if i < len(original) else None) != (command[i] if i < len(command) else None)]
    (directory / 'compiler-argv.json').write_text(json.dumps(
        {'original': original, 'candidate': command, 'changes': command_changes,
         'dependency_output_changes': dependency_changes}, indent=2) + '\n')
    assert [x for x in command if 'sanitize' in x] == [x for x in original if 'sanitize' in x]
    run(command, Path(entry['directory']), directory / 'compile.log')
    original_archive = build / 'upstream/lib/libjxl_cms.a'
    archive = directory / 'libjxl_cms.a'
    old_members = member_hashes(original_archive)
    assert list(old_members).count('skcms.cc.o') == 1
    shutil.copy2(original_archive, archive)
    run(['ar', 'rcs', str(archive), str(obj)], directory, directory / 'archive.log')
    new_members = member_hashes(archive)
    assert set(old_members) == set(new_members)
    assert all(old_members[k] == new_members[k] for k in old_members if k != 'skcms.cc.o')
    assert new_members['skcms.cc.o'] == sha(obj)
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    archive_indexes = [i for i, arg in enumerate(link) if Path(arg).name == 'libjxl_cms.a']
    assert len(archive_indexes) == 1
    link[archive_indexes[0]] = str(archive)
    binary, mapping = directory / 'bound', directory / 'link.map'
    link[link.index('-o') + 1] = str(binary)
    link.append('-Wl,-Map,' + str(mapping))
    run(link, build, directory / 'link.log')
    bound = bindings(mapping.read_text(), binary)
    with gzip.open(directory / 'source-object.asm.gz', 'wb') as assembly:
        assembly.write(subprocess.check_output(['objdump', '-d', '-C', str(obj)]))
    return binary, {'numeric_options': options, 'dependency_output_changes': dependency_changes,
                    'command_changes': command_changes, 'original_command': original,
                    'candidate_command': command, 'link': link, 'bindings': bound,
                    'original_archive_members': old_members, 'candidate_archive_members': new_members,
                    'original_object_sha256': sha(original_object), 'object_sha256': sha(obj),
                    'archive_sha256': sha(archive), 'binary_sha256': sha(binary),
                    'map_sha256': sha(mapping), 'source_assembly_sha256': sha(directory / 'source-object.asm.gz')}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--baseline-diagnostics', type=Path, required=True)
    parser.add_argument('--full-transfer', type=Path, required=True)
    parser.add_argument('--full-matrix', type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    assert platform.system() == 'Linux' and platform.machine() == 'x86_64', 'Native Linux x86_64 evidence only'
    sys.path.insert(0, str(root / 'tools'))
    from skcms_reference import configuration, bindings, retained_matrix_requests
    from skcms_matrix_diagnostics import paired_nan_product_requests
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    baseline = args.baseline_diagnostics.resolve()
    retained = [q for q in retained_matrix_requests() if q.startswith('skcms-concat ')]
    paired = paired_nan_product_requests()
    requests = retained + paired
    payload = ('\n'.join(requests) + '\n').encode()
    assert len(retained) == 24324 and len(paired) == 432
    assert payload == (baseline / 'requests.txt').read_bytes()
    (output / 'requests.txt').write_bytes(payload)
    pin = json.loads((root / 'upstream.json').read_text())
    sources = [root / pin['path']] + [root / pin['path'] / 'third_party' / n for n in ('highway', 'brotli', 'skcms')]
    pins = {}
    for source in sources:
        revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
        status = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--ignore-submodules=none'], text=True)
        wanted = pin['revision'] if source == sources[0] else subprocess.check_output(
                ['git', '-C', str(sources[0]), 'ls-tree', 'HEAD', 'third_party/' + source.name], text=True).split()[2]
        assert revision == wanted and not status, str(source)
        pins[str(source.relative_to(root))] = revision
    report = {'diagnostic_only': True, 'platform': platform.platform(), 'pins': pins,
              'requests': len(requests), 'graphs': {}, 'candidates': {}}
    transfer = args.full_transfer.read_text().splitlines()
    matrix = args.full_matrix.read_text().splitlines()
    assert Counter(q.split()[0] for q in transfer) == Counter({'skcms-tf': 89838, 'skcms-tf-in-place': 24310, 'skcms-make': 5120})
    assert len(matrix) in (87444, 87876)
    if len(matrix) == 87444:
        matrix += paired
    assert Counter(q.split()[0] for q in matrix) == Counter({'skcms-concat': 24756, 'skcms-invert': 40524, 'skcms-adapt': 10290, 'skcms-primaries': 12306})
    assert not (Counter(retained_matrix_requests()) - Counter(matrix))
    assert not (Counter(paired) - Counter(matrix))
    full_requests = transfer + matrix
    assert len(full_requests) == 207144
    full_payload = ('\n'.join(full_requests) + '\n').encode()
    (output / 'full-requests.txt').write_bytes(full_payload)
    report['full_requests'] = len(full_requests)
    baseline_rows, full_baseline_rows = {}, {}
    for graph in ['oracle', 'oracle-asan']:
        directory = output / 'baseline' / graph
        directory.mkdir(parents=True)
        build = root / 'target' / graph
        bound = configuration(build, directory)
        rows = observe(build / 'jxl_oracle', payload, directory / 'observations.txt')
        assert rows == (baseline / graph / 'observations.txt').read_text().splitlines()
        baseline_rows[graph] = rows
        report['graphs'][graph] = {'binding': bound, 'observations_sha256': sha(directory / 'observations.txt')}
        full_baseline_rows[graph] = observe(build / 'jxl_oracle', full_payload, directory / 'full-observations.txt')
        report['graphs'][graph]['full_observations_sha256'] = sha(directory / 'full-observations.txt')
    report['baseline_source_differences'] = differences(requests, baseline_rows['oracle'], baseline_rows['oracle-asan'], output / 'baseline-differences.json.gz')
    assert report['baseline_source_differences'] == 64
    assert baseline_rows['oracle'][:len(retained)] == baseline_rows['oracle-asan'][:len(retained)]
    expected_differences = json.loads((baseline / 'ordinary-instrumented-differences.json').read_text())
    actual_differences = [{'index': i, 'request': q, 'ordinary': a, 'instrumented': b}
                          for i, (q, a, b) in enumerate(zip(requests, baseline_rows['oracle'], baseline_rows['oracle-asan'])) if a != b]
    assert actual_differences == expected_differences and len(actual_differences) == 64
    assert all(row['index'] >= len(retained) for row in actual_differences)
    report['full_baseline_source_differences'] = differences(
        full_requests, full_baseline_rows['oracle'], full_baseline_rows['oracle-asan'],
        output / 'full-baseline-differences.json.gz')
    # Diagnose the least disruptive candidate first. Do not activate it even
    # when both source graphs agree; source configuration selection needs review.
    for name, options in [('o3-strict', ['-ffp-model=strict'])]:
        rows, full_rows = {}, {}
        item = {'graphs': {}}
        for graph in ['oracle', 'oracle-asan']:
            directory = output / name / graph
            directory.mkdir(parents=True)
            binary, proof = candidate(root, root / 'target' / graph, directory, options, bindings)
            rows[graph] = observe(binary, payload, directory / 'observations.txt')
            proof['baseline_word_changes'] = differences(requests, baseline_rows[graph], rows[graph], directory / 'baseline-changes.json.gz')
            proof['observations_sha256'] = sha(directory / 'observations.txt')
            full_rows[graph] = observe(binary, full_payload, directory / 'full-observations.txt')
            proof['full_baseline_word_changes'] = differences(full_requests, full_baseline_rows[graph], full_rows[graph], directory / 'full-baseline-changes.json.gz')
            proof['full_observations_sha256'] = sha(directory / 'full-observations.txt')
            item['graphs'][graph] = proof
        item['source_differences'] = differences(requests, rows['oracle'], rows['oracle-asan'], output / name / 'source-differences.json.gz')
        item['full_source_differences'] = differences(full_requests, full_rows['oracle'], full_rows['oracle-asan'], output / name / 'full-source-differences.json.gz')
        report['candidates'][name] = item
        for graph, data in report['graphs'].items():
            assert all(sha(root / relative) == digest for relative, digest in data['binding']['active_hashes'].items()), graph
        (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps({'candidate': name, 'source_differences': item['source_differences'],
                          'full_source_differences': item.get('full_source_differences'), 'diagnostic_only': True}), flush=True)
    assert all(not subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--ignore-submodules=none'], text=True) for source in sources)


if __name__ == '__main__':
    main()
