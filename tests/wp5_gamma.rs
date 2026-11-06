//! WP5 differential test: the transfer functions and the gamma tables of
//! `sharpyuv/sharpyuv_gamma.c` (`SharpYuvGammaToLinear` and
//! `SharpYuvLinearToGamma`). The C probe `tools/oracle/probes/wp5/gamma.c` sweeps
//! every valid transfer function over the bit depths of the conversion, with the
//! same strides as this test, and prints the values as little-endian bytes. The
//! test compares those bytes with the Rust values, group by group.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::sharpyuv::sharpyuv::SharpYuvTransferFunctionType;
use webp::sharpyuv::sharpyuv_gamma::{
    sharp_yuv_gamma_to_linear, sharp_yuv_init_gamma_tables, sharp_yuv_linear_to_gamma,
};

/// The valid transfer functions: 1 and 4 to 18. The values 0, 2 and 3 are reserved
/// or unspecified, and 19 is the count; the C switch asserts on them.
const TRANSFERS: [u32; 16] = [1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
const BIT_DEPTHS: [i32; 4] = [8, 10, 12, 16];
const SRGB: u32 = 13;

/// The inputs of one sweep: every value up to `max` with the given stride, and
/// `max` itself when the stride misses it.
fn sweep(max: u32, step: u32) -> Vec<u32> {
    let mut values: Vec<u32> = (0..=max).step_by(step as usize).collect();
    if !max.is_multiple_of(step) {
        values.push(max);
    }
    values
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
        if a != e {
            let position = a.1.bytes().zip(e.1.bytes()).position(|(x, y)| x != y);
            panic!(
                "{what}: group {group}, observation {index} ({}) differs at hex character {position:?}: C {:.120} / Rust {:.120}",
                a.0, a.1, e.1
            );
        }
    }
}

#[test]
fn gamma_and_transfer_tables_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("gamma_and_transfer_tables_match_the_c_functions") else {
        return;
    };
    let no_args: [&str; 0] = [];
    let output = oracle.probe("wp5", "gamma").run(&no_args, b"");
    let actual = output.success().fields();

    sharp_yuv_init_gamma_tables();
    let mut expected = Fields::new();
    let mut groups = 0;
    for &transfer in &TRANSFERS {
        for &bit_depth in &BIT_DEPTHS {
            // Exhaustive up to 12 bits and for sRGB; every 257th value otherwise.
            let max = (1_u32 << bit_depth) - 1;
            let step = if transfer == SRGB || bit_depth <= 12 {
                1
            } else {
                257
            };
            let mut bytes = Vec::new();
            for value in sweep(max, step) {
                let linear = sharp_yuv_gamma_to_linear(
                    value as u16,
                    bit_depth,
                    SharpYuvTransferFunctionType(transfer),
                );
                bytes.extend_from_slice(&linear.to_le_bytes());
            }
            expected = expected
                .with("group", format!("gamma-to-linear-t{transfer}-b{bit_depth}"))
                .hex("hex", &bytes);
            groups += 1;

            // The other transfer functions stop at 65535; see docs/status/wp5.md.
            let top = if transfer == SRGB {
                1_u32 << 16
            } else {
                65_535
            };
            let step = if transfer == SRGB { 1 } else { 257 };
            let mut bytes = Vec::new();
            for value in sweep(top, step) {
                let gamma = sharp_yuv_linear_to_gamma(
                    value,
                    bit_depth,
                    SharpYuvTransferFunctionType(transfer),
                );
                bytes.extend_from_slice(&gamma.to_le_bytes());
            }
            expected = expected
                .with("group", format!("linear-to-gamma-t{transfer}-b{bit_depth}"))
                .hex("hex", &bytes);
            groups += 1;
        }
    }
    assert_eq!(
        groups, 128,
        "the sweep covers 16 transfer functions, 4 bit depths and 2 tables"
    );
    assert_same_observations(&actual, &expected, "gamma tables");
}
