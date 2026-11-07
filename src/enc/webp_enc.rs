// libwebp v1.6.0, src/enc/webp_enc.c:
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
// WebP encoder: main entry point
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/webp_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! Exception: WP9 translates `WebPEncodingSetError` and `WebPReportProgress` from `src/enc/webp_enc.c`, because the picture functions and the VP8L encoder report errors and progress through them.
//!
//! Exception: WP13 translates `WebPEncode`, `GetPSNR`, `FinalizePSNR` and `StoreStats` from `src/enc/webp_enc.c`, because `WebPEncode` joins picture conversion, the VP8, VP8L and alpha encoders and the statistics.

use crate::webp::encode::{WebPEncodingError, WebPPicture};

// upstream: src/enc/webp_enc.c WebPEncodingSetError
/// Records `error` as the picture's error code, unless an earlier error is already
/// recorded: the oldest error takes precedence (C: `WebPEncodingSetError`). C
/// returns 0 in every case. This returns the code that the picture holds
/// afterwards, so that a caller that propagates the failure returns
/// `Err(webp_encoding_set_error(..))` with exactly what C leaves in the field.
pub fn webp_encoding_set_error(
    picture: &mut WebPPicture<'_>,
    error: WebPEncodingError,
) -> WebPEncodingError {
    debug_assert!(error.0 < WebPEncodingError::VP8_ENC_ERROR_LAST.0);
    if picture.error_code == WebPEncodingError::VP8_ENC_OK {
        picture.error_code = error;
    }
    picture.error_code
}

// upstream: src/enc/webp_enc.c WebPReportProgress
/// Reports `percent` to the progress hook when it differs from `*percent_store`,
/// and stores it there first (C: `WebPReportProgress`). A `false` answer from the
/// hook records `VP8_ENC_ERROR_USER_ABORT` and returns that recorded code, which is
/// the failure that C signals with 0. A `NULL` `percent_store` is `None`.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_report_progress(
    picture: &mut WebPPicture<'_>,
    percent: i32,
    percent_store: Option<&mut i32>,
) -> Result<(), WebPEncodingError> {
    if let Some(store) = percent_store
        && percent != *store
    {
        *store = percent;
        if let Some(hook) = picture.progress_hook.as_mut()
            && !hook(percent)
        {
            // user abort requested
            return Err(webp_encoding_set_error(
                picture,
                WebPEncodingError::VP8_ENC_ERROR_USER_ABORT,
            ));
        }
    }
    Ok(())
}
