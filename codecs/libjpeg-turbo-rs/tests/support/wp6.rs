//! The port's side of the WP6 probe `tools/oracle/probes/wp6/api.c`: the
//! same options, the same API calls in the same order, and the same printed
//! fields, so that `tests/wp6_api.rs` can compare both outputs exactly.
//! Include it next to `support/oracle.rs`, `support/wp2.rs` and
//! `support/wp4.rs`.

#![allow(dead_code, reason = "each test binary uses part of the support")]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use jpeg::jdapimin::{
    jpeg_consume_input, jpeg_create_decompress, jpeg_destroy_decompress, jpeg_finish_decompress,
    jpeg_input_complete, jpeg_read_header,
};
use jpeg::jdapistd::{
    jpeg_crop_scanline, jpeg_finish_output, jpeg_read_raw_data, jpeg_read_scanlines,
    jpeg_skip_scanlines, jpeg_start_decompress, jpeg_start_output,
};
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdmaster::{jpeg_calc_output_dimensions, jpeg_new_colormap};
use jpeg::jerror::ErrorExit;
use jpeg::jmorecfg::JDIMENSION;
use jpeg::jpeglib::{
    InputBuffer, J_COLOR_SPACE, JPEG_HEADER_OK, JPEG_REACHED_EOI, JPEG_SUSPENDED, RowListMut,
    jpeg_decompress_struct, jpeg_progress_methods, jpeg_progress_mgr, jpeg_source_mgr,
};

use crate::oracle::Fields;
use crate::wp2::{Log, SuspendState, SuspendingSource, print_memory, push, release};
use crate::wp4::{FNV_OFFSET, FNV_PRIME, MAX_CALLS, RunLog, new_object, parse_method, print_error};

/// A step of `-plan`.
#[derive(Clone, Copy, Debug)]
enum Step {
    Read(JDIMENSION),
    Skip(JDIMENSION),
    Crop(JDIMENSION, JDIMENSION),
    CropNull,
    Calc,
    NewColormap,
    StartDecompress,
    StartOutput,
    FinishOutput,
    Precision12,
    End,
}

/// The options of `api.c`.
#[derive(Default)]
struct Options {
    trace: i32,
    suspend: usize,
    out: Option<u32>,
    nofancy: bool,
    nosmooth: bool,
    ccir601: bool,
    quantize: bool,
    twopass: bool,
    precision: i32,
    calc: bool,
    progress: bool,
    raw: bool,
    buffered: bool,
    absorb: bool,
    newcolormap: bool,
    extra: bool,
    twice: bool,
    scale: Option<(u32, u32)>,
    dct: Option<String>,
    rawlines: JDIMENSION,
    first_scan: Option<i32>,
    passes: i64,
    rows: JDIMENSION,
    plan: Vec<Step>,
}

fn parse_plan(text: &str) -> Vec<Step> {
    text.split(',')
        .filter(|step| !step.is_empty())
        .map(|step| {
            let (op, rest) = step.split_at(1);
            match op {
                "r" => Step::Read(rest.parse().unwrap()),
                "s" => Step::Skip(rest.parse().unwrap()),
                "c" => {
                    let (x, w) = rest.split_once(':').expect("cX:W");
                    Step::Crop(x.parse().unwrap(), w.parse().unwrap())
                }
                "C" => Step::CropNull,
                "d" => Step::Calc,
                "n" => Step::NewColormap,
                "S" => Step::StartDecompress,
                "O" => Step::StartOutput,
                "F" => Step::FinishOutput,
                "P" => Step::Precision12,
                "e" => Step::End,
                other => panic!("bad plan step {other}"),
            }
        })
        .collect()
}

fn parse(args: &[&str]) -> Options {
    let mut options = Options {
        rows: 1,
        ..Options::default()
    };
    let mut args = args.iter();
    while let Some(&arg) = args.next() {
        let mut next = || *args.next().expect("option value");
        match arg {
            "-trace" => options.trace = next().parse().unwrap(),
            "-suspend" => options.suspend = next().parse().unwrap(),
            "-out" => options.out = Some(next().parse::<i32>().unwrap().cast_unsigned()),
            "-dct" => options.dct = Some(next().to_owned()),
            "-scale" => {
                let num = next().parse().unwrap();
                let denom = next().parse().unwrap();
                options.scale = Some((num, denom));
            }
            "-precision" => options.precision = next().parse().unwrap(),
            "-rows" => options.rows = next().parse().unwrap(),
            "-rawlines" => options.rawlines = next().parse().unwrap(),
            "-scan" => options.first_scan = Some(next().parse().unwrap()),
            "-passes" => options.passes = next().parse().unwrap(),
            "-plan" => options.plan = parse_plan(next()),
            "-nofancy" => options.nofancy = true,
            "-nosmooth" => options.nosmooth = true,
            "-ccir601" => options.ccir601 = true,
            "-quantize" => options.quantize = true,
            "-twopass" => options.twopass = true,
            "-calc" => options.calc = true,
            "-progress" => options.progress = true,
            "-raw" => options.raw = true,
            "-buffered" => options.buffered = true,
            "-absorb" => options.absorb = true,
            "-newcolormap" => options.newcolormap = true,
            "-extra" => options.extra = true,
            "-twice" => options.twice = true,
            other => panic!("unknown option {other}"),
        }
    }
    options
}

/// The probe's progress monitor: counts calls and hashes the fields.
struct Progress {
    calls: Rc<Cell<i64>>,
    hash: Rc<Cell<u64>>,
}

impl jpeg_progress_methods for Progress {
    fn progress_monitor(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<(), ErrorExit> {
        let mgr = cinfo.progress.as_deref().expect("progress");
        self.calls.set(self.calls.get() + 1);
        let mut hash = self.hash.get();
        for value in [
            mgr.pass_counter,
            mgr.pass_limit,
            i64::from(mgr.completed_passes),
            i64::from(mgr.total_passes),
        ] {
            hash = (hash ^ value.cast_unsigned()).wrapping_mul(FNV_PRIME);
        }
        self.hash.set(hash);
        Ok(())
    }
}

/// The result of a pass: `api.c`'s 1, 0 and 2.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pass {
    Complete,
    Stuck,
    End,
}

/// The state of an `api.c` run beside the object.
struct Run<'a> {
    options: &'a Options,
    log: Log,
    state: Rc<RefCell<SuspendState>>,
    calls: i64,
    stuck: bool,
    read: RunLog,
    skip: RunLog,
    raw: RunLog,
    rows: Vec<String>,
}

fn row_hash(row: &[u8]) -> String {
    let mut hash = FNV_OFFSET;
    for &sample in row {
        hash = (hash ^ u64::from(sample)).wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
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

    /// The probe's `ReadRows`: one `jpeg_read_scanlines` call.
    fn read_rows(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        max_lines: JDIMENSION,
    ) -> Result<JDIMENSION, ErrorExit> {
        let width = cinfo.output_width as usize * usize::try_from(cinfo.output_components).unwrap();
        let mut scanlines: Vec<Vec<u8>> = vec![vec![0; width.max(1)]; (max_lines as usize).max(1)];
        let mut rows: Vec<&mut [u8]> = scanlines
            .iter_mut()
            .take(max_lines as usize)
            .map(Vec::as_mut_slice)
            .collect();
        let n = jpeg_read_scanlines(cinfo, RowListMut::Separate(&mut rows), max_lines)?;
        self.read.log(n.cast_signed());
        for row in scanlines.iter().take(n as usize) {
            self.rows.push(row_hash(&row[..width]));
        }
        Ok(n)
    }

    /// The probe's `ReadLines`.
    fn read_lines(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        lines: JDIMENSION,
        max_lines: JDIMENSION,
    ) -> Result<bool, ErrorExit> {
        let mut got: JDIMENSION = 0;
        while got < lines && cinfo.output_scanline < cinfo.output_height {
            let n = self.read_rows(
                cinfo,
                if max_lines != 0 {
                    max_lines
                } else {
                    lines - got
                },
            )?;
            got += n;
            if n == 0 && !self.resume(cinfo) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// The probe's `RunSteps`.
    fn run_steps(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<Pass, ErrorExit> {
        for &step in &self.options.plan {
            match step {
                Step::Read(lines) => {
                    if !self.read_lines(cinfo, lines, 0)? {
                        return Ok(Pass::Stuck);
                    }
                }
                Step::Skip(lines) => {
                    let n = jpeg_skip_scanlines(cinfo, lines)?;
                    self.skip.log(n.cast_signed());
                }
                Step::Crop(x, w) => {
                    let (mut xoffset, mut width) = (x, w);
                    jpeg_crop_scanline(cinfo, Some(&mut xoffset), Some(&mut width))?;
                    push(
                        &self.log,
                        "crop",
                        format!("{xoffset},{width},{}", cinfo.output_width),
                    );
                }
                Step::CropNull => jpeg_crop_scanline(cinfo, None, None)?,
                Step::Calc => jpeg_calc_output_dimensions(cinfo)?,
                Step::NewColormap => jpeg_new_colormap(cinfo)?,
                Step::StartDecompress => {
                    jpeg_start_decompress(cinfo)?;
                }
                Step::StartOutput => {
                    let scan = cinfo.input_scan_number;
                    jpeg_start_output(cinfo, scan)?;
                }
                Step::FinishOutput => {
                    jpeg_finish_output(cinfo)?;
                }
                Step::Precision12 => cinfo.data_precision = 12,
                Step::End => return Ok(Pass::End),
            }
        }
        Ok(Pass::Complete)
    }

    /// The probe's `ReadPass`.
    fn read_pass(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<Pass, ErrorExit> {
        let result = self.run_steps(cinfo)?;
        if result != Pass::Complete {
            return Ok(result);
        }
        let (height, rows) = (cinfo.output_height, self.options.rows);
        Ok(if self.read_lines(cinfo, height, rows)? {
            Pass::Complete
        } else {
            Pass::Stuck
        })
    }

    /// The probe's `RawPass`.
    fn raw_pass(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<Pass, ErrorExit> {
        let result = self.run_steps(cinfo)?;
        if result != Pass::Complete {
            return Ok(result);
        }
        let mut lines = (cinfo.max_v_samp_factor * cinfo.min_DCT_scaled_size).cast_unsigned();
        if self.options.rawlines != 0 {
            lines = self.options.rawlines;
        }
        let shapes: Vec<(usize, usize)> = cinfo
            .comp_info()
            .iter()
            .take(usize::try_from(cinfo.num_components).unwrap())
            .map(|compptr| {
                (
                    usize::try_from(compptr.v_samp_factor * compptr.DCT_scaled_size).unwrap(),
                    compptr.width_in_blocks as usize
                        * usize::try_from(compptr.DCT_scaled_size).unwrap(),
                )
            })
            .collect();
        let mut planes: Vec<Vec<Vec<u8>>> = shapes
            .iter()
            .map(|&(rows, width)| vec![vec![0; width.max(1)]; rows])
            .collect();
        while cinfo.output_scanline < cinfo.output_height {
            let n = {
                let mut lists: Vec<Vec<&mut [u8]>> = planes
                    .iter_mut()
                    .zip(&shapes)
                    .map(|(plane, &(_, width))| {
                        plane.iter_mut().map(|row| &mut row[..width]).collect()
                    })
                    .collect();
                let mut image: Vec<RowListMut<'_, '_, u8>> = lists
                    .iter_mut()
                    .map(|list| RowListMut::Separate(list.as_mut_slice()))
                    .collect();
                jpeg_read_raw_data(cinfo, &mut image, lines)?
            };
            self.raw.log(n.cast_signed());
            if n == 0 {
                if !self.resume(cinfo) {
                    return Ok(Pass::Stuck);
                }
                continue;
            }
            for (plane, &(_, width)) in planes.iter().zip(&shapes) {
                for row in plane {
                    self.rows.push(row_hash(&row[..width]));
                }
            }
        }
        Ok(Pass::Complete)
    }

    /// The probe's `BufferedPasses`.
    fn buffered_passes(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<Pass, ErrorExit> {
        if self.options.newcolormap {
            jpeg_new_colormap(cinfo)?;
        }
        if self.options.absorb {
            loop {
                let value = jpeg_consume_input(cinfo)?;
                if value == JPEG_REACHED_EOI {
                    break;
                }
                if value == JPEG_SUSPENDED && !self.resume(cinfo) {
                    return Ok(Pass::Stuck);
                }
            }
            push(&self.log, "absorb", cinfo.input_scan_number);
        }
        let mut pass: i64 = 0;
        loop {
            let mut scan = cinfo.input_scan_number;
            if pass == 0
                && let Some(first) = self.options.first_scan
            {
                scan = first;
            }
            while !jpeg_start_output(cinfo, scan)? {
                if !self.resume(cinfo) {
                    return Ok(Pass::Stuck);
                }
            }
            self.rows.push(format!("p{}", cinfo.output_scan_number));
            let result = self.read_pass(cinfo)?;
            if result != Pass::Complete {
                return Ok(result);
            }
            while !jpeg_finish_output(cinfo)? {
                if !self.resume(cinfo) {
                    return Ok(Pass::Stuck);
                }
            }
            push(
                &self.log,
                "pass",
                format!(
                    "{},{},{}",
                    cinfo.output_scan_number,
                    cinfo.input_scan_number,
                    i32::from(cinfo.inputctl().pub_.eoi_reached)
                ),
            );
            if jpeg_input_complete(cinfo)? && cinfo.input_scan_number == cinfo.output_scan_number {
                return Ok(Pass::Complete);
            }
            if self.options.passes != 0 && pass + 1 >= self.options.passes {
                return Ok(Pass::Complete);
            }
            pass += 1;
        }
    }

    /// The probe's `DecodeOnce`: `false` when stuck.
    #[expect(clippy::too_many_lines, reason = "the probe's function")]
    fn decode_once<'src>(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'src>,
        data: &'src [u8],
    ) -> Result<bool, ErrorExit> {
        let options = self.options;
        if options.suspend > 0 {
            self.state = Rc::new(RefCell::new(SuspendState {
                size: data.len(),
                ..SuspendState::default()
            }));
            cinfo.src = Some(Box::new(jpeg_source_mgr {
                next_input_byte: 0,
                bytes_in_buffer: 0,
                buffer: InputBuffer::Borrowed(data),
                methods: Some(Box::new(SuspendingSource(Rc::clone(&self.state)))),
            }));
        } else {
            jpeg_mem_src(cinfo, data)?;
        }

        let mut status;
        loop {
            if options.suspend > 0 {
                release(cinfo, &self.state, options.suspend);
            }
            status = jpeg_read_header(cinfo, true)?;
            if status != JPEG_SUSPENDED || options.suspend == 0 {
                break;
            }
        }
        push(&self.log, "header", status);
        if status != JPEG_HEADER_OK {
            return Ok(false);
        }

        if let Some(out) = options.out {
            cinfo.out_color_space = J_COLOR_SPACE(out);
        }
        if let Some(dct) = &options.dct {
            cinfo.dct_method = parse_method(dct);
        }
        if let Some((num, denom)) = options.scale
            && denom != 0
        {
            cinfo.scale_num = num;
            cinfo.scale_denom = denom;
        }
        if options.nofancy {
            cinfo.do_fancy_upsampling = false;
        }
        if options.nosmooth {
            cinfo.do_block_smoothing = false;
        }
        if options.ccir601 {
            cinfo.CCIR601_sampling = true;
        }
        if options.quantize {
            cinfo.quantize_colors = true;
        }
        if options.twopass {
            cinfo.two_pass_quantize = true;
        }
        if options.precision != 0 {
            cinfo.data_precision = options.precision;
        }
        if options.raw {
            cinfo.raw_data_out = true;
        }
        if options.buffered {
            cinfo.buffered_image = true;
        }
        if options.calc {
            jpeg_calc_output_dimensions(cinfo)?;
            push(
                &self.log,
                "calc",
                format!(
                    "{},{},{},{},{},{}",
                    cinfo.output_width,
                    cinfo.output_height,
                    cinfo.out_color_components,
                    cinfo.output_components,
                    cinfo.rec_outbuf_height,
                    cinfo.min_DCT_scaled_size
                ),
            );
        }

        let mut starts = 0;
        while !jpeg_start_decompress(cinfo)? {
            starts += 1;
            if !self.resume(cinfo) {
                push(&self.log, "start", starts);
                return Ok(false);
            }
        }
        push(&self.log, "start", starts);
        push(
            &self.log,
            "dims",
            format!(
                "{},{},{},{},{},{},{}",
                cinfo.output_width,
                cinfo.output_height,
                cinfo.out_color_components,
                cinfo.output_components,
                cinfo.rec_outbuf_height,
                cinfo.min_DCT_scaled_size,
                cinfo.global_state
            ),
        );

        let result = if options.raw {
            self.raw_pass(cinfo)?
        } else if options.buffered {
            self.buffered_passes(cinfo)?
        } else {
            self.read_pass(cinfo)?
        };
        if result == Pass::Stuck {
            return Ok(false);
        }
        if result == Pass::Complete && options.extra {
            if options.raw {
                /* After the last row the call warns before it reads the
                 * planes, so none are passed. */
                let n = jpeg_read_raw_data(cinfo, &mut [], 1000)?;
                self.raw.log(n.cast_signed());
            } else {
                self.read_rows(cinfo, 1)?;
                let n = jpeg_skip_scanlines(cinfo, 5)?;
                self.skip.log(n.cast_signed());
            }
        }

        print_memory(&self.log, cinfo.mem());
        let mut finishes = 0;
        while !jpeg_finish_decompress(cinfo)? {
            finishes += 1;
            if !self.resume(cinfo) {
                push(&self.log, "finish", finishes);
                return Ok(false);
            }
        }
        push(&self.log, "finish", finishes);
        Ok(true)
    }

    /// The probe's `PrintFinal`.
    fn print_final(
        &mut self,
        cinfo: &jpeg_decompress_struct<'_>,
        progress: Option<(i64, u64)>,
        error: &Rc<RefCell<(i32, String)>>,
    ) {
        self.read.flush();
        self.skip.flush();
        self.raw.flush();
        if !self.rows.is_empty() {
            push(&self.log, "rows", self.rows.join(","));
        }
        push(&self.log, "read", &self.read.text);
        push(&self.log, "skip", &self.skip.text);
        push(&self.log, "raw", &self.raw.text);
        if let Some((calls, hash)) = progress {
            push(&self.log, "progress", format!("{calls},{hash:016x}"));
        }
        if self.stuck {
            push(&self.log, "stuck", 1);
        }
        push(
            &self.log,
            "final",
            format!(
                "{},{},{},{},{},{},{}",
                cinfo.global_state,
                cinfo.output_scanline,
                cinfo.input_scan_number,
                cinfo.output_scan_number,
                cinfo.input_iMCU_row,
                cinfo.output_iMCU_row,
                cinfo
                    .inputctl
                    .as_deref()
                    .map_or(-1, |inputctl| i32::from(inputctl.pub_.eoi_reached))
            ),
        );
        print_error(&self.log, cinfo, error);
    }
}

/// Runs `api.c`'s sequence on the port for `args` and the input `data`.
pub fn run_rust(args: &[&str], data: &[u8]) -> Fields {
    let options = parse(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));
    let error = Rc::new(RefCell::new((0, String::new())));
    let mut cinfo = new_object(&log, &error);
    let calls = Rc::new(Cell::new(0));
    let hash = Rc::new(Cell::new(FNV_OFFSET));
    let mut run = Run {
        options: &options,
        log: Rc::clone(&log),
        state: Rc::new(RefCell::new(SuspendState::default())),
        calls: 0,
        stuck: false,
        read: RunLog::default(),
        skip: RunLog::default(),
        raw: RunLog::default(),
        rows: Vec::new(),
    };

    let mut body = |run: &mut Run<'_>| -> Result<(), ErrorExit> {
        jpeg_create_decompress(&mut cinfo)?;
        cinfo.err.trace_level = options.trace;
        if options.progress {
            cinfo.progress = Some(Box::new(jpeg_progress_mgr {
                methods: Some(Box::new(Progress {
                    calls: Rc::clone(&calls),
                    hash: Rc::clone(&hash),
                })),
                pass_counter: 0,
                pass_limit: 0,
                completed_passes: 0,
                total_passes: 0,
            }));
        }
        if run.decode_once(&mut cinfo, data)? && options.twice {
            run.rows.push("again".to_owned());
            run.decode_once(&mut cinfo, data)?;
        }
        Ok(())
    };
    let _ = body(&mut run);
    drop(body);
    let progress = options.progress.then(|| (calls.get(), hash.get()));
    run.print_final(&cinfo, progress, &error);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}
