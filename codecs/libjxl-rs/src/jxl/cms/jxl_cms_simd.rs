// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The vector bodies of `BeforeTransform` and `AfterTransform` in
//! cms/jxl_cms.cc (`HWY_EXPORT(DoColorSpaceTransform)`) for the NEON and
//! AVX2 targets (ADR 0004): the PQ and sRGB pre- and post-processing around
//! the skcms engine, with the transfer functions of
//! `render_pipeline/simd_transfer.rs`. The HLG branches are scalar code in
//! the source (`TF_HLG_Base` in double and the scalar `ApplyHlgOotf`) and
//! keep the scalar translation in `jxl_cms.rs`. Compare only with the
//! original's SIMD build at the same target.
use super::jxl_cms::{ExtraTf, JxlCms, after_transform, before_transform};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::render_pipeline::simd_transfer::{
    TfPqSimd, srgb_display_from_encoded, srgb_encoded_from_display,
};

#[derive(Clone, Copy)]
enum Transfer {
    PqDisplayFromEncoded(TfPqSimd),
    SrgbDisplayFromEncoded,
    PqEncodedFromDisplay(TfPqSimd),
    SrgbEncodedFromDisplay,
}
impl Transfer {
    #[inline(always)]
    fn apply<D: SimdF32>(self, d: D, v: D::V) -> D::V {
        match self {
            Self::PqDisplayFromEncoded(tf_pq) => tf_pq.display_from_encoded(d, v),
            Self::SrgbDisplayFromEncoded => srgb_display_from_encoded(d, v),
            Self::PqEncodedFromDisplay(tf_pq) => tf_pq.encoded_from_display(d, v),
            Self::SrgbEncodedFromDisplay => srgb_encoded_from_display(d, v),
        }
    }
}

/// The loop shared by the vector branches: whole vectors up to
/// `buf_size - buf_size % Lanes(df)`, then one `LoadN`/`StoreN` vector for
/// the tail. `LoadN` zeroes the lanes past the tail and `StoreN` writes
/// only the tail; every operation is lane-wise, so the tail is computed in
/// a zeroed full vector and only its first `tail` lanes are copied back.
struct TransferRow<'a> {
    values: &'a mut [f32],
    transfer: Transfer,
}
impl Kernel for TransferRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let buf_size = self.values.len();
        let tail = buf_size % D::LANES;
        let vec_buf_end = buf_size - tail;
        let mut i = 0;
        while i < vec_buf_end {
            let val = d.load(&self.values[i..]);
            let result = self.transfer.apply(d, val);
            d.store(result, &mut self.values[i..]);
            i += D::LANES;
        }
        if tail != 0 {
            let mut lanes = [0.0f32; 16];
            lanes[..tail].copy_from_slice(&self.values[vec_buf_end..]);
            let val = d.load(&lanes);
            let result = self.transfer.apply(d, val);
            d.store(result, &mut lanes);
            self.values[vec_buf_end..].copy_from_slice(&lanes[..tail]);
        }
    }
}

/// Run `transfer` over `size` samples from `source` into `destination`
/// (which may be the same memory, as in the source's in-place call).
///
/// # Safety
/// `source` is readable and `destination` writable for `size` samples.
unsafe fn transfer_samples(
    target: Target,
    transfer: Transfer,
    source: *const f32,
    destination: *mut f32,
    size: usize,
) {
    // SAFETY: the caller's extents; the copy ends every read before the
    // first write, so aliasing source and destination is permitted.
    let mut values = unsafe { std::slice::from_raw_parts(source, size) }.to_vec();
    dispatch(
        target,
        TransferRow {
            values: &mut values,
            transfer,
        },
    );
    // SAFETY: the caller's writable extent.
    unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), destination, size) };
}

/// `BeforeTransform` instantiated for one SIMD target.
///
/// # Safety
/// As [`before_transform`].
pub(super) unsafe fn before_transform_simd(
    target: Target,
    t: *const JxlCms,
    source: *const f32,
    destination: *mut f32,
    size: usize,
) -> bool {
    // SAFETY: only the selected branch's fields are read (the caller's
    // contract); the extents are the caller's.
    unsafe {
        let transfer = match (*t).preprocess {
            ExtraTf::PQ => Transfer::PqDisplayFromEncoded(TfPqSimd::new((*t).intensity_target)),
            ExtraTf::SRGB => Transfer::SrgbDisplayFromEncoded,
            // kNone (JXL_ENSURE), kHLG (scalar code in the source) and values
            // outside the switch.
            _ => return before_transform(t, source, destination, size),
        };
        transfer_samples(target, transfer, source, destination, size);
    }
    true
}

/// `AfterTransform` instantiated for one SIMD target.
///
/// # Safety
/// As [`after_transform`].
pub(super) unsafe fn after_transform_simd(
    target: Target,
    t: *const JxlCms,
    buffer: *mut f32,
    size: usize,
) -> bool {
    // SAFETY: as before_transform_simd.
    unsafe {
        let transfer = match (*t).postprocess {
            ExtraTf::PQ => Transfer::PqEncodedFromDisplay(TfPqSimd::new((*t).intensity_target)),
            ExtraTf::SRGB => Transfer::SrgbEncodedFromDisplay,
            _ => return after_transform(t, buffer, size),
        };
        transfer_samples(target, transfer, buffer, buffer, size);
    }
    true
}
