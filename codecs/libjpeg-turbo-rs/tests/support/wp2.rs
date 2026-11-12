//! The port's side of the WP2 probe `tools/oracle/probes/wp2/header.c`:
//! the same options, the same suspending source and marker processor, and
//! the same printed fields, so that `tests/wp2_header.rs` can compare both
//! outputs exactly. Include it next to `support/oracle.rs`.
//!
//! `support/wp3.rs` reuses the error recorder, the suspending source and the
//! pool output.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use jpeg::jdapimin::{
    jpeg_CreateDecompress, jpeg_abort_decompress, jpeg_consume_input, jpeg_create_decompress,
    jpeg_destroy_decompress, jpeg_has_multiple_scans, jpeg_input_complete, jpeg_read_header,
};
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdicc::jpeg_read_icc_profile;
use jpeg::jdmarker::{jpeg_resync_to_restart, jpeg_save_markers, jpeg_set_marker_processor};
use jpeg::jerror::{
    ErrorExit, J_MESSAGE_CODE, WARNMS, emit_message, jpeg_error_methods, jpeg_std_error,
};
use jpeg::jmemmgr::my_memory_mgr;
use jpeg::jmorecfg::MAX_COMPONENTS;
use jpeg::jpeglib::{
    InputBuffer, JMSG_LENGTH_MAX, JPEG_EOI, JPEG_SUSPENDED, MAX_COMPS_IN_SCAN, NUM_ARITH_TBLS,
    NUM_HUFF_TBLS, NUM_QUANT_TBLS, SIZEOF_jpeg_decompress_struct, call_fill_input_buffer,
    call_skip_input_data, jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_processor,
    jpeg_source_methods, jpeg_source_mgr,
};

use crate::oracle::Fields;

/// The options of `header.c`, in the order the probe applies them.
#[derive(Default)]
struct Options {
    create: Option<(i32, usize)>,
    trace: i32,
    saves: Vec<(i32, u32)>,
    processors: Vec<i32>,
    suspend: usize,
    no_image: bool,
    again: bool,
    consume: usize,
    query: bool,
    icc: bool,
    resync: Option<i32>,
    rst: usize,
    abort: bool,
}

fn parse(args: &[&str]) -> Options {
    let mut options = Options::default();
    let mut index = 0;
    let next = |index: &mut usize| {
        *index += 1;
        args[*index].to_owned()
    };
    while index < args.len() {
        match args[index] {
            "-create" => {
                let version = next(&mut index).parse().unwrap();
                let size = next(&mut index).parse().unwrap();
                options.create = Some((version, size));
            }
            "-trace" => options.trace = next(&mut index).parse().unwrap(),
            "-save" => {
                let code = next(&mut index).parse().unwrap();
                let limit = next(&mut index).parse().unwrap();
                options.saves.push((code, limit));
            }
            "-processor" => options.processors.push(next(&mut index).parse().unwrap()),
            "-suspend" => options.suspend = next(&mut index).parse().unwrap(),
            "-noimage" => options.no_image = true,
            "-again" => options.again = true,
            "-consume" => options.consume = next(&mut index).parse().unwrap(),
            "-query" => options.query = true,
            "-icc" => options.icc = true,
            "-resync" => options.resync = Some(next(&mut index).parse().unwrap()),
            "-rst" => options.rst = next(&mut index).parse().unwrap(),
            "-abort" => options.abort = true,
            other => panic!("unknown option {other}"),
        }
        index += 1;
    }
    options
}

pub type Log = Rc<RefCell<Fields>>;

pub fn push(log: &Log, key: &str, value: impl ToString) {
    log.borrow_mut().0.push((key.to_owned(), value.to_string()));
}

pub fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        text.push(char::from(DIGITS[usize::from(byte >> 4)]));
        text.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
    text
}

fn until_nul(buffer: &[u8]) -> &[u8] {
    &buffer[..buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(buffer.len())]
}

/// The probe's `Wp2ErrorMgr`.
pub struct Recorder {
    pub log: Log,
    pub error: Rc<RefCell<(i32, String)>>,
}

impl jpeg_error_methods for Recorder {
    fn error_exit(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        let mut buffer = [0; JMSG_LENGTH_MAX];
        self.format_message(cinfo, &mut buffer);
        *self.error.borrow_mut() = (
            cinfo.err.msg_code,
            String::from_utf8_lossy(until_nul(&buffer)).into_owned(),
        );
    }

    fn emit_message(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        msg_level: i32,
    ) -> Result<(), ErrorExit> {
        let mut buffer = [0; JMSG_LENGTH_MAX];
        self.format_message(cinfo, &mut buffer);
        let value = format!(
            "{msg_level}:{}:={}",
            cinfo.err.msg_code,
            hex(until_nul(&buffer))
        );
        push(&self.log, "msg", value);
        emit_message(self, cinfo, msg_level)
    }

    fn output_message(&mut self, _cinfo: &mut jpeg_decompress_struct<'_>) {}
}

/// The probe's suspending source state.
#[derive(Default)]
pub struct SuspendState {
    pub size: usize,
    pub released: usize,
    pub skip_pending: i64,
    pub fake_eoi: bool,
}

static FAKE_EOI: [u8; 4] = [0xFF, JPEG_EOI as u8, 0, 0];

pub struct SuspendingSource(pub Rc<RefCell<SuspendState>>);

impl jpeg_source_methods for SuspendingSource {
    fn init_source(&mut self, _cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        Ok(())
    }

    fn fill_input_buffer(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<bool, ErrorExit> {
        {
            let state = self.0.borrow();
            if state.released < state.size {
                return Ok(false);
            }
        }
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_JPEG_EOF)?;
        let src = cinfo.src_mut();
        src.buffer = InputBuffer::Borrowed(&FAKE_EOI);
        src.next_input_byte = 0;
        src.bytes_in_buffer = 2;
        self.0.borrow_mut().fake_eoi = true;
        Ok(true)
    }

    fn skip_input_data(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        num_bytes: core::ffi::c_long,
    ) -> Result<(), ErrorExit> {
        if num_bytes <= 0 {
            return Ok(());
        }
        let num_bytes = usize::try_from(num_bytes).unwrap();
        let src = cinfo.src_mut();
        if num_bytes > src.bytes_in_buffer {
            self.0.borrow_mut().skip_pending +=
                i64::try_from(num_bytes - src.bytes_in_buffer).unwrap();
            src.next_input_byte += src.bytes_in_buffer;
            src.bytes_in_buffer = 0;
        } else {
            src.next_input_byte += num_bytes;
            src.bytes_in_buffer -= num_bytes;
        }
        Ok(())
    }

    fn resync_to_restart(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        desired: i32,
    ) -> Result<bool, ErrorExit> {
        jpeg_resync_to_restart(cinfo, desired)
    }

    fn term_source(&mut self, _cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        Ok(())
    }
}

/// The probe's `Wp2Release`.
pub fn release(
    cinfo: &mut jpeg_decompress_struct<'_>,
    state: &Rc<RefCell<SuspendState>>,
    step: usize,
) {
    let mut state = state.borrow_mut();
    if state.fake_eoi {
        return;
    }
    let src = cinfo.src_mut();
    let mut pos = src.next_input_byte + usize::try_from(state.skip_pending).unwrap();
    state.released = if state.size - state.released > step {
        state.released + step
    } else {
        state.size
    };
    if pos > state.released {
        state.skip_pending = i64::try_from(pos - state.released).unwrap();
        pos = state.released;
    } else {
        state.skip_pending = 0;
    }
    src.next_input_byte = pos;
    src.bytes_in_buffer = state.released - pos;
}

/// The probe's `Wp2CountingProcessor`.
struct Counter {
    calls: Rc<Cell<i32>>,
    log: Log,
}

impl jpeg_marker_processor for Counter {
    fn process_marker(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<bool, ErrorExit> {
        self.calls.set(self.calls.get() + 1);
        let mut length: u32 = 0;
        for _ in 0..2 {
            if cinfo.src().bytes_in_buffer == 0 && !call_fill_input_buffer(cinfo)? {
                return Ok(false);
            }
            let src = cinfo.src_mut();
            let byte = src.buffer.bytes()[src.next_input_byte];
            src.next_input_byte += 1;
            src.bytes_in_buffer -= 1;
            length = (length << 8) + u32::from(byte);
        }
        push(
            &self.log,
            "processor",
            format!("{}:{length}", cinfo.unread_marker),
        );
        if length > 2 {
            call_skip_input_data(cinfo, core::ffi::c_long::from(length) - 2)?;
        }
        Ok(true)
    }
}

/// The probe's `SourceOffset`.
fn source_offset(
    cinfo: &jpeg_decompress_struct<'_>,
    data: &[u8],
    suspending: Option<&SuspendState>,
) -> i64 {
    let Some(src) = cinfo.src.as_deref() else {
        return -1;
    };
    let offset = i64::try_from(src.next_input_byte).unwrap();
    if suspending.is_some_and(|state| state.fake_eoi) {
        return 1_000_000 + offset;
    }
    match src.buffer {
        InputBuffer::Borrowed(bytes) if bytes.as_ptr() == data.as_ptr() => offset,
        _ => 2_000_000,
    }
}

fn print_pools(log: &Log, key: &str, records: impl Iterator<Item = (usize, usize)>) {
    let value: Vec<String> = records
        .map(|(used, left)| format!("{used}/{left}"))
        .collect();
    push(log, key, value.join(","));
}

/// The probe's `Wp2PrintMemory`.
pub fn print_memory(log: &Log, mem: &my_memory_mgr) {
    push(log, "total", mem.total_space_allocated);
    print_pools(
        log,
        "small0",
        mem.small_list[0]
            .iter()
            .map(|h| (h.bytes_used, h.bytes_left)),
    );
    print_pools(
        log,
        "small1",
        mem.small_list[1]
            .iter()
            .map(|h| (h.bytes_used, h.bytes_left)),
    );
    print_pools(
        log,
        "large0",
        mem.large_list[0]
            .iter()
            .map(|h| (h.bytes_used, h.bytes_left)),
    );
    print_pools(
        log,
        "large1",
        mem.large_list[1]
            .iter()
            .map(|h| (h.bytes_used, h.bytes_left)),
    );
}

/// The probe's `Wp2PrintRound`.
fn print_round(
    log: &Log,
    cinfo: &jpeg_decompress_struct<'_>,
    round: i32,
    status: i32,
    data: &[u8],
    state: &SuspendState,
) {
    let marker = cinfo.marker();
    push(
        log,
        &format!("round{round}"),
        format!(
            "{status},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            cinfo.global_state,
            cinfo.unread_marker,
            source_offset(cinfo, data, Some(state)),
            cinfo.src().bytes_in_buffer,
            cinfo.image_width,
            cinfo.image_height,
            cinfo.num_components,
            cinfo.data_precision,
            u8::from(marker.pub_.saw_SOI),
            u8::from(marker.pub_.saw_SOF),
            cinfo.input_scan_number,
            marker.pub_.discarded_bytes,
            cinfo.err.num_warnings,
            cinfo.restart_interval
        ),
    );
}

/// The probe's `Wp2PrintObject`. After `-abort` the probe leaves out the
/// component layout, which C has released.
#[expect(clippy::too_many_lines, reason = "one line per probe field")]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn print_object(
    log: &Log,
    cinfo: &jpeg_decompress_struct<'_>,
    data: &[u8],
    suspending: Option<&SuspendState>,
    aborted: bool,
) {
    push(log, "global_state", cinfo.global_state);
    let Some(mem) = cinfo.mem.as_deref() else {
        push(log, "mem", 0);
        return;
    };
    print_memory(log, mem);
    push(
        log,
        "source",
        format!(
            "{},{}",
            source_offset(cinfo, data, suspending),
            cinfo.src.as_deref().map_or(0, |src| src.bytes_in_buffer)
        ),
    );
    push(
        log,
        "image",
        format!(
            "{},{},{},{},{}",
            cinfo.image_width,
            cinfo.image_height,
            cinfo.num_components,
            cinfo.data_precision,
            cinfo.jpeg_color_space.0
        ),
    );
    #[expect(clippy::cast_possible_truncation, reason = "the probe's (int) cast")]
    let gamma = (cinfo.output_gamma * 1000.0) as i32;
    push(
        log,
        "params",
        format!(
            "{},{},{},{gamma},{},{},{},{},{},{},{},{},{},{},{},{}",
            cinfo.out_color_space.0,
            cinfo.scale_num,
            cinfo.scale_denom,
            u8::from(cinfo.buffered_image),
            u8::from(cinfo.raw_data_out),
            cinfo.dct_method.0,
            u8::from(cinfo.do_fancy_upsampling),
            u8::from(cinfo.do_block_smoothing),
            u8::from(cinfo.quantize_colors),
            cinfo.dither_mode.0,
            u8::from(cinfo.two_pass_quantize),
            cinfo.desired_number_of_colors,
            u8::from(cinfo.enable_1pass_quant),
            u8::from(cinfo.enable_external_quant),
            u8::from(cinfo.enable_2pass_quant)
        ),
    );
    push(
        log,
        "progress",
        format!(
            "{},{},{},{},{},{}",
            u8::from(cinfo.progressive_mode),
            u8::from(cinfo.arith_code),
            u8::from(cinfo.CCIR601_sampling),
            cinfo.input_scan_number,
            cinfo.restart_interval,
            cinfo.unread_marker
        ),
    );
    push(
        log,
        "jfif",
        format!(
            "{},{},{},{},{},{}",
            u8::from(cinfo.saw_JFIF_marker),
            cinfo.JFIF_major_version,
            cinfo.JFIF_minor_version,
            cinfo.density_unit,
            cinfo.X_density,
            cinfo.Y_density
        ),
    );
    push(
        log,
        "adobe",
        format!(
            "{},{}",
            u8::from(cinfo.saw_Adobe_marker),
            cinfo.Adobe_transform
        ),
    );
    let arith: Vec<String> = (0..NUM_ARITH_TBLS)
        .map(|i| {
            format!(
                "{}/{}/{}",
                cinfo.arith_dc_L[i], cinfo.arith_dc_U[i], cinfo.arith_ac_K[i]
            )
        })
        .collect();
    push(log, "arith", arith.join(","));
    push(
        log,
        "sampling",
        format!(
            "{},{},{},{}",
            cinfo.max_h_samp_factor,
            cinfo.max_v_samp_factor,
            cinfo.min_DCT_scaled_size,
            cinfo.total_iMCU_rows
        ),
    );
    push(
        log,
        "scan",
        format!(
            "{},{},{},{},{}",
            cinfo.comps_in_scan, cinfo.Ss, cinfo.Se, cinfo.Ah, cinfo.Al
        ),
    );
    if aborted {
        push(log, "comp_info_present", "-");
    } else {
        let current: Vec<String> = (0..MAX_COMPS_IN_SCAN)
            .map(|i| cinfo.cur_comp_info[i].map_or("-1".to_owned(), |index| index.to_string()))
            .collect();
        push(log, "cur_comp_info", current.join(","));
        push(
            log,
            "comp_info_present",
            u8::from(cinfo.comp_info.is_some()),
        );
        if let Some(comp_info) = cinfo.comp_info.as_deref() {
            let count = usize::try_from(cinfo.num_components)
                .unwrap_or(0)
                .min(MAX_COMPONENTS);
            for (ci, c) in comp_info.iter().take(count).enumerate() {
                push(
                    log,
                    &format!("component{ci}"),
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        c.component_id,
                        c.component_index,
                        c.h_samp_factor,
                        c.v_samp_factor,
                        c.quant_tbl_no,
                        c.dc_tbl_no,
                        c.ac_tbl_no,
                        c.width_in_blocks,
                        c.height_in_blocks,
                        c.DCT_scaled_size,
                        c.downsampled_width,
                        c.downsampled_height,
                        u8::from(c.component_needed),
                        u8::from(c.quant_table.is_some())
                    ),
                );
            }
        }
    }
    for i in 0..NUM_QUANT_TBLS {
        let value = cinfo.quant_tbl_ptrs[i]
            .as_deref()
            .map_or("-".to_owned(), |table| {
                let values: Vec<String> = table.quantval.iter().map(u16::to_string).collect();
                format!("{}:{}", u8::from(table.sent_table), values.join(","))
            });
        push(log, &format!("quant{i}"), value);
    }
    for i in 0..2 * NUM_HUFF_TBLS {
        let table = if i < NUM_HUFF_TBLS {
            cinfo.dc_huff_tbl_ptrs[i].as_deref()
        } else {
            cinfo.ac_huff_tbl_ptrs[i - NUM_HUFF_TBLS].as_deref()
        };
        match table {
            None => push(log, &format!("huff{i}"), "-"),
            Some(table) => {
                push(
                    log,
                    &format!("huff{i}"),
                    format!("{}:={}", u8::from(table.sent_table), hex(&table.bits)),
                );
                push(log, "huffval", hex(&table.huffval));
            }
        }
    }
    let mut marker = cinfo.marker_list.as_deref();
    let mut index = 0;
    while let Some(saved) = marker {
        push(
            log,
            &format!("saved{index}"),
            format!(
                "{},{},{}:={}",
                saved.marker,
                saved.original_length,
                saved.data_length,
                hex(&saved.data)
            ),
        );
        marker = saved.next.as_deref();
        index += 1;
    }
    let master = &cinfo.master().pub_;
    let columns: Vec<String> = (0..MAX_COMPONENTS)
        .map(|ci| format!("{}/{}", master.first_MCU_col[ci], master.last_MCU_col[ci]))
        .collect();
    push(
        log,
        "master",
        format!(
            "{},{},{},{}",
            u8::from(master.lossless),
            master.jpeg_data_precision,
            u8::from(master.marker_list_end.is_some()),
            columns.join(",")
        ),
    );
    let inputctl = &cinfo.inputctl().pub_;
    push(
        log,
        "inputctl",
        format!(
            "{},{}",
            u8::from(inputctl.has_multiple_scans),
            u8::from(inputctl.eoi_reached)
        ),
    );
    let reader = &cinfo.marker().pub_;
    push(
        log,
        "marker",
        format!(
            "{},{},{},{}",
            u8::from(reader.saw_SOI),
            u8::from(reader.saw_SOF),
            reader.next_restart_num,
            reader.discarded_bytes
        ),
    );
    push(log, "coef_bits", u8::from(cinfo.coef_bits.is_some()));
}

/// The one field where the port differs by design: with
/// `QUANT_2PASS_SUPPORTED` treated as undefined (ADR 0005), upstream's
/// `default_decompress_parms` sets `two_pass_quantize` to `FALSE`, where the
/// oracle, which compiles color quantization, sets `TRUE`. The oracle's value
/// is mapped to the port's; nothing else is changed.
pub fn without_two_pass_quantize(mut fields: Fields) -> Fields {
    for (key, value) in &mut fields.0 {
        if key == "params" {
            let mut items: Vec<&str> = value.split(',').collect();
            assert!(items[11] == "0" || items[11] == "1", "{value}");
            items[11] = "0";
            *value = items.join(",");
        }
    }
    fields
}

/// Runs `header.c`'s sequence on the port for `args` and the input `data`.
#[expect(clippy::too_many_lines, reason = "the probe's main")]
pub fn run_rust(args: &[&str], data: &[u8]) -> Fields {
    let options = parse(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));
    push(
        &log,
        "sizeof_jpeg_decompress_struct",
        SIZEOF_jpeg_decompress_struct,
    );

    let error = Rc::new(RefCell::new((0, String::new())));
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    err.methods = Some(Box::new(Recorder {
        log: Rc::clone(&log),
        error: Rc::clone(&error),
    }));
    let mut cinfo = jpeg_decompress_struct::new(err);
    let calls = Rc::new(Cell::new(0));
    let state = Rc::new(RefCell::new(SuspendState {
        size: data.len(),
        ..SuspendState::default()
    }));
    let mut aborted = false;

    let mut body = || -> Result<(), ErrorExit> {
        match options.create {
            Some((version, size)) => jpeg_CreateDecompress(&mut cinfo, version, size)?,
            None => jpeg_create_decompress(&mut cinfo)?,
        }
        cinfo.err.trace_level = options.trace;
        for &(code, limit) in &options.saves {
            jpeg_save_markers(&mut cinfo, code, limit)?;
        }
        for &code in &options.processors {
            let counter = Counter {
                calls: Rc::clone(&calls),
                log: Rc::clone(&log),
            };
            jpeg_set_marker_processor(&mut cinfo, code, Box::new(counter))?;
        }
        if options.suspend > 0 {
            cinfo.src = Some(Box::new(jpeg_source_mgr {
                next_input_byte: 0,
                bytes_in_buffer: 0,
                buffer: InputBuffer::Borrowed(data),
                methods: Some(Box::new(SuspendingSource(Rc::clone(&state)))),
            }));
        } else {
            jpeg_mem_src(&mut cinfo, data)?;
        }

        let require_image = !options.no_image;
        let mut status;
        let mut round = 0;
        loop {
            if options.suspend > 0 {
                release(&mut cinfo, &state, options.suspend);
            }
            status = jpeg_read_header(&mut cinfo, require_image)?;
            if status != JPEG_SUSPENDED || options.suspend == 0 {
                break;
            }
            print_round(&log, &cinfo, round, status, data, &state.borrow());
            round += 1;
        }
        push(&log, "status", status);
        if options.again {
            status = jpeg_read_header(&mut cinfo, require_image)?;
            push(&log, "again", status);
        }
        for _ in 0..options.consume {
            let value = jpeg_consume_input(&mut cinfo)?;
            push(&log, "consume", value);
        }
        if options.query {
            let complete = jpeg_input_complete(&mut cinfo)?;
            push(&log, "input_complete", u8::from(complete));
            let multiple = jpeg_has_multiple_scans(&mut cinfo)?;
            push(&log, "multiple_scans", u8::from(multiple));
        }
        if options.icc {
            let mut icc_data = None;
            let mut icc_len = 0;
            let found = jpeg_read_icc_profile(&mut cinfo, Some(&mut icc_data), Some(&mut icc_len))?;
            push(&log, "icc", format!("{},{icc_len}", u8::from(found)));
            if let Some(icc_data) = icc_data {
                push(&log, "icc_data", hex(&icc_data));
            }
        }
        if let Some(desired) = options.resync {
            let result = jpeg_resync_to_restart(&mut cinfo, desired)?;
            push(&log, "resync", u8::from(result));
        }
        for _ in 0..options.rst {
            let read_restart_marker = cinfo.marker().pub_.read_restart_marker;
            let result = read_restart_marker(&mut cinfo)?;
            push(
                &log,
                "rst",
                format!(
                    "{},{},{}",
                    u8::from(result),
                    cinfo.unread_marker,
                    cinfo.marker().pub_.next_restart_num
                ),
            );
        }
        if options.abort {
            jpeg_abort_decompress(&mut cinfo)?;
            aborted = true;
            push(&log, "aborted", 1);
        }
        Ok(())
    };
    let _ = body();
    drop(body);

    push(&log, "processor_calls", calls.get());
    let suspending = (options.suspend > 0).then(|| state.borrow());
    print_object(&log, &cinfo, data, suspending.as_deref(), aborted);
    drop(suspending);
    let (code, message) = error.borrow().clone();
    push(&log, "error", code);
    if code != 0 {
        push(&log, "error_message", message);
    }
    push(&log, "num_warnings", cinfo.err.num_warnings);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}
