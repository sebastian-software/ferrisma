use super::{
    MetaError, Nclx, PrimaryProperties, SequenceColor, primary_properties, sequence_color,
};
use crate::codecs::avif::rendered_cicp;
use crate::color::ColorEncoding;
use crate::color_contract::{LinearSrgbReadiness, plan_to_linear_srgb};
use crate::error::Error;

fn boxed(kind: [u8; 4], body: &[u8]) -> Vec<u8> {
    let size = u32::try_from(body.len() + 8).unwrap();
    [&size.to_be_bytes()[..], &kind, body].concat()
}

fn full(kind: [u8; 4], version: u8, flags: u32, body: &[u8]) -> Vec<u8> {
    let header = (u32::from(version) << 24 | flags).to_be_bytes();
    boxed(kind, &[&header[..], body].concat())
}

fn nclx(primaries: u16, transfer: u16, matrix: u16, full_range: bool) -> Vec<u8> {
    let mut body = b"nclx".to_vec();
    for value in [primaries, transfer, matrix] {
        body.extend_from_slice(&value.to_be_bytes());
    }
    body.push(if full_range { 0x80 } else { 0 });
    boxed(*b"colr", &body)
}

fn ispe(width: u32, height: u32) -> Vec<u8> {
    full(
        *b"ispe",
        0,
        0,
        &[width.to_be_bytes(), height.to_be_bytes()].concat(),
    )
}

/// Build `ftyp` + `meta` with one property container and one `ipma` box.
/// Each association is `(item, essential, one-based property index)`.
fn file(properties: &[Vec<u8>], associations: &[(u16, bool, u8)]) -> Vec<u8> {
    let mut ipma = Vec::new();
    let items: Vec<u16> = {
        let mut items: Vec<u16> = associations.iter().map(|entry| entry.0).collect();
        items.dedup();
        items
    };
    ipma.extend_from_slice(&u32::try_from(items.len()).unwrap().to_be_bytes());
    for item in items {
        let entries: Vec<_> = associations
            .iter()
            .filter(|entry| entry.0 == item)
            .collect();
        ipma.extend_from_slice(&item.to_be_bytes());
        ipma.push(u8::try_from(entries.len()).unwrap());
        for (_, essential, index) in entries {
            ipma.push(if *essential { 0x80 | index } else { *index });
        }
    }
    let iprp = boxed(
        *b"iprp",
        &[
            boxed(*b"ipco", &properties.concat()),
            full(*b"ipma", 0, 0, &ipma),
        ]
        .concat(),
    );
    let meta = full(
        *b"meta",
        0,
        0,
        &[full(*b"pitm", 0, 0, &1_u16.to_be_bytes()), iprp].concat(),
    );
    [boxed(*b"ftyp", b"avifmif1"), meta].concat()
}

#[test]
fn reads_only_the_primary_items_color_extent_and_transforms() {
    let icc = boxed(*b"colr", &[&b"prof"[..], &[1, 2, 3]].concat());
    let parsed = primary_properties(&file(
        &[
            ispe(64, 32),
            nclx(12, 13, 6, true),
            icc,
            boxed(*b"irot", &[3]),
            boxed(*b"imir", &[1]),
            boxed(*b"clap", &[0; 32]),
            nclx(9, 16, 9, false),
        ],
        &[
            (1, false, 1),
            (1, true, 2),
            (1, false, 3),
            (1, true, 4),
            (1, true, 5),
            (1, true, 6),
            // Another item's properties never describe the primary item.
            (2, true, 7),
        ],
    ))
    .unwrap();
    assert_eq!(
        parsed,
        PrimaryProperties {
            nclx: Some(Nclx {
                primaries: 12,
                transfer: 13,
                matrix: 6,
                full_range: true,
            }),
            icc: Some(vec![1, 2, 3]),
            extent: Some((64, 32)),
            rotation: 3,
            mirror: Some(1),
            clean_aperture: true,
        }
    );
}

#[test]
fn rejects_ambiguous_and_unknown_essential_properties() {
    let duplicate = file(
        &[nclx(1, 13, 6, true), nclx(9, 16, 9, false)],
        &[(1, false, 1), (1, false, 2)],
    );
    assert_eq!(
        primary_properties(&duplicate),
        Err(MetaError::DuplicateBox(*b"colr"))
    );
    let unknown = file(&[boxed(*b"xyzw", &[])], &[(1, true, 1)]);
    assert_eq!(
        primary_properties(&unknown),
        Err(MetaError::UnsupportedEssentialProperty(*b"xyzw"))
    );
    let optional = file(&[boxed(*b"xyzw", &[])], &[(1, false, 1)]);
    assert_eq!(
        primary_properties(&optional),
        Ok(PrimaryProperties::default())
    );
    let out_of_range = file(&[ispe(1, 1)], &[(1, false, 2)]);
    assert_eq!(
        primary_properties(&out_of_range),
        Err(MetaError::InvalidPropertyIndex)
    );
}

#[test]
fn rejects_truncated_and_inconsistent_box_sizes() {
    let complete = file(&[ispe(2, 2)], &[(1, true, 1)]);
    for length in [3, 9, complete.len() - 1] {
        assert!(primary_properties(&complete[..length]).is_err(), "{length}");
    }
    let mut short_header = boxed(*b"ftyp", b"avif");
    short_header[3] = 4; // Smaller than its own header.
    assert_eq!(primary_properties(&short_header), Err(MetaError::Truncated));
    assert_eq!(
        primary_properties(&boxed(*b"ftyp", b"avif")),
        Err(MetaError::MissingBox(*b"meta"))
    );
}

#[test]
fn reads_large_size_and_to_end_boxes_and_sixteen_bit_indices() {
    let base = file(&[ispe(5, 7)], &[(1, true, 1)]);
    let meta = &base[16..];
    let mut large = [&1_u32.to_be_bytes()[..], b"ftyp"].concat();
    large.extend_from_slice(&24_u64.to_be_bytes());
    large.extend_from_slice(b"avifmif1");
    let mut to_end = meta.to_vec();
    to_end[..4].copy_from_slice(&0_u32.to_be_bytes());
    let parsed = primary_properties(&[large, to_end].concat()).unwrap();
    assert_eq!(parsed.extent, Some((5, 7)));

    let ipma = [
        &1_u32.to_be_bytes()[..],
        &1_u16.to_be_bytes(),
        &[1],
        &0x8001_u16.to_be_bytes(),
    ]
    .concat();
    let iprp = boxed(
        *b"iprp",
        &[boxed(*b"ipco", &ispe(9, 3)), full(*b"ipma", 0, 1, &ipma)].concat(),
    );
    let wide = full(
        *b"meta",
        0,
        0,
        &[full(*b"pitm", 0, 0, &1_u16.to_be_bytes()), iprp].concat(),
    );
    assert_eq!(primary_properties(&wide).unwrap().extent, Some((9, 3)));
}

/// Most-significant-bit-first writer for handcrafted AV1 headers.
#[derive(Default)]
struct Bits {
    bytes: Vec<u8>,
    length: usize,
}

impl Bits {
    fn put(&mut self, count: u32, value: u64) -> &mut Self {
        for shift in (0..count).rev() {
            if self.length.is_multiple_of(8) {
                self.bytes.push(0);
            }
            if (value >> shift) & 1 == 1 {
                *self.bytes.last_mut().unwrap() |= 0x80 >> (self.length % 8);
            }
            self.length += 1;
        }
        self
    }

    fn obus(&self) -> Vec<u8> {
        // A temporal delimiter precedes the sequence header in item payloads.
        let mut payload = vec![0x12, 0x00, 0x0a];
        payload.push(u8::try_from(self.bytes.len()).unwrap());
        payload.extend_from_slice(&self.bytes);
        payload
    }
}

/// Write `color_config()` for profile 0 followed by subsampling padding.
fn color(bits: &mut Bits, high_bitdepth: bool, description: Option<[u8; 3]>, range: bool) {
    bits.put(1, u64::from(high_bitdepth)).put(1, 0);
    match description {
        Some(values) => {
            bits.put(1, 1);
            for value in values {
                bits.put(8, u64::from(value));
            }
        }
        None => {
            bits.put(1, 0);
        }
    }
    bits.put(1, u64::from(range)).put(8, 0);
}

fn reduced_still(description: Option<[u8; 3]>, range: bool) -> Vec<u8> {
    let mut bits = Bits::default();
    // Profile 0, still picture, reduced header, seq_level_idx.
    bits.put(3, 0).put(1, 1).put(1, 1).put(5, 8);
    // 4-bit frame width/height fields and their values.
    bits.put(4, 3).put(4, 3).put(4, 15).put(4, 7);
    // 128x128, filter intra, intra edge, superres, CDEF, restoration.
    bits.put(6, 0b000_011);
    color(&mut bits, true, description, range);
    bits.obus()
}

#[test]
fn reads_reduced_still_picture_color_configuration() {
    assert_eq!(
        sequence_color(&reduced_still(Some([9, 16, 9]), false)),
        Ok(SequenceColor {
            bit_depth: 10,
            monochrome: false,
            primaries: 9,
            transfer: 16,
            matrix: 9,
            full_range: false,
        })
    );
    let unspecified = sequence_color(&reduced_still(None, true)).unwrap();
    assert_eq!(
        (
            unspecified.primaries,
            unspecified.transfer,
            unspecified.matrix
        ),
        (2, 2, 2)
    );
    assert!(unspecified.full_range);
}

#[test]
fn srgb_identity_coding_implies_full_range_without_a_range_bit() {
    // The trailing range bit is deliberately zero; it must not be read.
    let parsed = sequence_color(&reduced_still(Some([1, 13, 0]), false)).unwrap();
    assert!(parsed.full_range);
}

#[test]
fn skips_timing_decoder_model_and_operating_point_fields() {
    let mut bits = Bits::default();
    bits.put(3, 0).put(1, 0).put(1, 0); // Profile 0, not reduced.
    bits.put(1, 1)
        .put(32, 1)
        .put(32, 30)
        .put(1, 1)
        .put(5, 0b00101); // Timing, uvlc 4.
    bits.put(1, 1).put(5, 4).put(32, 90_000).put(5, 0).put(5, 0); // Decoder model.
    bits.put(1, 1).put(5, 1); // Initial display delay, two operating points.
    bits.put(12, 0)
        .put(5, 9)
        .put(1, 0)
        .put(1, 1)
        .put(5, 1)
        .put(5, 2)
        .put(1, 0);
    bits.put(1, 1).put(4, 3);
    bits.put(12, 0).put(5, 3).put(1, 0).put(1, 0);
    bits.put(4, 3).put(4, 3).put(4, 15).put(4, 7); // Frame size.
    bits.put(1, 1).put(4, 0).put(3, 0); // Frame id numbers.
    bits.put(3, 0); // 128x128, filter intra, intra edge.
    bits.put(4, 0).put(1, 1).put(2, 0); // Compound tools, order hint, jnt/ref mvs.
    bits.put(1, 0).put(1, 1).put(1, 0).put(1, 1); // Screen content and integer mv.
    bits.put(3, 6).put(3, 0b011); // Order hint bits, superres/CDEF/restoration.
    color(&mut bits, false, Some([1, 13, 6]), true);
    assert_eq!(
        sequence_color(&bits.obus()),
        Ok(SequenceColor {
            bit_depth: 8,
            monochrome: false,
            primaries: 1,
            transfer: 13,
            matrix: 6,
            full_range: true,
        })
    );
}

#[test]
fn rejects_missing_or_truncated_sequence_headers() {
    assert_eq!(
        sequence_color(&[0x12, 0x00]),
        Err(MetaError::MissingSequenceHeader)
    );
    let mut truncated = reduced_still(Some([1, 13, 6]), true);
    truncated.truncate(8);
    truncated[3] = 4;
    assert_eq!(
        sequence_color(&truncated),
        Err(MetaError::InvalidSequenceHeader)
    );
}

fn sequence(primaries: u8, transfer: u8, matrix: u8, full_range: bool) -> SequenceColor {
    SequenceColor {
        bit_depth: 8,
        monochrome: false,
        primaries,
        transfer,
        matrix,
        full_range,
    }
}

fn with_nclx(primaries: u16, transfer: u16, matrix: u16, full_range: bool) -> PrimaryProperties {
    PrimaryProperties {
        nclx: Some(Nclx {
            primaries,
            transfer,
            matrix,
            full_range,
        }),
        ..PrimaryProperties::default()
    }
}

#[test]
fn rendered_cicp_describes_full_range_rgb_output() {
    assert_eq!(
        rendered_cicp(&with_nclx(1, 13, 6, true), sequence(1, 13, 6, true)),
        Ok(Some([1, 13, 0, 1]))
    );
    // The bitstream may leave its description unspecified.
    assert_eq!(
        rendered_cicp(&with_nclx(12, 13, 6, false), sequence(2, 2, 2, false)),
        Ok(Some([12, 13, 0, 1]))
    );
    assert_eq!(
        rendered_cicp(&PrimaryProperties::default(), sequence(2, 2, 6, false)),
        Ok(None)
    );
}

#[test]
fn hdr_signals_are_reported_and_never_treated_as_srgb() {
    for properties in [with_nclx(9, 16, 9, false), PrimaryProperties::default()] {
        let cicp = rendered_cicp(&properties, sequence(9, 16, 9, false))
            .unwrap()
            .unwrap();
        assert_eq!(cicp, [9, 16, 0, 1]);
        assert_eq!(
            plan_to_linear_srgb(&ColorEncoding::Cicp(cicp)),
            LinearSrgbReadiness::RequiresHdrManagement
        );
    }
    let hlg = rendered_cicp(&with_nclx(9, 18, 9, true), sequence(9, 18, 9, true));
    assert_eq!(hlg, Ok(Some([9, 18, 0, 1])));
}

#[test]
fn contradictory_container_and_bitstream_signals_fail_closed() {
    for (properties, bitstream) in [
        // Decoding would use BT.601 while the container declares BT.709.
        (with_nclx(1, 13, 1, true), sequence(1, 13, 6, true)),
        (with_nclx(1, 13, 6, true), sequence(1, 13, 6, false)),
        (with_nclx(1, 16, 6, true), sequence(1, 13, 6, true)),
        (with_nclx(12, 13, 6, true), sequence(1, 13, 6, true)),
    ] {
        assert_eq!(
            rendered_cicp(&properties, bitstream),
            Err(Error::ContradictoryColorSignals)
        );
    }
    assert_eq!(
        rendered_cicp(&PrimaryProperties::default(), sequence(9, 16, 10, false)),
        Err(Error::UnsupportedCodecLayout)
    );
    assert_eq!(
        rendered_cicp(&with_nclx(300, 13, 6, true), sequence(2, 2, 6, true)),
        Err(Error::UnsupportedColorEncoding)
    );
}
