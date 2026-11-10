//! The port's side of the WP5 probe `tools/oracle/probes/wp5/decode.c`: the
//! same options, the same calls in the same order, and the same printed
//! fields, so that `tests/wp5_decode.rs` can compare both outputs exactly.
//! Include it next to `support/oracle.rs`, `support/wp2.rs` and
//! `support/wp4.rs`.
//!
//! The probe calls `jpeg_calc_output_dimensions` and copies
//! `use_merged_upsample`, which is local to `jdmaster.c`, and the parts of
//! `master_selection` and `prepare_for_output_pass` that set up and start
//! the modules. This file calls the port's `jpeg_calc_output_dimensions`
//! and carries the same copies.

#![allow(dead_code, reason = "each test binary uses part of the support")]

use std::cell::RefCell;
use std::rc::Rc;

use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress, jpeg_read_header};
use jpeg::jdarith::jinit_arith_decoder;
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdcoefct::jinit_d_coef_controller;
use jpeg::jdcolor::jinit_color_deconverter;
use jpeg::jddctmgr::jinit_inverse_dct;
use jpeg::jdhuff::jinit_huff_decoder;
use jpeg::jdmainct::jinit_d_main_controller;
use jpeg::jdmaster::jpeg_calc_output_dimensions;
use jpeg::jdmerge::jinit_merged_upsampler;
use jpeg::jdphuff::jinit_phuff_decoder;
use jpeg::jdpostct::jinit_d_post_controller;
use jpeg::jdsample::jinit_upsampler;
use jpeg::jerror::ErrorExit;
use jpeg::jmorecfg::{JDIMENSION, rgb_pixelsize};
use jpeg::jpegint::J_BUF_MODE;
use jpeg::jpeglib::{
    InputBuffer, J_COLOR_SPACE, JPEG_HEADER_OK, JPEG_SUSPENDED, RowListMut, jpeg_decompress_struct,
    jpeg_source_mgr,
};

use crate::oracle::Fields;
use crate::wp2::{Log, SuspendState, SuspendingSource, print_memory, push, release};
use crate::wp4::{
    FNV_OFFSET, FNV_PRIME, MAX_CALLS, RunLog, new_object, parse_method, prepare_range_limit_table,
    print_error,
};

/// `jdmaster.c`'s `use_merged_upsample` without lossless mode.
pub fn use_merged_upsample(cinfo: &jpeg_decompress_struct<'_>) -> bool {
    use J_COLOR_SPACE as CS;
    if cinfo.do_fancy_upsampling || cinfo.CCIR601_sampling {
        return false;
    }
    let out = cinfo.out_color_space;
    if cinfo.jpeg_color_space != CS::JCS_YCbCr
        || cinfo.num_components != 3
        || ![
            CS::JCS_RGB,
            CS::JCS_RGB565,
            CS::JCS_EXT_RGB,
            CS::JCS_EXT_RGBX,
            CS::JCS_EXT_BGR,
            CS::JCS_EXT_BGRX,
            CS::JCS_EXT_XBGR,
            CS::JCS_EXT_XRGB,
            CS::JCS_EXT_RGBA,
            CS::JCS_EXT_BGRA,
            CS::JCS_EXT_ABGR,
            CS::JCS_EXT_ARGB,
        ]
        .contains(&out)
    {
        return false;
    }
    if (out == CS::JCS_RGB565 && cinfo.out_color_components != 3)
        || (out != CS::JCS_RGB565 && cinfo.out_color_components != rgb_pixelsize[out.0 as usize])
    {
        return false;
    }
    let c = cinfo.comp_info();
    if c[0].h_samp_factor != 2
        || c[1].h_samp_factor != 1
        || c[2].h_samp_factor != 1
        || c[0].v_samp_factor > 2
        || c[1].v_samp_factor != 1
        || c[2].v_samp_factor != 1
    {
        return false;
    }
    let min = cinfo.min_DCT_scaled_size;
    if c[0].DCT_scaled_size != min || c[1].DCT_scaled_size != min || c[2].DCT_scaled_size != min {
        return false;
    }
    true
}

/// The options of `decode.c`.
#[derive(Default)]
struct Options {
    trace: i32,
    suspend: usize,
    rows: usize,
    out: Option<u32>,
    nofancy: bool,
    scale: Option<(u32, u32)>,
    dct: Option<String>,
    nosmooth: bool,
    ccir601: bool,
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
            "-rows" => options.rows = next().parse().unwrap(),
            "-out" => options.out = Some(next().parse::<i32>().unwrap().cast_unsigned()),
            "-dct" => options.dct = Some(next().to_owned()),
            "-scale" => {
                let num = next().parse().unwrap();
                let denom = next().parse().unwrap();
                options.scale = Some((num, denom));
            }
            "-nofancy" => options.nofancy = true,
            "-nosmooth" => options.nosmooth = true,
            "-ccir601" => options.ccir601 = true,
            other => panic!("unknown option {other}"),
        }
    }
    options
}

/// The state of a `decode.c` run beside the object.
struct Run {
    suspend: usize,
    state: Rc<RefCell<SuspendState>>,
    calls: i64,
    stuck: bool,
    process: RunLog,
    consume: RunLog,
    rows: Vec<String>,
}

impl Run {
    /// The probe's `Resume`.
    fn resume(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> bool {
        self.calls += 1;
        if self.suspend == 0 || self.state.borrow().fake_eoi || self.calls > MAX_CALLS {
            self.stuck = true;
            return false;
        }
        release(cinfo, &self.state, self.suspend);
        true
    }

    /// The probe's `ConsumeToEOI`.
    fn consume_to_eoi(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit> {
        while !cinfo.inputctl().pub_.eoi_reached {
            let consume_input = cinfo.inputctl().pub_.consume_input;
            let value = consume_input(cinfo)?;
            self.consume.log(value);
            if value == JPEG_SUSPENDED && !self.resume(cinfo) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// The probe's `PrintFinal`.
    fn print_final(
        &mut self,
        log: &Log,
        cinfo: &jpeg_decompress_struct<'_>,
        error: &Rc<RefCell<(i32, String)>>,
    ) {
        self.process.flush();
        self.consume.flush();
        if !self.rows.is_empty() {
            push(log, "rows", self.rows.join(","));
        }
        push(log, "process", &self.process.text);
        push(log, "consume", &self.consume.text);
        if self.stuck {
            push(log, "stuck", 1);
        }
        if cinfo.master.is_some() && cinfo.inputctl.is_some() {
            push(
                log,
                "final",
                format!(
                    "{},{},{},{},{},{}",
                    cinfo.input_scan_number,
                    cinfo.output_scan_number,
                    cinfo.input_iMCU_row,
                    cinfo.output_iMCU_row,
                    cinfo.output_scanline,
                    i32::from(cinfo.inputctl().pub_.eoi_reached)
                ),
            );
        }
        print_memory(log, cinfo.mem());
        print_error(log, cinfo, error);
    }
}

/// Runs `decode.c`'s sequence on the port for `args` and the input `data`.
#[expect(clippy::too_many_lines, reason = "the probe's main")]
#[expect(clippy::cast_sign_loss, reason = "C's conversions of sizes")]
pub fn run_rust(args: &[&str], data: &[u8]) -> Fields {
    let options = parse(args);
    let log: Log = Rc::new(RefCell::new(Fields::new()));
    let error = Rc::new(RefCell::new((0, String::new())));
    let mut cinfo = new_object(&log, &error);
    let mut run = Run {
        suspend: options.suspend,
        state: Rc::new(RefCell::new(SuspendState {
            size: data.len(),
            ..SuspendState::default()
        })),
        calls: 0,
        stuck: false,
        process: RunLog::default(),
        consume: RunLog::default(),
        rows: Vec::new(),
    };

    let mut body = |run: &mut Run| -> Result<(), ErrorExit> {
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

        if let Some(out) = options.out {
            cinfo.out_color_space = J_COLOR_SPACE(out);
        }
        if let Some(dct) = &options.dct {
            cinfo.dct_method = parse_method(dct);
        }
        if let Some((num, denom)) = options.scale {
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

        /* jdmaster.c's master_selection, without color quantization */
        jpeg_calc_output_dimensions(&mut cinfo)?;
        prepare_range_limit_table(&mut cinfo)?;
        let merged = use_merged_upsample(&cinfo);
        push(
            &log,
            "dims",
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
        push(&log, "merged", i32::from(merged));
        if merged {
            jinit_merged_upsampler(&mut cinfo)?;
        } else {
            jinit_color_deconverter(&mut cinfo)?;
            jinit_upsampler(&mut cinfo)?;
        }
        let enable_2pass_quant = cinfo.enable_2pass_quant;
        jinit_d_post_controller(&mut cinfo, enable_2pass_quant)?;
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
        jinit_d_main_controller(&mut cinfo, false)?;
        let realize_virt_arrays = cinfo.mem().pub_.realize_virt_arrays;
        realize_virt_arrays(&mut cinfo)?;
        let start_input_pass = cinfo.inputctl().pub_.start_input_pass;
        start_input_pass(&mut cinfo)?;
        let mcus_per_row = cinfo.MCUs_per_row;
        let master = &mut cinfo.master_mut().pub_;
        master.first_iMCU_col = 0;
        master.last_iMCU_col = mcus_per_row.wrapping_sub(1);
        master.last_good_iMCU_row = 0;
        for (ci, compptr) in cinfo.comp_info().iter().enumerate() {
            push(
                &log,
                &format!("comp{ci}"),
                format!(
                    "{},{},{},{}",
                    compptr.DCT_scaled_size,
                    compptr.downsampled_width,
                    compptr.downsampled_height,
                    i32::from(compptr.component_needed)
                ),
            );
        }
        push(
            &log,
            "context",
            i32::from(cinfo.upsample().need_context_rows),
        );

        /* jpeg_start_decompress */
        if cinfo.inputctl().pub_.has_multiple_scans {
            run.consume_to_eoi(&mut cinfo)?;
        }
        cinfo.output_scan_number = cinfo.input_scan_number;
        if run.stuck {
            return Ok(());
        }
        /* prepare_for_output_pass */
        let start_pass = cinfo.idct().start_pass;
        start_pass(&mut cinfo)?;
        let start_output_pass = cinfo.coef().start_output_pass;
        start_output_pass(&mut cinfo)?;
        if !merged {
            let start_pass = cinfo.cconvert().start_pass;
            start_pass(&mut cinfo);
        }
        let start_pass = cinfo.upsample().start_pass;
        start_pass(&mut cinfo);
        let start_pass = cinfo.post().start_pass;
        start_pass(&mut cinfo, J_BUF_MODE::JBUF_PASS_THRU)?;
        let start_pass = cinfo.main().start_pass;
        start_pass(&mut cinfo, J_BUF_MODE::JBUF_PASS_THRU)?;

        /* The application's rows */
        let row_stride = (cinfo.output_width * cinfo.output_components as JDIMENSION) as usize;
        let mut scanlines: Vec<Vec<u8>> = vec![vec![0; row_stride]; options.rows];

        /* jpeg_read_scanlines */
        while cinfo.output_scanline < cinfo.output_height {
            let mut row_ctr: JDIMENSION = 0;
            let process_data = cinfo.main().process_data.expect("process_data is NULL");
            let mut rows: Vec<&mut [u8]> = scanlines.iter_mut().map(Vec::as_mut_slice).collect();
            process_data(
                &mut cinfo,
                RowListMut::Separate(&mut rows),
                &mut row_ctr,
                options.rows as JDIMENSION,
            )?;
            cinfo.output_scanline += row_ctr;
            run.process.log(row_ctr.cast_signed());
            for row in scanlines.iter().take(row_ctr as usize) {
                let mut hash = FNV_OFFSET;
                for &sample in row {
                    hash = (hash ^ u64::from(sample)).wrapping_mul(FNV_PRIME);
                }
                run.rows.push(format!("{hash:016x}"));
            }
            if row_ctr == 0 && !run.resume(&mut cinfo) {
                break;
            }
        }
        /* jpeg_finish_decompress */
        if !run.stuck {
            run.consume_to_eoi(&mut cinfo)?;
        }
        Ok(())
    };
    let _ = body(&mut run);
    drop(body);
    run.print_final(&log, &cinfo, &error);
    let _ = jpeg_destroy_decompress(&mut cinfo);
    log.borrow().clone()
}
