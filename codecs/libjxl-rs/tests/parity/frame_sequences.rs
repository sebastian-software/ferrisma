// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_frame::{SequenceDecoder, frame_saved_as};
use jxl::jxl::image::Image3;
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::CodecMetadata;

pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "fs-decode" | "fs-decode-lm" | "fs-dc-state")
}
fn image(out: &mut String, image: &Image3<f32>, pixels: bool) {
    write!(out, " I {} {}", image.xsize(), image.ysize()).unwrap();
    if pixels {
        for c in 0..3 {
            for y in 0..image.ysize() {
                for x in 0..image.xsize() {
                    write!(out, " {}", image.plane_row(c, y)[x].to_bits()).unwrap();
                }
            }
        }
    }
}
fn bundle(out: &mut String, b: &ImageBundle<'_>, pixels: bool) {
    write!(
        out,
        " O {} {} {} {} {} {} {} {} {} {} {}",
        b.origin.x0,
        b.origin.y0,
        b.duration,
        b.timecode,
        u32::from(b.use_for_next_frame),
        u32::from(b.blend),
        b.blendmode.0,
        hex(&b.name),
        b.decoded_bytes(),
        u32::from(b.metadata().is_some()),
        b.color_transform.0,
    )
    .unwrap();
    image(out, b.color(), pixels);
    metadata::color_bundle(out, b.c_current());
    write!(out, " X {}", b.extra_channels().len()).unwrap();
    for ec in b.extra_channels() {
        write!(out, " {} {}", ec.xsize(), ec.ysize()).unwrap();
        if pixels {
            for y in 0..ec.ysize() {
                for x in 0..ec.xsize() {
                    write!(out, " {}", ec.row(y)[x].to_bits()).unwrap();
                }
            }
        }
    }
}
#[derive(Default)]
pub(super) struct DcCoverage {
    width: usize,
    height: usize,
    initialized: Vec<bool>,
}
#[derive(Default)]
pub(super) struct BundleCoverage {
    planes: Vec<DcCoverage>,
    moved_color_slot: Option<usize>,
}
#[derive(Default)]
pub(super) struct WriteCoverage {
    dc: [DcCoverage; 4],
    pub(super) bundles: [BundleCoverage; 2],
}
impl WriteCoverage {
    pub(super) fn update(&mut self, events: &[jxl::jxl::dec_frame::FrameWriteEvent]) {
        use jxl::jxl::dec_frame::{FrameWriteEvent, FrameWriteTarget};
        for event in events.iter().cloned() {
            match event {
                FrameWriteEvent::Allocate {
                    slot,
                    width,
                    height,
                } => {
                    self.dc[slot] = DcCoverage {
                        width,
                        height,
                        initialized: vec![false; width * height],
                    }
                }
                FrameWriteEvent::Row { slot, x, y, width } => self.dc[slot].mark(x, y, width),
                FrameWriteEvent::BundleClear { target } => {
                    self.bundles[match target {
                        FrameWriteTarget::Output => 0,
                        FrameWriteTarget::ReferenceStorage => 1,
                    }] = BundleCoverage::default()
                }
                FrameWriteEvent::BundleAllocate { target, sizes } => {
                    self.bundles[match target {
                        FrameWriteTarget::Output => 0,
                        FrameWriteTarget::ReferenceStorage => 1,
                    }] = BundleCoverage {
                        planes: sizes
                            .into_iter()
                            .map(|(width, height)| DcCoverage {
                                width,
                                height,
                                initialized: vec![false; width * height],
                            })
                            .collect(),
                        moved_color_slot: None,
                    }
                }
                FrameWriteEvent::BundleRow {
                    target,
                    channel,
                    x,
                    y,
                    width,
                } => self.bundles[match target {
                    FrameWriteTarget::Output => 0,
                    FrameWriteTarget::ReferenceStorage => 1,
                }]
                .planes[channel]
                    .mark(x, y, width),
                FrameWriteEvent::BundleMoveToReference { slot } => {
                    self.bundles[1].moved_color_slot = Some(slot);
                    // The C++ observer resizes the moved color planes to three.
                    self.bundles[1].planes.resize_with(3, DcCoverage::default);
                }
            }
        }
    }
}
impl DcCoverage {
    fn mark(&mut self, x: usize, y: usize, width: usize) {
        self.initialized[y * self.width + x..y * self.width + x + width].fill(true);
    }
}
pub(super) fn covered_bundle(
    out: &mut String,
    bundle_image: &ImageBundle<'_>,
    coverage: &BundleCoverage,
    color_alias: Option<&Image3<f32>>,
) {
    bundle(out, bundle_image, false);
    write!(out, " L {}", coverage.planes.len()).unwrap();
    for (c, plane) in coverage.planes.iter().enumerate() {
        write!(
            out,
            " {c} {}",
            plane.initialized.iter().filter(|&&v| v).count()
        )
        .unwrap();
        for y in 0..plane.height {
            for x in 0..plane.width {
                if plane.initialized[y * plane.width + x] {
                    let sample = if c < 3 {
                        color_alias.unwrap_or(bundle_image.color()).plane_row(c, y)[x]
                    } else {
                        bundle_image.extra_channels()[c - 3].row(y)[x]
                    };
                    write!(out, " {x} {y} {}", sample.to_bits()).unwrap();
                }
            }
        }
    }
}

pub(super) fn state(
    out: &mut String,
    s: &jxl::jxl::dec_cache::PassesDecoderState<'_, '_>,
    _pixels: bool,
    identities: &[usize; 4],
    coverage: &WriteCoverage,
) {
    let refs = s.shared_storage.reference_frames.borrow();
    let dc = s.shared_storage.dc_frames.borrow();
    let patches = s.shared_storage.image_features.patches.borrow();
    write!(
        out,
        " PD {} {} {} {}",
        u32::from(patches.has_any()),
        patches.get_references(),
        patches.positions().len(),
        patches.ref_positions().len()
    )
    .unwrap();
    for p in patches.ref_positions() {
        write!(
            out,
            " PR {} {} {} {} {}",
            p.reference, p.x0, p.y0, p.xsize, p.ysize
        )
        .unwrap();
    }
    let reference_mask = refs.iter().enumerate().fold(0, |mask, (i, r)| {
        mask | if r.frame.as_ref().unwrap().color().xsize() != 0 {
            1 << i
        } else {
            0
        }
    });
    let dc_mask = dc.iter().enumerate().fold(0, |mask, (i, d)| {
        mask | if d.xsize() != 0 { 16 << i } else { 0 }
    });
    write!(
        out,
        " Q {} {} {} {}",
        s.visible_frame_index, s.nonvisible_frame_index, reference_mask, dc_mask
    )
    .unwrap();
    for (i, r) in refs.iter().enumerate() {
        let b = r.frame.as_deref().unwrap();
        write!(
            out,
            " R {} {} {}",
            i,
            u32::from((b as *const ImageBundle<'_> as usize) == identities[i]),
            u32::from(r.ib_is_in_xyb)
        )
        .unwrap();
        bundle(out, b, b.color().xsize() != 0);
    }
    write!(
        out,
        " T {}",
        coverage.bundles[1]
            .moved_color_slot
            .map_or(-1, |slot| slot as i32)
    )
    .unwrap();
    let storage = s.frame_storage_for_referencing.borrow();
    let color_alias = coverage.bundles[1]
        .moved_color_slot
        .map(|slot| refs[slot].frame.as_ref().unwrap().color());
    covered_bundle(out, &storage, &coverage.bundles[1], color_alias);
    for (i, d) in dc.iter().enumerate() {
        write!(out, " C {i}").unwrap();
        image(out, d, false);
        let coverage = &coverage.dc[i];
        write!(
            out,
            " V {}",
            coverage.initialized.iter().filter(|&&v| v).count()
        )
        .unwrap();
        for y in 0..coverage.height {
            for x in 0..coverage.width {
                if coverage.initialized[y * coverage.width + x] {
                    write!(out, " {x} {y}").unwrap();
                    for c in 0..3 {
                        write!(out, " {}", d.plane_row(c, y)[x].to_bits()).unwrap();
                    }
                }
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "fs-dc-state" {
        use jxl::jxl::frame_header::{FLAG_USE_DC_FRAME, FrameHeader, FrameType};
        use jxl::jxl::passes_state::{DcSource, PassesSharedState, initialize_passes_shared_state};
        let width = args.next().unwrap().parse().unwrap();
        let height = args.next().unwrap().parse().unwrap();
        let level: u32 = args.next().unwrap().parse().unwrap();
        let use_frame: bool = args.next().unwrap() == "1";
        let existing: bool = args.next().unwrap() == "1";
        let mut metadata = CodecMetadata::default();
        metadata.size.set(width, height).unwrap();
        let mut header = FrameHeader::new(Some(&metadata));
        header.frame_type = FrameType::DC;
        header.dc_level = level;
        if use_frame {
            header.flags |= FLAG_USE_DC_FRAME;
        }
        let mut state = PassesSharedState::new();
        if existing && level < 4 {
            let mut dc = Image3::create(2, 3).unwrap();
            jxl::jxl::image_ops::zero_fill_image3(&mut dc);
            state.dc_frames.borrow_mut()[level as usize] = dc;
        }
        let status = initialize_passes_shared_state(&header, &mut state, false);
        let mut out = format!(
            "{} {}",
            status_code(&status),
            u32::from(state.dc_source == DcSource::Storage)
        );
        for i in 0..4 {
            write!(out, " {}", u32::from(state.dc_source == DcSource::Frame(i))).unwrap();
        }
        image(&mut out, &state.dc_storage, false);
        image(
            &mut out,
            &state.dc(),
            status.is_ok() && use_frame && existing,
        );
        return out;
    }

    let bytes = unhex(args.next().unwrap());
    let mut m = CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    let parsed = (|| {
        if reader.read_fixed_bits::<16>() != 0x0aff {
            return Err(StatusCode::GenericError);
        }
        jxl::jxl::headers::read_size_header(&mut reader, &mut m.size)?;
        jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut m.m)?;
        m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
        jxl::jxl::fields::bundle::read(&mut reader, &mut m.transform_data)?;
        assert!(
            !m.m.color_encoding.want_icc(),
            "Original sequence fixtures retain their genuine non-ICC metadata"
        );
        reader.jump_to_byte_boundary()?;
        reader.all_reads_within_bounds()
    })();
    let mut offset = reader.total_bits_consumed() / 8;
    let close = reader.close();
    let mut out = format!(
        " H {} {} {}",
        status_code(&parsed),
        reader.total_bits_consumed(),
        status_code(&close)
    );
    if parsed.is_err() || close.is_err() {
        return out;
    }
    write!(out, " M {} {}", m.size.xsize(), m.size.ysize()).unwrap();
    metadata::metadata(&mut out, &m.m);
    // fs-decode-lm constructs each FrameDecoder with use_slow_rendering_pipeline=false.
    let mut decoder = match SequenceDecoder::with_rendering_pipeline(&m, kind != "fs-decode-lm") {
        Ok(v) => {
            write!(out, " P 0").unwrap();
            v
        }
        Err(e) => {
            write!(out, " P {}", e as i32).unwrap();
            return out;
        }
    };
    let identities = std::array::from_fn(|i| {
        decoder.state().shared_storage.reference_frames.borrow()[i]
            .frame
            .as_deref()
            .unwrap() as *const ImageBundle<'_> as usize
    });
    let mut coverage = WriteCoverage::default();
    let mut preview = m.m.have_preview;
    let mut last = false;
    let mut frames = 0;
    let mut status = Ok(());
    while status.is_ok() && !last && frames < 64 {
        let decoded = decoder.decode_frame(&bytes[offset..], preview);
        let t = &decoded.trace;
        coverage.update(&t.write_events);
        write!(
            out,
            " F {} {} {} {} {}",
            frames,
            u32::from(preview),
            status_code(&t.init_status.unwrap()),
            t.header_bits,
            status_code(&t.header_close.unwrap())
        )
        .unwrap();
        frames += 1;
        if let Some(s) = t.output_status {
            write!(out, " A {}", status_code(&s)).unwrap();
        }
        status = decoded.status;
        if t.output_status != Some(Ok(()))
            || t.header_bounds != Some(Ok(()))
            || t.header_close != Some(Ok(()))
        {
            covered_bundle(&mut out, &decoder.output(), &coverage.bundles[0], None);
            state(&mut out, decoder.state(), false, &identities, &coverage);
            break;
        }
        let h = decoded.header.as_ref().unwrap();
        let d = h.to_frame_dimensions();
        write!(
            out,
            " K {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            h.frame_type.0,
            h.encoding.0,
            h.flags,
            h.dc_level,
            u32::from(h.is_last),
            h.save_as_reference,
            u32::from(h.save_before_color_transform),
            frame_saved_as(h),
            h.animation_frame.duration,
            h.animation_frame.timecode,
            h.frame_origin.x0,
            h.frame_origin.y0,
            h.default_xsize(),
            h.default_ysize(),
            h.blending_info.mode.0,
            h.blending_info.source,
            h.color_transform.0,
            u32::from(h.custom_size_or_origin),
            d.xsize,
            d.ysize
        )
        .unwrap();
        write!(
            out,
            " U {} {}",
            h.upsampling,
            h.extra_channel_upsampling.len()
        )
        .unwrap();
        for factor in &h.extra_channel_upsampling {
            write!(out, " {factor}").unwrap();
        }
        write!(
            out,
            " {} {} {}",
            d.num_groups, d.num_dc_groups, h.group_size_shift
        )
        .unwrap();
        let section_status = t.sections_status.unwrap_or(Err(StatusCode::GenericError));
        write!(
            out,
            " S {} {}",
            status_code(&section_status),
            t.section_statuses.len()
        )
        .unwrap();
        for s in &t.section_statuses {
            write!(out, " {s}").unwrap();
        }
        for (bits, closed) in &t.section_readers {
            write!(out, " B {} {}", bits, status_code(closed)).unwrap();
        }
        if let Some(refs) = t.references {
            write!(
                out,
                " D {} {} {}",
                refs,
                u32::from(t.has_decoded_dc),
                u32::from(t.has_decoded_all)
            )
            .unwrap();
            write!(out, " Z {}", status_code(&t.finalize_status.unwrap())).unwrap();
        }
        if status.is_ok() {
            offset += decoder.output().decoded_bytes();
            last = !preview && h.is_last;
        }
        covered_bundle(&mut out, &decoder.output(), &coverage.bundles[0], None);
        state(
            &mut out,
            decoder.state(),
            status.is_ok(),
            &identities,
            &coverage,
        );
        preview = false;
    }
    let eof = offset == bytes.len();
    if status.is_ok() && (!last || !eof) {
        status = Err(StatusCode::GenericError);
    }
    write!(
        out,
        " E {} {} {} {} {} {} {} {}",
        status_code(&status),
        offset,
        bytes.len(),
        frames,
        u32::from(last),
        u32::from(eof),
        decoder.state().visible_frame_index,
        decoder.state().nonvisible_frame_index
    )
    .unwrap();
    out
}

#[test]
fn sequence_references_dc_animation_and_previews_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut complete = std::collections::HashMap::new();
    let fixtures: Vec<_> = include_str!("../data/frame-sequences.requests")
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .collect();
    assert_eq!(fixtures.len(), 113);
    let encoded = original_observations(
        &fixtures
            .iter()
            .map(|(_, request)| request.to_string())
            .collect::<Vec<_>>(),
    );
    for ((name, _), result) in fixtures.iter().zip(encoded) {
        let mut fields = result.split_whitespace();
        let status: i32 = fields.next().unwrap().parse().unwrap();
        if status != 0 {
            assert!(name.starts_with("encoder-unsupported-dc-"));
            continue;
        }
        let bytes = unhex(fields.next().unwrap());
        let header: usize = fields.next().unwrap().parse().unwrap();
        let n: usize = fields.next().unwrap().parse().unwrap();
        let ends: Vec<usize> = (0..n)
            .map(|_| fields.next().unwrap().parse().unwrap())
            .collect();
        let exact = format!("fs-decode {}", hex(&bytes));
        complete.insert(exact.clone(), name);
        cases.push(exact);
        let mut trailing = bytes.clone();
        trailing.push(0);
        cases.push(format!("fs-decode {}", hex(&trailing)));
        cases.push(format!("fs-decode {}", hex(&bytes[..header])));
        let mut start = header;
        for &end in &ends {
            if start != header {
                // Preserve an already completed predecessor while failing the
                // genuine later frame at header, interior and tail boundaries.
                for cut in [start, (start + 1).min(end), start + (end - start) / 2] {
                    cases.push(format!("fs-decode {}", hex(&bytes[..cut])));
                }
                for index in [start, start + (end - start) / 2, end - 1] {
                    let mut broken = bytes.clone();
                    broken[index] ^= 0x80;
                    cases.push(format!("fs-decode {}", hex(&broken)));
                }
            }
            start = end;
        }
        for end in ends {
            let mut cuts = vec![end.saturating_sub(1), end];
            cuts.sort_unstable();
            cuts.dedup();
            for cut in cuts {
                if cut < bytes.len() {
                    cases.push(format!("fs-decode {}", hex(&bytes[..cut])));
                }
            }
        }
        let mut indexes = vec![
            header,
            (header + 2).min(bytes.len() - 1),
            bytes.len() / 2,
            bytes.len() - 1,
        ];
        indexes.sort_unstable();
        indexes.dedup();
        for index in indexes {
            let mut broken = bytes.clone();
            broken[index] ^= 0x80;
            cases.push(format!("fs-decode {}", hex(&broken)));
        }
    }
    assert_eq!(cases.len(), 1776); // All 1,110 old cases plus 666 later-frame failures.
    let mut failures_with_references = 0;
    let mut failures_with_dc = 0;
    for chunk in cases.chunks(8) {
        let originals = original_observations(chunk);
        for (request, cpp) in chunk.iter().zip(originals) {
            let tokens: Vec<_> = cpp.split_whitespace().collect();
            for fields in tokens.windows(4).filter(|fields| fields[0] == "R") {
                assert_eq!(fields[2], "1", "source reference bundle identity changed");
            }
            let last_state = tokens.iter().rposition(|&token| token == "Q").unwrap();
            let reference_mask: usize = tokens[last_state + 3].parse().unwrap();
            let dc_mask: usize = tokens[last_state + 4].parse().unwrap();
            let end = tokens.iter().rposition(|&token| token == "E").unwrap();
            if tokens[end + 1] != "0" {
                failures_with_references += usize::from(reference_mask != 0);
                failures_with_dc += usize::from(
                    dc_mask != 0
                        && tokens
                            .windows(2)
                            .any(|words| words[0] == "V" && words[1] != "0"),
                );
            }
            if let Some(&name) = complete.get(request) {
                if !name.starts_with("invalid-reference-") {
                    assert_eq!(
                        tokens[end + 1],
                        "0",
                        "complete original fixture failed: {name}"
                    );
                    assert_eq!(&tokens[end + 5..end + 7], ["1", "1"]);
                }
                if name.starts_with("preview-main-") {
                    let frames: Vec<_> = tokens
                        .windows(3)
                        .filter(|v| v[0] == "F")
                        .map(|v| (v[1], v[2]))
                        .collect();
                    assert_eq!(frames, [("0", "1"), ("1", "0")]);
                    let first = tokens.iter().position(|&v| v == "K").unwrap();
                    assert_eq!(
                        tokens[first + 5],
                        "1",
                        "preview is_last must not terminate main decoding"
                    );
                }
                if name.starts_with("save-") {
                    let slot: usize = name.rsplit('-').nth(1).unwrap().parse().unwrap();
                    assert_eq!(reference_mask, 1 << slot);
                }
            }
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &cpp) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = cpp.split_whitespace().collect();
                let differences: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(16)
                    .collect();
                panic!(
                    "request {request}: lengths {}/{}; differences {differences:?}",
                    a.len(),
                    b.len()
                );
            }
        }
    }
    assert!(failures_with_references > 0);
    assert!(failures_with_dc > 0);
    println!(
        "{} C++/Rust differential cases matched; {failures_with_references} failures retain initialized references; {failures_with_dc} retain initialized DC pixels",
        cases.len()
    );
}

#[test]
fn direct_dc_levels_and_missing_slots_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let cases: Vec<_> = (0..=4)
        .flat_map(|level| {
            (0..2).flat_map(move |use_frame| {
                (0..2)
                    .map(move |existing| format!("fs-dc-state 65 9 {level} {use_frame} {existing}"))
            })
        })
        .collect();
    assert_eq!(cases.len(), 20);
    compare(cases);
}

#[test]
fn original_section_reader_close_keeps_written_output_and_uncommitted_reference_pixels() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let requests: Vec<_> = include_str!("../data/frame-sequences.requests")
        .lines()
        .filter_map(|line| {
            let (name, request) = line.split_once('\t').unwrap();
            name.starts_with("xyb-saved-before-")
                .then(|| request.to_string())
        })
        .collect();
    assert_eq!(requests.len(), 2);
    let encoded = original_observations(&requests);
    let mut writer_requests = Vec::new();
    let mut genuine = Vec::new();
    for result in encoded {
        let words: Vec<_> = result.split_whitespace().collect();
        assert_eq!(words[0], "0");
        genuine.push(words[1].to_string());
        for remove in 0..9 {
            writer_requests.push(format!("fs-section-size {} 0 0 {remove}", words[1]));
        }
    }
    let changed = original_observations(&writer_requests);
    assert_eq!(changed.len(), 18);
    for (i, wire) in changed.iter().enumerate() {
        if i % 9 == 0 {
            assert_eq!(
                wire,
                &genuine[i / 9],
                "original writer control must preserve the encoder bytes"
            );
        }
    }
    let cases: Vec<_> = changed
        .into_iter()
        .map(|wire| format!("fs-decode {wire}"))
        .collect();
    for (i, request) in cases.iter().enumerate() {
        let original = original_observations(std::slice::from_ref(request))
            .pop()
            .unwrap();
        if i % 9 == 1 {
            let words: Vec<_> = original.split_whitespace().collect();
            let sections = words.iter().position(|&v| v == "S").unwrap();
            assert_eq!(&words[sections + 1..sections + 4], ["0", "1", "0"]);
            let reader = words.iter().position(|&v| v == "B").unwrap();
            assert_eq!(words[reader + 2], "1");
            assert!(
                !words.contains(&"Z"),
                "close failure precedes FinalizeFrame"
            );
            let state = words.iter().position(|&v| v == "Q").unwrap();
            assert_eq!(
                words[state + 3],
                "0",
                "written storage is not a committed reference"
            );
            let storage = words.iter().position(|&v| v == "T").unwrap();
            assert_eq!(words[storage + 1], "-1");
            for index in words
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| (v == "L").then_some(i))
            {
                assert_eq!(words[index + 1], "3");
                let mut at = index + 2;
                for channel in 0..3 {
                    assert_eq!(words[at], channel.to_string());
                    assert_eq!(words[at + 1], "153");
                    at += 2 + 153 * 3;
                }
            }
        }
        super::assert_parity(request, &super::observe(request), &original);
    }
    println!(
        "18 C++/Rust differential cases matched; 2 deferred close errors retain all written output and uncommitted reference pixels"
    );
}

pub(super) struct EncodedSequence {
    pub(super) bytes: Vec<u8>,
    pub(super) header: usize,
    pub(super) ends: Vec<usize>,
    pub(super) status: i32,
}
pub(super) fn extended_encodings(prefix: &str) -> Vec<(String, String, EncodedSequence)> {
    let fixtures: Vec<_> = include_str!("../data/frame-sequences-extended.requests")
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .filter(|(name, _)| name.starts_with(prefix))
        .collect();
    let encoded = original_observations(
        &fixtures
            .iter()
            .map(|(_, r)| r.to_string())
            .collect::<Vec<_>>(),
    );
    fixtures
        .into_iter()
        .zip(encoded)
        .map(|((name, request), line)| {
            let words: Vec<_> = line.split_whitespace().collect();
            let status = words[0].parse().unwrap();
            let bytes = unhex(words[1]);
            let header = words[2].parse().unwrap();
            let n: usize = words[3].parse().unwrap();
            let ends = words[4..4 + n].iter().map(|v| v.parse().unwrap()).collect();
            if request.starts_with("fs-encode-ext ") {
                let source: Vec<_> = request.split_whitespace().collect();
                let count: usize = source[9].parse().unwrap();
                let at = words.iter().position(|&v| v == "P").unwrap();
                assert_eq!(words[at + 1].parse::<usize>().unwrap(), count);
                let readback: Vec<_> = words[at + 2..]
                    .iter()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect();
                let requested: Vec<_> = source[10..]
                    .as_chunks::<22>()
                    .0
                    .iter()
                    .flat_map(|f| f[19..22].iter().map(|v| v.parse::<i32>().unwrap()))
                    .collect();
                assert_eq!(
                    readback, requested,
                    "actual ParamsPostInit readback: {name}"
                );
            }
            (
                name.to_string(),
                request.to_string(),
                EncodedSequence {
                    bytes,
                    header,
                    ends,
                    status,
                },
            )
        })
        .collect()
}
pub(super) fn sequence_variants(encoded: &EncodedSequence) -> Vec<String> {
    let bytes = &encoded.bytes;
    let mut cases = vec![
        format!("fs-decode {}", hex(bytes)),
        format!("fs-decode {}00", hex(bytes)),
        format!("fs-decode {}", hex(&bytes[..encoded.header])),
    ];
    let mut start = encoded.header;
    for &end in &encoded.ends {
        for cut in [start, start + (end - start) / 2, end - 1, end] {
            if cut < bytes.len() {
                cases.push(format!("fs-decode {}", hex(&bytes[..cut])));
            }
        }
        let mut broken = bytes.clone();
        broken[end - 1] ^= 0x80;
        cases.push(format!("fs-decode {}", hex(&broken)));
        start = end;
    }
    cases
}
fn strict_sequence_cases(cases: Vec<String>) {
    for chunk in cases.chunks(4) {
        let originals = original_observations(chunk);
        for (request, original) in chunk.iter().zip(originals) {
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &original) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = original.split_whitespace().collect();
                let differences: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(16)
                    .collect();
                panic!(
                    "request {request}: lengths {}/{}; differences {differences:?}",
                    a.len(),
                    b.len()
                );
            }
        }
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}
fn source_upsampling(observation: &str) -> Vec<(usize, Vec<usize>, usize, usize, usize)> {
    let words: Vec<_> = observation.split_whitespace().collect();
    words
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if v != "U" {
                return None;
            }
            let main = words[i + 1].parse().unwrap();
            let n: usize = words[i + 2].parse().unwrap();
            Some((
                main,
                words[i + 3..i + 3 + n]
                    .iter()
                    .map(|v| v.parse().unwrap())
                    .collect(),
                words[i + 3 + n].parse().unwrap(),
                words[i + 4 + n].parse().unwrap(),
                words[i + 5 + n].parse().unwrap(),
            ))
        })
        .collect()
}
#[test]
fn encoder_controlled_main_and_early_late_extra_upsampling_keep_source_acceptance() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let fixtures = extended_encodings("resampling-");
    assert_eq!(fixtures.len(), 46);
    let mut cases = Vec::new();
    let mut early = 0;
    let mut late = 0;
    let mut main_only = 0;
    let mut source_rejections = 0;
    for (name, request, encoded) in fixtures {
        assert_eq!(encoded.status, 0, "genuine source encoder: {name}");
        let source: Vec<_> = request.split_whitespace().collect();
        let main: usize = source[29].parse().unwrap();
        let ec: usize = source[30].parse().unwrap();
        let alpha = source[4] == "1";
        let complete = original_observations(&[format!("fs-decode {}", hex(&encoded.bytes))])
            .pop()
            .unwrap();
        let reached = source_upsampling(&complete);
        assert_eq!(reached.len(), 1);
        assert_eq!(
            reached[0],
            (
                main,
                if alpha {
                    vec![main.max(ec)]
                } else {
                    Vec::new()
                },
                1,
                1,
                1
            )
        );
        let status = complete
            .split(" E ")
            .last()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();
        let odd_modular = source[5] == "0" && source[1] == "17" && alpha && ec > main;
        if odd_modular {
            assert_eq!(status, "1");
            source_rejections += 1;
        } else {
            assert_eq!(status, "0", "source successful resampling: {name}");
            early += usize::from(alpha && ec > main);
            late += usize::from(alpha && main > 1 && ec <= main);
            main_only += usize::from(!alpha && main > 1);
        }
        cases.extend(sequence_variants(&encoded));
    }
    assert_eq!((early, late, main_only, source_rejections), (12, 18, 6, 6));
    let rejected = extended_encodings("encoder-unsupported-resampling-");
    assert_eq!(rejected.len(), 6);
    for (name, _, encoded) in rejected {
        assert_eq!(encoded.status, 1, "source encoder rejection: {name}");
    }
    strict_sequence_cases(cases);
}
#[test]
fn multigroup_reference_reuse_animation_and_preview_keep_exact_defined_snapshots() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let fixtures = extended_encodings("multigroup-");
    assert_eq!(fixtures.len(), 12);
    let mut cases = Vec::new();
    for (name, request, encoded) in fixtures {
        assert_eq!(encoded.status, 0, "source multigroup encoder: {name}");
        let complete = original_observations(&[format!("fs-decode {}", hex(&encoded.bytes))])
            .pop()
            .unwrap();
        let source: Vec<_> = request.split_whitespace().collect();
        let count = source[9].parse::<usize>().unwrap();
        let preview = source[7] != "0";
        let reached = source_upsampling(&complete);
        assert_eq!(reached.len(), count);
        for (i, entry) in reached.iter().enumerate() {
            assert_eq!(entry.0, 1);
            assert!(entry.1.is_empty());
            assert_eq!((entry.3, entry.4), (1, 1));
            assert_eq!(
                entry.2,
                if i == 0 && preview {
                    1
                } else {
                    if source[1] == "513" { 3 } else { 2 }
                }
            );
        }
        let end: Vec<_> = complete
            .split(" E ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect();
        assert_eq!(&end[..1], ["0"]);
        assert_eq!(end[3].parse::<usize>().unwrap(), count);
        assert_eq!(&end[4..], ["1", "1", "2", "0"]);
        assert!(complete.contains(" D 8 1 1"));
        assert!(complete.contains(" D 2 1 1"));
        cases.extend(sequence_variants(&encoded));
    }
    strict_sequence_cases(cases);
}
#[test]
fn source_writer_dc3_dc4_ladders_keep_payloads_dependencies_and_error_snapshots() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut originals = extended_encodings("dc-ladder-base");
    assert_eq!(originals.len(), 1);
    let (_, _, original) = originals.pop().unwrap();
    assert_eq!(original.status, 0);
    let requests: Vec<_> = (2..=4)
        .map(|highest| format!("fs-dc-ladder {} {highest}", hex(&original.bytes)))
        .collect();
    let rewritten = original_observations(&requests);
    let mut cases = Vec::new();
    for (highest, line) in (2..=4).zip(rewritten) {
        let words: Vec<_> = line.split_whitespace().collect();
        let bytes = unhex(words[0]);
        if highest == 2 {
            assert_eq!(
                bytes, original.bytes,
                "actual source writer exact-byte control"
            );
        }
        assert_eq!(&words[1..5], ["D", &(highest + 1).to_string(), "C", "1"]);
        let at = words.iter().rposition(|&v| v == "E").unwrap();
        assert_eq!(words[at + 1].parse::<usize>().unwrap(), highest + 1);
        let ends = words[at + 2..].iter().map(|v| v.parse().unwrap()).collect();
        let encoded = EncodedSequence {
            bytes,
            header: original.header,
            ends,
            status: 0,
        };
        let complete = original_observations(&[format!("fs-decode {}", hex(&encoded.bytes))])
            .pop()
            .unwrap();
        let tokens: Vec<_> = complete.split_whitespace().collect();
        let levels: Vec<_> = tokens
            .windows(5)
            .filter(|v| v[0] == "K")
            .map(|v| v[4].parse::<usize>().unwrap())
            .collect();
        assert_eq!(levels, (0..=highest).rev().collect::<Vec<_>>());
        let dependencies: Vec<_> = tokens
            .windows(4)
            .filter(|v| v[0] == "D")
            .map(|v| v[1].parse::<usize>().unwrap())
            .collect();
        let mut expected = vec![0];
        expected.extend((0..highest).rev().map(|level| 16 << level));
        assert_eq!(dependencies, expected);
        let state = tokens.iter().rposition(|&v| v == "Q").unwrap();
        assert_eq!(
            tokens[state + 4].parse::<usize>().unwrap(),
            (0..highest).map(|level| 16 << level).sum()
        );
        let end: Vec<_> = complete
            .split(" E ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect();
        assert_eq!(end[0], "0");
        assert_eq!(end[3].parse::<usize>().unwrap(), highest + 1);
        assert_eq!(&end[4..], ["1", "1", "1", "0"]);
        cases.extend(sequence_variants(&encoded));
    }
    strict_sequence_cases(cases);
}

#[test]
fn encoder_generated_patches_render_exact_glyphs_and_preserve_references_on_errors() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let (_, request) = include_str!("../data/frame-sequences-patches.requests")
        .trim()
        .split_once('\t')
        .unwrap();
    let encoded = original_observations(&[request.to_string()]).pop().unwrap();
    let words: Vec<_> = encoded.split_whitespace().collect();
    assert_eq!(words[0], "0");
    let bytes = unhex(words[1]);
    assert_eq!(
        bytes,
        include_bytes!("../data/repeated-glyphs-modular-none.jxl")
    );
    let frame_ends = original_observations(&[format!("fs-frame-ends {}", hex(&bytes))])
        .pop()
        .unwrap();
    let ends: Vec<usize> = frame_ends
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    assert_eq!(ends, [2, 40, 112]);
    let encoded = EncodedSequence {
        bytes,
        header: words[2].parse().unwrap(),
        ends: ends[1..].to_vec(),
        status: 0,
    };
    let complete = original_observations(&[format!("fs-decode {}", hex(&encoded.bytes))])
        .pop()
        .unwrap();
    let tokens: Vec<_> = complete.split_whitespace().collect();
    let headers: Vec<_> = tokens.windows(21).filter(|v| v[0] == "K").collect();
    assert_eq!(&headers[0][1..9], ["2", "1", "0", "0", "0", "3", "1", "8"]);
    assert_eq!(&headers[0][19..21], ["12", "12"]);
    assert_eq!(&headers[1][1..9], ["0", "1", "2", "0", "1", "0", "0", "0"]);
    let pd = tokens.iter().rposition(|&v| v == "PD").unwrap();
    assert_eq!(&tokens[pd + 1..pd + 5], ["1", "8", "24", "2"]);
    assert!(complete.contains(" D 8 1 1 Z 0"));
    assert!(complete.ends_with(" E 0 112 112 2 1 1 1 0"));
    // This is the actual output-writer coverage, not an assumption based on
    // allocation or final status. Every initialized sample equals the input.
    let output = tokens[..pd].iter().rposition(|&v| v == "L").unwrap();
    assert_eq!(tokens[output + 1], "3");
    let mut at = output + 2;
    for c in 0..3 {
        assert_eq!(tokens[at].parse::<usize>().unwrap(), c);
        assert_eq!(tokens[at + 1].parse::<usize>().unwrap(), 128 * 96);
        at += 2;
        for y in 0..96 {
            for x in 0..128 {
                assert_eq!(tokens[at].parse::<usize>().unwrap(), x);
                assert_eq!(tokens[at + 1].parse::<usize>().unwrap(), y);
                let black = (5..13).contains(&(x % 24))
                    && (5..17).contains(&(y % 24))
                    && (x % 24 < 7 || x % 24 >= 11 || y % 24 == 10 || y % 24 == 11);
                assert_eq!(
                    tokens[at + 2].parse::<u32>().unwrap(),
                    if black { 0.0_f32 } else { 1.0_f32 }.to_bits()
                );
                at += 3;
            }
        }
    }
    let mut cases = sequence_variants(&encoded);
    // Both boundaries are parsed by the original FrameHeader/TOC routines.
    // Exercise every main-frame byte, retaining the completed reference slot
    // and all writer-defined output/storage spans on each later failure.
    for index in encoded.ends[0]..encoded.bytes.len() {
        cases.push(format!("fs-decode {}", hex(&encoded.bytes[..index])));
        let mut broken = encoded.bytes.clone();
        broken[index] ^= 0x80;
        cases.push(format!("fs-decode {}", hex(&broken)));
    }
    let mut reserved_flags = encoded.bytes.clone();
    reserved_flags[encoded.ends[0]] ^= 0x80;
    let reserved = original_observations(&[format!("fs-decode {}", hex(&reserved_flags))])
        .pop()
        .unwrap();
    assert!(reserved.contains(" K 0 1 4 0 1 0 0 0"));
    assert!(
        reserved.contains(" A 0 K 0 1 4"),
        "reserved flags preserve InitFrameOutput before the later source error"
    );
    let snapshots = original_observations(&cases);
    let mut retained_reference_errors = 0;
    let mut partial_dictionary_errors = 0;
    for snapshot in snapshots {
        assert!(
            !snapshot.contains(" R 0 0 ")
                && !snapshot.contains(" R 1 0 ")
                && !snapshot.contains(" R 2 0 ")
                && !snapshot.contains(" R 3 0 ")
        );
        let tokens: Vec<_> = snapshot.split_whitespace().collect();
        let end = tokens.iter().rposition(|&v| v == "E").unwrap();
        if tokens[end + 1] != "0" {
            let state = tokens.iter().rposition(|&v| v == "Q").unwrap();
            retained_reference_errors += usize::from(tokens[state + 3] == "8");
            let pd = tokens.iter().rposition(|&v| v == "PD").unwrap();
            partial_dictionary_errors += usize::from(tokens[pd + 3] != "0");
        }
    }
    assert_eq!(retained_reference_errors, 133);
    assert_eq!(partial_dictionary_errors, 39);
    strict_sequence_cases(cases);
}
