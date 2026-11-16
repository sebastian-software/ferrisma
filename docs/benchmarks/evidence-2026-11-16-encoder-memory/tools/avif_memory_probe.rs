//! Memory probe for the engine's AVIF writer (2026-11-16). Copy into
//! `crates/ferrisma-avif-write/examples/` to build (see `../README.md`).
//!
//! `avif_memory_probe <in.ppm|in.pam> <depth 8|10|12>`
//!
//! The engine's AVIF path through the writer the `avif-write` feature
//! uses (`LibavifBackend`): `Frame::new` with the engine's default options
//! (quality 60, alpha quality 60, speed 6, 4:4:4, one thread), rows at the
//! image depth (`write_row` at 8 bits; `write_row_u16` above, the 16-bit
//! input shifted to the depth), `finish`. The writer's frame is the only
//! copy of the pixels during the encode: the input file is dropped first.
//! The engine reserves `required_frame_bytes` (printed) for this request.
#![allow(unsafe_code)]
use ferrisma_avif_write::{Chroma, Config, Frame, required_frame_bytes};
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
    let depth: u8 = args[2].parse().unwrap();
    assert!(depth == 8 || bits == 16, "deep output needs a 16-bit input");
    let config = Config {
        width: u32::try_from(w).unwrap(),
        height: u32::try_from(h).unwrap(),
        has_alpha: alpha,
        quality: 60,
        alpha_quality: 60,
        speed: 6,
        threads: 1,
        chroma: Chroma::Yuv444,
        depth,
        sample_depth: depth,
    };
    let reserved = required_frame_bytes(&config).unwrap();
    let mut frame = Frame::new(config).unwrap();
    let ch = if alpha { 4 } else { 3 };
    let s = if bits > 8 { 2 } else { 1 };
    let mut row8 = vec![0_u8; w * 4];
    let mut row16 = vec![0_u16; w * 4];
    for (y, src) in data.chunks_exact(w * ch * s).enumerate() {
        let y = u32::try_from(y).unwrap();
        for x in 0..w {
            for c in 0..4 {
                let v16 = if c < ch {
                    let i = (x * ch + c) * s;
                    if s == 2 {
                        u16::from_be_bytes([src[i], src[i + 1]])
                    } else {
                        u16::from(src[i]) << 8
                    }
                } else {
                    u16::MAX
                };
                row8[x * 4 + c] = (v16 >> 8) as u8;
                row16[x * 4 + c] = v16 >> (16 - u32::from(depth));
            }
        }
        if depth == 8 {
            frame.write_row(y, &row8).unwrap();
        } else {
            frame.write_row_u16(y, &row16).unwrap();
        }
    }
    drop(data);
    PEAK.store(LIVE.load(Ordering::Relaxed), Ordering::Relaxed);
    let out = frame.finish().unwrap();
    println!(
        "pixels={} alpha={alpha} depth={depth} reserved={reserved} bytes={}",
        w * h,
        out.len()
    );
    println!("heap_peak_bytes={}", PEAK.load(Ordering::Relaxed));
}
