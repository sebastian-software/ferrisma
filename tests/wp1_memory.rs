//! WP1 differential tests of the memory manager (`jmemmgr.c`, `jmemnobs.c`)
//! and of `jpeg_destroy` and the table allocators (`jcomapi.c`).
//!
//! The probe `tools/oracle/probes/wp1/memory.c` runs a script of memory
//! manager operations against the C library and prints the bookkeeping after
//! each one: `total_space_allocated`, every small and large pool record,
//! `last_rowsperchunk`, the limits and every virtual array, and the error a
//! failing operation reports with the state it leaves. [`run_rust`] runs the
//! same script on the port and prints the same fields; each test compares the
//! two outputs exactly.

#[path = "support/oracle.rs"]
mod oracle;

use std::cell::RefCell;
use std::rc::Rc;

use jpeg::jcomapi::{jpeg_alloc_huff_table, jpeg_alloc_quant_table, jpeg_destroy};
use jpeg::jerror::{ErrorExit, J_MESSAGE_CODE, emit_message, jpeg_error_methods, jpeg_std_error};
use jpeg::jmemmgr::{
    SIZEOF_jvirt_barray_control, SIZEOF_my_memory_mgr, alloc_large_vec, alloc_small_box,
    alloc_small_vec, jinit_memory_mgr,
};
use jpeg::jpeglib::{
    JMSG_LENGTH_MAX, JPOOL_IMAGE, jpeg_decompress_struct, jpeg_error_mgr, jvirt_barray_ptr,
};
use oracle::{Fields, Oracle};

/// The probe's `ProbeErrorMgr`: records the error code and message instead
/// of printing, and every warning code before the standard `emit_message`.
#[derive(Default)]
struct Record {
    error_code: i32,
    error_message: String,
    warnings: Vec<i32>,
}

struct Recorder(Rc<RefCell<Record>>);

impl jpeg_error_methods for Recorder {
    fn error_exit(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        let mut buffer = [0; JMSG_LENGTH_MAX];
        self.format_message(cinfo, &mut buffer);
        let length = buffer.iter().position(|&byte| byte == 0).unwrap();
        let mut record = self.0.borrow_mut();
        record.error_code = cinfo.err.msg_code;
        record.error_message = String::from_utf8_lossy(&buffer[..length]).into_owned();
    }

    fn emit_message(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        msg_level: i32,
    ) -> Result<(), ErrorExit> {
        if msg_level < 0 {
            self.0.borrow_mut().warnings.push(cinfo.err.msg_code);
        }
        emit_message(self, cinfo, msg_level)
    }
}

fn new_object(record: &Rc<RefCell<Record>>) -> jpeg_decompress_struct<'static> {
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    err.methods = Some(Box::new(Recorder(Rc::clone(record))));
    let mut cinfo = jpeg_decompress_struct::new(err);
    cinfo.data_precision = 8;
    cinfo
}

fn pools(records: impl Iterator<Item = (usize, usize)>) -> String {
    records
        .map(|(used, left)| format!("{used}/{left}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn print_state(fields: &mut Fields, cinfo: &jpeg_decompress_struct<'_>) {
    let push = |fields: &mut Fields, key: &str, value: String| {
        fields.0.push((key.to_owned(), value));
    };
    push(fields, "mem", u8::from(cinfo.mem.is_some()).to_string());
    push(fields, "global_state", cinfo.global_state.to_string());
    let Some(mem) = cinfo.mem.as_deref() else {
        return;
    };
    push(fields, "total", mem.total_space_allocated.to_string());
    for (key, list) in [("small0", 0), ("small1", 1)] {
        let value = pools(
            mem.small_list[list]
                .iter()
                .map(|hdr| (hdr.bytes_used, hdr.bytes_left)),
        );
        push(fields, key, value);
    }
    for (key, list) in [("large0", 0), ("large1", 1)] {
        let value = pools(
            mem.large_list[list]
                .iter()
                .map(|hdr| (hdr.bytes_used, hdr.bytes_left)),
        );
        push(fields, key, value);
    }
    push(
        fields,
        "last_rowsperchunk",
        mem.last_rowsperchunk.to_string(),
    );
    push(
        fields,
        "max_memory_to_use",
        mem.pub_.max_memory_to_use.to_string(),
    );
    push(
        fields,
        "max_alloc_chunk",
        mem.pub_.max_alloc_chunk.to_string(),
    );
    let arrays: Vec<String> = mem
        .virt_barray_list
        .iter()
        .map(|ptr| {
            let mut text = format!(
                "{}/{}/{}/{}",
                ptr.rows_in_array,
                ptr.blocksperrow,
                ptr.maxaccess,
                u8::from(ptr.pre_zero)
            );
            if ptr.mem_buffer.is_some() {
                text += &format!(
                    "/{}/{}/{}/{}/{}",
                    ptr.rows_in_mem,
                    ptr.rowsperchunk,
                    ptr.cur_start_row,
                    ptr.first_undef_row,
                    u8::from(ptr.dirty)
                );
            }
            text
        })
        .collect();
    push(fields, "virt", arrays.join(";"));
}

fn integer(script: &[&str], next: &mut usize) -> i64 {
    let value = script[*next]
        .parse()
        .unwrap_or_else(|_| panic!("bad integer {}", script[*next]));
    *next += 1;
    value
}

/// Runs one command of the script on the port; `Err` after an error exit.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the probe's conversions of its long arguments"
)]
#[expect(clippy::too_many_lines, reason = "one arm per probe command")]
fn command(
    cinfo: &mut jpeg_decompress_struct<'_>,
    script: &[&str],
    next: &mut usize,
    fields: &mut Fields,
) -> Result<(), ErrorExit> {
    let name = script[*next];
    *next += 1;
    let push = |fields: &mut Fields, key: &str, value: String| {
        fields.0.push((key.to_owned(), value));
    };
    match name {
        "precision" => cinfo.data_precision = integer(script, next) as i32,
        "init" => {
            jinit_memory_mgr(cinfo)?;
            push(
                fields,
                "sizeof_mem",
                cinfo.mem().total_space_allocated.to_string(),
            );
        }
        "maxmem" => cinfo.mem_mut().pub_.max_memory_to_use = integer(script, next),
        "small" => {
            let pool = integer(script, next) as i32;
            let size = integer(script, next) as usize;
            let alloc_small = cinfo.mem().pub_.alloc_small;
            alloc_small(cinfo, pool, size, &mut || true)?;
        }
        "large" => {
            let pool = integer(script, next) as i32;
            let size = integer(script, next) as usize;
            let alloc_large = cinfo.mem().pub_.alloc_large;
            alloc_large(cinfo, pool, size, &mut || true)?;
        }
        "sarray" => {
            let pool = integer(script, next) as i32;
            let width = integer(script, next) as u32;
            let rows = integer(script, next) as u32;
            let alloc_sarray = cinfo.mem().pub_.alloc_sarray;
            let array = alloc_sarray(cinfo, pool, width, rows)?;
            // The distance of rows 0 and 1 when one chunk holds both: C's
            // chunks are separate allocations.
            let stride = if rows > 1 && cinfo.mem().last_rowsperchunk > 1 {
                (array.rows[1] - array.rows[0]).to_string()
            } else {
                "-1".to_owned()
            };
            push(fields, "stride", stride);
        }
        "barray" => {
            let pool = integer(script, next) as i32;
            let width = integer(script, next) as u32;
            let rows = integer(script, next) as u32;
            let alloc_barray = cinfo.mem().pub_.alloc_barray;
            alloc_barray(cinfo, pool, width, rows)?;
        }
        "vbarray" => {
            let pool = integer(script, next) as i32;
            let pre_zero = integer(script, next) != 0;
            let width = integer(script, next) as u32;
            let rows = integer(script, next) as u32;
            let maxaccess = integer(script, next) as u32;
            let request_virt_barray = cinfo.mem().pub_.request_virt_barray;
            request_virt_barray(cinfo, pool, pre_zero, width, rows, maxaccess)?;
        }
        "realize" => {
            let realize_virt_arrays = cinfo.mem().pub_.realize_virt_arrays;
            realize_virt_arrays(cinfo)?;
        }
        "access" => {
            let array = jvirt_barray_ptr(integer(script, next) as usize);
            let start = integer(script, next) as u32;
            let rows = integer(script, next) as u32;
            let writable = integer(script, next) != 0;
            let access_virt_barray = cinfo.mem().pub_.access_virt_barray;
            let row = access_virt_barray(cinfo, array, start, rows, writable)?;
            push(fields, "access_row", row.to_string());
        }
        "poke" => {
            let array = integer(script, next) as usize;
            let row = integer(script, next) as usize;
            let block = integer(script, next) as usize;
            let coef = integer(script, next) as usize;
            let value = integer(script, next) as i16;
            let buffer = cinfo.mem_mut().virt_barray_list[array]
                .mem_buffer
                .as_mut()
                .expect("realized");
            buffer.row_mut(row)[block][coef] = value;
        }
        "peek" => {
            let array = integer(script, next) as usize;
            let row = integer(script, next) as usize;
            let block = integer(script, next) as usize;
            let coef = integer(script, next) as usize;
            let buffer = cinfo.mem().virt_barray_list[array]
                .mem_buffer
                .as_ref()
                .expect("realized");
            push(fields, "peek", buffer.row(row)[block][coef].to_string());
        }
        "free" => {
            let pool = integer(script, next) as i32;
            let free_pool = cinfo.mem().pub_.free_pool;
            free_pool(cinfo, pool)?;
        }
        "quant" => {
            let tbl = jpeg_alloc_quant_table(cinfo)?;
            push(fields, "sent_table", u8::from(tbl.sent_table).to_string());
        }
        "huff" => {
            let tbl = jpeg_alloc_huff_table(cinfo)?;
            push(fields, "sent_table", u8::from(tbl.sent_table).to_string());
        }
        "destroy" => jpeg_destroy(cinfo)?,
        other => panic!("unknown command {other}"),
    }
    Ok(())
}

/// Runs `script` on the port and prints what `memory.c` prints.
fn run_rust(script: &[&str]) -> Fields {
    let record = Rc::new(RefCell::new(Record::default()));
    let mut cinfo = new_object(&record);
    let mut fields = Fields::new()
        .with("sizeof_mem_copy", SIZEOF_my_memory_mgr)
        .with("sizeof_virt_copy", SIZEOF_jvirt_barray_control);
    let mut next = 0;
    let mut step = 0;
    while next < script.len() {
        let start = next;
        fields
            .0
            .push(("step".to_owned(), format!("{step}:{}", script[next])));
        if command(&mut cinfo, script, &mut next, &mut fields).is_err() {
            // realize_virt_arrays sets rows_in_mem of the array it fails
            // on, except for case 11, which it reports before it sets any.
            let case11 = record.borrow().error_code
                == J_MESSAGE_CODE::JERR_OUT_OF_MEMORY.0.cast_signed()
                && cinfo.err.msg_parm.i(0) == 11;
            if let Some(mem) = cinfo.mem.as_deref()
                && !case11
                && script[start] == "realize"
                && let Some(array) = mem
                    .virt_barray_list
                    .iter()
                    .rev()
                    .find(|array| array.mem_buffer.is_none())
            {
                fields.0.push((
                    "pending_rows_in_mem".to_owned(),
                    array.rows_in_mem.to_string(),
                ));
            }
            break;
        }
        print_state(&mut fields, &cinfo);
        step += 1;
    }
    if record.borrow().error_code != 0 {
        print_state(&mut fields, &cinfo);
    }
    let record = record.borrow();
    fields = fields.with("error", record.error_code);
    if record.error_code != 0 {
        fields = fields.with("error_message", &record.error_message);
    }
    let warnings: Vec<String> = record.warnings.iter().map(i32::to_string).collect();
    fields
        .with("num_warnings", cinfo.err.num_warnings)
        .with("warnings", warnings.join(","))
}

/// Runs `script` on the oracle and on the port and compares the outputs.
fn compare(oracle: &Oracle, script: &str) -> Fields {
    let words: Vec<&str> = script.split_whitespace().collect();
    let expected = oracle
        .probe("wp1", "memory")
        .run(&words, &[])
        .success()
        .fields();
    let actual = run_rust(&words);
    assert_eq!(actual, expected, "script: {script}");
    expected
}

fn compare_all(test: &str, scripts: &[&str]) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    for script in scripts {
        compare(&oracle, script);
    }
}

#[test]
fn small_and_large_pools_match_the_oracle() {
    compare_all(
        "small_and_large_pools_match_the_oracle",
        &[
            // First PERMANENT pool with 1600 bytes of slop, then a request
            // that does not fit and opens a pool without slop; first IMAGE
            // pool with 16000, later ones with 5000.
            "init small 0 100 small 0 1 small 0 1500 small 0 2000 small 0 8 \
             small 1 10 small 1 15990 small 1 7 small 1 16000 small 1 4999 small 1 1 \
             large 0 1000 large 1 5000 large 1 0 large 1 1 small 1 0",
            "init large 1 300 small 1 200 free 1 small 1 30 large 1 30 free 0 free 1",
            "init quant huff quant small 0 1500 huff destroy",
            "init small 1 100 destroy",
        ],
    );
}

#[test]
fn allocation_limits_match_the_oracle() {
    compare_all(
        "allocation_limits_match_the_oracle",
        &[
            // alloc_small: case 7 above MAX_ALLOC_CHUNK, case 1 when the
            // rounded request and header exceed it, then the pool check.
            "init small 1 1000000001",
            "init small 1 1000000000",
            "init small 1 999999969",
            "init small 1 -1",
            "init small 2 10",
            "init small -1 10",
            "init small 1 999999960 small 1 999999961",
            // alloc_large: cases 8 and 3, then the pool check.
            "init large 1 1000000001",
            "init large 1 999999969",
            "init large 0 -8",
            "init large 2 1",
            "init large -7 1",
            // alloc_sarray: case 9, JERR_WIDTH_OVERFLOW, the pool check of
            // the row list, row strides of 16 samples.
            "init sarray 1 0 10",
            "init sarray 1 1000000001 1",
            "init sarray 1 999999970 1",
            "init sarray 1 999999984 0",
            "init sarray 3 10 2",
            "init sarray 1 1 1 sarray 1 17 3 sarray 1 100 7 sarray 0 5 0 sarray 1 4096 2",
            // alloc_barray: case 6, JERR_WIDTH_OVERFLOW, the pool check.
            "init barray 1 0 5",
            "init barray 1 7812500 1",
            "init barray 1 7812499 0",
            "init barray 9 1 1",
            "init barray 1 3 5 barray 1 100 0 barray 0 1 1",
            // The memory manager's own limits.
            "init maxmem 12345 maxmem -1",
        ],
    );
}

#[test]
fn row_chunking_matches_the_oracle() {
    // Rows of 600 000 000 samples: one row per MAX_ALLOC_CHUNK request, so
    // two chunks of large objects.
    compare_all(
        "row_chunking_matches_the_oracle",
        &["init sarray 1 600000000 2"],
    );
}

#[test]
fn virtual_arrays_match_the_oracle() {
    compare_all(
        "virtual_arrays_match_the_oracle",
        &[
            // Realize, write, read back, continue writing; reader ahead of
            // the written rows with pre-zeroing.
            "init vbarray 1 1 10 20 4 vbarray 1 0 3 5 5 realize \
             access 0 0 4 1 poke 0 1 2 3 77 access 0 0 4 0 peek 0 1 2 3 \
             access 0 4 4 1 access 0 12 4 0 access 1 0 5 1 access 1 0 5 0 realize",
            // Pre-zeroing clears only rows that become defined.
            "init vbarray 1 1 2 3 2 realize access 0 0 1 1 poke 0 1 0 0 9 \
             poke 0 0 0 0 5 access 0 1 1 1 peek 0 1 0 0 peek 0 0 0 0",
            // A writer that skips rows.
            "init vbarray 1 1 10 20 4 realize access 0 0 4 1 access 0 12 4 1",
            // A reader of undefined rows without pre-zeroing.
            "init vbarray 1 0 3 5 5 realize access 0 0 5 0",
            // More rows than maxaccess, past the end, before realizing.
            "init vbarray 1 1 3 5 2 realize access 0 0 3 1",
            "init vbarray 1 1 3 5 5 realize access 0 3 3 1",
            "init vbarray 1 1 3 5 5 access 0 0 1 1",
            // Only the image pool holds virtual arrays.
            "init vbarray 0 1 3 5 5",
            // An empty array and a release of the image pool.
            "init vbarray 1 1 4 0 1 vbarray 1 0 2 3 3 realize free 1 realize \
             vbarray 1 1 1 1 1 realize access 0 0 1 1",
            // A limit that leaves no room: the newest array would need a
            // backing store.
            "init maxmem 1000 vbarray 1 1 10 100 4 realize",
            // A limit that fits everything.
            "init maxmem 200000 vbarray 1 1 10 100 4 realize",
            // The newest array fits the minimum heights, the older one not.
            "init maxmem 30000 vbarray 1 1 10 100 4 vbarray 1 1 10 8 4 realize",
            // Width overflow while realizing.
            "init vbarray 1 1 7812500 2 1 realize",
            // Case 11: two arrays of 2^63 bytes each overflow size_t.
            "init vbarray 1 1 33554432 2147483648 1 vbarray 1 1 33554432 2147483648 1 realize",
        ],
    );
}

#[test]
fn data_precision_above_8_is_out_of_scope() {
    // Upstream allocates 12- and 16-bit sample arrays; the port translates
    // only 8-bit samples and reports JERR_BAD_PRECISION there (ADR 0005), so
    // these scripts are not compared. The checks before the precision branch
    // (row rounding to 2 * ALIGN_SIZE / sample_size) still run.
    let Some(oracle) = Oracle::locate("data_precision_above_8_is_out_of_scope") else {
        return;
    };
    for precision in ["12", "16"] {
        let script = ["precision", precision, "init", "sarray", "1", "9", "2"];
        let expected = oracle
            .probe("wp1", "memory")
            .run(&script, &[])
            .success()
            .fields();
        assert_eq!(expected.int("error"), 0, "{expected:?}");
        // 9 samples of 2 bytes rounded up to 2 * ALIGN_SIZE / 2 = 8 samples:
        // 16 samples, 32 bytes apart.
        assert_eq!(expected.get("stride"), Some("32"), "{expected:?}");
        let actual = run_rust(&script);
        assert_eq!(
            actual.int("error"),
            i64::from(J_MESSAGE_CODE::JERR_BAD_PRECISION.0)
        );
        assert_eq!(
            actual.get("error_message"),
            Some(format!("Unsupported JPEG data precision {precision}").as_str())
        );
        assert_eq!(
            actual.get("last_rowsperchunk"),
            expected.get("last_rowsperchunk")
        );
    }
}

const CHILD: &str = "LIBJPEG_RS_WP1_JPEGMEM_CHILD";

/// Not a test of its own: `jpegmem_matches_the_oracle` runs this test
/// binary again with `JPEGMEM` set, because a test cannot change its own
/// environment without `unsafe`. Prints the port's state after `init`.
#[test]
fn jpegmem_child() {
    if std::env::var_os(CHILD).is_none() {
        return;
    }
    for (key, value) in run_rust(&["init"]).0 {
        println!("@@{key}={value}");
    }
}

#[test]
fn jpegmem_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("jpegmem_matches_the_oracle") else {
        return;
    };
    let values = [
        None,
        Some(""),
        Some("0"),
        Some("123"),
        Some("12m"),
        Some("7M"),
        Some("  42k"),
        Some("\t-3x"),
        Some("+5"),
        Some("-"),
        Some("abc"),
        Some("   "),
        Some("99999999999m"),
        Some("1e3"),
        Some("0x10"),
        // GETENV_S copies at most 29 characters: the 30-character value is
        // ignored. (A value that saturates `long` would overflow when it is
        // multiplied by 1000, which is undefined behavior in C.)
        Some("00000000000000000000000000042"),
        Some("000000000000000000000000000042"),
    ];
    for value in values {
        let expected = oracle
            .probe("wp1", "memory")
            .run_env(&["init"], &[], &[("JPEGMEM", value)])
            .success()
            .fields();
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--exact",
                "jpegmem_child",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(CHILD, "1");
        match value {
            Some(value) => child.env("JPEGMEM", value),
            None => child.env_remove("JPEGMEM"),
        };
        let output = child.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        let mut actual = Fields::new();
        for line in text.lines() {
            // libtest may print the test name on the same line.
            if let Some(start) = line.find("@@") {
                let (key, value) = line[start + 2..].split_once('=').unwrap();
                actual.0.push((key.to_owned(), value.to_owned()));
            }
        }
        assert_eq!(actual, expected, "JPEGMEM={value:?}");
    }
}

#[test]
fn typed_allocations_account_like_untyped_ones() {
    // The typed forms of alloc_small and alloc_large are Rust glue around the
    // method slots: they must account exactly as the slot does for the same
    // C size, and hand back the caller's storage.
    let record = Rc::new(RefCell::new(Record::default()));
    let mut typed = new_object(&record);
    let mut untyped = new_object(&record);
    jinit_memory_mgr(&mut typed).unwrap();
    jinit_memory_mgr(&mut untyped).unwrap();

    let boxed = alloc_small_box(&mut typed, JPOOL_IMAGE, 24, [7u64; 3]).unwrap();
    let small = alloc_small_vec(&mut typed, JPOOL_IMAGE, 40, 10, 9i32).unwrap();
    let large = alloc_large_vec(&mut typed, JPOOL_IMAGE, 300, 300, 1u8).unwrap();
    for (size, large_object) in [(24, false), (40, false), (300, true)] {
        let method = if large_object {
            untyped.mem().pub_.alloc_large
        } else {
            untyped.mem().pub_.alloc_small
        };
        method(&mut untyped, JPOOL_IMAGE, size, &mut || true).unwrap();
    }
    assert_eq!(*boxed, [7; 3]);
    assert_eq!(small, vec![9; 10]);
    assert_eq!(large, vec![1; 300]);
    let (typed, untyped) = (typed.mem(), untyped.mem());
    assert_eq!(typed.total_space_allocated, untyped.total_space_allocated);
    assert_eq!(typed.small_list, untyped.small_list);
    assert_eq!(typed.large_list, untyped.large_list);
}

#[test]
fn refused_reservations_report_the_get_failure_codes() {
    // A refused reservation stands for jpeg_get_small or jpeg_get_large
    // returning NULL, which the C oracle cannot be made to do: the small pool
    // halves its slop down to MIN_SLOP and reports case 2, the large pool
    // reports case 4, and neither accounts for the request.
    let record = Rc::new(RefCell::new(Record::default()));
    let mut cinfo = new_object(&record);
    jinit_memory_mgr(&mut cinfo).unwrap();
    let total = cinfo.mem().total_space_allocated;
    let mut attempts = 0;
    let alloc_small = cinfo.mem().pub_.alloc_small;
    assert!(
        alloc_small(&mut cinfo, JPOOL_IMAGE, 100, &mut || {
            attempts += 1;
            false
        })
        .is_err()
    );
    // Slop 16000, then halved for each retry down to 62; 31 is below
    // MIN_SLOP and gives up without another attempt.
    assert_eq!(attempts, 9);
    assert_eq!(
        record.borrow().error_code,
        J_MESSAGE_CODE::JERR_OUT_OF_MEMORY.0 as i32
    );
    assert_eq!(
        record.borrow().error_message,
        "Insufficient memory (case 2)"
    );
    assert_eq!(cinfo.mem().total_space_allocated, total);
    assert!(cinfo.mem().small_list[1].is_empty());

    let alloc_large = cinfo.mem().pub_.alloc_large;
    assert!(alloc_large(&mut cinfo, JPOOL_IMAGE, 100, &mut || false).is_err());
    assert_eq!(
        record.borrow().error_message,
        "Insufficient memory (case 4)"
    );
    assert_eq!(cinfo.mem().total_space_allocated, total);
    assert!(cinfo.mem().large_list[1].is_empty());
}

#[test]
fn destroying_releases_the_pool_fields() {
    // ADR 0005, "Pools": the tables allocated from the permanent pool are
    // dropped with it, and the object stays usable for jpeg_destroy again.
    let record = Rc::new(RefCell::new(Record::default()));
    let mut cinfo = new_object(&record);
    jinit_memory_mgr(&mut cinfo).unwrap();
    cinfo.quant_tbl_ptrs[0] = Some(jpeg_alloc_quant_table(&mut cinfo).unwrap());
    cinfo.dc_huff_tbl_ptrs[1] = Some(jpeg_alloc_huff_table(&mut cinfo).unwrap());
    cinfo.ac_huff_tbl_ptrs[3] = Some(jpeg_alloc_huff_table(&mut cinfo).unwrap());
    cinfo.global_state = 202;

    let free_pool = cinfo.mem().pub_.free_pool;
    free_pool(&mut cinfo, JPOOL_IMAGE).unwrap();
    assert!(cinfo.quant_tbl_ptrs[0].is_some());

    jpeg_destroy(&mut cinfo).unwrap();
    assert!(cinfo.mem.is_none());
    assert_eq!(cinfo.global_state, 0);
    assert!(cinfo.quant_tbl_ptrs.iter().all(Option::is_none));
    assert!(cinfo.dc_huff_tbl_ptrs.iter().all(Option::is_none));
    assert!(cinfo.ac_huff_tbl_ptrs.iter().all(Option::is_none));
    jpeg_destroy(&mut cinfo).unwrap();
}
