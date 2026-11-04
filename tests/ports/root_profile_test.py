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
from unittest.mock import patch

REPOSITORY = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPOSITORY / 'tools'))
import check_root_profile as subject
import build_root_references as reference
import check_aom_root_linkage as linkage


class ElfRuntimeGuards(unittest.TestCase):
    def graph(self, native=False, missing=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            binary, dependency = root / 'avifenc', root / 'libretained.so'
            binary.touch()
            dependency.touch()

            def output(command, **kwargs):
                if command[0] == 'ldd':
                    return f'libretained.so => {dependency} (0x1)\n'
                name = 'libaom.so.3' if native else 'libmissing.so' if missing else None
                if str(command[-1]) == str(binary):
                    name = 'libretained.so'
                return '' if name is None else f' (NEEDED) Shared library: [{name}]\n'

            with patch.object(linkage.subprocess, 'check_output', side_effect=output):
                return linkage.elf_dynamic_graph(binary)

    def test_runtime_proof_includes_transitive_elf_objects(self):
        self.assertEqual(len(self.graph()), 2)

    def test_transitive_native_codec_is_rejected(self):
        with self.assertRaisesRegex(SystemExit, 'Native default implementation'):
            self.graph(native=True)

    def test_unresolved_transitive_runtime_cannot_be_omitted(self):
        with self.assertRaisesRegex(SystemExit, 'Unresolved ELF DT_NEEDED'):
            self.graph(missing=True)


class ScalarReferenceGuards(unittest.TestCase):
    entry = dict(command='clang -DHAVE_CONFIG_H -mno-sse2 -msse2 -o dsp.o -c /original/sharpyuv_dsp.c',
                 file='/original/sharpyuv_dsp.c', directory='/original')

    def macros(self, definitions):
        result = subprocess.CompletedProcess([], 0, stdout=definitions, stderr='')
        with patch.object(reference.subprocess, 'run', return_value=result):
            return reference.scalar_macros(self.entry)

    def test_scalar_dsp_can_retain_the_required_x86_64_compiler_abi(self):
        self.macros('#define HAVE_CONFIG_H 1\n#define __SSE2__ 1\n')

    def test_a_defined_zero_dsp_macro_still_enables_original_ifdef_code(self):
        with self.assertRaises(SystemExit):
            self.macros('#define HAVE_CONFIG_H 1\n#define WEBP_HAVE_SSE2 0\n')

    def test_each_manual_dsp_path_is_rejected(self):
        for suffix in ['SSE2', 'SSE41', 'AVX2', 'NEON', 'MIPS32', 'MIPS_DSP_R2', 'MSA']:
            with self.subTest(suffix=suffix), self.assertRaises(SystemExit):
                self.macros('#define HAVE_CONFIG_H 1\n#define WEBP_USE_' + suffix + ' 1\n')

    def test_missing_generated_configuration_is_rejected(self):
        with self.assertRaises(SystemExit):
            self.macros('#define __SSE2__ 1\n')


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
