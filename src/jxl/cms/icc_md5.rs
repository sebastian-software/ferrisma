// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ICCComputeMD5 from `lib/jxl/cms/jxl_cms_internal.h`. Integer overflow in
//! this source algorithm is deliberate; no cryptographic API is introduced.

pub fn icc_compute_md5(data: &[u8]) -> [u8; 16] {
    let mut padded = data.to_vec();
    padded.push(128);
    let extra = (64 - ((padded.len() + 8) & 63)) & 63;
    padded.resize(padded.len() + extra, 0);
    for i in (0..64).step_by(8) {
        padded.push(((data.len().wrapping_shl(3) as u64) >> i) as u8);
    }
    const SINE: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    const SHIFT: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let (mut a0, mut b0, mut c0, mut d0) =
        (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    for block in padded.chunks_exact(64) {
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for j in 0..64 {
            let (f, g) = if j < 16 {
                ((b & c) | (!b & d), j)
            } else if j < 32 {
                ((d & b) | (!d & c), (5 * j + 1) & 15)
            } else if j < 48 {
                (b ^ c ^ d, (3 * j + 5) & 15)
            } else {
                (c ^ (b | !d), (7 * j) & 15)
            };
            let u = u32::from_le_bytes(block[g * 4..g * 4 + 4].try_into().unwrap());
            let f = f.wrapping_add(a).wrapping_add(SINE[j]).wrapping_add(u);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(SHIFT[j]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut sum = [0; 16];
    for (out, val) in sum.chunks_exact_mut(4).zip([a0, b0, c0, d0]) {
        out.copy_from_slice(&val.to_le_bytes());
    }
    sum
}
