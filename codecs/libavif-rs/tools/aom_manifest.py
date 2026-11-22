#!/usr/bin/env python3
"""The one translation record of the libaom port (route B, codec ADR 0063).

`crates/aom/TRANSLATION.json` describes the one owned source tree
`crates/aom/src/libaom/`: the prepared C presentation, both generation
targets (CPU dispatch headers per target) and every unit. Every build profile
on both supported targets compiles the same units, so the tools that used to
read `TRANSLATION-<platform>-<profile>.json` ask `view(platform, profile)` for
the record of what that build compiles; the shape is the old one.
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / 'crates/aom'
PATH = CRATE / 'TRANSLATION.json'
TREE = 'crates/aom/src/libaom'
PLATFORMS = ('macos', 'linux')
PROFILES = ('debug', 'release')
GENERATION_TARGETS = {'macos': 'aarch64-apple-darwin', 'linux': 'x86_64-unknown-linux-gnu'}


def load():
    return json.loads(PATH.read_text())


def save(record):
    PATH.write_text(json.dumps(record, indent=2) + '\n')


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def view(platform, profile, record=None):
    """The record of what `profile` compiles on `platform` (old manifest shape)."""
    if platform not in PLATFORMS or profile not in PROFILES:
        raise SystemExit(f'Unknown libaom profile: {platform}/{profile}')
    record = record or load()
    target = record['targets'][platform]
    return dict(translator=record['translator'], llvm=record['llvm'],
                preprocessing_profile='one-source', build_profile=profile,
                generation_target=GENERATION_TARGETS[platform],
                # The original C build of this profile, which the gates compare with.
                generated_reference_headers=target['reference_builds'][profile],
                presentation_headers=target['presentation_headers'],
                rtcd_dispatch=target['rtcd_dispatch'],
                optional_inline_test_bridges=record['optional_inline_test_bridges'],
                units=[dict(source=unit['source'], rust=unit['rust'], rust_sha256=unit['rust_sha256'],
                            production_rust_sha256=unit['production_rust_sha256'],
                            raw_sha256=unit['raw_sha256'][platform],
                            corrections=unit['corrections'][platform]) for unit in record['units']])


def reference(platform):
    """Directory of a target's prepared configuration and dispatch headers."""
    return CRATE / 'reference/one-source' / platform
