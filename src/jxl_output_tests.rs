//! JPEG XL output (ADR-0039): admitted-profile errors, budget accounting and
//! independent round trips. The admitted profile is lossless, so every
//! decoded sample must equal the finished sRGB8 row the sink received:
//! the pixel error bound is zero, not a tolerance.
// Test images are small; their coordinates and samples fit every cast.
#![allow(clippy::cast_possible_truncation)]

use crate::error::Error;
use crate::geometry::Size;
use crate::jxl_sink::{
    JxlSink, LOSSLESS_PEAK, known_codec_frame_bytes, lossless_codec_reservation_bytes,
    lossy_codec_reservation_bytes,
};
use crate::memory::MemoryBudget;
use crate::pipeline::{AvifOptions, JpegXlLossless, JpegXlLossy, OutputGeometry, OutputSettings};
use crate::pipeline::{ChromaSubsampling, JpegXlOptions, ProcessingErrorKind, TransformOptions};

/// Lossless output options; the lossy tests are in `jxl_lossy_tests`.
fn options(width: u32, height: u32) -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact { width, height }).with_output(
        OutputSettings::JpegXl(JpegXlOptions::Lossless(JpegXlLossless::default())),
    )
}

/// Deterministic photo-like content: gradients plus a hashed texture, and
/// (optionally) straight alpha that includes zero with nonzero color.
fn rows(width: usize, height: usize, alpha: bool) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let hash = (x as u32).wrapping_mul(2_654_435_761) ^ (y as u32).wrapping_mul(40_503);
            let noise = (hash >> 13) as u8 & 0x1f;
            data.extend_from_slice(&[
                ((x * 255) / width.max(2)) as u8 ^ noise,
                ((y * 255) / height.max(2)) as u8,
                ((x + y) * 7) as u8 ^ (noise << 2),
                if !alpha {
                    255
                } else if x == 0 {
                    0
                } else {
                    ((x * 13 + y * 5) % 256) as u8
                },
            ]);
        }
    }
    data
}

fn encode(
    width: usize,
    height: usize,
    data: &[u8],
    budget: &MemoryBudget,
) -> Result<Vec<u8>, Error> {
    let size = Size::new(width as u32, height as u32).unwrap();
    let mut sink = JxlSink::new(size, &options(size.width, size.height), budget)?;
    for (y, row) in data.chunks_exact(width * 4).enumerate() {
        sink.write_row(y as u32, row)?;
    }
    Ok(sink.finish()?.bytes)
}

#[test]
fn rejects_requests_outside_the_admitted_profile_before_allocating() {
    let budget = MemoryBudget::new(1 << 20);
    let size = Size::new(8, 8).unwrap();
    for (changed, expected) in [
        (
            options(8, 8).with_output(OutputSettings::JpegXl(JpegXlOptions::Lossless(
                JpegXlLossless::default().with_effort(7),
            ))),
            "effort",
        ),
        (options(8, 8).with_encoder_threads(2), "serial"),
    ] {
        let error = JxlSink::new(size, &changed, &budget).err().unwrap();
        assert!(matches!(error, Error::UnsupportedEncoderProfile(_)));
        assert!(error.to_string().contains(expected), "{error}");
    }
    // Another output's settings never reach this sink; it refuses them.
    let chroma = options(8, 8).with_output(OutputSettings::Avif(
        AvifOptions::default().with_chroma(ChromaSubsampling::Yuv420),
    ));
    assert_eq!(
        JxlSink::new(size, &chroma, &budget).err(),
        Some(Error::InvalidEncodeOptions)
    );
    // Level 5 limits: 2^18 per side and 2^28 pixels.
    let wide = Size::new((1 << 18) + 1, 1).unwrap();
    assert!(matches!(
        JxlSink::new(wide, &options(wide.width, 1), &budget).err(),
        Some(Error::UnsupportedEncoderProfile(_))
    ));
    let area = Size::new(1 << 15, (1 << 13) + 1).unwrap();
    assert!(matches!(
        JxlSink::new(area, &options(area.width, area.height), &budget).err(),
        Some(Error::UnsupportedEncoderProfile(_))
    ));
    assert_eq!(budget.snapshot().peak, 0);
}

/// The lossless sink reserves the measured peak of the path the encoder
/// takes (issue #223, ADR-0004, ADR-0039).
///
/// **Contract changes, not refreshed pins.** Until 2026-11-13 the sink
/// reserved only the planes known from the source path (about 28 B/px with
/// the frame) against a measured 70-76 B/px; from 2026-11-13 it reserved the
/// measured one-shot peak (76 B/px, 4/3 of it with alpha, derived). From
/// 2026-11-15 it encoded with the default buffering, chunked above 8 groups,
/// and reserved the peaks measured for that path
/// (`docs/benchmarks/v1.0-jxl-lossless-chunked-memory-2026-11-15.md`). Since
/// 2026-11-16 the model has a term for the second DC group and is fitted to
/// 10 % above every peak on record
/// (`docs/benchmarks/v1.0-jxl-encoder-memory-2026-11-16.md`,
/// `model_covers_every_measured_peak_with_margin`); the 2026-11-15
/// measurements below must still fit.
#[test]
#[allow(clippy::too_many_lines)] // One contract: formula pins, recorded peaks, budget boundaries.
fn input_frame_and_measured_lossless_peaks_are_reserved() {
    let (width, height) = (9, 5);
    let pixels = width * height;
    let frame = pixels * 4;
    // The source-path floor: Image3<f32> padded to 16x8 plus three i32
    // Modular channels, plus two alpha planes. A regression pin of the
    // formula, not evidence of the allocations.
    let known = known_codec_frame_bytes(width, height, false).unwrap();
    assert_eq!(known, 16 * 8 * 3 * 4 + pixels * 3 * 4);
    assert_eq!(
        known_codec_frame_bytes(width, height, true).unwrap(),
        known + pixels * 4 * 2
    );
    // One-shot (at most 8 groups): the per-pixel term and the constant,
    // minus the engine's frame (4 B/px), which the measured process held.
    let [[opaque_model, alpha_model], _] = LOSSLESS_PEAK;
    let opaque = lossless_codec_reservation_bytes(width, height, false, 8).unwrap();
    assert_eq!(
        opaque,
        pixels * opaque_model.one_shot + opaque_model.constant - frame
    );
    assert!(opaque > known);
    let alpha = lossless_codec_reservation_bytes(width, height, true, 8).unwrap();
    assert_eq!(
        alpha,
        pixels * alpha_model.one_shot + alpha_model.constant - frame
    );
    // Streaming (more than 8 groups): the frame term, the first DC group,
    // the second DC group's retained term, the constant, minus the frame.
    let (w, h) = (6000, 4000);
    let dc = 2048 * 2048;
    assert_eq!(
        lossless_codec_reservation_bytes(w, h, false, 8).unwrap(),
        w * h * opaque_model.frame
            + dc * (opaque_model.dc_group + opaque_model.retained)
            + opaque_model.constant
            - w * h * 4
    );
    // Between one and two DC groups the retained term counts the pixels
    // beyond the first group.
    let (w, h) = (2304, 2048);
    assert_eq!(
        lossless_codec_reservation_bytes(w, h, true, 8).unwrap(),
        w * h * alpha_model.frame
            + dc * alpha_model.dc_group
            + (w * h - dc) * alpha_model.retained
            + alpha_model.constant
            - w * h * 4
    );
    // The 2026-11-15 measurements (RSS of a fresh process, the larger of
    // two runs, the port's public encoder with the default buffering,
    // holding the input once at 3 or 4 B/px) fit in the engine's frame
    // plus reservation. The one-shot 24 MP peak of the previous path
    // (1,751 MB, 2026-11-15) is far above the new one.
    for (w, h, alpha, measured) in [
        (512, 1024, false, 44_990_464),
        (512, 1024, true, 59_375_616),
        (700, 700, false, 32_440_320),
        (700, 700, true, 39_206_912),
        (1024, 1024, false, 65_437_696),
        (1024, 1024, true, 85_000_192),
        (2048, 2048, false, 219_217_920),
        (2048, 2048, true, 293_699_584),
        (3000, 2000, false, 308_609_024),
        (3000, 2000, true, 400_621_568),
        (4032, 3024, false, 402_882_560),
        (4032, 3024, true, 516_194_304),
        (6000, 4000, false, 477_724_672),
        (6000, 4000, true, 693_764_096),
    ] {
        let reserved = w * h * 4 + lossless_codec_reservation_bytes(w, h, alpha, 8).unwrap();
        assert!(
            reserved >= measured,
            "{w}x{h} alpha={alpha}: {reserved} < {measured}"
        );
    }
    assert!(
        6000 * 4000 * 4 + lossless_codec_reservation_bytes(6000, 4000, false, 8).unwrap()
            < 1_751_056_384
    );

    // Samples above 8 bits (2026-11-15, `v1.0-jxl-deep-memory-2026-11-15.md`):
    // 16-bit photographs (libavif's `weld_16bit.png`, mirrored to size),
    // the same method; the engine's frame is 8 B/px. Measured, not scaled
    // from the 8-bit peaks. (That record's tool held one input copy more
    // than the sink, so these peaks are high by 4 or 8 B/px; they fit
    // anyway.)
    for (w, h, alpha, measured) in [
        (512, 1024, false, 58_572_800),
        (512, 1024, true, 68_403_200),
        (2048, 2048, false, 307_642_368),
        (2048, 2048, true, 407_977_984),
        (3000, 2000, false, 400_752_640),
        (3000, 2000, true, 544_817_152),
        (4032, 3024, false, 612_188_160),
        (4032, 3024, true, 817_496_064),
        (6000, 4000, false, 907_329_536),
        (6000, 4000, true, 1_192_198_144),
    ] {
        for depth in [10, 12, 16] {
            let reserved =
                w * h * 8 + lossless_codec_reservation_bytes(w, h, alpha, depth).unwrap();
            assert!(
                reserved >= measured,
                "{w}x{h} alpha={alpha} {depth}-bit: {reserved} < {measured}"
            );
        }
    }

    let data = rows(width, height, false);
    assert!(matches!(
        encode(width, height, &data, &MemoryBudget::new(frame - 1)),
        Err(Error::BudgetExceeded { .. })
    ));
    let short = MemoryBudget::new(frame + opaque - 1);
    assert!(matches!(
        encode(width, height, &data, &short),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(short.snapshot().live, 0);
    assert_eq!(short.snapshot().peak, frame);
    let exact = MemoryBudget::new(frame + opaque);
    encode(width, height, &data, &exact).unwrap();
    assert_eq!(exact.snapshot().peak, frame + opaque);
    assert_eq!(exact.snapshot().live, 0);
    // Transparent rows need the alpha reservation.
    let data = rows(width, height, true);
    assert!(encode(width, height, &data, &MemoryBudget::new(frame + opaque)).is_err());
    encode(width, height, &data, &MemoryBudget::new(frame + alpha)).unwrap();
}

/// The recorded peaks of `model_covers_every_measured_peak_with_margin`.
const MEASURED_PEAKS: &[(&str, usize, usize, bool, bool, u8, usize)] = &[
    ("2026-11-15 c700x700", 700, 700, true, false, 8, 32_930_320),
    ("n8", 700, 700, true, false, 8, 33_964_464),
    ("2026-11-15 weld8", 512, 1024, true, false, 8, 45_776_896),
    ("n8", 512, 1024, true, false, 8, 52_707_328),
    (
        "2026-11-15 c1024x1024",
        1024,
        1024,
        true,
        false,
        8,
        66_486_272,
    ),
    (
        "2026-11-15 c2048x2048",
        2048,
        2048,
        true,
        false,
        8,
        223_412_224,
    ),
    ("2026-11-15 weld8", 2048, 2048, true, false, 8, 227_164_160),
    ("n8", 2048, 2048, true, false, 8, 257_376_256),
    ("n8", 2304, 2048, true, false, 8, 282_640_384),
    (
        "2026-11-15 c3000x2000",
        3000,
        2000,
        true,
        false,
        8,
        314_609_024,
    ),
    ("2026-11-15 weld8", 3000, 2000, true, false, 8, 309_421_312),
    ("n8", 3000, 2000, true, false, 8, 343_588_864),
    (
        "2026-11-15 photo12",
        4032,
        3024,
        true,
        false,
        8,
        415_075_328,
    ),
    ("2026-11-15 weld8", 4032, 3024, true, false, 8, 402_892_800),
    ("n8", 4032, 3024, true, false, 8, 495_910_912),
    (
        "2026-11-15 photo24",
        6000,
        4000,
        true,
        false,
        8,
        501_724_672,
    ),
    ("2026-11-15 weld8", 6000, 4000, true, false, 8, 564_548_608),
    ("n8", 6000, 4000, true, false, 8, 625_770_496),
    ("n16", 700, 700, true, false, 16, 40_550_400),
    ("2026-11-15 weld16", 512, 1024, true, false, 16, 56_475_648),
    ("n16", 512, 1024, true, false, 16, 64_339_968),
    (
        "2026-11-15 weld16",
        2048,
        2048,
        true,
        false,
        16,
        290_865_152,
    ),
    ("n16", 2048, 2048, true, false, 16, 322_191_360),
    ("n16", 2304, 2048, true, false, 16, 361_938_944),
    (
        "2026-11-15 weld16",
        3000,
        2000,
        true,
        false,
        16,
        376_752_640,
    ),
    ("n16", 3000, 2000, true, false, 16, 454_557_696),
    (
        "2026-11-15 weld16",
        4032,
        3024,
        true,
        false,
        16,
        563_417_088,
    ),
    ("n16", 4032, 3024, true, false, 16, 636_715_008),
    (
        "2026-11-15 weld16",
        6000,
        4000,
        true,
        false,
        16,
        811_329_536,
    ),
    ("n16", 6000, 4000, true, false, 16, 919_470_080),
    ("2026-11-15 c700x700", 700, 700, true, true, 8, 39_206_912),
    ("n8", 700, 700, true, true, 8, 44_679_168),
    ("2026-11-15 weld8", 512, 1024, true, true, 8, 56_950_784),
    ("n8", 512, 1024, true, true, 8, 68_157_440),
    (
        "2026-11-15 c1024x1024",
        1024,
        1024,
        true,
        true,
        8,
        85_000_192,
    ),
    (
        "2026-11-15 c2048x2048",
        2048,
        2048,
        true,
        true,
        8,
        293_699_584,
    ),
    ("2026-11-15 weld8", 2048, 2048, true, true, 8, 289_079_296),
    ("n8", 2048, 2048, true, true, 8, 332_316_672),
    ("n8", 2304, 2048, true, true, 8, 374_931_456),
    (
        "2026-11-15 c3000x2000",
        3000,
        2000,
        true,
        true,
        8,
        400_621_568,
    ),
    ("2026-11-15 weld8", 3000, 2000, true, true, 8, 394_447_360),
    ("n8", 3000, 2000, true, true, 8, 444_907_520),
    ("p", 3000, 2000, true, true, 8, 407_175_168),
    ("w8", 3000, 2000, true, true, 8, 410_402_816),
    ("2026-11-15 photo12", 4032, 3024, true, true, 8, 516_194_304),
    ("2026-11-15 weld8", 4032, 3024, true, true, 8, 562_630_656),
    ("n8", 4032, 3024, true, true, 8, 609_091_584),
    ("p", 4032, 3024, true, true, 8, 570_818_560),
    ("w8", 4032, 3024, true, true, 8, 564_379_648),
    ("2026-11-15 photo24", 6000, 4000, true, true, 8, 693_764_096),
    ("2026-11-15 weld8", 6000, 4000, true, true, 8, 595_208_192),
    ("n8", 6000, 4000, true, true, 8, 797_556_736),
    ("p", 6000, 4000, true, true, 8, 653_885_440),
    ("w8", 6000, 4000, true, true, 8, 656_490_496),
    ("n16", 700, 700, true, true, 16, 52_969_472),
    ("2026-11-15 weld16", 512, 1024, true, true, 16, 64_208_896),
    ("n16", 512, 1024, true, true, 16, 75_628_544),
    ("2026-11-15 weld16", 2048, 2048, true, true, 16, 374_423_552),
    ("n16", 2048, 2048, true, true, 16, 409_960_448),
    ("n16", 2304, 2048, true, true, 16, 460_783_616),
    ("2026-11-15 weld16", 3000, 2000, true, true, 16, 496_817_152),
    ("n16", 3000, 2000, true, true, 16, 553_861_120),
    ("2026-11-15 weld16", 4032, 3024, true, true, 16, 719_953_920),
    ("n16", 4032, 3024, true, true, 16, 765_378_560),
    ("w16", 4032, 3024, true, true, 16, 740_573_184),
    (
        "2026-11-15 weld16",
        6000,
        4000,
        true,
        true,
        16,
        1_000_198_144,
    ),
    ("n16", 6000, 4000, true, true, 16, 1_163_821_056),
    ("n8", 700, 700, false, false, 8, 59_162_624),
    ("n8", 512, 1024, false, false, 8, 66_715_648),
    ("n8", 2048, 2048, false, false, 8, 440_582_144),
    ("n8", 2304, 2048, false, false, 8, 475_758_592),
    ("n8", 3000, 2000, false, false, 8, 520_830_976),
    ("n8", 4032, 3024, false, false, 8, 602_030_080),
    ("n8", 6000, 4000, false, false, 8, 648_496_088),
    ("n16", 700, 700, false, false, 16, 63_750_144),
    ("n16", 512, 1024, false, false, 16, 68_091_904),
    ("n16", 2048, 2048, false, false, 16, 470_663_168),
    ("n16", 2304, 2048, false, false, 16, 510_050_304),
    ("n16", 3000, 2000, false, false, 16, 539_672_576),
    ("n16", 4032, 3024, false, false, 16, 719_422_496),
    ("n16", 6000, 4000, false, false, 16, 810_090_496),
    ("n8", 700, 700, false, true, 8, 65_519_616),
    ("n8", 512, 1024, false, true, 8, 87_097_344),
    ("n8", 2048, 2048, false, true, 8, 492_388_352),
    ("w8", 2048, 2048, false, true, 8, 402_030_592),
    ("n8", 2304, 2048, false, true, 8, 534_102_016),
    ("n8", 3000, 2000, false, true, 8, 603_766_784),
    ("p", 3000, 2000, false, true, 8, 504_184_832),
    ("w8", 3000, 2000, false, true, 8, 502_251_520),
    ("n8", 4032, 3024, false, true, 8, 709_804_032),
    ("p", 4032, 3024, false, true, 8, 542_130_176),
    ("w8", 4032, 3024, false, true, 8, 609_746_944),
    ("n8", 6000, 4000, false, true, 8, 707_150_856),
    ("p", 6000, 4000, false, true, 8, 612_876_288),
    ("w8", 6000, 4000, false, true, 8, 618_348_544),
    ("n16", 700, 700, false, true, 16, 68_255_744),
    ("n16", 512, 1024, false, true, 16, 85_803_008),
    ("n16", 2048, 2048, false, true, 16, 544_194_560),
    ("n16", 2304, 2048, false, true, 16, 587_972_608),
    ("n16", 3000, 2000, false, true, 16, 611_745_792),
    ("n16", 4032, 3024, false, true, 16, 746_030_088),
    ("w16", 4032, 3024, false, true, 16, 645_447_680),
    ("n16", 6000, 4000, false, true, 16, 1_003_880_448),
];

/// The engine's JPEG XL memory model against every peak on record
/// (ADR-0004, ADR-0042 D3): for each measured case, the engine's frame plus
/// the codec reservation is at least 10 % above the measured peak, so that
/// a change of the model cannot silently drop below a measurement.
///
/// Rows: content, width, height, lossless, alpha, bits, peak bytes. The
/// peak is the largest maximum resident set size of the case's accepted
/// runs (fresh processes, one at a time, macOS `AArch64`; the peak footprint
/// of a rejected run counts as a sample too); lossy rows are the largest
/// over the distances measured (0.05, 1.0, 2.9). Contents: `p` photograph
/// crops (`photo24` of 2026-11-15), `w8` and `w16` libavif's
/// `weld_16bit.png` mirrored to size at 8 and 16 bits, `n8` and `n16`
/// uniform noise, `2026-11-15 …` the earlier records compared at the
/// engine's frame. Source: `docs/benchmarks/evidence-2026-11-16-encoder-memory/`
/// (`tools/fit.py --rust` prints these rows from its results).
#[test]
fn model_covers_every_measured_peak_with_margin() {
    let measured = MEASURED_PEAKS;
    for &(content, w, h, lossless, alpha, depth, peak) in measured {
        let frame = w * h * 4 * if depth > 8 { 2 } else { 1 };
        let codec = if lossless {
            lossless_codec_reservation_bytes(w, h, alpha, depth)
        } else {
            lossy_codec_reservation_bytes(w, h, alpha, depth)
        }
        .unwrap();
        let reserved = frame + codec;
        assert!(
            reserved * 10 >= peak * 11,
            "{content} {w}x{h} lossless={lossless} alpha={alpha} {depth}-bit: \
             {reserved} reserved, {peak} measured"
        );
    }
}

#[test]
fn incomplete_or_misordered_rows_are_rejected() {
    let budget = MemoryBudget::new(1 << 20);
    let size = Size::new(4, 2).unwrap();
    let mut sink = JxlSink::new(size, &options(4, 2), &budget).unwrap();
    assert!(sink.write_row(1, &[0; 16]).is_err());
    sink.write_row(0, &[0; 16]).unwrap();
    assert!(sink.write_row(1, &[0; 12]).is_err());
    assert!(sink.finish().is_err());
}

/// The original's public tool is the byte reference of the lossless sink
/// (issue #249, ADR-0039): each fixture was written by the pinned libjxl's
/// own `cjxl -d 0 -e 2 --num_threads=0 -x color_space=RGB_D65_SRG_Rel_SRG`
/// with its default buffering from these `rows` as PPM or PAM
/// (`tests/fixtures/jxl-lossless/README.md`). The sink must write the same
/// file, byte for byte: this is the original's output, not a pin of the
/// port's. Since 2026-11-15 (issue #223) the sink encodes like that default:
/// the 2049x40 patterns (9 groups) take the original's chunked path, whose
/// bytes differ from `--buffering=0`; the three smaller files have at most 8
/// groups, where both commands write the same bytes, and are unchanged.
#[test]
fn lossless_output_equals_the_original_cjxl_bytes() {
    for (width, height, alpha, reference) in [
        (
            37,
            19,
            false,
            &include_bytes!("../tests/fixtures/jxl-lossless/rgb-37x19.jxl")[..],
        ),
        (
            37,
            19,
            true,
            &include_bytes!("../tests/fixtures/jxl-lossless/rgba-37x19.jxl")[..],
        ),
        (
            300,
            260,
            false,
            &include_bytes!("../tests/fixtures/jxl-lossless/rgb-300x260.jxl")[..],
        ),
        (
            2049,
            40,
            false,
            &include_bytes!("../tests/fixtures/jxl-lossless/rgb-2049x40.jxl")[..],
        ),
        (
            2049,
            40,
            true,
            &include_bytes!("../tests/fixtures/jxl-lossless/rgba-2049x40.jxl")[..],
        ),
    ] {
        let budget = MemoryBudget::new(64 << 20);
        let encoded = encode(width, height, &rows(width, height, alpha), &budget).unwrap();
        assert!(
            encoded == reference,
            "{width}x{height} alpha={alpha}: {} bytes, cjxl wrote {}",
            encoded.len(),
            reference.len()
        );
    }
}

/// A profile the workspace encoder does not admit and a thread count it
/// cannot honor are refused before decoding. (Until 2026-11-16 this test
/// was `facade_rejects_mismatched_and_disabled_settings_before_decoding`
/// and also refused AVIF and WebP settings beside JPEG XL output, and JPEG
/// XL settings beside WebP output; the options can no longer express
/// those.)
#[test]
fn facade_rejects_unadmitted_settings_before_decoding() {
    // Invalid compressed input proves the check precedes opening a decoder.
    let garbage = b"not an image";
    for options in [
        options(8, 8).with_output(OutputSettings::JpegXl(JpegXlOptions::Lossless(
            JpegXlLossless::default().with_effort(3),
        ))),
        options(8, 8).with_encoder_threads(0),
    ] {
        let error =
            crate::pipeline::transcode(garbage, crate::pipeline::ImageFormat::JpegXl, &options)
                .unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions, "{error}");
    }
}

/// Cross-checks with jxl-oxide, an independent decoder that is a test-only
/// dev-dependency since 2026-11-15 (no longer a reader of the engine).
mod independent_decode {
    use jxl_oxide::color::{
        ColourEncoding, ColourSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
    };
    use jxl_oxide::image::BitDepth;
    use jxl_oxide::{ExtraChannelType, JxlImage};

    use super::{encode, rows};
    use crate::memory::MemoryBudget;

    /// Decode with jxl-oxide, check the emitted signaling and return
    /// interleaved 8-bit samples plus the channel count.
    pub(super) fn decode_oxide(bytes: &[u8]) -> (u32, u32, usize, Vec<u8>) {
        let image = JxlImage::builder()
            .read(std::io::Cursor::new(bytes))
            .unwrap();
        let metadata = &image.image_header().metadata;
        assert!(!metadata.xyb_encoded, "lossless samples are not XYB");
        assert_eq!(metadata.orientation, 1);
        assert_eq!(
            metadata.bit_depth,
            BitDepth::IntegerSample { bits_per_sample: 8 }
        );
        let ColourEncoding::Enum(color) = &metadata.colour_encoding else {
            panic!("expected enum color, found an ICC profile")
        };
        assert_eq!(color.colour_space, ColourSpace::Rgb);
        assert_eq!(color.white_point, WhitePoint::D65);
        assert_eq!(color.primaries, Primaries::Srgb);
        assert_eq!(color.tf, TransferFunction::Srgb);
        assert_eq!(color.rendering_intent, RenderingIntent::Relative);
        for ec in &metadata.ec_info {
            assert_eq!(
                ec.ty,
                ExtraChannelType::Alpha {
                    alpha_associated: false
                }
            );
            assert_eq!(ec.bit_depth, BitDepth::IntegerSample { bits_per_sample: 8 });
        }
        let channels = 3 + metadata.ec_info.len();
        let render = image.render_frame(0).unwrap();
        let mut stream = render.stream();
        assert_eq!(stream.channels() as usize, channels);
        let mut samples = vec![0_u8; (stream.width() * stream.height()) as usize * channels];
        stream.write_to_buffer(&mut samples);
        (image.width(), image.height(), channels, samples)
    }

    pub(super) fn to_rgba(channels: usize, samples: &[u8]) -> Vec<u8> {
        if channels == 4 {
            return samples.to_vec();
        }
        samples
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255])
            .collect()
    }

    #[test]
    fn sink_round_trips_exactly_through_jxl_oxide() {
        // Odd sizes, every x mod 8 boundary, and a 2x2 group layout.
        for (width, height) in [(1, 1), (9, 17), (37, 19), (300, 260)] {
            for alpha in [false, true] {
                let data = rows(width, height, alpha);
                let bytes = encode(width, height, &data, &MemoryBudget::new(64 << 20)).unwrap();
                assert_eq!(&bytes[..2], &[0xff, 0x0a], "raw codestream signature");
                let (w, h, channels, samples) = decode_oxide(&bytes);
                assert_eq!((w as usize, h as usize), (width, height));
                // A one-pixel image with x == 0 has alpha 0, so it keeps alpha.
                assert_eq!(channels, if alpha { 4 } else { 3 });
                assert_eq!(
                    to_rgba(channels, &samples),
                    data,
                    "{width}x{height} {alpha}"
                );
            }
        }
    }

    #[cfg(feature = "jpeg-xl-read")]
    #[test]
    fn sink_round_trips_exactly_through_the_workspace_decoder() {
        for alpha in [false, true] {
            let (width, height) = (37, 19);
            let data = rows(width, height, alpha);
            let bytes = encode(width, height, &data, &MemoryBudget::new(64 << 20)).unwrap();
            let info = ferrisma_jxl_decoder::inspect(&bytes, 1 << 20).unwrap();
            assert_eq!((info.width, info.height), (37, 19));
            assert_eq!(info.channels, if alpha { 4 } else { 3 });
            assert!(!info.alpha_premultiplied);
            let mut output = vec![0_f32; info.sample_count().unwrap()];
            ferrisma_jxl_decoder::decode_into(&bytes, &info, &mut output, 1 << 20).unwrap();
            // 8-bit lossless samples decode to k/255 exactly up to float
            // rounding; rounding back must recover every sample.
            let samples: Vec<u8> = output
                .iter()
                .map(|&v| {
                    let scaled = v * 255.0;
                    assert!((scaled - scaled.round()).abs() < 1e-3, "{v}");
                    assert!((0.0..=255.0).contains(&scaled), "{v}");
                    #[allow(clippy::cast_sign_loss)] // Range checked above.
                    let sample = scaled.round() as u8;
                    sample
                })
                .collect();
            assert_eq!(to_rgba(info.channels as usize, &samples), data);
        }
    }
}

#[cfg(feature = "webp-read")]
mod pipeline_round_trip {
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

    use super::independent_decode::{decode_oxide, to_rgba};
    use super::{options, rows};
    use crate::pipeline::{
        EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
        OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions, transcode, transcode_with,
    };

    /// Captures the finished rows the built-in sink would receive.
    struct Rows;
    struct Captured(Vec<u8>);
    impl EncoderBackend for Rows {
        fn format(&self) -> OutputFormat {
            OutputFormat::JpegXl
        }
        fn name(&self) -> &'static str {
            "rgba-reference"
        }
        fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            Ok(Box::new(Captured(Vec::new())))
        }
    }
    impl RowEncoder for Captured {
        fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
            self.0.extend_from_slice(rgba);
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
            Ok(EncodedImage::new(OutputFormat::JpegXl, self.0, 0, false))
        }
    }

    fn webp(width: usize, height: usize, alpha: bool) -> Vec<u8> {
        encode_lossless(PixelRows {
            layout: PixelLayout::Rgba,
            data: &rows(width, height, alpha),
            width: width as u32,
            height: height as u32,
            stride: width * 4,
        })
        .unwrap()
    }

    fn srgb(width: u32, height: u32) -> TransformOptions {
        options(width, height).with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    }

    #[test]
    fn transcoded_jpeg_xl_decodes_to_the_finished_rows() {
        // A downscale (resampled, quantized rows) and a same-size transcode.
        for (alpha, (width, height)) in [(false, (120, 90)), (true, (120, 90)), (true, (64, 48))] {
            let input = webp(120, 90, alpha);
            let output = transcode(&input, ImageFormat::WebP, &srgb(width, height)).unwrap();
            assert_eq!(output.format, OutputFormat::JpegXl);
            assert_eq!(output.report.encoder_backend, "libjxl-rs");
            assert!(output.report.opaque_encoder_memory);
            let (w, h) = (width as usize, height as usize);
            assert!(output.report.encoder_output_frame_bytes >= w * h * 4);
            let expected =
                transcode_with(&input, ImageFormat::WebP, &srgb(width, height), &Rows).unwrap();
            let (dw, dh, channels, samples) = decode_oxide(&output.bytes);
            assert_eq!((dw, dh), (width, height));
            assert_eq!(to_rgba(channels, &samples), expected.bytes);
            // Opaque finished rows drop the alpha channel; transparent keep it.
            assert_eq!(channels == 4, expected.bytes.chunks(4).any(|p| p[3] != 255));
        }
    }

    #[test]
    fn encoded_output_limit_applies_at_the_exact_byte() {
        let input = webp(40, 30, true);
        let full = transcode(&input, ImageFormat::WebP, &srgb(40, 30)).unwrap();
        let exact = srgb(40, 30).with_max_output_bytes(full.bytes.len());
        assert_eq!(
            transcode(&input, ImageFormat::WebP, &exact).unwrap().bytes,
            full.bytes
        );
        let short = srgb(40, 30).with_max_output_bytes(full.bytes.len() - 1);
        assert_eq!(
            transcode(&input, ImageFormat::WebP, &short)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::OutputLimitExceeded
        );
    }

    #[cfg(feature = "jpeg-read")]
    #[test]
    fn oriented_input_is_written_upright_with_orientation_one() {
        // EXIF orientation 6 rotates the stored 24x16 pattern by 90 degrees.
        let input = include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg");
        let reference = include_bytes!("../tests/fixtures/orientation/jpeg-6.rgb");
        let options = srgb(16, 24);
        let output = transcode(input, ImageFormat::Jpeg, &options).unwrap();
        let (w, h, channels, samples) = decode_oxide(&output.bytes);
        assert_eq!((w, h, channels), (16, 24, 3));
        let expected = transcode_with(input, ImageFormat::Jpeg, &options, &Rows).unwrap();
        assert_eq!(to_rgba(channels, &samples), expected.bytes);
        // The independent libjpeg upright reference matches exactly on this
        // fixture (measured 2026-11-12); the fixture README allows up to 2
        // between JPEG decoders, so a future decoder change may relax this.
        assert_eq!(&samples[..], &reference[..]);
    }
}

// ---------------------------------------------------------------------------
// Depths above 8 bits (ADR-0021 amendment of 2026-11-15, owner decision).

/// The pattern of `tests/fixtures/jxl-deep/rows16.py` at `depth`, RGBA.
fn deep_rows(width: usize, height: usize, depth: u8, alpha: bool) -> Vec<u16> {
    let m = (1_u32 << depth) - 1;
    let mut data = Vec::with_capacity(width * height * 4);
    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
            let noise = (hash >> 13) & 0x3f;
            let a = if !alpha {
                m
            } else if x == 0 {
                0
            } else {
                (x * 13 * (m / 255) + y * 5) % (m + 1)
            };
            data.extend(
                [
                    ((x * m) / (width as u32 - 1).max(1)) ^ noise,
                    (y * m) / (height as u32 - 1).max(1),
                    ((x + y) * 37) & m,
                    a,
                ]
                .map(|v| v as u16),
            );
        }
    }
    data
}

fn encode_deep(
    width: usize,
    height: usize,
    depth: u8,
    data: &[u16],
    jpeg_xl: JpegXlOptions,
) -> Result<Vec<u8>, Error> {
    let budget = MemoryBudget::new(256 << 20);
    let mut sink = JxlSink::with_depth(
        Size::new(width as u32, height as u32)?,
        &options(width as u32, height as u32).with_output(OutputSettings::JpegXl(jpeg_xl)),
        &budget,
        depth,
    )?;
    for (y, row) in data.chunks(width * 4).enumerate() {
        sink.write_output_row(y as u32, crate::output_depth::OutputRow::U16(row))?;
    }
    Ok(sink.finish()?.bytes)
}

/// Deep rows give the original's bytes, lossless and lossy: each fixture was
/// written by the pinned `cjxl` from `rows16.py` as PPM or PAM at
/// `MAXVAL 2^depth - 1` (`tests/fixtures/jxl-deep/README.md`). Lossless 16-bit
/// color and 16-bit alpha are containers (codestream level 10), as `cjxl`
/// writes them; `d16-2049x40-a1` takes the chunked path.
#[test]
#[allow(clippy::too_many_lines)] // One table of nine original files.
fn deep_output_equals_the_original_cjxl_bytes() {
    let lossless = JpegXlOptions::Lossless(JpegXlLossless::default());
    let lossy = JpegXlOptions::Lossy(JpegXlLossy::default().with_distance(1.0));
    for (width, height, depth, alpha, mode, reference) in [
        (
            37,
            19,
            16,
            false,
            lossless.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d16-37x19-a0-lossless.jxl")[..],
        ),
        (
            37,
            19,
            16,
            true,
            lossless.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d16-37x19-a1-lossless.jxl")[..],
        ),
        (
            37,
            19,
            10,
            false,
            lossless.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d10-37x19-a0-lossless.jxl")[..],
        ),
        (
            37,
            19,
            12,
            true,
            lossless.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d12-37x19-a1-lossless.jxl")[..],
        ),
        (
            2049,
            40,
            16,
            true,
            lossless.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d16-2049x40-a1-lossless.jxl")[..],
        ),
        (
            37,
            19,
            16,
            false,
            lossy.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d16-37x19-a0-d1.jxl")[..],
        ),
        (
            37,
            19,
            10,
            false,
            lossy.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d10-37x19-a0-d1.jxl")[..],
        ),
        (
            37,
            19,
            12,
            true,
            lossy.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d12-37x19-a1-d1.jxl")[..],
        ),
        (
            2049,
            40,
            10,
            false,
            lossy.clone(),
            &include_bytes!("../tests/fixtures/jxl-deep/d10-2049x40-a0-d1.jxl")[..],
        ),
    ] {
        let encoded = encode_deep(
            width,
            height,
            depth,
            &deep_rows(width, height, depth, alpha),
            mode.clone(),
        )
        .unwrap();
        assert!(
            encoded == reference,
            "{width}x{height} {depth}-bit alpha={alpha} {mode:?}: {} bytes, cjxl wrote {}",
            encoded.len(),
            reference.len()
        );
        let container = encoded.starts_with(&[0, 0, 0, 0x0c, b'J', b'X', b'L', b' ']);
        // Level 10: lossless color or any alpha above 12 bits.
        let level10 = depth > 12 && (alpha || matches!(mode, JpegXlOptions::Lossless { .. }));
        assert_eq!(
            container, level10,
            "{width}x{height} {depth}-bit alpha={alpha}"
        );
    }
}

/// A 16-bit lossless JPEG XL in, a lossless JPEG XL out at the default
/// (follow the input): the samples come back exactly, and nothing is
/// reported. An 8-bit bottleneck would keep 256 of the 65,536 levels.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn a_sixteen_bit_lossless_round_trip_is_exact() {
    use crate::pipeline::{Discarded, ImageFormat, InputProfilePolicy, transcode};
    let (width, height) = (37_usize, 19_usize);
    for (alpha, input) in [
        (
            false,
            &include_bytes!("../tests/fixtures/jxl-deep/d16-37x19-a0-lossless.jxl")[..],
        ),
        (
            true,
            &include_bytes!("../tests/fixtures/jxl-deep/d16-37x19-a1-lossless.jxl")[..],
        ),
    ] {
        let output = transcode(
            input,
            ImageFormat::JpegXl,
            &options(width as u32, height as u32).with_profile_policy(InputProfilePolicy::Strict),
        )
        .unwrap();
        assert!(
            !output
                .report
                .discarded
                .iter()
                .any(|c| c == Discarded::PrecisionReduced),
            "alpha={alpha}"
        );
        // The engine's own sink, fed the source pattern at 16 bits, writes
        // the input file (the cjxl reference above); a transcode at identity
        // that preserved every sample therefore writes it too, with color
        // under zero alpha dropped (transparent black, ADR-0014).
        let mut expected = deep_rows(width, height, 16, alpha);
        for pixel in expected.chunks_mut(4) {
            if pixel[3] == 0 {
                pixel[..3].fill(0);
            }
        }
        let reference = encode_deep(
            width,
            height,
            16,
            &expected,
            JpegXlOptions::Lossless(JpegXlLossless::default()),
        )
        .unwrap();
        assert!(output.bytes == reference, "alpha={alpha}: samples changed");
    }
}
