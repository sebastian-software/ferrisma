#!/usr/bin/env python3
"""Name native root configurations and derive their original libavif source closure."""
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CONTAINER = ['mini', 'extended-pixi']
# Optional external engines selected exactly as upstream CMake selects them.
# AVM/AV2 remains a separate configuration package.
ENGINES = ['libgav1', 'rav1e', 'svt', 'libyuv']
ORDER = CONTAINER + ENGINES
ADAPTERS = {name: f'upstream/libavif/src/codec_{name}.c' for name in ['libgav1', 'rav1e', 'svt']}
CMAKE_OPTIONS = {'libgav1': 'AVIF_CODEC_LIBGAV1', 'rav1e': 'AVIF_CODEC_RAV1E', 'svt': 'AVIF_CODEC_SVT',
                 'libyuv': 'AVIF_LIBYUV'}
BUNDLED_SCALER = '/third_party/libyuv/'
# Configurations admitted to the production package on GNU Linux x86-64 after
# final consumer checks with the real package in Release and Debug (ADR 0054).
LINUX_PRODUCTION = ['default', 'mini', 'extended-pixi', 'mini-extended-pixi', 'libgav1-rav1e-svt-libyuv',
                    'mini-extended-pixi-libgav1-rav1e-svt-libyuv']


def features(variant):
    """Return the ordered features of a variant name, rejecting unknown or reordered names."""
    if variant == 'default':
        return []
    remaining, selected = variant, []
    for name in ORDER:
        if remaining == name or remaining.startswith(name + '-'):
            selected.append(name)
            remaining = remaining[len(name) + 1:]
    if remaining or not selected or '-'.join(selected) != variant:
        raise ValueError(f'Unknown or noncanonical native root configuration: {variant}')
    return selected


def name(selected):
    unknown = set(selected) - set(ORDER)
    if unknown:
        raise ValueError(f'Unsupported native root features: {sorted(unknown)}')
    return '-'.join(feature for feature in ORDER if feature in selected) or 'default'


def cargo_features(variant):
    return ','.join(features(variant))


def engines(variant):
    return [feature for feature in features(variant) if feature in ENGINES]


def sources(variant):
    """Return the original libavif units that upstream CMake compiles for the configuration."""
    selected = features(variant)
    units = [unit['source'] for unit in json.loads((ROOT / 'TRANSLATION.json').read_text())['units']]
    if 'libyuv' in selected:
        # External libyuv replaces the bundled scalar subset; see ADR 0007.
        units = [source for source in units if BUNDLED_SCALER not in source]
    return units + [ADAPTERS[feature] for feature in selected if feature in ADAPTERS]


def libyuv_overlay(include, destination):
    """Write ADR 0007's translation-only header overlay for external libyuv.

    Only the pointer-only vector constant records become opaque; the external
    library and its build remain unchanged.
    """
    destination.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(include / 'libyuv.h', destination.parent / 'libyuv.h')
    header = (include / 'libyuv/row.h').read_text()
    header, count = re.subn(r'^typedef \w+ __attribute__\(\(vector_size\(\d+\)\)\) \w+;\n', '', header, flags=re.M)
    if count != 13:
        raise SystemExit('Unexpected libyuv vector declarations; review the pinned header')
    for record in ['YuvConstants', 'ArgbConstants']:
        header = re.sub(r'struct ' + record + r' \{.*?\};', 'struct ' + record + ';', header, flags=re.S)
    (destination / 'row.h').write_text(header)
    return destination.parent


def pkg_config(package, dependency_root=ROOT):
    environment = None
    local = dependency_root / 'build/deps/pkgconfig'
    if local.is_dir():
        environment = os.environ.copy()
        existing = environment.get('PKG_CONFIG_PATH', '')
        environment['PKG_CONFIG_PATH'] = existing + (':' if existing else '') + str(local)
    return subprocess.check_output(['pkg-config', '--libs', package], text=True, env=environment).split()


def native_links(variant, dependency_root=ROOT):
    """Return build-script directives linking the selected original external engines.

    Static engine archives are not bundled into the Rust staticlib; the
    original CMake consumers link the same imported engine targets themselves.
    """
    cxx = 'c++' if platform.system() == 'Darwin' else 'stdc++'
    directives, runtime_paths = [], []
    for engine in engines(variant):
        if engine in ['libgav1', 'libyuv']:
            library, directory = ('gav1', 'libgav1-build') if engine == 'libgav1' else ('yuv', 'libyuv-build')
            path = dependency_root / 'build/deps' / directory
            if not (path / f'lib{library}.a').is_file():
                raise SystemExit(f'Build the pinned {engine} dependency first: {path}')
            directives += [f'cargo:rustc-link-search=native={path}', f'cargo:rustc-link-lib=static:-bundle={library}',
                           f'cargo:rustc-link-lib=dylib={cxx}']
            continue
        for flag in pkg_config({'rav1e': 'rav1e', 'svt': 'SvtAv1Enc'}[engine], dependency_root):
            if flag.startswith('-L'):
                directives.append(f'cargo:rustc-link-search=native={flag[2:]}')
                runtime_paths.append(flag[2:])
            elif flag.startswith('-l'):
                directives.append(f'cargo:rustc-link-lib={flag[2:]}')
            else:
                raise SystemExit(f'Unexpected {engine} link flag: {flag}')
    # Shared engines resolve through the Rust shared library's own run path.
    directives += [f'cargo:rustc-link-arg-cdylib=-Wl,-rpath,{path}' for path in dict.fromkeys(runtime_paths)]
    return directives, list(dict.fromkeys(runtime_paths))
