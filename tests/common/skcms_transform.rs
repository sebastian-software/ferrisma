// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::skcms::profile::{self as p, IccProfile};
use jxl::skcms::transfer_function::TransferFunction as Tf;
use jxl::skcms::transform::{self as t};
use jxl::skcms::transform_types::{AlphaFormat, PixelFormat};
use std::fmt::Write;
use std::mem::size_of;
use std::ptr;
fn num<T: std::str::FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T {
    args.next().unwrap().parse().ok().unwrap()
}
fn unhex(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    s.as_bytes()
        .chunks_exact(2)
        .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect()
}
pub fn hex(data: &[u8]) -> String {
    let mut out = String::new();
    for v in data {
        write!(out, "{v:02x}").unwrap();
    }
    if out.is_empty() { "-".into() } else { out }
}
use crate::skcms_profile_observation as profile_observation;
struct Profile {
    value: Box<IccProfile>,
    _bytes: Vec<u8>,
    pointer: *const IccProfile,
    valid: bool,
}
impl Profile {
    #[expect(
        clippy::excessive_precision,
        reason = "Retain source HLG fixture literal."
    )]
    fn new(kind: i32, data: &str) -> Self {
        let mut value = Box::new(IccProfile::zeroed());
        let mut bytes = Vec::new();
        let mut valid = true;
        let pointer = match kind {
            -1 => ptr::null(),
            0 => p::srgb_profile(),
            1 => p::xyz_d50_profile(),
            _ => {
                // SAFETY: observer profiles and static source objects are fully
                // initialized, with zeroed inactive bytes. All copies preserve them.
                unsafe {
                    if kind == 2 {
                        bytes = unhex(data);
                        valid = p::parse(bytes.as_ptr(), bytes.len(), &mut *value);
                    } else {
                        ptr::copy_nonoverlapping(
                            if kind == 4 {
                                p::xyz_d50_profile()
                            } else {
                                p::srgb_profile()
                            },
                            &mut *value,
                            1,
                        );
                        if kind == 5 || kind == 6 {
                            value.has_cicp = true;
                            value.cicp.transfer_characteristics = if kind == 5 { 16 } else { 18 };
                        }
                        if kind == 7 {
                            for i in 0..3 {
                                value.trc[i].parametric.function = Tf {
                                    g: [1.8, 2.0, 2.2][i],
                                    a: 1.0,
                                    ..Tf::default()
                                };
                            }
                        }
                        if kind == 8 {
                            let mut tf = Tf::default();
                            tf.make_pqish(
                                -107.0 / 128.0,
                                1.0,
                                32.0 / 2523.0,
                                2413.0 / 128.0,
                                -2392.0 / 128.0,
                                8192.0 / 1305.0,
                            );
                            p::set_transfer_function(&mut *value, &tf);
                        }
                        if kind == 9 {
                            let mut tf = Tf::default();
                            tf.make_hlgish(2.0, 2.0, 1.0 / 0.17883277, 0.28466892, 0.55991073);
                            p::set_transfer_function(&mut *value, &tf);
                        }
                        if kind == 10 {
                            p::set_transfer_function(&mut *value, p::identity_transfer_function());
                        }
                        if kind == 11 {
                            value.has_to_xyz_d50 = false;
                        }
                        if kind == 12 {
                            p::set_xyz_d50(
                                &mut *value,
                                ptr::addr_of!((*p::xyz_d50_profile()).to_xyz_d50),
                            );
                        }
                    }
                }
                &*value
            }
        };
        Self {
            value,
            _bytes: bytes,
            pointer,
            valid,
        }
    }
}

// This is pointer identity normalization, never a floating-point mask. Counts
// select table pointers separately before and after the call. A successful
// source conversion replaces that union branch with seven exact function words.
fn normalize_trc_pointers(bytes: &mut [u8], base: *const u8) {
    use jxl::skcms::curve::{Curve, TableCurve};
    use std::mem::offset_of;
    for i in 0..3 {
        let start = offset_of!(IccProfile, trc) + i * size_of::<Curve>();
        let count = u32::from_ne_bytes(bytes[start..start + 4].try_into().unwrap());
        if count != 0 {
            for relative in [
                offset_of!(TableCurve, table_8),
                offset_of!(TableCurve, table_16),
            ] {
                let offset = start + relative;
                let pointer = usize::from_ne_bytes(
                    bytes[offset..offset + size_of::<usize>()]
                        .try_into()
                        .unwrap(),
                );
                let token = if pointer == 0 {
                    usize::MAX
                } else {
                    pointer.wrapping_sub(base as usize)
                };
                bytes[offset..offset + size_of::<usize>()].copy_from_slice(&token.to_ne_bytes());
            }
        }
    }
}

fn function_words(out: &mut String, tf: Tf) {
    for x in [tf.g, tf.a, tf.b, tf.c, tf.d, tf.e, tf.f] {
        write!(out, " {}", x.to_bits()).unwrap();
    }
}

pub fn observe(request: &str) -> String {
    let mut args = request.split_whitespace();
    let command = args.next().unwrap();
    if command == "skcms-transform-private-channel-count" {
        use jxl::skcms::curve::Curve;
        let count = num::<i32>(&mut args);
        assert!(count != i32::MIN && count <= 1);
        let mut curve = std::mem::MaybeUninit::<Curve>::uninit();
        // SAFETY: nonpositive source counts read no curve or output slot. The
        // positive control initializes exactly the reached invalid-g prefix.
        unsafe {
            let p = if count == 1 {
                let p = curve.as_mut_ptr();
                p.cast::<u32>().write(0);
                ptr::addr_of_mut!((*p).parametric.function.g).write(-129.0);
                p.cast_const()
            } else {
                ptr::null()
            };
            return t::oracle_curve_ops(p, count).len().to_string();
        }
    }
    if command == "skcms-transform-private-partial-select" {
        use jxl::skcms::curve::Curve;
        let mode = num::<usize>(&mut args);
        let mut curve = std::mem::MaybeUninit::<Curve>::uninit();
        let c = curve.as_mut_ptr();
        let markers = [
            -129.0,
            f32::NEG_INFINITY,
            -1.0,
            -1.5,
            -127.5,
            -128.0,
            -0.5,
            -2.0,
            -3.0,
            -4.0,
            -5.0,
            -6.0,
            -5.0,
            -6.0,
            -5.0,
            -6.0,
        ];
        // SAFETY: initialize only fields reached by the source's lazy marker
        // classification. Every other payload byte remains truly unknown.
        unsafe {
            c.cast::<u32>().write(0);
            let p = ptr::addr_of_mut!((*c).parametric.function);
            ptr::addr_of_mut!((*p).g).write(markers[mode]);
            match mode {
                10 => ptr::addr_of_mut!((*p).b).write(1.0),
                11 => ptr::addr_of_mut!((*p).d).write(1.0),
                12 => {
                    ptr::addr_of_mut!((*p).b).write(0.0);
                    ptr::addr_of_mut!((*p).c).write(1.0);
                }
                13 => {
                    ptr::addr_of_mut!((*p).d).write(0.0);
                    ptr::addr_of_mut!((*p).e).write(1.0);
                }
                14 => {
                    ptr::addr_of_mut!((*p).b).write(0.0);
                    ptr::addr_of_mut!((*p).c).write(0.0);
                    ptr::addr_of_mut!((*p).d).write(0.0);
                    ptr::addr_of_mut!((*p).e).write(0.0);
                    ptr::addr_of_mut!((*p).f).write(0.0);
                }
                15 => {
                    ptr::addr_of_mut!((*p).d).write(0.0);
                    ptr::addr_of_mut!((*p).e).write(0.0);
                    ptr::addr_of_mut!((*p).f).write(0.0);
                }
                _ => {}
            }
            let ops = t::oracle_curve_ops(c, 1);
            return if ops.is_empty() {
                "0 0".into()
            } else {
                format!("{} 1", ops[0].0)
            };
        }
    }
    if command == "skcms-transform-private-partial-stage" {
        return partial_stage(args.next().unwrap());
    }
    if command == "skcms-transform-private-partial-grid" {
        return partial_grid(&mut args);
    }
    if command == "skcms-transform-private-program" {
        return observe_program(&mut args);
    }
    if command == "skcms-transform-private-bytes" {
        return t::bytes_per_pixel(PixelFormat(num(&mut args))).to_string();
    }
    if command == "skcms-transform-private-cicp" {
        let tf = t::oracle_cicp(num::<u8>(&mut args) != 0);
        let mut out = "T".to_owned();
        function_words(&mut out, tf);
        return out;
    }
    if command == "skcms-transform-private-select" {
        use jxl::skcms::curve::Curve;
        let count = num::<usize>(&mut args);
        assert!(count <= 4);
        let mut curves = [Curve::zeroed(); 4];
        let tables = [[0u8, 255]; 4];
        // SAFETY: every observer union byte is initialized before active-branch
        // stores. Selection reads no table backing; all pointer identities stay live.
        unsafe {
            ptr::write_bytes(
                curves.as_mut_ptr().cast::<u8>(),
                0xa5,
                size_of::<[Curve; 4]>(),
            );
            for i in 0..count {
                let count = num::<u32>(&mut args);
                let values: [f32; 7] = std::array::from_fn(|_| f32::from_bits(num(&mut args)));
                let c = ptr::addr_of_mut!(curves[i]);
                c.cast::<u32>().write(count);
                if count == 0 {
                    ptr::addr_of_mut!((*c).parametric.function).write(Tf {
                        g: values[0],
                        a: values[1],
                        b: values[2],
                        c: values[3],
                        d: values[4],
                        e: values[5],
                        f: values[6],
                    });
                } else {
                    ptr::addr_of_mut!((*c).table.table_8).write(tables[i].as_ptr());
                    ptr::addr_of_mut!((*c).table.table_16).write(ptr::null());
                }
            }
            let selected = t::oracle_curve_ops(curves.as_ptr(), count as i32);
            let mut out = selected.len().to_string();
            for (op, arg) in selected {
                write!(out, " {op} {arg}").unwrap();
            }
            return out;
        }
    }
    if command == "skcms-transform-private-prep" {
        let kind = num::<i32>(&mut args);
        let p = Profile::new(kind, args.next().unwrap());
        if !p.valid {
            return "P".into();
        }
        let mut matrix = jxl::skcms::types::Matrix3x3 {
            vals: std::array::from_fn(|i| {
                std::array::from_fn(|j| f32::from_bits(0x3e800000 + (i * 3 + j) as u32 * 103))
            }),
        };
        let mut tf: [Tf; 3] = std::array::from_fn(|i| {
            let v: [f32; 7] =
                std::array::from_fn(|j| f32::from_bits(0x3f000000 + (i * 7 + j) as u32 * 137));
            Tf {
                g: v[0],
                a: v[1],
                b: v[2],
                c: v[3],
                d: v[4],
                e: v[5],
                f: v[6],
            }
        });
        let mut b2a = true;
        let mut hlg = true;
        // SAFETY: this observer uses initialized disjoint locals, as does the
        // actual Transform caller; profile and active backing remain stable.
        let ok = unsafe { t::oracle_prep(p.pointer, &mut matrix, &mut tf, &mut b2a, &mut hlg) };
        let mut out = format!("{} {} {}", u8::from(ok), u8::from(b2a), u8::from(hlg));
        for row in matrix.vals {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        for v in tf {
            function_words(&mut out, v);
        }
        return out;
    }
    if command == "skcms-transform-unknown" {
        let mode = num::<u8>(&mut args);
        let mut value = std::mem::MaybeUninit::<IccProfile>::uninit();
        let p = value.as_mut_ptr();
        // SAFETY: each case initializes exactly the fields reached by the
        // original short circuit. Inactive bytes are genuinely indeterminate.
        unsafe {
            return match mode {
                0 => {
                    ptr::addr_of_mut!((*p).has_b2a).write(true);
                    u8::from(t::make_usable_as_destination(p)).to_string()
                }
                1 => {
                    ptr::addr_of_mut!((*p).has_b2a).write(false);
                    ptr::addr_of_mut!((*p).has_trc).write(false);
                    u8::from(t::make_usable_as_destination(p)).to_string()
                }
                2 => u8::from(t::trcs_are_approximate_inverse(
                    ptr::null(),
                    ptr::without_provenance(1),
                ))
                .to_string(),
                3 => {
                    u8::from(t::approximately_equal_profiles(ptr::null(), ptr::null())).to_string()
                }
                4 => {
                    ptr::addr_of_mut!((*p).data_color_space).write(p::SIGNATURE_RGB);
                    let mut pixels: [f32; 16] = std::array::from_fn(|i| i as f32 / 7.0);
                    let ok = t::transform(
                        pixels.as_ptr().cast(),
                        PixelFormat::RGBA_ffff,
                        AlphaFormat::Unpremul,
                        p,
                        pixels.as_mut_ptr().cast(),
                        PixelFormat::RGBA_ffff,
                        AlphaFormat::Unpremul,
                        p,
                        4,
                    );
                    format!(
                        "{} {}",
                        u8::from(ok),
                        hex(std::slice::from_raw_parts(pixels.as_ptr().cast(), 64))
                    )
                }
                5 => {
                    ptr::addr_of_mut!((*p).has_b2a).write(false);
                    ptr::addr_of_mut!((*p).has_trc).write(true);
                    ptr::addr_of_mut!((*p).has_to_xyz_d50).write(true);
                    ptr::addr_of_mut!((*p).to_xyz_d50).write(jxl::skcms::types::Matrix3x3 {
                        vals: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                    });
                    ptr::addr_of_mut!((*p).trc[0]).cast::<u32>().write(0);
                    ptr::addr_of_mut!((*p).trc[0].parametric.function.g).write(-129.0);
                    u8::from(t::make_usable_as_destination(p)).to_string()
                }
                6 => {
                    ptr::addr_of_mut!((*p).has_trc).write(true);
                    ptr::addr_of_mut!((*p).trc[0]).cast::<u32>().write(0);
                    ptr::addr_of_mut!((*p).trc[0].parametric.function.g).write(-129.0);
                    let mut inverse = std::mem::MaybeUninit::<Tf>::uninit();
                    ptr::addr_of_mut!((*inverse.as_mut_ptr()).g).write(-129.0);
                    u8::from(t::trcs_are_approximate_inverse(p, inverse.as_ptr())).to_string()
                }
                7 => {
                    ptr::addr_of_mut!((*p).data_color_space).write(p::SIGNATURE_RGB);
                    ptr::addr_of_mut!((*p).has_trc).write(true);
                    ptr::addr_of_mut!((*p).has_to_xyz_d50).write(true);
                    ptr::addr_of_mut!((*p).has_a2b).write(false);
                    ptr::addr_of_mut!((*p).has_cicp).write(false);
                    ptr::addr_of_mut!((*p).to_xyz_d50).write(jxl::skcms::types::Matrix3x3 {
                        vals: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                    });
                    for i in 0..3 {
                        ptr::addr_of_mut!((*p).trc[i]).cast::<u32>().write(0);
                        ptr::addr_of_mut!((*p).trc[i].parametric.function.g).write(-129.0);
                    }
                    let mut pixels: [f32; 16] = std::array::from_fn(|i| i as f32 / 7.0);
                    let ok = t::transform(
                        pixels.as_ptr().cast(),
                        PixelFormat::RGBA_ffff,
                        AlphaFormat::Unpremul,
                        p,
                        pixels.as_mut_ptr().cast(),
                        PixelFormat::RGBA_ffff,
                        AlphaFormat::Unpremul,
                        p::xyz_d50_profile(),
                        4,
                    );
                    format!(
                        "{} {}",
                        u8::from(ok),
                        hex(std::slice::from_raw_parts(pixels.as_ptr().cast(), 64))
                    )
                }
                8..=15 => {
                    let counts = [u32::MAX, u32::MAX - 1, 0xffff_ff80, 0x8000_0001];
                    let count = counts[(mode as usize - 8) % 4];
                    let destination = mode >= 12;
                    ptr::addr_of_mut!((*p).data_color_space).write(p::SIGNATURE_RGB);
                    ptr::addr_of_mut!((*p).pcs).write(p::SIGNATURE_XYZ);
                    ptr::addr_of_mut!((*p).has_cicp).write(false);
                    if destination {
                        ptr::addr_of_mut!((*p).has_b2a).write(true);
                        ptr::addr_of_mut!((*p).b2a.input_channels).write(0);
                        ptr::addr_of_mut!((*p).b2a.matrix_channels).write(0);
                        ptr::addr_of_mut!((*p).b2a.output_channels).write(count);
                    } else {
                        ptr::addr_of_mut!((*p).has_a2b).write(true);
                        ptr::addr_of_mut!((*p).a2b.input_channels).write(count);
                        ptr::addr_of_mut!((*p).a2b.matrix_channels).write(0);
                        ptr::addr_of_mut!((*p).a2b.output_channels).write(0);
                    }
                    let xyz = p::xyz_d50_profile();
                    u8::from(t::transform(
                        ptr::null(),
                        PixelFormat::RGB_fff,
                        AlphaFormat::Opaque,
                        if destination { xyz } else { p },
                        ptr::null_mut(),
                        PixelFormat::RGB_fff,
                        AlphaFormat::Opaque,
                        if destination { p } else { xyz },
                        0,
                    ))
                    .to_string()
                }
                _ => panic!("unknown inactive-state fixture"),
            };
        }
    }
    if command == "skcms-transform" {
        let sf = num::<i32>(&mut args);
        let sa = num::<i32>(&mut args);
        let sk = num::<i32>(&mut args);
        let df = num::<i32>(&mut args);
        let da = num::<i32>(&mut args);
        let dk = num::<i32>(&mut args);
        let n = num::<usize>(&mut args);
        let mode = num::<u8>(&mut args);
        let payload = unhex(args.next().unwrap());
        let source = Profile::new(sk, args.next().unwrap());
        let destination = Profile::new(dk, args.next().unwrap());
        let mut src = [0xa5a5a5a5a5a5a5a5u64; 64];
        let mut dst = src;
        let sp = src.as_mut_ptr().cast::<u8>();
        let dp = match mode {
            1 => sp,
            2 => sp.wrapping_add(16),
            _ => dst.as_mut_ptr().cast::<u8>(),
        };
        // SAFETY: fixture storage is 8-byte aligned, initialized and covers
        // every reached pixel/overlap. Huge dimensions only test early guards.
        unsafe {
            assert!(payload.len() <= 512);
            ptr::copy_nonoverlapping(payload.as_ptr(), sp, payload.len());
            let ok = source.valid
                && destination.valid
                && t::transform(
                    sp,
                    PixelFormat(sf),
                    AlphaFormat(sa),
                    source.pointer,
                    dp,
                    PixelFormat(df),
                    AlphaFormat(da),
                    destination.pointer,
                    n,
                );
            format!(
                "{} {} {} {} {}",
                u8::from(source.valid),
                u8::from(destination.valid),
                u8::from(ok),
                hex(std::slice::from_raw_parts(
                    if mode == 0 { dp } else { sp },
                    512
                )),
                hex(std::slice::from_raw_parts(sp, 512))
            )
        }
    } else if command == "skcms-transform-usable" || command == "skcms-transform-single" {
        let kind = num::<i32>(&mut args);
        let mut profile = Profile::new(kind, args.next().unwrap());
        if !profile.valid {
            return "P".into();
        }
        // SAFETY: the observer initializes every profile byte before the call;
        // the heap object and borrowed table bytes remain stable through it.
        unsafe {
            let before = std::slice::from_raw_parts(
                (&*profile.value as *const IccProfile).cast::<u8>(),
                size_of::<IccProfile>(),
            )
            .to_vec();
            let ok = if command == "skcms-transform-single" {
                t::make_usable_as_destination_with_single_curve(&mut *profile.value)
            } else {
                t::make_usable_as_destination(&mut *profile.value)
            };
            let mut out = format!("{} ", u8::from(ok));
            profile_observation::profile(&mut out, &profile.value, profile._bytes.as_ptr());
            let mut before = before;
            let mut after = std::slice::from_raw_parts(
                (&*profile.value as *const IccProfile).cast::<u8>(),
                size_of::<IccProfile>(),
            )
            .to_vec();
            normalize_trc_pointers(&mut before, profile._bytes.as_ptr());
            normalize_trc_pointers(&mut after, profile._bytes.as_ptr());
            out.push_str(" D");
            for (i, (&a, &b)) in before.iter().zip(&after).enumerate() {
                if a != b {
                    write!(out, " {i}:{b}").unwrap();
                }
            }
            out
        }
    } else if command == "skcms-transform-equal" {
        let a = num::<i32>(&mut args);
        let b = num::<i32>(&mut args);
        let ap = Profile::new(a, args.next().unwrap());
        let bp = Profile::new(b, args.next().unwrap());
        // SAFETY: both observer profile representations and active backing are initialized.
        let ok = unsafe {
            ap.valid && bp.valid && t::approximately_equal_profiles(ap.pointer, bp.pointer)
        };
        format!(
            "{} {} {}",
            u8::from(ap.valid),
            u8::from(bp.valid),
            u8::from(ok)
        )
    } else if command == "skcms-transform-inverse" {
        let kind = num::<i32>(&mut args);
        let profile = Profile::new(kind, args.next().unwrap());
        let w: [f32; 7] = std::array::from_fn(|_| f32::from_bits(num(&mut args)));
        let tf = Tf {
            g: w[0],
            a: w[1],
            b: w[2],
            c: w[3],
            d: w[4],
            e: w[5],
            f: w[6],
        };
        // SAFETY: inverse has seven initialized words; each fixture supplies a
        // source-defined active curve (or the original null/flag short circuit).
        let ok = unsafe { profile.valid && t::trcs_are_approximate_inverse(profile.pointer, &tf) };
        format!("{} {}", u8::from(profile.valid), u8::from(ok))
    } else {
        panic!("unknown transform observation command")
    }
}
pub fn format_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for sf in 0..42 {
        for df in 0..42 {
            for n in [0, 1, 3, 4, 5, 9] {
                let mut payload = vec![0u8; 16 * n];
                for (i, v) in payload.iter_mut().enumerate() {
                    *v = (i * 37 + 13) as u8;
                }
                // Float inputs are finite words; half inputs retain all original patterns.
                if sf >> 1 == 17 || sf >> 1 == 18 {
                    for (i, v) in payload.chunks_exact_mut(4).enumerate() {
                        v.copy_from_slice(&((i as f32 - 3.0) / 7.0).to_ne_bytes());
                    }
                }
                let sa = (sf + df + n as i32) % 3;
                let da = (sf * 2 + df + n as i32) % 3;
                rows.push(format!(
                    "skcms-transform {sf} {sa} -1 {df} {da} -1 {n} 0 {} - -",
                    hex(&payload)
                ));
            }
        }
    }
    rows
}
pub fn color_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for sk in 0..=12 {
        for dk in 0..=12 {
            if sk == 2 || dk == 2 {
                continue;
            }
            for n in [0, 1, 3, 4, 5, 9] {
                let mut payload = Vec::new();
                for i in 0..4 * n {
                    payload.extend_from_slice(&((i as f32 - 4.0) / 19.0).to_ne_bytes());
                }
                rows.push(format!(
                    "skcms-transform 36 1 {sk} 36 1 {dk} {n} 0 {} - -",
                    hex(&payload)
                ));
            }
        }
    }
    rows
}

pub fn alias_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for sf in 0..42 {
        for df in 0..42 {
            for n in [0, 1, 3, 4, 5, 9] {
                for mode in [1, 2] {
                    let mut payload = vec![0u8; 16 * n];
                    for (i, v) in payload.iter_mut().enumerate() {
                        *v = (i * 37 + 13) as u8;
                    }
                    if sf >> 1 == 17 || sf >> 1 == 18 {
                        for (i, v) in payload.chunks_exact_mut(4).enumerate() {
                            v.copy_from_slice(&((i as f32 - 3.0) / 7.0).to_ne_bytes());
                        }
                    }
                    rows.push(format!(
                        "skcms-transform {sf} 1 -1 {df} 1 -1 {n} {mode} {} - -",
                        hex(&payload)
                    ));
                }
            }
        }
    }
    rows
}
pub fn guard_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for invalid in 42..64 {
        for n in [0, 1, 4] {
            for mode in [0, 1] {
                for (sf, df) in [(invalid, 12), (12, invalid), (invalid, invalid)] {
                    rows.push(format!(
                        "skcms-transform {sf} 3 -1 {df} 3 -1 {n} {mode} - - -"
                    ));
                }
            }
        }
    }
    for n in [
        i32::MAX as usize,
        i32::MAX as usize + 1,
        usize::MAX,
        1usize << (usize::BITS - 1),
        1usize << (usize::BITS - 4),
    ] {
        rows.push(format!("skcms-transform 36 3 -1 36 3 -1 {n} 0 - - -"));
    }
    rows
}
pub fn icc_requests(profiles: Vec<Vec<u8>>) -> Vec<String> {
    let mut rows = Vec::new();
    let payload: Vec<u8> = (0..36)
        .flat_map(|i| ((i as f32 - 2.0) / 43.0).to_ne_bytes())
        .collect();
    let payload = hex(&payload);
    for p in profiles {
        let h = hex(&p);
        rows.push(format!("skcms-transform 36 1 2 36 1 1 9 0 {payload} {h} -"));
        rows.push(format!("skcms-transform 36 1 1 36 1 2 9 0 {payload} - {h}"));
    }
    rows
}
pub fn usability_requests(profiles: Vec<Vec<u8>>) -> Vec<String> {
    let mut rows = Vec::new();
    for kind in 3..=12 {
        for command in ["skcms-transform-usable", "skcms-transform-single"] {
            rows.push(format!("{command} {kind} -"));
        }
    }
    for p in profiles {
        let h = hex(&p);
        for command in ["skcms-transform-usable", "skcms-transform-single"] {
            rows.push(format!("{command} 2 {h}"));
        }
    }
    rows
}
pub fn equal_requests(profiles: Vec<Vec<u8>>) -> Vec<String> {
    let mut rows = Vec::new();
    for a in 0..=12 {
        for b in 0..=12 {
            if a != 2 && b != 2 {
                rows.push(format!("skcms-transform-equal {a} {b} - -"));
            }
        }
    }
    for p in profiles {
        let h = hex(&p);
        for kind in [0, 1] {
            rows.push(format!("skcms-transform-equal 2 {kind} {h} -"));
            rows.push(format!("skcms-transform-equal {kind} 2 - {h}"));
        }
    }
    rows
}
pub fn inverse_requests(profiles: Vec<Vec<u8>>) -> Vec<String> {
    let mut rows = Vec::new();
    let functions = [
        [1.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [1.0 / 2.2, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];
    let mut descriptors: Vec<(i32, String)> = (-1..=12)
        .filter(|i| *i != 2)
        .map(|i| (i, "-".into()))
        .collect();
    descriptors.extend(profiles.into_iter().map(|p| (2, hex(&p))));
    for (kind, h) in descriptors {
        for tf in functions {
            let words = tf.map(f32::to_bits).map(|v| v.to_string()).join(" ");
            rows.push(format!("skcms-transform-inverse {kind} {h} {words}"));
        }
    }
    rows
}

pub fn selection_requests() -> Vec<String> {
    let identity = [1.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut gamma = identity;
    gamma[0] = 2.2;
    let mut negative_zero = gamma;
    negative_zero[6] = -0.0;
    let templates = [
        (0, identity),
        (0, gamma),
        (
            0,
            [2.4, 0.9478673, 0.0521327, 0.07739938, 0.04045, 0.0, 0.0],
        ),
        (0, [2.0, 1.0, 0.1, 0.2, 0.3, 0.4, 0.5]),
        (0, [0.0; 7]),
        (
            0,
            [
                -2.0,
                -107.0 / 128.0,
                1.0,
                32.0 / 2523.0,
                2413.0 / 128.0,
                -2392.0 / 128.0,
                8192.0 / 1305.0,
            ],
        ),
        (
            0,
            [-3.0, 2.0, 2.0, 1.0 / 0.17883277, 0.28466892, 0.5599107, 0.0],
        ),
        (0, [-4.0, 0.5, 0.5, 0.17883277, 0.28466892, 0.5599107, 0.0]),
        (0, [-5.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
        (0, [-6.0, 1.0, 1.0, 1.2, 0.0, 0.0, 0.0]),
        (1, identity),
        (u32::MAX, identity),
        (0, negative_zero),
    ];
    let mut rows = Vec::new();
    for channels in 0..=4 {
        for mut combination in 0..templates.len().pow(channels) {
            let mut row = format!("skcms-transform-private-select {channels}");
            for _ in 0..channels {
                let (count, words) = templates[combination % templates.len()];
                combination /= templates.len();
                write!(row, " {count}").unwrap();
                for word in words {
                    write!(row, " {}", word.to_bits()).unwrap();
                }
            }
            rows.push(row);
        }
    }
    rows
}
pub fn prep_requests(profiles: Vec<Vec<u8>>) -> Vec<String> {
    let mut rows: Vec<String> = (0..=12)
        .filter(|i| *i != 2)
        .map(|i| format!("skcms-transform-private-prep {i} -"))
        .collect();
    rows.extend(
        profiles
            .into_iter()
            .map(|p| format!("skcms-transform-private-prep 2 {}", hex(&p))),
    );
    rows
}
pub fn capture(requests: &[String]) {
    use std::io::Write;
    use std::sync::Mutex;
    static CAPTURE: Mutex<()> = Mutex::new(());
    if let Some(path) = std::env::var_os("JXL_SKCMS_TRANSFORM_CAPTURE") {
        let _guard = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        for q in requests {
            writeln!(file, "{q}").unwrap();
        }
    }
}

struct StageContext {
    tf: Tf,
    curve: jxl::skcms::curve::Curve,
    m3: jxl::skcms::types::Matrix3x3,
    m4: jxl::skcms::types::Matrix3x4,
    a2b: jxl::skcms::profile::A2B,
    b2a: jxl::skcms::profile::B2A,
    bytes: Vec<u8>,
}
impl StageContext {
    fn new() -> Self {
        Self {
            tf: Tf::default(),
            curve: jxl::skcms::curve::Curve::zeroed(),
            m3: Default::default(),
            m4: Default::default(),
            a2b: jxl::skcms::profile::A2B::zeroed(),
            b2a: jxl::skcms::profile::B2A::zeroed(),
            bytes: Vec::new(),
        }
    }
}
fn observe_program(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let n = num::<i32>(args);
    let sbpp = num::<usize>(args);
    let dbpp = num::<usize>(args);
    let input = unhex(args.next().unwrap());
    let count = num::<usize>(args);
    assert!(count <= 32 && input.len() <= 512);
    let names: Vec<_> = include_str!("../data/skcms-transform-ops.txt")
        .lines()
        .collect();
    let index = |name: &str| names.iter().position(|n| *n == name).unwrap() as i32;
    let mut contexts: Vec<StageContext> = (0..32).map(|_| StageContext::new()).collect();
    let mut ops = Vec::new();
    let mut ptrs = Vec::new();
    for c in contexts.iter_mut().take(count) {
        let name = args.next().unwrap();
        let op = index(name);
        ops.push(op);
        let mut arg = ptr::null();
        if name == "matrix_3x3" || name == "matrix_3x4" {
            let words: Vec<f32> = (0..if name == "matrix_3x3" { 9 } else { 12 })
                .map(|_| f32::from_bits(num(args)))
                .collect();
            if words.len() == 9 {
                c.m3.vals = std::array::from_fn(|i| std::array::from_fn(|j| words[i * 3 + j]));
                arg = (&raw const c.m3).cast();
            } else {
                c.m4.vals = std::array::from_fn(|i| std::array::from_fn(|j| words[i * 4 + j]));
                arg = (&raw const c.m4).cast();
            }
        } else if (op >= index("gamma_r") && op <= index("hlg_rgb"))
            || (op >= index("hlginv_r") && op <= index("hlginv_rgb"))
        {
            let v: [f32; 7] = std::array::from_fn(|_| f32::from_bits(num(args)));
            c.tf = Tf {
                g: v[0],
                a: v[1],
                b: v[2],
                c: v[3],
                d: v[4],
                e: v[5],
                f: v[6],
            };
            arg = (&raw const c.tf).cast();
        } else if name.starts_with("table_") {
            let entries = num::<u32>(args);
            let width = num::<u8>(args);
            c.bytes = unhex(args.next().unwrap());
            // SAFETY: only active table fields are assigned; backing remains
            // stable until execution finishes and all source indices are valid.
            unsafe {
                ptr::addr_of_mut!(c.curve).cast::<u32>().write(entries);
                ptr::addr_of_mut!(c.curve.table.table_8).write(if width == 1 {
                    c.bytes.as_ptr()
                } else {
                    ptr::null()
                });
                ptr::addr_of_mut!(c.curve.table.table_16).write(if width == 2 {
                    c.bytes.as_ptr()
                } else {
                    ptr::null()
                });
            }
            arg = (&raw const c.curve).cast();
        } else if name.starts_with("clut_") {
            let dim = num::<u32>(args);
            let outputs = num::<u32>(args);
            let width = num::<u8>(args);
            let points: [u8; 4] = std::array::from_fn(|_| num(args));
            c.bytes = unhex(args.next().unwrap());
            let grid = c.bytes.as_ptr().wrapping_add(2);
            if name == "clut_A2B" {
                c.a2b.input_channels = dim;
                c.a2b.output_channels = outputs;
                c.a2b.grid_points = points;
                c.a2b.grid_8 = if width == 1 { grid } else { ptr::null() };
                c.a2b.grid_16 = if width == 2 { grid } else { ptr::null() };
                arg = (&raw const c.a2b).cast();
            } else {
                c.b2a.input_channels = dim;
                c.b2a.output_channels = outputs;
                c.b2a.grid_points = points;
                c.b2a.grid_8 = if width == 1 { grid } else { ptr::null() };
                c.b2a.grid_16 = if width == 2 { grid } else { ptr::null() };
                arg = (&raw const c.b2a).cast();
            }
        }
        ptrs.push(arg);
    }
    let mut source = [0xa5a5a5a5a5a5a5a5u64; 64];
    let mut output = source;
    // SAFETY: initialized aligned storage covers every fixture's four-lane
    // load/store and tail. Each data-only context owns its complete live backing.
    unsafe {
        ptr::copy_nonoverlapping(input.as_ptr(), source.as_mut_ptr().cast(), input.len());
        t::oracle_program(
            &ops,
            &ptrs,
            source.as_ptr().cast(),
            output.as_mut_ptr().cast(),
            n,
            sbpp,
            dbpp,
        );
        format!(
            "{} {}",
            hex(std::slice::from_raw_parts(output.as_ptr().cast(), 512)),
            hex(std::slice::from_raw_parts(source.as_ptr().cast(), 512))
        )
    }
}

fn float_payload(n: usize, exceptional: bool) -> String {
    let exceptional_words: [u32; 16] = [
        0, 0x80000000, 0x3f800000, 0xbf800000, 1, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff,
        0x7f800000, 0xff800000, 0x7fc12345, 0xffc56789, 0x7fa23456, 0xffa76543, 0x3f000000,
    ];
    let bytes: Vec<u8> = (0..4 * n)
        .flat_map(|i| {
            if exceptional {
                exceptional_words[i % exceptional_words.len()].to_ne_bytes()
            } else {
                ((i as f32 - 4.0) / 19.0).to_ne_bytes()
            }
        })
        .collect();
    hex(&bytes)
}
fn word_arguments(values: &[f32]) -> String {
    values
        .iter()
        .map(|v| v.to_bits().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
fn stage_context(name: &str, width: usize, dim: usize, outputs: usize, points: usize) -> String {
    if name == "matrix_3x3" {
        return word_arguments(&[0.7, 0.2, 0.1, 0.1, 0.8, 0.1, 0.2, 0.2, 0.6]);
    }
    if name == "matrix_3x4" {
        return word_arguments(&[0.7, 0.2, 0.1, 0.03, 0.1, 0.8, 0.1, -0.1, 0.2, 0.2, 0.6, 0.2]);
    }
    if name.starts_with("gamma_") {
        return word_arguments(&[2.2, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
    if name.starts_with("tf_") {
        return word_arguments(&[2.4, 0.9478673, 0.0521327, 0.07739938, 0.04045, 0.0, 0.0]);
    }
    if name.starts_with("pq_") {
        return word_arguments(&[
            -2.0,
            -107.0 / 128.0,
            1.0,
            32.0 / 2523.0,
            2413.0 / 128.0,
            -2392.0 / 128.0,
            8192.0 / 1305.0,
        ]);
    }
    if name.starts_with("hlg_") && name != "hlg_ootf_scale" {
        return word_arguments(&[-3.0, 2.0, 2.0, 1.0 / 0.17883277, 0.28466892, 0.5599107, 0.0]);
    }
    if name.starts_with("hlginv_") && name != "hlginv_ootf_scale" {
        return word_arguments(&[-4.0, 0.5, 0.5, 0.17883277, 0.28466892, 0.5599107, 0.0]);
    }
    if name.starts_with("table_") {
        let bytes = if width == 1 {
            vec![0, 7, 100, 200, 255]
        } else {
            [0u16, 1, 32768, 50000, 65535]
                .into_iter()
                .flat_map(u16::to_be_bytes)
                .collect()
        };
        return format!("5 {width} {}", hex(&bytes));
    }
    if name.starts_with("clut_") {
        let mut bytes = vec![0xa5, 0x5a];
        let values = points.pow(dim as u32) * outputs;
        for i in 0..values {
            if width == 1 {
                bytes.push((i * 37 + 11) as u8);
            } else {
                bytes.extend_from_slice(&((i * 7919 + 53) as u16).to_be_bytes());
            }
        }
        let points = (0..4)
            .map(|i| {
                if i < dim {
                    points.to_string()
                } else {
                    "0".into()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        return format!("{dim} {outputs} {width} {points} {}", hex(&bytes));
    }
    String::new()
}
pub fn program_requests() -> Vec<String> {
    let names: Vec<_> = include_str!("../data/skcms-transform-ops.txt")
        .lines()
        .collect();
    let work = &names[18..61];
    let mut rows = Vec::new();
    for &name in work {
        for n in [0, 1, 2, 3, 4, 5, 7, 8, 9, 17] {
            for exceptional in [false, true] {
                if exceptional && name.starts_with("clut_") {
                    continue;
                }
                let context = stage_context(
                    name,
                    1,
                    if name == "clut_A2B" { 4 } else { 3 },
                    if name == "clut_A2B" { 3 } else { 4 },
                    2,
                );
                let clamp = if name.starts_with("clut_") {
                    "clamp "
                } else {
                    ""
                };
                let count = if clamp.is_empty() { 3 } else { 4 };
                rows.push(format!("skcms-transform-private-program {n} 16 16 {} {count} load_ffff {clamp}{name} {context} store_ffff",float_payload(n,exceptional)));
            }
        }
    }
    for name in ["table_r", "table_g", "table_b", "table_a"] {
        for n in [1, 3, 4, 5, 9, 17] {
            rows.push(format!(
                "skcms-transform-private-program {n} 16 16 {} 3 load_ffff {name} {} store_ffff",
                float_payload(n, false),
                stage_context(name, 2, 0, 0, 0)
            ));
        }
    }
    for dim in 1..=4 {
        for outputs in [3, 4] {
            for width in [1, 2] {
                for points in [2, 3] {
                    for n in [0, 1, 3, 4, 5, 9, 17] {
                        let name = if outputs == 3 { "clut_A2B" } else { "clut_B2A" };
                        if name == "clut_B2A" && dim != 3 {
                            continue;
                        }
                        rows.push(format!("skcms-transform-private-program {n} 16 16 {} 4 load_ffff clamp {name} {} store_ffff",float_payload(n,false),stage_context(name,width,dim,outputs,points)));
                    }
                }
            }
        }
    }
    rows
}
pub fn half_domain_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for fmt in [30, 32] {
        let channels = if fmt == 30 { 3 } else { 4 };
        for first in (0..=65535u32).step_by(4) {
            let bytes: Vec<u8> = (0..4)
                .flat_map(|i| {
                    (0..channels).flat_map(move |c| {
                        ((first + i) as u16 ^ [0u16, 0x8000, 0x1555, 0xaaaa][c]).to_ne_bytes()
                    })
                })
                .collect();
            rows.push(format!(
                "skcms-transform {fmt} 1 -1 36 1 -1 4 0 {} - -",
                hex(&bytes)
            ));
        }
    }
    rows
}
pub fn normalized_integer_rounding_requests() -> Vec<String> {
    let mut rows = Vec::new();
    for k in (0..=65535u32).step_by(7) {
        let center = ((k as f32 + 0.5) / 65535.0).to_bits();
        let words = [
            center.wrapping_sub(1),
            center,
            center.wrapping_add(1),
            0x3f800000,
        ];
        let bytes: Vec<u8> = (0..4)
            .flat_map(|_| words.into_iter().flat_map(u32::to_ne_bytes))
            .collect();
        for fmt in [18, 20, 22, 24] {
            rows.push(format!(
                "skcms-transform 36 1 -1 {fmt} 1 -1 4 0 {} - -",
                hex(&bytes)
            ));
        }
    }
    rows
}

fn partial_stage(name: &str) -> String {
    let mut tf = std::mem::MaybeUninit::<Tf>::uninit();
    let p = tf.as_mut_ptr();
    let names: Vec<_> = include_str!("../data/skcms-transform-ops.txt")
        .lines()
        .collect();
    let op = |s: &str| names.iter().position(|n| *n == s).unwrap() as i32;
    let source: [f32; 16] = std::array::from_fn(|i| (i as f32 - 3.0) / 19.0);
    let mut output = [0.0f32; 16];
    // SAFETY: gamma initializes exactly g; PQ/HLG initialize exactly a..f.
    // The other source words stay unknown and must never be materialized.
    unsafe {
        if name.starts_with("gamma_") {
            ptr::addr_of_mut!((*p).g).write(2.2);
        } else {
            let values = if name.starts_with("pq_") {
                [
                    -107.0 / 128.0,
                    1.0,
                    32.0 / 2523.0,
                    2413.0 / 128.0,
                    -2392.0 / 128.0,
                    8192.0 / 1305.0,
                ]
            } else if name.starts_with("hlg_") {
                [2.0, 2.0, 1.0 / 0.17883277, 0.28466892, 0.5599107, 0.0]
            } else {
                [0.5, 0.5, 0.17883277, 0.28466892, 0.5599107, 0.0]
            };
            ptr::addr_of_mut!((*p).a).write(values[0]);
            ptr::addr_of_mut!((*p).b).write(values[1]);
            ptr::addr_of_mut!((*p).c).write(values[2]);
            ptr::addr_of_mut!((*p).d).write(values[3]);
            ptr::addr_of_mut!((*p).e).write(values[4]);
            ptr::addr_of_mut!((*p).f).write(values[5]);
        }
        t::oracle_program(
            &[op("load_ffff"), op(name), op("store_ffff")],
            &[ptr::null(), p.cast(), ptr::null()],
            source.as_ptr().cast(),
            output.as_mut_ptr().cast(),
            4,
            16,
            16,
        );
        hex(std::slice::from_raw_parts(output.as_ptr().cast(), 64))
    }
}
fn partial_grid(args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::skcms::profile::{A2B, B2A};
    let dim = num::<usize>(args);
    let outputs = num::<u32>(args);
    let width = num::<u8>(args);
    let kind = num::<u8>(args);
    let bytes: [u8; 130] = std::array::from_fn(|i| (i * 13 + 5) as u8);
    let grid = bytes.as_ptr().wrapping_add(2);
    let mut a = std::mem::MaybeUninit::<A2B>::uninit();
    let mut b = std::mem::MaybeUninit::<B2A>::uninit();
    let names: Vec<_> = include_str!("../data/skcms-transform-ops.txt")
        .lines()
        .collect();
    let op = |s: &str| names.iter().position(|n| *n == s).unwrap() as i32;
    let source: [f32; 16] = std::array::from_fn(|i| i as f32 / 19.0);
    let mut output = [0.0f32; 16];
    // SAFETY: initialize exactly the wrapper's scalar fields, both evaluated
    // grid pointers and the reached dimension prefix. All other points and
    // curves/matrices remain genuinely unknown, matching original call sites.
    unsafe {
        let context = if kind == 0 {
            let p = a.as_mut_ptr();
            ptr::addr_of_mut!((*p).input_channels).write(dim as u32);
            ptr::addr_of_mut!((*p).output_channels).write(outputs);
            for i in 0..dim {
                ptr::addr_of_mut!((*p).grid_points)
                    .cast::<u8>()
                    .add(i)
                    .write(2);
            }
            ptr::addr_of_mut!((*p).grid_8).write(if width == 1 { grid } else { ptr::null() });
            ptr::addr_of_mut!((*p).grid_16).write(if width == 2 { grid } else { ptr::null() });
            p.cast()
        } else {
            let p = b.as_mut_ptr();
            ptr::addr_of_mut!((*p).input_channels).write(dim as u32);
            ptr::addr_of_mut!((*p).output_channels).write(outputs);
            for i in 0..dim {
                ptr::addr_of_mut!((*p).grid_points)
                    .cast::<u8>()
                    .add(i)
                    .write(2);
            }
            ptr::addr_of_mut!((*p).grid_8).write(if width == 1 { grid } else { ptr::null() });
            ptr::addr_of_mut!((*p).grid_16).write(if width == 2 { grid } else { ptr::null() });
            p.cast()
        };
        t::oracle_program(
            &[
                op("load_ffff"),
                op(if kind == 0 { "clut_A2B" } else { "clut_B2A" }),
                op("store_ffff"),
            ],
            &[ptr::null(), context, ptr::null()],
            source.as_ptr().cast(),
            output.as_mut_ptr().cast(),
            4,
            16,
            16,
        );
        hex(std::slice::from_raw_parts(output.as_ptr().cast(), 64))
    }
}
pub fn partial_context_requests() -> Vec<String> {
    let mut rows: Vec<String> = (0..16)
        .map(|i| format!("skcms-transform-private-partial-select {i}"))
        .collect();
    for count in [-2147483647, -128, -2, -1, 0, 1] {
        rows.push(format!("skcms-transform-private-channel-count {count}"));
    }
    for family in ["gamma", "pq", "hlg", "hlginv"] {
        for channel in ["r", "g", "b", "a", "rgb"] {
            rows.push(format!(
                "skcms-transform-private-partial-stage {family}_{channel}"
            ));
        }
    }
    for dim in 1..=4 {
        for width in [1, 2] {
            rows.push(format!(
                "skcms-transform-private-partial-grid {dim} 3 {width} 0"
            ));
        }
    }
    for outputs in [3, 4] {
        for width in [1, 2] {
            rows.push(format!(
                "skcms-transform-private-partial-grid 3 {outputs} {width} 1"
            ));
        }
    }
    rows
}
