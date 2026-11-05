// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::coeff_order::{
    COEFF_ORDER_MAX_SIZE, STRATEGY_ORDER, coeff_order_offset, decode_coeff_orders,
};
use jxl::jxl::dec_bit_reader::BitReader;

#[test]
fn absent_custom_orders_fill_only_selected_strategy_bucket() {
    for (raw, &ord) in STRATEGY_ORDER.iter().enumerate() {
        let mut output = vec![u32::MAX; COEFF_ORDER_MAX_SIZE + 4];
        let mut reader = BitReader::new(&[]);
        decode_coeff_orders(
            jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
            0,
            1 << raw,
            &mut output,
            &mut reader,
        )
        .unwrap();
        assert_eq!(reader.total_bits_consumed(), 0);
        assert!(reader.close().is_ok());
        // Aliased strategies use the first representative of the order bucket.
        let first = STRATEGY_ORDER.iter().position(|&x| x == ord).unwrap();
        let strategy = AcStrategy::from_raw_strategy(AcStrategyType(first as u32));
        let size = 64 * strategy.covered_blocks_x() * strategy.covered_blocks_y();
        let mut natural = vec![0; size];
        strategy.compute_natural_coeff_order(&mut natural);
        for c in 0..3 {
            let offset = coeff_order_offset(ord as usize, c);
            assert_eq!(&output[offset..offset + size], natural);
        }
        let start = coeff_order_offset(ord as usize, 0);
        let end = coeff_order_offset(ord as usize, 3);
        assert!(output[..start].iter().all(|&v| v == u32::MAX));
        assert!(output[end..].iter().all(|&v| v == u32::MAX));
    }
}

#[test]
fn no_selected_strategy_needs_no_output_storage_or_input() {
    let mut reader = BitReader::new(&[]);
    decode_coeff_orders(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        0,
        0xf800_0000,
        &mut [],
        &mut reader,
    )
    .unwrap();
    assert_eq!(reader.total_bits_consumed(), 0);
    assert!(reader.close().is_ok());
}

#[test]
fn captured_original_custom_order_stream_decodes_all_three_channels() {
    // Original EncodeCoeffOrders(mask=1, adjacent AC pairs swapped, prefix=3).
    let bytes = [
        0x37, 0xc1, 0x50, 0x3a, 0xac, 0x1e, 0x0b, 0xe8, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x80, 0x00, 0x00, 0x00, 0x00,
    ];
    let mut reader = BitReader::new(&bytes);
    reader.skip_bits(3);
    let mut output = vec![u32::MAX; COEFF_ORDER_MAX_SIZE + 4];
    decode_coeff_orders(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        1,
        1,
        &mut output,
        &mut reader,
    )
    .unwrap();
    assert_eq!(reader.total_bits_consumed(), 158);
    reader.close().unwrap();
    let mut expected = [0; 64];
    AcStrategy::from_raw_strategy(AcStrategyType(0)).compute_natural_coeff_order(&mut expected);
    for i in (1..63).step_by(2) {
        expected.swap(i, i + 1);
    }
    for c in 0..3 {
        assert_eq!(&output[64 * c..64 * (c + 1)], expected);
    }
    assert!(output[192..].iter().all(|&v| v == u32::MAX));
}

#[test]
fn packed_channel_boundaries_cover_exactly_the_original_storage() {
    let mut end = 0;
    for ord in 0..13 {
        assert_eq!(coeff_order_offset(ord, 0), end);
        let n = coeff_order_offset(ord, 1) - end;
        for c in 0..3 {
            assert_eq!(
                coeff_order_offset(ord, c + 1) - coeff_order_offset(ord, c),
                n
            );
        }
        end = coeff_order_offset(ord, 3);
    }
    assert_eq!(end, COEFF_ORDER_MAX_SIZE);
}
