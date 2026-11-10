// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete GroupBorderAssigner from lib/jxl/dec_group_border.{h,cc}.
//! The source's std::atomic<uint32_t> counters and their default sequentially
//! consistent fetch_or/fetch_and/store operations map to AtomicU32 with
//! Ordering::SeqCst. Output rectangles use a fixed array plus count, as in C++.
//! JXL_DASSERTs are omitted, as in the pinned NDEBUG reference graph.
use super::base::rect::Rect;
use super::frame_dimensions::{BLOCK_DIM, FrameDimensions};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default)]
pub struct GroupBorderAssigner {
    frame_dim: FrameDimensions,
    counters: Vec<Vec<AtomicU32>>,
}

impl GroupBorderAssigner {
    pub const MAX_TO_FINALIZE: usize = 3;
    // Constants to identify group positions relative to the corners.
    const TOP_LEFT: u32 = 0x01;
    const TOP_RIGHT: u32 = 0x02;
    const BOTTOM_RIGHT: u32 = 0x04;
    const BOTTOM_LEFT: u32 = 0x08;

    /// Prepare the GroupBorderAssigner to handle a given frame.
    pub fn init(&mut self, frame_dim: &FrameDimensions) {
        self.frame_dim = *frame_dim;
        let x_size = self.frame_dim.xsize_groups;
        let y_size = self.frame_dim.ysize_groups;
        self.counters.resize_with(y_size.wrapping_add(1), Vec::new);
        // Initialize counters.
        for current_row in &mut self.counters {
            let mut row: Vec<AtomicU32> = (0..x_size.wrapping_add(1 + 7) / 8)
                .map(|_| AtomicU32::new(0))
                .collect();
            for v in &row {
                v.store(0, Ordering::SeqCst);
            }
            std::mem::swap(&mut row, current_row);
        }
        // Counters at image borders don't have anything on the other side, we
        // pre-fill their value to have more uniform handling afterwards.
        let set = |counters: &Vec<Vec<AtomicU32>>, x: usize, y: usize, corners: u32| {
            counters[y][x / 8].fetch_or(corners << (4 * (x & 7)), Ordering::SeqCst);
        };
        for x in 0..x_size.wrapping_add(1) {
            set(&self.counters, x, 0, Self::TOP_LEFT | Self::TOP_RIGHT);
            set(
                &self.counters,
                x,
                y_size,
                Self::BOTTOM_LEFT | Self::BOTTOM_RIGHT,
            );
        }
        for y in 0..y_size.wrapping_add(1) {
            set(&self.counters, 0, y, Self::TOP_LEFT | Self::BOTTOM_LEFT);
            set(
                &self.counters,
                x_size,
                y,
                Self::TOP_RIGHT | Self::BOTTOM_RIGHT,
            );
        }
    }

    /// Marks a group as not-done, for running re-paints.
    pub fn clear_done(&self, group_id: usize) {
        let clear = |x: usize, y: usize, corners: u32| {
            self.counters[y][x / 8].fetch_and(!(corners << (4 * (x & 7))), Ordering::SeqCst);
        };
        let x = group_id % self.frame_dim.xsize_groups;
        let y = group_id / self.frame_dim.xsize_groups;
        clear(x, y, Self::BOTTOM_RIGHT);
        clear(x + 1, y, Self::BOTTOM_LEFT);
        clear(x, y + 1, Self::TOP_RIGHT);
        clear(x + 1, y + 1, Self::TOP_LEFT);
    }

    /// Marks a group as done, and returns the (at most 3) rects to run
    /// FinalizeImageRect on.
    ///
    /// Looking at each corner between groups, we can guarantee that the four
    /// involved groups will agree between each other regarding the order in
    /// which each of the four groups terminated. Thus, the last of the four
    /// groups gets the responsibility of handling the corner. For borders,
    /// every border is assigned to its top corner (for vertical borders) or to
    /// its left corner (for horizontal borders): the order as seen on those
    /// corners will decide who handles that border.
    #[expect(
        clippy::needless_range_loop,
        reason = "Retain the source's [x][y] mask indexing."
    )]
    pub fn group_done(
        &self,
        group_id: usize,
        padx: usize,
        pady: usize,
        rects_to_finalize: &mut [Rect; Self::MAX_TO_FINALIZE],
        num_to_finalize: &mut usize,
    ) {
        let fd = &self.frame_dim;
        let x = group_id % fd.xsize_groups;
        let y = group_id / fd.xsize_groups;
        let block_rect = Rect::new_clamped(
            x.wrapping_mul(fd.group_dim) / BLOCK_DIM,
            y.wrapping_mul(fd.group_dim) / BLOCK_DIM,
            fd.group_dim / BLOCK_DIM,
            fd.group_dim / BLOCK_DIM,
            fd.xsize_blocks,
            fd.ysize_blocks,
        );

        let fetch_status = |x: usize, y: usize, bit: u32| -> usize {
            // Note that the acq-rel semantics of this fetch are actually needed
            // to ensure that the pixel data of the group is already written to
            // memory.
            let shift = 4 * (x & 7);
            let mut status =
                self.counters[y][x / 8].fetch_or(bit << shift, Ordering::SeqCst) as usize;
            status >>= shift;
            (bit as usize | status) & 0xF
        };

        let top_left_status = fetch_status(x, y, Self::BOTTOM_RIGHT);
        let top_right_status = fetch_status(x + 1, y, Self::BOTTOM_LEFT);
        let bottom_right_status = fetch_status(x + 1, y + 1, Self::TOP_LEFT);
        let bottom_left_status = fetch_status(x, y + 1, Self::TOP_RIGHT);

        let x1 = block_rect.x0().wrapping_add(block_rect.xsize());
        let y1 = block_rect.y0().wrapping_add(block_rect.ysize());

        let is_last_group_x = fd.xsize_groups == x + 1;
        let is_last_group_y = fd.ysize_groups == y + 1;

        // Start of border of neighbouring group, end of border of this group,
        // start of border of this group (on the other side), end of border of
        // next group.
        let xpos: [usize; 4] = [
            if block_rect.x0() == 0 {
                0
            } else {
                (block_rect.x0() * BLOCK_DIM).wrapping_sub(padx)
            },
            if block_rect.x0() == 0 {
                0
            } else {
                fd.xsize
                    .min((block_rect.x0() * BLOCK_DIM).wrapping_add(padx))
            },
            if is_last_group_x {
                fd.xsize
            } else {
                (x1 * BLOCK_DIM).wrapping_sub(padx)
            },
            fd.xsize.min((x1 * BLOCK_DIM).wrapping_add(padx)),
        ];
        let ypos: [usize; 4] = [
            if block_rect.y0() == 0 {
                0
            } else {
                (block_rect.y0() * BLOCK_DIM).wrapping_sub(pady)
            },
            if block_rect.y0() == 0 {
                0
            } else {
                fd.ysize
                    .min((block_rect.y0() * BLOCK_DIM).wrapping_add(pady))
            },
            if is_last_group_y {
                fd.ysize
            } else {
                (y1 * BLOCK_DIM).wrapping_sub(pady)
            },
            fd.ysize.min((y1 * BLOCK_DIM).wrapping_add(pady)),
        ];

        *num_to_finalize = 0;
        let mut append_rect = |x0: usize, x1: usize, y0: usize, y1: usize| {
            let rect = Rect::new(
                xpos[x0],
                ypos[y0],
                xpos[x1].wrapping_sub(xpos[x0]),
                ypos[y1].wrapping_sub(ypos[y0]),
            );
            if rect.xsize() == 0 || rect.ysize() == 0 {
                return;
            }
            rects_to_finalize[*num_to_finalize] = rect;
            *num_to_finalize += 1;
        };

        // Because of how group borders are assigned, it is impossible that we
        // need to process the left and right side of some area but not the
        // center area. Thus, we compute the first/last part to process in every
        // horizontal strip and merge them together. We first collect a mask of
        // what parts should be processed.
        // We do this horizontally rather than vertically because horizontal
        // borders are larger.
        let mut available_parts_mask = [[false; 3]; 3]; // [x][y]
        // Center
        available_parts_mask[1][1] = true;
        // Corners
        if top_left_status == 0xF {
            available_parts_mask[0][0] = true;
        }
        if top_right_status == 0xF {
            available_parts_mask[2][0] = true;
        }
        if bottom_right_status == 0xF {
            available_parts_mask[2][2] = true;
        }
        if bottom_left_status == 0xF {
            available_parts_mask[0][2] = true;
        }
        // Other borders
        if top_left_status & Self::TOP_RIGHT as usize != 0 {
            available_parts_mask[1][0] = true;
        }
        if top_left_status & Self::BOTTOM_LEFT as usize != 0 {
            available_parts_mask[0][1] = true;
        }
        if top_right_status & Self::BOTTOM_RIGHT as usize != 0 {
            available_parts_mask[2][1] = true;
        }
        if bottom_left_status & Self::BOTTOM_RIGHT as usize != 0 {
            available_parts_mask[1][2] = true;
        }

        // Collect horizontal ranges.
        const NO_SEGMENT: usize = 3;
        let mut horizontal_segments = [(NO_SEGMENT, NO_SEGMENT); 3];
        for py in 0..3 {
            for px in 0..3 {
                if !available_parts_mask[px][py] {
                    continue;
                }
                if horizontal_segments[py].0 == NO_SEGMENT {
                    horizontal_segments[py].0 = px;
                }
                horizontal_segments[py].1 = px + 1;
            }
        }
        if horizontal_segments[0] == horizontal_segments[1]
            && horizontal_segments[0] == horizontal_segments[2]
        {
            append_rect(horizontal_segments[0].0, horizontal_segments[0].1, 0, 3);
        } else if horizontal_segments[0] == horizontal_segments[1] {
            append_rect(horizontal_segments[0].0, horizontal_segments[0].1, 0, 2);
            append_rect(horizontal_segments[2].0, horizontal_segments[2].1, 2, 3);
        } else if horizontal_segments[1] == horizontal_segments[2] {
            append_rect(horizontal_segments[0].0, horizontal_segments[0].1, 0, 1);
            append_rect(horizontal_segments[1].0, horizontal_segments[1].1, 1, 3);
        } else {
            append_rect(horizontal_segments[0].0, horizontal_segments[0].1, 0, 1);
            append_rect(horizontal_segments[1].0, horizontal_segments[1].1, 1, 2);
            append_rect(horizontal_segments[2].0, horizontal_segments[2].1, 2, 3);
        }
    }
}
