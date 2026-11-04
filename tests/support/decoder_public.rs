// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observation driver for the Rust public decoder. It mirrors
// tools/oracle/decoder_public.cc token for token; it calls the translated
// public entry points and implements no decoder or container algorithm.
#![allow(dead_code)]
use jxl::jxl::codestream_header::{JxlBasicInfo, JxlExtraChannelInfo};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::decode::{self as api, JxlColorProfileTarget, JxlDecoder, JxlDecoderStatus};
use jxl::jxl::memory_manager::JxlMemoryManager;
use std::alloc::{Layout, alloc, dealloc};
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::fmt::Write as _;

pub fn hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "-".into();
    }
    let mut result = String::with_capacity(data.len() * 2);
    for byte in data {
        write!(result, "{byte:02x}").unwrap();
    }
    result
}

pub fn unhex(data: &str) -> Vec<u8> {
    if data == "-" {
        return Vec::new();
    }
    (0..data.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&data[i..i + 2], 16).unwrap())
        .collect()
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

struct Allocator {
    calls: usize,
    fail: usize,
    next: usize,
    live: BTreeMap<usize, (usize, Layout)>,
    events: Vec<String>,
}

unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The driver retains this allocator for the decoder's lifetime and
    // callbacks never run concurrently.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    a.calls += 1;
    // The first request is the decoder object, whose size is a property of the
    // language layout rather than of the decoder algorithm.
    a.events.push(if a.calls == 1 {
        "Adec".into()
    } else {
        format!("A{size}")
    });
    if a.calls == a.fail {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: Nonzero checked layout, retained for the matching free.
    let p = unsafe { alloc(layout) };
    if !p.is_null() {
        a.next += 1;
        a.live.insert(p as usize, (a.next, layout));
    }
    p.cast()
}

unsafe extern "C" fn release(opaque: *mut c_void, p: *mut c_void) {
    // SAFETY: As in allocate.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    if p.is_null() {
        a.events.push("F0".into());
        return;
    }
    let (id, layout) = a.live.remove(&(p as usize)).unwrap();
    a.events.push(format!("F{id}"));
    // SAFETY: Unique callback-returned pointer and its saved layout.
    unsafe { dealloc(p.cast(), layout) };
}

fn f32_bits(value: f32) -> u32 {
    value.to_bits()
}

fn f64_bits(value: f64) -> u64 {
    value.to_bits()
}

/// cs.wp.prim.tf.gamma-bits.ri, with fixed custom white point and primaries.
fn parse_encoding(token: &str) -> JxlColorEncoding {
    use jxl::jxl::color_encoding::{
        ColorSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
    };
    let fields: Vec<u64> = token.split('.').map(|v| v.parse().unwrap()).collect();
    JxlColorEncoding {
        color_space: ColorSpace(fields[0] as u32),
        white_point: WhitePoint(fields[1] as u32),
        white_point_xy: [0.3127, 0.329],
        primaries: Primaries(fields[2] as u32),
        primaries_red_xy: [0.68, 0.32],
        primaries_green_xy: [0.265, 0.69],
        primaries_blue_xy: [0.15, 0.06],
        transfer_function: TransferFunction(fields[3] as u32),
        gamma: f64::from_bits(fields[4]),
        rendering_intent: RenderingIntent(fields[5] as u32),
    }
}

pub struct Driver {
    file: Vec<u8>,
    out: String,
    allocator: Box<Allocator>,
    dec: *mut JxlDecoder,
    pos: usize,
    set_len: usize,
    closed: bool,
    box_set: bool,
    box_buffer: Vec<u8>,
    jpeg_buffer: Vec<u8>,
    print_events: bool,
}

impl Drop for Driver {
    fn drop(&mut self) {
        if !self.dec.is_null() {
            // SAFETY: The driver's own live decoder.
            unsafe { api::jxl_decoder_destroy(self.dec) };
        }
    }
}

impl Driver {
    pub fn new(file: Vec<u8>) -> Self {
        Self {
            file,
            out: String::new(),
            allocator: Box::new(Allocator {
                calls: 0,
                fail: 0,
                next: 0,
                live: BTreeMap::new(),
                events: Vec::new(),
            }),
            dec: std::ptr::null_mut(),
            pos: 0,
            set_len: 0,
            closed: false,
            box_set: false,
            box_buffer: Vec::new(),
            jpeg_buffer: Vec::new(),
            print_events: false,
        }
    }

    fn dec(&mut self) -> &mut JxlDecoder {
        // SAFETY: Created before any operation; no other reference is live.
        unsafe { &mut *self.dec }
    }

    /// mm 0: default manager; 1: custom manager with its event log; 2: custom
    /// manager without the log, for owners whose allocation paths differ.
    pub fn create(&mut self, mm: i32, fail: usize) -> bool {
        self.allocator.fail = fail;
        self.print_events = mm == 1;
        let manager = JxlMemoryManager {
            opaque: std::ptr::from_mut(&mut *self.allocator).cast(),
            alloc: Some(allocate),
            free: Some(release),
        };
        // SAFETY: The boxed allocator outlives the decoder; callbacks obey the
        // memory-manager contract and return 16-byte aligned blocks.
        self.dec =
            unsafe { api::jxl_decoder_create(if mm != 0 { &manager } else { std::ptr::null() }) };
        write!(self.out, " create:{}", i32::from(!self.dec.is_null())).unwrap();
        !self.dec.is_null()
    }

    /// The original std::map destroys its nodes in an unspecified,
    /// standard-library-dependent order. Releases within one destruction
    /// checkpoint are compared in ascending allocation order, as the oracle
    /// observer does; the decoder object's own release stays last.
    fn sort_releases(&mut self, begin: usize, end: usize) {
        self.allocator.events[begin..end].sort_by_key(|e| e[1..].parse::<usize>().unwrap());
    }

    pub fn destroy(&mut self) -> String {
        let begin = self.allocator.events.len();
        // SAFETY: The driver's own live decoder, not used afterwards.
        unsafe { api::jxl_decoder_destroy(self.dec) };
        let end = self.allocator.events.len();
        if end > begin {
            self.sort_releases(begin, end - 1);
        }
        self.dec = std::ptr::null_mut();
        self.out.push_str(" E");
        if self.print_events {
            for e in &self.allocator.events {
                write!(self.out, " {e}").unwrap();
            }
        }
        std::mem::take(&mut self.out)
    }

    pub fn op(&mut self, op: &str) {
        let kind = op.as_bytes()[0] as char;
        let arg = &op[1..];
        write!(self.out, " {kind}:").unwrap();
        let int = |s: &str| s.parse::<i32>().unwrap();
        match kind {
            's' => {
                let s = api::jxl_decoder_subscribe_events(self.dec(), int(arg));
                self.status(s);
            }
            'k' => {
                let s = api::jxl_decoder_set_keep_orientation(self.dec(), int(arg));
                self.status(s);
            }
            'u' => {
                let s = api::jxl_decoder_set_unpremultiply_alpha(self.dec(), int(arg));
                self.status(s);
            }
            'r' => {
                let s = api::jxl_decoder_set_render_spotcolors(self.dec(), int(arg));
                self.status(s);
            }
            'g' => {
                let s = api::jxl_decoder_set_coalescing(self.dec(), int(arg));
                self.status(s);
            }
            'd' => {
                let s = api::jxl_decoder_set_decompress_boxes(self.dec(), int(arg));
                self.status(s);
            }
            'I' => self.set_input(arg),
            'i' => {
                let r = self.release();
                write!(self.out, "{r}").unwrap();
            }
            'c' => {
                api::jxl_decoder_close_input(self.dec());
                self.closed = true;
                self.out.push('0');
            }
            'p' => {
                // SAFETY: Input and box buffers are driver-owned and live.
                let s = unsafe { api::jxl_decoder_process_input(self.dec()) };
                self.status(s);
            }
            'h' => {
                let h = api::jxl_decoder_size_hint_basic_info(self.dec());
                write!(self.out, "{h}").unwrap();
            }
            'b' => self.basic_info(arg != "0"),
            'e' => self.extra_info(arg.parse().unwrap()),
            'n' => self.extra_name(arg),
            'E' => self.encoded(arg),
            'z' => self.icc_size(arg),
            'Z' => self.icc(arg),
            't' => self.box_type(int(arg)),
            'y' => self.box_size(false, arg != "0"),
            'Y' => self.box_size(true, arg != "0"),
            'O' => self.set_box(arg.parse().unwrap()),
            'o' => self.release_box(),
            'w' => {
                let begin = self.allocator.events.len();
                api::jxl_decoder_rewind(self.dec());
                let end = self.allocator.events.len();
                self.sort_releases(begin, end);
                self.restart();
                self.out.push('0');
            }
            'x' => {
                let begin = self.allocator.events.len();
                api::jxl_decoder_reset(self.dec());
                let end = self.allocator.events.len();
                self.sort_releases(begin, end);
                self.restart();
                self.out.push('0');
            }
            'C' => {
                // SAFETY: The translated default CMS is a static interface.
                let s = unsafe {
                    api::jxl_decoder_set_cms(self.dec(), *jxl::jxl::cms::jxl_cms::get_default_cms())
                };
                self.status(s);
            }
            'T' => {
                let target = f32::from_bits(arg.parse().unwrap());
                let s = api::jxl_decoder_set_desired_intensity_target(self.dec(), target);
                self.status(s);
            }
            'P' => {
                let s = if arg == "-" {
                    api::jxl_decoder_set_preferred_color_profile(self.dec(), None)
                } else {
                    let color = parse_encoding(arg);
                    api::jxl_decoder_set_preferred_color_profile(self.dec(), Some(&color))
                };
                self.status(s);
            }
            'Q' => {
                let mut icc = unhex(arg);
                icc.push(0); // A non-null pointer for an empty profile.
                // SAFETY: The vector holds the profile bytes; the CMS is static.
                let s = unsafe {
                    api::jxl_decoder_set_output_color_profile(
                        self.dec(),
                        None,
                        icc.as_ptr(),
                        icc.len() - 1,
                    )
                };
                self.status(s);
            }
            'R' => {
                let (encoding, icc) = arg.split_once(',').unwrap();
                let color = parse_encoding(encoding);
                let mut icc = unhex(icc);
                icc.push(0);
                // SAFETY: As for 'Q'.
                let s = unsafe {
                    api::jxl_decoder_set_output_color_profile(
                        self.dec(),
                        Some(&color),
                        icc.as_ptr(),
                        icc.len() - 1,
                    )
                };
                self.status(s);
            }
            'J' => self.set_jpeg(arg.parse().unwrap()),
            'j' => self.release_jpeg(),
            'G' => {
                self.out.push('0');
                self.info();
                self.color();
            }
            'L' => self.run_loop(arg),
            _ => panic!("decoder public op"),
        }
    }

    fn status(&mut self, status: JxlDecoderStatus) {
        write!(self.out, "{}", status.0).unwrap();
    }

    fn restart(&mut self) {
        self.pos = 0;
        self.set_len = 0;
        self.closed = false;
        if !self.box_buffer.is_empty() || self.box_set {
            write!(self.out, "{},", hex(&self.box_buffer)).unwrap();
            self.box_buffer.clear();
            self.box_set = false;
        }
        // Rewind and Reset forget the JPEG output buffer.
        if !self.jpeg_buffer.is_empty() {
            write!(self.out, "{},", hex(&self.jpeg_buffer)).unwrap();
            self.jpeg_buffer.clear();
        }
    }

    fn set_jpeg(&mut self, size: usize) {
        let mut buffer = vec![0xA5u8; size + 1];
        let data = buffer.as_mut_ptr();
        // SAFETY: The buffer's heap storage stays live and unmoved in the
        // driver until it is released or forgotten by Rewind/Reset.
        let status = unsafe { api::jxl_decoder_set_jpeg_buffer(self.dec(), data, size) };
        self.status(status);
        if status == JxlDecoderStatus::SUCCESS {
            self.jpeg_buffer = buffer;
        }
    }

    fn release_jpeg(&mut self) {
        let remaining = api::jxl_decoder_release_jpeg_buffer(self.dec());
        write!(self.out, "{},{}", remaining, hex(&self.jpeg_buffer)).unwrap();
        self.jpeg_buffer.clear();
    }

    fn set_input(&mut self, arg: &str) {
        if arg == "-" {
            // SAFETY: A null empty input is the source's distinct identity.
            let s = unsafe { api::jxl_decoder_set_input(self.dec(), std::ptr::null(), 0) };
            self.status(s);
            return;
        }
        let (off, len) = arg.split_once(',').unwrap();
        let (off, len): (usize, usize) = (off.parse().unwrap(), len.parse().unwrap());
        assert!(
            off <= self.file.len() && len <= self.file.len() - off,
            "input range"
        );
        let data = self.file[off..].as_ptr();
        // SAFETY: The driver's file outlives the decoder and is not mutated.
        let status = unsafe { api::jxl_decoder_set_input(self.dec(), data, len) };
        if status == JxlDecoderStatus::SUCCESS {
            self.pos = off;
            self.set_len = len;
        }
        self.status(status);
    }

    fn release(&mut self) -> usize {
        let remaining = api::jxl_decoder_release_input(self.dec());
        self.pos = self.pos + self.set_len - remaining;
        self.set_len = 0;
        remaining
    }

    fn basic_info(&mut self, present: bool) {
        let mut info = JxlBasicInfo::zeroed();
        // SAFETY: Fill the plain-data output with the oracle's sentinel bytes.
        unsafe {
            std::ptr::write_bytes(
                std::ptr::from_mut(&mut info).cast::<u8>(),
                0xA5,
                size_of::<JxlBasicInfo>(),
            );
        }
        let status = api::jxl_decoder_get_basic_info(
            self.dec(),
            if present { Some(&mut info) } else { None },
        );
        self.status(status);
        if status != JxlDecoderStatus::SUCCESS || !present {
            return;
        }
        write!(
            self.out,
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            info.have_container,
            info.xsize,
            info.ysize,
            info.bits_per_sample,
            info.exponent_bits_per_sample,
            f32_bits(info.intensity_target),
            f32_bits(info.min_nits),
            info.relative_to_max_display,
            f32_bits(info.linear_below),
            info.uses_original_profile,
            info.have_preview,
            info.have_animation,
            info.orientation.0,
            info.num_color_channels,
            info.num_extra_channels,
            info.alpha_bits,
            info.alpha_exponent_bits,
            info.alpha_premultiplied,
            info.preview.xsize,
            info.preview.ysize,
            info.animation.tps_numerator,
            info.animation.tps_denominator,
            info.animation.num_loops,
            info.animation.have_timecodes,
            info.intrinsic_xsize,
            info.intrinsic_ysize,
            hex(&info.padding),
        )
        .unwrap();
    }

    fn extra_info(&mut self, index: usize) {
        let mut info = JxlExtraChannelInfo::default();
        // SAFETY: Fill the plain-data output with the oracle's sentinel bytes.
        unsafe {
            std::ptr::write_bytes(
                std::ptr::from_mut(&mut info).cast::<u8>(),
                0xA5,
                size_of::<JxlExtraChannelInfo>(),
            );
        }
        let status = api::jxl_decoder_get_extra_channel_info(self.dec(), index, &mut info);
        self.status(status);
        if status != JxlDecoderStatus::SUCCESS {
            return;
        }
        write!(
            self.out,
            ",{},{},{},{},{},{}",
            info.channel_type.0,
            info.bits_per_sample,
            info.exponent_bits_per_sample,
            info.dim_shift,
            info.name_length,
            info.alpha_premultiplied
        )
        .unwrap();
        for v in info.spot_color {
            write!(self.out, ",{}", f32_bits(v)).unwrap();
        }
        write!(self.out, ",{}", info.cfa_channel).unwrap();
    }

    fn extra_name(&mut self, arg: &str) {
        let (index, size) = arg.split_once(',').unwrap();
        let (index, size): (usize, usize) = (index.parse().unwrap(), size.parse().unwrap());
        let mut name = vec![0xA5u8; size + 1];
        // SAFETY: The buffer holds size + 1 writable bytes.
        let status = unsafe {
            api::jxl_decoder_get_extra_channel_name(self.dec(), index, name.as_mut_ptr(), size)
        };
        write!(self.out, "{},{}", status.0, hex(&name[..size])).unwrap();
    }

    fn encoded(&mut self, arg: &str) {
        let present = arg.len() < 2;
        let target = JxlColorProfileTarget(u32::from(arg.as_bytes()[0] - b'0'));
        let mut color = JxlColorEncoding::default();
        // SAFETY: Fill the plain-data output with the oracle's sentinel bytes.
        unsafe {
            std::ptr::write_bytes(
                std::ptr::from_mut(&mut color).cast::<u8>(),
                0xA5,
                size_of::<JxlColorEncoding>(),
            );
        }
        let status = api::jxl_decoder_get_color_as_encoded_profile(
            self.dec(),
            target,
            if present { Some(&mut color) } else { None },
        );
        self.status(status);
        if status != JxlDecoderStatus::SUCCESS || !present {
            return;
        }
        write!(
            self.out,
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            color.color_space.0,
            color.white_point.0,
            f64_bits(color.white_point_xy[0]),
            f64_bits(color.white_point_xy[1]),
            color.primaries.0,
            f64_bits(color.primaries_red_xy[0]),
            f64_bits(color.primaries_red_xy[1]),
            f64_bits(color.primaries_green_xy[0]),
            f64_bits(color.primaries_green_xy[1]),
            f64_bits(color.primaries_blue_xy[0]),
            f64_bits(color.primaries_blue_xy[1]),
            color.transfer_function.0,
            f64_bits(color.gamma),
            color.rendering_intent.0
        )
        .unwrap();
    }

    fn icc_size(&mut self, arg: &str) {
        let present = arg.len() < 2;
        let target = JxlColorProfileTarget(u32::from(arg.as_bytes()[0] - b'0'));
        let mut size = 0xA5A5usize;
        let status = api::jxl_decoder_get_icc_profile_size(
            self.dec(),
            target,
            if present { Some(&mut size) } else { None },
        );
        write!(self.out, "{},{}", status.0, size).unwrap();
    }

    fn icc(&mut self, arg: &str) {
        let (target, size) = arg.split_once(',').unwrap();
        let target = JxlColorProfileTarget(target.parse().unwrap());
        let size: usize = size.parse().unwrap();
        let mut icc = vec![0xA5u8; size + 1];
        // SAFETY: The buffer holds size + 1 writable bytes.
        let status = unsafe {
            api::jxl_decoder_get_color_as_icc_profile(self.dec(), target, icc.as_mut_ptr(), size)
        };
        write!(self.out, "{},{}", status.0, hex(&icc)).unwrap();
    }

    fn box_type(&mut self, decompressed: i32) {
        let mut box_type = *b"????";
        let status = api::jxl_decoder_get_box_type(self.dec(), &mut box_type, decompressed);
        write!(self.out, "{},{}", status.0, hex(&box_type)).unwrap();
    }

    fn box_size(&mut self, contents: bool, present: bool) {
        let mut size = 0xA5A5u64;
        let out = if present { Some(&mut size) } else { None };
        let status = if contents {
            api::jxl_decoder_get_box_size_contents(self.dec(), out)
        } else {
            api::jxl_decoder_get_box_size_raw(self.dec(), out)
        };
        write!(self.out, "{},{}", status.0, size).unwrap();
    }

    fn set_box(&mut self, size: usize) {
        let mut buffer = vec![0xA5u8; size + 1];
        let data = buffer.as_mut_ptr();
        // SAFETY: The buffer's heap storage stays live and unmoved in the
        // driver until it is released or replaced after a release.
        let status = unsafe { api::jxl_decoder_set_box_buffer(self.dec(), data, size) };
        self.status(status);
        if status == JxlDecoderStatus::SUCCESS {
            self.box_buffer = buffer;
            self.box_set = true;
        }
    }

    fn release_box(&mut self) {
        let remaining = api::jxl_decoder_release_box_buffer(self.dec());
        write!(self.out, "{},{}", remaining, hex(&self.box_buffer)).unwrap();
        self.box_buffer.clear();
        self.box_set = false;
    }

    fn info(&mut self) {
        self.out.push(' ');
        self.basic_info(true);
        let mut info = JxlBasicInfo::zeroed();
        if api::jxl_decoder_get_basic_info(self.dec(), Some(&mut info)) != JxlDecoderStatus::SUCCESS
        {
            return;
        }
        for i in 0..info.num_extra_channels as usize {
            self.out.push_str(" e:");
            self.extra_info(i);
            let mut ec = JxlExtraChannelInfo::default();
            if api::jxl_decoder_get_extra_channel_info(self.dec(), i, &mut ec)
                == JxlDecoderStatus::SUCCESS
            {
                self.out.push_str(" n:");
                self.extra_name(&format!("{i},{}", ec.name_length + 1));
            }
        }
    }

    fn color(&mut self) {
        for target in 0..2u32 {
            self.out.push_str(" E:");
            self.encoded(&target.to_string());
            self.out.push_str(" z:");
            self.icc_size(&target.to_string());
            let mut size = 0usize;
            if api::jxl_decoder_get_icc_profile_size(
                self.dec(),
                JxlColorProfileTarget(target),
                Some(&mut size),
            ) == JxlDecoderStatus::SUCCESS
            {
                self.out.push_str(" Z:");
                self.icc(&format!("{target},{size}"));
            }
        }
    }

    fn run_loop(&mut self, arg: &str) {
        let fields: Vec<u64> = arg.split(',').map(|v| v.parse().unwrap()).collect();
        let (mut seed, max, boxbuf, steps, close) = (
            fields[0],
            fields[1],
            fields[2] as usize,
            fields[3],
            fields[4],
        );
        assert!(max != 0, "loop arguments");
        self.out.push('0');
        for _ in 0..steps {
            // SAFETY: Input and box buffers are driver-owned and live.
            let status = unsafe { api::jxl_decoder_process_input(self.dec()) };
            write!(self.out, " P{}", status.0).unwrap();
            if status == JxlDecoderStatus::NEED_MORE_INPUT {
                let remaining = self.release();
                write!(self.out, ",{remaining}").unwrap();
                let start = self.pos;
                if start >= self.file.len() {
                    if close != 0 && !self.closed {
                        api::jxl_decoder_close_input(self.dec());
                        self.closed = true;
                        self.out.push_str(",c");
                        continue;
                    }
                    break;
                }
                let chunk = 1 + (next(&mut seed) % max) as usize;
                let len = (self.file.len() - start).min(remaining + chunk);
                let data = self.file[start..].as_ptr();
                // SAFETY: The driver's file outlives the decoder.
                let set = unsafe { api::jxl_decoder_set_input(self.dec(), data, len) };
                if set == JxlDecoderStatus::SUCCESS {
                    self.set_len = len;
                }
                write!(self.out, ",{},{}", set.0, len).unwrap();
            } else if status == JxlDecoderStatus::BOX {
                self.out.push(' ');
                self.box_type(0);
                self.out.push(' ');
                self.box_type(1);
                self.out.push(' ');
                self.box_size(false, true);
                self.out.push(' ');
                self.box_size(true, true);
                if boxbuf != 0 {
                    if self.box_set {
                        self.out.push_str(" o:");
                        self.release_box();
                    }
                    self.out.push_str(" O:");
                    self.set_box(boxbuf);
                }
            } else if status == JxlDecoderStatus::BOX_NEED_MORE_OUTPUT {
                self.out.push_str(" o:");
                self.release_box();
                self.out.push_str(" O:");
                self.set_box(if boxbuf != 0 { boxbuf } else { 1 });
            } else if status == JxlDecoderStatus::BOX_COMPLETE {
                if self.box_set {
                    self.out.push_str(" o:");
                    self.release_box();
                }
            } else if status == JxlDecoderStatus::BASIC_INFO {
                self.info();
            } else if status == JxlDecoderStatus::COLOR_ENCODING {
                self.color();
            } else if status == JxlDecoderStatus::JPEG_RECONSTRUCTION {
                // The event carries no getter; decoding continues.
            } else {
                if status == JxlDecoderStatus::SUCCESS && self.box_set {
                    self.out.push_str(" o:");
                    self.release_box();
                }
                break;
            }
        }
    }
}

/// One `dpub-run` request: the same observation string as the oracle.
pub fn observe(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mm: i32 = args.next().unwrap().parse().unwrap();
    let fail: usize = args.next().unwrap().parse().unwrap();
    let file = unhex(args.next().unwrap());
    let mut driver = Driver::new(file);
    if driver.create(mm, fail) {
        for op in args {
            driver.op(op);
        }
        return driver.destroy();
    }
    std::mem::take(&mut driver.out)
}
