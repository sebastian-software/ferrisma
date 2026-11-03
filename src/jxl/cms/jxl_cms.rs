// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Default CMS owner, row execution and callbacks from pinned
//! `lib/jxl/cms/jxl_cms.cc`:60-317,1108-1207,1379-1662.
//! The reference selects Highway's scalar wrapper, independently of skcms N=4.
//! Context metadata stays immutable during row callbacks; only the source's
//! separate per-thread float allocations are interior mutable.

use super::color_encoding_cms::{ColorEncoding, PrimariesCieXy, TransferFunction};
use super::icc::{cie_xyz_from_white_cie_xy, maybe_create_profile};
use super::jxl_cms_profile::{decode_profile, set_fields_from_icc};
use super::transfer_functions::TfHlgBase;
use super::transfer_functions_scalar::{
    TfPq, srgb_display_from_encoded, srgb_encoded_from_display,
};
use crate::jxl::base::matrix_ops::inv3x3_matrix;
use crate::jxl::cms_interface::{JxlCmsInterface, JxlColorProfile};
use crate::skcms::profile::IccProfile;
use crate::skcms::transform::{make_usable_as_destination, transform};
use crate::skcms::transform_types::{AlphaFormat, PixelFormat};
use std::cell::UnsafeCell;
use std::ffi::{c_int, c_void};
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(super) struct ExtraTf(pub(super) i32);
impl ExtraTf {
    pub(super) const NONE: Self = Self(0);
    pub(super) const PQ: Self = Self(1);
    pub(super) const HLG: Self = Self(2);
    pub(super) const SRGB: Self = Self(3);
}

pub(super) struct JxlCms {
    pub(super) icc_src: Vec<u8>,
    pub(super) icc_dst: Vec<u8>,
    pub(super) profile_src: IccProfile,
    pub(super) profile_dst: IccProfile,
    pub(super) apply_hlg_ootf: bool,
    pub(super) hlg_ootf_num_channels: usize,
    pub(super) hlg_ootf_luminances: [f32; 3],
    pub(super) channels_src: usize,
    pub(super) channels_dst: usize,
    pub(super) src_storage: Vec<UnsafeCell<f32>>,
    pub(super) buf_src: Vec<*mut f32>,
    pub(super) dst_storage: Vec<UnsafeCell<f32>>,
    pub(super) buf_dst: Vec<*mut f32>,
    pub(super) intensity_target: f32,
    pub(super) skip_lcms: bool,
    pub(super) preprocess: ExtraTf,
    pub(super) postprocess: ExtraTf,
}
impl JxlCms {
    fn new() -> Self {
        // make_unique<JxlCms>() value-initializes scalar members before their
        // default member initializers and vector constructors.
        Self {
            icc_src: Vec::new(),
            icc_dst: Vec::new(),
            profile_src: IccProfile::zeroed(),
            profile_dst: IccProfile::zeroed(),
            apply_hlg_ootf: false,
            hlg_ootf_num_channels: 0,
            hlg_ootf_luminances: [0.0; 3],
            channels_src: 0,
            channels_dst: 0,
            src_storage: Vec::new(),
            buf_src: Vec::new(),
            dst_storage: Vec::new(),
            buf_dst: Vec::new(),
            intensity_target: 0.0,
            skip_lcms: false,
            preprocess: ExtraTf::NONE,
            postprocess: ExtraTf::NONE,
        }
    }
}
impl Drop for JxlCms {
    fn drop(&mut self) {
        // C++ destroys owning members in reverse declaration order. Empty the
        // Rust fields in that same order before its automatic field destruction;
        // raw profile words and pointer views never dereference their backing.
        drop(std::mem::take(&mut self.buf_dst));
        drop(std::mem::take(&mut self.dst_storage));
        drop(std::mem::take(&mut self.buf_src));
        drop(std::mem::take(&mut self.src_storage));
        drop(std::mem::take(&mut self.icc_dst));
        drop(std::mem::take(&mut self.icc_src));
    }
}

/// # Safety
/// Wrapped source allocation/stride arithmetic covers every returned thread view.
pub(super) unsafe fn allocate_buffer(
    length: usize,
    threads: usize,
    storage: &mut Vec<UnsafeCell<f32>>,
    view: &mut Vec<*mut f32>,
) {
    const ALIGN: usize = 32;
    let stride = length.wrapping_add(ALIGN - 1) / ALIGN * ALIGN;
    let size = stride.wrapping_mul(threads).wrapping_add(ALIGN);
    storage.resize_with(size, || UnsafeCell::new(0.0));
    let address = storage.as_ptr() as usize;
    let offset = (address.wrapping_add(127) / 128 * 128).wrapping_sub(address) / 4;
    view.clear();
    view.reserve(threads);
    for i in 0..threads {
        // SAFETY: source caller requires the wrapped allocation/stride to cover
        // every thread view. UnsafeCell<float> has float layout/alignment; no
        // reference to the float storage is retained across callback writes.
        view.push(unsafe {
            storage
                .as_mut_ptr()
                .add(offset + i.wrapping_mul(stride))
                .cast::<f32>()
        });
    }
}

/// Output is required writable only when the original prerequisites succeed.
pub(super) unsafe fn get_primaries_luminances(encoding: &ColorEncoding, output: *mut f32) -> bool {
    let mut white = [0.0; 3];
    let wp = encoding.get_white_point();
    if cie_xyz_from_white_cie_xy(wp.x, wp.y, &mut white).is_err() {
        return false;
    }
    let mut primaries = PrimariesCieXy::default();
    if encoding.get_primaries(&mut primaries).is_err() {
        return false;
    }
    let mut chromaticities = [
        [primaries.r.x, primaries.g.x, primaries.b.x],
        [primaries.r.y, primaries.g.y, primaries.b.y],
        [
            1.0 - primaries.r.x - primaries.r.y,
            1.0 - primaries.g.x - primaries.g.y,
            1.0 - primaries.b.x - primaries.b.y,
        ],
    ];
    if inv3x3_matrix(&mut chromaticities).is_err() {
        return false;
    }
    let ys = [primaries.r.y, primaries.g.y, primaries.b.y];
    for i in 0..3 {
        let value = (ys[i]
            * (chromaticities[i][0] * f64::from(white[0])
                + chromaticities[i][1] * f64::from(white[1])
                + chromaticities[i][2] * f64::from(white[2]))) as f32;
        // SAFETY: caller supplies three writable words on this reached branch;
        // their old values are not read and need not be initialized.
        unsafe {
            output.add(i).write(value);
        }
    }
    true
}

#[expect(
    clippy::manual_range_contains,
    reason = "Retain the original ordered intensity guards."
)]
pub(super) unsafe fn apply_hlg_ootf(
    t: *const JxlCms,
    buffer: *mut f32,
    size: usize,
    forward: bool,
) -> bool {
    // SAFETY: caller initializes only the source-reached context fields and
    // supplies the reached sample extent. RGB contexts require complete triples.
    unsafe {
        let intensity = (*t).intensity_target;
        if 295.0 <= intensity && intensity <= 305.0 {
            return true;
        }
        let mut gamma = 1.2_f32 * 1.111_f32.powf((intensity * 1e-3_f32).log2());
        if !forward {
            gamma = 1.0 / gamma;
        }
        match (*t).hlg_ootf_num_channels {
            1 => {
                for x in 0..size {
                    buffer.add(x).write(buffer.add(x).read().powf(gamma));
                }
            }
            3 => {
                let mut x = 0;
                while x < size {
                    // The actual original ARM row object and private helper
                    // round G*Yg first, then contract R*Yr and B*Yb in order.
                    #[cfg(target_arch = "aarch64")]
                    let luminance = (*buffer.add(x + 2)).mul_add(
                        (*t).hlg_ootf_luminances[2],
                        (*buffer.add(x)).mul_add(
                            (*t).hlg_ootf_luminances[0],
                            *buffer.add(x + 1) * (*t).hlg_ootf_luminances[1],
                        ),
                    );
                    #[cfg(not(target_arch = "aarch64"))]
                    let luminance = *buffer.add(x) * (*t).hlg_ootf_luminances[0]
                        + *buffer.add(x + 1) * (*t).hlg_ootf_luminances[1]
                        + *buffer.add(x + 2) * (*t).hlg_ootf_luminances[2];
                    let ratio = luminance.powf(gamma - 1.0);
                    if ratio.is_finite() {
                        for c in 0..3 {
                            *buffer.add(x + c) *= ratio;
                        }
                        if forward && gamma < 1.0 {
                            // std::max uses its ordered comparison, preserving
                            // first operands for unordered/equal float values.
                            let a = *buffer.add(x + 1);
                            let b = *buffer.add(x + 2);
                            let inner = if a < b { b } else { a };
                            let a = *buffer.add(x);
                            let maximum = if a < inner { inner } else { a };
                            if maximum > 1.0 {
                                let normalizer = 1.0 / maximum;
                                for c in 0..3 {
                                    *buffer.add(x + c) *= normalizer;
                                }
                            }
                        }
                    }
                    x += 3;
                }
            }
            _ => return false,
        }
        true
    }
}

pub(super) unsafe fn before_transform(
    t: *const JxlCms,
    source: *const f32,
    destination: *mut f32,
    size: usize,
) -> bool {
    // SAFETY: only fields belonging to the selected original TF branch are
    // reached. Each scalar load precedes its store, retaining permitted aliasing.
    unsafe {
        match (*t).preprocess {
            ExtraTf::NONE => return false,
            ExtraTf::PQ => {
                let tf = TfPq::new((*t).intensity_target);
                for i in 0..size {
                    destination
                        .add(i)
                        .write(tf.display_from_encoded(source.add(i).read()));
                }
            }
            ExtraTf::HLG => {
                for i in 0..size {
                    destination
                        .add(i)
                        .write(
                            TfHlgBase::display_from_encoded(f64::from(source.add(i).read())) as f32,
                        );
                }
                if (*t).apply_hlg_ootf && !apply_hlg_ootf(t, destination, size, true) {
                    return false;
                }
            }
            ExtraTf::SRGB => {
                for i in 0..size {
                    destination
                        .add(i)
                        .write(srgb_display_from_encoded(source.add(i).read()));
                }
            }
            _ => {} // Original switch has no default.
        }
        true
    }
}

pub(super) unsafe fn after_transform(t: *const JxlCms, buffer: *mut f32, size: usize) -> bool {
    // SAFETY: selected context fields and source-sized writable samples are
    // initialized. HLG OOTF errors retain its source partial writes.
    unsafe {
        match (*t).postprocess {
            ExtraTf::NONE => {} // JXL_DEBUG_ABORT is empty in the pinned NDEBUG graph.
            ExtraTf::PQ => {
                let tf = TfPq::new((*t).intensity_target);
                for i in 0..size {
                    buffer
                        .add(i)
                        .write(tf.encoded_from_display(buffer.add(i).read()));
                }
            }
            ExtraTf::HLG => {
                if (*t).apply_hlg_ootf && !apply_hlg_ootf(t, buffer, size, false) {
                    return false;
                }
                for i in 0..size {
                    buffer
                        .add(i)
                        .write(
                            TfHlgBase::encoded_from_display(f64::from(buffer.add(i).read())) as f32,
                        );
                }
            }
            ExtraTf::SRGB => {
                for i in 0..size {
                    buffer
                        .add(i)
                        .write(srgb_encoded_from_display(buffer.add(i).read()));
                }
            }
            _ => {} // Original switch has no default.
        }
        true
    }
}

/// Original row callback; concurrent calls use independent valid thread buffers.
///
/// # Safety
/// `data` is a live context returned by init. Thread/sample extents and aliasing
/// obey cms_interface.h, including nonoverlap on the source memcpy branch.
pub unsafe extern "C" fn run(
    data: *mut c_void,
    thread: usize,
    source: *const f32,
    destination: *mut f32,
    width: usize,
) -> c_int {
    let t = data.cast::<JxlCms>();
    let mut input = source;
    // SAFETY: caller supplies a live stable owner and distinct per-thread mutable
    // sample extents. No mutable context/Vec reference spans concurrent runs.
    unsafe {
        if (*t).preprocess != ExtraTf::NONE {
            let buffer = *(&*ptr::addr_of!((*t).buf_src)).get_unchecked(thread);
            if !before_transform(t, source, buffer, width.wrapping_mul((*t).channels_src)) {
                return 0;
            }
            input = buffer;
        }
        if (*t).channels_src == 1 && !(*t).skip_lcms {
            let buffer = *(&*ptr::addr_of!((*t).buf_src)).get_unchecked(thread);
            for i in 0..width {
                let x = width - i - 1;
                let v = *input.add(x);
                *buffer.add(x * 3 + 2) = v;
                *buffer.add(x * 3 + 1) = v;
                *buffer.add(x * 3) = v;
            }
            input = buffer;
        }
        if (*t).skip_lcms {
            if !ptr::eq(destination, input) {
                ptr::copy_nonoverlapping(input, destination, width.wrapping_mul((*t).channels_src));
            }
        } else {
            let output = if (*t).channels_dst == 1 {
                *(&*ptr::addr_of!((*t).buf_dst)).get_unchecked(thread)
            } else {
                destination
            };
            if !transform(
                input.cast(),
                if (*t).channels_src == 4 {
                    PixelFormat::RGBA_ffff
                } else {
                    PixelFormat::RGB_fff
                },
                AlphaFormat::Opaque,
                ptr::addr_of!((*t).profile_src),
                output.cast(),
                PixelFormat::RGB_fff,
                AlphaFormat::Opaque,
                ptr::addr_of!((*t).profile_dst),
                width,
            ) {
                return 0;
            }
            if (*t).channels_dst == 1 {
                for x in 0..width {
                    *destination.add(x) = *output.add(x * 3);
                }
            }
        }
        if (*t).postprocess != ExtraTf::NONE
            && !after_transform(t, destination, width.wrapping_mul((*t).channels_dst))
        {
            return 0;
        }
        1
    }
}

/// Original destroy callback, including its null no-op.
///
/// # Safety
/// Non-null data is an undestroyed init result with no outstanding callbacks.
pub unsafe extern "C" fn destroy(data: *mut c_void) {
    if data.is_null() {
        return;
    }
    // SAFETY: caller transfers the unique Box allocation after all uses finish.
    unsafe {
        drop(Box::from_raw(data.cast::<JxlCms>()));
    }
}

/// # Safety
/// Data is a live init result; thread is in its allocated view range.
pub unsafe extern "C" fn get_src_buf(data: *mut c_void, thread: usize) -> *mut f32 {
    // SAFETY: only immutable initialized vector metadata is borrowed. Its raw
    // float views refer to interior-mutable storage, not a mutable owner borrow.
    unsafe { *(&*ptr::addr_of!((*data.cast::<JxlCms>()).buf_src)).get_unchecked(thread) }
}
/// # Safety
/// Data is a live init result; thread is in its allocated view range.
pub unsafe extern "C" fn get_dst_buf(data: *mut c_void, thread: usize) -> *mut f32 {
    // SAFETY: same source buffer-view contract as get_src_buf.
    unsafe { *(&*ptr::addr_of!((*data.cast::<JxlCms>()).buf_dst)).get_unchecked(thread) }
}

unsafe fn set_encoding_from_icc(
    encoding: &mut ColorEncoding,
    icc: &mut Vec<u8>,
    cms: *const JxlCmsInterface,
) -> bool {
    if icc.is_empty() {
        return false;
    }
    // SAFETY: source reaches these two fields only after its empty-ICC guard.
    // Reload on each invocation: the preceding callback may modify its mutable
    // interface/descriptor backing. Other interface fields remain unobserved.
    unsafe {
        let callback = JxlCmsInterface {
            set_fields_data: ptr::addr_of!((*cms).set_fields_data).read(),
            set_fields_from_icc: ptr::addr_of!((*cms).set_fields_from_icc).read(),
            ..JxlCmsInterface::default()
        };
        encoding.set_fields_from_icc(icc, &callback).is_ok()
    }
}

/// Original default CMS init callback and generated linear-profile ownership.
///
/// # Safety
/// Non-null interface data initializes its reached ICC callback/data fields.
/// Reached profile ICC spans are valid; their backing outlives the returned
/// context unless replaced by its generated owned intermediate bytes. Scalar
/// sizes satisfy the original allocation and pointer-arithmetic preconditions.
pub unsafe extern "C" fn init(
    data: *mut c_void,
    threads: usize,
    width: usize,
    input: *const JxlColorProfile,
    output: *const JxlColorProfile,
    intensity: f32,
) -> *mut c_void {
    if data.is_null() {
        return ptr::null_mut();
    }
    let cms = data.cast::<JxlCmsInterface>();
    let mut t = Box::new(JxlCms::new());
    // SAFETY: raw prefix reads retain empty-span exits without reading inactive
    // external encoding/channel fields or constructing whole profile references.
    unsafe {
        let source_size = ptr::addr_of!((*input).icc.size).read();
        if source_size == 0 {
            return ptr::null_mut();
        }
        let destination_size = ptr::addr_of!((*output).icc.size).read();
        if destination_size == 0 {
            return ptr::null_mut();
        }
        let source_bytes = ptr::addr_of!((*input).icc.data).read();
        let mut icc_src = std::slice::from_raw_parts(source_bytes, source_size).to_vec();
        let mut c_src = ColorEncoding::default();
        if !set_encoding_from_icc(&mut c_src, &mut icc_src, cms) {
            return ptr::null_mut();
        }
        let destination_bytes = ptr::addr_of!((*output).icc.data).read();
        let destination_size = ptr::addr_of!((*output).icc.size).read();
        // The first callback may replace this span with null+zero after the
        // initial nonempty guard. Original vector assignment accepts that
        // empty range without reading its backing; a Rust slice requires a
        // non-null pointer even when empty. Still load both descriptor fields
        // at their original call site, then preserve the empty-copy path.
        let mut icc_dst = if destination_size == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(destination_bytes, destination_size).to_vec()
        };
        let mut c_dst = ColorEncoding::default();
        if !set_encoding_from_icc(&mut c_dst, &mut icc_dst, cms) {
            return ptr::null_mut();
        }
        if !decode_profile(
            ptr::addr_of!((*input).icc.data).read(),
            ptr::addr_of!((*input).icc.size).read(),
            &mut t.profile_src,
        ) || !decode_profile(
            ptr::addr_of!((*output).icc.data).read(),
            ptr::addr_of!((*output).icc.size).read(),
            &mut t.profile_dst,
        ) {
            return ptr::null_mut();
        }
        t.skip_lcms = c_src.same_color_encoding(&c_dst);
        t.apply_hlg_ootf = c_src.tf.is_hlg() != c_dst.tf.is_hlg();
        if t.apply_hlg_ootf {
            let hlg = if c_src.tf.is_hlg() { &c_src } else { &c_dst };
            t.hlg_ootf_num_channels = hlg.channels();
            if t.hlg_ootf_num_channels == 3
                && !get_primaries_luminances(hlg, t.hlg_ootf_luminances.as_mut_ptr())
            {
                return ptr::null_mut();
            }
        }
        let mut source_linear = c_src.tf.is_linear();
        let destination_linear = c_dst.tf.is_linear();
        if c_src.tf.is_pq()
            || c_src.tf.is_hlg()
            || (c_src.tf.is_srgb() && destination_linear && c_src.same_color_space(&c_dst))
        {
            let mut linear = c_src.clone();
            linear.tf.set_transfer_function(TransferFunction::LINEAR);
            let mut profile = MaybeUninit::<IccProfile>::uninit();
            if maybe_create_profile(&linear.to_external(), &mut icc_src).is_ok()
                && decode_profile(icc_src.as_ptr(), icc_src.len(), profile.as_mut_ptr())
            {
                t.icc_src = icc_src;
                t.profile_src = profile.assume_init();
                t.preprocess = if c_src.tf.is_srgb() {
                    ExtraTf::SRGB
                } else if c_src.tf.is_pq() {
                    ExtraTf::PQ
                } else {
                    ExtraTf::HLG
                };
                c_src = linear;
                source_linear = true;
            } else if t.apply_hlg_ootf {
                return ptr::null_mut();
            }
        }
        if c_dst.tf.is_pq()
            || c_dst.tf.is_hlg()
            || (c_dst.tf.is_srgb() && source_linear && c_src.same_color_space(&c_dst))
        {
            let mut linear = c_dst.clone();
            linear.tf.set_transfer_function(TransferFunction::LINEAR);
            let mut profile = MaybeUninit::<IccProfile>::uninit();
            if maybe_create_profile(&linear.to_external(), &mut icc_dst).is_ok()
                && decode_profile(icc_dst.as_ptr(), icc_dst.len(), profile.as_mut_ptr())
            {
                t.icc_dst = icc_dst;
                t.profile_dst = profile.assume_init();
                t.postprocess = if c_dst.tf.is_srgb() {
                    ExtraTf::SRGB
                } else if c_dst.tf.is_pq() {
                    ExtraTf::PQ
                } else {
                    ExtraTf::HLG
                };
                c_dst = linear;
            } else if t.apply_hlg_ootf {
                return ptr::null_mut();
            }
        }
        if c_src.same_color_encoding(&c_dst) {
            t.skip_lcms = true;
        }
        if !make_usable_as_destination(&mut t.profile_dst) {
            return ptr::null_mut();
        }
        let channels_src = if c_src.cmyk { 4 } else { c_src.channels() };
        let channels_dst = c_dst.channels();
        t.channels_src = channels_src;
        t.channels_dst = channels_dst;
        let actual_src = if channels_src == 4 { 4 } else { 3 };
        allocate_buffer(
            width.wrapping_mul(actual_src),
            threads,
            &mut t.src_storage,
            &mut t.buf_src,
        );
        allocate_buffer(
            width.wrapping_mul(3),
            threads,
            &mut t.dst_storage,
            &mut t.buf_dst,
        );
        t.intensity_target = intensity;
        Box::into_raw(t).cast()
    }
}

#[repr(transparent)]
struct StaticInterface(JxlCmsInterface);
// SAFETY: the static interface has immutable callback fields; its only non-null
// data pointer identifies this same immutable interface. Contexts are separate
// owned allocations and this does not promise Sync for any mutable context.
unsafe impl Sync for StaticInterface {}
static DEFAULT_INTERFACE: StaticInterface = StaticInterface(JxlCmsInterface {
    set_fields_data: ptr::null_mut(),
    set_fields_from_icc: Some(set_fields_from_icc),
    init_data: ptr::addr_of!(DEFAULT_INTERFACE).cast_mut().cast(),
    init: Some(init),
    get_src_buf: Some(get_src_buf),
    get_dst_buf: Some(get_dst_buf),
    run: Some(run),
    destroy: Some(destroy),
});

/// The stable default CMS interface; init_data points to this same interface.
pub fn get_default_cms() -> &'static JxlCmsInterface {
    &DEFAULT_INTERFACE.0
}

/// Data-only access to the original private matrix boundary for its oracle.
///
/// # Safety
/// All matrix/input/output words reached in source order are valid, including
/// the original supported input/output alias case.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_matrix_product(
    matrix: *const crate::skcms::types::Matrix3x3,
    input: *const f32,
    output: *mut f32,
) {
    // SAFETY: identical raw source helper contract, without another algorithm.
    unsafe {
        super::jxl_cms_profile::matrix_product(matrix, input, output);
    }
}

/// # Safety
/// Profile and output satisfy the reached original white-point helper contract.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_unadapted_white_point(
    profile: *const IccProfile,
    output: *mut super::color_encoding_cms::CieXy,
) -> bool {
    // SAFETY: direct forwarding of the caller's original raw helper contract.
    unsafe { super::jxl_cms_profile::unadapted_white_point(profile, output).is_ok() }
}
/// # Safety
/// Profile backing and initialized context satisfy source primary identification.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_identify_primaries(
    profile: *const IccProfile,
    white: super::color_encoding_cms::CieXy,
    encoding: &mut ColorEncoding,
) -> bool {
    // SAFETY: direct forwarding without an alternative implementation.
    unsafe { super::jxl_cms_profile::identify_primaries(profile, white, encoding).is_ok() }
}
/// # Safety
/// Profile backing and initialized context satisfy source transfer identification.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_detect_transfer_function(
    profile: *const IccProfile,
    encoding: &mut ColorEncoding,
) -> bool {
    // SAFETY: direct forwarding without an alternative implementation.
    unsafe { super::jxl_cms_profile::detect_transfer_function(profile, encoding).is_ok() }
}

#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub fn oracle_cie_xy_from_xyz(xyz: [f32; 3]) -> super::color_encoding_cms::CieXy {
    super::jxl_cms_profile::cie_xy_from_xyz(xyz)
}
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub fn oracle_acceptable_chad(chad: &crate::skcms::types::Matrix3x3) -> bool {
    super::jxl_cms_profile::is_acceptable_chad(chad)
}
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub fn oracle_known_transfer_function(tf: u32) -> bool {
    super::jxl_cms_profile::is_known_transfer_function(TransferFunction(tf))
}
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub fn oracle_known_color_primaries(primaries: u8) -> bool {
    super::jxl_cms_profile::is_known_color_primaries(primaries)
}
/// # Safety
/// Only the profile's data-color-space word is reached and must be initialized.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_color_space(profile: *const IccProfile) -> u32 {
    // SAFETY: identical initialized-prefix contract, without a whole profile load.
    unsafe { super::jxl_cms_profile::color_space_from_profile(profile).0 }
}
/// # Safety
/// Output addresses three writable words if the original prerequisites succeed.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_primaries_luminances(encoding: &ColorEncoding, output: *mut f32) -> bool {
    // SAFETY: forwards the reached output contract without another algorithm.
    unsafe { get_primaries_luminances(encoding, output) }
}
/// # Safety
/// Profile backing satisfies the original profile-comparison contract.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_approximately_equal(
    profile: *const IccProfile,
    encoding: &ColorEncoding,
) -> bool {
    // SAFETY: direct forwarding to the actual profile reconstruction/comparison.
    unsafe { super::jxl_cms_profile::is_approximately_equal(profile, encoding) }
}
/// # Safety
/// Every wrapped allocation/stride covers its returned thread views.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_allocate_buffer(
    length: usize,
    threads: usize,
    storage: &mut Vec<UnsafeCell<f32>>,
    views: &mut Vec<*mut f32>,
) {
    // SAFETY: the caller retains the original unsigned-size/view preconditions.
    unsafe {
        allocate_buffer(length, threads, storage, views);
    }
}

/// Data-only initialized-field fixture for the original private TF context.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub struct OracleTfContext {
    pub code: i32,
    pub mask: u32,
    pub intensity: f32,
    pub apply: bool,
    pub channels: usize,
    pub luminances: [f32; 3],
}

/// # Safety
/// The mask initializes every field reached by the selected original operation;
/// source sample bounds/alias/restrict contracts hold. Operations are before=0,
/// after=1, OOTF=2. Other context fields genuinely remain unknown.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_tf(
    operation: u32,
    state: &OracleTfContext,
    input: *const f32,
    output: *mut f32,
    size: usize,
    forward: bool,
) -> bool {
    let mut context = MaybeUninit::<JxlCms>::uninit();
    let p = context.as_mut_ptr();
    // SAFETY: raw writes initialize only the declared fixture fields. The
    // MaybeUninit owner neither creates a whole context reference nor drops
    // unknown vectors. Dispatch inherits the original reached-field contract.
    unsafe {
        if operation == 0 {
            ptr::addr_of_mut!((*p).preprocess).write(ExtraTf(state.code));
        }
        if operation == 1 {
            ptr::addr_of_mut!((*p).postprocess).write(ExtraTf(state.code));
        }
        if state.mask & 1 != 0 {
            ptr::addr_of_mut!((*p).intensity_target).write(state.intensity);
        }
        if state.mask & 2 != 0 {
            ptr::addr_of_mut!((*p).apply_hlg_ootf).write(state.apply);
        }
        if state.mask & 4 != 0 {
            ptr::addr_of_mut!((*p).hlg_ootf_num_channels).write(state.channels);
        }
        if state.mask & 8 != 0 {
            ptr::addr_of_mut!((*p).hlg_ootf_luminances).write(state.luminances);
        }
        match operation {
            0 => before_transform(p, input, output, size),
            1 => after_transform(p, output, size),
            2 => apply_hlg_ootf(p, output, size, forward),
            _ => unreachable!("observer operation"),
        }
    }
}

/// # Safety
/// Failed guards perform no output access; successful guards require only the
/// original selected writable scalar fields, with other fields left untouched.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_apply_cicp(
    primaries: u8,
    transfer: u8,
    matrix: u8,
    range: u8,
    encoding: *mut ColorEncoding,
) -> bool {
    // SAFETY: direct forwarding of the original raw output contract.
    unsafe { super::jxl_cms_profile::apply_cicp(primaries, transfer, matrix, range, encoding) }
}

/// # Safety
/// Fixtures initialize only the original predicate's reached profile/media
/// fields. Output storage is required only on the original successful writes.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_profile_predicate(
    which: u32,
    profile: *const IccProfile,
    signature: *const u8,
    media: *const f32,
    adapted: *mut f32,
) -> bool {
    use super::jxl_cms_profile as p;
    // SAFETY: data-only dispatch forwards the exact raw source helper contracts.
    unsafe {
        match which {
            0 => p::has_icc_version4_display_class(profile),
            1 => p::has_tag(profile, signature),
            2 => p::has_lut_tags(profile),
            3 => p::has_parametric_trcs(profile),
            4 => p::get_nonconforming_matrix_profile_white_point(profile, media, adapted),
            5 => p::recover_matrix_profile_white_point(profile, media, adapted),
            6 => p::is_approximately_d50(media),
            _ => unreachable!("observer predicate"),
        }
    }
}
