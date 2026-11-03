// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of c/dec/decode.c:983-1021. The actual source
//! mtf[65] backing is borrowed, including physical byte 3 for mtf_u8[-1].

use super::bit_reader::Reg;
use std::mem::MaybeUninit;

/// # Safety
/// `upper_bound` is initialized and in 0..64. `mtf` is the actual
/// source state field; `v[..v_len]` is initialized and disjoint from both state
/// fields. Every reached list byte is initialized by this call's native-word
/// prefix writes or retained source history. Source StateInit starts bound 63,
/// so its first call initializes all 256 list bytes before any list read.
/// A smaller initial bound is valid only if its reached tail bytes are already
/// initialized (or all input indices stay within the freshly written prefix).
/// Source word 0 bytes 0..2 may remain unknown forever. Byte3 is written before
/// its -1 read. The source's do-while resets at least 2 words even for bound 0.
#[inline(never)]
pub unsafe fn inverse_move_to_front_transform(
    v: &mut [u8],
    v_len: Reg,
    upper_bound: &mut MaybeUninit<Reg>,
    mtf: &mut MaybeUninit<[u32; 65]>,
) {
    let mut i: Reg = 1;
    // SAFETY: source StateInit/previous transform initialized this field.
    let mut bound = unsafe { upper_bound.assume_init_read() };
    let words = mtf.as_mut_ptr().cast::<u32>();
    // Array storage is live/aligned; no scalar payload is read by this view.
    let bytes = words.cast::<MaybeUninit<u8>>();
    let mut pattern = u32::from_ne_bytes([0, 1, 2, 3]);
    // SAFETY: actual mtf[65] contains aligned physical word 1; source store
    // initializes all 4 bytes without reading any prior value or padding.
    unsafe { words.add(1).write(pattern) };
    loop {
        pattern = pattern.wrapping_add(0x04040404);
        // SAFETY: caller bound <= 63 and source do-while i >= 1 ensure actual
        // physical word i+1 is within 65 words; this full native store initializes it.
        unsafe { words.add(i as usize + 1).write(pattern) };
        i += 1;
        if i > bound {
            break;
        }
    }
    bound = 0;
    i = 0;
    while i < v_len {
        let mut index = i32::from(v[i as usize]);
        // SAFETY: source prefix stores or retained history initialized this
        // exact list byte (physical origin4), as required by the entry domain.
        let value = unsafe { bytes.add(4 + index as usize).read().assume_init() };
        bound |= Reg::from(v[i as usize]);
        v[i as usize] = value;
        // SAFETY: source scratch mtf_u8[-1] is physical byte 3 inside actual
        // word 0; this write initializes it before the do-while's -1 read.
        unsafe { bytes.add(3).write(MaybeUninit::new(value)) };
        loop {
            index -= 1;
            // SAFETY: index is now -1..254, so physical source 3..258 and
            // destination 4..259 are inside the real65-word block. The exact
            // source byte is initialized: scratch was just written, otherwise
            // prefix/history or a prior iteration supplies an initialized byte.
            unsafe {
                let value = bytes.offset(4 + index as isize).read().assume_init();
                bytes
                    .offset(5 + index as isize)
                    .write(MaybeUninit::new(value));
            }
            if index < 0 {
                break;
            }
        }
        i += 1;
    }
    upper_bound.write(bound >> 2);
}
