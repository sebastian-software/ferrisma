// libwebp v1.6.0, src/utils/quant_levels_utils.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Quantize levels for specified number of quantization-levels ([2, 256]).
// Min and max values are preserved (usual 0 and 255 for alpha plane).
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/quant_levels_utils.h:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Alpha plane quantization utility
//
// Author:  Vikas Arora (vikasa@google.com)

//! Translation of `src/utils/quant_levels_utils.c` and `src/utils/quant_levels_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

// upstream: src/utils/quant_levels_utils.c NUM_SYMBOLS
/// The number of symbols of an 8-bit plane.
const NUM_SYMBOLS: usize = 256;
// upstream: src/utils/quant_levels_utils.c MAX_ITER
/// Maximum number of convergence steps.
const MAX_ITER: i32 = 6;
// upstream: src/utils/quant_levels_utils.c ERROR_THRESHOLD
/// MSE stopping criterion, per sample.
const ERROR_THRESHOLD: f64 = 1.0e-4;

// upstream: src/utils/quant_levels_utils.c QuantizeLevels
/// Quantizes the levels of an 8-bit plane to `num_levels` values with k-means
/// steps, and stores the sum of squared error in `sse` when it is given.
///
/// Returns `false` for a missing plane, a non-positive size, or a level count
/// outside `[2, 256]`; the plane and `sse` are then unchanged. The plane is
/// `width * height` samples, and `data` may be longer.
///
/// # Panics
///
/// Panics when `data` is shorter than `width * height`.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C casts the rounded centroids to uint8_t and the error to uint64_t"
)]
pub fn quantize_levels(
    data: Option<&mut [u8]>,
    width: i32,
    height: i32,
    num_levels: i32,
    sse: Option<&mut u64>,
) -> bool {
    let Some(data) = data else {
        return false;
    };
    if width <= 0 || height <= 0 {
        return false;
    }
    if !(2..=256).contains(&num_levels) {
        return false;
    }
    let size_int = height * width;
    let data_size = usize::try_from(size_int).expect("the plane size is positive");
    let mut freq = [0_i32; NUM_SYMBOLS];
    let mut q_level = [0_i32; NUM_SYMBOLS];
    let mut inv_q_level = [0.0_f64; NUM_SYMBOLS];
    let mut min_s = 255_i32;
    let mut max_s = 0_i32;
    let mut num_levels_in = 0_i32;
    let mut last_err = 1.0e38_f64;
    let mut err = 0.0_f64;
    let err_threshold = ERROR_THRESHOLD * f64::from(size_int);
    for &sample in &data[..data_size] {
        let value = i32::from(sample);
        num_levels_in += i32::from(freq[usize::from(sample)] == 0);
        if min_s > value {
            min_s = value;
        }
        if max_s < value {
            max_s = value;
        }
        freq[usize::from(sample)] += 1;
    }
    // nothing to do! when the plane already has few enough levels
    if num_levels_in > num_levels {
        // Start with uniformly spread centroids.
        for i in 0..num_levels {
            inv_q_level[i as usize] = f64::from(min_s)
                + f64::from(max_s - min_s) * f64::from(i) / f64::from(num_levels - 1);
        }
        // Fixed values. Won't be changed.
        q_level[min_s as usize] = 0;
        q_level[max_s as usize] = num_levels - 1;
        debug_assert_eq!(inv_q_level[0], f64::from(min_s));
        debug_assert_eq!(inv_q_level[(num_levels - 1) as usize], f64::from(max_s));
        // k-Means iterations.
        for _ in 0..MAX_ITER {
            let mut q_sum = [0.0_f64; NUM_SYMBOLS];
            let mut q_count = [0.0_f64; NUM_SYMBOLS];
            let mut slot = 0_i32;
            // Assign classes to representatives.
            for s in min_s..=max_s {
                // Keep track of the nearest neighbour 'slot'
                while slot < num_levels - 1
                    && f64::from(2 * s)
                        > inv_q_level[slot as usize] + inv_q_level[(slot + 1) as usize]
                {
                    slot += 1;
                }
                if freq[s as usize] > 0 {
                    q_sum[slot as usize] += f64::from(s * freq[s as usize]);
                    q_count[slot as usize] += f64::from(freq[s as usize]);
                }
                q_level[s as usize] = slot;
            }
            // Assign new representatives to classes.
            if num_levels > 2 {
                for slot in 1..num_levels - 1 {
                    let count = q_count[slot as usize];
                    if count > 0.0 {
                        inv_q_level[slot as usize] = q_sum[slot as usize] / count;
                    }
                }
            }
            // Compute convergence error.
            err = 0.0;
            for s in min_s..=max_s {
                let error = f64::from(s) - inv_q_level[q_level[s as usize] as usize];
                err += f64::from(freq[s as usize]) * error * error;
            }
            // Check for convergence: we stop as soon as the error is no
            // longer improving.
            if last_err - err < err_threshold {
                break;
            }
            last_err = err;
        }
        // Remap the alpha plane to quantized values.
        let mut map = [0_u8; NUM_SYMBOLS];
        for s in min_s..=max_s {
            let slot = q_level[s as usize];
            map[s as usize] = (inv_q_level[slot as usize] + 0.5) as u8;
        }
        // Final pass.
        for sample in &mut data[..data_size] {
            *sample = map[usize::from(*sample)];
        }
    }
    // Store sum of squared error if needed.
    if let Some(sse) = sse {
        *sse = err as u64;
    }
    true
}
