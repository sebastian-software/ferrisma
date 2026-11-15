"""PPM (P6) to PAM RGB_ALPHA with a deterministic alpha pattern:
alpha(x, y) = (13 * x + 5 * y) % 256, column 0 fully transparent."""
import sys
data = open(sys.argv[1], "rb").read()
parts = data.split(b"\n", 3)
w, h = map(int, parts[1].split())
assert parts[0] == b"P6" and parts[2] == b"255"
rgb = parts[3]
assert len(rgb) == w * h * 3
out = bytearray(w * h * 4)
base = bytes((13 * i) % 256 for i in range(w + 256))
for y in range(h):
    row = rgb[y * w * 3:(y + 1) * w * 3]
    o = y * w * 4
    seg = out[o:o + w * 4]
    seg[0::4] = row[0::3]
    seg[1::4] = row[1::3]
    seg[2::4] = row[2::3]
    s = (5 * y) % 256
    a = bytearray(base[s:s + w])
    a[0] = 0
    seg[3::4] = a
    out[o:o + w * 4] = seg
head = f"P7\nWIDTH {w}\nHEIGHT {h}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n".encode()
open(sys.argv[2], "wb").write(head + out)
