// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete command decoding and incremental ICCReader from lib/jxl/icc_codec.cc.
use super::base::status::{Status, StatusCode};
use super::dec_ans::{
    ANSCode, ANSSymbolReader, ANSSymbolReaderState, Checkpoint, MAX_CHECKPOINT_INTERVAL,
    decode_histograms,
};
use super::dec_bit_reader::BitReader;
use super::icc_codec_common::*;
use super::memory_manager_internal::MemoryManagerPtr;
use super::padded_bytes::PaddedBytes;
pub const PREAMBLE_SIZE: usize = 20;
pub fn shuffle(memory_manager: MemoryManagerPtr, data: &mut [u8], width: usize) -> Status {
    let size = data.len();
    let height = size.wrapping_add(width).wrapping_sub(1) / width;
    let mut result = PaddedBytes::with_initial_space_in(memory_manager, size)?;
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
    let memory_manager = result.memory_manager();
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
    let mut header = PaddedBytes::new(memory_manager);
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
                    let mut shuffled =
                        PaddedBytes::with_initial_space_in(memory_manager, num as usize)?;
                    shuffled.bytes_mut()[..num as usize]
                        .copy_from_slice(&enc[pos..pos + num as usize]);
                    shuffle(
                        memory_manager,
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
                let mut shuffled =
                    PaddedBytes::with_initial_space_in(memory_manager, num as usize)?;
                shuffled.bytes_mut()[..num as usize].copy_from_slice(&enc[pos..pos + num as usize]);
                if width > 1 {
                    shuffle(
                        memory_manager,
                        &mut shuffled.bytes_mut()[..num as usize],
                        width,
                    )?;
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

/// Incremental reader from icc_codec.{h,cc}. The code and code-independent ANS
/// state have separate owners; temporary borrowed readers move the same state
/// back on every Status return, retaining the LZ77 allocation and checkpoints.
pub struct ICCReader {
    i: usize,
    bits_to_skip: usize,
    used_bits_base: usize,
    enc_size: u64,
    context_map: Vec<u8>,
    code: ANSCode,
    ans_state: ANSSymbolReaderState,
    decompressed: PaddedBytes,
}

impl Default for ICCReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ICCReader {
    /// An ICCReader on the native allocator boundary.
    pub fn new() -> Self {
        Self::new_in(MemoryManagerPtr::NATIVE)
    }

    /// The source ICCReader(JxlMemoryManager*) constructor.
    pub fn new_in(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            i: 0,
            bits_to_skip: 0,
            used_bits_base: 0,
            enc_size: 0,
            context_map: Vec::new(),
            code: ANSCode::default(),
            ans_state: ANSSymbolReaderState::default(),
            decompressed: PaddedBytes::new(memory_manager),
        }
    }

    pub fn reset(&mut self) {
        self.bits_to_skip = 0;
        self.decompressed.clear();
    }

    fn check_eoi(reader: &mut BitReader<'_>) -> Status {
        reader
            .all_reads_within_bounds()
            .map_err(|_| StatusCode::NotEnoughBytes)
    }

    pub fn init(&mut self, reader: &mut BitReader<'_>) -> Status {
        Self::check_eoi(reader)?;
        let memory_manager = self.decompressed.memory_manager();
        self.used_bits_base = reader.total_bits_consumed();
        if self.bits_to_skip == 0 {
            self.enc_size = super::fields::u64_coder::read(reader);
            if self.enc_size > 268_435_456 {
                return Err(StatusCode::GenericError);
            }
            decode_histograms(
                memory_manager,
                reader,
                NUM_ICC_CONTEXTS,
                &mut self.code,
                &mut self.context_map,
                false,
            )?;
            self.ans_state = ANSSymbolReader::create(&self.code, reader, 0)?.into_state();
            self.i = 0;
            self.decompressed
                .resize((self.i + 0x400).min(self.enc_size as usize))?;
            let mut ans =
                ANSSymbolReader::from_state(&self.code, std::mem::take(&mut self.ans_state));
            let status = (|| {
                while self.i < 2.min(self.enc_size as usize) {
                    let context = icc_ans_context(
                        self.i,
                        if self.i > 0 {
                            usize::from(self.decompressed.as_slice()[self.i - 1])
                        } else {
                            0
                        },
                        if self.i > 1 {
                            usize::from(self.decompressed.as_slice()[self.i - 2])
                        } else {
                            0
                        },
                    );
                    self.decompressed.bytes_mut()[self.i] =
                        ans.read_hybrid_uint(context, reader, &self.context_map) as u8;
                    self.i += 1;
                }
                if self.enc_size > PREAMBLE_SIZE as u64 {
                    while self.i < PREAMBLE_SIZE {
                        let context = icc_ans_context(
                            self.i,
                            usize::from(self.decompressed.as_slice()[self.i - 1]),
                            usize::from(self.decompressed.as_slice()[self.i - 2]),
                        );
                        self.decompressed.bytes_mut()[self.i] =
                            ans.read_hybrid_uint(context, reader, &self.context_map) as u8;
                        self.i += 1;
                    }
                    Self::check_eoi(reader)?;
                    check_preamble(self.decompressed.as_slice(), self.enc_size as usize)?;
                }
                Ok(())
            })();
            self.ans_state = ans.into_state();
            status?;
            self.bits_to_skip = reader
                .total_bits_consumed()
                .wrapping_sub(self.used_bits_base);
        } else {
            reader.skip_bits(self.bits_to_skip);
        }
        Ok(())
    }

    #[expect(
        clippy::manual_is_multiple_of,
        reason = "Mirror the source modulo test."
    )]
    pub fn process(&mut self, reader: &mut BitReader<'_>, icc: &mut PaddedBytes) -> Status {
        let mut checkpoint = Box::new(Checkpoint::default());
        let mut ans = ANSSymbolReader::from_state(&self.code, std::mem::take(&mut self.ans_state));
        ans.save(&mut checkpoint);
        self.bits_to_skip = reader
            .total_bits_consumed()
            .wrapping_sub(self.used_bits_base);
        let mut saved_i = self.i;
        let status = (|| {
            while self.i < self.enc_size as usize {
                if self.i % MAX_CHECKPOINT_INTERVAL == 0 && self.i > 0 {
                    if let Err(status) = Self::check_eoi(reader) {
                        ans.restore(&checkpoint);
                        self.i = saved_i;
                        return Err(status);
                    }
                    ans.save(&mut checkpoint);
                    self.bits_to_skip = reader
                        .total_bits_consumed()
                        .wrapping_sub(self.used_bits_base);
                    saved_i = self.i;
                    if self.i & 0xffff == 0 {
                        let used_bytes = reader
                            .total_bits_consumed()
                            .wrapping_sub(self.used_bits_base)
                            as f32
                            / 8.0;
                        if self.i as f32 > used_bytes * 256.0 {
                            return Err(StatusCode::GenericError);
                        }
                    }
                    self.decompressed
                        .resize((self.i + 0x400).min(self.enc_size as usize))?;
                }
                if self.i < 2 {
                    return Err(StatusCode::GenericError);
                }
                let context = icc_ans_context(
                    self.i,
                    usize::from(self.decompressed.as_slice()[self.i - 1]),
                    usize::from(self.decompressed.as_slice()[self.i - 2]),
                );
                self.decompressed.bytes_mut()[self.i] =
                    ans.read_hybrid_uint(context, reader, &self.context_map) as u8;
                self.i += 1;
            }
            if let Err(status) = Self::check_eoi(reader) {
                ans.restore(&checkpoint);
                self.i = saved_i;
                return Err(status);
            }
            self.bits_to_skip = reader
                .total_bits_consumed()
                .wrapping_sub(self.used_bits_base);
            if !ans.check_ans_final_state() {
                return Err(StatusCode::GenericError);
            }
            icc.clear();
            unpredict_icc(self.decompressed.as_slice(), icc)
        })();
        self.ans_state = ans.into_state();
        status
    }
}

/// Read-only fixture access to initialized scalar fields and logical storage.
/// Only the observer's independently tracked written prefix of decompressed is
/// original-defined; capacity and unwritten logical bytes are not ICC output.
#[cfg(feature = "cpp-oracle")]
pub struct ICCReaderSnapshot<'a> {
    pub i: usize,
    pub bits_to_skip: usize,
    pub used_bits_base: usize,
    pub enc_size: u64,
    pub context_map: &'a [u8],
    pub code_sizes: [usize; 3],
    pub ans_scalars: [u32; 15],
    pub cached_use_prefix_code: bool,
    pub owner_use_prefix_code: bool,
    pub code_addresses: [usize; 3],
    pub lz_window: Option<&'a [u32]>,
    pub decompressed: &'a [u8],
    pub decompressed_capacity: usize,
    pub decompressed_address: usize,
}

#[cfg(feature = "cpp-oracle")]
impl ICCReader {
    pub fn observer_snapshot(&self) -> ICCReaderSnapshot<'_> {
        let storage = self.decompressed.bytes();
        ICCReaderSnapshot {
            i: self.i,
            bits_to_skip: self.bits_to_skip,
            used_bits_base: self.used_bits_base,
            enc_size: self.enc_size,
            context_map: &self.context_map,
            code_sizes: [
                self.code.huffman_data.len(),
                self.code.uint_config.len(),
                self.code.degenerate_symbols.len(),
            ],
            ans_scalars: self.ans_state.observer_scalars(),
            cached_use_prefix_code: self.ans_state.observer_use_prefix_code(),
            owner_use_prefix_code: self.code.use_prefix_code,
            code_addresses: [
                if self.code.alias_tables().is_empty() {
                    0
                } else {
                    self.code.alias_tables().as_ptr() as usize
                },
                if self.code.huffman_data.is_empty() {
                    0
                } else {
                    self.code.huffman_data.as_ptr() as usize
                },
                if self.code.uint_config.is_empty() {
                    0
                } else {
                    self.code.uint_config.as_ptr() as usize
                },
            ],
            lz_window: self.ans_state.observer_window(),
            decompressed: self.decompressed.as_slice(),
            decompressed_capacity: storage.len().saturating_sub(8),
            decompressed_address: if storage.is_empty() {
                0
            } else {
                storage.as_ptr() as usize
            },
        }
    }

    pub fn observer_check_eoi(reader: &mut BitReader<'_>) -> Status {
        Self::check_eoi(reader)
    }
}
