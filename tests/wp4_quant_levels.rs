//! WP4 differential test: `QuantizeLevels` (`src/utils/quant_levels_utils.c`)
//! and `WebPDequantizeLevels` (`src/utils/quant_levels_dec_utils.c`). The C
//! probe `tools/oracle/probes/wp4/quant_levels.c` runs in its own process.
//! `QuantizeLevels` is compared on seeded planes of odd sizes with few and many
//! levels, every level count from the valid range and the invalid ones, and
//! missing planes. `WebPDequantizeLevels` is compared on planes with few widely
//! spaced levels, which the filter changes, at every strength and with the
//! stride padded.
//!
//! The dequantizer's scratch memory is read before it is written: the first
//! rows of its ring are read by `VFilter` before any row has been stored there.
//! That read is undefined in C; the Rust translation zeroes the scratch. The
//! probe was also run with `MallocScribble=1`, which fills new allocations with
//! 0xaa, and the output did not change over 400 random planes; the cases here
//! are compared as the C build prints them, and the status page records the
//! finding.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::utils::quant_levels_dec_utils::webp_dequantize_levels;
use webp::utils::quant_levels_utils::quantize_levels;

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

    /// A plane with `levels` distinct values, spread over the byte range.
    fn plane(&mut self, len: usize, levels: u64) -> Vec<u8> {
        let step = 255 / levels.max(1);
        (0..len)
            .map(|_| (self.next_u64() % levels.max(1) * step) as u8)
            .collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_arg(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

fn first_difference(expected: &str, actual: &str) -> String {
    let mut expected_lines = expected.lines();
    let mut actual_lines = actual.lines();
    for index in 0.. {
        match (expected_lines.next(), actual_lines.next()) {
            (None, None) => return "the outputs are equal".to_owned(),
            (Some(e), Some(a)) if e == a => continue,
            (e, a) => {
                return format!("line {index}: expected {e:?}, the C probe printed {a:?}");
            }
        }
    }
    unreachable!()
}

#[test]
fn quantize_levels_matches_the_c_function() {
    let Some(oracle) = Oracle::locate("quantize_levels_matches_the_c_function") else {
        return;
    };
    let mut rng = Rng(0x5eed_0009);
    let mut commands = String::new();
    let mut expected = String::new();
    let shapes = [(1, 1), (3, 2), (5, 7), (8, 8), (17, 3), (33, 5), (64, 4)];
    let level_counts = [1_i32, 2, 3, 4, 7, 16, 255, 256, 257];
    for &(width, height) in &shapes {
        for &levels in &level_counts {
            for distinct in [2_u64, 3, 5, 40, 256] {
                for want_sse in [false, true] {
                    let size = (width * height) as usize;
                    let plane = rng.plane(size, distinct);
                    commands.push_str(&format!(
                        "quantize {width} {height} {levels} {} {}\n",
                        u8::from(want_sse),
                        hex_arg(&plane)
                    ));
                    let mut data = plane.clone();
                    let mut sse = 0_u64;
                    let ret = quantize_levels(
                        Some(&mut data),
                        width,
                        height,
                        levels,
                        if want_sse { Some(&mut sse) } else { None },
                    );
                    expected.push_str(&format!("ret={}\n", u8::from(ret)));
                    expected.push_str(&format!("data={}\n", hex(&data)));
                    if want_sse {
                        expected.push_str(&format!("sse={sse}\n"));
                    }
                }
            }
        }
    }
    // Missing planes and invalid dimensions leave the plane and the sse unchanged.
    for &(width, height, levels) in &[(0_i32, 3_i32, 4_i32), (3, 0, 4), (-1, 2, 4), (2, 2, 1)] {
        commands.push_str(&format!("quantize {width} {height} {levels} 1 -\n"));
        let mut data = vec![
            0_u8;
            if width > 0 && height > 0 {
                (width * height) as usize
            } else {
                0
            }
        ];
        let mut sse = 0_u64;
        let ret = quantize_levels(Some(&mut data), width, height, levels, Some(&mut sse));
        expected.push_str(&format!("ret={}\n", u8::from(ret)));
        expected.push_str(&format!("data={}\n", hex(&data)));
        expected.push_str(&format!("sse={sse}\n"));
    }
    commands.push_str("quantize 2 2 4 1 null\n");
    let mut sse = 0_u64;
    let ret = quantize_levels(None, 2, 2, 4, Some(&mut sse));
    expected.push_str(&format!("ret={}\nsse={sse}\n", u8::from(ret)));

    let probe = oracle.probe("wp4", "quant_levels");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let observations = expected.lines().count();
    assert!(
        observations > 500,
        "the script has {observations} observations"
    );
    assert!(
        actual == expected,
        "QuantizeLevels differs from the C function: {}",
        first_difference(&expected, &actual)
    );
}

#[test]
fn dequantize_levels_matches_the_c_function() {
    let Some(oracle) = Oracle::locate("dequantize_levels_matches_the_c_function") else {
        return;
    };
    let probe = oracle.probe("wp4", "quant_levels");
    let mut rng = Rng(0x5eed_000a);
    let shapes: [(i32, i32); 7] = [(1, 1), (2, 2), (5, 3), (9, 9), (16, 7), (33, 4), (48, 12)];
    let strengths = [-1_i32, 0, 1, 25, 50, 75, 100, 101];
    let mut commands = String::new();
    let mut expected = String::new();
    for &(width, height) in &shapes {
        for &strength in &strengths {
            for levels in [2_u64, 3, 6, 24] {
                let stride = width + 3;
                let plane = rng.plane(usize::try_from(stride * height).expect("positive"), levels);
                commands.push_str(&format!(
                    "dequant {width} {height} {stride} {strength} {}\n",
                    hex_arg(&plane)
                ));
                let mut data = plane.clone();
                let ret = webp_dequantize_levels(Some(&mut data), width, height, stride, strength);
                expected.push_str(&format!("ret={}\ndata={}\n", u8::from(ret), hex(&data)));
            }
        }
    }
    // Missing planes and invalid parameters return 0 without touching the plane.
    for &(width, height, strength) in &[(0_i32, 3_i32, 10_i32), (3, 0, 10), (2, 2, -1), (2, 2, 101)]
    {
        commands.push_str(&format!("dequant {width} {height} 2 {strength} -\n"));
        // The probe prints the stride * height samples of a valid size, and none otherwise.
        let samples = if width > 0 && height > 0 {
            2 * height as usize
        } else {
            0
        };
        let mut data = vec![0_u8; samples];
        let ret = webp_dequantize_levels(Some(&mut data), width, height, 2, strength);
        expected.push_str(&format!("ret={}\ndata={}\n", u8::from(ret), hex(&data)));
    }
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let cases = expected
        .lines()
        .filter(|line| line.starts_with("ret="))
        .count();
    assert!(cases > 200, "only {cases} cases were compared");
    assert!(
        actual == expected,
        "WebPDequantizeLevels differs from the C function: {}",
        first_difference(&expected, &actual)
    );
}
