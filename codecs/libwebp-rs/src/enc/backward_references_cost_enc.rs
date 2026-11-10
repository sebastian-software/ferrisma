// libwebp v1.6.0, src/enc/backward_references_cost_enc.c:
//
// Copyright 2017 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Improves a given set of backward references by analyzing its bit cost.
// The algorithm is similar to the Zopfli compression algorithm but tailored to
// images.
//
// Author: Vincent Rabaud (vrabaud@google.com)
//

//! Translation of `src/enc/backward_references_cost_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).

use crate::dsp::lossless_common::{
    LOG_2_PRECISION_BITS, WEBP_INT64_MAX, div_round, vp8l_fast_log2, vp8l_prefix_encode_bits,
};
use crate::enc::backward_references_enc::{
    MAX_LENGTH, VP8LBackwardRefs, VP8LHashChain, copy_len, pix_or_copy_create_cache_idx,
    pix_or_copy_create_copy, pix_or_copy_create_literal, safe_malloc_test, to_index,
    vp8l_backward_refs_cursor_add, vp8l_clear_backward_refs, vp8l_distance_to_plane_code,
    vp8l_hash_chain_find_copy, vp8l_hash_chain_find_offset,
};
use crate::enc::histogram_enc::{
    vp8l_allocate_histogram, vp8l_histogram_init, vp8l_histogram_num_codes,
    vp8l_histogram_store_refs,
};
use crate::utils::color_cache_utils::{
    VP8LColorCache, vp8l_color_cache_clear, vp8l_color_cache_contains, vp8l_color_cache_init,
    vp8l_color_cache_insert,
};
use crate::utils::utils::webp_safe_malloc;
use crate::webp::format_constants::{NUM_DISTANCE_CODES, NUM_LENGTH_CODES};

// upstream: src/enc/backward_references_cost_enc.c VALUES_IN_BYTE
const VALUES_IN_BYTE: i32 = 256;

// upstream: src/enc/backward_references_cost_enc.c CostModel
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CostModel {
    alpha: [u32; VALUES_IN_BYTE as usize],
    red: [u32; VALUES_IN_BYTE as usize],
    blue: [u32; VALUES_IN_BYTE as usize],
    distance: [u32; NUM_DISTANCE_CODES as usize],
    /// `VP8LHistogramNumCodes(cache_bits)` entries, which C carves from the end
    /// of the same allocation.
    literal: Vec<u32>,
}

impl CostModel {
    /// The C `sizeof(CostModel)`: four arrays and a pointer.
    pub const C_SIZE: usize = 4 * (3 * 256 + 40) + size_of::<usize>();
}

// upstream: src/enc/backward_references_cost_enc.c ConvertPopulationCountTableToBitEstimates
fn convert_population_count_table_to_bit_estimates(
    num_symbols: i32,
    population_counts: &[u32],
    output: &mut [u32],
) {
    let n = to_index(num_symbols);
    let mut sum: u32 = 0;
    let mut nonzeros: i32 = 0;
    for &count in &population_counts[..n] {
        sum = sum.wrapping_add(count);
        if count > 0 {
            nonzeros += 1;
        }
    }
    if nonzeros <= 1 {
        output[..n].fill(0);
    } else {
        let logsum = vp8l_fast_log2(sum);
        for (out, &count) in output[..n].iter_mut().zip(&population_counts[..n]) {
            *out = logsum.wrapping_sub(vp8l_fast_log2(count));
        }
    }
}

// upstream: src/enc/backward_references_cost_enc.c CostModelBuild
fn cost_model_build(
    m: &mut CostModel,
    xsize: i32,
    cache_bits: i32,
    refs: &VP8LBackwardRefs,
) -> bool {
    let Some(mut histo) = vp8l_allocate_histogram(cache_bits) else {
        return false;
    };

    // The following code is similar to VP8LHistogramCreate but converts the
    // distance to plane code.
    vp8l_histogram_init(&mut histo, cache_bits, /*init_arrays=*/ true);
    vp8l_histogram_store_refs(refs, Some(vp8l_distance_to_plane_code), xsize, &mut histo);

    convert_population_count_table_to_bit_estimates(
        vp8l_histogram_num_codes(histo.palette_code_bits),
        &histo.literal,
        &mut m.literal,
    );
    convert_population_count_table_to_bit_estimates(VALUES_IN_BYTE, &histo.red, &mut m.red);
    convert_population_count_table_to_bit_estimates(VALUES_IN_BYTE, &histo.blue, &mut m.blue);
    convert_population_count_table_to_bit_estimates(VALUES_IN_BYTE, &histo.alpha, &mut m.alpha);
    convert_population_count_table_to_bit_estimates(
        NUM_DISTANCE_CODES,
        &histo.distance,
        &mut m.distance,
    );
    true
}

// upstream: src/enc/backward_references_cost_enc.c GetLiteralCost
#[inline]
fn get_literal_cost(m: &CostModel, v: u32) -> i64 {
    i64::from(m.alpha[(v >> 24) as usize])
        + i64::from(m.red[((v >> 16) & 0xff) as usize])
        + i64::from(m.literal[((v >> 8) & 0xff) as usize])
        + i64::from(m.blue[(v & 0xff) as usize])
}

// upstream: src/enc/backward_references_cost_enc.c GetCacheCost
#[inline]
fn get_cache_cost(m: &CostModel, idx: u32) -> i64 {
    let literal_idx = to_index(VALUES_IN_BYTE + NUM_LENGTH_CODES) + idx as usize;
    i64::from(m.literal[literal_idx])
}

// upstream: src/enc/backward_references_cost_enc.c GetLengthCost
#[inline]
fn get_length_cost(m: &CostModel, length: u32) -> i64 {
    let (code, extra_bits) = vp8l_prefix_encode_bits(length.cast_signed());
    i64::from(m.literal[to_index(VALUES_IN_BYTE + code)])
        + (i64::from(extra_bits) << LOG_2_PRECISION_BITS)
}

// upstream: src/enc/backward_references_cost_enc.c GetDistanceCost
#[inline]
fn get_distance_cost(m: &CostModel, distance: u32) -> i64 {
    let (code, extra_bits) = vp8l_prefix_encode_bits(distance.cast_signed());
    i64::from(m.distance[to_index(code)]) + (i64::from(extra_bits) << LOG_2_PRECISION_BITS)
}

// upstream: src/enc/backward_references_cost_enc.c AddSingleLiteralWithCostModel
#[inline]
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of AddSingleLiteralWithCostModel has eight parameters"
)]
fn add_single_literal_with_cost_model(
    argb: &[u32],
    hashers: &mut VP8LColorCache,
    cost_model: &CostModel,
    idx: usize,
    use_color_cache: bool,
    prev_cost: i64,
    cost: &mut [i64],
    dist_array: &mut [u16],
) {
    let mut cost_val = prev_cost;
    let color = argb[idx];
    let ix = if use_color_cache {
        vp8l_color_cache_contains(hashers, color)
    } else {
        -1
    };
    if ix >= 0 {
        // use_color_cache is true and hashers contains color
        cost_val += div_round(get_cache_cost(cost_model, ix.cast_unsigned()) * 68, 100);
    } else {
        if use_color_cache {
            vp8l_color_cache_insert(hashers, color);
        }
        cost_val += div_round(get_literal_cost(cost_model, color) * 82, 100);
    }
    if cost[idx] > cost_val {
        cost[idx] = cost_val;
        dist_array[idx] = 1; // only one is inserted.
    }
}

// -----------------------------------------------------------------------------
// CostManager and interval handling

// upstream: src/enc/backward_references_cost_enc.c COST_CACHE_INTERVAL_SIZE_MAX
/// Empirical value to avoid high memory consumption but good for performance.
const COST_CACHE_INTERVAL_SIZE_MAX: i32 = 500;

// upstream: src/enc/backward_references_cost_enc.c CostInterval
/// To perform backward reference every pixel at index `index` is considered and
/// the cost for the `MAX_LENGTH` following pixels computed. Those following
/// pixels at index `index` + k (k from 0 to `MAX_LENGTH`) have a cost of:
///     cost = distance cost at index + `GetLengthCost(cost_model, k)`
/// and the minimum value is kept. `GetLengthCost(cost_model, k)` is cached in an
/// array of size `MAX_LENGTH`.
/// Instead of performing `MAX_LENGTH` comparisons per pixel, we keep track of
/// the minimal values using intervals of constant cost.
/// An interval is defined by the `index` of the pixel that generated it and
/// is only useful in a range of indices from `start` to `end` (exclusive), i.e.
/// it contains the minimum value for pixels between start and end.
/// Intervals are stored in a linked list and ordered by `start`. When a new
/// interval has a better value, old intervals are split or removed. There are
/// therefore no overlapping intervals.
///
/// The intervals live in the manager's arena; `previous` and `next` are arena
/// indices, `None` where C has `NULL`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CostInterval {
    cost: i64,
    start: i32,
    end: i32,
    index: i32,
    previous: Option<usize>,
    next: Option<usize>,
}

impl CostInterval {
    /// The C `sizeof(CostInterval)`.
    pub const C_SIZE: usize = 24 + 2 * size_of::<usize>();
}

// upstream: src/enc/backward_references_cost_enc.c CostCacheInterval
/// The `GetLengthCost(cost_model, k)` are cached in a `CostCacheInterval`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CostCacheInterval {
    cost: i64,
    start: i32,
    /// Exclusive.
    end: i32,
}

impl CostCacheInterval {
    /// The C `sizeof(CostCacheInterval)`.
    pub const C_SIZE: usize = 16;
}

// upstream: src/enc/backward_references_cost_enc.c COST_MANAGER_MAX_FREE_LIST
const COST_MANAGER_MAX_FREE_LIST: usize = 10;

// upstream: src/enc/backward_references_cost_enc.c CostManager
/// This structure is in charge of managing intervals and costs.
/// It caches the different `CostCacheInterval`, caches the different
/// `GetLengthCost(cost_model, k)` in `cost_cache` and the `CostInterval`'s
/// (whose `count` is limited by `COST_CACHE_INTERVAL_SIZE_MAX`).
///
/// `intervals` is the arena of every interval: its first
/// `COST_MANAGER_MAX_FREE_LIST` entries are C's `intervals[]` array, and the
/// entries after them are the intervals that C allocates one by one. `dist_array`
/// is the caller's array, which C keeps as a pointer.
struct CostManager<'d> {
    head: Option<usize>,
    /// The number of stored intervals.
    count: i32,
    cache_intervals: Vec<CostCacheInterval>,
    cache_intervals_size: usize,
    /// Contains the `GetLengthCost(cost_model, k)`; `MAX_LENGTH` entries, on the
    /// heap where C has an array in the struct.
    cost_cache: Vec<i64>,
    costs: Vec<i64>,
    dist_array: &'d mut [u16],
    /// Most of the time, we only need few intervals -> use a free-list, to avoid
    /// fragmentation with small allocs in most common cases.
    intervals: Vec<CostInterval>,
    free_intervals: Option<usize>,
    /// These are regularly malloc'd remains. This list can't grow larger than
    /// than size `COST_CACHE_INTERVAL_SIZE_MAX - COST_MANAGER_MAX_FREE_LIST`,
    /// note.
    recycled_intervals: Option<usize>,
}

// upstream: src/enc/backward_references_cost_enc.c CostIntervalAddToFreeList
fn cost_interval_add_to_free_list(manager: &mut CostManager<'_>, interval: usize) {
    manager.intervals[interval].next = manager.free_intervals;
    manager.free_intervals = Some(interval);
}

// upstream: src/enc/backward_references_cost_enc.c CostIntervalIsInFreeList
fn cost_interval_is_in_free_list(interval: usize) -> bool {
    interval < COST_MANAGER_MAX_FREE_LIST
}

// upstream: src/enc/backward_references_cost_enc.c CostManagerInitFreeList
fn cost_manager_init_free_list(manager: &mut CostManager<'_>) {
    manager.free_intervals = None;
    for i in 0..COST_MANAGER_MAX_FREE_LIST {
        cost_interval_add_to_free_list(manager, i);
    }
}

// upstream: src/enc/backward_references_cost_enc.c DeleteIntervalList
/// C frees the intervals of a list that it allocated one by one; the Rust
/// arena keeps them until `CostManagerClear` drops it, so only the list is
/// forgotten.
fn delete_interval_list(manager: &mut CostManager<'_>, interval: Option<usize>) {
    let mut interval = interval;
    while let Some(current) = interval {
        let next = manager.intervals[current].next;
        // C frees the intervals outside the free list here.
        let _ = cost_interval_is_in_free_list(current);
        interval = next;
    }
}

// upstream: src/enc/backward_references_cost_enc.c CostManagerClear
fn cost_manager_clear(manager: &mut CostManager<'_>) {
    manager.costs = Vec::new();
    manager.cache_intervals = Vec::new();

    // Clear the interval lists.
    delete_interval_list(manager, manager.head);
    manager.head = None;
    delete_interval_list(manager, manager.recycled_intervals);
    manager.recycled_intervals = None;

    // Reset pointers, 'count' and 'cache_intervals_size'.
    manager.count = 0;
    manager.cache_intervals_size = 0;
    manager.cost_cache.fill(0);
    manager.intervals.truncate(COST_MANAGER_MAX_FREE_LIST);
    manager.intervals.fill(CostInterval::default());
    cost_manager_init_free_list(manager);
}

// upstream: src/enc/backward_references_cost_enc.c CostManagerInit
/// Builds the manager over `dist_array` (C: `CostManagerInit`, which fills the
/// calloc'd manager). Returns `None` where C returns 0 after clearing the
/// manager.
fn cost_manager_init<'d>(
    dist_array: &'d mut [u16],
    pix_count: i32,
    cost_model: &CostModel,
) -> Option<Box<CostManager<'d>>> {
    let cost_cache_size = if pix_count > MAX_LENGTH {
        MAX_LENGTH
    } else {
        pix_count
    };
    let mut manager = Box::new(CostManager {
        head: None,
        count: 0,
        cache_intervals: Vec::new(),
        cache_intervals_size: 0,
        cost_cache: vec![0; MAX_LENGTH as usize],
        costs: Vec::new(),
        dist_array,
        intervals: vec![CostInterval::default(); COST_MANAGER_MAX_FREE_LIST],
        free_intervals: None,
        recycled_intervals: None,
    });
    cost_manager_init_free_list(&mut manager);

    // Fill in the 'cost_cache'.
    // Has to be done in two passes due to a GCC bug on i686
    // related to https://gcc.gnu.org/bugzilla/show_bug.cgi?id=323
    let cache = to_index(cost_cache_size);
    for (length, cost) in (0_u32..).zip(&mut manager.cost_cache[..cache]) {
        *cost = get_length_cost(cost_model, length);
    }
    manager.cache_intervals_size = 1;
    for i in 1..cache {
        // Get the number of bound intervals.
        if manager.cost_cache[i] != manager.cost_cache[i - 1] {
            manager.cache_intervals_size += 1;
        }
    }

    // With the current cost model, we usually have below 20 intervals.
    // The worst case scenario with a cost model would be if every length has a
    // different cost, hence MAX_LENGTH but that is impossible with the current
    // implementation that spirals around a pixel.
    debug_assert!(manager.cache_intervals_size <= to_index(MAX_LENGTH));
    let Some(mut cache_intervals) = webp_safe_malloc::<CostCacheInterval>(
        manager.cache_intervals_size as u64,
        CostCacheInterval::C_SIZE,
    ) else {
        cost_manager_clear(&mut manager);
        return None;
    };
    cache_intervals.resize(manager.cache_intervals_size, CostCacheInterval::default());
    manager.cache_intervals = cache_intervals;

    // Fill in the 'cache_intervals'.
    {
        let mut cur: usize = 0;
        let cost_cache = &manager.cost_cache;
        let intervals = &mut manager.cache_intervals;

        // Consecutive values in 'cost_cache' are compared and if a big enough
        // difference is found, a new interval is created and bounded.
        intervals[cur].start = 0;
        intervals[cur].end = 1;
        intervals[cur].cost = cost_cache[0];
        for i in 1..cost_cache_size {
            let cost_val = cost_cache[to_index(i)];
            if cost_val != intervals[cur].cost {
                cur += 1;
                // Initialize an interval.
                intervals[cur].start = i;
                intervals[cur].cost = cost_val;
            }
            intervals[cur].end = i + 1;
        }
        debug_assert_eq!(cur + 1, manager.cache_intervals_size);
    }

    let Some(mut costs) =
        webp_safe_malloc::<i64>(i64::from(pix_count).cast_unsigned(), size_of::<i64>())
    else {
        cost_manager_clear(&mut manager);
        return None;
    };
    // Set the initial 'costs' to INT64_MAX for every pixel as we will keep the
    // minimum.
    costs.resize(to_index(pix_count), WEBP_INT64_MAX);
    manager.costs = costs;

    Some(manager)
}

// upstream: src/enc/backward_references_cost_enc.c UpdateCost
/// Given the cost and the position that define an interval, update the cost
/// at pixel `i` if it is smaller than the previously computed value.
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the length to uint16_t"
)]
fn update_cost(manager: &mut CostManager<'_>, i: i32, position: i32, cost: i64) {
    let k = i - position;
    debug_assert!((0..MAX_LENGTH).contains(&k));

    let at = to_index(i);
    if manager.costs[at] > cost {
        manager.costs[at] = cost;
        manager.dist_array[at] = (k + 1) as u16;
    }
}

// upstream: src/enc/backward_references_cost_enc.c UpdateCostPerInterval
/// Given the cost and the position that define an interval, update the cost
/// for all the pixels between `start` and `end` excluded.
#[inline]
fn update_cost_per_interval(
    manager: &mut CostManager<'_>,
    start: i32,
    end: i32,
    position: i32,
    cost: i64,
) {
    for i in start..end {
        update_cost(manager, i, position, cost);
    }
}

// upstream: src/enc/backward_references_cost_enc.c ConnectIntervals
/// Given two intervals, make `prev` be the previous one of `next` in `manager`.
#[inline]
fn connect_intervals(manager: &mut CostManager<'_>, prev: Option<usize>, next: Option<usize>) {
    match prev {
        Some(prev) => manager.intervals[prev].next = next,
        None => manager.head = next,
    }

    if let Some(next) = next {
        manager.intervals[next].previous = prev;
    }
}

// upstream: src/enc/backward_references_cost_enc.c PopInterval
/// Pop an interval in the manager.
#[inline]
fn pop_interval(manager: &mut CostManager<'_>, interval: Option<usize>) {
    let Some(interval) = interval else {
        return;
    };

    let (previous, next) = (
        manager.intervals[interval].previous,
        manager.intervals[interval].next,
    );
    connect_intervals(manager, previous, next);
    if cost_interval_is_in_free_list(interval) {
        cost_interval_add_to_free_list(manager, interval);
    } else {
        // recycle regularly malloc'd intervals too
        manager.intervals[interval].next = manager.recycled_intervals;
        manager.recycled_intervals = Some(interval);
    }
    manager.count -= 1;
    debug_assert!(manager.count >= 0);
}

// upstream: src/enc/backward_references_cost_enc.c UpdateCostAtIndex
/// Update the cost at index `i` by going over all the stored intervals that
/// overlap with `i`.
/// If `do_clean_intervals` is set to something different than 0, intervals
/// that end before `i` will be popped.
#[inline]
fn update_cost_at_index(manager: &mut CostManager<'_>, i: i32, do_clean_intervals: bool) {
    let mut current = manager.head;

    while let Some(cur) = current
        && manager.intervals[cur].start <= i
    {
        let next = manager.intervals[cur].next;
        if manager.intervals[cur].end <= i {
            if do_clean_intervals {
                // We have an outdated interval, remove it.
                pop_interval(manager, Some(cur));
            }
        } else {
            let CostInterval { index, cost, .. } = manager.intervals[cur];
            update_cost(manager, i, index, cost);
        }
        current = next;
    }
}

// upstream: src/enc/backward_references_cost_enc.c PositionOrphanInterval
/// Given a current orphan interval and its previous interval, before
/// it was orphaned (which can be `NULL`), set it at the right place in the list
/// of intervals using the `start` ordering and the previous interval as a hint.
#[inline]
fn position_orphan_interval(
    manager: &mut CostManager<'_>,
    current: usize,
    mut previous: Option<usize>,
) {
    let current_start = manager.intervals[current].start;

    if previous.is_none() {
        previous = manager.head;
    }
    while let Some(prev) = previous
        && current_start < manager.intervals[prev].start
    {
        previous = manager.intervals[prev].previous;
    }
    while let Some(prev) = previous
        && let Some(next) = manager.intervals[prev].next
        && manager.intervals[next].start < current_start
    {
        previous = Some(next);
    }

    if let Some(prev) = previous {
        let next = manager.intervals[prev].next;
        connect_intervals(manager, Some(current), next);
    } else {
        let head = manager.head;
        connect_intervals(manager, Some(current), head);
    }
    connect_intervals(manager, previous, Some(current));
}

// upstream: src/enc/backward_references_cost_enc.c InsertInterval
/// Insert an interval in the list contained in the manager by starting at
/// `interval_in` as a hint. The intervals are sorted by `start` value.
#[inline]
fn insert_interval(
    manager: &mut CostManager<'_>,
    interval_in: Option<usize>,
    cost: i64,
    position: i32,
    start: i32,
    end: i32,
) {
    if start >= end {
        return;
    }
    if manager.count >= COST_CACHE_INTERVAL_SIZE_MAX {
        // Serialize the interval if we cannot store it.
        update_cost_per_interval(manager, start, end, position, cost);
        return;
    }
    let interval_new = if let Some(free) = manager.free_intervals {
        manager.free_intervals = manager.intervals[free].next;
        free
    } else if let Some(recycled) = manager.recycled_intervals {
        manager.recycled_intervals = manager.intervals[recycled].next;
        recycled
    } else {
        // malloc for good
        if !safe_malloc_test(1, CostInterval::C_SIZE) || manager.intervals.try_reserve(1).is_err() {
            // Write down the interval if we cannot create it.
            update_cost_per_interval(manager, start, end, position, cost);
            return;
        }
        manager.intervals.push(CostInterval::default());
        manager.intervals.len() - 1
    };

    let new = &mut manager.intervals[interval_new];
    new.cost = cost;
    new.index = position;
    new.start = start;
    new.end = end;
    position_orphan_interval(manager, interval_new, interval_in);

    manager.count += 1;
}

// upstream: src/enc/backward_references_cost_enc.c PushInterval
/// Given a new cost interval defined by its start at position, its length
/// value and `distance_cost`, add its contributions to the previous intervals
/// and costs. If handling the interval or one of its subintervals becomes to
/// heavy, its contribution is added to the costs right away.
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the length to uint16_t"
)]
fn push_interval(manager: &mut CostManager<'_>, distance_cost: i64, position: i32, len: i32) {
    // If the interval is small enough, no need to deal with the heavy
    // interval logic, just serialize it right away. This constant is empirical.
    const SKIP_DISTANCE: i32 = 10;
    let mut interval = manager.head;

    if len < SKIP_DISTANCE {
        for j in position..position + len {
            let k = j - position;
            debug_assert!((0..MAX_LENGTH).contains(&k));
            let cost_tmp = distance_cost + manager.cost_cache[to_index(k)];

            let at = to_index(j);
            if manager.costs[at] > cost_tmp {
                manager.costs[at] = cost_tmp;
                manager.dist_array[at] = (k + 1) as u16;
            }
        }
        return;
    }

    let mut i: usize = 0;
    while i < manager.cache_intervals_size && manager.cache_intervals[i].start < len {
        let cache_interval = manager.cache_intervals[i];
        // Define the intersection of the ith interval with the new one.
        let mut start = position + cache_interval.start;
        let end = position
            + if cache_interval.end > len {
                len
            } else {
                cache_interval.end
            };
        let cost = distance_cost + cache_interval.cost;

        while let Some(iv) = interval
            && manager.intervals[iv].start < end
        {
            let interval_next = manager.intervals[iv].next;
            let current = manager.intervals[iv];

            // Make sure we have some overlap
            if start >= current.end {
                interval = interval_next;
                continue;
            }

            if cost >= current.cost {
                // When intervals are represented, the lower, the better.
                // [**********************************************************[
                // start                                                    end
                //                   [----------------------------------[
                //                   interval->start        interval->end
                // If we are worse than what we already have, add whatever we have so
                // far up to interval.
                let start_new = current.end;
                insert_interval(manager, Some(iv), cost, position, start, current.start);
                start = start_new;
                if start >= end {
                    break;
                }
                interval = interval_next;
                continue;
            }

            if start <= current.start {
                if current.end <= end {
                    //                   [----------------------------------[
                    //                   interval->start        interval->end
                    // [**************************************************************[
                    // start                                                        end
                    // We can safely remove the old interval as it is fully included.
                    pop_interval(manager, Some(iv));
                } else {
                    //              [------------------------------------[
                    //              interval->start          interval->end
                    // [*****************************[
                    // start                       end
                    manager.intervals[iv].start = end;
                    break;
                }
            } else if end < current.end {
                // [--------------------------------------------------------------[
                // interval->start                                    interval->end
                //                     [*****************************[
                //                     start                       end
                // We have to split the old interval as it fully contains the new one.
                let end_original = current.end;
                manager.intervals[iv].end = start;
                insert_interval(
                    manager,
                    Some(iv),
                    current.cost,
                    current.index,
                    end,
                    end_original,
                );
                interval = manager.intervals[iv].next;
                break;
            } else {
                // [------------------------------------[
                // interval->start          interval->end
                //                     [*****************************[
                //                     start                       end
                manager.intervals[iv].end = start;
            }
            interval = interval_next;
        }
        // Insert the remaining interval from start to end.
        insert_interval(manager, interval, cost, position, start, end);
        i += 1;
    }
}

// upstream: src/enc/backward_references_cost_enc.c BackwardReferencesHashChainDistanceOnly
#[expect(
    clippy::too_many_lines,
    reason = "the C function BackwardReferencesHashChainDistanceOnly is translated as one function"
)]
#[expect(
    clippy::if_not_else,
    reason = "the C condition `offset != offset_prev`"
)]
fn backward_references_hash_chain_distance_only(
    xsize: i32,
    ysize: i32,
    argb: &[u32],
    cache_bits: i32,
    hash_chain: &VP8LHashChain,
    refs: &VP8LBackwardRefs,
    dist_array: &mut [u16],
) -> bool {
    let pix_count = xsize * ysize;
    let use_color_cache = cache_bits > 0;
    let literal_array_size = 4 * to_index(vp8l_histogram_num_codes(cache_bits));
    let cost_model_size = CostModel::C_SIZE + literal_array_size;
    let mut hashers = VP8LColorCache::default();
    let mut offset_prev: i32 = -1;
    let mut len_prev: i32 = -1;
    let mut offset_cost: i64 = -1;
    let mut first_offset_is_constant: i32 = -1; // initialized with 'impossible' value
    let mut reach: i32 = 0;

    if !safe_malloc_test(1, cost_model_size) {
        return false;
    }
    let mut cost_model = Box::new(CostModel {
        alpha: [0; VALUES_IN_BYTE as usize],
        red: [0; VALUES_IN_BYTE as usize],
        blue: [0; VALUES_IN_BYTE as usize],
        distance: [0; NUM_DISTANCE_CODES as usize],
        literal: vec![0; to_index(vp8l_histogram_num_codes(cache_bits))],
    });

    if use_color_cache && !vp8l_color_cache_init(&mut hashers, cache_bits) {
        return false;
    }

    if !cost_model_build(&mut cost_model, xsize, cache_bits, refs) {
        if use_color_cache {
            vp8l_color_cache_clear(&mut hashers);
        }
        return false;
    }

    let Some(mut cost_manager) = cost_manager_init(dist_array, pix_count, &cost_model) else {
        if use_color_cache {
            vp8l_color_cache_clear(&mut hashers);
        }
        return false;
    };

    // We loop one pixel at a time, but store all currently best points to
    // non-processed locations from this point.
    cost_manager.dist_array[0] = 0;
    // Add first pixel as literal.
    {
        let manager = &mut *cost_manager;
        add_single_literal_with_cost_model(
            argb,
            &mut hashers,
            &cost_model,
            /*idx=*/ 0,
            use_color_cache,
            /*prev_cost=*/ 0,
            &mut manager.costs,
            manager.dist_array,
        );
    }

    for i in 1..pix_count {
        let prev_cost = cost_manager.costs[to_index(i - 1)];
        let (offset, len) = vp8l_hash_chain_find_copy(hash_chain, i);

        // Try adding the pixel as a literal.
        {
            let manager = &mut *cost_manager;
            add_single_literal_with_cost_model(
                argb,
                &mut hashers,
                &cost_model,
                to_index(i),
                use_color_cache,
                prev_cost,
                &mut manager.costs,
                manager.dist_array,
            );
        }

        // If we are dealing with a non-literal.
        if len >= 2 {
            if offset != offset_prev {
                let code = vp8l_distance_to_plane_code(xsize, offset);
                offset_cost = get_distance_cost(&cost_model, code.cast_unsigned());
                first_offset_is_constant = 1;
                push_interval(&mut cost_manager, prev_cost + offset_cost, i, len);
            } else {
                debug_assert!(offset_cost >= 0);
                debug_assert!(len_prev >= 0);
                debug_assert!(first_offset_is_constant == 0 || first_offset_is_constant == 1);
                // Instead of considering all contributions from a pixel i by calling:
                //         PushInterval(cost_manager, prev_cost + offset_cost, i, len);
                // we optimize these contributions in case offset_cost stays the same
                // for consecutive pixels. This describes a set of pixels similar to a
                // previous set (e.g. constant color regions).
                if first_offset_is_constant != 0 {
                    reach = i - 1 + len_prev - 1;
                    first_offset_is_constant = 0;
                }

                if i + len - 1 > reach {
                    // We can only be go further with the same offset if the previous
                    // length was maxed, hence len_prev == len == MAX_LENGTH.
                    // TODO(vrabaud), bump i to the end right away (insert cache and
                    // update cost).
                    // TODO(vrabaud), check if one of the points in between does not have
                    // a lower cost.
                    // Already consider the pixel at "reach" to add intervals that are
                    // better than whatever we add.
                    let mut len_j: i32 = 0;
                    debug_assert!(len == MAX_LENGTH || len == pix_count - i);
                    // Figure out the last consecutive pixel within [i, reach + 1] with
                    // the same offset.
                    let mut j = i;
                    while j <= reach {
                        let (offset_j, length_j) = vp8l_hash_chain_find_copy(hash_chain, j + 1);
                        len_j = length_j;
                        if offset_j != offset {
                            let (_offset_j, length_j) = vp8l_hash_chain_find_copy(hash_chain, j);
                            len_j = length_j;
                            break;
                        }
                        j += 1;
                    }
                    // Update the cost at j - 1 and j.
                    update_cost_at_index(&mut cost_manager, j - 1, false);
                    update_cost_at_index(&mut cost_manager, j, false);

                    let cost_j = cost_manager.costs[to_index(j - 1)];
                    push_interval(&mut cost_manager, cost_j + offset_cost, j, len_j);
                    reach = j + len_j - 1;
                }
            }
        }

        update_cost_at_index(&mut cost_manager, i, true);
        offset_prev = offset;
        len_prev = len;
    }

    let ok = !refs.error;
    if use_color_cache {
        vp8l_color_cache_clear(&mut hashers);
    }
    cost_manager_clear(&mut cost_manager);
    ok
}

// upstream: src/enc/backward_references_cost_enc.c TraceBackwards
/// We pack the path at the end of `*dist_array` and return
/// a pointer to this part of the array. Example:
/// `dist_array` = [1x2xx3x2] => packed [1x2x1232], `chosen_path` = [1232]
///
/// Returns the position of the chosen path in `dist_array` and its size, the
/// two out-parameters of C.
fn trace_backwards(dist_array: &mut [u16], dist_array_size: i32) -> (usize, i32) {
    let mut path = to_index(dist_array_size);
    let mut cur = isize::try_from(dist_array_size).expect("the size fits isize") - 1;
    while cur >= 0 {
        let k = dist_array[cur.cast_unsigned()];
        path -= 1;
        dist_array[path] = k;
        cur -= isize::from(k.cast_signed());
    }
    let chosen_path_size = i32::try_from(to_index(dist_array_size) - path).expect("fits an int");
    (path, chosen_path_size)
}

// upstream: src/enc/backward_references_cost_enc.c BackwardReferencesHashChainFollowChosenPath
#[expect(clippy::if_not_else, reason = "the C condition `len != 1`")]
fn backward_references_hash_chain_follow_chosen_path(
    argb: &[u32],
    cache_bits: i32,
    chosen_path: &[u16],
    chosen_path_size: i32,
    hash_chain: &VP8LHashChain,
    refs: &mut VP8LBackwardRefs,
) -> bool {
    let use_color_cache = cache_bits > 0;
    let mut i: i32 = 0;
    let mut hashers = VP8LColorCache::default();

    if use_color_cache && !vp8l_color_cache_init(&mut hashers, cache_bits) {
        return false;
    }

    vp8l_clear_backward_refs(refs);
    for &len in &chosen_path[..to_index(chosen_path_size)] {
        let len = i32::from(len);
        if len != 1 {
            let offset = vp8l_hash_chain_find_offset(hash_chain, i);
            vp8l_backward_refs_cursor_add(
                refs,
                pix_or_copy_create_copy(offset.cast_unsigned(), copy_len(len)),
            );
            if use_color_cache {
                for k in 0..len {
                    vp8l_color_cache_insert(&mut hashers, argb[to_index(i + k)]);
                }
            }
            i += len;
        } else {
            let pixel = argb[to_index(i)];
            let idx = if use_color_cache {
                vp8l_color_cache_contains(&hashers, pixel)
            } else {
                -1
            };
            let v = if idx >= 0 {
                // use_color_cache is true and hashers contains argb[i]
                // push pixel as a color cache index
                pix_or_copy_create_cache_idx(idx)
            } else {
                if use_color_cache {
                    vp8l_color_cache_insert(&mut hashers, pixel);
                }
                pix_or_copy_create_literal(pixel)
            };
            vp8l_backward_refs_cursor_add(refs, v);
            i += 1;
        }
    }
    let ok = !refs.error;
    if use_color_cache {
        vp8l_color_cache_clear(&mut hashers);
    }
    ok
}

// upstream: src/enc/backward_references_cost_enc.c VP8LBackwardReferencesTraceBackwards
/// Returns 1 on success.
#[must_use]
pub fn vp8l_backward_references_trace_backwards(
    xsize: i32,
    ysize: i32,
    argb: &[u32],
    cache_bits: i32,
    hash_chain: &VP8LHashChain,
    refs_src: &VP8LBackwardRefs,
    refs_dst: &mut VP8LBackwardRefs,
) -> bool {
    let dist_array_size = xsize * ysize;
    let Some(mut dist_array) =
        webp_safe_malloc::<u16>(i64::from(dist_array_size).cast_unsigned(), 2)
    else {
        return false;
    };
    dist_array.resize(to_index(dist_array_size), 0);

    if !backward_references_hash_chain_distance_only(
        xsize,
        ysize,
        argb,
        cache_bits,
        hash_chain,
        refs_src,
        &mut dist_array,
    ) {
        return false;
    }
    let (chosen_path, chosen_path_size) = trace_backwards(&mut dist_array, dist_array_size);
    backward_references_hash_chain_follow_chosen_path(
        argb,
        cache_bits,
        &dist_array[chosen_path..],
        chosen_path_size,
        hash_chain,
        refs_dst,
    )
}
