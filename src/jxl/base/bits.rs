// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/base/bits.h`.
//! Rust's unsigned native operations replace platform compiler intrinsics and
//! SizeTag dispatch. Nonzero variants retain their caller precondition.

pub trait UnsignedWord: Copy {
    const BITS: usize;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn as_u64(self) -> u64;
}

macro_rules! unsigned_word {
    ($t:ty) => {
        impl UnsignedWord for $t {
            const BITS: usize = <$t>::BITS as usize;
            fn leading_zeros(self) -> u32 {
                <$t>::leading_zeros(self)
            }
            fn trailing_zeros(self) -> u32 {
                <$t>::trailing_zeros(self)
            }
            fn as_u64(self) -> u64 {
                self as u64
            }
        }
    };
}
unsigned_word!(u32);
unsigned_word!(u64);
unsigned_word!(usize);

pub fn num0_bits_above_ms1_bit_nonzero<T: UnsignedWord>(x: T) -> usize {
    assert_ne!(x.as_u64(), 0);
    x.leading_zeros() as usize
}

pub fn num0_bits_below_ls1_bit_nonzero<T: UnsignedWord>(x: T) -> usize {
    assert_ne!(x.as_u64(), 0);
    x.trailing_zeros() as usize
}

pub fn num0_bits_above_ms1_bit<T: UnsignedWord>(x: T) -> usize {
    if x.as_u64() == 0 {
        T::BITS
    } else {
        num0_bits_above_ms1_bit_nonzero(x)
    }
}

pub fn num0_bits_below_ls1_bit<T: UnsignedWord>(x: T) -> usize {
    if x.as_u64() == 0 {
        T::BITS
    } else {
        num0_bits_below_ls1_bit_nonzero(x)
    }
}

pub fn floor_log2_nonzero<T: UnsignedWord>(x: T) -> usize {
    (T::BITS - 1) ^ num0_bits_above_ms1_bit_nonzero(x)
}

pub fn ceil_log2_nonzero<T: UnsignedWord>(x: T) -> usize {
    let floor_log2 = floor_log2_nonzero(x);
    let value = x.as_u64();
    if value & (value - 1) == 0 {
        floor_log2
    } else {
        floor_log2 + 1
    }
}
