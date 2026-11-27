#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "test fixtures convert small, non-negative sample values"
)]

use super::{Orientation, OrientedSource};
use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;

type Grid = Vec<Vec<u8>>;

/// Independent reference operations on a row-major grid of pixel ids.
fn flip_h(grid: &Grid) -> Grid {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}
fn flip_v(grid: &Grid) -> Grid {
    grid.iter().rev().cloned().collect()
}
fn transpose(grid: &Grid) -> Grid {
    (0..grid[0].len())
        .map(|x| grid.iter().map(|row| row[x]).collect())
        .collect()
}
fn rotate_cw(grid: &Grid) -> Grid {
    flip_h(&transpose(grid))
}
fn rotate_ccw(grid: &Grid) -> Grid {
    flip_v(&transpose(grid))
}

/// EXIF orientation semantics as listed in the TIFF/EXIF specification.
fn exif_reference(value: u16, stored: &Grid) -> Grid {
    match value {
        1 => stored.clone(),
        2 => flip_h(stored),
        3 => flip_v(&flip_h(stored)),
        4 => flip_v(stored),
        5 => transpose(stored),
        6 => rotate_cw(stored),
        7 => flip_v(&flip_h(&transpose(stored))),
        8 => rotate_ccw(stored),
        _ => unreachable!(),
    }
}

/// A stored-order source holding one channel per pixel id (`Rgb` layout
/// with three equal channels keeps the adapter's channel handling honest).
struct GridSource {
    info: DecoderInfo,
    grid: Grid,
    orientation: Orientation,
    float: bool,
}

impl GridSource {
    fn new(grid: Grid, orientation: Orientation, float: bool) -> Self {
        let size = Size::new(
            u32::try_from(grid[0].len()).unwrap(),
            u32::try_from(grid.len()).unwrap(),
        )
        .unwrap();
        Self {
            info: DecoderInfo {
                depth: 8,
                size,
                layout: ChannelLayout::Rgb,
                sample: if float {
                    SampleType::F32
                } else {
                    SampleType::U8
                },
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: None,
                rendered_cicp: Some([1, 13, 0, 1]),
            },
            grid,
            orientation,
            float,
        }
    }
}

impl DecodeSource for GridSource {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn orientation(&self) -> Orientation {
        self.orientation
    }
    fn streams_rows(&self) -> bool {
        true
    }
    fn read_region(
        &mut self,
        region: Rect,
        _budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        assert!(region.fits(self.info.size), "{region:?}");
        for y in region.y..region.bottom() {
            let ids = &self.grid[y as usize][region.x as usize..region.right() as usize];
            let bytes: Vec<u8> = ids.iter().flat_map(|&id| [id, id, id]).collect();
            if self.float {
                let floats: Vec<f32> = bytes.iter().map(|&b| f32::from(b)).collect();
                sink(y, DecodedRow::F32(&floats))?;
            } else {
                sink(y, DecodedRow::U8(&bytes))?;
            }
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows: u64::from(region.height),
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

fn stored_grid(width: u8, height: u8) -> Grid {
    (0..height)
        .map(|y| (0..width).map(|x| y * width + x + 1).collect())
        .collect()
}

/// Read `region` from an oriented source and return pixel ids per row.
fn read(source: &mut OrientedSource<'_>, region: Rect, budget: &MemoryBudget) -> Grid {
    let mut rows = Vec::new();
    let mut next = region.y;
    source
        .read_region(region, budget, &mut |y, row| {
            assert_eq!(y, next);
            next += 1;
            let ids: Vec<u8> = match row {
                DecodedRow::U8(samples) => samples.chunks(3).map(|px| px[0]).collect(),
                DecodedRow::F32(samples) => samples
                    .chunks(3)
                    .map(|px| u8::try_from(px[0] as u32).unwrap())
                    .collect(),
            };
            rows.push(ids);
            Ok(())
        })
        .unwrap();
    rows
}

#[test]
fn every_exif_orientation_matches_the_reference_for_every_crop() {
    let stored = stored_grid(4, 3);
    for value in 1..=8 {
        for float in [false, true] {
            let orientation = Orientation::from_exif(value).unwrap();
            let expected = exif_reference(value, &stored);
            let mut source = OrientedSource::new(Box::new(GridSource::new(
                stored.clone(),
                orientation,
                float,
            )))
            .unwrap();
            let size = source.info().size;
            assert_eq!(size.width as usize, expected[0].len(), "EXIF {value}");
            assert_eq!(size.height as usize, expected.len(), "EXIF {value}");
            let budget = MemoryBudget::new(1 << 16);
            assert_eq!(
                read(
                    &mut source,
                    Rect::new(0, 0, size.width, size.height).unwrap(),
                    &budget
                ),
                expected,
                "EXIF {value}, float {float}"
            );
            // Every display sub-rectangle maps to exactly its stored footprint.
            for y in 0..size.height {
                for x in 0..size.width {
                    for height in 1..=size.height - y {
                        for width in 1..=size.width - x {
                            let crop = Rect::new(x, y, width, height).unwrap();
                            let got = read(&mut source, crop, &budget);
                            let want: Grid = expected[y as usize..(y + height) as usize]
                                .iter()
                                .map(|row| row[x as usize..(x + width) as usize].to_vec())
                                .collect();
                            assert_eq!(got, want, "EXIF {value}, crop {crop:?}");
                        }
                    }
                }
            }
            assert_eq!(budget.snapshot().live, 0, "buffers are released");
        }
    }
}

#[test]
fn avif_rotation_and_mirror_compose_in_association_order() {
    let stored = stored_grid(3, 2);
    for rotation in 0..4_u8 {
        for mirror in [None, Some(0_u8), Some(1)] {
            // irot: anticlockwise quarter turns; imir axis 0 exchanges top and
            // bottom, axis 1 exchanges left and right.
            let mut expected = stored.clone();
            for _ in 0..rotation {
                expected = rotate_ccw(&expected);
            }
            expected = match mirror {
                None => expected,
                Some(0) => flip_v(&expected),
                Some(_) => flip_h(&expected),
            };
            let orientation = Orientation::from_avif(rotation, mirror).unwrap();
            let mut source = OrientedSource::new(Box::new(GridSource::new(
                stored.clone(),
                orientation,
                false,
            )))
            .unwrap();
            let size = source.info().size;
            let budget = MemoryBudget::new(1 << 16);
            assert_eq!(
                read(
                    &mut source,
                    Rect::new(0, 0, size.width, size.height).unwrap(),
                    &budget
                ),
                expected,
                "irot {rotation}, imir {mirror:?}"
            );
        }
    }
    assert_eq!(Orientation::from_avif(4, None), None);
    assert_eq!(Orientation::from_avif(0, Some(2)), None);
}

#[test]
fn reorientation_is_budgeted_reported_and_bounds_checked() {
    let stored = stored_grid(4, 3);
    let orientation = Orientation::from_exif(6).unwrap();
    let mut source =
        OrientedSource::new(Box::new(GridSource::new(stored, orientation, false))).unwrap();
    let region = Rect::new(1, 1, 2, 2).unwrap();

    let tiny = MemoryBudget::new(4);
    assert!(matches!(
        source.read_region(region, &tiny, &mut |_, _| Ok(())),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(tiny.snapshot().live, 0);

    let budget = MemoryBudget::new(1 << 10);
    let report = source
        .read_region(region, &budget, &mut |_, _| Ok(()))
        .unwrap();
    // The 2×2 display crop needs a 2×2 stored region of RGB8 samples.
    assert_eq!(report.engine_buffer_bytes, 2 * 2 * 3);
    assert_eq!(report.requested, region);
    assert_eq!(report.rows, 2);
    assert_eq!(budget.snapshot().live, 0);

    let outside = Rect::new(2, 3, 2, 2).unwrap();
    assert_eq!(
        source
            .read_region(outside, &budget, &mut |_, _| Ok(()))
            .err(),
        Some(Error::OutOfBounds)
    );
    assert_eq!(Orientation::from_exif(0), None);
    assert_eq!(Orientation::from_exif(9), None);
}

/// A stored-order source with `channels` distinct samples per pixel, so a
/// channel or alpha mix-up shows; it also carries non-default alpha and color
/// metadata that reorientation must pass through unchanged.
struct ChannelSource {
    info: DecoderInfo,
    grid: Grid,
    orientation: Orientation,
}

const LAYOUTS: [(ChannelLayout, AlphaMode); 4] = [
    (ChannelLayout::Gray, AlphaMode::Opaque),
    (ChannelLayout::GrayAlpha, AlphaMode::Straight),
    (ChannelLayout::Rgb, AlphaMode::Opaque),
    (ChannelLayout::Rgba, AlphaMode::Premultiplied),
];

fn pixel(id: u8, channels: usize) -> Vec<u8> {
    [id, 255 - id, id ^ 0x55, id / 2 + 1][..channels].to_vec()
}

impl DecodeSource for ChannelSource {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn orientation(&self) -> Orientation {
        self.orientation
    }
    fn streams_rows(&self) -> bool {
        true
    }
    fn read_region(
        &mut self,
        region: Rect,
        _budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        let channels = usize::from(self.info.layout.channels());
        for y in region.y..region.bottom() {
            let row: Vec<u8> = self.grid[y as usize][region.x as usize..region.right() as usize]
                .iter()
                .flat_map(|&id| pixel(id, channels))
                .collect();
            if self.info.sample == SampleType::F32 {
                let floats: Vec<f32> = row.iter().map(|&b| f32::from(b)).collect();
                sink(y, DecodedRow::F32(&floats))?;
            } else {
                sink(y, DecodedRow::U8(&row))?;
            }
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows: u64::from(region.height),
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

/// #68: every orientation keeps each pixel's channels together (including
/// alpha in gray+alpha and RGBA), maps every display crop to the correct
/// stored samples, and passes the alpha mode and rendered color through.
/// Gray+alpha rows failed with `UnsupportedCodecLayout` before 2026-11-12.
#[test]
fn every_orientation_keeps_channels_alpha_and_color_for_every_crop() {
    let stored = stored_grid(5, 3);
    let icc = vec![1, 2, 3];
    for (layout, alpha) in LAYOUTS {
        let channels = usize::from(layout.channels());
        for float in [false, true] {
            for value in 1..=8 {
                let expected = exif_reference(value, &stored);
                let source = ChannelSource {
                    info: DecoderInfo {
                        depth: 8,
                        size: Size::new(5, 3).unwrap(),
                        layout,
                        sample: if float {
                            SampleType::F32
                        } else {
                            SampleType::U8
                        },
                        access: DecodeAccess::CodecRegionRequest,
                        alpha,
                        rendered_icc: Some(icc.clone()),
                        rendered_cicp: Some([12, 13, 0, 1]),
                    },
                    grid: stored.clone(),
                    orientation: Orientation::from_exif(value).unwrap(),
                };
                let mut oriented = OrientedSource::new(Box::new(source)).unwrap();
                let info = oriented.info().clone();
                assert_eq!((info.layout, info.alpha), (layout, alpha));
                assert_eq!(info.rendered_icc.as_deref(), Some(icc.as_slice()));
                assert_eq!(info.rendered_cicp, Some([12, 13, 0, 1]));
                let (width, height) = (info.size.width, info.size.height);
                for y in 0..height {
                    for x in 0..width {
                        let region = Rect::new(x, y, width - x, height - y).unwrap();
                        let budget = MemoryBudget::new(1 << 12);
                        let mut next = y;
                        let report = oriented
                            .read_region(region, &budget, &mut |row_y, row| {
                                assert_eq!(row_y, next);
                                next += 1;
                                let samples: Vec<u8> = match row {
                                    DecodedRow::U8(samples) => samples.to_vec(),
                                    DecodedRow::F32(samples) => {
                                        samples.iter().map(|&v| v as u8).collect()
                                    }
                                };
                                let want: Vec<u8> = expected[row_y as usize]
                                    [x as usize..width as usize]
                                    .iter()
                                    .flat_map(|&id| pixel(id, channels))
                                    .collect();
                                assert_eq!(samples, want, "{layout:?} {value} {region:?}");
                                Ok(())
                            })
                            .unwrap();
                        assert_eq!(next, height);
                        let bytes = region.width as usize
                            * region.height as usize
                            * channels
                            * if float { 4 } else { 1 };
                        if value == 2
                            && u64::from(region.width) * u64::from(region.height)
                                > u64::from(info.size.width)
                        {
                            // Orientation 2 streams row by row when the crop exceeds one source
                            // scanline: zero region buffer, one copy.
                            assert_eq!(report.engine_buffer_bytes, 0);
                            assert_eq!(report.engine_copied_bytes, bytes as u64);
                        } else {
                            assert_eq!(report.engine_buffer_bytes, bytes);
                            assert_eq!(report.engine_copied_bytes, 2 * bytes as u64);
                        }
                        assert_eq!(budget.snapshot().live, 0);
                    }
                }
            }
        }
    }
}

/// #68 accounting at camera sizes, without decoding pixels: orientation 2
/// streams row by row in O(1 row) memory; orientations 3, 4 and 6 materialize
/// the stored region once (and only transposing orientation 6 uses parallel
/// gather blocks when `threads > 1`). One byte too few fails before decoding;
/// the reported bytes are exact.
#[test]
fn camera_size_orientation_costs_are_reported_exactly() {
    for (width, height) in [(6000_u32, 4000_u32), (8000, 6000)] {
        let frame = width as usize * height as usize * 3;
        for value in [2_u16, 3, 4, 6] {
            let orientation = Orientation::from_exif(value).unwrap();
            let display = orientation
                .display_size(Size::new(width, height).unwrap())
                .unwrap();
            let row = display.width as usize * 3;
            let make = || ConstantSource {
                info: DecoderInfo {
                    depth: 8,
                    size: Size::new(width, height).unwrap(),
                    layout: ChannelLayout::Rgb,
                    sample: SampleType::U8,
                    access: DecodeAccess::CodecRegionRequest,
                    alpha: AlphaMode::Opaque,
                    rendered_icc: None,
                    rendered_cicp: Some([1, 13, 0, 1]),
                },
                orientation,
                row: vec![7; width as usize * 3],
            };
            let full = Rect::new(0, 0, display.width, display.height).unwrap();
            let min_budget = if value == 2 { row } else { frame };
            let short = MemoryBudget::new(min_budget - 1);
            let mut oriented = OrientedSource::new(Box::new(make())).unwrap();
            assert!(matches!(
                oriented.read_region(full, &short, &mut |_, _| Ok(())),
                Err(Error::BudgetExceeded { .. })
            ));
            assert_eq!(short.snapshot().live, 0);
            for threads in [1, 8] {
                let budget = MemoryBudget::new(usize::MAX / 4);
                let mut oriented = OrientedSource::new(Box::new(make()))
                    .unwrap()
                    .with_threads(threads);
                let mut rows = 0_u32;
                let report = oriented
                    .read_region(full, &budget, &mut |_, _| {
                        rows += 1;
                        Ok(())
                    })
                    .unwrap();
                assert_eq!(rows, display.height);
                let peak = budget.snapshot().peak;
                if value == 2 {
                    // Streamed horizontal mirror: zero region buffer, one copy, one-row peak.
                    assert_eq!(report.engine_buffer_bytes, 0);
                    assert_eq!(report.engine_copied_bytes, frame as u64);
                    assert_eq!(peak, row, "{width}x{height} {value} t={threads}");
                } else {
                    assert_eq!(report.engine_buffer_bytes, frame);
                    assert_eq!(report.engine_copied_bytes, 2 * frame as u64);
                    // Non-transposing (3, 4) always hold one display row;
                    // transposing (6) holds 16 display rows sequentially or up
                    // to `2 * threads + 1` blocks of 16 rows in parallel.
                    if !orientation.transpose {
                        assert_eq!(peak, frame + row, "{width}x{height} {value} t={threads}");
                    } else if threads == 1 {
                        assert_eq!(peak, frame + 16 * row, "{width}x{height} {value}");
                    } else {
                        // Emitting block is dropped before submitting the next
                        // job, so at most `threads + 1` blocks are live at once.
                        assert!(peak > frame + row && peak <= frame + (threads + 1) * 16 * row);
                    }
                }
                assert_eq!(budget.snapshot().live, 0);
            }
        }
    }
}

/// Emits one constant row per stored row without allocating a frame.
struct ConstantSource {
    info: DecoderInfo,
    orientation: Orientation,
    row: Vec<u8>,
}

impl DecodeSource for ConstantSource {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn orientation(&self) -> Orientation {
        self.orientation
    }
    fn streams_rows(&self) -> bool {
        true
    }
    fn read_region(
        &mut self,
        region: Rect,
        _budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        let start = region.x as usize * 3;
        for y in region.y..region.bottom() {
            sink(
                y,
                DecodedRow::U8(&self.row[start..start + region.width as usize * 3]),
            )?;
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows: u64::from(region.height),
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

/// A stored grid tall and wide enough that every orientation has more
/// display rows than the parallel threshold, with a partial last block.
fn tall_grid() -> Grid {
    (0..90_u32)
        .map(|y| {
            (0..70_u32)
                .map(|x| ((x * 31 + y * 17) ^ (y >> 2)) as u8)
                .collect()
        })
        .collect()
}

#[test]
fn parallel_gather_matches_the_reference_for_every_orientation_and_thread_count() {
    let stored = tall_grid();
    for value in 1..=8 {
        let expected = exif_reference(value, &stored);
        let orientation = Orientation::from_exif(value).unwrap();
        let (width, height) = (expected[0].len() as u32, expected.len() as u32);
        let regions = [
            Rect::new(0, 0, width, height).unwrap(),
            Rect::new(3, 5, width - 7, height - 6).unwrap(),
            Rect::new(1, 0, 9, 64).unwrap(),
        ];
        for float in [false, true] {
            for region in regions {
                let want: Grid = expected[region.y as usize..region.bottom() as usize]
                    .iter()
                    .map(|row| row[region.x as usize..region.right() as usize].to_vec())
                    .collect();
                for threads in [1, 2, 3, 8] {
                    let mut source = OrientedSource::new(Box::new(GridSource::new(
                        stored.clone(),
                        orientation,
                        float,
                    )))
                    .unwrap()
                    .with_threads(threads);
                    let budget = MemoryBudget::new(1 << 20);
                    assert_eq!(
                        read(&mut source, region, &budget),
                        want,
                        "EXIF {value}, float {float}, {region:?}, {threads} threads"
                    );
                    assert_eq!(budget.snapshot().live, 0, "buffers are released");
                }
            }
        }
    }
}

#[test]
fn parallel_gather_cancels_and_fails_cleanly() {
    let orientation = Orientation::from_exif(6).unwrap();
    let make = || {
        OrientedSource::new(Box::new(GridSource::new(tall_grid(), orientation, false)))
            .unwrap()
            .with_threads(4)
    };
    let mut source = make();
    let full = Rect::new(0, 0, 90, 70).unwrap();
    // A sink that cancels after 20 rows stops the workers and frees memory.
    let budget = MemoryBudget::new(1 << 20);
    let mut rows = 0;
    let result = source.read_region(full, &budget, &mut |y, _| {
        if rows == 20 {
            return Err(Error::Cancelled);
        }
        assert_eq!(y, rows);
        rows += 1;
        Ok(())
    });
    assert_eq!(result.err(), Some(Error::Cancelled));
    assert_eq!(rows, 20);
    assert_eq!(budget.snapshot().live, 0);
    // Room for the stored region but not for a block of gathered rows: the
    // worker's allocation error reaches the caller.
    let region_bytes = 90 * 70 * 3;
    let budget = MemoryBudget::new(region_bytes + 100);
    assert!(matches!(
        make().read_region(full, &budget, &mut |_, _| Ok(())),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(budget.snapshot().live, 0);
}

/// Compare an adapter's upright output with ImageMagick/libheif references
/// (`tests/fixtures/orientation/README.md`).
#[cfg(any(feature = "jpeg-read", feature = "avif-read"))]
fn assert_matches_reference(mut source: Box<dyn DecodeSource + '_>, reference: &[u8], name: &str) {
    if !source.orientation().is_identity() {
        source = Box::new(OrientedSource::new(source).unwrap());
    }
    let size = source.info().size;
    assert_eq!(
        reference.len(),
        size.width as usize * size.height as usize * 3,
        "{name}: upright dimensions"
    );
    let budget = MemoryBudget::new(1 << 20);
    let mut actual = Vec::new();
    source
        .read_region(
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &budget,
            &mut |_, row| {
                match row {
                    DecodedRow::U8(samples) => actual.extend_from_slice(samples),
                    DecodedRow::F32(samples) => actual.extend(
                        samples
                            .iter()
                            .map(|&v| (v * 255.0).round().clamp(0.0, 255.0) as u8),
                    ),
                }
                Ok(())
            },
        )
        .unwrap();
    let errors: Vec<u32> = actual
        .iter()
        .zip(reference)
        .map(|(&a, &b)| u32::from(a.abs_diff(b)))
        .collect();
    let mean = f64::from(errors.iter().sum::<u32>()) / errors.len() as f64;
    // A wrong orientation swaps the distinct 4×4 color blocks: mean ≫ 20.
    assert!(mean < 2.0, "{name}: mean error {mean}");
    assert!(errors.iter().all(|&e| e <= 24), "{name}: max error");
}

#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_exif_orientations_match_imagemagick() {
    use crate::codecs::jpeg::JpegSource;
    let fixtures: [(&[u8], &[u8]); 8] = [
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-1.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-1.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-2.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-2.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-3.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-3.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-4.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-4.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-5.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-5.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-6.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-7.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-7.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jpeg-8.jpg"),
            include_bytes!("../tests/fixtures/orientation/jpeg-8.rgb"),
        ),
    ];
    for (index, (file, reference)) in fixtures.iter().enumerate() {
        let source = JpegSource::new(file).unwrap();
        assert_matches_reference(Box::new(source), reference, &format!("jpeg-{}", index + 1));
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_irot_imir_match_libheif() {
    use crate::codecs::avif::AvifSource;
    let fixtures: [(&[u8], &[u8]); 8] = [
        (
            include_bytes!("../tests/fixtures/orientation/avif-1.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-1.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-2.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-2.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-3.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-3.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-4.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-4.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-5.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-5.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-6.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-6.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-7.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-7.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/avif-8.avif"),
            include_bytes!("../tests/fixtures/orientation/avif-8.rgb"),
        ),
    ];
    for (index, (file, reference)) in fixtures.iter().enumerate() {
        let source = AvifSource::new(file).unwrap();
        assert_matches_reference(Box::new(source), reference, &format!("avif-{}", index + 1));
    }
}

#[cfg(feature = "jpeg-xl-read")]
#[test]
fn jpeg_xl_orientations_are_rendered_upright_for_every_crop() {
    use crate::codecs::jxl::JxlSource;
    let fixtures: [(&[u8], &[u8]); 8] = [
        (
            include_bytes!("../tests/fixtures/orientation/jxl-1.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-1.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-2.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-2.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-3.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-3.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-4.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-4.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-5.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-5.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-6.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-6.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-7.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-7.rgb"),
        ),
        (
            include_bytes!("../tests/fixtures/orientation/jxl-8.jxl"),
            include_bytes!("../tests/fixtures/orientation/jxl-8.rgb"),
        ),
    ];
    for (index, (file, reference)) in fixtures.iter().enumerate() {
        let name = format!("jxl-{}", index + 1);
        let mut source = JxlSource::new(file).unwrap();
        assert!(
            source.orientation().is_identity(),
            "{name}: rendered upright"
        );
        let size = source.info().size;
        let width = size.width as usize;
        assert_eq!(reference.len(), width * size.height as usize * 3, "{name}");
        // Lossless fixtures: every display crop must match exactly. Since
        // the region-retaining callback (ADR-0032, 2026-11-12) also 1-pixel
        // crops at the corners, odd offsets and the far edges.
        let crops = [
            Rect::new(0, 0, size.width, size.height).unwrap(),
            Rect::new(3, 2, 5, 7).unwrap(),
            Rect::new(size.width - 6, size.height - 9, 6, 9).unwrap(),
            Rect::new(1, size.height - 1, size.width - 2, 1).unwrap(),
            Rect::new(0, 0, 1, 1).unwrap(),
            Rect::new(size.width - 1, size.height - 1, 1, 1).unwrap(),
            Rect::new(size.width - 1, 0, 1, size.height).unwrap(),
            Rect::new(5, 3, 3, 1).unwrap(),
            Rect::new(1, 1, size.width - 1, size.height - 1).unwrap(),
        ];
        for crop in crops {
            let budget = MemoryBudget::new(1 << 20);
            let mut next = crop.y;
            source
                .read_region(crop, &budget, &mut |y, row| {
                    assert_eq!(y, next);
                    next += 1;
                    let DecodedRow::F32(samples) = row else {
                        panic!("{name}: JPEG XL rows are float");
                    };
                    let start = (y as usize * width + crop.x as usize) * 3;
                    let want = &reference[start..start + crop.width as usize * 3];
                    for (got, &want) in samples.iter().zip(want) {
                        let got = (got * 255.0).round();
                        assert!(
                            (got - f32::from(want)).abs() <= 1.0,
                            "{name}: crop {crop:?} row {y}"
                        );
                    }
                    Ok(())
                })
                .unwrap();
            assert_eq!(next, crop.bottom(), "{name}: crop {crop:?}");
        }
    }
}

/// Lossy (`VarDCT`) JPEG XL files with every header orientation, written and
/// captured as upright RGBA8 by libjxl a16d32d (libjxl-rs
/// `decoder-geometry-fixtures.json`). All eight carry the same stored
/// codestream, so each upright output must be exactly the EXIF transform of
/// the orientation-1 output, and every display crop exactly a slice of it.
#[cfg(feature = "jpeg-xl-read")]
mod lossy_jpeg_xl {
    use super::{DecodeSource, DecodedRow, MemoryBudget, Rect, exif_reference, stored_grid};
    use crate::color::AlphaMode;
    use crate::decoder::ChannelLayout;

    /// RGB samples per pixel.
    const C: usize = 3;

    const FILES: [(&[u8], &[u8]); 8] = [
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-1.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-1.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-2.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-2.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-3.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-3.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-4.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-4.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-5.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-5.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-6.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-6.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-7.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-7.rgba8"),
        ),
        (
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-8.jxl"),
            include_bytes!("../codecs/libjxl-rs/tests/data/decoder-geometry/orientation-8.rgba8"),
        ),
    ];

    fn read(source: &mut dyn DecodeSource, region: Rect) -> Vec<f32> {
        let mut samples = Vec::new();
        let mut next = region.y;
        let report = source
            .read_region(region, &MemoryBudget::new(1 << 20), &mut |y, row| {
                assert_eq!(y, next);
                next += 1;
                let DecodedRow::F32(row) = row else {
                    panic!("JPEG XL rows are float")
                };
                assert_eq!(row.len(), region.width as usize * C);
                samples.extend_from_slice(row);
                Ok(())
            })
            .unwrap();
        assert_eq!(next, region.bottom());
        assert_eq!(report.engine_buffer_bytes, 0, "decoder-side orientation");
        samples
    }

    /// Returns the largest 8-bit difference to libjxl's captures.
    pub(super) fn check(make: &dyn Fn(&'static [u8]) -> Box<dyn DecodeSource>) -> u8 {
        let stored = read(&mut *make(FILES[0].0), Rect::new(0, 0, 13, 11).unwrap());
        let ids = stored_grid(13, 11);
        let mut worst = 0;
        for (index, &(file, capture)) in FILES.iter().enumerate() {
            let value = u16::try_from(index + 1).unwrap();
            let mut source = make(file);
            assert!(source.orientation().is_identity(), "orientation {value}");
            // The files carry no alpha channel; libjxl captured opaque RGBA.
            assert_eq!(source.info().layout, ChannelLayout::Rgb);
            assert_eq!(source.info().alpha, AlphaMode::Opaque);
            let expected = exif_reference(value, &ids);
            let (width, height) = (expected[0].len(), expected.len());
            let size = source.info().size;
            assert_eq!((size.width as usize, size.height as usize), (width, height));
            let full = read(
                &mut *source,
                Rect::new(0, 0, size.width, size.height).unwrap(),
            );
            for (y, row) in expected.iter().enumerate() {
                for (x, &id) in row.iter().enumerate() {
                    let at = (y * width + x) * C;
                    let from = (usize::from(id) - 1) * C;
                    assert_eq!(
                        full[at..at + C],
                        stored[from..from + C],
                        "orientation {value}: pixel ({x}, {y})"
                    );
                    // libjxl's capture: 16 padding bytes after each row.
                    let captured = &capture[y * (width * 4 + 16) + x * 4..][..4];
                    assert_eq!(captured[3], 255, "opaque capture");
                    for (&got, &want) in full[at..at + C].iter().zip(captured) {
                        let got = (got * 255.0).round().clamp(0.0, 255.0) as u8;
                        worst = worst.max(got.abs_diff(want));
                    }
                }
            }
            for crop in [
                Rect::new(3, 2, 5, 7).unwrap(),
                Rect::new(size.width - 6, size.height - 9, 6, 9).unwrap(),
                Rect::new(1, size.height - 1, size.width - 2, 1).unwrap(),
            ] {
                let got = read(&mut *source, crop);
                for (row, y) in got.chunks(crop.width as usize * C).zip(crop.y..) {
                    let at = (y as usize * width + crop.x as usize) * C;
                    assert_eq!(
                        row,
                        &full[at..at + crop.width as usize * C],
                        "orientation {value}: crop {crop:?}"
                    );
                }
            }
        }
        worst
    }
}

#[cfg(feature = "jpeg-xl-read")]
#[test]
fn lossy_jpeg_xl_orientations_and_crops_through_libjxl_rs() {
    use crate::codecs::jxl::JxlSource;
    let worst = lossy_jpeg_xl::check(&|bytes| Box::new(JxlSource::new(bytes).unwrap()));
    assert_eq!(worst, 0, "libjxl-rs versus libjxl capture");
}

#[test]
fn partial_gather_worker_start_failure_releases_buffers_and_returns_error() {
    let orientation = Orientation::from_exif(6).unwrap();
    for successful in [0, 1, 3] {
        let _failure = crate::thread_spawn::fail_after(successful);
        let mut source =
            OrientedSource::new(Box::new(GridSource::new(tall_grid(), orientation, false)))
                .unwrap()
                .with_threads(4);
        let budget = MemoryBudget::new(1 << 20);
        let mut rows = 0;
        let result = source.read_region(Rect::new(0, 0, 90, 70).unwrap(), &budget, &mut |_, _| {
            rows += 1;
            Ok(())
        });
        assert!(matches!(result, Err(Error::ThreadSpawnFailed)));
        assert_eq!(rows, 0);
        assert_eq!(budget.snapshot().live, 0);
    }
}

/// Issue #68: EXIF orientation 2 (`flip_x` only) streams row by row in a
/// single-row budget, cancels cleanly mid-stream, and mirrors crop x-offsets.
#[test]
fn orientation_2_streams_in_one_row_budget_and_cancels_cleanly() {
    let stored = tall_grid();
    let expected = exif_reference(2, &stored);
    let orientation = Orientation::from_exif(2).unwrap();
    let crop = Rect::new(7, 11, 31, 45).unwrap();
    let row_bytes = crop.width as usize * 3;
    let exact_budget = MemoryBudget::new(row_bytes);
    let mut source = OrientedSource::new(Box::new(GridSource::new(
        stored.clone(),
        orientation,
        false,
    )))
    .unwrap()
    .with_threads(8);
    let got = read(&mut source, crop, &exact_budget);
    let want: Grid = expected[crop.y as usize..crop.bottom() as usize]
        .iter()
        .map(|row| row[crop.x as usize..crop.right() as usize].to_vec())
        .collect();
    assert_eq!(got, want);
    assert_eq!(exact_budget.snapshot().peak, row_bytes);
    assert_eq!(exact_budget.snapshot().live, 0);

    // Cancelling mid-stream stops immediately and releases the single row buffer.
    let mut seen = 0_u32;
    let err = source.read_region(crop, &exact_budget, &mut |y, _| {
        assert_eq!(y, crop.y + seen);
        if seen == 5 {
            return Err(Error::Cancelled);
        }
        seen += 1;
        Ok(())
    });
    assert_eq!(err.err(), Some(Error::Cancelled));
    assert_eq!(seen, 5);
    assert_eq!(exact_budget.snapshot().live, 0);
}

/// Issue #68: orientations 3 and 4 use fast whole-row copy/reverse on one
/// thread even when `threads > 1`, keeping peak memory at `frame + 1 row`.
#[test]
fn orientations_3_and_4_match_reference_and_avoid_parallel_gather_blocks() {
    let stored: Grid = (0..97_u32)
        .map(|y| {
            (0..103_u32)
                .map(|x| ((x * 29 + y * 13) ^ (x >> 1)) as u8)
                .collect()
        })
        .collect();
    for (layout, alpha) in LAYOUTS {
        let channels = usize::from(layout.channels());
        for float in [false, true] {
            let sample_bytes = if float { 4 } else { 1 };
            for value in [3_u16, 4] {
                let orientation = Orientation::from_exif(value).unwrap();
                let expected = exif_reference(value, &stored);
                let source = ChannelSource {
                    info: DecoderInfo {
                        depth: 8,
                        size: Size::new(103, 97).unwrap(),
                        layout,
                        sample: if float {
                            SampleType::F32
                        } else {
                            SampleType::U8
                        },
                        access: DecodeAccess::CodecRegionRequest,
                        alpha,
                        rendered_icc: None,
                        rendered_cicp: Some([1, 13, 0, 1]),
                    },
                    grid: stored.clone(),
                    orientation,
                };
                let mut oriented = OrientedSource::new(Box::new(source))
                    .unwrap()
                    .with_threads(8);
                let full = Rect::new(0, 0, 103, 97).unwrap();
                let budget = MemoryBudget::new(1 << 22);
                let mut next = 0_u32;
                let report = oriented
                    .read_region(full, &budget, &mut |y, row| {
                        assert_eq!(y, next);
                        next += 1;
                        let samples: Vec<u8> = match row {
                            DecodedRow::U8(s) => s.to_vec(),
                            DecodedRow::F32(s) => s.iter().map(|&v| v as u8).collect(),
                        };
                        let want: Vec<u8> = expected[y as usize]
                            .iter()
                            .flat_map(|&id| pixel(id, channels))
                            .collect();
                        assert_eq!(samples, want);
                        Ok(())
                    })
                    .unwrap();
                let frame = 103 * 97 * channels * sample_bytes;
                let row = 103 * channels * sample_bytes;
                assert_eq!(report.engine_buffer_bytes, frame);
                assert_eq!(report.engine_copied_bytes, 2 * frame as u64);
                assert_eq!(budget.snapshot().peak, frame + row);
                assert_eq!(budget.snapshot().live, 0);
            }
        }
    }
}

/// Issue #68 / ADR-0019: frame-backed sources (`streams_rows() == false`) keep
/// the region-buffered path for EXIF orientation 2 so the decoder releases its
/// decoded frame before downstream sink/resize allocations run.
#[test]
fn orientation_2_buffers_frame_backed_sources_instead_of_streaming() {
    struct FrameBackedSource {
        inner: GridSource,
        frame_bytes: usize,
    }
    impl DecodeSource for FrameBackedSource {
        fn info(&self) -> &DecoderInfo {
            self.inner.info()
        }
        fn orientation(&self) -> Orientation {
            self.inner.orientation()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            let _frame = budget.allocate(self.frame_bytes)?;
            self.inner.read_region(region, budget, sink)
        }
    }

    let stored = tall_grid();
    let expected = exif_reference(2, &stored);
    let orientation = Orientation::from_exif(2).unwrap();
    let crop = Rect::new(5, 8, 20, 30).unwrap();
    let region_bytes = crop.width as usize * crop.height as usize * 3;
    let row_bytes = crop.width as usize * 3;
    let frame_bytes = 64 * 1024;
    let sink_bytes = 64 * 1024;
    // Budget fits `frame_bytes + region_bytes` and `region_bytes + row_bytes +
    // sink_bytes`, but NOT `frame_bytes + sink_bytes` simultaneously.
    let budget = MemoryBudget::new(frame_bytes + region_bytes + row_bytes);
    let mut source = OrientedSource::new(Box::new(FrameBackedSource {
        inner: GridSource::new(stored, orientation, false),
        frame_bytes,
    }))
    .unwrap();
    let mut rows = Vec::new();
    let report = source
        .read_region(crop, &budget, &mut |_, row| {
            let _downstream = budget.allocate(sink_bytes)?;
            let DecodedRow::U8(samples) = row else {
                panic!("expected u8");
            };
            rows.push(samples.chunks(3).map(|px| px[0]).collect::<Vec<u8>>());
            Ok(())
        })
        .unwrap();
    let want: Grid = expected[crop.y as usize..crop.bottom() as usize]
        .iter()
        .map(|row| row[crop.x as usize..crop.right() as usize].to_vec())
        .collect();
    assert_eq!(rows, want);
    assert_eq!(report.engine_buffer_bytes, region_bytes);
    assert_eq!(report.engine_copied_bytes, 2 * region_bytes as u64);
    assert_eq!(budget.snapshot().live, 0);
}

/// Regression for PR #297 review: a lossy WebP with EXIF orientation 2 holds
/// a decoded frame buffer inside `MemoryBudget` while emitting rows. Keeping
/// orientation 2 buffered for non-streaming sources releases that decoded
/// frame before `WorkingPrecision::F64Reference` resize buffers allocate,
/// fitting a 10 MiB budget for an `8×256` crop -> `1000×1`.
#[cfg(all(feature = "webp-read", feature = "webp-write"))]
#[test]
fn lossy_webp_orientation_2_crop_fits_tight_budget_without_overlapping_resize() {
    use crate::pipeline::{
        Crop, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings, ResizeFilter,
        TransformOptions, WebpOptions, WorkingPrecision, transcode,
    };
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossy};

    fn chunk(tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        if !payload.len().is_multiple_of(2) {
            out.push(0);
        }
        out
    }

    fn tiff_orientation(value: u16) -> Vec<u8> {
        let mut tiff = b"II".to_vec();
        tiff.extend_from_slice(&42_u16.to_le_bytes());
        tiff.extend_from_slice(&8_u32.to_le_bytes());
        tiff.extend_from_slice(&1_u16.to_le_bytes());
        tiff.extend_from_slice(&0x0112_u16.to_le_bytes());
        tiff.extend_from_slice(&3_u16.to_le_bytes());
        tiff.extend_from_slice(&1_u32.to_le_bytes());
        tiff.extend_from_slice(&value.to_le_bytes());
        tiff.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        tiff
    }

    let width = 1024_u32;
    let height = 512_u32;
    let mut rgb = vec![0_u8; width as usize * height as usize * 3];
    for y in 0..height as usize {
        for x in 0..width as usize {
            let at = (y * width as usize + x) * 3;
            rgb[at] = ((x * 3 + y) % 256) as u8;
            rgb[at + 1] = ((x + y * 5) % 256) as u8;
            rgb[at + 2] = ((x ^ y) % 256) as u8;
        }
    }
    let rows = PixelRows {
        layout: PixelLayout::Rgb,
        data: &rgb,
        width,
        height,
        stride: width as usize * 3,
    };
    let plain = encode_lossy(rows, 75.0).unwrap();
    let w1 = (width - 1).to_le_bytes();
    let h1 = (height - 1).to_le_bytes();
    let mut body = chunk(
        *b"VP8X",
        &[0x08, 0, 0, 0, w1[0], w1[1], w1[2], h1[0], h1[1], h1[2]],
    );
    body.extend_from_slice(&plain[12..]);
    body.extend(chunk(*b"EXIF", &tiff_orientation(2)));
    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&u32::try_from(body.len() + 4).unwrap().to_le_bytes());
    webp.extend_from_slice(b"WEBP");
    webp.extend(body);

    let make_opts = |budget: usize| {
        TransformOptions::new(OutputGeometry::Exact {
            width: 1000,
            height: 1,
        })
        .with_crop(Some(Crop::new(0, 0, 8, 256)))
        .with_filter(ResizeFilter::Area)
        .with_output(OutputSettings::WebP(WebpOptions::default()))
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_precision(WorkingPrecision::F64Reference)
        .with_max_managed_bytes(budget)
    };
    let tight = transcode(&webp, ImageFormat::WebP, &make_opts(10 << 20)).unwrap();
    let generous = transcode(&webp, ImageFormat::WebP, &make_opts(64 << 20)).unwrap();
    assert_eq!(tight.report.orientation_buffer_bytes, 8 * 256 * 3);
    assert!(tight.report.peak_managed_bytes <= 10 << 20);
    assert_eq!(tight.bytes, generous.bytes);
}

/// Regression for PR #297 Round-7 review (Finding #2): for a tiny `1×1` crop
/// of an EXIF-2 JPEG (`stored.width * stored.height <= source.info().size.width`),
/// `OrientedSource` buffers the 3-byte crop rather than streaming so the JPEG
/// decoder releases its iMCU scanline buffer before downstream `F64Reference`
/// resize buffers allocate.
#[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
#[test]
fn exif_orientation_2_tiny_crop_buffers_to_release_jpeg_scanline() {
    use crate::codecs::jpeg::JpegSource;
    use crate::pipeline::{
        Crop, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings, Threads,
        TransformOptions, WebpLossy, WebpOptions, WorkingPrecision, transcode,
    };

    let paris = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    // Minimal 36-byte EXIF APP1 segment setting Orientation = 2 (little-endian).
    let exif_app1: [u8; 36] = [
        0xff, 0xe1, 0x00, 0x22, b'E', b'x', b'i', b'f', 0x00, 0x00, b'I', b'I', 0x2a, 0x00, 0x08,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x12, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let mut exif2_jpeg = Vec::with_capacity(paris.len() + exif_app1.len());
    exif2_jpeg.extend_from_slice(&paris[..2]);
    exif2_jpeg.extend_from_slice(&exif_app1);
    exif2_jpeg.extend_from_slice(&paris[2..]);

    // Verify `OrientedSource` buffers the 1×1 crop (3 bytes) and releases the
    // JPEG scanline before invoking `sink`.
    let jpeg = JpegSource::new(&exif2_jpeg).unwrap();
    assert_eq!(jpeg.orientation(), Orientation::from_exif(2).unwrap());
    let mut oriented = OrientedSource::new(Box::new(jpeg)).unwrap();
    let probe_budget = MemoryBudget::new(64 * 1024);
    let report = oriented
        .read_region(
            Rect::new(0, 0, 1, 1).unwrap(),
            &probe_budget,
            &mut |_, _| Ok(()),
        )
        .unwrap();
    assert_eq!(report.engine_buffer_bytes, 3);
    assert_eq!(report.engine_copied_bytes, 6);
    let buffered_scanline_peak = probe_budget.snapshot().peak;

    // A downstream sink allocation that fits after the scanline is released
    // (but would overflow if the scanline stayed live) must succeed.
    let sink_bytes = buffered_scanline_peak / 2;
    let tight_budget = MemoryBudget::new(buffered_scanline_peak);
    oriented
        .read_region(
            Rect::new(0, 0, 1, 1).unwrap(),
            &tight_budget,
            &mut |_, _| {
                let _downstream = tight_budget.allocate(sink_bytes)?;
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(tight_budget.snapshot().live, 0);

    // End-to-end F64Reference transcode of the 1×1 crop reports buffered
    // orientation (3 bytes) and succeeds at its exact buffered peak.
    let opts = TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_crop(Some(Crop::new(0, 0, 1, 1)))
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_precision(WorkingPrecision::F64Reference)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);
    let generous = transcode(&exif2_jpeg, ImageFormat::Jpeg, &opts).unwrap();
    assert_eq!(generous.report.orientation_buffer_bytes, 3);
    let tight = transcode(
        &exif2_jpeg,
        ImageFormat::Jpeg,
        &opts.with_max_managed_bytes(generous.report.peak_managed_bytes),
    )
    .unwrap();
    assert_eq!(tight.bytes, generous.bytes);
}
