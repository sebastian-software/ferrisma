"""Insert an ICC profile as one APP2 ICC_PROFILE segment after SOI.

Usage: python3 -I add_icc.py IN.jpg PROFILE.icc OUT.jpg
"""
import struct
import sys

data = open(sys.argv[1], "rb").read()
icc = open(sys.argv[2], "rb").read()
assert data[:2] == b"\xff\xd8"
payload = b"ICC_PROFILE\x00" + bytes([1, 1]) + icc
assert len(payload) + 2 <= 0xFFFF
segment = b"\xff\xe2" + struct.pack(">H", len(payload) + 2) + payload
open(sys.argv[3], "wb").write(data[:2] + segment + data[2:])
