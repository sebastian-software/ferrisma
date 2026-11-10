"""Generate one ABI-checked Rust view of libaom's exported C functions."""
import json
from pathlib import Path
import re
import subprocess

from check_abi import compiler_flags

ROOT = Path(__file__).resolve().parents[1]


def facade(units, build):
    canonical = next(unit for unit in units if unit['source'].endswith('/av1/av1_cx_iface.c'))
    common = (ROOT / canonical['rust']).read_text()
    lines = ['// Generated from original export lists and translated signatures.',
             '// Public pointers, ownership, callbacks, and bounds follow the original headers.',
             'pub use crate::av1__av1_cx_iface::*;',
             'pub use crate::aom_dsp__bitreader_buffer::aom_read_bit_buffer;',
             'pub use crate::av1__common__obu_util::ObuHeader;']
    if 'pub type aom_codec_dec_cfg_t =' not in common:
        lines.append('pub type aom_codec_dec_cfg_t = aom_codec_dec_cfg;')
    declarations = {}
    for unit in units:
        text = (ROOT / unit['rust']).read_text()
        for match in re.finditer(r'pub unsafe extern "C-unwind" fn (\w+)\(', text):
            start = match.end() - 1
            depth, end = 0, None
            # Signatures contain no string literals or braces before the body.
            for offset in range(start, len(text)):
                if text[offset] == '(':
                    depth += 1
                elif text[offset] == ')':
                    depth -= 1
                elif text[offset] == '{' and not depth:
                    end = offset
                    break
            signature = text[match.start():end].strip()
            signature = signature.replace('pub unsafe extern "C-unwind" fn ', '    pub fn ', 1)
            signature = re.sub(r'\bmut (\w+):(?!:)', r'\1:', signature)
            signature = re.sub(r'\b\w+: \.\.\.', '...', signature)
            declarations[match[1]] = signature + ';'
    names = []
    for directory in ['aom', 'av1']:
        for suffix in ['com', 'dec', 'enc']:
            for line in (ROOT / f'upstream/aom/{directory}/exports_{suffix}').read_text().splitlines():
                kind, name = line.split()
                if kind == 'text':
                    if name not in declarations:
                        raise SystemExit(f'Missing original exported function: {name}')
                    names.append(name)
    if len(names) != len(set(names)):
        raise SystemExit('Duplicate original function export')
    lines.append('extern "C-unwind" {')
    for name in sorted(names):
        # The canonical module already exports this factory with its own types.
        if name != 'aom_codec_av1_cx':
            lines.append(declarations[name])
    lines.append('}')
    constants = {
        'AOM_IMAGE_ABI_VERSION': 'core::ffi::c_int',
        'AOM_CODEC_ABI_VERSION': 'core::ffi::c_int',
        'AOM_ENCODER_ABI_VERSION': 'core::ffi::c_int',
        'AOM_DECODER_ABI_VERSION': 'core::ffi::c_int',
        'AOM_EXT_PART_ABI_VERSION': 'core::ffi::c_int',
        'AOM_EXT_RATECTRL_ABI_VERSION': 'core::ffi::c_int',
        'AOM_TPL_ABI_VERSION': 'core::ffi::c_int',
        'AOM_USAGE_GOOD_QUALITY': 'core::ffi::c_uint',
        'AOM_USAGE_REALTIME': 'core::ffi::c_uint',
        'AOM_USAGE_ALL_INTRA': 'core::ffi::c_uint',
    }
    work = build / 'rust-api'
    work.mkdir(exist_ok=True)
    probe, ir = work / 'constants.c', work / 'constants.ll'
    include = '#include "aom/aomcx.h"\n#include "aom/aomdx.h"\n#include "aom/aom_decoder.h"\n'
    entry = json.loads((build / 'library-compile-commands.json').read_text())[0]
    flags = compiler_flags(entry)
    macros = subprocess.check_output(flags + ['-dM', '-E', '-x', 'c', '-'], input=include, text=True)
    for line in macros.splitlines():
        match = re.match(r'#define (AOM_\w+) ', line)
        if not match:
            continue
        name = match[1]
        if name.startswith('AOM_CODEC_CAP_'):
            constants[name] = 'aom_codec_caps_t'
        elif name.startswith('AOM_CODEC_USE_'):
            constants[name] = 'aom_codec_flags_t'
        elif name.startswith('AOM_EFLAG_'):
            constants[name] = 'aom_enc_frame_flags_t'
        elif name.startswith('AOM_FRAME_'):
            constants[name] = 'aom_codec_frame_flags_t'
        elif name.startswith('AOM_IMG_FMT_'):
            constants[name] = 'core::ffi::c_uint'
    constants = dict(sorted(constants.items()))
    probe.write_text(include + 'const unsigned long long aom_public_constants[] = {\n' +
                     ',\n'.join(constants) + '\n};\n')
    subprocess.run(flags + ['-S', '-emit-llvm', probe, '-o', ir], check=True, stderr=subprocess.PIPE)
    row = next(line for line in ir.read_text().splitlines() if line.startswith('@aom_public_constants ='))
    values = [int(value) for value in re.findall(r'i64 (\d+)', row)]
    if len(values) != len(constants):
        raise SystemExit('Public constant inventory changed')
    for (name, rust_type), value in zip(constants.items(), values):
        lines.append(f'pub const {name}: {rust_type} = {value};')
    return '\n'.join(lines) + '\n'
