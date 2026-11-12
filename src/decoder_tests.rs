//! Codec-to-planner contract tests with tiny committed real bitstreams.
use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
    execute_decoded,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::graph::Graph;
use crate::memory::MemoryBudget;

fn rect(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect::new(x, y, w, h).unwrap()
}

struct Probe {
    info: DecoderInfo,
    requested: Option<Rect>,
}
impl Probe {
    fn new() -> Self {
        Self {
            info: DecoderInfo {
                size: Size::new(10, 8).unwrap(),
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: None,
                rendered_cicp: None,
            },
            requested: None,
        }
    }
}
impl DecodeSource for Probe {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn read_region(
        &mut self,
        region: Rect,
        _: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.requested = Some(region);
        let row = vec![123_u8; region.width as usize * 3];
        for y in region.y..region.bottom() {
            sink(y, DecodedRow::U8(&row))?;
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows: u64::from(region.height),
            output_sample_bytes: u64::from(region.width) * u64::from(region.height) * 3,
            known_full_frame_buffers: 0,
            peak_engine_managed_bytes: 0,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
            engine_copied_bytes: 0,
            work: crate::decoder::CodecWork::region_at_least(region),
        })
    }
}

#[test]
fn nested_crops_request_only_needed_source_pixels() {
    let mut source = Probe::new();
    let mut graph = Graph::new();
    let id = graph
        .decoded_source(
            source.info.size,
            source.info.pixel_format(),
            source.info.semantics(),
        )
        .unwrap();
    let cropped = graph.crop(id, rect(2, 1, 7, 6)).unwrap();
    let cropped = graph.crop(cropped, rect(1, 2, 4, 3)).unwrap();
    let mut ys = Vec::new();
    let report = execute_decoded(
        &graph,
        cropped,
        id,
        &mut source,
        rect(1, 1, 2, 2),
        &MemoryBudget::new(0),
        |y, row| {
            ys.push(y);
            assert!(matches!(row, DecodedRow::U8(data) if data == [123; 6]));
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(source.requested, Some(rect(4, 4, 2, 2)));
    assert_eq!(ys, vec![1, 2]);
    assert_eq!(report.known_full_frame_buffers, 0);
}

#[test]
fn source_binding_and_cancellation_are_checked() {
    let mut source = Probe::new();
    let mut graph = Graph::new();
    let id = graph
        .decoded_source(
            source.info.size,
            source.info.pixel_format(),
            source.info.semantics(),
        )
        .unwrap();
    let wrong = graph
        .decoded_source(
            Size::new(2, 2).unwrap(),
            source.info.pixel_format(),
            source.info.semantics(),
        )
        .unwrap();
    assert_eq!(
        execute_decoded(
            &graph,
            id,
            wrong,
            &mut source,
            rect(0, 0, 1, 1),
            &MemoryBudget::new(0),
            |_, _| Ok(())
        ),
        Err(Error::SourceBindingMismatch)
    );
    let mut called = 0;
    assert_eq!(
        execute_decoded(
            &graph,
            id,
            id,
            &mut source,
            rect(0, 0, 1, 2),
            &MemoryBudget::new(0),
            |_, _| {
                called += 1;
                Err(Error::Cancelled)
            }
        ),
        Err(Error::Cancelled)
    );
    assert_eq!(called, 1);
}

#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_region_request_is_explicit_and_budgeted() {
    use crate::codecs::jpeg::JpegSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jpg");
    let mut source = JpegSource::new(bytes).unwrap();
    assert_eq!(source.info().size, Size::new(7, 5).unwrap());
    assert_eq!(source.info().layout, ChannelLayout::Rgb);
    assert_eq!(source.info().access, DecodeAccess::CodecRegionRequest);
    let mut graph = Graph::new();
    let id = graph
        .decoded_source(
            source.info().size,
            source.info().pixel_format(),
            source.info().semantics(),
        )
        .unwrap();
    let crop = graph.crop(id, rect(2, 1, 4, 3)).unwrap();
    let mut ys = Vec::new();
    let budget = MemoryBudget::new(18);
    let report = execute_decoded(
        &graph,
        crop,
        id,
        &mut source,
        rect(1, 1, 2, 2),
        &budget,
        |y, pixels| {
            assert!(matches!(pixels, DecodedRow::U8(row) if row.len() == 6));
            ys.push(y);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(ys, vec![1, 2]);
    assert_eq!(report.requested, rect(3, 2, 2, 2));
    assert_eq!(report.access, DecodeAccess::CodecRegionRequest);
    assert_eq!(report.known_full_frame_buffers, 0);
    assert_eq!(report.output_sample_bytes, 12);
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(budget.snapshot().peak, 18);
    assert_eq!(
        source.read_region(rect(3, 2, 2, 2), &MemoryBudget::new(17), &mut |_, _| Ok(())),
        Err(Error::BudgetExceeded {
            requested: 18,
            available: 17
        })
    );
    assert!(matches!(
        JpegSource::new(b"invalid"),
        Err(Error::DecodeFailed)
    ));
}

#[cfg(feature = "jpeg-xl-own-read")]
#[test]
fn jpeg_xl_reports_full_frame_fallback_and_delivers_float_scanlines() {
    use crate::codecs::jxl::JxlSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jxl");
    let mut source = JxlSource::new(bytes).unwrap();
    assert_eq!(source.info().size, Size::new(7, 5).unwrap());
    assert_eq!(source.info().layout, ChannelLayout::Rgb);
    assert_eq!(source.info().sample, SampleType::F32);
    assert_eq!(source.info().access, DecodeAccess::FullFrameFallback);
    let mut graph = Graph::new();
    let id = graph
        .decoded_source(
            source.info().size,
            source.info().pixel_format(),
            source.info().semantics(),
        )
        .unwrap();
    let crop = graph.crop(id, rect(1, 1, 5, 3)).unwrap();
    // The old ROI-sized allowance cannot hide a full-frame materialization.
    assert_eq!(
        source.read_region(rect(2, 2, 2, 2), &MemoryBudget::new(24), &mut |_, _| Ok(())),
        Err(Error::BudgetExceeded {
            requested: 420,
            available: 24
        })
    );
    let budget = MemoryBudget::new(420);
    let mut ys = Vec::new();
    let report = execute_decoded(
        &graph,
        crop,
        id,
        &mut source,
        rect(1, 1, 2, 2),
        &budget,
        |y, row| {
            assert!(matches!(row, DecodedRow::F32(samples) if samples.len() == 6));
            ys.push(y);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(ys, vec![1, 2]);
    assert_eq!(report.requested, rect(2, 2, 2, 2));
    assert_eq!(report.known_full_frame_buffers, 1);
    assert_eq!(report.output_sample_bytes, 48);
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(budget.snapshot().peak, 420);
    assert!(matches!(
        JxlSource::new(b"invalid"),
        Err(Error::DecodeFailed)
    ));
    assert!(matches!(
        JxlSource::new_bounded(bytes, 34),
        Err(Error::InputPixelLimitExceeded {
            pixels: 35,
            limit: 34
        })
    ));
}

#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_cropped_samples_match_full_decoded_output() {
    use crate::codecs::jpeg::JpegSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jpg");
    let mut source = JpegSource::new(bytes).unwrap();
    let mut full = Vec::new();
    source
        .read_region(rect(0, 0, 7, 5), &MemoryBudget::new(105), &mut |_, row| {
            let DecodedRow::U8(data) = row else {
                panic!("JPEG must deliver RGB8");
            };
            full.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    let mut crop = Vec::new();
    source
        .read_region(rect(2, 1, 3, 2), &MemoryBudget::new(105), &mut |_, row| {
            let DecodedRow::U8(data) = row else {
                panic!("JPEG must deliver RGB8");
            };
            crop.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    for y in 0..2_usize {
        let offset = ((y + 1) * 7 + 2) * 3;
        assert_eq!(&crop[y * 9..(y + 1) * 9], &full[offset..offset + 9]);
    }
}

#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_imcu_boundary_crops_match_full_rgb_rows() {
    use crate::codecs::jpeg::JpegSource;

    let bytes =
        include_bytes!("../codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg");
    let mut source = JpegSource::new(bytes).unwrap();
    let size = source.info().size;
    let row_bytes = size.width as usize * 3;
    let mut full = Vec::with_capacity(row_bytes * size.height as usize);
    source
        .read_region(
            rect(0, 0, size.width, size.height),
            &MemoryBudget::new(row_bytes),
            &mut |_, row| {
                let DecodedRow::U8(data) = row else {
                    panic!("RGB8 expected")
                };
                full.extend_from_slice(data);
                Ok(())
            },
        )
        .unwrap();

    for region in [
        rect(0, 17, 41, 19),
        rect(16, 17, 41, 19),
        rect(17, 17, 41, 19),
        rect(32, 29, 33, 23),
        rect(33, 29, 33, 23),
        rect(127, 55, 65, 17),
        rect(2240, 17, 28, 19),
    ] {
        source
            .read_region(region, &MemoryBudget::new(row_bytes), &mut |y, row| {
                let DecodedRow::U8(data) = row else {
                    panic!("RGB8 expected")
                };
                let offset = (y as usize * size.width as usize + region.x as usize) * 3;
                assert_eq!(data, &full[offset..offset + region.width as usize * 3]);
                Ok(())
            })
            .unwrap();
    }
}

#[cfg(feature = "jpeg-xl-own-read")]
#[test]
fn jpeg_xl_region_samples_match_full_render() {
    use crate::codecs::jxl::JxlSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jxl");
    let mut source = JxlSource::new(bytes).unwrap();
    let mut full = Vec::new();
    source
        .read_region(rect(0, 0, 7, 5), &MemoryBudget::new(420), &mut |_, row| {
            let DecodedRow::F32(data) = row else {
                panic!("JPEG XL must deliver f32");
            };
            full.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    let mut crop = Vec::new();
    source
        .read_region(rect(2, 1, 3, 2), &MemoryBudget::new(420), &mut |_, row| {
            let DecodedRow::F32(data) = row else {
                panic!("JPEG XL must deliver f32");
            };
            crop.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    for y in 0..2_usize {
        for x in 0..9_usize {
            let expected = full[(y + 1) * 21 + 6 + x];
            let actual = crop[y * 9 + x];
            assert!(actual.is_finite() && expected.is_finite());
            assert!((actual - expected).abs() <= 1.0e-4);
        }
    }
}

// Preserve the default decoder contract while the workspace port remains opt-in.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn interim_jpeg_xl_requests_codec_region_and_delivers_float_scanlines() {
    use crate::codecs::jxl_oxide::JxlSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jxl");
    let mut source = JxlSource::new(bytes).unwrap();
    assert_eq!(source.info().size, Size::new(7, 5).unwrap());
    assert_eq!(source.info().layout, ChannelLayout::Rgb);
    assert_eq!(source.info().sample, SampleType::F32);
    assert_eq!(source.info().access, DecodeAccess::CodecRegionRequest);
    let mut graph = Graph::new();
    let id = graph
        .decoded_source(
            source.info().size,
            source.info().pixel_format(),
            source.info().semantics(),
        )
        .unwrap();
    let crop = graph.crop(id, rect(1, 1, 5, 3)).unwrap();
    let budget = MemoryBudget::new(24);
    let mut ys = Vec::new();
    let report = execute_decoded(
        &graph,
        crop,
        id,
        &mut source,
        rect(1, 1, 2, 2),
        &budget,
        |y, row| {
            assert!(matches!(row, DecodedRow::F32(samples) if samples.len() == 6));
            ys.push(y);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(ys, vec![1, 2]);
    assert_eq!(report.requested, rect(2, 2, 2, 2));
    assert_eq!(report.known_full_frame_buffers, 0);
    assert_eq!(report.output_sample_bytes, 48);
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(budget.snapshot().peak, 24);
    assert!(matches!(
        JxlSource::new(b"invalid"),
        Err(Error::DecodeFailed)
    ));
}

#[cfg(feature = "jpeg-xl-read")]
#[test]
fn interim_jpeg_xl_region_samples_match_full_render() {
    use crate::codecs::jxl_oxide::JxlSource;
    let bytes = include_bytes!("../tests/fixtures/sample.jxl");
    let mut source = JxlSource::new(bytes).unwrap();
    let mut full = Vec::new();
    source
        .read_region(rect(0, 0, 7, 5), &MemoryBudget::new(84), &mut |_, row| {
            let DecodedRow::F32(data) = row else {
                panic!("JPEG XL must deliver f32");
            };
            full.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    let mut crop = Vec::new();
    source
        .read_region(rect(2, 1, 3, 2), &MemoryBudget::new(36), &mut |_, row| {
            let DecodedRow::F32(data) = row else {
                panic!("JPEG XL must deliver f32");
            };
            crop.extend_from_slice(data);
            Ok(())
        })
        .unwrap();
    for y in 0..2_usize {
        for x in 0..9_usize {
            let expected = full[(y + 1) * 21 + 6 + x];
            let actual = crop[y * 9 + x];
            assert!(actual.is_finite() && expected.is_finite());
            assert!((actual - expected).abs() <= 1.0e-4);
        }
    }
}
