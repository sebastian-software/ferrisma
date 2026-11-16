#!/usr/bin/env python3
"""The root is libavif with AVIF_CODEC_DAV1D=OFF (port ADR 0061): guards of the tools."""
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest

REPOSITORY = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPOSITORY / 'tools'))
import check_aom_root_linkage as linkage
import import_optional
import import_translation
import root_variants
import transpile_root
import translate_registry

DAV1D_ENTRY = re.compile(r'        \{\n            let mut init = AvailableCodec \{\n'
                         r'                choice: AVIF_CODEC_CHOICE_DAV1D,\n.*?\n            init\n        \},\n', re.S)


def raw_registry(with_dav1d):
    """Undo the registry's feature guards: the shape of C2Rust's all-guards avif.c table."""
    committed = json.loads((REPOSITORY / 'translations/codec-registry.json').read_text())
    table = re.sub(r'        #\[cfg\(feature = "\w+"\)\]\n', '', committed['table'])
    if not with_dav1d:
        table = DAV1D_ENTRY.sub('', table)
    count = table.count('let mut init = AvailableCodec {')
    table = table.replace(committed['array_type'], f'[AvailableCodec; {count}]')
    return table + '\nstatic mut availableCodecsCount: core::ffi::c_int = 0;\n'


class CodecRegistry(unittest.TestCase):
    def test_a_dav1d_entry_is_rejected(self):
        with self.assertRaisesRegex(SystemExit, 'AVIF_CODEC_DAV1D=OFF'):
            import_optional.registry(raw_registry(with_dav1d=True))

    def test_dav1d_off_table_keeps_aom_and_auto_unconditional(self):
        committed = json.loads((REPOSITORY / 'translations/codec-registry.json').read_text())
        record = import_optional.registry(raw_registry(with_dav1d=False))
        if 'AVIF_CODEC_CHOICE_DAV1D' in committed['table']:
            # The committed record predates the regeneration; its table minus
            # the dav1d entry is what the importer derives from the raw unit.
            expected = DAV1D_ENTRY.sub('', committed['table']).replace('[AvailableCodec; 3 + ', '[AvailableCodec; 2 + ')
        else:
            expected = committed['table']
        self.assertEqual(record['table'], expected)
        self.assertTrue(record['array_type'].startswith('[AvailableCodec; 2 + cfg!(feature = "libgav1")'))
        self.assertNotIn('DAV1D', record['table'])

    def test_registry_command_rejects_a_dav1d_reference(self):
        for flag in ['-DAVIF_CODEC_DAV1D=1', '-DAVIF_CODEC_DAV1D']:
            with self.subTest(flag=flag), self.assertRaisesRegex(SystemExit, 'AVIF_CODEC_DAV1D=OFF'):
                translate_registry.registry_command(f'clang -DAVIF_CODEC_AOM=1 {flag} -c avif.c')
        command = translate_registry.registry_command('clang -DAVIF_CODEC_AOM=1 -c avif.c')
        self.assertTrue(command.endswith(' '.join(translate_registry.GUARDS)))


class Closure(unittest.TestCase):
    def test_no_root_configuration_compiles_the_dav1d_adapter(self):
        for variant in [*root_variants.LINUX_PRODUCTION, 'libyuv']:
            with self.subTest(variant=variant):
                self.assertNotIn('upstream/libavif/src/codec_dav1d.c', root_variants.sources(variant))

    def test_build_cli_rejects_a_dav1d_root_reference(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            import platform
            target = 'aarch64-apple-darwin' if platform.system() == 'Darwin' else 'x86_64-unknown-linux-gnu'
            (root / 'reference-profile.json').write_text(json.dumps(dict(
                target=target, profile='release', original_dav1d_units=51, original_sharpyuv_units=7,
                references=dict(dav1d=dict(path=str(root / 'libdav1d.a'), sha256='0' * 64),
                                sharpyuv=dict(path=str(root / 'libsharpyuv.a'), sha256='0' * 64)))))
            result = subprocess.run([sys.executable, str(REPOSITORY / 'tools/build.py'), 'baseline',
                                     '--root-references', str(root)], cwd=REPOSITORY, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('different target, C profile, or source closure', result.stdout + result.stderr)

    def test_c_reference_is_configured_without_dav1d(self):
        text = (REPOSITORY / 'tools/build.py').read_text()
        self.assertIn('"-DAVIF_CODEC_DAV1D=OFF"', text)
        self.assertNotIn('DAV1D_LIBRARY', text)


class Linkage(unittest.TestCase):
    def test_any_dav1d_owner_is_rejected(self):
        for member in ['/x/libavif.a(dav1d.dav1d.1-cgu.0.rcgu.o)', '/usr/lib/libdav1d.a(lib.c.o)']:
            with self.subTest(member=member), self.assertRaisesRegex(SystemExit, 'dav1d implementation'):
                linkage.reject_dav1d('avifdec', {'dav1d_open': member, 'aom_codec_decode': '/x/libavif.a(aom.o)'})
        linkage.reject_dav1d('avifdec', {'aom_codec_decode': '/x/libavif.a(aom.o)'})


class Generation(unittest.TestCase):
    def test_units_outside_the_closure_are_stale(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            destination = root / 'src/profiles/linux/release/default'
            (destination / 'libyuv').mkdir(parents=True)
            for name in ['avif.rs', 'codec_dav1d.rs', 'modules.rs', 'libyuv/scale.rs']:
                (destination / name).write_text('// fixture\n')
            units = [dict(rust=f'src/profiles/linux/release/default/{name}') for name in ['avif.rs', 'libyuv/scale.rs']]
            stale = transpile_root.stale_units(destination, units, destination / 'modules.rs', root)
            self.assertEqual(stale, [destination / 'codec_dav1d.rs'])

    def test_hosted_database_is_rebased_onto_this_checkout(self):
        entries = [dict(file='/Users/runner/work/ferrisma/ferrisma/codecs/libavif-rs/upstream/libavif/src/avif.c',
                        command='clang -c avif.c'),
                   dict(file='/home/runner/work/x/codecs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c',
                        command='clang -c scale.c')]
        rebased = import_translation.rebased(entries, REPOSITORY)
        self.assertEqual([entry['file'] for entry in rebased],
                         [str(REPOSITORY / 'upstream/libavif/src/avif.c'),
                          str(REPOSITORY / 'upstream/libavif/third_party/libyuv/source/scale.c')])
        self.assertEqual(rebased[0]['command'], 'clang -c avif.c')
        with self.assertRaises(SystemExit):
            import_translation.rebased([dict(file='/elsewhere/avif.c', command='')], REPOSITORY)


if __name__ == '__main__':
    unittest.main()
