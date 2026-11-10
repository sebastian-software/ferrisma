//! Shared picture cases for the WP9 differential tests: a seeded generator of
//! pictures whose planes are caller-owned buffers, with transparent 8 by 8 blocks in
//! the alpha, and the builder that lends those buffers to a `WebPPicture` for one
//! operation. Used by `tests/wp9_tools.rs` and `tests/wp9_rescale.rs`.

use webp::webp::encode::{WebPEncCSP, WebPPicture};
use webp::webp::types::{PlaneOffset, PlaneStorage};

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

    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// One picture with its planes as caller-owned buffers, in the probe's layout.
pub struct Case {
    pub use_argb: i32,
    pub colorspace: u32,
    pub width: usize,
    pub height: usize,
    pub y_stride: usize,
    pub uv_stride: usize,
    pub a_stride: usize,
    pub argb_stride: usize,
    pub argb: Vec<u32>,
    pub y: Vec<u8>,
    pub u: Vec<u8>,
    pub v: Vec<u8>,
    pub a: Vec<u8>,
}

impl Case {
    pub fn random(rng: &mut Rng, use_argb: i32, colorspace: u32) -> Case {
        let width = 1 + rng.below(20) as usize;
        let height = 1 + rng.below(20) as usize;
        Case::with_size(rng, use_argb, colorspace, width, height)
    }

    /// A case of the given size, with the samples that `random` draws.
    pub fn with_size(
        rng: &mut Rng,
        use_argb: i32,
        colorspace: u32,
        width: usize,
        height: usize,
    ) -> Case {
        let y_stride = width + rng.below(3) as usize;
        let uv_width = width.div_ceil(2);
        let uv_stride = uv_width + rng.below(3) as usize;
        let a_stride = width + rng.below(3) as usize;
        let argb_stride = width + rng.below(3) as usize;
        // Transparent 8 by 8 blocks in a checkerboard of blocks, random elsewhere.
        let alpha_at = |rng: &mut Rng, x: usize, y: usize| -> u8 {
            if (x / 8 + y / 8).is_multiple_of(3) {
                0
            } else if rng.below(4) == 0 {
                255
            } else {
                rng.byte()
            }
        };
        let mut argb = vec![0_u32; argb_stride * height];
        for y in 0..height {
            for x in 0..width {
                let alpha = alpha_at(rng, x, y);
                let color = u32::from(rng.byte()) << 16
                    | u32::from(rng.byte()) << 8
                    | u32::from(rng.byte());
                argb[y * argb_stride + x] = u32::from(alpha) << 24 | color;
            }
        }
        let y = (0..y_stride * height)
            .map(|_| rng.byte())
            .collect::<Vec<u8>>();
        let u = (0..uv_stride * height.div_ceil(2))
            .map(|_| rng.byte())
            .collect::<Vec<u8>>();
        let v = (0..uv_stride * height.div_ceil(2))
            .map(|_| rng.byte())
            .collect::<Vec<u8>>();
        let mut a = vec![0_u8; a_stride * height];
        for row in 0..height {
            for x in 0..width {
                a[row * a_stride + x] = alpha_at(rng, x, row);
            }
        }
        Case {
            use_argb,
            colorspace,
            width,
            height,
            y_stride,
            uv_stride,
            a_stride,
            argb_stride,
            argb,
            y,
            u,
            v,
            a,
        }
    }

    pub fn has_alpha(&self) -> bool {
        self.colorspace & 4 != 0
    }

    /// The command fields `WIDTH HEIGHT ...` the probe reads, then the hex of each plane.
    pub fn command_fields(&self) -> String {
        let a_stride = if self.has_alpha() { self.a_stride } else { 0 };
        format!(
            "{} {} {} {} {} {} {} {}",
            self.use_argb,
            self.colorspace,
            self.width,
            self.height,
            self.y_stride,
            self.uv_stride,
            a_stride,
            self.argb_stride
        )
    }

    pub fn argb_bytes(&self) -> Vec<u8> {
        self.argb
            .iter()
            .flat_map(|word| word.to_ne_bytes())
            .collect()
    }

    /// The hex arguments of the planes: ARGB, then Y, U, V and A, `-` where unused.
    pub fn plane_arguments(&self) -> String {
        let (argb, y, u, v, a) = if self.use_argb != 0 {
            (
                hex(&self.argb_bytes()),
                "-".to_owned(),
                "-".to_owned(),
                "-".to_owned(),
                "-".to_owned(),
            )
        } else {
            (
                "-".to_owned(),
                hex(&self.y),
                hex(&self.u),
                hex(&self.v),
                if self.has_alpha() {
                    hex(&self.a)
                } else {
                    "-".to_owned()
                },
            )
        };
        format!("{argb} {y} {u} {v} {a}")
    }
}

/// Builds the picture of `case` over its buffers and runs `operation` on it.
pub fn with_picture<R>(case: &mut Case, operation: impl FnOnce(&mut WebPPicture<'_>) -> R) -> R {
    let mut words = std::mem::take(&mut case.argb);
    let mut picture = WebPPicture {
        use_argb: case.use_argb,
        colorspace: WebPEncCSP(case.colorspace),
        width: i32::try_from(case.width).unwrap(),
        height: i32::try_from(case.height).unwrap(),
        y_stride: i32::try_from(case.y_stride).unwrap(),
        uv_stride: i32::try_from(case.uv_stride).unwrap(),
        a_stride: if case.has_alpha() {
            i32::try_from(case.a_stride).unwrap()
        } else {
            0
        },
        argb_stride: i32::try_from(case.argb_stride).unwrap(),
        ..WebPPicture::default()
    };
    let result = if case.use_argb != 0 {
        picture.argb = Some(PlaneOffset {
            region: 0,
            offset: 0,
        });
        picture.argb_storage = PlaneStorage::External(vec![words.as_mut_slice()]);
        operation(&mut picture)
    } else {
        let has_alpha = case.has_alpha();
        picture.y = Some(PlaneOffset {
            region: 0,
            offset: 0,
        });
        picture.u = Some(PlaneOffset {
            region: 1,
            offset: 0,
        });
        picture.v = Some(PlaneOffset {
            region: 2,
            offset: 0,
        });
        let mut slices: Vec<&mut [u8]> = vec![
            case.y.as_mut_slice(),
            case.u.as_mut_slice(),
            case.v.as_mut_slice(),
        ];
        if has_alpha {
            picture.a = Some(PlaneOffset {
                region: 3,
                offset: 0,
            });
            slices.push(case.a.as_mut_slice());
        }
        picture.yuva = PlaneStorage::External(slices);
        operation(&mut picture)
    };
    drop(picture);
    case.argb = words;
    result
}
