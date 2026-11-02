"""Expose reviewed inline test subjects through optional original-ABI wrappers."""
import re

# Each subject is explicitly reviewed in its original test. Native reference
# and fixture helpers in other test inputs keep their original bindings.
SUBJECTS = {
    'aom_mem_test.cc': ('aom_mem/aom_mem.h', ['aom_memset16']),
    'log2_test.cc': ('aom_ports/bitops.h', ['get_msb', 'aom_ceil_log2']),
    'av1_get_qmlevel_test.cc': ('av1/common/quant_common.h', ['aom_get_qmlevel', 'aom_get_qmlevel_allintra']),
    'intrabc_test.cc': ('av1/common/mvref_common.h', ['av1_is_dv_valid']),
    'block_test.cc': ('av1/common/blockd.h', ['get_partition_subsize']),
    'av1_common_int_test.cc': ('av1/common/av1_common_int.h', ['get_tx_size']),
    'boolcoder_test.cc': (['aom_dsp/bitreader.h', 'aom_dsp/bitwriter.h'], ['aom_read_', 'aom_write']),
    'scan_test.cc': ('av1/common/scan.h', ['get_default_scan']),
}
FILTER = ('AomMemTest.*:Log2Test.*:GetQmLevelTest.*:IntrabcTest.DvValidation:'
          'BlockdTest.GetPartitionSubsize:AV1CommonInt.TestGetTxSize:'
          'AV1.TestBitIO:AV1.TestTell:AV1.TestHasOverflowed:Av1ScanTest.Dependency')


def append_bridges(text, emitted):
    added = []
    for name in sorted({name for _, names in SUBJECTS.values() for name in names} - emitted):
        match = re.search(r'(?m)^(?:pub )?unsafe extern "C-unwind" fn ' + name + r'\(', text)
        if not match:
            continue
        opening = match.end() - 1
        depth = 1
        closing = opening
        while depth:
            closing += 1
            depth += (text[closing] == '(') - (text[closing] == ')')
        end = text.index('{', closing)
        signature = text[match.start():end].strip().removeprefix('pub ')
        signature = signature.replace(f'fn {name}(', f'fn port_aom_inline_{name}(', 1)
        parameters = text[opening + 1:closing]
        arguments = re.findall(r'(?:^|,)\s*(?:mut )?(\w+):(?!:)', parameters)
        if len(arguments) != len(re.findall(r'(?<!:):(?!:)', parameters)):
            raise SystemExit(f'Unmapped inline bridge parameters: {name}')
        signature = re.sub(r'\bmut (\w+):(?!:)', r'\1:', signature)
        text += ('\n#[cfg(feature = "port-test-bridges")]\n#[no_mangle]\npub ' + signature
                 + ' {\n    ' + name + '(' + ', '.join(arguments) + ')\n}\n')
        emitted.add(name)
        added.append(name)
    return text, added
