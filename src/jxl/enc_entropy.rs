// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Entropy orchestration from `lib/jxl/enc_ans.cc`, with its release-state
//! fuzzer flag (false). The debug-only fuzzer setter remains unported.
//! The token-cost loop and maximum-vector padding use the existing Highway
//! scalar specialization. Borrowed writers replace optional source pointers.

use super::ans_params::{ANS_MAX_ALPHABET_SIZE, ANS_TAB_SIZE, PREFIX_MAX_BITS};
use super::base::bits::{ceil_log2_nonzero, floor_log2_nonzero};
use super::base::status::{Status, StatusCode, StatusOr};
use super::dec_ans::HybridUintConfig;
use super::enc_ans::{
    ANSEncSymbolInfo, EntropyEncodingData, SizeWriter, Token, Writer, encode_uint_config,
    encode_uint_config_to, encode_uint_configs, encode_uint_configs_to,
};
use super::enc_ans_params::{ClusteringType, Histogram, HistogramParams, HybridUintMethod};
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_cluster::cluster_histograms;
use super::enc_context_map::{CLUSTERS_LIMIT, encode_context_map};
use super::enc_fields::bundle;
use super::enc_lz77::apply_lz77;
use super::memory_manager_internal::AlignedMemory;

fn store_var_len_uint16(n: usize, writer: &mut impl Writer) {
    debug_assert!(n <= 65535);
    if n == 0 {
        writer.write(1, 0);
    } else {
        writer.write(1, 1);
        let nbits = floor_log2_nonzero(n);
        writer.write(4, nbits as u64);
        writer.write(nbits, (n - (1usize << nbits)) as u64);
    }
}

fn histogram_from_symbol_info(info: &[ANSEncSymbolInfo], prefix: bool) -> Histogram {
    let mut histogram = Histogram::new(info.len());
    for (i, symbol) in info.iter().enumerate() {
        let count = if prefix {
            if symbol.depth == 0 {
                0
            } else {
                1u32 << (PREFIX_MAX_BITS - usize::from(symbol.depth))
            }
        } else {
            u32::from(symbol.freq)
        };
        histogram.counts[i] = count as i32;
        histogram.total_count += count as usize;
    }
    histogram
}

// enc_ans_simd.cc's HWY_SCALAR specialization, not a replacement for its
// vector algorithm. Its extra-bit counter is deliberately 32-bit.
fn estimate_token_cost(values: &[u32], config: HybridUintConfig, out: &mut [u32]) -> u32 {
    let mut extra_bits = 0u32;
    for (i, &value) in values.iter().enumerate() {
        let (tok, nbits, _) = config.encode(value);
        extra_bits = extra_bits.wrapping_add(nbits);
        out[i] = tok;
    }
    extra_bits
}

impl EntropyEncodingData {
    pub fn choose_uint_configs(
        &mut self,
        params: &HistogramParams,
        tokens: &[Vec<Token>],
        clustered: &mut [Histogram],
    ) -> Status {
        self.log_alpha_size = if self.use_prefix_code {
            PREFIX_MAX_BITS
        } else if params.streaming_mode || self.lz77.enabled {
            8
        } else {
            7
        };
        self.uint_config = vec![params.uint_config(); clustered.len()];
        if !matches!(
            params.uint_method,
            HybridUintMethod::Best | HybridUintMethod::Fast
        ) || params.streaming_mode
        {
            return Ok(());
        }
        const BEST: [(u32, u32, u32); 28] = [
            (4, 2, 0),
            (4, 1, 0),
            (4, 2, 1),
            (4, 2, 2),
            (4, 1, 2),
            (5, 2, 0),
            (5, 1, 0),
            (5, 2, 1),
            (5, 2, 2),
            (5, 1, 2),
            (3, 2, 0),
            (3, 1, 0),
            (3, 2, 1),
            (3, 1, 2),
            (4, 1, 3),
            (5, 1, 4),
            (5, 2, 3),
            (6, 1, 5),
            (6, 2, 4),
            (6, 0, 0),
            (0, 0, 0),
            (2, 0, 1),
            (7, 0, 0),
            (8, 0, 0),
            (9, 0, 0),
            (10, 0, 0),
            (11, 0, 0),
            (12, 0, 0),
        ];
        const FAST: [(u32, u32, u32); 4] = [(4, 2, 0), (4, 1, 2), (0, 0, 0), (2, 0, 1)];
        let candidates = if params.uint_method == HybridUintMethod::Best {
            &BEST[..]
        } else {
            &FAST[..]
        };
        let configs: Vec<_> = candidates
            .iter()
            .map(|&(e, m, l)| HybridUintConfig::new(e, m, l))
            .collect();
        let num_histo = clustered.len();
        let _is_valid = vec![0u8; num_histo];
        let mut volume = vec![0usize; 2 * num_histo];
        let mut offset = vec![0usize; 2 * num_histo + 1];
        let mut max_value = vec![0u32; 2 * num_histo];
        for stream in tokens {
            for token in stream {
                let h = usize::from(self.context_map[token.context() as usize]);
                volume[h + if token.is_lz77_length() { num_histo } else { 0 }] += 1;
            }
        }
        let mut max_volume = 0;
        for h in 0..2 * num_histo {
            max_volume = max_volume.max(volume[h]);
            offset[h + 1] = offset[h] + volume[h];
        }
        // MaxVectorSize() returns four bytes in the scalar source. Upstream
        // adds that value to a count of u32 elements; retain that padding.
        const MAX_VEC_SIZE: usize = 4;
        let mut transposed = vec![0u32; offset[2 * num_histo] + MAX_VEC_SIZE];
        {
            let mut next_offset = offset.clone();
            for stream in tokens {
                for token in stream {
                    let h = usize::from(self.context_map[token.context() as usize])
                        + if token.is_lz77_length() { num_histo } else { 0 };
                    transposed[next_offset[h]] = token.value;
                    next_offset[h] += 1;
                }
            }
        }
        for h in 0..2 * num_histo {
            max_value[h] = transposed[offset[h]..offset[h + 1]]
                .iter()
                .copied()
                .max()
                .unwrap_or(0);
        }
        let mut _max_lz77 = 0;
        for h in num_histo..2 * num_histo {
            _max_lz77 = _max_lz77.max(
                transposed[offset[h]..offset[h + 1]]
                    .iter()
                    .copied()
                    .max()
                    .unwrap_or(0),
            );
        }
        let mut tmp = AlignedMemory::create((max_volume + MAX_VEC_SIZE) * 4, 0)?;
        for h in 0..num_histo {
            let mut best_cost = f32::MAX;
            for &cfg in &configs {
                let (mut tok, _, _) = cfg.encode(max_value[h]);
                tok |= cfg.lsb_mask();
                if tok as usize >= ANS_MAX_ALPHABET_SIZE
                    || (self.lz77.enabled && tok >= self.lz77.min_symbol)
                {
                    continue;
                }
                let mut histo = Histogram::new(tok as usize + 1);
                let extra_bits =
                    estimate_token_cost(&transposed[offset[h]..offset[h + 1]], cfg, tmp.u32s_mut());
                for &tok in &tmp.u32s()[..volume[h]] {
                    histo.fast_add(tok as usize);
                }
                histo.condition();
                let mut cost = histo.ans_population_cost()?;
                cost += extra_bits as f32;
                cost += ceil_log2_nonzero(cfg.split_exponent + 1) as f32;
                cost += ceil_log2_nonzero(cfg.split_exponent - cfg.msb_in_token + 1) as f32;
                if cost < best_cost {
                    self.uint_config[h] = cfg;
                    best_cost = cost;
                    clustered[h].swap(&mut histo);
                }
            }
        }
        let mut max_tok = 0usize;
        for h in 0..num_histo {
            let histo = &mut clustered[h];
            max_tok = max_tok.max(histo.max_symbol());
            let len = volume[num_histo + h];
            if len == 0 {
                continue;
            }
            let cfg = self.lz77.length_uint_config;
            let (mut tok, _, _) = cfg.encode(max_value[num_histo + h]);
            tok |= cfg.lsb_mask();
            tok = tok.wrapping_add(self.lz77.min_symbol);
            histo.ensure_capacity(tok as usize + 1);
            estimate_token_cost(
                &transposed[offset[num_histo + h]..offset[num_histo + h + 1]],
                cfg,
                tmp.u32s_mut(),
            );
            for &tok in &tmp.u32s()[..len] {
                histo.fast_add(tok.wrapping_add(self.lz77.min_symbol) as usize);
            }
            histo.condition();
            max_tok = max_tok.max(histo.max_symbol());
        }
        let mut log_size = 5;
        while max_tok >= (1usize << log_size) {
            log_size += 1;
        }
        if log_size
            > if self.use_prefix_code {
                PREFIX_MAX_BITS
            } else {
                8
            }
        {
            return Err(StatusCode::GenericError);
        }
        self.log_alpha_size = if self.use_prefix_code {
            PREFIX_MAX_BITS
        } else {
            log_size
        };
        Ok(())
    }

    pub fn build_and_store_entropy_codes(
        &mut self,
        params: &HistogramParams,
        tokens: &[Vec<Token>],
        builder: &mut [Histogram],
        mut writer: Option<&mut BitWriter>,
        layer: LayerType,
        mut aux: Option<&mut AuxOut>,
    ) -> StatusOr<usize> {
        let prev_histograms = self.encoding_info.len();
        let mut clustered: Vec<_> = self
            .encoding_info
            .iter()
            .map(|info| histogram_from_symbol_info(info, self.use_prefix_code))
            .collect();
        let context_offset = self.context_map.len();
        self.context_map.resize(context_offset + builder.len(), 0);
        if builder.len() > 1 {
            let mut symbols = Vec::new();
            cluster_histograms(
                params,
                builder,
                CLUSTERS_LIMIT,
                &mut clustered,
                &mut symbols,
            )?;
            for (c, &symbol) in symbols.iter().enumerate() {
                self.context_map[context_offset + c] = symbol as u8;
            }
            if let Some(w) = writer.as_deref_mut() {
                encode_context_map(
                    &self.context_map,
                    clustered.len(),
                    w,
                    layer,
                    aux.as_deref_mut(),
                )?;
            }
        } else {
            if !self.encoding_info.is_empty() {
                return Err(StatusCode::GenericError);
            }
            clustered.push(builder[0].clone());
        }
        if let Some(aux) = aux.as_deref_mut() {
            for histo in &mut clustered[prev_histograms..] {
                aux.layer_mut(layer).clustered_entropy += f64::from(histo.shannon_entropy());
            }
        }
        self.choose_uint_configs(params, tokens, &mut clustered)?;
        let mut size_writer = SizeWriter::default();
        let mut cost = if self.use_prefix_code { 1usize } else { 3 };
        if let Some(w) = writer.as_deref_mut() {
            w.write(1, u64::from(self.use_prefix_code));
            if !self.use_prefix_code {
                w.write(2, (self.log_alpha_size - 5) as u64);
            }
            encode_uint_configs(&self.uint_config, w, self.log_alpha_size);
        } else {
            encode_uint_configs_to(&self.uint_config, &mut size_writer, self.log_alpha_size);
        }
        if self.use_prefix_code {
            for histo in &clustered {
                let alphabet = histo.alphabet_size().max(1);
                if let Some(w) = writer.as_deref_mut() {
                    store_var_len_uint16(alphabet - 1, w);
                } else {
                    store_var_len_uint16(alphabet - 1, &mut size_writer);
                }
            }
        }
        cost += size_writer.size;
        for histogram in &clustered[prev_histograms..] {
            let alphabet_size = histogram.alphabet_size();
            self.encoding_info.push(
                (0..alphabet_size)
                    .map(|_| ANSEncSymbolInfo::default())
                    .collect(),
            );
            if params.streaming_mode {
                // The source appends this writer before encoding. Keep it even
                // on error, without cloning all encoding data to split borrows.
                let mut histo_writer = BitWriter::new();
                let status = histo_writer.with_max_bits(
                    256 + alphabet_size * 24,
                    layer,
                    aux.as_deref_mut(),
                    |w, _| {
                        cost += self.build_and_store_ans_encoding_data(
                            params.ans_histogram_strategy,
                            histogram,
                            Some(w),
                        )?;
                        Ok(())
                    },
                    true,
                );
                self.encoded_histograms.push(histo_writer);
                status?;
                if let Some(w) = writer.as_deref_mut() {
                    w.append_unaligned(self.encoded_histograms.last().unwrap())?;
                }
            } else if let Some(w) = writer.as_deref_mut() {
                w.with_max_bits(
                    256 + alphabet_size * 24,
                    layer,
                    aux.as_deref_mut(),
                    |w, _| {
                        cost += self.build_and_store_ans_encoding_data(
                            params.ans_histogram_strategy,
                            histogram,
                            Some(w),
                        )?;
                        Ok(())
                    },
                    true,
                )?;
            } else {
                cost += self.build_and_store_ans_encoding_data(
                    params.ans_histogram_strategy,
                    histogram,
                    None,
                )?;
            }
        }
        Ok(cost)
    }
}

pub fn encode_histograms(
    codes: &mut EntropyEncodingData,
    writer: &mut BitWriter,
    layer: LayerType,
    aux: Option<&mut AuxOut>,
) -> Status {
    writer.with_max_bits(
        128 + CLUSTERS_LIMIT * 136,
        layer,
        aux,
        |w, mut aux| {
            bundle::write(&mut codes.lz77, w, layer, aux.as_deref_mut())?;
            if codes.lz77.enabled {
                encode_uint_config(codes.lz77.length_uint_config, w, 8);
            }
            encode_context_map(&codes.context_map, codes.encoding_info.len(), w, layer, aux)?;
            w.write(1, u64::from(codes.use_prefix_code));
            let log_alpha_size = if codes.use_prefix_code {
                PREFIX_MAX_BITS
            } else {
                w.write(2, 3);
                8
            };
            encode_uint_configs(&codes.uint_config, w, log_alpha_size);
            if codes.use_prefix_code {
                for info in &codes.encoding_info {
                    store_var_len_uint16(info.len().wrapping_sub(1), w);
                }
            }
            for histo in &codes.encoded_histograms {
                w.append_unaligned(histo)?;
            }
            Ok(())
        },
        true,
    )
}

pub fn build_and_encode_histograms(
    params: &HistogramParams,
    mut num_contexts: usize,
    tokens: &mut Vec<Vec<Token>>,
    codes: &mut EntropyEncodingData,
    writer: Option<&mut BitWriter>,
    layer: LayerType,
    aux: Option<&mut AuxOut>,
) -> StatusOr<usize> {
    codes.lz77.nonserialized_distance_context = num_contexts;
    codes.lz77.min_symbol = if params.force_huffman { 512 } else { 224 };
    let mut compressed = apply_lz77(params, num_contexts, tokens, &codes.lz77);
    if !compressed.is_empty() {
        codes.lz77.enabled = true;
    }
    let mut cost = 0;
    let max_contexts = num_contexts.min(CLUSTERS_LIMIT);
    let max_bits = 128 + num_contexts * 40 + max_contexts * 96;
    let mut body = |mut writer: Option<&mut BitWriter>, mut aux: Option<&mut AuxOut>| -> Status {
        if let Some(w) = writer.as_deref_mut() {
            bundle::write(&mut codes.lz77, w, layer, aux.as_deref_mut())?;
        } else {
            let (_, bits) = super::fields::bundle::can_encode(&mut codes.lz77)?;
            cost += bits;
        }
        if codes.lz77.enabled {
            if let Some(w) = writer.as_deref_mut() {
                let start = w.bits_written();
                encode_uint_config(codes.lz77.length_uint_config, w, 8);
                cost += w.bits_written() - start;
            } else {
                let mut size_writer = SizeWriter::default();
                encode_uint_config_to(codes.lz77.length_uint_config, &mut size_writer, 8);
                cost += size_writer.size;
            }
            num_contexts += 1;
            debug_assert!(!compressed.is_empty());
            *tokens = std::mem::take(&mut compressed);
        }
        let mut total_tokens = 0;
        let mut builder = vec![Histogram::default(); num_contexts];
        let cfg = params.uint_config();
        for stream in &*tokens {
            if codes.lz77.enabled {
                for token in stream {
                    total_tokens += 1;
                    let config = if token.is_lz77_length() {
                        codes.lz77.length_uint_config
                    } else {
                        cfg
                    };
                    let (mut symbol, _, _) = config.encode(token.value);
                    if token.is_lz77_length() {
                        symbol = symbol.wrapping_add(codes.lz77.min_symbol);
                    }
                    builder[token.context() as usize].add(symbol as usize);
                }
            } else if num_contexts == 1 {
                for token in stream {
                    total_tokens += 1;
                    let (symbol, _, _) = cfg.encode(token.value);
                    builder[0].add(symbol as usize);
                }
            } else {
                for token in stream {
                    total_tokens += 1;
                    let (symbol, _, _) = cfg.encode(token.value);
                    builder[token.context() as usize].add(symbol as usize);
                }
            }
        }
        if params.add_missing_symbols {
            for histogram in &mut builder {
                for symbol in 0..ANS_MAX_ALPHABET_SIZE {
                    histogram.add(symbol);
                }
            }
        }
        if params.initialize_global_state {
            let mut prefix = params.force_huffman
                || total_tokens < 100
                || params.clustering == ClusteringType::Fastest;
            if !prefix {
                let mut all_singleton = true;
                for histogram in &mut builder {
                    if histogram.shannon_entropy() >= 1e-5 {
                        all_singleton = false;
                    }
                }
                if all_singleton {
                    prefix = true;
                }
            }
            codes.use_prefix_code = prefix;
        }
        if params.add_fixed_histograms {
            codes.log_alpha_size = 8;
            if ANS_MAX_ALPHABET_SIZE != 1usize << codes.log_alpha_size {
                return Err(StatusCode::GenericError);
            }
            codes.encoding_info.push(
                (0..ANS_MAX_ALPHABET_SIZE)
                    .map(|_| ANSEncSymbolInfo::default())
                    .collect(),
            );
            let mut histo_writer = BitWriter::new();
            let status = histo_writer.with_max_bits(
                256 + ANS_MAX_ALPHABET_SIZE * 24,
                LayerType::Header,
                None,
                |w, _| {
                    codes.build_and_store_ans_encoding_data(
                        params.ans_histogram_strategy,
                        &Histogram::flat(ANS_MAX_ALPHABET_SIZE as i32, ANS_TAB_SIZE as i32),
                        Some(w),
                    )?;
                    Ok(())
                },
                false,
            );
            codes.encoded_histograms.push(histo_writer);
            status?;
        }
        cost += codes.build_and_store_entropy_codes(
            params,
            tokens,
            &mut builder,
            writer,
            layer,
            aux,
        )?;
        Ok(())
    };
    let mut aux = aux;
    if let Some(w) = writer {
        w.with_max_bits(
            max_bits,
            layer,
            aux.as_deref_mut(),
            |w, aux| body(Some(w), aux),
            true,
        )?;
    } else {
        body(None, aux.as_deref_mut())?;
    }
    if let Some(aux) = aux {
        aux.layer_mut(layer).num_clustered_histograms += codes.encoding_info.len();
    }
    Ok(cost)
}
