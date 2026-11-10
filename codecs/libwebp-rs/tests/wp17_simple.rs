//! WP17 differential test: `encode_lossy` and `encode_lossless` of the safe
//! encoder API (ADR 0006) against C's simple encoding API, through the
//! `simple` command of WP13's probe `tools/oracle/probes/wp13/api.c`
//! (`WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA` and
//! the four `WebPEncodeLossless*` functions). The commands and the rows are
//! those of `tests/wp13_simple.rs`; the probe's xorshift32 generator is
//! reproduced by `fill`. Compared: the size, whether C wrote the output
//! (`untouched` where `WebPConfigPreset` refused the quality, which the API
//! reports as `InvalidParameter`; `null` for every other failure) and the
//! bytes.
//!
//! RGBX and BGRX have no simple function in C. Their importers read the same
//! three channels as RGB and BGR and never the fourth byte, so the API's
//! output for X rows built from the same pixels, with random fourth bytes, is
//! compared with C's output for the RGB and BGR commands.
//!
//! Excluded (undefined behavior in C, see `# Excluded undefined behavior` in
//! `src/enc/picture_csp_enc.rs`): `WebPEncodeLosslessRGBA` on rows whose
//! starts are not aligned for `uint32_t`.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::api::encode::{EncodeError, PixelLayout, PixelRows, encode_lossless, encode_lossy};

/// The probe's `Fill`: xorshift32 bytes, with the fourth byte of each pixel
/// forced by `amode`, or the repetitive opaque pattern of `amode` 3.
fn fill(size: usize, seed: u32, amode: i32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..size)
        .map(|i| {
            if amode == 3 {
                let (p, row) = (i / 4, i / 4 / 61);
                return if i & 3 == 3 {
                    0xff
                } else {
                    (((p % 23) * 11 + (row % 7) * 5 + (seed as usize & 7) * (i & 3)) & 0xf8) as u8
                };
            }
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            let mut byte = s as u8;
            if i & 3 == 3 {
                if amode == 1 {
                    byte = 0xff;
                }
                if amode == 2 {
                    byte = if byte & 0x10 != 0 { 0xff } else { 0 };
                }
            }
            byte
        })
        .collect()
}

/// The probe's function index `f % 4`: RGB, BGR, RGBA, BGRA.
const LAYOUTS: [PixelLayout; 4] = [
    PixelLayout::Rgb,
    PixelLayout::Bgr,
    PixelLayout::Rgba,
    PixelLayout::Bgra,
];

/// One `simple` command.
#[derive(Clone, Copy, Debug)]
struct Command {
    f: usize,
    w: i32,
    h: i32,
    stride: i32,
    quality: f32,
    seed: u32,
    amode: i32,
}

impl Command {
    fn line(&self) -> String {
        format!(
            "simple {} {} {} {} {} {} {} 0\n",
            self.f,
            self.w,
            self.h,
            self.stride,
            self.quality.to_bits() as i32,
            self.seed,
            self.amode
        )
    }

    /// The probe's input rows.
    fn input(&self) -> Vec<u8> {
        let rows = self.h.max(1) as usize;
        fill(rows * self.stride as usize + 64, self.seed, self.amode)
    }
}

/// What the probe prints for a result of the API.
fn fields(result: Result<Vec<u8>, EncodeError>) -> Fields {
    let fields = Fields::new().with("cmd", "simple");
    match result {
        Ok(bytes) => fields
            .with("size", bytes.len())
            .with("output", "set")
            .hex("out", &bytes),
        Err(EncodeError::InvalidParameter { name: "quality" }) => {
            fields.with("size", 0).with("output", "untouched")
        }
        Err(_) => fields.with("size", 0).with("output", "null"),
    }
}

fn encode(command: &Command, layout: PixelLayout, data: &[u8], stride: usize) -> Fields {
    let rows = PixelRows {
        layout,
        data,
        width: command.w as u32,
        height: command.h as u32,
        stride,
    };
    fields(if command.f < 4 {
        encode_lossy(rows, command.quality)
    } else {
        encode_lossless(rows)
    })
}

/// The API on the command's rows.
fn api(command: &Command) -> Fields {
    let input = command.input();
    encode(
        command,
        LAYOUTS[command.f % 4],
        &input,
        command.stride as usize,
    )
}

/// The API on X rows with the pixels of an RGB or BGR command: each pixel
/// gains a random fourth byte, and a valid stride four bytes per pixel.
fn api_x(command: &Command) -> Fields {
    let input = command.input();
    let (w, h) = (command.w.max(0) as usize, command.h.max(1) as usize);
    let stride = command.stride as usize;
    let x_stride = if stride >= 3 * w && w > 0 {
        stride + w
    } else {
        stride
    };
    let mut garbage = 0x2545_f491_u32 ^ command.seed;
    let mut x = vec![0_u8; h * x_stride + 64];
    for byte in &mut x {
        garbage ^= garbage << 13;
        garbage ^= garbage >> 17;
        garbage ^= garbage << 5;
        *byte = garbage as u8;
    }
    if x_stride != stride {
        for row in 0..h {
            for pixel in 0..w {
                let from = row * stride + 3 * pixel;
                let to = row * x_stride + 4 * pixel;
                x[to..to + 3].copy_from_slice(&input[from..from + 3]);
            }
        }
    }
    let layout = if command.f.is_multiple_of(4) {
        PixelLayout::Rgbx
    } else {
        PixelLayout::Bgrx
    };
    encode(command, layout, &x, x_stride)
}

/// The commands of `tests/wp13_simple.rs`.
fn commands() -> Vec<Command> {
    let mut commands = Vec::new();
    let mut seed = 0x1357_u32;
    for f in 0..8_usize {
        let step = if f % 4 < 2 { 3 } else { 4 };
        for (w, h) in [
            (1, 1),
            (2, 3),
            (7, 5),
            (16, 16),
            (33, 17),
            (64, 9),
            (0, 4),
            (5, 0),
        ] {
            let tight = step * w.max(1);
            for stride in [tight, tight + 3, tight - 1, 0] {
                for quality in [0.0_f32, 30.0, 75.0, 100.0, -0.5, 100.5] {
                    if f >= 4 && quality != 75.0 {
                        continue;
                    }
                    seed = seed.wrapping_mul(2_654_435_761).wrapping_add(17);
                    let amode = (seed >> 7) as i32 % 3;
                    let seed = seed >> 2;
                    // WebPEncodeLosslessRGBA loads rows whose starts are not
                    // aligned for uint32_t, which is undefined (excluded).
                    if f == 6 && h > 1 && stride % 4 != 0 {
                        continue;
                    }
                    commands.push(Command {
                        f,
                        w,
                        h,
                        stride,
                        quality,
                        seed,
                        amode,
                    });
                }
            }
        }
    }
    // Repetitive pictures, whose lossless encodings depend on the quality.
    for f in 0..8_usize {
        let step = if f % 4 < 2 { 3 } else { 4 };
        for (w, h) in [(61, 40), (128, 33), (200, 9)] {
            seed = seed.wrapping_mul(2_654_435_761).wrapping_add(17);
            commands.push(Command {
                f,
                w,
                h,
                stride: step * w,
                quality: 75.0,
                seed: seed >> 2,
                amode: 3,
            });
        }
    }
    commands
}

/// The probe's output split into one `Fields` per command.
fn split(all: Fields) -> Vec<Fields> {
    let mut out: Vec<Fields> = Vec::new();
    for (key, value) in all.0 {
        if key == "cmd" {
            out.push(Fields::new());
        }
        out.last_mut()
            .expect("the output starts with cmd=")
            .0
            .push((key, value));
    }
    out
}

#[test]
fn simple_functions_match_the_c_functions() {
    let test = "simple_functions_match_the_c_functions";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let commands = commands();
    let input: String = commands.iter().map(Command::line).collect();
    let output = oracle
        .probe("wp13", "api")
        .run::<&str>(&[], input.as_bytes());
    output.success();
    let expected = split(output.fields());
    assert_eq!(
        expected.len(),
        commands.len(),
        "{test}: one output per command"
    );
    let (mut encoded, mut x_encoded, mut untouched) = (0, 0, 0);
    for (command, c) in commands.iter().zip(&expected) {
        assert_eq!(&api(command), c, "{test}: {command:?}");
        if c.get("output") == Some("set") {
            encoded += 1;
        }
        if c.get("output") == Some("untouched") {
            untouched += 1;
        }
        if command.f % 4 < 2 {
            assert_eq!(&api_x(command), c, "{test}: X rows of {command:?}");
            if c.get("output") == Some("set") {
                x_encoded += 1;
            }
        }
    }
    assert!(encoded > 200 && x_encoded > 100 && untouched > 0);
    eprintln!(
        "{test}: {} commands, {encoded} encoded, {x_encoded} also from X rows, {untouched} refused qualities",
        commands.len()
    );
}
