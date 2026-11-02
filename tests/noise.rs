// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_noise::{bits_to_float, decode_noise};
use jxl::jxl::noise::NoiseParams;
use jxl::jxl::xorshift128plus::Xorshift128Plus;

#[test]
fn noise_four_seed_schedule_retains_independently_captured_original_words() {
    // Captured from pristine scalar Xorshift128Plus in the original oracle.
    let mut rng = Xorshift128Plus::from_seeds(0, 0, 0, 0);
    assert_eq!(
        rng.fill(),
        [
            14141672759607663454,
            10395157097929615742,
            7493171373717254342,
            3652224411983061744,
            12275406123174129834,
            12789623816865734746,
            12135240342936857922,
            13614302350257308844
        ]
    );
    assert_eq!(
        rng.fill(),
        [
            18288375065211622137,
            6440154974451639113,
            2297306774695940661,
            14575107651750102907,
            10143699702191346209,
            2112481154525151217,
            9552723461997081379,
            5957486393689989887
        ]
    );
}

#[test]
fn noise_decode_retains_original_deferred_bounds_failure() {
    let mut reader = BitReader::new(&[255]);
    let mut params = NoiseParams { lut: [17.0; 8] };
    assert_eq!(decode_noise(&mut reader, &mut params), Ok(()));
    assert_eq!(reader.total_bits_consumed(), 80);
    assert_eq!(
        params.lut,
        [255.0 / 1024.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    );
    assert!(reader.all_reads_within_bounds().is_err());
    // The prior bounds check records the failure; original Close does not
    // report the same checked overread twice.
    assert_eq!(reader.close(), Ok(()));
    assert_eq!(bits_to_float(0).to_bits(), 0x3f800000);
    assert_eq!(bits_to_float(u32::MAX).to_bits(), 0x3fffffff);
}
