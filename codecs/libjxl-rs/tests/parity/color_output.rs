// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Whole-image color output: genuine original files decoded through the
// sequence boundary after decode.cc's OutputEncodingInfo operations. Every
// file comes from the original public encoder or original header/ICC writers.
use super::*;
use jxl::jxl::color_encoding::{ColorSpace, JxlColorEncoding};
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::dec_frame::SequenceDecoder;
use jxl::jxl::icc_codec::ICCReader;
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::padded_bytes::PaddedBytes;

pub(super) fn handles(kind: &str) -> bool {
    kind == "co-decode"
}

struct Request {
    bytes: Vec<u8>,
    cms: bool,
    target: u32,
    color: Option<JxlColorEncoding>,
    icc: Option<Vec<u8>>,
}
fn request(args: &mut std::str::SplitWhitespace<'_>) -> Request {
    let bytes = unhex(args.next().unwrap());
    let cms = number::<u8>(args) != 0;
    let target = number(args);
    let (color, icc) = match args.next().unwrap() {
        "-" => (None, None),
        "e" => (Some(cms::color_encoding(args)), None),
        "i" => (None, Some(unhex(args.next().unwrap()))),
        kind => panic!("desired profile kind {kind}"),
    };
    Request {
        bytes,
        cms,
        target,
        color,
        icc,
    }
}

// decode.cc's header sequence: the ICC stream is stored with SetICCRaw.
fn header(reader: &mut BitReader<'_>, m: &mut CodecMetadata) -> Status {
    if reader.read_fixed_bits::<16>() != 0x0aff {
        return Err(StatusCode::GenericError);
    }
    jxl::jxl::headers::read_size_header(reader, &mut m.size)?;
    jxl::jxl::image_metadata::read_image_metadata(reader, &mut m.m)?;
    m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
    jxl::jxl::fields::bundle::read(reader, &mut m.transform_data)?;
    if m.m.color_encoding.want_icc() {
        let mut icc = ICCReader::new();
        let mut decoded = PaddedBytes::default();
        icc.init(reader)?;
        icc.process(reader, &mut decoded)?;
        if decoded.is_empty() {
            return Err(StatusCode::GenericError);
        }
        m.m.color_encoding.set_icc_raw(decoded.as_slice().to_vec());
    }
    reader.jump_to_byte_boundary()?;
    reader.all_reads_within_bounds()
}

// JxlDecoderSetOutputColorProfile's body after its header/post-header checks,
// with no image output buffer set; the same step numbers as the C++ adapter.
fn set_output_color_profile(
    decoder: &mut SequenceDecoder<'_>,
    orig_encoding: &mut ColorEncoding,
    xyb_encoded: bool,
    color_encoding: Option<&JxlColorEncoding>,
    icc_data: Option<&[u8]>,
) -> u32 {
    let mut c_out = ColorEncoding::default();
    let mut same_encoding = false;
    if let Some(color) = color_encoding {
        if c_out.from_external(color).is_err() {
            return 1;
        }
        same_encoding =
            c_out.same_color_encoding(&decoder.state().output_encoding_info.color_encoding);
    }
    let info = &decoder.state().output_encoding_info;
    let cms = info.color_management_system;
    if !info.cms_set && (icc_data.is_some() || (!xyb_encoded && !same_encoding)) {
        return 2;
    }
    if !orig_encoding.have_fields() && info.cms_set {
        let mut tmp_icc = orig_encoding.icc().to_vec();
        // SAFETY: The stored interface is the translated static default CMS.
        if unsafe { orig_encoding.set_icc(&mut tmp_icc, Some(&cms)) }.is_err() {
            return 3;
        }
        decoder.set_orig_color_encoding(orig_encoding.clone());
    }
    if let Some(color) = color_encoding {
        if color.color_space == ColorSpace::UNKNOWN {
            return 4;
        }
        if c_out.icc().is_empty() {
            return 5;
        }
        if decoder.maybe_set_color_encoding(&c_out).is_err() {
            return 6;
        }
        *orig_encoding = decoder.state().output_encoding_info.color_encoding.clone();
        return 0;
    }
    let mut c_dst = ColorEncoding::default();
    let mut padded_icc = icc_data.unwrap().to_vec();
    // SAFETY: The stored interface is the translated static default CMS.
    if unsafe { c_dst.set_icc(&mut padded_icc, Some(&cms)) }.is_err() {
        return 7;
    }
    if decoder.maybe_set_color_encoding(&c_dst).is_err() {
        return 8;
    }
    0
}

fn planes(out: &mut String, image: &ImageBundle<'_>) {
    write!(out, " I {} {}", image.xsize(), image.ysize()).unwrap();
    metadata::color_bundle(out, image.c_current());
    for c in 0..3 {
        for y in 0..image.ysize() {
            for x in 0..image.xsize() {
                write!(out, " {}", image.color().plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
    write!(out, " X {}", image.extra_channels().len()).unwrap();
    for ec in image.extra_channels() {
        write!(out, " {} {}", ec.xsize(), ec.ysize()).unwrap();
        for y in 0..ec.ysize() {
            for x in 0..ec.xsize() {
                write!(out, " {}", ec.row(y)[x].to_bits()).unwrap();
            }
        }
    }
}

pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let r = request(args);
    let mut m = CodecMetadata::default();
    let mut reader = BitReader::new(&r.bytes);
    let parsed = header(&mut reader, &mut m);
    let mut offset = reader.total_bits_consumed() / 8;
    let close = reader.close();
    let mut out = format!(" H {} {}", status_code(&parsed), status_code(&close));
    if parsed.is_err() || close.is_err() {
        return out;
    }
    metadata::metadata(&mut out, &m.m);
    let mut decoder = match SequenceDecoder::new(&m) {
        Ok(v) => {
            out.push_str(" P 0");
            v
        }
        Err(e) => {
            write!(out, " P {}", e as i32).unwrap();
            return out;
        }
    };
    if r.target != 0 {
        decoder.set_desired_intensity_target(f32::from_bits(r.target));
    }
    if r.cms {
        decoder.set_default_cms();
    }
    let mut orig_encoding = m.m.color_encoding.clone();
    if r.color.is_some() || r.icc.is_some() {
        let step = set_output_color_profile(
            &mut decoder,
            &mut orig_encoding,
            m.m.xyb_encoded,
            r.color.as_ref(),
            r.icc.as_deref(),
        );
        write!(out, " C {step}").unwrap();
        if step != 0 {
            return out;
        }
    }
    output_encoding::observation(&mut out, &decoder.state().output_encoding_info);
    let mut preview = m.m.have_preview;
    let mut last = false;
    let mut frames = 0;
    let mut status = Ok(());
    while status.is_ok() && !last && frames < 64 {
        let decoded = decoder.decode_frame(&r.bytes[offset..], preview);
        let t = &decoded.trace;
        let head = t
            .init_status
            .unwrap()
            .and(t.output_status.unwrap_or(Ok(())))
            .and(t.header_bounds.unwrap_or(Ok(())))
            .and(t.header_close.unwrap());
        write!(
            out,
            " F {} {} {}",
            frames,
            u32::from(preview),
            status_code(&head)
        )
        .unwrap();
        frames += 1;
        status = decoded.status;
        if head.is_err() {
            break;
        }
        write!(out, " S {}", status_code(&status)).unwrap();
        if status.is_ok() {
            offset += decoder.output().decoded_bytes();
            last = !preview && decoded.header.as_ref().unwrap().is_last;
        }
        preview = false;
    }
    if status.is_ok() && (!last || offset != r.bytes.len()) {
        status = Err(StatusCode::GenericError);
    }
    write!(out, " E {} {} {}", status_code(&status), offset, frames).unwrap();
    output_encoding::observation(&mut out, &decoder.state().output_encoding_info);
    if status.is_ok() {
        planes(&mut out, &decoder.output());
    }
    out
}

fn bits64(value: f64) -> u64 {
    value.to_bits()
}
// ReadColorEncoding protocol: enum words, then gamma and xy doubles as bits.
#[derive(Clone, Copy)]
struct Enc {
    cs: u32,
    wp: u32,
    p: u32,
    tf: u32,
    ri: u32,
    gamma: f64,
    white: (f64, f64),
    red: (f64, f64),
    green: (f64, f64),
    blue: (f64, f64),
}
const SRGB: Enc = Enc {
    cs: 0,
    wp: 1,
    p: 1,
    tf: TF_SRGB,
    ri: 1,
    gamma: 0.0,
    white: (0.3127, 0.329),
    red: (0.64, 0.33),
    green: (0.3, 0.6),
    blue: (0.15, 0.06),
};
impl Enc {
    fn words(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            self.cs,
            self.wp,
            self.p,
            self.tf,
            self.ri,
            bits64(self.gamma),
            bits64(self.white.0),
            bits64(self.white.1),
            bits64(self.red.0),
            bits64(self.red.1),
            bits64(self.green.0),
            bits64(self.green.1),
            bits64(self.blue.0),
            bits64(self.blue.1)
        )
    }
    fn gray(self) -> Self {
        Self { cs: 1, ..self }
    }
    fn tf(self, tf: u32) -> Self {
        Self { tf, ..self }
    }
    fn gamma(self, gamma: f64) -> Self {
        Self {
            tf: 65535,
            gamma,
            ..self
        }
    }
    fn primaries(self, p: u32) -> Self {
        Self { p, ..self }
    }
    fn white(self, wp: u32) -> Self {
        Self { wp, ..self }
    }
    fn intent(self, ri: u32) -> Self {
        Self { ri, ..self }
    }
}
const TF_709: u32 = 1;
const TF_LINEAR: u32 = 8;
const TF_SRGB: u32 = 13;
const TF_PQ: u32 = 16;
const TF_DCI: u32 = 17;
const TF_HLG: u32 = 18;
const P_CUSTOM: u32 = 2;
const P_2100: u32 = 9;
const P_P3: u32 = 11;
const WP_CUSTOM: u32 = 2;
const WP_E: u32 = 10;
const WP_DCI: u32 = 11;

fn profile(path: &str) -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("upstream/libjxl/third_party/skcms/profiles")
            .join(path),
    )
    .unwrap()
}

#[derive(Clone)]
struct Fixture {
    name: String,
    request: String,
    gray: bool,
    xyb: bool,
    pq: bool,
    hlg: bool,
    own: Option<Enc>,
}
#[allow(clippy::too_many_arguments)]
fn fixture(
    name: &str,
    size: (usize, usize),
    gray: bool,
    alpha: bool,
    bits: u32,
    floating: bool,
    mode: u32,
    target: f32,
    seed: u32,
    color: Result<Enc, Vec<u8>>,
) -> Fixture {
    let (pq, hlg) = match &color {
        Ok(e) => (e.tf == TF_PQ, e.tf == TF_HLG),
        Err(_) => (name.contains("pq"), name.contains("hlg")),
    };
    let profile = match &color {
        Ok(e) => format!("e {}", e.words()),
        Err(icc) => format!("i {}", hex(icc)),
    };
    Fixture {
        name: name.to_owned(),
        request: format!(
            "co-encode {} {} {} {} {bits} {} {mode} {} {} {seed} {profile}",
            size.0,
            size.1,
            u8::from(gray),
            u8::from(alpha),
            u8::from(floating),
            if mode == 0 { 3 } else { 7 },
            target.to_bits()
        ),
        gray,
        xyb: mode == 1,
        pq,
        hlg,
        own: color.ok(),
    }
}

fn fixtures() -> Vec<Fixture> {
    let adobe = Enc {
        p: P_CUSTOM,
        green: (0.21, 0.71),
        ..SRGB.gamma(1.0 / 2.2)
    };
    let prophoto_d50 = Enc {
        wp: WP_CUSTOM,
        white: (0.3457, 0.3585),
        p: P_CUSTOM,
        red: (0.7347, 0.2653),
        green: (0.1596, 0.8404),
        blue: (0.0366, 0.0001),
        ..SRGB.tf(TF_LINEAR)
    };
    let mut items = Vec::new();
    // Enum color spaces: every primaries/white-point/transfer family, both
    // lossless (original samples) and XYB VarDCT, plus non-XYB VarDCT.
    let enums: Vec<(&str, Enc, f32)> = vec![
        ("srgb", SRGB, 255.0),
        ("linear-srgb", SRGB.tf(TF_LINEAR), 255.0),
        ("rec709", SRGB.tf(TF_709), 255.0),
        ("p3-d65", SRGB.primaries(P_P3), 255.0),
        (
            "dci-p3",
            SRGB.primaries(P_P3).white(WP_DCI).tf(TF_DCI),
            255.0,
        ),
        ("adobe-gamma", adobe, 255.0),
        ("prophoto-d50-linear", prophoto_d50, 255.0),
        ("e-white-gamma", SRGB.white(WP_E).gamma(0.5), 255.0),
        ("rec2020-srgb-400", SRGB.primaries(P_2100), 400.0),
        ("perceptual", SRGB.intent(0), 255.0),
        ("saturation", SRGB.primaries(P_P3).intent(2), 255.0),
        ("absolute", SRGB.intent(3), 255.0),
        ("pq-10000", SRGB.primaries(P_2100).tf(TF_PQ), 10000.0),
        ("pq-4000", SRGB.primaries(P_2100).tf(TF_PQ), 4000.0),
        ("pq-p3-1000", SRGB.primaries(P_P3).tf(TF_PQ), 1000.0),
        ("hlg-1000", SRGB.primaries(P_2100).tf(TF_HLG), 1000.0),
        ("hlg-p3-300", SRGB.primaries(P_P3).tf(TF_HLG), 300.0),
    ];
    for (name, color, target) in &enums {
        for (mode, label) in [(0, "lossless"), (1, "xyb"), (2, "vardct")] {
            let bits = if *target > 255.0 && mode == 0 { 12 } else { 8 };
            items.push(fixture(
                &format!("{name}-{label}"),
                (9, 7),
                false,
                false,
                bits,
                false,
                mode,
                *target,
                items.len() as u32,
                Ok(*color),
            ));
        }
    }
    let grays: Vec<(&str, Enc, f32)> = vec![
        ("gray-srgb", SRGB.gray(), 255.0),
        ("gray-linear", SRGB.gray().tf(TF_LINEAR), 255.0),
        ("gray-709", SRGB.gray().tf(TF_709), 255.0),
        ("gray-gamma", SRGB.gray().gamma(1.0 / 1.8), 255.0),
        (
            "gray-dci-white",
            SRGB.gray().white(WP_DCI).tf(TF_DCI),
            255.0,
        ),
        ("gray-e-white", SRGB.gray().white(WP_E), 255.0),
        ("gray-pq", SRGB.gray().tf(TF_PQ), 10000.0),
        ("gray-hlg", SRGB.gray().tf(TF_HLG), 1000.0),
    ];
    for (name, color, target) in &grays {
        for (mode, label) in [(0, "lossless"), (1, "xyb")] {
            items.push(fixture(
                &format!("{name}-{label}"),
                (9, 7),
                true,
                false,
                8,
                false,
                mode,
                *target,
                items.len() as u32,
                Ok(*color),
            ));
        }
    }
    // Alpha, higher precision and float input with multigroup geometry.
    for (name, color, target, gray) in [
        ("alpha-p3", SRGB.primaries(P_P3), 255.0, false),
        ("alpha-pq", SRGB.primaries(P_2100).tf(TF_PQ), 10000.0, false),
        ("alpha-gray-hlg", SRGB.gray().tf(TF_HLG), 1000.0, true),
    ] {
        for mode in [0, 1] {
            items.push(fixture(
                &format!("{name}-{mode}"),
                (9, 7),
                gray,
                true,
                8,
                false,
                mode,
                target,
                items.len() as u32,
                Ok(color),
            ));
        }
    }
    items.push(fixture(
        "pq-16bit-lossless",
        (9, 7),
        false,
        false,
        16,
        false,
        0,
        10000.0,
        91,
        Ok(SRGB.primaries(P_2100).tf(TF_PQ)),
    ));
    items.push(fixture(
        "hlg-float-xyb-multigroup",
        (257, 3),
        false,
        false,
        32,
        true,
        1,
        1000.0,
        92,
        Ok(SRGB.primaries(P_2100).tf(TF_HLG)),
    ));
    items.push(fixture(
        "adobe-float-vardct",
        (9, 7),
        false,
        false,
        32,
        true,
        2,
        255.0,
        93,
        Ok(adobe),
    ));
    // Genuine ICC profiles from the pinned skcms corpus.
    for (name, path, gray, target) in [
        ("icc-adobe", "misc/AdobeRGB.icc", false, 255.0),
        (
            "icc-p3-parametric",
            "mobile/Display_P3_parametric.icc",
            false,
            255.0,
        ),
        ("icc-p3-lut", "mobile/Display_P3_LUT.icc", false, 255.0),
        (
            "icc-srgb-v4",
            "color.org/sRGB_v4_ICC_preference.icc",
            false,
            255.0,
        ),
        ("icc-wide-color", "misc/Apple_Wide_Color.icc", false, 255.0),
        (
            "icc-calibrated",
            "misc/sRGB_Calibrated_Heterogeneous.icc",
            false,
            255.0,
        ),
        ("icc-gray-gamma", "misc/Gray_Gamma_22.icc", true, 255.0),
        ("icc-pq-cicp", "misc/Rec2020_PQ_cicp.icc", false, 10000.0),
        ("icc-hlg-cicp", "misc/Rec2020_HLG_cicp.icc", false, 1000.0),
    ] {
        for (mode, label) in [(0, "lossless"), (1, "xyb"), (2, "vardct")] {
            items.push(fixture(
                &format!("{name}-{label}"),
                (9, 7),
                gray,
                false,
                8,
                false,
                mode,
                target,
                items.len() as u32,
                Err(profile(path)),
            ));
        }
    }
    items
}

struct Encoded {
    fixture: Fixture,
    bytes: Vec<u8>,
    level: u32,
}
fn encode(items: &[Fixture]) -> Vec<Encoded> {
    let requests: Vec<_> = items.iter().map(|f| f.request.clone()).collect();
    let mut encoded = Vec::new();
    for (item, line) in items.iter().zip(oracle(&requests)) {
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(
            words[0], "0",
            "genuine original encoding {}: {line}",
            item.name
        );
        assert_eq!(words[2], "L");
        let bytes = unhex(words[1]);
        assert!(!bytes.is_empty(), "{}", item.name);
        encoded.push(Encoded {
            fixture: item.clone(),
            bytes,
            level: words[3].parse().unwrap(),
        });
    }
    encoded
}

// The original simple pipeline cannot prepare a CMS stage with defined
// behavior (see dec_frame). Every option below selects a configuration whose
// source PreparePipeline adds no CMS stage: CMS-less output, the original
// encoding, gray/color mixing, or a failing setter step before decoding.
fn decode_options(f: &Fixture) -> Vec<String> {
    let srgb = format!("e {}", SRGB.words());
    let mut options = vec!["0 0 -".to_owned(), "1 0 -".to_owned()];
    for desired in [
        SRGB,
        SRGB.tf(TF_LINEAR),
        SRGB.tf(TF_709),
        SRGB.primaries(P_2100).tf(TF_PQ),
        SRGB.primaries(P_P3).tf(TF_HLG),
        SRGB.primaries(P_P3).gamma(1.0 / 2.6),
        SRGB.white(WP_DCI).primaries(P_P3).tf(TF_DCI),
        SRGB.gray(),
        SRGB.gray().tf(TF_LINEAR),
        SRGB.white(WP_E).gray(),
        Enc { cs: 2, ..SRGB },
        Enc { cs: 3, ..SRGB },
    ] {
        options.push(format!("0 0 e {}", desired.words()));
    }
    // Gray/color mixing and the original encoding never add a CMS stage.
    options.push(format!(
        "1 0 e {}",
        if f.gray { SRGB } else { SRGB.gray() }.words()
    ));
    if let Some(own) = f.own {
        options.push(format!("1 0 e {}", own.words()));
    }
    // Setter failures: unknown space, ICC without CMS, malformed ICC.
    options.push(format!("1 0 e {}", Enc { cs: 3, ..SRGB }.words()));
    options.push(format!("0 0 i {}", hex(&profile("misc/AdobeRGB.icc"))));
    options.push(format!("1 0 i {}", hex(&profile("misc/bad_pcs.icc"))));
    options.push(format!("1 0 i {}", hex(&[0x41; 64])));
    let targets: &[f32] = if f.pq || f.hlg {
        &[100.0, 203.0, 255.0, 1000.0, 4000.0]
    } else {
        &[1000.0]
    };
    for target in targets {
        let t = target.to_bits();
        options.push(format!("0 {t} -"));
        options.push(format!("1 {t} -"));
        options.push(format!("0 {t} {srgb}"));
        options.push(format!("0 {t} e {}", SRGB.tf(TF_LINEAR).words()));
        options.push(format!(
            "0 {t} e {}",
            SRGB.primaries(P_2100).tf(TF_PQ).words()
        ));
        options.push(format!(
            "0 {t} e {}",
            SRGB.primaries(P_2100).tf(TF_HLG).words()
        ));
        if let Some(own) = f.own {
            options.push(format!("1 {t} e {}", own.words()));
        }
    }
    options
}

// Interleaved public float output reconstructed from the adapter's planes.
fn interleaved(observation: &str) -> Option<Vec<String>> {
    let words: Vec<_> = observation.split_whitespace().collect();
    let at = words.iter().rposition(|w| *w == "I")?;
    let (w, h): (usize, usize) = (
        words[at + 1].parse().unwrap(),
        words[at + 2].parse().unwrap(),
    );
    let gray = words[at + 4].starts_with("1,");
    let color = &words[at + 6..at + 6 + 3 * w * h];
    assert_eq!(words[at + 6 + 3 * w * h], "X");
    let extra: usize = words[at + 7 + 3 * w * h].parse().unwrap();
    let alpha = if extra != 0 {
        Some(&words[at + 10 + 3 * w * h..at + 10 + 4 * w * h])
    } else {
        None
    };
    let mut out = Vec::new();
    for i in 0..w * h {
        for c in 0..if gray { 1 } else { 3 } {
            out.push(color[c * w * h + i].to_owned());
        }
        if let Some(a) = alpha {
            out.push(a[i].to_owned());
        }
    }
    Some(out)
}

// Optional request capture for independent original-only replay.
fn capture(requests: &[String]) {
    if let Some(path) = std::env::var_os("JXL_COLOR_OUTPUT_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        file.write_all((requests.join("\n") + "\n").as_bytes())
            .unwrap();
    }
}
fn oracle(requests: &[String]) -> Vec<String> {
    capture(requests);
    original_observations(requests)
}

fn compare(requests: &[String]) -> Vec<String> {
    let mut originals = Vec::new();
    for chunk in requests.chunks(16) {
        for (request, original) in chunk.iter().zip(oracle(chunk)) {
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &original) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = original.split_whitespace().collect();
                let first = a.iter().zip(&b).position(|(a, b)| a != b);
                panic!(
                    "C++/Rust differ at word {first:?} ({} vs {} words): {:?} / {:?}\nrequest {}",
                    a.len(),
                    b.len(),
                    first.map(|i| &a[i.saturating_sub(4)..(i + 4).min(a.len())]),
                    first.map(|i| &b[i.saturating_sub(4)..(i + 4).min(b.len())]),
                    &request[..request.len().min(160)]
                );
            }
            originals.push(original);
        }
    }
    originals
}

// Setter step and final status inventory of the original observations.
fn inventory(originals: &[String]) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    for original in originals {
        let words: Vec<_> = original.split_whitespace().collect();
        let key = if let Some(at) = words.iter().position(|w| *w == "E") {
            format!("E{}", words[at + 1])
        } else if let Some(at) = words.iter().position(|w| *w == "C") {
            format!("C{}", words[at + 1])
        } else if let Some(at) = words.iter().position(|w| *w == "P") {
            format!("P{}", words[at + 1])
        } else {
            format!("H{}", words[1])
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

fn end_status(observation: &str) -> Option<&str> {
    let words: Vec<_> = observation.split_whitespace().collect();
    let at = words.iter().position(|w| *w == "E")?;
    Some(words[at + 1])
}

#[test]
fn genuine_color_profiles_decode_to_original_samples_with_cms_and_tone_mapping() {
    let encoded = encode(&fixtures());
    assert_eq!(encoded.len(), 103);
    assert!(encoded.iter().any(|e| e.level == 10));
    let mut requests = Vec::new();
    let mut owners = Vec::new();
    for e in &encoded {
        for option in decode_options(&e.fixture) {
            requests.push(format!("co-decode {} {option}", hex(&e.bytes)));
            owners.push(&e.fixture);
        }
    }
    let originals = compare(&requests);
    let mut succeeded = 0;
    let mut publics = Vec::new();
    let mut expected = Vec::new();
    // Successful families: [tone-mapped HDR, ICC original output, XYB to a
    // non-sRGB enum target, gray, alpha].
    let mut families = [0; 5];
    for ((request, original), f) in requests.iter().zip(&originals).zip(&owners) {
        let options = &request["co-decode ".len()..];
        let words: Vec<_> = options.split_whitespace().collect();
        if end_status(original) == Some("0") {
            let target: u32 = words[2].parse().unwrap();
            families[0] += usize::from((f.pq || f.hlg) && target != 0);
            families[1] += usize::from(f.own.is_none());
            families[2] +=
                usize::from(f.xyb && words[3] == "e" && words[4..9] != ["0", "1", "1", "13", "1"]);
            families[3] += usize::from(f.gray);
            families[4] += usize::from(f.request.split_whitespace().nth(4) == Some("1"));
        }
        if options.split_whitespace().nth(3) == Some("-") {
            assert_eq!(
                end_status(original),
                Some("0"),
                "default decode: {original:.200}"
            );
        }
        if end_status(original) == Some("0") {
            succeeded += 1;
            publics.push(format!("co-public {options}"));
            expected.push(interleaved(original).unwrap());
        }
    }
    assert_eq!(requests.len(), 3571);
    assert_eq!(succeeded, 1510);
    assert_eq!(families, [675, 329, 486, 222, 131]);
    let counts = inventory(&originals);
    let expected_counts = [
        ("C2", 1326),
        ("C4", 141),
        ("C5", 38),
        ("C6", 5),
        ("C7", 206),
        ("E0", 1510),
        ("E1", 345),
    ];
    assert_eq!(
        counts,
        expected_counts
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    );
    // The actual public decoder writes the same float words to its buffer.
    let mut checked = 0;
    for (chunk, wanted) in publics.chunks(16).zip(expected.chunks(16)) {
        for (public, words) in oracle(chunk).iter().zip(wanted) {
            let fields: Vec<_> = public.split_whitespace().collect();
            assert_eq!(fields[0], "4096", "public full image: {public:.200}");
            let at = fields.iter().position(|w| *w == "O").unwrap();
            if fields[at + 1..] == words[..] {
                checked += 1;
            } else {
                panic!("public decoder buffer differs: {public:.300}");
            }
        }
    }
    println!(
        "{} C++/Rust color-output requests matched, {succeeded} complete; {checked} public buffers agree; {:?} {families:?}",
        requests.len(),
        inventory(&originals)
    );
}

#[test]
fn malformed_unsupported_and_truncated_profiles_keep_original_statuses() {
    let items: Vec<_> = fixtures()
        .into_iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "srgb-lossless"
                    | "srgb-xyb"
                    | "gray-srgb-lossless"
                    | "icc-adobe-lossless"
                    | "icc-adobe-xyb"
            )
        })
        .collect();
    let encoded = encode(&items);
    assert_eq!(encoded.len(), 5);
    let mut garbage = vec![0u8; 160];
    for (i, b) in garbage.iter_mut().enumerate() {
        *b = (i * 73 + 11) as u8;
    }
    let mut truncated = profile("misc/AdobeRGB.icc");
    truncated.truncate(200);
    let replacements = [
        profile("misc/bad_pcs.icc"),
        profile("fuzz/truncated_curv_tag.icc"),
        profile("fuzz/zero_g.icc"),
        profile("misc/US_Web_Coated_SWOP_CMYK.icc"),
        profile("misc/Gray_Gamma_22.icc"),
        profile("misc/AdobeRGB.icc"),
        profile("misc/Rec2020_PQ_cicp.icc"),
        truncated,
        garbage,
    ];
    let mut splices = Vec::new();
    for e in &encoded {
        for icc in &replacements {
            splices.push(format!("co-splice {} {}", hex(&e.bytes), hex(icc)));
        }
    }
    let spliced = oracle(&splices);
    let mut files = Vec::new();
    for line in &spliced {
        let words: Vec<_> = line.split_whitespace().collect();
        if words[0] == "0" {
            files.push(unhex(words[1]));
        }
    }
    assert!(
        files.len() >= 36,
        "original ICC writer accepted {}",
        files.len()
    );
    // Splicing an ICC file's own profile reproduces the genuine bytes.
    let own = oracle(&[format!(
        "co-splice {} {}",
        hex(&encoded[3].bytes),
        hex(&profile("misc/AdobeRGB.icc"))
    )]);
    assert_eq!(own[0], format!("0 {}", hex(&encoded[3].bytes)));
    let mut requests = Vec::new();
    let srgb = format!("e {}", SRGB.words());
    // With a CMS, SetICC parses the spliced original profile before the
    // unknown output space fails, so no spliced profile reaches a CMS stage.
    let unknown = format!("e {}", Enc { cs: 3, ..SRGB }.words());
    for bytes in &files {
        for option in [
            "0 0 -".to_owned(),
            "1 0 -".to_owned(),
            format!("1 0 {unknown}"),
            format!("0 0 {srgb}"),
        ] {
            requests.push(format!("co-decode {} {option}", hex(bytes)));
        }
    }
    // Truncated and corrupted genuine ICC/frame bytes.
    for e in &encoded {
        let n = e.bytes.len();
        for cut in [2, 5, 12, 24, n / 3, n / 2, n - 1] {
            requests.push(format!("co-decode {} 0 0 {srgb}", hex(&e.bytes[..cut])));
            requests.push(format!("co-decode {} 1 0 -", hex(&e.bytes[..cut])));
        }
        for index in [3, 6, 10, 16, 30, n / 2, n - 2] {
            let mut broken = e.bytes.clone();
            broken[index] ^= 0x41;
            requests.push(format!("co-decode {} 0 0 {srgb}", hex(&broken)));
            requests.push(format!("co-decode {} 0 0 -", hex(&broken)));
        }
    }
    let originals = compare(&requests);
    let failures = originals
        .iter()
        .filter(|o| end_status(o) != Some("0"))
        .count();
    assert_eq!(requests.len(), 320);
    assert!(failures > requests.len() / 3);
    let counts = inventory(&originals);
    for step in ["C2", "C3", "C4", "H1", "E0", "E1"] {
        assert!(
            counts.contains_key(step),
            "missing source outcome {step}: {counts:?}"
        );
    }
    println!(
        "{} C++/Rust malformed-profile requests matched, {failures} failing; {:?}",
        requests.len(),
        inventory(&originals)
    );
}

// The CMS-stage configurations stop at the documented admission checkpoint;
// the actual public decoder, with its low-memory pipeline, decodes them.
#[test]
fn cms_stage_configurations_stop_before_source_undefined_simple_pipeline_preparation() {
    let items: Vec<_> = fixtures()
        .into_iter()
        .filter(|f| {
            matches!(
                f.name.as_str(),
                "p3-d65-xyb" | "adobe-gamma-lossless" | "icc-adobe-lossless" | "pq-10000-xyb"
            )
        })
        .collect();
    let encoded = encode(&items);
    assert_eq!(encoded.len(), 4);
    let mut requests = Vec::new();
    for e in &encoded {
        for desired in [SRGB, SRGB.primaries(P_2100).tf(TF_HLG)] {
            requests.push(format!("{} 1 0 e {}", hex(&e.bytes), desired.words()));
        }
        // A different ICC target than the file's own profile.
        let target = if e.fixture.name.contains("adobe") {
            "mobile/Display_P3_parametric.icc"
        } else {
            "misc/AdobeRGB.icc"
        };
        requests.push(format!("{} 1 0 i {}", hex(&e.bytes), hex(&profile(target))));
    }
    let publics: Vec<_> = requests.iter().map(|r| format!("co-public {r}")).collect();
    for (request, public) in requests.iter().zip(oracle(&publics)) {
        assert!(
            public.starts_with("4096 "),
            "original public decoder: {public:.200}"
        );
        let rust = super::observe(&format!("co-decode {request}"));
        let tail = &rust[rust.find(" F ").unwrap_or(rust.len() - 300)..];
        assert!(
            tail.starts_with(" F 0 0 0 S 2 E 2 "),
            "Rust admission: {}",
            &tail[..tail.len().min(300)]
        );
    }
    println!(
        "{} CMS-stage configurations stop at admission",
        requests.len()
    );
}
