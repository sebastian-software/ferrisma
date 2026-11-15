//! The ISOBMFF boxes of a JPEG XL container that neither reader decodes
//! (ADR-0042 D2). Both readers see only the codestream; this walk names
//! what the output will not carry.

use crate::discarded::{Discarded, DiscardedSet};

const SIGNATURE: &[u8] = b"\0\0\0\x0cJXL \r\n\x87\n";

/// Classify one box type; `brob` boxes are classified by the type they
/// compress.
fn classify(kind: &[u8], discarded: &mut DiscardedSet) {
    match kind {
        b"Exif" => discarded.insert(Discarded::Exif),
        b"xml " => discarded.insert(Discarded::Xmp),
        b"jumb" => discarded.insert(Discarded::OtherMetadata),
        b"jbrd" => discarded.insert(Discarded::JpegReconstructionData),
        b"jhgm" => discarded.insert(Discarded::GainMap),
        b"JXL " | b"ftyp" | b"jxlc" | b"jxlp" | b"jxll" | b"jxli" | b"free" | b"skip" => {}
        _ => discarded.insert(Discarded::UnknownContainerData),
    }
}

/// The classes of the container's boxes; empty for a bare codestream. The
/// readers have validated the file; a walk that stops early reports what
/// it saw.
pub(crate) fn container_discards(bytes: &[u8]) -> DiscardedSet {
    let mut discarded = DiscardedSet::default();
    if !bytes.starts_with(SIGNATURE) {
        return discarded;
    }
    let mut at = 0_usize;
    while let Some(header) = bytes.get(at..at + 8) {
        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let kind = &header[4..8];
        let (header_len, total) = match size {
            0 => (8, bytes.len() - at),
            1 => {
                let Some(large) = bytes.get(at + 8..at + 16) else {
                    break;
                };
                let large = u64::from_be_bytes(large.try_into().unwrap_or_default());
                (16, usize::try_from(large).unwrap_or(usize::MAX))
            }
            size => (8, size as usize),
        };
        if kind == b"brob" {
            if let Some(inner) = bytes.get(at + header_len..at + header_len + 4) {
                classify(inner, &mut discarded);
            }
        } else {
            classify(kind, &mut discarded);
        }
        if total < header_len {
            break;
        }
        let Some(next) = at.checked_add(total) else {
            break;
        };
        at = next;
    }
    discarded
}
