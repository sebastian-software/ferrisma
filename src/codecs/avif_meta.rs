//! Minimal, read-only AVIF metadata needed to describe decoded output.
//!
//! This independent container-property parser reads the selected primary
//! item's `colr`, `irot`, `imir`, `clap`, and `ispe` properties. The production
//! AV1 sequence header comes from the workspace libavif port's retained color
//! samples. A small Rust header parser remains here as a test oracle. This
//! parser is bounds-checked, allocation-light, and never converts pixels.

/// Malformed or unsupported metadata structure. Allocation and ICC-limit
/// failures retain their own engine error categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MetaError {
    Truncated,
    Allocation,
    ProfileTooLarge,
    MissingBox([u8; 4]),
    DuplicateBox([u8; 4]),
    InvalidPropertyIndex,
    UnsupportedVersion,
    UnsupportedEssentialProperty([u8; 4]),
    #[cfg(test)]
    MissingSequenceHeader,
    #[cfg(test)]
    InvalidSequenceHeader,
}

/// ISO/IEC 23091-2 `nclx` color information from a `colr` property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Nclx {
    pub primaries: u16,
    pub transfer: u16,
    pub matrix: u16,
    pub full_range: bool,
}

/// Properties associated with the primary item, in association order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PrimaryProperties {
    pub nclx: Option<Nclx>,
    /// Restricted or unrestricted ICC profile bytes (`prof` or `rICC`).
    pub icc: Option<Vec<u8>>,
    /// Image spatial extent (`ispe`) in pixels.
    pub extent: Option<(u32, u32)>,
    /// Anticlockwise rotation in quarter turns (`irot`).
    pub rotation: u8,
    /// Mirror axis (`imir`): 0 exchanges top and bottom, 1 exchanges left
    /// and right (ISO/IEC 23008-12:2022 6.5.12).
    pub mirror: Option<u8>,
    /// A clean-aperture crop (`clap`) is associated.
    pub clean_aperture: bool,
    /// `imir` is associated before `irot`. Transformative properties apply
    /// in association order, so this is a different transform.
    pub mirror_before_rotation: bool,
}

const MAX_ICC_BYTES: usize = 16 * 1024 * 1024;

/// Color configuration from the AV1 sequence header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SequenceColor {
    pub bit_depth: u8,
    pub monochrome: bool,
    pub primaries: u8,
    pub transfer: u8,
    pub matrix: u8,
    pub full_range: bool,
}

struct Reader<'a> {
    data: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], MetaError> {
        if count > self.data.len() {
            return Err(MetaError::Truncated);
        }
        let (head, tail) = self.data.split_at(count);
        self.data = tail;
        Ok(head)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], MetaError> {
        self.take(N)?.try_into().map_err(|_| MetaError::Truncated)
    }

    fn u8(&mut self) -> Result<u8, MetaError> {
        Ok(self.array::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, MetaError> {
        Ok(u16::from_be_bytes(self.array()?))
    }

    fn u32(&mut self) -> Result<u32, MetaError> {
        Ok(u32::from_be_bytes(self.array()?))
    }

    fn u64(&mut self) -> Result<u64, MetaError> {
        Ok(u64::from_be_bytes(self.array()?))
    }

    /// Version and flags of an ISOBMFF full box.
    fn full_box_header(&mut self) -> Result<(u8, u32), MetaError> {
        let raw = self.u32()?;
        let [version, ..] = raw.to_be_bytes();
        Ok((version, raw & 0x00ff_ffff))
    }

    fn rest(&mut self) -> &'a [u8] {
        std::mem::take(&mut self.data)
    }
}

struct IsoBox<'a> {
    kind: [u8; 4],
    body: &'a [u8],
}

/// Split a byte range into its sequence of ISOBMFF boxes.
fn boxes(data: &[u8]) -> Result<Vec<IsoBox<'_>>, MetaError> {
    let mut reader = Reader::new(data);
    let mut parsed = Vec::new();
    while !reader.data.is_empty() {
        let available = reader.data.len();
        let size = reader.u32()?;
        let kind = reader.array::<4>()?;
        let (header, total) = match size {
            0 => (8, available),
            1 => {
                let large = usize::try_from(reader.u64()?).map_err(|_| MetaError::Truncated)?;
                (16, large)
            }
            size => (8, usize::try_from(size).map_err(|_| MetaError::Truncated)?),
        };
        let body_len = total.checked_sub(header).ok_or(MetaError::Truncated)?;
        parsed.push(IsoBox {
            kind,
            body: reader.take(body_len)?,
        });
    }
    Ok(parsed)
}

fn unique<'a, 'b>(
    list: &'b [IsoBox<'a>],
    kind: [u8; 4],
) -> Result<Option<&'b IsoBox<'a>>, MetaError> {
    let mut found = list.iter().filter(|item| item.kind == kind);
    let first = found.next();
    if found.next().is_some() {
        return Err(MetaError::DuplicateBox(kind));
    }
    Ok(first)
}

fn required<'a, 'b>(list: &'b [IsoBox<'a>], kind: [u8; 4]) -> Result<&'b IsoBox<'a>, MetaError> {
    unique(list, kind)?.ok_or(MetaError::MissingBox(kind))
}

/// Properties this adapter interprets or that `avif-decode` handles itself.
/// An essential property outside this list must not be silently ignored.
const UNDERSTOOD_PROPERTIES: [&[u8; 4]; 8] = [
    b"av1C", b"ispe", b"colr", b"pixi", b"irot", b"imir", b"clap", b"auxC",
];

/// Read the primary item's associated properties from an AVIF file.
pub(crate) fn primary_properties(file: &[u8]) -> Result<PrimaryProperties, MetaError> {
    let top = boxes(file)?;
    let mut meta = Reader::new(required(&top, *b"meta")?.body);
    let (version, _) = meta.full_box_header()?;
    if version != 0 {
        return Err(MetaError::UnsupportedVersion);
    }
    let children = boxes(meta.rest())?;

    let mut pitm = Reader::new(required(&children, *b"pitm")?.body);
    let primary = match pitm.full_box_header()?.0 {
        0 => u32::from(pitm.u16()?),
        1 => pitm.u32()?,
        _ => return Err(MetaError::UnsupportedVersion),
    };

    let iprp = boxes(required(&children, *b"iprp")?.body)?;
    let properties = boxes(required(&iprp, *b"ipco")?.body)?;
    let mut associated = Vec::new();
    for ipma in iprp.iter().filter(|item| &item.kind == b"ipma") {
        let mut reader = Reader::new(ipma.body);
        let (version, flags) = reader.full_box_header()?;
        if version > 1 {
            return Err(MetaError::UnsupportedVersion);
        }
        for _ in 0..reader.u32()? {
            let item = if version < 1 {
                u32::from(reader.u16()?)
            } else {
                reader.u32()?
            };
            for _ in 0..reader.u8()? {
                let (essential, index) = if flags & 1 == 1 {
                    let raw = reader.u16()?;
                    (raw & 0x8000 != 0, usize::from(raw & 0x7fff))
                } else {
                    let raw = reader.u8()?;
                    (raw & 0x80 != 0, usize::from(raw & 0x7f))
                };
                if item == primary {
                    associated.push((essential, index));
                }
            }
        }
    }

    let mut result = PrimaryProperties::default();
    for (essential, index) in associated {
        // Index zero means "no property"; property indices are one-based.
        if index == 0 {
            continue;
        }
        let property = properties
            .get(index - 1)
            .ok_or(MetaError::InvalidPropertyIndex)?;
        read_property(property, &mut result)?;
        if essential && !UNDERSTOOD_PROPERTIES.contains(&&property.kind) {
            return Err(MetaError::UnsupportedEssentialProperty(property.kind));
        }
    }
    Ok(result)
}

fn read_property(property: &IsoBox<'_>, result: &mut PrimaryProperties) -> Result<(), MetaError> {
    let mut reader = Reader::new(property.body);
    match &property.kind {
        b"colr" => match &reader.array::<4>()? {
            b"nclx" => {
                if result.nclx.is_some() {
                    return Err(MetaError::DuplicateBox(*b"colr"));
                }
                result.nclx = Some(Nclx {
                    primaries: reader.u16()?,
                    transfer: reader.u16()?,
                    matrix: reader.u16()?,
                    full_range: reader.u8()? & 0x80 != 0,
                });
            }
            b"prof" | b"rICC" => {
                if result.icc.is_some() {
                    return Err(MetaError::DuplicateBox(*b"colr"));
                }
                let source = reader.rest();
                if source.len() > MAX_ICC_BYTES {
                    return Err(MetaError::ProfileTooLarge);
                }
                let mut profile = Vec::new();
                profile
                    .try_reserve_exact(source.len())
                    .map_err(|_| MetaError::Allocation)?;
                profile.extend_from_slice(source);
                result.icc = Some(profile);
            }
            // Other colour types carry no rendered-color evidence used here.
            _ => {}
        },
        b"ispe" => {
            reader.full_box_header()?;
            if result.extent.is_some() {
                return Err(MetaError::DuplicateBox(*b"ispe"));
            }
            result.extent = Some((reader.u32()?, reader.u32()?));
        }
        b"irot" => {
            result.mirror_before_rotation |= result.mirror.is_some();
            result.rotation = reader.u8()? & 0b11;
        }
        b"imir" => result.mirror = Some(reader.u8()? & 1),
        b"clap" => result.clean_aperture = true,
        _ => {}
    }
    Ok(())
}

/// Most-significant-bit-first reader for AV1 `f(n)` and `uvlc()` fields.
#[cfg(test)]
struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
}

#[cfg(test)]
impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    fn bit(&mut self) -> Result<bool, MetaError> {
        let byte = self
            .data
            .get(self.position / 8)
            .ok_or(MetaError::InvalidSequenceHeader)?;
        let bit = (byte >> (7 - self.position % 8)) & 1 == 1;
        self.position += 1;
        Ok(bit)
    }

    fn flag(&mut self) -> Result<bool, MetaError> {
        self.bit()
    }

    fn bits(&mut self, count: u32) -> Result<u32, MetaError> {
        debug_assert!(count <= 32);
        let mut value = 0_u64;
        for _ in 0..count {
            value = (value << 1) | u64::from(self.bit()?);
        }
        u32::try_from(value).map_err(|_| MetaError::InvalidSequenceHeader)
    }

    fn byte(&mut self) -> Result<u8, MetaError> {
        u8::try_from(self.bits(8)?).map_err(|_| MetaError::InvalidSequenceHeader)
    }

    fn uvlc(&mut self) -> Result<(), MetaError> {
        let mut leading_zeros = 0_u32;
        while !self.bit()? {
            leading_zeros += 1;
            if leading_zeros >= 32 {
                return Ok(());
            }
        }
        self.bits(leading_zeros)?;
        Ok(())
    }
}

#[cfg(test)]
fn leb128(reader: &mut Reader<'_>) -> Result<usize, MetaError> {
    let mut value = 0_u64;
    for index in 0..8 {
        let byte = reader.u8()?;
        value |= u64::from(byte & 0x7f) << (index * 7);
        if byte & 0x80 == 0 {
            return usize::try_from(value).map_err(|_| MetaError::Truncated);
        }
    }
    Err(MetaError::InvalidSequenceHeader)
}

/// Find the first sequence header OBU in an AV1 payload and read its color
/// configuration (AV1 specification sections 5.3 and 5.5).
#[cfg(test)]
pub(crate) fn sequence_color(obus: &[u8]) -> Result<SequenceColor, MetaError> {
    const OBU_SEQUENCE_HEADER: u8 = 1;
    let mut reader = Reader::new(obus);
    while !reader.data.is_empty() {
        let header = reader.u8()?;
        let kind = (header >> 3) & 0x0f;
        if header & 0b100 != 0 {
            reader.u8()?; // Extension header: temporal and spatial ids.
        }
        let payload = if header & 0b10 != 0 {
            let size = leb128(&mut reader)?;
            reader.take(size)?
        } else {
            reader.rest()
        };
        if kind == OBU_SEQUENCE_HEADER {
            return parse_sequence_header(payload);
        }
    }
    Err(MetaError::MissingSequenceHeader)
}

#[cfg(test)]
#[allow(clippy::too_many_lines)] // Mirrors the specification's field order.
fn parse_sequence_header(payload: &[u8]) -> Result<SequenceColor, MetaError> {
    let mut bits = BitReader::new(payload);
    let seq_profile = bits.bits(3)?;
    let _still_picture = bits.flag()?;
    let reduced_still_picture_header = bits.flag()?;
    if reduced_still_picture_header {
        bits.bits(5)?; // seq_level_idx[0]
    } else {
        let timing_info_present = bits.flag()?;
        let mut decoder_model_info_present = false;
        let mut buffer_delay_length = 0;
        if timing_info_present {
            bits.bits(32)?; // num_units_in_display_tick
            bits.bits(32)?; // time_scale
            if bits.flag()? {
                bits.uvlc()?; // num_ticks_per_picture_minus_1
            }
            decoder_model_info_present = bits.flag()?;
            if decoder_model_info_present {
                buffer_delay_length = bits.bits(5)? + 1;
                bits.bits(32)?; // num_units_in_decoding_tick
                bits.bits(5)?; // buffer_removal_time_length_minus_1
                bits.bits(5)?; // frame_presentation_time_length_minus_1
            }
        }
        let initial_display_delay_present = bits.flag()?;
        let operating_points = bits.bits(5)? + 1;
        for _ in 0..operating_points {
            bits.bits(12)?; // operating_point_idc
            if bits.bits(5)? > 7 {
                bits.flag()?; // seq_tier
            }
            if decoder_model_info_present && bits.flag()? {
                bits.bits(buffer_delay_length)?; // decoder_buffer_delay
                bits.bits(buffer_delay_length)?; // encoder_buffer_delay
                bits.flag()?; // low_delay_mode_flag
            }
            if initial_display_delay_present && bits.flag()? {
                bits.bits(4)?; // initial_display_delay_minus_1
            }
        }
    }
    let frame_width_bits = bits.bits(4)? + 1;
    let frame_height_bits = bits.bits(4)? + 1;
    bits.bits(frame_width_bits)?;
    bits.bits(frame_height_bits)?;
    if !reduced_still_picture_header && bits.flag()? {
        bits.bits(4)?; // delta_frame_id_length_minus_2
        bits.bits(3)?; // additional_frame_id_length_minus_1
    }
    bits.flag()?; // use_128x128_superblock
    bits.flag()?; // enable_filter_intra
    bits.flag()?; // enable_intra_edge_filter
    if !reduced_still_picture_header {
        bits.flag()?; // enable_interintra_compound
        bits.flag()?; // enable_masked_compound
        bits.flag()?; // enable_warped_motion
        bits.flag()?; // enable_dual_filter
        let enable_order_hint = bits.flag()?;
        if enable_order_hint {
            bits.flag()?; // enable_jnt_comp
            bits.flag()?; // enable_ref_frame_mvs
        }
        let force_screen_content_tools = if bits.flag()? {
            2 // seq_choose_screen_content_tools: SELECT_SCREEN_CONTENT_TOOLS
        } else {
            bits.bits(1)?
        };
        if force_screen_content_tools > 0 && !bits.flag()? {
            bits.flag()?; // seq_force_integer_mv
        }
        if enable_order_hint {
            bits.bits(3)?; // order_hint_bits_minus_1
        }
    }
    bits.flag()?; // enable_superres
    bits.flag()?; // enable_cdef
    bits.flag()?; // enable_restoration

    // color_config()
    let high_bitdepth = bits.flag()?;
    let bit_depth = if seq_profile == 2 && high_bitdepth {
        if bits.flag()? { 12 } else { 10 }
    } else if high_bitdepth {
        10
    } else {
        8
    };
    let monochrome = seq_profile != 1 && bits.flag()?;
    let (primaries, transfer, matrix) = if bits.flag()? {
        (bits.byte()?, bits.byte()?, bits.byte()?)
    } else {
        (2, 2, 2) // CP/TC/MC_UNSPECIFIED
    };
    let full_range = if !monochrome && primaries == 1 && transfer == 13 && matrix == 0 {
        true // sRGB identity coding implies full range.
    } else {
        bits.flag()?
    };
    Ok(SequenceColor {
        bit_depth,
        monochrome,
        primaries,
        transfer,
        matrix,
        full_range,
    })
}

#[cfg(test)]
#[path = "avif_meta_tests.rs"]
mod tests;
