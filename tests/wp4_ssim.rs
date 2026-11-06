//! WP4 differential test: the SSIM and squared-error functions of
//! `src/dsp/ssim.c` and the statistics of `src/dsp/dsp.h`. The C probe
//! `tools/oracle/probes/wp4/ssim.c` prints each result as its 64-bit pattern,
//! so the comparison is exact. The statistics are those of real windows, which
//! keep the SSIM ratio in `[0, 1]` as the C assertion requires: the test
//! computes them from seeded planes with the window weights of `kWeight`, and
//! both sides then evaluate the same statistics.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::dsp::{VP8_SSIM_KERNEL, VP8DistoStats};
use webp::dsp::ssim::{
    vp8_accumulate_sse, vp8ssim_from_stats, vp8ssim_from_stats_clipped, vp8ssim_get,
    vp8ssim_get_clipped,
};

/// The window weights of `kWeight`, as the test accumulates the statistics.
const WEIGHT: [u32; 7] = [1, 2, 3, 4, 3, 2, 1];

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

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
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

/// The statistics of the window centred on `(xo, yo)`, clipped to a `w` by `h`
/// plane, with the same weights as `SSIMGetClipped_C`.
fn window_stats(
    src1: &[u8],
    stride1: usize,
    src2: &[u8],
    stride2: usize,
    (xo, yo, w, h): (i32, i32, i32, i32),
) -> VP8DistoStats {
    let k = VP8_SSIM_KERNEL;
    let mut stats = VP8DistoStats::default();
    let ymin = (yo - k).max(0);
    let ymax = (yo + k).min(h - 1);
    let xmin = (xo - k).max(0);
    let xmax = (xo + k).min(w - 1);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let weight = WEIGHT[(k + x - xo) as usize] * WEIGHT[(k + y - yo) as usize];
            let row1 = y as usize * stride1;
            let row2 = y as usize * stride2;
            let s1 = u32::from(src1[row1 + x as usize]);
            let s2 = u32::from(src2[row2 + x as usize]);
            stats.w += weight;
            stats.xm += weight * s1;
            stats.ym += weight * s2;
            stats.xxm += weight * s1 * s1;
            stats.xym += weight * s1 * s2;
            stats.yym += weight * s2 * s2;
        }
    }
    stats
}

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
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
fn ssim_and_squared_error_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("ssim_and_squared_error_match_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0x5eed_000b);
    let mut commands = String::from("init\n");
    let mut expected = String::new();

    // The statistics of real windows, through both SSIM forms.
    for _ in 0..300 {
        let w = 1 + rng.below(20) as i32;
        let h = 1 + rng.below(20) as i32;
        let stride = w as usize + rng.below(4) as usize;
        let plane1 = rng.bytes(stride * h as usize);
        let plane2 = rng.bytes(stride * h as usize);
        let xo = rng.below(w as u64) as i32;
        let yo = rng.below(h as u64) as i32;
        let stats = window_stats(&plane1, stride, &plane2, stride, (xo, yo, w, h));
        commands.push_str(&format!(
            "from_stats {} {} {} {} {} {}\n",
            stats.w, stats.xm, stats.ym, stats.xxm, stats.xym, stats.yym
        ));
        expected.push_str(&format!(
            "full={}\nclipped={}\n",
            bits(vp8ssim_from_stats(&stats)),
            bits(vp8ssim_from_stats_clipped(&stats))
        ));
    }

    // The clipped SSIM over planes, with windows at the edges.
    for _ in 0..300 {
        let w = 1 + rng.below(20) as i32;
        let h = 1 + rng.below(20) as i32;
        let stride1 = w as usize + rng.below(4) as usize;
        let stride2 = w as usize + rng.below(4) as usize;
        let plane1 = rng.bytes(stride1 * h as usize);
        let plane2 = rng.bytes(stride2 * h as usize);
        let xo = rng.below(w as u64) as i32;
        let yo = rng.below(h as u64) as i32;
        commands.push_str(&format!(
            "get_clipped {w} {h} {xo} {yo} {stride1} {stride2} {} {}\n",
            hex_arg(&plane1),
            hex_arg(&plane2)
        ));
        let value = vp8ssim_get_clipped(
            &plane1,
            i32::try_from(stride1).expect("int"),
            &plane2,
            i32::try_from(stride2).expect("int"),
            xo,
            yo,
            w,
            h,
        );
        expected.push_str(&format!("value={}\n", bits(value)));
    }

    // The full kernel over 7-row planes.
    for _ in 0..200 {
        let stride1 = 7 + rng.below(4) as usize;
        let stride2 = 7 + rng.below(4) as usize;
        let plane1 = rng.bytes(stride1 * 7);
        let plane2 = rng.bytes(stride2 * 7);
        commands.push_str(&format!(
            "get {stride1} {stride2} {} {}\n",
            hex_arg(&plane1),
            hex_arg(&plane2)
        ));
        let value = vp8ssim_get(
            &plane1,
            i32::try_from(stride1).expect("int"),
            &plane2,
            i32::try_from(stride2).expect("int"),
        );
        expected.push_str(&format!("value={}\n", bits(value)));
    }

    // The sum of squared differences over lengths from 0 to 300.
    for len in (0..=300).step_by(3) {
        let src1 = rng.bytes(len);
        let src2 = rng.bytes(len);
        commands.push_str(&format!(
            "accumulate {len} {} {}\n",
            hex_arg(&src1),
            hex_arg(&src2)
        ));
        let sse = vp8_accumulate_sse(&src1, &src2, i32::try_from(len).expect("int"));
        expected.push_str(&format!("sse={sse}\n"));
    }

    let probe = oracle.probe("wp4", "ssim");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let observations = expected.lines().count();
    assert!(
        observations > 1000,
        "the script has {observations} observations"
    );
    assert!(
        actual == expected,
        "the SSIM functions differ from the C functions: {}",
        first_difference(&expected, &actual)
    );
}
