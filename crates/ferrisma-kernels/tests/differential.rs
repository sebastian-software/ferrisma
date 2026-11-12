//! Every available level against the scalar body (ADR-0020 decision 7).
//! Levels the host cannot run are printed as untested, never skipped
//! silently.

use ferrisma_kernels::{IsaLevel, KernelError, Kernels, Taps};

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
            for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
                if exact {
                    assert_eq!(
                        e.to_bits(),
                        a.to_bits(),
                        "{} index {index}",
                        kernels.level()
                    );
                } else {
                    // Reassociated sums: bound relative to the magnitude
                    // of the summed terms, since cancellation can make the
                    // result itself tiny.
                    let tolerance =
                        4.0 * f32::EPSILON * f32::from(u16::try_from(max_taps).unwrap());
                    assert!(
                        ulps(*e, *a) <= 4 || (e - a).abs() <= tolerance,
                        "{} index {index}: {e} vs {a}",
                        kernels.level()
                    );
                }
            }
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
