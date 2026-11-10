#!/usr/bin/env python3
"""Retained actual API/defined size_t-alias and callback schedules."""

def requests(streams, metadata_stream, mixed_streams):
    assert len(streams) == 315 and len(mixed_streams) == 3
    rows = [f"api {mode} {first} {size} {stream}" for stream in streams[::9]
            for mode in range(4) for first in [0, 1, 7, 64]
            for size in [0, 1, 7, 64, 18446744073709551615]]
    rows += [f"attach {ty} {kind} {size} {count} {streams[0]}"
             for ty in [0, 1, 2, 4294967295] for kind in [0, 1]
             for size in [0, 1, 6, 256] for count in [0, 1, 2, 14, 15, 16, 17]]
    assert len(rows) == 3024
    rows += [f"watch {mode} {chunk} {stream}" for stream in [metadata_stream, *mixed_streams]
             for mode in range(3) for chunk in [0, 1, 2, 7, 64]]
    assert len(rows) == 3084
    return rows
