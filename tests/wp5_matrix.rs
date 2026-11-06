//! WP5 differential test: the color space and matrix functions of
//! `sharpyuv/sharpyuv_csp.c` and `sharpyuv/sharpyuv.c`: `SharpYuvGetVersion`,
//! `SharpYuvGetConversionMatrix`, `SharpYuvComputeConversionMatrix` and
//! `SharpYuvOptionsInitInternal`. The C probe `tools/oracle/probes/wp5/csp.c` runs
//! the inventory of libavif-rs `docs/SHARPYUV-VALIDATION.md` (version, predefined
//! and computed matrices, options version checks) and 256 seeded color spaces.
//!
//! The NULL arguments of `SharpYuvOptionsInitInternal` are not compared: the Rust
//! API takes references, so they exist only at the C ABI.

#[path = "support/oracle.rs"]
mod oracle;

use std::ptr;

use oracle::{Fields, Oracle};
use webp::sharpyuv::sharpyuv::{
    SHARPYUV_VERSION, SharpYuvConversionMatrix, SharpYuvTransferFunctionType,
    sharp_yuv_get_version, sharp_yuv_options_init, sharp_yuv_options_init_internal,
};
use webp::sharpyuv::sharpyuv_csp::{
    SharpYuvColorSpace, SharpYuvMatrixType, SharpYuvRange, sharp_yuv_compute_conversion_matrix,
    sharp_yuv_get_conversion_matrix,
};

/// The version of the C probe's `SharpYuvOptionsInitInternal` checks.
const OPTION_VERSIONS: [u32; 6] = [0x40002, 0x40000, 0x400ff, 0x30000, 0x0100_0000, 0];

/// The inventory's (Kr, Kb) pairs; the C probe rounds them to float.
const INVENTORY_KR: [f64; 3] = [0.299, 0.2126, 0.2627];
const INVENTORY_KB: [f64; 3] = [0.114, 0.0722, 0.0593];

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
}

/// The twelve coefficients, comma-separated, in the order of the C probe.
fn values(matrix: &SharpYuvConversionMatrix) -> String {
    matrix
        .rgb_to_y
        .iter()
        .chain(&matrix.rgb_to_u)
        .chain(&matrix.rgb_to_v)
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn group(fields: Fields, name: &str, matrix: &SharpYuvConversionMatrix) -> Fields {
    fields.with("group", name).with("values", values(matrix))
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
fn version_matrices_and_options_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("version_matrices_and_options_match_the_c_functions") else {
        return;
    };
    let no_args: [&str; 0] = [];
    let output = oracle.probe("wp5", "csp").run(&no_args, b"");
    let actual = output.success().fields();

    let mut expected = Fields::new()
        .with("group", "version")
        .with("value", sharp_yuv_get_version());
    assert_eq!(sharp_yuv_get_version(), SHARPYUV_VERSION);
    let mut groups = 1;

    for index in 0..5 {
        let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(index))
            .expect("the five predefined matrices exist");
        expected = group(expected, &format!("predefined-matrix-{index}"), matrix);
        groups += 1;
    }

    // Out-of-range types, including the value of kSharpYuvMatrixNum.
    let invalid = [5_u32, 6, 0xffff_ffff].map(|index| {
        i32::from(sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(index)).is_some())
    });
    expected = expected.with("group", "invalid-matrix").with(
        "values",
        invalid
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    groups += 1;

    let mut index = 0;
    for kr_kb in 0..3 {
        for depth in [8, 10, 12] {
            for range in [0_u32, 1] {
                let color_space = SharpYuvColorSpace {
                    kr: INVENTORY_KR[kr_kb] as f32,
                    kb: INVENTORY_KB[kr_kb] as f32,
                    bit_depth: depth,
                    range: SharpYuvRange(range),
                };
                let matrix = sharp_yuv_compute_conversion_matrix(&color_space);
                expected = group(expected, &format!("computed-matrix-{index}"), &matrix);
                index += 1;
                groups += 1;
            }
        }
    }

    // Options: a valid version sets the sRGB transfer function and the matrix;
    // an invalid one leaves the options unchanged (transfer 99, no matrix).
    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(0)).expect("WebP matrix");
    for version in OPTION_VERSIONS {
        let (code, transfer, has_matrix) =
            match sharp_yuv_options_init_internal(matrix, version as i32) {
                Some(options) => (1, options.transfer_type.0, 1),
                None => (0, 99, 0),
            };
        expected = expected
            .with("group", format!("options-version-{version}"))
            .with("code", code)
            .with("transfer", transfer)
            .with("matrix", has_matrix);
        groups += 1;
    }

    // Seeded color spaces beyond the inventory.
    let mut rng = Rng(0x0063_6f6c_6f72);
    for index in 0..256 {
        let kr = (rng.next_u64() >> 40) as f32 / 16_777_216.0_f32;
        let kb = (rng.next_u64() >> 40) as f32 / 16_777_216.0_f32;
        let bit_depth = 8 + rng.below(9) as i32;
        let range = rng.below(2) as u32;
        let color_space = SharpYuvColorSpace {
            kr,
            kb,
            bit_depth,
            range: SharpYuvRange(range),
        };
        let matrix = sharp_yuv_compute_conversion_matrix(&color_space);
        expected = group(expected, &format!("computed-random-{index}"), &matrix);
    }

    assert_eq!(groups, 31, "the inventory has 31 of these groups");
    assert_same_observations(&actual, &expected, "version, matrices and options");
}

#[test]
fn predefined_matrices_have_stable_addresses_and_invalid_types_have_none() {
    for index in 0..5 {
        let first = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(index)).unwrap();
        let second = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(index)).unwrap();
        assert!(ptr::eq(first, second), "matrix {index} moved between calls");
    }
    for index in [5_u32, 6, 0xffff_ffff] {
        assert!(sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(index)).is_none());
    }
}

#[test]
fn options_accept_only_this_major_and_minor_version() {
    let matrix = SharpYuvConversionMatrix::default();
    let accepted: Vec<bool> = [0x40002, 0x40000, 0x400ff, 0x30000, 0x0100_0000, 0, -1]
        .map(|version: i32| sharp_yuv_options_init_internal(&matrix, version).is_some())
        .to_vec();
    assert_eq!(accepted, [true, true, true, false, false, false, false]);
    let options = sharp_yuv_options_init(&matrix).expect("the library's own version");
    assert_eq!(
        options.transfer_type,
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb
    );
}
