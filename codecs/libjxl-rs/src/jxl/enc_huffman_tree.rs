// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_huffman_tree.{h,cc}`.
//! Slices replace pointer/length pairs. The source comparator totally orders
//! leaf nodes, so Rust's unstable sort retains the same order. See ADR 0003.

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HuffmanTree {
    pub total_count: u32,
    pub index_left: i16,
    pub index_right_or_value: i16,
}

pub fn set_depth(p: HuffmanTree, pool: &[HuffmanTree], depth: &mut [u8], mut level: u8) {
    if p.index_left >= 0 {
        level = level.wrapping_add(1);
        set_depth(pool[p.index_left as usize], pool, depth, level);
        set_depth(pool[p.index_right_or_value as usize], pool, depth, level);
    } else {
        depth[p.index_right_or_value as usize] = level;
    }
}

/// `data` must contain at least one populated symbol and admit a tree within
/// `tree_limit`. Output depths for absent symbols are left untouched.
pub fn create_huffman_tree(data: &[u32], tree_limit: i32, depth: &mut [u8]) {
    assert!(depth.len() >= data.len());
    let populated = data.iter().filter(|&&count| count != 0).count();
    assert!(populated > 0);
    if populated > 1 {
        assert!(tree_limit > 0);
        assert!(tree_limit >= 14 || populated <= 1usize << tree_limit);
    }
    // Source child indices are signed 16-bit values. Outside this range its
    // narrowed indices can address outside the node pool.
    assert!(populated <= (i16::MAX as usize).div_ceil(2));
    let mut count_limit = 1u32;
    loop {
        let mut tree = Vec::with_capacity(2 * data.len() + 1);
        for i in (0..data.len()).rev() {
            if data[i] != 0 {
                assert!(i <= i16::MAX as usize);
                tree.push(HuffmanTree {
                    total_count: data[i].max(count_limit.wrapping_sub(1)),
                    index_left: -1,
                    index_right_or_value: i as i16,
                });
            }
        }
        let n = tree.len();
        if n == 1 {
            depth[tree[0].index_right_or_value as usize] = 1;
            break;
        }
        tree.sort_unstable_by(|v0, v1| {
            v0.total_count
                .cmp(&v1.total_count)
                .then_with(|| v1.index_right_or_value.cmp(&v0.index_right_or_value))
        });
        let sentinel = HuffmanTree {
            total_count: u32::MAX,
            index_left: -1,
            index_right_or_value: -1,
        };
        tree.push(sentinel);
        tree.push(sentinel);
        let mut i = 0;
        let mut j = n + 1;
        for _ in (1..n).rev() {
            let left = if tree[i].total_count <= tree[j].total_count {
                i += 1;
                i - 1
            } else {
                j += 1;
                j - 1
            };
            let right = if tree[i].total_count <= tree[j].total_count {
                i += 1;
                i - 1
            } else {
                j += 1;
                j - 1
            };
            let j_end = tree.len() - 1;
            tree[j_end] = HuffmanTree {
                total_count: tree[left].total_count.wrapping_add(tree[right].total_count),
                index_left: left as i16,
                index_right_or_value: right as i16,
            };
            tree.push(sentinel);
        }
        debug_assert_eq!(tree.len(), 2 * n + 1);
        set_depth(tree[2 * n - 1], &tree, depth, 0);
        if i32::from(*depth[..data.len()].iter().max().unwrap()) <= tree_limit {
            break;
        }
        count_limit = count_limit.wrapping_mul(2);
    }
}

fn reverse(v: &mut [u8], mut start: usize, mut end: usize) {
    end -= 1;
    while start < end {
        v.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn emit(value: u8, extra: u8, tree_size: &mut usize, tree: &mut [u8], extra_bits_data: &mut [u8]) {
    tree[*tree_size] = value;
    extra_bits_data[*tree_size] = extra;
    *tree_size += 1;
}

pub fn write_huffman_tree_repetitions(
    previous_value: u8,
    value: u8,
    mut repetitions: usize,
    tree_size: &mut usize,
    tree: &mut [u8],
    extra_bits_data: &mut [u8],
) {
    debug_assert!(repetitions > 0);
    if previous_value != value {
        emit(value, 0, tree_size, tree, extra_bits_data);
        repetitions -= 1;
    }
    if repetitions == 7 {
        emit(value, 0, tree_size, tree, extra_bits_data);
        repetitions -= 1;
    }
    if repetitions < 3 {
        for _ in 0..repetitions {
            emit(value, 0, tree_size, tree, extra_bits_data);
        }
    } else {
        repetitions -= 3;
        let start = *tree_size;
        loop {
            emit(
                16,
                (repetitions & 0x3) as u8,
                tree_size,
                tree,
                extra_bits_data,
            );
            repetitions >>= 2;
            if repetitions == 0 {
                break;
            }
            repetitions -= 1;
        }
        reverse(tree, start, *tree_size);
        reverse(extra_bits_data, start, *tree_size);
    }
}

pub fn write_huffman_tree_repetitions_zeros(
    mut repetitions: usize,
    tree_size: &mut usize,
    tree: &mut [u8],
    extra_bits_data: &mut [u8],
) {
    if repetitions == 11 {
        emit(0, 0, tree_size, tree, extra_bits_data);
        repetitions -= 1;
    }
    if repetitions < 3 {
        for _ in 0..repetitions {
            emit(0, 0, tree_size, tree, extra_bits_data);
        }
    } else {
        repetitions -= 3;
        let start = *tree_size;
        loop {
            emit(
                17,
                (repetitions & 0x7) as u8,
                tree_size,
                tree,
                extra_bits_data,
            );
            repetitions >>= 3;
            if repetitions == 0 {
                break;
            }
            repetitions -= 1;
        }
        reverse(tree, start, *tree_size);
        reverse(extra_bits_data, start, *tree_size);
    }
}

fn decide_over_rle_use(depth: &[u8]) -> (bool, bool) {
    let mut total_reps_zero = 0;
    let mut total_reps_non_zero = 0;
    let mut count_reps_zero = 1;
    let mut count_reps_non_zero = 1;
    let mut i = 0;
    while i < depth.len() {
        let value = depth[i];
        let mut reps = 1;
        for &next in &depth[i + 1..] {
            if next != value {
                break;
            }
            reps += 1;
        }
        if reps >= 3 && value == 0 {
            total_reps_zero += reps;
            count_reps_zero += 1;
        }
        if reps >= 4 && value != 0 {
            total_reps_non_zero += reps;
            count_reps_non_zero += 1;
        }
        i += reps;
    }
    (
        total_reps_non_zero > count_reps_non_zero * 2,
        total_reps_zero > count_reps_zero * 2,
    )
}

pub fn write_huffman_tree(
    depth: &[u8],
    tree_size: &mut usize,
    tree: &mut [u8],
    extra_bits_data: &mut [u8],
) {
    let mut previous_value = 8;
    let mut new_length = depth.len();
    for &value in depth.iter().rev() {
        if value != 0 {
            break;
        }
        new_length -= 1;
    }
    let (use_rle_for_non_zero, use_rle_for_zero) = if depth.len() > 50 {
        decide_over_rle_use(&depth[..new_length])
    } else {
        (false, false)
    };
    let mut i = 0;
    while i < new_length {
        let value = depth[i];
        let mut reps = 1;
        if (value != 0 && use_rle_for_non_zero) || (value == 0 && use_rle_for_zero) {
            for &next in &depth[i + 1..new_length] {
                if next != value {
                    break;
                }
                reps += 1;
            }
        }
        if value == 0 {
            write_huffman_tree_repetitions_zeros(reps, tree_size, tree, extra_bits_data);
        } else {
            write_huffman_tree_repetitions(
                previous_value,
                value,
                reps,
                tree_size,
                tree,
                extra_bits_data,
            );
            previous_value = value;
        }
        i += reps;
    }
}

fn reverse_bits(num_bits: i32, mut bits: u16) -> u16 {
    const LUT: [usize; 16] = [
        0x0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe, 0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf,
    ];
    let mut retval = LUT[usize::from(bits & 0xf)];
    for _ in (4..num_bits).step_by(4) {
        retval <<= 4;
        bits >>= 4;
        retval |= LUT[usize::from(bits & 0xf)];
    }
    retval >>= -num_bits & 0x3;
    retval as u16
}

pub fn convert_bit_depths_to_symbols(depth: &[u8], bits: &mut [u16]) {
    let mut bl_count = [0u16; 16];
    for &value in depth {
        bl_count[usize::from(value)] = bl_count[usize::from(value)].wrapping_add(1);
    }
    bl_count[0] = 0;
    let mut next_code = [0u16; 16];
    let mut code = 0i32;
    for i in 1..16 {
        code = (code + i32::from(bl_count[i - 1])) << 1;
        next_code[i] = code as u16;
    }
    for (i, &value) in depth.iter().enumerate() {
        if value != 0 {
            let d = usize::from(value);
            bits[i] = reverse_bits(i32::from(value), next_code[d]);
            next_code[d] = next_code[d].wrapping_add(1);
        }
    }
}
