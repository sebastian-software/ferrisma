// libwebp v1.6.0, src/dec/vp8_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// main entry for the decoder
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/dec/vp8_dec.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  Low-level API for VP8 decoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/vp8_dec.c` and `src/dec/vp8_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! Exception: WP7 translates `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook` and `VP8IoTeardownHook` from `src/dec/vp8_dec.h`, because `VP8LDecoder` and `ALPHDecoder` hold a `VP8Io`.

// The items below are WP7's exception in this module (`tools/units.json`):
// `VP8Io` and its three hook types. ADR 0003 ("Decoding context per call")
// fixes their form: `VP8Io` keeps its scalar fields, and the input, the sample
// rows, `opaque` and the hooks are passed to each call.

// upstream: src/dec/vp8_dec.h VP8Io
/// Input and output parameters of a decoding call (C: `VP8Io`).
///
/// The C pointers are not fields (ADR 0003, "Decoding context per call"): the
/// input `data` and `data_size` is a `data: &[u8]` argument of every function that
/// reads it, the rows `y`, `u`, `v` and `a` reach the put hook as [`VP8IoRows`],
/// and `opaque` with `put`, `setup` and `teardown` is the [`VP8IoHooks`] argument
/// of the functions that call hooks. The `int` flags that C uses only as
/// predicates are `bool`. `Default` is the zeroed structure of `VP8InitIo`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the int flags of the C structure, which it uses as predicates"
)]
pub struct VP8Io {
    /// Picture width in pixels, set by the header parsers (invariable). These are
    /// the original, uncropped dimensions; the area passed to `put()` is
    /// `mb_w` by `mb_h`.
    pub width: i32,
    /// Picture height in pixels, set by the header parsers (invariable).
    pub height: i32,
    /// Position of the current rows, in pixels; set before calling `put()`.
    pub mb_y: i32,
    /// Number of columns in the sample.
    pub mb_w: i32,
    /// Number of rows in the sample.
    pub mb_h: i32,
    /// Row stride for luma.
    pub y_stride: i32,
    /// Row stride for chroma.
    pub uv_stride: i32,
    /// Recommendation for the user-side YUV to RGB converter. Set when calling
    /// the setup hook, which can overwrite it; `put()` can take it into account.
    pub fancy_upsampling: bool,
    /// If true, in-loop filtering is not performed even if present in the
    /// bitstream. The output is then not compliant with the VP8 specification.
    pub bypass_filtering: bool,
    /// Whether cropping applies.
    pub use_cropping: bool,
    /// Left edge of the crop window.
    pub crop_left: i32,
    /// Right edge (exclusive) of the crop window.
    pub crop_right: i32,
    /// Top edge of the crop window.
    pub crop_top: i32,
    /// Bottom edge (exclusive) of the crop window.
    pub crop_bottom: i32,
    /// Whether scaling applies.
    pub use_scaling: bool,
    /// Width after scaling.
    pub scaled_width: i32,
    /// Height after scaling.
    pub scaled_height: i32,
}

/// The rows that C passes to `put()` through `io->y`, `io->u`, `io->v` and
/// `io->a` (ADR 0003, "Decoding context per call"). The luma and chroma slices
/// start where the C pointers point, and the rows follow at `y_stride`,
/// `uv_stride` and `width`. `y` is mutable because `EmitRescaledYUV`
/// premultiplies the luma through the `const` pointer. The alpha data is
/// persistent, and `EmitAlphaRGB` steps back one row from `io->a`, so `a` holds
/// the alpha plane from its first row and `a_offset` is the index of C's `io->a`
/// in it (WP6). `a` is `None` where C has `io->a == NULL`. This type has no C
/// counterpart.
#[derive(Debug)]
pub struct VP8IoRows<'r> {
    /// Luma rows (C: `io->y`).
    pub y: &'r mut [u8],
    /// Chroma U rows (C: `io->u`).
    pub u: &'r [u8],
    /// Chroma V rows (C: `io->v`).
    pub v: &'r [u8],
    /// The alpha plane, from its first row; `None` where C has `io->a == NULL`.
    pub a: Option<&'r [u8]>,
    /// Index in `a` of C's `io->a`: the alpha rows, pre-offset by `mb_y` and the
    /// crop.
    pub a_offset: usize,
}

/// The hooks of a `VP8Io` and their user data (C: `put`, `setup`, `teardown` and
/// `opaque`), passed per call as `Option<&mut dyn VP8IoHooks>` (ADR 0003,
/// "Decoding context per call"). The implementing value is the C `opaque`. The
/// `has_*` methods answer C's test of each hook against `NULL`; a caller tests
/// them before it calls the hook, as C does.
pub trait VP8IoHooks {
    // upstream: src/dec/vp8_dec.h VP8IoSetupHook
    /// Called just before starting to decode the blocks. Must return false in
    /// case of setup error, true otherwise. If false is returned, `teardown()` is
    /// not called; if true is returned, `teardown()` is always called afterward.
    fn setup(&mut self, io: &mut VP8Io) -> bool;

    // upstream: src/dec/vp8_dec.h VP8IoPutHook
    /// Called when fresh samples are available. The samples are in YUV420 format
    /// and can be up to `width` by 24 in size. Returns false in case of error or
    /// abort request. The area to update is `mb_w` by `mb_h`, taking cropping into
    /// account.
    fn put(&mut self, io: &VP8Io, rows: &mut VP8IoRows<'_>) -> bool;

    // upstream: src/dec/vp8_dec.h VP8IoTeardownHook
    /// Called just after block decoding is finished, or when an error occurred
    /// during `put()`. Not called if `setup()` failed.
    fn teardown(&mut self, io: &VP8Io);

    /// Whether the setup hook is set (C: `io->setup != NULL`).
    fn has_setup(&self) -> bool {
        true
    }

    /// Whether the put hook is set (C: `io->put != NULL`).
    fn has_put(&self) -> bool {
        true
    }

    /// Whether the teardown hook is set (C: `io->teardown != NULL`).
    fn has_teardown(&self) -> bool {
        true
    }
}
