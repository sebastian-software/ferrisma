"""Validate finer original GoogleTest shards without weakening full-suite gates."""
import hashlib
import itertools
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET


def passing_results(record):
    results = record.get('results', {})
    if set(results) != {'c', 'rust'}:
        return False
    c, rust = results['c'], results['rust']
    return (all(not value['exit_code'] and value['valid_xml']
                and not any(case['failed'] for case in value['cases'])
                for value in (c, rust)) and c['cases'] == rust['cases'])


def merge_subshards(identity, index, parts):
    """Interleave the complete finer partition of one original shard."""
    count = len(parts)
    if count < 2 or not 0 <= index < identity['shards']:
        raise ValueError('Invalid original shard or finer partition count')
    finer_identity = dict(identity, shards=identity['shards'] * count)
    for segment, part in enumerate(parts):
        if (part.get('identity') != finer_identity or
                part.get('index') != index + identity['shards'] * segment or
                part.get('valid') is not True or not passing_results(part) or
                'subshards' in part):
            raise ValueError('A finer shard failed or differs from the original execution contract')
    lengths = [len(part['results']['c']['cases']) for part in parts]
    total = sum(lengths)
    if lengths != [len(range(segment, total, count)) for segment in range(count)]:
        raise ValueError('The finer shards do not form the original ordered partition')
    merged = []
    for row in itertools.zip_longest(*(part['results']['c']['cases'] for part in parts)):
        merged.extend(case for case in row if case is not None)
    names = [case['name'] for case in merged]
    if len(names) != len(set(names)):
        raise ValueError('Duplicate original cases across finer shards')
    results = {side: dict(exit_code=0, valid_xml=True, xml_errors=[], cases=merged)
               for side in ('c', 'rust')}
    return dict(identity=identity, index=index, valid=True, results=results,
                execution_mode='complete_finer_original_gtest_shards',
                xml_validation_scope='Every separate C/Rust subshard XML; no single combined process is claimed',
                subshards=parts)


def passing_subshards(record, root=None):
    try:
        expected = merge_subshards(record['identity'], record['index'], record['subshards'])
        if not all(record.get(key) == value for key, value in expected.items()):
            return False
        proof = record['subshard_provenance']
        if len(proof) != len(record['subshards']):
            return False
        for part, evidence in zip(record['subshards'], proof):
            serialized = (json.dumps(part, indent=2) + '\n').encode()
            if evidence['report_sha256'] != hashlib.sha256(serialized).hexdigest():
                return False
            for side in ('c', 'rust'):
                if not re.fullmatch('[0-9a-f]{64}', evidence['xml_sha256'][side]):
                    return False
            if root is not None:
                root = Path(root).resolve()
                report = (root / evidence['report_path']).resolve()
                if not report.is_relative_to(root) or report.read_bytes() != serialized:
                    return False
                for side in ('c', 'rust'):
                    path = (root / evidence['xml_paths'][side]).resolve()
                    if (not path.is_relative_to(root) or
                            hashlib.sha256(path.read_bytes()).hexdigest() != evidence['xml_sha256'][side]):
                        return False
                    xml = ET.parse(path).getroot()
                    cases = [dict(name=c.attrib['classname'] + '.' + c.attrib['name'],
                        result=c.get('result'), failed=bool(c.findall('failure')),
                        skipped=bool(c.findall('skipped'))) for c in xml.iter('testcase')
                        if c.get('status') == 'run']
                    if (int(xml.get('errors', 0)) or int(xml.get('failures', 0)) or
                            cases != part['results'][side]['cases']):
                        return False
        return True
    except (KeyError, TypeError, ValueError, OSError, ET.ParseError):
        return False
