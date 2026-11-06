//! Display orientation of decoded samples, applied as an explicit,
//! engine-budgeted source-side materialization (ADR-0019).
//!
//! Decoders emit samples in *stored* order. EXIF orientation and AVIF
//! `irot`/`imir` describe how to display them. `OrientedSource` presents an
//! upright image to the graph: it maps each requested display rectangle back
//! to the stored rectangle, reads that region once into a managed buffer, and
//! emits upright rows. Crop coordinates are therefore display coordinates.

use crate::decoder::{DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;

/// One of the eight EXIF orientations. Display pixel `(x, y)` reads the
/// stored pixel obtained by optionally swapping the axes (`transpose`) and
/// then mirroring the stored x and/or y coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Orientation {
    transpose: bool,
    flip_x: bool,
    flip_y: bool,
}

impl Orientation {
    pub(crate) const IDENTITY: Self = Self {
        transpose: false,
        flip_x: false,
        flip_y: false,
    };

    /// EXIF/TIFF `Orientation` values 1–8; anything else is not an orientation.
    pub(crate) fn from_exif(value: u16) -> Option<Self> {
        let (transpose, flip_x, flip_y) = match value {
            1 => (false, false, false),
            2 => (false, true, false),
            3 => (false, true, true),
            4 => (false, false, true),
            5 => (true, false, false),
            6 => (true, false, true),
            7 => (true, true, true),
            8 => (true, true, false),
            _ => return None,
        };
        Some(Self {
            transpose,
            flip_x,
            flip_y,
        })
    }

    /// HEIF `irot` (anticlockwise quarter turns) followed by `imir`, whose
    /// axis 0 exchanges top and bottom and axis 1 exchanges left and right
    /// (ISO/IEC 23008-12:2022 6.5.12, as documented by libavif).
    #[cfg_attr(
        not(feature = "avif-read"),
        allow(dead_code, reason = "constructed only by the AVIF adapter")
    )]
    pub(crate) fn from_avif(rotation: u8, mirror: Option<u8>) -> Option<Self> {
        let exif = match (rotation, mirror) {
            (0, None) => 1,
            (1, None) => 8,
            (2, None) => 3,
            (3, None) => 6,
            (0, Some(0)) | (2, Some(1)) => 4,
            (0, Some(1)) | (2, Some(0)) => 2,
            (1, Some(0)) | (3, Some(1)) => 5,
            (1, Some(1)) | (3, Some(0)) => 7,
            _ => return None,
        };
        Self::from_exif(exif)
    }

    #[cfg_attr(
        not(feature = "avif-write"),
        allow(dead_code, reason = "used by the facade")
    )]
    pub(crate) fn is_identity(self) -> bool {
        self == Self::IDENTITY
    }

    /// Upright dimensions of a stored image.
    pub(crate) fn display_size(self, stored: Size) -> Result<Size, Error> {
        if self.transpose {
            Size::new(stored.height, stored.width)
        } else {
            Ok(stored)
        }
    }

    /// The stored rectangle that contains exactly the samples of `display`.
    pub(crate) fn stored_rect(self, display: Rect, stored: Size) -> Result<Rect, Error> {
        let (x, y, width, height) = if self.transpose {
            (display.y, display.x, display.height, display.width)
        } else {
            (display.x, display.y, display.width, display.height)
        };
        let mirror = |offset: u32, extent: u32, total: u32| {
            total
                .checked_sub(offset)
                .and_then(|rest| rest.checked_sub(extent))
                .ok_or(Error::OutOfBounds)
        };
        let x = if self.flip_x {
            mirror(x, width, stored.width)?
        } else {
            x
        };
        let y = if self.flip_y {
            mirror(y, height, stored.height)?
        } else {
            y
        };
        Rect::new(x, y, width, height)
    }

    /// Stored coordinates of display pixel `(x, y)`; the caller guarantees
    /// that the pixel lies inside the display image.
    fn stored_point(self, x: u32, y: u32, stored: Size) -> (u32, u32) {
        let (u, v) = if self.transpose { (y, x) } else { (x, y) };
        (
            if self.flip_x { stored.width - 1 - u } else { u },
            if self.flip_y {
                stored.height - 1 - v
            } else {
                v
            },
        )
    }
}

/// Presents an upright view of a source whose samples need reorientation.
pub(crate) struct OrientedSource<'a> {
    source: Box<dyn DecodeSource + 'a>,
    orientation: Orientation,
    info: DecoderInfo,
}

impl<'a> OrientedSource<'a> {
    pub(crate) fn new(source: Box<dyn DecodeSource + 'a>) -> Result<Self, Error> {
        let orientation = source.orientation();
        let mut info = source.info().clone();
        info.size = orientation.display_size(info.size)?;
        Ok(Self {
            source,
            orientation,
            info,
        })
    }
}

/// Interleaved sample types that decoders emit.
trait RowSample: Copy + Default {
    fn wrap(row: &[Self]) -> DecodedRow<'_>;
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]>;
}

impl RowSample for u8 {
    fn wrap(row: &[Self]) -> DecodedRow<'_> {
        DecodedRow::U8(row)
    }
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]> {
        match row {
            DecodedRow::U8(samples) => Some(samples),
            DecodedRow::F32(_) => None,
        }
    }
}

impl RowSample for f32 {
    fn wrap(row: &[Self]) -> DecodedRow<'_> {
        DecodedRow::F32(row)
    }
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]> {
        match row {
            DecodedRow::F32(samples) => Some(samples),
            DecodedRow::U8(_) => None,
        }
    }
}

/// Copy decoded stored rows into one managed region, then emit upright rows.
/// Geometry shared by every row of one reorientation.
struct Layout {
    orientation: Orientation,
    display: Rect,
    stored: Rect,
    stored_size: Size,
    stride: usize,
}

/// Fill one display row from the stored-order buffer, `N` samples per pixel.
fn gather<T: Copy, const N: usize>(
    pixels: &[T],
    out: &mut [T],
    layout: &Layout,
    y: u32,
) -> Result<(), Error> {
    let out = out.as_chunks_mut::<N>().0;
    for (pixel, x) in out.iter_mut().zip(layout.display.x..layout.display.right()) {
        let (sx, sy) = layout.orientation.stored_point(x, y, layout.stored_size);
        let local = usize::try_from(sy - layout.stored.y).map_err(|_| Error::GeometryOverflow)?
            * layout.stride
            + usize::try_from(sx - layout.stored.x).map_err(|_| Error::GeometryOverflow)? * N;
        *pixel = *pixels
            .get(local..local + N)
            .and_then(|values| values.first_chunk::<N>())
            .ok_or(Error::OutOfBounds)?;
    }
    Ok(())
}

fn reorient<T: RowSample>(
    source: &mut dyn DecodeSource,
    orientation: Orientation,
    display: Rect,
    stored: Rect,
    channels: usize,
    budget: &MemoryBudget,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, usize), Error> {
    let stored_size = source.info().size;
    let stride = usize::try_from(stored.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(Error::GeometryOverflow)?;
    let length = usize::try_from(stored.height)
        .ok()
        .and_then(|height| height.checked_mul(stride))
        .ok_or(Error::GeometryOverflow)?;
    let mut region = budget.allocate_typed::<T>(length)?;
    let mut expected_y = stored.y;
    let report = source.read_region(stored, budget, &mut |y, row| {
        let samples = T::unwrap(row).ok_or(Error::IncompleteDecoderOutput)?;
        if y != expected_y || samples.len() != stride {
            return Err(Error::IncompleteDecoderOutput);
        }
        let start = usize::try_from(y - stored.y).map_err(|_| Error::GeometryOverflow)? * stride;
        region.as_mut_slice()[start..start + stride].copy_from_slice(samples);
        expected_y += 1;
        Ok(())
    })?;
    if expected_y != stored.bottom() {
        return Err(Error::IncompleteDecoderOutput);
    }

    let row_len = usize::try_from(display.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(Error::GeometryOverflow)?;
    let mut row = budget.allocate_typed::<T>(row_len)?;
    let pixels = region.as_slice();
    let layout = Layout {
        orientation,
        display,
        stored,
        stored_size,
        stride,
    };
    for y in display.y..display.bottom() {
        // Fixed-size pixel copies: a slice copy of `channels` elements per
        // pixel compiles to one `memmove` call per pixel.
        match channels {
            1 => gather::<T, 1>(pixels, row.as_mut_slice(), &layout, y)?,
            3 => gather::<T, 3>(pixels, row.as_mut_slice(), &layout, y)?,
            4 => gather::<T, 4>(pixels, row.as_mut_slice(), &layout, y)?,
            _ => return Err(Error::UnsupportedCodecLayout),
        }
        sink(y, T::wrap(row.as_slice()))?;
    }
    let bytes = length
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(Error::GeometryOverflow)?;
    Ok((report, bytes))
}

impl DecodeSource for OrientedSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
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
        let stored = self
            .orientation
            .stored_rect(region, self.source.info().size)?;
        let channels = usize::from(self.info.layout.channels());
        let source = &mut *self.source;
        let (inner, bytes) = match self.info.sample {
            SampleType::U8 => reorient::<u8>(
                source,
                self.orientation,
                region,
                stored,
                channels,
                budget,
                sink,
            )?,
            SampleType::F32 => reorient::<f32>(
                source,
                self.orientation,
                region,
                stored,
                channels,
                budget,
                sink,
            )?,
        };
        Ok(DecodeReport {
            requested: region,
            rows: u64::from(region.height),
            engine_buffer_bytes: inner
                .engine_buffer_bytes
                .checked_add(bytes)
                .ok_or(Error::GeometryOverflow)?,
            peak_engine_managed_bytes: budget.snapshot().peak,
            ..inner
        })
    }
}

#[cfg(test)]
#[path = "orientation_tests.rs"]
mod tests;
