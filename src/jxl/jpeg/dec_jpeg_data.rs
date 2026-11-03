// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Complete DecodeJPEGData from lib/jxl/jpeg/dec_jpeg_data.cc.
//! The original caller supplies independent encoded and JPEGData owners.
//! Every marker mutation and Brotli cursor survives a later source failure.

use super::jpeg_data::{AppMarkerType, JpegData};
use crate::brotli::compound_dictionary::CompoundDictionaryBindings;
use crate::brotli::instance::DecoderInstance;
use crate::brotli::status::{
    BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT, BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT,
    BROTLI_DECODER_RESULT_SUCCESS,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::dec_bit_reader::BitReader;
use crate::jxl::fields::bundle;
use std::mem::MaybeUninit;

// Source jpeg_data.h byte arrays include the terminating NUL bytes.
const ICC_PROFILE_TAG: &[u8; 12] = b"ICC_PROFILE\0";
const EXIF_TAG: &[u8; 6] = b"Exif\0\0";
const XMP_TAG: &[u8; 29] = b"http://ns.adobe.com/xap/1.0/\0";

fn br_read<'a>(
    decoder: &mut DecoderInstance<'a>,
    bindings: &CompoundDictionaryBindings<'a>,
    available_in: &mut usize,
    input: &mut *const u8,
    data: &mut [u8],
) -> Status {
    let mut available_out = data.len();
    let mut output = data.as_mut_ptr();
    while available_out != 0 {
        if decoder.is_finished() {
            return Err(StatusCode::GenericError);
        }
        // SAFETY: This source caller retains the original input allocation and
        // the current initialized vector extent. Its cursor cells are disjoint
        // locals; reached input/output copies satisfy the original nonoverlap
        // domain. The decoder keeps no input/output reference between calls.
        let result = unsafe {
            decoder.decompress_stream_raw(
                bindings,
                available_in,
                input,
                &mut available_out,
                &mut output,
                std::ptr::null_mut(),
            )
        };
        if result != BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT
            && result != BROTLI_DECODER_RESULT_SUCCESS
        {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}

/// Decode the complete field bundle and compressed marker payload into the
/// existing JPEG owner, retaining partial mutations at every failure.
pub fn decode_jpeg_data(encoded: &[u8], jpeg_data: &mut JpegData) -> Status {
    let mut reader = BitReader::new(encoded);
    let parsed = (|| {
        bundle::read(&mut reader, jpeg_data)?;
        reader.jump_to_byte_boundary()
    })();
    let consumed = reader.total_bits_consumed() / 8;
    let closed = reader.close();
    parsed?;
    closed?;
    let mut available_in = encoded.len().wrapping_sub(consumed);
    // SAFETY: A successful original reader close proves the consumed prefix is
    // within this same live input allocation. Form its one-past pointer too.
    let mut input = unsafe { encoded.as_ptr().add(consumed) };

    // The original default-allocation caller does not check constructor failure
    // before dereferencing the decoder. A successful allocation is its domain.
    let mut decoder = DecoderInstance::new().expect("source requires a live Brotli decoder");
    let bindings = CompoundDictionaryBindings::default();
    let mut num_icc = 0usize;
    for i in 0..jpeg_data.app_data.len() {
        let marker = &mut jpeg_data.app_data[i];
        if marker.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        if jpeg_data.app_marker_type[i] != AppMarkerType::UNKNOWN {
            let size_minus_1 = marker.len() - 1;
            marker[1] = (size_minus_1 >> 8) as u8;
            marker[2] = (size_minus_1 & 0xff) as u8;
            if jpeg_data.app_marker_type[i] == AppMarkerType::ICC {
                if marker.len() < 17 {
                    return Err(StatusCode::GenericError);
                }
                marker[0] = 0xe2;
                marker[3..15].copy_from_slice(ICC_PROFILE_TAG);
                num_icc = num_icc.wrapping_add(1);
                marker[15] = num_icc as u8;
            }
        } else {
            br_read(
                &mut decoder,
                &bindings,
                &mut available_in,
                &mut input,
                marker,
            )?;
            if (u32::from(marker[1]) * 256 + u32::from(marker[2]) + 1) as usize != marker.len() {
                return Err(StatusCode::GenericError);
            }
        }
    }
    for i in 0..jpeg_data.app_data.len() {
        let marker = &mut jpeg_data.app_data[i];
        if jpeg_data.app_marker_type[i] == AppMarkerType::ICC {
            marker[16] = num_icc as u8;
        }
        if jpeg_data.app_marker_type[i] == AppMarkerType::EXIF {
            if marker.len() < 3 + EXIF_TAG.len() {
                return Err(StatusCode::GenericError);
            }
            marker[0] = 0xe1;
            marker[3..3 + EXIF_TAG.len()].copy_from_slice(EXIF_TAG);
        }
        if jpeg_data.app_marker_type[i] == AppMarkerType::XMP {
            if marker.len() < 3 + XMP_TAG.len() {
                return Err(StatusCode::GenericError);
            }
            marker[0] = 0xe1;
            marker[3..3 + XMP_TAG.len()].copy_from_slice(XMP_TAG);
        }
    }
    for marker in &mut jpeg_data.com_data {
        br_read(
            &mut decoder,
            &bindings,
            &mut available_in,
            &mut input,
            marker,
        )?;
        if (u32::from(marker[1]) * 256 + u32::from(marker[2]) + 1) as usize != marker.len() {
            return Err(StatusCode::GenericError);
        }
    }
    for data in &mut jpeg_data.inter_marker_data {
        br_read(&mut decoder, &bindings, &mut available_in, &mut input, data)?;
    }
    br_read(
        &mut decoder,
        &bindings,
        &mut available_in,
        &mut input,
        &mut jpeg_data.tail_data,
    )?;

    let mut available_out = 1usize;
    let mut sink = MaybeUninit::<u64>::uninit();
    let mut output = sink.as_mut_ptr().cast::<u8>();
    // SAFETY: The original final probe exposes one writable byte of its local
    // eight-byte sink. No unknown sink byte is ever read. Input/cursor lifetimes
    // remain the same as above, and all scalar cells are distinct locals.
    let result = unsafe {
        decoder.decompress_stream_raw(
            &bindings,
            &mut available_in,
            &mut input,
            &mut available_out,
            &mut output,
            std::ptr::null_mut(),
        )
    };
    if available_out == 0 || result == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT {
        return Err(StatusCode::GenericError);
    }
    if result == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT {
        return Err(StatusCode::GenericError);
    }
    if !decoder.is_finished() || result != BROTLI_DECODER_RESULT_SUCCESS {
        return Err(StatusCode::GenericError);
    }
    if available_in != 0 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
