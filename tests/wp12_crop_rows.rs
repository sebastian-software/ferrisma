//! WP12 measurement for Ferrixel (`docs/PORT_PLAN.md`, WP21): which rows a
//! cropped or scaled decoding touches, counted in the C library by the probe
//! `tools/oracle/probes/wp12/crop_rows.c` and in the Rust translation here,
//! through the internal API that `DecodeInto` uses, with a counting `put()`.
//! The two must agree; `docs/status/wp12.md` records the numbers, which this
//! test prints.
//!
//! VP8: the macroblock rows of the image, the rows that `ParseFrame` parses
//! (`br_mb_y`), the filter type, the rows that are in-loop filtered, the
//! `put()` calls with their first row and number of rows, and the output
//! rows. VP8L: the height, the rows that `DecodeImageData` decodes
//! (`last_row`) and the output rows (`last_out_row`).

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
