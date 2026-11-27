//! What a successful transcode leaves out of a correct picture (ADR-0042
//! D2): the classes of `ProcessingReport::discarded`.

use std::fmt;

/// A class of input information that a successful transcode did not carry
/// into the output and that leaves the delivered picture correct under the
/// documented contract (ADR-0042 D2). Information whose absence would make
/// the picture wrong is refused instead (D1).
///
/// Experimental (ADR-0040); the names and class boundaries are a working
/// decision 2026-11-15 (delegated, revisable by the owner). New classes may
/// be added; a class keeps its meaning. Each class is reported only when it
/// applied to this input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Discarded {
    /// A gain map was not applied and the base image was read: an AVIF
    /// tone-map item (`tmap`), a JPEG XL `jhgm` box, or a JPEG MPF
    /// secondary image (in practice an Ultra HDR or Apple gain map; the
    /// engine does not open the secondary image to tell).
    GainMap,
    /// EXIF data (the orientation in it may have been applied; the data is
    /// not written to the output).
    Exif,
    /// XMP data, including extended XMP.
    Xmp,
    /// Other metadata: IPTC, comments, JUMBF/C2PA, other JPEG application
    /// segments, a non-default JFIF density, AVIF `mime` or `uri ` items that
    /// are not XMP.
    OtherMetadata,
    /// An embedded ICC profile. Its meaning is carried: the samples were
    /// converted and the output signals its own color (sRGB); the profile
    /// itself is not written.
    IccProfile,
    /// Container data the format lets a reader ignore and the engine does
    /// not interpret: unknown non-essential AVIF properties or `colr` types
    /// on an item the reader uses (for example `pasp`, `clli`, custom
    /// properties), unknown WebP chunks, unknown JPEG XL boxes.
    UnknownContainerData,
    /// JPEG XL JPEG-reconstruction data (`jbrd`): the pixels were decoded;
    /// bit-exact recovery of the original JPEG is not possible from the
    /// output.
    JpegReconstructionData,
    /// A JPEG XL preview frame.
    Preview,
    /// Color under fully transparent pixels: at least one input pixel of the
    /// processed region had alpha exactly zero and a nonzero color sample;
    /// the output stores transparent black there (ADR-0014).
    HiddenColor,
    /// Premultiplied (associated) alpha was converted to straight alpha by
    /// the decoder's own step: libavif's integer division for AVIF `prem`,
    /// libjxl's float one for JPEG XL `alpha_associated` (since
    /// 2026-11-15). Color at low alpha loses precision, and the two steps
    /// round differently (`docs/supported-matrix.md`, format parity).
    Unpremultiplied,
    /// The output has fewer bits per sample than the input
    /// ([`ProcessingReport::output_depth`] below `input_depth`): an explicit
    /// lower depth, an output that admits less (WebP 8, AVIF 12), or an
    /// external backend whose row depths stop below the input's. Until
    /// 2026-11-15 every output was 8-bit, so every deeper input reported it.
    ///
    /// [`ProcessingReport::output_depth`]: crate::pipeline::ProcessingReport::output_depth
    PrecisionReduced,
}

impl Discarded {
    /// Every class in report order.
    const ORDER: [Self; 11] = [
        Self::GainMap,
        Self::Exif,
        Self::Xmp,
        Self::OtherMetadata,
        Self::IccProfile,
        Self::UnknownContainerData,
        Self::JpegReconstructionData,
        Self::Preview,
        Self::HiddenColor,
        Self::Unpremultiplied,
        Self::PrecisionReduced,
    ];

    fn bit(self) -> u16 {
        1 << (self as u16)
    }
}

/// The classes of [`Discarded`] that applied to one transcode, without
/// duplicates and in a stable order (the declaration order of
/// [`Discarded`]). Empty when nothing was left out.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DiscardedSet {
    bits: u16,
}

impl DiscardedSet {
    /// No class applied.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// Whether `class` applied.
    #[must_use]
    pub fn contains(self, class: Discarded) -> bool {
        self.bits & class.bit() != 0
    }

    /// The classes that applied, in report order.
    pub fn iter(self) -> impl Iterator<Item = Discarded> {
        Discarded::ORDER
            .into_iter()
            .filter(move |class| self.contains(*class))
    }

    pub(crate) fn insert(&mut self, class: Discarded) {
        self.bits |= class.bit();
    }

    pub(crate) fn insert_if(&mut self, condition: bool, class: Discarded) {
        if condition {
            self.insert(class);
        }
    }
}

impl fmt::Debug for DiscardedSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_set().entries(self.iter()).finish()
    }
}

impl FromIterator<Discarded> for DiscardedSet {
    fn from_iter<I: IntoIterator<Item = Discarded>>(classes: I) -> Self {
        let mut set = Self::default();
        for class in classes {
            set.insert(class);
        }
        set
    }
}

/// A pass-through source that notices input pixels whose alpha is exactly
/// zero while a color sample is not, in the rows it delivers: their color
/// is cleared to transparent black on output ([`Discarded::HiddenColor`]).
/// Wraps the codec source of an RGBA or gray-with-alpha layout, before any reorientation or
/// color conversion, so it sees the decoder's own samples of the region
/// that is processed.
pub(crate) struct HiddenColorProbe<'a> {
    source: Box<dyn crate::decoder::DecodeSource + 'a>,
    found: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<'a> HiddenColorProbe<'a> {
    pub(crate) fn new(
        source: Box<dyn crate::decoder::DecodeSource + 'a>,
        found: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self { source, found }
    }
}

/// Branchless chunk size (pixels) for auto-vectorized hidden-color scans:
/// keeps the inner loop straight-line for NEON/AVX2 while still exiting early
/// on rows that contain hidden color.
const PROBE_CHUNK_PIXELS: usize = 64;

fn has_hidden_color_u8<const N: usize>(samples: &[u8]) -> bool {
    for chunk in samples.as_chunks::<N>().0.chunks(PROBE_CHUNK_PIXELS) {
        let mut found = 0_u8;
        for pixel in chunk {
            let mut color = 0_u8;
            let mut channel = 0;
            while channel + 1 < N {
                color |= pixel[channel];
                channel += 1;
            }
            let mask = u8::from(pixel[N - 1] == 0).wrapping_neg();
            found |= mask & color;
        }
        if found != 0 {
            return true;
        }
    }
    false
}

/// Bit-exact IEEE-754 binary32 hidden-color scan:
/// - `x == 0.0` iff `(x.to_bits() & 0x7FFF_FFFF) == 0` (`+0.0` and `-0.0` only;
///   `NaN`, infinities, subnormals, and normals are all non-zero);
/// - `x != 0.0` iff `(x.to_bits() & 0x7FFF_FFFF) != 0` (including `NaN`,
///   infinities, and subnormals).
fn has_hidden_color_f32<const N: usize>(samples: &[f32]) -> bool {
    const MAG_MASK: u32 = 0x7FFF_FFFF;
    for chunk in samples.as_chunks::<N>().0.chunks(PROBE_CHUNK_PIXELS) {
        let mut found = 0_u32;
        for pixel in chunk {
            let mut color = 0_u32;
            let mut channel = 0;
            while channel + 1 < N {
                color |= pixel[channel].to_bits();
                channel += 1;
            }
            let mask = u32::from((pixel[N - 1].to_bits() & MAG_MASK) == 0).wrapping_neg();
            found |= mask & (color & MAG_MASK);
        }
        if found != 0 {
            return true;
        }
    }
    false
}

/// A pixel of an RGBA (`channels == 4`) or gray-with-alpha (2) row whose
/// alpha is exactly zero while a color sample is not.
fn has_hidden_color(row: crate::decoder::DecodedRow<'_>, channels: usize) -> bool {
    use crate::decoder::DecodedRow;
    match (row, channels) {
        (DecodedRow::U8(samples), 4) => has_hidden_color_u8::<4>(samples),
        (DecodedRow::U8(samples), 2) => has_hidden_color_u8::<2>(samples),
        (DecodedRow::F32(samples), 4) => has_hidden_color_f32::<4>(samples),
        (DecodedRow::F32(samples), 2) => has_hidden_color_f32::<2>(samples),
        (DecodedRow::U8(samples), _) => samples.chunks_exact(channels).any(|pixel| {
            pixel[channels - 1] == 0 && pixel[..channels - 1].iter().any(|&value| value != 0)
        }),
        (DecodedRow::F32(samples), _) => samples.chunks_exact(channels).any(|pixel| {
            pixel[channels - 1] == 0.0 && pixel[..channels - 1].iter().any(|&value| value != 0.0)
        }),
    }
}

impl crate::decoder::DecodeSource for HiddenColorProbe<'_> {
    fn info(&self) -> &crate::decoder::DecoderInfo {
        self.source.info()
    }

    fn orientation(&self) -> crate::orientation::Orientation {
        self.source.orientation()
    }

    fn defer_conversion(&mut self) -> Option<std::sync::Arc<dyn crate::decoder::RowConversion>> {
        self.source.defer_conversion()
    }

    fn prepare_conversion(
        &mut self,
        expected_pixels: u64,
        budget: &crate::memory::MemoryBudget,
    ) -> Result<(), crate::error::Error> {
        self.source.prepare_conversion(expected_pixels, budget)
    }

    fn set_decode_threads(&mut self, threads: usize) {
        self.source.set_decode_threads(threads);
    }

    fn discarded(&self) -> DiscardedSet {
        self.source.discarded()
    }

    fn streams_rows(&self) -> bool {
        self.source.streams_rows()
    }

    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.source.set_downstream_headroom(bytes);
    }

    fn read_region(
        &mut self,
        region: crate::geometry::Rect,
        budget: &crate::memory::MemoryBudget,
        sink: &mut dyn FnMut(
            u32,
            crate::decoder::DecodedRow<'_>,
        ) -> Result<(), crate::error::Error>,
    ) -> Result<crate::decoder::DecodeReport, crate::error::Error> {
        let found = &self.found;
        let channels = usize::from(self.source.info().layout.channels());
        self.source.read_region(region, budget, &mut |y, row| {
            if !found.load(std::sync::atomic::Ordering::Relaxed) && has_hidden_color(row, channels)
            {
                found.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            sink(y, row)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Discarded, DiscardedSet};

    #[test]
    fn the_set_deduplicates_and_keeps_declaration_order() {
        let set: DiscardedSet = [
            Discarded::PrecisionReduced,
            Discarded::Exif,
            Discarded::GainMap,
            Discarded::Exif,
        ]
        .into_iter()
        .collect();
        assert_eq!(
            set.iter().collect::<Vec<_>>(),
            [
                Discarded::GainMap,
                Discarded::Exif,
                Discarded::PrecisionReduced
            ]
        );
        assert!(DiscardedSet::default().is_empty());
        for (index, class) in Discarded::ORDER.into_iter().enumerate() {
            assert_eq!(class as usize, index, "ORDER follows the declaration");
        }
    }

    #[test]
    fn vectorized_hidden_color_probe_matches_scalar_reference_on_edge_cases() {
        use super::has_hidden_color;
        use crate::decoder::DecodedRow;

        let reference = |row: DecodedRow<'_>, channels: usize| match row {
            DecodedRow::U8(samples) => samples.chunks_exact(channels).any(|pixel| {
                pixel[channels - 1] == 0 && pixel[..channels - 1].iter().any(|&v| v != 0)
            }),
            DecodedRow::F32(samples) => samples.chunks_exact(channels).any(|pixel| {
                pixel[channels - 1] == 0.0 && pixel[..channels - 1].iter().any(|&v| v != 0.0)
            }),
        };

        // Test every pixel position across multiple chunk boundaries plus trailing partial bytes.
        for len in [0_usize, 1, 3, 63, 64, 65, 127, 128, 131] {
            for channels in [2_usize, 4] {
                let mut u8_row = vec![0_u8; len * channels + 1]; // +1 trailing partial sample
                for px in u8_row.chunks_exact_mut(channels) {
                    px[0] = 77;
                    px[channels - 1] = 255;
                }
                // Trailing partial sample is non-zero and must be ignored.
                *u8_row.last_mut().unwrap() = 99;
                assert!(!has_hidden_color(DecodedRow::U8(&u8_row), channels));
                assert_eq!(
                    has_hidden_color(DecodedRow::U8(&u8_row), channels),
                    reference(DecodedRow::U8(&u8_row), channels)
                );

                // Transparent black at every position is NOT hidden color.
                for pos in 0..len {
                    let mut row = u8_row.clone();
                    row[pos * channels..(pos + 1) * channels].fill(0);
                    assert!(!has_hidden_color(DecodedRow::U8(&row), channels));
                }

                // Single non-zero color sample with alpha 0 at every position & channel.
                for pos in 0..len {
                    for ch in 0..channels - 1 {
                        let mut row = u8_row.clone();
                        row[pos * channels..(pos + 1) * channels].fill(0);
                        row[pos * channels + ch] = 1;
                        assert!(
                            has_hidden_color(DecodedRow::U8(&row), channels),
                            "u8 len={len} ch={channels} pos={pos} c={ch}"
                        );
                    }
                }
            }
        }

        // F32 IEEE-754 edge cases: +0.0, -0.0, subnormals, infinities, NaNs.
        let f32_cases: &[([f32; 4], bool)] = &[
            ([0.0, 0.0, 0.0, 0.0], false),
            ([-0.0, 0.0, -0.0, -0.0], false),
            ([-0.0, -0.0, -0.0, 0.0], false),
            ([1.0, 0.5, 0.25, 1.0], false),
            ([1.0, 0.5, 0.25, f32::from_bits(1)], false), // subnormal alpha != 0.0
            ([1.0, 0.5, 0.25, f32::NAN], false),          // NaN alpha != 0.0
            ([1.0, 0.5, 0.25, f32::INFINITY], false),
            ([1.0, 0.5, 0.25, f32::NEG_INFINITY], false),
            ([f32::from_bits(1), 0.0, 0.0, 0.0], true), // subnormal color, +0.0 alpha
            ([0.0, f32::from_bits(1), 0.0, -0.0], true), // subnormal color, -0.0 alpha
            ([0.0, 0.0, f32::NAN, 0.0], true),          // NaN color != 0.0, +0.0 alpha
            ([f32::NAN, 0.0, 0.0, -0.0], true),         // NaN color, -0.0 alpha
            ([0.0, f32::INFINITY, 0.0, 0.0], true),
            ([0.0, 0.0, f32::NEG_INFINITY, -0.0], true),
            ([0.0, -1e-30, 0.0, 0.0], true),
        ];
        for &(pixel, expected) in f32_cases {
            // Test both standalone and embedded past a 64-pixel chunk boundary.
            let mut row = vec![0.0_f32; 70 * 4];
            row[67 * 4..68 * 4].copy_from_slice(&pixel);
            assert_eq!(
                has_hidden_color(DecodedRow::F32(&row), 4),
                expected,
                "RGBA f32 pixel={pixel:?}"
            );
            assert_eq!(
                has_hidden_color(DecodedRow::F32(&row), 4),
                reference(DecodedRow::F32(&row), 4),
                "RGBA f32 parity pixel={pixel:?}"
            );

            // GrayAlpha f32 using [color, alpha] = [pixel[0] | pixel[1] | pixel[2], pixel[3]].
            for c in 0..3 {
                let mut ga = vec![0.0_f32; 70 * 2];
                ga[67 * 2] = pixel[c];
                ga[67 * 2 + 1] = pixel[3];
                let expected_ga = reference(DecodedRow::F32(&ga), 2);
                assert_eq!(
                    has_hidden_color(DecodedRow::F32(&ga), 2),
                    expected_ga,
                    "GrayAlpha f32 pixel=[{:?}, {:?}]",
                    pixel[c],
                    pixel[3]
                );
            }
        }
    }

    /// Run with:
    /// `cargo test --release -p ferrisma --lib bench_hidden_color_probe -- --ignored --nocapture`
    #[test]
    #[ignore = "manual benchmark for issue #59"]
    fn bench_hidden_color_probe() {
        use std::hint::black_box;
        use std::time::Instant;

        use super::has_hidden_color;
        use crate::decoder::DecodedRow;

        const PIXELS: usize = 8000 * 6000; // 48 MP worst-case scan (no hidden color)
        let mut rgba_u8 = vec![0_u8; PIXELS * 4];
        for (i, px) in rgba_u8.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            // Mix opaque pixels and transparent-black pixels (no hidden color).
            *px = if i % 7 == 0 {
                [0, 0, 0, 0]
            } else {
                [120, 80, 40, 255]
            };
        }
        let mut ga_u8 = vec![0_u8; PIXELS * 2];
        for (i, px) in ga_u8.as_chunks_mut::<2>().0.iter_mut().enumerate() {
            *px = if i % 7 == 0 { [0, 0] } else { [120, 255] };
        }
        let mut rgba_f32 = vec![0.0_f32; PIXELS * 4];
        for (i, px) in rgba_f32.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            *px = if i % 7 == 0 {
                [-0.0, 0.0, -0.0, -0.0]
            } else {
                [0.5, 0.25, 0.75, 1.0]
            };
        }
        let mut ga_f32 = vec![0.0_f32; PIXELS * 2];
        for (i, px) in ga_f32.as_chunks_mut::<2>().0.iter_mut().enumerate() {
            *px = if i % 7 == 0 { [-0.0, 0.0] } else { [0.5, 1.0] };
        }

        let cases: [(&str, DecodedRow<'_>, usize, usize); 4] = [
            ("48MP RGBA U8", DecodedRow::U8(&rgba_u8), 4, rgba_u8.len()),
            ("48MP GrayAlpha U8", DecodedRow::U8(&ga_u8), 2, ga_u8.len()),
            (
                "48MP RGBA F32",
                DecodedRow::F32(&rgba_f32),
                4,
                rgba_f32.len() * 4,
            ),
            (
                "48MP GrayAlpha F32",
                DecodedRow::F32(&ga_f32),
                2,
                ga_f32.len() * 4,
            ),
        ];

        println!("HiddenColorProbe 48MP worst-case scan (best of 7):");
        println!("| case | best ms | GB/s |");
        println!("| --- | ---: | ---: |");
        for (name, row, channels, bytes) in cases {
            assert!(!has_hidden_color(row, channels));
            let mut best = std::time::Duration::MAX;
            for _ in 0..7 {
                let start = Instant::now();
                let found = has_hidden_color(black_box(row), channels);
                let elapsed = start.elapsed();
                assert!(!found);
                best = best.min(elapsed);
            }
            let ms = best.as_secs_f64() * 1_000.0;
            #[allow(clippy::cast_precision_loss)]
            let gbps = (bytes as f64 / 1e9) / best.as_secs_f64();
            println!("| {name} | {ms:.3} | {gbps:.2} |");
        }
    }
}
