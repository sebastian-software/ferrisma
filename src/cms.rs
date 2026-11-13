//! Optional, explicit embedded-ICC -> linear-light sRGB pixel transform.
//!
//! No ICC profile is reinterpreted as sRGB. The caller must explicitly opt
//! into using the decoder's **rendered output ICC** signal. The transform
//! operates on borrowed input rows and engine-budgeted f32 scratch buffers.
//! The codec's own image allocations remain independently opaque.
//!
//! Opaque RGB and RGBA with **straight** alpha are accepted (issue #222).
//! Only the three color channels go through the profile transform; the
//! alpha sample passes through unchanged (8-bit alpha normalized to `[0, 1]`
//! like the color channels, float alpha copied as is), so the output stays
//! linear-light sRGB with straight alpha (ADR-0014). Premultiplied and
//! unspecified alpha are refused rather than guessed (ADR-0015).

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
    /// Interleaved samples per pixel: 3 (opaque RGB) or 4 (RGB followed by
    /// straight alpha, which bypasses the transform).
    channels: usize,
    /// Per-channel contributions for 8-bit matrix-shaper input; `None`
    /// keeps every row on the moxcms transform.
    table: Option<ChannelTable>,
}

/// Largest absolute difference, in linear light, that the per-channel table
/// may have from moxcms on any probed 8-bit triplet. Measured worst case over
/// all 2^24 triplets: 1.2e-7 (ADR-0016 and ADR-0023 amendments, 2026-11-12;
/// pending owner confirmation).
const CHANNEL_TABLE_BOUND: f32 = 2.5e-7;

/// 8-bit matrix-shaper conversion as a sum of per-channel contributions
/// (issue #219, pending owner confirmation in ADR-0016 and ADR-0023).
///
/// A matrix-shaper profile into linear sRGB computes `M · [trc_r(r),
/// trc_g(g), trc_b(b)]`: each output is a weighted sum of three independent
/// per-channel curves. With 8-bit input each channel has 256 values, so the
/// transform is the sum of three entries of a 768-entry table. The entries
/// are moxcms's own outputs for single-channel input (`[v, 0, 0]`,
/// `[0, v, 0]`, `[0, 0, v]`), so the table carries moxcms's curves and
/// matrix; only the order of the final f32 additions differs. That holds
/// only if moxcms maps black to zero and adds nothing non-separable (no
/// clamp, no gamut mapping, no black-point term), so the table is built
/// only for the extended-range matrix-shaper path, and every table is
/// checked against moxcms on a dense probe set when it is built; any
/// difference above `CHANNEL_TABLE_BOUND` keeps the moxcms path.
struct ChannelTable(Box<[[f32; 3]; 768]>);

impl ChannelTable {
    /// The probe set of `ChannelTable::build`: every single-channel and
    /// gray value, and a 17³ grid (every 16th code and 255).
    fn probes() -> Vec<[u8; 3]> {
        let mut probes = Vec::with_capacity(4 * 256 + 17 * 17 * 17);
        for value in 0..=255_u8 {
            probes.extend([[value, 0, 0], [0, value, 0], [0, 0, value], [value; 3]]);
        }
        let grid: Vec<u8> = (0..=16_u16)
            .map(|step| u8::try_from((step * 16).min(255)).unwrap_or(u8::MAX))
            .collect();
        for &r in &grid {
            for &g in &grid {
                for &b in &grid {
                    probes.push([r, g, b]);
                }
            }
        }
        probes
    }

    fn build(transformer: &TransformF32Executor) -> Option<Self> {
        let mut input = Vec::with_capacity(768 * 3);
        for channel in 0..3 {
            for value in 0..=255_u8 {
                let mut pixel = [0.0_f32; 3];
                pixel[channel] = f32::from(value) / 255.0;
                input.extend_from_slice(&pixel);
            }
        }
        let mut output = vec![0.0_f32; input.len()];
        transformer.transform(&input, &mut output).ok()?;
        let table = Self(
            output
                .as_chunks::<3>()
                .0
                .to_vec()
                .into_boxed_slice()
                .try_into()
                .ok()?,
        );
        // Black must contribute nothing, or the sum would count it thrice.
        if [0, 256, 512]
            .iter()
            .any(|&black| table.0.get(black) != Some(&[0.0; 3]))
        {
            return None;
        }
        let probes = Self::probes();
        let input: Vec<f32> = probes
            .iter()
            .flatten()
            .map(|&value| f32::from(value) / 255.0)
            .collect();
        let mut exact = vec![0.0_f32; input.len()];
        transformer.transform(&input, &mut exact).ok()?;
        for (pixel, want) in probes.iter().zip(exact.as_chunks::<3>().0) {
            let got = table.apply(*pixel);
            if got.iter().zip(want).any(|(got, want)| {
                let d = (got - want).abs();
                d.is_nan() || d > CHANNEL_TABLE_BOUND
            }) {
                return None;
            }
        }
        Some(table)
    }

    fn apply(&self, [r, g, b]: [u8; 3]) -> [f32; 3] {
        let r = self.0[usize::from(r)];
        let g = self.0[256 + usize::from(g)];
        let b = self.0[512 + usize::from(b)];
        [r[0] + g[0] + b[0], r[1] + g[1] + b[1], r[2] + g[2] + b[2]]
    }
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
        if row.len() != out.len()
            || row.len() != scratch.len()
            || row.sample_type() != self.input
            || !row.len().is_multiple_of(self.channels)
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        if self.channels == 4 {
            return self.convert_straight_rgba(row, scratch, out);
        }
        if let (Some(table), DecodedRow::U8(values)) = (&self.table, row) {
            for (dst, src) in out
                .as_chunks_mut::<3>()
                .0
                .iter_mut()
                .zip(values.as_chunks::<3>().0)
            {
                *dst = table.apply(*src);
            }
            return Ok(());
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

impl IccConversion {
    /// RGB plus straight alpha: the color channels are converted exactly as
    /// opaque RGB is (the channel table for 8-bit matrix-shaper input,
    /// moxcms otherwise), and the alpha sample is carried over unchanged.
    /// The caller has checked the row length and sample type.
    fn convert_straight_rgba(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        let pixels = row.len() / 4;
        if let (Some(table), DecodedRow::U8(values)) = (&self.table, row) {
            for (dst, src) in out
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(values.as_chunks::<4>().0)
            {
                let [r, g, b] = table.apply([src[0], src[1], src[2]]);
                *dst = [r, g, b, f32::from(src[3]) / 255.0];
            }
            return Ok(());
        }
        // Pack the color channels into the first three quarters of
        // `scratch`, transform them into the first three quarters of `out`,
        // then widen `out` to RGBA in place from the last pixel backwards:
        // pixel `i` is written at `4i..4i + 4` after it was read at
        // `3i..3i + 3`, and every pixel still to be read lies below `3i`.
        let color = scratch
            .get_mut(..pixels * 3)
            .ok_or(Error::IncompleteDecoderOutput)?;
        match row {
            DecodedRow::U8(values) => {
                for (dst, src) in color
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<4>().0)
                {
                    *dst = [src[0], src[1], src[2]].map(|v| f32::from(v) / 255.0);
                }
            }
            DecodedRow::F32(values) => {
                for (dst, src) in color
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<4>().0)
                {
                    *dst = [src[0], src[1], src[2]];
                }
            }
        }
        self.transformer
            .transform(color, &mut out[..pixels * 3])
            .map_err(|_| Error::UnsupportedColorEncoding)?;
        for pixel in (0..pixels).rev() {
            let alpha = match row {
                DecodedRow::U8(values) => f32::from(values[pixel * 4 + 3]) / 255.0,
                DecodedRow::F32(values) => values[pixel * 4 + 3],
            };
            let rgb = [out[pixel * 3], out[pixel * 3 + 1], out[pixel * 3 + 2]];
            out[pixel * 4..pixel * 4 + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], alpha]);
        }
        Ok(())
    }
}

impl<'a> IccSrgbSource<'a> {
    pub(crate) fn new(source: Box<dyn DecodeSource + 'a>) -> Result<Self, Error> {
        let info = source.info();
        // Opaque RGB, or RGB with straight alpha whose sample bypasses the
        // transform (issue #222). Premultiplied color would have to be
        // divided by alpha before a nonlinear profile curve, and unspecified
        // alpha has no defined meaning; neither is guessed (ADR-0015).
        let channels = match (info.layout, info.alpha) {
            (ChannelLayout::Rgb, AlphaMode::Opaque) => 3,
            (ChannelLayout::Rgba, AlphaMode::Straight) => 4,
            (ChannelLayout::Rgba, _) => return Err(Error::UnsupportedAlphaMode),
            _ => return Err(Error::UnsupportedCodecLayout),
        };
        let bytes = info
            .rendered_icc
            .as_deref()
            .ok_or(Error::UnsupportedColorEncoding)?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::UnsupportedColorEncoding);
        }
        // The same bounded header check as the color contract (ADR-0015):
        // moxcms 0.9.1 accepts a header whose declared size exceeds the data.
        crate::color_contract::IccHeader::inspect(bytes)
            .map_err(|_| Error::UnsupportedColorEncoding)?;
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
        // CICP evidence beside the profile must agree with what the profile
        // does (issues #17 and #41). The decoder's own CICP is independent
        // evidence and is always checked. The profile's `cicp` tag is checked
        // in full only when it is a well-formed RGB tag (matrix 0, full
        // range); profiles in the wild carry miscoded tags (moxcms 0.9.1's
        // own Display P3 profile writes `[11, 13, 1, 0]`), so a malformed tag
        // is not trusted as a description, but its PQ/HLG transfer is still
        // honored as a reason to reject.
        if let Some(signal) = info.rendered_cicp {
            check_cicp_agrees(signal, transformer.as_ref())?;
        }
        if let Some(cicp) = src.cicp.as_ref() {
            let tag = [
                cicp.color_primaries as u8,
                cicp.transfer_characteristics as u8,
                cicp.matrix_coefficients as u8,
                u8::from(cicp.full_range),
            ];
            if tag[2] == 0 && tag[3] == 1 {
                check_cicp_agrees(tag, transformer.as_ref())?;
            } else if matches!(tag[1], 16 | 18) {
                return Err(Error::CicpBesideIcc(HDR));
            }
        }
        let table = if info.sample == SampleType::U8 && extended_matrix_shaper {
            ChannelTable::build(transformer.as_ref())
        } else {
            None
        };
        let conversion = Arc::new(IccConversion {
            transformer,
            input: info.sample,
            channels,
            table,
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
/// Fail closed unless a CICP signal next to the rendered ICC is an SDR
/// family the engine can evaluate itself and the profile transform agrees
/// with it (issues #17 and #41).
///
/// The rendered-ICC policy makes the profile authoritative for the
/// conversion, but it must not silently override contradicting evidence:
/// - PQ and HLG need reference-white and tone-mapping decisions (ADR-0016
///   rule 7). An ICC approximation of those curves, such as the profiles
///   JPEG XL decoders synthesize for PQ/HLG, would be converted as if it were
///   SDR, so they are rejected.
/// - Code points outside sRGB, linear sRGB and Display P3 cannot be checked
///   here and are rejected rather than ignored.
/// - For the supported families, probe colors through the profile must match
///   the engine's own CICP reference within `2e-3` linear (the moxcms versus
///   reference P3 matrix bound of ADR-0023); otherwise the two signals
///   contradict each other.
///
/// MIAF gives an AVIF ICC `colr` precedence over `nclx` for color
/// interpretation; the engine does not apply that precedence silently. Each
/// refusal is a `CicpBesideIcc` that names the reason (review of 2026-11-12;
/// the outcome for SDR transfers outside these families, such as BT.709
/// transfer 1 beside an sRGB profile, awaits owner confirmation in ADR-0015).
fn check_cicp_agrees(raw: [u8; 4], transformer: &TransformF32Executor) -> Result<(), Error> {
    use crate::color::ColorEncoding;
    use crate::color_contract::{Cicp, CicpFamily};
    use crate::working_color::decode_to_linear_srgb;
    const PROBES: [[f32; 3]; 7] = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.04, 0.04, 0.04],
        [0.2, 0.6, 0.9],
    ];
    if matches!(raw[1], 16 | 18) {
        return Err(Error::CicpBesideIcc(HDR));
    }
    let cicp = Cicp::try_from(raw)
        .map_err(|_| Error::CicpBesideIcc("is not a valid full-range RGB code point"))?;
    if !matches!(
        cicp.family(),
        CicpFamily::Srgb | CicpFamily::LinearSrgb | CicpFamily::DisplayP3
    ) {
        return Err(Error::CicpBesideIcc(
            "is outside sRGB, linear sRGB and Display P3, so it cannot be checked against the profile",
        ));
    }
    let input: Vec<f32> = PROBES.iter().flatten().copied().collect();
    let mut output = vec![0.0_f32; input.len()];
    transformer
        .transform(&input, &mut output)
        .map_err(|_| Error::UnsupportedColorEncoding)?;
    for (probe, actual) in PROBES.iter().zip(output.as_chunks::<3>().0) {
        let expected = decode_to_linear_srgb(&ColorEncoding::Cicp(raw), probe.map(f64::from))?;
        for (want, got) in expected.rgb.iter().zip(actual) {
            if (want - f64::from(*got)).abs() > 2e-3 {
                return Err(Error::CicpBesideIcc("disagrees with the profile transform"));
            }
        }
    }
    Ok(())
}

/// The refusal reason for a PQ or HLG transfer beside the profile.
const HDR: &str = "signals a PQ or HLG transfer, which needs tone mapping (ADR-0016 rule 7)";

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
        let channels = self.conversion.channels;
        let len = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
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
        let pixel_bytes = u64::try_from(4 * channels).map_err(|_| Error::GeometryOverflow)?;
        report.output_sample_bytes = report
            .rows
            .checked_mul(u64::from(region.width))
            .and_then(|n| n.checked_mul(pixel_bytes))
            .ok_or(Error::GeometryOverflow)?;
        report.peak_engine_managed_bytes = budget.snapshot().peak;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::IccSrgbSource;
    use crate::color::{AlphaMode, ColorEncoding};
    use crate::decoder::RowConversion;
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

    fn signalled(icc: Vec<u8>, cicp: Option<[u8; 4]>) -> Result<(), Error> {
        let mut grid = p3_grid(vec![10, 20, 30]);
        grid.info.rendered_icc = Some(icc);
        grid.info.rendered_cicp = cicp;
        IccSrgbSource::new(Box::new(grid)).map(|_| ())
    }

    /// Issues #17 and #41: CICP beside the rendered ICC is never silently
    /// overridden. Agreeing SDR evidence passes; contradicting, unknown and
    /// HDR evidence is a controlled `CicpBesideIcc` naming the reason.
    #[test]
    fn cicp_beside_the_rendered_icc_must_agree() {
        let srgb = moxcms::ColorProfile::new_srgb().encode().unwrap();
        let p3 = moxcms::ColorProfile::new_display_p3().encode().unwrap();
        assert_eq!(signalled(srgb.clone(), Some([1, 13, 0, 1])), Ok(()));
        assert_eq!(signalled(p3.clone(), Some([12, 13, 0, 1])), Ok(()));
        // moxcms's own P3 profile carries a miscoded `[11, 13, 1, 0]` tag; a
        // malformed RGB tag is not trusted as a description.
        assert_eq!(signalled(p3.clone(), None), Ok(()));
        for (icc, cicp) in [
            (p3.clone(), [1, 13, 0, 1]),
            (srgb.clone(), [12, 13, 0, 1]),
            (srgb.clone(), [1, 8, 0, 1]),
            (srgb.clone(), [9, 16, 0, 1]),
            (srgb.clone(), [9, 18, 0, 1]),
            (srgb.clone(), [11, 13, 0, 1]),
            (srgb.clone(), [1, 13, 0, 0]),
            (srgb.clone(), [1, 13, 0, 7]),
        ] {
            assert!(
                matches!(signalled(icc, Some(cicp)), Err(Error::CicpBesideIcc(_))),
                "{cicp:?}"
            );
        }
        // A PQ profile is rejected by its own tag, even without decoder CICP.
        let pq = moxcms::ColorProfile::new_display_p3_pq().encode().unwrap();
        assert_eq!(signalled(pq, None), Err(Error::CicpBesideIcc(super::HDR)));
    }

    /// Hostile profiles fail with a controlled error, never a panic.
    #[test]
    fn malformed_and_hostile_profiles_are_controlled_errors() {
        let srgb = moxcms::ColorProfile::new_srgb().encode().unwrap();
        let mut cases: Vec<Vec<u8>> = vec![
            Vec::new(),
            vec![0; 131],
            srgb[..200].to_vec(),
            vec![0xff; 4096],
        ];
        // Declared size far beyond the data.
        let mut oversized = srgb.clone();
        oversized[0..4].copy_from_slice(&u32::MAX.to_be_bytes());
        cases.push(oversized);
        // A tag count that overflows the table.
        let mut tags = srgb.clone();
        tags[128..132].copy_from_slice(&u32::MAX.to_be_bytes());
        cases.push(tags);
        // Every tag offset pointing past the end.
        let mut offsets = srgb.clone();
        let count = u32::from_be_bytes(offsets[128..132].try_into().unwrap()) as usize;
        for i in 0..count {
            let at = 132 + 12 * i + 4;
            offsets[at..at + 4].copy_from_slice(&(u32::MAX - 8).to_be_bytes());
        }
        cases.push(offsets);
        // Larger than the 4 MiB cap.
        cases.push(vec![0; 4 * 1024 * 1024 + 1]);
        for (i, icc) in cases.into_iter().enumerate() {
            assert_eq!(
                signalled(icc, None),
                Err(Error::UnsupportedColorEncoding),
                "case {i}"
            );
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

    /// Deterministic hostile-profile corpus derived from a real v4 profile:
    /// truncations, every header and tag-table word forced to 0 and
    /// `u32::MAX`, wrong data/PCS color spaces, and seeded byte flips. Each
    /// profile must either build a transform that converts a row without
    /// panicking or be rejected as unsupported color.
    #[test]
    fn hostile_icc_profiles_are_rejected_or_converted_without_panicking() {
        use std::panic::{AssertUnwindSafe, catch_unwind};

        let valid = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let tag_table_end = 132 + 12 * usize::from(valid[131]);
        let mut corpus: Vec<(String, Vec<u8>)> = Vec::new();
        for length in [0, 1, 4, 127, 128, 131, 132, 143, 144, tag_table_end, 400] {
            corpus.push((format!("truncated-{length}"), valid[..length].to_vec()));
        }
        for offset in (0..tag_table_end).step_by(4) {
            for word in [[0; 4], [0xff; 4]] {
                let mut profile = valid.clone();
                profile[offset..offset + 4].copy_from_slice(&word);
                corpus.push((format!("word-{offset}-{:02x}", word[0]), profile));
            }
        }
        for (offset, value) in [
            (16, *b"GRAY"),
            (16, *b"CMYK"),
            (16, *b"Lab "),
            (20, *b"Lab "),
            (20, *b"RGB "),
        ] {
            let mut profile = valid.clone();
            profile[offset..offset + 4].copy_from_slice(&value);
            corpus.push((format!("space-{offset}-{value:?}"), profile));
        }
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        for case in 0..256 {
            let mut profile = valid.clone();
            for _ in 0..4 {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let index = usize::try_from(state >> 33).unwrap() % profile.len();
                profile[index] ^= u8::try_from(state >> 56).unwrap() | 1;
            }
            corpus.push((format!("flip-{case}"), profile));
        }

        let mut accepted = 0;
        for (name, profile) in corpus {
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                let mut grid = p3_grid(vec![0, 64, 128, 255, 255, 255, 12, 200, 7]);
                grid.info.rendered_icc = Some(profile);
                let mut source = IccSrgbSource::new(Box::new(grid))?;
                source.read_region(
                    Rect::new(0, 0, 3, 1)?,
                    &MemoryBudget::new(1 << 20),
                    &mut |_, _| Ok(()),
                )?;
                Ok::<_, Error>(())
            }))
            .unwrap_or_else(|_| panic!("ICC profile {name} panicked"));
            match outcome {
                Ok(()) => accepted += 1,
                Err(error) => assert_eq!(error, Error::UnsupportedColorEncoding, "{name}"),
            }
        }
        // The unchanged tag-table words and some flips keep a usable profile.
        assert!(accepted > 0);
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

    /// An 8-bit RGBA source with straight alpha in the Display P3 profile.
    fn p3_rgba_grid(row: Vec<u8>) -> Grid {
        let mut grid = p3_grid(row);
        grid.info.size = Size::new(u32::try_from(grid.row.len() / 4).unwrap(), 1).unwrap();
        grid.info.layout = ChannelLayout::Rgba;
        grid.info.alpha = AlphaMode::Straight;
        grid
    }

    /// RGBA test rows: every alpha class (0, partial, 255) against varied
    /// color, including saturated color under zero alpha.
    fn rgba_row() -> Vec<u8> {
        (0..=255_u8)
            .flat_map(|v| [v, 255 - v, v / 3, [0, 1, 128, 254, 255][usize::from(v) % 5]])
            .collect()
    }

    /// The color of an RGBA row converted as opaque RGB.
    fn rgb_reference(rgba: &[u8]) -> Vec<f32> {
        let rgb: Vec<u8> = rgba
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2]])
            .collect();
        let width = u32::try_from(rgb.len() / 3).unwrap();
        read_rows(
            &mut IccSrgbSource::new(Box::new(p3_grid(rgb))).unwrap(),
            width,
        )
        .pop()
        .unwrap()
    }

    /// Issue #222: straight RGBA converts its color exactly as opaque RGB
    /// does (bit for bit, on the 8-bit table path, the 8-bit moxcms path
    /// and the float moxcms path) and carries alpha through unchanged; the
    /// adapter still declares straight alpha, and deferral changes nothing.
    #[test]
    #[allow(clippy::too_many_lines)] // Three conversion paths, one fixture.
    fn straight_rgba_converts_color_and_carries_alpha_unchanged() {
        let row = rgba_row();
        let width = u32::try_from(row.len() / 4).unwrap();
        let reference = rgb_reference(&row);
        let check = |out: &[f32], alpha: &dyn Fn(usize) -> f32, label: &str| {
            assert_eq!(out.len(), row.len(), "{label}");
            for (i, (pixel, rgb)) in out
                .as_chunks::<4>()
                .0
                .iter()
                .zip(reference.as_chunks::<3>().0)
                .enumerate()
            {
                assert_eq!(
                    pixel[..3].iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    rgb.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    "{label} pixel {i}"
                );
                assert_eq!(pixel[3].to_bits(), alpha(i).to_bits(), "{label} pixel {i}");
            }
        };
        let alpha8 = |i: usize| f32::from(row[i * 4 + 3]) / 255.0;

        // 8-bit input, channel-table path, direct and deferred.
        let mut source = IccSrgbSource::new(Box::new(p3_rgba_grid(row.clone()))).unwrap();
        assert!(source.conversion.table.is_some());
        assert_eq!(source.info().layout, ChannelLayout::Rgba);
        assert_eq!(source.info().alpha, AlphaMode::Straight);
        assert_eq!(source.info().sample, SampleType::F32);
        assert_eq!(source.info().rendered_cicp, Some([1, 8, 0, 1]));
        let direct = read_rows(&mut source, width).pop().unwrap();
        check(&direct, &alpha8, "u8 table");
        let mut deferred = IccSrgbSource::new(Box::new(p3_rgba_grid(row.clone()))).unwrap();
        let conversion = deferred.defer_conversion().unwrap();
        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out, direct);

        // 8-bit input through moxcms (no table), as for LUT-based profiles.
        let moxcms = super::IccConversion {
            transformer: source.conversion.transformer.clone(),
            input: SampleType::U8,
            channels: 4,
            table: None,
        };
        let mut scratch = vec![f32::NAN; row.len()];
        let mut out = vec![f32::NAN; row.len()];
        moxcms
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        // Same moxcms evaluation as the opaque float path, so compare with
        // a float RGB conversion of the same normalized samples.
        let rgb_float: Vec<f32> = row
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2]].map(|v| f32::from(v) / 255.0))
            .collect();
        let mut expected = vec![0.0_f32; rgb_float.len()];
        moxcms
            .transformer
            .transform(&rgb_float, &mut expected)
            .unwrap();
        for (i, (pixel, rgb)) in out
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .enumerate()
        {
            assert_eq!(pixel[..3], rgb[..], "u8 moxcms pixel {i}");
            assert_eq!(
                pixel[3].to_bits(),
                alpha8(i).to_bits(),
                "u8 moxcms pixel {i}"
            );
        }

        // Float input: moxcms path; alpha is copied bit for bit, including
        // values the executor would later refuse (it validates, not this
        // stage).
        let float_alpha = [0.0_f32, 0.25, 1.0, 0.999, f32::MIN_POSITIVE, 0.5];
        let float_row: Vec<f32> = rgb_float
            .as_chunks::<3>()
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, p)| [p[0], p[1], p[2], float_alpha[i % float_alpha.len()]])
            .collect();
        let info = DecoderInfo {
            size: Size::new(width, 1).unwrap(),
            layout: ChannelLayout::Rgba,
            sample: SampleType::F32,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Straight,
            rendered_icc: Some(moxcms::ColorProfile::new_display_p3().encode().unwrap()),
            rendered_cicp: None,
        };
        let float = read_rows(
            &mut IccSrgbSource::new(Box::new(FloatRow {
                info,
                row: float_row,
            }))
            .unwrap(),
            width,
        )
        .pop()
        .unwrap();
        for (i, (pixel, rgb)) in float
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .enumerate()
        {
            assert_eq!(pixel[..3], rgb[..], "f32 pixel {i}");
            assert_eq!(
                pixel[3].to_bits(),
                float_alpha[i % float_alpha.len()].to_bits(),
                "f32 pixel {i}"
            );
        }

        // A row that is not whole RGBA pixels is rejected.
        assert_eq!(
            conversion.convert(
                DecodedRow::U8(&row[..row.len() - 2]),
                &mut scratch[..row.len() - 2],
                &mut out[..row.len() - 2]
            ),
            Err(Error::IncompleteDecoderOutput)
        );
    }

    /// Issue #222: only straight alpha is admitted. Premultiplied color
    /// would need division before the nonlinear profile curve and
    /// unspecified alpha has no meaning; neither is guessed. Layouts other
    /// than RGB and RGBA stay refused.
    #[test]
    fn non_straight_alpha_and_other_layouts_are_refused() {
        for alpha in [
            AlphaMode::Unspecified,
            AlphaMode::Premultiplied,
            AlphaMode::Opaque,
        ] {
            let mut grid = p3_rgba_grid(rgba_row());
            grid.info.alpha = alpha;
            assert_eq!(
                IccSrgbSource::new(Box::new(grid)).map(|_| ()),
                Err(Error::UnsupportedAlphaMode),
                "{alpha:?}"
            );
        }
        for (layout, alpha) in [
            (ChannelLayout::Rgb, AlphaMode::Straight),
            (ChannelLayout::Gray, AlphaMode::Opaque),
            (ChannelLayout::GrayAlpha, AlphaMode::Straight),
            (ChannelLayout::Cmyk, AlphaMode::Opaque),
        ] {
            let mut grid = p3_grid(vec![1, 2, 3]);
            grid.info.layout = layout;
            grid.info.alpha = alpha;
            assert_eq!(
                IccSrgbSource::new(Box::new(grid)).map(|_| ()),
                Err(Error::UnsupportedCodecLayout),
                "{layout:?} {alpha:?}"
            );
        }
    }

    /// The five matrix-shaper profiles of the channel-table evidence (issue
    /// #219): two sRGB serializations, Display P3 with the sRGB curve,
    /// Adobe RGB and Display P3 with gamma 2.2.
    fn table_profiles() -> [(&'static str, Vec<u8>); 5] {
        [
            (
                "moxcms sRGB",
                moxcms::ColorProfile::new_srgb().encode().unwrap(),
            ),
            (
                "paris sRGB",
                include_bytes!("../tests/fixtures/icc/paris-srgb.icc").to_vec(),
            ),
            (
                "Display P3 v4",
                include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec(),
            ),
            (
                "Adobe RGB",
                include_bytes!("../tests/fixtures/icc/adobe-rgb-1998.icc").to_vec(),
            ),
            (
                "P3 gamma 2.2",
                include_bytes!("../tests/fixtures/icc/display-p3-gamma22.icc").to_vec(),
            ),
        ]
    }

    fn table_conversion(icc: Vec<u8>, sample: SampleType) -> std::sync::Arc<super::IccConversion> {
        let mut grid = p3_grid(vec![0; 3]);
        grid.info.rendered_icc = Some(icc);
        grid.info.sample = sample;
        IccSrgbSource::new(Box::new(grid)).unwrap().conversion
    }

    /// Largest |table - moxcms| over `triplets`, through `convert` (the
    /// production row path) against moxcms's own transform.
    fn table_deviation(conversion: &super::IccConversion, triplets: &[[u8; 3]]) -> f32 {
        let row: Vec<u8> = triplets.iter().flatten().copied().collect();
        let mut scratch = vec![0.0_f32; row.len()];
        let mut actual = vec![0.0_f32; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut actual)
            .unwrap();
        let input: Vec<f32> = row.iter().map(|&v| f32::from(v) / 255.0).collect();
        let mut exact = vec![0.0_f32; row.len()];
        conversion
            .transformer
            .transform(&input, &mut exact)
            .unwrap();
        actual
            .iter()
            .zip(&exact)
            .map(|(a, e)| (a - e).abs())
            .fold(0.0, f32::max)
    }

    /// Issue #219: for 8-bit matrix-shaper input the per-channel table
    /// stays within `CHANNEL_TABLE_BOUND` of moxcms on a dense deterministic
    /// sample (every R value against 64 G and 17 B values, plus 65,536
    /// pseudo-random triplets), for each of the five profiles.
    ///
    /// Why a sample bounds the whole cube: each table entry is moxcms's own
    /// output for one channel, `M[:, c] · trc_c(v)`, and is monotone in `v`
    /// per output component (checked here: the curves are monotone and the
    /// matrix is fixed). The table's value at any triplet is therefore the
    /// exact separable sum; its difference from moxcms is only the f32
    /// rounding of moxcms's own matrix product versus three additions,
    /// a few units in the last place of the output, not a function of
    /// position that could peak between probes. The exhaustive 2^24 check
    /// is `channel_table_exhaustive_deviation` (ignored; run in release).
    #[test]
    fn channel_table_stays_within_the_bound_of_moxcms() {
        let mut triplets = Vec::new();
        for r in 0..=255_u8 {
            for g in (0..=255_u8).step_by(4).chain([255]) {
                for b in (0..=255_u8).step_by(16).chain([255]) {
                    triplets.push([r, g, b]);
                }
            }
        }
        let mut state = 0x9e37_79b9_u32;
        for _ in 0..65_536 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let [r, g, b, _] = state.to_le_bytes();
            triplets.push([r, g, b]);
        }
        for (name, icc) in table_profiles() {
            let conversion = table_conversion(icc, SampleType::U8);
            let table = conversion.table.as_ref().expect(name);
            for channel in 0..3 {
                for component in [0_usize, 1, 2] {
                    let column: Vec<f32> = (0..256)
                        .map(|v| table.0[channel * 256 + v][component])
                        .collect();
                    let rising = column.windows(2).all(|w| w[0] <= w[1]);
                    let falling = column.windows(2).all(|w| w[0] >= w[1]);
                    assert!(
                        rising || falling,
                        "{name} channel {channel} component {component}"
                    );
                }
            }
            let worst = table_deviation(&conversion, &triplets);
            eprintln!(
                "{name}: max |table - moxcms| over {} triplets = {worst:e}",
                triplets.len()
            );
            assert!(worst <= super::CHANNEL_TABLE_BOUND, "{name}: {worst:e}");
        }
    }

    /// The exhaustive form of the bound: every one of the 2^24 8-bit
    /// triplets for each of the five profiles. Slow in debug builds; run
    /// with `cargo test --release -p ferrisma --lib channel_table_exhaustive
    /// -- --ignored --nocapture`.
    #[test]
    #[ignore = "exhaustive 2^24 check per profile; run in release"]
    fn channel_table_exhaustive_deviation() {
        for (name, icc) in table_profiles() {
            let conversion = table_conversion(icc, SampleType::U8);
            assert!(conversion.table.is_some(), "{name}");
            let mut worst = 0.0_f32;
            let mut plane = Vec::with_capacity(65_536);
            for r in 0..=255_u8 {
                plane.clear();
                for g in 0..=255_u8 {
                    for b in 0..=255_u8 {
                        plane.push([r, g, b]);
                    }
                }
                worst = worst.max(table_deviation(&conversion, &plane));
            }
            eprintln!("{name}: max |table - moxcms| over all 2^24 triplets = {worst:e}");
            assert!(worst <= super::CHANNEL_TABLE_BOUND, "{name}: {worst:e}");
        }
    }

    /// Only 8-bit input of the extended matrix-shaper path gets a table
    /// (LUT-based profiles never enter that path; see `IccSrgbSource::new`):
    /// float input stays on moxcms, and the table path reproduces its own
    /// `apply` exactly through the row interface.
    #[test]
    fn channel_table_is_limited_to_eight_bit_matrix_shaper_input() {
        for (name, icc) in table_profiles() {
            assert!(
                table_conversion(icc, SampleType::F32).table.is_none(),
                "{name}"
            );
        }
        let p3 = table_conversion(
            moxcms::ColorProfile::new_display_p3().encode().unwrap(),
            SampleType::U8,
        );
        let table = p3.table.as_ref().unwrap();
        let row = [0_u8, 10, 11, 128, 200, 255];
        let mut scratch = [0.0_f32; 6];
        let mut out = [0.0_f32; 6];
        p3.convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out[..3], table.apply([0, 10, 11]));
        assert_eq!(out[3..], table.apply([128, 200, 255]));
    }
}
