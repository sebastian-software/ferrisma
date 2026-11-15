// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Read-only checkpoint observation of the frame encoder (encoder step 1,
//! #118). ComputeVarDCTEncodingData (`enc_frame_vardct.rs`) reports the
//! encoder state after LossyFrameHeuristics, InitializePassesEncoder and
//! ComputeARHeuristics to an observer that a test installs on its thread, so
//! that the state can be compared with the original's at the same calls
//! (tools/oracle/encoder_step1_checkpoints.cc). Without an observer nothing
//! happens. The observer only reads; it cannot change the encoder's state.
//! This is not a translated unit of the original.

use std::cell::RefCell;

use super::enc_cache::PassesEncoderState;
use super::frame_header::FrameHeader;
use super::image::Image3F;

/// The call after which the encoder state is reported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Checkpoint {
    /// LossyFrameHeuristics returned; the image is the Gaborished opsin.
    LossyFrameHeuristics,
    /// InitializePassesEncoder returned; the image is the opsin it read.
    InitializePassesEncoder,
    /// ComputeARHeuristics returned; the image is the pre-Gaborish copy.
    ArHeuristics,
}

/// A checkpoint observer.
pub type CheckpointObserver =
    Box<dyn FnMut(Checkpoint, &FrameHeader<'_>, &PassesEncoderState<'_>, &Image3F)>;

thread_local! {
    static OBSERVER: RefCell<Option<CheckpointObserver>> = const { RefCell::new(None) };
}

/// Installs `observer` for the calling thread and returns the previous one.
pub fn set_checkpoint_observer(observer: Option<CheckpointObserver>) -> Option<CheckpointObserver> {
    OBSERVER.with(|cell| std::mem::replace(&mut *cell.borrow_mut(), observer))
}

/// Reports one checkpoint to the calling thread's observer, if any.
pub(crate) fn observe(
    checkpoint: Checkpoint,
    frame_header: &FrameHeader<'_>,
    enc_state: &PassesEncoderState<'_>,
    image: &Image3F,
) {
    OBSERVER.with(|cell| {
        if let Some(observer) = cell.borrow_mut().as_mut() {
            observer(checkpoint, frame_header, enc_state, image);
        }
    });
}
