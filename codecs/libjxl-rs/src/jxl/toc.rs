// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Section table reading/group offsets from lib/jxl/toc.{h,cc}.
//! Owned vectors retain the source's resize, partial-state, and reorder order.

use super::base::status::{Status, StatusCode};
use super::coeff_order::decode_permutation;
use super::coeff_order_fwd::CoeffOrder;
use super::dec_bit_reader::BitReader;
use super::field_encodings::{U32Enc, bits, bits_offset};
use super::fields::u32_coder;
use super::memory_manager_internal::MemoryManagerPtr;
pub const TOC_DIST: U32Enc = U32Enc::new(
    bits(10),
    bits_offset(14, 1024),
    bits_offset(22, 17408),
    bits_offset(30, 4211712),
);
pub fn max_bits(num_sizes: usize) -> usize {
    let entry_bits = u32_coder::max_encoded_bits(TOC_DIST).wrapping_mul(num_sizes);
    1usize
        .wrapping_add(8)
        .wrapping_add(entry_bits)
        .wrapping_add(8)
}
pub const fn ac_group_index(
    pass: usize,
    group: usize,
    num_groups: usize,
    num_dc_groups: usize,
) -> usize {
    2usize
        .wrapping_add(num_dc_groups)
        .wrapping_add(pass.wrapping_mul(num_groups))
        .wrapping_add(group)
}
pub const fn num_toc_entries(num_groups: usize, num_dc_groups: usize, num_passes: usize) -> usize {
    if num_groups == 1 && num_passes == 1 {
        return 1;
    }
    ac_group_index(0, 0, num_groups, num_dc_groups)
        .wrapping_add(num_groups.wrapping_mul(num_passes))
}
fn check_bit_budget(reader: &BitReader<'_>, entries: usize) -> Status {
    let minimal = entries.wrapping_mul(2 + 10);
    let budget = reader.total_bytes().wrapping_mul(8);
    let spent = reader.total_bits_consumed();
    if spent <= budget && minimal <= budget - spent {
        Ok(())
    } else {
        Err(StatusCode::NotEnoughBytes)
    }
}
pub fn read_toc(
    memory_manager: MemoryManagerPtr,
    entries: usize,
    reader: &mut BitReader<'_>,
    sizes: &mut Vec<u32>,
    permutation: &mut Vec<CoeffOrder>,
) -> Status {
    if entries > 65536 {
        return Err(StatusCode::GenericError);
    }
    sizes.clear();
    sizes.resize(entries, 0);
    if reader.total_bits_consumed() >= reader.total_bytes().wrapping_mul(8) {
        return Err(StatusCode::NotEnoughBytes);
    }
    if entries == 0 {
        return Err(StatusCode::GenericError);
    }
    if reader.read_fixed_bits::<1>() == 1 {
        check_bit_budget(reader, entries)?;
        permutation.resize(entries, 0);
        decode_permutation(memory_manager, 0, entries, Some(permutation), reader)?;
    }
    reader.jump_to_byte_boundary()?;
    check_bit_budget(reader, entries)?;
    for size in sizes {
        *size = u32_coder::read(TOC_DIST, reader);
    }
    reader.jump_to_byte_boundary()?;
    check_bit_budget(reader, 0)
}
pub fn read_group_offsets(
    memory_manager: MemoryManagerPtr,
    entries: usize,
    reader: &mut BitReader<'_>,
    offsets: &mut Vec<u64>,
    sizes: &mut Vec<u32>,
    total_size: Option<&mut u64>,
) -> Status {
    let mut permutation = Vec::new();
    read_toc(memory_manager, entries, reader, sizes, &mut permutation)?;
    offsets.clear();
    offsets.resize(entries, 0);
    let mut offset = 0u64;
    for i in 0..entries {
        let next = offset.wrapping_add(u64::from(sizes[i]));
        if next < offset {
            return Err(StatusCode::GenericError);
        }
        offsets[i] = offset;
        offset = next;
    }
    if let Some(total) = total_size {
        *total = offset;
    }
    if !permutation.is_empty() {
        let mut permuted_offsets = Vec::with_capacity(entries);
        let mut permuted_sizes = Vec::with_capacity(entries);
        for index in permutation {
            permuted_offsets.push(offsets[index as usize]);
            permuted_sizes.push(sizes[index as usize]);
        }
        std::mem::swap(offsets, &mut permuted_offsets);
        std::mem::swap(sizes, &mut permuted_sizes);
    }
    Ok(())
}
