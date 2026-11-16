//! Memory probe for the engine's WebP sink (2026-11-16). Copy into
//! `codecs/libwebp-rs/examples/` to build (see `../README.md`).
//!
//! `webp_memory_probe <in.ppm|in.pam> <lossless LEVEL | lossy QUALITY METHOD>`
//!
//! What `src/webp_sink.rs` holds and calls: one ARGB `u32` frame, the
//! sink's `Config` (`lossless_level`, or the default preset at the quality
//! with the method; `exact`), `Picture::from_argb` and `Encoder::encode_to`
//! into a growing vector. 8-bit input only, as the sink. The input file is
//! dropped before the encode. Prints the engine's reservation for the
//! request (the frame plus, for lossy, the YUV420(A) planes).
#![allow(unsafe_code)]
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use webp::api::encode::{Config, Encoder, Picture, Preset};

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

/// Width, height, alpha, bits and the samples of a binary PPM (P6) or PAM
/// (P7, `RGB_ALPHA`) of maxval `2^n - 1`, 16-bit samples big-endian.
fn read_pnm(path: &str) -> (usize, usize, bool, u8, Vec<u8>) {
    let mut data = std::fs::read(path).unwrap();
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
    data.drain(..start);
    (w, h, alpha, bits, data)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (w, h, alpha, bits, data) = read_pnm(&args[1]);
    assert_eq!(bits, 8);
    let ch = if alpha { 4 } else { 3 };
    let mut frame: Vec<u32> = data
        .chunks_exact(ch)
        .map(|p| u32::from_be_bytes([if alpha { p[3] } else { 255 }, p[0], p[1], p[2]]))
        .collect();
    drop(data);
    let lossy = args[2] == "lossy";
    let config = if lossy {
        Config::preset(Preset::Default, args[3].parse().unwrap())
            .map(|c| c.method(args[4].parse().unwrap()))
    } else {
        Config::new().lossless_level(args[3].parse().unwrap())
    }
    .unwrap()
    .exact(true);
    let pixels = w * h;
    let converted = if lossy {
        pixels + w.div_ceil(2) * h.div_ceil(2) * 2 + if alpha { pixels } else { 0 }
    } else {
        0
    };
    let reserved = pixels * 4 + converted;
    PEAK.store(LIVE.load(Ordering::Relaxed), Ordering::Relaxed);
    let mut picture = Picture::from_argb(
        &mut frame,
        u32::try_from(w).unwrap(),
        u32::try_from(h).unwrap(),
        w,
    )
    .unwrap();
    let mut out = Vec::new();
    Encoder::new(&config)
        .encode_to(&mut picture, |chunk| {
            out.extend_from_slice(chunk);
            true
        })
        .unwrap();
    println!(
        "pixels={pixels} alpha={alpha} reserved={reserved} bytes={}",
        out.len()
    );
    println!("heap_peak_bytes={}", PEAK.load(Ordering::Relaxed));
}
