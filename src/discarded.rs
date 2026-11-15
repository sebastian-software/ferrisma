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
    /// the decoder (AVIF `prem`); color at low alpha loses precision.
    Unpremultiplied,
    /// The input has more bits per sample than reach the encoder (8-bit
    /// rows): 10- or 12-bit AVIF, JPEG XL above 8 bits, color or alpha.
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
/// Wraps the codec source of an RGBA layout, before any reorientation or
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

fn has_hidden_color(row: crate::decoder::DecodedRow<'_>) -> bool {
    use crate::decoder::DecodedRow;
    match row {
        DecodedRow::U8(samples) => samples
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[3] == 0 && pixel[..3] != [0, 0, 0]),
        DecodedRow::F32(samples) => samples
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[3] == 0.0 && pixel[..3].iter().any(|&value| value != 0.0)),
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

    fn set_decode_threads(&mut self, threads: usize) {
        self.source.set_decode_threads(threads);
    }

    fn discarded(&self) -> DiscardedSet {
        self.source.discarded()
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
        self.source.read_region(region, budget, &mut |y, row| {
            if !found.load(std::sync::atomic::Ordering::Relaxed) && has_hidden_color(row) {
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
}
