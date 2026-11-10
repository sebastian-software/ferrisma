#!/usr/bin/env python3
"""Source fixture generation and streaming schedules for actual box content."""


def encoder_requests():
    payloads = [b'', b'a', b'hello world\n', bytes(range(256)),
                b'compression dictionary transform testing the quick brown fox ' * 37,
                bytes(((i * 73 + (i >> 3) * 29) & 255) for i in range(4096)),
                b'abcd' * 2048]
    return [f"encode {quality} {window} {mode} {data.hex() or '-'}"
            for quality in [0, 4, 11] for window in [10, 22]
            for mode in [0, 2] for data in payloads]


def sequence(data, actions):
    return f"box {data.hex() or '-'} {len(actions)} " + ' '.join(actions)


def start(brob, eof, size):
    return f"S {int(brob)} {int(eof)} {size}"


def process(base, available, position, output):
    return f"P {base} {available} {position} {output}"


def requests(encoded_rows):
    rows = [sequence(b'', [])]
    # Passthrough is independently exercised; EOF means no remaining counter,
    # not an invented requirement for an external end-of-input signal.
    for data in [b'', b'a', b'Exif', bytes(range(256))]:
        n = len(data)
        for eof in [False, True]:
            for size in sorted({0, 1, max(0, n - 1), n, n + 1, (1 << 64) - 1}):
                for output in [0, 1, 7, 257]:
                    actions = [start(False, eof, size)]
                    actions += [process(0, n, 0, output) for _ in range(min(n + 2, 12))]
                    rows.append(sequence(data, actions))
        # Replay/position guards retain every earlier state/output checkpoint.
        rows.append(sequence(data, [start(False, False, n), process(0, n, 1, 7),
                                    process(0, n, 0, 1), process(0, 0, 0, 7),
                                    process(0, n, 0, 257), start(False, True, 0),
                                    process(0, n, 0, 257)]))
    for row in encoded_rows:
        ok, h = row.split()
        assert ok == '1'
        encoded = bytes.fromhex(h)
        data = b'Exif' + encoded
        n = len(data)
        for eof in [False, True]:
            for output in [0, 1, 7, 16384]:
                rows.append(sequence(data, [start(True, eof, n)] +
                                     [process(0, n, 0, output)] * (4 if output else 2)))
            for step in [1, 7, 64]:
                # Bound fixture size without dropping a source observation.
                sizes = sorted(set(list(range(min(n, 12) + 1)) +
                                   list(range(0, n + 1, step if n < 128 else 64)) + [n]))
                actions = [start(True, eof, n)]
                actions += [process(0, size, 0, 16384) for size in sizes]
                rows.append(sequence(data, actions))
        for cut in sorted(set(list(range(min(n, 16) + 1)) + [n // 2, max(0, n - 1), n])):
            truncated = data[:cut]
            for declared in sorted({cut, n}):
                rows.append(sequence(truncated, [start(True, False, declared),
                                                process(0, cut, 0, 16384),
                                                process(0, cut, 0, 16384)]))
        for bit in range(min(len(encoded) * 8, 24)):
            changed = bytearray(data)
            changed[4 + bit // 8] ^= 1 << (bit & 7)
            rows.append(sequence(changed, [start(True, False, n),
                                          process(0, n, 0, 16384),
                                          process(0, n, 0, 1)]))
        # Header bytes are supplied by the container layer and skipped here.
        for header in [b'\0' * 4, b'\xff' * 4]:
            other = header + encoded
            rows.append(sequence(other, [start(True, False, n), process(0, n, 0, 16384)]))
        trailing = data + b'\x00\xff'
        rows.append(sequence(trailing, [start(True, False, n + 2),
                                        process(0, n + 2, 0, 16384),
                                        process(0, n + 2, 0, 1)]))
        # Reset while partial, after success and after an error. All calls stay
        # observable; recovery never replaces or filters their earlier status.
        rows.append(sequence(data, [start(True, False, n), process(0, min(n, 5), 0, 1),
                                    start(True, True, 0), process(0, n, 0, 16384),
                                    start(False, False, n), process(0, n, 0, 16384),
                                    start(True, False, 3), process(0, n, 0, 16384),
                                    start(True, False, n), process(0, n, 0, 16384)]))
    # Too-short declared header is checked only after four input bytes exist.
    for size in range(4):
        rows.append(sequence(b'Exif', [start(True, False, size),
                                      *[process(0, n, 0, 1) for n in range(5)]]))
    return rows
