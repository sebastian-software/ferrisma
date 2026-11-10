#!/usr/bin/env python3
"""Protect complete artifact pagination and preservation of original evidence."""
import json
from pathlib import Path
import stat
import sys
import tempfile
import unittest
from unittest.mock import patch
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from download_aom_artifacts import list_artifacts, merge_archive


class CompleteArtifactDownload(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)

    def archive(self, entries):
        path = self.root / 'artifact.zip'
        with zipfile.ZipFile(path, 'w') as zipped:
            for name, content in entries:
                zipped.writestr(name, content)
        return path

    def test_more_than_one_hundred_artifacts_are_enumerated(self):
        records = [dict(id=i, name=f'aom-release-shard-test_libaom-{i}', expired=False) for i in range(106)]
        records += [dict(id=200+i, name=f'other-{i}', expired=False) for i in range(4)]
        def response(command, **kwargs):
            selected = records if '--paginate' in command else records[:100]
            return '\n'.join(json.dumps(record) for record in selected)
        with patch('download_aom_artifacts.subprocess.check_output', side_effect=response):
            all_items, selected = list_artifacts('org/repo', 1, 'aom-release-shard-test_libaom-*')
        self.assertEqual(len(all_items), 110)
        self.assertEqual(len(selected), 106)

    def test_expired_artifact_is_not_silently_omitted(self):
        with patch('download_aom_artifacts.subprocess.check_output', return_value=json.dumps(dict(id=1, name='part', expired=True))):
            with self.assertRaises(ValueError):
                list_artifacts('org/repo', 1, '*')

    def test_duplicate_named_artifacts_are_rejected(self):
        values = [dict(id=i, name='same', expired=False) for i in range(2)]
        with patch('download_aom_artifacts.subprocess.check_output', return_value='\n'.join(map(json.dumps, values))):
            with self.assertRaises(ValueError):
                list_artifacts('org/repo', 1, '*')

    def test_matching_evidence_can_be_merged_without_changes(self):
        archive = self.archive([('pairs/shard-0.xml', b'original')])
        output = self.root / 'build'
        first = merge_archive(archive, output)
        self.assertEqual(merge_archive(archive, output), first)
        self.assertEqual((output / 'pairs/shard-0.xml').read_bytes(), b'original')

    def test_conflict_preserves_existing_evidence_and_does_not_partially_import(self):
        output = self.root / 'build'
        output.mkdir()
        (output / 'original.xml').write_bytes(b'original')
        archive = self.archive([('new.xml', b'new'), ('original.xml', b'changed')])
        with self.assertRaises(ValueError):
            merge_archive(archive, output)
        self.assertEqual((output / 'original.xml').read_bytes(), b'original')
        self.assertFalse((output / 'new.xml').exists())

    def test_path_escape_is_rejected(self):
        archive = self.archive([('../outside', b'wrong')])
        with self.assertRaises(ValueError):
            merge_archive(archive, self.root / 'build')
        self.assertFalse((self.root / 'outside').exists())

    def test_symbolic_link_is_rejected(self):
        entry = zipfile.ZipInfo('link')
        entry.create_system = 3
        entry.external_attr = (stat.S_IFLNK | 0o777) << 16
        archive = self.archive([(entry, b'../outside')])
        with self.assertRaises(ValueError):
            merge_archive(archive, self.root / 'build')


if __name__ == '__main__':
    unittest.main()
