//! WP4 differential test: the fancy upsamplers `WebPUpsamplers` and the
//! converter returned by `WebPGetLinePairConverter` (`src/dsp/upsampling.c`),
//! and the YUV 4:4:4 converters `WebPYUV444Converters`. One script of commands
//! drives both the C probe `tools/oracle/probes/wp4/upsampling.c` and the Rust
//! translation. Row pairs run over lengths from 1 to 65, with and without the
//! bottom row, so the odd and even last pixels and the diagonal interpolation
//! are all exercised, for every output colorspace.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::upsampling::{
    webp_get_line_pair_converter, webp_upsamplers, webp_yuv444_converters,
};
use webp::webp::decode::WEBP_CSP_MODE;

/// The colorspaces with an upsampler and a YUV 4:4:4 converter, MODE_RGB
/// through MODE_rgbA_4444.
const MODES: u32 = 11;
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

/// The probe's `key=value` line for a byte buffer.
fn field(key: &str, bytes: &[u8]) -> String {
    format!("{key}={}\n", hex(bytes))
}

/// The inputs of one row pair, shared by the upsamplers and the getter.
struct Pair {
    top_y: Vec<u8>,
    bottom_y: Vec<u8>,
    top_u: Vec<u8>,
    top_v: Vec<u8>,
    cur_u: Vec<u8>,
    cur_v: Vec<u8>,
}

impl Pair {
    fn random(rng: &mut Rng, len: usize) -> Self {
        let uv = len.div_ceil(2);
        Pair {
            top_y: rng.bytes(len),
            bottom_y: rng.bytes(len),
            top_u: rng.bytes(uv),
            top_v: rng.bytes(uv),
            cur_u: rng.bytes(uv),
            cur_v: rng.bytes(uv),
        }
    }

    /// The command arguments after the mode, length and bottom flag.
    fn arguments(&self) -> String {
        [
            hex_arg(&self.top_y),
            hex_arg(&self.bottom_y),
            hex_arg(&self.top_u),
            hex_arg(&self.top_v),
            hex_arg(&self.cur_u),
            hex_arg(&self.cur_v),
        ]
        .join(" ")
    }
}

/// Runs an upsampler on the pair and returns the probe's output lines.
fn upsample_output(
    upsampler: impl FnOnce(
        &[u8],
        Option<(&[u8], &mut [u8])>,
        &[u8],
        &[u8],
        &[u8],
        &[u8],
        &mut [u8],
        i32,
    ),
    pair: &Pair,
    len: usize,
    bottom: bool,
) -> String {
    let mut top_dst = vec![SENTINEL; 4 * len + 8];
    let mut bottom_dst = vec![SENTINEL; 4 * len + 8];
    let len_int = i32::try_from(len).expect("the length fits in int");
    let bottom_pair = if bottom {
        Some((pair.bottom_y.as_slice(), bottom_dst.as_mut_slice()))
    } else {
        None
    };
    upsampler(
        &pair.top_y,
        bottom_pair,
        &pair.top_u,
        &pair.top_v,
        &pair.cur_u,
        &pair.cur_v,
        &mut top_dst,
        len_int,
    );
    let mut expected = field("top", &top_dst);
    if bottom {
        expected += &field("bottom", &bottom_dst);
    }
    expected
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0005);
    let mut commands = String::from("init\n");
    let mut expected = String::new();
    let lengths: Vec<usize> = (1..=40).chain([63, 64, 65]).collect();

    for mode in 0..MODES {
        for &len in &lengths {
            for bottom in [false, true] {
                let pair = Pair::random(&mut rng, len);
                commands.push_str(&format!(
                    "upsample {mode} {len} {} {}\n",
                    u8::from(bottom),
                    pair.arguments()
                ));
                expected.push_str(&upsample_output(
                    |top_y, bottom_pair, top_u, top_v, cur_u, cur_v, top_dst, len| {
                        webp_upsamplers(
                            WEBP_CSP_MODE(mode),
                            top_y,
                            bottom_pair,
                            top_u,
                            top_v,
                            cur_u,
                            cur_v,
                            top_dst,
                            len,
                        );
                    },
                    &pair,
                    len,
                    bottom,
                ));
            }
        }
    }

    // The converter that WebPGetLinePairConverter returns, for both alpha
    // orders; ARGB-like output for false and BGRA-like output for true.
    for alpha_is_last in [0_u8, 1] {
        for &len in &[1_usize, 2, 3, 17, 40] {
            let pair = Pair::random(&mut rng, len);
            commands.push_str(&format!(
                "getter {alpha_is_last} {len} 1 {}\n",
                pair.arguments()
            ));
            expected.push_str(&upsample_output(
                |top_y, bottom_pair, top_u, top_v, cur_u, cur_v, top_dst, len| {
                    webp_get_line_pair_converter(alpha_is_last != 0)(
                        top_y,
                        bottom_pair,
                        top_u,
                        top_v,
                        cur_u,
                        cur_v,
                        top_dst,
                        len,
                    );
                },
                &pair,
                len,
                true,
            ));
        }
    }

    // The YUV 4:4:4 converters, every length from 0 to 40 and the wide rows.
    for mode in 0..MODES {
        for len in (0..=40).chain([63, 64, 65]) {
            let y = rng.bytes(len);
            let u = rng.bytes(len);
            let v = rng.bytes(len);
            commands.push_str(&format!(
                "yuv444 {mode} {len} {} {} {}\n",
                hex_arg(&y),
                hex_arg(&u),
                hex_arg(&v)
            ));
            let mut dst = vec![SENTINEL; 4 * len + 8];
            webp_yuv444_converters(
                WEBP_CSP_MODE(mode),
                &y,
                &u,
                &v,
                &mut dst,
                i32::try_from(len).expect("the length fits in int"),
            );
            expected.push_str(&field("dst", &dst));
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
fn upsamplers_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("upsamplers_match_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 1500,
        "the script has {observations} observations"
    );
    let probe = oracle.probe("wp4", "upsampling");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the upsamplers differ from the C functions: {}",
        first_difference(&expected, &actual)
    );
}
