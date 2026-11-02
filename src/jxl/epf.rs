// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ComputeSigma and border helpers from lib/jxl/epf.{h,cc}.
//! Owner-relative row offsets replace pointer arithmetic; all source writes,
//! mirroring, and row-copy checkpoints retain their original order.
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::dec_cache::{PassesDecoderState, SIGMA_BORDER, SIGMA_PADDING};
use super::loop_filter::LoopFilter;
pub const INV_SIGMA_NUM: f32 = -1.171_572_9;
pub const MIN_SIGMA: f32 = -3.905_243;

pub fn compute_sigma(
    filter: &LoopFilter,
    rect: Rect,
    state: &mut PassesDecoderState<'_, '_>,
) -> Status {
    if filter.epf_iters == 0 {
        return Err(StatusCode::GenericError);
    }
    let quant_scale = state.shared().quantizer.scale();
    for by in 0..rect.ysize() {
        for bx in 0..rect.xsize() {
            let acs = state.shared().ac_strategy.const_rect_row(rect, by).get(bx);
            let llf_x = acs.covered_blocks_x();
            if !acs.is_first_block() {
                continue;
            }
            let quant = state.shared().raw_quant_field.row(rect.y0() + by)[rect.x0() + bx];
            let sigma_quant = filter.epf_quant_mul / (quant_scale * quant as f32 * INV_SIGMA_NUM);
            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    let sharpness =
                        state.shared().epf_sharpness.row(rect.y0() + by + iy)[rect.x0() + bx + ix];
                    let sigma = sigma_quant * filter.epf_sharp_lut[usize::from(sharpness)];
                    let sigma = if sigma < -1e-4 { sigma } else { -1e-4 };
                    state.sigma.row_mut(rect.y0() + by + iy + SIGMA_PADDING)
                        [rect.x0() + bx + ix + SIGMA_PADDING] = 1.0 / sigma;
                }
            }
            if bx + rect.x0() == 0 {
                for iy in 0..acs.covered_blocks_y() {
                    let row = state.sigma.row_mut(rect.y0() + by + iy + SIGMA_PADDING);
                    for i in 0..SIGMA_BORDER {
                        row[SIGMA_PADDING - 1 - i] = row[SIGMA_PADDING + i];
                    }
                }
            }
            if bx + rect.x0() + llf_x == state.shared().frame_dim.xsize_blocks {
                for iy in 0..acs.covered_blocks_y() {
                    let row = state.sigma.row_mut(rect.y0() + by + iy + SIGMA_PADDING);
                    let p = rect.x0() + SIGMA_PADDING + bx + llf_x;
                    for i in 0..SIGMA_BORDER {
                        row[p + i] = row[p - 1 - i];
                    }
                }
            }
            let before = if bx + rect.x0() == 0 {
                1
            } else {
                bx + SIGMA_PADDING
            };
            let after = if bx + rect.x0() + llf_x == state.shared().frame_dim.xsize_blocks {
                SIGMA_PADDING + llf_x + bx + SIGMA_BORDER
            } else {
                SIGMA_PADDING + llf_x + bx
            };
            let start = rect.x0() + before;
            let end = rect.x0() + after;
            if by + rect.y0() == 0 {
                for iy in 0..SIGMA_BORDER {
                    for x in start..end {
                        let value = state.sigma.row(rect.y0() + by + SIGMA_PADDING + iy)[x];
                        state.sigma.row_mut(rect.y0() + by + SIGMA_PADDING - 1 - iy)[x] = value;
                    }
                }
            }
            if by + rect.y0() + acs.covered_blocks_y() == state.shared().frame_dim.ysize_blocks {
                for iy in 0..SIGMA_BORDER {
                    for x in start..end {
                        let value = state
                            .sigma
                            .row(rect.y0() + by + acs.covered_blocks_y() + SIGMA_PADDING - 1 - iy)
                            [x];
                        state.sigma.row_mut(
                            rect.y0() + by + acs.covered_blocks_y() + SIGMA_PADDING + iy,
                        )[x] = value;
                    }
                }
            }
        }
    }
    Ok(())
}
