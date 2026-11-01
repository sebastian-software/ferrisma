// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Predictors, weighted state, properties, and flat-tree lookup from
//! lib/jxl/modular/encoding/context_predict.h. Relative positions in borrowed
//! sample storage replace source row pointers. Raw union slots retain the flat
//! node layout without invalid Rust enum values or inactive-union references.

use super::super::modular_image::{Channel, Image, Pixel, PixelWide};
use super::super::options::{
    NUM_MODULAR_PREDICTORS, NUM_STATIC_PROPERTIES, Predictor, Properties, PropertyVal,
};
use crate::jxl::image_ops::zero_fill_image;

pub mod weighted {
    use super::*;
    use crate::jxl::base::bits::floor_log2_nonzero;
    use crate::jxl::base::status::Status;
    use crate::jxl::field_encodings::Fields;
    use crate::jxl::fields::{Visitor, bundle};
    pub const NUM_PREDICTORS: usize = 4;
    pub const PRED_EXTRA_BITS: i64 = 3;
    pub const PREDICTION_ROUND: i64 = ((1 << PRED_EXTRA_BITS) >> 1) - 1;
    pub const NUM_PROPERTIES: usize = 1;
    #[derive(Clone, Debug)]
    pub struct Header {
        pub all_default: bool,
        pub p1_c: Pixel,
        pub p2_c: Pixel,
        pub p3_ca: Pixel,
        pub p3_cb: Pixel,
        pub p3_cc: Pixel,
        pub p3_cd: Pixel,
        pub p3_ce: Pixel,
        pub w: [u32; NUM_PREDICTORS],
    }
    impl Default for Header {
        fn default() -> Self {
            let mut h = Self {
                all_default: false,
                p1_c: 0,
                p2_c: 0,
                p3_ca: 0,
                p3_cb: 0,
                p3_cc: 0,
                p3_cd: 0,
                p3_ce: 0,
                w: [0; 4],
            };
            bundle::init(&mut h);
            h
        }
    }
    fn visit_p(visitor: &mut dyn Visitor, default: Pixel, p: &mut Pixel) -> Status {
        let mut up = *p as u32;
        visitor.bits(5, default as u32, &mut up)?;
        *p = up as Pixel;
        Ok(())
    }
    impl Fields for Header {
        fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
            let mut all_default = self.all_default;
            let result = visitor.all_default(self, &mut all_default);
            self.all_default = all_default;
            if result {
                visitor.set_default(self);
                return Ok(());
            }
            visit_p(visitor, 16, &mut self.p1_c)?;
            visit_p(visitor, 10, &mut self.p2_c)?;
            visit_p(visitor, 7, &mut self.p3_ca)?;
            visit_p(visitor, 7, &mut self.p3_cb)?;
            visit_p(visitor, 7, &mut self.p3_cc)?;
            visit_p(visitor, 0, &mut self.p3_cd)?;
            visit_p(visitor, 0, &mut self.p3_ce)?;
            visitor.bits(4, 0xd, &mut self.w[0])?;
            visitor.bits(4, 0xc, &mut self.w[1])?;
            visitor.bits(4, 0xc, &mut self.w[2])?;
            visitor.bits(4, 0xc, &mut self.w[3])
        }
    }
    pub struct State<'a> {
        pub prediction: [PixelWide; NUM_PREDICTORS],
        pub pred: PixelWide,
        pub pred_errors: [Vec<u32>; NUM_PREDICTORS],
        pub error: Vec<i32>,
        header: &'a Header,
        divlookup: [u32; 64],
    }
    impl<'a> State<'a> {
        pub fn header(&self) -> &'a Header {
            self.header
        }
        pub fn divlookup(&self) -> &[u32; 64] {
            &self.divlookup
        }
        pub const fn add_bits(x: PixelWide) -> PixelWide {
            ((x as u64) << PRED_EXTRA_BITS) as PixelWide
        }
        pub fn new(header: &'a Header, xsize: usize, _ysize: usize) -> Self {
            let length = xsize.wrapping_add(2).wrapping_mul(2);
            Self {
                prediction: [0; 4],
                pred: 0,
                pred_errors: std::array::from_fn(|_| vec![0; length]),
                error: vec![0; length],
                header,
                divlookup: [
                    16777216, 8388608, 5592405, 4194304, 3355443, 2796202, 2396745, 2097152,
                    1864135, 1677721, 1525201, 1398101, 1290555, 1198372, 1118481, 1048576, 986895,
                    932067, 883011, 838860, 798915, 762600, 729444, 699050, 671088, 645277, 621378,
                    599186, 578524, 559240, 541200, 524288, 508400, 493447, 479349, 466033, 453438,
                    441505, 430185, 419430, 409200, 399457, 390167, 381300, 372827, 364722, 356962,
                    349525, 342392, 335544, 328965, 322638, 316551, 310689, 305040, 299593, 294337,
                    289262, 284359, 279620, 275036, 270600, 266305, 262144,
                ],
            }
        }
        pub fn error_weight(&self, x: u64, maxweight: u32) -> u32 {
            let shift = (floor_log2_nonzero(x.wrapping_add(1)) as i32 - 5).max(0) as u32;
            4u32.wrapping_add(
                maxweight.wrapping_mul(self.divlookup[(x >> shift) as usize]) >> shift,
            )
        }
        pub fn weighted_average(&self, p: &[PixelWide; 4], mut w: [u32; 4]) -> PixelWide {
            let mut weight_sum = 0u32;
            for weight in w {
                weight_sum = weight_sum.wrapping_add(weight);
            }
            debug_assert!(weight_sum > 15);
            let log_weight = floor_log2_nonzero(weight_sum);
            weight_sum = 0;
            for weight in &mut w {
                *weight >>= log_weight - 4;
                weight_sum = weight_sum.wrapping_add(*weight);
            }
            let mut sum = i64::from((weight_sum >> 1).wrapping_sub(1));
            for i in 0..4 {
                sum += p[i] * i64::from(w[i]);
            }
            (sum * i64::from(self.divlookup[weight_sum as usize - 1])) >> 24
        }
        #[expect(
            clippy::too_many_arguments,
            reason = "Preserve the source weighted predictor's coordinates, neighbors, and property output."
        )]
        pub fn predict<const COMPUTE_PROPERTIES: bool>(
            &mut self,
            x: usize,
            y: usize,
            xsize: usize,
            n: PixelWide,
            w: PixelWide,
            ne: PixelWide,
            nw: PixelWide,
            nn: PixelWide,
            properties: Option<&mut Properties>,
            offset: usize,
        ) -> PixelWide {
            let cur_row = if y & 1 != 0 { 0 } else { xsize + 2 };
            let prev_row = if y & 1 != 0 { xsize + 2 } else { 0 };
            let pos_n = prev_row + x;
            let pos_ne = if x < xsize.wrapping_sub(1) {
                pos_n + 1
            } else {
                pos_n
            };
            let pos_nw = if x > 0 { pos_n - 1 } else { pos_n };
            let mut weights = [0u32; 4];
            for (i, weight) in weights.iter_mut().enumerate() {
                *weight = self.pred_errors[i][pos_n]
                    .wrapping_add(self.pred_errors[i][pos_ne])
                    .wrapping_add(self.pred_errors[i][pos_nw]);
                *weight = self.error_weight(u64::from(*weight), self.header.w[i]);
            }
            let n = Self::add_bits(n);
            let w = Self::add_bits(w);
            let ne = Self::add_bits(ne);
            let nw = Self::add_bits(nw);
            let nn = Self::add_bits(nn);
            let te_w = if x == 0 {
                0
            } else {
                i64::from(self.error[cur_row + x - 1])
            };
            let te_n = i64::from(self.error[pos_n]);
            let te_nw = i64::from(self.error[pos_nw]);
            let sum_wn = te_n + te_w;
            let te_ne = i64::from(self.error[pos_ne]);
            if COMPUTE_PROPERTIES {
                let mut p = te_w;
                if te_n.abs() > p.abs() {
                    p = te_n;
                }
                if te_nw.abs() > p.abs() {
                    p = te_nw;
                }
                if te_ne.abs() > p.abs() {
                    p = te_ne;
                }
                properties.expect("weighted property output")[offset] = p as PropertyVal;
            }
            self.prediction[0] = w + ne - n;
            self.prediction[1] = n - (((sum_wn + te_ne) * i64::from(self.header.p1_c)) >> 5);
            self.prediction[2] = w - (((sum_wn + te_nw) * i64::from(self.header.p2_c)) >> 5);
            self.prediction[3] = n
                - ((te_nw * i64::from(self.header.p3_ca)
                    + te_n * i64::from(self.header.p3_cb)
                    + te_ne * i64::from(self.header.p3_cc)
                    + (nn - n) * i64::from(self.header.p3_cd)
                    + (nw - w) * i64::from(self.header.p3_ce))
                    >> 5);
            self.pred = self.weighted_average(&self.prediction, weights);
            if ((te_n ^ te_w) | (te_n ^ te_nw)) > 0 {
                return (self.pred + PREDICTION_ROUND) >> PRED_EXTRA_BITS;
            }
            let mx = w.max(ne.max(n));
            let mn = w.min(ne.min(n));
            self.pred = mn.max(mx.min(self.pred));
            (self.pred + PREDICTION_ROUND) >> PRED_EXTRA_BITS
        }
        pub fn update_errors(&mut self, val: PixelWide, x: usize, y: usize, xsize: usize) {
            let cur_row = if y & 1 != 0 { 0 } else { xsize + 2 };
            let prev_row = if y & 1 != 0 { xsize + 2 } else { 0 };
            let val = Self::add_bits(val);
            self.error[cur_row + x] = (self.pred - val) as i32;
            for i in 0..4 {
                let err = ((self.prediction[i] - val).abs() + PREDICTION_ROUND) >> PRED_EXTRA_BITS;
                self.pred_errors[i][cur_row + x] = err as u32;
                self.pred_errors[i][prev_row + x + 1] =
                    (i64::from(self.pred_errors[i][prev_row + x + 1]) + err) as u32;
            }
        }
    }
    pub fn predictor_mode(i: i32, header: &mut Header) {
        let (w, p) = match i {
            0 => ([13, 12, 12, 12], [16, 10, 7, 7, 7, 0, 0]),
            1 => ([13, 12, 12, 11], [8, 8, 4, 0, 3, 23, 2]),
            2 => ([13, 12, 13, 12], [10, 9, 7, 0, 0, 16, 9]),
            3 => ([13, 13, 12, 12], [16, 8, 0, 16, 0, 23, 0]),
            _ => ([13, 12, 12, 12], [10, 10, 5, 5, 5, 12, 4]),
        };
        header.w = w;
        header.p1_c = p[0];
        header.p2_c = p[1];
        header.p3_ca = p[2];
        header.p3_cb = p[3];
        header.p3_cc = p[4];
        header.p3_cd = p[5];
        header.p3_ce = p[6];
    }
}
pub fn predictor_has_weighted(p: Predictor) -> bool {
    matches!(
        p,
        Predictor::WEIGHTED | Predictor::BEST | Predictor::VARIABLE
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct FlatDecisionNode {
    pub property0: i32,
    split_or_predictor: u32,
    split_or_multiplier: [i32; 2],
    pub child_id: u32,
    properties_or_offset: u32,
}
impl FlatDecisionNode {
    pub const fn new(property0: i32, child_id: u32) -> Self {
        Self {
            property0,
            child_id,
            split_or_predictor: 0,
            split_or_multiplier: [0; 2],
            properties_or_offset: 0,
        }
    }
    pub fn splitval0(&self) -> PropertyVal {
        self.split_or_predictor as i32
    }
    pub fn set_splitval0(&mut self, value: PropertyVal) {
        self.split_or_predictor = value as u32;
    }
    pub fn predictor(&self) -> Predictor {
        Predictor(self.split_or_predictor)
    }
    pub fn set_predictor(&mut self, value: Predictor) {
        self.split_or_predictor = value.0;
    }
    pub fn splitvals(&self) -> [PropertyVal; 2] {
        self.split_or_multiplier
    }
    pub fn set_splitval(&mut self, index: usize, value: PropertyVal) {
        self.split_or_multiplier[index] = value;
    }
    pub fn multiplier(&self) -> i32 {
        self.split_or_multiplier[0]
    }
    pub fn set_multiplier(&mut self, value: i32) {
        self.split_or_multiplier[0] = value;
    }
    pub fn properties(&self) -> [i16; 2] {
        let b = self.properties_or_offset.to_ne_bytes();
        [
            i16::from_ne_bytes([b[0], b[1]]),
            i16::from_ne_bytes([b[2], b[3]]),
        ]
    }
    pub fn set_property(&mut self, index: usize, value: i16) {
        let mut b = self.properties_or_offset.to_ne_bytes();
        b[index * 2..index * 2 + 2].copy_from_slice(&value.to_ne_bytes());
        self.properties_or_offset = u32::from_ne_bytes(b);
    }
    pub fn predictor_offset(&self) -> i32 {
        self.properties_or_offset as i32
    }
    pub fn set_predictor_offset(&mut self, value: i32) {
        self.properties_or_offset = value as u32;
    }
}
pub type FlatTree = Vec<FlatDecisionNode>;
pub struct MATreeLookup<'a> {
    nodes: &'a [FlatDecisionNode],
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LookupResult {
    pub context: u32,
    pub predictor: Predictor,
    pub offset: i32,
    pub multiplier: i32,
}
impl<'a> MATreeLookup<'a> {
    pub fn new(tree: &'a [FlatDecisionNode]) -> Self {
        Self { nodes: tree }
    }
    pub fn lookup(&self, properties: &[PropertyVal]) -> LookupResult {
        let mut pos = 0u32;
        macro_rules! traverse_the_tree {
            () => {{
                let node = &self.nodes[pos as usize];
                if node.property0 < 0 {
                    return LookupResult {
                        context: node.child_id,
                        predictor: node.predictor(),
                        offset: node.predictor_offset(),
                        multiplier: node.multiplier(),
                    };
                }
                let p0 = properties[node.property0 as usize] <= node.splitval0();
                let props = node.properties();
                let splits = node.splitvals();
                let off0 = u32::from(properties[props[0] as usize] <= splits[0]);
                let off1 = 2 | u32::from(properties[props[1] as usize] <= splits[1]);
                pos = node.child_id.wrapping_add(if p0 { off1 } else { off0 });
            }};
        }
        loop {
            traverse_the_tree!();
            traverse_the_tree!();
        }
    }
}
pub const EXTRA_PROPS_PER_CHANNEL: usize = 4;
pub const NUM_NONREF_PROPERTIES: usize =
    NUM_STATIC_PROPERTIES as usize + 13 + weighted::NUM_PROPERTIES;
pub const WP_PROP: usize = NUM_NONREF_PROPERTIES - weighted::NUM_PROPERTIES;
pub const GRADIENT_PROP: usize = 9;
pub fn clamped_gradient(n: i32, w: i32, l: i32) -> i32 {
    let m = n.min(w);
    let big = n.max(w);
    let grad = (n as u32).wrapping_add(w as u32).wrapping_sub(l as u32) as i32;
    let grad_clamp_big = if l < m { big } else { grad };
    if l > big { m } else { grad_clamp_big }
}
pub fn select(a: PixelWide, b: PixelWide, c: PixelWide) -> PixelWide {
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    if pa < pb { a } else { b }
}
pub fn precompute_references(
    ch: &Channel,
    y: usize,
    image: &Image,
    i: u32,
    references: &mut Channel,
) {
    zero_fill_image(&mut references.plane);
    let mut offset = 0u32;
    let num_extra = references.w;
    let mut j = i as i32 - 1;
    while j >= 0 && (offset as usize) < num_extra {
        let previous = &image.channel[j as usize];
        let current = &image.channel[i as usize];
        if previous.w != current.w
            || previous.h != current.h
            || previous.hshift != current.hshift
            || previous.vshift != current.vshift
        {
            j -= 1;
            continue;
        }
        let row = previous.row(y);
        let prev = previous.row(if y != 0 { y - 1 } else { 0 });
        for x in 0..ch.w {
            let v = i64::from(row[x]);
            let left = if x != 0 { i64::from(row[x - 1]) } else { 0 };
            let top = if y != 0 { i64::from(prev[x]) } else { left };
            let top_left = if x != 0 && y != 0 {
                i64::from(prev[x - 1])
            } else {
                left
            };
            let predicted = i64::from(clamped_gradient(left as i32, top as i32, top_left as i32));
            let out = references.row_mut(x);
            let off = offset as usize;
            out[off] = v.abs() as Pixel;
            out[off + 1] = v as Pixel;
            out[off + 2] = (v - predicted).abs() as Pixel;
            out[off + 3] = (v - predicted) as Pixel;
        }
        offset = offset.wrapping_add(EXTRA_PROPS_PER_CHANNEL as u32);
        j -= 1;
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PredictionResult {
    pub context: i32,
    pub guess: PixelWide,
    pub predictor: Predictor,
    pub multiplier: i32,
}
// The source leaves multiplier indeterminate when no tree is used. Zero here
// is initialized Rust storage, not an additional meaning for non-tree results.
pub fn init_props_row(
    p: &mut Properties,
    static_props: &[Pixel; NUM_STATIC_PROPERTIES as usize],
    y: i32,
) {
    p[..NUM_STATIC_PROPERTIES as usize].copy_from_slice(static_props);
    p[2] = y;
    p[9] = 0;
}
#[derive(Clone, Copy)]
pub struct PixelPosition<'a> {
    samples: &'a [Pixel],
    position: usize,
    onerow: isize,
}
impl<'a> PixelPosition<'a> {
    pub fn new(samples: &'a [Pixel], position: usize, onerow: isize) -> Self {
        assert!(position <= samples.len());
        Self {
            samples,
            position,
            onerow,
        }
    }
    fn read(self, relative: isize) -> PixelWide {
        let index = self
            .position
            .checked_add_signed(relative)
            .expect("relative sample before storage");
        i64::from(self.samples[index])
    }
}
pub mod detail {
    use super::*;
    pub const USE_TREE: u32 = 1;
    pub const USE_WP: u32 = 2;
    pub const FORCE_COMPUTE_PROPERTIES: u32 = 4;
    pub const ALL_PREDICTIONS: u32 = 8;
    pub const NO_EDGE_CASES: u32 = 16;
    #[expect(
        clippy::too_many_arguments,
        reason = "Preserve the source predictor's eight neighbor inputs."
    )]
    pub fn predict_one(
        p: Predictor,
        left: PixelWide,
        top: PixelWide,
        toptop: PixelWide,
        topleft: PixelWide,
        topright: PixelWide,
        leftleft: PixelWide,
        toprightright: PixelWide,
        wp_pred: PixelWide,
    ) -> PixelWide {
        match p {
            Predictor::ZERO => 0,
            Predictor::LEFT => left,
            Predictor::TOP => top,
            Predictor::SELECT => select(left, top, topleft),
            Predictor::WEIGHTED => wp_pred,
            Predictor::GRADIENT => {
                i64::from(clamped_gradient(top as i32, left as i32, topleft as i32))
            }
            Predictor::TOP_LEFT => topleft,
            Predictor::TOP_RIGHT => topright,
            Predictor::LEFT_LEFT => leftleft,
            Predictor::AVERAGE0 => (left + top) / 2,
            Predictor::AVERAGE1 => (left + topleft) / 2,
            Predictor::AVERAGE2 => (topleft + top) / 2,
            Predictor::AVERAGE3 => (top + topright) / 2,
            Predictor::AVERAGE4 => {
                (6 * top - 2 * toptop + 7 * left + leftleft + toprightright + 3 * topright + 8) / 16
            }
            _ => 0,
        }
    }
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep the source template's optional property/tree/state/output boundaries."
    )]
    pub fn predict<const MODE: u32>(
        mut p: Option<&mut Properties>,
        w: usize,
        pp: PixelPosition<'_>,
        x: usize,
        y: usize,
        mut predictor: Predictor,
        lookup: Option<&MATreeLookup<'_>>,
        references: Option<&Channel>,
        wp_state: Option<&mut weighted::State<'_>>,
        predictions: Option<&mut [PixelWide]>,
    ) -> PredictionResult {
        let mut offset = 3;
        let compute = MODE & USE_TREE != 0 || MODE & FORCE_COMPUTE_PROPERTIES != 0;
        let nec = MODE & NO_EDGE_CASES != 0;
        let onerow = pp.onerow;
        let left = if nec || x != 0 {
            pp.read(-1)
        } else if y != 0 {
            pp.read(-onerow)
        } else {
            0
        };
        let top = if nec || y != 0 {
            pp.read(-onerow)
        } else {
            left
        };
        let topleft = if nec || (x != 0 && y != 0) {
            pp.read(-1 - onerow)
        } else {
            left
        };
        let topright = if nec || (x.wrapping_add(1) < w && y != 0) {
            pp.read(1 - onerow)
        } else {
            top
        };
        let leftleft = if nec || x > 1 { pp.read(-2) } else { left };
        let toptop = if nec || y > 1 {
            pp.read(-onerow - onerow)
        } else {
            top
        };
        let toprightright = if nec || (x.wrapping_add(2) < w && y != 0) {
            pp.read(2 - onerow)
        } else {
            topright
        };
        if compute {
            let p = p.as_deref_mut().expect("prediction properties");
            p[offset] = x as PropertyVal;
            offset += 1;
            p[offset] = if top > 0 { top } else { -top } as PropertyVal;
            offset += 1;
            p[offset] = if left > 0 { left } else { -left } as PropertyVal;
            offset += 1;
            p[offset] = top as PropertyVal;
            offset += 1;
            p[offset] = left as PropertyVal;
            offset += 1;
            p[offset] = (left - i64::from(p[offset + 1])) as PropertyVal;
            offset += 1;
            p[offset] = (left + top - topleft) as PropertyVal;
            offset += 1;
            p[offset] = (left - topleft) as PropertyVal;
            offset += 1;
            p[offset] = (topleft - top) as PropertyVal;
            offset += 1;
            p[offset] = (top - topright) as PropertyVal;
            offset += 1;
            p[offset] = (top - toptop) as PropertyVal;
            offset += 1;
            p[offset] = (left - leftleft) as PropertyVal;
            offset += 1;
        }
        let mut wp_pred = 0;
        if MODE & USE_WP != 0 {
            let state = wp_state.expect("weighted predictor state");
            wp_pred = if compute {
                state.predict::<true>(
                    x,
                    y,
                    w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    p.as_deref_mut(),
                    offset,
                )
            } else {
                state.predict::<false>(
                    x,
                    y,
                    w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    p.as_deref_mut(),
                    offset,
                )
            };
        }
        if !nec && compute {
            offset += weighted::NUM_PROPERTIES;
            let references = references.expect("reference properties");
            let row = references.row(x);
            let p = p.as_deref_mut().unwrap();
            for &v in &row[..references.w] {
                p[offset] = v;
                offset += 1;
            }
        }
        let mut result = PredictionResult::default();
        if MODE & USE_TREE != 0 {
            let lr = lookup
                .expect("prediction tree")
                .lookup(p.as_deref().unwrap());
            result.context = lr.context as i32;
            result.guess = i64::from(lr.offset);
            result.multiplier = lr.multiplier;
            predictor = lr.predictor;
        }
        if MODE & ALL_PREDICTIONS != 0 {
            let predictions = predictions.expect("all-predictions output");
            for (i, value) in predictions[..NUM_MODULAR_PREDICTORS].iter_mut().enumerate() {
                *value = predict_one(
                    Predictor(i as u32),
                    left,
                    top,
                    toptop,
                    topleft,
                    topright,
                    leftleft,
                    toprightright,
                    wp_pred,
                );
            }
        }
        result.guess += predict_one(
            predictor,
            left,
            top,
            toptop,
            topleft,
            topright,
            leftleft,
            toprightright,
            wp_pred,
        );
        result.predictor = predictor;
        result
    }
}
pub fn predict_no_tree_no_wp(
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    predictor: Predictor,
) -> PredictionResult {
    detail::predict::<0>(
        None, w, pp, x as usize, y as usize, predictor, None, None, None, None,
    )
}
pub fn predict_no_tree_wp(
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    predictor: Predictor,
    wp_state: &mut weighted::State<'_>,
) -> PredictionResult {
    detail::predict::<{ detail::USE_WP }>(
        None,
        w,
        pp,
        x as usize,
        y as usize,
        predictor,
        None,
        None,
        Some(wp_state),
        None,
    )
}
pub fn predict_tree_no_wp(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    lookup: &MATreeLookup<'_>,
    references: &Channel,
) -> PredictionResult {
    detail::predict::<{ detail::USE_TREE }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        Some(lookup),
        Some(references),
        None,
        None,
    )
}
pub fn predict_tree_no_wp_nec(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    lookup: &MATreeLookup<'_>,
    references: &Channel,
) -> PredictionResult {
    detail::predict::<{ detail::USE_TREE | detail::NO_EDGE_CASES }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        Some(lookup),
        Some(references),
        None,
        None,
    )
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the tree, reference, and weighted-state inputs of the source wrapper."
)]
pub fn predict_tree_wp(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    lookup: &MATreeLookup<'_>,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
) -> PredictionResult {
    detail::predict::<{ detail::USE_TREE | detail::USE_WP }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        Some(lookup),
        Some(references),
        Some(wp_state),
        None,
    )
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the tree, reference, and weighted-state inputs of the source wrapper."
)]
pub fn predict_tree_wp_nec(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    lookup: &MATreeLookup<'_>,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
) -> PredictionResult {
    detail::predict::<{ detail::USE_TREE | detail::USE_WP | detail::NO_EDGE_CASES }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        Some(lookup),
        Some(references),
        Some(wp_state),
        None,
    )
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the predictor, reference, and weighted-state inputs of the source wrapper."
)]
pub fn predict_learn(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    predictor: Predictor,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
) -> PredictionResult {
    detail::predict::<{ detail::FORCE_COMPUTE_PROPERTIES | detail::USE_WP }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        predictor,
        None,
        Some(references),
        Some(wp_state),
        None,
    )
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve both mutable outputs and the state inputs of the source learning wrapper."
)]
pub fn predict_learn_all(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
    predictions: &mut [PixelWide],
) {
    detail::predict::<
        { detail::FORCE_COMPUTE_PROPERTIES | detail::USE_WP | detail::ALL_PREDICTIONS },
    >(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        None,
        Some(references),
        Some(wp_state),
        Some(predictions),
    );
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the predictor, reference, and weighted-state inputs of the source wrapper."
)]
pub fn predict_learn_nec(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    predictor: Predictor,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
) -> PredictionResult {
    detail::predict::<{ detail::FORCE_COMPUTE_PROPERTIES | detail::USE_WP | detail::NO_EDGE_CASES }>(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        predictor,
        None,
        Some(references),
        Some(wp_state),
        None,
    )
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve both mutable outputs and the state inputs of the source learning wrapper."
)]
pub fn predict_learn_all_nec(
    p: &mut Properties,
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
    predictions: &mut [PixelWide],
) {
    detail::predict::<
        {
            detail::FORCE_COMPUTE_PROPERTIES
                | detail::USE_WP
                | detail::ALL_PREDICTIONS
                | detail::NO_EDGE_CASES
        },
    >(
        Some(p),
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        None,
        Some(references),
        Some(wp_state),
        Some(predictions),
    );
}
pub fn predict_all_no_wp(
    w: usize,
    pp: PixelPosition<'_>,
    x: i32,
    y: i32,
    predictions: &mut [PixelWide],
) {
    detail::predict::<{ detail::ALL_PREDICTIONS }>(
        None,
        w,
        pp,
        x as usize,
        y as usize,
        Predictor::ZERO,
        None,
        None,
        None,
        Some(predictions),
    );
}
