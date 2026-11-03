// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::cms::jxl_cms::{get_default_cms, oracle_matrix_product};
use jxl::jxl::cms_interface::{IccProfileSpan, JxlCmsInterface, JxlColorProfile};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::skcms::types::Matrix3x3;
use std::ffi::{c_int, c_void};
use std::fmt::Write;
use std::mem::MaybeUninit;
use std::ptr;
fn number<T: std::str::FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T {
    args.next().unwrap().parse().ok().unwrap()
}
fn unhex(hex: &str) -> Vec<u8> {
    if hex == "-" {
        return Vec::new();
    }
    hex.as_bytes()
        .chunks_exact(2)
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect()
}
fn encoding(c: &JxlColorEncoding, out: &mut String) {
    write!(out, " {} {}", c.color_space.0, c.white_point.0).unwrap();
    for x in c.white_point_xy {
        write!(out, " {}", x.to_bits()).unwrap();
    }
    write!(out, " {}", c.primaries.0).unwrap();
    for xy in [
        c.primaries_red_xy,
        c.primaries_green_xy,
        c.primaries_blue_xy,
    ] {
        for x in xy {
            write!(out, " {}", x.to_bits()).unwrap();
        }
    }
    write!(
        out,
        " {} {} {}",
        c.transfer_function.0,
        c.gamma.to_bits(),
        c.rendering_intent.0
    )
    .unwrap();
}
struct CallbackData {
    state: *mut CallbackState,
    tag: u32,
}
unsafe extern "C" fn prefix_fail(
    data: *mut c_void,
    bytes: *const u8,
    size: usize,
    _output: *mut JxlColorEncoding,
    _cmyk: *mut c_int,
) -> c_int {
    // SAFETY: observer-owned count/output storage and copied live input span;
    // failure leaves the source callback's output carriers genuinely unknown.
    unsafe {
        let state = &mut *data.cast::<(usize, String)>();
        state.0 += 1;
        write!(
            state.1,
            " {size} {}",
            hex_bytes(std::slice::from_raw_parts(bytes, size))
        )
        .unwrap();
    }
    0
}
struct CallbackState {
    mode: u32,
    calls: usize,
    interface: *mut JxlCmsInterface,
    source: *mut JxlColorProfile,
    destination: *mut JxlColorProfile,
    replacement_source: IccProfileSpan,
    replacement_destination: IccProfileSpan,
    first: CallbackData,
    second: CallbackData,
    events: String,
}
unsafe extern "C" fn callback_second(
    data: *mut c_void,
    bytes: *const u8,
    size: usize,
    output: *mut JxlColorEncoding,
    cmyk: *mut c_int,
) -> c_int {
    // SAFETY: identical observer callback contract and live state as callback.
    unsafe { callback(data, bytes, size, output, cmyk) }
}
unsafe extern "C" fn callback(
    data: *mut c_void,
    bytes: *const u8,
    size: usize,
    output: *mut JxlColorEncoding,
    cmyk: *mut c_int,
) -> c_int {
    // SAFETY: data refers to one of the two stable local callback carriers.
    // All profile spans remain backed for the entire init/row/destroy cycle.
    unsafe {
        let data = data.cast::<CallbackData>();
        // Read the nested carrier before borrowing its containing state. Do
        // not retain an inner reference across the whole-state mutable borrow.
        let tag = ptr::addr_of!((*data).tag).read();
        let state = ptr::addr_of!((*data).state).read();
        let s = &mut *state;
        s.calls += 1;
        let call = s.calls;
        let mut value = MaybeUninit::<JxlColorEncoding>::uninit();
        ptr::write_bytes(
            value.as_mut_ptr().cast::<u8>(),
            0x5a,
            std::mem::size_of::<JxlColorEncoding>(),
        );
        let mut value = value.assume_init();
        let mut local_cmyk = 0x12345678;
        let original = get_default_cms();
        let delegated = original.set_fields_from_icc.unwrap()(
            original.set_fields_data,
            bytes,
            size,
            &mut value,
            &mut local_cmyk,
        );
        let forced = (call == 1 && s.mode & 1 != 0) || (call == 2 && s.mode & 2 != 0);
        let result = if forced || delegated == 0 {
            0
        } else if s.mode & 32 != 0 {
            -9
        } else {
            1
        };
        write!(
            s.events,
            " {} {} {} {size} {} {delegated} {result} {local_cmyk}",
            tag,
            u8::from(bytes == (*s.source).icc.data),
            u8::from(bytes == (*s.destination).icc.data),
            hex_bytes(std::slice::from_raw_parts(bytes, size))
        )
        .unwrap();
        encoding(&value, &mut s.events);
        if delegated != 0 {
            if s.mode & 64 != 0 {
                local_cmyk = -7;
            }
            cmyk.write(local_cmyk);
            if s.mode & 128 != 0 {
                use jxl::jxl::color_encoding::{
                    ColorSpace, Primaries, TransferFunction, WhitePoint,
                };
                ptr::addr_of_mut!((*output).color_space).write(value.color_space);
                ptr::addr_of_mut!((*output).white_point).write(value.white_point);
                if value.white_point == WhitePoint::CUSTOM {
                    ptr::addr_of_mut!((*output).white_point_xy).write(value.white_point_xy);
                }
                if value.color_space == ColorSpace::RGB || value.color_space == ColorSpace::UNKNOWN
                {
                    ptr::addr_of_mut!((*output).primaries).write(value.primaries);
                    if value.primaries == Primaries::CUSTOM {
                        ptr::addr_of_mut!((*output).primaries_red_xy).write(value.primaries_red_xy);
                        ptr::addr_of_mut!((*output).primaries_green_xy)
                            .write(value.primaries_green_xy);
                        ptr::addr_of_mut!((*output).primaries_blue_xy)
                            .write(value.primaries_blue_xy);
                    }
                }
                ptr::addr_of_mut!((*output).transfer_function).write(value.transfer_function);
                if value.transfer_function == TransferFunction::GAMMA {
                    ptr::addr_of_mut!((*output).gamma).write(value.gamma);
                }
                ptr::addr_of_mut!((*output).rendering_intent).write(value.rendering_intent);
            } else {
                output.write(value);
            }
        }
        if call == 1 && s.mode & 4 != 0 {
            ptr::addr_of_mut!((*s.interface).set_fields_data)
                .write(ptr::addr_of_mut!(s.second).cast());
            ptr::addr_of_mut!((*s.interface).set_fields_from_icc).write(Some(callback_second));
        }
        if call == 1 && s.mode & 8 != 0 {
            ptr::addr_of_mut!((*s.destination).icc).write(s.replacement_destination);
        }
        if call == 2 && s.mode & 16 != 0 {
            ptr::addr_of_mut!((*s.source).icc).write(s.replacement_source);
        }
        result
    }
}
fn hex_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".into();
    }
    bytes.iter().map(|x| format!("{x:02x}")).collect()
}
pub fn observe(line: &str) -> String {
    let mut args = line.split_whitespace();
    let command = args.next().unwrap();
    let cms = get_default_cms();
    match command {
        "default-cms-init-prefix" => {
            let mode: u32 = number(&mut args);
            let mut interface = MaybeUninit::<JxlCmsInterface>::uninit();
            let i = interface.as_mut_ptr();
            let mut source = MaybeUninit::<JxlColorProfile>::uninit();
            let s = source.as_mut_ptr();
            let mut destination = MaybeUninit::<JxlColorProfile>::uninit();
            let d = destination.as_mut_ptr();
            let byte = 0x5a_u8;
            let mut state = (0_usize, "calls".to_owned());
            // SAFETY: each case initializes exactly the fields reached before
            // its source guard/failing callback. Other interface, descriptor
            // and callback-output words remain unknown and are never borrowed.
            unsafe {
                if mode == 1 || mode == 2 {
                    ptr::addr_of_mut!((*s).icc.size).write(0);
                } else if mode >= 3 {
                    ptr::addr_of_mut!((*s).icc.size).write(1);
                    ptr::addr_of_mut!((*d).icc.size).write(if mode <= 4 { 0 } else { 1 });
                    if mode >= 5 {
                        ptr::addr_of_mut!((*s).icc.data).write(&byte);
                        ptr::addr_of_mut!((*i).set_fields_data).write((&raw mut state).cast());
                        ptr::addr_of_mut!((*i).set_fields_from_icc).write(Some(prefix_fail));
                    }
                }
                let dimension = if mode & 1 != 0 { usize::MAX } else { 0 };
                let context = cms.init.unwrap()(
                    if mode == 0 { ptr::null_mut() } else { i.cast() },
                    dimension,
                    dimension,
                    if mode == 0 { ptr::null() } else { s },
                    if mode <= 2 { ptr::null() } else { d },
                    f32::from_bits(0x7fc12345),
                );
                write!(state.1, " end {} {}", state.0, u8::from(!context.is_null())).unwrap();
                assert!(context.is_null());
                state.1
            }
        }
        "default-cms-numeric" => {
            use jxl::jxl::cms::jxl_cms as c;
            let which: u32 = number(&mut args);
            match which {
                0 => {
                    let xyz = std::array::from_fn(|_| f32::from_bits(number(&mut args)));
                    let xy = c::oracle_cie_xy_from_xyz(xyz);
                    format!("{} {}", xy.x.to_bits(), xy.y.to_bits())
                }
                1 => {
                    let matrix = Matrix3x3 {
                        vals: std::array::from_fn(|_| {
                            std::array::from_fn(|_| f32::from_bits(number(&mut args)))
                        }),
                    };
                    u8::from(c::oracle_acceptable_chad(&matrix)).to_string()
                }
                2 => {
                    let tf = number(&mut args);
                    let prim = number(&mut args);
                    format!(
                        "{} {}",
                        u8::from(c::oracle_known_transfer_function(tf)),
                        u8::from(c::oracle_known_color_primaries(prim))
                    )
                }
                3 => {
                    let mut profile = MaybeUninit::<jxl::skcms::profile::IccProfile>::uninit();
                    let p = profile.as_mut_ptr();
                    // SAFETY: ColorSpaceFromProfile reaches only this signature.
                    unsafe {
                        ptr::addr_of_mut!((*p).data_color_space).write(number(&mut args));
                        c::oracle_color_space(p).to_string()
                    }
                }
                _ => panic!("numeric fixture"),
            }
        }
        "default-cms-allocate" => {
            use std::cell::UnsafeCell;
            let initial: usize = number(&mut args);
            let count: usize = number(&mut args);
            let mut storage: Vec<_> = (0..initial)
                .map(|i| UnsafeCell::new(f32::from_bits(0x3f000000 + i as u32)))
                .collect();
            let mut views = Vec::new();
            let mut out = count.to_string();
            for _ in 0..count {
                let length = number(&mut args);
                let threads = number(&mut args);
                // SAFETY: fixtures retain source-defined wrapped allocations
                // covering every view. Cells remain owned and initialized.
                unsafe {
                    jxl::jxl::cms::jxl_cms::oracle_allocate_buffer(
                        length,
                        threads,
                        &mut storage,
                        &mut views,
                    );
                }
                write!(out, " {}", storage.len()).unwrap();
                for value in &storage {
                    // SAFETY: no callback executes during this immutable snapshot.
                    write!(out, " {}", unsafe { (*value.get()).to_bits() }).unwrap();
                }
                write!(out, " {}", views.len()).unwrap();
                for &view in &views {
                    let address = view as usize;
                    let start = storage.as_ptr() as usize;
                    let end = start + storage.len() * 4;
                    write!(
                        out,
                        " {} {} {}",
                        address % 128,
                        address.wrapping_sub(views[0] as usize),
                        u8::from(address >= start && address <= end)
                    )
                    .unwrap();
                }
            }
            out
        }
        "default-cms-luminances" | "default-cms-profile-equal" => {
            use jxl::jxl::cms::color_encoding_cms::ColorEncoding;
            use jxl::jxl::cms::jxl_cms as c;
            let mut bytes = unhex(args.next().unwrap());
            let mut encoding = ColorEncoding::default();
            // SAFETY: actual immutable default callback and live input ownership.
            let fields = unsafe { encoding.set_fields_from_icc(&mut bytes, cms).is_ok() };
            let mut out = u8::from(fields).to_string();
            if fields {
                if command == "default-cms-luminances" {
                    let null: u32 = number(&mut args);
                    let mut output = [f32::from_bits(0x3f123456); 3];
                    // SAFETY: null is used only for original prerequisite exits;
                    // success paths supply all three initialized writable words.
                    let status = unsafe {
                        c::oracle_primaries_luminances(
                            &encoding,
                            if null != 0 {
                                ptr::null_mut()
                            } else {
                                output.as_mut_ptr()
                            },
                        )
                    };
                    write!(out, " {}", u8::from(status)).unwrap();
                    for x in output {
                        write!(out, " {}", x.to_bits()).unwrap();
                    }
                } else {
                    let input = unhex(args.next().unwrap());
                    let mut profile = MaybeUninit::<jxl::skcms::profile::IccProfile>::uninit();
                    // SAFETY: parser receives live bytes and output storage;
                    // comparison reaches only successful parser-initialized data.
                    unsafe {
                        let parsed = jxl::skcms::profile::parse(
                            input.as_ptr(),
                            input.len(),
                            profile.as_mut_ptr(),
                        );
                        write!(out, " {}", u8::from(parsed)).unwrap();
                        if parsed {
                            write!(
                                out,
                                " {}",
                                u8::from(c::oracle_approximately_equal(
                                    profile.as_ptr(),
                                    &encoding
                                ))
                            )
                            .unwrap();
                        }
                    }
                }
            }
            out
        }
        "default-cms-callback" => {
            let mode = number(&mut args);
            let buffers: Vec<_> = (0..4).map(|_| unhex(args.next().unwrap())).collect();
            let span = |index: usize| IccProfileSpan {
                // Match the selected libc++ empty-vector null data pointer.
                data: if buffers[index].is_empty() {
                    ptr::null()
                } else {
                    buffers[index].as_ptr()
                },
                size: buffers[index].len(),
            };
            let mut interface = MaybeUninit::<JxlCmsInterface>::uninit();
            let i = interface.as_mut_ptr();
            let mut source = MaybeUninit::<JxlColorProfile>::uninit();
            let s = source.as_mut_ptr();
            let mut destination = MaybeUninit::<JxlColorProfile>::uninit();
            let d = destination.as_mut_ptr();
            let mut state = CallbackState {
                mode,
                calls: 0,
                interface: i,
                source: s,
                destination: d,
                replacement_source: span(2),
                replacement_destination: span(3),
                first: CallbackData {
                    state: ptr::null_mut(),
                    tag: 1,
                },
                second: CallbackData {
                    state: ptr::null_mut(),
                    tag: 2,
                },
                events: "events".into(),
            };
            let p = &raw mut state;
            state.first.state = p;
            state.second.state = p;
            // SAFETY: only the original's reached interface callback/data and
            // ICC-span fields are initialized. Unknown inactive members never
            // become aggregate references. All callback changes retain backing.
            unsafe {
                ptr::addr_of_mut!((*s).icc).write(span(0));
                ptr::addr_of_mut!((*d).icc).write(span(1));
                ptr::addr_of_mut!((*i).set_fields_data).write((&raw mut state.first).cast());
                ptr::addr_of_mut!((*i).set_fields_from_icc).write(Some(callback));
                let context = cms.init.unwrap()(i.cast(), 2, 2, s, d, 80.0);
                let mut out = state.events;
                write!(
                    out,
                    " end {} {} {} {} {} {}",
                    state.calls,
                    u8::from(!context.is_null()),
                    u8::from((*i).set_fields_data == (&raw mut state.second).cast()),
                    u8::from(
                        (*i).set_fields_from_icc
                            .is_some_and(|f| std::ptr::fn_addr_eq(
                                f,
                                callback_second as jxl::jxl::cms_interface::CmsSetFieldsFromIcc
                            ))
                    ),
                    if (*s).icc.size == 0 {
                        "-".into()
                    } else {
                        hex_bytes(std::slice::from_raw_parts((*s).icc.data, (*s).icc.size))
                    },
                    if (*d).icc.size == 0 {
                        "-".into()
                    } else {
                        hex_bytes(std::slice::from_raw_parts((*d).icc.data, (*d).icc.size))
                    }
                )
                .unwrap();
                if !context.is_null() {
                    let values = [0.2_f32, 0.7, 1.2, -0.1, 0.6, 0.3, 0.8, 0.4];
                    let mut result = [f32::from_bits(0xbf654321); 8];
                    write!(
                        out,
                        " {} {} {}",
                        cms.get_src_buf.unwrap()(context, 1) as usize % 128,
                        cms.get_dst_buf.unwrap()(context, 1) as usize % 128,
                        cms.run.unwrap()(context, 1, values.as_ptr(), result.as_mut_ptr(), 2)
                    )
                    .unwrap();
                    for x in values.into_iter().chain(result) {
                        write!(out, " {}", x.to_bits()).unwrap();
                    }
                    cms.destroy.unwrap()(context);
                }
                out
            }
        }
        "default-cms-prefix" => {
            use jxl::jxl::cms::jxl_cms::oracle_profile_predicate;
            use jxl::skcms::profile::{self, IccProfile};
            let which: u32 = number(&mut args);
            let mode: usize = number(&mut args);
            let mut bytes = unhex(args.next().unwrap());
            let size = bytes.len();
            bytes.resize(size + 16, 0);
            let mut profile = MaybeUninit::<IccProfile>::uninit();
            let p = profile.as_mut_ptr();
            let mut backing = MaybeUninit::<[u8; 160]>::uninit();
            let b = backing.as_mut_ptr().cast::<u8>();
            let mut media = MaybeUninit::<[f32; 3]>::uninit();
            let m = media.as_mut_ptr().cast::<f32>();
            let mut adapted = [f32::from_bits(0x3f123456); 3];
            let mut signature = b"chad".as_ptr();
            // SAFETY: fixtures initialize only the source-reached prefixes. No
            // aggregate reference/value is created from the unknown storage.
            unsafe {
                if which == 0 {
                    if mode == 0 {
                        ptr::addr_of_mut!((*p).buffer).write(ptr::null());
                    } else {
                        ptr::addr_of_mut!((*p).buffer).write(b);
                        ptr::addr_of_mut!((*p).size).write(if mode == 1 { 127 } else { 128 });
                        let offsets = [8, 12, 13, 14, 15, 36, 37, 38, 39];
                        let expected = [4, b'm', b'n', b't', b'r', b'a', b'c', b's', b'p'];
                        if mode >= 2 {
                            for i in 0..9 {
                                if i < mode - 2 {
                                    b.add(offsets[i]).write(expected[i]);
                                } else if i == mode - 2 {
                                    b.add(offsets[i]).write(0);
                                    break;
                                }
                            }
                        }
                    }
                } else if which == 1 || which == 2 {
                    if mode == 0 {
                        ptr::addr_of_mut!((*p).buffer).write(ptr::null());
                    } else {
                        ptr::addr_of_mut!((*p).buffer).write(b);
                        ptr::addr_of_mut!((*p).size).write(if mode == 1 {
                            131
                        } else if mode == 2 {
                            132
                        } else {
                            144
                        });
                        if mode >= 2 {
                            let count: u32 = if mode == 2 {
                                0
                            } else if mode == 3 {
                                2
                            } else {
                                1
                            };
                            for (i, v) in count.to_be_bytes().into_iter().enumerate() {
                                b.add(128 + i).write(v);
                            }
                        }
                        if mode == 2 || mode == 3 {
                            signature = ptr::null();
                        }
                        if mode >= 4 {
                            let tag = if mode == 4 {
                                b"chad"
                            } else if mode == 5 {
                                b"A2B0"
                            } else if mode == 6 {
                                b"B2D3"
                            } else {
                                b"ZZZZ"
                            };
                            ptr::copy_nonoverlapping(tag.as_ptr(), b.add(132), 4);
                        }
                    }
                } else if which == 3 {
                    for i in 0..3 {
                        if i < mode {
                            ptr::addr_of_mut!((*p).trc[i].table.table_entries).write(0);
                        } else if i == mode {
                            ptr::addr_of_mut!((*p).trc[i].table.table_entries).write(1);
                            break;
                        }
                    }
                } else if which == 4 || which == 5 {
                    if mode == 0 {
                        if which == 5 {
                            ptr::addr_of_mut!((*p).trc[0].table.table_entries).write(1);
                        } else {
                            ptr::addr_of_mut!((*p).buffer).write(ptr::null());
                        }
                    } else if mode == 1 {
                        for i in 0..3 {
                            ptr::addr_of_mut!((*p).trc[i].table.table_entries).write(0);
                        }
                        ptr::addr_of_mut!((*p).buffer).write(ptr::null());
                    } else {
                        assert!(profile::parse(bytes.as_ptr(), size, p));
                        m.write(if mode == 2 {
                            0.0
                        } else {
                            f32::from_bits(0x3f76d600)
                        });
                        if mode >= 3 {
                            m.add(1).write(if mode == 3 { 0.0 } else { 1.0 });
                        }
                        if mode >= 4 {
                            m.add(2).write(if mode == 4 {
                                0.0
                            } else {
                                f32::from_bits(0x3f532d00)
                            });
                        }
                        if mode >= 6 {
                            m.write(0.95);
                            m.add(1).write(1.0);
                            m.add(2).write(1.09);
                        }
                    }
                } else if which == 6 {
                    m.write(if mode == 0 {
                        0.0
                    } else {
                        f32::from_bits(0x3f76d600)
                    });
                    if mode >= 1 {
                        m.add(1).write(if mode == 1 { 0.0 } else { 1.0 });
                    }
                    if mode >= 2 {
                        m.add(2).write(if mode == 2 {
                            0.0
                        } else {
                            f32::from_bits(0x3f532d00)
                        });
                    }
                } else {
                    panic!("observer predicate");
                }
                let destination = if (which == 4 || which == 5) && mode == 7 {
                    m
                } else {
                    adapted.as_mut_ptr()
                };
                let status = oracle_profile_predicate(which, p, signature, m, destination);
                let mut out = u8::from(status).to_string();
                for i in 0..3 {
                    write!(out, " {}", (*destination.add(i)).to_bits()).unwrap();
                }
                out
            }
        }
        "default-cms-cicp" => {
            use jxl::jxl::cms::color_encoding_cms::{
                ColorEncoding, ColorSpace, CustomXy, Primaries, RenderingIntent, TransferFunction,
                WhitePoint,
            };
            use jxl::jxl::cms::jxl_cms::oracle_apply_cicp;
            let primaries: u8 = number(&mut args);
            let transfer: u8 = number(&mut args);
            let matrix: u8 = number(&mut args);
            let range: u8 = number(&mut args);
            let mode: u32 = number(&mut args);
            let mut c = ColorEncoding {
                white_point: WhitePoint::CUSTOM,
                primaries: Primaries::CUSTOM,
                rendering_intent: RenderingIntent::ABSOLUTE,
                have_fields: false,
                color_space: ColorSpace::GRAY,
                cmyk: true,
                ..ColorEncoding::default()
            };
            c.tf.have_gamma = true;
            c.tf.gamma = 1234567;
            c.tf.transfer_function = TransferFunction::UNKNOWN;
            c.white = CustomXy { x: 123, y: -456 };
            c.red = CustomXy { x: -234, y: 567 };
            c.green = CustomXy { x: 345, y: -678 };
            c.blue = CustomXy { x: -456, y: 789 };
            c.icc = vec![17, 34, 51];
            let mut unknown = MaybeUninit::<ColorEncoding>::uninit();
            let pointer = if mode == 1 {
                ptr::null_mut()
            } else if mode == 2 {
                unknown.as_mut_ptr()
            } else {
                &mut c
            };
            // SAFETY: null controls choose original failed no-access guards.
            // Unknown-object mode observes only the five fields actually written
            // on success and never materializes or drops its inactive vectors.
            let status = unsafe { oracle_apply_cicp(primaries, transfer, matrix, range, pointer) };
            let mut out = u8::from(status).to_string();
            if mode == 2 {
                if status {
                    // SAFETY: source success initializes precisely these fields.
                    unsafe {
                        write!(
                            out,
                            " {} {} {} {} {}",
                            (*pointer).color_space.0,
                            u8::from((*pointer).tf.have_gamma),
                            (*pointer).tf.transfer_function.0,
                            (*pointer).white_point.0,
                            (*pointer).primaries.0
                        )
                        .unwrap();
                    }
                }
            } else {
                write!(
                    out,
                    " {} {} {} {} {} {} {} {} {}",
                    c.white_point.0,
                    c.primaries.0,
                    c.rendering_intent.0,
                    u8::from(c.have_fields),
                    c.color_space.0,
                    u8::from(c.cmyk),
                    u8::from(c.tf.have_gamma),
                    c.tf.gamma,
                    c.tf.transfer_function.0
                )
                .unwrap();
                for xy in [c.white, c.red, c.green, c.blue] {
                    write!(out, " {} {}", xy.x as u32, xy.y as u32).unwrap();
                }
                write!(out, " {}", c.icc.len()).unwrap();
                for x in c.icc {
                    write!(out, " {x}").unwrap();
                }
            }
            out
        }
        "default-cms-tf" => {
            use jxl::jxl::cms::jxl_cms::{OracleTfContext, oracle_tf};
            let operation: u32 = number(&mut args);
            let code: i32 = number(&mut args);
            let mask: u32 = number(&mut args);
            let intensity = f32::from_bits(number(&mut args));
            let apply: u32 = number(&mut args);
            let channels: usize = number(&mut args);
            let forward: u32 = number(&mut args);
            let size: usize = number(&mut args);
            let alias: u32 = number(&mut args);
            let nulls: u32 = number(&mut args);
            let count: usize = number(&mut args);
            let mut luminances = [0.0; 3];
            for x in &mut luminances {
                *x = f32::from_bits(number(&mut args));
            }
            let state = OracleTfContext {
                code,
                mask,
                intensity,
                apply: apply != 0,
                channels,
                luminances,
            };
            let mut input = vec![f32::from_bits(0x3f123456); count + 16];
            let mut output = vec![f32::from_bits(0xbf654321); count + 16];
            for i in 0..count {
                input[i + 1] = f32::from_bits(number(&mut args));
                if operation != 0 {
                    output[i + 1] = input[i + 1];
                }
            }
            // SAFETY: fixture vectors include source-sized accesses and genuine
            // same/shifted aliases. Null controls select source no-access paths.
            let status = unsafe {
                let mut source = input.as_ptr().add(1);
                let mut destination = output.as_mut_ptr().add(1);
                if alias != 0 {
                    destination = input.as_mut_ptr().add(if alias == 1 {
                        1
                    } else if alias == 2 {
                        2
                    } else {
                        0
                    });
                }
                if nulls & 1 != 0 {
                    source = ptr::null();
                }
                if nulls & 2 != 0 {
                    destination = ptr::null_mut();
                }
                oracle_tf(operation, &state, source, destination, size, forward != 0)
            };
            let mut out = format!("{} {}", u8::from(status), input.len());
            for x in input {
                write!(out, " {}", x.to_bits()).unwrap();
            }
            write!(out, " {}", output.len()).unwrap();
            for x in output {
                write!(out, " {}", x.to_bits()).unwrap();
            }
            out
        }
        "default-cms-profile-detail" => {
            use jxl::jxl::cms::color_encoding_cms::{CieXy, ColorEncoding};
            use jxl::jxl::cms::jxl_cms::{
                oracle_detect_transfer_function, oracle_identify_primaries,
                oracle_unadapted_white_point,
            };
            use jxl::jxl::color_encoding::ColorSpace;
            use jxl::skcms::profile::{
                self, IccProfile, SIGNATURE_CMYK, SIGNATURE_GRAY, SIGNATURE_RGB,
            };
            let mut bytes = unhex(args.next().unwrap());
            let size = bytes.len();
            bytes.resize(size + 16, 0);
            let mut profile = IccProfile::zeroed();
            // SAFETY: initialized exact logical ICC backing and full writable output.
            let parsed = unsafe { profile::parse(bytes.as_ptr(), size, &mut profile) };
            let mut out = u8::from(parsed).to_string();
            if !parsed {
                return out;
            }
            let mut c = ColorEncoding {
                color_space: match profile.data_color_space {
                    SIGNATURE_RGB | SIGNATURE_CMYK => ColorSpace::RGB,
                    SIGNATURE_GRAY => ColorSpace::GRAY,
                    _ => ColorSpace::UNKNOWN,
                },
                ..ColorEncoding::default()
            };
            let mut white = CieXy { x: 0.25, y: 0.75 };
            // SAFETY: actual parsed profile, stable backing and initialized local outputs.
            let wp = unsafe { oracle_unadapted_white_point(&profile, &mut white) };
            write!(
                out,
                " {} {} {}",
                u8::from(wp),
                white.x.to_bits(),
                white.y.to_bits()
            )
            .unwrap();
            if !wp {
                return out;
            }
            let set = c.set_white_point(white).is_ok();
            write!(out, " {}", u8::from(set)).unwrap();
            if !set {
                return out;
            }
            // SAFETY: same parsed profile and initialized metadata prerequisites.
            let primaries = unsafe { oracle_identify_primaries(&profile, white, &mut c) };
            write!(out, " {}", u8::from(primaries)).unwrap();
            encoding(&c.to_external(), &mut out);
            if !primaries {
                return out;
            }
            // SAFETY: original ordering after the successful primary step.
            let tf = unsafe { oracle_detect_transfer_function(&profile, &mut c) };
            write!(out, " {}", u8::from(tf)).unwrap();
            encoding(&c.to_external(), &mut out);
            out
        }
        "default-cms-interface" => {
            // SAFETY: source destroy explicitly accepts null without access.
            unsafe {
                cms.destroy.unwrap()(ptr::null_mut());
            }
            format!(
                "{} {} {} {} {} {} {} {} {}",
                u8::from(ptr::eq(cms, get_default_cms())),
                u8::from(ptr::eq(
                    cms.init_data.cast_const(),
                    ptr::from_ref(cms).cast()
                )),
                u8::from(cms.set_fields_data.is_null()),
                u8::from(cms.set_fields_from_icc.is_some()),
                u8::from(cms.init.is_some()),
                u8::from(cms.get_src_buf.is_some()),
                u8::from(cms.get_dst_buf.is_some()),
                u8::from(cms.run.is_some()),
                u8::from(cms.destroy.is_some())
            )
        }
        "default-cms-fields" => {
            let flags: u32 = number(&mut args);
            let mut bytes = unhex(args.next().unwrap());
            let size = bytes.len();
            bytes.resize(size + 16, 0);
            let mut c = MaybeUninit::<JxlColorEncoding>::uninit();
            let mut cmyk = 0x12345678_i32;
            // SAFETY: every field in this external carrier accepts every bit
            // pattern; all bytes are seeded before source partial output writes.
            unsafe {
                ptr::write_bytes(
                    c.as_mut_ptr().cast::<u8>(),
                    0x5a,
                    std::mem::size_of::<JxlColorEncoding>(),
                );
            }
            // SAFETY: generated controls pass null ICC only on source-defined
            // no-access paths. Both output objects are fully writable/seeded.
            let status = unsafe {
                cms.set_fields_from_icc.unwrap()(
                    cms.set_fields_data,
                    if flags & 4 != 0 {
                        ptr::null()
                    } else {
                        bytes.as_ptr()
                    },
                    size,
                    if flags & 1 != 0 {
                        ptr::null_mut()
                    } else {
                        c.as_mut_ptr()
                    },
                    if flags & 2 != 0 {
                        ptr::null_mut()
                    } else {
                        &mut cmyk
                    },
                )
            };
            let mut out = format!("{status} {cmyk}");
            // SAFETY: every original field is either seeded or overwritten.
            let c = unsafe { c.assume_init() };
            encoding(&c, &mut out);
            out
        }
        "default-cms-matrix-product" => {
            let mut matrix = Matrix3x3::default();
            let mut input = [0.0; 3];
            let mut output = [f32::from_bits(0x3f123456); 3];
            for row in &mut matrix.vals {
                for x in row {
                    *x = f32::from_bits(number(&mut args));
                }
            }
            for x in &mut input {
                *x = f32::from_bits(number(&mut args));
            }
            let alias: u32 = number(&mut args);
            let destination = if alias != 0 {
                input.as_mut_ptr()
            } else {
                output.as_mut_ptr()
            };
            // SAFETY: initialized arrays cover every reached word, including
            // actual same-array alias rather than an equivalent copied input.
            unsafe {
                oracle_matrix_product(&matrix, input.as_ptr(), destination);
            }
            let result = if alias != 0 { input } else { output };
            result
                .iter()
                .map(|x| x.to_bits().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }
        "default-cms-row" => {
            let mode: u32 = number(&mut args);
            let threads: usize = number(&mut args);
            let width: usize = number(&mut args);
            let thread: usize = number(&mut args);
            let intensity = f32::from_bits(number(&mut args));
            let source_extent: usize = number(&mut args);
            let destination_extent: usize = number(&mut args);
            let mut source_icc = unhex(args.next().unwrap());
            let source_size = source_icc.len();
            source_icc.resize(source_size + 16, 0);
            let mut destination_icc = unhex(args.next().unwrap());
            let destination_size = destination_icc.len();
            destination_icc.resize(destination_size + 16, 0);
            let count: usize = number(&mut args);
            let mut source = MaybeUninit::<JxlColorProfile>::uninit();
            let mut destination = MaybeUninit::<JxlColorProfile>::uninit();
            // SAFETY: initialize only actual ICC prefixes. The remaining source
            // descriptor fields deliberately stay genuinely unknown throughout.
            unsafe {
                ptr::addr_of_mut!((*source.as_mut_ptr()).icc.data).write(source_icc.as_ptr());
                ptr::addr_of_mut!((*source.as_mut_ptr()).icc.size).write(source_size);
                ptr::addr_of_mut!((*destination.as_mut_ptr()).icc.data)
                    .write(destination_icc.as_ptr());
                ptr::addr_of_mut!((*destination.as_mut_ptr()).icc.size).write(destination_size);
            }
            // SAFETY: source-owned bytes outlive the context and its destruction;
            // null init data exits before either null descriptor is accessed.
            let context = unsafe {
                cms.init.unwrap()(
                    if mode & 8 != 0 {
                        ptr::null_mut()
                    } else {
                        cms.init_data
                    },
                    threads,
                    width,
                    if mode & 8 != 0 {
                        ptr::null()
                    } else {
                        source.as_ptr()
                    },
                    if mode & 8 != 0 {
                        ptr::null()
                    } else {
                        destination.as_ptr()
                    },
                    intensity,
                )
            };
            let mut out = u8::from(!context.is_null()).to_string();
            if context.is_null() {
                return out;
            }
            let mut input = vec![f32::from_bits(0x3f123456); (width * 4 + 16).max(count)];
            let mut output = vec![f32::from_bits(0xbf654321); width * 4 + 16];
            for x in input.iter_mut().take(count) {
                *x = f32::from_bits(number(&mut args));
            }
            write!(out, " {threads}").unwrap();
            let mut first_src = ptr::null_mut::<f32>();
            let mut first_dst = ptr::null_mut::<f32>();
            // SAFETY: corpus threads/extents are within the actual source
            // allocation. No Rust reference borrows interior context samples.
            unsafe {
                for i in 0..threads {
                    let s = cms.get_src_buf.unwrap()(context, i);
                    let d = cms.get_dst_buf.unwrap()(context, i);
                    if i == 0 {
                        first_src = s;
                        first_dst = d;
                    }
                    write!(
                        out,
                        " {} {} {} {} {} {} {}",
                        u8::from(!s.is_null()),
                        u8::from(!d.is_null()),
                        s as usize % 128,
                        d as usize % 128,
                        (s as usize).wrapping_sub(first_src as usize),
                        (d as usize).wrapping_sub(first_dst as usize),
                        u8::from(s == d)
                    )
                    .unwrap();
                }
                if threads != 0 {
                    let s = cms.get_src_buf.unwrap()(context, thread);
                    let d = cms.get_dst_buf.unwrap()(context, thread);
                    if mode & 1 != 0 {
                        ptr::copy_nonoverlapping(input.as_ptr(), s, count);
                    }
                    let src = if mode & 1 != 0 {
                        s.cast_const()
                    } else {
                        input.as_ptr()
                    };
                    let mut dst = if mode & 2 != 0 {
                        d
                    } else {
                        output.as_mut_ptr()
                    };
                    if mode & 4 != 0 {
                        dst = src.cast_mut();
                    }
                    write!(
                        out,
                        " {}",
                        cms.run.unwrap()(context, thread, src, dst, width)
                    )
                    .unwrap();
                    write!(out, " {}", input.len()).unwrap();
                    for x in &input {
                        write!(out, " {}", x.to_bits()).unwrap();
                    }
                    write!(out, " {}", output.len()).unwrap();
                    for x in &output {
                        write!(out, " {}", x.to_bits()).unwrap();
                    }
                    for i in 0..threads {
                        let s = cms.get_src_buf.unwrap()(context, i);
                        let d = cms.get_dst_buf.unwrap()(context, i);
                        write!(out, " {source_extent}").unwrap();
                        for j in 0..source_extent {
                            write!(out, " {}", (*s.add(j)).to_bits()).unwrap();
                        }
                        write!(out, " {destination_extent}").unwrap();
                        for j in 0..destination_extent {
                            write!(out, " {}", (*d.add(j)).to_bits()).unwrap();
                        }
                    }
                }
                cms.destroy.unwrap()(context);
            }
            out
        }
        _ => panic!("unknown default CMS command: {command}"),
    }
}
