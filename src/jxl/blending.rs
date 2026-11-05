// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete lib/jxl/blending.{h,cc}. PerformBlending returns the original
//! temporary image's rows; callers copy them to their output only on success.
use super::alpha::*;
use super::base::status::{StatusCode, StatusOr};
use super::dec_patch_dictionary::{PatchBlendMode as Mode, PatchBlending};
use super::frame_header::{BlendMode, FrameHeader, FrameType};
use super::image_metadata::ExtraChannel;
use super::image_metadata::ExtraChannelInfo;
use super::memory_manager_internal::MemoryManagerPtr;

pub fn needs_blending(frame: &FrameHeader) -> bool {
    if frame.frame_type != FrameType::REGULAR && frame.frame_type != FrameType::SKIP_PROGRESSIVE {
        return false;
    }
    let mut replace_all = frame.blending_info.mode == BlendMode::REPLACE;
    for ec in &frame.extra_channel_blending_info {
        if ec.mode != BlendMode::REPLACE {
            replace_all = false;
        }
    }
    frame.custom_size_or_origin || !replace_all
}
pub fn perform_blending(
    bg: &[&[f32]],
    fg: &[&[f32]],
    x0: usize,
    width: usize,
    color: &PatchBlending,
    ec: &[PatchBlending],
    extra: &[ExtraChannelInfo],
) -> StatusOr<super::image::Plane<f32>> {
    perform_blending_in(
        MemoryManagerPtr::NATIVE,
        bg,
        fg,
        x0,
        width,
        color,
        ec,
        extra,
    )
}
/// The source PerformBlending(memory_manager, bg, fg, ...).
#[expect(
    clippy::too_many_arguments,
    reason = "Retain the source PerformBlending parameters"
)]
pub fn perform_blending_in(
    memory_manager: MemoryManagerPtr,
    bg: &[&[f32]],
    fg: &[&[f32]],
    x0: usize,
    width: usize,
    color: &PatchBlending,
    ec: &[PatchBlending],
    extra: &[ExtraChannelInfo],
) -> StatusOr<super::image::Plane<f32>> {
    let n = extra.len();
    let has_alpha = extra
        .iter()
        .any(|info| info.channel_type == ExtraChannel::ALPHA);
    // ImageF::Create is the source allocation/failure checkpoint.
    let mut tmp = super::image::Plane::<f32>::create_in(width, 3 + n, 0, memory_manager)?;
    if width == 0 {
        return Ok(tmp);
    }
    for i in 0..n {
        let b = &bg[3 + i][x0..];
        let f = &fg[3 + i][x0..];
        let out = &mut tmp.row_mut(3 + i)[..width];
        let a = ec[i].alpha_channel as usize;
        match ec[i].mode {
            Mode::ADD => {
                for x in 0..width {
                    out[x] = b[x] + f[x];
                }
            }
            Mode::BLEND_ABOVE => perform_alpha_blending(
                b,
                &bg[3 + a][x0..],
                f,
                &fg[3 + a][x0..],
                out,
                width,
                extra[a].alpha_associated,
                ec[i].clamp,
            ),
            Mode::BLEND_BELOW => perform_alpha_blending(
                f,
                &fg[3 + a][x0..],
                b,
                &bg[3 + a][x0..],
                out,
                width,
                extra[a].alpha_associated,
                ec[i].clamp,
            ),
            Mode::ALPHA_WEIGHTED_ADD_ABOVE => {
                perform_alpha_weighted_add(b, f, &fg[3 + a][x0..], out, width, ec[i].clamp)
            }
            Mode::ALPHA_WEIGHTED_ADD_BELOW => {
                perform_alpha_weighted_add(f, b, &bg[3 + a][x0..], out, width, ec[i].clamp)
            }
            Mode::MUL => perform_mul_blending(b, f, out, width, ec[i].clamp),
            Mode::REPLACE => out.copy_from_slice(&f[..width]),
            Mode::NONE => out.copy_from_slice(&b[..width]),
            _ => return Err(StatusCode::GenericError),
        }
    }
    let a = color.alpha_channel as usize;
    match color.mode {
        Mode::ADD => {
            for c in 0..3 {
                for x in 0..width {
                    tmp.row_mut(c)[x] = bg[c][x + x0] + fg[c][x + x0];
                }
            }
        }
        Mode::ALPHA_WEIGHTED_ADD_ABOVE | Mode::ALPHA_WEIGHTED_ADD_BELOW => {
            if has_alpha {
                let (bottom, top) = if color.mode == Mode::ALPHA_WEIGHTED_ADD_ABOVE {
                    (bg, fg)
                } else {
                    (fg, bg)
                };
                for c in 0..3 {
                    perform_alpha_weighted_add(
                        &bottom[c][x0..],
                        &top[c][x0..],
                        &top[3 + a][x0..],
                        &mut tmp.row_mut(c)[..width],
                        width,
                        color.clamp,
                    );
                }
            } else {
                for c in 0..3 {
                    for x in 0..width {
                        tmp.row_mut(c)[x] = bg[c][x + x0] + fg[c][x + x0];
                    }
                }
            }
        }
        Mode::BLEND_ABOVE | Mode::BLEND_BELOW => {
            if has_alpha {
                let (bottom, top) = if color.mode == Mode::BLEND_ABOVE {
                    (bg, fg)
                } else {
                    (fg, bg)
                };
                let stride = tmp.pixels_per_row() as usize;
                let mut rows = tmp.pixels_mut().unwrap().chunks_mut(stride);
                let r = rows.next().unwrap();
                let g = rows.next().unwrap();
                let b = rows.next().unwrap();
                let alpha = rows.nth(a).unwrap();
                let mut rgba = [
                    &mut r[..width],
                    &mut g[..width],
                    &mut b[..width],
                    &mut alpha[..width],
                ];
                perform_alpha_blending_rgba(
                    [
                        &bottom[0][x0..],
                        &bottom[1][x0..],
                        &bottom[2][x0..],
                        &bottom[3 + a][x0..],
                    ],
                    [
                        &top[0][x0..],
                        &top[1][x0..],
                        &top[2][x0..],
                        &top[3 + a][x0..],
                    ],
                    &mut rgba,
                    width,
                    extra[a].alpha_associated,
                    color.clamp,
                );
            } else {
                for (c, row) in fg.iter().take(3).enumerate() {
                    tmp.row_mut(c)[..width].copy_from_slice(&row[x0..x0 + width]);
                }
            }
        }
        Mode::MUL => {
            for c in 0..3 {
                perform_mul_blending(
                    &bg[c][x0..],
                    &fg[c][x0..],
                    &mut tmp.row_mut(c)[..width],
                    width,
                    color.clamp,
                );
            }
        }
        Mode::REPLACE | Mode::NONE => {
            let src = if color.mode == Mode::REPLACE { fg } else { bg };
            for (c, row) in src.iter().take(3).enumerate() {
                tmp.row_mut(c)[..width].copy_from_slice(&row[x0..x0 + width]);
            }
        }
        _ => return Err(StatusCode::GenericError),
    }
    Ok(tmp)
}
