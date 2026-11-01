// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Defined-header state, signed-rounding, union-storage, and decoded-prefix
// regressions. The prefix replay is test infrastructure, not an image codec.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::fields::bundle;
use jxl::jxl::image::Plane;
use jxl::jxl::modular::encoding::context_predict::*;
use jxl::jxl::modular::options::Predictor;

#[test]
fn failed_weighted_parameter_read_keeps_the_local_parameter_until_success() {
    let mut h = weighted::Header::default();
    weighted::predictor_mode(1, &mut h);
    let bytes = [0];
    let mut br = BitReader::new(&bytes);
    assert_eq!(
        bundle::read(&mut br, &mut h),
        Err(StatusCode::NotEnoughBytes)
    );
    assert_eq!(h.p1_c, 0);
    assert_eq!(h.p2_c, 8);
    assert_eq!(br.total_bits_consumed(), 11);
    br.close().unwrap();
}
#[test]
fn signed_rounding_ties_and_flat_union_storage_keep_source_meanings() {
    assert_eq!(
        detail::predict_one(Predictor::AVERAGE0, -1, 0, 0, 0, 0, 0, 0, 0),
        0
    );
    assert_eq!(
        detail::predict_one(Predictor::AVERAGE4, -2, 0, 0, 0, 0, 0, 0, 0),
        0
    );
    assert_eq!(select(2, 4, 3), 4);
    assert_eq!(clamped_gradient(i32::MAX, i32::MAX, i32::MIN), i32::MAX);
    assert_eq!(clamped_gradient(i32::MIN, i32::MIN, i32::MAX), i32::MIN);
    let mut node = FlatDecisionNode::new(-1, 23);
    node.set_predictor(Predictor::WEIGHTED);
    node.set_multiplier(7);
    node.set_predictor_offset(-19);
    let nodes = [node];
    let r = MATreeLookup::new(&nodes).lookup(&[]);
    assert_eq!(
        (r.context, r.predictor, r.offset, r.multiplier),
        (23, Predictor::WEIGHTED, -19, 7)
    );
    node.set_property(0, -123);
    node.set_property(1, 321);
    assert_eq!(node.properties(), [-123, 321]);
    assert_eq!(std::mem::size_of::<FlatDecisionNode>(), 24);
    let header = weighted::Header::default();
    let state = weighted::State::new(&header, 1, 1);
    assert_eq!(state.error_weight(0, 15), 251658244);
    assert_eq!(state.weighted_average(&[-1; 4], [4; 4]), -1);
}
#[test]
fn raster_neighbors_and_weighted_state_replay_on_only_the_decoded_prefix() {
    for predictor in [
        Predictor::ZERO,
        Predictor::GRADIENT,
        Predictor::WEIGHTED,
        Predictor::AVERAGE4,
    ] {
        let mut input = Plane::<i32>::create(21, 4).unwrap();
        let mut decoded = Plane::<i32>::create(21, 4).unwrap();
        for y in 0..4 {
            for x in 0..21 {
                input.row_mut(y)[x] = ((x * 947 + y * 7919) % 65536) as i32 - 32768;
            }
        }
        let header = weighted::Header::default();
        let mut encoder = weighted::State::new(&header, 21, 4);
        let mut decoder = weighted::State::new(&header, 21, 4);
        let stride = input.pixels_per_row();
        for y in 0..4 {
            for x in 0..21 {
                let pos = y * stride as usize + x;
                let a = predict_no_tree_wp(
                    21,
                    PixelPosition::new(input.pixels().unwrap(), pos, stride),
                    x as i32,
                    y as i32,
                    predictor,
                    &mut encoder,
                );
                let b = predict_no_tree_wp(
                    21,
                    PixelPosition::new(decoded.pixels().unwrap(), pos, stride),
                    x as i32,
                    y as i32,
                    predictor,
                    &mut decoder,
                );
                assert_eq!(a.guess, b.guess);
                let value = i64::from(input.row(y)[x]);
                let residual = value - a.guess;
                decoded.row_mut(y)[x] = (b.guess + residual) as i32;
                encoder.update_errors(value, x, y, 21);
                decoder.update_errors(i64::from(decoded.row(y)[x]), x, y, 21);
                assert_eq!(encoder.pred_errors, decoder.pred_errors);
                assert_eq!(encoder.error, decoder.error);
            }
        }
        for y in 0..4 {
            assert_eq!(&input.row(y)[..21], &decoded.row(y)[..21]);
        }
    }
}
