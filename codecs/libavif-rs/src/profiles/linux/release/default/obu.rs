/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// OBU parsing and bit magic all originally from dav1d's obu.c and getbits.c,
// but heavily modified/reduced down to simply find the Sequence Header OBU
// and pull a few interesting pieces from it.
//
// Any other code in here is under this license:
//
// Copyright 2020 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/obu.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type avifBool = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifROData {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type avifPixelFormat = core::ffi::c_uint;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub type avifChromaSamplePosition = core::ffi::c_uint;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub type avifRange = core::ffi::c_uint;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_COLOR_PRIMARIES_EBU3213: C2RustUnnamed = 22;
pub const AVIF_COLOR_PRIMARIES_DCI_P3: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: C2RustUnnamed = 11;
pub const AVIF_COLOR_PRIMARIES_XYZ: C2RustUnnamed = 10;
pub const AVIF_COLOR_PRIMARIES_BT2100: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_BT2020: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: C2RustUnnamed = 8;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_COLOR_PRIMARIES_BT601: C2RustUnnamed = 6;
pub const AVIF_COLOR_PRIMARIES_BT470BG: C2RustUnnamed = 5;
pub const AVIF_COLOR_PRIMARIES_BT470M: C2RustUnnamed = 4;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_SRGB: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_BT709: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: C2RustUnnamed = 0;
pub type avifColorPrimaries = uint16_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: C2RustUnnamed_0 = 18;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: C2RustUnnamed_0 = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_PQ: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: C2RustUnnamed_0 = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: C2RustUnnamed_0 = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: C2RustUnnamed_0 = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: C2RustUnnamed_0 = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: C2RustUnnamed_0 = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: C2RustUnnamed_0 = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: C2RustUnnamed_0 = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: C2RustUnnamed_0 = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: C2RustUnnamed_0 = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: C2RustUnnamed_0 = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: C2RustUnnamed_0 = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: C2RustUnnamed_0 = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: C2RustUnnamed_0 = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: C2RustUnnamed_0 = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: C2RustUnnamed_0 = 0;
pub type avifTransferCharacteristics = uint16_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: C2RustUnnamed_1 = 18;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: C2RustUnnamed_1 = 17;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: C2RustUnnamed_1 = 16;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: C2RustUnnamed_1 = 14;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: C2RustUnnamed_1 = 13;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: C2RustUnnamed_1 = 12;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: C2RustUnnamed_1 = 11;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: C2RustUnnamed_1 = 10;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: C2RustUnnamed_1 = 9;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: C2RustUnnamed_1 = 8;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: C2RustUnnamed_1 = 7;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: C2RustUnnamed_1 = 6;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: C2RustUnnamed_1 = 5;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: C2RustUnnamed_1 = 4;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: C2RustUnnamed_1 = 2;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: C2RustUnnamed_1 = 1;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: C2RustUnnamed_1 = 0;
pub type avifMatrixCoefficients = uint16_t;
pub type avifCodecType = core::ffi::c_uint;
pub const AVIF_CODEC_TYPE_AV1: avifCodecType = 1;
pub const AVIF_CODEC_TYPE_UNKNOWN: avifCodecType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecConfigurationBox {
    pub seqProfile: uint8_t,
    pub seqLevelIdx0: uint8_t,
    pub seqTier0: uint8_t,
    pub highBitdepth: uint8_t,
    pub twelveBit: uint8_t,
    pub monochrome: uint8_t,
    pub chromaSubsamplingX: uint8_t,
    pub chromaSubsamplingY: uint8_t,
    pub chromaSamplePosition: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSequenceHeader {
    pub reduced_still_picture_header: uint8_t,
    pub maxWidth: uint32_t,
    pub maxHeight: uint32_t,
    pub bitDepth: uint32_t,
    pub yuvFormat: avifPixelFormat,
    pub chromaSamplePosition: avifChromaSamplePosition,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub range: avifRange,
    pub av1C: avifCodecConfigurationBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifBits {
    pub error: core::ffi::c_int,
    pub eof: core::ffi::c_int,
    pub state: uint64_t,
    pub bitsLeft: uint32_t,
    pub ptr: *const uint8_t,
    pub start: *const uint8_t,
    pub end: *const uint8_t,
}
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn avifBitsReadPos(mut bits: *const avifBits) -> uint32_t {
    return (((*bits).ptr).offset_from((*bits).start) as core::ffi::c_long as uint32_t)
        .wrapping_mul(8 as uint32_t)
        .wrapping_sub((*bits).bitsLeft);
}
unsafe extern "C" fn avifBitsInit(
    bits: *mut avifBits,
    data: *const uint8_t,
    size: size_t,
) {
    (*bits).start = data;
    (*bits).ptr = (*bits).start;
    (*bits).end = &*((*bits).start).offset(size as isize) as *const uint8_t;
    (*bits).bitsLeft = 0 as uint32_t;
    (*bits).state = 0 as uint64_t;
    (*bits).error = 0 as core::ffi::c_int;
    (*bits).eof = (size == 0 as size_t) as core::ffi::c_int;
}
unsafe extern "C" fn avifBitsRefill(bits: *mut avifBits, n: uint32_t) {
    let mut state: uint64_t = 0 as uint64_t;
    loop {
        state <<= 8 as core::ffi::c_int;
        (*bits).bitsLeft = ((*bits).bitsLeft).wrapping_add(8 as uint32_t);
        if (*bits).eof == 0 {
            let fresh0 = (*bits).ptr;
            (*bits).ptr = ((*bits).ptr).offset(1);
            state |= *fresh0 as uint64_t;
        }
        if (*bits).ptr >= (*bits).end {
            (*bits).error = (*bits).eof;
            (*bits).eof = 1 as core::ffi::c_int;
        }
        if !(n > (*bits).bitsLeft) {
            break;
        }
    }
    (*bits).state |= state << (64 as uint32_t).wrapping_sub((*bits).bitsLeft);
}
unsafe extern "C" fn avifBitsRead(bits: *mut avifBits, n: uint32_t) -> uint32_t {
    if n > (*bits).bitsLeft {
        avifBitsRefill(bits, n);
    }
    let state: uint64_t = (*bits).state;
    (*bits).bitsLeft = ((*bits).bitsLeft).wrapping_sub(n);
    (*bits).state <<= n;
    return (state >> (64 as uint32_t).wrapping_sub(n)) as uint32_t;
}
unsafe extern "C" fn avifBitsReadUleb128(mut bits: *mut avifBits) -> uint32_t {
    let mut val: uint64_t = 0 as uint64_t;
    let mut more: uint32_t = 0;
    let mut i: uint32_t = 0 as uint32_t;
    loop {
        let v: uint32_t = avifBitsRead(bits, 8 as uint32_t) as uint32_t;
        more = v & 0x80 as uint32_t;
        val |= ((v & 0x7f as uint32_t) as uint64_t) << i;
        i = i.wrapping_add(7 as uint32_t);
        if !(more != 0 && i < 56 as uint32_t) {
            break;
        }
    }
    if val > UINT32_MAX as uint64_t || more != 0 {
        (*bits).error = 1 as core::ffi::c_int;
        return 0 as uint32_t;
    }
    return val as uint32_t;
}
unsafe extern "C" fn avifBitsReadVLC(bits: *mut avifBits) -> uint32_t {
    let mut numBits: core::ffi::c_int = 0 as core::ffi::c_int;
    while avifBitsRead(bits, 1 as uint32_t) == 0 {
        numBits += 1;
        if numBits == 32 as core::ffi::c_int {
            return 0xffffffff as uint32_t;
        }
    }
    return if numBits != 0 {
        ((1 as uint32_t) << numBits)
            .wrapping_sub(1 as uint32_t)
            .wrapping_add(avifBitsRead(bits, numBits as uint32_t))
    } else {
        0 as uint32_t
    };
}
unsafe extern "C" fn parseAV1SequenceHeaderProfile(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    let mut seq_profile: uint32_t = avifBitsRead(bits, 3 as uint32_t);
    if seq_profile > 2 as uint32_t {
        return AVIF_FALSE;
    }
    (*header).av1C.seqProfile = seq_profile as uint8_t;
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn parseSequenceHeaderLevelIdxAndTier(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    let mut still_picture: uint32_t = avifBitsRead(bits, 1 as uint32_t);
    (*header).reduced_still_picture_header = avifBitsRead(bits, 1 as uint32_t)
        as uint8_t;
    if (*header).reduced_still_picture_header as core::ffi::c_int != 0
        && still_picture == 0
    {
        return AVIF_FALSE;
    }
    if (*header).reduced_still_picture_header != 0 {
        (*header).av1C.seqLevelIdx0 = avifBitsRead(bits, 5 as uint32_t) as uint8_t;
        (*header).av1C.seqTier0 = 0 as uint8_t;
    } else {
        let mut timing_info_present_flag: uint32_t = avifBitsRead(bits, 1 as uint32_t);
        let mut decoder_model_info_present_flag: uint32_t = 0 as uint32_t;
        let mut buffer_delay_length: uint32_t = 0 as uint32_t;
        if timing_info_present_flag != 0 {
            avifBitsRead(bits, 32 as uint32_t);
            avifBitsRead(bits, 32 as uint32_t);
            let mut equal_picture_interval: uint32_t = avifBitsRead(bits, 1 as uint32_t);
            if equal_picture_interval != 0 {
                let mut num_ticks_per_picture_minus_1: uint32_t = avifBitsReadVLC(bits);
                if num_ticks_per_picture_minus_1 == 0xffffffff as uint32_t {
                    return AVIF_FALSE;
                }
            }
            decoder_model_info_present_flag = avifBitsRead(bits, 1 as uint32_t);
            if decoder_model_info_present_flag != 0 {
                buffer_delay_length = (avifBitsRead(bits, 5 as uint32_t))
                    .wrapping_add(1 as uint32_t);
                avifBitsRead(bits, 32 as uint32_t);
                avifBitsRead(bits, 10 as uint32_t);
            }
        }
        let mut initial_display_delay_present_flag: uint32_t = avifBitsRead(
            bits,
            1 as uint32_t,
        );
        let mut operating_points_cnt: uint32_t = (avifBitsRead(bits, 5 as uint32_t))
            .wrapping_add(1 as uint32_t);
        let mut i: uint32_t = 0 as uint32_t;
        while i < operating_points_cnt {
            avifBitsRead(bits, 12 as uint32_t);
            let mut seq_level_idx: uint32_t = avifBitsRead(bits, 5 as uint32_t);
            if i == 0 as uint32_t {
                (*header).av1C.seqLevelIdx0 = seq_level_idx as uint8_t;
                (*header).av1C.seqTier0 = 0 as uint8_t;
            }
            if seq_level_idx > 7 as uint32_t {
                let mut seq_tier: uint32_t = avifBitsRead(bits, 1 as uint32_t);
                if i == 0 as uint32_t {
                    (*header).av1C.seqTier0 = seq_tier as uint8_t;
                }
            }
            if decoder_model_info_present_flag != 0 {
                let mut decoder_model_present_for_this_op: uint32_t = avifBitsRead(
                    bits,
                    1 as uint32_t,
                );
                if decoder_model_present_for_this_op != 0 {
                    avifBitsRead(bits, buffer_delay_length);
                    avifBitsRead(bits, buffer_delay_length);
                    avifBitsRead(bits, 1 as uint32_t);
                }
            }
            if initial_display_delay_present_flag != 0 {
                let mut initial_display_delay_present_for_this_op: uint32_t = avifBitsRead(
                    bits,
                    1 as uint32_t,
                );
                if initial_display_delay_present_for_this_op != 0 {
                    avifBitsRead(bits, 4 as uint32_t);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn parseSequenceHeaderFrameMaxDimensions(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    let mut frame_width_bits: uint32_t = (avifBitsRead(bits, 4 as uint32_t))
        .wrapping_add(1 as uint32_t);
    let mut frame_height_bits: uint32_t = (avifBitsRead(bits, 4 as uint32_t))
        .wrapping_add(1 as uint32_t);
    (*header).maxWidth = (avifBitsRead(bits, frame_width_bits))
        .wrapping_add(1 as uint32_t);
    (*header).maxHeight = (avifBitsRead(bits, frame_height_bits))
        .wrapping_add(1 as uint32_t);
    let mut frame_id_numbers_present_flag: uint32_t = 0 as uint32_t;
    if (*header).reduced_still_picture_header == 0 {
        frame_id_numbers_present_flag = avifBitsRead(bits, 1 as uint32_t);
    }
    if frame_id_numbers_present_flag != 0 {
        avifBitsRead(bits, 7 as uint32_t);
    }
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn parseSequenceHeaderEnabledFeatures(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    avifBitsRead(bits, 2 as uint32_t);
    if (*header).reduced_still_picture_header == 0 {
        avifBitsRead(bits, 4 as uint32_t);
        let mut enable_order_hint: uint32_t = avifBitsRead(bits, 1 as uint32_t);
        if enable_order_hint != 0 {
            avifBitsRead(bits, 2 as uint32_t);
        }
        let mut seq_force_screen_content_tools: uint32_t = 0 as uint32_t;
        let mut seq_choose_screen_content_tools: uint32_t = avifBitsRead(
            bits,
            1 as uint32_t,
        );
        if seq_choose_screen_content_tools != 0 {
            seq_force_screen_content_tools = 2 as uint32_t;
        } else {
            seq_force_screen_content_tools = avifBitsRead(bits, 1 as uint32_t);
        }
        if seq_force_screen_content_tools > 0 as uint32_t {
            let mut seq_choose_integer_mv: uint32_t = avifBitsRead(bits, 1 as uint32_t);
            if seq_choose_integer_mv == 0 {
                avifBitsRead(bits, 1 as uint32_t);
            }
        }
        if enable_order_hint != 0 {
            avifBitsRead(bits, 3 as uint32_t);
        }
    }
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn parseAV1SequenceHeaderColorConfig(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    (*header).bitDepth = 8 as uint32_t;
    (*header).chromaSamplePosition = AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN;
    (*header).av1C.chromaSamplePosition = (*header).chromaSamplePosition as uint8_t;
    let mut high_bitdepth: uint32_t = avifBitsRead(bits, 1 as uint32_t);
    (*header).av1C.highBitdepth = high_bitdepth as uint8_t;
    if (*header).av1C.seqProfile as core::ffi::c_int == 2 as core::ffi::c_int
        && high_bitdepth != 0
    {
        let mut twelve_bit: uint32_t = avifBitsRead(bits, 1 as uint32_t);
        (*header).bitDepth = (if twelve_bit != 0 {
            12 as core::ffi::c_int
        } else {
            10 as core::ffi::c_int
        }) as uint32_t;
        (*header).av1C.twelveBit = twelve_bit as uint8_t;
    } else {
        (*header).bitDepth = (if high_bitdepth != 0 {
            10 as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        }) as uint32_t;
        (*header).av1C.twelveBit = 0 as uint8_t;
    }
    let mut mono_chrome: uint32_t = 0 as uint32_t;
    if (*header).av1C.seqProfile as core::ffi::c_int != 1 as core::ffi::c_int {
        mono_chrome = avifBitsRead(bits, 1 as uint32_t);
    }
    (*header).av1C.monochrome = mono_chrome as uint8_t;
    let mut color_description_present_flag: uint32_t = avifBitsRead(bits, 1 as uint32_t);
    if color_description_present_flag != 0 {
        (*header).colorPrimaries = avifBitsRead(bits, 8 as uint32_t)
            as avifColorPrimaries;
        (*header).transferCharacteristics = avifBitsRead(bits, 8 as uint32_t)
            as avifTransferCharacteristics;
        (*header).matrixCoefficients = avifBitsRead(bits, 8 as uint32_t)
            as avifMatrixCoefficients;
    } else {
        (*header).colorPrimaries = AVIF_COLOR_PRIMARIES_UNSPECIFIED as core::ffi::c_int
            as avifColorPrimaries;
        (*header).transferCharacteristics = AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED
            as core::ffi::c_int as avifTransferCharacteristics;
        (*header).matrixCoefficients = AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED
            as core::ffi::c_int as avifMatrixCoefficients;
    }
    if mono_chrome != 0 {
        (*header).range = (if avifBitsRead(bits, 1 as uint32_t) != 0 {
            AVIF_RANGE_FULL as core::ffi::c_int
        } else {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        }) as avifRange;
        (*header).av1C.chromaSubsamplingX = 1 as uint8_t;
        (*header).av1C.chromaSubsamplingY = 1 as uint8_t;
        (*header).yuvFormat = AVIF_PIXEL_FORMAT_YUV400;
    } else if (*header).colorPrimaries as core::ffi::c_int
        == AVIF_COLOR_PRIMARIES_BT709 as core::ffi::c_int
        && (*header).transferCharacteristics as core::ffi::c_int
            == AVIF_TRANSFER_CHARACTERISTICS_SRGB as core::ffi::c_int
        && (*header).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int
    {
        (*header).range = AVIF_RANGE_FULL;
        (*header).av1C.chromaSubsamplingX = 0 as uint8_t;
        (*header).av1C.chromaSubsamplingY = 0 as uint8_t;
        (*header).yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
    } else {
        let mut subsampling_x: uint32_t = 0 as uint32_t;
        let mut subsampling_y: uint32_t = 0 as uint32_t;
        (*header).range = (if avifBitsRead(bits, 1 as uint32_t) != 0 {
            AVIF_RANGE_FULL as core::ffi::c_int
        } else {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        }) as avifRange;
        match (*header).av1C.seqProfile as core::ffi::c_int {
            0 => {
                subsampling_x = 1 as uint32_t;
                subsampling_y = 1 as uint32_t;
                (*header).yuvFormat = AVIF_PIXEL_FORMAT_YUV420;
            }
            1 => {
                subsampling_x = 0 as uint32_t;
                subsampling_y = 0 as uint32_t;
                (*header).yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
            }
            2 => {
                if (*header).bitDepth == 12 as uint32_t {
                    subsampling_x = avifBitsRead(bits, 1 as uint32_t);
                    if subsampling_x != 0 {
                        subsampling_y = avifBitsRead(bits, 1 as uint32_t);
                    }
                } else {
                    subsampling_x = 1 as uint32_t;
                    subsampling_y = 0 as uint32_t;
                }
                if subsampling_x != 0 {
                    (*header).yuvFormat = (if subsampling_y != 0 {
                        AVIF_PIXEL_FORMAT_YUV420 as core::ffi::c_int
                    } else {
                        AVIF_PIXEL_FORMAT_YUV422 as core::ffi::c_int
                    }) as avifPixelFormat;
                } else {
                    (*header).yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
                }
            }
            _ => return AVIF_FALSE,
        }
        if subsampling_x != 0 && subsampling_y != 0 {
            (*header).chromaSamplePosition = avifBitsRead(bits, 2 as uint32_t)
                as avifChromaSamplePosition;
            (*header).av1C.chromaSamplePosition = (*header).chromaSamplePosition
                as uint8_t;
        }
        (*header).av1C.chromaSubsamplingX = subsampling_x as uint8_t;
        (*header).av1C.chromaSubsamplingY = subsampling_y as uint8_t;
    }
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn parseAV1SequenceHeader(
    mut bits: *mut avifBits,
    mut header: *mut avifSequenceHeader,
) -> avifBool {
    if parseAV1SequenceHeaderProfile(bits, header) == 0 {
        return AVIF_FALSE;
    }
    if parseSequenceHeaderLevelIdxAndTier(bits, header) == 0 {
        return AVIF_FALSE;
    }
    if parseSequenceHeaderFrameMaxDimensions(bits, header) == 0 {
        return AVIF_FALSE;
    }
    avifBitsRead(bits, 1 as uint32_t);
    if parseSequenceHeaderEnabledFeatures(bits, header) == 0 {
        return AVIF_FALSE;
    }
    avifBitsRead(bits, 3 as uint32_t);
    if parseAV1SequenceHeaderColorConfig(bits, header) == 0 {
        return AVIF_FALSE;
    }
    if (*header).av1C.monochrome == 0 {
        avifBitsRead(bits, 1 as uint32_t);
    }
    avifBitsRead(bits, 1 as uint32_t);
    return ((*bits).error == 0) as core::ffi::c_int;
}
unsafe extern "C" fn av1SequenceHeaderParse(
    mut header: *mut avifSequenceHeader,
    mut sample: *const avifROData,
) -> avifBool {
    let mut obus: avifROData = *sample;
    while obus.size > 0 as size_t {
        let mut bits: avifBits = avifBits {
            error: 0,
            eof: 0,
            state: 0,
            bitsLeft: 0,
            ptr: 0 as *const uint8_t,
            start: 0 as *const uint8_t,
            end: 0 as *const uint8_t,
        };
        avifBitsInit(&mut bits, obus.data, obus.size);
        let obu_forbidden_bit: uint32_t = avifBitsRead(&mut bits, 1 as uint32_t)
            as uint32_t;
        if obu_forbidden_bit != 0 as uint32_t {
            return AVIF_FALSE;
        }
        let obu_type: uint32_t = avifBitsRead(&mut bits, 4 as uint32_t) as uint32_t;
        let obu_extension_flag: uint32_t = avifBitsRead(&mut bits, 1 as uint32_t)
            as uint32_t;
        let obu_has_size_field: uint32_t = avifBitsRead(&mut bits, 1 as uint32_t)
            as uint32_t;
        avifBitsRead(&mut bits, 1 as uint32_t);
        if obu_extension_flag != 0 {
            avifBitsRead(&mut bits, 8 as uint32_t);
        }
        let mut obu_size: uint32_t = 0 as uint32_t;
        if obu_has_size_field != 0 {
            obu_size = avifBitsReadUleb128(&mut bits);
        } else {
            obu_size = ((obus.size as core::ffi::c_int - 1 as core::ffi::c_int)
                as uint32_t)
                .wrapping_sub(obu_extension_flag);
        }
        if bits.error != 0 {
            return AVIF_FALSE;
        }
        let init_bit_pos: uint32_t = avifBitsReadPos(&mut bits) as uint32_t;
        let init_byte_pos: uint32_t = init_bit_pos >> 3 as core::ffi::c_int;
        if obu_size as size_t > (obus.size).wrapping_sub(init_byte_pos as size_t) {
            return AVIF_FALSE;
        }
        if obu_type == 1 as uint32_t {
            let mut seqHdrBits: avifBits = avifBits {
                error: 0,
                eof: 0,
                state: 0,
                bitsLeft: 0,
                ptr: 0 as *const uint8_t,
                start: 0 as *const uint8_t,
                end: 0 as *const uint8_t,
            };
            avifBitsInit(
                &mut seqHdrBits,
                (obus.data).offset(init_byte_pos as isize),
                obu_size as size_t,
            );
            return parseAV1SequenceHeader(&mut seqHdrBits, header);
        }
        obus.data = (obus.data)
            .offset((obu_size as size_t).wrapping_add(init_byte_pos as size_t) as isize);
        obus.size = (obus.size)
            .wrapping_sub((obu_size as size_t).wrapping_add(init_byte_pos as size_t));
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifSequenceHeaderParse(
    mut header: *mut avifSequenceHeader,
    mut sample: *const avifROData,
    mut codecType: avifCodecType,
) -> avifBool {
    match codecType as core::ffi::c_uint {
        1 => return av1SequenceHeaderParse(header, sample),
        _ => return AVIF_FALSE,
    };
}
