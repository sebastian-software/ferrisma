// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Fenwick-tree permutation coding from lib/jxl/lehmer_code.h.
//! Slice borrows replace pointers. The u16/u32 source instantiations are exposed;
//! caller-provided input, workspace, and output remain disjoint.

use super::base::bits::ceil_log2_nonzero;
use super::base::status::{Status, StatusCode};

pub type LehmerT = u32;
pub trait PermutationWord: Copy {
    fn value(self) -> u32;
    fn from_index(index: usize) -> Self;
}
impl PermutationWord for u16 {
    fn value(self) -> u32 {
        u32::from(self)
    }
    fn from_index(index: usize) -> Self {
        index as Self
    }
}
impl PermutationWord for u32 {
    fn value(self) -> u32 {
        self
    }
    fn from_index(index: usize) -> Self {
        index as Self
    }
}
pub fn compute_lehmer_code<T: PermutationWord>(
    permutation: &[T],
    temp: &mut [u32],
    n: usize,
    code: &mut [LehmerT],
) -> Status {
    for v in &mut temp[..n + 1] {
        *v = 0;
    }
    for idx in 0..n {
        let s = permutation[idx].value();
        let mut penalty = 0u32;
        let mut i = s.wrapping_add(1);
        while i != 0 {
            penalty = penalty.wrapping_add(temp[i as usize]);
            i &= i - 1;
        }
        if s < penalty {
            return Err(StatusCode::GenericError);
        }
        code[idx] = s - penalty;
        i = s.wrapping_add(1);
        while (i as usize) < n + 1 {
            temp[i as usize] = temp[i as usize].wrapping_add(1);
            i = i.wrapping_add(i & i.wrapping_neg());
        }
    }
    Ok(())
}
pub fn decode_lehmer_code<T: PermutationWord>(
    code: &[LehmerT],
    temp: &mut [u32],
    n: usize,
    permutation: &mut [T],
) -> Status {
    if n == 0 {
        return Err(StatusCode::GenericError);
    }
    let log2n = ceil_log2_nonzero(n);
    let padded_n = 1usize << log2n;
    for (i, v) in temp[..padded_n].iter_mut().enumerate() {
        let i1 = (i + 1) as i32;
        *v = (i1 & i1.wrapping_neg()) as u32;
    }
    for i in 0..n {
        if (code[i] as usize).wrapping_add(i) >= n {
            return Err(StatusCode::GenericError);
        }
        let mut rank = code[i].wrapping_add(1);
        let mut bit = padded_n;
        let mut next = 0;
        for _ in 0..=log2n {
            let cand = next + bit;
            if cand < 1 {
                return Err(StatusCode::GenericError);
            }
            bit >>= 1;
            if temp[cand - 1] < rank {
                next = cand;
                rank = rank.wrapping_sub(temp[cand - 1]);
            }
        }
        permutation[i] = T::from_index(next);
        next += 1;
        while next <= padded_n {
            temp[next - 1] = temp[next - 1].wrapping_sub(1);
            next += next & next.wrapping_neg();
        }
    }
    Ok(())
}
