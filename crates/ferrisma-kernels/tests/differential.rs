//! Every available level against the scalar body (ADR-0020 decision 7).
//! Levels the host cannot run are printed as untested, never skipped
//! silently.

use ferrisma_kernels::{IsaLevel, KernelError, Kernels, LevelReason, Taps};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn unit(&mut self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let value = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        value
    }
}

fn levels() -> Vec<Kernels> {
    let available = Kernels::available();
    for level in [
        IsaLevel::Neon,
        IsaLevel::Sve2,
        IsaLevel::Avx2,
        IsaLevel::Avx512,
    ] {
        if !available.iter().any(|k| k.level() == level) {
            println!("untested on this host: {level}");
        }
    }
    available
}

/// Distance in units in the last place, for finite values of equal sign
/// or near zero.
fn ulps(a: f32, b: f32) -> u32 {
    if a.to_bits() == b.to_bits() {
        return 0;
    }
    let order = |x: f32| {
        let bits = x.to_bits().cast_signed();
        if bits < 0 { i32::MIN - bits } else { bits }
    };
    order(a).abs_diff(order(b))
}

/// Random taps: runs of `1..=max_taps` weights, including negative lobes and
/// sums that are not one; starts anywhere that keeps the run in the row.
fn random_taps(rng: &mut Rng, source_pixels: usize, outputs: usize, max_taps: usize) -> Taps {
    let mut starts = Vec::new();
    let mut offsets = vec![0_u32];
    let mut weights = Vec::new();
    for _ in 0..outputs {
        let count =
            1 + usize::try_from(rng.next() % u64::try_from(max_taps.min(source_pixels)).unwrap())
                .unwrap();
        let start = usize::try_from(rng.next() % u64::try_from(source_pixels - count + 1).unwrap())
            .unwrap();
        starts.push(u32::try_from(start).unwrap());
        for _ in 0..count {
            weights.push(rng.unit().mul_add(1.6, -0.3));
        }
        offsets.push(u32::try_from(weights.len()).unwrap());
    }
    Taps::new(source_pixels, starts, offsets, weights).unwrap()
}

#[test]
fn axpy_is_bit_identical_at_every_level() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for length in (0..=40).chain([255, 256, 257, 4800, 4801]) {
        let row: Vec<f32> = (0..length).map(|_| rng.unit().mul_add(2.0, -0.5)).collect();
        let start: Vec<f32> = (0..length).map(|_| rng.unit()).collect();
        let weight = rng.unit().mul_add(1.4, -0.2);
        let mut expected = start.clone();
        scalar.axpy(&mut expected, &row, weight).unwrap();
        for kernels in levels() {
            let mut actual = start.clone();
            kernels.axpy(&mut actual, &row, weight).unwrap();
            for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                assert_eq!(
                    e.to_bits(),
                    a.to_bits(),
                    "{} length {length} index {index}",
                    kernels.level()
                );
            }
        }
    }
}

#[test]
fn vertical_rgba_is_bit_identical_to_axpy_at_every_level() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    // Cover unroll boundaries (1, 4, 8, 16, 32, 64), column-tile boundaries (256),
    // odd/unaligned lengths, and wide rows (24000).
    let lengths = [
        0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257,
        511, 512, 513, 4800, 4801,
    ];
    let tap_counts = [0, 1, 2, 3, 4, 7, 23, 31, 32, 33, 64, 65, 128];
    for &tap_count in &tap_counts {
        let weights: Vec<f32> = (0..tap_count)
            .map(|t| match t {
                0 if tap_count > 1 => -0.0_f32,
                1 if tap_count > 2 => 0.0_f32,
                _ => rng.unit().mul_add(1.6, -0.3),
            })
            .collect();
        for &length in &lengths {
            // Keep test fast while covering all tap_counts at boundary lengths.
            if length > 513 && !matches!(tap_count, 1 | 23 | 33 | 128) {
                continue;
            }
            let storage: Vec<Vec<f32>> = (0..tap_count)
                .map(|t| {
                    (0..length)
                        .map(|i| match (t, i % 17) {
                            (0, 0) => -0.0_f32,
                            (0, 1) => 0.0_f32,
                            (_, 2) => f32::from_bits(1), // smallest positive subnormal
                            (_, 3) => -1.0_f32,
                            _ => rng.unit().mul_add(2.0, -0.5),
                        })
                        .collect()
                })
                .collect();
            let row_refs: Vec<&[f32]> = storage.iter().map(Vec::as_slice).collect();
            let mut expected = vec![f32::NAN; length];
            expected.fill(0.0);
            for (&row, &weight) in row_refs.iter().zip(&weights) {
                scalar.axpy(&mut expected, row, weight).unwrap();
            }
            for kernels in &available {
                let mut actual = vec![f32::NAN; length];
                kernels
                    .vertical_rgba(&row_refs, &weights, &mut actual)
                    .unwrap();
                for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} vertical_rgba taps={tap_count} length={length} index={index}: {e} vs {a}",
                        kernels.level()
                    );
                }
            }
        }
    }
    // Wide row (6000 RGBA pixels = 24000 f32 = 96 KiB) with 24 and 65 taps.
    for tap_count in [24, 65] {
        let weights: Vec<f32> = (0..tap_count)
            .map(|_| rng.unit().mul_add(1.6, -0.3))
            .collect();
        let storage: Vec<Vec<f32>> = (0..tap_count)
            .map(|_| (0..24_000).map(|_| rng.unit().mul_add(2.0, -0.5)).collect())
            .collect();
        let row_refs: Vec<&[f32]> = storage.iter().map(Vec::as_slice).collect();
        let mut expected = vec![0.0_f32; 24_000];
        for (&row, &weight) in row_refs.iter().zip(&weights) {
            scalar.axpy(&mut expected, row, weight).unwrap();
        }
        for kernels in &available {
            let mut actual = vec![f32::NAN; 24_000];
            kernels
                .vertical_rgba(&row_refs, &weights, &mut actual)
                .unwrap();
            for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                assert_eq!(
                    e.to_bits(),
                    a.to_bits(),
                    "{} vertical_rgba wide taps={tap_count} index={index}",
                    kernels.level()
                );
            }
        }
    }
}

#[test]
fn vertical_rgba_add_is_bit_identical_to_axpy_and_supports_chunking() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0x94D0_49BB_1331_11EB);
    let lengths = [
        0, 1, 3, 5, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 513,
    ];
    for &length in &lengths {
        let total_taps = if length <= 65 { 150 } else { 70 };
        let storage: Vec<Vec<f32>> = (0..total_taps)
            .map(|_| (0..length).map(|_| rng.unit().mul_add(2.0, -0.5)).collect())
            .collect();
        let weights: Vec<f32> = (0..total_taps)
            .map(|_| rng.unit().mul_add(1.6, -0.3))
            .collect();
        let row_refs: Vec<&[f32]> = storage.iter().map(Vec::as_slice).collect();
        let mut expected = vec![0.0_f32; length];
        for (&row, &w) in row_refs.iter().zip(&weights) {
            scalar.axpy(&mut expected, row, w).unwrap();
        }
        for kernels in &available {
            for chunk_size in [7_usize, 64] {
                let mut actual = vec![f32::NAN; length];
                for (chunk_idx, (rc, wc)) in row_refs
                    .chunks(chunk_size)
                    .zip(weights.chunks(chunk_size))
                    .enumerate()
                {
                    if chunk_idx == 0 {
                        kernels.vertical_rgba(rc, wc, &mut actual).unwrap();
                    } else {
                        kernels.vertical_rgba_add(rc, wc, &mut actual).unwrap();
                    }
                }
                for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} vertical_rgba_add len={length} chunk={chunk_size} index={index}",
                        kernels.level()
                    );
                }
            }
            // Empty rows must be a no-op that preserves existing `acc` bits.
            let sentinel = vec![42.25_f32; length];
            let mut untouched = sentinel.clone();
            kernels.vertical_rgba_add(&[], &[], &mut untouched).unwrap();
            for (s, u) in sentinel.iter().zip(&untouched) {
                assert_eq!(s.to_bits(), u.to_bits());
            }
        }
    }
}

#[test]
fn horizontal_matches_the_scalar_body_within_four_ulp() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for (source_pixels, outputs, max_taps) in [
        (1, 1, 1),
        (2, 3, 2),
        (7, 5, 7),
        (64, 9, 40),
        (300, 1600, 12),
        (1000, 37, 300),
    ] {
        let taps = random_taps(&mut rng, source_pixels, outputs, max_taps);
        let source: Vec<f32> = (0..source_pixels * 4)
            .map(|_| rng.unit().mul_add(1.5, -0.2))
            .collect();
        let mut expected = vec![0.0; outputs * 4];
        scalar
            .horizontal_rgba(&source, &taps, &mut expected)
            .unwrap();
        for kernels in levels() {
            let mut actual = vec![f32::NAN; outputs * 4];
            kernels
                .horizontal_rgba(&source, &taps, &mut actual)
                .unwrap();
            let exact = matches!(kernels.level(), IsaLevel::Scalar | IsaLevel::Neon);
            #[allow(clippy::cast_precision_loss)]
            let cancellation_bound = 4.0 * f32::EPSILON * max_taps as f32;
            for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                if exact {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} index {index}: {e} vs {a}",
                        kernels.level()
                    );
                } else {
                    let distance = ulps(*e, *a);
                    let diff = (*e - *a).abs();
                    assert!(
                        distance <= 4 || diff <= cancellation_bound,
                        "{} index {index}: {e} vs {a} ({distance} ulps, diff {diff})",
                        kernels.level()
                    );
                }
            }
        }
    }
}

#[test]
fn horizontal_covers_every_small_tap_count_and_tail_remainder() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0xA076_1D64_78BD_642F);
    let source_pixels = 32;
    let source: Vec<f32> = (0..source_pixels * 4)
        .map(|_| rng.unit().mul_add(1.5, -0.25))
        .collect();
    for tap_count in 1..=20 {
        let outputs = 8;
        let mut starts = Vec::with_capacity(outputs);
        let mut offsets = Vec::with_capacity(outputs + 1);
        offsets.push(0_u32);
        let mut weights = Vec::with_capacity(outputs * tap_count);
        for out_idx in 0..outputs {
            starts.push(u32::try_from(out_idx % (source_pixels - tap_count + 1)).unwrap());
            for _ in 0..tap_count {
                weights.push(rng.unit().mul_add(1.6, -0.3));
            }
            offsets.push(u32::try_from(weights.len()).unwrap());
        }
        let taps = Taps::new(
            source_pixels,
            starts.clone(),
            offsets.clone(),
            weights.clone(),
        )
        .unwrap();
        let mut expected = vec![0.0_f32; outputs * 4];
        scalar
            .horizontal_rgba(&source, &taps, &mut expected)
            .unwrap();
        for kernels in &available {
            let mut actual = vec![f32::NAN; outputs * 4];
            kernels
                .horizontal_rgba(&source, &taps, &mut actual)
                .unwrap();
            let exact = matches!(kernels.level(), IsaLevel::Scalar | IsaLevel::Neon);
            #[allow(clippy::cast_precision_loss)]
            let cancellation_bound = 4.0 * f32::EPSILON * tap_count as f32;
            for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                if exact {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} tap_count={tap_count} index={index}: {e} vs {a}",
                        kernels.level()
                    );
                } else {
                    let distance = ulps(*e, *a);
                    let diff = (*e - *a).abs();
                    assert!(
                        distance <= 4 || diff <= cancellation_bound,
                        "{} tap_count={tap_count} index={index}: {e} vs {a} ({distance} ulps, diff {diff})",
                        kernels.level()
                    );
                }
            }
            // Multi-pixel pipelined output must be bit-identical to single-pixel evaluation.
            for out_idx in 0..outputs {
                let single_weights =
                    weights[offsets[out_idx] as usize..offsets[out_idx + 1] as usize].to_vec();
                let single_taps = Taps::new(
                    source_pixels,
                    vec![starts[out_idx]],
                    vec![0, u32::try_from(single_weights.len()).unwrap()],
                    single_weights,
                )
                .unwrap();
                let mut single = [f32::NAN; 4];
                kernels
                    .horizontal_rgba(&source, &single_taps, &mut single)
                    .unwrap();
                for c in 0..4 {
                    assert_eq!(
                        actual[out_idx * 4 + c].to_bits(),
                        single[c].to_bits(),
                        "{} pipelined vs single tap_count={tap_count} out={out_idx} c={c}",
                        kernels.level()
                    );
                }
            }
        }
    }
}

/// Non-cancelling positive-only horizontal sums:
/// 1. Same-ISA pipelining equivalence: for EVERY ISA level (including `Avx2`
///    and `Avx512`), multi-pixel pipelined outputs must be bit-identical
///    (`0 ULP`) to single-pixel (`outputs = 1`) evaluation of the exact same
///    taps across every integer tap count `1..=128`, the 8-tap skewed positive
///    case, and random positive tap sets.
/// 2. `Scalar == Neon` equivalence: exact bit-identity (`0 ULP`) across all
///    tap counts and output widths.
/// 3. Multi-slot (`Avx2` `SLOTS = 2`, `Avx512` `SLOTS = 4`) vs `Scalar` and
///    exact `f64` oracle across every integer tap count `1..=128`, the 8-tap
///    skewed positive sum `[0.5, a, b, a, b, a, b, a]` (5 ULP on `Avx2`), and
///    random positive sums: bounded by `4 * f32::EPSILON * taps` against both
///    `Scalar` and the `f64` oracle (multi-slot reassociation can exceed 4 ULP
///    against `Scalar` even on 8 positive taps, and can exceed `Scalar`'s
///    `f64`-oracle ULP distance, e.g. 8 ULP vs 3 ULP at 100 uniform `0.1_f32`
///    taps on `Avx2`).
#[test]
#[allow(clippy::too_many_lines)]
fn horizontal_non_cancelling_positive_sums_stay_within_reassociation_bound() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0x6C62_272E_07BB_0142);

    // 1. Every integer tap count 1..=128 (including 32, 64, 100, 128) on uniform 0.1_f32.
    for tap_count in 1..=128_usize {
        let source_pixels = tap_count + 16;
        let source = vec![0.1_f32; source_pixels * 4];
        #[allow(clippy::cast_precision_loss)]
        let weight = 1.0_f32 / tap_count as f32;
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let oracle_f32 = (tap_count as f64 * f64::from(0.1_f32) * f64::from(weight)) as f32;
        #[allow(clippy::cast_precision_loss)]
        let reassociation_bound = 4.0 * f32::EPSILON * tap_count as f32;
        let single_taps = Taps::new(
            source_pixels,
            vec![0],
            vec![0, u32::try_from(tap_count).unwrap()],
            vec![weight; tap_count],
        )
        .unwrap();
        let mut expected_single = [0.0_f32; 4];
        scalar
            .horizontal_rgba(&source, &single_taps, &mut expected_single)
            .unwrap();
        let output_counts: &[usize] = if matches!(tap_count, 1 | 4 | 8 | 16 | 32 | 64 | 100 | 128) {
            &[2, 3, 4, 5, 7, 8, 9, 17]
        } else {
            &[9]
        };
        for kernels in &available {
            let mut single_level = [f32::NAN; 4];
            kernels
                .horizontal_rgba(&source, &single_taps, &mut single_level)
                .unwrap();
            for (c, (e, a)) in expected_single.iter().zip(&single_level).enumerate() {
                match kernels.level() {
                    IsaLevel::Scalar | IsaLevel::Neon => {
                        assert_eq!(
                            e.to_bits(),
                            a.to_bits(),
                            "{} uniform 0.1 taps={tap_count} channel={c}: {e} ({:#010x}) vs {a} ({:#010x})",
                            kernels.level(),
                            e.to_bits(),
                            a.to_bits()
                        );
                    }
                    IsaLevel::Avx2 | IsaLevel::Avx512 => {
                        let diff_scalar = (*e - *a).abs();
                        let diff_oracle = (*a - oracle_f32).abs();
                        assert!(
                            diff_scalar <= reassociation_bound,
                            "{} uniform 0.1 taps={tap_count} channel={c}: {e} vs {a} (diff {diff_scalar} > {reassociation_bound})",
                            kernels.level()
                        );
                        assert!(
                            diff_oracle <= reassociation_bound,
                            "{} uniform 0.1 taps={tap_count} channel={c}: {a} vs f64 oracle {oracle_f32} (diff {diff_oracle} > {reassociation_bound})",
                            kernels.level()
                        );
                    }
                    _ => {}
                }
            }
            for &outputs in output_counts {
                let mut starts = Vec::with_capacity(outputs);
                let mut offsets = Vec::with_capacity(outputs + 1);
                offsets.push(0_u32);
                let mut weights = Vec::with_capacity(outputs * tap_count);
                for out_idx in 0..outputs {
                    starts.push(u32::try_from(out_idx % (source_pixels - tap_count + 1)).unwrap());
                    weights.extend(std::iter::repeat_n(weight, tap_count));
                    offsets.push(u32::try_from(weights.len()).unwrap());
                }
                let taps = Taps::new(source_pixels, starts, offsets, weights).unwrap();
                let mut actual = vec![f32::NAN; outputs * 4];
                kernels
                    .horizontal_rgba(&source, &taps, &mut actual)
                    .unwrap();
                for (index, a) in actual.iter().enumerate() {
                    assert_eq!(
                        a.to_bits(),
                        single_level[index % 4].to_bits(),
                        "{} multi-pixel pipelining mismatch taps={tap_count} outputs={outputs} index={index}",
                        kernels.level()
                    );
                }
            }
        }
    }

    // 2. Skewed 8-tap positive sum `[0.5, a, b, a, b, a, b, a]` (weights 1/8):
    // no cancellation, yet 2-slot `Avx2` differs by 5 ULP from `Scalar` while
    // staying well within `4 * f32::EPSILON * 8` of both `Scalar` and `f64`.
    let a = 9.0_f32 * 2.0_f32.powi(-28);
    let b = 8.0_f32 * 2.0_f32.powi(-28);
    let skewed_reds = [0.5_f32, a, b, a, b, a, b, a];
    let mut skewed_single_src = Vec::with_capacity(8 * 4);
    for &r in &skewed_reds {
        skewed_single_src.extend_from_slice(&[r, 0.0, 0.0, 0.0]);
    }
    let mut skewed_multi_src = Vec::with_capacity(9 * 8 * 4);
    for _ in 0..9 {
        skewed_multi_src.extend_from_slice(&skewed_single_src);
    }
    let skewed_single_taps = Taps::new(8, vec![0], vec![0, 8], vec![0.125_f32; 8]).unwrap();
    let skewed_multi_taps = Taps::new(
        72,
        (0..9_u32).map(|i| i * 8).collect(),
        (0..=9_u32).map(|i| i * 8).collect(),
        vec![0.125_f32; 9 * 8],
    )
    .unwrap();
    #[allow(clippy::cast_possible_truncation)]
    let skewed_oracle = (skewed_reds
        .iter()
        .map(|&r| f64::from(r) * 0.125_f64)
        .sum::<f64>()) as f32;
    let skewed_bound = 4.0 * f32::EPSILON * 8.0;
    let mut skewed_scalar = [0.0_f32; 4];
    scalar
        .horizontal_rgba(&skewed_single_src, &skewed_single_taps, &mut skewed_scalar)
        .unwrap();
    for kernels in &available {
        let mut single = [f32::NAN; 4];
        let mut multi = vec![f32::NAN; 9 * 4];
        kernels
            .horizontal_rgba(&skewed_single_src, &skewed_single_taps, &mut single)
            .unwrap();
        kernels
            .horizontal_rgba(&skewed_multi_src, &skewed_multi_taps, &mut multi)
            .unwrap();
        for p in 0..9 {
            for c in 0..4 {
                assert_eq!(
                    multi[p * 4 + c].to_bits(),
                    single[c].to_bits(),
                    "{} skewed 8-tap pipelined vs single p={p} c={c}",
                    kernels.level()
                );
            }
        }
        match kernels.level() {
            IsaLevel::Scalar | IsaLevel::Neon => {
                assert_eq!(
                    single[0].to_bits(),
                    skewed_scalar[0].to_bits(),
                    "{} skewed 8-tap mismatch vs Scalar",
                    kernels.level()
                );
            }
            IsaLevel::Avx2 | IsaLevel::Avx512 => {
                assert!(
                    (single[0] - skewed_scalar[0]).abs() <= skewed_bound,
                    "{} skewed 8-tap vs Scalar: {} vs {}",
                    kernels.level(),
                    single[0],
                    skewed_scalar[0]
                );
                assert!(
                    (single[0] - skewed_oracle).abs() <= skewed_bound,
                    "{} skewed 8-tap vs f64 oracle: {} vs {}",
                    kernels.level(),
                    single[0],
                    skewed_oracle
                );
            }
            _ => {}
        }
    }

    // 3. Random strictly positive samples and weights (non-cancelling), 1..=16 taps:
    // 0 ULP on Scalar/Neon, exact same-ISA pipelined == single-pixel bit-identity,
    // and bounded by `4 * f32::EPSILON * max_taps` vs Scalar & f64 oracle on Avx2/Avx512.
    for (source_pixels, outputs, max_taps) in [(16, 9, 16), (32, 17, 16), (64, 37, 16)] {
        let mut starts = Vec::with_capacity(outputs);
        let mut offsets = Vec::with_capacity(outputs + 1);
        offsets.push(0_u32);
        let mut weights = Vec::new();
        for _ in 0..outputs {
            let count = 1 + usize::try_from(
                rng.next() % u64::try_from(max_taps.min(source_pixels)).unwrap(),
            )
            .unwrap();
            let start =
                usize::try_from(rng.next() % u64::try_from(source_pixels - count + 1).unwrap())
                    .unwrap();
            starts.push(u32::try_from(start).unwrap());
            for _ in 0..count {
                weights.push(rng.unit().mul_add(0.9, 0.1));
            }
            offsets.push(u32::try_from(weights.len()).unwrap());
        }
        let taps = Taps::new(
            source_pixels,
            starts.clone(),
            offsets.clone(),
            weights.clone(),
        )
        .unwrap();
        let source: Vec<f32> = (0..source_pixels * 4)
            .map(|_| rng.unit().mul_add(0.9, 0.1))
            .collect();
        let mut expected = vec![0.0_f32; outputs * 4];
        scalar
            .horizontal_rgba(&source, &taps, &mut expected)
            .unwrap();
        let mut oracle = vec![0.0_f32; outputs * 4];
        for pos in 0..outputs {
            let start = starts[pos] as usize * 4;
            let w_slice = &weights[offsets[pos] as usize..offsets[pos + 1] as usize];
            let mut sum = [0.0_f64; 4];
            for (k, &w) in w_slice.iter().enumerate() {
                for c in 0..4 {
                    sum[c] += f64::from(source[start + k * 4 + c]) * f64::from(w);
                }
            }
            #[allow(clippy::cast_possible_truncation)]
            for c in 0..4 {
                oracle[pos * 4 + c] = sum[c] as f32;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let bound = 4.0 * f32::EPSILON * max_taps as f32;
        for kernels in &available {
            let mut actual = vec![f32::NAN; outputs * 4];
            kernels
                .horizontal_rgba(&source, &taps, &mut actual)
                .unwrap();
            for (index, ((e, o), a)) in expected.iter().zip(&oracle).zip(&actual).enumerate() {
                match kernels.level() {
                    IsaLevel::Scalar | IsaLevel::Neon => {
                        assert_eq!(
                            e.to_bits(),
                            a.to_bits(),
                            "{} positive random index={index}: {e} vs {a}",
                            kernels.level()
                        );
                    }
                    IsaLevel::Avx2 | IsaLevel::Avx512 => {
                        assert!(
                            (*e - *a).abs() <= bound,
                            "{} positive random vs Scalar index={index}: {e} vs {a}",
                            kernels.level()
                        );
                        assert!(
                            (*o - *a).abs() <= bound,
                            "{} positive random vs f64 oracle index={index}: {o} vs {a}",
                            kernels.level()
                        );
                    }
                    _ => {}
                }
            }
            for pos in 0..outputs {
                let w_slice = weights[offsets[pos] as usize..offsets[pos + 1] as usize].to_vec();
                let single_taps = Taps::new(
                    source_pixels,
                    vec![starts[pos]],
                    vec![0, u32::try_from(w_slice.len()).unwrap()],
                    w_slice,
                )
                .unwrap();
                let mut single = [f32::NAN; 4];
                kernels
                    .horizontal_rgba(&source, &single_taps, &mut single)
                    .unwrap();
                for c in 0..4 {
                    assert_eq!(
                        actual[pos * 4 + c].to_bits(),
                        single[c].to_bits(),
                        "{} positive random pipelined vs single pos={pos} c={c}",
                        kernels.level()
                    );
                }
            }
        }
    }
}

/// Regression for large opposite-sign tap cancellation and unequal-weight
/// cancellation:
/// - `[67108864, 0, -67108864, 0, 1, 0, 0, 0]` (`1/8` weights) preserves
///   `0.125` on `Scalar`, `Neon`, and `Avx2`.
/// - `[67108864, 1, 0, 0, -67108864, 0, 0, 0]` (`1/8` weights) preserves
///   `0.125` on `Avx2` (matching `main`'s 2-slot `Avx2`).
/// - `[-100663296, 67108864]` with Area weights `[0.4, 0.6]` stays `0.0` on
///   every ISA level.
/// - For EVERY level (including `Avx512`), multi-pixel pipelined calls match
///   single-pixel evaluation bit-for-bit.
#[test]
#[allow(clippy::too_many_lines)]
fn horizontal_preserves_cancellation_for_large_opposite_taps() {
    let available = levels();
    let patterns: [(&str, [f32; 8], bool); 2] = [
        (
            "cancel_at_tap_2",
            [
                67_108_864.0_f32,
                0.0,
                -67_108_864.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
            ],
            true,
        ),
        (
            "cancel_at_tap_4",
            [
                67_108_864.0_f32,
                1.0,
                0.0,
                0.0,
                -67_108_864.0,
                0.0,
                0.0,
                0.0,
            ],
            false,
        ),
    ];
    let single_taps = Taps::new(8, vec![0], vec![0, 8], vec![0.125_f32; 8]).unwrap();
    let multi_taps = Taps::new(
        72,
        (0..9_u32).map(|i| i * 8).collect(),
        (0..=9_u32).map(|i| i * 8).collect(),
        vec![0.125_f32; 9 * 8],
    )
    .unwrap();
    for (label, reds, expect_scalar_and_neon) in patterns {
        let mut single_src = Vec::with_capacity(8 * 4);
        for &r in &reds {
            single_src.extend_from_slice(&[r, 0.0, 0.0, 0.0]);
        }
        let mut multi_src = Vec::with_capacity(9 * 8 * 4);
        for _ in 0..9 {
            multi_src.extend_from_slice(&single_src);
        }
        for kernels in &available {
            let mut single = [f32::NAN; 4];
            let mut multi = vec![f32::NAN; 9 * 4];
            kernels
                .horizontal_rgba(&single_src, &single_taps, &mut single)
                .unwrap();
            kernels
                .horizontal_rgba(&multi_src, &multi_taps, &mut multi)
                .unwrap();
            for p in 0..9 {
                for c in 0..4 {
                    assert_eq!(
                        multi[p * 4 + c].to_bits(),
                        single[c].to_bits(),
                        "{} {label} pipelined vs single pixel {p} c {c}",
                        kernels.level()
                    );
                }
            }
            let check_0125 = match kernels.level() {
                IsaLevel::Scalar | IsaLevel::Neon => expect_scalar_and_neon,
                IsaLevel::Avx2 => true,
                _ => false,
            };
            if check_0125 {
                assert_eq!(
                    single[0].to_bits(),
                    0.125_f32.to_bits(),
                    "{} {label}: got {} ({:#010x})",
                    kernels.level(),
                    single[0],
                    single[0].to_bits()
                );
            }
        }
    }

    // Unequal-weight cancellation (5->3 Area pixel 2: -100663296 * 0.4 + 67108864 * 0.6).
    let unequal_src = [
        -100_663_296.0_f32,
        0.0,
        0.0,
        0.0,
        67_108_864.0_f32,
        0.0,
        0.0,
        0.0,
    ];
    let unequal_taps = Taps::new(2, vec![0], vec![0, 2], vec![0.4_f32, 0.6_f32]).unwrap();
    for kernels in &available {
        let mut out = [f32::NAN; 4];
        kernels
            .horizontal_rgba(&unequal_src, &unequal_taps, &mut out)
            .unwrap();
        assert_eq!(
            out[0],
            0.0_f32,
            "{} unequal-weight cancellation: got {}",
            kernels.level(),
            out[0]
        );
    }
}

/// Crop-region invariance: unrelated large/HDR pixels later in the same source
/// row must never change the bit pattern of earlier output pixels on any ISA
/// level.
#[test]
fn horizontal_output_is_invariant_to_unrelated_pixels_outside_its_taps() {
    let available = levels();
    let v = f32::from_bits(0x39ee_b40d);
    let clean_src = [
        v, 0.0, 0.0, 0.0, v, 0.0, 0.0, 0.0, v, 0.0, 0.0, 0.0, v, 0.0, 0.0, 0.0, v, 0.0, 0.0, 0.0,
        v, 0.0, 0.0, 0.0,
    ];
    let mut hdr_tail_src = clean_src;
    for px in 3..6 {
        hdr_tail_src[px * 4] = 3.0_f32;
    }
    let full_taps = Taps::new(6, vec![0, 3], vec![0, 3, 6], vec![1.0_f32 / 3.0; 6]).unwrap();
    let crop_taps = Taps::new(3, vec![0], vec![0, 3], vec![1.0_f32 / 3.0; 3]).unwrap();

    for kernels in &available {
        let mut crop_out = [f32::NAN; 4];
        let mut full_clean = [f32::NAN; 8];
        let mut full_hdr = [f32::NAN; 8];
        kernels
            .horizontal_rgba(&clean_src[..12], &crop_taps, &mut crop_out)
            .unwrap();
        kernels
            .horizontal_rgba(&clean_src, &full_taps, &mut full_clean)
            .unwrap();
        kernels
            .horizontal_rgba(&hdr_tail_src, &full_taps, &mut full_hdr)
            .unwrap();
        for c in 0..4 {
            assert_eq!(
                full_hdr[c].to_bits(),
                crop_out[c].to_bits(),
                "{} crop invariance mismatch c={c}",
                kernels.level()
            );
            assert_eq!(
                full_clean[c].to_bits(),
                crop_out[c].to_bits(),
                "{} full vs crop mismatch c={c}",
                kernels.level()
            );
        }
    }
}

#[test]
fn shape_errors_are_reported_not_panicked() {
    let kernels = Kernels::best();
    assert_eq!(
        Taps::new(4, vec![2], vec![0, 3], vec![0.5; 3]).unwrap_err(),
        KernelError::InvalidTaps
    );
    assert_eq!(
        Taps::new(4, vec![0], vec![1, 2], vec![0.5; 2]).unwrap_err(),
        KernelError::InvalidTaps
    );
    let taps = Taps::new(4, vec![0, 1], vec![0, 2, 4], vec![0.5; 4]).unwrap();
    assert_eq!(
        kernels.horizontal_rgba(&[0.0; 15], &taps, &mut [0.0; 8]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.axpy(&mut [0.0; 3], &[0.0; 4], 1.0),
        Err(KernelError::LengthMismatch)
    );
    let r4 = [1.0_f32; 4];
    let r3 = [1.0_f32; 3];
    assert_eq!(
        kernels.vertical_rgba(&[&r4[..]], &[1.0, 2.0], &mut [0.0; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.vertical_rgba(&[&r4[..], &r3[..]], &[1.0, 2.0], &mut [0.0; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.vertical_rgba(&[&r4[..]], &[1.0], &mut [0.0; 3]),
        Err(KernelError::LengthMismatch)
    );
    let mut empty_out = [f32::NAN; 5];
    kernels.vertical_rgba(&[], &[], &mut empty_out).unwrap();
    assert!(empty_out.iter().all(|v| v.to_bits() == 0.0_f32.to_bits()));
    kernels
        .vertical_rgba(&[&[], &[]], &[0.5, 0.5], &mut [])
        .unwrap();
    assert_eq!(
        kernels.vertical_rgba_add(&[&r4[..]], &[1.0, 2.0], &mut [0.0; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.vertical_rgba_add(&[&r4[..], &r3[..]], &[1.0, 2.0], &mut [0.0; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.vertical_rgba_add(&[&r4[..]], &[1.0], &mut [0.0; 3]),
        Err(KernelError::LengthMismatch)
    );
    kernels
        .vertical_rgba_add(&[&[], &[]], &[0.5, 0.5], &mut [])
        .unwrap();
    assert_eq!(
        kernels.srgb_to_linear(&[0.0; 3], &mut [0.0_f64; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert_eq!(
        kernels.linear_to_srgb_u16(&[0.0; 3], 65_535, &mut [0; 4]),
        Err(KernelError::LengthMismatch)
    );
    assert!(matches!(
        Kernels::for_level(if cfg!(target_arch = "x86_64") {
            IsaLevel::Neon
        } else {
            IsaLevel::Avx2
        }),
        Err(KernelError::UnsupportedLevel(_))
    ));
}

#[test]
fn best_reports_a_supported_level_and_honors_a_ceiling() {
    let best = Kernels::best();
    assert!(ferrisma_kernels::supports(best.level()));
    assert!(best.level() <= ferrisma_kernels::machine_level());
    assert_eq!(
        Kernels::with_ceiling(IsaLevel::Scalar).level(),
        IsaLevel::Scalar
    );
}

/// Decision D6: the default never exceeds AVX2 / NEON, even on a machine
/// with AVX-512; an explicit `Avx512` ceiling selects the highest available
/// level. On a host without AVX-512 both agree, so this test only proves
/// the cap where the host has AVX-512 (the injected-feature tests in
/// `src/lib.rs` cover the selection logic everywhere).
#[test]
fn best_stops_at_the_default_ceiling_and_avx512_is_opt_in() {
    let best = Kernels::best();
    assert!(best.level() <= ferrisma_kernels::DEFAULT_CEILING);
    assert_ne!(best.level(), IsaLevel::Avx512);
    let highest = Kernels::available()
        .last()
        .map_or(IsaLevel::Scalar, Kernels::level);
    let opted_in = Kernels::with_ceiling(IsaLevel::Avx512);
    assert_eq!(opted_in.level(), highest);
    if highest == IsaLevel::Avx512 {
        assert_eq!(best.level(), IsaLevel::Avx2);
        assert_eq!(best.reason(), LevelReason::DefaultCeiling);
    } else {
        assert_eq!(best.level(), highest);
        assert_ne!(best.reason(), LevelReason::DefaultCeiling);
    }
}

fn oracle_srgb_to_linear(encoded: f32) -> f64 {
    // Prevent compile-time constant folding of `powf` in `--release` builds so
    // the oracle always evaluates the target's runtime `f64::powf`.
    let value = std::hint::black_box(f64::from(encoded));
    let magnitude = value.abs();
    if magnitude <= 0.040_45 {
        value / 12.92
    } else {
        value.signum() * ((magnitude + 0.055) / 1.055).powf(2.4)
    }
}

fn oracle_linear_to_srgb_u16(linear: f32, max: u16) -> u16 {
    let clamped = f64::from(linear).clamp(0.0, 1.0);
    let encoded = if clamped <= 0.003_130_8 {
        12.92 * clamped
    } else {
        1.055 * clamped.powf(1.0 / 2.4) - 0.055
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    {
        (encoded * f64::from(max)).round() as u16
    }
}

#[test]
fn srgb_to_linear_matches_scalar_within_one_ulp_and_matches_f64_oracle() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0x517C_C1B7_2722_0A95);
    let specials: [f32; 18] = [
        -2.5,
        -1.000_000_1,
        -1.0,
        -0.5,
        -0.040_451,
        -0.040_45,
        -0.040_449,
        -1.0e-38,
        -0.0,
        0.0,
        1.0e-38,
        0.040_449,
        0.040_45,
        0.040_451,
        0.5,
        1.0,
        1.000_000_1,
        2.5,
    ];
    for length in (0..=40).chain([255, 256, 257, 4800, 4801]) {
        let mut src: Vec<f32> = (0..length)
            .map(|i| {
                if i < specials.len() {
                    specials[i]
                } else {
                    rng.unit().mul_add(2.6, -1.3)
                }
            })
            .collect();
        if length >= 11 {
            src[length - 3] = f32::NAN;
            src[length - 2] = f32::INFINITY;
            src[length - 1] = f32::NEG_INFINITY;
        }
        let mut expected = vec![0.0_f64; length];
        scalar.srgb_to_linear(&src, &mut expected).unwrap();
        for kernels in &available {
            let mut actual = vec![f64::NAN; length];
            kernels.srgb_to_linear(&src, &mut actual).unwrap();
            for (index, (&e, &a)) in expected.iter().zip(&actual).enumerate() {
                if e.is_nan() {
                    assert!(
                        a.is_nan(),
                        "{} length {length} index {index}",
                        kernels.level()
                    );
                } else if !e.is_finite() || e == 0.0 {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} length {length} index {index} src={}",
                        kernels.level(),
                        src[index]
                    );
                } else {
                    #[allow(clippy::cast_possible_truncation)]
                    let (e32, a32) = (e as f32, a as f32);
                    assert!(
                        ulps(e32, a32) <= 1,
                        "{} length {length} index {index} src={}: {e} vs {a}",
                        kernels.level(),
                        src[index]
                    );
                    let rel = (e - a).abs() / e.abs();
                    assert!(
                        rel <= 1.0e-13,
                        "{} length {length} index {index} src={}: rel={rel:.3e}",
                        kernels.level(),
                        src[index]
                    );
                }
            }
        }
    }
}

#[test]
fn srgb_to_linear_matches_f64_oracle_on_dense_sweep() {
    let available = levels();
    // Accuracy against exact f64::powf across [-1.5, 1.5] and all 16-bit codes
    // for every available ISA level (including Scalar without FMA).
    let mut dense = Vec::with_capacity(200_001 + 65_536);
    for i in 0..=200_000_u32 {
        #[allow(clippy::cast_precision_loss)]
        dense.push(-1.5 + 3.0 * (i as f32 / 200_000.0));
    }
    for code in 0..=65_535_u16 {
        #[allow(clippy::cast_possible_truncation)]
        dense.push((f64::from(code) / 65_535.0) as f32);
    }
    for kernels in &available {
        let mut out = vec![0.0_f64; dense.len()];
        kernels.srgb_to_linear(&dense, &mut out).unwrap();
        let mut max_ulp = 0_u32;
        let mut max_rel = 0.0_f64;
        for (&x, &y) in dense.iter().zip(&out) {
            let ref_f64 = oracle_srgb_to_linear(x);
            #[allow(clippy::cast_possible_truncation)]
            let (y_f32, ref_f32) = (y as f32, ref_f64 as f32);
            max_ulp = max_ulp.max(ulps(y_f32, ref_f32));
            if ref_f64 == 0.0 {
                assert_eq!(y.to_bits(), ref_f64.to_bits());
            } else {
                max_rel = max_rel.max((y - ref_f64).abs() / ref_f64.abs());
            }
        }
        assert!(
            max_ulp <= 1 && max_rel < 1.4e-10,
            "{} srgb_to_linear max ULP={max_ulp}, max_rel={max_rel:.3e}",
            kernels.level()
        );
    }
}

/// Regression for PR #292 finding 2: SIMD tails and mixed-range vectors must
/// use the SIMD level's own fused multiply-add so every SIMD level (NEON,
/// AVX2, AVX-512) emits bit-identical `f64` (and narrowed `f32`) outputs at
/// every slice length and alignment. Pinned on `f32::from_bits(0x3eb04b7a)`,
/// which previously narrowed to `0x3dc6ef34` on NEON (2-wide vector) and
/// `0x3dc6ef33` on AVX2 (2-element tail using unfused scalar arithmetic).
#[test]
fn srgb_to_linear_simd_tails_and_mixed_vectors_are_bit_identical() {
    const PROBE_SIMD_F64_BITS: u64 = 0x3fb8_dde6_7000_0000;
    let probe = f32::from_bits(0x3eb0_4b7a);
    let expected_f32_bits = 0x3dc6_ef34_u32;

    for kernels in levels() {
        if kernels.level() == IsaLevel::Scalar {
            continue;
        }
        // Full 8-element SIMD vector must match the cross-ISA pinned 64-bit FMA result.
        let mut full = [0.0_f64; 8];
        kernels.srgb_to_linear(&[probe; 8], &mut full).unwrap();
        let ref_bits = PROBE_SIMD_F64_BITS;
        for (idx, &lane) in full.iter().enumerate() {
            assert_eq!(
                lane.to_bits(),
                ref_bits,
                "{} full vector idx={idx} bits mismatch for 0x3eb04b7a (got {:#018x})",
                kernels.level(),
                lane.to_bits()
            );
        }
        #[allow(clippy::cast_possible_truncation)]
        let narrowed = (full[0] as f32).to_bits();
        assert_eq!(
            narrowed,
            expected_f32_bits,
            "{} narrowed bits mismatch for 0x3eb04b7a",
            kernels.level()
        );

        // Every slice length 1..=17 (covering every tail length on NEON, AVX2, AVX-512).
        for len in 1..=17 {
            let src = vec![probe; len];
            let mut dst = vec![0.0_f64; len];
            kernels.srgb_to_linear(&src, &mut dst).unwrap();
            for (idx, &v) in dst.iter().enumerate() {
                assert_eq!(
                    v.to_bits(),
                    ref_bits,
                    "{} len={len} idx={idx} tail diverged from SIMD vector",
                    kernels.level()
                );
            }
        }

        // Mixed-range vectors where an out-of-range neighbor forces per-lane fallback.
        // `1497.0 / 1024.0 == 1.4619140625_f32` exactly in binary.
        let ext_probe = 1497.0_f32 / 1024.0;
        let mixed_cases: &[&[f32]] = &[
            &[1.5, probe],
            &[probe, -1.5],
            &[ext_probe, probe, -ext_probe, probe],
            &[
                probe, 0.5, ext_probe, probe, -ext_probe, probe, 2.5, probe, probe,
            ],
        ];
        for (case_idx, &src) in mixed_cases.iter().enumerate() {
            let mut dst = vec![0.0_f64; src.len()];
            kernels.srgb_to_linear(src, &mut dst).unwrap();
            for (idx, (&s, &d)) in src.iter().zip(&dst).enumerate() {
                if s.to_bits() == probe.to_bits() {
                    assert_eq!(
                        d.to_bits(),
                        ref_bits,
                        "{} mixed case {case_idx} idx={idx} diverged from SIMD vector",
                        kernels.level()
                    );
                } else if s.abs() > 1.0 {
                    // Outside `[-1, 1]`, every level delegates to the target's `f64::powf`.
                    assert_eq!(
                        d.to_bits(),
                        oracle_srgb_to_linear(s).to_bits(),
                        "{} mixed case {case_idx} idx={idx} extended sample {s} diverged from target oracle",
                        kernels.level()
                    );
                }
            }
        }
    }

    // Outside `[-1, 1]`, every level on this target (including Scalar) matches
    // the target-local `f64::powf` oracle bit for bit (including 1.4619140625,
    // whose platform `powf` can differ by 1 ULP across targets).
    let ext_probe = 1497.0_f32 / 1024.0;
    let extended = [ext_probe, -ext_probe, 1.5, -2.5, 65_504.0];
    for kernels in levels() {
        let mut dst = vec![0.0_f64; extended.len()];
        kernels.srgb_to_linear(&extended, &mut dst).unwrap();
        for (&s, &d) in extended.iter().zip(&dst) {
            assert_eq!(
                d.to_bits(),
                oracle_srgb_to_linear(s).to_bits(),
                "{} extended sample {s} diverged from target-local f64::powf oracle",
                kernels.level()
            );
        }
    }
}

#[test]
fn linear_to_srgb_u16_is_bit_identical_at_every_level_and_matches_f64_oracle() {
    let scalar = Kernels::for_level(IsaLevel::Scalar).unwrap();
    let available = levels();
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    let specials: [f32; 13] = [
        -1.0,
        -1.0e-7,
        -0.0,
        0.0,
        1.0e-10,
        0.003_130_7,
        0.003_130_8,
        0.003_130_9,
        0.214_041_14,
        0.5,
        0.999_999_9,
        1.0,
        1.5,
    ];
    for max in [255_u16, 1023, 4095, 65_535] {
        for length in (0..=40).chain([255, 256, 257, 4800, 4801]) {
            let src: Vec<f32> = (0..length)
                .map(|i| {
                    if i < specials.len() {
                        specials[i]
                    } else {
                        rng.unit().mul_add(1.4, -0.2)
                    }
                })
                .collect();
            let mut expected = vec![0_u16; length];
            scalar.linear_to_srgb_u16(&src, max, &mut expected).unwrap();
            for kernels in &available {
                let mut actual = vec![u16::MAX; length];
                kernels.linear_to_srgb_u16(&src, max, &mut actual).unwrap();
                assert_eq!(
                    expected,
                    actual,
                    "{} max={max} length={length}",
                    kernels.level()
                );
            }
        }
    }

    // Exact identity round-trip for every 10-, 12- and 16-bit code.
    let best = Kernels::best();
    for max in [255_u16, 1023, 4095, 65_535] {
        let encoded: Vec<f32> = (0..=max)
            .map(|code| {
                #[allow(clippy::cast_possible_truncation)]
                let value = (f64::from(code) / f64::from(max)) as f32;
                value
            })
            .collect();
        let mut linear = vec![0.0_f64; encoded.len()];
        let mut round_tripped = vec![0_u16; encoded.len()];
        best.srgb_to_linear(&encoded, &mut linear).unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let linear_f32: Vec<f32> = linear.iter().map(|&v| v as f32).collect();
        best.linear_to_srgb_u16(&linear_f32, max, &mut round_tripped)
            .unwrap();
        for (code, &got) in round_tripped.iter().enumerate() {
            assert_eq!(
                got,
                u16::try_from(code).unwrap(),
                "identity round-trip failed at max={max} code={code}"
            );
        }
    }

    // Dense comparison against f64::powf oracle at 16-bit: zero mismatches at every level.
    let n = 200_000_usize;
    let linear: Vec<f32> = (0..=n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let value = i as f32 / n as f32;
            value
        })
        .collect();
    for kernels in &available {
        let mut out = vec![0_u16; linear.len()];
        kernels
            .linear_to_srgb_u16(&linear, 65_535, &mut out)
            .unwrap();
        for (&x, &got) in linear.iter().zip(&out) {
            let expected = oracle_linear_to_srgb_u16(x, 65_535);
            assert_eq!(
                got,
                expected,
                "{} dense mismatch at linear={x} (0x{:08x})",
                kernels.level(),
                x.to_bits()
            );
        }
    }
}

/// Exhaustive half-code threshold neighborhood check: for every code boundary
/// `(k + 0.5) / max` at 8, 10, 12 and 16 bits, test the five neighboring `f32`
/// bit patterns around the inverse OETF across every available ISA level, plus
/// the specific regression value `f32::from_bits(0x3b4f_ad81)` at `max = 65535`.
#[test]
fn linear_to_srgb_u16_resolves_every_half_code_threshold_exactly() {
    let available = levels();

    // Regression case: polynomial without tie refinement returned 2682 instead of 2683.
    let regression = f32::from_bits(0x3b4f_ad81);
    assert_eq!(oracle_linear_to_srgb_u16(regression, 65_535), 2683);
    for lane in 0..9 {
        let mut src = vec![0.25_f32; lane + 1];
        src[lane] = regression;
        for kernels in &available {
            let mut out = vec![0_u16; src.len()];
            kernels.linear_to_srgb_u16(&src, 65_535, &mut out).unwrap();
            assert_eq!(
                out[lane],
                2683,
                "{} lane={lane} regression 0x3b4fad81",
                kernels.level()
            );
        }
    }

    // Edge depths max = 0 and max = 1 (finite inputs only).
    for max in [0_u16, 1] {
        let edge = [-0.5_f32, 0.0, 0.003_130_8, 0.214_041_14, 0.5, 1.0, 2.0];
        for kernels in &available {
            let mut out = vec![u16::MAX; edge.len()];
            kernels.linear_to_srgb_u16(&edge, max, &mut out).unwrap();
            for (&x, &got) in edge.iter().zip(&out) {
                assert_eq!(got, oracle_linear_to_srgb_u16(x, max));
            }
        }
    }

    // Every half-code threshold (k + 0.5) / max at 8, 10, 12 and 16 bits.
    for max in [255_u16, 1023, 4095, 65_535] {
        let mut candidates = Vec::with_capacity(usize::from(max) * 5);
        for k in 0..max {
            let encoded = (f64::from(k) + 0.5) / f64::from(max);
            let linear = if encoded <= 0.040_45 {
                encoded / 12.92
            } else {
                ((encoded + 0.055) / 1.055).powf(2.4)
            };
            #[allow(clippy::cast_possible_truncation)]
            let center_bits = (linear as f32).to_bits();
            for delta in -2_i32..=2 {
                let bits = center_bits.saturating_add_signed(delta);
                let sample = f32::from_bits(bits);
                if sample.is_finite() && (0.0..=1.0).contains(&sample) {
                    candidates.push(sample);
                }
            }
        }
        let expected: Vec<u16> = candidates
            .iter()
            .map(|&x| oracle_linear_to_srgb_u16(x, max))
            .collect();
        for kernels in &available {
            let mut actual = vec![u16::MAX; candidates.len()];
            kernels
                .linear_to_srgb_u16(&candidates, max, &mut actual)
                .unwrap();
            for (index, (&e, &a)) in expected.iter().zip(&actual).enumerate() {
                assert_eq!(
                    a,
                    e,
                    "{} max={max} index={index} linear=0x{:08x} ({})",
                    kernels.level(),
                    candidates[index].to_bits(),
                    candidates[index]
                );
            }
        }
    }
}

/// `Kernels::linear_to_srgb_u16` must reject `NaN`, `+inf`, and `-inf` with
/// `KernelError::NonFiniteSample` on every available ISA level, matching
/// `working_color::encode_linear_srgb`, while still clamping finite values
/// outside `[0, 1]`.
#[test]
fn linear_to_srgb_u16_rejects_non_finite_samples() {
    let available = levels();
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        for lane in 0..9 {
            let mut src = vec![0.5_f32; lane + 1];
            src[lane] = bad;
            for kernels in &available {
                let mut out = vec![0xBEEF_u16; src.len()];
                assert_eq!(
                    kernels.linear_to_srgb_u16(&src, 65_535, &mut out),
                    Err(KernelError::NonFiniteSample),
                    "{} lane={lane} bad={bad:?}",
                    kernels.level()
                );
            }
        }
    }
    // Finite values outside `[0, 1]` still clamp cleanly to `0` and `max`.
    let clamped = [-1.0e20_f32, -0.1, 1.1, 1.0e20];
    for kernels in &available {
        let mut out = [0_u16; 4];
        kernels
            .linear_to_srgb_u16(&clamped, 65_535, &mut out)
            .unwrap();
        assert_eq!(out, [0, 0, 65_535, 65_535], "{}", kernels.level());
    }
}

/// Regression for PR #292 round-10 finding 1: all-dark slices (`|x| <= 0.04045`)
/// take the linear-only loop without evaluating the polynomial or square roots,
/// stay bit-identical across every ISA level and slice length (and identical to
/// dark lanes inside mixed bright+dark vectors), and never trigger on the first
/// `f32` above `0.04045`.
#[test]
fn srgb_to_linear_all_dark_chunks_are_bit_identical_across_levels() {
    let available = levels();
    let dark_samples: [f32; 11] = [
        -0.040_45_f32,
        -0.01,
        -1.0e-7,
        -f32::MIN_POSITIVE,
        -0.0,
        0.0,
        f32::MIN_POSITIVE,
        1.0e-7,
        0.01,
        0.040_449_996,
        0.040_45_f32,
    ];
    for len in (1..=17).chain([64, 192]) {
        let src: Vec<f32> = (0..len)
            .map(|i| dark_samples[i % dark_samples.len()])
            .collect();
        let expected_bits: Vec<u64> = src
            .iter()
            .map(|&s| (f64::from(s) * (1.0 / 12.92)).to_bits())
            .collect();
        for kernels in &available {
            let mut dst = vec![0.0_f64; len];
            kernels.srgb_to_linear(&src, &mut dst).unwrap();
            for (idx, (&got, &want)) in dst.iter().zip(&expected_bits).enumerate() {
                assert_eq!(
                    got.to_bits(),
                    want,
                    "{} dark len={len} idx={idx} sample={:?}",
                    kernels.level(),
                    src[idx]
                );
                let oracle = oracle_srgb_to_linear(src[idx]);
                if oracle == 0.0 {
                    assert_eq!(got.to_bits(), oracle.to_bits());
                } else {
                    assert!((got - oracle).abs() / oracle.abs() < 1.0e-15);
                }
            }
        }
    }
    // Every dark sample must emit the exact same bits inside a mixed bright+dark slice.
    for &sample in &dark_samples {
        let mixed = [0.5_f32, sample, -0.5, sample, 0.9, sample, 0.25, sample];
        let want = (f64::from(sample) * (1.0 / 12.92)).to_bits();
        for kernels in &available {
            let mut dst = [0.0_f64; 8];
            kernels.srgb_to_linear(&mixed, &mut dst).unwrap();
            for lane in [1, 3, 5, 7] {
                assert_eq!(
                    dst[lane].to_bits(),
                    want,
                    "{} mixed dark lane={lane} sample={sample:?}",
                    kernels.level()
                );
            }
        }
    }
    // First `f32` strictly above `0.04045_f64` must enter the curved polynomial path.
    let just_above = f32::from_bits(0.040_45_f32.to_bits() + 1);
    assert!(f64::from(just_above) > 0.040_45_f64);
    let oracle_above = oracle_srgb_to_linear(just_above);
    for kernels in &available {
        let mut dst = [0.0_f64; 8];
        kernels.srgb_to_linear(&[just_above; 8], &mut dst).unwrap();
        for &v in &dst {
            assert!(
                (v - oracle_above).abs() / oracle_above.abs() < 1.4e-10,
                "{} just_above 0.04045 diverged",
                kernels.level()
            );
        }
    }
}

/// Regression for PR #292 round-12 finding 2: exact half-codes on the linear
/// OETF segment (`<= 0.0031308`, such as `4.5_f32 / (65535.0 * 12.92)`) skip
/// refinement both in all-linear slices and in mixed linear+curved vectors,
/// while matching `oracle_linear_to_srgb_u16` bit for bit across every ISA
/// level and bit depth.
#[test]
fn linear_to_srgb_u16_linear_segment_half_codes_match_oracle_exactly() {
    let available = levels();
    for max in [255_u16, 1023, 4095, 65_535] {
        let mut linear_only = vec![-0.5_f32, -0.0_f32, 0.0_f32, 4.5_f32 / (65_535.0 * 12.92)];
        for k in 0..200_u32 {
            #[allow(clippy::cast_possible_truncation)]
            let mid = ((f64::from(k) + 0.5) / (f64::from(max) * 12.92)) as f32;
            if f64::from(mid) <= 0.003_130_8 {
                for delta in -2_i32..=2 {
                    let candidate = f32::from_bits(mid.to_bits().wrapping_add_signed(delta));
                    if candidate.is_finite() && f64::from(candidate) <= 0.003_130_8 {
                        linear_only.push(candidate);
                    }
                }
            }
        }
        // Check all-linear slice (hits the `all_oetf_linear` fast path) and
        // mixed slices (interleaving dark half-codes with `0.5_f32` and the
        // threshold boundary `0.0031308_f32.next_up()`).
        let mut mixed = Vec::with_capacity(linear_only.len() * 2 + 4);
        for &v in &linear_only {
            mixed.push(v);
            mixed.push(0.5_f32);
        }
        let thresh = 0.003_130_8_f32;
        mixed.extend_from_slice(&[thresh.next_down(), thresh, thresh.next_up(), 1.0]);

        for (label, slice) in [("all_linear", &linear_only), ("mixed", &mixed)] {
            let expected: Vec<u16> = slice
                .iter()
                .map(|&s| oracle_linear_to_srgb_u16(s, max))
                .collect();
            for kernels in &available {
                let mut actual = vec![u16::MAX; slice.len()];
                kernels.linear_to_srgb_u16(slice, max, &mut actual).unwrap();
                assert_eq!(
                    actual,
                    expected,
                    "{} max={max} {label} mismatch",
                    kernels.level()
                );
            }
        }
    }

    // Regression for PR #292 round-13 finding 1: mixed near-tie curved lane
    // (`0x3b4fad81` at 16-bit -> `2683`), normal curved lanes (`0.5`, `0.75`),
    // and linear-segment half-code lanes across every lane alignment `0..=15`
    // and slice length `1..=19`, verifying per-lane masked refinement matches
    // the exact oracle on every ISA level.
    let near_tie = f32::from_bits(0x3b4f_ad81);
    let dark_half = 4.5_f32 / (65_535.0 * 12.92);
    let pattern = [near_tie, 0.5_f32, 0.75_f32, dark_half, 0.25_f32];
    for len in 1..=19 {
        for offset in 0..pattern.len() {
            let slice: Vec<f32> = (0..len)
                .map(|i| pattern[(i + offset) % pattern.len()])
                .collect();
            let expected: Vec<u16> = slice
                .iter()
                .map(|&s| oracle_linear_to_srgb_u16(s, 65_535))
                .collect();
            for kernels in &available {
                let mut actual = vec![u16::MAX; len];
                kernels
                    .linear_to_srgb_u16(&slice, 65_535, &mut actual)
                    .unwrap();
                assert_eq!(
                    actual,
                    expected,
                    "{} mixed near-tie len={len} offset={offset}",
                    kernels.level()
                );
            }
        }
    }
}
