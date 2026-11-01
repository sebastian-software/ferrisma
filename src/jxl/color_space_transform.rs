// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete callback-backed ColorSpaceTransform from color_encoding_internal.h.
//! Callback/context pointers are non-owning except for the original destroy
//! obligation on the current context. Cached ICC byte owners remain alive until
//! after destroy, including when this native owner moves. No CMS is substituted.
use super::base::status::{Status, StatusCode};
use super::cms_interface::{IccProfileSpan, JxlCmsInterface, JxlColorProfile};
use super::color_encoding_internal::ColorEncoding;
use std::ffi::c_void;

pub struct ColorSpaceTransform {
    cms: JxlCmsInterface,
    cms_data: *mut c_void,
    icc_src: Vec<u8>,
    icc_dst: Vec<u8>,
}
fn span(bytes: &Vec<u8>) -> IccProfileSpan {
    // The reference libc++ vector has a null data pointer before allocation;
    // an emptied vector retains its allocation. Pointer addresses are not ABI
    // parity, but preserve this observable null/non-null distinction for CMS.
    IccProfileSpan {
        data: if bytes.capacity() == 0 {
            std::ptr::null()
        } else {
            bytes.as_ptr()
        },
        size: bytes.len(),
    }
}
impl ColorSpaceTransform {
    pub fn new(cms: &JxlCmsInterface) -> Self {
        Self {
            cms: *cms,
            cms_data: std::ptr::null_mut(),
            icc_src: Vec::new(),
            icc_dst: Vec::new(),
        }
    }
    /// Initialize the original callback-backed transform.
    ///
    /// # Safety
    /// Invoked callbacks/opaque data must remain valid through this owner's
    /// destruction and obey cms_interface.h, including non-unwinding calls and
    /// valid context/buffer lifetimes. Callbacks must not reenter this owner.
    /// Reinitialization must not leave a prior context accessing replaced ICC
    /// storage: the original method overwrites its context without destroying
    /// the previous one. Profile structs are valid during init only; retained
    /// ICC pointers refer to this owner's read-only caches through the current
    /// cycle and must not be written by callbacks.
    pub unsafe fn init(
        &mut self,
        src: &ColorEncoding,
        dst: &ColorEncoding,
        intensity: f32,
        width: usize,
        threads: usize,
    ) -> Status {
        self.icc_src.clear();
        self.icc_src.extend_from_slice(src.icc());
        let input = JxlColorProfile {
            icc: span(&self.icc_src),
            color_encoding: src.to_external(),
            num_channels: if src.is_cmyk() { 4 } else { src.channels() },
        };
        self.icc_dst.clear();
        self.icc_dst.extend_from_slice(dst.icc());
        let external = dst.to_external();
        if dst.is_cmyk() {
            return Err(StatusCode::GenericError);
        }
        let output = JxlColorProfile {
            icc: span(&self.icc_dst),
            color_encoding: external,
            num_channels: dst.channels(),
        };
        // SAFETY: Both stack profile structs are initialized and live for the
        // call. Their ICC pointers refer to owned, stable allocations; the
        // unsafe caller supplies the foreign-function/opaque-data contract.
        self.cms_data = unsafe {
            self.cms.init.expect("source requires CMS init")(
                self.cms.init_data,
                threads,
                width,
                &input,
                &output,
                intensity,
            )
        };
        if self.cms_data.is_null() {
            Err(StatusCode::GenericError)
        } else {
            Ok(())
        }
    }
    /// # Safety
    /// The CMS context and callback must be valid, and thread must satisfy the
    /// successful initialization's thread bound and callback contract.
    pub unsafe fn buf_src(&self, thread: usize) -> *mut f32 {
        // SAFETY: The caller supplies valid context/thread/callback lifetimes.
        unsafe {
            self.cms
                .get_src_buf
                .expect("source requires a source-buffer callback")(
                self.cms_data, thread
            )
        }
    }
    /// # Safety
    /// The same context/thread/callback obligations as buf_src apply.
    pub unsafe fn buf_dst(&self, thread: usize) -> *mut f32 {
        // SAFETY: The caller supplies valid context/thread/callback lifetimes.
        unsafe {
            self.cms
                .get_dst_buf
                .expect("source requires a destination-buffer callback")(
                self.cms_data, thread
            )
        }
    }
    /// # Safety
    /// Context/thread must be initialized and valid. Input/output must cover
    /// width times their respective channel counts, with initialized input and
    /// the overlap rules in cms_interface.h. The callback must initialize the
    /// successful output and must not unwind or reenter this owner.
    pub unsafe fn run(
        &mut self,
        thread: usize,
        input: *const f32,
        output: *mut f32,
        width: usize,
    ) -> Status {
        // SAFETY: The caller supplies the initialized buffer ranges, overlap,
        // context/thread, and foreign-call guarantees stated above.
        let success = unsafe {
            self.cms.run.expect("source requires a run callback")(
                self.cms_data,
                thread,
                input,
                output,
                width,
            )
        };
        if success != 0 {
            Ok(())
        } else {
            Err(StatusCode::GenericError)
        }
    }
    pub fn has_context(&self) -> bool {
        !self.cms_data.is_null()
    }
    pub fn icc_src(&self) -> &[u8] {
        &self.icc_src
    }
    pub fn icc_dst(&self) -> &[u8] {
        &self.icc_dst
    }
}
impl Drop for ColorSpaceTransform {
    fn drop(&mut self) {
        if !self.cms_data.is_null() {
            // SAFETY: A non-null context can only be installed by unsafe init,
            // whose contract keeps destroy/context valid until this drop.
            // This owner is not Copy/Clone; a move transfers the obligation.
            unsafe { self.cms.destroy.expect("source requires CMS destroy")(self.cms_data) };
        }
        // Source C++ members are destroyed in reverse declaration order.
        drop(std::mem::take(&mut self.icc_dst));
        drop(std::mem::take(&mut self.icc_src));
    }
}
