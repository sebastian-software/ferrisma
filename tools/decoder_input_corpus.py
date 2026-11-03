#!/usr/bin/env python3
"""Source-defined persistent input histories; emits requests, never results."""
from itertools import permutations

def requests():
    for mode in range(5):
        for n in [0,1,7,(1<<64)-1]:
            yield f'di-short {mode} {n}'
    for fail in [0,1,2,3]:
        yield f'di-run {fail} set 0 010203 mutate 0 0 170 request release set 0 aabbcc get release'
    yield 'di-run 0 set 0 010203 get advance 1 get release set 0 02030405 get request release set 0 060708 get advance 7 get release close set 0 -'
    for fail in range(0, 9):
        for first in range(0, 9):
            for consumed in range(first + 5):
                data=bytes(range(first)).hex() or '-'
                rest=bytes(range(9,16)).hex()
                yield f'di-run {fail} set 0 {data} get advance {consumed} request release set 0 {rest} get get request release set 0 aabbcc get advance 1 get release close set 0 -'
    for order in permutations(range(4)):
        for fail in range(0, 10):
            ops=[]
            for i in order:
                payload=bytes([i+1]*(i*23)).hex() or '-'
                ops += [f'queue {i} {int(i==3)} {payload}', 'inject']
            ops+=['inject']*5+['set 0 aabbcc','get','request','release','set 0 -','advance 1000','release','set 0 010203','get','release']
            yield f'di-run {fail} '+ ' '.join(ops)
    for null in [0,1]:
        for close in [False,True]:
            for size in [0,1,7]:
                data=bytes(range(size)).hex() or '-'
                ops=f'set {null} {data} set 0 aabb release set 0 cc release'
                if close:ops='close '+ops
                yield 'di-run 0 '+ops
    for total in [0,1,(1<<48)-1,1<<48,(1<<64)-1]:
        for size in [0,1,(1<<48)-1,1<<48,(1<<64)-1]:
            yield f'di-run 0 seed 0 0 {total} 0 0 limit {size}'
    for pos in range(0,8):
        for unconsumed in range(0,8):
            yield f'di-run 0 queue 0 0 010203 inject seed {pos} {unconsumed} 0 1 0 set 0 040506 get release'
    for end in range(0,8):
        for advance in range(0,11):
            yield f'di-run 0 box 0 {end} set 0 01020304050607 get advance {advance} request release set 0 aabbccdd get release'
    yield 'di-run 0 set 0 010203 get advance 8 release set 0 0405 get-check release set 0 06070809 get-check release'
    yield 'di-run 0 queue 0 0 010203 inject seed 4 0 0 1 0 set 0 aabbcc get-check release'
    yield 'di-run 0 queue 0 0 010203 inject seed 3 0 0 1 0 set 0 - get-check release'
    for fail in [0,1,2,3]:
        data=bytes(range(100)).hex()
        yield f'di-run {fail} set 0 {data} request release set 0 {data} get-check request release'
    yield 'di-run 0 queue 0 0 010203 inject set 0 aabbcc get seed 7 0 0 1 0 get-check release'
    yield 'di-run 0 seed 0 0 0 4294967295 0 queue 4294967295 1 010203 inject inject set 0 - get-check release'

if __name__=='__main__':
    print('\n'.join(requests()))
