//! The port's side of the WP4 probes `tools/oracle/probes/wp4/idct.c` and
//! `tools/oracle/probes/wp4/coefidct.c`: the same options, the same calls in
//! the same order, and the same printed fields, so that `tests/wp4_idct.rs`
//! and `tests/wp4_coefidct.rs` can compare both outputs exactly. Include it
//! next to `support/oracle.rs` and `support/wp2.rs`.
//!
//! `jdmaster.c`, which builds `sample_range_limit` and initializes the
//! modules, arrives in work package 6, so both probes and this file carry a
//! copy of `prepare_range_limit_table` and of the part of `master_selection`
//! and `prepare_for_output_pass` that concerns these modules.

#![allow(dead_code, reason = "each test binary uses part of the support")]

use std::cell::RefCell;
use std::rc::Rc;

use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress, jpeg_read_header};
use jpeg::jdarith::jinit_arith_decoder;
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdcoefct::jinit_d_coef_controller;
use jpeg::jddctmgr::jinit_inverse_dct;
use jpeg::jdhuff::jinit_huff_decoder;
use jpeg::jdphuff::jinit_phuff_decoder;
use jpeg::jerror::{ErrorExit, jpeg_std_error};
use jpeg::jmemmgr::{alloc_small_box, alloc_small_vec};
use jpeg::jmorecfg::{
    CENTERJSAMPLE, JCOEF, JDIMENSION, JSAMPLE, MAX_COMPONENTS, MAXJSAMPLE, UINT16,
};
use jpeg::jpeglib::{
    DCTSIZE, DCTSIZE2, InputBuffer, J_DCT_METHOD, JBLOCK, JPEG_HEADER_OK, JPEG_SUSPENDED,
    JPOOL_IMAGE, JQUANT_TBL, JSAMPROWS_MUT, SIZEOF_JQUANT_TBL, SIZEOF_jpeg_component_info,
    SampleArray, jpeg_component_info, jpeg_decompress_struct, jpeg_error_mgr, jpeg_source_mgr,
};

use crate::oracle::Fields;
use crate::wp2::{Log, Recorder, SuspendState, SuspendingSource, print_memory, push, release};

/// The FNV-1a offset basis and prime of the probes' hashes.
pub const FNV_OFFSET: u64 = 1_469_598_103_934_665_603;
pub const FNV_PRIME: u64 = 1_099_511_628_211;

/// `jdmaster.c`'s `prepare_range_limit_table`, 8-bit branch, as the probes
/// copy it.
///
/// # Errors
///
/// What the allocation reports.
pub fn prepare_range_limit_table(cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
    let max = MAXJSAMPLE as usize;
    let center = CENTERJSAMPLE as usize;
    let size = 5 * (max + 1) + center;
    let mut table: Vec<JSAMPLE> = alloc_small_vec(cinfo, JPOOL_IMAGE, size, size, 0)?;
    let base = max + 1; /* C's table after `table += MAXJSAMPLE + 1` */
    /* table[base - (MAXJSAMPLE + 1)..base] is zero already */
    for i in 0..=max {
        table[base + i] = u8::try_from(i).unwrap();
    }
    let post = base + center; /* C's table after `table += CENTERJSAMPLE` */
    for i in center..2 * (max + 1) {
        table[post + i] = MAXJSAMPLE as JSAMPLE;
    }
    table[post + 2 * (max + 1)..post + 2 * (max + 1) + (2 * (max + 1) - center)].fill(0);
    table.copy_within(base..base + center, post + 4 * (max + 1) - center);
    cinfo.sample_range_limit = Some(table);
    Ok(())
}

/// The probes' error manager and object: a decompression object whose
/// recorder logs into `log` and keeps the error in `error`.
pub fn new_object<'src>(
    log: &Log,
    error: &Rc<RefCell<(i32, String)>>,
) -> jpeg_decompress_struct<'src> {
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    err.methods = Some(Box::new(Recorder {
        log: Rc::clone(log),
        error: Rc::clone(error),
    }));
    jpeg_decompress_struct::new(err)
}

/// The probes' `Wp2PrintError`.
pub fn print_error(
    log: &Log,
    cinfo: &jpeg_decompress_struct<'_>,
    error: &Rc<RefCell<(i32, String)>>,
) {
    let (code, message) = error.borrow().clone();
    push(log, "error", code);
    if code != 0 {
        push(log, "error_message", message);
    }
    push(log, "num_warnings", cinfo.err.num_warnings);
}

/// `idct.c`'s xorshift64 generator.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        u32::try_from(self.0 >> 32).unwrap()
    }

    #[expect(
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        reason = "C's int arithmetic"
    )]
    fn rand(&mut self, n: i32) -> i32 {
        (self.next() % n as u32) as i32
    }
}

/// The options of `idct.c`.
struct IdctOptions {
    method: J_DCT_METHOD,
    size: i32,
    blocks: i32,
    seed: u64,
    qmax: i32,
    extreme: bool,
}

#[expect(clippy::cast_sign_loss, reason = "C's (J_DCT_METHOD)atoi")]
fn parse_idct(args: &[&str]) -> IdctOptions {
    let mut options = IdctOptions {
        method: J_DCT_METHOD::JDCT_ISLOW,
        size: 8,
        blocks: 100,
        seed: 1,
        qmax: 255,
        extreme: false,
    };
    let mut args = args.iter();
    while let Some(&arg) = args.next() {
        let mut next = || *args.next().expect("option value");
        match arg {
            "-method" => {
                options.method = match next() {
                    "islow" => J_DCT_METHOD::JDCT_ISLOW,
                    "ifast" => J_DCT_METHOD::JDCT_IFAST,
                    "float" => J_DCT_METHOD::JDCT_FLOAT,
                    other => J_DCT_METHOD(other.parse::<i32>().unwrap() as u32),
                }
            }
            "-size" => options.size = next().parse().unwrap(),
            "-blocks" => options.blocks = next().parse().unwrap(),
            "-seed" => options.seed = next().parse().unwrap(),
            "-qmax" => options.qmax = next().parse().unwrap(),
            "-extreme" => options.extreme = true,
            other => panic!("unknown option {other}"),
        }
    }
    options
}

/// `idct.c`'s `MakeBlock`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversions to JCOEF and of nonnegative indexes"
)]
#[expect(clippy::needless_range_loop, reason = "the probe's loop")]
fn make_block(rng: &mut Rng, block: &mut JBLOCK, qtbl: &JQUANT_TBL, extreme: bool) {
    let kind = rng.rand(if extreme { 5 } else { 4 });
    block.fill(0);
    let q = |k: usize| i32::from(qtbl.quantval[k]);
    match kind {
        0 => {
            for k in 0..DCTSIZE2 {
                let lim = 1024 / (1 + (k / DCTSIZE) as i32 + (k % DCTSIZE) as i32);
                if k == 0 || rng.rand(4) == 0 {
                    block[k] = ((rng.rand(2 * lim + 1) - lim) / q(k)) as JCOEF;
                }
            }
        }
        1 => {
            block[0] = ((rng.rand(2049) - 1024) / q(0)) as JCOEF;
            if rng.rand(2) != 0 {
                let k = 1 + rng.rand(DCTSIZE as i32 - 1) as usize;
                block[k] = (rng.rand(201) - 100) as JCOEF;
            }
        }
        2 => {
            let k = rng.rand(DCTSIZE2 as i32) as usize;
            block[k] = ((rng.rand(4097) - 2048) / q(k)) as JCOEF;
        }
        3 => {
            for coef in block.iter_mut() {
                *coef = (rng.rand(257) - 128) as JCOEF;
            }
        }
        _ => {
            const EDGES: [i32; 5] = [-32768, 32767, -1, 0, 1];
            for coef in block.iter_mut() {
                *coef = if rng.rand(2) != 0 {
                    (rng.next() & 0xFFFF) as u16 as JCOEF
                } else {
                    EDGES[rng.rand(5) as usize] as JCOEF
                };
            }
        }
    }
}

/// Runs `idct.c`'s sequence on the port for `args`.
#[expect(
    clippy::cast_sign_loss,
    reason = "C's conversions of nonnegative sizes"
)]
pub fn run_idct(args: &[&str]) -> Fields {
    let options = parse_idct(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));
    let error = Rc::new(RefCell::new((0, String::new())));
    let mut cinfo = new_object(&log, &error);
    let mut rng = Rng(options.seed.wrapping_mul(2_654_435_761).wrapping_add(1));
    let mut out: Vec<String> = Vec::new();

    let mut body = || -> Result<(), ErrorExit> {
        jpeg_create_decompress(&mut cinfo)?;
        cinfo.data_precision = 8;
        cinfo.num_components = 1;
        cinfo.dct_method = options.method;
        let mut comp_info = alloc_small_vec(
            &mut cinfo,
            JPOOL_IMAGE,
            SIZEOF_jpeg_component_info,
            1,
            jpeg_component_info::default(),
        )?;
        comp_info[0].component_needed = true;
        comp_info[0].DCT_scaled_size = options.size;
        cinfo.comp_info = Some(comp_info);
        let mut qtbl = JQUANT_TBL::default();
        for value in &mut qtbl.quantval {
            *value = UINT16::try_from(1 + rng.rand(options.qmax)).unwrap();
        }
        let qtbl = alloc_small_box(&mut cinfo, JPOOL_IMAGE, SIZEOF_JQUANT_TBL, qtbl)?;
        cinfo.comp_info_mut()[0].quant_table = Some(qtbl);

        prepare_range_limit_table(&mut cinfo)?;
        jinit_inverse_dct(&mut cinfo)?;
        let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
        let mut buffer: SampleArray = alloc_sarray(&mut cinfo, JPOOL_IMAGE, 2 * 16, 16)?;
        let start_pass = cinfo.idct().start_pass;
        start_pass(&mut cinfo)?;

        let mut block: JBLOCK = [0; DCTSIZE2];
        for b in 0..options.blocks {
            let output_col = ((b & 1) * options.size) as JDIMENSION;
            let qtbl = cinfo.comp_info()[0].quant_table.as_deref().unwrap();
            make_block(&mut rng, &mut block, qtbl, options.extreme);
            let inverse_dct = cinfo.idct().inverse_DCT[0].unwrap();
            inverse_dct(
                &cinfo,
                &cinfo.comp_info()[0],
                &block,
                buffer.as_rows_mut(),
                output_col,
            );
            let mut hash = FNV_OFFSET;
            for row in 0..options.size as usize {
                for col in 0..options.size as usize {
                    hash = (hash ^ u64::from(buffer.row(row)[output_col as usize + col]))
                        .wrapping_mul(FNV_PRIME);
                }
            }
            out.push(format!("{hash:016x}"));
        }
        Ok(())
    };
    let _ = body();
    drop(body);

    push(&log, "out", out.join(","));
    print_memory(&log, cinfo.mem());
    print_error(&log, &cinfo, &error);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}

/// `coefidct.c`'s `MAX_CALLS`.
pub const MAX_CALLS: i64 = 1_000_000;

/// The options of `coefidct.c`.
#[derive(Default)]
struct CoefOptions {
    trace: i32,
    suspend: usize,
    dct: Option<String>,
    dct2: Option<String>,
    scaled: i32,
    scaledc: [i32; MAX_COMPONENTS],
    unneeded: [bool; MAX_COMPONENTS],
    buffered: bool,
    nosmooth: bool,
    crop: Option<(i32, i32)>,
}

fn parse_coef(args: &[&str]) -> CoefOptions {
    let mut options = CoefOptions::default();
    let mut args = args.iter();
    while let Some(&arg) = args.next() {
        let mut next = || -> i32 { args.next().expect("option value").parse().unwrap() };
        match arg {
            "-trace" => options.trace = next(),
            "-suspend" => options.suspend = usize::try_from(next()).unwrap(),
            "-scaled" => options.scaled = next(),
            "-scaledc" => {
                let ci = next();
                let n = next();
                if (0..MAX_COMPONENTS as i32).contains(&ci) {
                    options.scaledc[usize::try_from(ci).unwrap()] = n;
                }
            }
            "-unneeded" => {
                let ci = next();
                if (0..MAX_COMPONENTS as i32).contains(&ci) {
                    options.unneeded[usize::try_from(ci).unwrap()] = true;
                }
            }
            "-crop" => {
                let first = next();
                let last = next();
                options.crop = Some((first, last));
            }
            "-buffered" => options.buffered = true,
            "-nosmooth" => options.nosmooth = true,
            "-dct" => options.dct = Some((*args.next().unwrap()).to_owned()),
            "-dct2" => options.dct2 = Some((*args.next().unwrap()).to_owned()),
            other => panic!("unknown option {other}"),
        }
    }
    options
}

/// `coefidct.c`'s `ParseMethod`.
#[expect(clippy::cast_sign_loss, reason = "C's (J_DCT_METHOD)atoi")]
pub fn parse_method(name: &str) -> J_DCT_METHOD {
    match name {
        "ifast" => J_DCT_METHOD::JDCT_IFAST,
        "float" => J_DCT_METHOD::JDCT_FLOAT,
        "islow" => J_DCT_METHOD::JDCT_ISLOW,
        other => J_DCT_METHOD(other.parse::<i32>().unwrap_or(0) as u32),
    }
}

/// `coefidct.c`'s `RunLog`.
#[derive(Default)]
pub struct RunLog {
    pub text: String,
    last_value: Option<i32>,
    last_count: i64,
}

impl RunLog {
    pub fn flush(&mut self) {
        if self.last_count == 0 {
            return;
        }
        if !self.text.is_empty() {
            self.text.push(',');
        }
        self.text
            .push_str(&format!("{}*{}", self.last_value.unwrap(), self.last_count));
        self.last_count = 0;
    }

    pub fn log(&mut self, value: i32) {
        if self.last_value != Some(value) {
            self.flush();
        }
        self.last_value = Some(value);
        self.last_count += 1;
    }
}

/// The state of a `coefidct.c` run beside the object.
struct Run<'a> {
    options: &'a CoefOptions,
    log: Log,
    state: Rc<RefCell<SuspendState>>,
    calls: i64,
    stuck: bool,
    consume: RunLog,
    decompress: RunLog,
    rows: Vec<String>,
    output_buf: Vec<SampleArray>,
    buf_rows: Vec<usize>,
    buf_cols: Vec<usize>,
}

impl Run<'_> {
    /// The probe's `Resume`.
    fn resume(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> bool {
        self.calls += 1;
        if self.options.suspend == 0 || self.state.borrow().fake_eoi || self.calls > MAX_CALLS {
            self.stuck = true;
            return false;
        }
        release(cinfo, &self.state, self.options.suspend);
        true
    }

    /// The probe's `ConsumeInput`.
    fn consume_input(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<i32, ErrorExit> {
        let consume_input = cinfo.inputctl().pub_.consume_input;
        let value = consume_input(cinfo)?;
        self.consume.log(value);
        Ok(value)
    }

    /// The probe's `HashRows`.
    fn hash_rows(&mut self) {
        let mut hash = FNV_OFFSET;
        for (ci, buffer) in self.output_buf.iter().enumerate() {
            for row in 0..self.buf_rows[ci] {
                for &sample in &buffer.row(row)[..self.buf_cols[ci]] {
                    hash = (hash ^ u64::from(sample)).wrapping_mul(FNV_PRIME);
                }
            }
        }
        self.rows.push(format!("{hash:016x}"));
    }

    /// The probe's `OutputPass`.
    fn output_pass(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        push(
            &self.log,
            "pass",
            format!(
                "{},{},{}",
                cinfo.output_scan_number, cinfo.input_scan_number, cinfo.dct_method.0
            ),
        );
        self.rows.clear();
        let start_pass = cinfo.idct().start_pass;
        start_pass(cinfo)?;
        let start_output_pass = cinfo.coef().start_output_pass;
        start_output_pass(cinfo)?;
        while cinfo.output_iMCU_row < cinfo.total_iMCU_rows {
            let decompress_data = cinfo.coef().decompress_data;
            let mut rows: Vec<JSAMPROWS_MUT<'_, '_>> = self
                .output_buf
                .iter_mut()
                .map(SampleArray::as_rows_mut)
                .collect();
            let status = decompress_data(cinfo, &mut rows)?;
            drop(rows);
            self.decompress.log(status);
            if status == JPEG_SUSPENDED {
                if !self.resume(cinfo) {
                    break;
                }
                continue;
            }
            self.hash_rows();
        }
        push(&self.log, "rows", self.rows.join(","));
        Ok(())
    }

    /// The probe's `FinishOutput`.
    fn finish_output(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        while cinfo.input_scan_number <= cinfo.output_scan_number
            && !cinfo.inputctl().pub_.eoi_reached
        {
            if self.consume_input(cinfo)? == JPEG_SUSPENDED && !self.resume(cinfo) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// The probe's `ConsumeToEOI`.
    fn consume_to_eoi(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        while !cinfo.inputctl().pub_.eoi_reached {
            if self.consume_input(cinfo)? == JPEG_SUSPENDED && !self.resume(cinfo) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// The probe's `PrintFinal`.
    fn print_final(
        &mut self,
        cinfo: &jpeg_decompress_struct<'_>,
        error: &Rc<RefCell<(i32, String)>>,
    ) {
        self.consume.flush();
        self.decompress.flush();
        push(&self.log, "consume", &self.consume.text);
        push(&self.log, "decompress", &self.decompress.text);
        if self.stuck {
            push(&self.log, "stuck", 1);
        }
        if cinfo.master.is_some() && cinfo.inputctl.is_some() {
            push(
                &self.log,
                "final",
                format!(
                    "{},{},{},{},{},{}",
                    cinfo.input_scan_number,
                    cinfo.output_scan_number,
                    cinfo.input_iMCU_row,
                    cinfo.output_iMCU_row,
                    cinfo.master().pub_.last_good_iMCU_row,
                    i32::from(cinfo.inputctl().pub_.eoi_reached)
                ),
            );
        }
        print_memory(&self.log, cinfo.mem());
        print_error(&self.log, cinfo, error);
    }
}

/// Runs `coefidct.c`'s sequence on the port for `args` and the input `data`.
#[expect(clippy::too_many_lines, reason = "the probe's main")]
#[expect(clippy::cast_sign_loss, reason = "C's conversions of sizes")]
pub fn run_coefidct(args: &[&str], data: &[u8]) -> Fields {
    let options = parse_coef(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));
    let error = Rc::new(RefCell::new((0, String::new())));
    let mut cinfo = new_object(&log, &error);
    let mut run = Run {
        options: &options,
        log: Rc::clone(&log),
        state: Rc::new(RefCell::new(SuspendState {
            size: data.len(),
            ..SuspendState::default()
        })),
        calls: 0,
        stuck: false,
        consume: RunLog::default(),
        decompress: RunLog::default(),
        rows: Vec::new(),
        output_buf: Vec::new(),
        buf_rows: Vec::new(),
        buf_cols: Vec::new(),
    };

    let mut body = |run: &mut Run<'_>| -> Result<(), ErrorExit> {
        jpeg_create_decompress(&mut cinfo)?;
        cinfo.err.trace_level = options.trace;
        if options.suspend > 0 {
            cinfo.src = Some(Box::new(jpeg_source_mgr {
                next_input_byte: 0,
                bytes_in_buffer: 0,
                buffer: InputBuffer::Borrowed(data),
                methods: Some(Box::new(SuspendingSource(Rc::clone(&run.state)))),
            }));
        } else {
            jpeg_mem_src(&mut cinfo, data)?;
        }

        let mut status;
        loop {
            if options.suspend > 0 {
                release(&mut cinfo, &run.state, options.suspend);
            }
            status = jpeg_read_header(&mut cinfo, true)?;
            if status != JPEG_SUSPENDED || options.suspend == 0 {
                break;
            }
        }
        push(&log, "header", status);
        if status != JPEG_HEADER_OK {
            return Ok(());
        }

        if let Some(dct) = &options.dct {
            cinfo.dct_method = parse_method(dct);
        }
        for ci in 0..cinfo.num_components as usize {
            let compptr = &mut cinfo.comp_info_mut()[ci];
            if options.scaled > 0 {
                compptr.DCT_scaled_size = options.scaled;
            }
            if ci < MAX_COMPONENTS && options.scaledc[ci] != 0 {
                compptr.DCT_scaled_size = options.scaledc[ci];
            }
            if ci < MAX_COMPONENTS && options.unneeded[ci] {
                compptr.component_needed = false;
            }
        }
        cinfo.buffered_image = options.buffered;
        if options.nosmooth {
            cinfo.do_block_smoothing = false;
        }

        /* jdmaster.c's master_selection, for these modules */
        prepare_range_limit_table(&mut cinfo)?;
        jinit_inverse_dct(&mut cinfo)?;
        if cinfo.arith_code {
            jinit_arith_decoder(&mut cinfo)?;
        } else if cinfo.progressive_mode {
            jinit_phuff_decoder(&mut cinfo)?;
        } else {
            jinit_huff_decoder(&mut cinfo)?;
        }
        let use_c_buffer = cinfo.inputctl().pub_.has_multiple_scans || cinfo.buffered_image;
        jinit_d_coef_controller(&mut cinfo, use_c_buffer)?;
        /* The main controller's buffer */
        for ci in 0..cinfo.num_components as usize {
            let compptr = &cinfo.comp_info()[ci];
            /* C's unsigned and int arithmetic, which a negative size wraps */
            let cols = compptr
                .width_in_blocks
                .wrapping_mul(compptr.DCT_scaled_size as JDIMENSION);
            let rows = compptr.v_samp_factor.wrapping_mul(compptr.DCT_scaled_size) as JDIMENSION;
            let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
            run.output_buf
                .push(alloc_sarray(&mut cinfo, JPOOL_IMAGE, cols, rows)?);
            run.buf_cols.push(cols as usize);
            run.buf_rows.push(rows as usize);
        }
        let realize_virt_arrays = cinfo.mem().pub_.realize_virt_arrays;
        realize_virt_arrays(&mut cinfo)?;
        let start_input_pass = cinfo.inputctl().pub_.start_input_pass;
        start_input_pass(&mut cinfo)?;
        let mcus_per_row = cinfo.MCUs_per_row;
        let master = &mut cinfo.master_mut().pub_;
        master.first_iMCU_col = 0;
        master.last_iMCU_col = mcus_per_row.wrapping_sub(1);
        master.last_good_iMCU_row = 0;
        if let Some((first, last)) = options.crop {
            let single = cinfo.comps_in_scan == 1 && cinfo.num_components == 1;
            let hsf: Vec<i32> = cinfo
                .comp_info()
                .iter()
                .map(|compptr| if single { 1 } else { compptr.h_samp_factor })
                .collect();
            let master = &mut cinfo.master_mut().pub_;
            master.first_iMCU_col = first as JDIMENSION;
            master.last_iMCU_col = last as JDIMENSION;
            for (ci, hsf) in hsf.into_iter().enumerate() {
                master.first_MCU_col[ci] = (first * hsf) as JDIMENSION;
                master.last_MCU_col[ci] = ((last + 1) * hsf - 1) as JDIMENSION;
            }
        }

        if options.buffered {
            let mut passes = 0;
            while !cinfo.inputctl().pub_.eoi_reached && !run.stuck {
                /* jpeg_start_output(&cinfo, cinfo.input_scan_number) */
                let mut scan_number = cinfo.input_scan_number;
                if scan_number <= 0 {
                    scan_number = 1;
                }
                if cinfo.inputctl().pub_.eoi_reached && scan_number > cinfo.input_scan_number {
                    scan_number = cinfo.input_scan_number;
                }
                cinfo.output_scan_number = scan_number;
                passes += 1;
                if passes > 1
                    && let Some(dct2) = &options.dct2
                {
                    cinfo.dct_method = parse_method(dct2);
                }
                run.output_pass(&mut cinfo)?;
                if run.stuck {
                    break;
                }
                run.finish_output(&mut cinfo)?;
            }
            if !run.stuck && cinfo.output_scan_number != cinfo.input_scan_number {
                cinfo.output_scan_number = cinfo.input_scan_number;
                passes += 1;
                if passes > 1
                    && let Some(dct2) = &options.dct2
                {
                    cinfo.dct_method = parse_method(dct2);
                }
                run.output_pass(&mut cinfo)?;
            }
        } else {
            /* jpeg_start_decompress */
            if cinfo.inputctl().pub_.has_multiple_scans {
                run.consume_to_eoi(&mut cinfo)?;
            }
            cinfo.output_scan_number = cinfo.input_scan_number;
            if !run.stuck {
                run.output_pass(&mut cinfo)?;
            }
            /* jpeg_finish_decompress */
            if !run.stuck {
                run.consume_to_eoi(&mut cinfo)?;
            }
        }
        Ok(())
    };
    if body(&mut run).is_err() && !run.rows.is_empty() {
        push(&log, "rows", run.rows.join(","));
    }
    drop(body);
    run.print_final(&cinfo, &error);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}
