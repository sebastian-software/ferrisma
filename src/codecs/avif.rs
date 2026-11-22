//! Read-only AVIF bridge via the workspace libavif port, decoding AV1 with
//! the chosen [`AvifDecoder`]: the dav1d port by default since 2026-11-22,
//! or the libaom port, the reference decoder every read used before.
//!
//! The constructor only parses container metadata and the AV1 sequence
//! header, so callers can apply their color policy before any pixel decode.
//! A region request decodes a full frame into an engine-budgeted RGB buffer;
//! codec scratch remains opaque and outside that budget.
//!
//! libavif converts YUV to full-range RGB with nearest-neighbor chroma
//! upsampling. The adapter reports that rendered meaning as CICP
//! `[primaries, transfer, 0, 1]`, or, when the item has a `colr` ICC
//! profile, as that profile alone: MIAF gives the profile the color
//! primaries and transfer, and `nclx` only the matrix and range, which
//! libavif has already applied (owner decision 2026-11-15). It fails closed when the container `nclx` disagrees with the
//! bitstream and for `clap`. Monochrome input is emitted as gray (libavif's
//! `GRAY`/`GRAYA` output, the converted Y plane), which the pipeline expands
//! to RGB with every other gray input (ADR-0043 D2). `irot`/`imir` are reported
//! as the source orientation and applied by the pipeline (ADR-0019).
//! 10/12-bit input is emitted as normalized float samples.

use ferrisma_avif_read::{Codec, Decoder, ReadError, Samples};

use super::avif_meta::{MetaError, PrimaryProperties, SequenceColor, primary_properties};
use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, CodecWork, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
    SampleType,
};
use crate::discarded::{Discarded, DiscardedSet};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use crate::orientation::Orientation;
use crate::pipeline::AvifDecoder;

fn codec(decoder: AvifDecoder) -> Codec {
    match decoder {
        AvifDecoder::Dav1d => Codec::Dav1d,
        AvifDecoder::Libaom => Codec::Libaom,
    }
}

fn bridge_error(error: ReadError) -> Error {
    match error {
        ReadError::Allocation => Error::AllocationFailed,
        ReadError::InvalidInput | ReadError::OutputSize => Error::DecodeFailed,
        ReadError::Unsupported => Error::UnsupportedCodecLayout,
        ReadError::Animation => Error::UnsupportedAnimation,
        ReadError::PixelLimit { pixels, limit } => Error::InputPixelLimitExceeded { pixels, limit },
    }
}

/// H.273 table 4 treats these matrix codes as equivalent BT.601 transforms.
fn same_decode_matrix(signalled: u16, bitstream: u8) -> bool {
    let bt601 = |code: u16| matches!(code, 2 | 5 | 6);
    signalled == u16::from(bitstream) || (bt601(signalled) && bt601(u16::from(bitstream)))
}

/// Describe the RGB samples emitted for this file as CICP, or `None` when
/// neither the container nor the bitstream specifies primaries or transfer.
pub(crate) fn rendered_cicp(
    properties: &PrimaryProperties,
    sequence: SequenceColor,
) -> Result<Option<[u8; 4]>, Error> {
    // Constant-luminance BT.2020 needs a separately verified output contract.
    if sequence.matrix == 10 {
        return Err(Error::UnsupportedLayout);
    }
    let (primaries, transfer) = if let Some(nclx) = properties.nclx {
        let field_conflicts =
            |container: u16, bitstream: u8| bitstream != 2 && container != u16::from(bitstream);
        if !same_decode_matrix(nclx.matrix, sequence.matrix)
            || nclx.full_range != sequence.full_range
            || field_conflicts(nclx.primaries, sequence.primaries)
            || field_conflicts(nclx.transfer, sequence.transfer)
        {
            return Err(Error::ContradictoryColorSignals);
        }
        (
            u8::try_from(nclx.primaries).map_err(|_| Error::UnsupportedColorEncoding)?,
            u8::try_from(nclx.transfer).map_err(|_| Error::UnsupportedColorEncoding)?,
        )
    } else {
        (sequence.primaries, sequence.transfer)
    };
    if primaries == 2 && transfer == 2 {
        return Ok(None);
    }
    // Emitted samples are RGB (identity matrix) at full range.
    Ok(Some([primaries, transfer, 0, 1]))
}

/// The libavif output for an adapter layout.
fn samples(layout: ChannelLayout) -> Samples {
    match layout {
        ChannelLayout::Gray => Samples::Gray,
        ChannelLayout::GrayAlpha => Samples::GrayAlpha,
        ChannelLayout::Rgba => Samples::Rgba,
        _ => Samples::Rgb,
    }
}

/// What the read leaves out of the output (ADR-0042 D2).
/// A precision reduction is the pipeline's to report: it depends on the
/// output depth (ADR-0021, amendment of 2026-11-15).
fn discards(properties: &PrimaryProperties, premultiplied: bool) -> DiscardedSet {
    let findings = properties.findings;
    let mut discarded = DiscardedSet::default();
    discarded.insert_if(findings.gain_map, Discarded::GainMap);
    discarded.insert_if(findings.exif, Discarded::Exif);
    discarded.insert_if(findings.xmp, Discarded::Xmp);
    discarded.insert_if(findings.other_metadata, Discarded::OtherMetadata);
    discarded.insert_if(properties.icc.is_some(), Discarded::IccProfile);
    discarded.insert_if(findings.unknown_properties, Discarded::UnknownContainerData);
    // libavif un-premultiplies: the wrapper requests straight RGB.
    discarded.insert_if(premultiplied, Discarded::Unpremultiplied);
    discarded
}

pub(crate) struct AvifSource<'a> {
    decoder: Decoder<'a>,
    decoded: bool,
    info: DecoderInfo,
    orientation: Orientation,
    discarded: DiscardedSet,
}

impl<'a> AvifSource<'a> {
    /// Parse metadata and check frame dimensions *before* the decoder
    /// allocates a pixel surface. This is not an RSS limit: parsing and the
    /// later decode can still allocate opaque codec memory.
    #[allow(clippy::too_many_lines)] // The checks stay in the order they run.
    pub(crate) fn new_bounded(
        bytes: &'a [u8],
        max_pixels: u64,
        avif_decoder: AvifDecoder,
    ) -> Result<Self, Error> {
        let properties = primary_properties(bytes).map_err(|error| match error {
            MetaError::Allocation => Error::AllocationFailed,
            MetaError::ProfileTooLarge => Error::UnsupportedColorEncoding,
            // Well-formed, but marked essential and not understood: an
            // unsupported input, not a malformed one.
            MetaError::UnsupportedEssentialProperty(kind) => {
                Error::UnsupportedEssentialProperty(kind)
            }
            _ => Error::DecodeFailed,
        })?;
        // irot/imir are applied by the pipeline (ADR-0019); clap and an imir
        // that precedes irot are not supported yet.
        if properties.clean_aperture || properties.mirror_before_rotation {
            return Err(Error::UnsupportedImageTransform);
        }
        let orientation = Orientation::from_avif(properties.rotation, properties.mirror)
            .ok_or(Error::UnsupportedImageTransform)?;
        let (width, height) = properties.extent.ok_or(Error::DecodeFailed)?;
        let size = Size::new(width, height)?;
        let rendered_pixels = u64::from(size.width) * u64::from(size.height);
        if rendered_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: rendered_pixels,
                limit: max_pixels,
            });
        }
        let (decoder, decoded_info) =
            Decoder::parse_with(bytes, max_pixels, codec(avif_decoder)).map_err(bridge_error)?;
        let coded_pixels = u64::from(decoded_info.sequence.max_width)
            .checked_mul(u64::from(decoded_info.sequence.max_height))
            .ok_or(Error::GeometryOverflow)?;
        if coded_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: coded_pixels,
                limit: max_pixels,
            });
        }
        let sequence = SequenceColor {
            bit_depth: decoded_info.sequence.depth,
            monochrome: decoded_info.sequence.monochrome,
            primaries: decoded_info.sequence.primaries,
            transfer: decoded_info.sequence.transfer,
            matrix: decoded_info.sequence.matrix,
            full_range: decoded_info.sequence.full_range,
        };
        // The container/bitstream consistency checks run either way.
        let signalled_cicp = rendered_cicp(&properties, sequence)?;
        // MIAF (ISO/IEC 23000-22, 7.3.6.4): with an ICC `colr` the profile
        // governs color primaries and transfer; matrix coefficients and range
        // come from `nclx`, and libavif's YUV-to-RGB conversion has used
        // them. The emitted RGB samples are therefore described by the
        // profile alone (owner decision 2026-11-15, ADR-0015); without
        // `UseRenderedIcc` the file is refused like any ICC file. Until then
        // the `nclx` primaries and transfer were reported beside the
        // profile and had to agree with it (`CicpBesideIcc`).
        let rendered_cicp = if properties.icc.is_some() {
            None
        } else {
            signalled_cicp
        };
        let has_alpha = decoded_info.alpha;
        if decoded_info.image_count > 1 {
            return Err(Error::UnsupportedAnimation);
        }
        if decoded_info.width != width
            || decoded_info.height != height
            || decoded_info.depth != u32::from(sequence.bit_depth)
            || decoded_info.image_count != 1
            || decoded_info.alpha_premultiplied && !has_alpha
        {
            return Err(Error::DecodeFailed);
        }
        if decoded_info.transform_flags & 2 != 0 {
            return Err(Error::UnsupportedImageTransform);
        }
        // libavif's parsed image describes container signaling. Retain the
        // independent AV1 sequence-header check above for emitted RGB meaning.
        if let Some(nclx) = properties.nclx
            && (decoded_info.primaries != nclx.primaries
                || decoded_info.transfer != nclx.transfer
                || decoded_info.matrix != nclx.matrix
                || decoded_info.full_range != nclx.full_range)
        {
            return Err(Error::ContradictoryColorSignals);
        }
        if decoded_info.icc != properties.icc {
            return Err(Error::ContradictoryColorSignals);
        }
        let discarded = discards(&properties, has_alpha && decoded_info.alpha_premultiplied);
        let layout = match (sequence.monochrome, has_alpha) {
            (true, false) => ChannelLayout::Gray,
            (true, true) => ChannelLayout::GrayAlpha,
            (false, false) => ChannelLayout::Rgb,
            (false, true) => ChannelLayout::Rgba,
        };
        let info = DecoderInfo {
            // AV1 `BitDepth`; libavif's alpha item has the color depth.
            depth: sequence.bit_depth,
            size,
            layout,
            sample: if sequence.bit_depth > 8 {
                SampleType::F32
            } else {
                SampleType::U8
            },
            alpha: if has_alpha {
                // The wrapper requests straight RGB output from libavif.
                AlphaMode::Straight
            } else {
                AlphaMode::Opaque
            },
            access: DecodeAccess::FullFrameFallback,
            rendered_icc: properties.icc,
            rendered_cicp,
        };
        Ok(Self {
            decoder,
            decoded: false,
            info,
            orientation,
            discarded,
        })
    }

    #[cfg(test)]
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_bounded(bytes, 100_000_000, AvifDecoder::default())
    }
}

impl DecodeSource for AvifSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn discarded(&self) -> DiscardedSet {
        self.discarded
    }

    fn set_decode_threads(&mut self, threads: usize) {
        // Only libavif's nearest-neighbor YUV-to-RGB conversion uses them:
        // its jobs own disjoint rows, so samples are thread-independent. The
        // AV1 decode stays single-threaded (see `Decoder::set_threads`).
        self.decoder
            .set_threads(u32::try_from(threads).unwrap_or(u32::MAX));
    }

    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        if !region.fits(self.info.size) {
            return Err(Error::OutOfBounds);
        }
        let size = self.info.size;
        let channels = usize::from(self.info.layout.channels());
        let full_samples = usize::try_from(size.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(usize::try_from(size.height).map_err(|_| Error::GeometryOverflow)?)
            .and_then(|pixels| pixels.checked_mul(channels))
            .ok_or(Error::GeometryOverflow)?;
        let row_samples = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        let left = usize::try_from(region.x).map_err(|_| Error::GeometryOverflow)?;
        let stride = usize::try_from(size.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        if self.info.sample == SampleType::U8 {
            let mut frame = budget.allocate(full_samples)?;
            self.decoder
                .decode_u8_samples(
                    frame.as_mut_slice(),
                    size.width,
                    size.height,
                    samples(self.info.layout),
                )
                .map_err(bridge_error)?;
            self.decoded = true;
            for y in region.y..region.bottom() {
                let start = usize::try_from(y)
                    .map_err(|_| Error::GeometryOverflow)?
                    .checked_mul(stride)
                    .and_then(|n| n.checked_add(left * channels))
                    .ok_or(Error::GeometryOverflow)?;
                sink(
                    y,
                    DecodedRow::U8(&frame.as_slice()[start..start + row_samples]),
                )?;
            }
        } else {
            let mut frame = budget.allocate_typed::<u16>(full_samples)?;
            let mut scratch = budget.allocate_typed::<f32>(row_samples)?;
            self.decoder
                .decode_u16_samples(
                    frame.as_mut_slice(),
                    size.width,
                    size.height,
                    samples(self.info.layout),
                )
                .map_err(bridge_error)?;
            self.decoded = true;
            for y in region.y..region.bottom() {
                let start = usize::try_from(y)
                    .map_err(|_| Error::GeometryOverflow)?
                    .checked_mul(stride)
                    .and_then(|n| n.checked_add(left * channels))
                    .ok_or(Error::GeometryOverflow)?;
                for (dst, &sample) in scratch
                    .as_mut_slice()
                    .iter_mut()
                    .zip(&frame.as_slice()[start..start + row_samples])
                {
                    *dst = f32::from(sample) / 65_535.0;
                }
                sink(y, DecodedRow::F32(scratch.as_slice()))?;
            }
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::FullFrameFallback,
            rows: u64::from(region.height),
            output_sample_bytes: u64::from(region.width)
                * u64::from(region.height)
                * channels as u64
                * if self.info.sample == SampleType::F32 {
                    4
                } else {
                    1
                },
            known_full_frame_buffers: 1,
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
            engine_copied_bytes: 0,
            // AV1 has no region decode in this path: every request decodes and
            // converts the whole frame (ADR-0031).
            work: CodecWork::full_frame(region, size),
        })
    }
}

#[cfg(all(test, feature = "avif-write"))]
mod tests {
    use super::{AvifSource, Decoder};
    use crate::decoder::{DecodeSource, DecodedRow, SampleType};
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;

    /// A 6x4 AVIF from the workspace writer (the rav1e sink until
    /// 2026-11-15).
    fn encoded() -> Vec<u8> {
        crate::own_avif::encode_for_test(
            6,
            4,
            crate::pipeline::ChromaSubsampling::Yuv444,
            90.0,
            |_| [200, 40, 90, 255].repeat(6),
        )
    }

    fn nclx_offset(file: &[u8]) -> usize {
        file.windows(8).position(|w| w == b"colrnclx").unwrap() + 8
    }

    #[test]
    fn real_encoder_output_reports_srgb_before_decoding() {
        let file = encoded();
        let mut source = AvifSource::new(&file).unwrap();
        assert_eq!(source.info().rendered_cicp, Some([1, 13, 0, 1]));
        assert_eq!(source.info().size, Size::new(6, 4).unwrap());
        assert!(!source.decoded, "metadata must not decode pixels");
        let budget = MemoryBudget::new(1 << 16);
        let mut rows = 0;
        source
            .read_region(Rect::new(0, 0, 6, 4).unwrap(), &budget, &mut |_, _| {
                rows += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(rows, 4);
        assert!(source.decoded);
    }

    #[test]
    fn full_frame_is_budgeted_and_released_on_cancellation() {
        let file = encoded();
        let mut source = AvifSource::new(&file).unwrap();
        let frame_bytes = 6 * 4 * 3 * 2;
        let row_bytes = 2 * 3 * 4;
        let too_small = MemoryBudget::new(frame_bytes - 1);
        assert!(matches!(
            source.read_region(Rect::new(1, 1, 2, 2).unwrap(), &too_small, &mut |_, _| Ok(
                ()
            )),
            Err(Error::BudgetExceeded { .. })
        ));
        assert_eq!(too_small.snapshot().live, 0);

        let budget = MemoryBudget::new(frame_bytes + row_bytes);
        let report = source
            .read_region(Rect::new(1, 1, 2, 2).unwrap(), &budget, &mut |_, _| Ok(()))
            .unwrap();
        assert_eq!(report.known_full_frame_buffers, 1);
        assert_eq!(report.peak_engine_managed_bytes, frame_bytes + row_bytes);
        assert_eq!(budget.snapshot().live, 0);
        assert_eq!(
            source.read_region(Rect::new(1, 1, 2, 2).unwrap(), &budget, &mut |_, _| {
                Err(Error::DecodeFailed)
            }),
            Err(Error::DecodeFailed)
        );
        assert_eq!(budget.snapshot().live, 0);
    }

    /// A 6 x 4, 10-bit, identity-matrix RGB still of one color (200, 40, 90):
    /// identity coding makes the encoder write an explicit `colr` box. The
    /// workspace writer emits BT.601 YCbCr only, so the file is a committed
    /// fixture (`tests/fixtures/avif-color/README.md`).
    fn encoded_with_nclx() -> Vec<u8> {
        include_bytes!("../../tests/fixtures/avif-color/identity-rgb-10bit.avif").to_vec()
    }

    #[test]
    fn explicit_nclx_ten_bit_rgb_is_normalized_and_reported_as_srgb() {
        let file = encoded_with_nclx();
        nclx_offset(&file);
        let mut source = AvifSource::new(&file).unwrap();
        assert_eq!(source.info().rendered_cicp, Some([1, 13, 0, 1]));
        assert_eq!(source.info().sample, SampleType::F32);
        let budget = MemoryBudget::new(1 << 16);
        let mut first = Vec::new();
        source
            .read_region(Rect::new(0, 0, 6, 1).unwrap(), &budget, &mut |_, row| {
                if let DecodedRow::F32(samples) = row {
                    first.extend_from_slice(&samples[..3]);
                }
                Ok(())
            })
            .unwrap();
        for (actual, expected) in first.iter().zip([200.0_f32, 40.0, 90.0]) {
            assert!((actual * 255.0 - expected).abs() < 3.0, "{first:?}");
        }
    }

    #[test]
    fn container_signal_that_contradicts_the_bitstream_is_rejected() {
        let file = encoded_with_nclx();
        let offset = nclx_offset(&file);
        // Declare PQ transfer while the coded sequence header says sRGB.
        let mut pq = file.clone();
        pq[offset + 2..offset + 4].copy_from_slice(&16_u16.to_be_bytes());
        assert_eq!(
            AvifSource::new(&pq).err(),
            Some(Error::ContradictoryColorSignals)
        );
        // Declare a YUV matrix although the payload is identity-coded RGB.
        let mut matrix = file.clone();
        matrix[offset + 4..offset + 6].copy_from_slice(&1_u16.to_be_bytes());
        assert_eq!(
            AvifSource::new(&matrix).err(),
            Some(Error::ContradictoryColorSignals)
        );
    }

    #[test]
    fn pinned_grid_and_twelve_bit_still_images_read_through_engine() {
        let fixtures: &[(&[u8], SampleType)] = &[
            (
                include_bytes!(
                    "../../codecs/libavif-rs/upstream/libavif/tests/data/sofa_grid1x5_420.avif"
                ),
                SampleType::U8,
            ),
            (
                include_bytes!(
                    "../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
                ),
                SampleType::F32,
            ),
        ];
        for (bytes, sample) in fixtures {
            let mut source = AvifSource::new(bytes).unwrap();
            assert_eq!(source.info().sample, *sample);
            let size = source.info().size;
            let budget = MemoryBudget::new(64 << 20);
            let mut rows = 0;
            source
                .read_region(
                    Rect::new(0, 0, size.width, size.height).unwrap(),
                    &budget,
                    &mut |_, _| {
                        rows += 1;
                        Ok(())
                    },
                )
                .unwrap();
            assert_eq!(rows, size.height);
        }
    }

    #[test]
    fn twelve_bit_rows_normalize_pinned_c_oracle_sixteen_bit_samples() {
        let bytes = include_bytes!(
            "../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
        );
        let (mut decoder, raw_info) = Decoder::parse(bytes, 100_000_000).unwrap();
        let mut raw = vec![0_u16; raw_info.width as usize * raw_info.height as usize * 3];
        decoder
            .decode_u16(&mut raw, raw_info.width, raw_info.height, false)
            .unwrap();
        let mut source = AvifSource::new(bytes).unwrap();
        let budget = MemoryBudget::new(64 << 20);
        source
            .read_region(Rect::new(0, 0, 2, 1).unwrap(), &budget, &mut |_, row| {
                let DecodedRow::F32(samples) = row else {
                    panic!("12-bit samples must be normalized floats");
                };
                for (actual, coded) in samples.iter().zip(&raw[..6]) {
                    assert_eq!(*actual, f32::from(*coded) / 65_535.0);
                }
                Ok(())
            })
            .unwrap();
    }

    /// Every sample of a full-frame read, as bit patterns.
    fn decode_all(bytes: &[u8], threads: usize) -> Vec<u32> {
        decode_all_with(bytes, threads, crate::pipeline::AvifDecoder::default())
    }

    fn decode_all_with(
        bytes: &[u8],
        threads: usize,
        avif_decoder: crate::pipeline::AvifDecoder,
    ) -> Vec<u32> {
        let mut source = AvifSource::new_bounded(bytes, 100_000_000, avif_decoder).unwrap();
        source.set_decode_threads(threads);
        let size = source.info().size;
        let mut samples = Vec::new();
        source
            .read_region(
                Rect::new(0, 0, size.width, size.height).unwrap(),
                &MemoryBudget::new(256 << 20),
                &mut |_, row| {
                    match row {
                        DecodedRow::U8(values) => {
                            samples.extend(values.iter().map(|&v| u32::from(v)));
                        }
                        DecodedRow::F32(values) => {
                            samples.extend(values.iter().map(|v| v.to_bits()));
                        }
                    }
                    Ok(())
                },
            )
            .unwrap();
        samples
    }

    /// Decoding with several threads (libavif's row-split YUV-to-RGB jobs)
    /// emits exactly the single-thread samples.
    #[test]
    fn decode_threads_do_not_change_samples() {
        use crate::pipeline::ChromaSubsampling;
        let (width, height) = (96_u32, 64_u32);
        let mut files = Vec::new();
        for chroma in [ChromaSubsampling::Yuv444, ChromaSubsampling::Yuv420] {
            files.push(crate::own_avif::encode_for_test(
                width,
                height,
                chroma,
                70.0,
                |y| {
                    (0..width)
                        .flat_map(|x| {
                            let v = u8::try_from((x * 7 + y * 3) % 256).unwrap();
                            [v, v.wrapping_mul(3), 255 - v, 255]
                        })
                        .collect()
                },
            ));
        }
        files.push(include_bytes!(
            "../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
        ).to_vec());
        for (index, file) in files.iter().enumerate() {
            let single = decode_all(file, 1);
            assert_ne!(single.len(), 0);
            for threads in [2, 3, 8] {
                assert_eq!(
                    single,
                    decode_all(file, threads),
                    "file {index}, {threads} threads"
                );
            }
        }
    }

    /// The engine rows of the dav1d default equal those of the libaom
    /// reference on the engine's AVIF fixtures above 8 bits, with alpha and
    /// with orientation (owner decision of 2026-11-22; the reader crate
    /// compares the whole repository corpus at the bridge).
    #[test]
    fn dav1d_default_rows_equal_the_libaom_reference() {
        use crate::pipeline::AvifDecoder;
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/");
        let mut checked = 0;
        for directory in ["avif-deep", "avif-color", "orientation", "parity"] {
            let mut files: Vec<_> = std::fs::read_dir(format!("{root}{directory}"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.extension().is_some_and(|ext| ext == "avif"))
                .collect();
            files.sort();
            for path in files {
                let bytes = std::fs::read(&path).unwrap();
                if AvifSource::new(&bytes).is_err() {
                    // Refused before any pixel decode, alike for both.
                    assert!(
                        AvifSource::new_bounded(&bytes, 100_000_000, AvifDecoder::Libaom).is_err(),
                        "{}",
                        path.display()
                    );
                    continue;
                }
                assert_eq!(
                    decode_all_with(&bytes, 1, AvifDecoder::Dav1d),
                    decode_all_with(&bytes, 1, AvifDecoder::Libaom),
                    "{}",
                    path.display()
                );
                checked += 1;
            }
        }
        assert!(checked >= 16, "only {checked} fixtures decoded");
    }
}
