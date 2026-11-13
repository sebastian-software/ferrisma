// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The non-scalar `InvHSqueeze`/`InvVSqueeze` of modular/transform/squeeze.cc
//! (the `HWY_TARGET != HWY_SCALAR` branches with `FastUnsqueeze`) for the
//! NEON and AVX2 targets (port ADR 0004). `FastUnsqueeze` is written once
//! against [`SimdF32`]'s signed lanes; `HWY_CAPPED(pixel_type, 8)` has the
//! full vector's lanes on both targets (4 and 8). `squeeze.rs` keeps the
//! scalar (`HWY_SCALAR`) translation; `inv_squeeze` calls these when the
//! decoder selected a SIMD target, as `HWY_DYNAMIC_DISPATCH(InvSqueeze)`
//! does.
//!
//! `FastUnsqueeze` computes in wrapping 32-bit lanes with the source's
//! vector operations (`MulEven`/`MulOdd` and `InterleaveOdd` for the division
//! by three); the scalar tails keep the source's 64-bit `pixel_type_w`
//! arithmetic and `SmoothTendency`, so both paths give the original's
//! results, including where they differ from each other. The 8x8 transposes
//! and `StoreInterleaved` of the horizontal path only move words; they are
//! plain copies here, which store the same words in the same places.
//! `FastUnsqueeze`'s rows are copied in and out of local blocks of eight
//! samples, so that the previous output row can be read while two output
//! rows of the same channel are written; the source reads that row before it
//! writes any of these samples.

use super::squeeze::smooth_tendency;
use crate::jxl::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::modular::modular_image::{Channel, Image};

/// `FastUnsqueeze(p_residual, p_avg, p_navg, p_pout, p_out, p_nout)` on
/// blocks of eight samples.
struct FastUnsqueeze<'a> {
    residual: &'a [i32; 8],
    avg: &'a [i32; 8],
    navg: &'a [i32; 8],
    pout: &'a [i32; 8],
    out: &'a mut [i32; 8],
    nout: &'a mut [i32; 8],
}
impl Kernel for FastUnsqueeze<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let n = D::LANES;
        let onethird = d.set_i32(0x5555_5556);
        let zero = d.set_i32(0);
        let one = d.set_i32(1);
        let mut x = 0;
        while x < 8 {
            let avg = d.load_i32(&self.avg[x..]);
            let next_avg = d.load_i32(&self.navg[x..]);
            let top = d.load_i32(&self.pout[x..]);
            // Equivalent to SmoothTendency(top, avg, next_avg), but without
            // branches.
            let ba = d.sub_i32(top, avg);
            let an = d.sub_i32(avg, next_avg);
            let nonmono = d.xor_i32(ba, an);
            let absba = d.abs_i32(ba);
            let absan = d.abs_i32(an);
            let absbn = d.abs_i32(d.sub_i32(top, next_avg));
            // Compute a3 = absBa / 3.
            let a3eh = d.mul_even_i32(absba, onethird);
            let a3oh = d.mul_odd_i32(absba, onethird);
            // HWY 1.2, little endian.
            let mut a3 = d.interleave_odd_i32(a3eh, a3oh);
            a3 = d.add_i32(a3, d.add_i32(absbn, d.set_i32(2)));
            let mut absdiff = d.shift_right_i32::<2>(a3);
            let mut skipdiff = d.ne_i32(ba, zero);
            skipdiff = d.and_i32(skipdiff, d.ne_i32(an, zero));
            skipdiff = d.and_i32(skipdiff, d.lt_i32(nonmono, zero));
            let absba2 = d.add_i32(d.shift_left_i32::<1>(absba), d.and_i32(absdiff, one));
            absdiff = d.if_then_else_i32(
                d.gt_i32(absdiff, absba2),
                d.add_i32(d.shift_left_i32::<1>(absba), one),
                absdiff,
            );
            let absan2 = d.shift_left_i32::<1>(absan);
            absdiff = d.if_then_else_i32(
                d.gt_i32(d.add_i32(absdiff, d.and_i32(absdiff, one)), absan2),
                absan2,
                absdiff,
            );
            let diff1 = d.if_then_else_i32(d.lt_i32(top, next_avg), d.neg_i32(absdiff), absdiff);
            let tendency = d.if_then_zero_else_i32(skipdiff, diff1);

            let diff_minus_tendency = d.load_i32(&self.residual[x..]);
            let diff = d.add_i32(diff_minus_tendency, tendency);
            let out = d.add_i32(
                avg,
                d.shift_right_i32::<1>(d.add_i32(diff, d.shift_right_u32::<31>(diff))),
            );
            d.store_i32(out, &mut self.out[x..]);
            d.store_i32(d.sub_i32(out, diff), &mut self.nout[x..]);
            x += n;
        }
    }
}

fn block(row: &[i32], x: usize) -> [i32; 8] {
    row[x..x + 8].try_into().unwrap()
}

/// `Transpose8x8Block(from, to, fromstride)`: `to[j * 8 + k]` is sample `j`
/// of row `k`.
fn transpose8x8(rows: &[&[i32]], x: usize, to: &mut [i32]) {
    for (k, row) in rows.iter().enumerate() {
        for j in 0..8 {
            to[j * 8 + k] = row[x + j];
        }
    }
}

/// `Transpose8x8Block(from, to, 8)` of an 8x8 block.
fn transpose_block(from: &[i32; 64], to: &mut [i32; 64]) {
    for k in 0..8 {
        for j in 0..8 {
            to[j * 8 + k] = from[k * 8 + j];
        }
    }
}

pub fn inv_h_squeeze_simd(
    target: Target,
    input: &mut Image,
    c: u32,
    rc: u32,
    pool: Option<&ThreadPool>,
) -> Status {
    if c as usize >= input.channel.len() || rc as usize >= input.channel.len() {
        return Err(StatusCode::GenericError);
    }
    let ci = c as usize;
    let ri = rc as usize;
    let chin = &input.channel[ci];
    let residual = &input.channel[ri];
    if chin.w != chin.w.wrapping_add(residual.w).wrapping_add(1) / 2 || chin.h != residual.h {
        return Err(StatusCode::GenericError);
    }
    if residual.w == 0 {
        input.channel[ci].hshift -= 1;
        return Ok(());
    }
    let mut chout = Channel::create_in(
        input.memory_manager(),
        chin.w.wrapping_add(residual.w),
        chin.h,
        chin.hshift - 1,
        chin.vshift,
    )?;
    if residual.h == 0 {
        input.channel[ci] = chout;
        return Ok(());
    }
    let unsqueeze_row = |chout: &mut Channel, y: usize, x0: usize| {
        let p_residual = residual.row(y);
        let p_avg = chin.row(y);
        let outw = chout.w;
        let p_out = chout.row_mut(y);
        for x in x0..residual.w {
            let diff_minus_tendency = i64::from(p_residual[x]);
            let avg = i64::from(p_avg[x]);
            let next_avg = if x + 1 < chin.w {
                i64::from(p_avg[x + 1])
            } else {
                avg
            };
            let left = if x != 0 {
                i64::from(p_out[(x << 1) - 1])
            } else {
                avg
            };
            let tendency = smooth_tendency(left, avg, next_avg);
            let diff = diff_minus_tendency + tendency;
            let a = avg + diff / 2;
            p_out[x << 1] = a as i32;
            let b = a - diff;
            p_out[(x << 1) + 1] = b as i32;
        }
        if outw & 1 != 0 {
            p_out[outw - 1] = p_avg[chin.w - 1];
        }
    };
    // Horizontal unsqueeze has horizontal data dependencies, so the source
    // does 8 rows at a time and treats them as a vertical unsqueeze of a
    // transposed 8x8 block (or 9x8 for one input).
    const ROWS_PER_THREAD: usize = 8;
    let unsqueeze_span = |chout: &mut Channel, task: u32, _thread: usize| -> Status {
        let y0 = task as usize * ROWS_PER_THREAD;
        let rows = ROWS_PER_THREAD.min(chin.h - y0);
        let mut x = 0;
        let mut b_p_avg = [0i32; 9 * ROWS_PER_THREAD];
        let mut b_p_residual = [0i32; 8 * ROWS_PER_THREAD];
        let mut b_p_out_even = [0i32; 8 * ROWS_PER_THREAD];
        let mut b_p_out_odd = [0i32; 8 * ROWS_PER_THREAD];
        let mut b_p_out_even_t = [0i32; 8 * ROWS_PER_THREAD];
        let mut b_p_out_odd_t = [0i32; 8 * ROWS_PER_THREAD];
        if residual.w > 16 && rows == ROWS_PER_THREAD {
            let residual_rows: Vec<&[i32]> = (y0..y0 + 8).map(|y| residual.row(y)).collect();
            let avg_rows: Vec<&[i32]> = (y0..y0 + 8).map(|y| chin.row(y)).collect();
            while x < residual.w - 9 {
                transpose8x8(&residual_rows, x, &mut b_p_residual);
                transpose8x8(&avg_rows, x, &mut b_p_avg);
                for (y, row) in avg_rows.iter().enumerate() {
                    b_p_avg[8 * 8 + y] = row[x + 8];
                }
                for i in 0..8 {
                    let pout = if x + i != 0 {
                        block(&b_p_out_odd, 8 * ((x + i - 1) & 7))
                    } else {
                        block(&b_p_avg, 8 * i)
                    };
                    let mut out = [0i32; 8];
                    let mut nout = [0i32; 8];
                    dispatch(
                        target,
                        FastUnsqueeze {
                            residual: &block(&b_p_residual, 8 * i),
                            avg: &block(&b_p_avg, 8 * i),
                            navg: &block(&b_p_avg, 8 * (i + 1)),
                            pout: &pout,
                            out: &mut out,
                            nout: &mut nout,
                        },
                    );
                    b_p_out_even[8 * i..8 * i + 8].copy_from_slice(&out);
                    b_p_out_odd[8 * i..8 * i + 8].copy_from_slice(&nout);
                }
                transpose_block(&b_p_out_even, &mut b_p_out_even_t);
                transpose_block(&b_p_out_odd, &mut b_p_out_odd_t);
                // StoreInterleaved(d, even, odd, p_out + ((x + i) << 1) +
                // onerow_out * y) for every vector of the row.
                for y in 0..ROWS_PER_THREAD {
                    let p_out = chout.row_mut(y0 + y);
                    for i in 0..ROWS_PER_THREAD {
                        p_out[(x + i) << 1] = b_p_out_even_t[8 * y + i];
                        p_out[((x + i) << 1) + 1] = b_p_out_odd_t[8 * y + i];
                    }
                }
                x += 8;
            }
        }
        for y in 0..rows {
            unsqueeze_row(chout, y0 + y, x);
        }
        Ok(())
    };
    let num_tasks = chin
        .h
        .wrapping_add(ROWS_PER_THREAD - 1)
        .wrapping_div(ROWS_PER_THREAD) as u32;
    // SAFETY: The spans write only the local output channel and read the
    // input channels through shared borrows; none holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, num_tasks, &mut chout, unsqueeze_span) }?;
    input.channel[ci] = chout;
    Ok(())
}

pub fn inv_v_squeeze_simd(
    target: Target,
    input: &mut Image,
    c: u32,
    rc: u32,
    pool: Option<&ThreadPool>,
) -> Status {
    if c as usize >= input.channel.len() || rc as usize >= input.channel.len() {
        return Err(StatusCode::GenericError);
    }
    let ci = c as usize;
    let ri = rc as usize;
    let chin = &input.channel[ci];
    let residual = &input.channel[ri];
    if chin.h != chin.h.wrapping_add(residual.h).wrapping_add(1) / 2 || chin.w != residual.w {
        return Err(StatusCode::GenericError);
    }
    if residual.h == 0 {
        input.channel[ci].vshift -= 1;
        return Ok(());
    }
    let mut chout = Channel::create_in(
        input.memory_manager(),
        chin.w,
        chin.h.wrapping_add(residual.h),
        chin.hshift,
        chin.vshift - 1,
    )?;
    if residual.w == 0 {
        input.channel[ci] = chout;
        return Ok(());
    }
    const COLS_PER_THREAD: u32 = 64;
    let unsqueeze_slice = |chout: &mut Channel, task: u32, _thread: usize| -> Status {
        let x0 = task.wrapping_mul(COLS_PER_THREAD) as usize;
        let x1 = ((task.wrapping_add(1) as usize) * COLS_PER_THREAD as usize).min(chin.w);
        let w = x1 - x0;
        // We only iterate up to min(chin_residual.h, chin.h), which is always
        // chin_residual.h.
        for y in 0..residual.h {
            let p_residual = &residual.row(y)[x0..];
            let p_avg = &chin.row(y)[x0..];
            let p_navg = &chin.row(if y + 1 < chin.h { y + 1 } else { y })[x0..];
            let mut x = 0;
            while x + 7 < w {
                let pout = if y > 0 {
                    block(chout.row((y << 1) - 1), x0 + x)
                } else {
                    block(p_avg, x)
                };
                let mut out = [0i32; 8];
                let mut nout = [0i32; 8];
                dispatch(
                    target,
                    FastUnsqueeze {
                        residual: &block(p_residual, x),
                        avg: &block(p_avg, x),
                        navg: &block(p_navg, x),
                        pout: &pout,
                        out: &mut out,
                        nout: &mut nout,
                    },
                );
                chout.row_mut(y << 1)[x0 + x..x0 + x + 8].copy_from_slice(&out);
                chout.row_mut((y << 1) + 1)[x0 + x..x0 + x + 8].copy_from_slice(&nout);
                x += 8;
            }
            while x < w {
                let j = x + x0;
                let avg = i64::from(p_avg[x]);
                let next_avg = i64::from(p_navg[x]);
                let top = if y > 0 {
                    i64::from(chout.row((y << 1) - 1)[j])
                } else {
                    avg
                };
                let tendency = smooth_tendency(top, avg, next_avg);
                let diff_minus_tendency = i64::from(p_residual[x]);
                let diff = diff_minus_tendency + tendency;
                let out = avg + diff / 2;
                chout.row_mut(y << 1)[j] = out as i32;
                // If chin_residual.h == chin.h, the output has an even number
                // of rows so the next line is fine. Otherwise, this loop
                // won't write to the last output row, which is handled
                // separately.
                chout.row_mut((y << 1) + 1)[j] = (out - diff) as i32;
                x += 1;
            }
        }
        Ok(())
    };
    let num_tasks = chin
        .w
        .wrapping_add(COLS_PER_THREAD as usize - 1)
        .wrapping_div(COLS_PER_THREAD as usize) as u32;
    // SAFETY: As in inv_h_squeeze_simd: the slices write only the local
    // output channel and read the input channels through shared borrows.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, num_tasks, &mut chout, unsqueeze_slice) }?;
    if chout.h & 1 != 0 {
        let y = chin.h - 1;
        let p_avg = chin.row(y);
        let p_out = chout.row_mut(y << 1);
        p_out[..chin.w].copy_from_slice(&p_avg[..chin.w]);
    }
    input.channel[ci] = chout;
    Ok(())
}
