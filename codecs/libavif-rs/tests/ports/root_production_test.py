#!/usr/bin/env python3
"""Check production root selection against the admitted native profiles (ADR 0054)."""
import itertools
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest

REPOSITORY = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPOSITORY / 'tools'))
import root_production as production
import root_variants
import update_production_selection as selection

LINUX = {'target_os': 'linux', 'target_arch': 'x86_64', 'target_env': 'gnu'}


def tokens(text):
    return re.findall(r'"[^"]*"|\w+|[(),=]', text)


def evaluate(expression, features, debug, target=LINUX):
    """Evaluate the cfg predicates used by the generated selector."""
    items, position = tokens(expression), 0

    def parse():
        nonlocal position
        name = items[position]
        position += 1
        if name in ['all', 'any', 'not']:
            assert items[position] == '('
            position += 1
            values = []
            while items[position] != ')':
                values.append(parse())
                if items[position] == ',':
                    position += 1
            position += 1
            if name == 'not':
                assert len(values) == 1
                return not values[0]
            return all(values) if name == 'all' else any(values)
        if name == 'debug_assertions':
            return debug
        assert items[position] == '='
        value = items[position + 1].strip('"')
        position += 2
        return value in features if name == 'feature' else target[name] == value

    result = parse()
    assert position == len(items), expression
    return result


def selector_items():
    blocks = selection.OUTPUT.read_text().split('\n\n')
    guard = re.fullmatch(r'(?:// .*\n)*#\[cfg\((.*)\)\]\ncompile_error!.*', blocks[0], re.S).group(1)
    includes = [re.fullmatch(r'#\[cfg\((.*)\)\]\ninclude!\("([^"]+)"\);\n?', block, re.S).groups()
                for block in blocks[1:]]
    return guard, includes


class ProductionSelection(unittest.TestCase):
    def test_selector_is_generated_from_the_admitted_configurations(self):
        self.assertEqual(selection.render(), selection.OUTPUT.read_text())

    def test_each_feature_set_selects_exactly_its_admitted_profile(self):
        guard, includes = selector_items()
        self.assertEqual(len(includes), 2 * len(root_variants.LINUX_PRODUCTION))
        for count in range(len(selection.FEATURES) + 1):
            for selected in itertools.combinations(selection.FEATURES, count):
                variant = '-'.join(name for name in selection.FEATURES if name in selected) or 'default'
                admitted = variant in root_variants.LINUX_PRODUCTION
                for profile, debug in [('release', False), ('debug', True)]:
                    with self.subTest(features=selected, profile=profile):
                        chosen = [path for condition, path in includes if evaluate(condition, set(selected), debug)]
                        self.assertEqual(evaluate(guard, set(selected), debug), not admitted)
                        self.assertEqual(chosen, [f'profiles/linux/{profile}/{variant}/modules.rs'] if admitted else [])

    def test_other_targets_include_no_linux_profile(self):
        _, includes = selector_items()
        macos = dict(target_os='macos', target_arch='aarch64', target_env='')
        musl = dict(LINUX, target_env='musl')
        for target in [macos, musl]:
            self.assertFalse(any(evaluate(condition, set(), debug, target)
                                 for condition, _ in includes for debug in [False, True]))

    def test_each_admitted_profile_is_a_complete_generated_manifest(self):
        for variant in root_variants.LINUX_PRODUCTION:
            # build.rs links engines only as the verified complete set.
            self.assertIn(root_variants.engines(variant), [[], root_variants.ENGINES])
            for profile in ['release', 'debug']:
                record = json.loads((REPOSITORY / f'translations/root-linux-{profile}-{variant}.json').read_text())
                self.assertEqual(record['selector'], f'src/profiles/linux/{profile}/{variant}/modules.rs')
                self.assertEqual(len(record['units']), len(root_variants.sources(variant)))

    def test_crate_root_keeps_the_macos_tree_and_gates_linux(self):
        text = (REPOSITORY / 'src/lib.rs').read_text()
        self.assertIn('#[cfg(all(target_os = "macos", target_arch = "aarch64"))]\ninclude!("modules.rs");', text)
        self.assertIn('#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]\n'
                      'include!("linux_profiles.rs");', text)
        self.assertIn('compile_error!("This generated configuration is validated only for aarch64-apple-darwin '
                      'and x86_64-unknown-linux-gnu")', text)


class ProductionSourceProof(unittest.TestCase):
    def record(self, profile='release', variant='default', native='linux'):
        return json.loads((REPOSITORY / f'translations/root-{native}-{profile}-{variant}.json').read_text())

    def linux_sources(self, record):
        compiled = {unit['rust']: unit['rust_sha256'] for unit in record['units']}
        compiled[record['selector']] = record['selector_sha256']
        compiled.update({'src/lib.rs': 'root', 'src/linux_profiles.rs': 'selection'})
        return compiled

    def test_linux_package_must_compile_exactly_the_profile(self):
        record = self.record('debug', 'mini-extended-pixi')
        compiled = self.linux_sources(record)
        self.assertEqual(production.match_profile(compiled, record, production.LINUX)['units'], 28)
        for change in ['missing', 'extra', 'hash', 'release', 'selector']:
            with self.subTest(change=change):
                altered = dict(compiled)
                unit = record['units'][0]
                if change == 'missing':
                    del altered[unit['rust']]
                elif change == 'extra':
                    altered['src/avif.rs'] = 'macos'
                elif change == 'hash':
                    altered[unit['rust']] = '0' * 64
                elif change == 'release':
                    altered = self.linux_sources(self.record('release', 'mini-extended-pixi'))
                else:
                    del altered['src/linux_profiles.rs']
                with self.assertRaises(SystemExit):
                    production.match_profile(altered, record, production.LINUX)

    def test_macos_package_units_must_equal_the_profile_bytes(self):
        record = self.record('release', 'mini-extended-pixi', 'macos')
        compiled = {'src/lib.rs': 'root', 'src/modules.rs': 'selection'}
        for unit in record['units']:
            name = Path(unit['rust']).relative_to('src/profiles/macos/release/mini-extended-pixi')
            source = next(path for path in [Path('src/variants/mini-extended-pixi') / name, Path('src') / name]
                          if (REPOSITORY / path).is_file())
            compiled[str(source)] = production.digest(REPOSITORY / source)
        self.assertFalse(production.match_profile(compiled, record, production.MACOS)['exact_profile_paths'])
        compiled['src/read.rs'] = production.digest(REPOSITORY / 'src/read.rs')
        with self.assertRaises(SystemExit):
            production.match_profile(compiled, record, production.MACOS)
        with self.assertRaises(SystemExit):
            production.match_profile(compiled, record, production.LINUX)

    def test_dep_info_names_only_sources_below_src(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            for name in ['src/lib.rs', 'src/a b.rs', 'crates/aom/src/lib.rs', 'build.rs']:
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                (root / name).write_text(name)
            info = root / 'libavif.d'
            info.write_text(f'{root}/target/libavif.a {root}/target/libavif.so: {root}/build.rs \\\n'
                            f' {root}/src/lib.rs {root}/src/a\\ b.rs {root}/crates/aom/src/lib.rs\n\n'
                            f'{root}/src/lib.rs:\n')
            self.assertEqual(list(production.dependency_sources(info, root)), ['src/a b.rs', 'src/lib.rs'])


class ProductionCommandGuards(unittest.TestCase):
    def pipeline(self, *arguments):
        return subprocess.run([sys.executable, str(REPOSITORY / 'tools/root_pipeline.py'), '--phase', 'validate',
                               '--clang-include', '/nonexistent', '--package', 'production', *arguments],
                              cwd=REPOSITORY, capture_output=True, text=True)

    def test_production_sanitizer_is_not_claimed(self):
        result = self.pipeline('--sanitizer', 'address', '--profile', 'release', '--tier', 'full')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Production sanitizer parity', result.stderr)

    def test_production_validator_rejects_sanitizer(self):
        result = subprocess.run([sys.executable, str(REPOSITORY / 'tools/validate_root_profile.py'), '--baseline',
                                 'build/none', '--port', 'build/none', '--root-profile', 'none.json', '--tier', 'full',
                                 '--sanitizer', 'address', '--reference-baseline', 'build/none', '--production'],
                                cwd=REPOSITORY, capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Production sanitizer parity', result.stderr)


if __name__ == '__main__':
    unittest.main()
