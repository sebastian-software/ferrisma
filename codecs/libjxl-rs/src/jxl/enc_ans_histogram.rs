// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ANSEncodingHistogram from the anonymous namespace of `lib/jxl/enc_ans.cc`.
//! OnceLock-owned arrays replace C++ static initialization. Generic writer
//! bounds replace Writer templates. Keep first-winner ties and numeric order.

use super::ans_common::{AliasTable, Entry, create_flat_histogram, get_population_count_precision};
use super::ans_params::{ANS_LOG_TAB_SIZE, ANS_MAX_ALPHABET_SIZE, ANS_TAB_SIZE};
use super::base::bits::floor_log2_nonzero;
use super::base::status::{Status, StatusCode, StatusOr};
use super::enc_ans::{ANSEncSymbolInfo, RECIPROCAL_PRECISION, SizeWriter, Writer};
use super::enc_ans_params::{ANSHistogramStrategy, Histogram};
use std::sync::OnceLock;

const TABLE_SIZE: usize = ANS_TAB_SIZE as usize;
const LOG_TABLE_SIZE: usize = ANS_LOG_TAB_SIZE as usize;
const MAX_SMALL: usize = 2;

fn lg2() -> &'static [u32; TABLE_SIZE + 1] {
    static LG2: OnceLock<[u32; TABLE_SIZE + 1]> = OnceLock::new();
    LG2.get_or_init(|| {
        let mut values = [0; TABLE_SIZE + 1];
        for (i, value) in values.iter_mut().enumerate().skip(1) {
            *value =
                ((i as f64).log2() / f64::from(ANS_LOG_TAB_SIZE) * 2147483648.0).round() as u32;
        }
        values
    })
}

#[derive(Clone, Copy, Default)]
struct CountsEntropy {
    count: i16,
    step_log: i16,
    delta_lg2: i32,
}
struct AllowedCounts {
    array: Vec<CountsEntropy>,
    index: Vec<u16>,
}

fn smallest_increment_log(count: u32, shift: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let bits = floor_log2_nonzero(count) as u32;
    bits - get_population_count_precision(bits, shift)
}

fn allowed_counts() -> &'static AllowedCounts {
    static ALLOWED: OnceLock<AllowedCounts> = OnceLock::new();
    ALLOWED.get_or_init(|| {
        let mut result = AllowedCounts {
            array: vec![CountsEntropy::default(); LOG_TABLE_SIZE * TABLE_SIZE],
            index: vec![0; LOG_TABLE_SIZE * TABLE_SIZE],
        };
        for shift in 0..LOG_TABLE_SIZE {
            let ac = &mut result.array[shift * TABLE_SIZE..(shift + 1) * TABLE_SIZE];
            let ai = &mut result.index[shift * TABLE_SIZE..(shift + 1) * TABLE_SIZE];
            let mut last = -1;
            let mut slot = 0;
            ac[0].delta_lg2 = 0;
            ac[0].step_log = 0;
            for i in (0..TABLE_SIZE as i32).rev() {
                let curr = i & !((1 << smallest_increment_log(i as u32, shift as u32)) - 1);
                if curr == last {
                    continue;
                }
                last = curr;
                ac[slot].count = curr as i16;
                ai[curr as usize] = slot as u16;
                if curr == 0 {
                    ac[slot].delta_lg2 = i32::MAX;
                    ac[slot].step_log = 0;
                } else if slot > 0 {
                    let prev = i32::from(ac[slot - 1].count);
                    ac[slot].delta_lg2 = ((f64::from(prev) / f64::from(curr)).log2()
                        / f64::from(ANS_LOG_TAB_SIZE)
                        * 2147483648.0)
                        .round() as i32;
                    ac[slot].step_log = floor_log2_nonzero((prev - curr) as u32) as i16;
                }
                slot += 1;
            }
        }
        result
    })
}

fn store_var_len_uint8(n: usize, writer: &mut impl Writer) {
    debug_assert!(n <= 255);
    if n == 0 {
        writer.write(1, 0);
    } else {
        writer.write(1, 1);
        let nbits = floor_log2_nonzero(n);
        writer.write(3, nbits as u64);
        writer.write(nbits, (n - (1usize << nbits)) as u64);
    }
}

#[derive(Clone, Default)]
pub(crate) struct ANSEncodingHistogram {
    cost: f32,
    method: u32,
    omit_pos: usize,
    alphabet_size: usize,
    num_symbols: usize,
    symbols: [usize; MAX_SMALL],
    counts: Vec<i32>,
}

impl ANSEncodingHistogram {
    pub(crate) fn counts(&self) -> &[i32] {
        &self.counts
    }
    pub(crate) fn cost(&self) -> f32 {
        self.cost
    }

    pub(crate) fn compute_best(
        histo: &Histogram,
        strategy: ANSHistogramStrategy,
    ) -> StatusOr<Self> {
        let mut result = Self {
            alphabet_size: histo.alphabet_size(),
            ..Self::default()
        };
        if result.alphabet_size > ANS_MAX_ALPHABET_SIZE {
            return Err(StatusCode::GenericError);
        }
        if result.alphabet_size > 0 {
            result.method = 0;
            result.num_symbols = result.alphabet_size;
            result.counts = create_flat_histogram(result.alphabet_size as i32, ANS_TAB_SIZE as i32);
            result.counts.resize(histo.counts.len(), 0);
            let mut writer = SizeWriter::default();
            result.encode(&mut writer)?;
            result.cost = writer.size as f32 + Self::estimate_data_bits_flat(histo);
        } else {
            result.method = 1;
            result.num_symbols = 0;
            result.cost = 3.0;
            return Ok(result);
        }
        let mut symbol_count = 0;
        for n in 0..result.alphabet_size {
            if histo.counts[n] > 0 {
                if symbol_count < MAX_SMALL {
                    result.symbols[symbol_count] = n;
                }
                symbol_count += 1;
            }
        }
        result.num_symbols = symbol_count;
        if symbol_count == 1 {
            result.method = 1;
            result.counts = histo.counts.clone();
            result.counts[result.symbols[0]] = ANS_TAB_SIZE as i32;
            let mut writer = SizeWriter::default();
            result.encode(&mut writer)?;
            result.cost = writer.size as f32;
            return Ok(result);
        }
        let mut normalized = result.clone();
        let mut try_shift = |shift: u32| -> Status {
            normalized.method = shift.min(ANS_LOG_TAB_SIZE - 1) + 1;
            if !normalized.rebalance_histogram(histo) {
                return Err(StatusCode::GenericError);
            }
            let mut writer = SizeWriter::default();
            normalized.encode(&mut writer)?;
            normalized.cost = writer.size as f32 + normalized.estimate_data_bits(histo);
            if normalized.cost < result.cost {
                result = normalized.clone();
            }
            Ok(())
        };
        match strategy {
            ANSHistogramStrategy::Precise => {
                for shift in 0..ANS_LOG_TAB_SIZE {
                    try_shift(shift)?;
                }
            }
            ANSHistogramStrategy::Approximate => {
                for shift in (0..=ANS_LOG_TAB_SIZE).step_by(2) {
                    try_shift(shift)?;
                }
            }
            ANSHistogramStrategy::Fast => {
                try_shift(0)?;
                try_shift(ANS_LOG_TAB_SIZE / 2)?;
                try_shift(ANS_LOG_TAB_SIZE)?;
            }
        }
        Ok(result)
    }

    pub(crate) fn encode(&self, writer: &mut impl Writer) -> Status {
        if self.alphabet_size > ANS_MAX_ALPHABET_SIZE {
            return Err(StatusCode::GenericError);
        }
        if self.method == 0 {
            writer.write(1, 0);
            writer.write(1, 1);
            if self.alphabet_size == 0 {
                return Err(StatusCode::GenericError);
            }
            store_var_len_uint8(self.alphabet_size - 1, writer);
            return Ok(());
        }
        if self.num_symbols <= MAX_SMALL {
            writer.write(1, 1);
            if self.num_symbols == 0 {
                writer.write(1, 0);
                store_var_len_uint8(0, writer);
            } else {
                writer.write(1, (self.num_symbols - 1) as u64);
                for &symbol in &self.symbols[..self.num_symbols] {
                    store_var_len_uint8(symbol, writer);
                }
            }
            if self.num_symbols == 2 {
                writer.write(
                    ANS_LOG_TAB_SIZE as usize,
                    self.counts[self.symbols[0]] as u64,
                );
            }
            return Ok(());
        }
        writer.write(1, 0);
        writer.write(1, 0);
        let upper_bound_log = floor_log2_nonzero(ANS_LOG_TAB_SIZE + 1);
        let log = floor_log2_nonzero(self.method);
        writer.write(log, (1u64 << log) - 1);
        if log != upper_bound_log {
            writer.write(1, 0);
        }
        writer.write(log, ((1u64 << log) - 1) & u64::from(self.method));
        store_var_len_uint8(self.alphabet_size - 3, writer);
        let mut same = [0u8; ANS_MAX_ALPHABET_SIZE];
        let mut last = 0;
        for i in 1..=self.alphabet_size {
            if i == self.alphabet_size
                || i == self.omit_pos
                || i == self.omit_pos + 1
                || self.counts[i] != self.counts[last]
            {
                same[last] = (i - last) as u8;
                last = i;
            }
        }
        let mut bit_width = [0u8; ANS_MAX_ALPHABET_SIZE];
        let mut omit_width = 10;
        for (i, width) in bit_width.iter_mut().enumerate().take(self.alphabet_size) {
            if i != self.omit_pos && self.counts[i] > 0 {
                *width = (floor_log2_nonzero(self.counts[i] as u32) + 1) as u8;
                omit_width = omit_width.max(i32::from(*width) + i32::from(i < self.omit_pos));
            }
        }
        bit_width[self.omit_pos] = omit_width as u8;
        const BIT_WIDTH_LENGTHS: [usize; 14] = [5, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 6, 7, 7];
        const BIT_WIDTH_SYMBOLS: [u64; 14] = [17, 11, 15, 3, 9, 7, 4, 2, 5, 6, 0, 33, 1, 65];
        const MIN_REPS: u8 = 5;
        let rep = LOG_TABLE_SIZE + 1;
        let mut i = 0;
        while i < self.alphabet_size {
            writer.write(
                BIT_WIDTH_LENGTHS[bit_width[i] as usize],
                BIT_WIDTH_SYMBOLS[bit_width[i] as usize],
            );
            if same[i] >= MIN_REPS {
                writer.write(BIT_WIDTH_LENGTHS[rep], BIT_WIDTH_SYMBOLS[rep]);
                store_var_len_uint8((same[i] - MIN_REPS) as usize, writer);
                i += usize::from(same[i]) - 1;
            }
            i += 1;
        }
        let shift = self.method - 1;
        if shift != 0 {
            let mut i = 0;
            while i < self.alphabet_size {
                if bit_width[i] > 1 && i != self.omit_pos {
                    let bitcount =
                        get_population_count_precision(u32::from(bit_width[i]) - 1, shift);
                    let drop_bits = u32::from(bit_width[i]) - 1 - bitcount;
                    debug_assert_eq!(self.counts[i] & ((1 << drop_bits) - 1), 0);
                    writer.write(
                        bitcount as usize,
                        ((self.counts[i] >> drop_bits) - (1 << bitcount)) as u64,
                    );
                }
                if same[i] >= MIN_REPS {
                    i += usize::from(same[i]) - 1;
                }
                i += 1;
            }
        }
        Ok(())
    }

    pub(crate) fn ans_build_info_table(
        &self,
        table: &[Entry],
        log_alpha_size: usize,
        info: &mut [ANSEncSymbolInfo],
    ) {
        for (s, symbol) in info.iter_mut().enumerate().take(self.alphabet_size.max(1)) {
            let freq = if s == self.alphabet_size {
                ANS_TAB_SIZE as i32
            } else {
                self.counts[s]
            };
            symbol.freq = freq as u16;
            symbol.ifreq = if freq != 0 {
                ((1u64 << RECIPROCAL_PRECISION) + u64::from(symbol.freq) - 1)
                    / u64::from(symbol.freq)
            } else {
                1
            };
            symbol.reverse_map.resize(freq as usize, 0);
        }
        let log_entry_size = LOG_TABLE_SIZE - log_alpha_size;
        let entry_size_minus_1 = (1 << log_entry_size) - 1;
        for i in 0..TABLE_SIZE {
            let s = AliasTable::lookup(table, i, log_entry_size, entry_size_minus_1);
            info[s.value].reverse_map[s.offset] = i as u16;
        }
    }

    fn estimate_data_bits(&self, histo: &Histogram) -> f32 {
        let mut sum = 0i64;
        for i in 0..self.alphabet_size {
            sum += i64::from(histo.counts[i]) * i64::from(lg2()[self.counts[i] as usize]);
        }
        (histo.total_count as f32 - sum as f32 * (1.0 / 2147483648.0)) * ANS_LOG_TAB_SIZE as f32
    }
    fn estimate_data_bits_flat(histo: &Histogram) -> f32 {
        let flat_bits = i64::from(lg2()[histo.alphabet_size()]) * i64::from(ANS_LOG_TAB_SIZE);
        (histo.total_count as u64).wrapping_mul(flat_bits as u64) as f32 * (1.0 / 2147483648.0)
    }

    fn rebalance_histogram(&mut self, histo: &Histogram) -> bool {
        struct EntropyDelta {
            freq: i32,
            count_ind: usize,
            bin_ind: usize,
        }
        let shift = self.method as usize - 1;
        let mut balance_inc = [0i64; LOG_TABLE_SIZE - 1];
        let mut balance_dec = [0i64; LOG_TABLE_SIZE - 1];
        let ac = &allowed_counts().array[shift * TABLE_SIZE..(shift + 1) * TABLE_SIZE];
        let ai = &allowed_counts().index[shift * TABLE_SIZE..(shift + 1) * TABLE_SIZE];
        let delta_inc = |a: &EntropyDelta, balance: &[i64; LOG_TABLE_SIZE - 1]| {
            i64::from(a.freq) * i64::from(ac[a.count_ind].delta_lg2)
                - balance[ac[a.count_ind].step_log as usize]
        };
        let delta_dec = |a: &EntropyDelta, balance: &[i64; LOG_TABLE_SIZE - 1]| {
            i64::from(a.freq) * i64::from(ac[a.count_ind + 1].delta_lg2)
                - balance[ac[a.count_ind + 1].step_log as usize]
        };
        let mut bins = Vec::with_capacity(256);
        let norm = TABLE_SIZE as f64 / histo.total_count as f64;
        let mut remainder_pos = 0;
        let mut max_freq = 0i64;
        let mut rest = TABLE_SIZE as i32;
        for n in 0..self.alphabet_size {
            let freq = histo.counts[n];
            if i64::from(freq) > max_freq {
                remainder_pos = n;
                max_freq = i64::from(freq);
            }
            let target = f64::from(freq) * norm;
            let mut count = (target.round() as i32).max(i32::from(freq > 0));
            count = count.min(TABLE_SIZE as i32 - 1);
            let step_log = smallest_increment_log(count as u32, shift as u32);
            let inc = 1 << step_log;
            count &= !(inc - 1);
            self.counts[n] = count;
            rest -= count;
            if target > 1.0 {
                bins.push(EntropyDelta {
                    freq,
                    count_ind: ai[count as usize] as usize,
                    bin_ind: n,
                });
            }
        }
        bins.remove(
            bins.iter()
                .position(|a| a.bin_ind == remainder_pos)
                .expect("balancing bin"),
        );
        rest += self.counts[remainder_pos];
        if !bins.is_empty() {
            let max_log = ac[1].step_log as usize;
            loop {
                for log in 0..=max_log {
                    let delta = 1 << log;
                    if rest >= TABLE_SIZE as i32 {
                        balance_inc[log] = 0;
                        balance_dec[log] = 0;
                    } else if rest > 1 {
                        balance_inc[log] = if rest > delta {
                            max_freq
                                * i64::from(lg2()[rest as usize] - lg2()[(rest - delta) as usize])
                        } else {
                            i64::MAX
                        };
                        balance_dec[log] = if rest + delta < TABLE_SIZE as i32 {
                            max_freq
                                * i64::from(lg2()[(rest + delta) as usize] - lg2()[rest as usize])
                        } else {
                            0
                        };
                    } else {
                        balance_inc[log] = i64::MAX;
                        balance_dec[log] = i64::MAX;
                    }
                }
                // std::max_element/min_element keep the first equal winner.
                let mut best_inc = 0;
                for i in 1..bins.len() {
                    let a = &bins[best_inc];
                    let b = &bins[i];
                    if delta_inc(a, &balance_inc) >> ac[a.count_ind].step_log
                        < delta_inc(b, &balance_inc) >> ac[b.count_ind].step_log
                    {
                        best_inc = i;
                    }
                }
                if delta_inc(&bins[best_inc], &balance_inc) > 0 {
                    rest -= 1 << ac[bins[best_inc].count_ind].step_log;
                    bins[best_inc].count_ind -= 1;
                } else {
                    let mut best_dec = 0;
                    for i in 1..bins.len() {
                        let a = &bins[i];
                        let b = &bins[best_dec];
                        if delta_dec(a, &balance_dec) >> ac[a.count_ind + 1].step_log
                            < delta_dec(b, &balance_dec) >> ac[b.count_ind + 1].step_log
                        {
                            best_dec = i;
                        }
                    }
                    if delta_dec(&bins[best_dec], &balance_dec) >= 0 {
                        break;
                    }
                    bins[best_dec].count_ind += 1;
                    rest += 1 << ac[bins[best_dec].count_ind].step_log;
                }
            }
            for a in &bins {
                self.counts[a.bin_ind] = i32::from(ac[a.count_ind].count);
            }
            for n in 0..remainder_pos {
                if self.counts[n] >= 2048 {
                    self.counts[remainder_pos] = self.counts[n];
                    remainder_pos = n;
                    break;
                }
            }
        }
        self.counts[remainder_pos] = rest;
        self.omit_pos = remainder_pos;
        self.counts[remainder_pos] > 0
    }
}
