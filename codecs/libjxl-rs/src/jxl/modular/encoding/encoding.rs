// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! GroupHeader, FilterTree, and TreeToLookupTable from modular/encoding/
//! encoding.{h,cc}, including complete modular image/channel decoding.
//! Conditional arrays
//! use vectors of exactly the source lengths (zero or 2 * PROP_RANGE_FAST).
//! Rust initializes LUT storage; callers must only observe source-defined data.

use super::context_predict::{
    EXTRA_PROPS_PER_CHANNEL, FlatDecisionNode, FlatTree, GRADIENT_PROP, NUM_NONREF_PROPERTIES,
    WP_PROP, weighted,
};
use super::context_predict::{
    MATreeLookup, PixelPosition, clamped_gradient, init_props_row, precompute_references,
    predict_tree_no_wp, predict_tree_no_wp_nec, predict_tree_wp, predict_tree_wp_nec,
};
use super::dec_ma::PropertyDecisionNode;
use super::dec_ma::{Tree, decode_tree};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use crate::jxl::dec_bit_reader::BitReader;
use crate::jxl::field_encodings::{Fields, U32Enc, bits_offset, val};
use crate::jxl::fields::{Visitor, bundle};
use crate::jxl::frame_dimensions::BLOCK_DIM;
use crate::jxl::image_ops::zero_fill_image;
use crate::jxl::modular::modular_image::{Channel, Image};
use crate::jxl::modular::options::{ModularOptions, NUM_STATIC_PROPERTIES, Predictor};
use crate::jxl::modular::transform::transform::Transform;
use crate::jxl::pack_signed::unpack_signed;
use std::collections::VecDeque;

pub const PROP_RANGE_FAST: i32 = 512 << 4;
#[derive(Clone, Debug)]
pub struct GroupHeader {
    pub use_global_tree: bool,
    pub wp_header: weighted::Header,
    pub transforms: Vec<Transform>,
}
impl Default for GroupHeader {
    fn default() -> Self {
        let mut h = Self {
            use_global_tree: false,
            wp_header: weighted::Header::default(),
            transforms: Vec::new(),
        };
        bundle::init(&mut h);
        h
    }
}
impl Fields for GroupHeader {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.use_global_tree)?;
        visitor.visit_nested(&mut self.wp_header)?;
        let mut num_transforms = self.transforms.len() as u32;
        visitor.u32(
            U32Enc::new(val(0), val(1), bits_offset(4, 2), bits_offset(8, 18)),
            0,
            &mut num_transforms,
        )?;
        if visitor.is_reading() {
            self.transforms
                .resize_with(num_transforms as usize, Transform::default);
        }
        for i in 0..num_transforms as usize {
            visitor.visit_nested(&mut self.transforms[i])?;
        }
        Ok(())
    }
}

pub fn filter_tree(
    global_tree: &[PropertyDecisionNode],
    static_props: &mut [i32; NUM_STATIC_PROPERTIES as usize],
    num_props: &mut usize,
    use_wp: &mut bool,
    wp_only: &mut bool,
    gradient_only: &mut bool,
) -> FlatTree {
    *num_props = 0;
    let mut has_wp = false;
    let mut has_non_wp = false;
    *gradient_only = true;
    let mut output = Vec::new();
    let mut nodes = VecDeque::from([0usize]);
    while let Some(mut cur) = nodes.pop_front() {
        while i32::from(global_tree[cur].property) < NUM_STATIC_PROPERTIES as i32
            && global_tree[cur].property != -1
        {
            cur = if static_props[global_tree[cur].property as usize] > global_tree[cur].splitval {
                global_tree[cur].lchild as usize
            } else {
                global_tree[cur].rchild as usize
            };
        }
        let node = &global_tree[cur];
        let mut flat = FlatDecisionNode::default();
        if node.property == -1 {
            flat.property0 = -1;
            flat.child_id = node.lchild;
            flat.set_predictor(node.predictor);
            flat.set_predictor_offset(node.predictor_offset as i32);
            flat.set_multiplier(node.multiplier as i32);
            *gradient_only &= flat.predictor() == Predictor::GRADIENT;
            has_wp |= flat.predictor() == Predictor::WEIGHTED;
            has_non_wp |= flat.predictor() != Predictor::WEIGHTED;
            output.push(flat);
            continue;
        }
        flat.child_id = (output.len() + nodes.len() + 1) as u32;
        flat.property0 = i32::from(node.property);
        *num_props = (*num_props).max((flat.property0 + 1) as usize);
        flat.set_splitval0(node.splitval);
        for i in 0..2 {
            let mut cur_child = if i == 0 { node.lchild } else { node.rchild } as usize;
            while i32::from(global_tree[cur_child].property) < NUM_STATIC_PROPERTIES as i32
                && global_tree[cur_child].property != -1
            {
                let child = &global_tree[cur_child];
                cur_child = if static_props[child.property as usize] > child.splitval {
                    child.lchild as usize
                } else {
                    child.rchild as usize
                };
            }
            let child = &global_tree[cur_child];
            if child.property == -1 {
                flat.set_property(i, 0);
                flat.set_splitval(i, 0);
                nodes.push_back(cur_child);
                nodes.push_back(cur_child);
            } else {
                flat.set_property(i, child.property);
                flat.set_splitval(i, child.splitval);
                nodes.push_back(child.lchild as usize);
                nodes.push_back(child.rchild as usize);
                *num_props = (*num_props).max((i32::from(flat.properties()[i]) + 1) as usize);
            }
        }
        let mut mark_property = |p: i32| {
            if p == WP_PROP as i32 {
                has_wp = true;
            } else if p >= NUM_STATIC_PROPERTIES as i32 {
                has_non_wp = true;
            }
            if p >= NUM_STATIC_PROPERTIES as i32 && p != GRADIENT_PROP as i32 {
                *gradient_only = false;
            }
        };
        for p in flat.properties() {
            mark_property(i32::from(p));
        }
        mark_property(flat.property0);
        output.push(flat);
    }
    if *num_props > NUM_NONREF_PROPERTIES {
        *num_props = (*num_props - NUM_NONREF_PROPERTIES).div_ceil(EXTRA_PROPS_PER_CHANNEL)
            * EXTRA_PROPS_PER_CHANNEL
            + NUM_NONREF_PROPERTIES;
    } else {
        *num_props = NUM_NONREF_PROPERTIES;
    }
    *use_wp = has_wp;
    *wp_only = has_wp && !has_non_wp;
    output
}

pub trait TreeLutWord: Copy + Default {
    fn from_context(context: u32) -> Self;
}
impl TreeLutWord for u8 {
    fn from_context(context: u32) -> Self {
        context as Self
    }
}
impl TreeLutWord for u16 {
    fn from_context(context: u32) -> Self {
        context as Self
    }
}
impl TreeLutWord for u32 {
    fn from_context(context: u32) -> Self {
        context
    }
}
pub struct TreeLut<T, const HAS_OFFSETS: bool, const HAS_MULTIPLIERS: bool> {
    pub context_lookup: [T; 2 * PROP_RANGE_FAST as usize],
    pub offsets: Vec<i8>,
    pub multipliers: Vec<i8>,
}
impl<T: TreeLutWord, const O: bool, const M: bool> Default for TreeLut<T, O, M> {
    fn default() -> Self {
        Self {
            context_lookup: [T::default(); 2 * PROP_RANGE_FAST as usize],
            offsets: vec![0; if O { 2 * PROP_RANGE_FAST as usize } else { 0 }],
            multipliers: vec![0; if M { 2 * PROP_RANGE_FAST as usize } else { 0 }],
        }
    }
}
pub fn tree_to_lookup_table<T: TreeLutWord, const O: bool, const M: bool>(
    tree: &[FlatDecisionNode],
    lut: &mut TreeLut<T, O, M>,
) -> bool {
    struct TreeRange {
        begin: i32,
        end: i32,
        pos: usize,
    }
    let mut ranges = vec![TreeRange {
        begin: -PROP_RANGE_FAST - 1,
        end: PROP_RANGE_FAST - 1,
        pos: 0,
    }];
    while let Some(cur) = ranges.pop() {
        if cur.begin < -PROP_RANGE_FAST - 1
            || cur.begin >= PROP_RANGE_FAST - 1
            || cur.end > PROP_RANGE_FAST - 1
        {
            return false;
        }
        let node = &tree[cur.pos];
        if node.property0 == -1 {
            if node.predictor_offset() < i32::from(i8::MIN)
                || node.predictor_offset() > i32::from(i8::MAX)
            {
                return false;
            }
            if node.multiplier() < i32::from(i8::MIN) || node.multiplier() > i32::from(i8::MAX) {
                return false;
            }
            if !M && node.multiplier() != 1 {
                return false;
            }
            if !O && node.predictor_offset() != 0 {
                return false;
            }
            for i in cur.begin + 1..cur.end + 1 {
                let j = (i + PROP_RANGE_FAST) as usize;
                lut.context_lookup[j] = T::from_context(node.child_id);
                if M {
                    lut.multipliers[j] = node.multiplier() as i8;
                }
                if O {
                    lut.offsets[j] = node.predictor_offset() as i8;
                }
            }
            continue;
        }
        if i32::from(node.properties()[0]) >= NUM_STATIC_PROPERTIES as i32 {
            ranges.push(TreeRange {
                begin: node.splitvals()[0],
                end: cur.end,
                pos: node.child_id as usize,
            });
            ranges.push(TreeRange {
                begin: node.splitval0(),
                end: node.splitvals()[0],
                pos: node.child_id.wrapping_add(1) as usize,
            });
        } else {
            ranges.push(TreeRange {
                begin: node.splitval0(),
                end: cur.end,
                pos: node.child_id as usize,
            });
        }
        if i32::from(node.properties()[1]) >= NUM_STATIC_PROPERTIES as i32 {
            ranges.push(TreeRange {
                begin: node.splitvals()[1],
                end: node.splitval0(),
                pos: node.child_id.wrapping_add(2) as usize,
            });
            ranges.push(TreeRange {
                begin: cur.begin,
                end: node.splitvals()[1],
                pos: node.child_id.wrapping_add(3) as usize,
            });
        } else {
            ranges.push(TreeRange {
                begin: cur.begin,
                end: node.splitval0(),
                pos: node.child_id.wrapping_add(2) as usize,
            });
        }
    }
    true
}

fn make_pixel(v: u64, multiplier: i32, offset: i64) -> i32 {
    // The source explicitly permits the final narrowing to overflow.
    let val = i64::from(unpack_signed(v as usize));
    (val * i64::from(multiplier) + offset) as i32
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the source channel/tree/reader/RLE boundaries"
)]
fn decode_modular_channel_maans_impl<const USES_LZ77: bool>(
    br: &mut BitReader<'_>,
    reader: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
    global_tree: &Tree,
    wp_header: &weighted::Header,
    chan: i32,
    group_id: usize,
    tree_lut: &mut TreeLut<u8, false, false>,
    image: &mut Image,
    fl_run: &mut u32,
    fl_v: &mut u32,
) -> Status {
    let ci = chan as usize;
    let (w, h, onerow) = {
        let c = &image.channel[ci];
        (c.w, c.h, c.plane.pixels_per_row())
    };
    // Rows are resolved once per row: `samples` is the channel's whole storage
    // and `row0` the first sample of row y. Reads and writes address the same
    // samples, in the same order, as the per-sample row lookups they replace.
    let stride = onerow as usize;
    let mut static_props = [chan, group_id as i32];
    if w == 0 || h == 0 {
        return Ok(());
    }
    let (mut tree_has_wp_prop_or_pred, mut is_wp_only, mut is_gradient_only) =
        (false, false, false);
    let mut num_props = 0;
    let mut tree = filter_tree(
        global_tree,
        &mut static_props,
        &mut num_props,
        &mut tree_has_wp_prop_or_pred,
        &mut is_wp_only,
        &mut is_gradient_only,
    );
    for node in &mut tree {
        if node.property0 == -1 {
            node.child_id = u32::from(context_map[node.child_id as usize]);
        }
    }
    let global_tree_is_all_gradient_noop = global_tree.iter().all(|n| {
        if n.property == -1 {
            n.predictor == Predictor::GRADIENT && n.predictor_offset == 0 && n.multiplier == 1
        } else {
            i32::from(n.property) < NUM_STATIC_PROPERTIES as i32
        }
    });
    if tree.len() == 1 {
        let predictor = tree[0].predictor();
        let offset = i64::from(tree[0].predictor_offset());
        let multiplier = tree[0].multiplier();
        let ctx_id = tree[0].child_id as usize;
        if predictor == Predictor::ZERO {
            let mut value = 0;
            if reader.is_single_value_and_advance(ctx_id, &mut value, w * h) {
                let v = make_pixel(u64::from(value), multiplier, offset);
                for y in 0..h {
                    image.channel[ci].row_mut(y)[..w].fill(v);
                }
            } else if multiplier == 1 && offset == 0 {
                for y in 0..h {
                    let row0 = y * stride;
                    let samples = image.channel[ci].plane.pixels_mut().unwrap();
                    for x in 0..w {
                        let v = reader.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx_id, br)
                            as u32;
                        samples[row0 + x] = unpack_signed(v as usize);
                    }
                }
            } else {
                for y in 0..h {
                    let row0 = y * stride;
                    let samples = image.channel[ci].plane.pixels_mut().unwrap();
                    for x in 0..w {
                        let v = reader
                            .read_hybrid_uint_clustered_maybe_inlined::<USES_LZ77>(ctx_id, br)
                            as u32;
                        samples[row0 + x] = make_pixel(u64::from(v), multiplier, offset);
                    }
                }
            }
            return Ok(());
        } else if USES_LZ77 && reader.is_huff_rle_only() && global_tree_is_all_gradient_noop {
            let mut sv = i64::from(unpack_signed(*fl_v as usize));
            for y in 0..h {
                let row0 = y * stride;
                let samples = image.channel[ci].plane.pixels_mut().unwrap();
                let guess_0 = if y != 0 {
                    i64::from(samples[row0 - stride])
                } else {
                    0
                };
                if *fl_run == 0 {
                    reader.read_hybrid_uint_clustered_huff_rle_only(ctx_id, br, fl_v, fl_run);
                    sv = i64::from(unpack_signed(*fl_v as usize));
                } else {
                    *fl_run -= 1;
                }
                samples[row0] = (sv + guess_0) as i32;
                for x in 1..w {
                    let left = samples[row0 + x - 1];
                    let top = if y != 0 {
                        samples[row0 - stride + x]
                    } else {
                        samples[row0 + x - 1]
                    };
                    let topleft = if y != 0 {
                        samples[row0 - stride + x - 1]
                    } else {
                        samples[row0 + x - 1]
                    };
                    let guess = i64::from(clamped_gradient(top, left, topleft));
                    if *fl_run == 0 {
                        reader.read_hybrid_uint_clustered_huff_rle_only(ctx_id, br, fl_v, fl_run);
                        sv = i64::from(unpack_signed(*fl_v as usize));
                    } else {
                        *fl_run -= 1;
                    }
                    samples[row0 + x] = (sv + guess) as i32;
                }
            }
            return Ok(());
        } else if predictor == Predictor::GRADIENT && offset == 0 && multiplier == 1 {
            for y in 0..h {
                let row0 = y * stride;
                let samples = image.channel[ci].plane.pixels_mut().unwrap();
                for x in 0..w {
                    let p = PixelPosition::new(samples, row0 + x, onerow);
                    let left = if x != 0 {
                        samples[row0 + x - 1]
                    } else if y != 0 {
                        p.read_pixel(-onerow)
                    } else {
                        0
                    };
                    let top = if y != 0 { p.read_pixel(-onerow) } else { left };
                    let topleft = if x != 0 && y != 0 {
                        p.read_pixel(-1 - onerow)
                    } else {
                        left
                    };
                    let guess = clamped_gradient(top, left, topleft);
                    let v = reader.read_hybrid_uint_clustered_maybe_inlined::<USES_LZ77>(ctx_id, br)
                        as u64;
                    samples[row0 + x] = make_pixel(v, 1, i64::from(guess));
                }
            }
            return Ok(());
        }
    }
    if is_wp_only {
        is_wp_only = tree_to_lookup_table(&tree, tree_lut);
    }
    if is_gradient_only {
        is_gradient_only = tree_to_lookup_table(&tree, tree_lut);
    }
    if is_gradient_only {
        for y in 0..h {
            let row0 = y * stride;
            let samples = image.channel[ci].plane.pixels_mut().unwrap();
            for x in 0..w {
                let p = PixelPosition::new(samples, row0 + x, onerow);
                let left = i64::from(if x != 0 {
                    samples[row0 + x - 1]
                } else if y != 0 {
                    p.read_pixel(-onerow)
                } else {
                    0
                });
                let top = if y != 0 {
                    i64::from(p.read_pixel(-onerow))
                } else {
                    left
                };
                let topleft = if x != 0 && y != 0 {
                    i64::from(p.read_pixel(-1 - onerow))
                } else {
                    left
                };
                let guess = clamped_gradient(top as i32, left as i32, topleft as i32);
                let pos = (i64::from(PROP_RANGE_FAST)
                    + (top + left - topleft)
                        .clamp(i64::from(-PROP_RANGE_FAST), i64::from(PROP_RANGE_FAST - 1)))
                    as u32;
                let ctx_id = tree_lut.context_lookup[pos as usize] as usize;
                let v =
                    reader.read_hybrid_uint_clustered_maybe_inlined::<USES_LZ77>(ctx_id, br) as u64;
                samples[row0 + x] = make_pixel(v, 1, i64::from(guess));
            }
        }
    } else if !USES_LZ77 && is_wp_only && w > 8 {
        let mut wp_state = weighted::State::new(wp_header, w, h);
        let mut properties = vec![0; 1];
        for y in 0..h {
            let row0 = y * stride;
            let samples = image.channel[ci].plane.pixels_mut().unwrap();
            let left = if y != 0 {
                i64::from(samples[row0 - stride])
            } else {
                0
            };
            let toptop = if y > 1 {
                i64::from(samples[row0 - 2 * stride])
            } else if y != 0 {
                i64::from(samples[row0 - stride])
            } else {
                0
            };
            let topright = if 1 < w && y != 0 {
                i64::from(samples[row0 - stride + 1])
            } else {
                left
            };
            let guess = wp_state.predict::<true>(
                0,
                y,
                w,
                left,
                left,
                topright,
                left,
                toptop,
                Some(&mut properties),
                0,
            ) as i32;
            let pos = (PROP_RANGE_FAST + properties[0].clamp(-PROP_RANGE_FAST, PROP_RANGE_FAST - 1))
                as u32;
            let ctx_id = tree_lut.context_lookup[pos as usize] as usize;
            let v = reader.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx_id, br) as u64;
            let value = make_pixel(v, 1, i64::from(guess));
            samples[row0] = value;
            wp_state.update_errors(i64::from(value), 0, y, w);
            for x in 1..w - 1 {
                let left = i64::from(samples[row0 + x - 1]);
                let top = if y != 0 {
                    i64::from(samples[row0 - stride + x])
                } else {
                    left
                };
                let topright = if y != 0 {
                    i64::from(samples[row0 - stride + x + 1])
                } else {
                    left
                };
                let topleft = if y != 0 {
                    i64::from(samples[row0 - stride + x - 1])
                } else {
                    left
                };
                let toptop = if y > 1 {
                    i64::from(samples[row0 - 2 * stride + x])
                } else {
                    top
                };
                let guess = wp_state.predict::<true>(
                    x,
                    y,
                    w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    Some(&mut properties),
                    0,
                ) as i32;
                let pos = (PROP_RANGE_FAST
                    + properties[0].clamp(-PROP_RANGE_FAST, PROP_RANGE_FAST - 1))
                    as u32;
                let ctx_id = tree_lut.context_lookup[pos as usize] as usize;
                let v = reader.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx_id, br) as u64;
                let value = make_pixel(v, 1, i64::from(guess));
                samples[row0 + x] = value;
                wp_state.update_errors(i64::from(value), x, y, w);
            }
            let x = w - 1;
            let left = i64::from(samples[row0 + x - 1]);
            let top = if y != 0 {
                i64::from(samples[row0 - stride + x])
            } else {
                left
            };
            let topleft = if y != 0 {
                i64::from(samples[row0 - stride + x - 1])
            } else {
                left
            };
            let toptop = if y > 1 {
                i64::from(samples[row0 - 2 * stride + x])
            } else {
                top
            };
            let guess = wp_state.predict::<true>(
                x,
                y,
                w,
                top,
                left,
                top,
                topleft,
                toptop,
                Some(&mut properties),
                0,
            ) as i32;
            let pos = (PROP_RANGE_FAST + properties[0].clamp(-PROP_RANGE_FAST, PROP_RANGE_FAST - 1))
                as u32;
            let ctx_id = tree_lut.context_lookup[pos as usize] as usize;
            let v = reader.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx_id, br) as u64;
            let value = make_pixel(v, 1, i64::from(guess));
            samples[row0 + x] = value;
            wp_state.update_errors(i64::from(value), x, y, w);
        }
    } else if !tree_has_wp_prop_or_pred {
        let tree_lookup = MATreeLookup::new(&tree);
        let mut properties = vec![0; num_props];
        let mut references = Channel::create_in(
            image.memory_manager(),
            properties.len() - NUM_NONREF_PROPERTIES,
            w,
            0,
            0,
        )?;
        for y in 0..h {
            precompute_references(&image.channel[ci], y, image, chan as u32, &mut references);
            init_props_row(&mut properties, &static_props, y as i32);
            let row0 = y * stride;
            let samples = image.channel[ci].plane.pixels_mut().unwrap();
            if y > 1 && w > 8 && references.w == 0 {
                for x in 0..2 {
                    let res = predict_tree_no_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                    );
                    let v = reader.read_hybrid_uint_clustered::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    samples[row0 + x] = make_pixel(v, res.multiplier, res.guess);
                }
                for x in 2..w - 2 {
                    let res = predict_tree_no_wp_nec(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                    );
                    let v = reader
                        .read_hybrid_uint_clustered_inlined::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    samples[row0 + x] = make_pixel(v, res.multiplier, res.guess);
                }
                for x in w - 2..w {
                    let res = predict_tree_no_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                    );
                    let v = reader.read_hybrid_uint_clustered::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    samples[row0 + x] = make_pixel(v, res.multiplier, res.guess);
                }
            } else {
                for x in 0..w {
                    let res = predict_tree_no_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                    );
                    let v = reader.read_hybrid_uint_clustered_maybe_inlined::<USES_LZ77>(
                        res.context as usize,
                        br,
                    ) as u64;
                    samples[row0 + x] = make_pixel(v, res.multiplier, res.guess);
                }
            }
        }
    } else {
        let tree_lookup = MATreeLookup::new(&tree);
        let mut properties = vec![0; num_props];
        let mut references = Channel::create_in(
            image.memory_manager(),
            properties.len() - NUM_NONREF_PROPERTIES,
            w,
            0,
            0,
        )?;
        let mut wp_state = weighted::State::new(wp_header, w, h);
        for y in 0..h {
            init_props_row(&mut properties, &static_props, y as i32);
            precompute_references(&image.channel[ci], y, image, chan as u32, &mut references);
            let row0 = y * stride;
            let samples = image.channel[ci].plane.pixels_mut().unwrap();
            if !USES_LZ77 && y > 1 && w > 8 && references.w == 0 {
                for x in 0..2 {
                    let res = predict_tree_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                        &mut wp_state,
                    );
                    let v = reader.read_hybrid_uint_clustered::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    let value = make_pixel(v, res.multiplier, res.guess);
                    samples[row0 + x] = value;
                    wp_state.update_errors(i64::from(value), x, y, w);
                }
                for x in 2..w - 2 {
                    let res = predict_tree_wp_nec(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                        &mut wp_state,
                    );
                    let v = reader
                        .read_hybrid_uint_clustered_inlined::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    let value = make_pixel(v, res.multiplier, res.guess);
                    samples[row0 + x] = value;
                    wp_state.update_errors(i64::from(value), x, y, w);
                }
                for x in w - 2..w {
                    let res = predict_tree_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                        &mut wp_state,
                    );
                    let v = reader.read_hybrid_uint_clustered::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    let value = make_pixel(v, res.multiplier, res.guess);
                    samples[row0 + x] = value;
                    wp_state.update_errors(i64::from(value), x, y, w);
                }
            } else {
                for x in 0..w {
                    let res = predict_tree_wp(
                        &mut properties,
                        w,
                        PixelPosition::new(samples, row0 + x, onerow),
                        x as i32,
                        y as i32,
                        &tree_lookup,
                        &references,
                        &mut wp_state,
                    );
                    let v = reader.read_hybrid_uint_clustered::<USES_LZ77>(res.context as usize, br)
                        as u64;
                    let value = make_pixel(v, res.multiplier, res.guess);
                    samples[row0 + x] = value;
                    wp_state.update_errors(i64::from(value), x, y, w);
                }
            }
        }
    }
    Ok(())
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source dispatch and shared cross-channel RLE state"
)]
pub fn decode_modular_channel_maans(
    br: &mut BitReader<'_>,
    reader: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
    global_tree: &Tree,
    wp_header: &weighted::Header,
    chan: i32,
    group_id: usize,
    tree_lut: &mut TreeLut<u8, false, false>,
    image: &mut Image,
    fl_run: &mut u32,
    fl_v: &mut u32,
) -> Status {
    if reader.uses_lz77() {
        decode_modular_channel_maans_impl::<true>(
            br,
            reader,
            context_map,
            global_tree,
            wp_header,
            chan,
            group_id,
            tree_lut,
            image,
            fl_run,
            fl_v,
        )
    } else {
        decode_modular_channel_maans_impl::<false>(
            br,
            reader,
            context_map,
            global_tree,
            wp_header,
            chan,
            group_id,
            tree_lut,
            image,
            fl_run,
            fl_v,
        )
    }
}

pub fn validate_channel_dimensions(image: &Image, options: &ModularOptions) -> Status {
    let nb_channels = image.channel.len();
    for is_dc in [true, false] {
        let group_dim = options
            .group_dim
            .wrapping_mul(if is_dc { BLOCK_DIM } else { 1 });
        let mut c = image.nb_meta_channels;
        while c < nb_channels {
            let ch = &image.channel[c];
            if ch.w > options.group_dim || ch.h > options.group_dim {
                break;
            }
            c += 1;
        }
        while c < nb_channels {
            let ch = &image.channel[c];
            c += 1;
            if ch.w == 0 || ch.h == 0 {
                continue;
            }
            let is_dc_channel = ch.hshift.min(ch.vshift) >= 3;
            if is_dc_channel != is_dc {
                continue;
            }
            let shift = ch.hshift.max(ch.vshift);
            // Source shifts outside the machine word are undefined caller
            // preconditions. Serialized transforms keep these shifts bounded.
            assert!((0..usize::BITS as i32).contains(&shift));
            let tile_dim = group_dim >> shift;
            if tile_dim == 0 {
                return Err(StatusCode::GenericError);
            }
        }
    }
    Ok(())
}
fn meta_apply_transforms(image: &mut Image) -> Status {
    // MetaApply uses channel/size/meta state and never reads image.transform.
    // Move its owner out temporarily to permit separate mutable borrows of the
    // actual entries and image. Restore all entry mutations even on failure.
    let mut transforms = std::mem::take(&mut image.transform);
    let result = (|| {
        for t in &mut transforms {
            t.meta_apply(image)?;
        }
        Ok(())
    })();
    image.transform = transforms;
    result
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve original optional global entropy/tree inputs and truncation control"
)]
pub fn modular_decode(
    br: &mut BitReader<'_>,
    image: &mut Image,
    header: &mut GroupHeader,
    group_id: usize,
    options: &mut ModularOptions,
    global_tree: Option<&Tree>,
    global_code: Option<&ANSCode>,
    global_ctx_map: Option<&[u8]>,
    allow_truncated_group: bool,
) -> Status {
    if image.channel.is_empty() {
        return Ok(());
    }
    let memory_manager = image.memory_manager();
    let status = bundle::read(br, header);
    if !allow_truncated_group {
        status?;
    }
    if status.is_err_and(StatusCode::is_fatal_error) {
        return status;
    }
    if br.all_reads_within_bounds().is_err() {
        header.transforms.clear();
        image.transform = header.transforms.clone();
        for ch in &mut image.channel {
            zero_fill_image(&mut ch.plane);
        }
        return Err(StatusCode::NotEnoughBytes);
    }
    image.transform = header.transforms.clone();
    meta_apply_transforms(image)?;
    if image.error {
        return Err(StatusCode::GenericError);
    }
    validate_channel_dimensions(image, options)?;
    let nb_channels = image.channel.len();
    let mut num_chans = 0;
    let mut distance_multiplier = 0;
    for (i, channel) in image.channel.iter().enumerate() {
        if i >= image.nb_meta_channels
            && (channel.w > options.max_chan_size || channel.h > options.max_chan_size)
        {
            break;
        }
        if channel.w == 0 || channel.h == 0 {
            continue;
        }
        if channel.w > distance_multiplier {
            distance_multiplier = channel.w;
        }
        num_chans += 1;
    }
    if num_chans == 0 {
        return Ok(());
    }
    let mut next_channel = 0;
    let mut guard_armed = !allow_truncated_group;
    // A Result scope preserves the source guard's cleanup at every return,
    // including table/reader creation failures, without aliased image borrows.
    let result = (|| {
        let mut tree_storage = Vec::new();
        let mut map_storage = Vec::new();
        let mut code_storage = ANSCode::default();
        let (tree, code, context_map) = if !header.use_global_tree {
            let mut max_tree_size = 1024u64;
            for (i, channel) in image.channel.iter().enumerate() {
                if i >= image.nb_meta_channels
                    && (channel.w > options.max_chan_size || channel.h > options.max_chan_size)
                {
                    break;
                }
                let pixels = channel.w.wrapping_mul(channel.h) as u64;
                max_tree_size = max_tree_size.wrapping_add(pixels);
            }
            max_tree_size = max_tree_size.min(1 << 20);
            decode_tree(
                memory_manager,
                br,
                &mut tree_storage,
                max_tree_size as usize,
            )?;
            decode_histograms(
                memory_manager,
                br,
                tree_storage.len().div_ceil(2),
                &mut code_storage,
                &mut map_storage,
                false,
            )?;
            (&tree_storage, &code_storage, map_storage.as_slice())
        } else {
            let (Some(tree), Some(code), Some(map)) = (global_tree, global_code, global_ctx_map)
            else {
                return Err(StatusCode::GenericError);
            };
            if tree.is_empty() {
                return Err(StatusCode::GenericError);
            }
            (tree, code, map)
        };
        let mut reader = ANSSymbolReader::create(code, br, distance_multiplier)?;
        let mut tree_lut = Box::new(TreeLut::<u8, false, false>::default());
        let mut fl_run = 0;
        let mut fl_v = 0;
        while next_channel < nb_channels {
            let channel = &image.channel[next_channel];
            if next_channel >= image.nb_meta_channels
                && (channel.w > options.max_chan_size || channel.h > options.max_chan_size)
            {
                break;
            }
            if channel.w == 0 || channel.h == 0 {
                next_channel += 1;
                continue;
            }
            decode_modular_channel_maans(
                br,
                &mut reader,
                context_map,
                tree,
                &header.wp_header,
                next_channel as i32,
                group_id,
                &mut tree_lut,
                image,
                &mut fl_run,
                &mut fl_v,
            )?;
            if br.all_reads_within_bounds().is_err() {
                return Err(if allow_truncated_group {
                    StatusCode::NotEnoughBytes
                } else {
                    StatusCode::GenericError
                });
            }
            next_channel += 1;
        }
        guard_armed = false;
        if !reader.check_ans_final_state() {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    })();
    if guard_armed {
        for ch in image.channel.iter_mut().skip(next_channel) {
            zero_fill_image(&mut ch.plane);
        }
    }
    result
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the original public modular decompression boundary"
)]
pub fn modular_generic_decompress(
    br: &mut BitReader<'_>,
    image: &mut Image,
    header: Option<&mut GroupHeader>,
    group_id: usize,
    options: &mut ModularOptions,
    undo_transforms: bool,
    tree: Option<&Tree>,
    code: Option<&ANSCode>,
    ctx_map: Option<&[u8]>,
    allow_truncated_group: bool,
) -> Status {
    let mut req_sizes = Vec::with_capacity(image.channel.len());
    for c in &image.channel {
        req_sizes.push((c.w, c.h));
    }
    let mut local_header = GroupHeader::default();
    let header = header.unwrap_or(&mut local_header);
    let _bit_pos = br.total_bits_consumed();
    let dec_status = modular_decode(
        br,
        image,
        header,
        group_id,
        options,
        tree,
        code,
        ctx_map,
        allow_truncated_group,
    );
    if !allow_truncated_group {
        dec_status?;
    }
    if dec_status.is_err_and(StatusCode::is_fatal_error) {
        return dec_status;
    }
    if undo_transforms {
        image.undo_transforms(&header.wp_header, None);
    }
    if image.error {
        return Err(StatusCode::GenericError);
    }
    if undo_transforms {
        if image.channel.len() != req_sizes.len() {
            return Err(StatusCode::GenericError);
        }
        for (c, &(w, h)) in req_sizes.iter().enumerate() {
            if w != image.channel[c].w || h != image.channel[c].h {
                return Err(StatusCode::GenericError);
            }
        }
    }
    dec_status
}
