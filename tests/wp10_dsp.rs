//! WP10 differential test: the encoder DSP functions of `src/dsp/enc.c` and
//! `src/dsp/quant.h`. The C probe `tools/oracle/probes/wp10/dsp.c` and the Rust
//! translation apply the same operations to the same seeded blocks: the
//! forward and inverse transforms, the Walsh-Hadamard transform, the squared
//! errors and the weighted texture distortion, the means, the coefficient
//! histograms, the three quantizers, the intra predictions with and without
//! their left and top samples, the block copies and the flatness tests.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dsp::dsp::{BPS, MAX_COEFF_THRESH, VP8Histogram};
use webp::dsp::enc::{
    disto4x4_c, disto16x16_c, i_transform_c, vp8_collect_histogram, vp8_copy4x4, vp8_copy16x8,
    vp8_enc_pred_chroma8, vp8_enc_pred_luma4, vp8_enc_pred_luma16, vp8_enc_quantize_block,
    vp8_enc_quantize_block_wht, vp8_enc_quantize2_blocks, vp8_mean16x4, vp8_set_histogram_data,
    vp8f_transform, vp8f_transform_wht, vp8f_transform2, vp8sse4x4, vp8sse8x8, vp8sse16x8,
    vp8sse16x16,
};
use webp::dsp::quant::{is_flat, is_flat_source16};
use webp::enc::vp8i_enc::{PRED_SIZE_ENC, VP8Matrix};
use wp11_images::Rng;

const AREA: usize = 16 * BPS as usize;
const BPS_U: usize = BPS as usize;

const OP_FTRANSFORM: i32 = 0;
const OP_FTRANSFORM2: i32 = 1;
const OP_FTRANSFORM_WHT: i32 = 2;
const OP_ITRANSFORM: i32 = 3;
const OP_SSE: i32 = 4;
const OP_TDISTO: i32 = 5;
const OP_MEAN16X4: i32 = 6;
const OP_HISTOGRAM: i32 = 7;
const OP_QUANTIZE: i32 = 8;
const OP_PRED16: i32 = 9;
const OP_PREDC8: i32 = 10;
const OP_PRED4: i32 = 11;
const OP_COPY: i32 = 12;
const OP_IS_FLAT: i32 = 13;
const OP_SET_HISTOGRAM_DATA: i32 = 14;

#[derive(Clone)]
struct Record {
    op: i32,
    p: [i32; 4],
    a: Vec<u8>,
    b: Vec<u8>,
    d: Vec<u8>,
    input: [i16; 256],
    w: [u16; 16],
    mtx: VP8Matrix,
}

impl Record {
    fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.op.to_le_bytes());
        for value in self.p {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out.extend_from_slice(&self.a);
        out.extend_from_slice(&self.b);
        out.extend_from_slice(&self.d);
        for value in self.input {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in self.w {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in self.mtx.q.iter().chain(&self.mtx.iq) {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in self.mtx.bias.iter().chain(&self.mtx.zthresh) {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for value in self.mtx.sharpen {
            out.extend_from_slice(&value.to_le_bytes());
        }
    }

    /// What the probe prints for the record.
    fn expected(&self) -> String {
        let mut r = String::from("r=");
        let i16s = |r: &mut String, v: &[i16]| {
            for value in v {
                write!(r, "{:04x}", *value as u16).unwrap();
            }
        };
        let i32s = |r: &mut String, v: i32| write!(r, "{:08x}", v as u32).unwrap();
        let bytes = |r: &mut String, v: &[u8]| {
            for value in v {
                write!(r, "{value:02x}").unwrap();
            }
        };
        match self.op {
            OP_FTRANSFORM => {
                let mut out = [0_i16; 16];
                vp8f_transform(&self.a, &self.b, &mut out);
                i16s(&mut r, &out);
            }
            OP_FTRANSFORM2 => {
                let mut out = [0_i16; 32];
                vp8f_transform2(&self.a, &self.b, &mut out);
                i16s(&mut r, &out);
            }
            OP_FTRANSFORM_WHT => {
                let mut out = [0_i16; 16];
                vp8f_transform_wht(&self.input, &mut out);
                i16s(&mut r, &out);
            }
            OP_ITRANSFORM => {
                // The C translation `ITransform_C`, which the case verifies:
                // with `simd` on 64-bit ARM the dispatch selects upstream's
                // NEON kernel, which saturates at 16 bits and differs from C
                // for the coefficients above about 5,000 in magnitude that
                // these records contain and the encoder never passes (WP20;
                // `tests/wp20_enc.rs` covers the NEON kernel).
                let mut d = self.d.clone();
                i_transform_c(&self.a, &self.input, &mut d, self.p[0] != 0);
                bytes(&mut r, &d[..4 * BPS_U]);
            }
            OP_SSE => {
                i32s(&mut r, vp8sse16x16(&self.a, &self.b));
                i32s(&mut r, vp8sse16x8(&self.a, &self.b));
                i32s(&mut r, vp8sse8x8(&self.a, &self.b));
                i32s(&mut r, vp8sse4x4(&self.a, &self.b));
            }
            OP_TDISTO => {
                // The C translations `Disto4x4_C` and `Disto16x16_C`, which
                // the case verifies: with `simd` on 64-bit ARM the dispatch
                // selects upstream's NEON kernels, which sum in transposed
                // order and equal C only for the symmetric weights that
                // `VP8TDisto4x4` documents and the encoder passes, not for
                // these random ones (WP20; `tests/wp20_enc.rs` covers them).
                i32s(&mut r, disto4x4_c(&self.a, &self.b, &self.w));
                i32s(&mut r, disto16x16_c(&self.a, &self.b, &self.w));
            }
            OP_MEAN16X4 => {
                let mut dc = [0_u32; 4];
                vp8_mean16x4(&self.a, &mut dc);
                for value in dc {
                    i32s(&mut r, value as i32);
                }
            }
            OP_HISTOGRAM => {
                let mut histo = VP8Histogram {
                    max_value: 0,
                    last_non_zero: 1,
                };
                vp8_collect_histogram(&self.a, &self.b, self.p[0], self.p[1], &mut histo);
                i32s(&mut r, histo.max_value);
                i32s(&mut r, histo.last_non_zero);
            }
            OP_QUANTIZE => {
                let mut input = [0_i16; 32];
                let mut out = [0_i16; 32];
                input.copy_from_slice(&self.input[..32]);
                let ret = vp8_enc_quantize_block(&mut input, &mut out, &self.mtx);
                i32s(&mut r, ret);
                i16s(&mut r, &input[..16]);
                i16s(&mut r, &out[..16]);
                input.copy_from_slice(&self.input[..32]);
                let ret = vp8_enc_quantize2_blocks(&mut input, &mut out, &self.mtx);
                i32s(&mut r, ret);
                i16s(&mut r, &input);
                i16s(&mut r, &out);
                input.copy_from_slice(&self.input[..32]);
                let ret = vp8_enc_quantize_block_wht(&mut input, &mut out, &self.mtx);
                i32s(&mut r, ret);
                i16s(&mut r, &input[..16]);
                i16s(&mut r, &out[..16]);
            }
            OP_PRED16 | OP_PREDC8 => {
                let mut dst = vec![0x5a_u8; PRED_SIZE_ENC as usize];
                let left = (self.p[0] != 0).then_some((self.b.as_slice(), self.p[2] as usize));
                let top = (self.p[1] != 0).then(|| &self.b[self.p[3] as usize..]);
                if self.op == OP_PRED16 {
                    vp8_enc_pred_luma16(&mut dst, left, top);
                } else {
                    vp8_enc_pred_chroma8(&mut dst, left, top);
                }
                bytes(&mut r, &dst);
            }
            OP_PRED4 => {
                let mut dst = vec![0x5a_u8; PRED_SIZE_ENC as usize];
                vp8_enc_pred_luma4(&mut dst, &self.b, self.p[0] as usize);
                bytes(&mut r, &dst);
            }
            OP_COPY => {
                let mut dst = self.d.clone();
                vp8_copy4x4(&self.a, &mut dst);
                bytes(&mut r, &dst[..4 * BPS_U]);
                let mut dst = self.d.clone();
                vp8_copy16x8(&self.a, &mut dst);
                bytes(&mut r, &dst[..8 * BPS_U]);
            }
            OP_IS_FLAT => {
                i32s(
                    &mut r,
                    i32::from(is_flat(&self.input, self.p[0], self.p[1])),
                );
                i32s(&mut r, i32::from(is_flat_source16(&self.a)));
            }
            OP_SET_HISTOGRAM_DATA => {
                let mut distribution = [0_i32; MAX_COEFF_THRESH as usize + 1];
                for (entry, &value) in distribution.iter_mut().zip(&self.input) {
                    *entry = i32::from(value);
                }
                let mut histo = VP8Histogram::default();
                vp8_set_histogram_data(&distribution, &mut histo);
                i32s(&mut r, histo.max_value);
                i32s(&mut r, histo.last_non_zero);
            }
            _ => unreachable!(),
        }
        r
    }
}

/// A block of samples: random, smooth, flat, or extreme.
fn samples(rng: &mut Rng) -> Vec<u8> {
    match rng.below(6) {
        0 => vec![0; AREA],
        1 => vec![255; AREA],
        2 => {
            let v = rng.below(256) as u8;
            vec![v; AREA]
        }
        3 => (0..AREA)
            .map(|i| ((i % BPS_U) * 7 + (i / BPS_U) * 5) as u8)
            .collect(),
        4 => (0..AREA)
            .map(|i| {
                if (i + i / BPS_U).is_multiple_of(2) {
                    0
                } else {
                    255
                }
            })
            .collect(),
        _ => (0..AREA).map(|_| rng.below(256) as u8).collect(),
    }
}

/// A quantization matrix as ExpandMatrix builds it, from random steps.
fn matrix(rng: &mut Rng) -> VP8Matrix {
    let mut m = VP8Matrix::default();
    let dc = 4 + rng.below(300) as u16;
    let ac = 4 + rng.below(300) as u16;
    let biases = [96_u32, 110, 108, 115];
    for i in 0..16 {
        m.q[i] = if i == 0 { dc } else { ac };
        m.iq[i] = ((1_u32 << 17) / u32::from(m.q[i])) as u16;
        m.bias[i] = biases[rng.below(4) as usize] << 9;
        m.zthresh[i] = ((1_u32 << 17) - 1 - m.bias[i]) / u32::from(m.iq[i]);
        m.sharpen[i] = if rng.below(2) == 0 {
            0
        } else {
            rng.below(40) as u16
        };
    }
    m
}

fn record(rng: &mut Rng, op: i32) -> Record {
    let mut input = [0_i16; 256];
    let range = [16, 256, 2048, 4096, 32768][rng.below(5) as usize];
    for value in &mut input {
        *value = match rng.below(4) {
            0 => 0,
            _ => (rng.below(2 * range) as i64 - range as i64) as i16,
        };
    }
    let mut w = [0_u16; 16];
    for value in &mut w {
        *value = rng.below(64) as u16;
    }
    Record {
        op,
        p: [0; 4],
        a: samples(rng),
        b: samples(rng),
        d: samples(rng),
        input,
        w,
        mtx: matrix(rng),
    }
}

#[test]
fn encoder_dsp_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("encoder_dsp_matches_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0x0010_d5b0_0001);
    let mut records = Vec::new();
    for op in [
        OP_FTRANSFORM,
        OP_FTRANSFORM2,
        OP_FTRANSFORM_WHT,
        OP_SSE,
        OP_TDISTO,
        OP_MEAN16X4,
        OP_QUANTIZE,
        OP_COPY,
    ] {
        for _ in 0..150 {
            records.push(record(&mut rng, op));
        }
    }
    for _ in 0..150 {
        let mut r = record(&mut rng, OP_ITRANSFORM);
        r.p[0] = rng.below(2) as i32;
        // Above about 15,700 in magnitude the int products of the horizontal
        // pass overflow, which is undefined in C (see the status page); the
        // encoder's dequantized levels stay far below.
        let bound = [16_i64, 256, 2048, 8192, 15_000][rng.below(5) as usize];
        for value in &mut r.input {
            *value = (rng.below(2 * bound as u64 + 1) as i64 - bound) as i16;
        }
        records.push(r);
    }
    for _ in 0..200 {
        let mut r = record(&mut rng, OP_HISTOGRAM);
        let start = rng.below(24) as i32;
        r.p[0] = start;
        r.p[1] = start + rng.below(25 - start as u64) as i32;
        records.push(r);
    }
    for op in [OP_PRED16, OP_PREDC8] {
        for left in 0..=1 {
            for top in 0..=1 {
                for _ in 0..25 {
                    let mut r = record(&mut rng, op);
                    r.p = [left, top, 1 + rng.below(64) as i32, rng.below(64) as i32];
                    records.push(r);
                }
            }
        }
    }
    for _ in 0..150 {
        let mut r = record(&mut rng, OP_PRED4);
        r.p[0] = 5 + rng.below(100) as i32;
        records.push(r);
    }
    for _ in 0..200 {
        let mut r = record(&mut rng, OP_IS_FLAT);
        // Sparse levels, so that some blocks are flat.
        for value in &mut r.input {
            if rng.below(8) != 0 {
                *value = 0;
            }
        }
        r.p[0] = 1 + rng.below(16) as i32;
        r.p[1] = rng.below(6) as i32;
        if rng.below(3) == 0 {
            let v = rng.below(256) as u8;
            r.a = vec![v; AREA];
            if rng.below(2) == 0 {
                let i = rng.below(16) as usize * BPS_U + rng.below(16) as usize;
                r.a[i] ^= 1;
            }
        }
        records.push(r);
    }
    for _ in 0..100 {
        let mut r = record(&mut rng, OP_SET_HISTOGRAM_DATA);
        for value in r.input.iter_mut().take(MAX_COEFF_THRESH as usize + 1) {
            *value = if rng.below(3) == 0 {
                0
            } else {
                rng.below(40) as i16 - 4
            };
        }
        records.push(r);
    }

    let mut stdin = Vec::new();
    for r in &records {
        r.write(&mut stdin);
    }
    let output = oracle.probe("wp10", "dsp").run::<&str>(&[], &stdin);
    let actual = output.success().fields();
    assert_eq!(actual.0.len(), records.len(), "one line per record");
    let mut mismatches = 0;
    for (index, (r, (key, value))) in records.iter().zip(&actual.0).enumerate() {
        assert_eq!(key, "r");
        let expected = r.expected();
        if expected[2..] != value[..] {
            mismatches += 1;
            if mismatches <= 5 {
                eprintln!(
                    "record {index} op {} p {:?}:\n  C    {}\n  Rust {}",
                    r.op,
                    r.p,
                    &value[..value.len().min(200)],
                    &expected[2..expected.len().min(202)]
                );
            }
        }
    }
    assert_eq!(
        mismatches,
        0,
        "{mismatches} of {} records differ",
        records.len()
    );
}
