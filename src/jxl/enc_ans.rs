// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported token/state and uint-configuration portions of `enc_ans.{h,cc}`.
//! Histogram selection, clustering, and LZ77 token generation are not yet
//! translated. Token bitfields use an explicit packed integer representation.

use super::ans_params::{ANS_LOG_TAB_SIZE, ANS_SIGNATURE};
use super::base::bits::ceil_log2_nonzero;
use super::base::status::Status;
use super::dec_ans::{HybridUintConfig, LZ77Params};
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;

const RECIPROCAL_PRECISION: u32 = 32 + ANS_LOG_TAB_SIZE;

#[derive(Default)]
pub struct ANSEncSymbolInfo {
    pub freq: u16,
    pub reverse_map: Vec<u16>,
    pub ifreq: u64,
    pub depth: u8,
    pub bits: u16,
}

pub struct ANSCoder {
    state: u32,
}

impl Default for ANSCoder {
    fn default() -> Self {
        Self {
            state: ANS_SIGNATURE << 16,
        }
    }
}

impl ANSCoder {
    pub fn put_symbol(&mut self, t: &ANSEncSymbolInfo, nbits: &mut u8) -> u32 {
        let mut bits = 0;
        *nbits = 0;
        if self.state >> (32 - ANS_LOG_TAB_SIZE) >= u32::from(t.freq) {
            bits = self.state & 0xffff;
            self.state >>= 16;
            *nbits = 16;
        }
        let v = (u64::from(self.state).wrapping_mul(t.ifreq) >> RECIPROCAL_PRECISION) as u32;
        let offset =
            t.reverse_map[self.state.wrapping_sub(v.wrapping_mul(u32::from(t.freq))) as usize];
        self.state = (v << ANS_LOG_TAB_SIZE).wrapping_add(u32::from(offset));
        bits
    }
    pub fn get_state(&self) -> u32 {
        self.state
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Token {
    context_and_lz77: u32,
    pub value: u32,
}

impl Token {
    pub const fn new(context: u32, value: u32) -> Self {
        Self {
            context_and_lz77: context << 1,
            value,
        }
    }
    pub const fn context(self) -> u32 {
        self.context_and_lz77 >> 1
    }
    pub const fn is_lz77_length(self) -> bool {
        self.context_and_lz77 & 1 != 0
    }
    pub fn set_context(&mut self, context: u32) {
        self.context_and_lz77 = (context << 1) | (self.context_and_lz77 & 1);
    }
    pub fn set_is_lz77_length(&mut self, value: bool) {
        self.context_and_lz77 = (self.context_and_lz77 & !1) | u32::from(value);
    }
}

#[derive(Default)]
pub struct EntropyEncodingData {
    pub encoding_info: Vec<Vec<ANSEncSymbolInfo>>,
    pub use_prefix_code: bool,
    pub uint_config: Vec<HybridUintConfig>,
    pub log_alpha_size: usize,
    pub lz77: LZ77Params,
    pub encoded_histograms: Vec<BitWriter>,
    pub context_map: Vec<u8>,
}

pub fn encode_uint_config(config: HybridUintConfig, writer: &mut BitWriter, log_alpha_size: usize) {
    writer.write(
        ceil_log2_nonzero(log_alpha_size + 1),
        u64::from(config.split_exponent),
    );
    if config.split_exponent as usize == log_alpha_size {
        return;
    }
    let nbits = ceil_log2_nonzero(config.split_exponent + 1);
    writer.write(nbits, u64::from(config.msb_in_token));
    let nbits = ceil_log2_nonzero(config.split_exponent - config.msb_in_token + 1);
    writer.write(nbits, u64::from(config.lsb_in_token));
}

pub fn encode_uint_configs(
    configs: &[HybridUintConfig],
    writer: &mut BitWriter,
    log_alpha_size: usize,
) {
    for &cfg in configs {
        encode_uint_config(cfg, writer, log_alpha_size);
    }
}

/// The caller supplies the writer allotment and populated encoding tables.
pub fn write_tokens(
    tokens: &[Token],
    codes: &EntropyEncodingData,
    context_offset: usize,
    writer: &mut BitWriter,
) -> usize {
    let mut num_extra_bits = 0;
    if codes.use_prefix_code {
        for &token in tokens {
            let histo = usize::from(codes.context_map[context_offset + token.context() as usize]);
            let config = if token.is_lz77_length() {
                codes.lz77.length_uint_config
            } else {
                codes.uint_config[histo]
            };
            let (mut tok, nbits, bits) = config.encode(token.value);
            tok = tok.wrapping_add(if token.is_lz77_length() {
                codes.lz77.min_symbol
            } else {
                0
            });
            let info = &codes.encoding_info[histo][tok as usize];
            let mut data = u64::from(info.bits);
            data |= u64::from(bits) << info.depth;
            writer.write(usize::from(info.depth) + nbits as usize, data);
            num_extra_bits += nbits as usize;
        }
        return num_extra_bits;
    }
    let mut out = Vec::with_capacity(tokens.len());
    let mut out_nbits = Vec::with_capacity(tokens.len());
    let mut allbits = 0u64;
    let mut numallbits = 0;
    let mut addbits = |bits: usize, nbits: usize| {
        if nbits != 0 {
            debug_assert_eq!(bits >> nbits, 0);
            if numallbits + nbits > BitWriter::MAX_BITS_PER_CALL {
                out.push(allbits);
                out_nbits.push(numallbits as u8);
                numallbits = 0;
                allbits = 0;
            }
            allbits <<= nbits;
            allbits |= bits as u64;
            numallbits += nbits;
        }
    };
    let mut ans = ANSCoder::default();
    if codes.lz77.enabled || codes.context_map.len() > 1 {
        for &token in tokens.iter().rev() {
            let histo = usize::from(codes.context_map[context_offset + token.context() as usize]);
            let config = if token.is_lz77_length() {
                codes.lz77.length_uint_config
            } else {
                codes.uint_config[histo]
            };
            let (mut tok, nbits, bits) = config.encode(token.value);
            tok = tok.wrapping_add(if token.is_lz77_length() {
                codes.lz77.min_symbol
            } else {
                0
            });
            let info = &codes.encoding_info[histo][tok as usize];
            debug_assert!(info.freq > 0);
            addbits(bits as usize, nbits as usize);
            num_extra_bits += nbits as usize;
            let mut ans_nbits = 0;
            let ans_bits = ans.put_symbol(info, &mut ans_nbits);
            addbits(ans_bits as usize, usize::from(ans_nbits));
        }
    } else {
        for token in tokens.iter().rev() {
            let (tok, nbits, bits) = codes.uint_config[0].encode(token.value);
            let info = &codes.encoding_info[0][tok as usize];
            addbits(bits as usize, nbits as usize);
            num_extra_bits += nbits as usize;
            let mut ans_nbits = 0;
            let ans_bits = ans.put_symbol(info, &mut ans_nbits);
            addbits(ans_bits as usize, usize::from(ans_nbits));
        }
    }
    writer.write(32, u64::from(ans.get_state()));
    writer.write(numallbits, allbits);
    for i in (0..out.len()).rev() {
        writer.write(usize::from(out_nbits[i]), out[i]);
    }
    num_extra_bits
}

pub fn write_tokens_with_allotment(
    tokens: &[Token],
    codes: &EntropyEncodingData,
    context_offset: usize,
    writer: &mut BitWriter,
    layer: LayerType,
    aux_out: Option<&mut AuxOut>,
) -> Status {
    writer.with_max_bits(
        46 * tokens.len() + 32 * 1024 * 4,
        layer,
        aux_out,
        |w, aux| {
            let num_extra_bits = write_tokens(tokens, codes, context_offset, w);
            if let Some(aux) = aux {
                aux.layer_mut(layer).extra_bits += num_extra_bits;
            }
            Ok(())
        },
        false,
    )
}
