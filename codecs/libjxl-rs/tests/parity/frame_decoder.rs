// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Drive the Rust FrameDecoder exactly as the oracle's decode.cc-shaped driver
// drives the original: one decoder and bundle per frame, partial input,
// section selection, reader checks, progressive pauses and Flush.
use super::frame_sequences::{WriteCoverage, covered_bundle, state as state_snapshot};
use super::*;
use jxl::jxl::dec_cache::{PassesDecoderState, PixelCallback};
use jxl::jxl::dec_frame::{FrameDecoder, FrameWriteTrace, SectionInfo, SectionStatus};
use jxl::jxl::decode::JxlProgressiveDetail;
use jxl::jxl::frame_header::FrameType;
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::memory_manager::{JxlMemoryManager, memory_manager_init};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn handles(kind: &str) -> bool {
    kind == "ifd-decode"
}

fn bytes_per_channel(data_type: JxlDataType) -> usize {
    if data_type == JxlDataType::UINT8 {
        1
    } else if data_type == JxlDataType::FLOAT {
        4
    } else {
        2
    }
}

fn buffer(out: &mut String, tag: &str, bytes: &[u8]) {
    write!(out, " {tag} {} {}", bytes.len(), hex(bytes)).unwrap();
}

// Pixel callback record; rows are stored at the buffer output layout.
struct Callback {
    out: *mut u8,
    size: usize,
    stride: usize,
    pixel_bytes: usize,
    inits: usize,
    threads: usize,
    pixels: usize,
    runs: usize,
    destroys: usize,
}
impl Default for Callback {
    fn default() -> Self {
        Self {
            out: std::ptr::null_mut(),
            size: 0,
            stride: 0,
            pixel_bytes: 0,
            inits: 0,
            threads: 0,
            pixels: 0,
            runs: 0,
            destroys: 0,
        }
    }
}
unsafe extern "C" fn callback_init(
    opaque: *mut std::ffi::c_void,
    threads: usize,
    pixels: usize,
) -> *mut std::ffi::c_void {
    // SAFETY: The opaque value is a live boxed record owned by the observer.
    let r = unsafe { &mut *opaque.cast::<Callback>() };
    r.inits += 1;
    r.threads = threads;
    r.pixels = pixels;
    opaque
}
unsafe extern "C" fn callback_run(
    opaque: *mut std::ffi::c_void,
    _thread: usize,
    x: usize,
    y: usize,
    n: usize,
    pixels: *const std::ffi::c_void,
) {
    // SAFETY: The opaque value is a live boxed record whose `out` addresses
    // its `size`-byte buffer; the stage supplies `n` pixels of `pixel_bytes`.
    unsafe {
        let r = &mut *opaque.cast::<Callback>();
        let at = y * r.stride + x * r.pixel_bytes;
        if at > r.size || n * r.pixel_bytes > r.size - at {
            std::process::abort();
        }
        std::ptr::copy_nonoverlapping(pixels.cast::<u8>(), r.out.add(at), n * r.pixel_bytes);
        r.runs += 1;
    }
}
unsafe extern "C" fn callback_destroy(opaque: *mut std::ffi::c_void) {
    // SAFETY: The opaque value is a live boxed record owned by the observer.
    unsafe { (*opaque.cast::<Callback>()).destroys += 1 };
}

// The C++ observer's AfterSections: reference storage without color (for
// example an unwritten coalescing-off bundle) starts a new coverage generation.
fn after_sections(coverage: &mut WriteCoverage, state: &PassesDecoderState<'_, '_>) {
    if state.frame_storage_for_referencing.borrow().color().xsize() == 0 {
        coverage.bundles[1] = Default::default();
    }
}

fn flag(args: &mut std::str::SplitWhitespace<'_>) -> bool {
    number::<u8>(args) != 0
}

#[expect(
    clippy::too_many_lines,
    reason = "Mirror the oracle driver's single decode.cc-shaped loop."
)]
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let bytes = unhex(args.next().unwrap());
    let limit: usize = number(args);
    let delivery: usize = number(args);
    let seed: u64 = number(args);
    let coalescing = flag(args);
    let spot = flag(args);
    let detail: u32 = number(args);
    let flush: usize = number(args);
    let mut skip: usize = number(args);
    let channels: u32 = number(args);
    let data_type = JxlDataType(number(args));
    let unpremul = flag(args);
    let undo = flag(args);
    let extra = flag(args);
    let callback = flag(args);
    let slow = flag(args);
    assert!(limit <= bytes.len() && delivery <= 3 && flush <= 2);
    let data = &bytes[..limit];
    let mut m = CodecMetadata::default();
    let mut reader = BitReader::new(data);
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
            "incremental fixtures retain their genuine non-ICC metadata"
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
    // Output buffers outlive every pipeline that may retain their addresses:
    // `state` is declared later and is dropped first.
    let mut keep: Vec<Vec<u8>> = Vec::new();
    let mut records: Vec<Box<Callback>> = Vec::new();
    let mut manager = JxlMemoryManager::default();
    // SAFETY: Both pointers address this live local manager; a null source
    // manager selects the original default callbacks.
    unsafe { memory_manager_init(&mut manager, std::ptr::null()) }.unwrap();
    let mut state = PassesDecoderState::new();
    let status = state.output_encoding_info.set_from_metadata(&m);
    write!(out, " P {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    let identities = std::array::from_fn(|i| {
        state.shared_storage.reference_frames.borrow()[i]
            .frame
            .as_deref()
            .unwrap() as *const ImageBundle<'_> as usize
    });
    let trace: FrameWriteTrace = Rc::new(RefCell::new(Vec::new()));
    let mut coverage = WriteCoverage::default();
    let drain = |coverage: &mut WriteCoverage| {
        let events: Vec<_> = trace.borrow_mut().drain(..).collect();
        coverage.update(&events);
    };
    let mut rng = seed.wrapping_mul(2).wrapping_add(1);
    let mut preview = m.m.have_preview;
    let mut last_total = false;
    let mut frames = 0usize;
    while frames < 64 && !last_total {
        if offset > limit {
            out.push_str(" @N");
            break;
        }
        let ib = Rc::new(RefCell::new(ImageBundle::new(Some(&m.m))));
        // SAFETY: Every output descriptor installed below addresses a `keep`
        // buffer that outlives `state` and therefore every pipeline. The
        // manager is a live initialized local default manager, and the state
        // receives no CMS (SetFromMetadata leaves cms_set false). The pool is null.
        let mut frame = unsafe { FrameDecoder::new(&m, &manager, std::ptr::null(), slow) }
            .expect("incremental fixtures are inside the admitted profile");
        frame.set_write_trace(Some(trace.clone()));
        let mut header = BitReader::new(&data[offset..]);
        let status = frame.init_frame(&mut state, &mut header, ib.clone(), preview);
        drain(&mut coverage);
        // The fresh bundle has no color after InitFrame.
        coverage.bundles[0] = Default::default();
        let within = header.all_reads_within_bounds().is_ok();
        let bits = header.total_bits_consumed();
        let close = header.close();
        write!(
            out,
            " @F {frames} {} {} {} {bits} {}",
            u32::from(preview),
            status_code(&status),
            u32::from(within),
            status_code(&close)
        )
        .unwrap();
        frames += 1;
        if !within || status == Err(StatusCode::NotEnoughBytes) {
            out.push_str(" @N");
            break;
        }
        if status.is_err() {
            break;
        }
        offset += bits / 8;
        let h = frame.get_frame_header().clone();
        let d = h.to_frame_dimensions();
        write!(
            out,
            " @K {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            h.frame_type.0,
            h.encoding.0,
            h.flags,
            h.dc_level,
            u32::from(h.is_last),
            h.save_as_reference,
            u32::from(h.save_before_color_transform),
            FrameDecoder::saved_as(&h),
            h.animation_frame.duration,
            u32::from(h.custom_size_or_origin),
            h.blending_info.mode.0,
            h.passes.num_passes,
            h.passes.num_downsample,
            d.xsize,
            d.ysize,
            d.num_groups,
            d.num_dc_groups
        )
        .unwrap();
        let status = frame.init_frame_output(&mut state);
        write!(out, " @A {}", status_code(&status)).unwrap();
        if status.is_err() {
            break;
        }
        let toc = frame.toc().to_vec();
        let n = toc.len();
        let remaining = frame.sum_section_sizes() as usize;
        write!(out, " @T {n} {remaining}").unwrap();
        for entry in &toc {
            write!(out, " {} {}", entry.size, entry.id).unwrap();
        }
        let mut skipping = false;
        let mut last_still = false;
        if !preview {
            last_total = h.is_last;
            last_still = h.is_last
                || h.animation_frame.duration > 0
                || (!coalescing && h.frame_type == FrameType::REGULAR);
            if skip > 0 {
                skipping = true;
                if last_still {
                    skip -= 1;
                }
            }
            if skipping && !(h.can_be_referenced() || h.frame_type == FrameType::DC) {
                out.push_str(" @X");
                offset += remaining;
                continue;
            }
        }
        frame.set_render_spotcolors(spot);
        frame.set_coalescing(coalescing);
        let effective = if preview {
            JxlProgressiveDetail::FRAMES
        } else {
            frame.set_pause_at_progressive(JxlProgressiveDetail(detail))
        };
        write!(out, " @G {}", effective.0).unwrap();
        let want = channels != 0 && (preview || (last_still && !skipping));
        let (mut xsize, mut ysize) = (0, 0);
        let format = JxlPixelFormat {
            num_channels: channels,
            data_type,
            endianness: JxlEndianness::NATIVE,
            align: 0,
        };
        let ec_format = JxlPixelFormat {
            num_channels: 1,
            ..format
        };
        let bpc = bytes_per_channel(data_type);
        let (mut main_index, mut first_extra) = (0, 0);
        let mut record: Option<*mut Callback> = None;
        if want {
            if preview {
                xsize = m.oriented_preview_xsize(!undo);
                ysize = m.oriented_preview_ysize(!undo);
            } else {
                xsize = m.oriented_xsize(!undo);
                ysize = m.oriented_ysize(!undo);
                if !coalescing {
                    xsize = d.xsize_upsampled;
                    ysize = d.ysize_upsampled;
                    if undo && m.m.get_orientation().0 > 4 {
                        std::mem::swap(&mut xsize, &mut ysize);
                    }
                }
            }
            main_index = keep.len();
            keep.push(vec![0xa5; bpc * channels as usize * xsize * ysize]);
            first_extra = keep.len();
            if callback {
                let pixel_bytes = bpc * channels as usize;
                records.push(Box::new(Callback {
                    out: keep[main_index].as_mut_ptr(),
                    size: keep[main_index].len(),
                    stride: pixel_bytes * xsize,
                    pixel_bytes,
                    ..Callback::default()
                }));
                record = Some(&mut **records.last_mut().unwrap() as *mut Callback);
            }
            if extra {
                for _ in 0..m.m.extra_channel_info.len() {
                    keep.push(vec![0x5a; bpc * xsize * ysize]);
                }
            }
        }
        let extras = if want && extra {
            m.m.extra_channel_info.len()
        } else {
            0
        };
        write!(out, " @W {} {xsize} {ysize}", u32::from(want)).unwrap();
        let dump = |out: &mut String, keep: &[Vec<u8>]| {
            buffer(out, "@Y", &keep[main_index]);
            if let Some(record) = record {
                // SAFETY: The boxed record outlives the decoder state, and no
                // callback runs while this snapshot reads its counters.
                let r = unsafe { &*record };
                write!(
                    out,
                    " @C {} {} {} {} {}",
                    r.inits, r.threads, r.pixels, r.runs, r.destroys
                )
                .unwrap();
            }
            for i in 0..extras {
                buffer(out, "@YE", &keep[first_extra + i]);
            }
        };
        let mut starts = vec![0; n];
        let mut position = offset;
        for i in 0..n {
            starts[i] = position;
            position += toc[i].size;
        }
        let mut order: Vec<usize> = (0..n).collect();
        if delivery >= 2 {
            for i in (1..n).rev() {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                order.swap(i, (rng % (i as u64 + 1)) as usize);
            }
        }
        out.push_str(" @Q");
        for i in &order {
            write!(out, " {i}").unwrap();
        }
        let mut processed = vec![0u8; n];
        let (mut next_section, mut available, mut cursor, mut idle) = (0, 1, 0, 0usize);
        let (mut dc_done, mut done) = (false, false);
        let end_of = |k: usize| {
            limit.min(if k == 0 {
                offset
            } else {
                starts[k - 1] + toc[k - 1].size
            })
        };
        let mut avail_end = if delivery == 1 {
            end_of(available)
        } else {
            limit
        };
        for call in 0.. {
            assert!(call < 4096, "incremental call limit");
            if want {
                // decode.cc installs the output on every kFull iteration.
                let main = &mut keep[main_index];
                let (pixel_callback, image_buffer, image_buffer_size) = match record {
                    Some(record) => (
                        PixelCallback::new(
                            Some(callback_init),
                            Some(callback_run),
                            Some(callback_destroy),
                            record.cast(),
                        ),
                        std::ptr::null_mut(),
                        0,
                    ),
                    None => (
                        PixelCallback::default(),
                        main.as_mut_ptr().cast(),
                        main.len(),
                    ),
                };
                let set = frame.set_image_output(
                    &mut state,
                    pixel_callback,
                    image_buffer,
                    image_buffer_size,
                    xsize,
                    ysize,
                    format,
                    bpc * 8,
                    unpremul,
                    undo,
                );
                write!(out, " @I {}", status_code(&set)).unwrap();
                for i in 0..extras {
                    let ec = &mut keep[first_extra + i];
                    let added = frame.add_extra_channel_output(
                        &mut state,
                        ec.as_mut_ptr().cast(),
                        ec.len(),
                        xsize,
                        ec_format,
                        bpc * 8,
                    );
                    write!(out, " {}", status_code(&added)).unwrap();
                }
            }
            let next_pause = frame.next_num_passes_to_pause();
            let mut chosen = Vec::new();
            if delivery <= 1 {
                for i in next_section..n {
                    if processed[i] != 0 {
                        continue;
                    }
                    if starts[i] + toc[i].size > avail_end {
                        break;
                    }
                    chosen.push(i);
                }
            } else if delivery == 2 {
                for k in 0..n {
                    let i = order[(cursor + k) % n];
                    if processed[i] == 0 && starts[i] + toc[i].size <= limit {
                        chosen.push(i);
                        cursor = (cursor + k + 1) % n;
                        break;
                    }
                }
            } else {
                for &i in &order {
                    if processed[i] == 0 && starts[i] + toc[i].size <= limit {
                        chosen.push(i);
                    }
                }
            }
            let mut readers: Vec<_> = chosen
                .iter()
                .map(|&i| BitReader::new(&data[starts[i]..starts[i] + toc[i].size]))
                .collect();
            let mut statuses = vec![SectionStatus::default(); chosen.len()];
            let status = {
                let mut sections: Vec<_> = readers
                    .iter_mut()
                    .zip(&chosen)
                    .map(|(br, &i)| SectionInfo {
                        br,
                        id: toc[i].id,
                        index: i,
                    })
                    .collect();
                frame.process_sections(&mut state, &mut sections, &mut statuses)
            };
            drain(&mut coverage);
            after_sections(&mut coverage, &state);
            write!(out, " @S {} {}", status_code(&status), chosen.len()).unwrap();
            for (&i, s) in chosen.iter().zip(&statuses) {
                write!(out, " {i} {} {}", toc[i].id, *s as u8).unwrap();
            }
            let (mut out_of_bounds, mut close_error) = (false, false);
            for section in &mut readers {
                let within = section.all_reads_within_bounds().is_ok();
                out_of_bounds |= !within;
                let consumed = section.total_bits_consumed();
                let closed = section.close();
                close_error |= closed.is_err();
                write!(
                    out,
                    " @B {consumed} {} {}",
                    u32::from(within),
                    status_code(&closed)
                )
                .unwrap();
            }
            let mut outcome = if close_error {
                1
            } else if out_of_bounds {
                2
            } else if status.is_err() {
                3
            } else {
                0
            };
            let mut progress = 0;
            if outcome == 0 {
                for (&i, s) in chosen.iter().zip(&statuses) {
                    if *s == SectionStatus::Done {
                        processed[i] = 1;
                        progress += 1;
                    } else if *s != SectionStatus::Skipped {
                        outcome = 4;
                        break;
                    }
                }
            }
            write!(out, " @O {outcome}").unwrap();
            if outcome != 0 {
                break;
            }
            while next_section < n && processed[next_section] != 0 {
                next_section += 1;
            }
            let all = frame.has_decoded_all();
            let dc = frame.has_decoded_dc();
            let passes = frame.num_complete_passes();
            write!(
                out,
                " @D {} {} {passes} {} {next_section}",
                u32::from(dc),
                u32::from(all),
                frame.next_num_passes_to_pause()
            )
            .unwrap();
            let mut event = 0;
            if effective >= JxlProgressiveDetail::DC && !dc_done && !all && dc {
                dc_done = true;
                event = 1;
                out.push_str(" @V 1 8");
            } else if !all && effective >= JxlProgressiveDetail::LAST_PASSES && passes >= next_pause
            {
                event = 2;
                write!(
                    out,
                    " @V 2 {}",
                    h.passes
                        .get_downsampling_target_for_completed_passes(passes as u32)
                )
                .unwrap();
            }
            if want && ((event != 0 && flush >= 1) || (event == 0 && !all && flush == 2 && dc)) {
                let flushed = frame.flush(&mut state);
                drain(&mut coverage);
                after_sections(&mut coverage, &state);
                write!(out, " @L {}", status_code(&flushed)).unwrap();
                dump(&mut out, &keep);
            }
            if event != 0 {
                continue;
            }
            if all {
                done = true;
                break;
            }
            let mut more = false;
            if delivery == 1 {
                if available < n && avail_end < limit {
                    available += 1;
                    avail_end = end_of(available);
                    more = true;
                }
            } else if delivery == 2 {
                idle = if progress != 0 { 0 } else { idle + 1 };
                let left = processed.iter().filter(|&&v| v == 0).count();
                more = left != 0 && idle < left;
            }
            if !more {
                out.push_str(" @N");
                if want && flush >= 1 && dc {
                    let flushed = frame.flush(&mut state);
                    drain(&mut coverage);
                    after_sections(&mut coverage, &state);
                    write!(out, " @L {}", status_code(&flushed)).unwrap();
                    dump(&mut out, &keep);
                }
                break;
            }
        }
        if !done {
            covered_bundle(&mut out, &ib.borrow(), &coverage.bundles[0], None);
            state_snapshot(&mut out, &state, false, &identities, &coverage);
            break;
        }
        if !preview {
            write!(out, " @R {}", frame.references(&state)).unwrap();
        }
        let status = frame.finalize_frame(&mut state);
        drain(&mut coverage);
        write!(out, " @Z {}", status_code(&status)).unwrap();
        if want {
            dump(&mut out, &keep);
        }
        covered_bundle(&mut out, &ib.borrow(), &coverage.bundles[0], None);
        state_snapshot(&mut out, &state, status.is_ok(), &identities, &coverage);
        if status.is_err() {
            break;
        }
        offset += remaining;
        preview = false;
    }
    write!(
        out,
        " @E {frames} {offset} {} {} {}",
        u32::from(last_total),
        state.visible_frame_index,
        state.nonvisible_frame_index
    )
    .unwrap();
    out
}

// Physical frame starts, header ends and section ends of a complete original
// observation: the actual source InitFrame bit counts and TOC sizes.
fn boundaries(observation: &str) -> Vec<usize> {
    let tokens: Vec<_> = observation.split_whitespace().collect();
    let mut offset = tokens[2].parse::<usize>().unwrap() / 8;
    let mut result = vec![offset];
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "@F" {
            offset += tokens[i + 5].parse::<usize>().unwrap() / 8;
            result.push(offset);
            let t = i + tokens[i..].iter().position(|&v| v == "@T").unwrap();
            let n: usize = tokens[t + 1].parse().unwrap();
            for k in 0..n {
                offset += tokens[t + 3 + 2 * k].parse::<usize>().unwrap();
                result.push(offset);
            }
            i = t;
        }
        i += 1;
    }
    result.sort_unstable();
    result.dedup();
    result
}

fn matches_original(requests: &[String]) -> Vec<String> {
    let mut observations = Vec::new();
    for chunk in requests.chunks(8) {
        for (request, cpp) in chunk.iter().zip(original_observations(chunk)) {
            let rust = super::observe(request);
            if rust != cpp {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = cpp.split_whitespace().collect();
                let first = a.iter().zip(&b).position(|(a, b)| a != b);
                let context = first.map(|p| {
                    let start = p.saturating_sub(12);
                    (
                        a[start..(p + 6).min(a.len())].join(" "),
                        b[start..(p + 6).min(b.len())].join(" "),
                    )
                });
                panic!(
                    "request {}: lengths {}/{}; first difference at {first:?}: {context:?}",
                    &request[..request.len().min(160)],
                    a.len(),
                    b.len()
                );
            }
            observations.push(cpp);
        }
    }
    observations
}

fn genuine(lines: &str, filter: impl Fn(&str) -> bool) -> Vec<(String, Vec<u8>)> {
    let fixtures: Vec<_> = lines
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .filter(|(name, _)| filter(name))
        .collect();
    let encoded = original_observations(
        &fixtures
            .iter()
            .map(|(_, request)| request.to_string())
            .collect::<Vec<_>>(),
    );
    fixtures
        .iter()
        .zip(encoded)
        .map(|((name, _), result)| {
            let mut fields = result.split_whitespace();
            assert_eq!(fields.next(), Some("0"), "original encoder failed: {name}");
            (name.to_string(), unhex(fields.next().unwrap()))
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Mode {
    delivery: usize,
    seed: u64,
    coalescing: bool,
    spot: bool,
    detail: u32,
    flush: usize,
    skip: usize,
    channels: u32,
    data_type: u32,
    unpremul: bool,
    undo: bool,
    extra: bool,
    callback: bool,
    slow: bool,
}
impl Mode {
    fn new(channels: u32) -> Self {
        Self {
            delivery: 0,
            seed: 0,
            coalescing: true,
            spot: true,
            detail: 0,
            flush: 0,
            skip: 0,
            channels,
            data_type: 2,
            unpremul: false,
            undo: false,
            extra: false,
            callback: false,
            slow: true,
        }
    }
    fn request(self, bytes: &[u8], limit: usize) -> String {
        format!(
            "ifd-decode {} {limit} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            hex(bytes),
            self.delivery,
            self.seed,
            u8::from(self.coalescing),
            u8::from(self.spot),
            self.detail,
            self.flush,
            self.skip,
            self.channels,
            self.data_type,
            u8::from(self.unpremul),
            u8::from(self.undo),
            u8::from(self.extra),
            u8::from(self.callback),
            u8::from(self.slow)
        )
    }
}

#[derive(Default)]
struct Coverage {
    requests: usize,
    dc_pauses: usize,
    pass_pauses: usize,
    flushes: usize,
    failed_flushes: usize,
    truncated: usize,
    skipped_sections: usize,
    skipped_frames: usize,
    finalized: usize,
    callback_runs: usize,
}
impl Coverage {
    fn add(&mut self, observations: &[String]) {
        for cpp in observations {
            let t: Vec<_> = cpp.split_whitespace().collect();
            self.requests += 1;
            for (i, w) in t.iter().enumerate() {
                match *w {
                    "@V" if t[i + 1] == "1" => self.dc_pauses += 1,
                    "@V" => self.pass_pauses += 1,
                    "@L" if t[i + 1] == "0" => self.flushes += 1,
                    "@L" => self.failed_flushes += 1,
                    "@N" => self.truncated += 1,
                    "@X" => self.skipped_frames += 1,
                    "@Z" if t[i + 1] == "0" => self.finalized += 1,
                    "@C" => self.callback_runs += t[i + 4].parse::<usize>().unwrap(),
                    "@S" => {
                        let n: usize = t[i + 2].parse().unwrap();
                        for k in 0..n {
                            self.skipped_sections += usize::from(t[i + 5 + 3 * k] == "1");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn incremental_multipass_sections_truncation_and_flush_match_original() {
    let fixtures = genuine(include_str!("../data/frame-decoder.requests"), |_| true);
    assert_eq!(fixtures.len(), 10);
    let mut coverage = Coverage::default();
    for (name, bytes) in &fixtures {
        let channels = if name.contains("gray") {
            1
        } else if name.contains("alpha") {
            4
        } else {
            3
        };
        let base = Mode::new(channels);
        let mut modes = vec![
            base,
            Mode {
                delivery: 1,
                detail: 3,
                flush: 1,
                ..base
            },
            Mode {
                delivery: 1,
                detail: 2,
                flush: 1,
                ..base
            },
            Mode {
                delivery: 1,
                detail: 1,
                flush: 2,
                ..base
            },
            Mode {
                delivery: 1,
                detail: 3,
                flush: 2,
                data_type: 0,
                ..base
            },
            Mode {
                data_type: 3,
                ..base
            },
            Mode {
                delivery: 1,
                detail: 3,
                flush: 1,
                channels: 0,
                ..base
            },
            Mode {
                delivery: 2,
                seed: 4,
                detail: 3,
                flush: 1,
                ..base
            },
            Mode {
                coalescing: false,
                ..base
            },
            Mode { skip: 1, ..base },
            Mode {
                callback: true,
                ..base
            },
            Mode {
                delivery: 1,
                detail: 3,
                flush: 2,
                data_type: 0,
                callback: true,
                ..base
            },
        ];
        for seed in 1..=3 {
            modes.push(Mode {
                delivery: 2,
                seed,
                ..base
            });
        }
        for seed in 1..=2 {
            modes.push(Mode {
                delivery: 3,
                seed,
                detail: 1,
                flush: 1,
                ..base
            });
        }
        if channels == 4 {
            modes.push(Mode {
                unpremul: true,
                extra: true,
                ..base
            });
            modes.push(Mode {
                delivery: 1,
                detail: 3,
                flush: 2,
                data_type: 0,
                undo: true,
                extra: true,
                ..base
            });
        }
        // The same modes with the low-memory pipeline (use_slow_rendering_pipeline=false).
        let low: Vec<_> = modes.iter().map(|m| Mode { slow: false, ..*m }).collect();
        modes.extend(low);
        let requests: Vec<_> = modes
            .iter()
            .map(|m| m.request(bytes, bytes.len()))
            .collect();
        let observations = matches_original(&requests);
        coverage.add(&observations);
        // Truncate at every frame, header and section boundary and one byte
        // before it, then flush what decode.cc could flush at that point.
        let mut cuts = Vec::new();
        for end in boundaries(&observations[0]) {
            if end < bytes.len() {
                cuts.push(end);
            }
            if end > 0 && end - 1 < bytes.len() {
                cuts.push(end - 1);
            }
        }
        cuts.sort_unstable();
        cuts.dedup();
        let truncated: Vec<_> = cuts
            .iter()
            .flat_map(|&cut| {
                [
                    Mode { flush: 1, ..base }.request(bytes, cut),
                    Mode {
                        flush: 1,
                        slow: false,
                        ..base
                    }
                    .request(bytes, cut),
                    Mode {
                        delivery: 1,
                        detail: 3,
                        flush: 1,
                        ..base
                    }
                    .request(bytes, cut),
                ]
            })
            .collect();
        let observations = matches_original(&truncated);
        coverage.add(&observations);
        for (cut, o) in truncated.iter().zip(&observations) {
            assert!(
                o.contains(" @N") || !o.starts_with(" H 0 "),
                "{name} {}: {}",
                cut.split_whitespace().nth(2).unwrap(),
                o.split_whitespace()
                    .filter(|w| w.len() < 24)
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
    assert!(coverage.dc_pauses > 0 && coverage.pass_pauses > 0);
    assert!(coverage.flushes > 0 && coverage.truncated > 0 && coverage.skipped_sections > 0);
    assert!(coverage.finalized > 0 && coverage.callback_runs > 0);
    println!(
        "{} incremental C++/Rust cases matched: {} DC and {} pass pauses, {} flushes, {} truncations, {} skipped sections",
        coverage.requests,
        coverage.dc_pauses,
        coverage.pass_pauses,
        coverage.flushes,
        coverage.truncated,
        coverage.skipped_sections
    );
}

#[test]
fn incremental_sequences_coalescing_skips_and_permutations_match_original() {
    let prefixes = [
        "crop-blend-",
        "reference-overwrite-animation-",
        "preview-main-",
        "progressive-dc-",
        "skip-progressive-animation",
        "xyb-saved-before-",
        "save-9-7-",
    ];
    let mut fixtures = genuine(include_str!("../data/frame-sequences.requests"), |name| {
        prefixes.iter().any(|p| name.starts_with(p))
    });
    fixtures.extend(genuine(
        include_str!("../data/frame-sequences-extended.requests"),
        |name| name.starts_with("multigroup-"),
    ));
    fixtures.push((
        "extra-channel-all-types".into(),
        include_bytes!("../data/extra-channel-all-types.jxl").to_vec(),
    ));
    fixtures.push((
        "extra-channel-all-types-horizontal-groups".into(),
        include_bytes!("../data/extra-channel-all-types-horizontal-groups.jxl").to_vec(),
    ));
    assert_eq!(fixtures.len(), 89);
    let mut coverage = Coverage::default();
    for (name, bytes) in &fixtures {
        let base = Mode::new(4);
        let mut modes = vec![
            base,
            Mode {
                coalescing: false,
                ..base
            },
            Mode {
                delivery: 1,
                flush: 2,
                ..base
            },
            Mode {
                delivery: 2,
                seed: 9,
                ..base
            },
            Mode {
                delivery: 3,
                seed: 3,
                coalescing: false,
                ..base
            },
            Mode { skip: 1, ..base },
            Mode {
                skip: 2,
                coalescing: false,
                ..base
            },
            Mode {
                channels: 0,
                delivery: 1,
                ..base
            },
            Mode {
                callback: true,
                coalescing: false,
                ..base
            },
        ];
        if name.starts_with("extra-channel") {
            modes.push(Mode {
                spot: false,
                ..base
            });
            modes.push(Mode {
                spot: false,
                extra: true,
                data_type: 0,
                ..base
            });
            modes.push(Mode {
                extra: true,
                unpremul: true,
                undo: true,
                ..base
            });
        }
        // The same modes with the low-memory pipeline (use_slow_rendering_pipeline=false).
        let low: Vec<_> = modes.iter().map(|m| Mode { slow: false, ..*m }).collect();
        modes.extend(low);
        let requests: Vec<_> = modes
            .iter()
            .map(|m| m.request(bytes, bytes.len()))
            .collect();
        let observations = matches_original(&requests);
        coverage.add(&observations);
        if [
            "reference-overwrite-animation-",
            "preview-main-1-1-1",
            "progressive-dc-",
            "skip-progressive-animation",
            "multigroup-257-3-1-1",
            "crop-blend-1-2-1",
        ]
        .iter()
        .any(|p| name.starts_with(p))
        {
            let cuts: Vec<_> = boundaries(&observations[0])
                .into_iter()
                .filter(|&end| end < bytes.len())
                .collect();
            let truncated: Vec<_> = cuts
                .iter()
                .flat_map(|&cut| {
                    [
                        Mode { flush: 1, ..base }.request(bytes, cut),
                        Mode {
                            flush: 1,
                            slow: false,
                            ..base
                        }
                        .request(bytes, cut),
                        Mode {
                            flush: 1,
                            coalescing: false,
                            ..base
                        }
                        .request(bytes, cut),
                    ]
                })
                .collect();
            let observations = matches_original(&truncated);
            coverage.add(&observations);
            assert!(
                observations
                    .iter()
                    .all(|o| o.contains(" @N") || !o.starts_with(" H 0 ")),
                "{name}"
            );
        }
    }
    assert!(coverage.flushes > 0 && coverage.failed_flushes > 0);
    assert!(coverage.truncated > 0 && coverage.skipped_sections > 0);
    assert!(coverage.skipped_frames > 0 && coverage.finalized > 0 && coverage.callback_runs > 0);
    println!(
        "{} incremental sequence C++/Rust cases matched: {} frames finalized, {} skipped frames, {} flushes ({} rejected), {} truncations",
        coverage.requests,
        coverage.finalized,
        coverage.skipped_frames,
        coverage.flushes,
        coverage.failed_flushes,
        coverage.truncated
    );
}
