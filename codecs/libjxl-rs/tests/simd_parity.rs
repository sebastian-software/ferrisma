// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity (ADR 0004): the port's NEON/AVX2 operations and stages
//! against the original built for the same single Highway target
//! (`JXL_CPP_ORACLE_SIMD`, built by `tools/check.py`). Never compared with the
//! scalar oracle. Words are compared bit for bit except that any NaN equals
//! any NaN (decision D8). Words that the original computes from memory it
//! never initialized (vector over-reads past the request's row length) are
//! not comparable and are skipped explicitly, see `defined_*`.
#![cfg(feature = "cpp-oracle-simd")]

use jxl::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use jxl::jxl::image::Plane;
use jxl::jxl::loop_filter::LoopFilter;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_chroma_simd::{
    get_chroma_upsampling_stage_simd, get_ycbcr_stage_simd,
};
use jxl::jxl::render_pipeline::stage_epf_simd::{EpfPass, get_epf_stage_simd};
use jxl::jxl::render_pipeline::stage_gaborish_simd::get_gaborish_stage_simd;
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

fn target() -> Target {
    Target::detect().expect("the host has no NEON/AVX2 target")
}

fn oracle(requests: &[String]) -> Vec<String> {
    let path = std::env::var_os("JXL_CPP_ORACLE_SIMD")
        .expect("set JXL_CPP_ORACLE_SIMD to the NEON/AVX2 oracle (python3 tools/check.py)");
    let mut child = Command::new(path)
        .env("ROSETTA_ADVERTISE_AVX", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start the SIMD oracle");
    let mut input = child.stdin.take().unwrap();
    let mut lines = requests.join("\n");
    lines.push('\n');
    let feed = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().expect("wait for the SIMD oracle");
    let fed = feed.join().unwrap();
    // The exit status first: an oracle that dies early (a signal, an
    // exception) also breaks the request pipe, and the write error alone
    // would hide the cause (issue #252).
    assert!(
        output.status.success(),
        "SIMD oracle failed ({}) after {} response lines; write: {:?}; stderr: {}",
        output.status,
        output.stdout.iter().filter(|&&b| b == b'\n').count(),
        fed.as_ref().err(),
        String::from_utf8_lossy(&output.stderr)
    );
    fed.expect("write the oracle requests");
    let text = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = text.lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), requests.len(), "oracle response count");
    lines
}

fn is_nan(word: u32) -> bool {
    f32::from_bits(word).is_nan()
}

/// Compare whitespace-separated observations word by word. `float` says
/// whether word `i` is a binary32 sample (NaN-equal) and `defined` whether
/// the original computes it from initialized memory.
fn same(
    rust: &str,
    cpp: &str,
    float: impl Fn(usize) -> bool,
    defined: impl Fn(usize) -> bool,
) -> Result<usize, String> {
    let r: Vec<&str> = rust.split_whitespace().collect();
    let c: Vec<&str> = cpp.split_whitespace().collect();
    if r.len() != c.len() {
        return Err(format!("word count {} != {}", r.len(), c.len()));
    }
    let mut compared = 0;
    for (i, (a, b)) in r.iter().zip(&c).enumerate() {
        if !defined(i) {
            continue;
        }
        compared += 1;
        if a == b {
            continue;
        }
        if float(i)
            && let (Ok(x), Ok(y)) = (a.parse::<u32>(), b.parse::<u32>())
            && is_nan(x)
            && is_nan(y)
        {
            continue;
        }
        return Err(format!("word {i}: rust {a} != original {b}"));
    }
    Ok(compared)
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// Adversarial binary32 words: zeros, signs, subnormals, extremes,
/// infinities, quiet and signaling NaNs of both signs, rounding ties, integer
/// conversion limits, and reciprocal-estimate boundaries.
fn special_words() -> Vec<u32> {
    let mut words = vec![
        0x0000_0000,
        0x8000_0000,
        0x3f80_0000,
        0xbf80_0000,
        0x3f00_0000,
        0xbf00_0000,
        0x3fc0_0000,
        0x4020_0000,
        0xc020_0000,
        0x0080_0000,
        0x8080_0000,
        0x0000_0001,
        0x8000_0001,
        0x007f_ffff,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_0000,
        0x7fa1_2345,
        0xffa5_4321,
        0x7fc9_8765,
        0x4f00_0000,
        0xcf00_0000,
        0x4eff_ffff,
        0xcf00_0001,
        0x4b00_0000,
        0x4aff_fffe,
        0xcaff_ffff,
        0x3eff_ffff,
        0x3eaa_aaab,
        0x7e80_0000,
        0x7f00_0000,
        0x0040_0000,
        0x4000_0000,
        0x4040_0000,
        0x447a_0000,
        0xc47a_0000,
    ];
    let mut seed = 0x1234_5678_9abc_def1;
    for _ in 0..40 {
        words.push(next(&mut seed) as u32);
    }
    words
}

/// Highway operations of the layer, by hwy-op request name, with their arity
/// and whether the result is a float (NaN-equal) or integer vector.
const OPS: &[(&str, usize, bool)] = &[
    ("add", 2, true),
    ("sub", 2, true),
    ("mul", 2, true),
    ("div", 2, true),
    ("mul_add", 3, true),
    ("neg_mul_add", 3, true),
    ("mul_sub", 3, true),
    ("neg_mul_sub", 3, true),
    ("min", 2, true),
    ("max", 2, true),
    ("clamp", 3, true),
    ("abs", 1, true),
    ("neg", 1, true),
    ("abs_diff", 2, true),
    ("sqrt", 1, true),
    ("approximate_reciprocal", 1, true),
    ("approximate_reciprocal_sqrt", 1, true),
    ("floor", 1, true),
    ("ceil", 1, true),
    ("round", 1, true),
    ("trunc", 1, true),
    ("zero_if_negative", 1, true),
    ("copy_sign", 2, true),
    ("copy_sign_to_abs", 2, true),
    ("and", 2, false),
    ("or", 2, false),
    ("xor", 2, false),
    ("and_not", 2, false),
    ("lt", 2, false),
    ("le", 2, false),
    ("gt", 2, false),
    ("ge", 2, false),
    ("eq", 2, false),
    ("ne", 2, false),
    ("if_lt_then_else", 4, false),
    ("if_lt_then_zero_else", 3, false),
    ("if_lt_then_else_zero", 3, false),
    ("convert_to_i32", 1, false),
    ("convert_to_f32", 1, true),
    ("nearest_int", 1, false),
    ("add_i32", 2, false),
    ("sub_i32", 2, false),
    ("min_i32", 2, false),
    ("max_i32", 2, false),
    ("lt_i32", 2, false),
    ("eq_i32", 2, false),
    ("gt_i32", 2, false),
    ("shl_i32_1", 1, false),
    ("shl_i32_23", 1, false),
    ("shr_i32_1", 1, false),
    ("shr_i32_23", 1, false),
    ("shr_i32_31", 1, false),
    ("shr_u32_1", 1, false),
    ("shr_u32_23", 1, false),
    ("sum_of_lanes", 1, true),
    ("max_of_lanes", 1, true),
    ("min_of_lanes", 1, true),
    ("reduce_sum", 1, true),
    ("get_lane", 1, true),
    ("interleave_lower", 2, false),
    ("interleave_upper", 2, false),
    ("store_interleaved2", 2, false),
    ("store_interleaved3", 3, false),
];

struct Op<'a> {
    name: &'a str,
    input: &'a [u32],
}
impl Kernel for Op<'_> {
    type Output = Vec<u32>;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Vec<u32> {
        let n = D::LANES;
        let mut words = self.input.to_vec();
        words.resize(4 * n, 0);
        let [a, b, c, e] = [0, 1, 2, 3].map(|i| d.load_bits(&words[i * n..]));
        let [ai, bi] = [a, b].map(|v| d.bitcast_to_i32(v));
        let mut out = vec![0u32; 4 * n];
        let put = |v: D::V, out: &mut Vec<u32>| {
            d.store_bits(v, out);
            n
        };
        let puti = |v: D::I, out: &mut Vec<u32>| {
            d.store_bits(d.bitcast_to_f32(v), out);
            n
        };
        let count = match self.name {
            "add" => put(d.add(a, b), &mut out),
            "sub" => put(d.sub(a, b), &mut out),
            "mul" => put(d.mul(a, b), &mut out),
            "div" => put(d.div(a, b), &mut out),
            "mul_add" => put(d.mul_add(a, b, c), &mut out),
            "neg_mul_add" => put(d.neg_mul_add(a, b, c), &mut out),
            "mul_sub" => put(d.mul_sub(a, b, c), &mut out),
            "neg_mul_sub" => put(d.neg_mul_sub(a, b, c), &mut out),
            "min" => put(d.min(a, b), &mut out),
            "max" => put(d.max(a, b), &mut out),
            "clamp" => put(d.clamp(a, b, c), &mut out),
            "abs" => put(d.abs(a), &mut out),
            "neg" => put(d.neg(a), &mut out),
            "abs_diff" => put(d.abs_diff(a, b), &mut out),
            "sqrt" => put(d.sqrt(a), &mut out),
            "approximate_reciprocal" => put(d.approximate_reciprocal(a), &mut out),
            "approximate_reciprocal_sqrt" => put(d.approximate_reciprocal_sqrt(a), &mut out),
            "floor" => put(d.floor(a), &mut out),
            "ceil" => put(d.ceil(a), &mut out),
            "round" => put(d.round(a), &mut out),
            "trunc" => put(d.trunc(a), &mut out),
            "zero_if_negative" => put(d.zero_if_negative(a), &mut out),
            "copy_sign" => put(d.copy_sign(a, b), &mut out),
            "copy_sign_to_abs" => put(d.copy_sign_to_abs(a, b), &mut out),
            "and" => put(d.and(a, b), &mut out),
            "or" => put(d.or(a, b), &mut out),
            "xor" => put(d.xor(a, b), &mut out),
            "and_not" => put(d.and_not(a, b), &mut out),
            "lt" => put(d.vec_from_mask(d.lt(a, b)), &mut out),
            "le" => put(d.vec_from_mask(d.le(a, b)), &mut out),
            "gt" => put(d.vec_from_mask(d.gt(a, b)), &mut out),
            "ge" => put(d.vec_from_mask(d.ge(a, b)), &mut out),
            "eq" => put(d.vec_from_mask(d.eq(a, b)), &mut out),
            "ne" => put(d.vec_from_mask(d.ne(a, b)), &mut out),
            "if_lt_then_else" => put(d.if_then_else(d.lt(a, b), c, e), &mut out),
            "if_lt_then_zero_else" => put(d.if_then_zero_else(d.lt(a, b), c), &mut out),
            "if_lt_then_else_zero" => put(d.if_then_else_zero(d.lt(a, b), c), &mut out),
            "convert_to_i32" => puti(d.convert_to_i32(a), &mut out),
            "convert_to_f32" => put(d.convert_to_f32(ai), &mut out),
            "nearest_int" => puti(d.nearest_int(a), &mut out),
            "add_i32" => puti(d.add_i32(ai, bi), &mut out),
            "sub_i32" => puti(d.sub_i32(ai, bi), &mut out),
            "min_i32" => puti(d.min_i32(ai, bi), &mut out),
            "max_i32" => puti(d.max_i32(ai, bi), &mut out),
            "lt_i32" => puti(d.lt_i32(ai, bi), &mut out),
            "eq_i32" => puti(d.eq_i32(ai, bi), &mut out),
            "gt_i32" => puti(d.gt_i32(ai, bi), &mut out),
            "shl_i32_1" => puti(d.shift_left_i32::<1>(ai), &mut out),
            "shl_i32_23" => puti(d.shift_left_i32::<23>(ai), &mut out),
            "shr_i32_1" => puti(d.shift_right_i32::<1>(ai), &mut out),
            "shr_i32_23" => puti(d.shift_right_i32::<23>(ai), &mut out),
            "shr_i32_31" => puti(d.shift_right_i32::<31>(ai), &mut out),
            "shr_u32_1" => puti(d.shift_right_u32::<1>(ai), &mut out),
            "shr_u32_23" => puti(d.shift_right_u32::<23>(ai), &mut out),
            "sum_of_lanes" => put(d.sum_of_lanes(a), &mut out),
            "max_of_lanes" => put(d.max_of_lanes(a), &mut out),
            "min_of_lanes" => put(d.min_of_lanes(a), &mut out),
            "reduce_sum" => {
                out[0] = d.reduce_sum(a).to_bits();
                1
            }
            "get_lane" => {
                out[0] = d.get_lane(a).to_bits();
                1
            }
            "interleave_lower" => put(d.interleave_lower(a, b), &mut out),
            "interleave_upper" => put(d.interleave_upper(a, b), &mut out),
            "store_interleaved2" => {
                let mut f = vec![0.0f32; 2 * n];
                d.store_interleaved2(a, b, &mut f);
                out = f.iter().map(|v| v.to_bits()).collect();
                2 * n
            }
            "store_interleaved3" => {
                let mut f = vec![0.0f32; 3 * n];
                d.store_interleaved3(a, b, c, &mut f);
                out = f.iter().map(|v| v.to_bits()).collect();
                3 * n
            }
            other => panic!("unknown operation {other}"),
        };
        out.truncate(count);
        out
    }
}

#[test]
fn simd_oracle_reports_the_same_highway_target_and_lanes() {
    let t = target();
    let observed = oracle(&["hwy-target".to_owned()]);
    assert_eq!(observed[0], format!("{} {}", t.name(), t.lanes()));
}

#[test]
fn highway_operations_match_the_original_target_bit_for_bit() {
    let t = target();
    let n = t.lanes();
    let words = special_words();
    let mut requests = Vec::new();
    let mut inputs = Vec::new();
    let mut seed = 0x0dd_ba11_u64;
    for &(name, arity, _) in OPS {
        // Every special word in every operand position (rotated through the
        // lanes), plus random pairings.
        let mut cases: Vec<Vec<u32>> = Vec::new();
        for (i, _) in words.iter().enumerate() {
            for j in 0..words.len().min(if arity == 1 { 1 } else { words.len() }) {
                if arity > 1 && (i * 7 + j) % 3 != 0 {
                    continue;
                }
                let mut input = Vec::new();
                for k in 0..arity {
                    for lane in 0..n {
                        let w = match k {
                            0 => words[(i + lane) % words.len()],
                            1 => words[(j + 3 * lane) % words.len()],
                            _ => words[(i + j + 5 * lane + k) % words.len()],
                        };
                        input.push(w);
                    }
                }
                cases.push(input);
            }
        }
        for _ in 0..64 {
            cases.push(
                (0..arity * n)
                    .map(|_| words[(next(&mut seed) % words.len() as u64) as usize])
                    .collect(),
            );
        }
        for input in cases {
            let mut request = format!("hwy-op {name} {}", input.len());
            for w in &input {
                write!(request, " {w}").unwrap();
            }
            requests.push(request);
            inputs.push((name, input));
        }
    }
    let observed = oracle(&requests);
    let mut per_op = std::collections::BTreeMap::<&str, usize>::new();
    for ((request, (name, input)), cpp) in requests.iter().zip(&inputs).zip(&observed) {
        let float = OPS.iter().find(|op| op.0 == *name).unwrap().2;
        let words = dispatch(t, Op { name, input });
        let mut rust = n.to_string();
        for w in &words {
            write!(rust, " {w}").unwrap();
        }
        same(&rust, cpp, |i| float && i > 0, |_| true)
            .unwrap_or_else(|e| panic!("{e}\nrequest: {request}\noriginal: {cpp}\nrust: {rust}"));
        *per_op.entry(name).or_default() += 1;
    }
    println!(
        "{}: {} Highway operations, {} requests matched: {per_op:?}",
        t.name(),
        per_op.len(),
        requests.len()
    );
}

fn plane(len: usize, height: usize) -> Plane<f32> {
    // Room for the original's full-vector over-reads and over-writes, which
    // land in the source's row padding (BytesPerRow with MaxVectorSize()).
    Plane::create(len + 32, height).unwrap()
}

fn words(out: &mut String, planes: &[Plane<f32>], rows: usize, len: usize) {
    for p in planes {
        for y in 0..rows {
            for x in 0..len {
                write!(out, " {}", p.row(y)[x].to_bits()).unwrap();
            }
        }
    }
}

fn floats<const N: usize>(args: &mut std::str::SplitWhitespace<'_>) -> [f32; N] {
    std::array::from_fn(|_| f32::from_bits(args.next().unwrap().parse().unwrap()))
}
fn num(args: &mut std::str::SplitWhitespace<'_>) -> usize {
    args.next().unwrap().parse().unwrap()
}

fn status(s: &jxl::jxl::base::status::Status) -> i32 {
    jxl::jxl::base::status::status_code(s)
}

/// gab-row as tools/oracle/gaborish_stage.inc observes it.
fn gab_row(request: &str, t: Target) -> (String, usize) {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("gab-row"));
    let gab = num(&mut args) != 0;
    let w: [f32; 6] = floats(&mut args);
    let lf = LoopFilter {
        gab,
        gab_x_weight1: w[0],
        gab_x_weight2: w[1],
        gab_y_weight1: w[2],
        gab_y_weight2: w[3],
        gab_b_weight1: w[4],
        gab_b_weight2: w[5],
        ..LoopFilter::default()
    };
    let (left, right, width, origin) = (
        num(&mut args),
        num(&mut args),
        num(&mut args),
        num(&mut args),
    );
    let values: [f32; 9] = floats(&mut args);
    let len = origin + OFFSET + width + right + 3;
    let mut input: Vec<_> = (0..5).map(|_| plane(len, 7)).collect();
    let mut output: Vec<_> = (0..5).map(|_| plane(len, 4)).collect();
    for c in 0..5 {
        for y in 0..7 {
            for x in 0..len + 32 {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..4 {
            for x in 0..len {
                output[c].row_mut(y)[x] =
                    f32::from_bits(0x7fa00000 + (c * 256 + y * 64 + x) as u32);
            }
        }
    }
    let ir = (0..5)
        .map(|c| {
            [1, 3, 5]
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y,
                    x: origin,
                })
                .to_vec()
        })
        .collect();
    let or = (0..5)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Output,
                channel: c,
                y: 2,
                x: origin,
            }]
        })
        .collect();
    let mut stage = get_gaborish_stage_simd(&lf, t).unwrap();
    let s = stage.process_row(
        &ir,
        &or,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        width,
        23,
        41,
        17,
    );
    let mut out = status(&s).to_string();
    words(&mut out, &output, 4, len);
    (out, len)
}

/// cs-row as tools/oracle/chroma_stages.cc observes it.
fn cs_row(request: &str, t: Target) -> (String, usize) {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("cs-row"));
    let kind = num(&mut args);
    let channel = num(&mut args);
    let (left, right, width, origin) = (
        num(&mut args),
        num(&mut args),
        num(&mut args),
        num(&mut args),
    );
    let values: [f32; 9] = floats(&mut args);
    let len = origin + OFFSET + 2 * (width + right) + 5;
    let mut input: Vec<_> = (0..5).map(|_| plane(len, 7)).collect();
    let mut output: Vec<_> = (0..5).map(|_| plane(len, 5)).collect();
    for c in 0..5 {
        for y in 0..7 {
            for x in 0..len + 32 {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..5 {
            for x in 0..len {
                output[c].row_mut(y)[x] =
                    f32::from_bits(0x7fa00000 + (c * 8192 + y * 1024 + x) as u32);
            }
        }
    }
    let row = |storage, c, y| Row {
        storage,
        channel: c,
        y,
        x: origin,
    };
    let (ir, or): (Vec<Vec<Row>>, Vec<Vec<Row>>) = (0..5)
        .map(|c| {
            if kind == 2 {
                (
                    [1, 3, 5].map(|y| row(RowStorage::Input, c, y)).to_vec(),
                    [1, 3].map(|y| row(RowStorage::Output, c, y)).to_vec(),
                )
            } else {
                (
                    vec![row(RowStorage::Input, c, 3)],
                    vec![row(RowStorage::Output, c, 2)],
                )
            }
        })
        .unzip();
    let mut stage: Box<dyn RenderPipelineStage> = if kind == 0 {
        get_ycbcr_stage_simd(t)
    } else {
        get_chroma_upsampling_stage_simd(channel, kind == 1, t)
    };
    let s = stage.process_row(
        &ir,
        &or,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        width,
        23,
        41,
        17,
    );
    let mut out = status(&s).to_string();
    words(&mut out, &input, 7, len);
    words(&mut out, &output, 5, len);
    (out, len)
}

/// epf-row as tools/oracle/epf.cc observes it.
fn epf_row(request: &str, t: Target) -> (String, usize, usize) {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("epf-row"));
    let pass = num(&mut args);
    let iters = num(&mut args);
    let p: [f32; 8] = floats(&mut args);
    let lf = LoopFilter {
        epf_iters: iters as u32,
        epf_pass0_sigma_scale: p[0],
        epf_pass2_sigma_scale: p[1],
        epf_border_sad_mul: p[2],
        epf_channel_scale: [p[3], p[4], p[5]],
        epf_pass1_zeroflush: p[6],
        epf_pass2_zeroflush: p[7],
        ..LoopFilter::default()
    };
    let (left, right, width, origin, xpos, ypos) = (
        num(&mut args),
        num(&mut args),
        num(&mut args),
        num(&mut args),
        num(&mut args),
        num(&mut args),
    );
    let values: [f32; 9] = floats(&mut args);
    let sigmas: [f32; 3] = floats(&mut args);
    let mut sigma = Plane::create(128, 64).unwrap();
    for y in 0..64 {
        for x in 0..128 {
            sigma.row_mut(y)[x] = sigmas[(x + y) % 3];
        }
    }
    let len = origin + OFFSET + width + right + 5;
    let mut input: Vec<_> = (0..5).map(|_| plane(len, 9)).collect();
    let mut output: Vec<_> = (0..5).map(|_| plane(len, 3)).collect();
    for c in 0..5 {
        for y in 0..9 {
            for x in 0..len + 32 {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..3 {
            for x in 0..len {
                output[c].row_mut(y)[x] =
                    f32::from_bits(0x7fa00000 + (c * 4096 + y * 256 + x) as u32);
            }
        }
    }
    let border = 3 - pass;
    let ir = (0..5)
        .map(|c| {
            (0..2 * border + 1)
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: y + 1,
                    x: origin,
                })
                .collect()
        })
        .collect();
    let or = (0..5)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Output,
                channel: c,
                y: 1,
                x: origin,
            }]
        })
        .collect();
    let pass_kind = [EpfPass::Zero, EpfPass::One, EpfPass::Two][pass];
    let mut stage = get_epf_stage_simd(&lf, sigma, pass_kind, t);
    let s = stage.process_row(
        &ir,
        &or,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        width,
        xpos,
        ypos,
        17,
    );
    let mut out = status(&s).to_string();
    words(&mut out, &output, 3, len);
    (out, len, border)
}

/// Whether word `i` (after the status word) of a `planes x rows x len`
/// dump is defined: column `x` of an output row may only use input columns
/// up to `x + reach`, which must lie inside the request's `len` samples.
fn defined_columns(i: usize, len: usize, reach: usize) -> bool {
    if i == 0 {
        return true;
    }
    (i - 1) % len + reach < len
}

fn check_rows(
    family: &str,
    requests: &[String],
    observe: impl Fn(&str) -> (String, Box<dyn Fn(usize) -> bool>),
) -> usize {
    let observed = oracle(requests);
    let mut words = 0;
    for (request, cpp) in requests.iter().zip(&observed) {
        let (rust, defined) = observe(request);
        words += same(&rust, cpp, |i| i > 0, defined)
            .unwrap_or_else(|e| panic!("{family}: {e}\nrequest: {request}"));
    }
    println!(
        "{family} on {}: {} requests, {words} words matched",
        target().name(),
        requests.len()
    );
    words
}

fn value_sets() -> Vec<[u32; 9]> {
    let mut sets = vec![
        [0; 9],
        [0x8000_0000; 9],
        [0x3f80_0000; 9],
        [
            0,
            0x8000_0000,
            1,
            0x8000_0001,
            0x0080_0000,
            0x7f7f_ffff,
            0xff7f_ffff,
            0x3f80_0000,
            0xbf80_0000,
        ],
        [
            0x7fa1_2345,
            0xffa5_4321,
            0x7fc9_8765,
            0xffc4_3210,
            0x7f80_0000,
            0xff80_0000,
            0x3f00_0000,
            0x3f80_0000,
            0x8000_0000,
        ],
    ];
    let mut seed = 617;
    for _ in 0..10 {
        sets.push(std::array::from_fn(|_| next(&mut seed) as u32));
    }
    // Finite, image-like samples.
    for _ in 0..10 {
        sets.push(std::array::from_fn(|_| {
            ((next(&mut seed) % 2000) as f32 / 1000.0 - 0.5).to_bits()
        }));
    }
    sets
}

const SPANS: &[(usize, usize, usize, usize)] = &[
    (0, 0, 0, 0),
    (1, 2, 0, 3),
    (0, 0, 1, 0),
    (1, 2, 3, 3),
    (0, 7, 17, 3),
    (20, 7, 17, 0),
    (8, 8, 32, 5),
    (3, 1, 9, 1),
];

/// The horizontal upsampler writes `row_out + 2 * x` from `x_start =
/// -RoundUpTo(xextra_left, Lanes)`; requests whose start lies before the
/// output row (origin + kRenderPipelineXOffset) write outside it in the
/// original, which is outside the stage's contract and not comparable.
fn chroma_in_contract(kind: usize, left: usize, origin: usize, lanes: usize) -> bool {
    kind != 1 || 2 * left.div_ceil(lanes) * lanes <= origin + OFFSET
}

/// EPF indexes `row_sigma[(x + xpos + kSigmaPadding * 8) / 8]` in size_t;
/// a negative sum wraps and reads far outside the sigma row in the original.
/// It also loads `Lanes` entries of an 8-entry array from `(x + xpos) % 8`,
/// defined only when `xpos` is a multiple of the lane count.
fn epf_in_contract(left: usize, xpos: usize, lanes: usize) -> bool {
    xpos + 16 >= left.div_ceil(lanes) * lanes && xpos.is_multiple_of(lanes)
}

fn join(words: impl IntoIterator<Item = u32>) -> String {
    words
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn simd_gaborish_rows_match_the_original_target() {
    let t = target();
    let lf = LoopFilter::default();
    let mut weights = vec![
        [
            lf.gab_x_weight1,
            lf.gab_x_weight2,
            lf.gab_y_weight1,
            lf.gab_y_weight2,
            lf.gab_b_weight1,
            lf.gab_b_weight2,
        ]
        .map(f32::to_bits),
        [0; 6],
        [0.125f32, -0.0625, 0.5, 0.25, -0.1, 0.35].map(f32::to_bits),
        [
            0x7fa1_2345,
            0x8000_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc9_8765,
            1,
        ],
    ];
    let mut seed = 99;
    for _ in 0..6 {
        weights.push(std::array::from_fn(|_| next(&mut seed) as u32));
    }
    let mut requests = Vec::new();
    for w in &weights {
        for values in value_sets() {
            for &(left, right, width, origin) in SPANS {
                requests.push(format!(
                    "gab-row 1 {} {left} {right} {width} {origin} {}",
                    join(*w),
                    join(values)
                ));
            }
        }
    }
    check_rows("gab-row", &requests, |r| {
        let (out, len) = gab_row(r, t);
        (out, Box::new(move |i| defined_columns(i, len, 1)))
    });
}

#[test]
fn simd_chroma_upsampling_and_ycbcr_rows_match_the_original_target() {
    let t = target();
    let mut requests = Vec::new();
    for values in value_sets() {
        for &(left, right, width, origin) in SPANS {
            requests.push(format!("cs-row 0 0 0 0 {width} {origin} {}", join(values)));
            for kind in [1, 2] {
                if !chroma_in_contract(kind, left, origin, t.lanes()) {
                    continue;
                }
                for channel in 0..3 {
                    requests.push(format!(
                        "cs-row {kind} {channel} {left} {right} {width} {origin} {}",
                        join(values)
                    ));
                }
            }
        }
    }
    check_rows("cs-row", &requests, |r| {
        let (out, len) = cs_row(r, t);
        let kind: usize = r.split_whitespace().nth(1).unwrap().parse().unwrap();
        let origin: usize = r.split_whitespace().nth(6).unwrap().parse().unwrap();
        let input_words = 5 * 7 * len;
        (
            out,
            Box::new(move |i| {
                if i == 0 || kind != 1 || i <= input_words {
                    return true;
                }
                // Horizontal: output column a uses input column
                // base + (a - base) / 2 + 1, base = origin + OFFSET.
                let a = (i - 1 - input_words) % len;
                let base = origin + OFFSET;
                a < base || base + (a - base) / 2 + 1 < len
            }),
        )
    });
}

#[test]
fn simd_epf_rows_of_all_passes_match_the_original_target() {
    let t = target();
    let lf = LoopFilter::default();
    let mut params = vec![
        [
            lf.epf_pass0_sigma_scale,
            lf.epf_pass2_sigma_scale,
            lf.epf_border_sad_mul,
            lf.epf_channel_scale[0],
            lf.epf_channel_scale[1],
            lf.epf_channel_scale[2],
            lf.epf_pass1_zeroflush,
            lf.epf_pass2_zeroflush,
        ]
        .map(f32::to_bits),
        [0.9f32, 0.6, 2.0 / 3.0, 40.0, 5.0, 3.5, 0.45, 0.6].map(f32::to_bits),
    ];
    let mut seed = 4242;
    for _ in 0..2 {
        params.push(std::array::from_fn(|_| {
            ((next(&mut seed) % 4000) as f32 / 1000.0).to_bits()
        }));
    }
    let sigma_sets: [[f32; 3]; 4] = [
        [-10.0, -3.0, -0.1],
        [-0.5, -0.25, -1.0],
        [-3.0, -3.9, -4.0],
        [-0.01, -2.0, -100.0],
    ];
    let lanes = t.lanes();
    let mut requests = Vec::new();
    for pass in 0..3 {
        for p in &params {
            for values in value_sets() {
                for sigmas in &sigma_sets {
                    for &(left, right, width, origin) in SPANS {
                        // The source loads 8-entry sad_mul arrays at
                        // ix = (x + xpos) % 8 with full vectors; only xpos
                        // that keep ix + Lanes <= 8 read defined memory.
                        for (xpos, ypos) in [(0, 0), (8, 7), (lanes, 3), (64, 9)] {
                            if !epf_in_contract(left, xpos, lanes) {
                                continue;
                            }
                            requests.push(format!(
                                "epf-row {pass} 1 {} {left} {right} {width} {origin} {xpos} {ypos} {} {}",
                                join(*p),
                                join(values),
                                join(sigmas.map(f32::to_bits))
                            ));
                        }
                    }
                }
            }
        }
    }
    check_rows("epf-row", &requests, |r| {
        let (out, len, border) = epf_row(r, t);
        (out, Box::new(move |i| defined_columns(i, len, border)))
    });
}

/// Optional: replay every gab-row/cs-row/epf-row line of a captured request
/// corpus (`JXL_SIMD_CORPUS`, a directory of `*.in` files, such as the
/// requests the scalar parity tests send), with the same definedness rules.
#[test]
fn simd_row_corpus_replay_matches_the_original_target() {
    let Some(dir) = std::env::var_os("JXL_SIMD_CORPUS") else {
        println!("JXL_SIMD_CORPUS not set: no captured corpus to replay");
        return;
    };
    let t = target();
    let lanes = t.lanes();
    let mut families: [(Vec<String>, &str); 3] =
        [(vec![], "gab-row"), (vec![], "cs-row"), (vec![], "epf-row")];
    let mut seen = std::collections::BTreeSet::new();
    let mut excluded = 0;
    for entry in std::fs::read_dir(dir).unwrap() {
        let text = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        for line in text.lines() {
            for (list, name) in &mut families {
                if line.starts_with(&format!("{name} ")) && seen.insert(line.to_owned()) {
                    let tokens: Vec<usize> = line
                        .split_whitespace()
                        .skip(1)
                        .map(|w| w.parse().unwrap())
                        .collect();
                    let in_contract = match *name {
                        "epf-row" => epf_in_contract(tokens[10], tokens[14], lanes),
                        "cs-row" => chroma_in_contract(tokens[0], tokens[2], tokens[5], lanes),
                        _ => true,
                    };
                    if !in_contract {
                        excluded += 1;
                        continue;
                    }
                    list.push(line.to_owned());
                }
            }
        }
    }
    println!("requests outside the original stage contract (not comparable): {excluded}");
    let [(gab, _), (cs, _), (epf, _)] = families;
    check_rows("corpus gab-row", &gab, |r| {
        let (out, len) = gab_row(r, t);
        (out, Box::new(move |i| defined_columns(i, len, 1)))
    });
    check_rows("corpus cs-row", &cs, |r| {
        let (out, len) = cs_row(r, t);
        let kind: usize = r.split_whitespace().nth(1).unwrap().parse().unwrap();
        let origin: usize = r.split_whitespace().nth(6).unwrap().parse().unwrap();
        let input_words = 5 * 7 * len;
        (
            out,
            Box::new(move |i| {
                if i == 0 || kind != 1 || i <= input_words {
                    return true;
                }
                let a = (i - 1 - input_words) % len;
                let base = origin + OFFSET;
                a < base || base + (a - base) / 2 + 1 < len
            }),
        )
    });
    check_rows("corpus epf-row", &epf, |r| {
        let (out, len, border) = epf_row(r, t);
        (out, Box::new(move |i| defined_columns(i, len, border)))
    });
}
