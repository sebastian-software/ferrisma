//! A narrow safe reader for the workspace libavif port. Codec-owned scratch
//! memory remains opaque. Only caller-owned RGB output buffers are bounded.

#![deny(clippy::undocumented_unsafe_blocks)]

use avif::{avif as api, read, reformat};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadError {
    Allocation,
    InvalidInput,
    Unsupported,
    /// An image sequence (`avis` tracks or several images); only still
    /// images are decoded.
    Animation,
    PixelLimit {
        pixels: u64,
        limit: u64,
    },
    OutputSize,
}

#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub alpha: bool,
    pub alpha_premultiplied: bool,
    pub primaries: u16,
    pub transfer: u16,
    pub matrix: u16,
    pub full_range: bool,
    pub transform_flags: u32,
    pub icc: Option<Vec<u8>>,
    pub image_count: i32,
    pub sequence: SequenceInfo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SequenceInfo {
    pub max_width: u32,
    pub max_height: u32,
    pub depth: u8,
    pub monochrome: bool,
    pub primaries: u8,
    pub transfer: u8,
    pub matrix: u8,
    pub full_range: bool,
}

/// Owns one libavif decoder. The input slice must outlive its memory reader;
/// the borrow checker enforces that the decoder cannot outlive its input:
///
/// ```compile_fail,E0597
/// let decoder = {
///     let bytes = vec![0_u8; 16];
///     ferrisma_avif_read::Decoder::parse(&bytes, 1).map(|(decoder, _)| decoder)
/// };
/// ```
///
/// The raw decoder pointer also keeps the type `!Send` and `!Sync`.
pub struct Decoder<'a> {
    raw: NonNull<read::avifDecoder>,
    /// Row jobs of the YUV-to-RGB conversion.
    threads: u32,
    _input: PhantomData<&'a [u8]>,
}

impl Drop for Decoder<'_> {
    fn drop(&mut self) {
        // SAFETY: `raw` came from avifDecoderCreate and is destroyed once.
        unsafe { read::avifDecoderDestroy(self.raw.as_ptr()) };
    }
}

impl<'a> Decoder<'a> {
    pub fn parse(bytes: &'a [u8], max_pixels: u64) -> Result<(Self, ImageInfo), ReadError> {
        // SAFETY: The returned pointer is checked and guarded immediately.
        let raw =
            NonNull::new(unsafe { read::avifDecoderCreate() }).ok_or(ReadError::Allocation)?;
        let owner = Self {
            raw,
            threads: 1,
            _input: PhantomData,
        };
        // SAFETY: The decoder is live; `bytes` outlives owner, and libavif's
        // memory reader retains the slice only until the owner is dropped.
        let result = unsafe {
            let decoder = &mut *owner.raw.as_ptr();
            decoder.codecChoice = api::AVIF_CODEC_CHOICE_AOM;
            decoder.requestedSource = read::AVIF_DECODER_SOURCE_PRIMARY_ITEM;
            // Parse does not allocate pixel surfaces. Use libavif's largest
            // representable parser limit so the exact coded size reaches our
            // typed caller limit below; the engine preflights rendered ispe.
            decoder.imageSizeLimit = u32::MAX;
            decoder.imageCountLimit = 1;
            read::avifDecoderSetIOMemory(owner.raw.as_ptr(), bytes.as_ptr(), bytes.len())
        };
        if result != api::AVIF_RESULT_OK {
            return Err(if result == api::AVIF_RESULT_OUT_OF_MEMORY {
                ReadError::Allocation
            } else {
                ReadError::InvalidInput
            });
        }
        // SAFETY: The live decoder has a persistent memory reader.
        let result = unsafe { read::avifDecoderParse(owner.raw.as_ptr()) };
        if result != api::AVIF_RESULT_OK {
            return Err(if result == api::AVIF_RESULT_OUT_OF_MEMORY {
                ReadError::Allocation
            } else {
                ReadError::InvalidInput
            });
        }
        // SAFETY: Parse initialized decoder data. The engine describes the
        // primary item, so a file carrying tracks is outside its still-image
        // contract even when the primary item itself has only one frame.
        let has_tracks = unsafe {
            let data = owner.raw.as_ref().data;
            if data.is_null() {
                return Err(ReadError::InvalidInput);
            }
            (*data).tracks.count != 0
        };
        if has_tracks {
            return Err(ReadError::Animation);
        }
        // SAFETY: Parse succeeded; owner exclusively holds the decoder and
        // keeps the input memory reader alive for sample preparation.
        let headers = unsafe { read::avif_decoder_color_sequence_headers(owner.raw.as_ptr()) }
            .map_err(|result| {
                if result == api::AVIF_RESULT_OUT_OF_MEMORY {
                    ReadError::Allocation
                } else {
                    ReadError::InvalidInput
                }
            })?;
        let mut sequence = None;
        for header in headers {
            let current = SequenceInfo {
                max_width: header.maxWidth,
                max_height: header.maxHeight,
                depth: u8::try_from(header.bitDepth).map_err(|_| ReadError::Unsupported)?,
                monochrome: header.yuvFormat == read::AVIF_PIXEL_FORMAT_YUV400,
                primaries: u8::try_from(header.colorPrimaries)
                    .map_err(|_| ReadError::Unsupported)?,
                transfer: u8::try_from(header.transferCharacteristics)
                    .map_err(|_| ReadError::Unsupported)?,
                matrix: u8::try_from(header.matrixCoefficients)
                    .map_err(|_| ReadError::Unsupported)?,
                full_range: header.range == read::AVIF_RANGE_FULL,
            };
            if let Some(first) = sequence {
                if first != current {
                    return Err(ReadError::Unsupported);
                }
            } else {
                sequence = Some(current);
            }
        }
        let sequence = sequence.ok_or(ReadError::InvalidInput)?;
        let coded_pixels = u64::from(sequence.max_width) * u64::from(sequence.max_height);
        if coded_pixels == 0 {
            return Err(ReadError::InvalidInput);
        }
        if coded_pixels > max_pixels {
            return Err(ReadError::PixelLimit {
                pixels: coded_pixels,
                limit: max_pixels,
            });
        }
        // SAFETY: Parse succeeded and owns the image until decoder destruction.
        let info = unsafe {
            let decoder = owner.raw.as_ref();
            if decoder.codecChoice != api::AVIF_CODEC_CHOICE_AOM || decoder.image.is_null() {
                return Err(ReadError::InvalidInput);
            }
            let image = &*decoder.image;
            let pixels = u64::from(image.width) * u64::from(image.height);
            if pixels == 0 {
                return Err(ReadError::InvalidInput);
            }
            if pixels > max_pixels {
                return Err(ReadError::PixelLimit {
                    pixels,
                    limit: max_pixels,
                });
            }
            if decoder.imageCount > 1 {
                return Err(ReadError::Animation);
            }
            if decoder.imageCount != 1 || image.depth != 8 && image.depth != 10 && image.depth != 12
            {
                return Err(ReadError::Unsupported);
            }
            let icc = if image.icc.size == 0 {
                None
            } else {
                if image.icc.data.is_null() || image.icc.size > 16 * 1024 * 1024 {
                    return Err(ReadError::Unsupported);
                }
                let mut profile = Vec::new();
                profile
                    .try_reserve_exact(image.icc.size)
                    .map_err(|_| ReadError::Allocation)?;
                profile
                    .extend_from_slice(std::slice::from_raw_parts(image.icc.data, image.icc.size));
                Some(profile)
            };
            ImageInfo {
                width: image.width,
                height: image.height,
                depth: image.depth,
                alpha: decoder.alphaPresent != 0,
                alpha_premultiplied: image.alphaPremultiplied != 0,
                primaries: image.colorPrimaries,
                transfer: image.transferCharacteristics,
                matrix: image.matrixCoefficients,
                full_range: image.yuvRange == read::AVIF_RANGE_FULL,
                transform_flags: image.transformFlags,
                icc,
                image_count: decoder.imageCount,
                sequence,
            }
        };
        Ok((owner, info))
    }

    /// Let libavif's YUV-to-RGB conversion split the frame into up to
    /// `threads` row jobs (libavif caps it at eight and keeps one job where a
    /// job would depend on its neighbor's rows). Each job owns disjoint
    /// output rows, so the samples do not depend on the count.
    ///
    /// The AV1 decode itself stays on one thread: with more than one,
    /// libaom's row-based multithreading keeps coefficient buffers for every
    /// superblock of the frame, about 1.4 GB of opaque memory for a 24 MP
    /// image on the 2026-11-12 measurement, whatever the thread count.
    pub fn set_threads(&mut self, threads: u32) {
        self.threads = threads.clamp(1, 64);
    }

    fn next_image(&mut self) -> Result<*mut read::avifImage, ReadError> {
        // SAFETY: The live decoder has a parsed image and a valid memory reader.
        let result = unsafe { read::avifDecoderNthImage(self.raw.as_ptr(), 0) };
        if result != api::AVIF_RESULT_OK {
            return Err(if result == api::AVIF_RESULT_OUT_OF_MEMORY {
                ReadError::Allocation
            } else {
                ReadError::InvalidInput
            });
        }
        // SAFETY: NextImage succeeded and the decoder owns the non-null image.
        let image = unsafe { self.raw.as_ref().image };
        NonNull::new(image)
            .map(NonNull::as_ptr)
            .ok_or(ReadError::InvalidInput)
    }

    fn rgb(
        &mut self,
        pixels: *mut u8,
        row_bytes: u32,
        width: u32,
        height: u32,
        depth: u32,
        alpha: bool,
    ) -> Result<(), ReadError> {
        let image = self.next_image()?;
        // SAFETY: Both modules reproduce the same repr(C) libavif image ABI.
        // `pixels` points to a caller-owned buffer with `row_bytes * height`
        // initialized writable bytes. The decoder outlives the conversion.
        unsafe {
            let mut rgb: reformat::avifRGBImage = std::mem::zeroed();
            api::avifRGBImageSetDefaults((&raw mut rgb).cast(), image.cast());
            if rgb.width != width || rgb.height != height {
                return Err(ReadError::OutputSize);
            }
            rgb.depth = depth;
            rgb.format = if alpha {
                api::AVIF_RGB_FORMAT_RGBA
            } else {
                api::AVIF_RGB_FORMAT_RGB
            };
            rgb.alphaPremultiplied = 0;
            rgb.chromaUpsampling = api::AVIF_CHROMA_UPSAMPLING_NEAREST;
            rgb.maxThreads = i32::try_from(self.threads).unwrap_or(1);
            rgb.pixels = pixels;
            rgb.rowBytes = row_bytes;
            let result = reformat::avifImageYUVToRGB(image.cast(), &raw mut rgb);
            if result != api::AVIF_RESULT_OK {
                return Err(if result == api::AVIF_RESULT_OUT_OF_MEMORY {
                    ReadError::Allocation
                } else {
                    ReadError::Unsupported
                });
            }
        }
        Ok(())
    }

    pub fn decode_u8(
        &mut self,
        pixels: &mut [u8],
        width: u32,
        height: u32,
        alpha: bool,
    ) -> Result<(), ReadError> {
        let channels = if alpha { 4_usize } else { 3 };
        let row_bytes = usize::try_from(width)
            .ok()
            .and_then(|n| n.checked_mul(channels))
            .ok_or(ReadError::OutputSize)?;
        if pixels.len()
            != row_bytes
                .checked_mul(height as usize)
                .ok_or(ReadError::OutputSize)?
        {
            return Err(ReadError::OutputSize);
        }
        self.rgb(
            pixels.as_mut_ptr(),
            u32::try_from(row_bytes).map_err(|_| ReadError::OutputSize)?,
            width,
            height,
            8,
            alpha,
        )
    }

    pub fn decode_u16(
        &mut self,
        pixels: &mut [u16],
        width: u32,
        height: u32,
        alpha: bool,
    ) -> Result<(), ReadError> {
        let channels = if alpha { 4_usize } else { 3 };
        let row_samples = usize::try_from(width)
            .ok()
            .and_then(|n| n.checked_mul(channels))
            .ok_or(ReadError::OutputSize)?;
        let row_bytes = row_samples.checked_mul(2).ok_or(ReadError::OutputSize)?;
        if pixels.len()
            != row_samples
                .checked_mul(height as usize)
                .ok_or(ReadError::OutputSize)?
        {
            return Err(ReadError::OutputSize);
        }
        self.rgb(
            pixels.as_mut_ptr().cast(),
            u32::try_from(row_bytes).map_err(|_| ReadError::OutputSize)?,
            width,
            height,
            16,
            alpha,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Decoder, ReadError, api, read, reformat};
    use std::mem::{align_of, offset_of, size_of};

    /// The `rgb` pointer casts rely on each generated module's own copy of
    /// these `repr(C)` structs having the same layout (issue #54).
    #[test]
    fn cross_module_image_layouts_match() {
        macro_rules! same_layout {
            ($a:ty, $b:ty, [$($field:ident),*]) => {
                assert_eq!(size_of::<$a>(), size_of::<$b>());
                assert_eq!(align_of::<$a>(), align_of::<$b>());
                $(assert_eq!(offset_of!($a, $field), offset_of!($b, $field), stringify!($field));)*
            };
        }
        macro_rules! image_fields {
            ($a:ty, $b:ty) => {
                same_layout!(
                    $a,
                    $b,
                    [
                        width,
                        height,
                        depth,
                        yuvFormat,
                        yuvRange,
                        yuvChromaSamplePosition,
                        yuvPlanes,
                        yuvRowBytes,
                        imageOwnsYUVPlanes,
                        alphaPlane,
                        alphaRowBytes,
                        imageOwnsAlphaPlane,
                        alphaPremultiplied,
                        icc,
                        colorPrimaries,
                        transferCharacteristics,
                        matrixCoefficients,
                        clli,
                        transformFlags,
                        pasp,
                        clap,
                        irot,
                        imir,
                        exif,
                        xmp,
                        properties,
                        numProperties,
                        gainMap
                    ]
                );
            };
        }
        image_fields!(read::avifImage, api::avifImage);
        image_fields!(reformat::avifImage, api::avifImage);
        same_layout!(
            reformat::avifRGBImage,
            api::avifRGBImage,
            [
                width,
                height,
                depth,
                format,
                chromaUpsampling,
                chromaDownsampling,
                avoidLibYUV,
                ignoreAlpha,
                alphaPremultiplied,
                isFloat,
                maxThreads,
                pixels,
                rowBytes
            ]
        );
    }

    #[test]
    fn selected_grid_color_tiles_have_matching_sequence_headers() {
        let bytes = include_bytes!(
            "../../../codecs/libavif-rs/upstream/libavif/tests/data/sofa_grid1x5_420.avif"
        );
        let (mut decoder, info) = Decoder::parse(bytes, 100_000_000).unwrap();
        assert_eq!(info.depth, 8);
        let mut pixels = vec![0; info.width as usize * info.height as usize * 3];
        decoder
            .decode_u8(&mut pixels, info.width, info.height, false)
            .unwrap();
        assert!(pixels.iter().any(|sample| *sample != 0));
    }

    #[test]
    fn twelve_bit_native_fixture_emits_sixteen_bit_rgb() {
        let bytes = include_bytes!(
            "../../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
        );
        let (mut decoder, info) = Decoder::parse(bytes, 100_000_000).unwrap();
        assert_eq!(info.depth, 12);
        let mut pixels = vec![0; info.width as usize * info.height as usize * 3];
        decoder
            .decode_u16(&mut pixels, info.width, info.height, false)
            .unwrap();
        assert!(pixels.iter().any(|sample| *sample != 0));
    }

    #[test]
    fn coded_pixel_limit_reports_exact_sequence_geometry() {
        let bytes = include_bytes!(
            "../../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
        );
        let (_, info) = Decoder::parse(bytes, 100_000_000).unwrap();
        let coded = u64::from(info.sequence.max_width) * u64::from(info.sequence.max_height);
        assert!(coded > 1);
        assert!(matches!(
            Decoder::parse(bytes, coded - 1),
            Err(ReadError::PixelLimit { pixels, limit }) if pixels == coded && limit == coded - 1
        ));
    }

    #[test]
    fn animation_is_not_accepted_as_a_still_image() {
        let bytes = include_bytes!(
            "../../../codecs/libavif-rs/upstream/libavif/tests/data/colors-animated-8bpc.avif"
        );
        assert!(matches!(
            Decoder::parse(bytes, 100_000_000),
            Err(ReadError::Animation)
        ));
        // libavif AUTO would select tracks for `avis` and for this non-avif
        // major brand. Explicit primary selection must still reject both.
        let mut other_brand = bytes.to_vec();
        assert_eq!(&other_brand[4..8], b"ftyp");
        other_brand[8..12].copy_from_slice(b"mif1");
        assert!(matches!(
            Decoder::parse(&other_brand, 100_000_000),
            Err(ReadError::Animation)
        ));
    }
}
