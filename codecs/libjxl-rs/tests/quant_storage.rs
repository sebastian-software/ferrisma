// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::quant_weights::{
    DctQuantWeightParams, DequantMatrices, QuantEncoding, QuantEncodingData, QuantMode,
};

fn raw_values(q: &QuantEncoding) -> Option<&[i32]> {
    match q.data() {
        QuantEncodingData::Raw { qtable, .. } => qtable.as_deref(),
        _ => panic!("RAW fixture"),
    }
}
#[test]
fn default_matrix_offsets_follow_original_block_sizes_and_start_with_library_encodings() {
    let m = DequantMatrices::default();
    assert_eq!(m.computed_mask, 0);
    assert!(!m.have_table_storage());
    assert_eq!(m.encodings().len(), 17);
    for q in m.encodings() {
        assert_eq!(q.mode(), QuantMode::Library);
        assert_eq!(q.predefined, 0);
    }
    assert_eq!(&m.table_offsets()[..6], &[0, 64, 128, 192, 256, 320]);
    assert_eq!(
        &m.table_offsets()[12..18],
        &[768, 1024, 1280, 1536, 2560, 3584]
    );
    assert_eq!(&m.table_offsets()[78..], &[296448, 329216, 361984]);
    assert_eq!(DequantMatrices::TOTAL_TABLE_SIZE, 394752);
}

#[test]
fn raw_tables_copy_inputs_and_copies_while_moves_leave_only_the_source_owner_empty() {
    let mut input = vec![10, 20, 30];
    let mut q = QuantEncoding::raw(&input, 31);
    input[0] = 99;
    assert_eq!(raw_values(&q), Some([10, 20, 30].as_slice()));
    let mut copy = q.clone();
    q.raw_table_mut().unwrap()[0] = 11;
    assert_eq!(raw_values(&copy), Some([10, 20, 30].as_slice()));
    let mut assigned = QuantEncoding::raw(&[101], 0);
    assigned.clone_from(&copy);
    copy.raw_table_mut().unwrap()[0] = 12;
    assert_eq!(raw_values(&assigned), Some([10, 20, 30].as_slice()));
    let moved = QuantEncoding::move_from(&mut copy);
    assert_eq!(copy.mode(), QuantMode::Raw);
    assert_eq!(raw_values(&copy), None);
    assert_eq!(raw_values(&moved), Some([12, 20, 30].as_slice()));
    assert_eq!(raw_values(&copy.clone()), None);
    if let QuantEncodingData::Raw { qtable_den, .. } = moved.data() {
        assert!(qtable_den.is_sign_negative());
    }
}

#[test]
fn short_distance_band_arrays_copy_the_prefix_and_keep_the_initialized_suffix_zero() {
    let bands = [[1.0, -0.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let p = DctQuantWeightParams::from_bands(&bands);
    assert_eq!(p.num_distance_bands, 3);
    for (row, input) in p.distance_bands.iter().zip(bands) {
        for i in 0..3 {
            assert_eq!(row[i].to_bits(), input[i].to_bits());
        }
        assert!(row[3..].iter().all(|&x| x.to_bits() == 0));
    }
    const PARAMS: DctQuantWeightParams = DctQuantWeightParams::new([[1.0; 17]; 3], 17);
    const ENCODING: QuantEncoding = QuantEncoding::dct4(&PARAMS, &[[2.0; 2]; 3]);
    assert_eq!(ENCODING.mode(), QuantMode::Dct4);
    assert_eq!(ENCODING.dct_params.num_distance_bands, 17);
}
