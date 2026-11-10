// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/base/status.h`.
//! Rust's `Result` carries the same numeric error codes. Diagnostic macros and
//! crash-on-error instrumentation are outside the current slice.

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum StatusCode {
    NotEnoughBytes = -1,
    GenericError = 1,
    Unsupported = 2,
}

impl StatusCode {
    pub const fn is_fatal_error(self) -> bool {
        self as i32 > 0
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotEnoughBytes => "not enough bytes",
            Self::GenericError => "generic error",
            Self::Unsupported => "unsupported",
        })
    }
}

impl std::error::Error for StatusCode {}

pub type Status = Result<(), StatusCode>;
pub type StatusOr<T> = Result<T, StatusCode>;

pub const fn status_code(status: &Status) -> i32 {
    match status {
        Ok(()) => 0,
        Err(code) => *code as i32,
    }
}
