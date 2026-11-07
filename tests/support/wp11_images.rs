//! Shared image generators for the WP11 differential tests: seeded ARGB images
//! of the kinds that drive the VP8L encoder down its different paths (photo-like
//! gradients, noise, small and large palettes with and without the color 0,
//! flat areas with long runs, transparent areas, gray images whose red and blue
//! vanish after subtract-green, and repeating stripes).

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the generators"
)]

/// SplitMix64, a small deterministic generator for the cases.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `0..bound`.
    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

/// The kinds of generated images.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Smooth gradients with a little noise, opaque.
    Photo,
    /// Uniformly random pixels.
    Noise,
    /// Random runs of `n` palette colors; `zero` adds the color 0.
    Palette { n: usize, zero: bool },
    /// One color with a few rectangles of other colors.
    Flat,
    /// A gradient with transparent areas whose RGB values are random.
    Transparent,
    /// Gray pixels, whose red and blue are zero after subtract-green.
    Gray,
    /// A repeating pattern of a few colors.
    Stripes,
}

impl Kind {
    /// Every kind, with a few palette sizes.
    pub const ALL: [Kind; 11] = [
        Kind::Photo,
        Kind::Noise,
        Kind::Palette { n: 2, zero: false },
        Kind::Palette { n: 4, zero: true },
        Kind::Palette { n: 11, zero: false },
        Kind::Palette { n: 40, zero: true },
        Kind::Palette {
            n: 200,
            zero: false,
        },
        Kind::Flat,
        Kind::Transparent,
        Kind::Gray,
        Kind::Stripes,
    ];
}

/// A `width` by `height` image of `kind`, with rows `stride` pixels apart.
/// The padding between rows holds random pixels, which the encoder must not
/// read.
pub fn image(rng: &mut Rng, kind: Kind, width: usize, height: usize, stride: usize) -> Vec<u32> {
    let mut argb: Vec<u32> = (0..stride * height).map(|_| rng.u32()).collect();
    let mut set = |x: usize, y: usize, value: u32| argb[y * stride + x] = value;
    match kind {
        Kind::Photo | Kind::Transparent => {
            let (r0, g0, b0) = (rng.below(256), rng.below(256), rng.below(256));
            let (dx, dy) = (1 + rng.below(5), 1 + rng.below(5));
            let noise = 1 + rng.below(6);
            for y in 0..height {
                for x in 0..width {
                    let r = (r0 + x as u64 * dx + rng.below(noise)) & 0xff;
                    let g = (g0 + y as u64 * dy + rng.below(noise)) & 0xff;
                    let b = (b0 + (x + y) as u64 + rng.below(noise)) & 0xff;
                    let mut a = 0xff;
                    if kind == Kind::Transparent && ((x / 5 + y / 3) % 3 == 0) {
                        a = if rng.below(4) == 0 { rng.below(256) } else { 0 };
                    }
                    set(x, y, (a << 24 | r << 16 | g << 8 | b) as u32);
                }
            }
        }
        Kind::Noise => {
            for y in 0..height {
                for x in 0..width {
                    let value = rng.u32();
                    set(x, y, value);
                }
            }
        }
        Kind::Palette { n, zero } => {
            let mut colors: Vec<u32> = (0..n).map(|_| rng.u32() | 0xff00_0000).collect();
            if zero {
                colors[0] = 0;
            }
            let mut run = 0;
            let mut color = colors[0];
            for y in 0..height {
                for x in 0..width {
                    if run == 0 {
                        color = colors[rng.below(n as u64) as usize];
                        run = 1 + rng.below(12);
                    }
                    run -= 1;
                    set(x, y, color);
                }
            }
        }
        Kind::Flat => {
            let background = rng.u32();
            for y in 0..height {
                for x in 0..width {
                    set(x, y, background);
                }
            }
            for _ in 0..rng.below(4) {
                let color = rng.u32();
                let (x0, y0) = (
                    rng.below(width as u64) as usize,
                    rng.below(height as u64) as usize,
                );
                let (w, h) = (
                    1 + rng.below((width - x0) as u64) as usize,
                    1 + rng.below((height - y0) as u64) as usize,
                );
                for y in y0..y0 + h {
                    for x in x0..x0 + w {
                        set(x, y, color);
                    }
                }
            }
        }
        Kind::Gray => {
            let start = rng.below(256);
            for y in 0..height {
                for x in 0..width {
                    let v = (start + (x * 3 + y * 2) as u64 + rng.below(3)) & 0xff;
                    set(x, y, (0xff00_0000 | v << 16 | v << 8 | v) as u32);
                }
            }
        }
        Kind::Stripes => {
            let colors: Vec<u32> = (0..1 + rng.below(5)).map(|_| rng.u32()).collect();
            let period = 1 + rng.below(9) as usize;
            for y in 0..height {
                for x in 0..width {
                    set(x, y, colors[((x + y * 3) / period) % colors.len()]);
                }
            }
        }
    }
    argb
}
