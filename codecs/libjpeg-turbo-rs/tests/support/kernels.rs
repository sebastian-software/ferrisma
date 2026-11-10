//! The inputs and outputs of the WP10 kernel tests (ADR 0004, ADR 0008).
//!
//! The kernel probes `tools/oracle/probes/wp10/<family>.c` call the oracle's
//! x86-64 SIMD kernels directly with inputs from a seeded xorshift64
//! generator and print FNV-1a hashes of the outputs. This module is the Rust
//! side of `tools/oracle/probes/wp10/support/wp10_support.c`: it draws the
//! same values in the same order and hashes the same way, so that
//! `tests/wp10_<family>.rs` can give the port's kernels the probe's inputs
//! and compare the hashes.
//!
//! Sample rows follow `alloc_sarray` with `WITH_SIMD` (`ALIGN_SIZE` 32): one
//! storage, each row padded to a multiple of 64 samples, so that a kernel
//! that reads or writes into the padding stays inside the storage, as in a
//! decode, and panics where it would leave it.
//!
//! Include this file from a test crate with
//! `#[path = "support/kernels.rs"] mod kernels;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use jpeg::jpeglib::SampleArray;

/// The probes' FNV-1a constants.
const FNV_OFFSET: u64 = 1_469_598_103_934_665_603;
const FNV_PRIME: u64 = 1_099_511_628_211;

/// The xorshift64 generator of `wp10_support.c`.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// `Wp10Seed`: seed 0 counts as 1, since xorshift64 stays at 0.
    #[must_use]
    pub fn new(seed: u64) -> Rng {
        Rng {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// `Wp10Next`: the upper 32 bits of the next state.
    pub fn next(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        (self.state >> 32) as u32
    }

    /// `Wp10Rand`: a value in `0..n`.
    pub fn rand(&mut self, n: u32) -> u32 {
        self.next() % n
    }

    /// `Wp10Sample`: the top 8 bits of `next`.
    pub fn sample(&mut self) -> u8 {
        self.next().to_be_bytes()[0]
    }
}

/// `Wp10Stride`: `samplesperrow` rounded up to a multiple of 64.
#[must_use]
pub fn stride(samplesperrow: usize) -> usize {
    samplesperrow.next_multiple_of(64)
}

/// `Wp10Rows`: `numrows` zeroed rows of `stride(samplesperrow)` samples in
/// one storage.
#[must_use]
pub fn rows(numrows: usize, samplesperrow: usize) -> SampleArray {
    let stride = stride(samplesperrow);
    SampleArray {
        buffer: vec![0; numrows * stride],
        rows: (0..numrows).map(|row| row * stride).collect(),
    }
}

/// `Wp10FillRows`: fills the whole storage of `array`, padding included,
/// with `Rng::sample`, row by row.
pub fn fill_rows(rng: &mut Rng, array: &mut SampleArray) {
    for sample in &mut array.buffer {
        *sample = rng.sample();
    }
}

/// `Wp10Hash`: the FNV-1a hash of `bytes`.
#[must_use]
pub fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
    })
}

/// The value that `Wp10PrintRows` prints for `rows`: the hash of the first
/// `len` bytes of each row, as 16 hex digits, separated by commas.
#[must_use]
pub fn row_hashes<R: AsRef<[u8]>>(rows: &[R], len: usize) -> String {
    rows.iter()
        .map(|row| format!("{:016x}", hash(&row.as_ref()[..len])))
        .collect::<Vec<_>>()
        .join(",")
}

/// The rows of `array` as slices of `len` samples, for [`row_hashes`].
#[must_use]
pub fn array_rows(array: &SampleArray, len: usize) -> Vec<&[u8]> {
    array
        .rows
        .iter()
        .map(|&offset| &array.buffer[offset..offset + len])
        .collect()
}
