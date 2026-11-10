// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! `JxlToJpegDecoder` from `lib/jxl/decode_to_jpeg.{h,cc}` at the pinned
//! source revision, in its JPEGXL_ENABLE_TRANSCODE_JPEG configuration.
//!
//! Language-required deviations:
//! - The `next_in`/`avail_in` pointer and count pair of `Process` is one
//!   borrowed slice cursor. The source always advances both by the same
//!   amount, so the remaining slice carries exactly the same two values.
//! - Static `SetExif`/`SetXmp` data pointer and size pairs are borrowed
//!   slices. Rust slices are never null, which removes no defined source call.
//! - The caller-owned output buffer stays a raw pointer and count, as in the
//!   source. Storing and releasing it is safe; only `write_output` writes
//!   through it and carries the caller's buffer contract.
//! - `GetJpegData` returns an optional mutable borrow instead of a nullable
//!   non-owning pointer. `SetImageBundleJpegData` dereferences the bundle's
//!   metadata pointer, which the source requires to be non-null.
//! - Source size_t arithmetic wraps explicitly. Undefined source calls, such
//!   as `SetExif` with an Exif marker shorter than its nine header bytes,
//!   stop at a Rust bounds check rather than receiving a new status code.

use super::base::status::Status;
use super::decode::JxlDecoderStatus;
use super::image_bundle::ImageBundle;
use super::jpeg::dec_jpeg_data::decode_jpeg_data;
use super::jpeg::dec_jpeg_data_writer::write_jpeg;
use super::jpeg::jpeg_data::{AppMarkerType, EXIF_TAG, JpegData, XMP_TAG};
use super::jpeg::jpeg_data_fields::set_jpeg_data_from_icc;

pub struct JxlToJpegDecoder {
    // Content of the most recently parsed JPEG reconstruction box if any.
    buffer: Vec<u8>,
    // Decoded content of the most recently parsed JPEG reconstruction box.
    jpeg_data: Option<Box<JpegData>>,
    // True while reading bytes inside a JPEG reconstruction box.
    inside_box: bool,
    // True if the JPEG reconstruction box had undefined size.
    box_until_eof: bool,
    // Size of the most recently parsed JPEG reconstruction box contents.
    box_size: usize,
    // Next bytes to write JPEG reconstruction to (caller-owned).
    next_out: *mut u8,
    // Available bytes to write JPEG reconstruction to.
    avail_size: usize,
}

impl Default for JxlToJpegDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl JxlToJpegDecoder {
    /// The source constructor calls Reset, which assigns every field.
    pub fn new() -> Self {
        let mut decoder = Self {
            buffer: Vec::new(),
            jpeg_data: None,
            inside_box: false,
            box_until_eof: false,
            box_size: 0,
            next_out: std::ptr::null_mut(),
            avail_size: 0,
        };
        decoder.reset();
        decoder
    }

    /// Returns whether an output buffer is set.
    pub fn is_output_set(&self) -> bool {
        !self.next_out.is_null()
    }

    /// Returns whether the decoder is parsing a JPEG reconstruction box.
    pub fn is_parsing_box(&self) -> bool {
        self.inside_box
    }

    /// Sets the output buffer used when producing JPEG output. Storing the
    /// pointer has no requirement; `write_output` documents its use.
    pub fn set_output_buffer(&mut self, data: *mut u8, size: usize) -> JxlDecoderStatus {
        if !self.next_out.is_null() {
            return JxlDecoderStatus::ERROR;
        }
        self.next_out = data;
        self.avail_size = size;
        JxlDecoderStatus::SUCCESS
    }

    /// Releases the buffer set with `set_output_buffer`.
    pub fn release_output_buffer(&mut self) -> usize {
        let result = self.avail_size;
        self.next_out = std::ptr::null_mut();
        self.avail_size = 0;
        result
    }

    /// Sets all per-decode state to its initial value, forgetting the
    /// caller-owned output buffer.
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.jpeg_data = None;
        self.inside_box = false;
        self.box_until_eof = false;
        self.box_size = 0;
        self.next_out = std::ptr::null_mut();
        self.avail_size = 0;
    }

    /// A new box clears the buffer. The source never clears box_until_eof
    /// here, and an unbounded box retains the previous box_size.
    pub fn start_box(&mut self, box_until_eof: bool, contents_size: usize) {
        self.buffer.clear();
        self.inside_box = true;
        if box_until_eof {
            self.box_until_eof = true;
        } else {
            self.box_size = contents_size;
        }
    }

    /// Consumes data from the input cursor to reconstruct JPEG data. Returns
    /// JPEG_RECONSTRUCTION when finished, NEED_MORE_INPUT if more input is
    /// needed and ERROR on a parsing error.
    pub fn process(&mut self, next_in: &mut &[u8]) -> JxlDecoderStatus {
        if !self.inside_box {
            // Source JXL_WARNING diagnostics are not translated.
            return JxlDecoderStatus::ERROR;
        }
        let input: &[u8] = next_in;
        let consumed = if self.box_until_eof {
            // Until EOF means consume all data.
            input.len()
        } else {
            // Defined size means consume min(available, needed).
            input
                .len()
                .min(self.box_size.wrapping_sub(self.buffer.len()))
        };
        let incoming = &input[..consumed];
        *next_in = &input[consumed..];
        let old_data_exists = !self.buffer.is_empty();
        if old_data_exists {
            // Append incoming data to buffer if we already had data in it.
            self.buffer.extend_from_slice(incoming);
        }
        let to_decode: &[u8] = if old_data_exists {
            &self.buffer
        } else {
            incoming
        };
        if !self.box_until_eof && to_decode.len() > self.box_size {
            return JxlDecoderStatus::ERROR;
        }
        if self.box_until_eof || to_decode.len() == self.box_size {
            // If undefined size, or the right size, try to decode.
            let jpeg_data = self.jpeg_data.insert(Box::default());
            let status = decode_jpeg_data(to_decode, jpeg_data);
            if let Err(code) = status {
                if code.is_fatal_error() {
                    return JxlDecoderStatus::ERROR;
                }
            } else {
                // Successful decoding, emit event after updating state to
                // track that we are no longer parsing reconstruction data.
                self.inside_box = false;
                return JxlDecoderStatus::JPEG_RECONSTRUCTION;
            }
            if self.box_until_eof {
                // Unsuccessful decoding and undefined size, assume incomplete
                // data. Copy the data if we haven't already.
                if !old_data_exists {
                    self.buffer.extend_from_slice(incoming);
                }
            } else {
                // Unsuccessful decoding of correct amount of data.
                return JxlDecoderStatus::ERROR;
            }
        } else {
            // Not enough data, copy the data if we haven't already.
            if !old_data_exists {
                self.buffer.extend_from_slice(incoming);
            }
        }
        JxlDecoderStatus::NEED_MORE_INPUT
    }

    /// Returns the retained JPEGData, only after Process finished and before
    /// it was moved to an image bundle.
    pub fn get_jpeg_data(&mut self) -> Option<&mut JpegData> {
        self.jpeg_data.as_deref_mut()
    }

    /// Returns how many Exif app markers are present in the JPEG data.
    pub fn num_exif_markers(jpeg_data: &JpegData) -> usize {
        let mut num = 0usize;
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::EXIF {
                num += 1;
            }
        }
        num
    }

    /// Returns how many XMP app markers are present in the JPEG data.
    pub fn num_xmp_markers(jpeg_data: &JpegData) -> usize {
        let mut num = 0usize;
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::XMP {
                num += 1;
            }
        }
        num
    }

    /// Returns the Exif box content size from the first Exif app marker.
    /// `size` is unchanged on error.
    pub fn exif_box_content_size(jpeg_data: &JpegData, size: &mut usize) -> JxlDecoderStatus {
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::EXIF {
                if jpeg_data.app_data[i].len() < 3 + EXIF_TAG.len() {
                    // too small for app marker header
                    return JxlDecoderStatus::ERROR;
                }
                // The first 4 bytes are the TIFF header from the box contents,
                // and are not included in the JPEG.
                *size = jpeg_data.app_data[i].len() + 4 - 3 - EXIF_TAG.len();
                return JxlDecoderStatus::SUCCESS;
            }
        }
        JxlDecoderStatus::ERROR
    }

    /// Returns the XML box content size from the first XMP app marker.
    /// `size` is unchanged on error.
    pub fn xml_box_content_size(jpeg_data: &JpegData, size: &mut usize) -> JxlDecoderStatus {
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::XMP {
                if jpeg_data.app_data[i].len() < 3 + XMP_TAG.len() {
                    // too small for app marker header
                    return JxlDecoderStatus::ERROR;
                }
                *size = jpeg_data.app_data[i].len() - 3 - XMP_TAG.len();
                return JxlDecoderStatus::SUCCESS;
            }
        }
        JxlDecoderStatus::ERROR
    }

    /// Fills the first Exif marker from the complete Exif box contents,
    /// including its 4-byte TIFF header offset, which is not copied.
    ///
    /// The source size check wraps; a matching marker shorter than nine bytes
    /// (data shorter than four bytes) overflows both source copies. That call
    /// is undefined in the source and panics at a Rust bounds check here.
    pub fn set_exif(data: &[u8], jpeg_data: &mut JpegData) -> JxlDecoderStatus {
        let size = data.len();
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::EXIF {
                let marker = &mut jpeg_data.app_data[i];
                if marker.len() != size.wrapping_add(3 + EXIF_TAG.len()).wrapping_sub(4) {
                    return JxlDecoderStatus::ERROR;
                }
                // The first 9 bytes are used for the JPEG marker header.
                marker[0] = 0xE1;
                // The second and third byte are already filled in correctly.
                marker[3..3 + EXIF_TAG.len()].copy_from_slice(EXIF_TAG);
                // The first 4 bytes are the TIFF header from the box contents,
                // and are not included in the JPEG.
                marker[3 + EXIF_TAG.len()..].copy_from_slice(&data[4..]);
                return JxlDecoderStatus::SUCCESS;
            }
        }
        JxlDecoderStatus::ERROR
    }

    /// Fills the first XMP marker from the complete XML box contents.
    pub fn set_xmp(data: &[u8], jpeg_data: &mut JpegData) -> JxlDecoderStatus {
        let size = data.len();
        for i in 0..jpeg_data.app_data.len() {
            if jpeg_data.app_marker_type[i] == AppMarkerType::XMP {
                let marker = &mut jpeg_data.app_data[i];
                if marker.len() != size.wrapping_add(3 + XMP_TAG.len()) {
                    return JxlDecoderStatus::ERROR;
                }
                // The first 9 bytes are used for the JPEG marker header.
                marker[0] = 0xE1;
                // The second and third byte are already filled in correctly.
                marker[3..3 + XMP_TAG.len()].copy_from_slice(XMP_TAG);
                marker[3 + XMP_TAG.len()..].copy_from_slice(data);
                return JxlDecoderStatus::SUCCESS;
            }
        }
        JxlDecoderStatus::ERROR
    }

    /// Sets the JpegData of the bundle if an output buffer is set and JPEG
    /// data was decoded, releasing it from this decoder. A failed ICC
    /// injection retains the partial marker writes and the decoder's owner.
    pub fn set_image_bundle_jpeg_data(&mut self, ib: &mut ImageBundle<'_>) -> Status {
        if self.is_output_set()
            && let Some(jpeg_data) = self.jpeg_data.as_deref_mut()
        {
            let metadata = ib
                .metadata()
                .expect("source dereferences the image bundle metadata");
            set_jpeg_data_from_icc(metadata.color_encoding.icc(), jpeg_data)?;
            ib.jpeg_data = self.jpeg_data.take();
        }
        Ok(())
    }

    /// Writes the complete JPEG bytestream into the output buffer. On error,
    /// the bytes written so far remain in the buffer, but the stored output
    /// position and size are unchanged; a full buffer reports
    /// JPEG_NEED_MORE_OUTPUT.
    ///
    /// # Safety
    /// When `avail_size` is nonzero, the stored output pointer must be valid
    /// for writes of `avail_size` bytes and not overlap `jpeg_data` or any
    /// storage it borrows. No other access to that buffer may occur during
    /// the call. A null pointer is permitted only with a zero size.
    pub unsafe fn write_output(&mut self, jpeg_data: &JpegData) -> JxlDecoderStatus {
        // Copy JPEG bytestream if desired.
        let mut tmp_next_out = self.next_out;
        let mut tmp_avail_size = self.avail_size;
        let write_result = write_jpeg(jpeg_data, |buf| {
            let buf = buf.unwrap_or(&[]);
            let to_write = tmp_avail_size.min(buf.len());
            if to_write != 0 {
                // SAFETY: tmp_next_out starts at the caller's buffer and only
                // advances by bytes counted against tmp_avail_size, so the
                // caller contract covers these to_write bytes. The writer's
                // borrowed or owned chunk cannot overlap that buffer.
                unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), tmp_next_out, to_write) };
            }
            // Pointer arithmetic stays within the buffer, or is zero for null.
            tmp_next_out = tmp_next_out.wrapping_add(to_write);
            tmp_avail_size -= to_write;
            to_write
        });
        if write_result.is_err() {
            if tmp_avail_size == 0 {
                return JxlDecoderStatus::JPEG_NEED_MORE_OUTPUT;
            }
            return JxlDecoderStatus::ERROR;
        }
        self.next_out = tmp_next_out;
        self.avail_size = tmp_avail_size;
        JxlDecoderStatus::SUCCESS
    }
}
