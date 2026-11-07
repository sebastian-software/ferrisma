//! The port's side of the WP3 probe `tools/oracle/probes/wp3/entropy.c`:
//! the same options, the same coefficient controller (a mirror of
//! `jdcoefct.c`'s multi-scan `start_input_pass` and `consume_data`), and the
//! same printed fields, so that `tests/wp3_entropy.rs` can compare both
//! outputs exactly. Include it next to `support/oracle.rs` and
//! `support/wp2.rs`.
//!
//! The probe installs its own controller, so `jdinput.c`'s
//! `start_input_pass` and `consume_markers`, which call through
//! `cinfo->coef`, are mirrored here too: they call the port's `per_scan_setup`,
//! `latch_quant_tables`, `read_markers` and `finish_input_pass` in C's order,
//! and the controller below where C calls `cinfo->coef`.

use std::cell::RefCell;
use std::rc::Rc;

use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress, jpeg_read_header};
use jpeg::jdarith::jinit_arith_decoder;
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdhuff::jinit_huff_decoder;
use jpeg::jdinput::{finish_input_pass, latch_quant_tables, per_scan_setup};
use jpeg::jdphuff::jinit_phuff_decoder;
use jpeg::jerror::{ERREXIT, ErrorExit, J_MESSAGE_CODE, jpeg_std_error};
use jpeg::jmorecfg::JDIMENSION;
use jpeg::jpeglib::{
    DCTSIZE2, InputBuffer, JBLOCK, JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_ROW_COMPLETED,
    JPEG_SCAN_COMPLETED, JPEG_SUSPENDED, jpeg_decompress_struct, jpeg_error_mgr, jpeg_source_mgr,
};
use jpeg::jutils::jround_up;

use crate::oracle::Fields;
use crate::wp2::{Log, Recorder, SuspendState, SuspendingSource, print_memory, push, release};

/// The probe's `MAX_CALLS`.
const MAX_CALLS: u64 = 1_000_000;

/// The options of `entropy.c`.
#[derive(Default)]
struct Options {
    trace: i32,
    suspend: usize,
    scaled: i32,
    unneeded: Vec<i32>,
    null_rows: i32,
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
            "-trace" => options.trace = next(&mut index).parse().unwrap(),
            "-suspend" => options.suspend = next(&mut index).parse().unwrap(),
            "-scaled" => options.scaled = next(&mut index).parse().unwrap(),
            "-unneeded" => options.unneeded.push(next(&mut index).parse().unwrap()),
            "-nullrows" => options.null_rows = next(&mut index).parse().unwrap(),
            other => panic!("unknown option {other}"),
        }
        index += 1;
    }
    options
}

/// The probe's `FakeCoef`, with the whole image as rows of blocks per
/// component, sized as `jdcoefct.c`'s virtual arrays.
#[expect(non_snake_case, reason = "C field names")]
struct FakeCoef {
    MCU_ctr: JDIMENSION,
    MCU_vert_offset: i32,
    MCU_rows_per_iMCU_row: i32,
    whole_image: Vec<Vec<Vec<JBLOCK>>>,
    null_rows: i32,
}

/// Which routine `cinfo->inputctl->consume_input` points to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ConsumeInput {
    Markers,
    Data,
}

/// The probe's consume log: every value, with runs counted.
#[derive(Default)]
struct ConsumeLog {
    text: String,
    last_value: Option<i32>,
    last_count: u64,
}

impl ConsumeLog {
    fn flush(&mut self) {
        if self.last_count == 0 {
            return;
        }
        if !self.text.is_empty() {
            self.text.push(',');
        }
        let value = self.last_value.unwrap();
        self.text.push_str(&format!("{value}*{}", self.last_count));
        self.last_count = 0;
    }

    fn log(&mut self, value: i32) {
        if self.last_value != Some(value) {
            self.flush();
        }
        self.last_value = Some(value);
        self.last_count += 1;
    }
}

/// `jdcoefct.h`'s `start_iMCU_row`.
fn start_imcu_row(cinfo: &jpeg_decompress_struct<'_>, coef: &mut FakeCoef) {
    if cinfo.comps_in_scan > 1 {
        coef.MCU_rows_per_iMCU_row = 1;
    } else {
        let index = cinfo.cur_comp_info[0].unwrap();
        let compptr = &cinfo.comp_info()[index];
        if cinfo.input_iMCU_row < cinfo.total_iMCU_rows - 1 {
            coef.MCU_rows_per_iMCU_row = compptr.v_samp_factor;
        } else {
            coef.MCU_rows_per_iMCU_row = compptr.last_row_height;
        }
    }
    coef.MCU_ctr = 0;
    coef.MCU_vert_offset = 0;
}

/// `jdcoefct.c`'s `start_input_pass`.
fn coef_start_input_pass(cinfo: &mut jpeg_decompress_struct<'_>, coef: &mut FakeCoef) {
    cinfo.input_iMCU_row = 0;
    start_imcu_row(cinfo, coef);
}

/// The blocks of one MCU, in C's `MCU_buffer` order.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
fn mcu_blocks<'a>(
    cinfo: &jpeg_decompress_struct<'_>,
    whole_image: &'a mut [Vec<Vec<JBLOCK>>],
    yoffset: i32,
    mcu_col_num: JDIMENSION,
) -> Vec<&'a mut JBLOCK> {
    let comps_in_scan = cinfo.comps_in_scan as usize;
    let mut per_scan_component: Vec<Vec<&'a mut JBLOCK>> =
        (0..comps_in_scan).map(|_| Vec::new()).collect();
    for (cindex, rows) in whole_image.iter_mut().enumerate() {
        let Some(ci) = (0..comps_in_scan).find(|&ci| {
            cinfo.comp_info()[cinfo.cur_comp_info[ci].unwrap()].component_index as usize == cindex
        }) else {
            continue;
        };
        let compptr = &cinfo.comp_info()[cinfo.cur_comp_info[ci].unwrap()];
        let first_row =
            (cinfo.input_iMCU_row as usize) * compptr.v_samp_factor as usize + yoffset as usize;
        let start_col = (mcu_col_num as usize) * compptr.MCU_width as usize;
        for row in rows
            .iter_mut()
            .skip(first_row)
            .take(compptr.MCU_height as usize)
        {
            per_scan_component[ci]
                .extend(row[start_col..start_col + compptr.MCU_width as usize].iter_mut());
        }
    }
    per_scan_component.into_iter().flatten().collect()
}

/// `jdcoefct.c`'s multi-scan `consume_data`, with `MCU_data = NULL` for the
/// first `-nullrows` iMCU rows.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C's comparison of the row number"
)]
fn coef_consume_data(
    cinfo: &mut jpeg_decompress_struct<'_>,
    coef: &mut FakeCoef,
    consume_input: &mut ConsumeInput,
) -> Result<i32, ErrorExit> {
    let mut yoffset = coef.MCU_vert_offset;
    while yoffset < coef.MCU_rows_per_iMCU_row {
        let mut mcu_col_num = coef.MCU_ctr;
        while mcu_col_num < cinfo.MCUs_per_row {
            let null = (cinfo.input_iMCU_row as i32) < coef.null_rows;
            let mut blocks = mcu_blocks(cinfo, &mut coef.whole_image, yoffset, mcu_col_num);
            if !cinfo.entropy().insufficient_data {
                cinfo.master_mut().pub_.last_good_iMCU_row = cinfo.input_iMCU_row;
            }
            let decode_mcu = cinfo.entropy().decode_mcu.expect("decode_mcu is NULL");
            let mcu_data = if null { None } else { Some(&mut blocks[..]) };
            if !decode_mcu(cinfo, mcu_data)? {
                coef.MCU_vert_offset = yoffset;
                coef.MCU_ctr = mcu_col_num;
                return Ok(JPEG_SUSPENDED);
            }
            mcu_col_num += 1;
        }
        coef.MCU_ctr = 0;
        yoffset += 1;
    }
    cinfo.input_iMCU_row += 1;
    if cinfo.input_iMCU_row < cinfo.total_iMCU_rows {
        start_imcu_row(cinfo, coef);
        return Ok(JPEG_ROW_COMPLETED);
    }
    finish_input_pass(cinfo);
    *consume_input = ConsumeInput::Markers;
    Ok(JPEG_SCAN_COMPLETED)
}

/// `jdinput.c`'s `start_input_pass`, with the controller above.
fn start_input_pass(
    cinfo: &mut jpeg_decompress_struct<'_>,
    coef: &mut FakeCoef,
    consume_input: &mut ConsumeInput,
) -> Result<(), ErrorExit> {
    per_scan_setup(cinfo)?;
    if !cinfo.master().pub_.lossless {
        latch_quant_tables(cinfo)?;
    }
    let start_pass = cinfo.entropy().start_pass;
    start_pass(cinfo)?;
    coef_start_input_pass(cinfo, coef);
    *consume_input = ConsumeInput::Data;
    Ok(())
}

/// `jdinput.c`'s `consume_markers` after the header, with the
/// `start_input_pass` above.
fn consume_markers(
    cinfo: &mut jpeg_decompress_struct<'_>,
    coef: &mut FakeCoef,
    consume_input: &mut ConsumeInput,
) -> Result<i32, ErrorExit> {
    if cinfo.inputctl().pub_.eoi_reached {
        return Ok(JPEG_REACHED_EOI);
    }
    let read_markers = cinfo.marker().pub_.read_markers;
    let val = read_markers(cinfo)?;
    match val {
        JPEG_REACHED_SOS => {
            // The header ends at the first SOS, so inheaders is FALSE here.
            assert!(!cinfo.inputctl().inheaders);
            if !cinfo.inputctl().pub_.has_multiple_scans {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_EOI_EXPECTED));
            }
            start_input_pass(cinfo, coef, consume_input)?;
        }
        JPEG_REACHED_EOI => {
            cinfo.inputctl_mut().pub_.eoi_reached = true;
            assert!(!cinfo.inputctl().inheaders);
            if cinfo.output_scan_number > cinfo.input_scan_number {
                cinfo.output_scan_number = cinfo.input_scan_number;
            }
        }
        _ => {}
    }
    Ok(val)
}

/// The probe's `SourceOffset`.
fn source_offset(cinfo: &jpeg_decompress_struct<'_>, data: &[u8], state: &SuspendState) -> i64 {
    if state.fake_eoi {
        return -1;
    }
    let src = cinfo.src();
    match src.buffer {
        InputBuffer::Borrowed(bytes) if bytes.as_ptr() == data.as_ptr() => {
            i64::try_from(src.next_input_byte).unwrap()
        }
        _ => -1,
    }
}

/// The probe's `PrintState`.
fn print_state(
    log: &Log,
    key: &str,
    cinfo: &jpeg_decompress_struct<'_>,
    data: &[u8],
    state: &SuspendState,
) {
    let insufficient_data = cinfo
        .entropy
        .as_deref()
        .map_or(-1, |entropy| i32::from(entropy.pub_().insufficient_data));
    let marker = &cinfo.marker().pub_;
    push(
        log,
        key,
        format!(
            "{},{},{},{},{},{},{insufficient_data},{},{},{},{},{},{},{},{}",
            cinfo.input_scan_number,
            cinfo.comps_in_scan,
            cinfo.Ss,
            cinfo.Se,
            cinfo.Ah,
            cinfo.Al,
            cinfo.unread_marker,
            marker.next_restart_num,
            marker.discarded_bytes,
            source_offset(cinfo, data, state),
            cinfo.src().bytes_in_buffer,
            cinfo.input_iMCU_row,
            cinfo.master().pub_.last_good_iMCU_row,
            cinfo.err.num_warnings
        ),
    );
    if let Some(coef_bits) = cinfo.coef_bits.as_deref() {
        let values: Vec<String> = coef_bits
            .iter()
            .flat_map(|row| row.iter().map(i32::to_string))
            .collect();
        push(log, "coef_bits", values.join(","));
    }
}

/// The probe's `PrintRows`: an FNV-1a hash of every block row.
#[expect(clippy::cast_sign_loss, reason = "C's (unsigned short) cast")]
fn print_rows(log: &Log, coef: &FakeCoef) {
    for (ci, rows) in coef.whole_image.iter().enumerate() {
        let hashes: Vec<String> = rows
            .iter()
            .map(|row| {
                let mut hash: u64 = 1_469_598_103_934_665_603;
                for block in row {
                    for &value in block {
                        let value = value as u16;
                        hash = (hash ^ u64::from(value & 0xFF)).wrapping_mul(1_099_511_628_211);
                        hash = (hash ^ u64::from(value >> 8)).wrapping_mul(1_099_511_628_211);
                    }
                }
                format!("{hash:016x}")
            })
            .collect();
        push(log, &format!("rows{ci}"), hashes.join(","));
    }
}

/// The probe's `AllocateImage`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions of the rounded sizes"
)]
fn allocate_image(cinfo: &jpeg_decompress_struct<'_>) -> Vec<Vec<Vec<JBLOCK>>> {
    cinfo
        .comp_info()
        .iter()
        .take(cinfo.num_components as usize)
        .map(|compptr| {
            let rows = jround_up(
                compptr.height_in_blocks.into(),
                compptr.v_samp_factor.into(),
            ) as usize;
            let cols =
                jround_up(compptr.width_in_blocks.into(), compptr.h_samp_factor.into()) as usize;
            vec![vec![[0; DCTSIZE2]; cols]; rows]
        })
        .collect()
}

/// Runs `entropy.c`'s sequence on the port for `args` and the input `data`.
#[expect(clippy::too_many_lines, reason = "the probe's main")]
pub fn run_rust(args: &[&str], data: &[u8]) -> Fields {
    let options = parse(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));

    let error = Rc::new(RefCell::new((0, String::new())));
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    err.methods = Some(Box::new(Recorder {
        log: Rc::clone(&log),
        error: Rc::clone(&error),
    }));
    let mut cinfo = jpeg_decompress_struct::new(err);
    let state = Rc::new(RefCell::new(SuspendState {
        size: data.len(),
        ..SuspendState::default()
    }));
    let mut coef = FakeCoef {
        MCU_ctr: 0,
        MCU_vert_offset: 0,
        MCU_rows_per_iMCU_row: 0,
        whole_image: Vec::new(),
        null_rows: options.null_rows,
    };
    let mut consume = ConsumeLog::default();
    let mut completed = false;

    let mut body = || -> Result<(), ErrorExit> {
        jpeg_create_decompress(&mut cinfo)?;
        cinfo.err.trace_level = options.trace;
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

        let mut status;
        loop {
            if options.suspend > 0 {
                release(&mut cinfo, &state, options.suspend);
            }
            status = jpeg_read_header(&mut cinfo, true)?;
            if status != JPEG_SUSPENDED || options.suspend == 0 {
                break;
            }
        }
        push(&log, "header", status);

        if options.scaled > 0 {
            for compptr in cinfo.comp_info_mut() {
                compptr.DCT_scaled_size = options.scaled;
            }
        }
        for &ci in &options.unneeded {
            if ci >= 0 && ci < cinfo.num_components {
                cinfo.comp_info_mut()[usize::try_from(ci).unwrap()].component_needed = false;
            }
        }

        /* jdmaster.c's master_selection */
        if cinfo.arith_code {
            push(&log, "decoder", "arith");
            jinit_arith_decoder(&mut cinfo)?;
        } else if cinfo.progressive_mode {
            push(&log, "decoder", "phuff");
            jinit_phuff_decoder(&mut cinfo)?;
        } else {
            push(&log, "decoder", "huff");
            jinit_huff_decoder(&mut cinfo)?;
        }

        coef.whole_image = allocate_image(&cinfo);
        let mut consume_input = ConsumeInput::Markers;
        start_input_pass(&mut cinfo, &mut coef, &mut consume_input)?;
        let mut calls: u64 = 0;
        loop {
            calls += 1;
            if calls > MAX_CALLS {
                push(&log, "call_limit", 1);
                break;
            }
            let value = match consume_input {
                ConsumeInput::Data => coef_consume_data(&mut cinfo, &mut coef, &mut consume_input)?,
                ConsumeInput::Markers => {
                    consume_markers(&mut cinfo, &mut coef, &mut consume_input)?
                }
            };
            consume.log(value);
            if value == JPEG_SCAN_COMPLETED {
                print_state(&log, "scan", &cinfo, data, &state.borrow());
            }
            if value == JPEG_REACHED_EOI {
                break;
            }
            if value == JPEG_SUSPENDED {
                if options.suspend == 0 || state.borrow().fake_eoi {
                    push(&log, "stuck", 1);
                    break;
                }
                release(&mut cinfo, &state, options.suspend);
            }
        }
        completed = true;
        Ok(())
    };
    let _ = body();
    drop(body);

    consume.flush();
    push(&log, "consume", &consume.text);
    if completed || (cinfo.master.is_some() && cinfo.marker.is_some() && cinfo.src.is_some()) {
        print_state(&log, "final", &cinfo, data, &state.borrow());
    }
    print_memory(&log, cinfo.mem());
    print_rows(&log, &coef);
    let (code, message) = error.borrow().clone();
    push(&log, "error", code);
    if code != 0 {
        push(&log, "error_message", message);
    }
    push(&log, "num_warnings", cinfo.err.num_warnings);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}
