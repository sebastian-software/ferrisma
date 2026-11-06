//! Kernel bodies, written once against [`Lanes`]. Safe Rust: slices are
//! bounds-checked, and the `Lanes` methods take fixed-size arrays.

use crate::Taps;
use crate::lanes::Lanes;

/// See `Kernels::horizontal_rgba`. The caller has validated the lengths.
///
/// Interleaving four output positions with separate accumulators was
/// measured and was slower on Apple M1 (24 Mpx transform 106 ms to 116 ms
/// with NEON, 107 ms to 155 ms scalar), so each position runs on its own.
#[inline(always)]
pub(crate) fn horizontal_rgba<L: Lanes>(source: &[f32], taps: &Taps, out: &mut [f32]) {
    let slots = L::SLOTS;
    for (position, pixel) in out.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let start = taps.starts[position] as usize * 4;
        let weights =
            &taps.weights[taps.offsets[position] as usize..taps.offsets[position + 1] as usize];
        let run = &source[start..start + weights.len() * 4];
        let full = weights.len() / slots * slots;
        let mut acc = L::zero();
        // Iterators instead of indices: one bounds check per run, not per tap.
        for (values, slot_weights) in run[..full * 4]
            .chunks_exact(slots * 4)
            .zip(weights[..full].chunks_exact(slots))
        {
            acc = L::fma(acc, L::load(values), L::slot_weights(slot_weights));
        }
        let mut sum = L::reduce_slots(acc);
        for (k, &weight) in weights.iter().enumerate().skip(full) {
            for (channel, value) in sum.iter_mut().enumerate() {
                *value = run[k * 4 + channel].mul_add(weight, *value);
            }
        }
        *pixel = sum;
    }
}

/// See `Kernels::axpy`. The caller has validated the lengths.
#[inline(always)]
pub(crate) fn axpy<L: Lanes>(acc: &mut [f32], row: &[f32], weight: f32) {
    let width = L::SLOTS * 4;
    let full = acc.len() / width * width;
    let splat = L::splat(weight);
    let (acc_body, acc_tail) = acc.split_at_mut(full);
    let (row_body, row_tail) = row.split_at(full);
    for (a, r) in acc_body
        .chunks_exact_mut(width)
        .zip(row_body.chunks_exact(width))
    {
        L::store(a, L::fma(L::load(a), L::load(r), splat));
    }
    for (a, &r) in acc_tail.iter_mut().zip(row_tail) {
        *a = r.mul_add(weight, *a);
    }
}
