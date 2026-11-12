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
                        assert_eq!(report.engine_buffer_bytes, bytes);
                        assert_eq!(report.engine_copied_bytes, 2 * bytes as u64);
                        assert_eq!(budget.snapshot().live, 0);
                    }
                }
            }
        }
    }
}

/// #68 accounting at camera sizes, without decoding pixels: the stored
/// region, the bytes copied and the bounded parallel blocks for a full 24 Mpx
/// and 48 Mpx RGB8 frame in every orientation. One full-frame budget too few
/// bytes fails before decoding; the reported bytes are exact.
#[test]
fn camera_size_orientation_costs_are_reported_exactly() {
    for (width, height) in [(6000_u32, 4000_u32), (8000, 6000)] {
        let frame = width as usize * height as usize * 3;
        for value in [2_u16, 6] {
            let orientation = Orientation::from_exif(value).unwrap();
            let display = orientation
                .display_size(Size::new(width, height).unwrap())
                .unwrap();
            let make = || ConstantSource {
                info: DecoderInfo {
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
            let short = MemoryBudget::new(frame - 1);
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
                assert_eq!(report.engine_buffer_bytes, frame);
                assert_eq!(report.engine_copied_bytes, 2 * frame as u64);
                let row = display.width as usize * 3;
                // Sequential: the stored frame plus one display row. Parallel:
                // at most 2 × threads + 1 blocks of 16 rows on top of the frame.
                let peak = budget.snapshot().peak;
                if threads == 1 {
                    assert_eq!(peak, frame + row, "{width}x{height} {value}");
                } else {
                    assert!(peak > frame + row && peak <= frame + (2 * threads + 1) * 16 * row);
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

#[cfg(feature = "jpeg-xl-own-read")]
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
        // Lossless fixtures: every display crop must match exactly.
        let crops = [
            Rect::new(0, 0, size.width, size.height).unwrap(),
            Rect::new(3, 2, 5, 7).unwrap(),
            Rect::new(size.width - 6, size.height - 9, 6, 9).unwrap(),
            Rect::new(1, size.height - 1, size.width - 2, 1).unwrap(),
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

// Preserve the default decoder contract while the workspace port remains opt-in.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn interim_jpeg_xl_orientations_are_rendered_upright_for_every_crop() {
    use crate::codecs::jxl_oxide::JxlSource;
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
        // Lossless fixtures: every display crop must match exactly.
        let crops = [
            Rect::new(0, 0, size.width, size.height).unwrap(),
            Rect::new(3, 2, 5, 7).unwrap(),
            Rect::new(size.width - 6, size.height - 9, 6, 9).unwrap(),
            Rect::new(1, size.height - 1, size.width - 2, 1).unwrap(),
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
#[cfg(any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"))]
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
fn lossy_jpeg_xl_orientations_and_crops_through_jxl_oxide() {
    use crate::codecs::jxl_oxide::JxlSource;
    let worst = lossy_jpeg_xl::check(&|bytes| Box::new(JxlSource::new(bytes).unwrap()));
    assert_eq!(worst, 0, "jxl-oxide versus libjxl capture");
}

#[cfg(feature = "jpeg-xl-own-read")]
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
