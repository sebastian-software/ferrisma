#![cfg(feature = "avif-write")]
use crate::avif_sink::{AvifSink, EncodeSettings, MAX_ENCODER_THREADS, test_settings};
use crate::error::Error;
use crate::geometry::Size;
use crate::memory::MemoryBudget;

const BUDGET: usize = 16 << 20;

fn encode(
    width: u32,
    height: u32,
    transparent: bool,
    settings: EncodeSettings,
    pixel: impl Fn(u32, u32) -> [u8; 4],
) -> Vec<u8> {
    let budget = MemoryBudget::new(BUDGET);
    let mut sink = AvifSink::new(
        Size::new(width, height).unwrap(),
        transparent,
        settings,
        &budget,
    )
    .unwrap();
    for y in 0..height {
        let row: Vec<u8> = (0..width).flat_map(|x| pixel(x, y)).collect();
        sink.write_row(y, &row).unwrap();
    }
    let encoded = sink.finish(&budget).unwrap();
    assert_eq!(budget.snapshot().live, 0);
    assert!(encoded.frame_materialization_bytes > 0);
    assert!(budget.snapshot().peak >= encoded.frame_materialization_bytes);
    encoded.bytes
}

#[test]
fn sink_requires_complete_ordered_rows_and_releases_its_budget() {
    let budget = MemoryBudget::new(BUDGET);
    let size = Size::new(4, 4).unwrap();
    let mut sink = AvifSink::new(size, false, test_settings(75.0, 9), &budget).unwrap();
    assert_eq!(sink.write_row(1, &[0; 16]), Err(Error::IncompleteOutput));
    assert_eq!(sink.write_row(0, &[0; 12]), Err(Error::IncompleteOutput));
    for y in 0..3 {
        sink.write_row(y, &[80, 120, 200, 255].repeat(4)).unwrap();
    }
    assert!(budget.snapshot().live > 0);
    assert_eq!(sink.finish(&budget).unwrap_err(), Error::IncompleteOutput);
    assert_eq!(budget.snapshot().live, 0);

    let bytes = encode(4, 4, false, test_settings(75.0, 9), |_, _| {
        [80, 120, 200, 255]
    });
    assert_eq!(&bytes[4..8], b"ftyp");
}

#[test]
fn the_encoder_frame_is_charged_to_the_budget() {
    let size = Size::new(4, 4).unwrap();
    assert!(matches!(
        AvifSink::new(
            size,
            false,
            test_settings(75.0, 9),
            &MemoryBudget::new(1024)
        ),
        Err(Error::BudgetExceeded { .. })
    ));
}

#[test]
fn invalid_encode_settings_fail_closed() {
    for settings in [
        test_settings(f32::NAN, 9),
        test_settings(0.5, 9),
        test_settings(50.0, 11),
        test_settings(50.0, 0),
        EncodeSettings {
            threads: 0,
            ..test_settings(50.0, 9)
        },
        EncodeSettings {
            threads: MAX_ENCODER_THREADS + 1,
            ..test_settings(50.0, 9)
        },
    ] {
        assert_eq!(settings.validate(), Err(Error::InvalidEncodeOptions));
    }
}

#[test]
fn size_derived_tiles_make_bytes_independent_of_the_thread_count() {
    // 1024×512 is eight 256×256 tile areas.
    let pattern = |x: u32, y: u32| {
        let value = u8::try_from((x * 7 + y * 3) % 256).unwrap();
        [value, 255 - value, value / 2, 255]
    };
    let parallel = |threads| EncodeSettings {
        threads,
        parallel_tiles: true,
        ..test_settings(70.0, 10)
    };
    let two = encode(1024, 512, false, parallel(2), pattern);
    let four = encode(1024, 512, false, parallel(4), pattern);
    assert_eq!(two, four);
    let single_tile = encode(1024, 512, false, test_settings(70.0, 10), pattern);
    assert_ne!(single_tile, two);
}

#[cfg(feature = "avif-read")]
mod roundtrip {
    use super::encode;
    use crate::avif_sink::{ChromaSubsampling, EncodeSettings, test_settings};
    use crate::codecs::avif::AvifSource;
    use crate::decoder::{ChannelLayout, DecodeSource, DecodedRow};
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;

    fn decode(bytes: &[u8]) -> (ChannelLayout, Vec<f32>) {
        let mut source = AvifSource::new(bytes).unwrap();
        assert_eq!(source.info().rendered_cicp, Some([1, 13, 0, 1]));
        let size = source.info().size;
        let layout = source.info().layout;
        let mut samples = Vec::new();
        source
            .read_region(
                Rect::new(0, 0, size.width, size.height).unwrap(),
                &MemoryBudget::new(1 << 24),
                &mut |_, row| {
                    match row {
                        DecodedRow::U8(row) => samples.extend(row.iter().map(|&v| f32::from(v))),
                        DecodedRow::F32(row) => samples.extend(row.iter().map(|&v| v * 255.0)),
                    }
                    Ok(())
                },
            )
            .unwrap();
        (layout, samples)
    }

    fn assert_close(actual: &[f32], expected: [u8; 4], channels: usize, tolerance: f32) {
        for pixel in actual.chunks_exact(channels) {
            for (channel, &value) in pixel.iter().enumerate() {
                let want = f32::from(expected[channel]);
                assert!(
                    (value - want).abs() <= tolerance,
                    "{pixel:?} vs {expected:?}"
                );
            }
        }
    }

    #[test]
    fn opaque_output_decodes_without_alpha_in_both_chroma_layouts() {
        let color = [200, 40, 90, 255];
        for chroma in [ChromaSubsampling::Yuv444, ChromaSubsampling::Yuv420] {
            // Odd sizes exercise the last chroma column and row of 4:2:0.
            let settings = EncodeSettings {
                chroma,
                ..test_settings(90.0, 10)
            };
            let bytes = encode(7, 5, false, settings, |_, _| color);
            let (layout, samples) = decode(&bytes);
            assert_eq!(layout, ChannelLayout::Rgb);
            assert_eq!(samples.len(), 7 * 5 * 3);
            assert_close(&samples, color, 3, 4.0);
        }
    }

    #[test]
    fn transparent_output_carries_an_alpha_item() {
        // Two flat alpha halves; lossy alpha may ring at the edge between
        // them, so only pixels at least four columns away are checked tightly.
        let bytes = encode(16, 8, true, test_settings(90.0, 10), |x, _| {
            if x < 8 {
                [10, 200, 30, 255]
            } else {
                [10, 200, 30, 128]
            }
        });
        let (layout, samples) = decode(&bytes);
        assert_eq!(layout, ChannelLayout::Rgba);
        for (index, pixel) in samples.as_chunks::<4>().0.iter().enumerate() {
            let x = index % 16;
            let alpha = if x < 8 { 255.0 } else { 128.0 };
            let tolerance = if (4..12).contains(&x) { 16.0 } else { 3.0 };
            assert!((pixel[3] - alpha).abs() <= tolerance, "x {x}: {pixel:?}");
        }
        // A transparent-capable sink with fully opaque rows writes no alpha.
        let opaque = encode(6, 4, true, test_settings(90.0, 10), |_, _| {
            [10, 200, 30, 255]
        });
        assert_eq!(decode(&opaque).0, ChannelLayout::Rgb);
    }
}
