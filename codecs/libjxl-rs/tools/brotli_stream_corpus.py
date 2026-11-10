#!/usr/bin/env python3
"""Genuine encoder inputs and retained full-entry decoder schedules.

The first315 encodes/3,780 requests and next8,542 requests retain the initial
whole-stream observations exactly. Encoder operations are actual public APIs;
no handcrafted compressed bitstream stands in for a successful encoder call.
"""

def encoder_requests():
    payloads = [b"", b"a", b"hello world\n", bytes(range(256)),
        b"time down life left back code data show only site city open just like free work text year over body love form book play live line\n" * 200,
        bytes((i * 97 + i // 13) & 255 for i in range(8192)), b"x" * 65536]
    return [f"encode {q} {w} {m} {data.hex() or '-'}"
            for q in [0, 1, 4, 9, 11] for w in [10, 16, 22]
            for m in [0, 1, 2] for data in payloads]


def metadata_parameters():
    # Preserve the same548 source-only callback traces, including the first8.
    requests = [(0, 0, 32, 17), (1, 0, 32, 17), (2, 0, 32, 17),
                (3, 0, 32, 17), (4, 0, 32, 17), (5, 0, 32, 17),
                (1, 1, 32, 17), (2, 7, 32, 17)]
    for mode in range(6):
        for chunk in [0, 1, 2, 3, 7, 8, 16, 31, 64]:
            for first, second in [(0, 0), (1, 0), (0, 1), (1, 1), (3, 2),
                    (32, 17), (63, 65), (255, 256), (4096, 17), (16384, 3)]:
                requests.append((mode, chunk, first, second))
    assert len(requests) == 548
    return requests


def extra_encoder_requests():
    requests, metadata = [], []
    payload = b"first plain frame compressed data and dictionary transforms. " * 100
    for q in [0, 4, 11]:
        for w in [10, 16, 22, 25, 30]:
            large = int(w > 24)
            operations = [(0, payload[:91]), (1, payload[91:1000]),
                (3, b"metadata with callback switch"), (0, payload[1000:2000]),
                (3, bytes(range(255))), (2, payload[2000:])]
            request = f"encode_ops {q} {w} 0 {large} 0 {len(operations)} "
            request += " ".join(f"{op} {data.hex() or '-'}" for op, data in operations)
            requests.append(request)
            metadata.append(("mixed", large, []))
    for q in [4, 9, 11]:
        for count in [1, 2, 5, 15]:
            dictionaries = [bytes((i * 97 + n * 33 + i // 7) & 255
                            for i in range(1024)) for n in range(count)]
            prefix = b"".join(dictionaries)
            data = prefix[-800:] + b" hello original compound dictionary " + prefix[-500:]
            request = f"encode_ops {q} 16 0 0 {count} "
            request += " ".join(d.hex() for d in dictionaries) + f" 1 2 {data.hex()}"
            requests.append(request)
            metadata.append(("dict", 0, dictionaries))
    return requests, metadata


def decoder_requests(streams, metadata_streams, extras):
    assert len(streams) == 315 and len(extras) == 27
    requests = [f"stream {ci} {co} {disable} 0 {stream}" for stream in streams
        for ci, co in [(1000000, 1000000), (1, 1), (2, 7), (7, 2),
                       (31, 511), (1024, 17)] for disable in [0, 1]]
    assert len(requests) == 3780
    starts = {"initial": [0, len(requests)]}
    extended = [f"state 1024 4096 0 0 {stream}" for stream in streams]
    for stream in streams[::9]:
        for fail in range(1, 16):
            for seed in [0, 165, 255]:
                extended.append(f"alloc 7 64 0 0 {fail} {seed} {stream}")
    for stream in streams[::6]:
        data = bytes.fromhex(stream)
        for n in sorted({0, 1, 2, 3, 4, len(data)//2, len(data)-2, len(data)-1} - {-1, -2}):
            extended.append(f"state 7 64 0 0 {data[:n].hex() or '-'}")
        positions = set(range(min(len(data), 12))) | set(range(max(0, len(data)-4), len(data)))
        for n in sorted(positions):
            for mask in [1, 2, 4, 8, 16, 32, 64, 128, 255]:
                changed = data[:n] + bytes([data[n] ^ mask]) + data[n+1:]
                extended.append(f"state 7 64 0 0 {changed.hex()}")
    for mode, chunk, first, second in metadata_parameters():
        extended.append(f"callbacks {mode} {chunk} {metadata_streams[first, second]}")
    assert len(extended) == 8542
    starts["extended"] = [len(requests), len(requests) + len(extended)]
    requests.extend(extended)
    _, descriptors = extra_encoder_requests()
    additional = []
    for stream, (kind, large, dictionaries) in zip(extras, descriptors):
        for ci, co in [(1, 1), (2, 7), (7, 2), (4096, 4096)]:
            if kind == "dict":
                base = f"{len(dictionaries)} " + " ".join(d.hex() for d in dictionaries) + " " + stream
                additional.append(f"dict {ci} {co} 0 {large} {base}")
            else:
                additional.append(f"state {ci} {co} 0 {large} {stream}")
                for mode in range(6):
                    additional.append(f"callbacks {mode} {ci} {stream}")
        if kind == "dict":
            for fail in range(1, 18):
                additional.append(f"dictalloc 7 64 0 0 {fail} 165 {base}")
    for stream in streams[::7]:
        for ci in [1, 7, 65536]:
            for co in [0, 1, 7, 65536]:
                additional.append(f"take {ci} {co} 0 0 {stream}")
    additional.append("entry " + streams[2])
    assert len(additional) == 1213
    starts["additional"] = [len(requests), len(requests)+len(additional)]
    requests.extend(additional)
    assert len(requests) == 13535
    lifecycle=[]
    for stream in streams:
        lifecycle.append(f"swap 2 7 0 0 {stream}")
    for stream in streams[::9]:
        lifecycle.append(f"replay 1024 4096 0 0 {stream}")
        data=bytes.fromhex(stream)
        lifecycle.append(f"replay 7 64 0 0 {data[:1].hex()}")
    starts["lifecycle"]=[len(requests),len(requests)+len(lifecycle)]
    requests.extend(lifecycle)
    assert len(lifecycle)==385 and len(requests)==13920
    return requests, starts
