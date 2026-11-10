// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Matrix carriers and operations from src/skcms_public.h and
//! skcms.cc:1810-1980. Native contraction follows the actual pinned source
//! build; f64 inverse temporaries retain the original transposed names.

pub use super::types::{Matrix3x3, Matrix3x4};

fn finite(x: f32) -> bool {
    0.0 == x * 0.0
}
#[expect(
    clippy::manual_range_contains,
    reason = "Preserve the original ordered comparisons."
)]
fn zero_to_one(x: f32) -> bool {
    0.0 <= x && x <= 1.0
}
// Keep the pinned original's first input as the first floating-point operand.
// LLVM otherwise commutes selected unrolled products in ARM Release builds.
#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "x86_64", target_os = "linux")
))]
#[inline(never)]
fn concat_product(a: f32, b: f32) -> f32 {
    a * b
}
#[cfg(target_arch = "aarch64")]
#[inline(never)]
fn concat_mul_add(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
}
// The isolated original x86 observer adds product 0 to product 1, then
// product 2 to that sum. Keep its MULSS/ADDSS destination operand checkpoints.
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
#[inline(never)]
fn concat_add(a: f32, b: f32) -> f32 {
    a + b
}
fn concat_dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        concat_mul_add(
            a[2],
            b[2],
            concat_mul_add(a[0], b[0], concat_product(a[1], b[1])),
        )
    }
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    {
        concat_add(
            concat_product(a[2], b[2]),
            concat_add(concat_product(a[0], b[0]), concat_product(a[1], b[1])),
        )
    }
    #[cfg(not(any(
        target_arch = "aarch64",
        all(target_arch = "x86_64", target_os = "linux")
    )))]
    {
        dot3(a, b)
    }
}
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a[2].mul_add(b[2], a[0].mul_add(b[0], a[1] * b[1]))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}
// Keep the source ARM FNEG checkpoint separate from the following FMUL/FMA.
// LLVM may otherwise move negation across an operation and change a NaN sign.
#[cfg(target_arch = "aarch64")]
#[inline(never)]
fn negate(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ (1_u64 << 63))
}
fn subtract_product(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, negate(c))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b - c
    }
}
fn add_product(c: f64, a: f64, b: f64) -> f64 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        c + a * b
    }
}

impl Matrix3x3 {
    pub(crate) fn mul_vector(&self, vector: [f32; 3]) -> [f32; 3] {
        let mut dst = [0.0; 3];
        for (row, out) in dst.iter_mut().enumerate() {
            *out = dot3(self.vals[row], vector);
        }
        dst
    }
    pub fn concat(&self, other: &Self) -> Self {
        let mut matrix = Self::default();
        for r in 0..3 {
            for c in 0..3 {
                matrix.vals[r][c] = concat_dot3(
                    self.vals[r],
                    [other.vals[0][c], other.vals[1][c], other.vals[2][c]],
                );
            }
        }
        matrix
    }
    pub fn invert_into(&self, dst: &mut Self) -> bool {
        let a00 = self.vals[0][0] as f64;
        let a01 = self.vals[1][0] as f64;
        let a02 = self.vals[2][0] as f64;
        let a10 = self.vals[0][1] as f64;
        let a11 = self.vals[1][1] as f64;
        let a12 = self.vals[2][1] as f64;
        let a20 = self.vals[0][2] as f64;
        let a21 = self.vals[1][2] as f64;
        let a22 = self.vals[2][2] as f64;
        let mut b0 = subtract_product(a00, a11, a01 * a10);
        let mut b1 = subtract_product(a00, a12, a02 * a10);
        let mut b2 = subtract_product(a01, a12, a02 * a11);
        let mut b3 = a20;
        let mut b4 = a21;
        let mut b5 = a22;
        let determinant = add_product(subtract_product(b0, b5, b1 * b4), b2, b3);
        if determinant == 0.0 {
            return false;
        }
        let invdet = 1.0 / determinant;
        if invdet > f32::MAX as f64 || invdet < -(f32::MAX as f64) || !finite(invdet as f32) {
            return false;
        }
        b0 *= invdet;
        b1 *= invdet;
        b2 *= invdet;
        b3 *= invdet;
        b4 *= invdet;
        b5 *= invdet;
        dst.vals[0][0] = subtract_product(a11, b5, a12 * b4) as f32;
        dst.vals[1][0] = subtract_product(a02, b4, a01 * b5) as f32;
        dst.vals[2][0] = b2 as f32;
        #[cfg(target_arch = "aarch64")]
        {
            // The pinned original negates a10 before multiplying by b5.
            dst.vals[0][1] = add_product(b5 * negate(a10), a12, b3) as f32;
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            dst.vals[0][1] = subtract_product(a12, b3, a10 * b5) as f32;
        }
        dst.vals[1][1] = subtract_product(a00, b5, a02 * b3) as f32;
        dst.vals[2][1] = -b1 as f32;
        #[cfg(target_arch = "aarch64")]
        {
            // The pinned original negates a11 before multiplying by b3.
            dst.vals[0][2] = add_product(b3 * negate(a11), a10, b4) as f32;
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            dst.vals[0][2] = subtract_product(a10, b4, a11 * b3) as f32;
        }
        dst.vals[1][2] = subtract_product(a01, b3, a00 * b4) as f32;
        dst.vals[2][2] = b0 as f32;
        for row in dst.vals {
            for value in row {
                if !finite(value) {
                    return false;
                }
            }
        }
        true
    }
    pub fn invert_in_place(&mut self) -> bool {
        let source = *self;
        source.invert_into(self)
    }
}

pub fn adapt_to_xyz_d50(wx: f32, wy: f32, output: Option<&mut Matrix3x3>) -> bool {
    if !zero_to_one(wx) || !zero_to_one(wy) || output.is_none() {
        return false;
    }
    let output = output.unwrap();
    let white = [wx / wy, 1.0, (1.0 - wx - wy) / wy];
    let white_d50 = [0.96422, 1.0, 0.82521];
    let xyz_to_lms = Matrix3x3 {
        vals: [
            [0.8951, 0.2664, -0.1614],
            [-0.7502, 1.7135, 0.0367],
            [0.0389, -0.0685, 1.0296],
        ],
    };
    let lms_to_xyz = Matrix3x3 {
        vals: [
            [0.9869929, -0.1470543, 0.1599627],
            [0.4323053, 0.5183603, 0.0492912],
            [-0.0085287, 0.0400428, 0.9684867],
        ],
    };
    let src_cone = xyz_to_lms.mul_vector(white);
    let dst_cone = xyz_to_lms.mul_vector(white_d50);
    *output = Matrix3x3 {
        vals: [
            [dst_cone[0] / src_cone[0], 0.0, 0.0],
            [0.0, dst_cone[1] / src_cone[1], 0.0],
            [0.0, 0.0, dst_cone[2] / src_cone[2]],
        ],
    };
    *output = output.concat(&xyz_to_lms);
    *output = lms_to_xyz.concat(output);
    true
}

#[expect(
    clippy::too_many_arguments,
    reason = "Original eight chromaticities and destination."
)]
pub fn primaries_to_xyz_d50(
    rx: f32,
    ry: f32,
    gx: f32,
    gy: f32,
    bx: f32,
    by: f32,
    wx: f32,
    wy: f32,
    output: Option<&mut Matrix3x3>,
) -> bool {
    if !zero_to_one(rx)
        || !zero_to_one(ry)
        || !zero_to_one(gx)
        || !zero_to_one(gy)
        || !zero_to_one(bx)
        || !zero_to_one(by)
        || !zero_to_one(wx)
        || !zero_to_one(wy)
        || output.is_none()
    {
        return false;
    }
    let output = output.unwrap();
    let primaries = Matrix3x3 {
        vals: [
            [rx, gx, bx],
            [ry, gy, by],
            [1.0 - rx - ry, 1.0 - gx - gy, 1.0 - bx - by],
        ],
    };
    let mut primaries_inv = Matrix3x3::default();
    if !primaries.invert_into(&mut primaries_inv) {
        return false;
    }
    let white = [wx / wy, 1.0, (1.0 - wx - wy) / wy];
    let xyz = primaries_inv.mul_vector(white);
    let mut to_xyz = Matrix3x3 {
        vals: [[xyz[0], 0.0, 0.0], [0.0, xyz[1], 0.0], [0.0, 0.0, xyz[2]]],
    };
    to_xyz = primaries.concat(&to_xyz);
    let mut to_d50 = Matrix3x3::default();
    if !adapt_to_xyz_d50(wx, wy, Some(&mut to_d50)) {
        return false;
    }
    *output = to_d50.concat(&to_xyz);
    true
}
