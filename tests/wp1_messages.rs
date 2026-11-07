//! WP1 differential tests of the message codes and strings (`jerror.h`),
//! `format_message`, `emit_message`, `reset_error_mgr`, `jpeg_std_error` and
//! the error and trace macros (`jerror.c`, `jerror.h`).
//!
//! The probe `tools/oracle/probes/wp1/messages.c` runs a script of commands
//! against the C library with an error manager whose `output_message` records
//! the formatted message and whose `error_exit` records that it ran and calls
//! `output_message`, as the port's default `error_exit` does. [`run_rust`]
//! runs the same script on the port with the same overrides; each test
//! compares both outputs exactly.

#[path = "support/oracle.rs"]
mod oracle;

use std::cell::RefCell;
use std::rc::Rc;

use jpeg::jerror::{
    ERREXIT, ERREXIT1, ERREXIT2, ERREXIT3, ERREXIT4, ERREXIT6, ERREXITS, J_MESSAGE_CODE, TRACEMS,
    TRACEMS1, TRACEMS2, TRACEMS3, TRACEMS4, TRACEMS5, TRACEMS8, TRACEMSS, WARNMS, WARNMS1, WARNMS2,
    call_emit_message, call_format_message, call_reset_error_mgr, error_exit, jpeg_error_methods,
    jpeg_std_error, jpeg_std_message_table,
};
use jpeg::jpeglib::{JMSG_LENGTH_MAX, JMSG_STR_PARM_MAX, jpeg_decompress_struct, jpeg_error_mgr};
use oracle::{Fields, Oracle};

/// Every `JMESSAGE` entry of `jerror.h` as the port names it.
const NAMES: &[(&str, J_MESSAGE_CODE)] = &[
    ("JMSG_NOMESSAGE", J_MESSAGE_CODE::JMSG_NOMESSAGE),
    ("JERR_ARITH_NOTIMPL", J_MESSAGE_CODE::JERR_ARITH_NOTIMPL),
    ("JERR_BAD_ALIGN_TYPE", J_MESSAGE_CODE::JERR_BAD_ALIGN_TYPE),
    ("JERR_BAD_ALLOC_CHUNK", J_MESSAGE_CODE::JERR_BAD_ALLOC_CHUNK),
    ("JERR_BAD_BUFFER_MODE", J_MESSAGE_CODE::JERR_BAD_BUFFER_MODE),
    (
        "JERR_BAD_COMPONENT_ID",
        J_MESSAGE_CODE::JERR_BAD_COMPONENT_ID,
    ),
    ("JERR_BAD_DCT_COEF", J_MESSAGE_CODE::JERR_BAD_DCT_COEF),
    ("JERR_BAD_DCTSIZE", J_MESSAGE_CODE::JERR_BAD_DCTSIZE),
    ("JERR_BAD_HUFF_TABLE", J_MESSAGE_CODE::JERR_BAD_HUFF_TABLE),
    (
        "JERR_BAD_IN_COLORSPACE",
        J_MESSAGE_CODE::JERR_BAD_IN_COLORSPACE,
    ),
    (
        "JERR_BAD_J_COLORSPACE",
        J_MESSAGE_CODE::JERR_BAD_J_COLORSPACE,
    ),
    ("JERR_BAD_LENGTH", J_MESSAGE_CODE::JERR_BAD_LENGTH),
    ("JERR_BAD_LIB_VERSION", J_MESSAGE_CODE::JERR_BAD_LIB_VERSION),
    ("JERR_BAD_MCU_SIZE", J_MESSAGE_CODE::JERR_BAD_MCU_SIZE),
    ("JERR_BAD_POOL_ID", J_MESSAGE_CODE::JERR_BAD_POOL_ID),
    ("JERR_BAD_PRECISION", J_MESSAGE_CODE::JERR_BAD_PRECISION),
    ("JERR_BAD_PROGRESSION", J_MESSAGE_CODE::JERR_BAD_PROGRESSION),
    ("JERR_BAD_PROG_SCRIPT", J_MESSAGE_CODE::JERR_BAD_PROG_SCRIPT),
    ("JERR_BAD_SAMPLING", J_MESSAGE_CODE::JERR_BAD_SAMPLING),
    ("JERR_BAD_SCAN_SCRIPT", J_MESSAGE_CODE::JERR_BAD_SCAN_SCRIPT),
    ("JERR_BAD_STATE", J_MESSAGE_CODE::JERR_BAD_STATE),
    ("JERR_BAD_STRUCT_SIZE", J_MESSAGE_CODE::JERR_BAD_STRUCT_SIZE),
    (
        "JERR_BAD_VIRTUAL_ACCESS",
        J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS,
    ),
    ("JERR_BUFFER_SIZE", J_MESSAGE_CODE::JERR_BUFFER_SIZE),
    ("JERR_CANT_SUSPEND", J_MESSAGE_CODE::JERR_CANT_SUSPEND),
    ("JERR_CCIR601_NOTIMPL", J_MESSAGE_CODE::JERR_CCIR601_NOTIMPL),
    ("JERR_COMPONENT_COUNT", J_MESSAGE_CODE::JERR_COMPONENT_COUNT),
    (
        "JERR_CONVERSION_NOTIMPL",
        J_MESSAGE_CODE::JERR_CONVERSION_NOTIMPL,
    ),
    ("JERR_DAC_INDEX", J_MESSAGE_CODE::JERR_DAC_INDEX),
    ("JERR_DAC_VALUE", J_MESSAGE_CODE::JERR_DAC_VALUE),
    ("JERR_DHT_INDEX", J_MESSAGE_CODE::JERR_DHT_INDEX),
    ("JERR_DQT_INDEX", J_MESSAGE_CODE::JERR_DQT_INDEX),
    ("JERR_EMPTY_IMAGE", J_MESSAGE_CODE::JERR_EMPTY_IMAGE),
    ("JERR_EMS_READ", J_MESSAGE_CODE::JERR_EMS_READ),
    ("JERR_EMS_WRITE", J_MESSAGE_CODE::JERR_EMS_WRITE),
    ("JERR_EOI_EXPECTED", J_MESSAGE_CODE::JERR_EOI_EXPECTED),
    ("JERR_FILE_READ", J_MESSAGE_CODE::JERR_FILE_READ),
    ("JERR_FILE_WRITE", J_MESSAGE_CODE::JERR_FILE_WRITE),
    (
        "JERR_FRACT_SAMPLE_NOTIMPL",
        J_MESSAGE_CODE::JERR_FRACT_SAMPLE_NOTIMPL,
    ),
    (
        "JERR_HUFF_CLEN_OVERFLOW",
        J_MESSAGE_CODE::JERR_HUFF_CLEN_OVERFLOW,
    ),
    (
        "JERR_HUFF_MISSING_CODE",
        J_MESSAGE_CODE::JERR_HUFF_MISSING_CODE,
    ),
    ("JERR_IMAGE_TOO_BIG", J_MESSAGE_CODE::JERR_IMAGE_TOO_BIG),
    ("JERR_INPUT_EMPTY", J_MESSAGE_CODE::JERR_INPUT_EMPTY),
    ("JERR_INPUT_EOF", J_MESSAGE_CODE::JERR_INPUT_EOF),
    (
        "JERR_MISMATCHED_QUANT_TABLE",
        J_MESSAGE_CODE::JERR_MISMATCHED_QUANT_TABLE,
    ),
    ("JERR_MISSING_DATA", J_MESSAGE_CODE::JERR_MISSING_DATA),
    ("JERR_MODE_CHANGE", J_MESSAGE_CODE::JERR_MODE_CHANGE),
    ("JERR_NOTIMPL", J_MESSAGE_CODE::JERR_NOTIMPL),
    ("JERR_NOT_COMPILED", J_MESSAGE_CODE::JERR_NOT_COMPILED),
    (
        "JERR_NO_BACKING_STORE",
        J_MESSAGE_CODE::JERR_NO_BACKING_STORE,
    ),
    ("JERR_NO_HUFF_TABLE", J_MESSAGE_CODE::JERR_NO_HUFF_TABLE),
    ("JERR_NO_IMAGE", J_MESSAGE_CODE::JERR_NO_IMAGE),
    ("JERR_NO_QUANT_TABLE", J_MESSAGE_CODE::JERR_NO_QUANT_TABLE),
    ("JERR_NO_SOI", J_MESSAGE_CODE::JERR_NO_SOI),
    ("JERR_OUT_OF_MEMORY", J_MESSAGE_CODE::JERR_OUT_OF_MEMORY),
    (
        "JERR_QUANT_COMPONENTS",
        J_MESSAGE_CODE::JERR_QUANT_COMPONENTS,
    ),
    (
        "JERR_QUANT_FEW_COLORS",
        J_MESSAGE_CODE::JERR_QUANT_FEW_COLORS,
    ),
    (
        "JERR_QUANT_MANY_COLORS",
        J_MESSAGE_CODE::JERR_QUANT_MANY_COLORS,
    ),
    ("JERR_SOF_DUPLICATE", J_MESSAGE_CODE::JERR_SOF_DUPLICATE),
    ("JERR_SOF_NO_SOS", J_MESSAGE_CODE::JERR_SOF_NO_SOS),
    ("JERR_SOF_UNSUPPORTED", J_MESSAGE_CODE::JERR_SOF_UNSUPPORTED),
    ("JERR_SOI_DUPLICATE", J_MESSAGE_CODE::JERR_SOI_DUPLICATE),
    ("JERR_SOS_NO_SOF", J_MESSAGE_CODE::JERR_SOS_NO_SOF),
    ("JERR_TFILE_CREATE", J_MESSAGE_CODE::JERR_TFILE_CREATE),
    ("JERR_TFILE_READ", J_MESSAGE_CODE::JERR_TFILE_READ),
    ("JERR_TFILE_SEEK", J_MESSAGE_CODE::JERR_TFILE_SEEK),
    ("JERR_TFILE_WRITE", J_MESSAGE_CODE::JERR_TFILE_WRITE),
    ("JERR_TOO_LITTLE_DATA", J_MESSAGE_CODE::JERR_TOO_LITTLE_DATA),
    ("JERR_UNKNOWN_MARKER", J_MESSAGE_CODE::JERR_UNKNOWN_MARKER),
    ("JERR_VIRTUAL_BUG", J_MESSAGE_CODE::JERR_VIRTUAL_BUG),
    ("JERR_WIDTH_OVERFLOW", J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW),
    ("JERR_XMS_READ", J_MESSAGE_CODE::JERR_XMS_READ),
    ("JERR_XMS_WRITE", J_MESSAGE_CODE::JERR_XMS_WRITE),
    ("JMSG_COPYRIGHT", J_MESSAGE_CODE::JMSG_COPYRIGHT),
    ("JMSG_VERSION", J_MESSAGE_CODE::JMSG_VERSION),
    ("JTRC_16BIT_TABLES", J_MESSAGE_CODE::JTRC_16BIT_TABLES),
    ("JTRC_ADOBE", J_MESSAGE_CODE::JTRC_ADOBE),
    ("JTRC_APP0", J_MESSAGE_CODE::JTRC_APP0),
    ("JTRC_APP14", J_MESSAGE_CODE::JTRC_APP14),
    ("JTRC_DAC", J_MESSAGE_CODE::JTRC_DAC),
    ("JTRC_DHT", J_MESSAGE_CODE::JTRC_DHT),
    ("JTRC_DQT", J_MESSAGE_CODE::JTRC_DQT),
    ("JTRC_DRI", J_MESSAGE_CODE::JTRC_DRI),
    ("JTRC_EMS_CLOSE", J_MESSAGE_CODE::JTRC_EMS_CLOSE),
    ("JTRC_EMS_OPEN", J_MESSAGE_CODE::JTRC_EMS_OPEN),
    ("JTRC_EOI", J_MESSAGE_CODE::JTRC_EOI),
    ("JTRC_HUFFBITS", J_MESSAGE_CODE::JTRC_HUFFBITS),
    ("JTRC_JFIF", J_MESSAGE_CODE::JTRC_JFIF),
    (
        "JTRC_JFIF_BADTHUMBNAILSIZE",
        J_MESSAGE_CODE::JTRC_JFIF_BADTHUMBNAILSIZE,
    ),
    ("JTRC_JFIF_EXTENSION", J_MESSAGE_CODE::JTRC_JFIF_EXTENSION),
    ("JTRC_JFIF_THUMBNAIL", J_MESSAGE_CODE::JTRC_JFIF_THUMBNAIL),
    ("JTRC_MISC_MARKER", J_MESSAGE_CODE::JTRC_MISC_MARKER),
    ("JTRC_PARMLESS_MARKER", J_MESSAGE_CODE::JTRC_PARMLESS_MARKER),
    ("JTRC_QUANTVALS", J_MESSAGE_CODE::JTRC_QUANTVALS),
    ("JTRC_QUANT_3_NCOLORS", J_MESSAGE_CODE::JTRC_QUANT_3_NCOLORS),
    ("JTRC_QUANT_NCOLORS", J_MESSAGE_CODE::JTRC_QUANT_NCOLORS),
    ("JTRC_QUANT_SELECTED", J_MESSAGE_CODE::JTRC_QUANT_SELECTED),
    ("JTRC_RECOVERY_ACTION", J_MESSAGE_CODE::JTRC_RECOVERY_ACTION),
    ("JTRC_RST", J_MESSAGE_CODE::JTRC_RST),
    ("JTRC_SMOOTH_NOTIMPL", J_MESSAGE_CODE::JTRC_SMOOTH_NOTIMPL),
    ("JTRC_SOF", J_MESSAGE_CODE::JTRC_SOF),
    ("JTRC_SOF_COMPONENT", J_MESSAGE_CODE::JTRC_SOF_COMPONENT),
    ("JTRC_SOI", J_MESSAGE_CODE::JTRC_SOI),
    ("JTRC_SOS", J_MESSAGE_CODE::JTRC_SOS),
    ("JTRC_SOS_COMPONENT", J_MESSAGE_CODE::JTRC_SOS_COMPONENT),
    ("JTRC_SOS_PARAMS", J_MESSAGE_CODE::JTRC_SOS_PARAMS),
    ("JTRC_TFILE_CLOSE", J_MESSAGE_CODE::JTRC_TFILE_CLOSE),
    ("JTRC_TFILE_OPEN", J_MESSAGE_CODE::JTRC_TFILE_OPEN),
    ("JTRC_THUMB_JPEG", J_MESSAGE_CODE::JTRC_THUMB_JPEG),
    ("JTRC_THUMB_PALETTE", J_MESSAGE_CODE::JTRC_THUMB_PALETTE),
    ("JTRC_THUMB_RGB", J_MESSAGE_CODE::JTRC_THUMB_RGB),
    ("JTRC_UNKNOWN_IDS", J_MESSAGE_CODE::JTRC_UNKNOWN_IDS),
    ("JTRC_XMS_CLOSE", J_MESSAGE_CODE::JTRC_XMS_CLOSE),
    ("JTRC_XMS_OPEN", J_MESSAGE_CODE::JTRC_XMS_OPEN),
    ("JWRN_ADOBE_XFORM", J_MESSAGE_CODE::JWRN_ADOBE_XFORM),
    (
        "JWRN_BOGUS_PROGRESSION",
        J_MESSAGE_CODE::JWRN_BOGUS_PROGRESSION,
    ),
    ("JWRN_EXTRANEOUS_DATA", J_MESSAGE_CODE::JWRN_EXTRANEOUS_DATA),
    ("JWRN_HIT_MARKER", J_MESSAGE_CODE::JWRN_HIT_MARKER),
    ("JWRN_HUFF_BAD_CODE", J_MESSAGE_CODE::JWRN_HUFF_BAD_CODE),
    ("JWRN_JFIF_MAJOR", J_MESSAGE_CODE::JWRN_JFIF_MAJOR),
    ("JWRN_JPEG_EOF", J_MESSAGE_CODE::JWRN_JPEG_EOF),
    ("JWRN_MUST_RESYNC", J_MESSAGE_CODE::JWRN_MUST_RESYNC),
    ("JWRN_NOT_SEQUENTIAL", J_MESSAGE_CODE::JWRN_NOT_SEQUENTIAL),
    ("JWRN_TOO_MUCH_DATA", J_MESSAGE_CODE::JWRN_TOO_MUCH_DATA),
    ("JERR_BAD_CROP_SPEC", J_MESSAGE_CODE::JERR_BAD_CROP_SPEC),
    ("JERR_NO_ARITH_TABLE", J_MESSAGE_CODE::JERR_NO_ARITH_TABLE),
    ("JWRN_ARITH_BAD_CODE", J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE),
    ("JWRN_BOGUS_ICC", J_MESSAGE_CODE::JWRN_BOGUS_ICC),
    (
        "JERR_BAD_DROP_SAMPLING",
        J_MESSAGE_CODE::JERR_BAD_DROP_SAMPLING,
    ),
    ("JERR_BAD_RESTART", J_MESSAGE_CODE::JERR_BAD_RESTART),
];

/// The probe's addon table, string for string.
static ADDON_TABLE: [Option<&str>; 12] = [
    Some("Addon %d and %d"),
    Some("%-5d|%5d|%+d|% d|%05d|%.3d|%8.3d|%-8.3d|"),
    Some("%u|%x|%X|%#x|%#X|%o|%#o|%08x|"),
    Some("%c%c|%5c|%-3c|%c|"),
    Some("%hd|%hhd|%hu|%hhu|%hx|%hhx|%%|%i|"),
    Some("%*d|%-*d|%.*d|%0*d|"),
    Some("%#.0o|%.0d|%.0x|%#.3o|%+.0d|% .0d|%#5o|%-#8x|"),
    Some("%s|"),
    Some("%+05d|% 05d|%-05d|%#08o|%.5u|%3.5x|%+i|%%%d"),
    None,
    Some(concat!(
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxx%s"
    )),
    Some(concat!(
        "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
        "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
        "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy%d%d%d%d%d%d%d%d"
    )),
];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn until_nul(buffer: &[u8]) -> &[u8] {
    &buffer[..buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(buffer.len())]
}

/// The probe's `RecordOutput` and `RecordErrorExit`.
struct Recorder(Rc<RefCell<Fields>>);

impl jpeg_error_methods for Recorder {
    fn error_exit(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        self.0
            .borrow_mut()
            .0
            .push(("error_exit".to_owned(), cinfo.err.msg_code.to_string()));
        error_exit(self, cinfo);
    }

    fn output_message(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        let mut buffer = [0x55; JMSG_LENGTH_MAX];
        self.format_message(cinfo, &mut buffer);
        self.0
            .borrow_mut()
            .0
            .push(("output".to_owned(), hex(until_nul(&buffer))));
    }
}

fn install(cinfo: &mut jpeg_decompress_struct<'_>, log: &Rc<RefCell<Fields>>) {
    jpeg_std_error(&mut cinfo.err);
    cinfo.err.methods = Some(Box::new(Recorder(Rc::clone(log))));
}

fn integer(text: &str) -> i32 {
    text.parse()
        .unwrap_or_else(|_| panic!("bad integer {text}"))
}

/// The integer arguments of a command: up to the next word that is not one.
fn integers(script: &[&str], next: &mut usize, max: usize) -> Vec<i32> {
    let mut values = Vec::new();
    while values.len() < max
        && let Some(word) = script.get(*next)
        && let Ok(value) = word.parse()
    {
        values.push(value);
        *next += 1;
    }
    values
}

fn code(value: i32) -> J_MESSAGE_CODE {
    J_MESSAGE_CODE(value.cast_unsigned())
}

/// Runs `script` on the port and prints what `messages.c` prints.
#[expect(clippy::too_many_lines, reason = "one arm per probe command")]
fn run_rust(script: &[&str]) -> Fields {
    let log = Rc::new(RefCell::new(Fields::new()));
    let mut cinfo = jpeg_decompress_struct::new(Box::new(jpeg_error_mgr::default()));
    install(&mut cinfo, &log);
    let push = |key: &str, value: String| log.borrow_mut().0.push((key.to_owned(), value));
    let mut next = 0;
    let mut step = 0;
    while next < script.len() {
        let command = script[next];
        next += 1;
        push("step", format!("{step}:{command}"));
        step += 1;
        match command {
            "table" => {
                push("count", J_MESSAGE_CODE::JMSG_LASTMSGCODE.0.to_string());
                push("last_jpeg_message", cinfo.err.last_jpeg_message.to_string());
                for (name, value) in NAMES {
                    push(&format!("name{}", value.0), (*name).to_owned());
                    let text = jpeg_std_message_table[value.0 as usize].unwrap();
                    push("text", hex(text.as_bytes()));
                }
                let end = J_MESSAGE_CODE::JMSG_LASTMSGCODE.0 as usize;
                push(
                    "table_end_is_null",
                    u8::from(cinfo.err.jpeg_message_table[end].is_none()).to_string(),
                );
            }
            "std" => {
                install(&mut cinfo, &log);
                push("last_jpeg_message", cinfo.err.last_jpeg_message.to_string());
                push(
                    "addon_message_table_is_null",
                    u8::from(cinfo.err.addon_message_table.is_none()).to_string(),
                );
                push(
                    "first_addon_message",
                    cinfo.err.first_addon_message.to_string(),
                );
                push(
                    "last_addon_message",
                    cinfo.err.last_addon_message.to_string(),
                );
            }
            "code" => {
                cinfo.err.msg_code = integer(script[next]);
                next += 1;
            }
            "ints" => {
                for index in 0..8 {
                    cinfo.err.msg_parm.set_i(index, integer(script[next]));
                    next += 1;
                }
            }
            "str" => {
                let text = script[next].as_bytes();
                next += 1;
                let s = cinfo.err.msg_parm.s_mut();
                s.fill(0);
                let length = text.len().min(JMSG_STR_PARM_MAX);
                s[..length].copy_from_slice(&text[..length]);
            }
            "trace" => {
                cinfo.err.trace_level = integer(script[next]);
                next += 1;
            }
            "warnings" => {
                cinfo.err.num_warnings = integer(script[next]).into();
                next += 1;
            }
            "addon" => {
                cinfo.err.addon_message_table = Some(&ADDON_TABLE);
                cinfo.err.first_addon_message = integer(script[next]);
                cinfo.err.last_addon_message = integer(script[next + 1]);
                next += 2;
            }
            "noaddon" => cinfo.err.addon_message_table = None,
            "format" => {
                let mut buffer = [0x55; JMSG_LENGTH_MAX];
                call_format_message(&mut cinfo, &mut buffer);
                push("message", hex(until_nul(&buffer)));
            }
            "emit" => {
                let level = integer(script[next]);
                next += 1;
                call_emit_message(&mut cinfo, level).unwrap();
            }
            "reset" => call_reset_error_mgr(&mut cinfo),
            "errexit" => {
                let value = code(integer(script[next]));
                next += 1;
                let p = integers(script, &mut next, 6);
                let _ = match p.len() {
                    0 => ERREXIT(&mut cinfo, value),
                    1 => ERREXIT1(&mut cinfo, value, p[0]),
                    2 => ERREXIT2(&mut cinfo, value, p[0], p[1]),
                    3 => ERREXIT3(&mut cinfo, value, p[0], p[1], p[2]),
                    4 => ERREXIT4(&mut cinfo, value, p[0], p[1], p[2], p[3]),
                    6 => ERREXIT6(&mut cinfo, value, p[0], p[1], p[2], p[3], p[4], p[5]),
                    count => panic!("no ERREXIT with {count} parameters"),
                };
            }
            "errexits" => {
                let value = code(integer(script[next]));
                let _ = ERREXITS(&mut cinfo, value, script[next + 1].as_bytes());
                next += 2;
            }
            "warnms" => {
                let value = code(integer(script[next]));
                next += 1;
                let p = integers(script, &mut next, 2);
                match p.len() {
                    0 => WARNMS(&mut cinfo, value),
                    1 => WARNMS1(&mut cinfo, value, p[0]),
                    _ => WARNMS2(&mut cinfo, value, p[0], p[1]),
                }
                .unwrap();
            }
            "tracems" => {
                let level = integer(script[next]);
                let value = code(integer(script[next + 1]));
                next += 2;
                let p = integers(script, &mut next, 8);
                let c = &mut cinfo;
                match p.len() {
                    0 => TRACEMS(c, level, value),
                    1 => TRACEMS1(c, level, value, p[0]),
                    2 => TRACEMS2(c, level, value, p[0], p[1]),
                    3 => TRACEMS3(c, level, value, p[0], p[1], p[2]),
                    4 => TRACEMS4(c, level, value, p[0], p[1], p[2], p[3]),
                    5 => TRACEMS5(c, level, value, p[0], p[1], p[2], p[3], p[4]),
                    8 => TRACEMS8(
                        c, level, value, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7],
                    ),
                    count => panic!("no TRACEMS with {count} parameters"),
                }
                .unwrap();
            }
            "tracemss" => {
                let level = integer(script[next]);
                let value = code(integer(script[next + 1]));
                TRACEMSS(&mut cinfo, level, value, script[next + 2].as_bytes()).unwrap();
                next += 3;
            }
            other => panic!("unknown command {other}"),
        }
        push("msg_code", cinfo.err.msg_code.to_string());
        push("msg_parm", hex(cinfo.err.msg_parm.s()));
        push("trace_level", cinfo.err.trace_level.to_string());
        push("num_warnings", cinfo.err.num_warnings.to_string());
    }
    log.borrow().clone()
}

/// Runs `script` on the oracle and on the port and compares the outputs.
fn compare(oracle: &Oracle, script: &[&str]) -> Fields {
    let expected = oracle
        .probe("wp1", "messages")
        .run(script, &[])
        .success()
        .fields();
    let actual = run_rust(script);
    assert_eq!(actual, expected, "script: {script:?}");
    expected
}

fn words(script: &str) -> Vec<&str> {
    script.split_whitespace().collect()
}

#[test]
fn message_codes_and_strings_match_the_oracle() {
    let Some(oracle) = Oracle::locate("message_codes_and_strings_match_the_oracle") else {
        return;
    };
    let fields = compare(&oracle, &["table", "std"]);
    assert_eq!(fields.int("count"), 130);
    assert_eq!(fields.int("last_jpeg_message"), 129);
    assert_eq!(fields.get("table_end_is_null"), Some("1"));
}

#[test]
fn every_message_formats_as_in_the_oracle() {
    let Some(oracle) = Oracle::locate("every_message_formats_as_in_the_oracle") else {
        return;
    };
    // Each code from -2 to 131 with three sets of integer parameters, then
    // with a string parameter, which only the %s messages read.
    for ints in [
        "0 0 0 0 0 0 0 0",
        "1 -2 300 4000 -50000 65 66 2147483647",
        "-2147483648 -1 255 256 65535 65536 10 13",
    ] {
        let mut script = vec!["ints"];
        script.extend(words(ints));
        let codes: Vec<String> = (-2..=131).map(|code| code.to_string()).collect();
        for code in &codes {
            script.extend(["code", code.as_str(), "format"]);
        }
        compare(&oracle, &script);
    }
    let codes: Vec<String> = (-2..=131).map(|code| code.to_string()).collect();
    let mut script = vec!["str", "/tmp/jpeg%d %s 100% \\ \"q\""];
    for code in &codes {
        script.extend(["code", code.as_str(), "format"]);
    }
    compare(&oracle, &script);
}

#[test]
fn addon_messages_and_printf_conversions_match_the_oracle() {
    let Some(oracle) = Oracle::locate("addon_messages_and_printf_conversions_match_the_oracle")
    else {
        return;
    };
    let codes: Vec<String> = (995..=1013).map(|code| code.to_string()).collect();
    for parms in [
        "ints 42 -42 7 0 -7 5 1234 -9",
        "ints 0 0 0 0 0 0 0 0",
        "ints -1 255 65 -128 70000 -70000 300 2147483647",
        "ints 8 -8 5 3 -4 12 6 4",
        "str hello",
        "str",
    ] {
        let mut script = words(parms);
        if script.len() == 1 {
            // An empty string parameter.
            script.push("");
        }
        script.extend(words("addon 1000 1011"));
        for code in &codes {
            script.extend(["code", code.as_str(), "format"]);
        }
        compare(&oracle, &script);
    }
    // The longest string parameter: 80 bytes without a NUL are cut at 79.
    let long = "s".repeat(85);
    let mut script = vec!["str", long.as_str(), "addon", "1000", "1011"];
    for code in ["1007", "1010"] {
        script.extend(["code", code, "format"]);
    }
    compare(&oracle, &script);
    // An addon range that overlaps the library's codes, and none at all.
    compare(
        &oracle,
        &words(
            "ints 5 6 0 0 0 0 0 0 addon 100 111 code 100 format code 129 format code 130 format noaddon code 1000 format",
        ),
    );
}

#[test]
fn error_and_trace_macros_match_the_oracle() {
    let Some(oracle) = Oracle::locate("error_and_trace_macros_match_the_oracle") else {
        return;
    };
    let long = "e".repeat(100);
    let exact = "f".repeat(79);
    // Codes from jerror.h: 51 JERR_NO_IMAGE, 14 JERR_BAD_POOL_ID, 12
    // JERR_BAD_LIB_VERSION, 21 JERR_BAD_STRUCT_SIZE, 16 JERR_BAD_PROGRESSION,
    // 128 JERR_BAD_DROP_SAMPLING, 63 JERR_TFILE_CREATE, 85 JTRC_EOI, 98
    // JTRC_RST, 81 JTRC_DQT, 111 JTRC_UNKNOWN_IDS, 100 JTRC_SOF, 87
    // JTRC_JFIF, 86 JTRC_HUFFBITS, 93 JTRC_QUANTVALS, 76 JTRC_ADOBE, 106 and
    // 107 JTRC_TFILE_CLOSE and _OPEN, 120 JWRN_JPEG_EOF, 116
    // JWRN_EXTRANEOUS_DATA, 119 JWRN_JFIF_MAJOR, 121 JWRN_MUST_RESYNC.
    let scripts: Vec<Vec<&str>> = vec![
        words("errexit 51 errexit 14 9 errexit 12 62 80 errexit 21 632 -1 7 errexit 16 0 63 1 0"),
        words("errexit 128 1 2 1 1 2 89 errexit 5000 1 errexit -3 errexit 0"),
        vec![
            "errexits", "63", "temp.jpg", "errexits", "63", &long, "errexits", "63", &exact,
        ],
        vec![
            "ints", "1", "2", "3", "4", "5", "6", "7", "8", "errexits", "63", "", "errexits", "63",
            "ab",
        ],
        words("tracems 1 85 tracems 1 98 3 tracems 1 81 1 8 tracems 1 111 1 2 3"),
        words(
            "trace 1 tracems 1 85 tracems 2 98 3 tracems 1 100 192 227 149 3 tracems 1 87 1 2 72 72 0",
        ),
        words(
            "trace 3 tracems 2 76 100 0 0 1 tracems 3 86 1 2 3 4 5 6 7 8 tracems 3 93 1 16 255 4096 0 9 10 99999",
        ),
        words("trace 1 tracems 1 87 1 1 300 72 2 tracems 0 98 7"),
        vec![
            "trace", "1", "tracemss", "1", "107", "/tmp/x", "tracemss", "2", "106", "/tmp/y",
        ],
        vec![
            "tracemss", "0", "107", &long, "tracemss", "0", "106", &exact,
        ],
        words("warnms 120 warnms 120 warnms 116 64 217 warnms 119 3 1 warnings 0 warnms 121 208 3"),
        words("trace 3 warnms 120 warnms 121 216 1 warnms 116 4 255 warnings 7 trace 2 warnms 120"),
        words("warnms 120 reset trace 5 errexit 49 reset warnms 120"),
        words("emit -1 emit 0 emit 1 trace 1 emit 1 emit 2 trace -1 emit 0 emit -1 emit -2"),
        words("code 98 ints 7 0 0 0 0 0 0 0 trace 3 emit 3 emit 4 emit -5 std emit -1"),
    ];
    for script in &scripts {
        compare(&oracle, script);
    }
}
