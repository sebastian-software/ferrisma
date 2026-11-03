#!/usr/bin/env python3
"""Per-row source-history evidence. No row is excluded from Rust comparison.

These are necessary phase/caller invariants, not a complete reachability proof.
A failed invariant classifies a synthetic direct state, not C arithmetic UB.
"""
import collections
import hashlib
import json
import pathlib
import struct

SOURCE_RULES = {
    'outer_size': 'decode.c2730-2732/2755-2757; constants.h92/95: literal 64*n, distance 4*n, n=1..256 from DecodeVarLenUint8+1 at2634-2647',
    'tree_count': 'decode.c1060-1065: DecodeVarLenUint8 range0..255, then increment=>1..256; >1 required to reach READ_PREFIX',
    'prefix': 'decode.c1084-1095: five-bit peek, prefix0 or (bits>>1)+1 =>0..16',
    'alphabet': 'decode.c1100-1105 and ReadHuffmanCode: alphabet=num_htrees+prefix, decoded symbol strictly below alphabet; successful table sets codeFFFF',
    'pending': 'decode.c1124-1142: only nonzero decoded code<=prefix reaches suffix; h.code persisted only on shortage=>1..prefix<=16',
    'saved_index': 'decode.c1065/1115/1125/1129/1146-1152: index starts0; guarded writes/runs; saved shortage index stays within map; TRANSFORM may retain stale index',
    'long': 'decode.c207-228: SHORT nonzero three-bit value=>1..7 is the only stored LONG width; failed read persists LONG',
    'transform': 'decode.c1158-1168: full initialized map prefix is required only for reached MTF, independent of stale h.context_index',
    'raw_width': 'bit_reader.h323-337 documents positive<=24; NDEBUG removes DCHECK; actual native64 requests0..32 observed defined, retained in full Rust comparison through approved guard extension',
}


def classify(words):
    size, which, mode, num, prefix, code, index, bound, _, symbol, available, val, fail, steps, length = map(int, words[:15])
    scale = 4 if which else 64
    invariants = {'outer_call_size_matches': scale <= size <= scale * 256 and size % scale == 0}
    origins = {
        0: 'actual NONE checkpoint; input is genuine encoder bytes, truncation/alignment/chunking or explicit mutated bytes',
        1: 'direct READ_PREFIX phase with existing actual map owner',
        2: 'direct HUFFMAN phase with initialized prefix/count and source Huffman NONE',
        3: 'direct DECODE phase with actual original singleton table',
        4: 'direct pending-RLE state with actual original singleton table',
        5: 'direct TRANSFORM with fully initialized map and actual identity-tail MTF initialization',
        6: 'explicit unknown context-map discriminant',
        7: 'direct DECODE phase with actual original two-symbol table and seeded initialized map prefix',
        8: 'direct uint8 LONG temporary; allocation failure forced after success',
        9: 'explicit unknown uint8 discriminant',
    }
    if mode in (1, 2, 3, 4, 7):
        invariants['allocated_count_2_to_256'] = 2 <= num <= 256
        invariants['index_within_map'] = 0 <= index <= size
    if mode == 1:
        invariants['prefix_return_shortage_bits_below5'] = available < 5
    if mode in (2, 3, 4, 7):
        invariants['prefix_0_to_16'] = 0 <= prefix <= 16
    if mode in (3, 7):
        invariants['table_symbol_below_context_alphabet'] = symbol < num + prefix
        invariants['code_is_no_pending_sentinel'] = code == 65535
    if mode == 4:
        invariants['pending_code_1_to_maxprefix'] = 1 <= code <= prefix <= 16
        invariants['saved_suffix_shortage_bits_below_code'] = available < code
        invariants['saved_pending_index_below_size'] = index < size
    if mode == 5:
        invariants['stale_index_within_original_write_history'] = index <= size
        invariants['mtf_bound_0_to_63'] = 0 <= bound <= 63
        invariants['map_prefix_initialized_independently_of_index'] = True
    if mode == 6:
        invariants['context_discriminant_from_source_assignments'] = False
    if mode == 8:
        invariants['long_width_from_nonzero_three_bits'] = 1 <= num <= 7
        invariants['saved_long_shortage_bits_below_width'] = available < num
    if mode == 9:
        invariants['uint8_discriminant_from_source_assignments'] = False
    return {'mode': mode, 'origin': origins[mode], 'which': which,
            'size': size, 'num': num, 'prefix': prefix, 'code': code,
            'index': index, 'available_bits': available,
            'necessary_invariants': invariants,
            'all_listed_necessary_invariants': all(invariants.values()),
            'comparison_policy': 'retain exact source/Rust comparison regardless of history classification',
            'C_arithmetic_UB_claim': False}


def read_record(stream):
    head = stream.read(96)
    assert len(head) == 96
    values = struct.unpack('<12Q', head)
    rest = stream.read(values[11] + 3756 + 4 + 8 + 260 + values[8])
    assert len(rest) == values[11] + 3756 + 4 + 8 + 260 + values[8]
    status = values[0] if values[0] < 1 << 63 else values[0] - (1 << 64)
    phase, = struct.unpack_from('<I', rest, values[11] + 4)
    return head + rest, {'status': status, 'returned_context_phase': phase,
                         'proved_map_prefix_lower_bound': values[9],
                         'allocation_callbacks': values[10]}


def capture_inventory(requests, source, output, retained=None):
    rows = requests.read_text().splitlines()
    counts = collections.Counter()
    records = 0
    kept_records = 0
    kept_bytes = 0
    old = retained.open('rb') if retained else None
    try:
        with source.open('rb') as stream, output.open('w') as report:
            for row_number, row in enumerate(rows, 1):
                words = row.split()
                history = classify(words)
                mode = int(words[2])
                width = int(words[5]) if mode == 4 else int(words[3]) if mode == 8 else None
                previously_compared = width is None or 1 <= width <= 24
                observations = []
                for seed in (165, 90):
                    seed_observations = []
                    for _ in range(int(words[13])):
                        record, observation = read_record(stream)
                        records += 1
                        seed_observations.append(observation)
                        counts[str(observation['status'])] += 1
                        if old and previously_compared:
                            assert old.read(len(record)) == record, (row_number, seed)
                            kept_records += 1
                            kept_bytes += len(record)
                        if observation['status'] != 2:
                            break
                    observations.append({'seed': seed, 'records': seed_observations})
                history.update({'row': row_number,
                                'request_sha256': hashlib.sha256((row + '\n').encode()).hexdigest(),
                                'source_observations': observations})
                report.write(json.dumps(history, separators=(',', ':')) + '\n')
            assert not stream.read(1)
            if old:
                assert not old.read(1)
    finally:
        if old:
            old.close()
    return {'requests': len(rows), 'records': records, 'statuses': dict(counts),
            'necessary_rules': SOURCE_RULES,
            'retained_previous_records': kept_records,
            'retained_previous_bytes': kept_bytes,
            'no_history_filtering': True,
            'caller_history_is_not_C_UB_classification': True}
