// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Normal scalar XYBStage and GetXYBStage from stage_xyb.cc. The NEON-specific
//! FastXYBStage and SIMD conversions remain separate work. One-lane tails have
//! zero length, so the source MSan unpoison/poison annotations touch no storage.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::cms::opsin_params::{SCALED_XYB_OFFSET, SCALED_XYB_SCALE};
use crate::jxl::color_encoding::ColorSpace;
use crate::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use crate::jxl::dec_xyb_scalar::mul;

use super::scalar_rows::{self as k, Broadcast, lanes};

/// Scratch rows of the XYB row kernel.
#[derive(Default)]
struct XybRows {
    f: [Vec<f32>; 8],
    d: [Vec<f64>; 5],
    /// Broadcast rows: biases_cbrt[0..3], biases[0..3], then the nine matrix
    /// entries in the order the kernel uses them.
    scalars: [Broadcast; 15],
}

pub struct XybStage {
    opsin_params: OpsinParams,
    output_is_xyb: bool,
    rows: XybRows,
}
impl XybStage {
    pub fn new(info: &OutputEncodingInfo) -> Self {
        Self {
            opsin_params: info.opsin_params.clone(),
            output_is_xyb: info.color_encoding.get_color_space() == ColorSpace::XYB,
            rows: XybRows::default(),
        }
    }
}
impl RenderPipelineStage for XybStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "XYB"
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        // In place on three disjoint spans resolved once per row.
        let [r0, r1, r2] = buffers.three_rows_mut(rows, width);
        if self.output_is_xyb {
            for x in 0..width {
                let in_x = r0[x];
                let in_y = r1[x];
                let in_b = r2[x];
                let out_x = mul(in_x + SCALED_XYB_OFFSET[0], SCALED_XYB_SCALE[0]);
                let out_y = mul(in_y + SCALED_XYB_OFFSET[1], SCALED_XYB_SCALE[1]);
                let out_b = mul((in_b - in_y) + SCALED_XYB_OFFSET[2], SCALED_XYB_SCALE[2]);
                r0[x] = out_x;
                r1[x] = out_y;
                r2[x] = out_b;
            }
        } else {
            xyb_to_rgb_rows(&mut self.rows, [r0, r1, r2], &self.opsin_params);
        }
        Ok(())
    }
}
pub fn get_xyb_stage(info: &OutputEncodingInfo) -> Box<dyn RenderPipelineStage> {
    Box::new(XybStage::new(info))
}

/// XybToRgb (dec_xyb_scalar::xyb_to_rgb) over whole rows, in place.
///
/// Every lane performs the operations of `xyb_to_rgb` with the same operands
/// in the same order; the row kernels of `scalar_rows` keep its out-of-line
/// Add/promote/product boundaries per operation instead of per element.
fn xyb_to_rgb_rows(scratch: &mut XybRows, [r0, r1, r2]: [&mut [f32]; 3], params: &OpsinParams) {
    let n = r0.len();
    let XybRows { f, d, scalars } = scratch;
    let [
        s_cbrt_r,
        s_cbrt_g,
        s_cbrt_b,
        s_bias_r,
        s_bias_g,
        s_bias_b,
        m0,
        m12,
        m24,
        m4,
        m16,
        m28,
        m8,
        m20,
        m32,
    ] = scalars;
    let [f0, f1, f2, f3, f4, f5, f6, f7] = f;
    let [d0, d1, dr, dg, db] = d;
    let matrix = &params.inverse_opsin_matrix;
    // gamma_r = add(opsin_y, opsin_x) - biases_cbrt[0]
    let t = lanes(f0, n);
    k::add(t, r1, r0);
    let gamma_r = lanes(f1, n);
    k::sub(gamma_r, t, s_cbrt_r.row(params.opsin_biases_cbrt[0], n));
    // gamma_g = (opsin_y - opsin_x) - biases_cbrt[1]
    k::sub(t, r1, r0);
    let gamma_g = lanes(f2, n);
    k::sub(gamma_g, t, s_cbrt_g.row(params.opsin_biases_cbrt[1], n));
    // gamma_b = opsin_b - biases_cbrt[2]
    let gamma_b = lanes(f3, n);
    k::sub(gamma_b, r2, s_cbrt_b.row(params.opsin_biases_cbrt[2], n));
    // mixed = MulAdd(Mul(gamma, gamma), gamma, neg_bias), promoted for the
    // matrix products: add(mul(gamma2, gamma), neg_bias).
    let square = lanes(f4, n);
    let cube = lanes(f5, n);
    let mixed = lanes(f6, n);
    for (gamma, bias, value, wide_mixed) in [
        (&*gamma_r, s_bias_r, params.opsin_biases[0], &mut *dr),
        (&*gamma_g, s_bias_g, params.opsin_biases[1], &mut *dg),
        (&*gamma_b, s_bias_b, params.opsin_biases[2], &mut *db),
    ] {
        let wide = lanes(d0, n);
        k::promote(wide, gamma);
        k::product(square, wide, wide);
        let wide_square = lanes(d1, n);
        k::promote(wide_square, square);
        k::product(cube, wide_square, wide);
        k::add(mixed, cube, bias.row(value, n));
        k::promote(lanes(wide_mixed, n), mixed);
    }
    let (wr, wg, wb) = (&dr[..n], &dg[..n], &db[..n]);
    // linear_c = mul(m[c0], mixed_r); then MulAdd(m[c1], mixed_g, linear_c)
    // and MulAdd(m[c2], mixed_b, linear_c), as add(mul(m, mixed), linear).
    let product = lanes(f7, n);
    let acc = lanes(f0, n);
    let channels: [(&mut [f32], [(&mut Broadcast, f32); 3]); 3] = [
        (r0, [(m0, matrix[0]), (m4, matrix[4]), (m8, matrix[8])]),
        (
            r1,
            [(m12, matrix[12]), (m16, matrix[16]), (m20, matrix[20])],
        ),
        (
            r2,
            [(m24, matrix[24]), (m28, matrix[28]), (m32, matrix[32])],
        ),
    ];
    // All three inputs were consumed above; outputs overwrite them now.
    for (out, [(a, va), (b, vb), (c, vc)]) in channels {
        k::product(acc, a.promoted(va, n), wr);
        k::product(product, b.promoted(vb, n), wg);
        k::add(out, product, acc);
        k::product(product, c.promoted(vc, n), wb);
        k::add(acc, product, out);
        out.copy_from_slice(acc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jxl::dec_xyb_scalar::xyb_to_rgb;
    use crate::jxl::render_pipeline::scalar_rows::tests::adversarial;

    /// The row kernel equals the per-element `xyb_to_rgb` bit for bit on
    /// every triple of adversarial words (NaN payloads in all operand
    /// positions, infinities, denormals, signed zeros), for the default
    /// parameters and for parameters that are themselves NaNs with payloads.
    #[test]
    fn xyb_row_kernel_matches_per_element_helper_bit_for_bit() {
        let v = adversarial();
        let (mut x, mut y, mut b) = (Vec::new(), Vec::new(), Vec::new());
        for &i in &v {
            for &j in &v {
                for &k in &v {
                    x.push(i);
                    y.push(j);
                    b.push(k);
                }
            }
        }
        let mut nan_params = OpsinParams::default();
        for (n, value) in nan_params
            .inverse_opsin_matrix
            .iter_mut()
            .chain(&mut nan_params.opsin_biases)
            .chain(&mut nan_params.opsin_biases_cbrt)
            .enumerate()
        {
            if n % 3 != 1 {
                *value = v[n % 8];
            }
        }
        let mut scratch = XybRows::default();
        for params in [OpsinParams::default(), nan_params] {
            let want: Vec<[u32; 3]> = (0..x.len())
                .map(|i| xyb_to_rgb(x[i], y[i], b[i], &params).map(f32::to_bits))
                .collect();
            let (mut r0, mut r1, mut r2) = (x.clone(), y.clone(), b.clone());
            xyb_to_rgb_rows(&mut scratch, [&mut r0, &mut r1, &mut r2], &params);
            for (i, w) in want.iter().enumerate() {
                let got = [r0[i].to_bits(), r1[i].to_bits(), r2[i].to_bits()];
                assert_eq!(got, *w, "lane {i}");
            }
        }
    }
}
