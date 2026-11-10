//! WP5 differential test: the three SharpYUV dispatch functions of
//! `sharpyuv/sharpyuv_dsp.c`: `SharpYuvUpdateY`, `SharpYuvUpdateRGB` and
//! `SharpYuvFilterRow`. The C probe `tools/oracle/probes/wp5/dsp.c` runs them on
//! 200 seeded cases, and this test draws the same inputs from the same generator
//! and compares the outputs, including the entries beyond the length.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::sharpyuv::sharpyuv_dsp::{
    sharp_yuv_filter_row_c, sharp_yuv_update_rgb, sharp_yuv_update_y,
};

const CASES: u64 = 200;
const MAX_LEN: usize = 40;

/// SplitMix64, the generator of `tools/oracle/probes/wp5/support/wp5_rng.c`.
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

    fn sample(&mut self, bit_depth: i32) -> u16 {
        self.below(1_u64 << bit_depth) as u16
    }

    fn chroma(&mut self) -> i16 {
        (self.below(32768) as i32 - 16384) as i16
    }
}

fn hex_u16(values: &[u16]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn hex_i16(values: &[i16]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

/// Compares two observation lists and names the group and key of the first difference.
fn assert_same_observations(actual: &Fields, expected: &Fields, what: &str) {
    assert_eq!(
        actual.0.len(),
        expected.0.len(),
        "{what}: number of observations"
    );
    let mut group = String::new();
    for (index, (a, e)) in actual.0.iter().zip(&expected.0).enumerate() {
        if a.0 == "group" {
            group.clone_from(&a.1);
        }
        assert!(
            a == e,
            "{what}: group {group}, observation {index} ({}) differs: C {:.200} / Rust {:.200}",
            a.0,
            a.1,
            e.1
        );
    }
}

#[test]
fn dispatch_functions_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("dispatch_functions_match_the_c_functions") else {
        return;
    };
    let no_args: [&str; 0] = [];
    let output = oracle.probe("wp5", "dsp").run(&no_args, b"");
    let actual = output.success().fields();

    let mut expected = Fields::new();
    for case in 0..CASES {
        // One generator stream per case, drawn in the order of the C probe.
        let mut rng = Rng(0x5eed + case);
        let bit_depth = 10 + 2 * rng.below(3) as i32;

        // update-y
        let len = 1 + rng.below(MAX_LEN as u64) as i32;
        let src: Vec<u16> = (0..MAX_LEN).map(|_| rng.sample(bit_depth)).collect();
        let reference: Vec<u16> = (0..MAX_LEN).map(|_| rng.sample(bit_depth)).collect();
        let mut dst: Vec<u16> = (0..MAX_LEN).map(|_| rng.sample(bit_depth)).collect();
        let diff = sharp_yuv_update_y(&src, &reference, &mut dst, len, bit_depth);
        expected = expected
            .with("group", format!("update-y-{case}"))
            .with("diff", diff)
            .hex("dst", &hex_u16(&dst));

        // update-rgb
        let len = 1 + rng.below(MAX_LEN as u64) as i32;
        let src: Vec<i16> = (0..MAX_LEN).map(|_| rng.chroma()).collect();
        let reference: Vec<i16> = (0..MAX_LEN).map(|_| rng.chroma()).collect();
        let mut dst: Vec<i16> = (0..MAX_LEN).map(|_| rng.chroma()).collect();
        sharp_yuv_update_rgb(&src, &reference, &mut dst, len);
        expected = expected
            .with("group", format!("update-rgb-{case}"))
            .hex("dst", &hex_i16(&dst));

        // filter-row
        let len = 1 + rng.below(20) as i32;
        let a: Vec<i16> = (0..=MAX_LEN).map(|_| rng.chroma()).collect();
        let b: Vec<i16> = (0..=MAX_LEN).map(|_| rng.chroma()).collect();
        let best_y: Vec<u16> = (0..2 * MAX_LEN + 2)
            .map(|_| rng.sample(bit_depth))
            .collect();
        let mut out = vec![0_u16; 2 * MAX_LEN + 2];
        // The C translation `SharpYuvFilterRow_C`, which the case verifies:
        // with `simd` on 64-bit ARM the dispatch selects upstream's NEON
        // kernel, whose 16-bit filter (bit depth 10) wraps for chroma values
        // of this magnitude, which `SharpYuvConvert` never produces (WP20;
        // `tests/wp20_sharpyuv.rs` covers the NEON kernel).
        sharp_yuv_filter_row_c(&a, &b, len, &best_y, &mut out, bit_depth);
        expected = expected
            .with("group", format!("filter-row-{case}"))
            .hex("out", &hex_u16(&out));
    }
    assert_same_observations(&actual, &expected, "dispatch functions");
}
