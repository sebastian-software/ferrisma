//! Memory probe for the lossless JPEG XL encode of issue #223 (2026-11-15).
//! Copy into `codecs/libjxl-rs/examples/` to build (see `../README.md`).
//!
//! `fixed <in.ppm|in.pam> [out.jxl]`: the fixed one-shot entry point
//! `LosslessThunderConfig::encode_pixels` (buffering 0), the engine sink's
//! profile before this change.
//! `api <in.ppm|in.pam> <buffering> [out.jxl]`: the public encoder API with
//! `JxlEncoderSetFrameLossless`, effort 2, `uses_original_profile`, enum sRGB
//! (`JxlColorEncodingSetToSRGB`), `JxlEncoderAddImageFrame` (the encoder
//! copies the frame) and `JxlEncoderProcessOutput` into a growing vector
//! (first window 64 KiB, then doubling: the drain of `ferrisma-jxl-encoder`).
//! `buffering` -1 leaves `JXL_ENC_FRAME_SETTING_BUFFERING` at its default,
//! which is what `cjxl` does without `--buffering`.
//!
//! Input: binary PPM (P6, RGB) or PAM (P7, `RGB_ALPHA`), maxval 255. The
//! counting global allocator reports the peak of live heap bytes
//! (`heap_peak_bytes`) over the whole process, including the input file and
//! the output vector; peak RSS comes from `/usr/bin/time -l` around a fresh
//! process.
#![allow(unsafe_code)]
use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::encode::{self as api, JxlEncoderFrameSettingId, JxlEncoderStatus};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counting;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

// SAFETY: Delegates every call to the system allocator unchanged.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(live, Ordering::Relaxed);
        // SAFETY: Same layout contract as the caller's.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: Same pointer/layout pair the caller allocated.
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(live, Ordering::Relaxed);
        // SAFETY: Same layout contract as the caller's.
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // Count a reallocation as allocate-then-free, its transient peak.
        let live = LIVE.fetch_add(new_size, Ordering::Relaxed) + new_size;
        PEAK.fetch_max(live, Ordering::Relaxed);
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: Same contract as the caller's.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

/// (width, height, channels, pixels) of a P6 PPM or a P7 RGB_ALPHA PAM.
fn read_image(path: &str) -> (usize, usize, usize, Vec<u8>) {
    let data = std::fs::read(path).unwrap();
    let text_end = |marker: &[u8]| {
        data.windows(marker.len())
            .position(|w| w == marker)
            .map(|p| p + marker.len())
    };
    if data.starts_with(b"P7") {
        let end = text_end(b"ENDHDR\n").unwrap();
        let header = std::str::from_utf8(&data[..end]).unwrap();
        let field = |name: &str| -> usize {
            header
                .lines()
                .find_map(|line| line.strip_prefix(name))
                .unwrap()
                .trim()
                .parse()
                .unwrap()
        };
        let (w, h, d) = (field("WIDTH "), field("HEIGHT "), field("DEPTH "));
        assert_eq!(field("MAXVAL "), 255);
        assert_eq!(d, 4);
        let pixels = data[end..].to_vec();
        assert_eq!(pixels.len(), w * h * d);
        return (w, h, d, pixels);
    }
    let mut fields = Vec::new();
    let mut pos = 0;
    while fields.len() < 4 {
        while data[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if data[pos] == b'#' {
            while data[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }
        let start = pos;
        while !data[pos].is_ascii_whitespace() {
            pos += 1;
        }
        fields.push(String::from_utf8(data[start..pos].to_vec()).unwrap());
    }
    assert_eq!(fields[0], "P6");
    assert_eq!(fields[3], "255");
    let (w, h): (usize, usize) = (fields[1].parse().unwrap(), fields[2].parse().unwrap());
    let pixels = data[pos + 1..].to_vec();
    assert_eq!(pixels.len(), w * h * 3);
    (w, h, 3, pixels)
}

fn ok(status: JxlEncoderStatus) {
    assert_eq!(status, JxlEncoderStatus::SUCCESS);
}

fn encode_api(w: usize, h: usize, channels: usize, pixels: &[u8], buffering: i64) -> Vec<u8> {
    // SAFETY: Null manager selects the default allocator; destroyed below.
    let enc = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    assert!(!enc.is_null());
    // SAFETY: `enc` is live and uniquely used in this function.
    let e = unsafe { &mut *enc };
    let mut info = JxlBasicInfo::zeroed();
    api::jxl_encoder_init_basic_info(&mut info);
    info.xsize = w as u32;
    info.ysize = h as u32;
    info.bits_per_sample = 8;
    info.num_color_channels = 3;
    info.uses_original_profile = 1;
    if channels == 4 {
        info.num_extra_channels = 1;
        info.alpha_bits = 8;
    }
    ok(api::jxl_encoder_set_basic_info(e, &info));
    let mut color = JxlColorEncoding::default();
    api::jxl_color_encoding_set_to_srgb(&mut color, 0);
    ok(api::jxl_encoder_set_color_encoding(e, &color));
    // SAFETY: Live encoder; the settings are owned by it.
    let settings = unsafe { api::jxl_encoder_frame_settings_create(enc, std::ptr::null()) };
    assert!(!settings.is_null());
    // SAFETY: Live settings of a live encoder.
    unsafe {
        ok(api::jxl_encoder_frame_settings_set_option(
            settings,
            JxlEncoderFrameSettingId::EFFORT,
            2,
        ));
        if buffering != -1 {
            ok(api::jxl_encoder_frame_settings_set_option(
                settings,
                JxlEncoderFrameSettingId::BUFFERING,
                buffering,
            ));
        }
        ok(api::jxl_encoder_set_frame_lossless(settings, 1));
    }
    let format = JxlPixelFormat {
        num_channels: channels as u32,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    // SAFETY: Live settings; the buffer outlives the call (it is copied).
    ok(unsafe { api::jxl_encoder_add_image_frame(settings, &format, Some(pixels)) });
    api::jxl_encoder_close_input(e);
    let mut out: Vec<u8> = Vec::new();
    loop {
        let room = out.len().max(64 * 1024);
        out.reserve(room);
        let mut next = out.spare_capacity_mut().as_mut_ptr().cast::<u8>();
        let mut avail = room;
        // SAFETY: `next` points to `room` writable bytes of spare capacity.
        let status = unsafe { api::jxl_encoder_process_output(&mut *enc, &mut next, &mut avail) };
        // SAFETY: The encoder initialized the first `room - avail` bytes.
        unsafe { out.set_len(out.len() + room - avail) };
        if status == JxlEncoderStatus::NEED_MORE_OUTPUT {
            continue;
        }
        ok(status);
        break;
    }
    // SAFETY: Not used again.
    unsafe { api::jxl_encoder_destroy(enc) };
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h, channels, pixels) = read_image(&args[2]);
    let (encoded, out_path) = match args[1].as_str() {
        "fixed" => {
            use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
            let mut config = if channels == 4 {
                LosslessThunderConfig::new_with_alpha(w, h, false, LosslessSampleDepth::Uint8)
            } else {
                LosslessThunderConfig::new(w, h, false, LosslessSampleDepth::Uint8)
            }
            .unwrap();
            let encoded = config.encode_pixels(&pixels, None).unwrap();
            (encoded.as_slice().to_vec(), args.get(3))
        }
        "api" => {
            let buffering: i64 = args[3].parse().unwrap();
            (encode_api(w, h, channels, &pixels, buffering), args.get(4))
        }
        _ => panic!("mode"),
    };
    println!("pixels={} channels={channels} bytes={}", w * h, encoded.len());
    if let Some(path) = out_path {
        std::fs::write(path, &encoded).unwrap();
    }
    println!("heap_peak_bytes={}", PEAK.load(Ordering::Relaxed));
}
