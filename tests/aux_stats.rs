// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::enc_aux_out::AuxOut;

#[test]
fn assimilation_wraps_unsigned_counts_retains_signed_counts_and_adds_negative_zero() {
    let mut a = AuxOut::default();
    let mut b = AuxOut::default();
    a.layers[0].total_bits = usize::MAX - 2;
    b.layers[0].total_bits = 7;
    a.layers[0].clustered_entropy = -0.0;
    b.layers[0].clustered_entropy = -0.0;
    a.layers[14].histogram_bits = 12;
    b.layers[14].histogram_bits = 3;
    a.num_blocks = usize::MAX;
    b.num_blocks = 1;
    a.num_butteraugli_iters = -5;
    b.num_butteraugli_iters = 4;
    a.assimilate(&b);
    assert_eq!(a.total_bits(), 4);
    assert_eq!(a.num_blocks, 0);
    assert_eq!(a.num_butteraugli_iters, -1);
    assert_eq!(a.layers[14].histogram_bits, 15);
    assert_eq!(a.layers[0].clustered_entropy.to_bits(), (-0.0f64).to_bits());
}
