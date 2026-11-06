//! WP4 differential test: the spatial prediction filters and their unfilters
//! (`src/dsp/filters.c`) and the filter estimate `WebPEstimateBestFilter`
//! (`src/utils/filters_utils.c`). One script of commands drives both the C
//! probe `tools/oracle/probes/wp4/filters.c` and the Rust translation. The
//! filters run over padded planes of odd sizes, the unfilters over every filter
//! with and without a previous row and in every aliasing form the C code
//! allows, and the estimate over seeded random and smooth planes.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::dsp::WEBP_FILTER_TYPE;
use webp::dsp::filters::{webp_filters, webp_unfilters};
use webp::utils::filters_utils::webp_estimate_best_filter;

/// The sentinel that the probe fills destinations with.
const SENTINEL: u8 = 0xaa;

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

    fn bytes(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| self.next_u64() as u8).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// A byte array as the probe reads it: its hex digits, or `-` when empty.
fn hex_arg(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

fn field(key: &str, bytes: &[u8]) -> String {
    format!("{key}={}\n", hex(bytes))
}

/// A seeded plane: random bytes, or a smooth ramp with a little noise.
fn plane(rng: &mut Rng, len: usize, smooth: bool) -> Vec<u8> {
    if smooth {
        (0..len)
            .map(|index| ((index * 3 + index / 7) as u8).wrapping_add((rng.next_u64() % 3) as u8))
            .collect()
    } else {
        rng.bytes(len)
    }
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0007);
    let mut commands = String::from("init\n");
    let mut expected = String::new();

    // Forward filters over padded planes of odd sizes.
    for filter in 1..=3_u32 {
        for &(width, height) in &[
            (1, 1),
            (2, 3),
            (3, 2),
            (5, 5),
            (8, 1),
            (11, 4),
            (17, 6),
            (33, 2),
        ] {
            for smooth in [false, true] {
                let stride = width + 3;
                let input = plane(&mut rng, stride * height, smooth);
                commands.push_str(&format!(
                    "forward {filter} {width} {height} {stride} {}\n",
                    hex_arg(&input)
                ));
                let mut out = vec![SENTINEL; stride * height];
                webp_filters(
                    WEBP_FILTER_TYPE(filter),
                    &input,
                    i32::try_from(width).expect("int"),
                    i32::try_from(height).expect("int"),
                    i32::try_from(stride).expect("int"),
                    &mut out,
                );
                expected.push_str(&field("out", &out));
            }
        }
    }

    // Unfilters of every filter, with and without a previous row, and with the
    // input and the output in the aliasing forms that the C code allows.
    for filter in 0..=3_u32 {
        for width in (1..=20).chain([33]) {
            for has_prev in [false, true] {
                for alias in 0..=2_u32 {
                    if alias == 2 && !has_prev {
                        continue;
                    }
                    let prev = rng.bytes(width);
                    let input = rng.bytes(width);
                    let out0 = rng.bytes(width);
                    commands.push_str(&format!(
                        "unfilter {filter} {width} {} {alias} {} {} {}\n",
                        u8::from(has_prev),
                        hex_arg(&prev),
                        hex_arg(&input),
                        hex_arg(&out0)
                    ));
                    let width_int = i32::try_from(width).expect("int");
                    let mut out = out0.clone();
                    let filter_type = WEBP_FILTER_TYPE(filter);
                    match alias {
                        0 => {
                            let mut dst = out0.clone();
                            webp_unfilters(
                                filter_type,
                                has_prev.then_some(prev.as_slice()),
                                &input,
                                &mut dst,
                                width_int,
                            );
                            out = dst;
                        }
                        1 => {
                            // In place: the input is the output, as C reads it
                            // before it writes each element.
                            let snapshot = out0.clone();
                            webp_unfilters(
                                filter_type,
                                has_prev.then_some(prev.as_slice()),
                                &snapshot,
                                &mut out,
                                width_int,
                            );
                        }
                        _ => {
                            // The previous row is the output: prev is a snapshot of it.
                            let snapshot = out0.clone();
                            webp_unfilters(
                                filter_type,
                                Some(snapshot.as_slice()),
                                &input,
                                &mut out,
                                width_int,
                            );
                        }
                    }
                    expected.push_str(&field("out", &out));
                }
            }
        }
    }

    // The filter estimate over seeded planes, including plane sizes too small to sample.
    for &(width, height) in &[(1, 1), (3, 3), (4, 4), (5, 7), (16, 9), (33, 12), (64, 8)] {
        for smooth in [false, true] {
            for stride_pad in [0, 5] {
                let stride = width + stride_pad;
                let data = plane(&mut rng, stride * height, smooth);
                commands.push_str(&format!(
                    "estimate {width} {height} {stride} {}\n",
                    hex_arg(&data)
                ));
                let filter = webp_estimate_best_filter(
                    &data,
                    i32::try_from(width).expect("int"),
                    i32::try_from(height).expect("int"),
                    i32::try_from(stride).expect("int"),
                );
                expected.push_str(&format!("filter={}\n", filter.0));
            }
        }
    }
    (commands, expected)
}

/// The first line where the two outputs differ, for the failure message.
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
fn filters_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("filters_match_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 300,
        "the script has {observations} observations"
    );
    let probe = oracle.probe("wp4", "filters");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the filters differ from the C functions: {}",
        first_difference(&expected, &actual)
    );
}
