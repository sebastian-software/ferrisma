//! WP6 differential test: the output functions of `src/dec/io_dec.c` through
//! `WebPInitCustomIo` (`CustomSetup`, `CustomPut` with every emit function, the
//! YUV and RGB rescalers and the alpha emitters, `CustomTeardown`), against the
//! C probe `tools/oracle/probes/wp6/io.c`.
//!
//! Each case is a seeded picture of W by H pixels with random Y, U, V and
//! optional alpha planes, fed to `put()` in the row chunks that `FinishRow`
//! computes for 0, 2 and 8 rows held back by the loop filter, into an output
//! buffer that the test lays out: every `WEBP_CSP_MODE`, padded strides, flipped
//! buffers (negative strides), one allocation or one per plane, with and
//! without options, cropping (valid and invalid windows), scaling (explicit,
//! derived and invalid sizes) and point or fancy upsampling. The test compares
//! the result of each call, the io after setup, `last_y` and every byte of the
//! output allocations, which start filled with 0xa5.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::io_dec::webp_init_custom_io;
use webp::dec::vp8_dec::{VP8Io, VP8IoHooks, VP8IoRows, vp8_init_io};
use webp::dec::webpi_dec::WebPDecParams;
use webp::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use webp::webp::decode::{
    WEBP_CSP_MODE, WebPDecBuffer, WebPDecBufferPlanes, WebPDecoderOptions, WebPRGBABuffer,
    WebPYUVABuffer, webp_is_alpha_mode, webp_is_rgb_mode,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};
use wp6_support::{Rng, compare, hex, print_digest, print_io, token};

/// One output plane: region, offset, stride and size.
#[derive(Clone, Copy)]
struct Plane {
    region: i64,
    offset: usize,
    stride: i32,
    size: usize,
}

struct Case {
    mode: u32,
    width: i32,
    height: i32,
    extra: i32,
    options: Option<WebPDecoderOptions>,
    y: Vec<u8>,
    u: Vec<u8>,
    v: Vec<u8>,
    a: Vec<u8>,
    out_w: i32,
    out_h: i32,
    regions: Vec<usize>,
    planes: Vec<Plane>,
    dump: bool,
}

impl Case {
    fn command(&self) -> String {
        let mut c = format!(
            "case {} {} {} {} ",
            self.mode, self.width, self.height, self.extra
        );
        match &self.options {
            None => c.push_str("0 "),
            Some(o) => write!(
                c,
                "1 {} {} {} {} {} {} {} {} {} {} ",
                o.bypass_filtering,
                o.no_fancy_upsampling,
                o.use_cropping,
                o.crop_left,
                o.crop_top,
                o.crop_width,
                o.crop_height,
                o.use_scaling,
                o.scaled_width,
                o.scaled_height
            )
            .unwrap(),
        }
        write!(
            c,
            "{} {} {} {} {} {} ",
            token(&self.y),
            token(&self.u),
            token(&self.v),
            token(&self.a),
            self.out_w,
            self.out_h
        )
        .unwrap();
        let sizes: Vec<String> = self.regions.iter().map(usize::to_string).collect();
        write!(c, "{} ", sizes.join(",")).unwrap();
        for plane in &self.planes {
            write!(
                c,
                "{} {} {} {} ",
                plane.region, plane.offset, plane.stride, plane.size
            )
            .unwrap();
        }
        writeln!(c, "{}", i32::from(self.dump)).unwrap();
        c
    }
}

fn bytes_per_pixel(mode: u32) -> i32 {
    match mode {
        0 | 2 => 3,
        5 | 6 | 10 => 2,
        _ => 4,
    }
}

/// The output size that `WebPAllocateDecBuffer` derives from the options.
fn output_size(width: i32, height: i32, options: Option<&WebPDecoderOptions>) -> (i32, i32) {
    let (mut w, mut h) = (width, height);
    if let Some(o) = options {
        if o.use_cropping != 0 {
            w = o.crop_width;
            h = o.crop_height;
        }
        if o.use_scaling != 0 {
            let (mut sw, mut sh) = (o.scaled_width, o.scaled_height);
            if webp_rescaler_get_scaled_dimensions(w, h, &mut sw, &mut sh) {
                w = sw;
                h = sh;
            }
        }
    }
    (w, h)
}

fn random_case(rng: &mut Rng, mode: u32) -> Case {
    let (width, height) = match rng.below(10) {
        0 => (1, rng.range(1, 40) as i32),
        1 => (rng.range(1, 40) as i32, 1),
        2 => (16 * rng.range(1, 2) as i32, 16 * rng.range(1, 2) as i32),
        _ => (rng.range(1, 40) as i32, rng.range(1, 40) as i32),
    };
    let extra = [0, 2, 8][rng.below(3) as usize];
    let has_alpha = rng.chance(1, 2);
    let options = if rng.chance(1, 4) {
        None
    } else {
        let mut o = WebPDecoderOptions {
            bypass_filtering: rng.below(2) as i32,
            no_fancy_upsampling: rng.below(2) as i32,
            ..WebPDecoderOptions::default()
        };
        if rng.chance(2, 5) {
            o.use_cropping = 1;
            o.crop_left = rng.range(0, i64::from(width) - 1) as i32;
            o.crop_top = rng.range(0, i64::from(height) - 1) as i32;
            o.crop_width = rng.range(1, i64::from(width - o.crop_left)) as i32;
            o.crop_height = rng.range(1, i64::from(height - o.crop_top)) as i32;
            if rng.chance(1, 10) {
                // A window past the edge, which setup rejects.
                o.crop_width += rng.range(1, 3) as i32;
            }
        }
        if rng.chance(2, 5) {
            o.use_scaling = 1;
            o.scaled_width = rng.range(1, 60) as i32;
            o.scaled_height = rng.range(1, 60) as i32;
            match rng.below(8) {
                0 => o.scaled_width = 0,
                1 => o.scaled_height = 0,
                2 => o.scaled_width = -rng.range(1, 3) as i32,
                _ => {}
            }
        }
        Some(o)
    };
    let mut options = options;
    if let Some(o) = &mut options {
        // Keep derived scaled sizes small: an output larger than the buffer is
        // not a case of interest.
        let (w, h) = output_size(width, height, Some(o));
        if o.use_scaling != 0 && (w > 128 || h > 128) {
            o.scaled_width = rng.range(1, 60) as i32;
            o.scaled_height = rng.range(1, 60) as i32;
        }
    }
    let uv_w = (width + 1) / 2;
    let uv_h = (height + 1) / 2;
    let smooth = rng.chance(1, 2);
    let plane = |n: usize, rng: &mut Rng| -> Vec<u8> {
        if smooth {
            let base = rng.range(0, 255);
            (0..n)
                .map(|i| (base + (i as i64 % 7) * 3 + rng.range(-4, 4)).clamp(0, 255) as u8)
                .collect()
        } else {
            rng.bytes(n)
        }
    };
    let y = plane((width * height) as usize, rng);
    let u = plane((uv_w * uv_h) as usize, rng);
    let v = plane((uv_w * uv_h) as usize, rng);
    let a = if has_alpha {
        match rng.below(3) {
            0 => vec![0xff; (width * height) as usize],
            1 => (0..width * height)
                .map(|_| if rng.chance(1, 8) { rng.byte() } else { 0xff })
                .collect(),
            _ => rng.bytes((width * height) as usize),
        }
    } else {
        Vec::new()
    };

    let (out_w, out_h) = output_size(width, height, options.as_ref());
    // An invalid crop or scale fails in setup, before anything is written.
    let (out_w, out_h) = (out_w.max(1), out_h.max(1));
    let flip = rng.chance(1, 3);
    let mut regions = Vec::new();
    let mut planes = Vec::new();
    let layout = |w: i32, h: i32, pad: i32, separate: bool, regions: &mut Vec<usize>| {
        let stride = w + pad;
        let size = (stride * h) as usize;
        let (region, base) = if separate || regions.is_empty() {
            regions.push(size);
            (regions.len() - 1, 0)
        } else {
            let base = regions[0];
            regions[0] += size;
            (0, base)
        };
        let (offset, stride) = if flip {
            (base + (stride * (h - 1)) as usize, -stride)
        } else {
            (base, stride)
        };
        Plane {
            region: region as i64,
            offset,
            stride,
            size,
        }
    };
    let mode_csp = WEBP_CSP_MODE(mode);
    if webp_is_rgb_mode(mode_csp) {
        let pad = rng.range(0, 7) as i32;
        planes.push(layout(
            out_w * bytes_per_pixel(mode),
            out_h,
            pad,
            false,
            &mut regions,
        ));
    } else {
        let separate = rng.chance(1, 2);
        let pad = rng.range(0, 5) as i32;
        let uv_w = (out_w + 1) / 2;
        let uv_h = (out_h + 1) / 2;
        planes.push(layout(out_w, out_h, pad, separate, &mut regions));
        planes.push(layout(uv_w, uv_h, pad, separate, &mut regions));
        planes.push(layout(uv_w, uv_h, pad, separate, &mut regions));
        if webp_is_alpha_mode(mode_csp) {
            planes.push(layout(out_w, out_h, pad, separate, &mut regions));
        } else {
            planes.push(Plane {
                region: -1,
                offset: 0,
                stride: 0,
                size: 0,
            });
        }
    }
    let dump = out_w * out_h <= 64;
    Case {
        mode,
        width,
        height,
        extra,
        options,
        y,
        u,
        v,
        a,
        out_w,
        out_h,
        regions,
        planes,
        dump,
    }
}

fn plane_offset(plane: &Plane) -> Option<PlaneOffset> {
    (plane.region >= 0).then_some(PlaneOffset {
        region: plane.region as usize,
        offset: plane.offset,
    })
}

fn print_regions(out: &mut String, storage: &PlaneStorage<'_, u8>, count: usize, dump: bool) {
    for r in 0..count {
        let bytes = storage.region(r).expect("a region");
        if dump {
            writeln!(out, "region{r}={}", hex(bytes)).unwrap();
        } else {
            print_digest(out, &format!("region{r}"), bytes);
        }
    }
}

/// Runs a case through the Rust translation and returns its output.
fn run_case(case: &Case) -> String {
    if std::env::var_os("WP6_IO_DEBUG").is_some() {
        eprintln!("{}", case.command().chars().take(400).collect::<String>());
    }
    let mut out = String::new();
    let mode = WEBP_CSP_MODE(case.mode);
    let mut memory: Vec<Vec<u8>> = case.regions.iter().map(|&size| vec![0xa5; size]).collect();
    let storage = if memory.len() == 1 {
        PlaneStorage::Internal(memory.pop().expect("one region"))
    } else {
        PlaneStorage::External(memory.iter_mut().map(Vec::as_mut_slice).collect())
    };
    let planes = if webp_is_rgb_mode(mode) {
        let p = &case.planes[0];
        WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
            rgba: plane_offset(p).expect("an RGBA plane"),
            stride: p.stride,
            size: p.size,
        })
    } else {
        let [y, u, v, a] = [0, 1, 2, 3].map(|i| case.planes[i]);
        WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
            y: plane_offset(&y).expect("a Y plane"),
            u: plane_offset(&u).expect("a U plane"),
            v: plane_offset(&v).expect("a V plane"),
            a: plane_offset(&a),
            y_stride: y.stride,
            u_stride: u.stride,
            v_stride: v.stride,
            a_stride: a.stride,
            y_size: y.size,
            u_size: u.size,
            v_size: v.size,
            a_size: a.size,
        })
    };
    let mut output = WebPDecBuffer {
        colorspace: mode,
        width: case.out_w,
        height: case.out_h,
        u: planes,
        storage,
        ..WebPDecBuffer::default()
    };
    let mut params = WebPDecParams {
        options: case.options.as_ref(),
        ..WebPDecParams::default()
    };
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    let (width, height) = (case.width, case.height);
    io.width = width;
    io.height = height;
    io.crop_right = width;
    io.crop_bottom = height;
    io.scaled_width = width;
    io.scaled_height = height;
    io.mb_w = width;
    io.mb_h = height;
    io.y_stride = width;
    io.uv_stride = (width + 1) / 2;

    let mut y = case.y.clone();
    let u = &case.u;
    let v = &case.v;
    let alpha = (!case.a.is_empty()).then_some(case.a.as_slice());
    {
        let mut hooks = webp_init_custom_io(&mut params, &mut output, &mut io);
        let ok = hooks.setup(&mut io);
        writeln!(out, "setup={}", i32::from(ok)).unwrap();
        print_io(&mut out, &io);
        if ok {
            let mb_h = (height + 15) >> 4;
            let br_mb_y = ((io.crop_bottom + 15 + case.extra) >> 4).min(mb_h);
            let mut ok = true;
            let mut mb_y = 0;
            while mb_y < br_mb_y && ok {
                let is_first_row = mb_y == 0;
                let is_last_row = mb_y >= br_mb_y - 1;
                let mut y_start = mb_y * 16;
                let mut y_end = (mb_y + 1) * 16;
                if !is_first_row {
                    y_start -= case.extra;
                }
                if !is_last_row {
                    y_end -= case.extra;
                }
                y_end = y_end.min(io.crop_bottom);
                let mut y_pos = (y_start * io.y_stride) as usize;
                let mut uv_pos = ((y_start >> 1) * io.uv_stride) as usize;
                let mut a_pos =
                    (alpha.is_some() && y_start < y_end).then(|| (y_start * width) as usize);
                if y_start < io.crop_top {
                    let delta_y = io.crop_top - y_start;
                    y_start = io.crop_top;
                    y_pos += (io.y_stride * delta_y) as usize;
                    uv_pos += (io.uv_stride * (delta_y >> 1)) as usize;
                    if let Some(pos) = &mut a_pos {
                        *pos += (io.width * delta_y) as usize;
                    }
                }
                if y_start < y_end {
                    y_pos += io.crop_left as usize;
                    let uv = uv_pos + (io.crop_left >> 1) as usize;
                    if let Some(pos) = &mut a_pos {
                        *pos += io.crop_left as usize;
                    }
                    io.mb_y = y_start - io.crop_top;
                    io.mb_w = io.crop_right - io.crop_left;
                    io.mb_h = y_end - y_start;
                    let mut rows = VP8IoRows {
                        y: &mut y[y_pos..],
                        u: &u[uv..],
                        v: &v[uv..],
                        a: a_pos.and(alpha),
                        a_offset: a_pos.unwrap_or(0),
                    };
                    ok = hooks.put(&io, &mut rows);
                    writeln!(
                        out,
                        "put={},{},{},{} last_y={}",
                        i32::from(ok),
                        io.mb_y,
                        io.mb_w,
                        io.mb_h,
                        hooks.params.last_y
                    )
                    .unwrap();
                    print_regions(
                        &mut out,
                        &hooks.output.storage,
                        case.regions.len(),
                        case.dump,
                    );
                }
                mb_y += 1;
            }
            hooks.teardown(&io);
        }
    }
    print_regions(&mut out, &output.storage, case.regions.len(), true);
    out
}

#[test]
fn custom_io_matches_the_c_output_functions() {
    let Some(oracle) = Oracle::locate("custom_io_matches_the_c_output_functions") else {
        return;
    };
    let probe = oracle.probe("wp6", "io");
    let mut rng = Rng(0x5750_3649_4f00);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut cases = 0;
    for round in 0..120 {
        for mode in 0..13 {
            let case = random_case(&mut rng, mode);
            let _ = round;
            commands.push_str(&case.command());
            expected.push_str(&run_case(&case));
            cases += 1;
        }
    }
    let output = probe.run::<&str>(&[], commands.as_bytes());
    output.success();
    compare("io", &output.stdout, &expected);
    eprintln!("io: {cases} cases");
}
