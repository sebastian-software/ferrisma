//! WP3 differential test: the inline helpers of `src/dsp/lossless_common.h`
//! that WP3 translates: sub-sampled sizes, near-lossless bits, rounded shifts
//! and divisions, the prefix encoding without lookup table, the per-channel
//! pixel sums and differences, and the color-map accessors. One script drives
//! the C probe `tools/oracle/probes/wp3/lossless_common.c` and the Rust
//! translation, which builds the expected output.
//!
//! Excluded inputs, where C is undefined: shifts of 64 or more for
//! `RightShiftRound`, a zero divisor or a quotient that overflows for
//! `DivRound`, and distances below 3 for the prefix encoding.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::lossless_common::{
    div_round, right_shift_round, vp8_get_alpha_index, vp8_get_alpha_value, vp8_get_argb_index,
    vp8_get_argb_value, vp8l_add_pixels, vp8l_near_lossless_bits, vp8l_prefix_encode_bits_no_lut,
    vp8l_prefix_encode_no_lut, vp8l_sub_pixels, vp8l_sub_sample_size,
};

/// SplitMix64, a small deterministic generator for the script.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0053);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut emit = |command: String, result: String| {
        commands += &command;
        commands += "\n";
        expected += &result;
        expected += "\n";
    };

    for bits in 0..10_u32 {
        for size in [
            0_u32,
            1,
            2,
            3,
            15,
            16,
            17,
            255,
            16_383,
            16_384,
            u32::MAX - 3,
            u32::MAX,
        ] {
            emit(
                format!("subsample {size} {bits}"),
                format!("sub={}", vp8l_sub_sample_size(size, bits)),
            );
        }
    }
    for quality in -20..=120 {
        emit(
            format!("near {quality}"),
            format!("near={}", vp8l_near_lossless_bits(quality)),
        );
    }
    for _ in 0..400 {
        let v = match rng.below(3) {
            0 => rng.next_u64(),
            1 => rng.below(1 << 40),
            _ => u64::MAX - rng.below(1 << 20),
        };
        let shift = rng.below(64) as u32;
        emit(
            format!("rsr {v} {shift}"),
            format!("rsr={}", right_shift_round(v, shift)),
        );
    }
    for _ in 0..400 {
        // |a| < 2^62 and |b| < 2^40, so that a +- b / 2 cannot overflow.
        let a = (rng.next_u64() as i64) >> (1 + rng.below(40));
        let mut b = (rng.next_u64() as i64) >> (24 + rng.below(39));
        if b == 0 {
            b = -7;
        }
        emit(
            format!("divround {a} {b}"),
            format!("div={}", div_round(a, b)),
        );
    }
    let mut distances: Vec<i32> = (3..1200).collect();
    distances.extend((0..300).map(|_| 3 + rng.below((1 << 30) - 3) as i32));
    distances.extend([i32::MAX, 1 << 20, (1 << 20) + 1]);
    for distance in distances {
        let (code, extra_bits) = vp8l_prefix_encode_bits_no_lut(distance);
        let (full_code, full_extra, value) = vp8l_prefix_encode_no_lut(distance);
        emit(
            format!("prefix {distance}"),
            format!("bits={code},{extra_bits}\nfull={full_code},{full_extra},{value}"),
        );
    }
    for _ in 0..400 {
        let a = rng.next_u64() as u32;
        let b = rng.next_u64() as u32;
        emit(
            format!("pixels {a} {b}"),
            format!(
                "add={}\nsub={}",
                vp8l_add_pixels(a, b),
                vp8l_sub_pixels(a, b)
            ),
        );
    }
    for _ in 0..200 {
        let v = rng.next_u64() as u32;
        emit(
            format!("index {v}"),
            format!(
                "index={},{},{},{}",
                vp8_get_argb_index(v),
                vp8_get_alpha_index(v as u8),
                vp8_get_argb_value(v),
                vp8_get_alpha_value(v)
            ),
        );
    }
    (commands, expected)
}

#[test]
fn lossless_common_helpers_match_the_c_header() {
    let Some(oracle) = Oracle::locate("lossless_common_helpers_match_the_c_header") else {
        return;
    };
    let (commands, expected) = script();
    let output = oracle
        .probe("wp3", "lossless_common")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let differs = expected
        .lines()
        .zip(actual.lines())
        .position(|(e, a)| e != a)
        .map_or_else(String::new, |index| {
            format!(
                "line {index}: expected {}, the C probe printed {}",
                expected.lines().nth(index).unwrap_or_default(),
                actual.lines().nth(index).unwrap_or_default()
            )
        });
    assert!(actual == expected, "the helpers differ from C: {differs}");
}
