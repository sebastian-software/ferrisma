//! Memory probe for the engine's JPEG XL sink (2026-11-16). Copy into
//! `crates/ferrisma-jxl-encoder/examples/` to build (see `../README.md`).
//!
//! `jxl_memory_probe <in.ppm|in.pam> <lossless|distance> [out.jxl]`
//!
//! The process holds what the engine's `JxlSink` holds when it calls the
//! bridge: one RGBA frame of `4 * width * height` samples (1 byte at 8 bits,
//! a native-endian `u16` above), compacted in place to RGB when the input
//! has no alpha, and the bridge call the sink makes (`encode_lossless`, or
//! `encode` with `FrameOptions::with_distance(d)`, patches off). The input
//! file is read, expanded into that frame and dropped before the encode.
//! Input: binary PPM (P6) or PAM (P7, `RGB_ALPHA`), maxval `2^n - 1`.
//! A counting global allocator prints the peak of live heap bytes
//! (`heap_peak_bytes`) over the encode; peak RSS comes from
//! `/usr/bin/time -l` around a fresh process.
#![allow(unsafe_code)]
use ferrisma_jxl_encoder::{FrameOptions, ImageInfo, encode, encode_lossless};
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
        // A reallocation counts as allocate-then-free, its transient peak.
        let live = LIVE.fetch_add(new_size, Ordering::Relaxed) + new_size;
        PEAK.fetch_max(live, Ordering::Relaxed);
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: Same contract as the caller's.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

/// Width, height, alpha, bits and the RGBA frame as the sink holds it.
fn read_frame(path: &str) -> (usize, usize, bool, u8, Vec<u8>) {
    let data = std::fs::read(path).unwrap();
    let (tokens, start) = if data.starts_with(b"P7") {
        let end = data.windows(7).position(|w| w == b"ENDHDR\n").unwrap() + 7;
        let text = std::str::from_utf8(&data[..end]).unwrap();
        (
            text.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            end,
        )
    } else {
        let mut tokens = Vec::new();
        let mut i = 0;
        while tokens.len() < 4 {
            while data[i].is_ascii_whitespace() {
                i += 1;
            }
            let s = i;
            while !data[i].is_ascii_whitespace() {
                i += 1;
            }
            tokens.push(String::from_utf8_lossy(&data[s..i]).into_owned());
        }
        (tokens, i + 1)
    };
    let field = |key: &str| -> usize {
        let i = tokens.iter().position(|t| t == key).unwrap();
        tokens[i + 1].parse().unwrap()
    };
    let (w, h, maxval, alpha) = if tokens[0] == "P7" {
        let alpha = tokens.iter().any(|t| t == "RGB_ALPHA");
        (field("WIDTH"), field("HEIGHT"), field("MAXVAL"), alpha)
    } else {
        (
            tokens[1].parse().unwrap(),
            tokens[2].parse().unwrap(),
            tokens[3].parse().unwrap(),
            false,
        )
    };
    let bits = u8::try_from(usize::BITS - maxval.leading_zeros()).unwrap();
    assert_eq!((1_usize << bits) - 1, maxval);
    let s = if bits > 8 { 2 } else { 1 };
    let ch = if alpha { 4 } else { 3 };
    let raw = &data[start..];
    assert_eq!(raw.len(), w * h * ch * s);
    // The sink's frame: always four samples per pixel.
    let mut frame = vec![0_u8; w * h * 4 * s];
    for (px, src) in frame.chunks_exact_mut(4 * s).zip(raw.chunks_exact(ch * s)) {
        for c in 0..4 {
            let sample = if c < ch {
                &src[c * s..(c + 1) * s]
            } else if s == 2 {
                &[0xff, 0xff][..]
            } else {
                &[0xff][..]
            };
            if s == 2 {
                let v = u16::from_be_bytes([sample[0], sample[1]]);
                px[c * 2..c * 2 + 2].copy_from_slice(&v.to_ne_bytes());
            } else {
                px[c] = sample[0];
            }
        }
    }
    drop(data);
    if !alpha {
        // The sink's in-place compaction to RGB.
        for p in 0..w * h {
            frame.copy_within(p * 4 * s..(p * 4 + 3) * s, p * 3 * s);
        }
    }
    (w, h, alpha, bits, frame)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h, alpha, bits, frame) = read_frame(&args[1]);
    let s = if bits > 8 { 2 } else { 1 };
    let samples = w * h * if alpha { 4 } else { 3 } * s;
    let (wu, hu) = (u32::try_from(w).unwrap(), u32::try_from(h).unwrap());
    let image = if alpha {
        ImageInfo::srgba8(wu, hu)
    } else {
        ImageInfo::srgb8(wu, hu)
    }
    .with_bits(bits);
    PEAK.store(LIVE.load(Ordering::Relaxed), Ordering::Relaxed);
    let mut out = Vec::new();
    if args[2] == "lossless" {
        encode_lossless(image, &frame[..samples], &mut out, usize::MAX).unwrap();
    } else {
        let distance: f32 = args[2].parse().unwrap();
        encode(
            image,
            FrameOptions::with_distance(distance),
            &frame[..samples],
            &mut out,
            usize::MAX,
        )
        .unwrap();
    }
    if let Some(path) = args.get(3) {
        std::fs::write(path, &out).unwrap();
    }
    println!(
        "pixels={} alpha={alpha} bits={bits} bytes={}",
        w * h,
        out.len()
    );
    println!("heap_peak_bytes={}", PEAK.load(Ordering::Relaxed));
}
