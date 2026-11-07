//! WP9 differential test: `WebPMemoryWriter` (`WebPMemoryWriterInit`, `WebPMemoryWrite`
//! and `WebPMemoryWriterClear`, `src/enc/picture_enc.c`), `WebPEncodingSetError` and
//! `WebPReportProgress` (`src/enc/webp_enc.c`). The probe
//! `tools/oracle/probes/wp9/writer_errors.c` runs one sequence of commands in one
//! process, so the writer, the picture and the hook persist across the commands. The
//! test runs the same sequence on the Rust translation.
//!
//! The writes cross the 8192-byte minimum and the doubling, the error setter keeps
//! the oldest error, and the progress hook aborts while it is below its threshold.

#[path = "support/oracle.rs"]
mod oracle;

use std::cell::Cell;
use std::rc::Rc;

use oracle::{Fields, Oracle};
use webp::enc::picture_enc::{
    webp_memory_write, webp_memory_writer_clear, webp_memory_writer_init,
};
use webp::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use webp::webp::encode::{WebPEncodingError, WebPMemoryWriter, WebPPicture, webp_picture_init};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn writer_errors_and_progress_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("writer_errors_and_progress_match_the_c_functions") else {
        return;
    };
    let probe = oracle.probe("wp9", "writer_errors");

    let mut stdin = String::new();
    let mut expected = Fields::new();

    let mut writer = WebPMemoryWriter::default();
    let mut picture = WebPPicture::default();
    let hook_calls = Rc::new(Cell::new(0_i32));
    let threshold = Rc::new(Cell::new(0_i32));

    // The writer starts empty, and the first write allocates the 8192-byte minimum.
    stdin.push_str("writer_init\n");
    webp_memory_writer_init(&mut writer);
    expected = expected
        .with("size", writer.size)
        .with("max_size", writer.max_size)
        .with("mem_null", i32::from(writer.mem.is_empty()));

    let writes: [&[u8]; 6] = [
        &[1, 2, 3],
        &[4, 5],
        &[],
        &[6; 8189],
        &[7; 10],
        &[8, 9, 10, 11],
    ];
    for data in writes {
        let text = if data.is_empty() {
            "-".to_owned()
        } else {
            hex(data)
        };
        stdin.push_str(&format!("write {} {text}\n", data.len()));
        let ret = webp_memory_write(&mut writer, data);
        expected = expected
            .with("ret", i32::from(ret))
            .with("size", writer.size)
            .with("max_size", writer.max_size);
        if !writer.mem.is_empty() {
            expected = expected.hex("mem", &writer.mem[..writer.size]);
        }
    }

    stdin.push_str("writer_clear\n");
    webp_memory_writer_clear(&mut writer);
    expected = expected
        .with("size", writer.size)
        .with("max_size", writer.max_size)
        .with("mem_null", i32::from(writer.mem.is_empty()));

    // The picture's error code keeps the oldest error (ADR 0003).
    stdin.push_str("picture_init\n");
    let _ = webp_picture_init(&mut picture);
    expected = expected.with("error", picture.error_code.0);

    for code in [0_u32, 0, 5, 3, 1] {
        stdin.push_str(&format!("set {code}\n"));
        let held = webp_encoding_set_error(&mut picture, WebPEncodingError(code));
        // The C setter returns 0; the Rust one returns the code the picture holds.
        expected = expected.with("ret", 0).with("error", picture.error_code.0);
        assert_eq!(held, picture.error_code);
    }

    // The hook continues while the percent is below the threshold, and each call
    // counts. The store changes only when the percent does.
    stdin.push_str("hook 50\n");
    threshold.set(50);
    let counter = Rc::clone(&hook_calls);
    let limit = Rc::clone(&threshold);
    picture.progress_hook = Some(Box::new(move |percent| {
        counter.set(counter.get() + 1);
        percent < limit.get()
    }));
    expected = expected.with("hook", 1);

    for (store, percent) in [(0_i32, 0_i32), (0, 20), (20, 20), (20, 60), (60, 70)] {
        stdin.push_str(&format!("progress {store} {percent}\n"));
        hook_calls.set(0);
        let mut value = store;
        let result = webp_report_progress(&mut picture, percent, Some(&mut value));
        expected = expected
            .with("ret", i32::from(result.is_ok()))
            .with("store", value)
            .with("error", picture.error_code.0)
            .with("hook_calls", hook_calls.get());
    }

    stdin.push_str("progress_null 10\n");
    hook_calls.set(0);
    let result = webp_report_progress(&mut picture, 10, None);
    expected = expected
        .with("ret", i32::from(result.is_ok()))
        .with("error", picture.error_code.0)
        .with("hook_calls", hook_calls.get());

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
