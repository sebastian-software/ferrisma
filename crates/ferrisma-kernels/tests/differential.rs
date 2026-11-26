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
