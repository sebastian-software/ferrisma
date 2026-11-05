// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Palette lookup, inverse pixels, and metadata from modular/transform/
//! palette.{h,cc}. RunOnPool keeps the original task order with a null pool;
//! a runner's tasks are serialized by `run_on_pool_exclusive`. Plane owners
//! move through the source index-buffer swap; short shared borrows end before
//! each output write. Channels allocate through the image's memory manager,
//! and channel erasure keeps the libc++ release order.

use super::transform::check_equal_channels;
use crate::jxl::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::image::ImageI;
use crate::jxl::memory_manager_internal::MemoryManagerPtr;
use crate::jxl::modular::encoding::context_predict::{
    PixelPosition, predict_no_tree_no_wp, predict_no_tree_wp, weighted,
};
use crate::jxl::modular::modular_image::{Channel, Image};
use crate::jxl::modular::options::Predictor;

pub mod palette_internal {
    pub const MAX_PALETTE_LOOKUP_TABLE_SIZE: i32 = 1 << 16;
    pub const RGB_CHANNELS: usize = 3;
    pub const LARGE_CUBE: i32 = 5;
    pub const SMALL_CUBE: i32 = 4;
    pub const SMALL_CUBE_BITS: usize = 2;
    pub const LARGE_CUBE_OFFSET: i32 = SMALL_CUBE * SMALL_CUBE * SMALL_CUBE;
    pub const IMPLICIT_PALETTE_SIZE: i32 = LARGE_CUBE_OFFSET + LARGE_CUBE * LARGE_CUBE * LARGE_CUBE;
    pub fn scale<const DENOM: i32>(value: u64, bit_depth: u64) -> i32 {
        const {
            assert!(DENOM == 4);
        }
        (value.wrapping_mul((1u64 << bit_depth) - 1) >> 2) as i32
    }
    pub fn get_palette_value(
        palette: &[i32],
        mut index: i32,
        c: usize,
        palette_size: i32,
        onerow: i32,
        bit_depth: i32,
    ) -> i32 {
        if index < 0 {
            const DELTA_PALETTE: [[i32; 3]; 72] = [
                [0, 0, 0],
                [4, 4, 4],
                [11, 0, 0],
                [0, 0, -13],
                [0, -12, 0],
                [-10, -10, -10],
                [-18, -18, -18],
                [-27, -27, -27],
                [-18, -18, 0],
                [0, 0, -32],
                [-32, 0, 0],
                [-37, -37, -37],
                [0, -32, -32],
                [24, 24, 45],
                [50, 50, 50],
                [-45, -24, -24],
                [-24, -45, -45],
                [0, -24, -24],
                [-34, -34, 0],
                [-24, 0, -24],
                [-45, -45, -24],
                [64, 64, 64],
                [-32, 0, -32],
                [0, -32, 0],
                [-32, 0, 32],
                [-24, -45, -24],
                [45, 24, 45],
                [24, -24, -45],
                [-45, -24, 24],
                [80, 80, 80],
                [64, 0, 0],
                [0, 0, -64],
                [0, -64, -64],
                [-24, -24, 45],
                [96, 96, 96],
                [64, 64, 0],
                [45, -24, -24],
                [34, -34, 0],
                [112, 112, 112],
                [24, -45, -45],
                [45, 45, -24],
                [0, -32, 32],
                [24, -24, 45],
                [0, 96, 96],
                [45, -24, 24],
                [24, -45, -24],
                [-24, -45, 24],
                [0, -64, 0],
                [96, 0, 0],
                [128, 128, 128],
                [64, 0, 64],
                [144, 144, 144],
                [96, 96, 0],
                [-36, -36, 36],
                [45, -24, -45],
                [45, -45, -24],
                [0, 0, -96],
                [0, 128, 128],
                [0, 96, 0],
                [45, 24, -45],
                [-128, 0, 0],
                [24, -45, 24],
                [-45, 24, -45],
                [64, 0, -64],
                [64, -64, -64],
                [96, 0, 96],
                [45, -45, 24],
                [24, 45, -45],
                [64, 64, -64],
                [128, 128, 0],
                [0, 0, -128],
                [-24, 45, -45],
            ];
            if c >= RGB_CHANNELS {
                return 0;
            }
            // Keep the brackets: negating INT32_MIN itself would overflow.
            index = -(index + 1);
            index %= (1 + 2 * (DELTA_PALETTE.len() - 1)) as i32;
            const MULTIPLIER: [i32; 2] = [-1, 1];
            let mut result =
                DELTA_PALETTE[((index + 1) >> 1) as usize][c] * MULTIPLIER[(index & 1) as usize];
            if bit_depth > 8 {
                result *= 1i32 << (bit_depth - 8);
            }
            result
        } else if palette_size <= index && index < palette_size + LARGE_CUBE_OFFSET {
            if c >= RGB_CHANNELS {
                return 0;
            }
            index -= palette_size;
            index >>= c * SMALL_CUBE_BITS;
            scale::<SMALL_CUBE>((index % SMALL_CUBE) as u64, bit_depth as u64)
                + (1 << 0.max(bit_depth - 3))
        } else if palette_size + LARGE_CUBE_OFFSET <= index {
            if c >= RGB_CHANNELS {
                return 0;
            }
            index -= palette_size + LARGE_CUBE_OFFSET;
            match c {
                1 => index /= LARGE_CUBE,
                2 => index /= LARGE_CUBE * LARGE_CUBE,
                _ => (),
            }
            scale::<{ LARGE_CUBE - 1 }>((index % LARGE_CUBE) as u64, bit_depth as u64)
        } else {
            palette[c.wrapping_mul(onerow as usize).wrapping_add(index as usize)]
        }
    }
}
pub fn inv_palette(
    input: &mut Image,
    begin_c: u32,
    _nb_colors: u32,
    nb_deltas: u32,
    predictor: Predictor,
    wp_header: &weighted::Header,
    pool: Option<&ThreadPool>,
) -> Status {
    if input.nb_meta_channels < 1 {
        return Err(StatusCode::GenericError);
    }
    let nb = input.channel[0].h as i32;
    let c0 = begin_c.wrapping_add(1);
    if c0 as usize >= input.channel.len() {
        return Err(StatusCode::GenericError);
    }
    let c0i = c0 as usize;
    let (w, h) = (input.channel[c0i].w, input.channel[c0i].h);
    if nb < 1 {
        return Err(StatusCode::GenericError);
    }
    let memory_manager: MemoryManagerPtr = input.memory_manager();
    for _ in 1..nb {
        let ch = &input.channel[c0i];
        let c = Channel::create_in(memory_manager, w, h, ch.hshift, ch.vshift)?;
        input.channel.insert(c0i + 1, c);
    }
    let palette_size = input.channel[0].w as i32;
    let onerow = input.channel[0].plane.pixels_per_row() as i32;
    let onerow_image = input.channel[c0i].plane.pixels_per_row();
    let bit_depth = input.bitdepth.min(24);
    if w == 0 {
        // Avoid touching empty channels with nonzero height, as in the source.
    } else if nb_deltas == 0 && predictor == Predictor::ZERO {
        if nb == 1 {
            let process_row = |input: &mut Image, task: u32, _thread: usize| -> Status {
                let y = task as usize;
                for x in 0..w {
                    // Clamp1 preserves this order even when palette_size==0.
                    let value = input.channel[c0i].row(y)[x];
                    let index = if value < 0 {
                        0
                    } else if value > palette_size - 1 {
                        palette_size - 1
                    } else {
                        value
                    };
                    let value = palette_internal::get_palette_value(
                        input.channel[0].plane.pixels().unwrap_or(&[]),
                        index,
                        0,
                        palette_size,
                        onerow,
                        bit_depth,
                    );
                    input.channel[c0i].row_mut(y)[x] = value;
                }
                Ok(())
            };
            // SAFETY: The rows borrow only the image's channel owners, which
            // hold no thread-affine values.
            unsafe { run_on_pool_exclusive_no_init(pool, 0, h as u32, input, process_row) }?;
        } else {
            let process_row = |input: &mut Image, task: u32, _thread: usize| -> Status {
                let y = task as usize;
                for x in 0..w {
                    let index = input.channel[c0i].row(y)[x];
                    for c in 0..nb as usize {
                        let value = palette_internal::get_palette_value(
                            input.channel[0].plane.pixels().unwrap_or(&[]),
                            index,
                            c,
                            palette_size,
                            onerow,
                            bit_depth,
                        );
                        input.channel[c0i + c].row_mut(y)[x] = value;
                    }
                }
                Ok(())
            };
            // SAFETY: As above.
            unsafe { run_on_pool_exclusive_no_init(pool, 0, h as u32, input, process_row) }?;
        }
    } else {
        let plane = &mut input.channel[c0i].plane;
        let mut indices = ImageI::create_in(plane.xsize(), plane.ysize(), 0, memory_manager)?;
        plane.swap(&mut indices);
        if predictor == Predictor::WEIGHTED {
            let process_channel = |input: &mut Image, c: u32, _thread: usize| -> Status {
                let ci = c0i + c as usize;
                let (cw, ch) = (input.channel[ci].w, input.channel[ci].h);
                let mut state = weighted::State::new(wp_header, cw, ch);
                for y in 0..ch {
                    for x in 0..cw {
                        let index = indices.row(y)[x];
                        let palette_entry = palette_internal::get_palette_value(
                            input.channel[0].plane.pixels().unwrap_or(&[]),
                            index,
                            c as usize,
                            palette_size,
                            onerow,
                            bit_depth,
                        );
                        let channel = &input.channel[ci];
                        let pp = PixelPosition::new(
                            channel.plane.pixels().unwrap(),
                            y * channel.plane.pixels_per_row() as usize + x,
                            onerow_image,
                        );
                        let pred =
                            predict_no_tree_wp(cw, pp, x as i32, y as i32, predictor, &mut state);
                        let val = if index < (nb_deltas as i32) {
                            pred.guess + i64::from(palette_entry)
                        } else {
                            i64::from(palette_entry)
                        };
                        input.channel[ci].row_mut(y)[x] = val as i32;
                        state.update_errors(i64::from(input.channel[ci].row(y)[x]), x, y, cw);
                    }
                }
                Ok(())
            };
            // SAFETY: The channels borrow the image's owners and the local
            // index plane, which hold no thread-affine values.
            unsafe { run_on_pool_exclusive_no_init(pool, 0, nb as u32, input, process_channel) }?;
        } else {
            let process_channel = |input: &mut Image, c: u32, _thread: usize| -> Status {
                let ci = c0i + c as usize;
                let (cw, ch) = (input.channel[ci].w, input.channel[ci].h);
                for y in 0..ch {
                    for x in 0..cw {
                        let index = indices.row(y)[x];
                        let palette_entry = palette_internal::get_palette_value(
                            input.channel[0].plane.pixels().unwrap_or(&[]),
                            index,
                            c as usize,
                            palette_size,
                            onerow,
                            bit_depth,
                        );
                        let val = if index < (nb_deltas as i32) {
                            let channel = &input.channel[ci];
                            let pp = PixelPosition::new(
                                channel.plane.pixels().unwrap(),
                                y * channel.plane.pixels_per_row() as usize + x,
                                onerow_image,
                            );
                            let pred = predict_no_tree_no_wp(cw, pp, x as i32, y as i32, predictor);
                            pred.guess + i64::from(palette_entry)
                        } else {
                            i64::from(palette_entry)
                        };
                        input.channel[ci].row_mut(y)[x] = val as i32;
                    }
                }
                Ok(())
            };
            // SAFETY: As above.
            unsafe { run_on_pool_exclusive_no_init(pool, 0, nb as u32, input, process_channel) }?;
        }
    }
    if c0 as usize >= input.nb_meta_channels {
        input.nb_meta_channels -= 1;
    } else {
        if (input.nb_meta_channels as i32) < 2 - nb {
            return Err(StatusCode::GenericError);
        }
        input.nb_meta_channels = input.nb_meta_channels.wrapping_sub((2 - nb) as usize);
        if begin_c.wrapping_add(nb as u32).wrapping_sub(1) as usize >= input.nb_meta_channels {
            return Err(StatusCode::GenericError);
        }
    }
    crate::jxl::base::libcpp_vector::erase(&mut input.channel, 0, 1);
    Ok(())
}
pub fn meta_palette(
    input: &mut Image,
    begin_c: u32,
    end_c: u32,
    nb_colors: u32,
    nb_deltas: u32,
    _lossy: bool,
) -> Status {
    check_equal_channels(input, begin_c, end_c)?;
    let nb = end_c.wrapping_sub(begin_c).wrapping_add(1) as usize;
    if begin_c as usize >= input.nb_meta_channels {
        input.nb_meta_channels = input.nb_meta_channels.wrapping_add(1);
    } else {
        if end_c as usize >= input.nb_meta_channels {
            return Err(StatusCode::GenericError);
        }
        input.nb_meta_channels = input.nb_meta_channels.wrapping_add(2usize.wrapping_sub(nb));
    }
    crate::jxl::base::libcpp_vector::erase(
        &mut input.channel,
        begin_c.wrapping_add(1) as usize,
        end_c.wrapping_add(1) as usize,
    );
    let memory_manager = input.memory_manager();
    let mut pch = Channel::create_in(
        memory_manager,
        nb_colors.wrapping_add(nb_deltas) as usize,
        nb,
        0,
        0,
    )?;
    pch.hshift = -1;
    pch.vshift = -1;
    input.channel.insert(0, pch);
    Ok(())
}
