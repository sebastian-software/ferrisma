//! The ISO/IEC 10918-2 (ITU-T T.83) compliance data of work package 11.
//!
//! `tools/fetch_t83.py` downloads ITU's zip, verifies it and extracts the
//! 8-bit DCT streams and the encoder and decoder reference test data
//! (`EREF_X8.DCT`, `DREF_X8.DCT`) into `build/t83/`; the
//! data is not redistributed (ADR 0002). `LIBJPEG_T83_DIR` names another
//! directory; a relative path is resolved against the repository root.
//!
//! A test that needs the data starts with [`T83::locate`]. When the
//! directory holds no data the test prints why it is skipped and returns,
//! unless the `oracle` Cargo feature is enabled (as `tools/check.py
//! --oracle` does), in which case it fails, as [`crate::oracle::Oracle`]
//! does.
//!
//! The rest of this file works without the port: the zigzag order of T.81
//! Figure A.6, the quantization tables of a stream's DQT segments, the
//! blocks of a reference data file, and the reference IDCT, FDCT and
//! quantizer of ISO/IEC 10918-2 3.1.41 to 3.1.43 (T.81 A.3.3 and A.3.4).
//!
//! Include it next to `support/oracle.rs`.

#![allow(dead_code, reason = "each test crate uses a different subset")]

use std::path::{Path, PathBuf};

use crate::oracle::repository;

/// The environment variable that names the data directory.
pub const T83_DIR: &str = "LIBJPEG_T83_DIR";

/// The manifest that `tools/fetch_t83.py` writes last.
const MANIFEST: &str = "t83.json";

/// The image streams in scope: name, T.81 process and entropy coder.
pub const IMAGE_STREAMS: [(&str, &str, &str); 9] = [
    ("A1", "baseline sequential", "Huffman"),
    ("C1", "extended sequential", "Huffman"),
    ("C2", "extended sequential", "Huffman"),
    ("D1", "extended sequential", "arithmetic"),
    ("D2", "extended sequential", "arithmetic"),
    ("G1", "progressive, spectral selection", "Huffman"),
    ("H1", "progressive, spectral selection", "arithmetic"),
    ("K1", "progressive, full", "Huffman"),
    ("L1", "progressive, full", "arithmetic"),
];

/// The streams with more than one scan per component.
pub const PROGRESSIVE_STREAMS: [&str; 4] = ["G1", "H1", "K1", "L1"];

/// The four components every 8-bit stream codes, in SOF order: the letter of
/// their reference data, the component identifier and the sampling factors
/// (A1.TXT).
pub const COMPONENTS: [(char, i32, i32, i32); 4] = [
    ('A', 200, 1, 1),
    ('B', 150, 1, 2),
    ('C', 100, 3, 1),
    ('D', 50, 1, 4),
];

/// The located data directory.
pub struct T83 {
    directory: PathBuf,
}

impl T83 {
    /// Returns the data named by `LIBJPEG_T83_DIR`, or `build/t83`.
    ///
    /// Without data, returns `None` after printing that `test` is skipped,
    /// or panics when the `oracle` feature requires the data. Panics when the
    /// variable names a directory that holds no data.
    pub fn locate(test: &str) -> Option<T83> {
        let (directory, named) = match std::env::var_os(T83_DIR) {
            Some(value) => (repository().join(value), true),
            None => (repository().join("build/t83"), false),
        };
        if !directory.join(MANIFEST).is_file() {
            let reason = format!(
                "{} holds no T.83 data; fetch it with `python3 tools/fetch_t83.py` \
                 (or set {T83_DIR})",
                directory.display()
            );
            assert!(!named, "{test}: {T83_DIR}: {reason}");
            if cfg!(feature = "oracle") {
                panic!("{test}: the oracle feature requires the T.83 data: {reason}");
            }
            eprintln!("skipped {test}: {reason}");
            return None;
        }
        Some(T83 { directory })
    }

    /// The data directory.
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// The bytes of the file `name`, for example `A1.JPG`.
    pub fn file(&self, name: &str) -> Vec<u8> {
        let path = self.directory.join(name);
        std::fs::read(&path).unwrap_or_else(|error| {
            panic!(
                "cannot read {}: {error}; rerun python3 tools/fetch_t83.py",
                path.display()
            )
        })
    }

    /// The stream `name`, for example `A1`.
    pub fn stream(&self, name: &str) -> Vec<u8> {
        self.file(&format!("{name}.JPG"))
    }

    /// The blocks of `EREF_<letter>8.DCT`, the encoder reference test data
    /// (ISO/IEC 10918-2 3.1.35): the quantized coefficients of the source
    /// image, which the streams code.
    pub fn encoder_reference(&self, letter: char) -> Vec<[i16; 64]> {
        reference_blocks(&self.file(&format!("EREF_{letter}8.DCT")))
    }

    /// The blocks of `DREF_<letter>8.DCT`, the decoder reference test data
    /// (3.1.34): the reference FDCT and quantizer applied to the reference
    /// decoder's output.
    pub fn decoder_reference(&self, letter: char) -> Vec<[i16; 64]> {
        reference_blocks(&self.file(&format!("DREF_{letter}8.DCT")))
    }
}

/// The zigzag order of T.81 Figure A.6: entry `k` is the natural (row-major)
/// index of the `k`th coefficient in zigzag order. Built from the figure's
/// rule, not from the port's `jpeg_natural_order`.
pub fn zigzag() -> [usize; 64] {
    let mut order = [0; 64];
    let mut k = 0;
    for diagonal in 0..15_usize {
        let rows = diagonal.saturating_sub(7)..=diagonal.min(7);
        let rows: Vec<usize> = if diagonal % 2 == 0 {
            rows.rev().collect()
        } else {
            rows.collect()
        };
        for row in rows {
            order[k] = row * 8 + (diagonal - row);
            k += 1;
        }
    }
    order
}

/// The blocks of a reference data file: big-endian 16-bit coefficients, 64
/// per block in zigzag order (Readme). Returned in natural order.
pub fn reference_blocks(data: &[u8]) -> Vec<[i16; 64]> {
    assert_eq!(data.len() % 128, 0, "a reference file holds whole blocks");
    let order = zigzag();
    let (chunks, _) = data.as_chunks::<128>();
    chunks
        .iter()
        .map(|chunk| {
            let mut block = [0; 64];
            let (pairs, _) = chunk.as_chunks::<2>();
            for (k, &pair) in pairs.iter().enumerate() {
                block[order[k]] = i16::from_be_bytes(pair);
            }
            block
        })
        .collect()
}

/// The quantization tables that a stream defines before its first scan, by
/// table number, in natural order: a DQT parse of T.81 B.2.4.1, independent
/// of the port. Panics on a DQT segment after the first scan, which none of
/// the streams in scope has, and on 16-bit tables, which 8-bit streams do
/// not use.
pub fn quantization_tables(data: &[u8]) -> [Option<[u16; 64]>; 4] {
    let order = zigzag();
    let mut tables = [None; 4];
    let mut position = 2;
    assert_eq!(&data[..2], &[0xff, 0xd8], "SOI");
    loop {
        assert_eq!(data[position], 0xff, "a marker at {position}");
        let marker = data[position + 1];
        if marker == 0xff {
            position += 1;
            continue;
        }
        let length = usize::from(u16::from_be_bytes([data[position + 2], data[position + 3]]));
        let segment = &data[position + 4..position + 2 + length];
        match marker {
            0xdb => {
                let mut rest = segment;
                while !rest.is_empty() {
                    let (precision, number) = (rest[0] >> 4, usize::from(rest[0] & 15));
                    assert_eq!(precision, 0, "an 8-bit quantization table");
                    let mut table = [0; 64];
                    for (k, &value) in rest[1..65].iter().enumerate() {
                        table[order[k]] = u16::from(value);
                    }
                    tables[number] = Some(table);
                    rest = &rest[65..];
                }
            }
            0xda => return tables,
            0xd9 => panic!("EOI before the first scan"),
            _ => {}
        }
        position += 2 + length;
    }
}

/// The distance from a half below which the quantizer treats a quotient as
/// a tie, as it is in exact arithmetic. The `f64` error of the FDCT is about
/// 1e-13, so the cosines' last bit cannot decide a tie.
pub const TIE: f64 = 1e-9;

/// A coefficient of the reference quantizer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quantized {
    /// The one nearest integer.
    Nearest(i16),
    /// A tie: the two nearest integers, toward and away from zero.
    Tie(i16, i16),
}

impl Quantized {
    /// The distance from `value` to the nearest of the integers.
    pub fn distance(self, value: i16) -> u16 {
        match self {
            Quantized::Nearest(integer) => integer.abs_diff(value),
            Quantized::Tie(toward, away) => toward.abs_diff(value).min(away.abs_diff(value)),
        }
    }

    pub fn is_tie(self) -> bool {
        matches!(self, Quantized::Tie(..))
    }
}

/// The reference DCTs of ISO/IEC 10918-2 3.1.41 and 3.1.42, the equations
/// of T.81 A.3.3 evaluated in `f64`, and the reference quantizer of 3.1.43
/// (T.81 A.3.4).
pub struct ReferenceDct {
    /// `basis[k][n]` is C(k) cos((2n + 1) k pi / 16), with C(0) = 1/sqrt(2)
    /// and C(k) = 1 otherwise.
    basis: [[f64; 8]; 8],
}

impl ReferenceDct {
    pub fn new() -> Self {
        let mut basis = [[0.0; 8]; 8];
        for (k, row) in basis.iter_mut().enumerate() {
            let c = if k == 0 {
                std::f64::consts::FRAC_1_SQRT_2
            } else {
                1.0
            };
            for (n, value) in row.iter_mut().enumerate() {
                *value = c * (((2 * n + 1) * k) as f64 * std::f64::consts::PI / 16.0).cos();
            }
        }
        ReferenceDct { basis }
    }

    /// The reference IDCT of a block of dequantized coefficients in natural
    /// order, with the level shift of T.81 A.3.1 (+128), rounding half away
    /// from zero and clamping to 0..=255: the reference decoder's output
    /// (3.1.39).
    pub fn idct(&self, coefficients: &[f64; 64]) -> [u8; 64] {
        let mut samples = [0; 64];
        for y in 0..8 {
            for x in 0..8 {
                let mut sum = 0.0;
                for v in 0..8 {
                    for u in 0..8 {
                        sum += self.basis[u][x] * self.basis[v][y] * coefficients[v * 8 + u];
                    }
                }
                let value = (sum / 4.0 + 128.0).round().clamp(0.0, 255.0);
                samples[y * 8 + x] = value as u8; /* an integer in 0..=255 */
            }
        }
        samples
    }

    /// The reference FDCT of a block of samples in natural order, after the
    /// level shift (-128): unquantized coefficients in natural order.
    pub fn fdct(&self, samples: &[u8; 64]) -> [f64; 64] {
        let mut coefficients = [0.0; 64];
        for v in 0..8 {
            for u in 0..8 {
                let mut sum = 0.0;
                for y in 0..8 {
                    for x in 0..8 {
                        sum += self.basis[u][x]
                            * self.basis[v][y]
                            * (f64::from(samples[y * 8 + x]) - 128.0);
                    }
                }
                coefficients[v * 8 + u] = sum / 4.0;
            }
        }
        coefficients
    }

    /// The reference quantizer: Sq = round(S / Q), the nearest integer
    /// (T.81 A.3.4). A quotient within [`TIE`] of a half is a tie: it has
    /// two nearest integers, and the standard picks neither, so both are
    /// kept.
    pub fn quantize(coefficients: &[f64; 64], table: &[u16; 64]) -> [Quantized; 64] {
        let mut quantized = [Quantized::Nearest(0); 64];
        for (k, value) in quantized.iter_mut().enumerate() {
            let ratio = coefficients[k] / f64::from(table[k]);
            let (magnitude, whole) = (ratio.abs(), ratio.abs().floor());
            /* |S| <= 1024 * 8 for 8-bit samples, so the results fit */
            let signed = |integer: f64| integer.copysign(ratio) as i16;
            *value = if (magnitude - whole - 0.5).abs() < TIE {
                Quantized::Tie(signed(whole), signed(whole + 1.0))
            } else {
                Quantized::Nearest(signed((magnitude + 0.5).floor()))
            };
        }
        quantized
    }

    /// Dequantizes a block of quantized coefficients with `table`.
    pub fn dequantize(block: &[i16; 64], table: &[u16; 64]) -> [f64; 64] {
        let mut coefficients = [0.0; 64];
        for (k, value) in coefficients.iter_mut().enumerate() {
            *value = f64::from(block[k]) * f64::from(table[k]);
        }
        coefficients
    }
}
