// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! FindTextLikePatches from lib/jxl/enc_patch_dictionary.cc, serial, without
//! the debug-image dumps (WantDebugOutput is false without a debug callback).
//!
//! The admitted encoder profile uses only whether detection returns patches:
//! a non-empty result enters the reference-frame encode, which is not
//! translated (step 5). The duplicate removal sorts PatchInfo pairs; equal
//! patches are adjacent under any strict weak ordering consistent with the
//! source's operator==, so the surviving groups and therefore the result's
//! emptiness and its patch shapes do not depend on the sort algorithm. The
//! returned shapes are in sorted order of (xsize, ysize, pixels).
//!
//! Contraction map: `is_similar_v`'s `distance += std::abs(v1[c] - v2[c]) *
//! kChannelWeights[c]` is one front-end fmuladd site, fused on AArch64 and
//! separate on x86-64.

use super::base::float::mul_add_f32;
use super::base::override_option::{Override, apply_override};
use super::base::status::StatusOr;
use super::enc_params::CompressParams;
use super::frame_dimensions::FrameDimensions;
use super::image::{Image3, Plane};
use super::image_ops::{zero_fill_image, zero_fill_image3};
use std::cmp::Ordering;

/// kMaxPatchSize from enc_patch_dictionary.h.
pub const MAX_PATCH_SIZE: usize = 32;
const PATCH_SIDE: usize = 4;

struct PatchColorspaceInfo {
    channel_dequant: [f32; 3],
    channel_weights: [f32; 3],
}
impl PatchColorspaceInfo {
    fn new(is_xyb: bool) -> Self {
        if is_xyb {
            Self {
                channel_dequant: [0.01615, 0.08875, 0.1922],
                channel_weights: [30.0, 3.0, 1.0],
            }
        } else {
            Self {
                channel_dequant: [20.0f32 / 255.0, 22.0f32 / 255.0, 20.0f32 / 255.0],
                channel_weights: [
                    (0.017f64 * 255.0) as f32,
                    (0.02f64 * 255.0) as f32,
                    (0.017f64 * 255.0) as f32,
                ],
            }
        }
    }
    fn quantize(&self, val: f32, c: usize) -> i32 {
        let mut scaled = val / self.channel_dequant[c];
        // Clamping allows values outside of target range (int8_t); caller should
        // deal with out-of-range values.
        scaled = if scaled < -32768.0 {
            -32768.0
        } else if scaled > 32767.0 {
            32767.0
        } else {
            scaled
        };
        scaled.trunc() as i32
    }
    fn is_similar_v(&self, v1: [f32; 3], v2: [f32; 3], threshold: f32) -> bool {
        let mut distance = 0.0f32;
        for c in 0..3 {
            distance = mul_add_f32((v1[c] - v2[c]).abs(), self.channel_weights[c], distance);
        }
        distance <= threshold
    }
}

/// A quantized patch, compared by shape and pixels as QuantizedPatch is.
#[derive(Clone)]
pub struct QuantizedPatch {
    pub xsize: usize,
    pub ysize: usize,
    pub pixels: [Vec<i8>; 3],
}
impl QuantizedPatch {
    fn same(&self, other: &Self) -> bool {
        self.cmp_key(other) == Ordering::Equal
    }
    fn cmp_key(&self, other: &Self) -> Ordering {
        if self.xsize != other.xsize {
            return self.xsize.cmp(&other.xsize);
        }
        if self.ysize != other.ysize {
            return self.ysize.cmp(&other.ysize);
        }
        let n = self.xsize * self.ysize;
        for c in 0..3 {
            // memcmp compares unsigned bytes.
            let a = self.pixels[c][..n].iter().map(|&v| v as u8);
            let b = other.pixels[c][..n].iter().map(|&v| v as u8);
            match a.cmp(b) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

/// Patches with their occurrences, after the source's duplicate removal and
/// minimum-size check.
pub struct PatchInfo {
    pub patch: QuantizedPatch,
    pub positions: Vec<(u32, u32)>,
}

/// FindTextLikePatches.
pub fn find_text_like_patches(
    cparams: &CompressParams<'_>,
    opsin: &Image3<f32>,
    frame_dim: &FrameDimensions,
    is_xyb: bool,
) -> StatusOr<Vec<PatchInfo>> {
    let mut info: Vec<PatchInfo> = Vec::new();
    if cparams.patches == Override::OFF {
        return Ok(info);
    }
    let pci = PatchColorspaceInfo::new(is_xyb);
    const SIMILAR_THRESHOLD: f32 = 0.8;

    let opsin_stride = opsin.pixels_per_row() as usize;
    let opsin_rows: [&[f32]; 3] =
        std::array::from_fn(|c| opsin.plane(c).pixels().expect("allocated opsin plane"));
    let pick = |x: usize, y: usize| -> [f32; 3] {
        let offset = y * opsin_stride + x;
        [
            opsin_rows[0][offset],
            opsin_rows[1][offset],
            opsin_rows[2][offset],
        ]
    };
    let is_same_color = |x: usize, y: usize, c: [f32; 3]| -> usize {
        let offset = y * opsin_stride + x;
        for i in 0..3 {
            if f64::from((c[i] - opsin_rows[i][offset]).abs()) > 1e-4 {
                return 0;
            }
        }
        1
    };

    // Look for kPatchSide size squares, naturally aligned, that all have the same
    // pixel values.
    let mut is_screenshot_like = Plane::<u8>::create(
        frame_dim.xsize.div_ceil(PATCH_SIDE),
        frame_dim.ysize.div_ceil(PATCH_SIDE),
    )?;
    zero_fill_image(&mut is_screenshot_like);
    let pw = frame_dim.xsize / PATCH_SIDE;
    let ph = frame_dim.ysize / PATCH_SIDE;

    let flat_patch = |ox: usize, oy: usize, base: [f32; 3]| -> bool {
        for iy in 0..PATCH_SIDE {
            for ix in 0..PATCH_SIDE {
                if is_same_color(ox + ix, oy + iy, base) == 0 {
                    return false;
                }
            }
        }
        true
    };

    let mut screenshot_area_seeds = 0u32;
    let can_have_seeds = pw >= 3 && ph >= 3;
    if can_have_seeds {
        // RunOnPool(pool, 1, ph - 2): rows 1 .. ph - 3.
        for py in 1..ph - 2 {
            let mut found = 0u32;
            for px in 1..=pw - 2 {
                let (ox, oy) = (px * PATCH_SIDE, py * PATCH_SIDE);
                let base = pick(ox, oy);
                if !flat_patch(ox, oy, base) {
                    continue;
                }
                let mut num_same = 0;
                let mut y = (py - 1) * PATCH_SIDE;
                while y <= (py + 1) * PATCH_SIDE {
                    let mut x = (px - 1) * PATCH_SIDE;
                    while x <= (px + 1) * PATCH_SIDE {
                        num_same += is_same_color(x, y, base);
                        x += PATCH_SIDE;
                    }
                    y += PATCH_SIDE;
                }
                // Too few equal pixels nearby.
                if num_same < 8 {
                    continue;
                }
                is_screenshot_like.row_mut(py)[px] = 1;
                found += 1;
            }
            screenshot_area_seeds = screenshot_area_seeds.wrapping_add(found);
        }
    }

    const SEARCH_RADIUS: i32 = 1;

    let num_seeds = screenshot_area_seeds as usize;
    if !apply_override(cparams.patches, num_seeds > 0) {
        return Ok(info);
    }

    // Search for "similar enough" pixels near the screenshot-like areas.
    let mut is_background = Plane::<u8>::create(frame_dim.xsize, frame_dim.ysize)?;
    zero_fill_image(&mut is_background);
    let mut background = Image3::<f32>::create(frame_dim.xsize, frame_dim.ysize)?;
    zero_fill_image3(&mut background);
    const DISTANCE_LIMIT: u32 = 50;
    let background_stride = background.pixels_per_row() as usize;
    let is_background_stride = is_background.pixels_per_row() as usize;
    let mut queue: Vec<((i32, i32), (i32, i32))> =
        Vec::with_capacity(2 * num_seeds * PATCH_SIDE * PATCH_SIDE);
    let mut queue_front = 0usize;
    {
        let is_bg = is_background.pixels_mut().expect("allocated plane");
        if can_have_seeds {
            for py in 1..ph - 1 {
                for px in 1..pw - 1 {
                    if is_screenshot_like.row(py)[px] == 0 {
                        continue;
                    }
                    for y in py * PATCH_SIDE..(py + 1) * PATCH_SIDE {
                        for x in px * PATCH_SIDE..(px + 1) * PATCH_SIDE {
                            let p = (x as i32, y as i32);
                            queue.push((p, p));
                            is_bg[y * is_background_stride + x] = 1;
                        }
                    }
                }
            }
        }
        let mut background_rows: Vec<&mut [f32]> = Vec::with_capacity(3);
        {
            let [p0, p1, p2] = background.planes_mut();
            background_rows.push(p0.pixels_mut().expect("allocated plane"));
            background_rows.push(p1.pixels_mut().expect("allocated plane"));
            background_rows.push(p2.pixels_mut().expect("allocated plane"));
        }
        while queue_front < queue.len() {
            let (cur, src) = queue[queue_front];
            queue_front += 1;
            for c in 0..3 {
                let clr = opsin_rows[c][src.1 as usize * opsin_stride + src.0 as usize];
                background_rows[c][cur.1 as usize * background_stride + cur.0 as usize] = clr;
            }
            for dx in -SEARCH_RADIUS..=SEARCH_RADIUS {
                for dy in -SEARCH_RADIUS..=SEARCH_RADIUS {
                    let next = (cur.0 + dx, cur.1 + dy);
                    if next.0 < 0
                        || next.1 < 0
                        || next.0 as u32 >= frame_dim.xsize as u32
                        || next.1 as u32 >= frame_dim.ysize as u32
                    {
                        continue;
                    }
                    let bg = &mut is_bg[next.1 as usize * is_background_stride + next.0 as usize];
                    if *bg != 0 {
                        continue;
                    }
                    if ((next.0 - src.0).abs() + (next.1 - src.1).abs()) as u32 > DISTANCE_LIMIT {
                        continue;
                    }
                    let v1 = pick(src.0 as usize, src.1 as usize);
                    let v2 = pick(next.0 as usize, next.1 as usize);
                    if pci.is_similar_v(v1, v2, SIMILAR_THRESHOLD) {
                        queue.push((next, src));
                        *bg = 1;
                    }
                }
            }
        }
    }
    queue.clear();

    const VERY_SIMILAR_THRESHOLD: f32 = 0.03;
    const HAS_SIMILAR_THRESHOLD: f32 = 0.03;
    let bg_rows: [&[f32]; 3] =
        std::array::from_fn(|c| background.plane(c).pixels().expect("allocated plane"));
    let bg_pick = |x: u32, y: u32| -> [f32; 3] {
        let offset = y as usize * background_stride + x as usize;
        [bg_rows[0][offset], bg_rows[1][offset], bg_rows[2][offset]]
    };
    let is_background_row = is_background.pixels().expect("allocated plane");

    const MIN_PEAK: i32 = 2;
    const HAS_SIMILAR_RADIUS: i32 = 2;

    // Find small CC outside the "similar enough" areas, compute bounding boxes,
    // and run heuristics to exclude some patches.
    let mut visited = Plane::<u8>::create(frame_dim.xsize, frame_dim.ysize)?;
    zero_fill_image(&mut visited);
    let visited_stride = visited.pixels_per_row() as usize;
    let visited_row = visited.pixels_mut().expect("allocated plane");
    let mut stack: Vec<(u32, u32)> = Vec::new();
    for y in 0..frame_dim.ysize {
        for x in 0..frame_dim.xsize {
            if is_background_row[y * is_background_stride + x] != 0 {
                continue;
            }
            stack.clear();
            stack.push((x as u32, y as u32));
            let (mut min_x, mut max_x, mut min_y, mut max_y) = (x, x, y, y);
            let mut reference = (0u32, 0u32);
            let mut found_border = false;
            let mut all_similar = true;
            while let Some(cur) = stack.pop() {
                let vi = cur.1 as usize * visited_stride + cur.0 as usize;
                if visited_row[vi] != 0 {
                    continue;
                }
                visited_row[vi] = 1;
                min_x = min_x.min(cur.0 as usize);
                max_x = max_x.max(cur.0 as usize);
                min_y = min_y.min(cur.1 as usize);
                max_y = max_y.max(cur.1 as usize);
                for dx in -SEARCH_RADIUS..=SEARCH_RADIUS {
                    for dy in -SEARCH_RADIUS..=SEARCH_RADIUS {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let next_first = cur.0 as i32 + dx;
                        let next_second = cur.1 as i32 + dy;
                        if next_first < 0
                            || next_second < 0
                            || next_first as u32 >= frame_dim.xsize as u32
                            || next_second as u32 >= frame_dim.ysize as u32
                        {
                            continue;
                        }
                        let next = (next_first as u32, next_second as u32);
                        if is_background_row
                            [next.1 as usize * is_background_stride + next.0 as usize]
                            == 0
                        {
                            stack.push(next);
                        } else if !found_border {
                            reference = next;
                            found_border = true;
                        } else if !pci.is_similar_v(
                            bg_pick(next.0, next.1),
                            bg_pick(reference.0, reference.1),
                            VERY_SIMILAR_THRESHOLD,
                        ) {
                            all_similar = false;
                        }
                    }
                }
            }
            if !found_border
                || !all_similar
                || max_x - min_x >= MAX_PATCH_SIZE
                || max_y - min_y >= MAX_PATCH_SIZE
            {
                continue;
            }
            let reference_color = bg_pick(reference.0, reference.1);
            let mut has_similar = false;
            let iy0 = (min_y as i32 - HAS_SIMILAR_RADIUS).max(0) as usize;
            let iy1 = (max_y + HAS_SIMILAR_RADIUS as usize + 1).min(frame_dim.ysize);
            let ix0 = (min_x as i32 - HAS_SIMILAR_RADIUS).max(0) as usize;
            let ix1 = (max_x + HAS_SIMILAR_RADIUS as usize + 1).min(frame_dim.xsize);
            for iy in iy0..iy1 {
                for ix in ix0..ix1 {
                    if pci.is_similar_v(reference_color, pick(ix, iy), HAS_SIMILAR_THRESHOLD) {
                        has_similar = true;
                    }
                }
            }
            if !has_similar {
                continue;
            }
            let xsize = max_x - min_x + 1;
            let ysize = max_y - min_y + 1;
            let mut patch = QuantizedPatch {
                xsize,
                ysize,
                pixels: std::array::from_fn(|_| vec![0i8; MAX_PATCH_SIZE * MAX_PATCH_SIZE]),
            };
            let mut too_big = false;
            let mut too_small = true;
            for c in [1usize, 0, 2] {
                for iy in min_y..=max_y {
                    for ix in min_x..=max_x {
                        let offset = (iy - min_y) * xsize + ix - min_x;
                        let fval = opsin_rows[c][iy * opsin_stride + ix] - reference_color[c];
                        let val = pci.quantize(fval, c);
                        let qval = val as i8;
                        patch.pixels[c][offset] = qval;
                        too_big |= val != i32::from(qval);
                        too_small &= val < MIN_PEAK && val > -MIN_PEAK;
                    }
                }
            }
            if too_small || too_big {
                continue;
            }
            info.push(PatchInfo {
                patch,
                positions: vec![(min_x as u32, min_y as u32)],
            });
        }
    }

    if info.is_empty() {
        return Ok(info);
    }

    // Remove duplicates.
    const MIN_PATCH_OCCURRENCES: usize = 2;
    info.sort_by(|a, b| {
        a.patch
            .cmp_key(&b.patch)
            .then_with(|| a.positions.cmp(&b.positions))
    });
    let mut unique = 0usize;
    for i in 1..info.len() {
        if info[i].patch.same(&info[unique].patch) {
            let positions = info[i].positions.clone();
            info[unique].positions.extend(positions);
        } else {
            if info[unique].positions.len() >= MIN_PATCH_OCCURRENCES {
                unique += 1;
            }
            info.swap(unique, i);
        }
    }
    if info[unique].positions.len() >= MIN_PATCH_OCCURRENCES {
        unique += 1;
    }
    info.truncate(unique);

    let mut max_patch_size = 0usize;
    for patch in &info {
        let pixels = patch.patch.xsize * patch.patch.ysize;
        if pixels > max_patch_size {
            max_patch_size = pixels;
        }
    }

    // don't use patches if all patches are smaller than this
    const MIN_MAX_PATCH_SIZE: usize = 20;
    if max_patch_size < MIN_MAX_PATCH_SIZE {
        info.clear();
    }

    Ok(info)
}
