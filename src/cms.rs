//! Optional, explicit embedded-ICC -> linear-light sRGB pixel transform.
//!
//! No ICC profile is reinterpreted as sRGB. The caller must explicitly opt
//! into using the decoder's **rendered output ICC** signal. The transform
//! operates on borrowed input rows and engine-budgeted f32 scratch buffers.
//! The codec's own image allocations remain independently opaque.

use std::sync::Arc;

use moxcms::{
    ColorProfile, DataColorSpace, Layout, ToneReprCurve, TransformF32Executor, TransformOptions,
};

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, RowConversion, SampleType,
};
use crate::error::Error;
use crate::geometry::Rect;
use crate::memory::MemoryBudget;

pub(crate) struct IccSrgbSource<'a> {
    source: Box<dyn DecodeSource + 'a>,
    output: DecoderInfo,
    conversion: Arc<IccConversion>,
    /// The executor took the conversion; rows pass through unconverted.
    deferred: bool,
}

/// The moxcms transform of one profile into linear-light sRGB, per row.
pub(crate) struct IccConversion {
    transformer: Arc<TransformF32Executor>,
    input: SampleType,
}

impl RowConversion for IccConversion {
    fn input_sample(&self) -> SampleType {
        self.input
    }

    fn convert(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        if row.len() != out.len() || row.len() != scratch.len() || row.sample_type() != self.input {
            return Err(Error::IncompleteDecoderOutput);
        }
        let input: &[f32] = match row {
            DecodedRow::U8(values) => {
                for (dst, src) in scratch.iter_mut().zip(values) {
                    *dst = f32::from(*src) / 255.0;
                }
                scratch
            }
            DecodedRow::F32(values) => values,
        };
        self.transformer
            .transform(input, out)
            .map_err(|_| Error::UnsupportedColorEncoding)
    }
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
        // moxcms' extended floating-point path avoids the destination-gamut
        // clamp for matrix-shaper profiles. Keep its existing LUT path and
        // intent selection unchanged; those profiles need separate evidence.
        let extended_matrix_shaper = src.pcs == DataColorSpace::Xyz
            && src.is_matrix_shaper()
            && src.lut_a_to_b_perceptual.is_none()
            && src.lut_a_to_b_colorimetric.is_none()
            && src.lut_a_to_b_saturation.is_none();
        let options = TransformOptions {
            allow_extended_range_rgb_xyz: extended_matrix_shaper,
            ..TransformOptions::default()
        };
        let transformer = src
            .create_transform_f32(Layout::Rgb, &dst, Layout::Rgb, options)
            .map_err(|_| Error::UnsupportedColorEncoding)?;
        let conversion = Arc::new(IccConversion {
            transformer,
            input: info.sample,
        });
        let mut output = info.clone();
        output.sample = SampleType::F32;
        output.rendered_icc = None;
        output.rendered_cicp = Some([1, 8, 0, 1]);
        Ok(Self {
            source,
            output,
            conversion,
            deferred: false,
        })
    }
}
impl DecodeSource for IccSrgbSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.output
    }
    fn defer_conversion(&mut self) -> Option<Arc<dyn RowConversion>> {
        self.deferred = true;
        Some(self.conversion.clone())
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
        let expected_sample = self.source.info().sample;
        let mut report = if self.deferred {
            self.source.read_region(region, budget, &mut |y, row| {
                if row.len() != len || row.sample_type() != expected_sample {
                    return Err(Error::IncompleteDecoderOutput);
                }
                sink(y, row)
            })?
        } else {
            let mut scratch = budget.allocate_typed::<f32>(len)?;
            let mut output = budget.allocate_typed::<f32>(len)?;
            let conversion = &*self.conversion;
            self.source.read_region(region, budget, &mut |y, row| {
                conversion.convert(row, scratch.as_mut_slice(), output.as_mut_slice())?;
                sink(y, DecodedRow::F32(output.as_slice()))
            })?
        };
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
                engine_copied_bytes: 0,
                work: crate::decoder::CodecWork::region_at_least(region),
            })
        }
    }

    struct FloatRow {
        info: DecoderInfo,
        row: Vec<f32>,
    }

    impl DecodeSource for FloatRow {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            sink(0, DecodedRow::F32(&self.row))?;
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: 1,
                output_sample_bytes: 0,
                known_full_frame_buffers: 0,
                peak_engine_managed_bytes: 0,
                opaque_codec_memory_unbounded: false,
                engine_buffer_bytes: 0,
                engine_copied_bytes: 0,
                work: crate::decoder::CodecWork::region_at_least(region),
            })
        }
    }

    fn p3_grid(row: Vec<u8>) -> Grid {
        Grid {
            info: DecoderInfo {
                size: Size::new(u32::try_from(row.len() / 3).unwrap(), 1).unwrap(),
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: Some(moxcms::ColorProfile::new_display_p3().encode().unwrap()),
                rendered_cicp: None,
            },
            row,
        }
    }

    fn read_rows(source: &mut IccSrgbSource<'_>, width: u32) -> Vec<Vec<f32>> {
        let mut rows = Vec::new();
        source
            .read_region(
                Rect::new(0, 0, width, 1).unwrap(),
                &MemoryBudget::new(1 << 20),
                &mut |_, row| {
                    rows.push(match row {
                        DecodedRow::F32(values) => values.to_vec(),
                        DecodedRow::U8(values) => values.iter().map(|&v| f32::from(v)).collect(),
                    });
                    Ok(())
                },
            )
            .unwrap();
        rows
    }

    #[test]
    fn deferred_conversion_reproduces_the_adapter_output_exactly() {
        let row: Vec<u8> = (0..=255_u8).flat_map(|v| [v, 255 - v, v / 2]).collect();
        let width = u32::try_from(row.len() / 3).unwrap();
        let converted = read_rows(
            &mut IccSrgbSource::new(Box::new(p3_grid(row.clone()))).unwrap(),
            width,
        );

        let mut source = IccSrgbSource::new(Box::new(p3_grid(row.clone()))).unwrap();
        let conversion = source.defer_conversion().expect("the ICC adapter defers");
        assert_eq!(conversion.input_sample(), SampleType::U8);
        // The info still describes converted rows; the rows pass through.
        assert_eq!(source.info().sample, SampleType::F32);
        let raw = read_rows(&mut source, width);
        let expected_raw: Vec<f32> = row.iter().map(|&v| f32::from(v)).collect();
        assert_eq!(raw, vec![expected_raw]);

        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(
            out.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            converted[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        // A row of the wrong type or length is rejected.
        assert_eq!(
            conversion.convert(DecodedRow::F32(&out.clone()), &mut scratch, &mut out),
            Err(Error::IncompleteDecoderOutput)
        );
        assert_eq!(
            conversion.convert(DecodedRow::U8(&row[3..]), &mut scratch, &mut out),
            Err(Error::IncompleteDecoderOutput)
        );
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
        let mut worst_extended = 0.0_f64;
        let mut extended = 0_usize;
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
                                worst_extended = worst_extended.max(difference);
                                assert!(difference < 2.0e-3, "{expected} vs {actual}");
                            }
                        }
                        extended += usize::from(!in_gamut);
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
        assert!(extended > 0, "grid must include out-of-sRGB-gamut pixels");
        eprintln!(
            "display P3 ICC via moxcms vs reference: in gamut worst {worst:e}; {extended} out-of-gamut pixels retained, worst {worst_extended:e}"
        );
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn independent_littlecms_profiles_preserve_extended_linear_samples() {
        // Profiles and expected values are generated by LittleCMS 2.19, not
        // moxcms. See tests/fixtures/icc/README.md for the exact recipe.
        const INPUT: [u8; 24] = [
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 51, 102, 153,
            255, 255, 255,
        ];
        const P3: [[f32; 3]; 8] = [
            [1.22493947, -0.0420472585, -0.0196422264],
            [-0.224948391, 1.04206085, -0.078627646],
            [-0.00000937440291, 0.000013065016, 1.09827411],
            [-0.224957764, 1.04207385, 1.01964653],
            [1.22493005, -0.0420341939, 1.078632],
            [0.999991059, 1.00001359, -0.0982698724],
            [0.00554445293, 0.137596995, 0.345936954],
            [0.999981701, 1.00002658, 1.00000429],
        ];
        const ADOBE: [[f32; 3]; 8] = [
            [1.39835644, -0.00000232951379, 0.00000708362313],
            [-0.398333818, 0.999988735, -0.0429379642],
            [-0.00000875071964, 0.0000104610817, 1.04291725],
            [-0.39834258, 0.999999166, 0.999979258],
            [1.39834774, 0.00000813156839, 1.04292428],
            [1.00002265, 0.99998641, -0.0429308824],
            [-0.0125112822, 0.133305743, 0.333398402],
            [1.00001383, 0.999996841, 0.999986351],
        ];
        for (name, icc, expected) in [
            (
                "Display P3 gamma 2.2",
                &include_bytes!("../tests/fixtures/icc/display-p3-gamma22.icc")[..],
                P3,
            ),
            (
                "Adobe RGB 1998",
                &include_bytes!("../tests/fixtures/icc/adobe-rgb-1998.icc")[..],
                ADOBE,
            ),
        ] {
            let grid = Grid {
                info: DecoderInfo {
                    size: Size::new(8, 1).unwrap(),
                    layout: ChannelLayout::Rgb,
                    sample: SampleType::U8,
                    access: DecodeAccess::CodecRegionRequest,
                    alpha: AlphaMode::Opaque,
                    rendered_icc: Some(icc.to_vec()),
                    rendered_cicp: None,
                },
                row: INPUT.to_vec(),
            };
            let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
            let rows = read_rows(&mut source, 8);
            let actual = &rows[0];
            let mut worst_in_gamut = 0.0_f32;
            let mut worst_out_of_gamut = 0.0_f32;
            for (index, (pixel, oracle)) in
                actual.as_chunks::<3>().0.iter().zip(expected).enumerate()
            {
                let out_of_gamut = oracle.iter().any(|&v| !(0.0..=1.0).contains(&v));
                for (channel, (&actual, expected)) in pixel.iter().zip(oracle).enumerate() {
                    if out_of_gamut {
                        worst_out_of_gamut = worst_out_of_gamut.max((actual - expected).abs());
                    } else {
                        worst_in_gamut = worst_in_gamut.max((actual - expected).abs());
                    }
                    assert!(
                        (actual - expected).abs() < 8.0e-4,
                        "{name} pixel {index} channel {channel}: {actual} vs {expected}"
                    );
                }
            }
            eprintln!("{name}: in gamut {worst_in_gamut:e}, out of gamut {worst_out_of_gamut:e}");
            assert!(actual.iter().any(|&v| v < -0.01));
            assert!(actual.iter().any(|&v| v > 1.01));
        }
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn float_input_and_deferred_conversion_keep_extended_values() {
        let icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let info = DecoderInfo {
            size: Size::new(2, 1).unwrap(),
            layout: ChannelLayout::Rgb,
            sample: SampleType::F32,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Opaque,
            rendered_icc: Some(icc),
            rendered_cicp: None,
        };
        let row = vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.0];
        let direct = read_rows(
            &mut IccSrgbSource::new(Box::new(FloatRow {
                info: info.clone(),
                row: row.clone(),
            }))
            .unwrap(),
            2,
        );
        let mut deferred = IccSrgbSource::new(Box::new(FloatRow {
            info,
            row: row.clone(),
        }))
        .unwrap();
        let conversion = deferred.defer_conversion().unwrap();
        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::F32(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out, direct[0]);
        assert!((out[0] - 1.22494018).abs() < 2.0e-3);
        assert!((out[3] + 0.224940181).abs() < 2.0e-3);
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn external_profiles_match_littlecms_including_srgb_knee() {
        // These are the exact CC0 profile bytes reopened by the LittleCMS
        // oracle, including the sRGB parametric TRC of DisplayP3-v4.
        const INPUT: [u8; 30] = [
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 51, 102, 153,
            255, 255, 255, 10, 11, 12, 11, 10, 9,
        ];
        const P3: [[f32; 3]; 10] = [
            [1.22493947, -0.0420472585, -0.0196422264],
            [-0.224948391, 1.04206085, -0.078627646],
            [0.0000153022465, -0.0000161728094, 1.09827769],
            [-0.224933088, 1.04204464, 1.01964998],
            [1.22495472, -0.0420634337, 1.07863545],
            [0.999991059, 1.00001359, -0.0982698724],
            [0.010667868, 0.137058184, 0.338752538],
            [1.00000644, 0.999997377, 1.00000775],
            [0.0029649022, 0.00335985888, 0.0037153177],
            [0.00341690634, 0.00302189123, 0.00269557559],
        ];
        const ADOBE: [[f32; 3]; 10] = [
            [1.39835644, -0.00000232951379, 0.00000708362313],
            [-0.398333818, 0.999988735, -0.0429379642],
            [-0.0000162381166, 0.0000109712355, 1.04293871],
            [-0.39835006, 0.999999702, 1.00000072],
            [1.39834023, 0.00000864172125, 1.04294574],
            [1.00002265, 0.99998641, -0.0429308824],
            [-0.0125137167, 0.133305907, 0.333405375],
            [1.00000644, 0.999997377, 1.00000775],
            [0.000731762324, 0.00099481456, 0.0012136274],
            [0.00106976007, 0.000806692697, 0.000632695912],
        ];
        for (name, icc, expected) in [
            (
                "DisplayP3-v4",
                &include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc")[..],
                P3,
            ),
            (
                "AdobeCompat-v2",
                &include_bytes!("../tests/fixtures/icc/AdobeCompat-v2.icc")[..],
                ADOBE,
            ),
        ] {
            let grid = Grid {
                info: DecoderInfo {
                    size: Size::new(10, 1).unwrap(),
                    layout: ChannelLayout::Rgb,
                    sample: SampleType::U8,
                    access: DecodeAccess::CodecRegionRequest,
                    alpha: AlphaMode::Opaque,
                    rendered_icc: Some(icc.to_vec()),
                    rendered_cicp: None,
                },
                row: INPUT.to_vec(),
            };
            let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
            let actual = read_rows(&mut source, 10).pop().unwrap();
            let mut worst_in_gamut = 0.0_f32;
            let mut worst_out_of_gamut = 0.0_f32;
            let mut worst_knee = 0.0_f32;
            for (index, (pixel, oracle)) in
                actual.as_chunks::<3>().0.iter().zip(expected).enumerate()
            {
                let out_of_gamut = oracle.iter().any(|&v| !(0.0..=1.0).contains(&v));
                for (channel, (&actual, expected)) in pixel.iter().zip(oracle).enumerate() {
                    let difference = (actual - expected).abs();
                    if out_of_gamut {
                        worst_out_of_gamut = worst_out_of_gamut.max(difference);
                    } else {
                        worst_in_gamut = worst_in_gamut.max(difference);
                    }
                    if index >= 8 {
                        worst_knee = worst_knee.max(difference);
                    }
                    let limit = if index >= 8 { 1.0e-5 } else { 8.0e-4 };
                    assert!(
                        difference < limit,
                        "{name} pixel {index} channel {channel}: {actual} vs {expected} (limit {limit})"
                    );
                }
            }
            eprintln!(
                "{name}: in gamut {worst_in_gamut:e}, out of gamut {worst_out_of_gamut:e}, knee {worst_knee:e}"
            );
            assert!(actual.iter().any(|&v| v < -0.01));
            assert!(actual.iter().any(|&v| v > 1.01));
        }
    }
}
