// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar Highway specialization of `lib/jxl/inverse_mtf-inl.h` (one lane).
//! Optimized SIMD specializations remain unported.

fn move_to_front(v: &mut [u8], index: u8) {
    let value = v[usize::from(index)];
    let mut i = usize::from(index);
    if i < 4 {
        while i != 0 {
            v[i] = v[i - 1];
            i -= 1;
        }
    } else {
        // With one Highway lane, tail is zero and each vector move copies one
        // byte. Keep the source's descending traversal and branch boundary.
        while i != 0 {
            i -= 1;
            v[i + 1] = v[i];
        }
    }
    v[0] = value;
}

pub fn inverse_move_to_front_transform(v: &mut [u8]) {
    let mut mtf = [0u8; 256 + 64];
    for (i, value) in mtf[..256].iter_mut().enumerate() {
        *value = i as u8;
    }
    for value in v {
        let index = *value;
        *value = mtf[usize::from(index)];
        if index != 0 {
            move_to_front(&mut mtf, index);
        }
    }
}
