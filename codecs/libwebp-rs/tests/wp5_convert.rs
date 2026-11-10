//! WP5 differential test: `SharpYuvConvert` and `SharpYuvConvertWithOptions`
//! (`sharpyuv/sharpyuv.c`), compared with the C probe
//! `tools/oracle/probes/wp5/convert.c`.
//!
//! The probe runs the inventory of libavif-rs `docs/SHARPYUV-VALIDATION.md`, which
//! `tools/check_sharpyuv.py` executes: every RGB depth, YUV depth, transfer function
//! and predefined matrix over nine shapes and three layouts (960 groups), 256
//! seeded conversions, 15 rejected calls, and 24 cold conversions on eight threads.
//! Its random inputs come from SplitMix64 (`tools/oracle/probes/wp5/support`), not
//! from Python's generator, so the inputs differ from the libavif run while the
//! groups, shapes, layouts and parameters are the same. The six NULL-pointer
//! rejections and the two NULL-argument checks of the options are not compared:
//! the Rust API takes references, so NULL exists only at the C ABI.
//!
//! Every group compares the return value and each whole output plane, guard bytes
//! included. The output starts as 0xA5 bytes, as the probe's does. The last 64
//! groups run with negative row strides and pixel steps (ADR 0003), which the
//! inventory does not contain.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **NULL arguments.** The C inventory passes NULL to
//!    `SharpYuvOptionsInitInternal` and to `SharpYuvConvertWithOptions` for
//!    each of the six pointer arguments. The Rust API takes references, so NULL
//!    is not representable (PORTING rule 8). No SharpYUV C ABI is planned
//!    (ADR 0004), so these cases are not applicable to the safe API. A future
//!    SharpYUV ABI must run them. The probes still run them on the C side and
//!    require the result 0 (`csp.c` and `convert.c` exit nonzero otherwise);
//!    they are not printed.
//! 2. **`SharpYuvInit`, the CPU-information pointer and the repeated callback
//!    initialization.** The inventory's three calls with a callback and its
//!    `SharpYuvInit(NULL)` call depend on `SharpYuvInit`, which stays absent
//!    (item 1 of `# Representation` in `src/sharpyuv/sharpyuv.rs`).
//!    `SharpYuvGetCPUInfo` is translated in `src/sharpyuv/sharpyuv_cpu.rs`.
//! 3. **Allocation failure.** The inventory injects failures into the seven
//!    conversion allocations. The library has no test-only allocator
//!    (ADR 0003), so the `try_reserve_exact` failure path, which returns
//!    `false`, is not tested.
//! 4. **Input immutability.** The input planes are `&[u8]`, so the library
//!    cannot write them. The probe checks that its input buffer is unchanged
//!    and exits nonzero otherwise; the guard bytes of every output plane are
//!    checked the same way.
//! 5. **Thread sanitizer.** Not run. The concurrent cold conversions pass in
//!    the normal build and under the asan-ubsan oracle.

#[path = "support/oracle.rs"]
mod oracle;

use std::thread;

use oracle::{Fields, Oracle};
use webp::sharpyuv::sharpyuv::{
    SharpYuvTransferFunctionType, sharp_yuv_convert, sharp_yuv_convert_with_options,
    sharp_yuv_options_init,
};
use webp::sharpyuv::sharpyuv_csp::{SharpYuvMatrixType, sharp_yuv_get_conversion_matrix};

/// Bytes of 0xA5 before the first sample of every buffer, as in the probe.
const PAD: usize = 32;
const GUARD: u8 = 0xa5;
const RGB_DEPTHS: [i32; 4] = [8, 10, 12, 16];
const YUV_DEPTHS: [i32; 3] = [8, 10, 12];
/// The transfer functions 1 and 4 to 18; the values 0, 2 and 3 are undefined.
const TRANSFERS: [u32; 16] = [1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
const SRGB: u32 = 13;
const SHAPES: [(usize, usize); 9] = [
    (1, 1),
    (1, 9),
    (9, 1),
    (2, 2),
    (3, 5),
    (15, 7),
    (16, 8),
    (17, 9),
    (33, 11),
];
/// The fifteen calls that the C function rejects with one argument changed. The
/// index is the argument position of `SharpYuvConvertWithOptions`: 3 `rgb_step`,
/// 4 `rgb_stride`, 5 `rgb_bit_depth`, 7 `y_stride`, 9 `u_stride`, 11 `v_stride`,
/// 12 `yuv_bit_depth`, 13 `width`, 14 `height`.
const REJECTS: [(usize, i64); 15] = [
    (13, 0),
    (13, -1),
    (13, 0x7fff_ffff),
    (14, 0),
    (14, -1),
    (14, 0x7fff_ffff),
    (5, 9),
    (5, 15),
    (12, 9),
    (12, 11),
    (3, 3),
    (4, 3),
    (7, 3),
    (9, 3),
    (11, 3),
];

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

/// One conversion of the inventory.
#[derive(Clone, Copy, Default)]
struct Case {
    rgb: i32,
    yuv: i32,
    transfer: u32,
    matrix: u32,
    layout: usize,
    width: usize,
    height: usize,
    seed: u64,
    /// An argument replaced before the call, as `(position, value)`.
    reject: Option<(usize, i64)>,
    /// The input rows, the input pixels and the output rows run backwards.
    flip_in_rows: bool,
    flip_in_cols: bool,
    flip_out_rows: bool,
}

/// The result of one call: the return value and the three whole planes.
#[derive(Clone, PartialEq, Debug)]
struct Outcome {
    code: bool,
    planes: [Vec<u8>; 3],
}

/// Writes one input sample: a byte for 8-bit input, two native-endian bytes otherwise.
fn write_sample(buffer: &mut [u8], at: usize, value: u64, unit: usize) {
    if unit == 1 {
        buffer[at] = value as u8;
    } else {
        buffer[at..at + 2].copy_from_slice(&(value as u16).to_ne_bytes());
    }
}

/// Runs one call the way the C probe's `RunCase` does, with `SharpYuvConvert` when
/// `plain` is set and `SharpYuvConvertWithOptions` otherwise.
fn run_case(case: &Case, plain: bool) -> Outcome {
    let unit = if case.rgb == 8 { 1 } else { 2 };
    let step = unit
        * match case.layout {
            2 => 1,
            1 => 4,
            _ => 3,
        };
    let stride = case.width * step + 6 * unit;
    let span = stride * case.height;
    let raw_bytes = 2 * (3 * span + 64).div_ceil(2);
    let offsets = if case.layout == 2 {
        [0, span, 2 * span]
    } else {
        [0, unit, 2 * unit]
    };
    let max_value = (1_u64 << case.rgb) - 1;

    let mut raw = vec![GUARD; raw_bytes];
    let mut rng = Rng(case.seed);
    for y in 0..case.height {
        for x in 0..case.width {
            for (channel, &offset) in offsets.iter().enumerate() {
                let mut value = rng.below(1_u64 << case.rgb);
                if case.seed.is_multiple_of(4) {
                    value = if (x + y + channel) % 2 == 1 {
                        max_value
                    } else {
                        0
                    };
                }
                let phys_y = if case.flip_in_rows {
                    case.height - 1 - y
                } else {
                    y
                };
                let phys_x = if case.flip_in_cols {
                    case.width - 1 - x
                } else {
                    x
                };
                let at = PAD + offset + phys_y * stride + phys_x * step;
                write_sample(&mut raw, at, value, unit);
            }
        }
    }

    let out_unit = if case.yuv == 8 { 1 } else { 2 };
    let chroma = (case.width.div_ceil(2), case.height.div_ceil(2));
    let dims = [(case.width, case.height), chroma, chroma];
    let mut planes: Vec<Vec<u8>> = Vec::new();
    let mut strides = [0_i32; 3];
    let mut origins = [PAD; 3];
    for (index, (width, height)) in dims.iter().enumerate() {
        let stride = (width + 5) * out_unit;
        strides[index] = stride as i32;
        if case.flip_out_rows {
            // The origin is the last row of the plane, and the stride runs backwards.
            origins[index] = PAD + (height - 1) * stride;
            strides[index] = -strides[index];
        }
        planes.push(vec![GUARD; 2 * ((stride * height + 65) / 2)]);
    }
    let [mut y_plane, mut u_plane, mut v_plane]: [Vec<u8>; 3] =
        planes.try_into().expect("three planes");

    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(case.matrix))
        .expect("a predefined matrix");
    let mut options = sharp_yuv_options_init(matrix).expect("the library's version");
    options.transfer_type = SharpYuvTransferFunctionType(case.transfer);

    let mut rgb_step = step as i32;
    let mut rgb_stride = stride as i32;
    let mut rgb = case.rgb;
    let mut yuv = case.yuv;
    let mut width = case.width as i32;
    let mut height = case.height as i32;
    let (mut y_stride, mut u_stride, mut v_stride) = (strides[0], strides[1], strides[2]);
    if let Some((position, value)) = case.reject {
        match position {
            3 => rgb_step = value as i32,
            4 => rgb_stride = value as i32,
            5 => rgb = value as i32,
            7 => y_stride = value as i32,
            9 => u_stride = value as i32,
            11 => v_stride = value as i32,
            12 => yuv = value as i32,
            13 => width = value as i32,
            14 => height = value as i32,
            _ => unreachable!("NULL-pointer rejections are not compared"),
        }
    }
    // The origin is the last row or pixel of a flipped input.
    let row_adjust = if case.flip_in_rows {
        (case.height - 1) * stride
    } else {
        0
    };
    let col_adjust = if case.flip_in_cols {
        (case.width - 1) * step
    } else {
        0
    };
    let r_origin = PAD + offsets[0] + row_adjust + col_adjust;
    let g_origin = PAD + offsets[1] + row_adjust + col_adjust;
    let b_origin = PAD + offsets[2] + row_adjust + col_adjust;
    if case.flip_in_cols {
        rgb_step = -rgb_step;
    }
    if case.flip_in_rows {
        rgb_stride = -rgb_stride;
    }

    let code = if plain {
        sharp_yuv_convert(
            &raw,
            r_origin,
            &raw,
            g_origin,
            &raw,
            b_origin,
            rgb_step,
            rgb_stride,
            rgb,
            &mut y_plane,
            origins[0],
            y_stride,
            &mut u_plane,
            origins[1],
            u_stride,
            &mut v_plane,
            origins[2],
            v_stride,
            yuv,
            width,
            height,
            matrix,
        )
    } else {
        sharp_yuv_convert_with_options(
            &raw,
            r_origin,
            &raw,
            g_origin,
            &raw,
            b_origin,
            rgb_step,
            rgb_stride,
            rgb,
            &mut y_plane,
            origins[0],
            y_stride,
            &mut u_plane,
            origins[1],
            u_stride,
            &mut v_plane,
            origins[2],
            v_stride,
            yuv,
            width,
            height,
            &options,
        )
    };
    Outcome {
        code,
        planes: [y_plane, u_plane, v_plane],
    }
}

/// Runs the case through `SharpYuvConvertWithOptions`, and for the sRGB transfer
/// without a rejected argument also through `SharpYuvConvert`, which must agree.
fn run_checked(case: &Case) -> Outcome {
    let outcome = run_case(case, false);
    if case.transfer == SRGB && case.reject.is_none() {
        let plain = run_case(case, true);
        assert_eq!(
            plain, outcome,
            "SharpYuvConvert differs from SharpYuvConvertWithOptions"
        );
    }
    outcome
}

/// The probe's group line set: the code, then the three planes in hex.
fn group(fields: Fields, name: &str, outcome: &Outcome) -> Fields {
    fields
        .with("group", name)
        .with("code", i32::from(outcome.code))
        .hex("y", &outcome.planes[0])
        .hex("u", &outcome.planes[1])
        .hex("v", &outcome.planes[2])
}

/// The concurrent group: 24 conversions on eight threads, all equal.
fn concurrent_cold_initialization() -> Outcome {
    let case = Case {
        rgb: 8,
        yuv: 8,
        transfer: SRGB,
        matrix: 0,
        layout: 0,
        width: 17,
        height: 9,
        seed: 42,
        reject: None,
        ..Case::default()
    };
    let outcomes: Vec<Vec<Outcome>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| (0..3).map(|_| run_checked(&case)).collect::<Vec<_>>()))
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("conversion thread"))
            .collect()
    });
    let first = outcomes[0][0].clone();
    for outcome in outcomes.iter().flatten() {
        assert_eq!(*outcome, first, "a concurrent conversion differs");
    }
    first
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
fn convert_matches_the_oracle_inventory() {
    let Some(oracle) = Oracle::locate("convert_matches_the_oracle_inventory") else {
        return;
    };
    let no_args: [&str; 0] = [];
    let output = oracle.probe("wp5", "convert").run(&no_args, b"");
    let actual = output.success().fields();

    let mut expected = Fields::new();
    let mut groups = 0;
    let cold = concurrent_cold_initialization();
    expected = group(expected, "concurrent-cold-initialization-24", &cold);
    groups += 1;

    let mut index = 0_usize;
    for &rgb in &RGB_DEPTHS {
        for &yuv in &YUV_DEPTHS {
            for &transfer in &TRANSFERS {
                for matrix in 0..5 {
                    let (width, height) = SHAPES[index % SHAPES.len()];
                    let layout = index % 3;
                    let case = Case {
                        rgb,
                        yuv,
                        transfer,
                        matrix,
                        layout,
                        width,
                        height,
                        seed: index as u64,
                        reject: None,
                        ..Case::default()
                    };
                    let name = format!(
                        "convert-rgb{rgb}-yuv{yuv}-tf{transfer}-matrix{matrix}-layout{layout}-{width}x{height}"
                    );
                    expected = group(expected, &name, &run_checked(&case));
                    groups += 1;
                    index += 1;
                }
            }
        }
    }
    assert_eq!(index, 960);

    let mut rng = Rng(0x4156_4946);
    for index in 0..256 {
        let rgb = RGB_DEPTHS[rng.below(4) as usize];
        let yuv = YUV_DEPTHS[rng.below(3) as usize];
        let transfer = TRANSFERS[rng.below(16) as usize];
        let matrix = rng.below(5) as u32;
        let layout = rng.below(3) as usize;
        let width = 1 + rng.below(64) as usize;
        let height = 1 + rng.below(32) as usize;
        let seed = rng.below(1 << 30);
        let case = Case {
            rgb,
            yuv,
            transfer,
            matrix,
            layout,
            width,
            height,
            seed,
            reject: None,
            ..Case::default()
        };
        expected = group(expected, &format!("seeded-{index}"), &run_checked(&case));
        groups += 1;
    }

    for (index, reject) in REJECTS.iter().enumerate() {
        let case = Case {
            rgb: 10,
            yuv: 10,
            transfer: SRGB,
            matrix: 0,
            layout: 0,
            width: 3,
            height: 3,
            seed: 111,
            reject: Some(*reject),
            ..Case::default()
        };
        let outcome = run_case(&case, false);
        assert!(!outcome.code, "reject-{index} was accepted");
        expected = group(expected, &format!("reject-{index}"), &outcome);
        groups += 1;
    }

    let mut rng = Rng(0x666c_6970);
    for index in 0..64 {
        let rgb = RGB_DEPTHS[rng.below(4) as usize];
        let yuv = YUV_DEPTHS[rng.below(3) as usize];
        let transfer = TRANSFERS[rng.below(16) as usize];
        let matrix = rng.below(5) as u32;
        let layout = rng.below(3) as usize;
        let width = 1 + rng.below(24) as usize;
        let height = 1 + rng.below(16) as usize;
        let seed = rng.below(1 << 30);
        let flip_in_rows = rng.below(2) == 1;
        let flip_in_cols = rng.below(2) == 1;
        let flip_out_rows = rng.below(2) == 1;
        let case = Case {
            rgb,
            yuv,
            transfer,
            matrix,
            layout,
            width,
            height,
            seed,
            flip_in_rows,
            flip_in_cols,
            flip_out_rows,
            ..Case::default()
        };
        expected = group(expected, &format!("flipped-{index}"), &run_checked(&case));
        groups += 1;
    }

    assert_eq!(
        groups, 1296,
        "the reproduced inventory has 1,296 of these groups"
    );
    assert_same_observations(&actual, &expected, "conversions");
}
