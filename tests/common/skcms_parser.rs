// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::skcms::curve::{Curve, ParametricCurve, TableCurve};
use jxl::skcms::profile::{self as p, A2B, B2A, Cicp, IccProfile, IccTag};
use jxl::skcms::transfer_function::TransferFunction;
use std::fmt::Write;
use std::mem::{align_of, offset_of, size_of};

fn number<T: std::str::FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T {
    args.next()
        .expect("protocol argument")
        .parse()
        .ok()
        .expect("numeric argument")
}
fn unhex(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    text.as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
pub fn hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "-".into();
    }
    let mut text = String::with_capacity(2 * data.len());
    for byte in data {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}
fn pointer(out: &mut String, pointer: *const u8, base: *const u8) {
    let offset = if pointer.is_null() {
        u64::MAX
    } else {
        (pointer as usize).wrapping_sub(base as usize) as u64
    };
    write!(out, " {offset}").unwrap();
}
fn function(out: &mut String, function: &TransferFunction) {
    for x in [
        function.g, function.a, function.b, function.c, function.d, function.e, function.f,
    ] {
        write!(out, " {}", x.to_bits()).unwrap();
    }
}
fn curve(out: &mut String, curve: &Curve, base: *const u8) {
    // SAFETY: every observer curve is initialized by source-equivalent memset,
    // static construction or an explicit active-branch fixture. Count selects
    // exactly the initialized source branch; no padding bytes are read.
    unsafe {
        let count = curve.table.table_entries;
        write!(out, " {count}").unwrap();
        if count == 0 {
            function(out, &curve.parametric.function);
        } else {
            pointer(out, curve.table.table_8, base);
            pointer(out, curve.table.table_16, base);
            // SAFETY: fixtures initialize every union byte before source-style
            // field stores. Pointer representations are omitted and normalized
            // separately; every retained non-pointer word stays initialized.
            for offset in (4..size_of::<Curve>()).step_by(4) {
                if (offset_of!(TableCurve, table_8)
                    ..offset_of!(TableCurve, table_8) + size_of::<*const u8>())
                    .contains(&offset)
                    || (offset_of!(TableCurve, table_16)
                        ..offset_of!(TableCurve, table_16) + size_of::<*const u8>())
                        .contains(&offset)
                {
                    continue;
                }
                let word = (curve as *const Curve)
                    .cast::<u8>()
                    .add(offset)
                    .cast::<u32>()
                    .read_unaligned();
                write!(out, " {word}").unwrap();
            }
        }
    }
}
fn curves(out: &mut String, curves: &[Curve], base: *const u8) {
    for c in curves {
        curve(out, c, base);
    }
}
fn matrix<const N: usize>(out: &mut String, matrix: &[[f32; N]; 3]) {
    for row in matrix {
        for x in row {
            write!(out, " {}", x.to_bits()).unwrap();
        }
    }
}
fn profile(out: &mut String, p: &IccProfile, base: *const u8) {
    write!(
        out,
        "{} {} {} {} {}",
        u8::from(p.has_trc),
        u8::from(p.has_to_xyz_d50),
        u8::from(p.has_a2b),
        u8::from(p.has_b2a),
        u8::from(p.has_cicp)
    )
    .unwrap();
    pointer(out, p.buffer, base);
    write!(
        out,
        " {} {} {} {}",
        p.size, p.data_color_space, p.pcs, p.tag_count
    )
    .unwrap();
    curves(out, &p.trc, base);
    matrix(out, &p.to_xyz_d50.vals);
    let a = &p.a2b;
    curves(out, &a.input_curves, base);
    pointer(out, a.grid_8, base);
    pointer(out, a.grid_16, base);
    write!(out, " {}", a.input_channels).unwrap();
    for x in a.grid_points {
        write!(out, " {x}").unwrap();
    }
    curves(out, &a.matrix_curves, base);
    matrix(out, &a.matrix.vals);
    write!(out, " {} {}", a.matrix_channels, a.output_channels).unwrap();
    curves(out, &a.output_curves, base);
    let b = &p.b2a;
    curves(out, &b.input_curves, base);
    write!(out, " {} {}", b.input_channels, b.matrix_channels).unwrap();
    curves(out, &b.matrix_curves, base);
    matrix(out, &b.matrix.vals);
    curves(out, &b.output_curves, base);
    pointer(out, b.grid_8, base);
    pointer(out, b.grid_16, base);
    for x in b.grid_points {
        write!(out, " {x}").unwrap();
    }
    write!(
        out,
        " {} {} {} {} {}",
        b.output_channels,
        p.cicp.color_primaries,
        p.cicp.transfer_characteristics,
        p.cicp.matrix_coefficients,
        p.cicp.video_full_range_flag
    )
    .unwrap();
}
fn tag(out: &mut String, tag: &IccTag, base: *const u8) {
    write!(out, " {} {} {}", tag.signature, tag.kind, tag.size).unwrap();
    pointer(out, tag.buffer, base);
}
pub fn observe(request: &str) -> String {
    let mut args = request.split_whitespace();
    let command = args.next().unwrap();
    if command.starts_with("skcms-curve-") {
        return observe_curve(command, &mut args);
    }
    let mut out = String::new();
    if command == "skcms-profile-layout" {
        let numbers = [
            size_of::<Curve>(),
            align_of::<Curve>(),
            offset_of!(ParametricCurve, function),
            offset_of!(TableCurve, table_8),
            offset_of!(TableCurve, table_16),
            size_of::<A2B>(),
            align_of::<A2B>(),
            size_of::<B2A>(),
            align_of::<B2A>(),
            size_of::<Cicp>(),
            size_of::<IccProfile>(),
            align_of::<IccProfile>(),
            offset_of!(IccProfile, trc),
            offset_of!(IccProfile, to_xyz_d50),
            offset_of!(IccProfile, a2b),
            offset_of!(IccProfile, b2a),
            offset_of!(IccProfile, cicp),
            offset_of!(IccProfile, has_trc),
            size_of::<IccTag>(),
            align_of::<IccTag>(),
            offset_of!(IccTag, buffer),
        ];
        for (i, n) in numbers.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            write!(out, "{n}").unwrap();
        }
        return out;
    }
    if command == "skcms-profile-static" {
        let which: u32 = number(&mut args);
        let p = if which != 0 {
            p::xyz_d50_profile()
        } else {
            p::srgb_profile()
        };
        // SAFETY: these accessors return fully initialized immutable statics.
        unsafe {
            profile(&mut out, &*p, std::ptr::null());
            write!(
                out,
                " {} {}",
                u8::from(
                    p == if which != 0 {
                        p::xyz_d50_profile()
                    } else {
                        p::srgb_profile()
                    }
                ),
                u8::from(
                    p::srgb_transfer_function()
                        == std::ptr::addr_of!((*p::srgb_profile()).trc[0].parametric.function)
                )
            )
            .unwrap();
            function(&mut out, &*p::srgb_transfer_function());
            function(&mut out, &*p::srgb_inverse_transfer_function());
            function(&mut out, &*p::identity_transfer_function());
        }
        return out;
    }
    if command == "skcms-profile-short-zero" {
        let len: usize = number(&mut args);
        assert!(len < 132);
        let mut value = IccProfile::zeroed();
        // SAFETY: source len guard returns before touching the null input and
        // initializes every output representation byte with its exact memset.
        let ok = unsafe { p::parse(std::ptr::null(), len, &raw mut value) };
        write!(out, "{}", u8::from(ok)).unwrap();
        // SAFETY: the reached source-equivalent memset initialized every byte,
        // including explicit MaybeUninit padding and inactive union storage.
        let bytes = unsafe {
            std::slice::from_raw_parts((&raw const value).cast::<u8>(), size_of::<IccProfile>())
        };
        for byte in bytes {
            write!(out, " {byte}").unwrap();
        }
        return out;
    }
    if command == "skcms-profile-setters" {
        let source: i32 = number(&mut args);
        let self_matrix: u32 = number(&mut args);
        let mut value = IccProfile::zeroed();
        // SAFETY: complete initialized output; only actual source TF field
        // pointers/self matrix aliases are passed, with no exclusive borrows.
        unsafe {
            p::init(&raw mut value);
            for i in 0..3 {
                std::ptr::addr_of_mut!(value.trc[i].parametric.function).write(TransferFunction {
                    g: (i + 1) as f32,
                    a: (i + 2) as f32,
                    b: (i + 3) as f32,
                    c: (i + 4) as f32,
                    d: (i + 5) as f32,
                    e: (i + 6) as f32,
                    f: (i + 7) as f32,
                });
            }
            let external = TransferFunction {
                g: 2.4,
                a: 0.8,
                b: 0.1,
                c: 0.2,
                d: 0.3,
                e: 0.4,
                f: 0.5,
            };
            let tf = if source < 0 {
                &raw const external
            } else {
                std::ptr::addr_of!(value.trc[source as usize].parametric.function)
            };
            p::set_transfer_function(&raw mut value, tf);
            let m = jxl::skcms::types::Matrix3x3 {
                vals: [
                    [0.125, 0.25, 0.375],
                    [0.5, 0.625, 0.75],
                    [0.875, 1.0, 1.125],
                ],
            };
            let matrix = if self_matrix != 0 {
                &raw const value.to_xyz_d50
            } else {
                &raw const m
            };
            p::set_xyz_d50(&raw mut value, matrix);
        }
        profile(&mut out, &value, std::ptr::null());
        return out;
    }
    if command == "skcms-profile-null-getters" {
        let flags: u32 = number(&mut args);
        let mut value = IccProfile::zeroed();
        let backing = [0u8; 132];
        // SAFETY: init writes the full profile. The optional physical buffer
        // has zero tags, so every getter guard/lookup returns before reading it.
        unsafe {
            p::init(&raw mut value);
        }
        if flags & 4 != 0 {
            value.buffer = backing.as_ptr();
        }
        let pp = if flags & 1 != 0 {
            std::ptr::null()
        } else {
            &raw const value
        };
        let mut t = IccTag::zeroed();
        t.signature = 0x12345678;
        t.kind = 0xabcdef01;
        t.size = 0x87654321;
        let tt = if flags & 2 != 0 {
            std::ptr::null_mut()
        } else {
            &raw mut t
        };
        // SAFETY: the actual source null/count guards permit these pointers;
        // every non-null reached output is valid and initialized.
        unsafe {
            p::get_tag_by_index(pp, u32::MAX, tt);
            tag(&mut out, &t, backing.as_ptr());
            write!(
                out,
                " {}",
                u8::from(p::get_tag_by_signature(pp, 0x12345678, tt))
            )
            .unwrap();
            tag(&mut out, &t, backing.as_ptr());
            let mut m = jxl::skcms::types::Matrix3x3 {
                vals: [[0.0; 3]; 3],
            };
            let mut xyz = [0.25_f32, 0.5, 0.75];
            write!(out, " {}", u8::from(p::get_chad(pp, &raw mut m))).unwrap();
            matrix(&mut out, &m.vals);
            write!(out, " {}", u8::from(p::get_wtpt(pp, xyz.as_mut_ptr()))).unwrap();
            for x in xyz {
                write!(out, " {}", x.to_bits()).unwrap();
            }
        }
        return out;
    }
    let mode: u32 = number(&mut args);
    let count: i32 = number(&mut args);
    let priority: Vec<i32> = (0..count).map(|_| number(&mut args)).collect();
    let data = unhex(args.next().unwrap());
    let mut value = IccProfile::zeroed();
    // SAFETY: source size checks protect all reads of the exact live ICC slice;
    // output storage and priority elements remain initialized and disjoint.
    let ok = unsafe {
        if mode == 0 {
            p::parse(data.as_ptr(), data.len(), &raw mut value)
        } else {
            p::parse_with_a2b_priority(
                data.as_ptr(),
                data.len(),
                priority.as_ptr(),
                count,
                if mode == 2 {
                    std::ptr::null_mut()
                } else {
                    &raw mut value
                },
            )
        }
    };
    write!(out, "{}", u8::from(ok)).unwrap();
    if mode == 2 {
        return out;
    }
    out.push(' ');
    profile(&mut out, &value, data.as_ptr());
    if command == "skcms-profile-parse" {
        return out;
    }
    if command == "skcms-profile-getters" {
        assert!(ok, "getter fixture did not parse");
        let index = number(&mut args);
        let signature = number(&mut args);
        let mut t = IccTag::zeroed();
        t.signature = 0x12345678;
        t.kind = 0xabcdef01;
        t.size = 0x87654321;
        // SAFETY: requests use successful profiles and physically backed index
        // slots (including explicit source idx==count boundary fixtures).
        unsafe {
            p::get_tag_by_index(&raw const value, index, &raw mut t);
            tag(&mut out, &t, data.as_ptr());
            write!(
                out,
                " {}",
                u8::from(p::get_tag_by_signature(
                    &raw const value,
                    signature,
                    &raw mut t
                ))
            )
            .unwrap();
            tag(&mut out, &t, data.as_ptr());
            let mut m = jxl::skcms::types::Matrix3x3 {
                vals: [[0.25; 3]; 3],
            };
            write!(
                out,
                " {}",
                u8::from(p::get_chad(&raw const value, &raw mut m))
            )
            .unwrap();
            matrix(&mut out, &m.vals);
            let mut xyz = [0.25, 0.5, 0.75];
            write!(
                out,
                " {}",
                u8::from(p::get_wtpt(&raw const value, xyz.as_mut_ptr()))
            )
            .unwrap();
            for x in xyz {
                write!(out, " {}", x.to_bits()).unwrap();
            }
            write!(out, " {}", p::get_input_channel_count(&raw const value)).unwrap();
        }
        return out;
    }
    panic!("unknown skcms parser command: {command}")
}

pub fn original_profiles() -> Vec<Vec<u8>> {
    fn visit(path: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                visit(&p, files);
            } else if p.extension().is_some_and(|x| x == "icc" || x == "icm") {
                files.push(p);
            }
        }
    }
    let mut files = Vec::new();
    visit(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("upstream/libjxl/third_party/skcms/profiles"),
        &mut files,
    );
    files.sort();
    assert_eq!(files.len(), 88, "pinned original ICC corpus inventory");
    files
        .into_iter()
        .map(|p| std::fs::read(p).unwrap())
        .collect()
}
pub fn genuine_requests() -> Vec<String> {
    original_profiles()
        .iter()
        .map(|bytes| format!("skcms-profile-parse 0 0 {}", hex(bytes)))
        .collect()
}

fn put32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}
fn put16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
}
fn signature(text: &str) -> u32 {
    u32::from_be_bytes(text.as_bytes().try_into().unwrap())
}
fn para(kind: u16, words: &[i32]) -> Vec<u8> {
    let mut bytes = vec![0; 12 + 4 * words.len()];
    put32(&mut bytes, 0, signature("para"));
    put16(&mut bytes, 8, kind);
    for (i, x) in words.iter().enumerate() {
        put32(&mut bytes, 12 + 4 * i, *x as u32);
    }
    bytes
}
fn curv(values: &[u16]) -> Vec<u8> {
    let mut bytes = vec![0; 12 + 2 * values.len()];
    put32(&mut bytes, 0, signature("curv"));
    put32(&mut bytes, 8, values.len() as u32);
    for (i, x) in values.iter().enumerate() {
        put16(&mut bytes, 12 + 2 * i, *x);
    }
    bytes
}
fn xyz(values: [i32; 3]) -> Vec<u8> {
    let mut bytes = vec![0; 20];
    put32(&mut bytes, 0, signature("XYZ "));
    for (i, x) in values.iter().enumerate() {
        put32(&mut bytes, 8 + 4 * i, *x as u32);
    }
    bytes
}
fn icc(tags: &[(u32, Vec<u8>)], space: u32, pcs: u32) -> Vec<u8> {
    let mut bytes = vec![0; 132 + 12 * tags.len()];
    put32(&mut bytes, 8, 4 << 24);
    put32(&mut bytes, 16, space);
    put32(&mut bytes, 20, pcs);
    put32(&mut bytes, 36, signature("acsp"));
    for (i, value) in [63190u32, 65536, 54061].iter().enumerate() {
        put32(&mut bytes, 68 + 4 * i, *value);
    }
    put32(&mut bytes, 128, tags.len() as u32);
    for (i, (sig, payload)) in tags.iter().enumerate() {
        let offset = bytes.len();
        bytes.extend_from_slice(payload);
        while bytes.len() % 4 != 0 {
            bytes.push(0);
        }
        put32(&mut bytes, 132 + 12 * i, *sig);
        put32(&mut bytes, 136 + 12 * i, offset as u32);
        put32(&mut bytes, 140 + 12 * i, payload.len() as u32);
    }
    let len = bytes.len();
    put32(&mut bytes, 0, len as u32);
    bytes
}
fn rgb_tags(curve: Vec<u8>) -> Vec<(u32, Vec<u8>)> {
    vec![
        (signature("rTRC"), curve.clone()),
        (signature("gTRC"), curve.clone()),
        (signature("bTRC"), curve),
        (signature("rXYZ"), xyz([65536, 0, 0])),
        (signature("gXYZ"), xyz([0, 65536, 0])),
        (signature("bXYZ"), xyz([0, 0, 65536])),
    ]
}
fn mft(width: u32, inputs: u8, outputs: u8, points: u8, entries: u16, identity: bool) -> Vec<u8> {
    let fixed = if width == 1 { 48 } else { 52 };
    let mut bytes = vec![0; fixed];
    put32(
        &mut bytes,
        0,
        signature(if width == 1 { "mft1" } else { "mft2" }),
    );
    bytes[8] = inputs;
    bytes[9] = outputs;
    bytes[10] = points;
    if width == 2 {
        put16(&mut bytes, 48, entries);
        put16(&mut bytes, 50, entries);
    }
    if !(1..=4).contains(&inputs) || outputs > 4 || points < 2 || !(2..=4096).contains(&entries) {
        return bytes;
    }
    let entries = if width == 1 { 256 } else { entries as usize };
    let mut table = Vec::new();
    for i in 0..entries {
        let mut value = (i * 65535 / (entries - 1)) as u16;
        if !identity && i > 0 {
            value = value.saturating_sub(1);
        }
        if width == 1 {
            table.push((value >> 8) as u8);
        } else {
            table.extend_from_slice(&value.to_be_bytes());
        }
    }
    for _ in 0..inputs {
        bytes.extend_from_slice(&table);
    }
    bytes.resize(
        bytes.len() + outputs as usize * width as usize * (points as usize).pow(inputs as u32),
        0x35,
    );
    for _ in 0..outputs {
        bytes.extend_from_slice(&table);
    }
    bytes
}
#[expect(
    clippy::too_many_arguments,
    reason = "Fixture axes match the independent mAB/mBA source fields."
)]
fn mab(
    reverse: bool,
    inputs: u8,
    outputs: u8,
    m_curve: bool,
    matrix: bool,
    a_curve: bool,
    clut: bool,
    width: u8,
) -> Vec<u8> {
    let mut bytes = vec![0; 32];
    put32(
        &mut bytes,
        0,
        signature(if reverse { "mBA " } else { "mAB " }),
    );
    bytes[8] = inputs;
    bytes[9] = outputs;
    let b_count = if reverse { inputs } else { outputs };
    let a_count = if reverse { outputs } else { inputs };
    let offset = bytes.len();
    put32(&mut bytes, 12, offset as u32);
    for _ in 0..b_count {
        bytes.extend_from_slice(&para(0, &[65536]));
    }
    if m_curve {
        let offset = bytes.len();
        put32(&mut bytes, 20, offset as u32);
        for _ in 0..b_count {
            bytes.extend_from_slice(&curv(&[0, 65535]));
        }
    }
    if matrix {
        let offset = bytes.len();
        put32(&mut bytes, 16, offset as u32);
        for x in [65536, 2, -3, 4, 65536, 6, 7, 8, 65536, 9, -10, 11] {
            bytes.extend_from_slice(&i32::to_be_bytes(x));
        }
    }
    if a_curve {
        let offset = bytes.len();
        put32(&mut bytes, 28, offset as u32);
        for _ in 0..a_count {
            bytes.extend_from_slice(&para(0, &[65536]));
        }
    }
    if clut {
        let offset = bytes.len();
        put32(&mut bytes, 24, offset as u32);
        bytes.extend_from_slice(&[2; 16]);
        bytes.extend_from_slice(&[width, 0, 0, 0]);
        if inputs <= 4 {
            bytes.resize(
                bytes.len() + outputs as usize * width as usize * 2usize.pow(inputs as u32),
                0x53,
            );
        }
    }
    bytes
}
pub fn synthetic_profiles() -> Vec<Vec<u8>> {
    let mut all = Vec::new();
    for curve in [
        para(0, &[65536]),
        para(1, &[144179, 65536, 0]),
        para(2, &[144179, 65536, 0, 1]),
        para(3, &[144179, 65536, 0, 4096, 2048]),
        para(4, &[144179, 65536, 0, 4096, 2048, 3, 4]),
        curv(&[]),
        curv(&[563]),
        curv(&[0, 65535]),
        curv(&[0, 100, 32000, 65000]),
        para(1, &[65536, 0, 1]),
        para(2, &[65536, 0, 1, 4]),
        para(5, &[65536]),
    ] {
        all.push(icc(
            &rgb_tags(curve.clone()),
            p::SIGNATURE_RGB,
            p::SIGNATURE_XYZ,
        ));
        for pcs in [p::SIGNATURE_XYZ, p::SIGNATURE_LAB] {
            all.push(icc(
                &[(signature("kTRC"), curve.clone())],
                p::SIGNATURE_GRAY,
                pcs,
            ));
        }
    }
    for width in [1, 2] {
        for inputs in 0..=5 {
            for outputs in 0..=5 {
                for identity in [false, true] {
                    let tag = mft(
                        width,
                        inputs,
                        outputs,
                        2,
                        if width == 1 { 256 } else { 2 },
                        identity,
                    );
                    for sig in ["A2B0", "B2A0"] {
                        all.push(icc(
                            &[(signature(sig), tag.clone())],
                            p::SIGNATURE_RGB,
                            p::SIGNATURE_XYZ,
                        ));
                    }
                }
            }
        }
    }
    for entries in [0, 1, 2, 3, 4096, 4097, u16::MAX] {
        for points in [0, 1, 2, 3] {
            all.push(icc(
                &[(signature("A2B0"), mft(2, 3, 3, points, entries, false))],
                p::SIGNATURE_RGB,
                p::SIGNATURE_XYZ,
            ));
        }
    }
    for reverse in [false, true] {
        for inputs in 0..=5 {
            for outputs in 0..=5 {
                for m in [false, true] {
                    for matrix in [false, true] {
                        for a in [false, true] {
                            for clut in [false, true] {
                                for width in [0, 1, 2, 3] {
                                    let tag =
                                        mab(reverse, inputs, outputs, m, matrix, a, clut, width);
                                    all.push(icc(
                                        &[(signature(if reverse { "B2A0" } else { "A2B0" }), tag)],
                                        p::SIGNATURE_RGB,
                                        p::SIGNATURE_XYZ,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for space in [
        p::SIGNATURE_RGB,
        p::SIGNATURE_GRAY,
        p::SIGNATURE_CMYK,
        p::SIGNATURE_LAB,
        0,
        0x46434c52,
    ] {
        for pcs in [p::SIGNATURE_XYZ, p::SIGNATURE_LAB, p::SIGNATURE_RGB, 0] {
            all.push(icc(&rgb_tags(curv(&[])), space, pcs));
        }
    }
    let mut tags = rgb_tags(curv(&[]));
    for size in 0..=16 {
        let mut cicp = vec![0; size];
        if size >= 4 {
            put32(&mut cicp, 0, signature("cicp"));
        }
        if size >= 12 {
            cicp[8..12].copy_from_slice(&[9, 16, 0, 1]);
        }
        tags.push((signature("cicp"), cicp));
        all.push(icc(&tags, p::SIGNATURE_RGB, p::SIGNATURE_XYZ));
        tags.pop();
    }
    all
}
pub fn synthetic_requests() -> Vec<String> {
    synthetic_profiles()
        .iter()
        .map(|data| format!("skcms-profile-parse 0 0 {}", hex(data)))
        .collect()
}
pub fn malformed_and_truncated_requests() -> Vec<String> {
    let mut cases = Vec::new();
    let representative = icc(
        &rgb_tags(para(4, &[144179, 65536, 0, 4096, 2048, 3, 4])),
        p::SIGNATURE_RGB,
        p::SIGNATURE_XYZ,
    );
    for len in 0..=representative.len() {
        cases.push(format!(
            "skcms-profile-parse 0 0 {}",
            hex(&representative[..len])
        ));
    }
    for offset in [
        0, 8, 16, 20, 36, 68, 72, 76, 128, 132, 136, 140, 148, 152, 160, 164,
    ] {
        for value in [
            0,
            1,
            3,
            4,
            5,
            12,
            131,
            132,
            representative.len() as u32,
            u32::MAX,
        ] {
            let mut data = representative.clone();
            put32(&mut data, offset, value);
            cases.push(format!("skcms-profile-parse 0 0 {}", hex(&data)));
        }
    }
    for (index, profile) in synthetic_profiles().iter().enumerate() {
        // Every generated structure retains all its fixed-header/table/curve
        // checkpoints. Bytewise prefixes are additional coverage for small
        // representative LUTs, rather than a replacement for the full census.
        let mut cuts = vec![
            0,
            1,
            3,
            4,
            7,
            8,
            11,
            12,
            31,
            32,
            47,
            48,
            51,
            52,
            127,
            128,
            131,
            132,
            143,
            144,
            profile.len() - 1,
        ];
        if index % 97 == 0 && profile.len() < 2048 {
            cuts.extend(0..profile.len());
        }
        if profile.len() >= 144 {
            let offset = u32::from_be_bytes(profile[136..140].try_into().unwrap()) as usize;
            for boundary in [4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52] {
                if offset + boundary < profile.len() {
                    cuts.push(offset + boundary);
                }
            }
        }
        cuts.sort_unstable();
        cuts.dedup();
        for len in cuts {
            if len <= profile.len() {
                cases.push(format!("skcms-profile-parse 0 0 {}", hex(&profile[..len])));
            }
        }
    }
    cases
}

fn observe_curve(command: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::skcms::curve::{canonicalize_identity, eval_curve, fit_linear};
    let mut out = String::new();
    if command == "skcms-curve-source-large" {
        let operation: u32 = number(args);
        let count = number(args);
        let x = f32::from_bits(number(args));
        let table = [0x3fu8, 0x80];
        let mut c = Curve::zeroed();
        // SAFETY: defined requests reach only table index zero, or return at
        // canonicalize's count guard; source-only UB probes never enter Rust.
        unsafe {
            std::ptr::addr_of_mut!(c.table.table_entries).write(count);
            std::ptr::addr_of_mut!(c.table.table_8).write(if operation == 2 {
                std::ptr::null()
            } else {
                table.as_ptr()
            });
            std::ptr::addr_of_mut!(c.table.table_16).write(if operation == 2 {
                table.as_ptr()
            } else {
                std::ptr::null()
            });
            if operation == 0 {
                canonicalize_identity(&raw mut c);
                out.push('C');
                curve(&mut out, &c, table.as_ptr());
            } else {
                write!(out, "{}", eval_curve(&raw const c, x).to_bits()).unwrap();
            }
        }
        return out;
    }
    if command == "skcms-curve-read" {
        let size = number(args);
        let nulls: u32 = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut c = Curve::from_parametric(TransferFunction {
            g: 2.2,
            a: 0.8,
            b: 0.1,
            c: 0.2,
            d: 0.3,
            e: 0.4,
            f: 0.5,
        });
        let mut curve_size = 0x87654321;
        // SAFETY: cases never declare more physically readable bytes than the
        // fixture owns. Explicit null controls exercise original early guards.
        let ok = unsafe {
            p::read_curve(
                if nulls & 1 != 0 {
                    std::ptr::null()
                } else {
                    bytes.as_ptr()
                },
                size,
                if nulls & 2 != 0 {
                    std::ptr::null_mut()
                } else {
                    &raw mut c
                },
                if nulls & 4 != 0 {
                    std::ptr::null_mut()
                } else {
                    &raw mut curve_size
                },
            )
        };
        write!(out, "{} {curve_size}", u8::from(ok)).unwrap();
        curve(&mut out, &c, bytes.as_ptr());
        return out;
    }
    let kind: u32 = number(args);
    let mut c = Curve::zeroed();
    let bytes;
    if kind == 0 {
        let words: [f32; 7] = std::array::from_fn(|_| f32::from_bits(number(args)));
        c = Curve::from_parametric(TransferFunction {
            g: words[0],
            a: words[1],
            b: words[2],
            c: words[3],
            d: words[4],
            e: words[5],
            f: words[6],
        });
        bytes = Vec::new();
    } else {
        bytes = unhex(args.next().unwrap());
        // SAFETY: writable union object; selected table branch is completely
        // initialized, and bytes outlives all reached curve operations.
        unsafe {
            std::ptr::addr_of_mut!(c.table.table_entries)
                .write((bytes.len() / if kind == 1 { 1 } else { 2 }) as u32);
            std::ptr::addr_of_mut!(c.table.table_8).write(if kind == 1 {
                bytes.as_ptr()
            } else {
                std::ptr::null()
            });
            std::ptr::addr_of_mut!(c.table.table_16).write(if kind == 2 {
                bytes.as_ptr()
            } else {
                std::ptr::null()
            });
        }
    }
    // SAFETY: fixtures provide the initialized selected branch, live tables and
    // defined arithmetic/sample bounds. Raw slots preserve the source's allowed
    // aliases without retaining exclusive references across pointer writes.
    unsafe {
        if command == "skcms-curve-eval" {
            let x = f32::from_bits(number(args));
            write!(out, "{}", eval_curve(&raw const c, x).to_bits()).unwrap();
        } else if command == "skcms-curve-linear" {
            let n = number(args);
            let tol = f32::from_bits(number(args));
            let aliases: u32 = number(args);
            let mut slots = [0.125_f32, 0.25, 0.5];
            let base = slots.as_mut_ptr();
            let d = base.add(if aliases & 1 != 0 { 0 } else { 1 });
            let f = if aliases & 2 != 0 {
                std::ptr::null_mut()
            } else {
                base.add(if aliases & 4 != 0 {
                    0
                } else if aliases & 8 != 0 {
                    1
                } else {
                    2
                })
            };
            let points = fit_linear(&raw const c, n, tol, base, d, f);
            write!(out, "{points}").unwrap();
            for x in slots {
                write!(out, " {}", x.to_bits()).unwrap();
            }
        } else if command == "skcms-curve-canonical" {
            canonicalize_identity(&raw mut c);
            out.push('C');
            curve(&mut out, &c, bytes.as_ptr());
        } else {
            panic!("unknown curve command");
        }
    }
    out
}
pub fn curve_requests() -> Vec<String> {
    let mut curves = Vec::new();
    for function in [
        [1.0_f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [
            2.4,
            (1.0_f64 / 1.055) as f32,
            (0.055_f64 / 1.055) as f32,
            (1.0_f64 / 12.92) as f32,
            0.04045,
            0.0,
            0.0,
        ],
        [2.2, 0.8, 0.1, 0.2, 0.3, 0.4, 0.5],
    ] {
        curves.push(format!(
            "0 {}",
            function
                .iter()
                .map(|x| x.to_bits().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    for count in [2, 3, 4, 7, 16, 255, 256, 257, 1024] {
        for width in [1, 2] {
            for pattern in 0..4 {
                let mut table = Vec::new();
                for i in 0..count {
                    let x = match pattern {
                        0 => (i * 65535 / (count - 1)) as u16,
                        1 => (65535 - i * 65535 / (count - 1)) as u16,
                        2 => 32768,
                        _ => ((i * 12345 + 51) % 65536) as u16,
                    };
                    if width == 1 {
                        table.push((x >> 8) as u8);
                    } else {
                        table.extend_from_slice(&x.to_be_bytes());
                    }
                }
                curves.push(format!("{width} {}", hex(&table)));
            }
        }
    }
    let mut requests = Vec::new();
    for c in &curves {
        for x in [
            f32::NEG_INFINITY,
            -2.0,
            -0.0,
            0.0,
            0.00001,
            0.003130805,
            0.04045,
            0.25,
            0.5,
            0.99999,
            1.0,
            2.0,
            f32::INFINITY,
            f32::from_bits(0x7fc12345),
            f32::from_bits(0xffc12345),
        ] {
            requests.push(format!("skcms-curve-eval {c} {}", x.to_bits()));
        }
        requests.push(format!("skcms-curve-canonical {c}"));
        for n in [2, 3, 17, 256] {
            for tol in [0.0_f32, 0.0001, 0.01, 0.25] {
                for aliases in 0..16 {
                    requests.push(format!(
                        "skcms-curve-linear {c} {n} {} {aliases}",
                        tol.to_bits()
                    ));
                }
            }
        }
    }
    requests
}
pub fn read_curve_requests() -> Vec<String> {
    let mut curves = Vec::new();
    for kind in 0..=6 {
        for gamma in [0, 1, 65536, -65536, i32::MAX, i32::MIN] {
            for a in [0, 1, 65536, -65536] {
                curves.push(para(kind, &[gamma, a, -32768, 4096, 2048, 3, -4]));
            }
        }
    }
    for values in [
        vec![],
        vec![0],
        vec![1],
        vec![65535],
        vec![0, 65535],
        vec![0, 100, 32000, 65000],
    ] {
        curves.push(curv(&values));
    }
    let mut requests = Vec::new();
    for curve in curves {
        for size in 0..=curve.len() {
            for nulls in [0, 1, 2, 4, 7] {
                requests.push(format!(
                    "skcms-curve-read {size} {nulls} {}",
                    hex(&curve[..size])
                ));
            }
        }
    }
    requests
}

pub fn priority_requests() -> Vec<String> {
    let mut cases = Vec::new();
    // All public priorities are kept distinct; negative priorities/counts and
    // duplicate entries retain the actual source loop's early-return behavior.
    let orders: &[&[i32]] = &[
        &[],
        &[0],
        &[1],
        &[2],
        &[2, 1, 0],
        &[1, 0, 2],
        &[0, 1, 2],
        &[0, 0, 1],
        &[-1],
        &[3],
        &[0, -1],
        &[2, 3],
    ];
    for data in original_profiles() {
        let text = hex(&data);
        for order in orders {
            cases.push(format!(
                "skcms-profile-parse 1 {} {} {text}",
                order.len(),
                order
                    .iter()
                    .map(i32::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
    }
    let mut tags = rgb_tags(curv(&[]));
    for sig in ["A2B0", "A2B1", "A2B2", "B2A0", "B2A1", "B2A2"] {
        tags.push((signature(sig), mft(2, 3, 3, 2, 2, true)));
    }
    for broken in 0..=6 {
        let mut tags = tags.clone();
        if broken < 6 {
            tags[6 + broken].1 = vec![0; 4];
        }
        let text = hex(&icc(&tags, p::SIGNATURE_RGB, p::SIGNATURE_XYZ));
        cases.push(format!("skcms-profile-parse 0 0 {text}"));
        for order in orders {
            cases.push(format!(
                "skcms-profile-parse 1 {} {} {text}",
                order.len(),
                order
                    .iter()
                    .map(i32::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        cases.push(format!("skcms-profile-parse 1 -1 {text}"));
        cases.push(format!("skcms-profile-parse 2 0 {text}"));
    }
    cases
}
pub fn getter_requests() -> Vec<String> {
    let mut cases = Vec::new();
    let base = rgb_tags(curv(&[]));
    for chad_mode in 0..5 {
        for white_mode in 0..5 {
            let mut tags = base.clone();
            if chad_mode != 0 {
                let mut chad = vec![0; 44];
                put32(&mut chad, 0, signature("sf32"));
                for i in 0..9 {
                    put32(&mut chad, 8 + 4 * i, (i as i32 * 12345 - 32768) as u32);
                }
                if chad_mode == 2 {
                    chad.truncate(43);
                }
                if chad_mode == 3 {
                    put32(&mut chad, 0, signature("XYZ "));
                }
                tags.push((signature("chad"), chad));
                if chad_mode == 4 {
                    tags.push((signature("chad"), vec![0; 4]));
                }
            }
            if white_mode != 0 {
                let mut white = xyz([63190, 65536, 54061]);
                if white_mode == 2 {
                    white.truncate(19);
                }
                if white_mode == 3 {
                    put32(&mut white, 0, signature("sf32"));
                }
                tags.push((signature("wtpt"), white));
                if white_mode == 4 {
                    tags.push((signature("wtpt"), vec![0; 4]));
                }
            }
            let data = icc(&tags, p::SIGNATURE_RGB, p::SIGNATURE_XYZ);
            let text = hex(&data);
            for index in 0..=tags.len() + 1 {
                for sig in [
                    0,
                    signature("rTRC"),
                    signature("bXYZ"),
                    signature("chad"),
                    signature("wtpt"),
                    u32::MAX,
                ] {
                    // At index==count the first curv payload physically supplies the
                    // source's extra tag slot with offset 0. This preserves its literal
                    // > guard while avoiding invented semantics for unbacked slots.
                    cases.push(format!("skcms-profile-getters 0 0 {text} {index} {sig}"));
                }
            }
        }
    }
    for space in [
        p::SIGNATURE_RGB,
        p::SIGNATURE_GRAY,
        p::SIGNATURE_CMYK,
        p::SIGNATURE_LAB,
        0,
        0x32434c52,
        0x46434c52,
    ] {
        for with_k in [false, true] {
            for with_a in [false, true] {
                let mut tags = base.clone();
                if with_k {
                    tags.push((signature("kTRC"), curv(&[])));
                }
                if with_a {
                    tags.push((signature("A2B0"), mft(2, 3, 3, 2, 2, true)));
                }
                cases.push(format!(
                    "skcms-profile-getters 0 0 {} 0 0",
                    hex(&icc(&tags, space, p::SIGNATURE_XYZ))
                ));
            }
        }
    }
    cases
}
pub fn lifecycle_requests() -> Vec<String> {
    let mut cases = Vec::new();
    for len in 0..132 {
        cases.push(format!("skcms-profile-short-zero {len}"));
    }
    for source in -1..3 {
        for self_matrix in 0..2 {
            cases.push(format!("skcms-profile-setters {source} {self_matrix}"));
        }
    }
    for flags in 0..8 {
        cases.push(format!("skcms-profile-null-getters {flags}"));
    }
    cases
}
pub fn large_defined_controls() -> Vec<String> {
    let mut cases = Vec::new();
    for count in [
        2,
        3,
        16777216,
        1073741824,
        1073741825,
        1610612736,
        2147483647,
        2147483648,
        u32::MAX,
    ] {
        for operation in [1, 2] {
            for x in [0.0_f32, -0.0, -1.0, f32::NEG_INFINITY] {
                cases.push(format!(
                    "skcms-curve-source-large {operation} {count} {}",
                    x.to_bits()
                ));
            }
        }
    }
    for count in [2147483648, u32::MAX] {
        cases.push(format!("skcms-curve-source-large 0 {count} 0"));
    }
    cases
}
pub fn capture(requests: &[String]) {
    use std::io::Write;
    use std::sync::Mutex;
    static CAPTURE: Mutex<()> = Mutex::new(());
    if let Some(path) = std::env::var_os("JXL_SKCMS_PARSER_CAPTURE") {
        let _guard = CAPTURE.lock().unwrap();
        let mut output = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        for request in requests {
            writeln!(output, "{request}").unwrap();
        }
    }
}
