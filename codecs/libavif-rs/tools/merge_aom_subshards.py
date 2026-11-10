#!/usr/bin/env python3
"""Recover one original shard from complete finer C/Rust GoogleTest partitions."""
import argparse
import hashlib
import json
from pathlib import Path
import xml.etree.ElementTree as ET

from aom_subshards import merge_subshards, passing_subshards
from run_aom_paired_shard import disabled, inventory

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def xml_cases(path):
    document = ET.parse(path).getroot()
    if int(document.get('errors', 0)) or int(document.get('failures', 0)):
        raise ValueError('A finer original test XML contains failures or errors')
    return [dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                 result=case.get('result'), failed=bool(case.findall('failure')),
                 skipped=bool(case.findall('skipped')))
            for case in document.iter('testcase') if case.get('status') == 'run']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--program', default='test_libaom', choices=['test_libaom'])
    parser.add_argument('--shards', type=int, required=True)
    parser.add_argument('--index', type=int, required=True)
    parser.add_argument('--parts', type=int, default=8)
    parser.add_argument('--input-dir', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    parser.add_argument('--reference-build', type=Path)
    parser.add_argument('--filter', default='*')
    args = parser.parse_args()
    if args.shards < 1 or not 0 <= args.index < args.shards or args.parts < 2:
        parser.error('Require a valid original shard index and at least two finer parts')
    profile = 'debug' if args.debug else 'release'
    native = args.reference_build or ROOT / f'build/aom-native-{profile}'
    selected = [name for name in inventory(native / args.program, args.filter) if not disabled(name)]
    expected = selected[args.index::args.shards]
    records, proof = [], []
    fixture_bytes = None
    for segment in range(args.parts):
        index = args.index + args.shards * segment
        directory = args.input_dir / f'segment-{segment}'
        path = directory / f'shard-{index}.json'
        part = json.loads(path.read_text())
        if (part['identity']['profile'] != profile or part['identity']['filter'] != args.filter
                or part['identity']['program'] != args.program):
            raise ValueError('Finer shard profile or filter differs from the requested original shard')
        if [c['name'] for c in part['results']['c']['cases']] != expected[segment::args.parts]:
            raise ValueError('Finer shard does not match the original active registration partition')
        evidence = dict(report_path=str(path.resolve().relative_to(ROOT)),
                        report_sha256=digest(path), xml_sha256={}, xml_paths={})
        for side in ('c', 'rust'):
            xml = directory / f'shard-{index}-{side}.xml'
            if xml_cases(xml) != part['results'][side]['cases']:
                raise ValueError('Finer shard JSON differs from its actual original test XML')
            evidence['xml_sha256'][side] = digest(xml)
            evidence['xml_paths'][side] = str(xml.resolve().relative_to(ROOT))
        if args.filter == '*':
            data = (directory / 'fixtures.json').read_bytes()
            if fixture_bytes is not None and data != fixture_bytes:
                raise ValueError('Finer shard fixture inventories differ')
            fixture_bytes = data
        records.append(part)
        proof.append(evidence)
    identity = dict(records[0]['identity'], shards=args.shards)
    merged = merge_subshards(identity, args.index, records)
    if [case['name'] for case in merged['results']['c']['cases']] != expected:
        raise ValueError('Recovery did not preserve the complete original shard inventory')
    merged['subshard_provenance'] = proof
    if not passing_subshards(merged, ROOT):
        raise ValueError('Recovered evidence does not pass the complete-suite collector contract')
    args.output_dir.mkdir(parents=True, exist_ok=True)
    if fixture_bytes is not None:
        target = args.output_dir / 'fixtures.json'
        if target.exists() and target.read_bytes() != fixture_bytes:
            raise ValueError('Existing original fixture inventory differs; retained without changes')
        target.write_bytes(fixture_bytes)
    target = args.output_dir / f'shard-{args.index}.json'
    if target.exists():
        previous = json.loads(target.read_text())
        if previous.get('valid') and previous != merged:
            raise ValueError('A passing original record already exists; retained without changes')
    target.write_text(json.dumps(merged, indent=2) + '\n')
    print(f'Recovered {profile}/{args.program} original shard {args.index + 1}/{args.shards}: '
          f'{len(expected)} original cases in {args.parts} complete finer C/Rust shards')


if __name__ == '__main__':
    main()
