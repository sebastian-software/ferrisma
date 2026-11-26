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
    let mut rest = bytes;
    while let Some((header, after_header)) = rest.split_at_checked(8) {
        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let kind = &header[4..8];
        let (header_len, total, body) = match size {
            0 => (8, rest.len(), after_header),
            1 => {
                let Some((large, body)) = after_header.split_at_checked(8) else {
                    break;
                };
                let large = u64::from_be_bytes(large.try_into().unwrap_or_default());
                (16, usize::try_from(large).unwrap_or(usize::MAX), body)
            }
            size => (8, usize::try_from(size).unwrap_or(usize::MAX), after_header),
        };
        if kind == b"brob" {
            if let Some(inner) = body.get(..4) {
                classify(inner, &mut discarded);
            }
        } else {
            classify(kind, &mut discarded);
        }
        if total < header_len {
            break;
        }
        let Some(next) = rest.get(total..) else {
            break;
        };
        rest = next;
    }
    discarded
}

#[cfg(test)]
mod tests {
    use super::{SIGNATURE, container_discards};
    use crate::discarded::Discarded;

    fn iso_box(kind: [u8; 4], body: &[u8]) -> Vec<u8> {
        let size = u32::try_from(body.len() + 8).unwrap();
        let mut out = size.to_be_bytes().to_vec();
        out.extend_from_slice(&kind);
        out.extend_from_slice(body);
        out
    }

    fn largesize_box(kind: [u8; 4], declared_size: u64, body: &[u8]) -> Vec<u8> {
        let mut out = 1_u32.to_be_bytes().to_vec();
        out.extend_from_slice(&kind);
        out.extend_from_slice(&declared_size.to_be_bytes());
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn non_container_bytes_report_no_discards() {
        assert!(container_discards(b"").is_empty());
        assert!(container_discards(b"\xff\x0a").is_empty());
    }

    #[test]
    fn crafted_largesize_near_max_does_not_overflow_or_panic() {
        let near_max_sizes = [
            (usize::MAX - 15) as u64,
            (usize::MAX - 12) as u64,
            (usize::MAX - 8) as u64,
            usize::MAX as u64,
            u64::MAX - 15,
            u64::MAX - 7,
            u64::MAX,
        ];
        for declared_size in near_max_sizes {
            let mut bytes = SIGNATURE.to_vec();
            bytes.extend(largesize_box(*b"Exif", declared_size, b"payload"));
            let discards = container_discards(&bytes);
            assert_eq!(
                discards.iter().collect::<Vec<_>>(),
                vec![Discarded::Exif],
                "declared_size={declared_size:#x}"
            );
        }
    }

    #[test]
    fn crafted_largesize_boundary_and_malformed_sizes_terminate_cleanly() {
        // Sizes smaller than the 16-byte largesize header must stop without looping.
        for small in [0_u64, 1, 8, 15] {
            let mut bytes = SIGNATURE.to_vec();
            bytes.extend(largesize_box(*b"Exif", small, b""));
            bytes.extend(iso_box(*b"xml ", b"ignored"));
            assert_eq!(
                container_discards(&bytes).iter().collect::<Vec<_>>(),
                vec![Discarded::Exif],
                "small largesize={small}"
            );
        }

        // Exact 16-byte largesize box advances cleanly to the following box.
        let mut exact_empty = SIGNATURE.to_vec();
        exact_empty.extend(largesize_box(*b"free", 16, b""));
        exact_empty.extend(iso_box(*b"xml ", b"<x/>"));
        assert_eq!(
            container_discards(&exact_empty).iter().collect::<Vec<_>>(),
            vec![Discarded::Xmp]
        );

        // Largesize brob box advances cleanly to a subsequent box.
        let mut brob_large = SIGNATURE.to_vec();
        brob_large.extend(largesize_box(*b"brob", 20, b"Exif"));
        brob_large.extend(iso_box(*b"jhgm", b"gain"));
        assert_eq!(
            container_discards(&brob_large).iter().collect::<Vec<_>>(),
            vec![Discarded::GainMap, Discarded::Exif]
        );

        // Declared size equal to exact remaining length vs one byte past the end.
        let mut exact_eof = SIGNATURE.to_vec();
        exact_eof.extend(largesize_box(*b"jbrd", 20, b"data"));
        assert_eq!(
            container_discards(&exact_eof).iter().collect::<Vec<_>>(),
            vec![Discarded::JpegReconstructionData]
        );
        let mut one_past_eof = SIGNATURE.to_vec();
        one_past_eof.extend(largesize_box(*b"jbrd", 21, b"data"));
        assert_eq!(
            container_discards(&one_past_eof).iter().collect::<Vec<_>>(),
            vec![Discarded::JpegReconstructionData]
        );

        // Truncated largesize header (missing the full 8-byte extended size).
        let mut truncated = SIGNATURE.to_vec();
        truncated.extend_from_slice(&1_u32.to_be_bytes());
        truncated.extend_from_slice(b"Exif");
        truncated.extend_from_slice(&[0, 0, 0, 0]);
        assert!(container_discards(&truncated).is_empty());

        // Box extending to EOF (`size == 0`) stops cleanly after classifying.
        let mut to_eof = SIGNATURE.to_vec();
        to_eof.extend_from_slice(&0_u32.to_be_bytes());
        to_eof.extend_from_slice(b"jumb");
        to_eof.extend_from_slice(b"data");
        assert_eq!(
            container_discards(&to_eof).iter().collect::<Vec<_>>(),
            vec![Discarded::OtherMetadata]
        );

        // Malformed 32-bit box size smaller than 8 bytes stops cleanly.
        let mut small_box = SIGNATURE.to_vec();
        small_box.extend_from_slice(&4_u32.to_be_bytes());
        small_box.extend_from_slice(b"Exif");
        small_box.extend(iso_box(*b"xml ", b"ignored"));
        assert_eq!(
            container_discards(&small_box).iter().collect::<Vec<_>>(),
            vec![Discarded::Exif]
        );
    }
}
