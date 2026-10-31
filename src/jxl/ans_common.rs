// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/ans_common.{h,cc}`.
//! Keeps alias-table construction order, packed entries, and word-based lookup.
//! Output slices replace caller-owned pointers. Prefetch uses the original
//! cache hint, including its disabled branch in the reference configuration.

use super::ans_params::{ANS_LOG_TAB_SIZE, ANS_TAB_SIZE};
use super::base::status::{Status, StatusCode};

pub fn get_population_count_precision(logcount: u32, shift: u32) -> u32 {
    let r = (logcount as i32)
        .min((shift as i32).wrapping_sub(((ANS_LOG_TAB_SIZE.wrapping_sub(logcount)) >> 1) as i32));
    if r < 0 { 0 } else { r as u32 }
}

pub fn create_flat_histogram(length: i32, total_count: i32) -> Vec<i32> {
    assert!(length > 0 && length <= total_count);
    let count = total_count / length;
    let mut result = vec![count; length as usize];
    let rem_counts = total_count % length;
    for i in 0..rem_counts {
        result[i as usize] += 1;
    }
    result
}

pub struct AliasTable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Symbol {
    pub value: usize,
    pub offset: usize,
    pub freq: usize,
}

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
pub struct Entry {
    pub cutoff: u8,
    pub right_value: u8,
    pub freq0: u16,
    pub offsets1: u16,
    pub freq1_xor_freq0: u16,
}

const _: () = assert!(std::mem::size_of::<Entry>() == 8);

impl AliasTable {
    pub fn prefetch(table: &[Entry], value: usize, log_entry_size: usize) {
        super::base::cache_control::prefetch(&table[value >> log_entry_size]);
    }

    pub fn lookup(
        table: &[Entry],
        value: usize,
        log_entry_size: usize,
        entry_size_minus_1: usize,
    ) -> Symbol {
        let i = value >> log_entry_size;
        let pos = value & entry_size_minus_1;
        #[cfg(target_endian = "little")]
        let (cutoff, right_value, freq0, offsets1_or_0, freq1_xor_freq0_or_0) = {
            let entry_ref = &table[i];
            // SAFETY: repr(C, packed) gives Entry exactly eight initialized
            // bytes with no padding. The indexed shared borrow is live and
            // bounds-checked. read_unaligned permits its alignment of one and
            // copies the bits without creating a misaligned u64 reference.
            let entry =
                unsafe { std::ptr::read_unaligned((entry_ref as *const Entry).cast::<u64>()) };
            let cutoff = (entry & 0xFF) as usize;
            let right_value = ((entry >> 8) & 0xFF) as usize;
            let freq0 = ((entry >> 16) & 0xFFFF) as usize;
            let greater = pos >= cutoff;
            let conditional = if greater { entry } else { 0 };
            (
                cutoff,
                right_value,
                freq0,
                ((conditional >> 32) & 0xFFFF) as usize,
                (conditional >> 48) as usize,
            )
        };
        #[cfg(not(target_endian = "little"))]
        let (cutoff, right_value, freq0, offsets1_or_0, freq1_xor_freq0_or_0) = {
            let entry = table[i];
            let greater = pos >= usize::from(entry.cutoff);
            (
                usize::from(entry.cutoff),
                usize::from(entry.right_value),
                usize::from(entry.freq0),
                if greater {
                    usize::from(entry.offsets1)
                } else {
                    0
                },
                if greater {
                    usize::from(entry.freq1_xor_freq0)
                } else {
                    0
                },
            )
        };
        let greater = pos >= cutoff;
        Symbol {
            value: if greater { right_value } else { i },
            offset: offsets1_or_0 + pos,
            freq: freq0 ^ freq1_xor_freq0_or_0,
        }
    }
}

pub fn init_alias_table(
    mut distribution: Vec<i32>,
    log_range: u32,
    log_alpha_size: usize,
    a: &mut [Entry],
) -> Status {
    assert!(log_range < 32 && log_alpha_size < usize::BITS as usize);
    let range = 1u32 << log_range;
    let table_size = 1usize << log_alpha_size;
    if table_size > range as usize {
        return Err(StatusCode::GenericError);
    }
    assert!(a.len() >= table_size);
    while distribution.last() == Some(&0) {
        distribution.pop();
    }
    if distribution.is_empty() {
        distribution.push(range as i32);
    }
    if distribution.len() > table_size {
        return Err(StatusCode::GenericError);
    }
    let entry_size = range >> log_alpha_size;
    let mut single_symbol: i32 = -1;
    let mut sum: i32 = 0;
    for (sym, &v) in distribution.iter().enumerate() {
        sum = sum.wrapping_add(v);
        if v == ANS_TAB_SIZE as i32 {
            if single_symbol != -1 {
                return Err(StatusCode::GenericError);
            }
            single_symbol = sym as i32;
        }
    }
    if sum as u32 != range {
        return Err(StatusCode::GenericError);
    }
    if single_symbol != -1 {
        let sym = single_symbol as u8;
        if single_symbol != i32::from(sym) {
            return Err(StatusCode::GenericError);
        }
        for (i, entry) in a[..table_size].iter_mut().enumerate() {
            entry.right_value = sym;
            entry.cutoff = 0;
            entry.offsets1 = (entry_size as usize * i) as u16;
            entry.freq0 = 0;
            entry.freq1_xor_freq0 = ANS_TAB_SIZE as u16;
        }
        return Ok(());
    }
    let mut underfull_posn = Vec::new();
    let mut overfull_posn = Vec::new();
    let mut cutoffs = vec![0u32; table_size];
    for i in 0..distribution.len() {
        cutoffs[i] = distribution[i] as u32;
        if cutoffs[i] > entry_size {
            overfull_posn.push(i as u32);
        } else if cutoffs[i] < entry_size {
            underfull_posn.push(i as u32);
        }
    }
    for (i, cutoff) in cutoffs
        .iter_mut()
        .enumerate()
        .take(table_size)
        .skip(distribution.len())
    {
        *cutoff = 0;
        underfull_posn.push(i as u32);
    }
    while let Some(overfull_i) = overfull_posn.pop() {
        let Some(underfull_i) = underfull_posn.pop() else {
            return Err(StatusCode::GenericError);
        };
        let overfull_i = overfull_i as usize;
        let underfull_i = underfull_i as usize;
        let underfull_by = entry_size - cutoffs[underfull_i];
        cutoffs[overfull_i] = cutoffs[overfull_i].wrapping_sub(underfull_by);
        a[underfull_i].right_value = overfull_i as u8;
        a[underfull_i].offsets1 = cutoffs[overfull_i] as u16;
        if cutoffs[overfull_i] < entry_size {
            underfull_posn.push(overfull_i as u32);
        } else if cutoffs[overfull_i] > entry_size {
            overfull_posn.push(overfull_i as u32);
        }
    }
    for i in 0..table_size {
        if cutoffs[i] == entry_size {
            a[i].right_value = i as u8;
            a[i].offsets1 = 0;
            a[i].cutoff = 0;
        } else {
            a[i].offsets1 = a[i].offsets1.wrapping_sub(cutoffs[i] as u16);
            a[i].cutoff = cutoffs[i] as u8;
        }
        let freq0 = if i < distribution.len() {
            distribution[i] as usize
        } else {
            0
        };
        let i1 = usize::from(a[i].right_value);
        let freq1 = if i1 < distribution.len() {
            distribution[i1] as usize
        } else {
            0
        };
        a[i].freq0 = freq0 as u16;
        a[i].freq1_xor_freq0 = (freq1 ^ freq0) as u16;
    }
    Ok(())
}
