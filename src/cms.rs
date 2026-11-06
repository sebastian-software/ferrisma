//! Optional, explicit embedded-ICC -> linear-light sRGB pixel transform.
//!
//! No ICC profile is reinterpreted as sRGB. The caller must explicitly opt
//! into using the decoder's **rendered output ICC** signal. The transform
//! operates on borrowed input rows and engine-budgeted f32 scratch buffers.
//! The codec's own image allocations remain independently opaque.

use std::sync::Arc;

use moxcms::{ColorProfile, Layout, ToneReprCurve, TransformF32Executor, TransformOptions};

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::Rect;
use crate::memory::MemoryBudget;

pub(crate) struct IccSrgbSource<'a> {
    source: Box<dyn DecodeSource + 'a>,
    output: DecoderInfo,
    transformer: Arc<TransformF32Executor>,
}

impl<'a> IccSrgbSource<'a> {
    pub(crate) fn new(source: Box<dyn DecodeSource + 'a>) -> Result<Self, Error> {
        let info = source.info();
        if info.layout != ChannelLayout::Rgb || info.alpha != AlphaMode::Opaque {
            return Err(Error::UnsupportedCodecLayout);
        }
        let bytes = info
            .rendered_icc
            .as_deref()
            .ok_or(Error::UnsupportedColorEncoding)?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::UnsupportedColorEncoding);
        }
        let src =
            ColorProfile::new_from_slice(bytes).map_err(|_| Error::UnsupportedColorEncoding)?;
        // Linear-light sRGB: the working space of the resample chain, so
        // the executor neither re-encodes nor re-decodes the transfer curve
        // (an empty LUT is the identity curve, as in moxcms' linear ACEScg).
        let mut dst = ColorProfile::new_srgb();
        let linear = ToneReprCurve::Lut(Vec::new());
        dst.red_trc = Some(linear.clone());
        dst.green_trc = Some(linear.clone());
        dst.blue_trc = Some(linear);
        dst.cicp = None;
        let transformer = src
            .create_transform_f32(Layout::Rgb, &dst, Layout::Rgb, TransformOptions::default())
            .map_err(|_| Error::UnsupportedColorEncoding)?;
        let mut output = info.clone();
        output.sample = SampleType::F32;
        output.rendered_icc = None;
        output.rendered_cicp = Some([1, 8, 0, 1]);
        Ok(Self {
            source,
            output,
            transformer,
        })
    }
}
impl DecodeSource for IccSrgbSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.output
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        let len = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(3)
            .ok_or(Error::GeometryOverflow)?;
        let mut input = budget.allocate_typed::<f32>(len)?;
        let mut output = budget.allocate_typed::<f32>(len)?;
        let expected_sample = self.source.info().sample;
        let mut report = self.source.read_region(region, budget, &mut |y, row| {
            if row.len() != len || row.sample_type() != expected_sample {
                return Err(Error::IncompleteDecoderOutput);
            }
            match row {
                DecodedRow::U8(values) => {
                    for (dst, src) in input.as_mut_slice().iter_mut().zip(values) {
                        *dst = f32::from(*src) / 255.0;
                    }
                }
                DecodedRow::F32(values) => input.as_mut_slice().copy_from_slice(values),
            }
            self.transformer
                .transform(input.as_slice(), output.as_mut_slice())
                .map_err(|_| Error::UnsupportedColorEncoding)?;
            sink(y, DecodedRow::F32(output.as_slice()))
        })?;
        report.output_sample_bytes = report
            .rows
            .checked_mul(u64::from(region.width))
            .and_then(|n| n.checked_mul(12))
            .ok_or(Error::GeometryOverflow)?;
        report.peak_engine_managed_bytes = budget.snapshot().peak;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::IccSrgbSource;
    use crate::color::{AlphaMode, ColorEncoding};
    use crate::decoder::{
        ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
        SampleType,
    };
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;
    use crate::working_color::decode_to_linear_srgb;

    /// One row holding every value of a coarse 8-bit RGB grid.
    struct Grid {
        info: DecoderInfo,
        row: Vec<u8>,
    }

    impl DecodeSource for Grid {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            sink(0, DecodedRow::U8(&self.row))?;
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: 1,
                output_sample_bytes: 0,
                known_full_frame_buffers: 0,
                peak_engine_managed_bytes: 0,
                opaque_codec_memory_unbounded: false,
                engine_buffer_bytes: 0,
            })
        }
    }

    #[test]
    fn display_p3_icc_becomes_linear_srgb_matching_the_reference_matrix() {
        let icc = moxcms::ColorProfile::new_display_p3().encode().unwrap();
        let steps: Vec<u8> = (0..=255).step_by(17).collect();
        let mut row = Vec::new();
        for &r in &steps {
            for &g in &steps {
                for &b in &steps {
                    row.extend([r, g, b]);
                }
            }
        }
        let width = u32::try_from(row.len() / 3).unwrap();
        let grid = Grid {
            info: DecoderInfo {
                size: Size::new(width, 1).unwrap(),
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: Some(icc),
                rendered_cicp: None,
            },
            row: row.clone(),
        };
        let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
        // The adapter reports what it emits: linear-light sRGB.
        assert_eq!(source.info().rendered_cicp, Some([1, 8, 0, 1]));
        assert_eq!(source.info().rendered_icc, None);
        let mut worst = 0.0_f64;
        let mut worst_clipped = 0.0_f64;
        let mut clipped = 0_usize;
        source
            .read_region(
                Rect::new(0, 0, width, 1).unwrap(),
                &MemoryBudget::new(1 << 20),
                &mut |_, out| {
                    let DecodedRow::F32(out) = out else {
                        panic!("the adapter emits f32");
                    };
                    for (pixel, linear) in row.as_chunks::<3>().0.iter().zip(out.as_chunks::<3>().0)
                    {
                        let reference = decode_to_linear_srgb(
                            &ColorEncoding::Cicp([12, 13, 0, 1]),
                            pixel.map(|v| f64::from(v) / 255.0),
                        )
                        .unwrap();
                        let in_gamut = reference.rgb.iter().all(|v| (0.0..=1.0).contains(v));
                        for (expected, &actual) in reference.rgb.iter().zip(linear) {
                            let difference = (expected - f64::from(actual)).abs();
                            if in_gamut {
                                worst = worst.max(difference);
                            } else {
                                worst_clipped = worst_clipped.max(difference);
                                // Out of the sRGB gamut, moxcms clamps to
                                // [0, 1]; the reference keeps the value.
                                let clamped = expected.clamp(0.0, 1.0);
                                assert!(
                                    (clamped - f64::from(actual)).abs() < 2.0e-3,
                                    "{expected} vs {actual}"
                                );
                            }
                        }
                        clipped += usize::from(!in_gamut);
                    }
                    Ok(())
                },
            )
            .unwrap();
        // moxcms (Q1.30 matrix shaper) against the CSS Color 4 matrices:
        // a few 1e-4 in linear light is below one 8-bit sRGB code at the
        // bright end and far below it in the dark range, where the linear
        // segment dominates.
        assert!(worst < 2.0e-3, "worst in-gamut linear difference {worst}");
        eprintln!(
            "display P3 ICC via moxcms vs reference: in gamut worst {worst:e}; {clipped} out-of-gamut pixels clamped, worst {worst_clipped:e}"
        );
    }
}
