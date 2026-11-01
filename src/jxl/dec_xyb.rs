// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! OpsinParams, CanOutputToColorEncoding, and complete OutputEncodingInfo
//! storage/setters from dec_xyb.{h,cc}. The scalar pixel helper is translated
//! separately in dec_xyb_scalar; general image conversion remains unported.
//! Safe zero storage represents source-indeterminate scalar members; it does
//! not assign them source meaning before the corresponding original write.
use super::base::matrix_ops::{Matrix3x3, Vector3, inv3x3_matrix, mul3x3_matrix};
use super::base::status::{Status, StatusCode};
use super::cms::color_encoding_cms::PrimariesCieXy;
use super::cms::icc::{adapt_to_xyz_d50, primaries_to_xyz, primaries_to_xyz_d50};
use super::cms::opsin_params::NEG_OPSIN_ABSORBANCE_BIAS_RGB;
use super::cms_interface::JxlCmsInterface;
use super::color_encoding::{ColorSpace, Primaries, TransferFunction, WhitePoint};
use super::color_encoding_internal::ColorEncoding;
use super::image_metadata::{CodecMetadata, DEFAULT_QUANT_BIAS};
use super::opsin_params::{get_opsin_absorbance_inverse_matrix, init_simd_inverse_matrix};

#[derive(Clone)]
pub struct OpsinParams {
    pub inverse_opsin_matrix: [f32; 36],
    pub opsin_biases: [f32; 4],
    pub opsin_biases_cbrt: [f32; 4],
    pub quant_biases: [f32; 4],
}
impl Default for OpsinParams {
    fn default() -> Self {
        Self {
            inverse_opsin_matrix: [0.0; 36],
            opsin_biases: [0.0; 4],
            opsin_biases_cbrt: [0.0; 4],
            quant_biases: [0.0; 4],
        }
    }
}
impl OpsinParams {
    pub fn init(&mut self, intensity_target: f32) {
        init_simd_inverse_matrix(
            get_opsin_absorbance_inverse_matrix(),
            &mut self.inverse_opsin_matrix,
            intensity_target,
        );
        self.opsin_biases = NEG_OPSIN_ABSORBANCE_BIAS_RGB;
        self.quant_biases = DEFAULT_QUANT_BIAS;
        for c in 0..4 {
            self.opsin_biases_cbrt[c] = self.opsin_biases[c].cbrt();
        }
    }
}
#[derive(Clone)]
pub struct OutputEncodingInfo {
    pub orig_color_encoding: ColorEncoding,
    pub orig_intensity_target: f32,
    pub orig_inverse_matrix: Matrix3x3,
    pub default_transform: bool,
    pub xyb_encoded: bool,
    pub color_encoding: ColorEncoding,
    pub linear_color_encoding: ColorEncoding,
    pub color_encoding_is_original: bool,
    pub opsin_params: OpsinParams,
    pub all_default_opsin: bool,
    pub inverse_gamma: f32,
    pub luminances: Vector3,
    pub desired_intensity_target: f32,
    pub cms_set: bool,
    pub color_management_system: JxlCmsInterface,
}
impl Default for OutputEncodingInfo {
    fn default() -> Self {
        Self {
            orig_color_encoding: ColorEncoding::default(),
            orig_intensity_target: 0.0,
            orig_inverse_matrix: [[0.0; 3]; 3],
            default_transform: false,
            xyb_encoded: false,
            color_encoding: ColorEncoding::default(),
            linear_color_encoding: ColorEncoding::default(),
            color_encoding_is_original: false,
            opsin_params: OpsinParams::default(),
            all_default_opsin: false,
            inverse_gamma: 0.0,
            luminances: [0.0; 3],
            desired_intensity_target: 0.0,
            cms_set: false,
            color_management_system: JxlCmsInterface::default(),
        }
    }
}
pub fn can_output_to_color_encoding(desired: &ColorEncoding) -> bool {
    if !desired.have_fields() {
        return false;
    }
    let tf = desired.tf();
    if !tf.is_pq()
        && !tf.is_srgb()
        && !tf.have_gamma
        && !tf.is_linear()
        && !tf.is_hlg()
        && !tf.is_dci()
        && !tf.is709()
    {
        return false;
    }
    if desired.is_gray() && desired.get_white_point_type() != WhitePoint::D65 {
        return false;
    }
    true
}
impl OutputEncodingInfo {
    pub fn set_from_metadata(&mut self, metadata: &CodecMetadata) -> Status {
        self.orig_color_encoding = metadata.m.color_encoding.clone();
        self.orig_intensity_target = metadata.m.intensity_target();
        self.desired_intensity_target = self.orig_intensity_target;
        let im = &metadata.transform_data.opsin_inverse_matrix;
        self.orig_inverse_matrix = im.inverse_matrix;
        self.default_transform = im.all_default;
        self.xyb_encoded = metadata.m.xyb_encoded;
        self.opsin_params.opsin_biases[..3].copy_from_slice(&im.opsin_biases);
        for i in 0..3 {
            self.opsin_params.opsin_biases_cbrt[i] = self.opsin_params.opsin_biases[i].cbrt();
        }
        self.opsin_params.opsin_biases_cbrt[3] = 1.0;
        self.opsin_params.opsin_biases[3] = 1.0;
        self.opsin_params.quant_biases = im.quant_biases;
        let orig_ok = can_output_to_color_encoding(&self.orig_color_encoding);
        let orig_grey = self.orig_color_encoding.is_gray();
        // This is SetColorEncoding's original first assignment. Resolving the
        // same-owner original argument here avoids cloning it a second time.
        self.color_encoding = if !self.xyb_encoded || orig_ok {
            self.orig_color_encoding.clone()
        } else {
            ColorEncoding::linear_srgb(orig_grey).clone()
        };
        self.set_color_encoding_fields()
    }
    pub fn maybe_set_color_encoding(&mut self, desired: &ColorEncoding) -> Status {
        if desired.get_color_space() == ColorSpace::XYB
            && ((self.color_encoding.get_color_space() == ColorSpace::RGB
                && self.color_encoding.get_primaries_type() != Primaries::SRGB)
                || self.color_encoding.tf().is_pq())
        {
            return Err(StatusCode::GenericError);
        }
        if !self.xyb_encoded && !can_output_to_color_encoding(desired) {
            return Err(StatusCode::GenericError);
        }
        self.color_encoding = desired.clone();
        self.set_color_encoding_fields()
    }
    fn set_color_encoding_fields(&mut self) -> Status {
        // After its first copy, the original const argument and this field have
        // identical values and neither changes during the remaining method.
        let desired = &self.color_encoding;
        self.linear_color_encoding = self.color_encoding.clone();
        self.linear_color_encoding
            .tf_mut()
            .set_transfer_function(TransferFunction::LINEAR);
        self.color_encoding_is_original = self.orig_color_encoding.same_color_encoding(desired);
        let mut inverse_matrix_is_default = self.default_transform;
        let mut inverse_matrix = self.orig_inverse_matrix;
        self.luminances = [0.2126, 0.7152, 0.0722];
        if (desired.get_primaries_type() != Primaries::SRGB
            || desired.get_white_point_type() != WhitePoint::D65)
            && !desired.is_gray()
        {
            let mut srgb_to_xyzd50 = [[0.0; 3]; 3];
            let srgb = ColorEncoding::srgb(false);
            let mut p = PrimariesCieXy::default();
            srgb.get_primaries(&mut p)?;
            let w = srgb.get_white_point();
            primaries_to_xyz_d50(
                p.r.x as f32,
                p.r.y as f32,
                p.g.x as f32,
                p.g.y as f32,
                p.b.x as f32,
                p.b.y as f32,
                w.x as f32,
                w.y as f32,
                &mut srgb_to_xyzd50,
            )?;
            let mut original_to_xyz = [[0.0; 3]; 3];
            desired.get_primaries(&mut p)?;
            let w = desired.get_white_point();
            primaries_to_xyz(
                p.r.x as f32,
                p.r.y as f32,
                p.g.x as f32,
                p.g.y as f32,
                p.b.x as f32,
                p.b.y as f32,
                w.x as f32,
                w.y as f32,
                &mut original_to_xyz,
            )?;
            self.luminances = original_to_xyz[1];
            if self.xyb_encoded {
                let mut adapt = [[0.0; 3]; 3];
                adapt_to_xyz_d50(
                    desired.get_white_point().x as f32,
                    desired.get_white_point().y as f32,
                    &mut adapt,
                )?;
                let mut xyzd50_to_original = [[0.0; 3]; 3];
                mul3x3_matrix(&adapt, &original_to_xyz, &mut xyzd50_to_original);
                inv3x3_matrix(&mut xyzd50_to_original)?;
                let mut srgb_to_original = [[0.0; 3]; 3];
                mul3x3_matrix(&xyzd50_to_original, &srgb_to_xyzd50, &mut srgb_to_original);
                mul3x3_matrix(
                    &srgb_to_original,
                    &self.orig_inverse_matrix,
                    &mut inverse_matrix,
                );
                inverse_matrix_is_default = false;
            }
        }
        if desired.is_gray() {
            let tmp_inverse = inverse_matrix;
            let to_luma = [self.luminances; 3];
            mul3x3_matrix(&to_luma, &tmp_inverse, &mut inverse_matrix);
        }
        if self.xyb_encoded {
            init_simd_inverse_matrix(
                &inverse_matrix,
                &mut self.opsin_params.inverse_opsin_matrix,
                self.orig_intensity_target,
            );
            self.all_default_opsin = (f64::from(self.orig_intensity_target) - 255.0).abs()
                <= f64::from(0.1f32)
                && inverse_matrix_is_default;
        }
        let tf = desired.tf();
        self.inverse_gamma = if tf.have_gamma {
            tf.get_gamma() as f32
        } else if tf.is_dci() {
            1.0f32 / 2.6f32
        } else {
            1.0
        };
        Ok(())
    }
}
