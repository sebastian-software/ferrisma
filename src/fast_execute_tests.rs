//! Differential tests: the `f32` executor against the `f64` oracle, using the
//! acceptance rules of ADR-0021 section 4, plus the contracts both executors
//! share (one decoder request, released memory, cancellation).

use crate::color::{AlphaMode, ColorEncoding};
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::fast_execute::{
    Primaries, execute_fast, execute_fast_srgb8, linearize_row, quantize_row_deep, tables,
};
use crate::geometry::{Rect, Size};
use crate::graph::{Graph, NodeId};
use crate::memory::MemoryBudget;
use crate::output_depth::{OutputRow, max_code};
use crate::quality::{Filter, taps};
use crate::quality_execute::{execute_quality, execute_quality_srgb8};
use crate::working_color::{
    decode_straight_to_premultiplied_linear_srgb, decode_to_linear_srgb, encode_linear_srgb,
    encode_linear_srgb8,
};
use ferrisma_kernels::{IsaLevel, Kernels, Taps};

const SRGB: [u8; 4] = [1, 13, 0, 1];
const P3: [u8; 4] = [12, 13, 0, 1];

#[derive(Clone)]
enum Samples {
    U8(Vec<u8>),
    F32(Vec<f32>),
}

#[derive(Clone)]
struct TestSource {
    info: DecoderInfo,
    samples: Samples,
    calls: usize,
}

impl TestSource {
    fn new(size: Size, layout: ChannelLayout, cicp: [u8; 4], samples: Samples) -> Self {
        let sample = match samples {
            Samples::U8(_) => SampleType::U8,
            Samples::F32(_) => SampleType::F32,
        };
        Self {
            info: DecoderInfo {
                depth: 8,
                size,
                layout,
                sample,
                access: DecodeAccess::CodecRegionRequest,
                alpha: if layout == ChannelLayout::Rgba {
                    AlphaMode::Straight
                } else {
                    AlphaMode::Opaque
                },
                rendered_icc: None,
                rendered_cicp: Some(cicp),
            },
            samples,
            calls: 0,
        }
    }
}

impl DecodeSource for TestSource {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn read_region(
        &mut self,
        region: Rect,
        _budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.calls += 1;
        let channels = usize::from(self.info.layout.channels());
        let stride = self.info.size.width as usize * channels;
        let width = region.width as usize * channels;
        for y in region.y..region.bottom() {
            let start = y as usize * stride + region.x as usize * channels;
            let row = match &self.samples {
                Samples::U8(data) => DecodedRow::U8(&data[start..start + width]),
                Samples::F32(data) => DecodedRow::F32(&data[start..start + width]),
            };
            sink(y, row)?;
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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn byte(&mut self, below: u64) -> u8 {
        u8::try_from(self.next() % below).unwrap()
    }
    fn unit(&mut self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let value = (self.next() >> 40) as f32 / (1_u64 << 24) as f32;
        value
    }
}

/// Whether an input belongs to ADR-0021's full-range set (rule B) or the
/// dark and tie-prone set (rule C).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    FullRange,
    DarkOrTies,
}

struct Case {
    name: &'static str,
    class: Class,
    source: TestSource,
}

const W: u32 = 72;
const H: u32 = 60;

fn rgb_u8(cicp: [u8; 4], pixel: impl FnMut(u32, u32) -> [u8; 3]) -> TestSource {
    let mut pixel = pixel;
    let mut data = Vec::new();
    for y in 0..H {
        for x in 0..W {
            data.extend(pixel(x, y));
        }
    }
    TestSource::new(
        Size::new(W, H).unwrap(),
        ChannelLayout::Rgb,
        cicp,
        Samples::U8(data),
    )
}

#[allow(clippy::too_many_lines)] // One list of named inputs reads best.
fn cases() -> Vec<Case> {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut cases = Vec::new();
    let mut noise = |below: u64, offset: u8| {
        let mut rng = Rng(rng.next() | 1);
        move |_, _| [0; 3].map(|_| offset + rng.byte(below))
    };
    cases.push(Case {
        name: "uniform noise",
        class: Class::FullRange,
        source: rgb_u8(SRGB, noise(256, 0)),
    });
    cases.push(Case {
        name: "bright noise",
        class: Class::FullRange,
        source: rgb_u8(SRGB, noise(64, 192)),
    });
    cases.push(Case {
        name: "dark noise",
        class: Class::DarkOrTies,
        source: rgb_u8(SRGB, noise(25, 0)),
    });
    cases.push(Case {
        name: "P3 noise",
        class: Class::FullRange,
        source: rgb_u8(P3, noise(256, 0)),
    });
    cases.push(Case {
        name: "saturated edges",
        class: Class::FullRange,
        source: rgb_u8(SRGB, |x, y| match ((x / 7) + (y / 5)) % 3 {
            0 => [255, 0, 0],
            1 => [0, 255, 0],
            _ => [0, 0, 255],
        }),
    });
    cases.push(Case {
        name: "gradient",
        class: Class::DarkOrTies,
        source: rgb_u8(SRGB, |x, y| {
            let value = u8::try_from((x * 255 / (W - 1) + y) % 256).unwrap();
            [value, value / 2, 255 - value]
        }),
    });
    cases.push(Case {
        name: "1-pixel checkerboard",
        class: Class::DarkOrTies,
        source: rgb_u8(SRGB, |x, y| [if (x + y) % 2 == 0 { 0 } else { 255 }; 3]),
    });
    cases.push(Case {
        name: "2-pixel checkerboard",
        class: Class::DarkOrTies,
        source: rgb_u8(SRGB, |x, y| {
            [if (x / 2 + y / 2) % 2 == 0 { 30 } else { 220 }; 3]
        }),
    });
    cases.push(Case {
        name: "P3 checkerboard",
        class: Class::DarkOrTies,
        source: rgb_u8(P3, |x, y| {
            if (x + y) % 2 == 0 {
                [255, 0, 0]
            } else {
                [0, 255, 0]
            }
        }),
    });
    cases.push(Case {
        name: "flat blocks",
        class: Class::DarkOrTies,
        source: rgb_u8(SRGB, |x, y| {
            let value = u8::try_from(((x / 32) * 97 + (y / 32) * 61) % 256).unwrap();
            [value, value, value]
        }),
    });

    let mut rgba = |below_alpha: u64, cicp: [u8; 4]| {
        let mut rng = Rng(rng.next() | 1);
        let mut data = Vec::new();
        for _ in 0..W * H {
            data.extend([
                rng.byte(256),
                rng.byte(256),
                rng.byte(256),
                rng.byte(below_alpha),
            ]);
        }
        TestSource::new(
            Size::new(W, H).unwrap(),
            ChannelLayout::Rgba,
            cicp,
            Samples::U8(data),
        )
    };
    cases.push(Case {
        name: "RGBA noise",
        class: Class::FullRange,
        source: rgba(256, SRGB),
    });
    cases.push(Case {
        name: "RGBA low alpha",
        class: Class::FullRange,
        source: rgba(8, SRGB),
    });

    let mut floats = |low: f32, high: f32, cicp: [u8; 4]| {
        let mut rng = Rng(rng.next() | 1);
        let data = (0..W * H * 3)
            .map(|_| low + (high - low) * rng.unit())
            .collect();
        TestSource::new(
            Size::new(W, H).unwrap(),
            ChannelLayout::Rgb,
            cicp,
            Samples::F32(data),
        )
    };
    cases.push(Case {
        name: "F32 noise",
        class: Class::FullRange,
        source: floats(0.0, 1.0, SRGB),
    });
    cases.push(Case {
        name: "F32 extended range",
        class: Class::FullRange,
        source: floats(-0.2, 1.5, SRGB),
    });
    cases.push(Case {
        name: "F32 P3 noise",
        class: Class::FullRange,
        source: floats(0.0, 1.0, P3),
    });
    cases.push(Case {
        name: "F32 dark",
        class: Class::DarkOrTies,
        source: floats(0.0, 0.01, SRGB),
    });
    cases.push(Case {
        name: "F32 linear sRGB",
        class: Class::FullRange,
        source: floats(-0.1, 1.3, [1, 8, 0, 1]),
    });
    cases
}

/// (crop, output width, output height) on the 72x60 inputs.
fn geometries() -> Vec<(Option<Rect>, u32, u32)> {
    vec![
        (None, W, H),
        (None, 108, 90),
        (None, 48, 40),
        (None, 36, 30),
        (None, 30, 25),
        (None, 9, 7),
        (None, 4, 4),
        (None, 2, 2),
        (None, 72, 4),
        (Some(Rect::new(5, 3, 61, 50).unwrap()), 23, 17),
        (Some(Rect::new(1, 7, 13, 11).unwrap()), 29, 31),
    ]
}

fn graph_for(
    source: &TestSource,
    crop: Option<Rect>,
    output: Size,
    filter: Filter,
) -> (Graph, NodeId, NodeId) {
    graph_for_info(&source.info, crop, output, filter)
}

fn graph_for_info(
    info: &DecoderInfo,
    crop: Option<Rect>,
    output: Size,
    filter: Filter,
) -> (Graph, NodeId, NodeId) {
    let mut graph = Graph::new();
    let input = graph
        .decoded_source(info.size, info.pixel_format(), info.semantics())
        .unwrap();
    let cropped = match crop {
        Some(rect) => graph.crop(input, rect).unwrap(),
        None => input,
    };
    let linear = graph.add_linear_srgb(cropped).unwrap();
    let resized = graph.resize_working(linear, output, filter).unwrap();
    let finished = graph.add_srgb8(resized).unwrap();
    (graph, input, finished)
}

fn render(
    source: &TestSource,
    crop: Option<Rect>,
    size: Size,
    filter: Filter,
    fast: Option<&Kernels>,
) -> Vec<u8> {
    let (graph, input, output) = graph_for(source, crop, size, filter);
    let mut source = source.clone();
    let budget = MemoryBudget::new(64 * 1024 * 1024);
    let mut pixels = Vec::new();
    let request = Rect::new(0, 0, size.width, size.height).unwrap();
    let sink = |_: u32, row: &[u8]| {
        pixels.extend_from_slice(row);
        Ok(())
    };
    let report = if let Some(kernels) = fast {
        execute_fast_srgb8(
            &graph,
            output,
            input,
            &mut source,
            request,
            &budget,
            kernels,
            1,
            sink,
        )
    } else {
        execute_quality_srgb8(&graph, output, input, &mut source, request, &budget, sink)
    }
    .unwrap();
    assert_eq!(source.calls, 1);
    assert_eq!(report.decoder_requests, 1);
    assert_eq!(report.explicit_full_frame_intermediates, 0);
    assert_eq!(budget.snapshot().live, 0);
    pixels
}

#[derive(Default)]
struct Tally {
    rgb: u64,
    rgb_exact: u64,
    alpha: u64,
    alpha_exact: u64,
    absolute_error: u64,
}

#[test]
fn f32_executor_meets_the_adr_0021_bound_against_the_f64_oracle() {
    // Every kernel level this host can run (ADR-0020), each against the oracle.
    for kernels in Kernels::available() {
        let level = kernels.level();
        for case in cases() {
            let mut tally = Tally::default();
            for filter in [Filter::Area, Filter::Lanczos3] {
                for (crop, width, height) in geometries() {
                    let size = Size::new(width, height).unwrap();
                    let reference = render(&case.source, crop, size, filter, None);
                    let fast = render(&case.source, crop, size, filter, Some(&kernels));
                    assert_eq!(reference.len(), fast.len());
                    for (index, (expected, actual)) in reference
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .zip(fast.as_chunks::<4>().0)
                        .enumerate()
                    {
                        let context = || {
                            format!(
                                "{level} {} {filter:?} {crop:?} {width}x{height} pixel {index}: oracle {expected:?} fast {actual:?}",
                                case.name
                            )
                        };
                        // Rule A on alpha.
                        assert!(expected[3].abs_diff(actual[3]) <= 1, "{}", context());
                        tally.alpha += 1;
                        tally.alpha_exact += u64::from(expected[3] == actual[3]);
                        // Rule E, per executor: an alpha byte of 0 carries RGB 0.
                        if actual[3] == 0 {
                            assert_eq!(actual[..3], [0, 0, 0], "{}", context());
                        }
                        if expected[3] == 0 || actual[3] == 0 {
                            // At an alpha tie (exactly half a code) one side may
                            // round to 0 and the other to 1; rule A already bounds
                            // alpha, and RGB behind alpha 0 is not compared.
                            continue;
                        }
                        for channel in 0..3 {
                            let error = expected[channel].abs_diff(actual[channel]);
                            // Rule A on RGB.
                            assert!(error <= 1, "{}", context());
                            tally.rgb += 1;
                            tally.rgb_exact += u64::from(error == 0);
                            tally.absolute_error += u64::from(error);
                        }
                    }
                }
            }
            #[allow(clippy::cast_precision_loss)]
            let exact = tally.rgb_exact as f64 / tally.rgb as f64;
            #[allow(clippy::cast_precision_loss)]
            let mean = tally.absolute_error as f64 / tally.rgb as f64;
            eprintln!(
                "{level:<7} {:<22} {:?}: {:.4}% exact RGB, mean {:.5}, alpha {:.4}% exact",
                case.name,
                case.class,
                exact * 100.0,
                mean,
                {
                    #[allow(clippy::cast_precision_loss)]
                    let share = tally.alpha_exact as f64 / tally.alpha as f64;
                    share * 100.0
                }
            );
            // Rules B and C (pooled per input), D.
            let required = match case.class {
                Class::FullRange => 0.999,
                Class::DarkOrTies => 0.995,
            };
            assert!(
                exact >= required,
                "{}: {:.4}% exact RGB, required {:.1}%",
                case.name,
                exact * 100.0,
                required * 100.0
            );
            assert!(mean <= 0.002, "{}: mean absolute error {mean}", case.name);
            if case.source.info.layout == ChannelLayout::Rgba {
                // Rule C'.
                #[allow(clippy::cast_precision_loss)]
                let alpha_exact = tally.alpha_exact as f64 / tally.alpha as f64;
                assert!(alpha_exact >= 0.98, "{}: alpha {alpha_exact}", case.name);
            }
        }
    }
}

#[test]
fn exact_encoder_matches_the_reference_at_every_threshold_and_on_a_dense_sweep() {
    let tables = tables();
    let reference = |value: f32| encode_linear_srgb8(f64::from(value)).unwrap();
    // Every code boundary: the boundary value and its predecessor.
    let one = 1.0_f32.to_bits();
    let mut boundaries = 0;
    let mut bits = 1_u32;
    while bits <= one {
        let value = f32::from_bits(bits);
        let below = f32::from_bits(bits - 1);
        if reference(value) != reference(below) {
            boundaries += 1;
            assert_eq!(tables.encode(value), reference(value));
            assert_eq!(tables.encode(below), reference(below));
        }
        // Exhaustive is 1.07e9 values; a prime stride keeps debug tests fast
        // while touching every exponent. Boundaries are found by bisection
        // inside each stride.
        let next = bits.saturating_add(65_521).min(one + 1);
        if reference(f32::from_bits(next.min(one))) != reference(value) {
            let (mut low, mut high) = (bits, next.min(one));
            while high - low > 1 {
                let middle = low + (high - low) / 2;
                if reference(f32::from_bits(middle)) == reference(value) {
                    low = middle;
                } else {
                    high = middle;
                }
            }
            for probe in [low, high] {
                let value = f32::from_bits(probe);
                assert_eq!(tables.encode(value), reference(value), "{value:e}");
            }
        }
        assert_eq!(tables.encode(value), reference(value), "{value:e}");
        bits = next;
    }
    assert_eq!(tables.encode(0.0), 0);
    assert_eq!(tables.encode(1.0), 255);
    assert!(boundaries <= 255);
}

#[test]
fn fast_executor_uses_less_managed_memory_and_releases_it_on_cancellation() {
    let case = cases().into_iter().next().unwrap();
    let size = Size::new(30, 25).unwrap();
    let (graph, input, output) = graph_for(&case.source, None, size, Filter::Lanczos3);
    let request = Rect::new(0, 0, 30, 25).unwrap();

    let reference_budget = MemoryBudget::new(1 << 20);
    let mut source = case.source.clone();
    execute_quality_srgb8(
        &graph,
        output,
        input,
        &mut source,
        request,
        &reference_budget,
        |_, _| Ok(()),
    )
    .unwrap();
    let fast_budget = MemoryBudget::new(1 << 20);
    let mut source = case.source.clone();
    execute_fast_srgb8(
        &graph,
        output,
        input,
        &mut source,
        request,
        &fast_budget,
        &Kernels::best(),
        1,
        |_, _| Ok(()),
    )
    .unwrap();
    assert!(fast_budget.snapshot().peak < reference_budget.snapshot().peak);

    let mut calls = 0;
    let budget = MemoryBudget::new(1 << 20);
    let mut source = case.source.clone();
    assert_eq!(
        execute_fast_srgb8(
            &graph,
            output,
            input,
            &mut source,
            request,
            &budget,
            &Kernels::best(),
            1,
            |_, _| {
                calls += 1;
                Err(Error::Cancelled)
            }
        ),
        Err(Error::Cancelled)
    );
    assert_eq!(calls, 1);
    assert_eq!(budget.snapshot().live, 0);

    let tiny = MemoryBudget::new(64);
    let mut source = case.source.clone();
    assert!(matches!(
        execute_fast_srgb8(
            &graph,
            output,
            input,
            &mut source,
            request,
            &tiny,
            &Kernels::best(),
            1,
            |_, _| Ok(())
        ),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(tiny.snapshot().live, 0);
}

#[test]
fn fast_executor_rejects_non_finite_float_samples_like_the_reference() {
    // Unknown color encodings are rejected earlier, when the graph adds the
    // linear-sRGB node, so both executors only ever see sRGB or Display P3.
    let not_finite = TestSource::new(
        Size::new(2, 2).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::F32(vec![
            0.5,
            f32::NAN,
            0.5,
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
            0.7,
            0.8,
            0.9,
        ]),
    );
    let size = Size::new(1, 1).unwrap();
    let (graph, input, output) = graph_for(&not_finite, None, size, Filter::Area);
    let request = Rect::new(0, 0, 1, 1).unwrap();
    let budget = MemoryBudget::new(1 << 20);
    for fast in [false, true] {
        let mut source = not_finite.clone();
        let result = if fast {
            execute_fast_srgb8(
                &graph,
                output,
                input,
                &mut source,
                request,
                &budget,
                &Kernels::best(),
                1,
                |_, _| Ok(()),
            )
        } else {
            execute_quality_srgb8(
                &graph,
                output,
                input,
                &mut source,
                request,
                &budget,
                |_, _| Ok(()),
            )
        };
        assert_eq!(result.unwrap_err(), Error::NonFinitePixelSample);
        assert_eq!(budget.snapshot().live, 0);
    }
}

/// Inputs tall enough (more than 64 needed rows) to take the parallel path.
fn tall_sources() -> Vec<(&'static str, TestSource)> {
    const TW: u32 = 80;
    const TH: u32 = 300;
    let mut rng = Rng(0xA076_1D64_78BD_642F);
    let size = Size::new(TW, TH).unwrap();
    let pixels = (TW * TH) as usize;
    let rgb: Vec<u8> = (0..pixels * 3).map(|_| rng.byte(256)).collect();
    let rgba: Vec<u8> = (0..pixels * 4).map(|_| rng.byte(256)).collect();
    let float: Vec<f32> = (0..pixels * 3)
        .map(|_| rng.unit().mul_add(1.4, -0.2))
        .collect();
    vec![
        (
            "RGB noise",
            TestSource::new(size, ChannelLayout::Rgb, SRGB, Samples::U8(rgb.clone())),
        ),
        (
            "P3 noise",
            TestSource::new(size, ChannelLayout::Rgb, P3, Samples::U8(rgb)),
        ),
        (
            "RGBA noise",
            TestSource::new(size, ChannelLayout::Rgba, SRGB, Samples::U8(rgba)),
        ),
        (
            "F32 extended",
            TestSource::new(size, ChannelLayout::Rgb, SRGB, Samples::F32(float)),
        ),
    ]
}

/// Run the f32 executor with `threads`, returning the bytes or the error,
/// and checking that every managed byte was released.
#[allow(clippy::too_many_arguments)] // A test driver over every knob.
fn run_threads(
    source: &TestSource,
    crop: Option<Rect>,
    size: Size,
    filter: Filter,
    kernels: &Kernels,
    threads: usize,
    budget_bytes: usize,
    mut stop_after: Option<usize>,
) -> Result<Vec<u8>, Error> {
    let (graph, input, output) = graph_for(source, crop, size, filter);
    let mut source = source.clone();
    let budget = MemoryBudget::new(budget_bytes);
    let mut pixels = Vec::new();
    let mut expected_y = 0;
    let result = execute_fast_srgb8(
        &graph,
        output,
        input,
        &mut source,
        Rect::new(0, 0, size.width, size.height).unwrap(),
        &budget,
        kernels,
        threads,
        |y, row| {
            assert_eq!(y, expected_y, "rows must reach the sink in order");
            expected_y += 1;
            if let Some(remaining) = stop_after.as_mut() {
                if *remaining == 0 {
                    return Err(Error::Cancelled);
                }
                *remaining -= 1;
            }
            pixels.extend_from_slice(row);
            Ok(())
        },
    );
    assert_eq!(budget.snapshot().live, 0, "managed memory must be released");
    result.map(|report| {
        assert_eq!(source.calls, 1);
        assert_eq!(report.output_rows, u64::from(size.height));
        pixels
    })
}

#[test]
fn parallel_pipeline_is_byte_identical_to_the_sequential_executor() {
    let geometries = [
        (None, 80, 300),
        (None, 120, 450),
        (None, 40, 150),
        (None, 11, 40),
        (None, 80, 3),
        (Some(Rect::new(3, 17, 70, 250).unwrap()), 33, 101),
    ];
    let mut levels = vec![Kernels::best()];
    if Kernels::best().level() != ferrisma_kernels::IsaLevel::Scalar {
        levels.push(Kernels::for_level(ferrisma_kernels::IsaLevel::Scalar).unwrap());
    }
    for (name, source) in tall_sources() {
        for filter in [Filter::Area, Filter::Lanczos3] {
            for (crop, width, height) in geometries {
                let size = Size::new(width, height).unwrap();
                for kernels in &levels {
                    let sequential =
                        run_threads(&source, crop, size, filter, kernels, 1, 64 << 20, None)
                            .unwrap();
                    for threads in [2, 3, 8, 12, 16, 20] {
                        let parallel = run_threads(
                            &source,
                            crop,
                            size,
                            filter,
                            kernels,
                            threads,
                            64 << 20,
                            None,
                        )
                        .unwrap();
                        assert!(
                            sequential == parallel,
                            "{name} {filter:?} {crop:?} {width}x{height} {} threads {threads}",
                            kernels.level()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn parallel_pipeline_cancels_and_fails_like_the_sequential_executor() {
    let sources = tall_sources();
    let size = Size::new(40, 150).unwrap();
    let kernels = Kernels::best();
    // A sink that cancels after 7 rows stops the pipeline and frees memory.
    for (_, source) in &sources {
        assert_eq!(
            run_threads(
                source,
                None,
                size,
                Filter::Lanczos3,
                &kernels,
                4,
                64 << 20,
                Some(7)
            ),
            Err(Error::Cancelled)
        );
    }
    // A budget too small for the worker scratch fails cleanly.
    let (_, source) = &sources[0];
    assert!(matches!(
        run_threads(
            source,
            None,
            size,
            Filter::Lanczos3,
            &kernels,
            4,
            8 << 10,
            None
        ),
        Err(Error::BudgetExceeded { .. })
    ));
    // A non-finite float sample found by a worker reaches the caller.
    let mut broken = sources[3].1.clone();
    if let Samples::F32(values) = &mut broken.samples {
        values[3 * 80 * 120 + 5] = f32::NAN;
    }
    assert_eq!(
        run_threads(
            &broken,
            None,
            size,
            Filter::Area,
            &kernels,
            4,
            64 << 20,
            None
        ),
        Err(Error::NonFinitePixelSample)
    );
}

/// The ICC adapter's conversion, deferred to the executor's workers
/// (ADR-0023), gives the same bytes as converting inside the adapter, for
/// every thread count.
#[cfg(feature = "icc")]
#[test]
fn deferred_icc_conversion_is_byte_identical_to_converting_in_the_adapter() {
    use crate::cms::IccSrgbSource;

    /// Forwards everything except `defer_conversion`, so rows arrive
    /// converted.
    struct Converted<'a>(IccSrgbSource<'a>);
    impl DecodeSource for Converted<'_> {
        fn info(&self) -> &DecoderInfo {
            self.0.info()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.0.read_region(region, budget, sink)
        }
    }

    let (_, mut noise) = tall_sources().into_iter().next().unwrap();
    noise.info.rendered_icc =
        Some(include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec());
    noise.info.rendered_cicp = None;
    if let Samples::U8(values) = &mut noise.samples {
        // Saturated alternating colors require negative and >1 linear
        // channels before the resize filter mixes them.
        for row in values.as_chunks_mut::<{ 80 * 3 }>().0 {
            row[..3].copy_from_slice(&[255, 0, 0]);
            row[3..6].copy_from_slice(&[0, 255, 255]);
        }
    }
    let size = Size::new(40, 150).unwrap();
    let kernels = Kernels::best();
    let run = |source: &mut dyn DecodeSource, threads: usize| -> Vec<u8> {
        let (graph, input, output) = graph_for_info(source.info(), None, size, Filter::Lanczos3);
        let budget = MemoryBudget::new(64 << 20);
        let mut pixels = Vec::new();
        execute_fast_srgb8(
            &graph,
            output,
            input,
            source,
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &budget,
            &kernels,
            threads,
            |_, row| {
                pixels.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(budget.snapshot().live, 0, "managed memory must be released");
        pixels
    };
    let adapter = || IccSrgbSource::new(Box::new(noise.clone())).unwrap();
    let reference = run(&mut Converted(adapter()), 1);
    let quality = {
        let mut source = adapter();
        let (graph, input, output) = graph_for_info(source.info(), None, size, Filter::Lanczos3);
        let budget = MemoryBudget::new(64 << 20);
        let mut pixels = Vec::new();
        execute_quality_srgb8(
            &graph,
            output,
            input,
            &mut source,
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &budget,
            |_, row| {
                pixels.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(budget.snapshot().live, 0);
        pixels
    };
    assert_eq!(reference.len(), quality.len());
    assert!(
        reference
            .iter()
            .zip(&quality)
            .all(|(&fast, &scalar)| fast.abs_diff(scalar) <= 1),
        "ICC f32 output must stay within one code of the f64 executor"
    );
    for threads in [1, 2, 8] {
        assert_eq!(
            run(&mut adapter(), threads),
            reference,
            "deferred, {threads} threads"
        );
        assert_eq!(
            run(&mut Converted(adapter()), threads),
            reference,
            "in the adapter, {threads} threads"
        );
    }
}

/// Issue #219: Gray and `GrayAlpha` ICC conversions (both 8-bit `GrayTable` and
/// float `moxcms` fallback) defer onto worker threads and stay byte-identical
/// across thread counts and against non-deferred conversion.
#[cfg(feature = "icc")]
#[test]
fn deferred_gray_and_gray_alpha_icc_are_byte_identical_across_thread_counts() {
    use crate::cms::IccSrgbSource;

    struct Converted<'a>(IccSrgbSource<'a>);
    impl DecodeSource for Converted<'_> {
        fn info(&self) -> &DecoderInfo {
            self.0.info()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.0.read_region(region, budget, sink)
        }
    }

    let gray_icc = include_bytes!("../tests/fixtures/parity/gray-gamma22.icc").to_vec();
    let in_size = Size::new(80, 160).unwrap();
    let out_size = Size::new(40, 80).unwrap();
    let kernels = Kernels::best();

    let run = |source: &mut dyn DecodeSource, threads: usize| -> Vec<u8> {
        let (graph, input, output) =
            graph_for_info(source.info(), None, out_size, Filter::Lanczos3);
        let budget = MemoryBudget::new(64 << 20);
        let mut pixels = Vec::new();
        execute_fast_srgb8(
            &graph,
            output,
            input,
            source,
            Rect::new(0, 0, out_size.width, out_size.height).unwrap(),
            &budget,
            &kernels,
            threads,
            |_, row| {
                pixels.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(budget.snapshot().live, 0);
        pixels
    };

    for (layout, alpha) in [
        (ChannelLayout::Gray, AlphaMode::Opaque),
        (ChannelLayout::GrayAlpha, AlphaMode::Straight),
    ] {
        let channels = usize::from(layout.channels());
        let u8_samples: Vec<u8> = (0..(80 * 160))
            .flat_map(|i| {
                let g = u8::try_from((i * 37 + i / 80 * 13) % 256).unwrap();
                let a = [0_u8, 1, 128, 254, 255][i % 5];
                [g, a].into_iter().take(channels)
            })
            .collect();
        let f32_samples: Vec<f32> = u8_samples.iter().map(|&v| f32::from(v) / 255.0).collect();

        for (label, samples) in [
            ("u8", Samples::U8(u8_samples)),
            ("f32", Samples::F32(f32_samples)),
        ] {
            let mut raw = TestSource::new(in_size, layout, SRGB, samples);
            raw.info.alpha = alpha;
            raw.info.rendered_icc = Some(gray_icc.clone());
            raw.info.rendered_cicp = None;
            let adapter = || IccSrgbSource::new(Box::new(raw.clone())).unwrap();
            let reference = run(&mut Converted(adapter()), 1);
            for threads in [1, 2, 8] {
                assert_eq!(
                    run(&mut adapter(), threads),
                    reference,
                    "{layout:?} {label} deferred t={threads}"
                );
            }
        }
    }
}

/// Issue #68: `partition_transform_threads` only splits off gather threads when
/// `OrientedSource` actually spawns a gather pool (`has_transposing_orientation`,
/// `source_height >= PARALLEL_MIN_ROWS`, and `used_threads >= 6`); whenever
/// gathering stays serial (`gather == 1`), all `used_threads` remain available
/// to `parallel_execute`.
#[test]
fn partition_transform_threads_keeps_all_workers_when_gather_is_serial() {
    use crate::fast_execute::partition_transform_threads;

    for threads in [1, 2, 3, 4, 5] {
        assert_eq!(
            partition_transform_threads(threads, true, 1000),
            (1, threads),
            "serial gather at t={threads} keeps all {threads} transform workers"
        );
    }
    assert_eq!(partition_transform_threads(6, true, 1000), (2, 4));
    assert_eq!(partition_transform_threads(8, true, 1000), (2, 6));
    assert_eq!(partition_transform_threads(9, true, 1000), (3, 6));
    assert_eq!(partition_transform_threads(20, true, 1000), (4, 16));
    // Short height (< PARALLEL_MIN_ROWS = 64) stays serial in OrientedSource.
    assert_eq!(partition_transform_threads(8, true, 63), (1, 8));
    assert_eq!(partition_transform_threads(8, true, 64), (2, 6));
    // Non-transposing orientation never splits transform workers.
    assert_eq!(partition_transform_threads(8, false, 1000), (1, 8));
    assert_eq!(partition_transform_threads(20, false, 4000), (1, 20));
}

#[test]
fn partial_parallel_worker_start_failure_releases_buffers_and_returns_error() {
    let sources = tall_sources();
    for successful in [0, 1, 3] {
        let _failure = crate::thread_spawn::fail_after(successful);
        assert_eq!(
            run_threads(
                &sources[0].1,
                None,
                Size::new(40, 150).unwrap(),
                Filter::Lanczos3,
                &Kernels::best(),
                4,
                64 << 20,
                None,
            ),
            Err(Error::ThreadSpawnFailed)
        );
    }
}

#[test]
fn wide_downscale_charges_filter_planning_before_decoding() {
    // A 10,000-to-1 area filter needs 10,000 f64 taps; the input is tiny
    // enough for a bounded regression but the tap vector exceeds 64 bytes.
    let size = Size::new(10_000, 1).unwrap();
    let mut source = TestSource::new(size, ChannelLayout::Rgb, SRGB, Samples::U8(vec![0; 30_000]));
    let output = Size::new(1, 1).unwrap();
    let (graph, input, target) = graph_for(&source, None, output, Filter::Area);
    let budget = MemoryBudget::new(64);
    let result = execute_quality_srgb8(
        &graph,
        target,
        input,
        &mut source,
        Rect::new(0, 0, 1, 1).unwrap(),
        &budget,
        |_, _| Ok(()),
    );
    assert!(matches!(result, Err(Error::BudgetExceeded { .. })));
    assert_eq!(source.calls, 0, "rejection must precede decoder work");
    assert_eq!(budget.snapshot().live, 0);
}

#[test]
fn extreme_geometry_is_rejected_before_work_in_both_executors() {
    for (input_size, output_size) in [
        (Size::new(u32::MAX, 1).unwrap(), Size::new(1, 1).unwrap()),
        (Size::new(1, 1).unwrap(), Size::new(u32::MAX, 1).unwrap()),
    ] {
        let source = TestSource::new(
            input_size,
            ChannelLayout::Rgb,
            SRGB,
            Samples::U8(Vec::new()),
        );
        let (graph, input, target) = graph_for(&source, None, output_size, Filter::Lanczos3);
        let request = Rect::new(0, 0, output_size.width, output_size.height).unwrap();
        for threads in [0, 1, 4] {
            let mut source = source.clone();
            let budget = MemoryBudget::new(1024);
            let result = if threads == 0 {
                execute_quality_srgb8(
                    &graph,
                    target,
                    input,
                    &mut source,
                    request,
                    &budget,
                    |_, _| Ok(()),
                )
            } else {
                execute_fast_srgb8(
                    &graph,
                    target,
                    input,
                    &mut source,
                    request,
                    &budget,
                    &Kernels::best(),
                    threads,
                    |_, _| Ok(()),
                )
            };
            assert!(matches!(
                result,
                Err(Error::BudgetExceeded { .. } | Error::GeometryOverflow)
            ));
            assert_eq!(source.calls, 0);
            assert_eq!(budget.snapshot().live, 0);
        }
    }
}

#[test]
fn planning_reservation_is_shared_and_released() {
    use crate::quality_execute::reserve_resize_plan;
    let source = TestSource::new(
        Size::new(20, 10).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(vec![0; 600]),
    );
    let (graph, _, target) = graph_for(&source, None, Size::new(3, 2).unwrap(), Filter::Lanczos3);
    let request = Rect::new(0, 0, 3, 2).unwrap();
    let probe = MemoryBudget::new(usize::MAX);
    let reservation = reserve_resize_plan(&graph, target, request, &probe).unwrap();
    let required = probe.snapshot().live;
    assert!(required >= 20 * std::mem::size_of::<crate::quality::Tap>());
    drop(reservation);
    assert_eq!(probe.snapshot().live, 0);
    let budget = MemoryBudget::new(required);
    let first = reserve_resize_plan(&graph, target, request, &budget).unwrap();
    assert!(matches!(
        reserve_resize_plan(&graph, target, request, &budget),
        Err(Error::BudgetExceeded { .. })
    ));
    drop(first);
    assert_eq!(budget.snapshot().live, 0);
}

// ---------------------------------------------------------------------------
// Output depths above 8 bits (ADR-0021, amendment of 2026-11-15).

/// Render at `depth` bits with either executor (`threads` for the fast one).
fn render_depth(
    source: &TestSource,
    crop: Option<Rect>,
    size: Size,
    filter: Filter,
    fast: Option<(&Kernels, usize)>,
    depth: u8,
) -> Vec<u16> {
    let (graph, input, output) = graph_for(source, crop, size, filter);
    let mut source = source.clone();
    let budget = MemoryBudget::new(64 * 1024 * 1024);
    let mut samples = Vec::new();
    let request = Rect::new(0, 0, size.width, size.height).unwrap();
    let sink = |_: u32, row: OutputRow<'_>| {
        match row {
            OutputRow::U8(bytes) => samples.extend(bytes.iter().map(|&b| u16::from(b))),
            OutputRow::U16(values) => {
                assert!(depth > 8, "an 8-bit plan emits u8 rows");
                samples.extend_from_slice(values);
            }
        }
        Ok(())
    };
    let report = if let Some((kernels, threads)) = fast {
        execute_fast(
            &graph,
            output,
            input,
            &mut source,
            request,
            &budget,
            kernels,
            threads,
            depth,
            sink,
        )
    } else {
        execute_quality(
            &graph,
            output,
            input,
            &mut source,
            request,
            &budget,
            depth,
            sink,
        )
    }
    .unwrap();
    assert_eq!(source.calls, 1);
    assert_eq!(report.decoder_requests, 1);
    assert_eq!(budget.snapshot().live, 0);
    samples
}

/// Deep inputs, as the AVIF and JPEG XL readers deliver them: float
/// samples that are codes of a deeper integer format (AVIF arrives through
/// libavif's 16-bit RGB output, `v / 65535`; JPEG XL as libjxl's floats).
fn deep_cases() -> Vec<Case> {
    let mut rng = Rng(0x5851_F42D_4C95_7F2D);
    let mut cases = Vec::new();
    let mut coded =
        |name, class, bits: u32, alpha: bool, mut code: Box<dyn FnMut(u32, u32, usize) -> u32>| {
            let max = f64::from((1_u32 << bits) - 1);
            let channels = if alpha { 4 } else { 3 };
            let mut data = Vec::new();
            for y in 0..H {
                for x in 0..W {
                    for channel in 0..channels {
                        #[allow(clippy::cast_possible_truncation)]
                        data.push((f64::from(code(x, y, channel)) / max) as f32);
                    }
                }
            }
            cases.push(Case {
                name,
                class,
                source: TestSource::new(
                    Size::new(W, H).unwrap(),
                    if alpha {
                        ChannelLayout::Rgba
                    } else {
                        ChannelLayout::Rgb
                    },
                    SRGB,
                    Samples::F32(data),
                ),
            });
        };
    let mut noise = Rng(rng.next() | 1);
    coded(
        "16-bit noise",
        Class::FullRange,
        16,
        false,
        Box::new(move |_, _, _| u32::try_from(noise.next() % 65_536).unwrap()),
    );
    let mut noise = Rng(rng.next() | 1);
    coded(
        "10-bit noise",
        Class::FullRange,
        10,
        false,
        Box::new(move |_, _, _| u32::try_from(noise.next() % 1024).unwrap()),
    );
    let mut noise = Rng(rng.next() | 1);
    coded(
        "12-bit dark noise",
        Class::DarkOrTies,
        12,
        false,
        Box::new(move |_, _, _| u32::try_from(noise.next() % 100).unwrap()),
    );
    coded(
        "10-bit ramp",
        Class::DarkOrTies,
        10,
        false,
        Box::new(|x, y, channel| ((x + W * y) * 3 + u32::try_from(channel).unwrap()) % 1024),
    );
    let mut noise = Rng(rng.next() | 1);
    coded(
        "16-bit RGBA noise",
        Class::FullRange,
        16,
        true,
        Box::new(move |_, _, _| u32::try_from(noise.next() % 65_536).unwrap()),
    );
    cases
}

/// ADR-0021's bound at every deeper output depth: rule A (at most one code,
/// every sample) and rule E hold as at 8 bits; the shares of exactly equal
/// samples (rules B to D) are stated per depth from this test's measurement
/// (ADR-0021 amendment of 2026-11-15), because one code is a finer step.
#[test]
fn deep_output_meets_the_bound_against_the_f64_oracle() {
    let all: Vec<Case> = cases().into_iter().chain(deep_cases()).collect();
    for kernels in Kernels::available() {
        let level = kernels.level();
        for depth in [10_u8, 12, 16] {
            let mut worst = [1.0_f64; 2];
            let mut worst_mean = 0.0_f64;
            for case in &all {
                let mut tally = Tally::default();
                for filter in [Filter::Area, Filter::Lanczos3] {
                    for (crop, width, height) in geometries() {
                        let size = Size::new(width, height).unwrap();
                        let reference = render_depth(&case.source, crop, size, filter, None, depth);
                        let fast = render_depth(
                            &case.source,
                            crop,
                            size,
                            filter,
                            Some((&kernels, 1)),
                            depth,
                        );
                        assert_eq!(reference.len(), fast.len());
                        for (index, (expected, actual)) in reference
                            .as_chunks::<4>()
                            .0
                            .iter()
                            .zip(fast.as_chunks::<4>().0)
                            .enumerate()
                        {
                            let context = || {
                                format!(
                                    "{level} {depth}-bit {} {filter:?} {crop:?} {width}x{height} pixel {index}: oracle {expected:?} fast {actual:?}",
                                    case.name
                                )
                            };
                            assert!(expected.iter().chain(actual).all(|&v| v <= max_code(depth)));
                            assert!(expected[3].abs_diff(actual[3]) <= 1, "{}", context());
                            tally.alpha += 1;
                            tally.alpha_exact += u64::from(expected[3] == actual[3]);
                            if actual[3] == 0 {
                                assert_eq!(actual[..3], [0, 0, 0], "{}", context());
                            }
                            if expected[3] == 0 || actual[3] == 0 {
                                continue;
                            }
                            for channel in 0..3 {
                                let error = expected[channel].abs_diff(actual[channel]);
                                assert!(error <= 1, "{}", context());
                                tally.rgb += 1;
                                tally.rgb_exact += u64::from(error == 0);
                                tally.absolute_error += u64::from(error);
                            }
                        }
                    }
                }
                #[allow(clippy::cast_precision_loss)]
                let exact = tally.rgb_exact as f64 / tally.rgb as f64;
                #[allow(clippy::cast_precision_loss)]
                let mean = tally.absolute_error as f64 / tally.rgb as f64;
                eprintln!(
                    "{level:<7} {depth:>2}-bit {:<22} {:?}: {:.4}% exact RGB, mean {:.5}",
                    case.name,
                    case.class,
                    exact * 100.0,
                    mean
                );
                let class = usize::from(case.class == Class::DarkOrTies);
                worst[class] = worst[class].min(exact);
                worst_mean = worst_mean.max(mean);
            }
            eprintln!(
                "{level:<7} {depth:>2}-bit worst exact: full range {:.4}%, dark/ties {:.4}%; worst mean {:.5}",
                worst[0] * 100.0,
                worst[1] * 100.0,
                worst_mean
            );
            let (full, dark, mean) = deep_bound(depth);
            assert!(worst[0] >= full, "{depth}-bit full range {}", worst[0]);
            assert!(worst[1] >= dark, "{depth}-bit dark/ties {}", worst[1]);
            assert!(worst_mean <= mean, "{depth}-bit mean {worst_mean}");
        }
    }
}

/// Rules B, C and D at a deeper output depth (ADR-0021 amendment of
/// 2026-11-15): (full-range exact share, dark and tie-prone exact share,
/// largest mean absolute error in codes of that depth), each set below the
/// minimum measured on 2026-11-15 (scalar and NEON identical: 10-bit 99.9974
/// / 99.9854 % and 0.00015; 12-bit 99.9889 / 99.2742 % and 0.00726, the
/// low value on 12-bit codes 0-99, whose linear-segment averages land on
/// exact half codes; 16-bit 99.8929 / 99.8758 % and 0.00124). One code at
/// 16 bits is 1/257 of an 8-bit code, so these shares bound a much smaller
/// error than the 8-bit rules B and C do.
fn deep_bound(depth: u8) -> (f64, f64, f64) {
    match depth {
        10 => (0.9999, 0.9995, 0.001),
        12 => (0.9995, 0.99, 0.01),
        _ => (0.998, 0.998, 0.002),
    }
}

/// At 8 bits the depth-generic executors are the sRGB8 executors, bit for
/// bit (the 8-bit path is unchanged).
#[test]
fn eight_bit_output_of_the_depth_generic_executors_is_the_srgb8_output() {
    for case in cases().into_iter().take(4) {
        for (crop, width, height) in geometries().into_iter().take(4) {
            let size = Size::new(width, height).unwrap();
            let reference = render(&case.source, crop, size, Filter::Lanczos3, None);
            let fast = render(
                &case.source,
                crop,
                size,
                Filter::Lanczos3,
                Some(&Kernels::best()),
            );
            let widen = |bytes: Vec<u8>| bytes.into_iter().map(u16::from).collect::<Vec<_>>();
            assert_eq!(
                render_depth(&case.source, crop, size, Filter::Lanczos3, None, 8),
                widen(reference)
            );
            assert_eq!(
                render_depth(
                    &case.source,
                    crop,
                    size,
                    Filter::Lanczos3,
                    Some((&Kernels::best(), 1)),
                    8
                ),
                widen(fast)
            );
        }
    }
}

/// Every code of a 10-, 12- and 16-bit sRGB input survives an identity
/// transform at its own depth, in both executors: no 8-bit bottleneck (an
/// 8-bit one keeps at most 256 levels) and no padding (a padded 8-bit value
/// would not equal the code).
#[test]
fn every_deep_code_survives_an_identity_transform_at_its_depth() {
    for depth in [10_u8, 12, 16] {
        let max = u32::from(max_code(depth));
        let width = 256_u32;
        let height = (max + 1).div_ceil(width);
        let codes: Vec<u32> = (0..width * height).map(|i| i.min(max)).collect();
        let data: Vec<f32> = codes
            .iter()
            .flat_map(|&code| {
                #[allow(clippy::cast_possible_truncation)]
                let value = (f64::from(code) / f64::from(max)) as f32;
                [value; 3]
            })
            .collect();
        let size = Size::new(width, height).unwrap();
        let source = TestSource::new(size, ChannelLayout::Rgb, SRGB, Samples::F32(data));
        for fast in [
            None,
            Some((&Kernels::best(), 1)),
            Some((&Kernels::best(), 4)),
        ] {
            let out = render_depth(&source, None, size, Filter::Lanczos3, fast, depth);
            for (pixel, &code) in out.as_chunks::<4>().0.iter().zip(&codes) {
                let code = u16::try_from(code).unwrap();
                assert_eq!(
                    pixel,
                    &[code, code, code, max_code(depth)],
                    "{depth}-bit {fast:?}",
                    fast = fast.map(|f| f.1)
                );
            }
        }
    }
}

/// The parallel pipeline emits the same deep rows as the sequential one.
#[test]
fn parallel_pipeline_is_identical_at_deeper_depths() {
    for (name, source) in tall_sources() {
        for depth in [10_u8, 16] {
            let size = Size::new(37, 111).unwrap();
            let one = render_depth(
                &source,
                None,
                size,
                Filter::Lanczos3,
                Some((&Kernels::best(), 1)),
                depth,
            );
            let four = render_depth(
                &source,
                None,
                size,
                Filter::Lanczos3,
                Some((&Kernels::best(), 4)),
                depth,
            );
            assert_eq!(one, four, "{name} {depth}-bit");
        }
    }
}

/// The deep encoder is the 8-bit encoder at `max = 255`, and every depth
/// rounds the same way.
#[test]
fn the_encoder_is_one_function_for_every_depth() {
    for code in 0..=255_u16 {
        let encoded = f64::from(code) / 255.0;
        let linear = crate::working_color::decode_srgb(encoded);
        assert_eq!(encode_linear_srgb(linear, 255).unwrap(), code);
        assert_eq!(u16::from(encode_linear_srgb8(linear).unwrap()), code);
    }
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..100_000 {
        let value = f64::from(rng.unit()) * 1.2 - 0.1;
        assert_eq!(
            u16::from(encode_linear_srgb8(value).unwrap()),
            encode_linear_srgb(value, 255).unwrap()
        );
    }
    assert_eq!(encode_linear_srgb(1.0, 1023).unwrap(), 1023);
    assert_eq!(encode_linear_srgb(2.0, 65_535).unwrap(), 65_535);
    assert_eq!(encode_linear_srgb(-1.0, 4095).unwrap(), 0);
    assert_eq!(
        encode_linear_srgb(f64::NAN, 4095),
        Err(Error::NonFinitePixelSample)
    );
}

// ---------------------------------------------------------------------------
// 1:1 sRGB identity pass-through (#284).

/// Every `(value, alpha)` pair in `0..=255 × 0..=255` through the 1:1 identity
/// pass-through matches the `f64` oracle bit-for-bit at depths 8, 10, 12 and
/// 16, including ADR-0014 transparent black when `alpha == 0`.
#[test]
fn identity_passthrough_u8_rgba_and_rgb_match_f64_oracle_exhaustively() {
    let mut rgba_data = Vec::with_capacity(256 * 256 * 4);
    for a in 0..=255_u8 {
        for v in 0..=255_u8 {
            rgba_data.extend_from_slice(&[v, v.wrapping_mul(3), 255 - v, a]);
        }
    }
    let rgba_size = Size::new(256, 256).unwrap();
    let rgba_source = TestSource::new(rgba_size, ChannelLayout::Rgba, SRGB, Samples::U8(rgba_data));

    let mut opaque_data = Vec::with_capacity(256 * 3);
    for v in 0..=255_u8 {
        opaque_data.extend_from_slice(&[v, 255 - v, v.rotate_left(3)]);
    }
    let opaque_size = Size::new(256, 1).unwrap();
    let opaque_source = TestSource::new(
        opaque_size,
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(opaque_data),
    );

    let kernels = Kernels::best();
    for depth in [8_u8, 10, 12, 16] {
        for (source, size) in [(&rgba_source, rgba_size), (&opaque_source, opaque_size)] {
            let oracle = render_depth(source, None, size, Filter::Lanczos3, None, depth);
            let fast_seq = render_depth(
                source,
                None,
                size,
                Filter::Lanczos3,
                Some((&kernels, 1)),
                depth,
            );
            let fast_par = render_depth(
                source,
                None,
                size,
                Filter::Lanczos3,
                Some((&kernels, 4)),
                depth,
            );
            assert_eq!(fast_seq, oracle, "depth={depth} size={size:?} seq");
            assert_eq!(fast_par, oracle, "depth={depth} size={size:?} par");
        }
    }
}

/// 1:1 identity with a pre-resize crop and a sub-rectangle `requested` region
/// matches the `f64` oracle, delivers exact output `y` coordinates, and
/// allocates only a single output row buffer in the managed budget.
#[test]
fn identity_passthrough_crop_subrect_and_memory_match_contract() {
    let size = Size::new(40, 30).unwrap();
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    let u8_rgb: Vec<u8> = (0..(40 * 30 * 3)).map(|_| rng.byte(256)).collect();
    let u8_rgba: Vec<u8> = (0..(40 * 30 * 4)).map(|_| rng.byte(256)).collect();
    let f32_rgba: Vec<f32> = (0..(40 * 30 * 4))
        .map(|i| {
            if i % 4 == 3 {
                if (i / 4) % 11 == 0 { 0.0 } else { rng.unit() }
            } else {
                rng.unit()
            }
        })
        .collect();
    let crop = Rect::new(5, 7, 23, 19).unwrap();
    let out_size = Size::new(23, 19).unwrap();
    let requested = Rect::new(3, 4, 11, 9).unwrap();
    let kernels = Kernels::best();

    for (name, source) in [
        (
            "u8 rgb",
            TestSource::new(size, ChannelLayout::Rgb, SRGB, Samples::U8(u8_rgb)),
        ),
        (
            "u8 rgba",
            TestSource::new(size, ChannelLayout::Rgba, SRGB, Samples::U8(u8_rgba)),
        ),
        (
            "f32 rgba",
            TestSource::new(size, ChannelLayout::Rgba, SRGB, Samples::F32(f32_rgba)),
        ),
    ] {
        for depth in [8_u8, 16] {
            let (graph, input, target) = graph_for(&source, Some(crop), out_size, Filter::Lanczos3);

            let mut oracle_src = source.clone();
            let oracle_budget = MemoryBudget::new(64 << 20);
            let mut oracle_rows = Vec::new();
            let mut oracle_ys = Vec::new();
            execute_quality(
                &graph,
                target,
                input,
                &mut oracle_src,
                requested,
                &oracle_budget,
                depth,
                |y, row| {
                    oracle_ys.push(y);
                    match row {
                        OutputRow::U8(b) => oracle_rows.extend(b.iter().map(|&v| u16::from(v))),
                        OutputRow::U16(s) => oracle_rows.extend_from_slice(s),
                    }
                    Ok(())
                },
            )
            .unwrap();

            let mut fast_src = source.clone();
            let fast_budget = MemoryBudget::new(64 << 20);
            let mut fast_rows = Vec::new();
            let mut fast_ys = Vec::new();
            let report = execute_fast(
                &graph,
                target,
                input,
                &mut fast_src,
                requested,
                &fast_budget,
                &kernels,
                4,
                depth,
                |y, row| {
                    fast_ys.push(y);
                    match row {
                        OutputRow::U8(b) => fast_rows.extend(b.iter().map(|&v| u16::from(v))),
                        OutputRow::U16(s) => fast_rows.extend_from_slice(s),
                    }
                    Ok(())
                },
            )
            .unwrap();

            assert_eq!(fast_ys, oracle_ys, "{name} depth={depth}");
            assert_eq!(fast_src.calls, 1);
            assert_eq!(report.decoder_requests, 1);
            assert_eq!(report.output_rows, u64::from(requested.height));
            assert_eq!(fast_budget.snapshot().live, 0);
            let expected_row_bytes = (requested.width as usize) * 4 * if depth > 8 { 2 } else { 1 };
            assert_eq!(
                report.peak_engine_managed_bytes, expected_row_bytes,
                "{name} depth={depth}: identity path must only allocate one output row"
            );
            for (expected, actual) in oracle_rows.iter().zip(&fast_rows) {
                assert!(
                    expected.abs_diff(*actual) <= 1,
                    "{name} depth={depth}: oracle={expected} fast={actual}"
                );
            }
            if source.info.sample == SampleType::U8 {
                assert_eq!(fast_rows, oracle_rows, "{name} depth={depth} must be exact");
            }
        }
    }
}

/// Non-finite samples and invalid alpha on the 1:1 identity path fail with the
/// same errors as the reference executor.
#[test]
fn identity_passthrough_validates_float_errors() {
    let kernels = Kernels::best();
    let run_err = |source: &TestSource, depth: u8| -> Error {
        let size = source.info.size;
        let (graph, input, target) = graph_for(source, None, size, Filter::Lanczos3);
        let mut src = source.clone();
        let budget = MemoryBudget::new(1 << 20);
        let err = execute_fast(
            &graph,
            target,
            input,
            &mut src,
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &budget,
            &kernels,
            1,
            depth,
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert_eq!(budget.snapshot().live, 0);
        err
    };

    // Non-finite RGB sample.
    let nan_rgb = TestSource::new(
        Size::new(2, 2).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::F32(vec![
            0.2,
            0.3,
            0.4,
            0.5,
            f32::NAN,
            0.5,
            0.1,
            0.2,
            0.3,
            0.4,
            0.5,
            0.6,
        ]),
    );
    for depth in [8_u8, 16] {
        assert_eq!(run_err(&nan_rgb, depth), Error::NonFinitePixelSample);
    }

    // Invalid alpha takes precedence over non-finite RGB, matching `working_color`.
    for bad_alpha in [-0.01_f32, 1.01, f32::NAN, f32::INFINITY] {
        let bad = TestSource::new(
            Size::new(1, 1).unwrap(),
            ChannelLayout::Rgba,
            SRGB,
            Samples::F32(vec![f32::NAN, 0.0, 0.0, bad_alpha]),
        );
        assert_eq!(
            run_err(&bad, 8),
            Error::InvalidAlphaValue,
            "alpha={bad_alpha}"
        );
    }

    // Zero alpha with non-finite RGB is still rejected.
    let zero_alpha_nan = TestSource::new(
        Size::new(1, 1).unwrap(),
        ChannelLayout::Rgba,
        SRGB,
        Samples::F32(vec![f32::INFINITY, 0.0, 0.0, 0.0]),
    );
    assert_eq!(run_err(&zero_alpha_nan, 16), Error::NonFinitePixelSample);
}

/// Transparent-black clamping, budget admission and sink cancellation on the
/// 1:1 identity path release all managed buffers cleanly.
#[test]
fn identity_passthrough_enforces_clamping_budget_and_cancellation() {
    let kernels = Kernels::best();
    // Tiny positive alpha rounds to 0 at 8 bits (transparent black) and to 7 at 16 bits.
    let tiny_alpha = TestSource::new(
        Size::new(1, 1).unwrap(),
        ChannelLayout::Rgba,
        SRGB,
        Samples::F32(vec![1.5, -0.5, 0.5, 1.0e-4]),
    );
    let out8 = render_depth(
        &tiny_alpha,
        None,
        Size::new(1, 1).unwrap(),
        Filter::Lanczos3,
        Some((&kernels, 1)),
        8,
    );
    assert_eq!(out8, vec![0, 0, 0, 0]);
    let out16 = render_depth(
        &tiny_alpha,
        None,
        Size::new(1, 1).unwrap(),
        Filter::Lanczos3,
        Some((&kernels, 1)),
        16,
    );
    assert_eq!(out16, vec![65_535, 0, 32_768, 7]);

    // Budget smaller than one output row fails before calling the decoder.
    let valid = TestSource::new(
        Size::new(16, 4).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(vec![128; 16 * 4 * 3]),
    );
    let (g, i, t) = graph_for(&valid, None, Size::new(16, 4).unwrap(), Filter::Lanczos3);
    let mut src = valid.clone();
    let tiny_budget = MemoryBudget::new(16);
    assert!(matches!(
        execute_fast(
            &g,
            t,
            i,
            &mut src,
            Rect::new(0, 0, 16, 4).unwrap(),
            &tiny_budget,
            &kernels,
            1,
            8,
            |_, _| Ok(()),
        ),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(src.calls, 0);
    assert_eq!(tiny_budget.snapshot().live, 0);

    // Sink cancellation stops cleanly and releases the output row buffer.
    let mut src = valid;
    let budget = MemoryBudget::new(1 << 20);
    let mut calls = 0;
    assert_eq!(
        execute_fast(
            &g,
            t,
            i,
            &mut src,
            Rect::new(0, 0, 16, 4).unwrap(),
            &budget,
            &kernels,
            1,
            8,
            |_, _| {
                calls += 1;
                if calls == 2 {
                    return Err(Error::Cancelled);
                }
                Ok(())
            },
        ),
        Err(Error::Cancelled)
    );
    assert_eq!(calls, 2);
    assert_eq!(budget.snapshot().live, 0);
}

/// Display P3, Linear sRGB and deferred ICC inputs at 1:1 geometry must NEVER
/// bypass their color transform.
#[test]
fn identity_passthrough_does_not_bypass_p3_or_linear_srgb() {
    let kernels = Kernels::best();
    let size = Size::new(4, 2).unwrap();

    // Display P3 saturated green differs substantially from sRGB green.
    let p3_src = TestSource::new(
        size,
        ChannelLayout::Rgb,
        P3,
        Samples::U8([0, 255, 0, 255, 0, 0, 0, 0, 255, 180, 90, 40].repeat(2)),
    );
    let p3_oracle = render_depth(&p3_src, None, size, Filter::Lanczos3, None, 8);
    let p3_fast = render_depth(
        &p3_src,
        None,
        size,
        Filter::Lanczos3,
        Some((&kernels, 1)),
        8,
    );
    assert_eq!(p3_fast, p3_oracle);
    // P3 red `[255, 0, 0]` maps differently in linear sRGB than raw `[255, 0, 0]`
    // and P3 `[180, 90, 40]` must visibly shift.
    assert_ne!(&p3_fast[12..15], &[180, 90, 40]);

    // Linear sRGB mid-light 0.5 encodes to ~188 in sRGB8, not 128.
    let linear_src = TestSource::new(
        size,
        ChannelLayout::Rgb,
        [1, 8, 0, 1],
        Samples::F32(vec![0.5; 4 * 2 * 3]),
    );
    let lin_oracle = render_depth(&linear_src, None, size, Filter::Lanczos3, None, 8);
    let lin_fast = render_depth(
        &linear_src,
        None,
        size,
        Filter::Lanczos3,
        Some((&kernels, 1)),
        8,
    );
    assert_eq!(lin_fast, lin_oracle);
    assert_eq!(lin_fast[0], 188);
}

#[cfg(feature = "icc")]
#[test]
fn identity_passthrough_does_not_bypass_deferred_icc_at_1_to_1() {
    use crate::cms::IccSrgbSource;
    let size = Size::new(8, 4).unwrap();
    let mut raw = TestSource::new(
        size,
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8([180, 90, 40, 0, 220, 50].repeat(16)),
    );
    raw.info.rendered_icc = Some(include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec());
    raw.info.rendered_cicp = None;

    let kernels = Kernels::best();
    let run_icc = |fast: Option<usize>| -> Vec<u8> {
        let mut source = IccSrgbSource::new(Box::new(raw.clone())).unwrap();
        let (graph, input, target) = graph_for_info(source.info(), None, size, Filter::Lanczos3);
        let budget = MemoryBudget::new(64 << 20);
        let request = Rect::new(0, 0, size.width, size.height).unwrap();
        let mut pixels = Vec::new();
        if let Some(threads) = fast {
            execute_fast_srgb8(
                &graph,
                target,
                input,
                &mut source,
                request,
                &budget,
                &kernels,
                threads,
                |_, row| {
                    pixels.extend_from_slice(row);
                    Ok(())
                },
            )
            .unwrap();
        } else {
            execute_quality_srgb8(
                &graph,
                target,
                input,
                &mut source,
                request,
                &budget,
                |_, row| {
                    pixels.extend_from_slice(row);
                    Ok(())
                },
            )
            .unwrap();
        }
        pixels
    };
    let oracle = run_icc(None);
    let fast1 = run_icc(Some(1));
    let fast4 = run_icc(Some(4));
    assert_eq!(fast1, fast4);
    assert!(fast1.iter().zip(&oracle).all(|(&a, &b)| a.abs_diff(b) <= 1));
    assert_ne!(&fast1[..3], &[180, 90, 40]);
}

/// 1:1 sRGB identity pass-through executes on the decoder callback thread, so
/// `QualityReport::transform_threads` reports `1` even when `threads > 1` and
/// `height >= PARALLEL_MIN_ROWS`, while non-identity and P3 paths report `threads`.
#[test]
fn identity_passthrough_reports_one_transform_thread() {
    let size = Size::new(80, 100).unwrap();
    let kernels = Kernels::best();
    let srgb = TestSource::new(
        size,
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(vec![100; 80 * 100 * 3]),
    );
    let p3 = TestSource::new(
        size,
        ChannelLayout::Rgb,
        P3,
        Samples::U8(vec![100; 80 * 100 * 3]),
    );
    let run_report = |source: &TestSource, out: Size, threads: usize| {
        let (graph, input, target) = graph_for(source, None, out, Filter::Lanczos3);
        let mut src = source.clone();
        let budget = MemoryBudget::new(64 << 20);
        execute_fast(
            &graph,
            target,
            input,
            &mut src,
            Rect::new(0, 0, out.width, out.height).unwrap(),
            &budget,
            &kernels,
            threads,
            8,
            |_, _| Ok(()),
        )
        .unwrap()
    };

    let id = run_report(&srgb, size, 4);
    assert_eq!(id.transform_threads, 1);
    assert!(!id.used_kernels);

    let resized = run_report(&srgb, Size::new(40, 80).unwrap(), 4);
    assert_eq!(resized.transform_threads, 4);
    assert!(resized.used_kernels);

    let p3_id = run_report(&p3, size, 4);
    assert_eq!(p3_id.transform_threads, 4);
    assert!(p3_id.used_kernels);
}

/// An overproducing decoder delivering an extra row must be rejected with
/// `IncompleteDecoderOutput` BEFORE the sink is invoked outside the requested
/// rectangle, so an encoder's out-of-range error cannot mask the decoder defect.
#[test]
fn identity_passthrough_rejects_surplus_decoder_rows_before_calling_sink() {
    struct OverproducingSource {
        info: DecoderInfo,
        row: Vec<u8>,
    }

    impl DecodeSource for OverproducingSource {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }

        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            // Deliver one extra correctly numbered row past `region.bottom()`.
            for y in region.y..=region.bottom() {
                sink(y, DecodedRow::U8(&self.row))?;
            }
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: u64::from(region.height) + 1,
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

    let size = Size::new(8, 4).unwrap();
    let mut source = OverproducingSource {
        info: DecoderInfo {
            depth: 8,
            size,
            layout: ChannelLayout::Rgb,
            sample: SampleType::U8,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Opaque,
            rendered_icc: None,
            rendered_cicp: Some(SRGB),
        },
        row: vec![42; 8 * 3],
    };
    let (graph, input, target) = graph_for_info(&source.info, None, size, Filter::Lanczos3);
    let budget = MemoryBudget::new(1 << 20);
    let mut sink_calls = 0_u32;
    let result = execute_fast(
        &graph,
        target,
        input,
        &mut source,
        Rect::new(0, 0, size.width, size.height).unwrap(),
        &budget,
        &Kernels::best(),
        1,
        8,
        |y, _| {
            if y >= size.height {
                return Err(Error::EncodeFailed);
            }
            sink_calls += 1;
            Ok(())
        },
    );
    assert_eq!(result, Err(Error::IncompleteDecoderOutput));
    assert_eq!(sink_calls, size.height);
    assert_eq!(budget.snapshot().live, 0);
}

/// Reproducible micro-benchmark for issue #284: planning (`footprint` +
/// `TapTable::build` for both axes) on 6000×4000 geometries and 1:1 identity
/// execution on 24 MP 8-bit and 12 MP 16-bit synthetic sources.
///
/// Run on this revision:
/// `cargo test --release -p ferrisma --lib bench_issue_284 -- --ignored --nocapture`
///
/// To reproduce the base (`cef60399`) column in ADR-0021 on the identical
/// workload, check out `cef60399` in a temporary worktree, apply the two-part
/// harness-only patch (change `fn build(` on `TapTable` in `src/fast_execute.rs`
/// to `pub(crate) fn build(` and append this test function to
/// `src/fast_execute_tests.rs`), and run the same command.
#[test]
#[ignore = "manual benchmark for issue #284"]
#[allow(clippy::too_many_lines)] // Self-contained micro-benchmark driver.
fn bench_issue_284_planning_and_identity() {
    use crate::fast_execute::TapTable;
    use crate::quality::footprint;
    use std::hint::black_box;
    use std::time::Instant;

    const WARMUP: usize = 3;
    const ITERS: usize = 15;

    // 1. Planning: exact `footprint` + `TapTable::build` for both axes.
    for (label, iw, ih, ow, oh, filter) in [
        (
            "6000x4000->1600x1067 Lanczos3",
            6000,
            4000,
            1600,
            1067,
            Filter::Lanczos3,
        ),
        (
            "6000x4000->1600x1067 Area",
            6000,
            4000,
            1600,
            1067,
            Filter::Area,
        ),
        (
            "6000x4000->6000x4000 Lanczos3",
            6000,
            4000,
            6000,
            4000,
            Filter::Lanczos3,
        ),
    ] {
        let mut samples = Vec::with_capacity(ITERS);
        for rep in 0..(WARMUP + ITERS) {
            let started = Instant::now();
            let fx = footprint(iw, ow, 0, ow, filter).unwrap();
            let fy = footprint(ih, oh, 0, oh, filter).unwrap();
            let cols = TapTable::build(iw, ow, 0, ow, filter, fx.0).unwrap();
            let rows = TapTable::build(ih, oh, 0, oh, filter, 0).unwrap();
            black_box((fx, fy, cols, rows));
            let elapsed = started.elapsed().as_micros();
            if rep >= WARMUP {
                samples.push(elapsed);
            }
        }
        samples.sort_unstable();
        let median_us = samples[samples.len() / 2];
        eprintln!("plan {label}: median={median_us} us");
    }

    // 2. 1:1 identity execution on 24 MP U8 RGB/RGBA and 12 MP F32 16-bit RGB/RGBA.
    let kernels = Kernels::best();
    let run_id = |label: &str, size: Size, layout: ChannelLayout, samples: Samples, depth: u8| {
        let mut src = TestSource::new(size, layout, SRGB, samples);
        let (graph, input, target) = graph_for(&src, None, size, Filter::Lanczos3);
        let budget = MemoryBudget::new(256 << 20);
        let request = Rect::new(0, 0, size.width, size.height).unwrap();
        let mut checksum = 0_u64;
        let report = execute_fast(
            &graph,
            target,
            input,
            &mut src,
            request,
            &budget,
            &kernels,
            1,
            depth,
            |_, row| {
                match row {
                    OutputRow::U8(b) => {
                        checksum =
                            checksum.wrapping_add(u64::from(b[0]) + u64::from(b[b.len() - 1]));
                    }
                    OutputRow::U16(s) => {
                        checksum =
                            checksum.wrapping_add(u64::from(s[0]) + u64::from(s[s.len() - 1]));
                    }
                }
                Ok(())
            },
        )
        .unwrap();
        black_box(checksum);
        eprintln!(
            "identity {label}: transform_us={} peak_managed_bytes={}",
            report.transform_time.as_micros(),
            report.peak_engine_managed_bytes
        );
    };

    let s24 = Size::new(6000, 4000).unwrap();
    run_id(
        "24MP U8 RGB->U8",
        s24,
        ChannelLayout::Rgb,
        Samples::U8(vec![123; 6000 * 4000 * 3]),
        8,
    );
    run_id(
        "24MP U8 RGBA->U8",
        s24,
        ChannelLayout::Rgba,
        Samples::U8(vec![123; 6000 * 4000 * 4]),
        8,
    );
    let s12 = Size::new(4000, 3000).unwrap();
    run_id(
        "12MP F32 RGB->U16",
        s12,
        ChannelLayout::Rgb,
        Samples::F32(vec![0.5; 4000 * 3000 * 3]),
        16,
    );
    run_id(
        "12MP F32 RGBA->U16",
        s12,
        ChannelLayout::Rgba,
        Samples::F32(vec![0.5; 4000 * 3000 * 4]),
        16,
    );
}

#[test]
fn fast_executor_f32_rejects_invalid_alpha_and_overflowing_extended_samples() {
    let size = Size::new(2, 2).unwrap();
    let out = Size::new(1, 1).unwrap();
    let request = Rect::new(0, 0, 1, 1).unwrap();
    let budget = MemoryBudget::new(1 << 20);
    let kernels = Kernels::best();

    // Out-of-range alpha on F32 RGBA must return InvalidAlphaValue, even if RGB is NaN.
    for alpha in [-0.01_f32, 1.01, f32::NAN, f32::INFINITY] {
        let mut samples = vec![0.5_f32; 2 * 2 * 4];
        samples[0] = f32::NAN;
        samples[3] = alpha;
        let mut source = TestSource::new(size, ChannelLayout::Rgba, SRGB, Samples::F32(samples));
        let (graph, input, target) = graph_for(&source, None, out, Filter::Area);
        let result = execute_fast(
            &graph,
            target,
            input,
            &mut source,
            request,
            &budget,
            &kernels,
            1,
            10,
            |_, _| Ok(()),
        );
        assert_eq!(result.unwrap_err(), Error::InvalidAlphaValue);
        assert_eq!(budget.snapshot().live, 0);
    }

    // Huge finite F32 sample whose x^2.4 overflows f32 to infinity must be rejected
    // when opaque (RGB) or when alpha keeps the premultiplied result out of f32 range (RGBA).
    for cicp in [SRGB, P3] {
        for layout in [ChannelLayout::Rgb, ChannelLayout::Rgba] {
            let channels = usize::from(layout.channels());
            let mut samples = vec![0.5_f32; 2 * 2 * channels];
            samples[0] = 1.0e20;
            if layout == ChannelLayout::Rgba {
                samples[3] = 1.0;
            }
            let mut source = TestSource::new(size, layout, cicp, Samples::F32(samples));
            let (graph, input, target) = graph_for(&source, None, out, Filter::Area);
            let result = execute_fast(
                &graph,
                target,
                input,
                &mut source,
                request,
                &budget,
                &kernels,
                1,
                16,
                |_, _| Ok(()),
            );
            assert_eq!(result.unwrap_err(), Error::NonFinitePixelSample);
            assert_eq!(budget.snapshot().live, 0);
        }
    }
}

/// Finding 1 regression: extended-range Display P3 pixels must stay in `f64`
/// through the exact two-step P3 -> XYZ -> linear sRGB reference matrix before
/// narrowing to `f32`, and mixed-range P3 pixels must decode all channels with
/// the exact EOTF so large canceling terms (such as `[12.0, 3.1096375, 0.0]`,
/// `[0.0, 0.0, 65504.0]`, and `[0.32031983, 0.6952312, 99416.45]`) match the
/// `f64` oracle in both opaque RGB and straight-alpha RGBA paths.
#[test]
fn extended_range_p3_matches_f64_oracle_in_rgb_and_rgba() {
    let pixels_rgb: &[[f32; 3]] = &[
        [12.0, 3.109_637_5, 0.0],
        [0.0, 0.0, 65_504.0],
        [0.320_319_83, 0.695_231_2, 99_416.45],
        [-12.0, -3.109_637_5, 0.0],
        [4.0, 1.25, -0.5],
        [-2.0, 3.5, 1.5],
        [0.5, 0.25, 0.75],
    ];
    // Span > 64 pixels per row (multi-chunk SIMD) and >= 64 rows (PARALLEL_MIN_ROWS)
    // so `threads = 4` genuinely exercises the parallel worker executor.
    let width = 70_u32;
    let height = 64_u32;
    let size = Size::new(width, height).unwrap();
    let mut opaque_data = Vec::with_capacity((width * height * 3) as usize);
    let mut alpha_data = Vec::with_capacity((width * height * 4) as usize);
    let alphas = [1.0_f32, 1.0, 1.0, 0.5, 0.25, 0.0, 0.75];
    for i in 0..(width * height) as usize {
        let p = pixels_rgb[i % pixels_rgb.len()];
        opaque_data.extend_from_slice(&p);
        let alpha = alphas[i % alphas.len()];
        alpha_data.extend_from_slice(&[p[0], p[1], p[2], alpha]);
    }

    for (layout, data) in [
        (ChannelLayout::Rgb, opaque_data),
        (ChannelLayout::Rgba, alpha_data),
    ] {
        let source = TestSource::new(size, layout, P3, Samples::F32(data));
        let reference = render_depth(&source, None, size, Filter::Lanczos3, None, 16);
        // Pixel 0 is `[12.0, 3.1096375, 0.0]` (opaque or alpha 1.0): green code must be 847.
        assert_eq!(reference[1], 847, "{layout:?} oracle green code");
        for kernels in Kernels::available() {
            for threads in [1, 4] {
                let fast = render_depth(
                    &source,
                    None,
                    size,
                    Filter::Lanczos3,
                    Some((&kernels, threads)),
                    16,
                );
                assert_eq!(
                    fast,
                    reference,
                    "{layout:?} {} threads={threads} 16-bit identity",
                    kernels.level()
                );
            }
        }
        // Also verify a resampled output stays within ADR-0021's 1-code maximum.
        let resized = Size::new(35, 32).unwrap();
        let ref_resized = render_depth(&source, None, resized, Filter::Area, None, 16);
        let fast_resized = render_depth(
            &source,
            None,
            resized,
            Filter::Area,
            Some((&Kernels::best(), 4)),
            16,
        );
        for (idx, (&r, &f)) in ref_resized.iter().zip(&fast_resized).enumerate() {
            assert!(
                r.abs_diff(f) <= 1,
                "{layout:?} resized index {idx}: ref={r} fast={f}"
            );
        }
    }
}

/// Regression for PR #292 finding 1 (P1): decoded float RGB must stay in `f64`
/// through alpha premultiplication before narrowing once to `f32`, so canceling
/// straight-alpha pixels such as `[100, 0, 0, 0.2]` and
/// `[-150, 0, 0, 0.07561396]` in a 2x1 -> 1x1 Area resize emit red code 0,
/// matching the `f64` oracle instead of drifting to 2982 from double rounding.
#[test]
fn rgba_f32_premultiplies_in_f64_before_narrowing() {
    let row = [100.0_f32, 0.0, 0.0, 0.2, -150.0, 0.0, 0.0, 0.075_613_96];
    // 2x1 -> 1x1 sequential case and 2x64 -> 1x64 parallel case.
    for (height, threads) in [(1_u32, 1_usize), (64, 4)] {
        let mut data = Vec::with_capacity(row.len() * height as usize);
        for _ in 0..height {
            data.extend_from_slice(&row);
        }
        let source = TestSource::new(
            Size::new(2, height).unwrap(),
            ChannelLayout::Rgba,
            SRGB,
            Samples::F32(data),
        );
        let out_size = Size::new(1, height).unwrap();
        let reference = render_depth(&source, None, out_size, Filter::Area, None, 16);
        assert_eq!(reference[0], 0, "oracle red code must be 0");
        for kernels in Kernels::available() {
            let fast = render_depth(
                &source,
                None,
                out_size,
                Filter::Area,
                Some((&kernels, threads)),
                16,
            );
            assert_eq!(
                fast,
                reference,
                "{} height={height} threads={threads} premultiply cancellation",
                kernels.level()
            );
        }
    }
}

fn reference_linear_srgb_f32<const C: usize>(row: &[f32]) -> Vec<f32> {
    let srgb = ColorEncoding::Cicp(SRGB);
    let mut out = Vec::with_capacity(row.len() / C * 4);
    for px in row.as_chunks::<C>().0 {
        if C == 3 {
            let lin = decode_to_linear_srgb(
                &srgb,
                [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
            )
            .unwrap();
            #[allow(clippy::cast_possible_truncation)]
            out.extend_from_slice(&[lin.rgb[0] as f32, lin.rgb[1] as f32, lin.rgb[2] as f32, 0.0]);
        } else {
            let premul = decode_straight_to_premultiplied_linear_srgb(
                &srgb,
                [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
                f64::from(px[3]),
            )
            .unwrap();
            #[allow(clippy::cast_possible_truncation)]
            out.extend_from_slice(&[
                premul.rgb[0] as f32,
                premul.rgb[1] as f32,
                premul.rgb[2] as f32,
                px[3],
            ]);
        }
    }
    out
}

fn assert_linearize_srgb_bit_exact<const C: usize>(row: &[f32], kernels: &Kernels) -> Vec<f32> {
    let expected = reference_linear_srgb_f32::<C>(row);
    let mut actual = vec![0.0_f32; expected.len()];
    let zero_p3 = [[0.0_f32; 3]; 3];
    linearize_row::<C>(
        DecodedRow::F32(row),
        Primaries::Srgb,
        &zero_p3,
        kernels,
        &mut actual,
    )
    .unwrap();
    for (idx, (&got, &want)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(
            got.to_bits(),
            want.to_bits(),
            "{} linearize_row::<{C}> sample {idx} diverged from working_color",
            kernels.level()
        );
    }
    expected
}

/// Regression for PR #292 round-4 finding 2: when a polynomial-decoded
/// premultiplied value lies near an `f32` rounding boundary, division by a
/// small nonzero filtered alpha can amplify a 1-ULP rounding flip past
/// ADR-0021's 1-code limit. Refining with exact `decode_srgb` whenever the
/// error interval straddles an `f32` boundary keeps `linearize_row`
/// bit-identical to `working_color` on every ISA level, keeps `2x1 -> 4x1`
/// Lanczos3 exact (`26639`) on 1-slot levels (`Scalar`, `Neon`), and matches
/// each SIMD level's own `horizontal_rgba` + `quantize_row_deep` fed the exact
/// reference working pixels (isolating transfer accuracy from pre-existing
/// AVX2/AVX-512 tap reassociation).
#[test]
fn straight_alpha_lanczos3_refines_premultiplied_f32_boundaries() {
    let row = [
        0.406_489_43_f32,
        0.0,
        0.0,
        0.149_299_04,
        0.406_489_43,
        0.0,
        0.0,
        1.0,
    ];
    let mut starts = Vec::new();
    let mut offsets = vec![0_u32];
    let mut weights = Vec::new();
    for x in 0..4_u32 {
        let run = taps(2, 4, x, Filter::Lanczos3).unwrap();
        let first = run[0].index;
        starts.push(first);
        let mut next = first;
        for tap in run {
            while next < tap.index {
                weights.push(0.0);
                next += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            weights.push(tap.weight as f32);
            next = tap.index + 1;
        }
        offsets.push(u32::try_from(weights.len()).unwrap());
    }
    let col_taps = Taps::new(2, starts, offsets, weights).unwrap();

    let source = TestSource::new(
        Size::new(2, 1).unwrap(),
        ChannelLayout::Rgba,
        SRGB,
        Samples::F32(row.to_vec()),
    );
    let out_size = Size::new(4, 1).unwrap();
    let reference = render_depth(&source, None, out_size, Filter::Lanczos3, None, 16);
    assert_eq!(reference[0], 26_639, "oracle first-pixel red code");
    for kernels in Kernels::available() {
        let ref_linear = assert_linearize_srgb_bit_exact::<4>(&row, &kernels);
        let mut expected_acc = [0.0_f32; 16];
        kernels
            .horizontal_rgba(&ref_linear, &col_taps, &mut expected_acc)
            .unwrap();
        let mut expected_u16 = [0_u16; 16];
        quantize_row_deep::<4>(&expected_acc, &mut expected_u16, 16, &kernels).unwrap();

        let fast = render_depth(
            &source,
            None,
            out_size,
            Filter::Lanczos3,
            Some((&kernels, 1)),
            16,
        );
        assert_eq!(
            fast.as_slice(),
            expected_u16.as_slice(),
            "{} 2x1->4x1 Lanczos3 diverged from exact working_color inputs",
            kernels.level()
        );
        if matches!(kernels.level(), IsaLevel::Scalar | IsaLevel::Neon) {
            assert_eq!(
                fast,
                reference,
                "{} 2x1->4x1 Lanczos3 straight-alpha boundary",
                kernels.level()
            );
        }
    }
}

/// Regression for PR #292 round-5 finding 1 (P1): opaque F32 sRGB must also
/// refine samples near an `f32` rounding boundary before narrowing, because a
/// 1-ULP decode flip can cross a larger accumulator rounding boundary during
/// canceling filters (here `4x1 -> 1x1` Area of `[20, s, -20, -s]` where
/// `s = f32::from_bits(0x3ec56ad4)` must emit red code `0`, not `13`).
/// Also exercises extended-range sRGB boundary value `0x3f800294` in RGBA.
#[test]
fn opaque_srgb_refines_f32_boundaries_and_extended_rgba_matches_oracle() {
    let s = f32::from_bits(0x3ec5_6ad4);
    let row = [
        20.0_f32, 0.0, 0.0, s, 0.0, 0.0, -20.0, 0.0, 0.0, -s, 0.0, 0.0,
    ];
    let source = TestSource::new(
        Size::new(4, 1).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::F32(row.to_vec()),
    );
    let out_size = Size::new(1, 1).unwrap();
    let reference = render_depth(&source, None, out_size, Filter::Area, None, 16);
    assert_eq!(reference[0], 0, "oracle red code must be 0");
    for kernels in Kernels::available() {
        assert_linearize_srgb_bit_exact::<3>(&row, &kernels);
        let fast = render_depth(
            &source,
            None,
            out_size,
            Filter::Area,
            Some((&kernels, 1)),
            16,
        );
        assert_eq!(
            fast,
            reference,
            "{} 4x1->1x1 opaque sRGB cancellation",
            kernels.level()
        );
    }

    // Extended-range sRGB sample near an f32 rounding boundary (`0x3f800294`)
    // in straight-alpha RGBA must reuse the already-exact `powf` decode and
    // match `working_color` and the oracle bit for bit. Resize `4x1 -> 2x1`
    // and assert `report.used_kernels` so #284's 1:1 sRGB pass-through cannot
    // bypass the transfer kernels.
    let ext = f32::from_bits(0x3f80_0294);
    let rgba_row = [
        ext, 0.5, 0.25, 1.0, ext, 0.5, 0.25, 0.5, ext, 0.25, 0.5, 0.75, ext, 0.5, 0.25, 0.25,
    ];
    let rgba_in = Size::new(4, 1).unwrap();
    let rgba_out = Size::new(2, 1).unwrap();
    let rgba_rect = Rect::new(0, 0, 2, 1).unwrap();
    let rgba_source = TestSource::new(
        rgba_in,
        ChannelLayout::Rgba,
        SRGB,
        Samples::F32(rgba_row.to_vec()),
    );
    let (rgba_graph, rgba_input, rgba_target) =
        graph_for(&rgba_source, None, rgba_out, Filter::Area);
    let rgba_ref = render_depth(&rgba_source, None, rgba_out, Filter::Area, None, 16);
    for kernels in Kernels::available() {
        assert_linearize_srgb_bit_exact::<4>(&rgba_row, &kernels);
        let mut src = rgba_source.clone();
        let budget = MemoryBudget::new(1 << 20);
        let report = execute_fast(
            &rgba_graph,
            rgba_target,
            rgba_input,
            &mut src,
            rgba_rect,
            &budget,
            &kernels,
            1,
            16,
            |_, _| Ok(()),
        )
        .unwrap();
        assert!(
            report.used_kernels,
            "{} extended RGBA 0x3f800294 must exercise transfer kernels",
            kernels.level()
        );
        let fast = render_depth(
            &rgba_source,
            None,
            rgba_out,
            Filter::Area,
            Some((&kernels, 1)),
            16,
        );
        assert_eq!(
            fast,
            rgba_ref,
            "{} extended RGBA 0x3f800294",
            kernels.level()
        );
    }
}

/// Regressions for PR #292 round-7 findings:
/// 1. Transparent Display P3 pixel whose matrix cancellation flips the sign of
///    green (`[0, 0, 0x3a0c008c, 0]`): multiplying by zero alpha must preserve
///    the exact `-0.0` sign bit of `working_color` across every ISA level.
/// 2. Finite straight-alpha working pixel whose unpremultiplication `c / alpha`
///    overflows `f32` to `±inf` must saturate to `0` / `max` in
///    `quantize_row_deep` rather than failing `KernelError::NonFiniteSample`.
#[test]
fn transparent_p3_signed_zero_and_deep_unpremul_overflow_match_reference() {
    let p3_zero = [0.0_f32, 0.0, f32::from_bits(0x3a0c_008c), 0.0];
    let p3_color = ColorEncoding::Cicp(P3);
    let premul = decode_straight_to_premultiplied_linear_srgb(
        &p3_color,
        [
            f64::from(p3_zero[0]),
            f64::from(p3_zero[1]),
            f64::from(p3_zero[2]),
        ],
        f64::from(p3_zero[3]),
    )
    .unwrap();
    #[allow(clippy::cast_possible_truncation)]
    let expected_p3_zero = [
        premul.rgb[0] as f32,
        premul.rgb[1] as f32,
        premul.rgb[2] as f32,
        p3_zero[3],
    ];
    let zero_p3 = [[0.0_f32; 3]; 3];
    for kernels in Kernels::available() {
        let mut actual = [0.0_f32; 4];
        linearize_row::<4>(
            DecodedRow::F32(&p3_zero),
            Primaries::DisplayP3,
            &zero_p3,
            &kernels,
            &mut actual,
        )
        .unwrap();
        for (ch, (&got, &want)) in actual.iter().zip(&expected_p3_zero).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} transparent P3 channel {ch} signed-zero mismatch",
                kernels.level()
            );
        }

        let acc = [f32::MAX, -f32::MAX, 0.5_f32, 0.001_f32];
        let mut out = [0_u16; 4];
        quantize_row_deep::<4>(&acc, &mut out, 16, &kernels).unwrap();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let expected_alpha = (0.001_f32 * 65_535.0).round() as u16;
        assert_eq!(
            out,
            [65_535, 0, 65_535, expected_alpha],
            "{} quantize_row_deep unpremul overflow saturation",
            kernels.level()
        );
    }
}

fn assert_rgba_overflow_case(
    name: &str,
    pixel: [f32; 4],
    cicp: [u8; 4],
    primaries: Primaries,
    size: Size,
    resized: Size,
) {
    let color = ColorEncoding::Cicp(cicp);
    let premul = decode_straight_to_premultiplied_linear_srgb(
        &color,
        [
            f64::from(pixel[0]),
            f64::from(pixel[1]),
            f64::from(pixel[2]),
        ],
        f64::from(pixel[3]),
    )
    .unwrap();
    #[allow(clippy::cast_possible_truncation)]
    let expected_linear = [
        premul.rgb[0] as f32,
        premul.rgb[1] as f32,
        premul.rgb[2] as f32,
        pixel[3],
    ];
    let mut data = Vec::with_capacity(4 * 2 * 4);
    for i in 0..8 {
        let px = if i % 2 == 0 {
            pixel
        } else {
            [0.25, 0.5, 0.75, 1.0]
        };
        data.extend_from_slice(&px);
    }
    let source = TestSource::new(size, ChannelLayout::Rgba, cicp, Samples::F32(data));
    let (graph, input, target) = graph_for(&source, None, resized, Filter::Area);
    let resized_rect = Rect::new(0, 0, resized.width, resized.height).unwrap();
    let zero_p3 = [[0.0_f32; 3]; 3];
    for kernels in Kernels::available() {
        let mut actual_linear = [0.0_f32; 4];
        linearize_row::<4>(
            DecodedRow::F32(&pixel),
            primaries,
            &zero_p3,
            &kernels,
            &mut actual_linear,
        )
        .unwrap();
        for (ch, (&got, &want)) in actual_linear.iter().zip(&expected_linear).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{name} cicp={cicp:?} {} linearize_row ch={ch}",
                kernels.level()
            );
        }
        let mut src = source.clone();
        let budget = MemoryBudget::new(1 << 20);
        let report = execute_fast(
            &graph,
            target,
            input,
            &mut src,
            resized_rect,
            &budget,
            &kernels,
            1,
            16,
            |_, _| Ok(()),
        )
        .unwrap();
        assert!(
            report.used_kernels,
            "{name} cicp={cicp:?} {} must exercise linearize_row",
            kernels.level()
        );
    }
    for depth in [8_u8, 16] {
        let reference = render_depth(&source, None, resized, Filter::Area, None, depth);
        for kernels in Kernels::available() {
            let fast = render_depth(
                &source,
                None,
                resized,
                Filter::Area,
                Some((&kernels, 1)),
                depth,
            );
            assert_eq!(
                fast,
                reference,
                "{name} cicp={cicp:?} depth={depth} {}",
                kernels.level()
            );
        }
    }
}

/// Finding 2 regression: straight-alpha RGBA `F32` pixels whose pre-alpha EOTF
/// overflows `f32` (such as `[1e20, 0, 0, 0]`, `[1e20, 0, 0, 1e-20]`, or
/// `[2e16, 0, 0, 0.1]`) must be recomputed in `f64` through premultiplication
/// and only rejected if the resulting working pixel is non-finite in `f32`.
/// Uses a non-identity resize (`4x2 -> 2x2`, asserting `report.used_kernels`)
/// plus direct `linearize_row::<4>` checks so #284's 1:1 sRGB pass-through
/// cannot bypass `linearize_f32_rgba_row`.
#[test]
fn rgba_f32_overflow_is_checked_after_premultiplication() {
    let size = Size::new(4, 2).unwrap();
    let resized = Size::new(2, 2).unwrap();
    let resized_rect = Rect::new(0, 0, 2, 2).unwrap();
    let cases: &[(&str, [f32; 4])] = &[
        (
            "zero alpha with overflowing RGB",
            [1.0e20, -1.0e20, 1.0e20, 0.0],
        ),
        (
            "tiny alpha rescuing overflowing RGB",
            [1.0e20, 0.0, 0.0, 1.0e-20],
        ),
        (
            "ultra-tiny alpha rescuing all-channel overflow",
            [1.0e20, 1.0e20, 1.0e20, 1.0e-25],
        ),
        (
            "visible alpha rescuing f32 EOTF overflow",
            [2.0e16, 0.0, 0.0, 0.1],
        ),
    ];
    for (cicp, primaries) in [(SRGB, Primaries::Srgb), (P3, Primaries::DisplayP3)] {
        for &(name, pixel) in cases {
            assert_rgba_overflow_case(name, pixel, cicp, primaries, size, resized);
        }
        // Invalid alpha still takes precedence over overflowing finite RGB.
        let mut samples = vec![0.5_f32; 4 * 2 * 4];
        samples[0] = 1.0e20;
        samples[3] = 1.5;
        let mut source = TestSource::new(size, ChannelLayout::Rgba, cicp, Samples::F32(samples));
        let (graph, input, target) = graph_for(&source, None, resized, Filter::Area);
        let budget = MemoryBudget::new(1 << 20);
        let err = execute_fast(
            &graph,
            target,
            input,
            &mut source,
            resized_rect,
            &budget,
            &Kernels::best(),
            1,
            16,
            |_, _| Ok(()),
        )
        .unwrap_err();
        assert_eq!(err, Error::InvalidAlphaValue);
        assert_eq!(budget.snapshot().live, 0);
    }
}

/// Regressions for PR #292 round-11 findings 1 & 2:
/// - All signed-zero combinations `[±0.0, ±0.0, ±0.0]` (opaque and with
///   `alpha in [0.0, -0.0, 0.5, 1.0]`) in both sRGB and Display P3 must match
///   `working_color` bit-for-bit on the fast path without fallback.
/// - Straight-alpha negative sRGB (`[-0.6, -0.52, -0.3, 0.0]`, `-0.0` products)
///   and tiny-alpha subnormal products (`[0.5, -0.5, 0.25, 1e-38]`) must match
///   `working_color` bit-for-bit across every ISA level.
#[test]
fn signed_zero_srgb_and_p3_match_reference_bit_for_bit() {
    let zero_p3 = [[0.0_f32; 3]; 3];
    let zeros = [0.0_f32, -0.0_f32];
    let mut opaque_pixels = Vec::new();
    let mut alpha_pixels = Vec::new();
    for &r in &zeros {
        for &g in &zeros {
            for &b in &zeros {
                opaque_pixels.extend_from_slice(&[r, g, b]);
                for &a in &[0.0_f32, -0.0_f32, 0.5_f32, 1.0_f32] {
                    alpha_pixels.extend_from_slice(&[r, g, b, a]);
                }
            }
        }
    }
    for extra in [
        [-0.6_f32, -0.52, -0.3, 0.0],
        [-0.6, -0.52, -0.3, -0.0],
        [-0.6, -0.52, -0.3, 0.5],
        [0.5, -0.5, 0.25, 1.0e-38],
    ] {
        alpha_pixels.extend_from_slice(&extra);
    }

    for (cicp, primaries) in [(SRGB, Primaries::Srgb), (P3, Primaries::DisplayP3)] {
        let color = ColorEncoding::Cicp(cicp);
        let mut expected_opaque = Vec::with_capacity(opaque_pixels.len() / 3 * 4);
        for px in opaque_pixels.as_chunks::<3>().0 {
            let lin = decode_to_linear_srgb(
                &color,
                [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
            )
            .unwrap();
            #[allow(clippy::cast_possible_truncation)]
            expected_opaque.extend_from_slice(&[
                lin.rgb[0] as f32,
                lin.rgb[1] as f32,
                lin.rgb[2] as f32,
                0.0,
            ]);
        }
        let mut expected_alpha = Vec::with_capacity(alpha_pixels.len());
        for px in alpha_pixels.as_chunks::<4>().0 {
            let lin = decode_straight_to_premultiplied_linear_srgb(
                &color,
                [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
                f64::from(px[3]),
            )
            .unwrap();
            #[allow(clippy::cast_possible_truncation)]
            expected_alpha.extend_from_slice(&[
                lin.rgb[0] as f32,
                lin.rgb[1] as f32,
                lin.rgb[2] as f32,
                px[3],
            ]);
        }
        for kernels in Kernels::available() {
            let mut got_opaque = vec![0.0_f32; expected_opaque.len()];
            linearize_row::<3>(
                DecodedRow::F32(&opaque_pixels),
                primaries,
                &zero_p3,
                &kernels,
                &mut got_opaque,
            )
            .unwrap();
            for (i, (&g, &w)) in got_opaque.iter().zip(&expected_opaque).enumerate() {
                assert_eq!(
                    g.to_bits(),
                    w.to_bits(),
                    "{cicp:?} {} RGB i={i}",
                    kernels.level()
                );
            }
            let mut got_alpha = vec![0.0_f32; expected_alpha.len()];
            linearize_row::<4>(
                DecodedRow::F32(&alpha_pixels),
                primaries,
                &zero_p3,
                &kernels,
                &mut got_alpha,
            )
            .unwrap();
            for (i, (&g, &w)) in got_alpha.iter().zip(&expected_alpha).enumerate() {
                assert_eq!(
                    g.to_bits(),
                    w.to_bits(),
                    "{cicp:?} {} RGBA i={i}",
                    kernels.level()
                );
            }
        }
    }
}

/// Regression for PR #292 round-9 finding 2: mixed-range and in-range Display
/// P3 pixels sharing a 64-pixel chunk must produce bit-identical working
/// samples to `working_color` for both RGB and RGBA across every ISA level.
#[test]
fn mixed_p3_chunk_matches_reference() {
    const N: usize = 70;
    let p3_color = ColorEncoding::Cicp(P3);
    let mut opaque_row = Vec::with_capacity(N * 3);
    let mut alpha_row = Vec::with_capacity(N * 4);
    let mut expected_opaque = Vec::with_capacity(N * 4);
    let mut expected_alpha = Vec::with_capacity(N * 4);
    let patterns: [[f32; 4]; 5] = [
        [0.2, 0.4, 0.6, 0.5],
        [1.1, 0.5, 0.6, 0.75],
        [0.0, 0.0, f32::from_bits(0x3a0c_008c), 0.0],
        [0.320_319_83, 0.695_231_2, 99_416.45, 1.0],
        [1.5, 2.0, 1.2, 0.25],
    ];
    for i in 0..N {
        let px = patterns[i % patterns.len()];
        opaque_row.extend_from_slice(&px[..3]);
        alpha_row.extend_from_slice(&px);
        let lin_opaque = decode_to_linear_srgb(
            &p3_color,
            [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
        )
        .unwrap();
        #[allow(clippy::cast_possible_truncation)]
        expected_opaque.extend_from_slice(&[
            lin_opaque.rgb[0] as f32,
            lin_opaque.rgb[1] as f32,
            lin_opaque.rgb[2] as f32,
            0.0,
        ]);
        let lin_alpha = decode_straight_to_premultiplied_linear_srgb(
            &p3_color,
            [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
            f64::from(px[3]),
        )
        .unwrap();
        #[allow(clippy::cast_possible_truncation)]
        expected_alpha.extend_from_slice(&[
            lin_alpha.rgb[0] as f32,
            lin_alpha.rgb[1] as f32,
            lin_alpha.rgb[2] as f32,
            px[3],
        ]);
    }
    let zero_p3 = [[0.0_f32; 3]; 3];
    for kernels in Kernels::available() {
        let mut actual_opaque = vec![0.0_f32; N * 4];
        linearize_row::<3>(
            DecodedRow::F32(&opaque_row),
            Primaries::DisplayP3,
            &zero_p3,
            &kernels,
            &mut actual_opaque,
        )
        .unwrap();
        for (idx, (&got, &want)) in actual_opaque.iter().zip(&expected_opaque).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} mixed P3 RGB idx={idx}",
                kernels.level()
            );
        }
        let mut actual_alpha = vec![0.0_f32; N * 4];
        linearize_row::<4>(
            DecodedRow::F32(&alpha_row),
            Primaries::DisplayP3,
            &zero_p3,
            &kernels,
            &mut actual_alpha,
        )
        .unwrap();
        for (idx, (&got, &want)) in actual_alpha.iter().zip(&expected_alpha).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} mixed P3 RGBA idx={idx}",
                kernels.level()
            );
        }
    }
}

/// Regression for PR #292 round-9 finding 1: `quantize_row_deep::<4>` must
/// skip the OETF kernel cleanly on fully transparent chunks while matching
/// reference encoding on partially transparent chunks (visible pixel at
/// first/middle/last lane) and still rejecting non-finite RGB behind zero alpha.
#[test]
fn transparent_deep_chunks_match_reference() {
    const N: usize = 70;
    let mut acc = vec![0.0_f32; N * 4];
    for px in acc.as_chunks_mut::<4>().0 {
        *px = [0.25, 0.5, 0.75, 0.0];
    }
    for kernels in Kernels::available() {
        let mut out = vec![u16::MAX; N * 4];
        quantize_row_deep::<4>(&acc, &mut out, 16, &kernels).unwrap();
        assert!(
            out.iter().all(|&v| v == 0),
            "{} fully transparent row",
            kernels.level()
        );

        for visible_idx in [0_usize, 31, 63, 69] {
            let mut partial = acc.clone();
            partial[visible_idx * 4..visible_idx * 4 + 4].copy_from_slice(&[0.25, 0.5, 0.125, 0.5]);
            let mut partial_out = vec![u16::MAX; N * 4];
            quantize_row_deep::<4>(&partial, &mut partial_out, 16, &kernels).unwrap();
            for (idx, px) in partial_out.as_chunks::<4>().0.iter().enumerate() {
                if idx == visible_idx {
                    let expected = [
                        encode_linear_srgb(0.5, 65_535).unwrap(),
                        encode_linear_srgb(1.0, 65_535).unwrap(),
                        encode_linear_srgb(0.25, 65_535).unwrap(),
                        32_768_u16,
                    ];
                    assert_eq!(
                        *px,
                        expected,
                        "{} partial visible_idx={visible_idx}",
                        kernels.level()
                    );
                } else {
                    assert_eq!(
                        *px,
                        [0, 0, 0, 0],
                        "{} partial transparent idx={idx}",
                        kernels.level()
                    );
                }
            }
        }

        let mut bad = acc.clone();
        bad[0] = f32::NAN;
        let err = quantize_row_deep::<4>(&bad, &mut out, 16, &kernels).unwrap_err();
        assert_eq!(err, Error::NonFinitePixelSample);
    }
}

fn run_synthetic_bench_case(
    label: &str,
    source: &TestSource,
    target_size: Size,
    depth: u8,
    kernels: &Kernels,
    budget: &MemoryBudget,
) {
    use std::hint::black_box;
    use std::time::{Duration, Instant};

    let oracle = render_depth(source, None, target_size, Filter::Area, None, depth);
    let fast = render_depth(
        source,
        None,
        target_size,
        Filter::Area,
        Some((kernels, 1)),
        depth,
    );
    let max_diff = oracle
        .iter()
        .zip(&fast)
        .map(|(&a, &b)| a.abs_diff(b))
        .max()
        .unwrap_or(0);
    assert!(
        max_diff <= 1,
        "{label} exceeded ADR-0021 1-code bound: {max_diff}"
    );
    let (graph, input, output) = graph_for(source, None, target_size, Filter::Area);
    let request = Rect::new(0, 0, target_size.width, target_size.height).unwrap();
    let mut src = source.clone();
    let mut oracle_time = Duration::MAX;
    for _ in 0..5 {
        src.calls = 0;
        let started = Instant::now();
        execute_quality(
            &graph,
            output,
            input,
            &mut src,
            request,
            budget,
            depth,
            |_, row| {
                black_box(row);
                Ok(())
            },
        )
        .unwrap();
        oracle_time = oracle_time.min(started.elapsed());
    }
    let mut fast_time = Duration::MAX;
    for _ in 0..5 {
        src.calls = 0;
        let started = Instant::now();
        execute_fast(
            &graph,
            output,
            input,
            &mut src,
            request,
            budget,
            kernels,
            1,
            depth,
            |_, row| {
                black_box(row);
                Ok(())
            },
        )
        .unwrap();
        fast_time = fast_time.min(started.elapsed());
    }
    let oracle_ms = oracle_time.as_secs_f64() * 1e3;
    let fast_ms = fast_time.as_secs_f64() * 1e3;
    println!(
        "| {label} | {oracle_ms:.2} | {fast_ms:.2} | {:.2}x | {max_diff} |",
        oracle_ms / fast_ms
    );
}

/// Regression for PR #292 round-10 finding 2: blue-only Display P3 ramps
/// (`[0, 0, b]`, alone or mixed with normal P3 pixels in the same 64-pixel
/// chunk) must match `working_color` bit-for-bit across every ISA level in
/// both RGB and RGBA without double-decoding.
#[test]
fn blue_only_p3_ramp_matches_reference_bit_for_bit() {
    const N: usize = 72;
    let p3_color = ColorEncoding::Cicp(P3);
    let mut opaque_row = Vec::with_capacity(N * 3);
    let mut alpha_row = Vec::with_capacity(N * 4);
    let mut expected_opaque = Vec::with_capacity(N * 4);
    let mut expected_alpha = Vec::with_capacity(N * 4);
    for i in 0..N {
        #[allow(clippy::cast_precision_loss)]
        let b = (i as f32) / ((N - 1) as f32);
        let px = if i % 11 == 5 {
            [0.2, 0.4, b]
        } else {
            [0.0, 0.0, b]
        };
        let alpha = [0.0_f32, 0.5, 1.0][i % 3];
        opaque_row.extend_from_slice(&px);
        alpha_row.extend_from_slice(&[px[0], px[1], px[2], alpha]);
        let lin_opaque = decode_to_linear_srgb(
            &p3_color,
            [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
        )
        .unwrap();
        #[allow(clippy::cast_possible_truncation)]
        expected_opaque.extend_from_slice(&[
            lin_opaque.rgb[0] as f32,
            lin_opaque.rgb[1] as f32,
            lin_opaque.rgb[2] as f32,
            0.0,
        ]);
        let lin_alpha = decode_straight_to_premultiplied_linear_srgb(
            &p3_color,
            [f64::from(px[0]), f64::from(px[1]), f64::from(px[2])],
            f64::from(alpha),
        )
        .unwrap();
        #[allow(clippy::cast_possible_truncation)]
        expected_alpha.extend_from_slice(&[
            lin_alpha.rgb[0] as f32,
            lin_alpha.rgb[1] as f32,
            lin_alpha.rgb[2] as f32,
            alpha,
        ]);
    }
    let zero_p3 = [[0.0_f32; 3]; 3];
    for kernels in Kernels::available() {
        let mut actual_opaque = vec![0.0_f32; N * 4];
        linearize_row::<3>(
            DecodedRow::F32(&opaque_row),
            Primaries::DisplayP3,
            &zero_p3,
            &kernels,
            &mut actual_opaque,
        )
        .unwrap();
        for (idx, (&got, &want)) in actual_opaque.iter().zip(&expected_opaque).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} blue-only P3 RGB idx={idx}",
                kernels.level()
            );
        }
        let mut actual_alpha = vec![0.0_f32; N * 4];
        linearize_row::<4>(
            DecodedRow::F32(&alpha_row),
            Primaries::DisplayP3,
            &zero_p3,
            &kernels,
            &mut actual_alpha,
        )
        .unwrap();
        for (idx, (&got, &want)) in actual_alpha.iter().zip(&expected_alpha).enumerate() {
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{} blue-only P3 RGBA idx={idx}",
                kernels.level()
            );
        }
    }
}

/// Repeatable synthetic benchmarks for ADR-0021 (issue #281):
/// 1. Extended-range Display P3 `1000x1000 -> 500x500` Area resize (RGB/RGBA,
///    all RGB samples in `[1.1, 2.1]`, 16-bit output).
/// 2. Fully transparent linear-RGBA `4096x512` Area identity (16-bit output).
/// 3. Mixed-range Display P3 `[1.1, 0.5, 0.6]` `4096x512` Area identity (8-bit).
/// 4. All-dark `[0, 0.01]` F32 sRGB `4096x512 -> 2048x256` Area resize (8-bit,
///    non-1:1 so it exercises `srgb_to_linear` past #284's identity bypass).
/// 5. Blue-only Display P3 `[0, 0, b]` ramp `4096x512` Area identity (8-bit).
/// 6. Transparent-black Display P3 `[0, 0, 0, 0]` `4096x512` Area identity (8-bit).
/// 7. Negative-zero straight-alpha sRGB `[-0.6, -0.52, -0.3, 0]` `4096x512 -> 2048x256` Area (8-bit).
///
/// Run with:
/// `cargo test --release -p ferrisma --lib bench_extended_p3_area_resize -- --ignored --nocapture`
#[test]
#[ignore = "manual benchmark for issue #281"]
#[allow(clippy::too_many_lines)] // Single self-contained benchmark table of all 8 synthetic cases.
fn bench_extended_p3_area_resize() {
    const LINEAR_SRGB: [u8; 4] = [1, 8, 0, 1];
    let in_size = Size::new(1000, 1000).unwrap();
    let out_size = Size::new(500, 500).unwrap();
    let wide_size = Size::new(4096, 512).unwrap();
    let half_wide = Size::new(2048, 256).unwrap();
    let wide_pixels = 4096 * 512;

    let opaque_samples: Vec<f32> = (0..1000 * 1000 * 3)
        .map(|i| 1.1 + f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let mut alpha_samples = Vec::with_capacity(1000 * 1000 * 4);
    for px in opaque_samples.as_chunks::<3>().0 {
        alpha_samples.extend_from_slice(&[px[0], px[1], px[2], 0.75]);
    }
    let mut blue_p3_samples = Vec::with_capacity(wide_pixels * 3);
    for i in 0..wide_pixels {
        let b = f32::from(u16::try_from(i % 4096).unwrap()) / 4095.0;
        blue_p3_samples.extend_from_slice(&[0.0, 0.0, b]);
    }
    let dark_samples: Vec<f32> = (0..wide_pixels * 3)
        .map(|i| 0.01 * f32::from(u16::try_from(i % 256).unwrap()) / 255.0)
        .collect();
    let mut sparse_alpha = vec![0.0_f32; wide_pixels * 4];
    for (idx, px) in sparse_alpha.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        if idx % 64 == 0 {
            *px = [0.25, 0.5, 0.75, 1.0];
        }
    }
    let dark_half = 4.5_f32 / (65_535.0 * 12.92);
    let dark_oetf_samples = [dark_half, dark_half, dark_half, 1.0_f32].repeat(wide_pixels);
    let mixed_tie_oetf_samples =
        [f32::from_bits(0x3b4f_ad81), 0.5_f32, 0.75_f32].repeat(wide_pixels);

    let kernels = Kernels::best();
    let budget = MemoryBudget::new(64 * 1024 * 1024);
    println!(
        "issue #281 synthetic benchmarks (level: {}, threads=1, best of 5):",
        kernels.level()
    );
    println!("| case | execute_quality ms | execute_fast ms | speedup | max_code_diff |");
    println!("| --- | ---: | ---: | ---: | ---: |");
    let cases = [
        (
            "extended-p3 RGB 1000x1000->500x500 16b",
            TestSource::new(
                in_size,
                ChannelLayout::Rgb,
                P3,
                Samples::F32(opaque_samples),
            ),
            out_size,
            16_u8,
        ),
        (
            "extended-p3 RGBA 1000x1000->500x500 16b",
            TestSource::new(
                in_size,
                ChannelLayout::Rgba,
                P3,
                Samples::F32(alpha_samples),
            ),
            out_size,
            16,
        ),
        (
            "transparent-linear-rgba 4096x512 16b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgba,
                LINEAR_SRGB,
                Samples::F32([0.5_f32, 0.25, 0.75, 0.0].repeat(wide_pixels)),
            ),
            wide_size,
            16,
        ),
        (
            "sparse-alpha-rgba (1/64 visible) 4096x512 16b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgba,
                LINEAR_SRGB,
                Samples::F32(sparse_alpha),
            ),
            wide_size,
            16,
        ),
        (
            "dark-oetf-half-code 4096x512 16b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgba,
                LINEAR_SRGB,
                Samples::F32(dark_oetf_samples),
            ),
            wide_size,
            16,
        ),
        (
            "mixed-tie-oetf [0x3b4fad81,0.5,0.75] 4096x512 16b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgb,
                LINEAR_SRGB,
                Samples::F32(mixed_tie_oetf_samples),
            ),
            wide_size,
            16,
        ),
        (
            "mixed-p3 [1.1,0.5,0.6] 4096x512 8b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgb,
                P3,
                Samples::F32([1.1_f32, 0.5, 0.6].repeat(wide_pixels)),
            ),
            wide_size,
            8,
        ),
        (
            "dark-srgb [0,0.01] 4096x512->2048x256 8b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgb,
                SRGB,
                Samples::F32(dark_samples),
            ),
            half_wide,
            8,
        ),
        (
            "blue-only-p3 [0,0,b] 4096x512 8b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgb,
                P3,
                Samples::F32(blue_p3_samples),
            ),
            wide_size,
            8,
        ),
        (
            "transparent-black-p3 [0,0,0,0] 4096x512 8b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgba,
                P3,
                Samples::F32(vec![0.0_f32; wide_pixels * 4]),
            ),
            wide_size,
            8,
        ),
        (
            "neg-zero-srgb [-0.6,-0.52,-0.3,0] 4096x512->2048x256 8b",
            TestSource::new(
                wide_size,
                ChannelLayout::Rgba,
                SRGB,
                Samples::F32([-0.6_f32, -0.52, -0.3, 0.0].repeat(wide_pixels)),
            ),
            half_wide,
            8,
        ),
    ];
    for (label, source, target_size, depth) in &cases {
        run_synthetic_bench_case(label, source, *target_size, *depth, &kernels, &budget);
    }
}

/// Regression for issue #283 review findings: Area downscales of opaque
/// linear-f32 pixels must preserve cancellation on `Kernels::best()` (default
/// v1.0 level: `Neon` on `AArch64`, `Avx2` on x86-64) across both single-pixel
/// tail and multi-pixel pipelined + parallel worker paths:
/// - `[67108864, 0, -67108864, 0, 1, 0, 0, 0]` (`8x1 -> 1x1` and
///   `64x64 -> 8x32`) -> sRGB8 red code `99`.
/// - `[0, 0, 0, -100663296, 67108864]` (`5x1 -> 3x1` and `40x64 -> 24x32`) ->
///   sRGB8 red code `0` (exact match to `f64` oracle).
#[test]
#[allow(clippy::too_many_lines)]
fn linear_f32_area_downscale_preserves_large_opposite_tap_cancellation() {
    struct Case<'a> {
        block: &'a [f32],
        pixel: [u8; 4],
        source: (u32, u32),
        target: (u32, u32),
        expect_parallel: bool,
    }
    let cancel_8 = [
        67_108_864.0_f32,
        0.0,
        -67_108_864.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
    ];
    let unequal_5 = [0.0_f32, 0.0, 0.0, -100_663_296.0, 67_108_864.0];
    let cases = [
        Case {
            block: &cancel_8,
            pixel: [99, 0, 0, 255],
            source: (8, 1),
            target: (1, 1),
            expect_parallel: false,
        },
        Case {
            block: &cancel_8,
            pixel: [99, 0, 0, 255],
            source: (64, 64),
            target: (8, 32),
            expect_parallel: true,
        },
        Case {
            block: &unequal_5,
            pixel: [0, 0, 0, 255],
            source: (5, 1),
            target: (3, 1),
            expect_parallel: false,
        },
        Case {
            block: &unequal_5,
            pixel: [0, 0, 0, 255],
            source: (40, 64),
            target: (24, 32),
            expect_parallel: true,
        },
    ];
    let kernels = Kernels::best();
    for case in cases {
        let source_size = Size::new(case.source.0, case.source.1).unwrap();
        let target_size = Size::new(case.target.0, case.target.1).unwrap();
        let mut samples = Vec::with_capacity((case.source.0 * case.source.1 * 3) as usize);
        for _ in 0..source_size.height {
            for x in 0..(source_size.width as usize) {
                samples.extend_from_slice(&[case.block[x % case.block.len()], 0.0, 0.0]);
            }
        }
        let source = TestSource::new(
            source_size,
            ChannelLayout::Rgb,
            [1, 8, 0, 1],
            Samples::F32(samples),
        );
        let oracle: Vec<u8> = render_depth(&source, None, target_size, Filter::Area, None, 8)
            .into_iter()
            .map(|v| u8::try_from(v).unwrap())
            .collect();
        let expected: Vec<u8> = case
            .pixel
            .into_iter()
            .cycle()
            .take((case.target.0 * case.target.1 * 4) as usize)
            .collect();
        assert_eq!(oracle, expected);

        for threads in [1_usize, 4] {
            let mut src = source.clone();
            let (graph, input, target) =
                graph_for_info(src.info(), None, target_size, Filter::Area);
            let budget = MemoryBudget::new(8 << 20);
            let request = Rect::new(0, 0, target_size.width, target_size.height).unwrap();
            let mut actual = Vec::new();
            let report = execute_fast_srgb8(
                &graph,
                target,
                input,
                &mut src,
                request,
                &budget,
                &kernels,
                threads,
                |_, row| {
                    actual.extend_from_slice(row);
                    Ok(())
                },
            )
            .unwrap();
            assert!(report.used_kernels);
            if case.expect_parallel && threads > 1 {
                assert!(report.transform_threads > 1);
            }
            assert_eq!(actual.len(), oracle.len());
            assert_eq!(actual, oracle);
        }
    }
}

/// Regression for issue #283 review finding: requesting a sub-rect crop must
/// produce bit-identical bytes to the same pixels inside a wider full-row
/// request, even when unrelated pixels outside the crop are HDR (> 2.0).
#[test]
fn horizontal_crop_request_is_invariant_to_unrelated_hdr_pixels() {
    let source_size = Size::new(6, 1).unwrap();
    let target_size = Size::new(2, 1).unwrap();
    let v = f32::from_bits(0x39ee_b40d);
    let samples = vec![
        v, 0.0, 0.0, v, 0.0, 0.0, v, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0, 3.0, 0.0, 0.0,
    ];
    let source = TestSource::new(
        source_size,
        ChannelLayout::Rgb,
        [1, 8, 0, 1],
        Samples::F32(samples),
    );
    for kernels in Kernels::available() {
        let mut src_full = source.clone();
        let (graph_full, input_full, target_full) =
            graph_for_info(src_full.info(), None, target_size, Filter::Area);
        let budget = MemoryBudget::new(8 << 20);
        let mut full = Vec::new();
        execute_fast_srgb8(
            &graph_full,
            target_full,
            input_full,
            &mut src_full,
            Rect::new(0, 0, 2, 1).unwrap(),
            &budget,
            &kernels,
            1,
            |_, row| {
                full.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();

        let mut src_crop = source.clone();
        let (graph_crop, input_crop, target_crop) =
            graph_for_info(src_crop.info(), None, target_size, Filter::Area);
        let mut crop = Vec::new();
        execute_fast_srgb8(
            &graph_crop,
            target_crop,
            input_crop,
            &mut src_crop,
            Rect::new(0, 0, 1, 1).unwrap(),
            &budget,
            &kernels,
            1,
            |_, row| {
                crop.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();

        assert_eq!(
            &full[..4],
            crop.as_slice(),
            "{} crop pixel 0 diverged from full-row pixel 0",
            kernels.level()
        );
    }
}

/// Regression for issue #283 review finding: on `Avx2` (`SLOTS = 2`), preserve
/// `main`'s exact 2-slot accumulation behavior for `4000x1 -> 1x1` flat sRGB8
/// code `235` at 16-bit depth (`60395`) and `128x1 -> 1x1`
/// `32*2.0 + 64*2^-18 + 32*-2.0` at 16-bit depth (`2`).
#[test]
fn avx2_two_slot_horizontal_preserves_long_and_mixed_scale_16bit_accuracy() {
    let Ok(kernels) = Kernels::for_level(IsaLevel::Avx2) else {
        return;
    };
    let budget = MemoryBudget::new(16 << 20);

    // 1. 4000 identical sRGB8 code-235 pixels -> 1x1 at 16-bit (`60395`).
    let flat_src = TestSource::new(
        Size::new(4000, 1).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(vec![235_u8; 4000 * 3]),
    );
    let flat_oracle = render_depth(
        &flat_src,
        None,
        Size::new(1, 1).unwrap(),
        Filter::Area,
        None,
        16,
    );
    assert_eq!(flat_oracle, vec![60_395, 60_395, 60_395, 65_535]);
    let mut flat_actual = Vec::new();
    let mut src = flat_src.clone();
    let (graph, input, target) =
        graph_for_info(src.info(), None, Size::new(1, 1).unwrap(), Filter::Area);
    let report = execute_fast(
        &graph,
        target,
        input,
        &mut src,
        Rect::new(0, 0, 1, 1).unwrap(),
        &budget,
        &kernels,
        1,
        16,
        |_, row| {
            match row {
                OutputRow::U16(s) => flat_actual.extend_from_slice(s),
                OutputRow::U8(_) => panic!("expected u16"),
            }
            Ok(())
        },
    )
    .unwrap();
    assert!(report.used_kernels);
    assert_eq!(flat_actual.len(), flat_oracle.len());
    assert_eq!(flat_actual, flat_oracle);

    // 2. 128 linear-f32 pixels: 32*2.0 + 64*2^-18 + 32*-2.0 -> 1x1 at 16-bit (`2`).
    let mut mixed_samples = Vec::with_capacity(128 * 3);
    for i in 0..128_usize {
        let r = if i < 32 {
            2.0_f32
        } else if i < 96 {
            2.0_f32.powi(-18)
        } else {
            -2.0_f32
        };
        mixed_samples.extend_from_slice(&[r, 0.0, 0.0]);
    }
    let mixed_src = TestSource::new(
        Size::new(128, 1).unwrap(),
        ChannelLayout::Rgb,
        [1, 8, 0, 1],
        Samples::F32(mixed_samples),
    );
    let mixed_oracle = render_depth(
        &mixed_src,
        None,
        Size::new(1, 1).unwrap(),
        Filter::Area,
        None,
        16,
    );
    assert_eq!(mixed_oracle, vec![2, 0, 0, 65_535]);
    let mut mixed_actual = Vec::new();
    let mut src2 = mixed_src.clone();
    let (graph2, input2, target2) =
        graph_for_info(src2.info(), None, Size::new(1, 1).unwrap(), Filter::Area);
    let report2 = execute_fast(
        &graph2,
        target2,
        input2,
        &mut src2,
        Rect::new(0, 0, 1, 1).unwrap(),
        &budget,
        &kernels,
        1,
        16,
        |_, row| {
            match row {
                OutputRow::U16(s) => mixed_actual.extend_from_slice(s),
                OutputRow::U8(_) => panic!("expected u16"),
            }
            Ok(())
        },
    )
    .unwrap();
    assert!(report2.used_kernels);
    assert_eq!(mixed_actual.len(), mixed_oracle.len());
    assert_eq!(mixed_actual, mixed_oracle);
}

/// Regression for issue #283 review finding: vertical downscales with >64 taps
/// chunk through `vertical_rgba` + `vertical_rgba_add` without heap-allocating
/// row-slice vectors, staying byte-identical across sequential/parallel runs
/// and every available ISA level and matching the `f64` oracle.
#[test]
fn vertical_downscale_above_64_taps_matches_oracle_sequential_and_parallel() {
    let source_size = Size::new(8, 150).unwrap();
    let target_size = Size::new(4, 1).unwrap();
    let mut samples = Vec::with_capacity(8 * 150 * 4);
    for y in 0..150_u32 {
        for x in 0..8_u32 {
            let r = u8::try_from((y * 7 + x * 29) % 256).unwrap();
            let g = u8::try_from((y * 13 + x * 17 + 41) % 256).unwrap();
            let b = u8::try_from((y * 19 + x * 11 + 83) % 256).unwrap();
            let a = u8::try_from(128 + ((y * 5 + x * 3) % 128)).unwrap();
            samples.extend_from_slice(&[r, g, b, a]);
        }
    }
    let source = TestSource::new(
        source_size,
        ChannelLayout::Rgba,
        [1, 13, 0, 1],
        Samples::U8(samples),
    );
    let oracle: Vec<u8> = render_depth(&source, None, target_size, Filter::Area, None, 8)
        .into_iter()
        .map(|v| u8::try_from(v).unwrap())
        .collect();

    let mut reference: Option<Vec<u8>> = None;
    for kernels in Kernels::available() {
        for threads in [1_usize, 4] {
            let mut src = source.clone();
            let (graph, input, target) =
                graph_for_info(src.info(), None, target_size, Filter::Area);
            let budget = MemoryBudget::new(8 << 20);
            let request = Rect::new(0, 0, target_size.width, target_size.height).unwrap();
            let mut actual = Vec::new();
            let report = execute_fast_srgb8(
                &graph,
                target,
                input,
                &mut src,
                request,
                &budget,
                &kernels,
                threads,
                |_, row| {
                    actual.extend_from_slice(row);
                    Ok(())
                },
            )
            .unwrap();
            assert!(report.used_kernels);
            if threads > 1 {
                assert!(
                    report.transform_threads > 1,
                    "{} threads={threads} must exercise parallel workers",
                    kernels.level()
                );
            }
            if let Some(first) = &reference {
                assert_eq!(
                    &actual,
                    first,
                    "{} threads={threads} diverged from reference execution",
                    kernels.level()
                );
            } else {
                reference = Some(actual.clone());
            }
            assert_eq!(actual.len(), oracle.len());
            for (i, (&got, &exp)) in actual.iter().zip(&oracle).enumerate() {
                assert!(
                    got.abs_diff(exp) <= 1,
                    "{} threads={threads} byte {i}: got {got}, oracle {exp}",
                    kernels.level()
                );
            }
        }
    }
}

/// Issue #68 / ADR-0022: `QualityReport::transform_threads` reports the actual
/// row-pipeline worker count spawned by `parallel_execute::run` after thread
/// partitioning with a transposing `OrientedSource`.
#[test]
fn transform_threads_report_matches_actual_worker_count_with_transposing_orientation() {
    use crate::orientation::{Orientation, OrientedSource};

    struct OrientedTestSource {
        inner: TestSource,
        orientation: Orientation,
    }
    impl DecodeSource for OrientedTestSource {
        fn info(&self) -> &DecoderInfo {
            self.inner.info()
        }
        fn orientation(&self) -> Orientation {
            self.orientation
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.inner.read_region(region, budget, sink)
        }
    }

    let stored_size = Size::new(100, 80).unwrap();
    let samples: Vec<u8> = (0..100 * 80)
        .flat_map(|i| {
            [
                u8::try_from((i * 7) % 256).unwrap(),
                u8::try_from((i * 13 + 31) % 256).unwrap(),
                u8::try_from((i * 19 + 73) % 256).unwrap(),
            ]
        })
        .collect();
    let base = TestSource::new(
        stored_size,
        ChannelLayout::Rgb,
        [1, 13, 0, 1],
        Samples::U8(samples),
    );
    let target_size = Size::new(40, 50).unwrap();
    let request = Rect::new(0, 0, target_size.width, target_size.height).unwrap();
    let kernels = Kernels::best();

    let run_with = |threads: usize| {
        let mut src = OrientedSource::new(Box::new(OrientedTestSource {
            inner: base.clone(),
            orientation: Orientation::from_exif(6).unwrap(),
        }))
        .unwrap();
        let (graph, input, target) =
            graph_for_info(src.info(), None, target_size, Filter::Lanczos3);
        let budget = MemoryBudget::new(8 << 20);
        let mut out = Vec::new();
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut src,
            request,
            &budget,
            &kernels,
            threads,
            |_, row| {
                out.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
        (report, out)
    };

    let (r1, out1) = run_with(1);
    let (r5, out5) = run_with(5);
    let (r8, out8) = run_with(8);
    assert_eq!(r1.transform_threads, 1);
    // At threads=5, gather stays serial (1), so all 5 threads go to workers.
    assert_eq!(r5.transform_threads, 5);
    // At threads=8 on a 100-row transposing source (>= PARALLEL_MIN_ROWS=64),
    // gather takes 2 threads and row-pipeline workers take 6.
    assert_eq!(r8.transform_threads, 6);
    assert_eq!(out5, out1);
    assert_eq!(out8, out1);
}

/// Regression for issue #210: on a large vertical upscale (`100x100 -> 1000x1000`),
/// draining completed strips inside `Driver::advance` and bounding in-flight
/// vertical rows prevents `OutputBuffer`s from piling up unboundedly in `done`,
/// keeping `peak_managed_bytes` well below full-frame output storage and letting
/// tight-budget runs succeed with byte-identical output and `live == 0`.
#[test]
fn parallel_upscale_bounds_in_flight_output_buffers_under_tight_budget() {
    let in_size = Size::new(100, 100).unwrap();
    let out_size = Size::new(1000, 1000).unwrap();
    let mut data = Vec::with_capacity(100 * 100 * 3);
    for y in 0..100_u32 {
        for x in 0..100_u32 {
            data.push(u8::try_from((x * 5 + y * 3) % 256).unwrap());
            data.push(u8::try_from((x * 7 + y * 11) % 256).unwrap());
            data.push(u8::try_from((x * 13 + y * 2) % 256).unwrap());
        }
    }
    let source = TestSource::new(in_size, ChannelLayout::Rgb, SRGB, Samples::U8(data));
    let (graph, input, target) = graph_for(&source, None, out_size, Filter::Lanczos3);
    let request = Rect::new(0, 0, out_size.width, out_size.height).unwrap();
    let kernels = Kernels::best();

    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_src = source.clone();
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    let seq_report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    // Full output frame would be 1000 * 1000 * 4 = 4,000,000 bytes; bounded
    // draining keeps parallel overhead far below half an output frame (1 MiB
    // above sequential peak for 4 threads).
    let tight_limit = seq_report.peak_engine_managed_bytes + 1_024 * 1_024;
    for _ in 0..5 {
        let budget = MemoryBudget::new(tight_limit);
        let mut src = source.clone();
        let mut hash = 0xCBF2_9CE4_8422_2325_u64;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut src,
            request,
            &budget,
            &kernels,
            4,
            |_, row| {
                hash = fnv1a64_update(hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(budget.snapshot().live, 0);
        assert_eq!(hash, seq_hash);
        assert_eq!(report.transform_threads, 4);
        assert!(
            report.peak_engine_managed_bytes <= tight_limit,
            "peak {} exceeded tight limit {}",
            report.peak_engine_managed_bytes,
            tight_limit
        );
    }
}

fn make_rgb_pattern_source(w: u32, h: u32) -> TestSource {
    let row_bytes = w as usize * 3;
    let mut data = vec![0_u8; row_bytes * h as usize];
    for y in 0..h as usize {
        let row = &mut data[y * row_bytes..(y + 1) * row_bytes];
        let seed = u8::try_from((y * 29) & 0xFF).unwrap();
        for (i, slot) in row.iter_mut().enumerate() {
            *slot = seed.wrapping_add(u8::try_from((i * 7) & 0xFF).unwrap());
        }
    }
    TestSource::new(
        Size::new(w, h).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(data),
    )
}

fn sequential_hash_and_peak(
    src: &TestSource,
    out_size: Size,
    filter: Filter,
    kernels: &Kernels,
) -> (u64, usize) {
    let (graph, input, target) = graph_for(src, None, out_size, filter);
    let request = Rect::new(0, 0, out_size.width, out_size.height).unwrap();
    let mut seq_src = src.clone();
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);
    (seq_hash, report.peak_engine_managed_bytes)
}

/// Regression for Codex PR #295 finding 1 [P2]: steep downscale
/// (`4096x1280 -> 4096x128` Area, 2 workers, 10 MB budget) must not retain an
/// oversized multi-row vertical strip before releasing horizontal rows.
#[test]
fn parallel_steep_downscale_completes_under_tight_budget() {
    let src = make_rgb_pattern_source(4096, 1280);
    let out_size = Size::new(4096, 128).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&src, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&src, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 4096, 128).unwrap();
    let mut par_src = src;
    let par_budget = MemoryBudget::new(10_000_000);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &par_budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 128);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(par_budget.snapshot().live, 0);
    assert!(report.peak_engine_managed_bytes <= 10_000_000);
}

/// Regression for Codex PR #295 finding 2 [P2]: tight-budget 8-worker resize
/// (`4096x128 -> 4096x64` Area) must reclaim idle driver pools instead of
/// failing output/horizontal allocations while another pool holds memory.
#[test]
fn parallel_eight_worker_tight_budget_reclaims_idle_pools() {
    let src = make_rgb_pattern_source(4096, 128);
    let out_size = Size::new(4096, 64).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, seq_peak) = sequential_hash_and_peak(&src, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&src, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 4096, 64).unwrap();
    let tight_limit = seq_peak + 1_572_864;
    for _ in 0..3 {
        let mut par_src = src.clone();
        let par_budget = MemoryBudget::new(tight_limit);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            8,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 64);
        assert_eq!(report.transform_threads, 8);
        assert_eq!(par_hash, seq_hash);
        assert_eq!(par_budget.snapshot().live, 0);
        assert!(report.peak_engine_managed_bytes <= tight_limit);
    }
}

/// Regression for Codex PR #295 finding 3 [P3]: 2-worker upscale
/// (`4096x128 -> 4096x256` Area) must count each incoming strip against
/// `max_vertical_in_flight` both during decode and after decoder EOF
/// (enforced by `debug_assert!` in `Driver::advance`).
#[test]
fn parallel_upscale_enforces_strict_vertical_cap_after_eof() {
    let src = make_rgb_pattern_source(4096, 128);
    let out_size = Size::new(4096, 256).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&src, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&src, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 4096, 256).unwrap();
    let mut par_src = src;
    let par_budget = MemoryBudget::new(64 << 20);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &par_budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 256);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(par_budget.snapshot().live, 0);
}

/// Regression for Codex PR #295 follow-up finding [P2]: horizontal batch
/// pre-allocation must never consume the last remaining bytes of a tight
/// budget and starve vertical emission (`16x256 -> 4096x128` Area, 2 workers,
/// at `6,894,192` and `7,074,416` bytes).
#[test]
fn parallel_horizontal_batch_keeps_reserved_output_buffer_under_tight_budget() {
    let src = make_rgb_pattern_source(16, 256);
    let out_size = Size::new(4096, 128).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&src, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&src, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 4096, 128).unwrap();
    for limit in [6_894_192_usize, 7_074_416] {
        let mut par_src = src.clone();
        let par_budget = MemoryBudget::new(limit);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            2,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 128, "limit={limit}");
        assert_eq!(report.transform_threads, 2, "limit={limit}");
        assert_eq!(par_hash, seq_hash, "limit={limit}");
        assert_eq!(par_budget.snapshot().live, 0, "limit={limit}");
        assert!(report.peak_engine_managed_bytes <= limit, "limit={limit}");
    }
}

/// Regression for Codex PR #295 follow-up finding 1 [P2]: leave budget
/// headroom for concurrent `OrientedSource` parallel gather blocks
/// (`512x256` EXIF orientation 4 vertically flipped RGB -> `128x256` Area,
/// 2 workers per stage, `1,320,000`-byte budget).
#[test]
fn parallel_oriented_gather_preserves_source_headroom_under_tight_budget() {
    struct FlippedSource(TestSource);
    impl DecodeSource for FlippedSource {
        fn info(&self) -> &DecoderInfo {
            self.0.info()
        }
        fn orientation(&self) -> crate::orientation::Orientation {
            crate::orientation::Orientation::from_exif(4).unwrap()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.0.read_region(region, budget, sink)
        }
    }

    let base = make_rgb_pattern_source(512, 256);
    let out_size = Size::new(128, 256).unwrap();
    let kernels = Kernels::best();
    let request = Rect::new(0, 0, 128, 256).unwrap();

    let mut seq_src =
        crate::orientation::OrientedSource::new(Box::new(FlippedSource(base.clone())))
            .unwrap()
            .with_threads(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    for _ in 0..3 {
        let mut par_src =
            crate::orientation::OrientedSource::new(Box::new(FlippedSource(base.clone())))
                .unwrap()
                .with_threads(2);
        let par_budget = MemoryBudget::new(1_320_000);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            2,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 256);
        assert_eq!(report.transform_threads, 2);
        assert_eq!(par_hash, seq_hash);
        assert_eq!(par_budget.snapshot().live, 0);
        assert!(report.peak_engine_managed_bytes <= 1_320_000);
    }
}

/// Regression for Codex PR #295 follow-up finding 2 [P2]: release
/// `reserved_output` as a last resort when output row 0 needs every source row
/// (`512x64 -> 4096x1` RGB Area, 2 workers, `10,750,888`-byte budget).
#[test]
fn parallel_releases_reserved_output_when_blocking_final_source_row() {
    let src = make_rgb_pattern_source(512, 64);
    let out_size = Size::new(4096, 1).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&src, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&src, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 4096, 1).unwrap();
    let mut par_src = src;
    let par_budget = MemoryBudget::new(10_750_888);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &par_budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 1);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(par_budget.snapshot().live, 0);
    assert!(report.peak_engine_managed_bytes <= 10_750_888);
}

/// Regression for Codex PR #295 Round-4 finding 1 [P2]: `reserved_output` must
/// be deferred until the first delivered needed row so it does not overlap
/// with full-frame decoder scratch that is released before streaming rows.
#[test]
fn parallel_defers_reserved_output_past_decoder_scratch() {
    struct ScratchBeforeRows {
        inner: TestSource,
        scratch_bytes: usize,
    }
    impl DecodeSource for ScratchBeforeRows {
        fn info(&self) -> &DecoderInfo {
            self.inner.info()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            // Simulate a codec that holds temporary decode scratch and drops
            // it before streaming rows.
            let decode_scratch = budget.allocate(self.scratch_bytes)?;
            drop(decode_scratch);
            self.inner.read_region(region, budget, sink)
        }
    }

    let base = make_rgb_pattern_source(128, 128);
    let out_size = Size::new(32, 128).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&base, out_size, Filter::Area, &kernels);
    let (graph, input, target) = graph_for(&base, None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 32, 128).unwrap();

    // Measure exact budget live at the start of `read_region` (plan + 2 worker
    // scratches, with zero output row pre-reserved).
    let probe_budget = MemoryBudget::new(64 << 20);
    let scratch_bytes = 512 * 1024;
    let mut probe_src = ScratchBeforeRows {
        inner: base.clone(),
        scratch_bytes,
    };
    let probe_report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut probe_src,
        request,
        &probe_budget,
        &kernels,
        2,
        |_, _| Ok(()),
    )
    .unwrap();
    assert_eq!(probe_budget.snapshot().live, 0);

    // Budget equals the exact peak observed with the temporary decode scratch
    // (if `reserved_output` were allocated before `read_region`, `decode_scratch`
    // would exceed this limit by 128 bytes).
    let exact_limit = probe_report.peak_engine_managed_bytes;
    let mut par_src = ScratchBeforeRows {
        inner: base,
        scratch_bytes,
    };
    let par_budget = MemoryBudget::new(exact_limit);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &par_budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 128);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(par_budget.snapshot().live, 0);
}

/// Regression for Codex PR #295 Round-4 finding 1 [P2] with the exact flipped
/// `128x128` WebP -> `32x128` 2-worker reproducer at `381,296` bytes.
#[cfg(feature = "webp-read")]
#[test]
fn parallel_flipped_webp_fits_exact_tight_budget_without_early_output_reserve() {
    struct FlippedWebp<'a> {
        inner: crate::codecs::webp::WebpSource<'a>,
        info: DecoderInfo,
    }
    impl DecodeSource for FlippedWebp<'_> {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn orientation(&self) -> crate::orientation::Orientation {
            crate::orientation::Orientation::from_exif(4).unwrap()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.inner.read_region(region, budget, sink)
        }
    }

    let webp = include_bytes!("../tests/fixtures/webp/reference-lossy.webp");
    let out_size = Size::new(32, 128).unwrap();
    let request = Rect::new(0, 0, 32, 128).unwrap();
    let kernels = Kernels::best();

    let make_oriented = |threads: usize| {
        let raw = crate::codecs::webp::WebpSource::new_bounded(webp, 1_000_000).unwrap();
        let mut info = raw.info().clone();
        info.rendered_cicp = Some(SRGB);
        crate::orientation::OrientedSource::new(Box::new(FlippedWebp { inner: raw, info }))
            .unwrap()
            .with_threads(threads)
    };

    let mut seq_src = make_oriented(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    for _ in 0..3 {
        let mut par_src = make_oriented(2);
        let par_budget = MemoryBudget::new(381_296);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            2,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 128);
        assert_eq!(report.transform_threads, 2);
        assert_eq!(par_hash, seq_hash);
        assert_eq!(par_budget.snapshot().live, 0);
        assert!(report.peak_engine_managed_bytes <= 381_296);
    }
}

struct ConcurrencyProbe {
    state: std::sync::Mutex<(usize, usize)>,
    changed: std::sync::Condvar,
    waited: std::sync::atomic::AtomicUsize,
}

impl crate::decoder::RowConversion for ConcurrencyProbe {
    fn input_sample(&self) -> SampleType {
        SampleType::U8
    }
    fn input_channels(&self) -> usize {
        3
    }
    fn scratch_channels(&self) -> (usize, usize) {
        (0, 3)
    }
    fn convert(
        &self,
        row: DecodedRow<'_>,
        _scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        use std::sync::atomic::Ordering;
        use std::time::Duration;
        {
            let mut guard = self.state.lock().unwrap();
            guard.0 += 1;
            guard.1 = guard.1.max(guard.0);
            if guard.1 >= 2 {
                self.changed.notify_all();
            } else if self.waited.fetch_add(1, Ordering::Relaxed) == 0 {
                let _ = self
                    .changed
                    .wait_timeout_while(guard, Duration::from_millis(50), |s| s.1 < 2)
                    .unwrap();
            }
        }
        let DecodedRow::U8(bytes) = row else {
            return Err(Error::IncompleteDecoderOutput);
        };
        for (dst, &src) in out.iter_mut().zip(bytes) {
            *dst = f32::from(src) / 255.0;
        }
        let mut guard = self.state.lock().unwrap();
        guard.0 -= 1;
        Ok(())
    }
}

struct DeferredTestSource {
    inner: TestSource,
    info: DecoderInfo,
    conv: std::sync::Arc<dyn crate::decoder::RowConversion>,
    delivered: Option<std::sync::Arc<std::sync::atomic::AtomicU32>>,
}

impl DecodeSource for DeferredTestSource {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn defer_conversion(&mut self) -> Option<std::sync::Arc<dyn crate::decoder::RowConversion>> {
        Some(self.conv.clone())
    }
    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.inner.set_downstream_headroom(bytes);
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        let delivered = self.delivered.clone();
        self.inner.read_region(region, budget, &mut |y, row| {
            if let Some(counter) = &delivered {
                counter.fetch_add(1, std::sync::atomic::Ordering::Release);
            }
            sink(y, row)
        })
    }
}

struct StallFirstRow {
    stall_enabled: bool,
    stalled: std::sync::atomic::AtomicBool,
    delivered: std::sync::Arc<std::sync::atomic::AtomicU32>,
    delivered_while_stalled: std::sync::atomic::AtomicU32,
}

impl crate::decoder::RowConversion for StallFirstRow {
    fn input_sample(&self) -> SampleType {
        SampleType::U8
    }
    fn input_channels(&self) -> usize {
        3
    }
    fn scratch_channels(&self) -> (usize, usize) {
        (0, 3)
    }
    fn convert(
        &self,
        row: DecodedRow<'_>,
        _scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        use std::sync::atomic::Ordering;
        let DecodedRow::U8(bytes) = row else {
            return Err(Error::IncompleteDecoderOutput);
        };
        // Row 0 is tagged with sentinel byte `255` (no other row uses `255`),
        // so the stall deterministically hits strip 0 regardless of scheduling.
        if self.stall_enabled && bytes.first() == Some(&255) {
            std::thread::sleep(std::time::Duration::from_millis(30));
            self.delivered_while_stalled
                .store(self.delivered.load(Ordering::Acquire), Ordering::Release);
            self.stalled.store(true, Ordering::Release);
        }
        for (dst, &src) in out.iter_mut().zip(bytes) {
            *dst = f32::from(src) / 255.0;
        }
        Ok(())
    }
}

/// Regression for Codex PR #295 Round-4 finding 2 [P2]: non-oriented sources
/// must retain true multi-worker concurrency under tight budgets
/// (`512x256 -> 128x256` Area, 2 workers, `802,816` bytes) instead of
/// serializing horizontal rows.
#[test]
fn parallel_non_oriented_source_stays_concurrent_under_tight_budget() {
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Condvar, Mutex};

    let base = make_rgb_pattern_source(512, 256);
    let mut linear_info = base.info().clone();
    linear_info.sample = SampleType::F32;
    linear_info.rendered_cicp = Some([1, 8, 0, 1]);

    let probe = Arc::new(ConcurrencyProbe {
        state: Mutex::new((0, 0)),
        changed: Condvar::new(),
        waited: AtomicUsize::new(0),
    });
    let mut src = DeferredTestSource {
        inner: base,
        info: linear_info,
        conv: probe.clone(),
        delivered: None,
    };
    let out_size = Size::new(128, 256).unwrap();
    let (graph, input, target) = graph_for_info(src.info(), None, out_size, Filter::Area);
    let request = Rect::new(0, 0, 128, 256).unwrap();
    let budget = MemoryBudget::new(802_816);
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut src,
        request,
        &budget,
        &Kernels::best(),
        2,
        |y, _| {
            assert_eq!(y, rows);
            rows += 1;
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 256);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(budget.snapshot().live, 0);
    let max_active = probe.state.lock().unwrap().1;
    assert!(
        max_active >= 2,
        "expected >= 2 concurrent horizontal row conversions on non-oriented source, got {max_active}"
    );
}

/// Regression for Codex PR #295 Round-4 finding 3 [P2]: stalling horizontal
/// strip 0 on one worker must bound completed horizontal rows waiting in
/// `Driver::rows` so the decoder cannot advance arbitrarily far ahead of
/// vertical work.
#[test]
fn parallel_stalled_first_strip_bounds_decoder_advance() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

    // Row 0 carries sentinel `255`; all subsequent rows stay in `0..=250` so
    // `StallFirstRow` deterministically stalls source row 0.
    let mut data = vec![0_u8; 64 * 256 * 3];
    for y in 0..256_usize {
        let row = &mut data[y * 64 * 3..(y + 1) * 64 * 3];
        if y == 0 {
            row.fill(255);
        } else {
            let v = u8::try_from((y * 17) % 251).unwrap();
            row.fill(v);
        }
    }
    let base = TestSource::new(
        Size::new(64, 256).unwrap(),
        ChannelLayout::Rgb,
        SRGB,
        Samples::U8(data),
    );
    let mut linear_info = base.info().clone();
    linear_info.sample = SampleType::F32;
    linear_info.rendered_cicp = Some([1, 8, 0, 1]);
    let out_size = Size::new(64, 128).unwrap();
    let request = Rect::new(0, 0, 64, 128).unwrap();
    let kernels = Kernels::best();

    let delivered_seq = Arc::new(AtomicU32::new(0));
    let mut seq_src = DeferredTestSource {
        inner: base.clone(),
        info: linear_info.clone(),
        conv: Arc::new(StallFirstRow {
            stall_enabled: false,
            stalled: AtomicBool::new(false),
            delivered: delivered_seq.clone(),
            delivered_while_stalled: AtomicU32::new(0),
        }),
        delivered: Some(delivered_seq),
    };
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    let delivered_par = Arc::new(AtomicU32::new(0));
    let conv_par = Arc::new(StallFirstRow {
        stall_enabled: true,
        stalled: AtomicBool::new(false),
        delivered: delivered_par.clone(),
        delivered_while_stalled: AtomicU32::new(0),
    });
    let mut par_src = DeferredTestSource {
        inner: base,
        info: linear_info,
        conv: conv_par.clone(),
        delivered: Some(delivered_par),
    };
    let par_budget = MemoryBudget::new(64 << 20);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &par_budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 128);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(par_budget.snapshot().live, 0);
    assert!(conv_par.stalled.load(Ordering::Acquire));

    // With 2 workers, h_batch=8, max_h_in_flight=4, plan.capacity=2,
    // max_h_rows = 2 + 2 * 4 * 8 = 66. `pace_horizontal` counts retained,
    // in-flight, pending, deferred, and the incoming row before admitting it,
    // so at most 66 rows are admitted and the 67th row blocks in
    // `pace_horizontal` (strictly below 72).
    let delivered_while_stalled = conv_par.delivered_while_stalled.load(Ordering::Acquire);
    assert!(
        delivered_while_stalled <= 67,
        "decoder advanced to row {delivered_while_stalled} while strip 0 was stalled (expected <= 67)"
    );
}

struct FlippedTestSource(TestSource);

impl DecodeSource for FlippedTestSource {
    fn info(&self) -> &DecoderInfo {
        self.0.info()
    }
    fn orientation(&self) -> crate::orientation::Orientation {
        crate::orientation::Orientation::from_exif(4).unwrap()
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.0.read_region(region, budget, sink)
    }
}

struct DeferredOrientedSource<'a> {
    inner: crate::orientation::OrientedSource<'a>,
    info: DecoderInfo,
    conv: std::sync::Arc<dyn crate::decoder::RowConversion>,
}

impl DecodeSource for DeferredOrientedSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn defer_conversion(&mut self) -> Option<std::sync::Arc<dyn crate::decoder::RowConversion>> {
        Some(self.conv.clone())
    }
    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.inner.set_downstream_headroom(bytes);
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.inner.read_region(region, budget, sink)
    }
}

/// Regression for Codex PR #295 Round-5 finding 1 [P2]: when a final callback
/// cannot fit a horizontal `Row` while `OrientedSource` still holds its decoded
/// region (`64x128` flipped RGB -> `2048x1` Area, 2 workers, `7,510,000` bytes),
/// defer horizontal allocation until `read_region` returns and drops its buffers.
#[test]
fn parallel_flipped_defers_horizontal_row_allocation_past_read_region() {
    let base = make_rgb_pattern_source(64, 128);
    let out_size = Size::new(2048, 1).unwrap();
    let request = Rect::new(0, 0, 2048, 1).unwrap();
    let kernels = Kernels::best();

    let mut seq_src =
        crate::orientation::OrientedSource::new(Box::new(FlippedTestSource(base.clone())))
            .unwrap()
            .with_threads(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    for run in 0..10 {
        let mut par_src =
            crate::orientation::OrientedSource::new(Box::new(FlippedTestSource(base.clone())))
                .unwrap()
                .with_threads(2);
        let par_budget = MemoryBudget::new(7_510_000);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            2,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 1, "run={run}");
        assert_eq!(report.transform_threads, 2, "run={run}");
        assert_eq!(par_hash, seq_hash, "run={run}");
        assert_eq!(par_budget.snapshot().live, 0, "run={run}");
        assert!(report.peak_engine_managed_bytes <= 7_510_000, "run={run}");
    }
}

/// Regression for Codex PR #295 Round-5 finding 2 [P2] & Round-7 finding 1 [P2]:
/// `OrientedSource` pre-allocates and reuses its gather blocks before row
/// callbacks begin, so `2048x640 -> 1024x640` flipped RGB with 20 workers and
/// a 14 MB budget preserves multi-worker horizontal concurrency with zero
/// inter-callback gather allocations.
#[test]
fn parallel_flipped_gather_headroom_tracks_only_unreserved_blocks() {
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Arc, Condvar, Mutex};

    let base = make_rgb_pattern_source(2048, 640);
    let out_size = Size::new(1024, 640).unwrap();
    let request = Rect::new(0, 0, 1024, 640).unwrap();
    let kernels = Kernels::best();

    let oriented = crate::orientation::OrientedSource::new(Box::new(FlippedTestSource(base)))
        .unwrap()
        .with_threads(20);
    let mut linear_info = oriented.info().clone();
    linear_info.sample = SampleType::F32;
    linear_info.rendered_cicp = Some([1, 8, 0, 1]);

    let probe = Arc::new(ConcurrencyProbe {
        state: Mutex::new((0, 0)),
        changed: Condvar::new(),
        waited: AtomicUsize::new(0),
    });
    let mut src = DeferredOrientedSource {
        inner: oriented,
        info: linear_info,
        conv: probe.clone(),
    };
    let (graph, input, target) = graph_for_info(src.info(), None, out_size, Filter::Area);
    let budget = MemoryBudget::new(14_000_000);
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut src,
        request,
        &budget,
        &kernels,
        20,
        |y, _| {
            assert_eq!(y, rows);
            rows += 1;
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 640);
    assert_eq!(report.transform_threads, 20);
    assert_eq!(budget.snapshot().live, 0);
    assert!(report.peak_engine_managed_bytes <= 14_000_000);
    let max_active = probe.state.lock().unwrap().1;
    assert!(
        max_active >= 2,
        "expected >= 2 concurrent horizontal row conversions on flipped 2048x640 at 14 MB, got {max_active}"
    );
}

/// Regression for Codex PR #295 Round-7 finding 1 [P2]: pre-allocating and
/// reusing gather blocks in `OrientedSource::emit_parallel` eliminates any
/// double-counting race window (`500x65 -> 100x88` flipped Area, 5 workers,
/// `699,140`-byte budget).
#[test]
fn parallel_flipped_preallocated_gather_blocks_avoid_double_counting() {
    let base = make_rgb_pattern_source(500, 65);
    let out_size = Size::new(100, 88).unwrap();
    let request = Rect::new(0, 0, 100, 88).unwrap();
    let kernels = Kernels::best();

    let mut seq_src =
        crate::orientation::OrientedSource::new(Box::new(FlippedTestSource(base.clone())))
            .unwrap()
            .with_threads(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    for run in 0..10 {
        let mut par_src =
            crate::orientation::OrientedSource::new(Box::new(FlippedTestSource(base.clone())))
                .unwrap()
                .with_threads(5);
        let par_budget = MemoryBudget::new(699_140);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &par_budget,
            &kernels,
            5,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                par_hash = fnv1a64_update(par_hash, row);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(rows, 88, "run={run}");
        assert_eq!(report.transform_threads, 5, "run={run}");
        assert_eq!(par_hash, seq_hash, "run={run}");
        assert_eq!(par_budget.snapshot().live, 0, "run={run}");
        assert!(report.peak_engine_managed_bytes <= 699_140, "run={run}");
    }
}

/// Regression for Codex PR #295 Round-7 finding 2 [P3]: a sink returning
/// `Error::BudgetExceeded` during horizontal-allocation reclamation must
/// propagate immediately instead of being misinterpreted as a deferred
/// horizontal row allocation.
#[test]
fn parallel_propagates_sink_budget_exceeded_without_deferring() {
    let base = make_rgb_pattern_source(16, 128);
    let out_size = Size::new(4096, 128).unwrap();
    let request = Rect::new(0, 0, 4096, 128).unwrap();
    let (graph, input, target) = graph_for(&base, None, out_size, Filter::Area);
    let kernels = Kernels::best();

    let mut src = base;
    // Tight enough that horizontal row allocation enters reclamation and emits
    // finished rows via `sink`, while still allowing the first vertical output
    // to complete and invoke `sink`.
    let budget = MemoryBudget::new(6_894_192);
    let sentinel = Error::BudgetExceeded {
        requested: 12_345,
        available: 67,
    };
    let result = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut src,
        request,
        &budget,
        &kernels,
        2,
        |_, _| Err(sentinel.clone()),
    );
    assert_eq!(result, Err(sentinel));
    assert_eq!(budget.snapshot().live, 0);
}

struct TransposedTestSource(TestSource);

impl DecodeSource for TransposedTestSource {
    fn info(&self) -> &DecoderInfo {
        self.0.info()
    }
    fn orientation(&self) -> crate::orientation::Orientation {
        crate::orientation::Orientation::from_exif(6).unwrap()
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.0.read_region(region, budget, sink)
    }
}

/// Regression for Codex PR #295 Round-8 & Round-9 finding [P2]: `OrientedSource::emit_parallel`
/// must preserve `downstream_headroom` when pre-allocating optional gather blocks
/// so budget admission is strictly monotonic (`302x403` EXIF-6 transposing RGB ->
/// `403x302` display -> `100x302` Area, 6 threads => 2 gather + 4 transform workers:
/// if `min_budget` succeeds, every budget across all gather-block thresholds succeeds).
#[test]
fn parallel_flipped_gather_budget_admission_is_monotonic() {
    let base = make_rgb_pattern_source(302, 403);
    let out_size = Size::new(100, 302).unwrap();
    let request = Rect::new(0, 0, 100, 302).unwrap();
    let kernels = Kernels::best();

    let mut seq_src =
        crate::orientation::OrientedSource::new(Box::new(TransposedTestSource(base.clone())))
            .unwrap()
            .with_threads(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    let run_with_limit = |limit: usize| -> Result<u64, Error> {
        let mut par_src =
            crate::orientation::OrientedSource::new(Box::new(TransposedTestSource(base.clone())))
                .unwrap();
        let budget = MemoryBudget::new(limit);
        let mut hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let result = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &budget,
            &kernels,
            6,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                hash = fnv1a64_update(hash, row);
                Ok(())
            },
        );
        assert_eq!(budget.snapshot().live, 0);
        result.map(|_| {
            assert_eq!(rows, 302);
            hash
        })
    };

    // Binary-search the exact minimum budget that succeeds.
    let mut lo = 1_usize;
    let mut hi = 4 << 20;
    assert_eq!(run_with_limit(hi).unwrap(), seq_hash);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if run_with_limit(mid).is_ok() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let min_budget = lo;
    let block_bytes = 403 * 16 * 3;
    assert!(matches!(
        run_with_limit(min_budget - 1),
        Err(Error::BudgetExceeded { .. })
    ));
    let mut extras = vec![0, 1, 64];
    for k in 1..=4 {
        extras.extend([k * block_bytes - 1, k * block_bytes, k * block_bytes + 1]);
    }
    for extra in extras {
        let limit = min_budget + extra;
        assert_eq!(
            run_with_limit(limit).unwrap(),
            seq_hash,
            "budget={limit} (min_budget={min_budget} + {extra}) must succeed monotonically"
        );
    }
}

#[cfg(feature = "webp-read")]
struct TaggedWebp<'a> {
    inner: crate::codecs::webp::WebpSource<'a>,
    info: DecoderInfo,
}

#[cfg(feature = "webp-read")]
impl DecodeSource for TaggedWebp<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.inner.read_region(region, budget, sink)
    }
}

/// Regression for Codex PR #295 Round-9 finding 1 [P2]: `try_alloc_horiz_row`
/// must keep `reserved_output` intact during `read_region` so a +1 byte budget
/// increase cannot release `reserved_output`, let a horizontal row fit, and
/// starve vertical emission (`4x64` crop of `reference-lossy.webp` -> `1024x2`
/// Area, 2 workers, monotonic across `1,892,811`..`1,900,992` bytes).
#[cfg(feature = "webp-read")]
#[test]
fn parallel_webp_crop_reserved_output_stays_monotonic_across_deferral() {
    let webp = include_bytes!("../tests/fixtures/webp/reference-lossy.webp");
    let make_src = || {
        let mut raw = crate::codecs::webp::WebpSource::new_bounded(webp, 1_000_000).unwrap();
        let mut info = raw.info().clone();
        info.rendered_cicp = Some(SRGB);
        raw.set_decode_threads(1);
        TaggedWebp { inner: raw, info }
    };

    let crop = Rect::new(0, 0, 4, 64).unwrap();
    let out_size = Size::new(1024, 2).unwrap();
    let request = Rect::new(0, 0, 1024, 2).unwrap();
    let kernels = Kernels::best();

    let mut seq_src = make_src();
    let (graph, input, target) = graph_for_info(seq_src.info(), Some(crop), out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    let run_with_limit = |limit: usize| -> Result<u64, Error> {
        let mut par_src = make_src();
        let budget = MemoryBudget::new(limit);
        let mut hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let res = execute_fast_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &budget,
            &kernels,
            2,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                hash = fnv1a64_update(hash, row);
                Ok(())
            },
        );
        assert_eq!(budget.snapshot().live, 0);
        res.map(|_| {
            assert_eq!(rows, 2);
            hash
        })
    };

    for limit in [1_892_811_usize, 1_892_812, 1_892_813, 1_900_992] {
        assert_eq!(run_with_limit(limit).unwrap(), seq_hash, "limit={limit}");
    }
    let mut lo = 1_usize;
    let mut hi = 1_892_811_usize;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if run_with_limit(mid).is_ok() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let min_budget = lo;
    assert!(matches!(
        run_with_limit(min_budget - 1),
        Err(Error::BudgetExceeded { .. })
    ));
    for extra in [0, 1, 4096, 16_384] {
        let limit = min_budget + extra;
        assert_eq!(
            run_with_limit(limit).unwrap(),
            seq_hash,
            "budget={limit} (min_budget={min_budget} + {extra}) must succeed monotonically"
        );
    }
}

/// Regression for Codex PR #295 Round-9 finding 2 [P2]: `OrientedSource::emit_parallel`
/// must shrink its pre-allocated gather window to `1..=max_window` reusable
/// blocks when the budget fits stored region + 3 gather blocks (not 4), matching
/// `main`'s ADR-0019 upper-bound contract (transposing EXIF-6 `8192x64` and
/// `512x256` display RGB, 3 gather workers => `max_window = 4`).
#[test]
fn parallel_oriented_shrinks_gather_window_to_three_blocks_under_tight_budget() {
    struct TransposedSource(TestSource);
    impl DecodeSource for TransposedSource {
        fn info(&self) -> &DecoderInfo {
            self.0.info()
        }
        fn orientation(&self) -> crate::orientation::Orientation {
            crate::orientation::Orientation::from_exif(6).unwrap()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.0.read_region(region, budget, sink)
        }
    }

    for (display_w, display_h) in [(8192_u32, 64_u32), (512_u32, 256_u32)] {
        // Stored dimensions are transposed relative to display dimensions.
        let base = make_rgb_pattern_source(display_h, display_w);
        let region = Rect::new(0, 0, display_w, display_h).unwrap();

        let mut seq_src =
            crate::orientation::OrientedSource::new(Box::new(TransposedSource(base.clone())))
                .unwrap()
                .with_threads(1);
        let seq_budget = MemoryBudget::new(64 << 20);
        let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
        seq_src
            .read_region(region, &seq_budget, &mut |_, row| {
                let DecodedRow::U8(bytes) = row else {
                    return Err(Error::IncompleteDecoderOutput);
                };
                seq_hash = fnv1a64_update(seq_hash, bytes);
                Ok(())
            })
            .unwrap();
        assert_eq!(seq_budget.snapshot().live, 0);

        let region_bytes = (display_w as usize) * (display_h as usize) * 3;
        let block_bytes = (display_w as usize) * 16 * 3;
        // Budget fits stored region + 3 gather blocks, NOT 4 (`threads + 1`).
        let three_block_limit = region_bytes + 3 * block_bytes;
        let mut par_src = crate::orientation::OrientedSource::new(Box::new(TransposedSource(base)))
            .unwrap()
            .with_threads(3);
        let par_budget = MemoryBudget::new(three_block_limit);
        let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let report = par_src
            .read_region(region, &par_budget, &mut |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                let DecodedRow::U8(bytes) = row else {
                    return Err(Error::IncompleteDecoderOutput);
                };
                par_hash = fnv1a64_update(par_hash, bytes);
                Ok(())
            })
            .unwrap();
        assert_eq!(rows, display_h, "{display_w}x{display_h}");
        assert_eq!(par_hash, seq_hash, "{display_w}x{display_h}");
        assert_eq!(par_budget.snapshot().live, 0, "{display_w}x{display_h}");
        assert!(report.peak_engine_managed_bytes <= three_block_limit);
    }
}

/// Regression for Codex PR #295 Round-10 finding [P2]: `drain_deferred_horizontal`
/// must restore `reserved_output` once `read_region` drops decoder buffers and
/// protect it from lookahead deferred horizontal rows (`1x64` crop of
/// `reference-lossy.webp` -> `47x2` Area, 2 workers, 40/40 at `68,321` and
/// `68,396` bytes).
#[cfg(feature = "webp-read")]
#[test]
fn parallel_webp_crop_restores_reserved_output_before_draining_deferred() {
    let webp = include_bytes!("../tests/fixtures/webp/reference-lossy.webp");
    let make_src = || {
        let mut raw = crate::codecs::webp::WebpSource::new_bounded(webp, 1_000_000).unwrap();
        let mut info = raw.info().clone();
        info.rendered_cicp = Some(SRGB);
        raw.set_decode_threads(1);
        TaggedWebp { inner: raw, info }
    };

    let crop = Rect::new(0, 0, 1, 64).unwrap();
    let out_size = Size::new(47, 2).unwrap();
    let request = Rect::new(0, 0, 47, 2).unwrap();
    let kernels = Kernels::best();

    let mut seq_src = make_src();
    let (graph, input, target) = graph_for_info(seq_src.info(), Some(crop), out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        &kernels,
        1,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    for limit in [68_321_usize, 68_396] {
        for run in 0..40 {
            let mut par_src = make_src();
            let par_budget = MemoryBudget::new(limit);
            let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
            let mut rows = 0_u32;
            let report = execute_fast_srgb8(
                &graph,
                target,
                input,
                &mut par_src,
                request,
                &par_budget,
                &kernels,
                2,
                |y, row| {
                    assert_eq!(y, rows);
                    rows += 1;
                    par_hash = fnv1a64_update(par_hash, row);
                    Ok(())
                },
            )
            .unwrap();
            assert_eq!(rows, 2, "limit={limit} run={run}");
            assert_eq!(report.transform_threads, 2, "limit={limit} run={run}");
            assert_eq!(par_hash, seq_hash, "limit={limit} run={run}");
            assert_eq!(par_budget.snapshot().live, 0, "limit={limit} run={run}");
            assert!(
                report.peak_engine_managed_bytes <= limit,
                "limit={limit} run={run}"
            );
        }
    }
}

/// Regression for Codex PR #295 Round-11 finding [P2]: `execute_quality`
/// (`WorkingPrecision::F64Reference`) must report its peak horizontal `f64`
/// cache via `DecodeSource::set_downstream_headroom` so `OrientedSource`
/// parallel gather preserves strict budget monotonicity (`64x64` transposing
/// EXIF-6 RGB -> `32x64` Area, 3 gather workers => `max_window = 4`, monotonic
/// across `min_budget`, `175,600`, `177,648`, and every gather-block threshold).
#[test]
fn quality_executor_flipped_gather_budget_admission_is_monotonic() {
    let base = make_rgb_pattern_source(64, 64);
    let out_size = Size::new(32, 64).unwrap();
    let request = Rect::new(0, 0, 32, 64).unwrap();

    let mut seq_src =
        crate::orientation::OrientedSource::new(Box::new(TransposedTestSource(base.clone())))
            .unwrap()
            .with_threads(1);
    let (graph, input, target) = graph_for_info(seq_src.info(), None, out_size, Filter::Area);
    let seq_budget = MemoryBudget::new(64 << 20);
    let mut seq_hash = 0xCBF2_9CE4_8422_2325_u64;
    execute_quality_srgb8(
        &graph,
        target,
        input,
        &mut seq_src,
        request,
        &seq_budget,
        |_, row| {
            seq_hash = fnv1a64_update(seq_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(seq_budget.snapshot().live, 0);

    let run_with_limit = |limit: usize| -> Result<u64, Error> {
        let mut par_src =
            crate::orientation::OrientedSource::new(Box::new(TransposedTestSource(base.clone())))
                .unwrap()
                .with_threads(3);
        let budget = MemoryBudget::new(limit);
        let mut hash = 0xCBF2_9CE4_8422_2325_u64;
        let mut rows = 0_u32;
        let res = execute_quality_srgb8(
            &graph,
            target,
            input,
            &mut par_src,
            request,
            &budget,
            |y, row| {
                assert_eq!(y, rows);
                rows += 1;
                hash = fnv1a64_update(hash, row);
                Ok(())
            },
        );
        assert_eq!(budget.snapshot().live, 0);
        res.map(|_| {
            assert_eq!(rows, 64);
            hash
        })
    };

    let mut lo = 1_usize;
    let mut hi = 4 << 20;
    assert_eq!(run_with_limit(hi).unwrap(), seq_hash);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if run_with_limit(mid).is_ok() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let min_budget = lo;
    let block_bytes = 64 * 16 * 3;
    assert!(matches!(
        run_with_limit(min_budget - 1),
        Err(Error::BudgetExceeded { .. })
    ));
    let mut limits = vec![min_budget, min_budget + 1, 175_600, 177_648];
    for k in 1..=4 {
        limits.extend([
            min_budget + k * block_bytes - 1,
            min_budget + k * block_bytes,
            min_budget + k * block_bytes + 1,
        ]);
    }
    for limit in limits {
        assert_eq!(
            run_with_limit(limit).unwrap(),
            seq_hash,
            "budget={limit} (min_budget={min_budget}) must succeed monotonically"
        );
    }
}

/// Regression for Codex PR #295 Round-6 finding 1 [P2]: when decoder-held
/// buffers block horizontal row allocation (`outstanding == 0`, `deferred_h`
/// growing), `pace_horizontal` must refuse with `BudgetExceeded` instead of
/// queuing past `max_h_rows`.
#[test]
fn parallel_deferred_horizontal_enforces_max_h_rows_cap() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct HeldDecoderBuffer {
        inner: TestSource,
        leave_bytes: usize,
        delivered: Arc<AtomicU32>,
    }
    impl DecodeSource for HeldDecoderBuffer {
        fn info(&self) -> &DecoderInfo {
            self.inner.info()
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            // Leave room for dozens of 48-byte source row copies in `deferred_h`
            // but not for a single 65,536-byte horizontal `Row`.
            let hold_bytes = budget.available().saturating_sub(self.leave_bytes);
            let _held = budget.allocate(hold_bytes)?;
            let delivered = self.delivered.clone();
            self.inner.read_region(region, budget, &mut |y, row| {
                delivered.fetch_add(1, Ordering::Release);
                sink(y, row)
            })
        }
    }

    let base = make_rgb_pattern_source(16, 128);
    let out_size = Size::new(4096, 128).unwrap();
    let request = Rect::new(0, 0, 4096, 128).unwrap();
    let (graph, input, target) = graph_for(&base, None, out_size, Filter::Area);
    let kernels = Kernels::best();

    // With 2 workers, h_batch=8, max_h_in_flight=4, plan.capacity=1,
    // max_h_rows = 65; rows 0..65 defer into `deferred_h`, and row 65 (the 66th
    // row) must fail in `pace_horizontal` with `BudgetExceeded` instead of
    // queueing all 128 source rows.
    let delivered = Arc::new(AtomicU32::new(0));
    let mut src = HeldDecoderBuffer {
        inner: base,
        leave_bytes: 32_768,
        delivered: delivered.clone(),
    };
    let budget = MemoryBudget::new(8 << 20);
    let result = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut src,
        request,
        &budget,
        &kernels,
        2,
        |_, _| Ok(()),
    );
    assert!(
        matches!(result, Err(Error::BudgetExceeded { .. })),
        "expected BudgetExceeded past max_h_rows, got {result:?}"
    );
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(
        delivered.load(Ordering::Acquire),
        66,
        "deferred_h must admit at most max_h_rows=65 rows and refuse the 66th"
    );
}

/// Regression for Codex PR #295 Round-6 finding 1 [P2] with the exact WebP
/// `16x128` crop -> `4096x128` 2-worker reproducer at `6,817,392` bytes.
#[cfg(feature = "webp-read")]
#[test]
fn parallel_webp_crop_deferred_horizontal_refuses_past_max_h_rows() {
    struct TaggedWebp<'a> {
        inner: crate::codecs::webp::WebpSource<'a>,
        info: DecoderInfo,
    }
    impl DecodeSource for TaggedWebp<'_> {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn read_region(
            &mut self,
            region: Rect,
            budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            self.inner.read_region(region, budget, sink)
        }
    }

    let webp = include_bytes!("../tests/fixtures/webp/reference-lossy.webp");
    let mut raw = crate::codecs::webp::WebpSource::new_bounded(webp, 1_000_000).unwrap();
    let mut info = raw.info().clone();
    info.rendered_cicp = Some(SRGB);
    raw.set_decode_threads(1);
    let mut src = TaggedWebp { inner: raw, info };
    let crop = Rect::new(0, 0, 16, 128).unwrap();
    let out_size = Size::new(4096, 128).unwrap();
    let request = Rect::new(0, 0, 4096, 128).unwrap();
    let (graph, input, target) = graph_for_info(src.info(), Some(crop), out_size, Filter::Area);
    let budget = MemoryBudget::new(6_817_392);
    let result = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut src,
        request,
        &budget,
        &Kernels::best(),
        2,
        |_, _| Ok(()),
    );
    assert!(
        matches!(result, Err(Error::BudgetExceeded { .. })),
        "expected BudgetExceeded on tight WebP crop -> 4096x128, got {result:?}"
    );
    assert_eq!(budget.snapshot().live, 0);
}

/// Regression for Codex PR #295 Round-6 finding 2 [P2]: steep vertical
/// downscales (`4x16_000 -> 4x1` Area, 16,000 taps on output row 0) must stay
/// linear-time (`O(1)` `pace_horizontal` and readiness checks) and match `t=1`.
#[test]
fn parallel_steep_vertical_downscale_stays_linear_and_byte_identical() {
    let base = make_rgb_pattern_source(4, 16_000);
    let out_size = Size::new(4, 1).unwrap();
    let request = Rect::new(0, 0, 4, 1).unwrap();
    let kernels = Kernels::best();
    let (seq_hash, _) = sequential_hash_and_peak(&base, out_size, Filter::Area, &kernels);

    let (graph, input, target) = graph_for(&base, None, out_size, Filter::Area);
    let mut par_src = base;
    let budget = MemoryBudget::new(64 << 20);
    let mut par_hash = 0xCBF2_9CE4_8422_2325_u64;
    let mut rows = 0_u32;
    let report = execute_fast_srgb8(
        &graph,
        target,
        input,
        &mut par_src,
        request,
        &budget,
        &kernels,
        2,
        |y, row| {
            assert_eq!(y, rows);
            rows += 1;
            par_hash = fnv1a64_update(par_hash, row);
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(rows, 1);
    assert_eq!(report.transform_threads, 2);
    assert_eq!(par_hash, seq_hash);
    assert_eq!(budget.snapshot().live, 0);
}

/// FNV-1a 64-bit digest for fast byte-identity checks across large benchmark outputs.
fn fnv1a64_update(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

/// Benchmark parallel row pipeline scaling (`1..=20` threads), peak managed memory,
/// `#210` peak variance, and scoped thread spawn overhead.
///
/// Run with:
/// `cargo test --release -p ferrisma --lib --features webp-read,webp-write bench_parallel_pipeline_scaling -- --ignored --nocapture`
#[test]
#[ignore = "manual benchmark for issues #59 and #210"]
#[allow(clippy::too_many_lines)] // Comprehensive scaling, variance, and thread-pool benchmark report.
fn bench_parallel_pipeline_scaling() {
    use std::time::{Duration, Instant};

    const THREAD_COUNTS: &[usize] = &[1, 2, 4, 8, 12, 16, 20];
    let kernels = Kernels::best();
    println!(
        "Parallel pipeline scaling benchmark (level={}, available_parallelism={:?}, best of 3 after warmup):",
        kernels.level(),
        std::thread::available_parallelism().ok()
    );

    // Deterministic patterned source buffer (fast to build, non-trivial pixel values).
    let make_u8_source = |w: u32, h: u32, layout: ChannelLayout| -> TestSource {
        let ch = usize::from(layout.channels());
        let row_bytes = w as usize * ch;
        let mut data = vec![0_u8; row_bytes * h as usize];
        for y in 0..h as usize {
            let row = &mut data[y * row_bytes..(y + 1) * row_bytes];
            let seed = u8::try_from((y * 37) & 0xFF).unwrap();
            for (i, slot) in row.iter_mut().enumerate() {
                let v = seed.wrapping_add(u8::try_from((i * 13) & 0xFF).unwrap());
                *slot = if ch == 4 && i % 4 == 3 { v.max(1) } else { v };
            }
        }
        TestSource::new(Size::new(w, h).unwrap(), layout, SRGB, Samples::U8(data))
    };

    let bench_cases = [
        (
            "24MP 6000x4000->1600x1067 RGB",
            6000,
            4000,
            ChannelLayout::Rgb,
            1600,
            1067,
            Filter::Lanczos3,
            false,
        ),
        (
            "24MP 6000x4000->4000x2667 RGB",
            6000,
            4000,
            ChannelLayout::Rgb,
            4000,
            2667,
            Filter::Lanczos3,
            false,
        ),
        (
            "24MP 6000x4000->1600x1067 RGBA+Probe",
            6000,
            4000,
            ChannelLayout::Rgba,
            1600,
            1067,
            Filter::Lanczos3,
            true,
        ),
        (
            "48MP 8000x6000->2000x1500 RGB",
            8000,
            6000,
            ChannelLayout::Rgb,
            2000,
            1500,
            Filter::Lanczos3,
            false,
        ),
        (
            "Upscale 1000x1000->4000x4000 RGB",
            1000,
            1000,
            ChannelLayout::Rgb,
            4000,
            4000,
            Filter::Lanczos3,
            false,
        ),
        (
            "Steep vertical 64x16000->64x1 RGB Area",
            64,
            16_000,
            ChannelLayout::Rgb,
            64,
            1,
            Filter::Area,
            false,
        ),
    ];

    for (name, sw, sh, layout, dw, dh, filter, with_probe) in bench_cases {
        let base_source = make_u8_source(sw, sh, layout);
        let out_size = Size::new(dw, dh).unwrap();
        let (graph, input, target) = graph_for_info(base_source.info(), None, out_size, filter);
        let request = Rect::new(0, 0, dw, dh).unwrap();
        println!("\n### {name}");
        println!("| threads | transform_ms | total_ms | speedup_x | peak_managed_bytes | hash |");
        println!("| ---: | ---: | ---: | ---: | ---: | --- |");

        let mut t1_transform_ms = 0.0_f64;
        let mut expected_hash = None;

        for &threads in THREAD_COUNTS {
            let mut best_transform = Duration::MAX;
            let mut best_total = Duration::MAX;
            let mut peak_bytes = 0_usize;
            let mut run_hash = 0_u64;

            for iter in 0..4 {
                let budget = MemoryBudget::new(2 << 30);
                let mut hash = 0xCBF2_9CE4_8422_2325_u64;
                let check_hash = iter == 0;
                let (report, total) = if with_probe {
                    let found = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                    let mut probed = crate::discarded::HiddenColorProbe::new(
                        Box::new(base_source.clone()),
                        found,
                    );
                    let started = Instant::now();
                    let report = execute_fast_srgb8(
                        &graph,
                        target,
                        input,
                        &mut probed,
                        request,
                        &budget,
                        &kernels,
                        threads,
                        |_, row| {
                            if check_hash {
                                hash = fnv1a64_update(hash, row);
                            } else {
                                std::hint::black_box(row);
                            }
                            Ok(())
                        },
                    )
                    .unwrap();
                    (report, started.elapsed())
                } else {
                    let mut src = base_source.clone();
                    let started = Instant::now();
                    let report = execute_fast_srgb8(
                        &graph,
                        target,
                        input,
                        &mut src,
                        request,
                        &budget,
                        &kernels,
                        threads,
                        |_, row| {
                            if check_hash {
                                hash = fnv1a64_update(hash, row);
                            } else {
                                std::hint::black_box(row);
                            }
                            Ok(())
                        },
                    )
                    .unwrap();
                    (report, started.elapsed())
                };
                assert_eq!(budget.snapshot().live, 0);
                if check_hash {
                    if let Some(exp) = expected_hash {
                        assert_eq!(
                            hash, exp,
                            "{name} threads={threads} diverged from sequential output!"
                        );
                    } else {
                        expected_hash = Some(hash);
                    }
                    run_hash = hash;
                } else if report.transform_time < best_transform {
                    best_transform = report.transform_time;
                    best_total = total;
                    peak_bytes = report.peak_engine_managed_bytes;
                }
            }
            let tr_ms = best_transform.as_secs_f64() * 1_000.0;
            let tot_ms = best_total.as_secs_f64() * 1_000.0;
            if threads == 1 {
                t1_transform_ms = tr_ms;
            }
            let speedup = t1_transform_ms / tr_ms;
            println!(
                "| {threads} | {tr_ms:.2} | {tot_ms:.2} | {speedup:.2}x | {peak_bytes} | {run_hash:016x} |"
            );
        }
    }

    // Issue #210: measure `peak_managed_bytes` variance across 20 repeated runs.
    println!("\n### Issue #210 peak_managed_bytes variance (20 repeated runs)");
    #[cfg(all(feature = "webp-read", feature = "webp-write"))]
    {
        use crate::pipeline::{
            ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings, Threads,
            TransformOptions, WebpOptions, transcode,
        };
        use std::num::NonZeroUsize;

        let webp = include_bytes!("../tests/fixtures/webp/reference-lossy.webp");
        for threads in [4_usize, 8, 20] {
            let options = TransformOptions::new(OutputGeometry::Exact {
                width: 96,
                height: 96,
            })
            .with_output(OutputSettings::WebP(WebpOptions::default()))
            .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
            .with_transform_threads(Threads::Count(NonZeroUsize::new(threads).unwrap()));
            let mut peaks = Vec::with_capacity(20);
            for _ in 0..20 {
                let out = transcode(webp, ImageFormat::WebP, &options).unwrap();
                peaks.push(out.report.peak_managed_bytes);
            }
            let min = *peaks.iter().min().unwrap();
            let max = *peaks.iter().max().unwrap();
            let mut unique = peaks.clone();
            unique.sort_unstable();
            unique.dedup();
            println!(
                "WebP 128x128->96x96 threads={threads}: min={min} max={max} spread={} distinct={}",
                max - min,
                unique.len()
            );
        }
    }

    // Also measure upscale 20-run peak variance (100x100 -> 1000x1000).
    {
        let up_src = make_u8_source(100, 100, ChannelLayout::Rgb);
        let up_size = Size::new(1000, 1000).unwrap();
        let (graph, input, target) = graph_for_info(up_src.info(), None, up_size, Filter::Lanczos3);
        let request = Rect::new(0, 0, 1000, 1000).unwrap();
        for threads in [4_usize, 8, 20] {
            let mut peaks = Vec::with_capacity(20);
            for _ in 0..20 {
                let budget = MemoryBudget::new(2 << 30);
                let mut src = up_src.clone();
                let report = execute_fast_srgb8(
                    &graph,
                    target,
                    input,
                    &mut src,
                    request,
                    &budget,
                    &kernels,
                    threads,
                    |_, _| Ok(()),
                )
                .unwrap();
                assert_eq!(budget.snapshot().live, 0);
                peaks.push(report.peak_engine_managed_bytes);
            }
            let min = *peaks.iter().min().unwrap();
            let max = *peaks.iter().max().unwrap();
            let mut unique = peaks.clone();
            unique.sort_unstable();
            unique.dedup();
            println!(
                "Upscale 100x100->1000x1000 threads={threads}: min={min} max={max} spread={} distinct={}",
                max - min,
                unique.len()
            );
        }
    }

    // Thread spawn/join overhead benchmark for issue #59 pool evaluation.
    println!("\n### Thread spawn/join overhead (100 iterations x 20 threads)");
    let spawn_start = Instant::now();
    for _ in 0..100 {
        std::thread::scope(|scope| {
            for _ in 0..20 {
                scope.spawn(|| {});
            }
        });
    }
    let spawn_elapsed = spawn_start.elapsed();
    println!(
        "bare thread::scope (20 threads x 100 calls): total={:.2} ms ({:.1} us/call)",
        spawn_elapsed.as_secs_f64() * 1_000.0,
        spawn_elapsed.as_secs_f64() * 1_000_000.0 / 100.0
    );

    // Small/medium image 100 calls on 20 threads vs 1 thread.
    for (label, sw, sh, dw, dh) in [
        ("small 400x300->200x150", 400, 300, 200, 150),
        ("medium 1200x800->600x400", 1200, 800, 600, 400),
    ] {
        let src = make_u8_source(sw, sh, ChannelLayout::Rgb);
        let out_size = Size::new(dw, dh).unwrap();
        let (graph, input, target) = graph_for_info(src.info(), None, out_size, Filter::Lanczos3);
        let request = Rect::new(0, 0, dw, dh).unwrap();
        for threads in [1_usize, 8, 20] {
            let start = Instant::now();
            for _ in 0..100 {
                let budget = MemoryBudget::new(256 << 20);
                let mut s = src.clone();
                let _ = execute_fast_srgb8(
                    &graph,
                    target,
                    input,
                    &mut s,
                    request,
                    &budget,
                    &kernels,
                    threads,
                    |_, _| Ok(()),
                )
                .unwrap();
            }
            let elapsed = start.elapsed();
            println!(
                "{label} x100 threads={threads}: total={:.2} ms ({:.1} us/call)",
                elapsed.as_secs_f64() * 1_000.0,
                elapsed.as_secs_f64() * 1_000_000.0 / 100.0
            );
        }
    }
}
