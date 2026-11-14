//! Decode timing harness for the JPEG XL port (package libjxl-rs, lib `jxl`),
//! the Rust side of a pair with `jxl_decode_bench.c`: the same call sequence
//! through the port's public decoder API (`jxl::jxl::decode`) and its
//! translated `JxlThreadParallelRunner`.
//!
//! ```text
//! jxl_decode_bench in.jxl workers scalar|default [out.f32]
//! ```
//!
//! `scalar` forces the decoder's scalar path with
//! `hwy::set_decoder_simd_for_test(Some(false))` (the switch the port's
//! scalar-reference tests use); `default` leaves the decoder's own selection
//! (NEON on AArch64, port ADR 0004). The timed region, the pixel format and
//! the output file are those of the C harness. The printed line names the
//! selected path and how many SIMD selections the calling thread made
//! (`simd_selections`, from `hwy::simd_selections_on_this_thread`): with 0
//! workers every task runs on the calling thread, so the count shows which
//! path really ran. `JXL_BENCH_REPEAT=n` (profiling only) repeats the whole
//! timed decode n times in the process and reports the last one.
use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::decode::{self as api, JxlDecoderStatus as Status};
use jxl::jxl::hwy;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use jxl::threads::thread_parallel_runner as tpr;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 4 || args.len() == 5,
        "usage: jxl_decode_bench in.jxl workers scalar|default [out.f32]"
    );
    let data = std::fs::read(&args[1]).expect("read input");
    let workers: usize = args[2].parse().expect("workers");
    let path = match args[3].as_str() {
        "scalar" => {
            hwy::set_decoder_simd_for_test(Some(false));
            "scalar".to_string()
        }
        "default" => hwy::decoder_default_target().map_or("scalar".into(), |t| t.name().into()),
        other => panic!("unknown path {other}"),
    };

    let repeat: usize = std::env::var("JXL_BENCH_REPEAT").map_or(1, |v| v.parse().expect("repeat"));
    let mut info = JxlBasicInfo::default();
    let mut format = JxlPixelFormat {
        num_channels: 0,
        data_type: JxlDataType::FLOAT,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let mut out: Vec<f32> = Vec::new();
    let mut elapsed = 0.0;
    for _ in 0..repeat {
        let t0 = Instant::now();
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe { tpr::jxl_thread_parallel_runner_create(std::ptr::null(), workers) };
        // SAFETY: As above.
        let raw = unsafe { api::jxl_decoder_create(std::ptr::null()) };
        assert!(!runner.is_null() && !raw.is_null());
        // SAFETY: The decoder is live and uniquely owned until destroyed below.
        let dec = unsafe { &mut *raw };
        // SAFETY: The runner is live until after the decoder is destroyed.
        let status = unsafe {
            api::jxl_decoder_set_parallel_runner(dec, Some(tpr::jxl_thread_parallel_runner), runner)
        };
        assert_eq!(status, Status::SUCCESS);
        let events =
            (Status::BASIC_INFO.0 | Status::COLOR_ENCODING.0 | Status::FULL_IMAGE.0) as i32;
        assert_eq!(
            api::jxl_decoder_subscribe_events(dec, events),
            Status::SUCCESS
        );
        // SAFETY: `data` outlives the decoder and is not modified.
        assert_eq!(
            unsafe { api::jxl_decoder_set_input(dec, data.as_ptr(), data.len()) },
            Status::SUCCESS
        );
        api::jxl_decoder_close_input(dec);
        loop {
            // SAFETY: The input and the output buffer stay live and untouched
            // until the decoder is destroyed.
            let s = unsafe { api::jxl_decoder_process_input(dec) };
            if s == Status::BASIC_INFO {
                assert_eq!(
                    api::jxl_decoder_get_basic_info(dec, Some(&mut info)),
                    Status::SUCCESS
                );
                format.num_channels = info.num_color_channels + u32::from(info.alpha_bits != 0);
            } else if s == Status::COLOR_ENCODING || s == Status::FULL_IMAGE {
            } else if s == Status::NEED_IMAGE_OUT_BUFFER {
                let mut bytes = 0;
                assert_eq!(
                    api::jxl_decoder_image_out_buffer_size(dec, Some(&format), Some(&mut bytes)),
                    Status::SUCCESS
                );
                out = vec![0f32; bytes / 4];
                // SAFETY: `out` is writable for `bytes` bytes until the decoder
                // is destroyed and is not touched before that.
                let set = unsafe {
                    api::jxl_decoder_set_image_out_buffer(
                        dec,
                        Some(&format),
                        out.as_mut_ptr().cast(),
                        bytes,
                    )
                };
                assert_eq!(set, Status::SUCCESS);
            } else if s == Status::SUCCESS {
                break;
            } else {
                panic!("decode failed: status {}", s.0);
            }
        }
        // SAFETY: Sole owner; the runner is destroyed after the decoder.
        unsafe {
            api::jxl_decoder_destroy(raw);
            tpr::jxl_thread_parallel_runner_destroy(runner);
        }
        elapsed = t0.elapsed().as_secs_f64() * 1000.0;
    }

    let checksum: f64 = out.iter().step_by(4099).map(|&v| f64::from(v)).sum();
    println!(
        "elapsed_ms={elapsed:.3} width={} height={} channels={} workers={workers} impl=port \
         path={path} simd_selections={} checksum={checksum:.6}",
        info.xsize,
        info.ysize,
        format.num_channels,
        hwy::simd_selections_on_this_thread()
    );
    if let Some(path) = args.get(4) {
        let bytes: Vec<u8> = out.iter().flat_map(|v| v.to_le_bytes()).collect();
        std::fs::write(path, bytes).expect("write output");
    }
}
