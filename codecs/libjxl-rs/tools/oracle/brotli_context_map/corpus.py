#!/usr/bin/env python3
"""Defined original ContextMap requests, retaining genuine encoder fixture bytes."""
import struct


def fixtures(data):
    at = 0
    def word():
        nonlocal at
        out, = struct.unpack_from('<Q', data, at)
        at += 8
        return out
    count = word()
    assert count == 265
    out = []
    for _ in range(count):
        size, clusters, bits, n = [word() for _ in range(4)]
        assert n == (bits + 7) // 8
        encoded = data[at:at+n]
        at += n
        expected = data[at:at+size]
        at += size
        assert len(expected) == size and 1 <= clusters <= 256
        out.append((size, clusters, bits, encoded, expected))
    assert at == len(data)
    return out


def generate(data, include_synthetic_width_boundaries=True):
    rows = []
    def add(size=4, which=0, mode=0, num=2, prefix=0, code=65535,
            index=0, bound=63, symbol=0, available=0, val=0, fail=0,
            encoded=b'', chunks=None):
        if chunks is None:
            chunks = [len(encoded)]
        assert sum(chunks) <= len(encoded)
        values = [size, which, mode, num, prefix, code, index, bound, 0,
                  symbol, available, val, fail, len(chunks), len(encoded)]
        rows.append(' '.join(map(str, values + chunks)) + ' ' + encoded.hex() + '\n')
    for size, _, bits, encoded, _ in fixtures(data):
        for which in [0, 1]:
            add(size=size, which=which, encoded=encoded)
        add(size=size, encoded=encoded, fail=1)
        for available in range(1, min(8, bits + 1)):
            val = int.from_bytes(encoded[:1], 'little') & ((1 << available)-1)
            shifted = (int.from_bytes(encoded, 'little') >> available).to_bytes((bits-available+7)//8, 'little')
            add(size=size, available=available, val=val, encoded=shifted)
        for n in sorted(set(list(range(min(8, len(encoded))+1)) + [len(encoded)//2, max(0, len(encoded)-1)])):
            add(size=size, encoded=encoded[:n])
        for step in [1, 3, 7]:
            if step == 1 and len(encoded) > 64 or step == 3 and len(encoded) > 512:
                continue
            chunks = [0, 0] + [min(step, len(encoded)-i) for i in range(0, len(encoded), step)] + [0, 0]
            add(size=size, encoded=encoded, chunks=chunks)
        for bit in range(min(bits, 16)):
            changed = bytearray(encoded)
            changed[bit//8] ^= 1 << (bit & 7)
            add(size=size, encoded=bytes(changed))
    for which in [0, 1]:
        for pattern in range(32):
            for num in [2, 3, 16, 256]:
                for bits in [0, 1, 4, 5, 8, 15]:
                    add(which=which, mode=1, num=num, available=bits,
                        val=pattern & ((1 << bits)-1))
        for symbol in [0, 1, 16, 17, 255, 256, 271, 65535]:
            for prefix in [0, 1, 16]:
                for size in [0, 1, 8]:
                    for encoded in [b'', b'\0', b'\1']:
                        add(which=which, mode=3, size=size, symbol=symbol,
                            prefix=prefix, encoded=encoded)
        for code in (range(33) if include_synthetic_width_boundaries else range(1, 25)):
            for bits in range(33):
                for size in [0, 1, 2, 8, 64]:
                    for val in [0, (1 << bits)-1]:
                        add(which=which, mode=4, code=code, prefix=16,
                            size=size, available=bits, val=val)
        for symbol in [1, 17, 256, 65535]:
            for bits in range(16):
                for val in [0, (1 << bits)-1]:
                    for index in [0, 1, 4]:
                        add(which=which, mode=7, symbol=symbol, index=index,
                            available=bits, val=val)
        for size in [0, 1, 8, 256]:
            for bound in [0, 1, 63]:
                for encoded in [b'', b'\0', b'\1']:
                    add(which=which, mode=5, size=size, bound=bound,
                        index=(1 << 64)-1, encoded=encoded)
        for bits in range(33):
            for val in [0, (1 << bits)-1]:
                for width in (range(33) if include_synthetic_width_boundaries else range(1, 25)):
                    add(which=which, mode=8, num=width, available=bits,
                        val=val, fail=1)
        add(which=which, mode=6)
        add(which=which, mode=9)
        for prefix in range(17):
            for num in [2, 3, 16, 256]:
                for encoded in [b'', b'\0', b'\x11', b'\xff'*8]:
                    add(which=which, mode=2, num=num, prefix=prefix, encoded=encoded)
    return ''.join(rows).encode()
