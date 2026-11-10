// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Cases adapted from base/bits_test.cc and byte_order_test.cc.

use jxl::jxl::base::{bits::*, byte_order::*};

#[test]
fn zero_bit_counts_and_machine_word_boundaries() {
    assert_eq!(num0_bits_above_ms1_bit(0u32), 32);
    assert_eq!(num0_bits_above_ms1_bit(0u64), 64);
    assert_eq!(num0_bits_below_ls1_bit(0u32), 32);
    assert_eq!(num0_bits_below_ls1_bit(0u64), 64);
    for i in 0..32 {
        assert_eq!(num0_bits_above_ms1_bit(1u32 << i), 31 - i);
        assert_eq!(num0_bits_below_ls1_bit(1u32 << i), i);
    }
    for i in 0..64 {
        assert_eq!(num0_bits_above_ms1_bit(1u64 << i), 63 - i);
        assert_eq!(num0_bits_below_ls1_bit(1u64 << i), i);
    }
}

#[test]
fn logarithms_match_source_expected_values() {
    for (i, (floor, ceil)) in [(0, 0), (1, 1), (1, 2), (2, 2), (2, 3), (2, 3), (2, 3)]
        .into_iter()
        .enumerate()
    {
        let value = i as u32 + 1;
        assert_eq!(floor_log2_nonzero(value), floor);
        assert_eq!(floor_log2_nonzero(u64::from(value)), floor);
        assert_eq!(ceil_log2_nonzero(value), ceil);
        assert_eq!(ceil_log2_nonzero(u64::from(value)), ceil);
    }
    for (value, floor, ceil) in [
        (4095u64, 11, 12),
        (4096, 12, 12),
        (4097, 12, 13),
        (0x8000_0001, 31, 32),
        (u64::MAX, 63, 64),
    ] {
        assert_eq!(floor_log2_nonzero(value), floor);
        assert_eq!(ceil_log2_nonzero(value), ceil);
    }
}

#[test]
fn byte_order_source_round_trips_and_float_bits() {
    let mut bytes = [0; 10];
    store_be16(0x1234, &mut bytes[1..]);
    assert_eq!(load_be16(&bytes[1..]), 0x1234);
    assert_ne!(load_le16(&bytes[1..]), 0x1234);
    store_le16(0xFFFF_1234, &mut bytes[1..]);
    assert_eq!(load_le16(&bytes[1..]), 0x1234);
    store_be32(0xFEDC_BA98, &mut bytes[1..]);
    assert_eq!(load_be32(&bytes[1..]), 0xFEDC_BA98);
    store_le32(0xFEDC_BA98, &mut bytes[1..]);
    assert_eq!(load_le32(&bytes[1..]), 0xFEDC_BA98);
    store_le64(0xFEDC_BA98_7654_3210, &mut bytes[1..]);
    assert_eq!(load_le64(&bytes[1..]), 0xFEDC_BA98_7654_3210);
    store_be64(0xFEDC_BA98_7654_3210, &mut bytes[1..]);
    assert_eq!(load_be64(&bytes[1..]), 0xFEDC_BA98_7654_3210);
    for bits in [0, 0x8000_0000, 0x7F80_0001, 0x7FFF_FFFF] {
        store_le32(bits, &mut bytes[1..]);
        assert_eq!(load_le_float(&bytes[1..]).to_bits(), bits);
        assert_eq!(
            bswap_float(f32::from_bits(bits)).to_bits(),
            bits.swap_bytes()
        );
    }
}
