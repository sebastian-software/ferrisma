// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Command decoding from lib/jxl/icc_codec.cc, through complete UnpredictICC.
//! The streaming ICCReader entropy/lifecycle boundary is ported separately.
use super::base::status::{Status, StatusCode};
use super::icc_codec_common::*;
use super::padded_bytes::PaddedBytes;
pub const PREAMBLE_SIZE: usize = 20;
pub fn shuffle(data: &mut [u8], width: usize) -> Status {
    let size = data.len();
    let height = size.wrapping_add(width).wrapping_sub(1) / width;
    let mut result = PaddedBytes::with_initial_space(size)?;
    let mut s = 0;
    let mut j = 0;
    for i in 0..size {
        result.bytes_mut()[i] = data[j];
        j = j.wrapping_add(height);
        if j >= size {
            s += 1;
            j = s;
        }
    }
    data.copy_from_slice(result.as_slice());
    Ok(())
}
/// Failure retains the advanced position and leaves the output unchanged.
pub fn decode_var_int(input: &[u8], pos: &mut usize, out: &mut u64) -> Status {
    let mut ret = 0;
    for i in 0..9 {
        if *pos >= input.len() {
            return Err(StatusCode::GenericError);
        }
        let byte = input[*pos];
        *pos += 1;
        ret |= u64::from(byte & 0x7f) << (7 * i);
        if byte & 0x80 == 0 {
            *out = ret;
            return Ok(());
        }
    }
    if *pos >= input.len() {
        return Err(StatusCode::GenericError);
    }
    let byte = input[*pos];
    *pos += 1;
    if byte & 0x80 != 0 || byte & 0x7e != 0 {
        return Err(StatusCode::GenericError);
    }
    ret |= u64::from(byte & 1) << 63;
    *out = ret;
    Ok(())
}
pub fn check_preamble(data: &[u8], enc_size: usize) -> Status {
    let mut pos = 0;
    let mut osize = 0;
    decode_var_int(data, &mut pos, &mut osize)?;
    check_is_32_bit(osize)?;
    let mut csize = 0;
    decode_var_int(data, &mut pos, &mut csize)?;
    check_is_32_bit(csize)?;
    check_out_of_bounds(pos as u64, csize, data.len() as u64)?;
    if osize + 65536 < enc_size as u64 || osize > 1 << 28 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
pub fn unpredict_icc(enc: &[u8], result: &mut PaddedBytes) -> Status {
    if !result.is_empty() {
        return Err(StatusCode::GenericError);
    }
    let mut pos = 0;
    let mut osize = 0;
    decode_var_int(enc, &mut pos, &mut osize)?;
    check_is_32_bit(osize)?;
    let mut csize = 0;
    decode_var_int(enc, &mut pos, &mut csize)?;
    check_is_32_bit(csize)?;
    let mut cpos = pos;
    check_out_of_bounds(pos as u64, csize, enc.len() as u64)?;
    let commands_end = cpos + csize as usize;
    pos = commands_end;
    let mut header = PaddedBytes::default();
    header.append(&icc_initial_header_prediction(osize as u32))?;
    for i in 0..=ICC_HEADER_SIZE {
        if result.len() as u64 == osize {
            if cpos != commands_end || pos != enc.len() {
                return Err(StatusCode::GenericError);
            }
            return Ok(());
        }
        if i == ICC_HEADER_SIZE {
            break;
        }
        icc_predict_header(
            result.as_slice(),
            (&mut header.bytes_mut()[..ICC_HEADER_SIZE])
                .try_into()
                .unwrap(),
            i,
        );
        if pos >= enc.len() {
            return Err(StatusCode::GenericError);
        }
        let value = enc[pos].wrapping_add(header.as_slice()[i]);
        pos += 1;
        result.push_back(value)?;
    }
    if cpos >= commands_end {
        return Err(StatusCode::GenericError);
    }
    let mut numtags = 0;
    decode_var_int(&enc[..commands_end], &mut cpos, &mut numtags)?;
    if numtags != 0 {
        numtags -= 1;
        check_is_32_bit(numtags)?;
        append_uint32(numtags as u32, result)?;
        let mut prevtagstart = ICC_HEADER_SIZE as u64 + numtags * 12;
        let mut prevtagsize = 0;
        loop {
            if result.len() as u64 > osize || cpos > commands_end {
                return Err(StatusCode::GenericError);
            }
            if cpos == commands_end {
                break;
            }
            let command = enc[cpos];
            cpos += 1;
            let tagcode = command & 63;
            let tag = match tagcode {
                0 => break,
                COMMAND_TAG_UNKNOWN => {
                    check_out_of_bounds(pos as u64, 4, enc.len() as u64)?;
                    let tag = decode_keyword(enc, pos);
                    pos += 4;
                    tag
                }
                COMMAND_TAG_TRC => RTRC_TAG,
                COMMAND_TAG_XYZ => RXYZ_TAG,
                _ => {
                    let index = (tagcode - COMMAND_TAG_STRING_FIRST) as usize;
                    if index >= TAG_STRINGS.len() {
                        return Err(StatusCode::GenericError);
                    }
                    TAG_STRINGS[index]
                }
            };
            append_keyword(&tag, result)?;
            let mut tagsize = prevtagsize;
            if [
                RXYZ_TAG, GXYZ_TAG, BXYZ_TAG, KXYZ_TAG, WTPT_TAG, BKPT_TAG, LUMI_TAG,
            ]
            .contains(&tag)
            {
                tagsize = 20;
            }
            let mut tagstart = 0;
            if command & FLAG_BIT_OFFSET != 0 {
                decode_var_int(&enc[..commands_end], &mut cpos, &mut tagstart)?;
            } else {
                check_is_32_bit(prevtagstart)?;
                tagstart = prevtagstart + prevtagsize;
            }
            check_is_32_bit(tagstart)?;
            append_uint32(tagstart as u32, result)?;
            if command & FLAG_BIT_SIZE != 0 {
                decode_var_int(&enc[..commands_end], &mut cpos, &mut tagsize)?;
            }
            check_is_32_bit(tagsize)?;
            append_uint32(tagsize as u32, result)?;
            prevtagstart = tagstart;
            prevtagsize = tagsize;
            if tagcode == COMMAND_TAG_TRC {
                append_keyword(&GTRC_TAG, result)?;
                append_uint32(tagstart as u32, result)?;
                append_uint32(tagsize as u32, result)?;
                append_keyword(&BTRC_TAG, result)?;
                append_uint32(tagstart as u32, result)?;
                append_uint32(tagsize as u32, result)?;
            }
            if tagcode == COMMAND_TAG_XYZ {
                check_is_32_bit(tagstart + tagsize * 2)?;
                append_keyword(&GXYZ_TAG, result)?;
                append_uint32((tagstart + tagsize) as u32, result)?;
                append_uint32(tagsize as u32, result)?;
                append_keyword(&BXYZ_TAG, result)?;
                append_uint32((tagstart + tagsize * 2) as u32, result)?;
                append_uint32(tagsize as u32, result)?;
            }
        }
    }
    loop {
        if result.len() as u64 > osize || cpos > commands_end {
            return Err(StatusCode::GenericError);
        }
        if cpos == commands_end {
            break;
        }
        let command = enc[cpos];
        cpos += 1;
        match command {
            COMMAND_INSERT | COMMAND_SHUFFLE2 | COMMAND_SHUFFLE4 => {
                let mut num = 0;
                decode_var_int(&enc[..commands_end], &mut cpos, &mut num)?;
                check_out_of_bounds(pos as u64, num, enc.len() as u64)?;
                if command == COMMAND_INSERT {
                    for _ in 0..num {
                        let value = enc[pos];
                        pos += 1;
                        result.push_back(value)?;
                    }
                } else {
                    let mut shuffled = PaddedBytes::with_initial_space(num as usize)?;
                    shuffled.bytes_mut()[..num as usize]
                        .copy_from_slice(&enc[pos..pos + num as usize]);
                    shuffle(
                        &mut shuffled.bytes_mut()[..num as usize],
                        if command == COMMAND_SHUFFLE2 { 2 } else { 4 },
                    )?;
                    for &value in shuffled.as_slice() {
                        result.push_back(value)?;
                        pos += 1;
                    }
                }
            }
            COMMAND_PREDICT => {
                check_out_of_bounds(cpos as u64, 2, commands_end as u64)?;
                let flags = enc[cpos];
                cpos += 1;
                let width = usize::from((flags & 3) + 1);
                if width == 3 {
                    return Err(StatusCode::GenericError);
                }
                let order = i32::from((flags & 12) >> 2);
                if order == 3 {
                    return Err(StatusCode::GenericError);
                }
                let mut stride = width as u64;
                if flags & 16 != 0 {
                    decode_var_int(&enc[..commands_end], &mut cpos, &mut stride)?;
                    if stride < width as u64 {
                        return Err(StatusCode::GenericError);
                    }
                }
                if result.is_empty() || (((result.len() - 1) >> 2) as u64) < stride {
                    return Err(StatusCode::GenericError);
                }
                let mut num = 0;
                decode_var_int(&enc[..commands_end], &mut cpos, &mut num)?;
                check_out_of_bounds(pos as u64, num, enc.len() as u64)?;
                let mut shuffled = PaddedBytes::with_initial_space(num as usize)?;
                shuffled.bytes_mut()[..num as usize].copy_from_slice(&enc[pos..pos + num as usize]);
                if width > 1 {
                    shuffle(&mut shuffled.bytes_mut()[..num as usize], width)?;
                }
                let start = result.len();
                for (i, &value) in shuffled.as_slice().iter().enumerate() {
                    let predicted = linear_predict_icc_value(
                        result.as_slice(),
                        start,
                        i,
                        stride as usize,
                        width,
                        order,
                    );
                    result.push_back(predicted.wrapping_add(value))?;
                }
                pos += num as usize;
            }
            COMMAND_XYZ => {
                append_keyword(&XYZ_TAG, result)?;
                for _ in 0..4 {
                    result.push_back(0)?;
                }
                check_out_of_bounds(pos as u64, 12, enc.len() as u64)?;
                for _ in 0..12 {
                    let value = enc[pos];
                    pos += 1;
                    result.push_back(value)?;
                }
            }
            _ if command >= COMMAND_TYPE_START_FIRST
                && (command - COMMAND_TYPE_START_FIRST) < TYPE_STRINGS.len() as u8 =>
            {
                append_keyword(
                    &TYPE_STRINGS[(command - COMMAND_TYPE_START_FIRST) as usize],
                    result,
                )?;
                for _ in 0..4 {
                    result.push_back(0)?;
                }
            }
            _ => return Err(StatusCode::GenericError),
        }
    }
    if pos != enc.len() || result.len() as u64 != osize {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
