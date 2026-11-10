//! WP3 differential test: palette analysis of `src/utils/palette.c` and
//! `WebPGetColorPalette` of `src/utils/utils.c`. One script drives the C probe
//! `tools/oracle/probes/wp3/palette.c` and the Rust translation, which builds
//! the expected output: color counting and palette extraction on seeded
//! pictures with up to 256 colors and with more, odd sizes, padded and negative
//! strides; the sorted map and the binary search; and every sorting method
//! (`kSortedDefault`, `kMinimizeDelta`, `kModifiedZeng`) on palettes with and
//! without black, monotonous ramps, palettes larger than the picture's colors
//! and pictures with one color.
//!
//! The sorts see distinct colors only, so the order of the C `qsort` is the
//! ascending order (PORTING rule 8, the `Sorting` paragraph in the
//! documentation of `src/utils/palette.rs`). `kModifiedZeng` needs two colors
//! in the picture, as its C assertion `c1 != c2` does.

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::utils::palette::{
    PaletteSorting, get_color_palette, palette_sort, prepare_map_to_palette, search_color_no_idx,
};
use webp::utils::utils::webp_get_color_palette;
use webp::webp::encode::WebPPicture;
use webp::webp::types::{PlaneOffset, PlaneStorage};

const SENTINEL: u32 = 0xdead_beef;
const ENTRIES: usize = 256;

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

    fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.iter().map(|value| format!("{value:08x}")).collect()
    }
}

/// A seeded picture as the probe reads it: the pixels in their layout, the
/// stride and the position of the first row.
struct Picture {
    width: i32,
    height: i32,
    stride: i32,
    offset: usize,
    pixels: Vec<u32>,
}

impl Picture {
    fn args(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.width,
            self.height,
            self.stride,
            self.offset,
            hex32(&self.pixels)
        )
    }

    fn webp_picture(&self) -> WebPPicture<'static> {
        WebPPicture {
            use_argb: 1,
            width: self.width,
            height: self.height,
            argb_stride: self.stride,
            argb: Some(PlaneOffset {
                region: 0,
                offset: self.offset,
            }),
            argb_storage: PlaneStorage::Internal(self.pixels.clone()),
            ..WebPPicture::default()
        }
    }

    fn colors(&self) -> Vec<u32> {
        let mut colors = Vec::new();
        for y in 0..self.height {
            let row = (self.offset as i64 + i64::from(y) * i64::from(self.stride)) as usize;
            colors.extend_from_slice(&self.pixels[row..row + self.width as usize]);
        }
        colors.sort_unstable();
        colors.dedup();
        colors
    }
}

/// A set of distinct colors: random, with black, a gray ramp or green only.
fn color_set(rng: &mut Rng, count: usize) -> Vec<u32> {
    let mut colors: Vec<u32> = match rng.below(5) {
        0 => (0..count)
            .map(|i| 0xff00_0000 | (i as u32 * 0x0001_0101))
            .collect(),
        1 => (0..count)
            .map(|i| 0xff00_0000 | ((i as u32) << 8))
            .collect(),
        _ => (0..count).map(|_| rng.next_u64() as u32).collect(),
    };
    if rng.below(3) == 0 {
        colors[0] = 0;
    }
    colors.sort_unstable();
    colors.dedup();
    colors
}

/// A picture of `colors`, with runs along the rows and copies of the row
/// above, or of random colors only when `noisy`.
fn picture(rng: &mut Rng, colors: &[u32], noisy: bool) -> Picture {
    let width = 1 + rng.below(40) as i32 + if noisy { 30 } else { 0 };
    let height = 1 + rng.below(20) as i32 + if noisy { 20 } else { 0 };
    let pad = rng.below(4) as i32;
    let negative = rng.below(4) == 0;
    let row = width + pad;
    let lead = rng.below(3);
    let len = lead + (height * row) as usize;
    let mut pixels = vec![0x1234_5678_u32; len];
    let (stride, offset) = if negative {
        (-row, lead + ((height - 1) * row) as usize)
    } else {
        (row, lead)
    };
    let mut previous = colors[rng.below(colors.len())];
    for y in 0..height {
        let start = (offset as i64 + i64::from(y) * i64::from(stride)) as usize;
        let above = (y > 0).then(|| (start as i64 - i64::from(stride)) as usize);
        for x in 0..width as usize {
            let pix = match (if noisy { 3 } else { rng.below(4) }, above) {
                (0, Some(above)) => pixels[above + x],
                (1, _) => previous,
                _ => colors[rng.below(colors.len())],
            };
            pixels[start + x] = pix;
            previous = pix;
        }
    }
    Picture {
        width,
        height,
        stride,
        offset,
        pixels,
    }
}

fn palette_case(commands: &mut String, expected: &mut String, picture: &Picture, with: bool) {
    writeln!(commands, "palette {} {}", picture.args(), i32::from(with)).unwrap();
    let pic = picture.webp_picture();
    let mut palette = [SENTINEL; ENTRIES];
    let mut palette2 = [SENTINEL; ENTRIES];
    let count = get_color_palette(&pic, with.then_some(&mut palette[..]));
    let count2 = webp_get_color_palette(&pic, with.then_some(&mut palette2[..]));
    writeln!(expected, "count={count}\ncount2={count2}").unwrap();
    if with {
        writeln!(
            expected,
            "palette={}\npalette2={}",
            hex32(&palette),
            hex32(&palette2)
        )
        .unwrap();
    }
}

fn sort_case(
    commands: &mut String,
    expected: &mut String,
    method: u32,
    picture: &Picture,
    sorted: &[u32],
) {
    writeln!(
        commands,
        "sort {method} {} {}",
        picture.args(),
        hex32(sorted)
    )
    .unwrap();
    let pic = picture.webp_picture();
    let mut palette = [SENTINEL; ENTRIES];
    let ok = palette_sort(
        PaletteSorting(method),
        &pic,
        sorted,
        sorted.len() as u32,
        &mut palette,
    );
    writeln!(
        expected,
        "sort={}\npalette={}",
        i32::from(ok),
        hex32(&palette)
    )
    .unwrap();
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0033);
    let mut commands = String::new();
    let mut expected = String::new();
    let methods = [
        PaletteSorting::kSortedDefault.0,
        PaletteSorting::kMinimizeDelta.0,
        PaletteSorting::kModifiedZeng.0,
    ];

    for round in 0..300 {
        let count = match round % 5 {
            0 => 1 + rng.below(4),
            1 => 2 + rng.below(30),
            2 => 200 + rng.below(57),
            3 => 257 + rng.below(100),
            _ => 1 + rng.below(256),
        };
        let colors = color_set(&mut rng, count);
        let picture = picture(&mut rng, &colors, count > 256 || round % 11 == 0);
        palette_case(&mut commands, &mut expected, &picture, true);
        if round % 7 == 0 {
            palette_case(&mut commands, &mut expected, &picture, false);
        }
        let used = picture.colors();
        if used.len() > ENTRIES {
            continue;
        }

        // The inverse map of an unsorted palette, and searches in the sorted one.
        let mut shuffled = used.clone();
        for i in (1..shuffled.len()).rev() {
            shuffled.swap(i, rng.below(i + 1));
        }
        writeln!(commands, "prepare {}", hex32(&shuffled)).unwrap();
        let mut sorted = [SENTINEL; ENTRIES];
        let mut map = [SENTINEL; ENTRIES];
        prepare_map_to_palette(&shuffled, shuffled.len() as u32, &mut sorted, &mut map);
        writeln!(expected, "sorted={}\nmap={}", hex32(&sorted), hex32(&map)).unwrap();
        for _ in 0..3 {
            let color = used[rng.below(used.len())];
            writeln!(commands, "search {} {color}", hex32(&used)).unwrap();
            let index = search_color_no_idx(&used, color, used.len() as i32);
            writeln!(expected, "index={index}").unwrap();
        }

        // Every sorting method on the picture's palette, and on palettes with
        // colors the picture does not have; one color returns early.
        for &method in &methods {
            sort_case(&mut commands, &mut expected, method, &picture, &used);
        }
        if used.len() >= 2 && used.len() < ENTRIES - 20 {
            let mut larger = used.clone();
            let extra = 1 + rng.below(20);
            larger.extend(color_set(&mut rng, extra));
            larger.sort_unstable();
            larger.dedup();
            larger.truncate(ENTRIES);
            for &method in &methods {
                sort_case(&mut commands, &mut expected, method, &picture, &larger);
            }
        }
    }
    (commands, expected)
}

#[test]
fn palette_analysis_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("palette_analysis_matches_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let sorts = expected
        .lines()
        .filter(|line| line.starts_with("sort="))
        .count();
    let large = expected.lines().filter(|line| *line == "count=257").count();
    eprintln!(
        "{sorts} sorts, {large} pictures with more than 256 colors, {} lines",
        expected.lines().count()
    );
    assert!(
        sorts > 700 && large > 30,
        "{sorts} sorts, {large} large pictures"
    );
    let output = oracle
        .probe("wp3", "palette")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let differs = expected
        .lines()
        .zip(actual.lines())
        .position(|(e, a)| e != a)
        .map_or_else(String::new, |line| {
            format!(
                "line {line}: expected {:.120}, the C probe printed {:.120}",
                expected.lines().nth(line).unwrap_or_default(),
                actual.lines().nth(line).unwrap_or_default()
            )
        });
    assert!(
        actual == expected,
        "the palette functions differ from C: {differs}"
    );
}
