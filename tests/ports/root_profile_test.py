#!/usr/bin/env python3
"""Reject incomplete, modified, and cross-target generated root evidence."""
import copy
import json
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
import unittest

REPOSITORY = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPOSITORY / 'tools'))
import check_root_profile as subject


class RootProfileGuards(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.previous = subject.ROOT
        subject.ROOT = self.root
        self.addCleanup(setattr, subject, 'ROOT', self.previous)
        units = json.loads((REPOSITORY / 'TRANSLATION.json').read_text())['units']
        (self.root / 'TRANSLATION.json').write_text(json.dumps(dict(units=units)))
        prefix = Path('src/profiles/linux/release/default')
        entries = []
        for index, unit in enumerate(units):
            source = self.root / unit['source']
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_bytes((REPOSITORY / unit['source']).read_bytes())
            output = prefix / f'subject_{index}.rs'
            path = self.root / output
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(f'pub const SUBJECT_{index}: u32 = {index};\n')
            entries.append(dict(source=unit['source'], source_sha256=subject.digest(source),
                                rust=str(output), rust_sha256=subject.digest(path)))
        selector = prefix / 'modules.rs'
        (self.root / selector).write_text('// Guard fixture; this is not codec execution evidence.\n')
        self.record = dict(generation_target='x86_64-unknown-linux-gnu', preprocessing_profile='release',
                           variant='default', original_translation_units=28, units=entries,
                           selector=str(selector), selector_sha256=subject.digest(self.root / selector))
        self.manifest = self.root / 'profile.json'

    def validate(self, record=None):
        self.manifest.write_text(json.dumps(self.record if record is None else record))
        return subject.validate_profile(self.manifest, 'x86_64-unknown-linux-gnu')

    def test_complete_matching_profile_is_readable(self):
        self.assertEqual(len(self.validate()['units']), 28)

    def test_missing_original_unit_is_rejected(self):
        self.record['units'].pop()
        with self.assertRaises(SystemExit):
            self.validate()

    def test_duplicate_source_cannot_replace_an_original_unit(self):
        self.record['units'][-1] = copy.deepcopy(self.record['units'][0])
        with self.assertRaises(SystemExit):
            self.validate()

    def test_other_native_target_is_rejected(self):
        self.record['generation_target'] = 'aarch64-apple-darwin'
        with self.assertRaises(SystemExit):
            self.validate()

    def test_unknown_preprocessing_profile_is_rejected(self):
        self.record['preprocessing_profile'] = 'checked'
        with self.assertRaises(SystemExit):
            self.validate()

    def test_changed_original_source_is_rejected(self):
        (self.root / self.record['units'][0]['source']).write_text('changed original source')
        with self.assertRaises(SystemExit):
            self.validate()

    def test_changed_translation_is_rejected(self):
        (self.root / self.record['units'][0]['rust']).write_text('changed translation')
        with self.assertRaises(SystemExit):
            self.validate()

    def test_output_outside_the_declared_profile_is_rejected(self):
        self.record['units'][0]['rust'] = 'src/alpha.rs'
        with self.assertRaises(SystemExit):
            self.validate()

    def test_changed_selector_is_rejected(self):
        (self.root / self.record['selector']).write_text('different module selection')
        with self.assertRaises(SystemExit):
            self.validate()

    def test_original_c_profile_mismatch_is_rejected_by_build_cli(self):
        target = 'aarch64-apple-darwin' if platform.system() == 'Darwin' else 'x86_64-unknown-linux-gnu'
        (self.root / 'reference-profile.json').write_text(json.dumps(dict(
            target=target, profile='release', original_dav1d_units=51, original_sharpyuv_units=7)))
        result = subprocess.run([sys.executable, str(REPOSITORY / 'tools/build.py'), 'baseline',
                                 '--debug', '--upstream-debug', '--root-references', str(self.root)],
                                cwd=REPOSITORY, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('different target, C profile, or source closure', result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()
