// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Test observation of what a decode reaches (port ADR 0004, "Decoder
//! path"): the frame header facts and render pipeline stages that
//! `PassesDecoderState::PreparePipeline` builds, and the modular inverse
//! transforms it applies. `tests/simd_public.rs` prints them for every file
//! whose whole decode through the public decoder still differs from the
//! original's SIMD build, so that each entry of its punch list names the
//! still-scalar stage or kernel the file reaches. Not part of the original;
//! it records only while a test has started it and never changes a decode.
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);
static EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Start recording, discarding earlier events. Process-wide: the caller
/// decodes on one thread while it records.
#[doc(hidden)]
pub fn start() {
    EVENTS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    ACTIVE.store(true, Ordering::Relaxed);
}

/// Stop recording and return the events in order.
#[doc(hidden)]
pub fn stop() -> Vec<String> {
    ACTIVE.store(false, Ordering::Relaxed);
    std::mem::take(&mut *EVENTS.lock().unwrap_or_else(|e| e.into_inner()))
}

/// Record one event while a test records; `event` is evaluated only then.
pub(crate) fn note(event: impl FnOnce() -> String) {
    if ACTIVE.load(Ordering::Relaxed) {
        let event = event();
        EVENTS.lock().unwrap_or_else(|e| e.into_inner()).push(event);
    }
}
