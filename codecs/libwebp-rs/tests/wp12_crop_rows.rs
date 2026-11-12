//! Cropping measurement for Ferrisma (`docs/adr/0010-ferrisma-adoption.md`):
//! which rows a cropped or scaled decoding touches, counted in the C library
//! by the probe `tools/oracle/probes/wp12/crop_rows.c` and in the Rust
//! translation here, through the internal API that `DecodeInto` uses
//! (`VP8Decode`, `VP8LDecodeImage`) with a counting `put()`, into RGBA. The
//! two must agree on every number; the test prints them.
//!
//! VP8 columns: the macroblock rows of the image, the rows that `ParseFrame`
//! parses (`br_mb_y`), the filter type (0 none or bypassed, 1 simple, 2
//! complex), the rows that are in-loop filtered, the `put()` calls with their
//! first row and number of rows, and the output rows. VP8L: the height, the
//! rows that `DecodeImageData` decodes (`last_row`) and the output rows
//! (`last_out_row`).
//!
//! The test covers six vectors with six crops each, with and without scaling.
//! This representative subset shows the result (crop as left,top and size):
//!
//! | Vector | Crop (left,top size) | Scale | Parsed rows | Filtered rows | `put()` | Output rows |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | `vp80-00-comprehensive-008.webp` (1432 x 888) | none | - | 56 of 56 | 0 (type 0) | 56 (888 rows) | 888 |
//! | | none | 716x444 | 56 of 56 | 0 (type 0) | 56 (888 rows) | 444 |
//! | | 0,0 1432x222 | - | 14 of 56 | 0 (type 0) | 14 (222 rows) | 222 |
//! | | 0,333 1432x222 | - | 35 of 56 | 0 (type 0) | 15 (222 rows) | 222 |
//! | | 0,666 1432x222 | - | 56 of 56 | 0 (type 0) | 15 (222 rows) | 222 |
//! | | 0,0 716x888 | - | 56 of 56 | 0 (type 0) | 56 (888 rows) | 888 |
//! | | 684,412 64x64 | - | 30 of 56 | 0 (type 0) | 5 (64 rows) | 64 |
//! | `vp80-05-sharpness-1430.webp` (352 x 288) | none | - | 18 of 18 | 18 (type 2) | 18 (288 rows) | 288 |
//! | | none | 176x144 | 18 of 18 | 0 (bypassed) | 18 (288 rows) | 144 |
//! | | 0,0 352x72 | - | 5 of 18 | 5 (type 2) | 5 (72 rows) | 72 |
//! | | 0,108 352x72 | - | 12 of 18 | 12 (type 2) | 5 (72 rows) | 72 |
//! | | 0,216 352x72 | - | 18 of 18 | 18 (type 2) | 4 (72 rows) | 72 |
//! | | 0,0 176x288 | - | 18 of 18 | 18 (type 2) | 18 (288 rows) | 288 |
//! | | 144,112 64x64 | - | 12 of 18 | 12 (type 2) | 5 (64 rows) | 64 |
//! | | 144,112 64x64 | 32x32 | 11 of 18 | 0 (bypassed) | 4 (64 rows) | 32 |
//! | `lossy_alpha1.webp` (1000 x 307) | none | - | 20 of 20 | 20 (type 1) | 20 (307 rows) | 307 |
//! | | 0,0 1000x76 | - | 5 of 20 | 5 (type 1) | 5 (76 rows) | 76 |
//! | | 0,115 1000x76 | - | 12 of 20 | 5 (type 1) | 5 (76 rows) | 76 |
//! | | 0,231 1000x76 | - | 20 of 20 | 6 (type 1) | 6 (76 rows) | 76 |
//! | | 0,0 500x307 | - | 20 of 20 | 20 (type 1) | 20 (307 rows) | 307 |
//! | | 468,121 64x64 | - | 12 of 20 | 5 (type 1) | 5 (64 rows) | 64 |
//! | | 468,121 64x64 | 32x32 | 12 of 20 | 0 (bypassed) | 5 (64 rows) | 32 |
//! | `lossless1.webp` (1000 x 307, VP8L) | none | - | 307 of 307 | | | 307 |
//! | | 0,0 1000x76 | - | 76 of 307 | | | 76 |
//! | | 0,115 1000x76 | - | 191 of 307 | | | 76 |
//! | | 0,231 1000x76 | - | 307 of 307 | | | 76 |
//! | | 0,0 500x307 | - | 307 of 307 | | | 307 |
//! | | 468,121 64x64 | 32x32 | 185 of 307 | | | 32 |
//! | `lossless_color_transform.webp` (512 x 512, VP8L) | 0,192 512x128 | - | 320 of 512 | | | 128 |
//! | | 224,224 64x64 | - | 288 of 512 | | | 64 |
//!
//! What they show, which the source predicts (`src/dec/frame_dec.c`,
//! `vp8_dec.c`, `io_dec.c`, `vp8l_dec.c`):
//!
//! - **VP8 parsing** always starts at macroblock row 0 and stops at
//!   `br_mb_y = (crop_bottom + 15 + extra) >> 4`, where `extra` is 0, 2 or 8 rows
//!   of filter margin for no, simple and complex filtering. A crop saves the rows
//!   below it, never the rows above it.
//! - **VP8 filtering**: the complex filter (type 2) filters every row from 0 to
//!   `br_mb_y`, because of its dependency chain; the simple filter (type 1) filters
//!   from `(crop_top - 2) >> 4` on. Scaling below 3/4 in both dimensions bypasses
//!   the in-loop filter (`WebPIoInitFromOptions`), which changes the pixels.
//! - **VP8 output**: `put()` receives only the cropped rows, at full resolution;
//!   scaling runs after reconstruction, inside the emit functions.
//! - **Horizontal cropping saves nothing** in VP8 or VP8L: every macroblock or
//!   pixel of the parsed rows is decoded.
//! - **VP8L** decodes every row from 0 to `crop_bottom` (`last_row`) and outputs
//!   only the cropped rows; scaling runs after full-resolution decoding.
//! - **The incremental decoder** parses every macroblock row of a VP8 image,
//!   whatever the crop: `DecodeRemaining` loops to `mb_h`, not to `br_mb_y`, and
//!   `VP8ProcessRow` emits nothing below the crop (source reading;
//!   `WebPIDecoder` exposes no counters).
//! - **Alpha** (`ALPH` chunk of a lossy image): `VP8DecompressAlphaRows` decodes
//!   alpha rows from 0 up to the last row it is asked for, so the same rows as the
//!   luma; with alpha dithering (pre-processed levels) it decodes the whole plane
//!   at once (source reading).

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::buffer_dec::webp_allocate_dec_buffer;
use webp::dec::frame_dec::vp8_init_dithering;
use webp::dec::io_dec::{CustomIo, webp_init_custom_io};
use webp::dec::vp8_dec::{
    VP8Io, VP8IoHooks, VP8IoRows, vp8_decode, vp8_get_headers, vp8_init_io, vp8_new,
};
use webp::dec::vp8l_dec::{vp8l_decode_header, vp8l_decode_image, vp8l_new};
use webp::dec::webp_dec::webp_parse_headers;
use webp::dec::webpi_dec::{WebPDecParams, WebPHeaderStructure};
use webp::webp::decode::{WEBP_CSP_MODE, WebPDecBuffer};
use wp12_support::{Options, compare};

/// The custom hooks with a counting `put()`.
struct Counting<'p, 'o, 'a> {
    inner: CustomIo<'p, 'o, 'a>,
    calls: i32,
    first: i32,
    rows: i32,
}

impl VP8IoHooks for Counting<'_, '_, '_> {
    fn setup(&mut self, io: &mut VP8Io) -> bool {
        self.inner.setup(io)
    }

    fn put(&mut self, io: &VP8Io, rows: &mut VP8IoRows<'_>) -> bool {
        if self.calls == 0 {
            self.first = io.mb_y;
        }
        self.calls += 1;
        self.rows += io.mb_h;
        self.inner.put(io, rows)
    }

    fn teardown(&mut self, io: &VP8Io) {
        self.inner.teardown(io);
    }
}

/// The probe's `rows` command in the translation.
fn rows(out: &mut String, data: &[u8], mode: u32, options: Options) {
    let options = options.decoder_options();
    let mut headers = WebPHeaderStructure {
        data,
        have_all_data: true,
        ..WebPHeaderStructure::default()
    };
    webp_parse_headers(&mut headers).expect("valid headers");
    let mut output = WebPDecBuffer::default();
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    let mut params = WebPDecParams {
        options: Some(&options),
        ..WebPDecParams::default()
    };
    output.colorspace = WEBP_CSP_MODE(mode);
    let io_data = &headers.data[headers.offset..];
    if headers.is_lossless {
        let mut dec = vp8l_new();
        vp8l_decode_header(&mut dec, &mut io, io_data).expect("valid header");
        webp_allocate_dec_buffer(io.width, io.height, Some(&options), &mut output)
            .expect("valid options");
        let ok = vp8l_decode_image(&mut dec, &mut io, io_data, &mut params, &mut output).is_ok();
        writeln!(
            out,
            "vp8l={},{},{},{}",
            i32::from(ok),
            io.height,
            dec.last_row,
            dec.last_out_row
        )
        .unwrap();
    } else {
        let mut dec = vp8_new();
        let alpha = headers
            .alpha_data
            .map(|alpha| &headers.data[alpha..alpha + headers.alpha_data_size]);
        vp8_get_headers(&mut dec, &mut io, io_data).expect("valid header");
        webp_allocate_dec_buffer(io.width, io.height, Some(&options), &mut output)
            .expect("valid options");
        dec.mt_method = 0;
        vp8_init_dithering(Some(&options), &mut dec);
        let mut hooks = Counting {
            inner: webp_init_custom_io(&mut params, &mut output, &mut io),
            calls: 0,
            first: -1,
            rows: 0,
        };
        let ok = vp8_decode(&mut dec, &mut io, io_data, alpha, Some(&mut hooks));
        let (calls, first, put_rows) = (hooks.calls, hooks.first, hooks.rows);
        let filtered = if dec.filter_type > 0 {
            dec.br_mb_y - dec.tl_mb_y
        } else {
            0
        };
        writeln!(
            out,
            "vp8={},{},{},{},{filtered},{calls},{first},{put_rows},{}",
            i32::from(ok),
            dec.mb_h,
            dec.br_mb_y,
            dec.filter_type,
            params.last_y
        )
        .unwrap();
    }
}

#[test]
fn cropped_decodings_touch_the_same_rows_as_c() {
    let Some(oracle) = Oracle::locate("cropped_decodings_touch_the_same_rows_as_c") else {
        return;
    };
    let mut commands = String::new();
    let mut expected = String::new();
    for (name, width, height) in [
        ("vp80-00-comprehensive-008.webp", 1432, 888),
        ("vp80-05-sharpness-1430.webp", 352, 288),
        ("lossy_alpha1.webp", 1000, 307),
        ("test.webp", 128, 128),
        ("lossless1.webp", 1000, 307),
        ("lossless_color_transform.webp", 512, 512),
    ] {
        let path = oracle::test_data().join(name);
        let data = std::fs::read(&path).unwrap();
        let crops = [
            None,
            Some((0, 0, width, height / 4)),
            Some((0, height / 2 - height / 8, width, height / 4)),
            Some((0, height - height / 4, width, height / 4)),
            Some((0, 0, width / 2, height)),
            Some((width / 2 - 32, height / 2 - 32, 64, 64)),
        ];
        for crop in crops {
            for scale in [false, true] {
                let mut options = Options::default();
                let (w, h) = if let Some((left, top, w, h)) = crop {
                    options.0[Options::CROP] = 1;
                    options.0[Options::CROP_LEFT] = left;
                    options.0[Options::CROP_TOP] = top;
                    options.0[Options::CROP_WIDTH] = w;
                    options.0[Options::CROP_HEIGHT] = h;
                    (w, h)
                } else {
                    (width, height)
                };
                if scale {
                    options.0[Options::SCALE] = 1;
                    options.0[Options::SCALED_WIDTH] = (w / 2).max(1);
                    options.0[Options::SCALED_HEIGHT] = (h / 2).max(1);
                }
                let line = format!("rows {} 1 {}\n", path.display(), options.token());
                commands.push_str(&line);
                write!(expected, "> {line}").unwrap();
                rows(&mut expected, &data, 1, options);
            }
        }
    }
    let output = oracle
        .probe("wp12", "crop_rows")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    compare(
        "cropped_decodings_touch_the_same_rows_as_c",
        &output.stdout,
        &expected,
    );
    println!("{expected}");
}
