// libwebp v1.6.0, src/utils/palette.c:
//
// Copyright 2023 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Utilities for palette analysis.
//
// Author: Vincent Rabaud (vrabaud@google.com)

// libwebp v1.6.0, src/utils/palette.h:
//
// Copyright 2023 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Utilities for palette analysis.
//
// Author: Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/utils/palette.c` and `src/utils/palette.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).
//!
//! Sorting (PORTING rule 8): `PrepareMapToPalette` and `GetColorPalette` sort
//! with `qsort`. `GetColorPalette` sorts distinct colors, and the comparator of
//! `PrepareMapToPalette` asserts that no two colors are equal, so no two keys
//! tie and the order is the ascending order of the values, which any sort
//! gives. A palette with repeated colors is outside the contract of
//! `PrepareMapToPalette` and is not compared.

use crate::dsp::lossless_common::vp8l_sub_pixels;
use crate::utils::color_cache_utils::vp8l_hash_pix;
use crate::utils::utils::{webp_safe_calloc, webp_safe_malloc};
use crate::webp::encode::WebPPicture;
use crate::webp::format_constants::MAX_PALETTE_SIZE;

// upstream: src/utils/palette.h PaletteSorting
/// The different ways a palette can be sorted.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaletteSorting(pub u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl PaletteSorting {
    // upstream: src/utils/palette.h kSortedDefault
    pub const kSortedDefault: Self = Self(0);
    // upstream: src/utils/palette.h kMinimizeDelta
    /// Sorts by minimizing L1 deltas between consecutive colors, giving more
    /// weight to RGB colors.
    pub const kMinimizeDelta: Self = Self(1);
    // upstream: src/utils/palette.h kModifiedZeng
    /// Implements the modified Zeng method from "A Survey on Palette Reordering
    /// Methods for Improving the Compression of Color-Indexed Images" by Armando
    /// J. Pinho and Antonio J. R. Neves.
    pub const kModifiedZeng: Self = Self(2);
    // upstream: src/utils/palette.h kUnusedPalette
    pub const kUnusedPalette: Self = Self(3);
    // upstream: src/utils/palette.h kPaletteSortingNum
    pub const kPaletteSortingNum: Self = Self(4);
}

/// The `MAX_PALETTE_SIZE` colors of the fixed C arrays.
const PALETTE_ENTRIES: usize = MAX_PALETTE_SIZE as usize;

/// Converts a count or an index of the palette code, which is never negative,
/// to a slice position.
fn position(value: i32) -> usize {
    usize::try_from(value).expect("a palette position is not negative")
}

/// The ARGB samples of `pic` and the position of its first row, which C reads
/// through `pic->argb`.
///
/// # Panics
///
/// Panics when the picture has no ARGB plane.
fn argb_plane<'p>(pic: &'p WebPPicture<'_>) -> (&'p [u32], usize) {
    let plane = pic.argb.expect("the picture has an ARGB plane");
    let samples = pic
        .argb_storage
        .region(plane.region)
        .expect("the ARGB region exists");
    (samples, plane.offset)
}

/// The position `rows` rows of `stride` pixels after `start`; a negative stride
/// walks upward, as the C pointer `argb += pic->argb_stride` does.
fn advance_rows(start: usize, stride: i32, rows: i32) -> usize {
    let start = i64::try_from(start).expect("a plane offset fits in i64");
    usize::try_from(start + i64::from(rows) * i64::from(stride))
        .expect("the row starts inside the plane")
}

// -----------------------------------------------------------------------------

// Palette reordering for smaller sum of deltas (and for smaller storage).

// upstream: src/utils/palette.c PaletteCompareColorsForQsort
/// The ascending order of two colors, which must differ.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the comparator of the sort takes references, as the qsort comparator takes pointers"
)]
fn palette_compare_colors_for_qsort(a: &u32, b: &u32) -> core::cmp::Ordering {
    debug_assert_ne!(a, b);
    if a < b {
        core::cmp::Ordering::Less
    } else {
        core::cmp::Ordering::Greater
    }
}

// upstream: src/utils/palette.c PaletteComponentDistance
#[must_use]
#[inline]
fn palette_component_distance(v: u32) -> u32 {
    if v <= 128 { v } else { 256 - v }
}

// upstream: src/utils/palette.c PaletteColorDistance
/// Computes a value that is related to the entropy created by the
/// palette entry diff.
///
/// Note that the last & 0xff is a no-operation in the next statement, but
/// removed by most compilers and is here only for regularity of the code.
#[must_use]
#[inline]
fn palette_color_distance(col1: u32, col2: u32) -> u32 {
    let diff = vp8l_sub_pixels(col1, col2);
    let k_more_weight_for_rgb_than_for_alpha: u32 = 9;
    let mut score: u32 = palette_component_distance(diff & 0xff);
    score += palette_component_distance((diff >> 8) & 0xff);
    score += palette_component_distance((diff >> 16) & 0xff);
    score *= k_more_weight_for_rgb_than_for_alpha;
    score += palette_component_distance((diff >> 24) & 0xff);
    score
}

// upstream: src/utils/palette.c SwapColor
#[inline]
fn swap_color(palette: &mut [u32], col1: usize, col2: usize) {
    palette.swap(col1, col2);
}

// upstream: src/utils/palette.c SearchColorNoIdx
/// Returns the index of `color` in the sorted palette `sorted` of size
/// `num_colors`.
///
/// `color` must be in the palette: C loops forever otherwise, and this panics
/// or loops in the same way.
#[must_use]
#[expect(
    clippy::comparison_chain,
    reason = "mirrors the if chain of the C search"
)]
pub fn search_color_no_idx(sorted: &[u32], color: u32, num_colors: i32) -> i32 {
    let mut low: i32 = 0;
    let mut hi: i32 = num_colors;
    if sorted[position(low)] == color {
        return low; // loop invariant: sorted[low] != color
    }
    loop {
        let mid = (low + hi) >> 1;
        if sorted[position(mid)] == color {
            return mid;
        } else if sorted[position(mid)] < color {
            low = mid;
        } else {
            hi = mid;
        }
    }
}

// upstream: src/utils/palette.c PrepareMapToPalette
/// Sort palette in increasing order and prepare an inverse mapping array.
///
/// The colors must be distinct; see the module documentation.
pub fn prepare_map_to_palette(
    palette: &[u32],
    num_colors: u32,
    sorted: &mut [u32],
    idx_map: &mut [u32],
) {
    let count = num_colors as usize;
    sorted[..count].copy_from_slice(&palette[..count]);
    sorted[..count].sort_unstable_by(palette_compare_colors_for_qsort);
    let num_colors_int = num_colors.cast_signed();
    for (i, &color) in (0..num_colors).zip(&palette[..count]) {
        idx_map[position(search_color_no_idx(sorted, color, num_colors_int))] = i;
    }
}

//------------------------------------------------------------------------------

// upstream: src/utils/palette.c COLOR_HASH_SIZE
const COLOR_HASH_SIZE: i32 = MAX_PALETTE_SIZE * 4;
// upstream: src/utils/palette.c COLOR_HASH_RIGHT_SHIFT
/// 32 - log2(`COLOR_HASH_SIZE`).
const COLOR_HASH_RIGHT_SHIFT: i32 = 22;

// upstream: src/utils/palette.c GetColorPalette
/// Returns count of unique colors in `pic`, assuming `pic.use_argb` is true.
/// If the unique color count is more than `MAX_PALETTE_SIZE`, returns
/// `MAX_PALETTE_SIZE`+1.
/// If `palette` is not `None` and the number of unique colors is less than or
/// equal to `MAX_PALETTE_SIZE`, also outputs the actual unique colors into
/// `palette` in a sorted order. Note: `palette` is assumed to be an array
/// already allocated with at least `MAX_PALETTE_SIZE` elements.
///
/// # Panics
///
/// Panics when the picture has no ARGB plane or a row lies outside it.
#[must_use]
pub fn get_color_palette(pic: &WebPPicture<'_>, palette: Option<&mut [u32]>) -> i32 {
    let mut num_colors: i32 = 0;
    let mut in_use = [0_u8; COLOR_HASH_SIZE as usize];
    let mut colors = [0_u32; COLOR_HASH_SIZE as usize];
    let (argb, first_row) = argb_plane(pic);
    let width = pic.width;
    let height = pic.height;
    let mut last_pix: u32 = !argb[first_row]; // so we're sure that last_pix != argb[0]
    debug_assert!(pic.use_argb != 0);

    for y in 0..height {
        // argb += pic->argb_stride after each row, as in C.
        let row = advance_rows(first_row, pic.argb_stride, y);
        for x in 0..width {
            let pix = argb[row + position(x)];
            if pix == last_pix {
                continue;
            }
            last_pix = pix;
            let mut key = vp8l_hash_pix(last_pix, COLOR_HASH_RIGHT_SHIFT);
            loop {
                if in_use[position(key)] == 0 {
                    colors[position(key)] = last_pix;
                    in_use[position(key)] = 1;
                    num_colors += 1;
                    if num_colors > MAX_PALETTE_SIZE {
                        return MAX_PALETTE_SIZE + 1; // Exact count not needed.
                    }
                    break;
                } else if colors[position(key)] == last_pix {
                    break; // The color is already there.
                }
                // Some other color sits here, so do linear conflict resolution.
                key += 1;
                key &= COLOR_HASH_SIZE - 1; // Key mask.
            }
        }
    }

    if let Some(palette) = palette {
        // Fill the colors into palette.
        num_colors = 0;
        for (&used, &color) in in_use.iter().zip(&colors) {
            if used != 0 {
                palette[position(num_colors)] = color;
                num_colors += 1;
            }
        }
        palette[..position(num_colors)].sort_unstable_by(palette_compare_colors_for_qsort);
    }
    num_colors
}

// -----------------------------------------------------------------------------

// upstream: src/utils/palette.c PaletteHasNonMonotonousDeltas
/// The palette has been sorted by alpha. This function checks if the other
/// components of the palette have a monotonic development with regards to
/// position in the palette. If all have monotonic development, there is
/// no benefit to re-organize them greedily. A monotonic development
/// would be spotted in green-only situations (like lossy alpha) or gray-scale
/// images.
#[must_use]
fn palette_has_non_monotonous_deltas(palette: &[u32], num_colors: i32) -> bool {
    let mut predict: u32 = 0x00_0000;
    let mut sign_found: u8 = 0x00;
    for &color in &palette[..position(num_colors)] {
        let diff = vp8l_sub_pixels(color, predict);
        let rd = ((diff >> 16) & 0xff) as u8;
        let gd = ((diff >> 8) & 0xff) as u8;
        let bd = (diff & 0xff) as u8;
        if rd != 0x00 {
            sign_found |= if rd < 0x80 { 1 } else { 2 };
        }
        if gd != 0x00 {
            sign_found |= if gd < 0x80 { 8 } else { 16 };
        }
        if bd != 0x00 {
            sign_found |= if bd < 0x80 { 64 } else { 128 };
        }
        predict = color;
    }
    // two consequent signs.
    (i32::from(sign_found) & (i32::from(sign_found) << 1)) != 0
}

// upstream: src/utils/palette.c PaletteSortMinimizeDeltas
#[expect(
    clippy::needless_range_loop,
    reason = "mirrors the C index loops over the palette"
)]
fn palette_sort_minimize_deltas(palette_sorted: &[u32], mut num_colors: i32, palette: &mut [u32]) {
    let mut predict: u32 = 0x0000_0000;
    let count = position(num_colors);
    palette[..count].copy_from_slice(&palette_sorted[..count]);
    if !palette_has_non_monotonous_deltas(palette_sorted, num_colors) {
        return;
    }
    // Find greedily always the closest color of the predicted color to minimize
    // deltas in the palette. This reduces storage needs since the
    // palette is stored with delta encoding.
    if num_colors > 17 && palette[0] == 0 {
        num_colors -= 1;
        swap_color(palette, position(num_colors), 0);
    }
    for i in 0..position(num_colors) {
        let mut best_ix = i;
        let mut best_score: u32 = !0;
        for k in i..position(num_colors) {
            let cur_score = palette_color_distance(palette[k], predict);
            if best_score > cur_score {
                best_score = cur_score;
                best_ix = k;
            }
        }
        swap_color(palette, best_ix, i);
        predict = palette[i];
    }
}

// -----------------------------------------------------------------------------
// Modified Zeng method from "A Survey on Palette Reordering
// Methods for Improving the Compression of Color-Indexed Images" by Armando J.
// Pinho and Antonio J. R. Neves.

/// Truncates a palette index to the `uint8_t` of the C code.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion to uint8_t"
)]
fn index_byte(value: u32) -> u8 {
    value as u8
}

// upstream: src/utils/palette.c CoOccurrenceFindMax
/// Finds the biggest cooccurrence in the matrix. Returns `(c1, c2)`, the two
/// out-parameters of the C function.
#[must_use]
fn co_occurrence_find_max(cooccurrence: &[u32], num_colors: u32) -> (u8, u8) {
    // Find the index that is most frequently located adjacent to other
    // (different) indexes.
    let n = num_colors as usize;
    let mut best_sum: u32 = 0;
    let mut c1: u8 = 0;
    for i in 0..num_colors {
        let mut sum: u32 = 0;
        for j in 0..n {
            sum = sum.wrapping_add(cooccurrence[i as usize * n + j]);
        }
        if sum > best_sum {
            best_sum = sum;
            c1 = index_byte(i);
        }
    }
    // Find the index that is most frequently found adjacent to *c1.
    let mut c2: u8 = 0;
    let mut best_cooccurrence: u32 = 0;
    for i in 0..num_colors {
        let value = cooccurrence[usize::from(c1) * n + i as usize];
        if value > best_cooccurrence {
            best_cooccurrence = value;
            c2 = index_byte(i);
        }
    }
    debug_assert_ne!(c1, c2);
    (c1, c2)
}

// upstream: src/utils/palette.c CoOccurrenceBuild
/// Builds the cooccurrence matrix. Returns `false` on memory allocation error.
///
/// The counters are `uint32_t` and wrap, as in C. Every pixel's color must be
/// in `palette`.
#[must_use]
fn co_occurrence_build(
    pic: &WebPPicture<'_>,
    palette: &[u32],
    num_colors: u32,
    cooccurrence: &mut [u32],
) -> bool {
    let (argb, first_row) = argb_plane(pic);
    let mut prev_pix: u32 = !argb[first_row];
    let mut prev_idx: u32 = 0;
    let mut idx_map = [0_u32; PALETTE_ENTRIES];
    let mut palette_sorted = [0_u32; PALETTE_ENTRIES];
    let Some(mut lines) = webp_safe_malloc::<u32>(i64::from(2 * pic.width).cast_unsigned(), 4)
    else {
        return false;
    };
    let width = position(pic.width);
    // C leaves the lines uninitialized and reads only what it wrote.
    lines.resize(2 * width, 0);
    // line_top is &lines[0] and line_current is &lines[pic->width], as in C.
    let (mut line_top, mut line_current) = lines.split_at_mut(width);
    prepare_map_to_palette(palette, num_colors, &mut palette_sorted, &mut idx_map);
    let n = num_colors as usize;
    let num_colors_int = num_colors.cast_signed();
    for y in 0..pic.height {
        // src += pic->argb_stride after each row, as in C.
        let src = advance_rows(first_row, pic.argb_stride, y);
        for x in 0..width {
            let pix = argb[src + x];
            if pix != prev_pix {
                prev_idx =
                    idx_map[position(search_color_no_idx(&palette_sorted, pix, num_colors_int))];
                prev_pix = pix;
            }
            line_current[x] = prev_idx;
            // 4-connectivity is what works best as mentioned in "On the relation
            // between Memon's and the modified Zeng's palette reordering methods".
            if x > 0 && prev_idx != line_current[x - 1] {
                let left_idx = line_current[x - 1] as usize;
                let at = prev_idx as usize * n + left_idx;
                cooccurrence[at] = cooccurrence[at].wrapping_add(1);
                let at = left_idx * n + prev_idx as usize;
                cooccurrence[at] = cooccurrence[at].wrapping_add(1);
            }
            if y > 0 && prev_idx != line_top[x] {
                let top_idx = line_top[x] as usize;
                let at = prev_idx as usize * n + top_idx;
                cooccurrence[at] = cooccurrence[at].wrapping_add(1);
                let at = top_idx * n + prev_idx as usize;
                cooccurrence[at] = cooccurrence[at].wrapping_add(1);
            }
        }
        core::mem::swap(&mut line_top, &mut line_current);
    }
    true
}

// upstream: src/utils/palette.c Sum
#[derive(Clone, Copy, Debug, Default)]
struct Sum {
    index: u8,
    sum: u32,
}

// upstream: src/utils/palette.c PaletteSortModifiedZeng
/// Returns `false` on memory allocation error, where `palette` is left as it
/// is.
///
/// The signed `delta` sums of the C code overflow only for co-occurrence
/// counts that no picture within the size limits reaches.
#[must_use]
fn palette_sort_modified_zeng(
    pic: &WebPPicture<'_>,
    palette_in: &[u32],
    num_colors: u32,
    palette: &mut [u32],
) -> bool {
    let mut remapping = [0_u8; PALETTE_ENTRIES];
    let mut sums = [Sum::default(); PALETTE_ENTRIES];
    // TODO(vrabaud) check whether one color images should use palette or not.
    if num_colors <= 1 {
        return true;
    }
    let n = num_colors as usize;
    // Build the co-occurrence matrix.
    let Some(mut cooccurrence) =
        webp_safe_calloc::<u32>(u64::from(num_colors.wrapping_mul(num_colors)), 4)
    else {
        return false;
    };
    if !co_occurrence_build(pic, palette_in, num_colors, &mut cooccurrence) {
        return false;
    }

    // Initialize the mapping list with the two best indices.
    (remapping[0], remapping[1]) = co_occurrence_find_max(&cooccurrence, num_colors);

    // We need to append and prepend to the list of remapping. To this end, we
    // actually define the next start/end of the list as indices in a vector (with
    // a wrap around when the end is reached).
    let mut first: u32 = 0;
    let mut last: u32 = 1;
    let mut num_sums: u32 = num_colors - 2; // -2 because we know the first two values
    if num_sums > 0 {
        // Initialize the sums with the first two remappings and find the best one
        let mut best_sum: usize = 0;
        sums[0].index = 0;
        sums[0].sum = 0;
        let mut j: usize = 0;
        for i in 0..num_colors {
            if i == u32::from(remapping[0]) || i == u32::from(remapping[1]) {
                continue;
            }
            sums[j].index = index_byte(i);
            sums[j].sum = cooccurrence[i as usize * n + usize::from(remapping[0])]
                .wrapping_add(cooccurrence[i as usize * n + usize::from(remapping[1])]);
            if sums[j].sum > sums[best_sum].sum {
                best_sum = j;
            }
            j += 1;
        }

        while num_sums > 0 {
            let best_index = sums[best_sum].index;
            // Compute delta to know if we need to prepend or append the best index.
            let mut delta: i32 = 0;
            let n_placed: i32 = (num_colors - num_sums).cast_signed();
            let ind = first;
            let mut j: u32 = 0;
            while (ind + j) % num_colors != last + 1 {
                let l_j = u16::from(remapping[((ind + j) % num_colors) as usize]);
                delta += (n_placed - 1 - 2 * j.cast_signed())
                    * cooccurrence[usize::from(best_index) * n + usize::from(l_j)].cast_signed();
                j += 1;
            }
            if delta > 0 {
                first = if first == 0 {
                    num_colors - 1
                } else {
                    first - 1
                };
                remapping[first as usize] = best_index;
            } else {
                last += 1;
                remapping[last as usize] = best_index;
            }
            // Remove best_sum from sums.
            sums[best_sum] = sums[(num_sums - 1) as usize];
            num_sums -= 1;
            // Update all the sums and find the best one.
            best_sum = 0;
            for i in 0..num_sums as usize {
                sums[i].sum = sums[i].sum.wrapping_add(
                    cooccurrence[usize::from(best_index) * n + usize::from(sums[i].index)],
                );
                if sums[i].sum > sums[best_sum].sum {
                    best_sum = i;
                }
            }
        }
    }
    debug_assert_eq!((last + 1) % num_colors, first);

    // Re-map the palette.
    for i in 0..num_colors {
        palette[i as usize] =
            palette_in[usize::from(remapping[((first + i) % num_colors) as usize])];
    }
    true
}

// -----------------------------------------------------------------------------

// upstream: src/utils/palette.c PaletteSort
/// Sorts the palette according to the criterion defined by `method`.
/// `palette_sorted` is the input palette sorted lexicographically, as done in
/// `PrepareMapToPalette`. Returns `false` on memory allocation error.
/// For `kSortedDefault` and `kMinimizeDelta` methods, 0 (if present) is set as
/// the last element to optimize later storage.
///
/// C asserts and returns 0 for `kUnusedPalette`, `kPaletteSortingNum` and
/// other values; this debug-asserts and returns `false`.
#[must_use]
pub fn palette_sort(
    method: PaletteSorting,
    pic: &WebPPicture<'_>,
    palette_sorted: &[u32],
    num_colors: u32,
    palette: &mut [u32],
) -> bool {
    let count = num_colors as usize;
    match method {
        PaletteSorting::kSortedDefault => {
            if palette_sorted[0] == 0 && num_colors > 17 {
                palette[..count - 1].copy_from_slice(&palette_sorted[1..count]);
                palette[count - 1] = 0;
            } else {
                palette[..count].copy_from_slice(&palette_sorted[..count]);
            }
            true
        }
        PaletteSorting::kMinimizeDelta => {
            palette_sort_minimize_deltas(palette_sorted, num_colors.cast_signed(), palette);
            true
        }
        PaletteSorting::kModifiedZeng => {
            palette_sort_modified_zeng(pic, palette_sorted, num_colors, palette)
        }
        _ => {
            debug_assert!(false, "PaletteSort needs a sorting method");
            false
        }
    }
}
