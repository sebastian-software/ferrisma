//! WP4 differential test: the rescaler (`src/dsp/rescaler.c` and
//! `src/utils/rescaler_utils.c`). One script of commands drives both the C
//! probe `tools/oracle/probes/wp4/rescaler.c` and the Rust translation. The
//! rescalers run over identities, shrinks and expansions in each direction,
//! extreme downscales and upscales, the single-column special case, padded
//! strides, flipped destinations (negative strides) and several channel counts, and every step of the driver is compared
//! with the state it leaves. The scaled-dimension query runs over zeros,
//! negatives and large values.
//!
//! The rescaler owns its side rows and takes the output buffer on each export,
//! so the driver keeps the output in its own vector and passes it per call. The
//! side rows start with `WORK_FILL`, which the rescaler zeroes as the C `memset`
//! does; the C probe starts from zeroed memory, so the comparison checks the
//! zeroing.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::utils::rescaler_utils::{
    WebPRescaler, webp_rescale_needed_lines, webp_rescaler_export,
    webp_rescaler_get_scaled_dimensions, webp_rescaler_import, webp_rescaler_init,
    webp_rescaler_input_done,
};

/// The sentinel that the probe fills destinations with.
const SENTINEL: u8 = 0xaa;

/// The value that the side rows of the work vector start with. The rescaler
/// zeroes them, so the output must not depend on it.
const WORK_FILL: u32 = 0x5a5a_5a5a;

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

fn hex_arg(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

fn int(value: usize) -> i32 {
    i32::try_from(value).expect("the value fits in int")
}

/// Drives a rescaler the way the probe does and appends each step to `out`.
/// A flipped destination starts at its last row with a negative stride, and its
/// first row is set as the owner sets it.
fn drive(
    out: &mut String,
    (src_w, src_h, dst_w, dst_h, channels): (usize, usize, usize, usize, usize),
    (dst_stride, src_stride, chunk): (usize, usize, usize),
    src: &[u8],
    flip: bool,
) {
    let mut dst = vec![SENTINEL; dst_stride * dst_h];
    let work = vec![WORK_FILL; 2 * dst_w * channels];
    let stride = if flip {
        -int(dst_stride)
    } else {
        int(dst_stride)
    };
    let Some(mut wrk): Option<WebPRescaler> = webp_rescaler_init(
        int(src_w),
        int(src_h),
        int(dst_w),
        int(dst_h),
        stride,
        int(channels),
        work,
    ) else {
        panic!("the rescaler initializes");
    };
    if flip {
        wrk.dst_pos = (dst_h - 1) * dst_stride;
    }
    out.push_str("init=1\n");
    let mut src_row = 0_usize;
    let mut steps = 0_u32;
    while !webp_rescaler_input_done(&wrk) {
        let needed = webp_rescale_needed_lines(&wrk, int(chunk));
        let mut num = chunk;
        if src_h - src_row < num {
            num = src_h - src_row;
        }
        let imported = webp_rescaler_import(
            &mut wrk,
            int(num),
            &src[src_row * src_stride..],
            int(src_stride),
        );
        let exported = webp_rescaler_export(&mut wrk, &mut dst);
        out.push_str(&format!(
            "step needed={needed} imported={imported} exported={exported} src_y={} dst_y={} y_accum={}\n",
            wrk.src_y, wrk.dst_y, wrk.y_accum
        ));
        src_row += usize::try_from(imported).expect("imported rows are not negative");
        steps += 1;
        if steps > 100_000 || (imported == 0 && exported == 0) {
            break;
        }
    }
    out.push_str(&format!("dst={}\n", hex(&dst)));
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0008);
    let mut commands = String::new();
    let mut expected = String::new();

    // Scaled dimensions over zeros, negatives and large values.
    let sources = [1_i32, 7, 100, 4096];
    let scaled = [0_i32, 1, 7, 100, 3000, -1, 2_000_000_000];
    for &src_w in &sources {
        for &src_h in &sources {
            for &sw in &scaled {
                for &sh in &scaled {
                    commands.push_str(&format!("dims {src_w} {src_h} {sw} {sh}\n"));
                    let (mut width, mut height) = (sw, sh);
                    let ret =
                        webp_rescaler_get_scaled_dimensions(src_w, src_h, &mut width, &mut height);
                    expected.push_str(&format!("ret={} sw={width} sh={height}\n", u8::from(ret)));
                }
            }
        }
    }

    // Rescalers: (src_w, src_h, dst_w, dst_h, channels).
    let cases = [
        (4, 4, 4, 4, 1),
        (5, 3, 5, 3, 3),
        (8, 8, 4, 4, 1),
        (7, 5, 3, 2, 1),
        (9, 4, 3, 1, 4),
        (64, 40, 1, 3, 1),
        (33, 20, 5, 7, 2),
        (300, 20, 1, 2, 1),
        (2, 2, 5, 7, 1),
        (3, 4, 9, 4, 3),
        (1, 1, 5, 5, 1),
        (1, 9, 64, 9, 1),
        (1, 6, 2, 6, 1),
        (1, 6, 1, 6, 4),
        (6, 9, 3, 20, 2),
        (3, 4, 8, 2, 1),
        (17, 13, 11, 19, 3),
    ];
    for &(src_w, src_h, dst_w, dst_h, channels) in &cases {
        for (chunk, flip) in [
            (1_usize, false),
            (3, false),
            (1000, false),
            (1, true),
            (1000, true),
        ] {
            let dst_stride = dst_w * channels + 2;
            let src_stride = src_w * channels + 3;
            let src = rng.bytes(src_stride * src_h);
            let command = if flip { "rescaleflip" } else { "rescale" };
            commands.push_str(&format!(
                "{command} {src_w} {src_h} {dst_w} {dst_h} {channels} {dst_stride} {src_stride} {chunk} {}\n",
                hex_arg(&src)
            ));
            drive(
                &mut expected,
                (src_w, src_h, dst_w, dst_h, channels),
                (dst_stride, src_stride, chunk),
                &src,
                flip,
            );
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
fn rescaler_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("rescaler_matches_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 1000,
        "the script has {observations} observations"
    );
    let probe = oracle.probe("wp4", "rescaler");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the rescaler differs from the C functions: {}",
        first_difference(&expected, &actual)
    );
}

#[test]
#[should_panic(expected = "the two side rows need 8")]
fn init_panics_on_a_work_vector_of_the_wrong_length() {
    // One channel and four output pixels need two side rows of 4 elements each.
    let _rescaler = webp_rescaler_init(4, 4, 4, 4, 4, 1, vec![0; 7]);
}
