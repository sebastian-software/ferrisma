//! Differential tests: the `f32` executor against the `f64` oracle, using the
//! acceptance rules of ADR-0021 section 4, plus the contracts both executors
//! share (one decoder request, released memory, cancellation).

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::fast_execute::{execute_fast, execute_fast_srgb8, tables};
use crate::geometry::{Rect, Size};
use crate::graph::{Graph, NodeId};
use crate::memory::MemoryBudget;
use crate::output_depth::{OutputRow, max_code};
use crate::quality::Filter;
use crate::quality_execute::{execute_quality, execute_quality_srgb8};
use crate::working_color::{encode_linear_srgb, encode_linear_srgb8};
use ferrisma_kernels::Kernels;

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
                    for threads in [2, 3, 8] {
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
