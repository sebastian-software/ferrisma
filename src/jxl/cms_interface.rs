// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Descriptor storage and callback signatures from lib/include/jxl/
//! cms_interface.h. These are non-owning C data carriers: storing/copying them
//! never invokes a callback or transfers ownership. Callback execution and the
//! color-transform lifecycle remain a separate, unported boundary. Any future
//! invocation must uphold the header's buffer, lifetime, and thread contracts.

use super::color_encoding::JxlColorEncoding;
use std::ffi::{c_int, c_void};

pub type CmsSetFieldsFromIcc =
    unsafe extern "C" fn(*mut c_void, *const u8, usize, *mut JxlColorEncoding, *mut c_int) -> c_int;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct IccProfileSpan {
    pub data: *const u8,
    pub size: usize,
}
#[derive(Clone, Debug)]
#[repr(C)]
pub struct JxlColorProfile {
    pub icc: IccProfileSpan,
    pub color_encoding: JxlColorEncoding,
    pub num_channels: usize,
}
pub type CmsInit = unsafe extern "C" fn(
    *mut c_void,
    usize,
    usize,
    *const JxlColorProfile,
    *const JxlColorProfile,
    f32,
) -> *mut c_void;
pub type CmsGetBuffer = unsafe extern "C" fn(*mut c_void, usize) -> *mut f32;
pub type CmsRun = unsafe extern "C" fn(*mut c_void, usize, *const f32, *mut f32, usize) -> c_int;
pub type CmsDestroy = unsafe extern "C" fn(*mut c_void);
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct JxlCmsInterface {
    pub set_fields_data: *mut c_void,
    pub set_fields_from_icc: Option<CmsSetFieldsFromIcc>,
    pub init_data: *mut c_void,
    pub init: Option<CmsInit>,
    pub get_src_buf: Option<CmsGetBuffer>,
    pub get_dst_buf: Option<CmsGetBuffer>,
    pub run: Option<CmsRun>,
    pub destroy: Option<CmsDestroy>,
}
impl Default for JxlCmsInterface {
    fn default() -> Self {
        Self {
            set_fields_data: std::ptr::null_mut(),
            set_fields_from_icc: None,
            init_data: std::ptr::null_mut(),
            init: None,
            get_src_buf: None,
            get_dst_buf: None,
            run: None,
            destroy: None,
        }
    }
}
