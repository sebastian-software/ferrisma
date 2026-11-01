// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Options and predictor values from lib/jxl/modular/options.h.
//! Predictor is raw storage because serialized invalid values remain observable.

use crate::jxl::enc_ans_params::HistogramParams;
pub type PropertyVal = i32;
pub type Properties = Vec<PropertyVal>;
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Predictor(pub u32);
impl Predictor {
    pub const ZERO: Self = Self(0);
    pub const LEFT: Self = Self(1);
    pub const TOP: Self = Self(2);
    pub const AVERAGE0: Self = Self(3);
    pub const SELECT: Self = Self(4);
    pub const GRADIENT: Self = Self(5);
    pub const WEIGHTED: Self = Self(6);
    pub const TOP_RIGHT: Self = Self(7);
    pub const TOP_LEFT: Self = Self(8);
    pub const LEFT_LEFT: Self = Self(9);
    pub const AVERAGE1: Self = Self(10);
    pub const AVERAGE2: Self = Self(11);
    pub const AVERAGE3: Self = Self(12);
    pub const AVERAGE4: Self = Self(13);
    pub const BEST: Self = Self(14);
    pub const VARIABLE: Self = Self(15);
}
pub const UNDEFINED_PREDICTOR: Predictor = Predictor(u32::MAX);
pub const NUM_MODULAR_PREDICTORS: usize = 14;
pub const NUM_MODULAR_ENCODER_PREDICTORS: usize = 16;
pub const NUM_STATIC_PROPERTIES: isize = 2;
pub type StaticPropRange = [[u32; 2]; NUM_STATIC_PROPERTIES as usize];
pub struct ModularMultiplierInfo {
    pub range: StaticPropRange,
    pub multiplier: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum TreeMode {
    GradientOnly,
    WPOnly,
    NoWP,
    Default,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum TreeKind {
    TrivialTreeNoPredictor,
    Learn,
    JpegTranscodeACMeta,
    FalconACMeta,
    ACMeta,
    WPFixedDC,
    GradientFixedDC,
}
#[derive(Clone)]
pub struct ModularOptions {
    pub max_chan_size: usize,
    pub group_dim: usize,
    pub nb_repeats: f32,
    pub max_properties: i32,
    pub splitting_heuristics_properties: Vec<u32>,
    pub splitting_heuristics_node_threshold: f32,
    pub max_property_values: usize,
    pub predictor: Predictor,
    pub wp_mode: i32,
    pub fast_decode_multiplier: f32,
    pub wp_tree_mode: TreeMode,
    pub skip_encoder_fast_path: bool,
    pub tree_kind: TreeKind,
    pub histogram_params: HistogramParams,
    pub zero_tokens: bool,
}
impl Default for ModularOptions {
    fn default() -> Self {
        Self {
            max_chan_size: 0xffffff,
            group_dim: 0x1fffffff,
            nb_repeats: 0.5,
            max_properties: 0,
            splitting_heuristics_properties: vec![0, 1, 15, 9, 10, 11, 12, 13],
            splitting_heuristics_node_threshold: 96.0,
            max_property_values: 32,
            predictor: UNDEFINED_PREDICTOR,
            wp_mode: 0,
            fast_decode_multiplier: 1.01,
            wp_tree_mode: TreeMode::Default,
            skip_encoder_fast_path: false,
            tree_kind: TreeKind::Learn,
            histogram_params: HistogramParams::default(),
            zero_tokens: false,
        }
    }
}
