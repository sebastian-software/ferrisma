//! Profiling probe for the JPEG XL performance analysis (2026-11-12).
//! `decode <in.jxl> <iterations>`: full upright F32 decode through the wrapper.
//! `lossy <in.ppm> <effort> <distance> <iterations> [out.jxl]`: public-API
//! VarDCT encode of 8-bit RGB through the port (one thread, no runner).
#![allow(unsafe_code)]
use ferrisma_jxl_decoder::{decode_into, inspect};
use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::encode::{self as api, JxlEncoderFrameSettingId, JxlEncoderStatus};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

/// Counts live heap bytes, the peak, and logs allocations of at least 1 MiB
/// (size, live bytes after it) when `JXL_PROBE_ALLOC=1`.
struct Counting;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static LOG: AtomicBool = AtomicBool::new(false);
static BIG: [AtomicUsize; 512] = [const { AtomicUsize::new(0) }; 512];
static BIG_LIVE: [AtomicUsize; 512] = [const { AtomicUsize::new(0) }; 512];
static BIG_N: AtomicUsize = AtomicUsize::new(0);

// SAFETY: Delegates every call to the system allocator unchanged.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(live, Ordering::Relaxed);
        if LOG.load(Ordering::Relaxed) && layout.size() >= 1 << 20 {
            let i = BIG_N.fetch_add(1, Ordering::Relaxed);
            if i < 512 {
                BIG[i].store(layout.size(), Ordering::Relaxed);
                BIG_LIVE[i].store(live, Ordering::Relaxed);
            }
        }
        // SAFETY: Same layout contract as the caller's.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let live = LIVE.fetch_sub(layout.size(), Ordering::Relaxed) - layout.size();
        if LOG.load(Ordering::Relaxed) && layout.size() >= 1 << 20 {
            let i = BIG_N.fetch_add(1, Ordering::Relaxed);
            if i < 512 {
                // High bit marks a release.
                BIG[i].store(layout.size() | 1 << 63, Ordering::Relaxed);
                BIG_LIVE[i].store(live, Ordering::Relaxed);
            }
        }
        // SAFETY: Same pointer/layout pair the caller allocated.
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let live = LIVE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(live, Ordering::Relaxed);
        if LOG.load(Ordering::Relaxed) && layout.size() >= 1 << 20 {
            let i = BIG_N.fetch_add(1, Ordering::Relaxed);
            if i < 512 {
                BIG[i].store(layout.size(), Ordering::Relaxed);
                BIG_LIVE[i].store(live, Ordering::Relaxed);
            }
        }
        // SAFETY: Same layout contract as the caller's.
        unsafe { System.alloc_zeroed(layout) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

fn report_alloc() {
    if !LOG.load(Ordering::Relaxed) {
        return;
    }
    println!("heap_peak_bytes={}", PEAK.load(Ordering::Relaxed));
    for i in 0..BIG_N.load(Ordering::Relaxed).min(512) {
        let size = BIG[i].load(Ordering::Relaxed);
        println!(
            "big_{} size={} live_after={}",
            if size >> 63 == 1 { "free" } else { "alloc" },
            size & !(1 << 63),
            BIG_LIVE[i].load(Ordering::Relaxed)
        );
    }
}

fn read_ppm(path: &str) -> (u32, u32, Vec<u8>) {
    let data = std::fs::read(path).unwrap();
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
    let (w, h): (u32, u32) = (fields[1].parse().unwrap(), fields[2].parse().unwrap());
    let pixels = data[pos + 1..].to_vec();
    assert_eq!(pixels.len(), w as usize * h as usize * 3);
    (w, h, pixels)
}

fn encode_lossy(w: u32, h: u32, pixels: &[u8], effort: i64, distance: f32) -> Vec<u8> {
    // SAFETY: Null manager selects the default allocator; destroyed below.
    let enc = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    assert!(!enc.is_null());
    // SAFETY: `enc` is live and uniquely used in this function.
    let e = unsafe { &mut *enc };
    let mut info = JxlBasicInfo::zeroed();
    api::jxl_encoder_init_basic_info(&mut info);
    info.xsize = w;
    info.ysize = h;
    info.bits_per_sample = 8;
    info.num_color_channels = 3;
    info.uses_original_profile = 0;
    assert_eq!(
        api::jxl_encoder_set_basic_info(e, &info),
        JxlEncoderStatus::SUCCESS
    );
    let mut color = JxlColorEncoding::default();
    api::jxl_color_encoding_set_to_srgb(&mut color, 0);
    assert_eq!(
        api::jxl_encoder_set_color_encoding(e, &color),
        JxlEncoderStatus::SUCCESS
    );
    // SAFETY: Live encoder; the settings are owned by it.
    let settings = unsafe { api::jxl_encoder_frame_settings_create(enc, std::ptr::null()) };
    assert!(!settings.is_null());
    // SAFETY: Live settings of a live encoder.
    unsafe {
        assert_eq!(
            api::jxl_encoder_frame_settings_set_option(
                settings,
                JxlEncoderFrameSettingId::EFFORT,
                effort
            ),
            JxlEncoderStatus::SUCCESS
        );
        assert_eq!(
            api::jxl_encoder_set_frame_distance(settings, distance),
            JxlEncoderStatus::SUCCESS
        );
    }
    let format = JxlPixelFormat {
        num_channels: 3,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    // SAFETY: Live settings; the buffer outlives the call (it is copied).
    let status = unsafe { api::jxl_encoder_add_image_frame(settings, &format, Some(pixels)) };
    assert_eq!(status, JxlEncoderStatus::SUCCESS);
    // SAFETY: `enc` is live.
    api::jxl_encoder_close_input(unsafe { &mut *enc });
    let mut out = vec![0u8; 1 << 20];
    let mut used = 0usize;
    loop {
        let mut next = out[used..].as_mut_ptr();
        let mut avail = out.len() - used;
        let before = avail;
        // SAFETY: `next` points to `avail` writable bytes of `out`.
        let status = unsafe { api::jxl_encoder_process_output(&mut *enc, &mut next, &mut avail) };
        used += before - avail;
        if status == JxlEncoderStatus::NEED_MORE_OUTPUT {
            out.resize(out.len() * 2, 0);
            continue;
        }
        assert_eq!(status, JxlEncoderStatus::SUCCESS, "encode failed");
        break;
    }
    out.truncate(used);
    // SAFETY: Not used again.
    unsafe { api::jxl_encoder_destroy(enc) };
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if std::env::var_os("JXL_PROBE_ALLOC").is_some() {
        LOG.store(true, Ordering::Relaxed);
    }
    let begin;
    match args[1].as_str() {
        "decode" => {
            let bytes = std::fs::read(&args[2]).unwrap();
            let iterations: usize = args[3].parse().unwrap();
            begin = Instant::now();
            for _ in 0..iterations {
                let info = inspect(&bytes, 1 << 30).unwrap();
                let mut output = vec![0f32; info.sample_count().unwrap()];
                decode_into(&bytes, &info, &mut output, 1 << 30).unwrap();
                black_box(&output);
                if std::env::var_os("JXL_PROBE_HASH").is_some() {
                    // FNV-1a over the exact F32 bit patterns.
                    let mut h = 0xcbf29ce484222325u64;
                    for s in &output {
                        for b in s.to_bits().to_le_bytes() {
                            h = (h ^ u64::from(b)).wrapping_mul(0x100000001b3);
                        }
                    }
                    println!("output_fnv1a64={h:016x} samples={}", output.len());
                }
            }
        }
        "lossy" => {
            let (w, h, pixels) = read_ppm(&args[2]);
            let effort: i64 = args[3].parse().unwrap();
            let distance: f32 = args[4].parse().unwrap();
            let iterations: usize = args[5].parse().unwrap();
            begin = Instant::now();
            let mut encoded = Vec::new();
            for _ in 0..iterations {
                encoded = encode_lossy(w, h, &pixels, effort, distance);
            }
            println!("bytes={}", encoded.len());
            if let Some(path) = args.get(6) {
                std::fs::write(path, &encoded).unwrap();
            }
        }
        "lossless" => {
            // The engine sink's admitted profile: Modular effort 2, 8-bit RGB.
            use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
            let (w, h, pixels) = read_ppm(&args[2]);
            begin = Instant::now();
            let mut config = LosslessThunderConfig::new(
                w as usize,
                h as usize,
                false,
                LosslessSampleDepth::Uint8,
            )
            .unwrap();
            let encoded = config.encode_pixels(&pixels, None).unwrap();
            println!("bytes={}", encoded.as_slice().len());
        }
        _ => panic!("mode"),
    }
    println!(
        "elapsed_ms={:.1}",
        begin.elapsed().as_secs_f64() * 1000.0
    );
    report_alloc();
}
