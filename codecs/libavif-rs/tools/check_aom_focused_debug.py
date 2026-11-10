#!/usr/bin/env python3
"""Run unchanged original Debug API, memory, and alignment regression cases."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

from merge_aom_subshards import xml_cases
from run_aom_paired_shard import passing_pair

ROOT = Path(__file__).resolve().parents[1]
FILTER = ':'.join(['AomImageTest.*', 'AomLeb128.*', 'AomMemTest.*', 'DecodeAPI.*',
    'EncodeAPI.Invalid*', 'FilmGrainTableIOTest.*', 'FilmGrainTableTest.*',
    'MetadataTest.*', 'KeyValAPI/*.*', 'EncodeAPI.SizeAlignOverflow',
    'HighbdEncodeSmallWidthHeight.*'])


def verify_focused_debug(root):
    report = json.loads((root / 'build/aom-focused-debug.json').read_text())
    suffix = hashlib.sha256(FILTER.encode()).hexdigest()[:12]
    directory = root / f'build/aom-paired-debug-test_libaom-focused-{suffix}'
    pair = json.loads((directory / 'shard-0.json').read_text())
    if (not report['valid'] or report['executed'] != 69 or report['dynamic_skips']
            or report['identity']['filter'] != FILTER or report['identity']['profile'] != 'debug'
            or pair['identity'] != report['identity'] or not passing_pair(pair)
            or len(pair['results']['c']['cases']) != 69):
        raise ValueError('Focused original Debug evidence is incomplete')
    for side, kind in [('c', 'native'), ('rust', 'rust')]:
        xml = directory / f'shard-0-{side}.xml'
        binary = root / f'build/aom-{kind}-debug/test_libaom'
        if (xml_cases(xml) != pair['results'][side]['cases']
                or hashlib.sha256(xml.read_bytes()).hexdigest() != report['xml_sha256'][side]
                or hashlib.sha256(binary.read_bytes()).hexdigest() != report['identity'][f'{side}_executable_sha256']):
            raise ValueError('Focused Debug XML or original binary changed')
    target = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest = root / f'crates/aom/TRANSLATION-{target}-debug.json'
    if hashlib.sha256(manifest.read_bytes()).hexdigest() != report['identity']['translation_manifest_sha256']:
        raise ValueError('Focused Debug translation differs from the current source')
    return report


def main():
    command = ['python3', str(ROOT / 'tools/run_aom_paired_shard.py'),
               '--debug', '--shards', '1', '--filter', FILTER]
    subprocess.run([*command, '--index', '0'], cwd=ROOT, check=True)
    subprocess.run([*command, '--collect'], cwd=ROOT, check=True)
    suffix = hashlib.sha256(FILTER.encode()).hexdigest()[:12]
    directory = ROOT / f'build/aom-paired-debug-test_libaom-focused-{suffix}'
    report = json.loads((directory / 'summary.json').read_text())
    if not report['valid'] or report['executed'] != 69 or report['dynamic_skips']:
        raise ValueError('Focused original Debug coverage changed or skipped')
    report.update(acceptance_contract='ADR-0048', full_debug_acceptance=False,
                  xml_sha256={side: hashlib.sha256((directory / f'shard-0-{side}.xml').read_bytes()).hexdigest()
                              for side in ('c', 'rust')})
    (ROOT / 'build/aom-focused-debug.json').write_text(json.dumps(report, indent=2) + '\n')
    verify_focused_debug(ROOT)


if __name__ == '__main__':
    main()
