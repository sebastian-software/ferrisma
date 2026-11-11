//! Feature-gated AVIF output through `rav1e` and `avif-serialize`.
//!
//! Output rows (straight sRGB RGBA8) are converted to BT.601 full-range
//! 10-bit YCbCr as they arrive and written straight into `rav1e`'s input
//! frame, so the engine keeps no RGBA copy of the output. `rav1e` still needs
//! the complete frame before it encodes; that frame is allocated by `rav1e`
//! on the sink's request, its exact size is charged to the managed budget
//! and reported, and `rav1e`'s internal encoder state stays opaque.
//!
//! The AV1 tile count depends only on the output size and on whether the
//! caller asked for parallel encoding, never on how many threads that turned
//! out to be, so a given set of options produces the same bytes on every
//! machine (M2 work package 2).

// `rav1e` does not export `PlaneOffset`, so plane slices start at
// `Default::default()`.
#![allow(clippy::default_trait_access)]

use rav1e::prelude::{
    ChromaSamplePosition, ChromaSampling, ColorDescription, ColorPrimaries, Config, Context,
    EncoderConfig, EncoderStatus, Frame, FrameType, MatrixCoefficients, PixelRange,
    TransferCharacteristics, Tune,
};

use crate::av1_tuning::{quality_to_quantizer, speed_settings};
use crate::error::Error;
use crate::geometry::Size;
use crate::memory::{ExternalReservation, ManagedBuffer, MemoryBudget};
use crate::pipeline::ChromaSubsampling;

/// Upper bound for `EncodeSettings::threads`; more threads than AV1 tiles
/// cannot help a still image, and `rav1e` builds one pool per encode.
pub(crate) const MAX_ENCODER_THREADS: usize = 256;

/// Output bit depth. 8-bit sources are expanded; the encoder experiment
/// found 10-bit 4:2:0 smaller than 8-bit 4:2:0 at equal SSIMULACRA2.
const DEPTH: u8 = 10;
const MAX_SAMPLE: f32 = 1023.0;

/// Alpha keeps the fixed quality `ravif` used, independent of `quality`.
const ALPHA_QUALITY: f32 = 80.0;

/// Pixels per AV1 tile when encoding in parallel: one tile per 256×256
/// pixels of area, at most 64 tiles. On the M2 baseline this encodes a
/// 1600×1067 output in 137 ms on 20 cores (512×512: 223 ms, about 2 % fewer
/// bytes); one-thread encoding uses a single tile and stays smallest.
const PIXELS_PER_TILE: usize = 256 * 256;
const MAX_TILES: usize = 64;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EncodeSettings {
    pub quality: f32,
    pub speed: u8,
    /// Threads for `rav1e`'s pool. Changes speed only, never bytes.
    pub threads: usize,
    /// Split the frame into size-derived tiles so that threads can work in
    /// parallel. With one tile, extra threads have nothing to do.
    pub parallel_tiles: bool,
    pub chroma: ChromaSubsampling,
}

impl EncodeSettings {
    pub(crate) fn validate(self) -> Result<(), Error> {
        if !(1.0..=100.0).contains(&self.quality)
            || !self.quality.is_finite()
            || !(1..=10).contains(&self.speed)
            || !(1..=MAX_ENCODER_THREADS).contains(&self.threads)
        {
            return Err(Error::InvalidEncodeOptions);
        }
        Ok(())
    }

    fn tiles(self, width: usize, height: usize) -> usize {
        if self.parallel_tiles {
            (width.saturating_mul(height) / PIXELS_PER_TILE).clamp(1, MAX_TILES)
        } else {
            1
        }
    }
}

#[derive(Debug)]
pub(crate) struct EncodedAvif {
    pub bytes: Vec<u8>,
    /// Bytes of `rav1e`'s input frame(s), charged to the budget while alive.
    pub frame_materialization_bytes: usize,
    /// `rav1e` allocates further encoder state that is not budgeted.
    pub opaque_encoder_memory_unbounded: bool,
}

pub(crate) struct AvifSink {
    width: usize,
    height: usize,
    next_y: usize,
    settings: EncodeSettings,
    context: Context<u16>,
    frame: Frame<u16>,
    _frame_budget: ExternalReservation,
    /// 4:2:0 only: unrounded Cb and Cr sums of the current row pair.
    chroma_sums: Option<ManagedBuffer<f32>>,
    /// Straight alpha bytes, only when the output can be transparent.
    alpha: Option<ManagedBuffer<u8>>,
}

fn context(
    width: usize,
    height: usize,
    settings: EncodeSettings,
    quality: f32,
    chroma: ChromaSampling,
    color: bool,
) -> Result<Context<u16>, Error> {
    let quantizer = quality_to_quantizer(quality);
    Config::new()
        .with_encoder_config(EncoderConfig {
            width,
            height,
            bit_depth: usize::from(DEPTH),
            chroma_sampling: chroma,
            chroma_sample_position: ChromaSamplePosition::Unknown,
            pixel_range: PixelRange::Full,
            color_description: color.then_some(ColorDescription {
                transfer_characteristics: TransferCharacteristics::SRGB,
                color_primaries: ColorPrimaries::BT709,
                matrix_coefficients: MatrixCoefficients::BT601,
            }),
            still_picture: true,
            quantizer: usize::from(quantizer),
            min_quantizer: quantizer,
            bitrate: 0,
            tune: Tune::Psychovisual,
            tiles: settings.tiles(width, height),
            speed_settings: speed_settings(settings.speed, quantizer),
            ..EncoderConfig::default()
        })
        .with_threads(settings.threads)
        .new_context()
        .map_err(|_| Error::InvalidEncodeOptions)
}

fn frame_bytes(frame: &Frame<u16>) -> Result<usize, Error> {
    frame.planes.iter().try_fold(0_usize, |sum, plane| {
        plane
            .cfg
            .stride
            .checked_mul(plane.cfg.alloc_height)
            .and_then(|samples| samples.checked_mul(std::mem::size_of::<u16>()))
            .and_then(|bytes| sum.checked_add(bytes))
            .ok_or(Error::GeometryOverflow)
    })
}

/// BT.601 full-range YCbCr at 10 bits, unrounded.
fn ycbcr(r: u8, g: u8, b: u8) -> [f32; 3] {
    const KR: f32 = 0.299;
    const KG: f32 = 0.587;
    const KB: f32 = 0.114;
    let scale = MAX_SAMPLE / 255.0;
    let (r, g, b) = (
        f32::from(r) * scale,
        f32::from(g) * scale,
        f32::from(b) * scale,
    );
    let y = KR.mul_add(r, KG.mul_add(g, KB * b));
    let middle = (MAX_SAMPLE * 0.5).round();
    [
        y,
        (b - y).mul_add(0.5 / (1.0 - KB), middle),
        (r - y).mul_add(0.5 / (1.0 - KR), middle),
    ]
}

fn sample(value: f32) -> u16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let sample = value.round().clamp(0.0, MAX_SAMPLE) as u16;
    sample
}

/// 8-bit to 10-bit by bit replication (0 -> 0, 255 -> 1023).
fn widen(value: u8) -> u16 {
    (u16::from(value) << 2) | (u16::from(value) >> 6)
}

fn encode_frame(mut context: Context<u16>, frame: Frame<u16>) -> Result<Vec<u8>, Error> {
    context.send_frame(frame).map_err(|_| Error::EncodeFailed)?;
    context.flush();
    let mut av1 = Vec::new();
    loop {
        match context.receive_packet() {
            Ok(mut packet) => {
                if packet.frame_type == FrameType::KEY {
                    av1.append(&mut packet.data);
                }
            }
            Err(EncoderStatus::Encoded) => {}
            Err(EncoderStatus::LimitReached) => break,
            Err(_) => return Err(Error::EncodeFailed),
        }
    }
    Ok(av1)
}

impl AvifSink {
    /// Create the encoder and its input frame up front so that rows can be
    /// converted on arrival. `transparent` is false for opaque sources,
    /// which then carry no alpha buffer at all.
    pub(crate) fn new(
        size: Size,
        transparent: bool,
        settings: EncodeSettings,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        settings.validate()?;
        let width = usize::try_from(size.width).map_err(|_| Error::GeometryOverflow)?;
        let height = usize::try_from(size.height).map_err(|_| Error::GeometryOverflow)?;
        let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
        let chroma = match settings.chroma {
            ChromaSubsampling::Yuv444 => ChromaSampling::Cs444,
            ChromaSubsampling::Yuv420 => ChromaSampling::Cs420,
        };
        let context = context(width, height, settings, settings.quality, chroma, true)?;
        let frame = context.new_frame();
        let frame_budget = budget.reserve_external(frame_bytes(&frame)?)?;
        let chroma_sums = match settings.chroma {
            ChromaSubsampling::Yuv444 => None,
            ChromaSubsampling::Yuv420 => Some(
                budget.allocate_typed::<f32>(
                    width
                        .div_ceil(2)
                        .checked_mul(2)
                        .ok_or(Error::GeometryOverflow)?,
                )?,
            ),
        };
        let alpha = if transparent {
            Some(budget.allocate(pixels)?)
        } else {
            None
        };
        Ok(Self {
            width,
            height,
            next_y: 0,
            settings,
            context,
            frame,
            _frame_budget: frame_budget,
            chroma_sums,
            alpha,
        })
    }

    /// Rows are output-space straight RGBA8, top to bottom.
    pub(crate) fn write_row(&mut self, y: u32, bytes: &[u8]) -> Result<(), Error> {
        let row = usize::try_from(y).map_err(|_| Error::GeometryOverflow)?;
        if row != self.next_y
            || row >= self.height
            || bytes.len() != self.width.checked_mul(4).ok_or(Error::GeometryOverflow)?
        {
            return Err(Error::IncompleteOutput);
        }
        let pixels = bytes.as_chunks::<4>().0;
        let [luma, blue_plane, red_plane] = &mut self.frame.planes;
        {
            let mut plane = luma.mut_slice(Default::default());
            for (out, px) in plane[row][..self.width].iter_mut().zip(pixels) {
                *out = sample(ycbcr(px[0], px[1], px[2])[0]);
            }
        }
        match &mut self.chroma_sums {
            None => {
                let mut blue = blue_plane.mut_slice(Default::default());
                let mut red = red_plane.mut_slice(Default::default());
                for ((blue_out, red_out), px) in blue[row][..self.width]
                    .iter_mut()
                    .zip(&mut red[row][..self.width])
                    .zip(pixels)
                {
                    let [_, u, v] = ycbcr(px[0], px[1], px[2]);
                    *blue_out = sample(u);
                    *red_out = sample(v);
                }
            }
            Some(sums) => {
                let chroma_width = self.width.div_ceil(2);
                let (blue_sums, red_sums) = sums.as_mut_slice().split_at_mut(chroma_width);
                if row % 2 == 0 {
                    blue_sums.fill(0.0);
                    red_sums.fill(0.0);
                }
                for (column, px) in pixels.iter().enumerate() {
                    let [_, u, v] = ycbcr(px[0], px[1], px[2]);
                    blue_sums[column / 2] += u;
                    red_sums[column / 2] += v;
                }
                if row % 2 == 1 || row + 1 == self.height {
                    let rows = if row % 2 == 1 { 2.0 } else { 1.0 };
                    let mut blue = blue_plane.mut_slice(Default::default());
                    let mut red = red_plane.mut_slice(Default::default());
                    for column in 0..chroma_width {
                        let columns = if 2 * column + 1 < self.width {
                            2.0
                        } else {
                            1.0
                        };
                        let count: f32 = rows * columns;
                        blue[row / 2][column] = sample(blue_sums[column] / count);
                        red[row / 2][column] = sample(red_sums[column] / count);
                    }
                }
            }
        }
        if let Some(alpha) = &mut self.alpha {
            let start = row * self.width;
            for (out, px) in alpha.as_mut_slice()[start..start + self.width]
                .iter_mut()
                .zip(pixels)
            {
                *out = px[3];
            }
        }
        self.next_y += 1;
        Ok(())
    }

    pub(crate) fn finish(self, budget: &MemoryBudget) -> Result<EncodedAvif, Error> {
        if self.next_y != self.height {
            return Err(Error::IncompleteOutput);
        }
        let Self {
            width,
            height,
            settings,
            context: color_context,
            frame,
            _frame_budget: color_budget,
            chroma_sums,
            alpha,
            ..
        } = self;
        drop(chroma_sums);
        let mut materialized = color_budget.bytes();
        let color = encode_frame(color_context, frame)?;
        drop(color_budget);

        // Opaque output gets no alpha item, like `ravif` did.
        let alpha_av1 = match alpha {
            Some(alpha) if alpha.as_slice().iter().any(|&a| a != u8::MAX) => {
                let alpha_context = context(
                    width,
                    height,
                    settings,
                    ALPHA_QUALITY,
                    ChromaSampling::Cs400,
                    false,
                )?;
                let mut alpha_frame = alpha_context.new_frame();
                let alpha_budget = budget.reserve_external(frame_bytes(&alpha_frame)?)?;
                materialized = materialized.max(alpha_budget.bytes());
                {
                    let mut plane = alpha_frame.planes[0].mut_slice(Default::default());
                    for (row, values) in alpha.as_slice().chunks_exact(width).enumerate() {
                        for (out, &value) in plane[row][..width].iter_mut().zip(values) {
                            *out = widen(value);
                        }
                    }
                }
                drop(alpha);
                Some(encode_frame(alpha_context, alpha_frame)?)
            }
            _ => None,
        };

        let subsampled = settings.chroma == ChromaSubsampling::Yuv420;
        let mut container = avif_serialize::Aviffy::new();
        container
            .set_color_primaries(avif_serialize::constants::ColorPrimaries::Bt709)
            .set_transfer_characteristics(avif_serialize::constants::TransferCharacteristics::Srgb)
            .set_matrix_coefficients(avif_serialize::constants::MatrixCoefficients::Bt601)
            .set_full_color_range(true)
            .set_chroma_subsampling((subsampled, subsampled))
            // Main profile covers 4:2:0 (and monochrome alpha); 4:4:4 needs High.
            .set_seq_profile(u8::from(!subsampled));
        let bytes = container.to_vec(
            &color,
            alpha_av1.as_deref(),
            u32::try_from(width).map_err(|_| Error::GeometryOverflow)?,
            u32::try_from(height).map_err(|_| Error::GeometryOverflow)?,
            DEPTH,
        );
        Ok(EncodedAvif {
            bytes,
            frame_materialization_bytes: materialized,
            opaque_encoder_memory_unbounded: true,
        })
    }
}

/// Single-threaded 4:4:4 settings for tests.
#[cfg(test)]
pub(crate) fn test_settings(quality: f32, speed: u8) -> EncodeSettings {
    EncodeSettings {
        quality,
        speed,
        threads: 1,
        parallel_tiles: false,
        chroma: ChromaSubsampling::Yuv444,
    }
}
