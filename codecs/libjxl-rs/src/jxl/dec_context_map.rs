// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/dec_context_map.{h,cc}`. Recursive entropy decoding
//! retains the source's LZ77 restriction on small context maps.

use super::base::status::{Status, StatusCode};
use super::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::inverse_mtf::inverse_move_to_front_transform;
use super::memory_manager_internal::MemoryManagerPtr;

fn verify_context_map(context_map: &[u8], num_htrees: usize) -> Status {
    let mut have_htree = vec![false; num_htrees];
    let mut num_found = 0;
    for &htree in context_map {
        let htree = usize::from(htree);
        if htree >= num_htrees {
            return Err(StatusCode::GenericError);
        }
        if !have_htree[htree] {
            have_htree[htree] = true;
            num_found += 1;
        }
    }
    if num_found != num_htrees {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}

pub fn decode_context_map(
    memory_manager: MemoryManagerPtr,
    context_map: &mut [u8],
    num_htrees: &mut usize,
    input: &mut BitReader<'_>,
) -> Status {
    assert!(!context_map.is_empty());
    let is_simple = input.read_fixed_bits::<1>() != 0;
    if is_simple {
        let bits_per_entry = input.read_fixed_bits::<2>() as usize;
        if bits_per_entry != 0 {
            for entry in context_map.iter_mut() {
                *entry = input.read_bits(bits_per_entry) as u8;
            }
        } else {
            context_map.fill(0);
        }
    } else {
        let use_mtf = input.read_fixed_bits::<1>() != 0;
        let mut code = ANSCode::default();
        let mut sink_ctx_map = Vec::new();
        decode_histograms(
            memory_manager,
            input,
            1,
            &mut code,
            &mut sink_ctx_map,
            context_map.len() <= 2,
        )?;
        let mut reader = ANSSymbolReader::create(&code, input, 0)?;
        let mut maxsym = 0;
        for entry in context_map.iter_mut() {
            let sym = reader.read_hybrid_uint_inlined::<true>(0, input, &sink_ctx_map) as u32;
            maxsym = sym.max(maxsym);
            *entry = sym as u8;
        }
        if maxsym >= 256 {
            return Err(StatusCode::GenericError);
        }
        if !reader.check_ans_final_state() {
            return Err(StatusCode::GenericError);
        }
        if use_mtf {
            inverse_move_to_front_transform(context_map);
        }
    }
    *num_htrees = usize::from(*context_map.iter().max().unwrap()) + 1;
    verify_context_map(context_map, *num_htrees)
}
