//! WP6 differential test: the frame header parsing of `src/dec/vp8_dec.c`
//! (`VP8GetHeaders` with the segment, filter and partition headers,
//! `VP8ParseQuant` of `quant_dec.c` and `VP8ParseProba` of `tree_dec.c`),
//! `VP8GetInfo` and `VP8CheckSignature`, against the C probe
//! `tools/oracle/probes/wp6/headers.c`.
//!
//! The inputs are the `VP8 ` payloads of every lossy test vector, in full, at
//! every size up to 40 bytes, around the end of the first partition and of the
//! partition size table, in normal and incremental mode, and with mutations that
//! change bytes or insert the VP8 signature of the decoder fuzz dictionary.
//! After each call the test compares the result, the status and message, every
//! header field, the quantizers, the probabilities, the bit readers and the io.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use std::collections::BTreeSet;
use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::vp8_dec::{
    VP8Io, vp8_check_signature, vp8_get_headers, vp8_get_info, vp8_init_io, vp8_new,
};
use wp6_support::{
    Rng, compare, find_chunk, lossy_vectors, print_headers, print_io, read_vector, token,
};

/// The VP8 signature of `tests/fuzzer/fuzz.dict`.
const SIGNATURE: [u8; 3] = [0x9d, 0x01, 0x2a];

struct Script {
    commands: String,
    expected: String,
    calls: usize,
    statuses: BTreeSet<u32>,
}

impl Script {
    fn headers_rust(&mut self, data: &[u8], incremental: bool) {
        let mut dec = vp8_new();
        dec.incremental = incremental;
        let mut io = VP8Io::default();
        assert!(vp8_init_io(&mut io));
        let ok = vp8_get_headers(&mut dec, &mut io, data).is_ok();
        writeln!(self.expected, "ok={}", i32::from(ok)).unwrap();
        print_headers(&mut self.expected, &dec);
        print_io(&mut self.expected, &io);
        self.calls += 1;
        self.statuses.insert(dec.status.0);
    }

    fn headers(&mut self, data: &[u8], incremental: bool) {
        writeln!(
            self.commands,
            "headers {} {}",
            token(data),
            i32::from(incremental)
        )
        .unwrap();
        self.headers_rust(data, incremental);
    }

    fn file(&mut self, path: &str, file: &[u8], offset: usize, size: usize, incremental: bool) {
        writeln!(
            self.commands,
            "file {path} {offset} {size} {}",
            i32::from(incremental)
        )
        .unwrap();
        self.headers_rust(&file[offset..offset + size], incremental);
    }

    fn info(&mut self, data: &[u8], chunk_size: usize) {
        writeln!(self.commands, "info {} {chunk_size}", token(data)).unwrap();
        let (ok, width, height) = match vp8_get_info(data, chunk_size) {
            Some((width, height)) => (1, width, height),
            None => (0, -1, -1),
        };
        writeln!(self.expected, "info={ok},{width},{height}").unwrap();
        self.calls += 1;
    }

    fn signature(&mut self, data: &[u8]) {
        writeln!(self.commands, "signature {}", token(data)).unwrap();
        writeln!(
            self.expected,
            "signature={}",
            i32::from(vp8_check_signature(data))
        )
        .unwrap();
        self.calls += 1;
    }
}

/// The sizes at which a payload is cut: every size up to 40, around the end of
/// the first partition, around the partition size table, and a few more.
fn cut_sizes(payload: &[u8], rng: &mut Rng) -> BTreeSet<usize> {
    let len = payload.len();
    let mut sizes: BTreeSet<usize> = (0..=40.min(len)).collect();
    if len >= 3 {
        let bits =
            u32::from(payload[0]) | (u32::from(payload[1]) << 8) | (u32::from(payload[2]) << 16);
        let part0_end = 10 + (bits >> 5) as usize;
        for delta in 0..=8 {
            for size in [
                part0_end.saturating_sub(delta),
                part0_end + delta,
                part0_end + 21 + delta,
            ] {
                if size <= len {
                    sizes.insert(size);
                }
            }
        }
    }
    for _ in 0..6 {
        sizes.insert(rng.below(len as u64 + 1) as usize);
    }
    sizes.insert(len);
    sizes
}

#[test]
fn frame_headers_match_the_c_parser() {
    let Some(oracle) = Oracle::locate("frame_headers_match_the_c_parser") else {
        return;
    };
    let probe = oracle.probe("wp6", "headers");
    let dir = oracle::test_data();
    let mut rng = Rng(0x5750_3648_4452);
    let mut script = Script {
        commands: String::new(),
        expected: String::new(),
        calls: 0,
        statuses: BTreeSet::new(),
    };

    let names = lossy_vectors(&dir);
    assert!(names.len() >= 56, "expected the 56 vp80 vectors and more");
    for name in &names {
        let file = read_vector(&dir, name);
        let (offset, size) = find_chunk(&file, b"VP8 ").expect("a VP8 chunk");
        let size = size.min(file.len() - offset);
        let path = dir.join(name);
        let path = path.to_str().expect("a UTF-8 path");
        let payload = &file[offset..offset + size];
        for cut in cut_sizes(payload, &mut rng) {
            for incremental in [false, true] {
                script.file(path, &file, offset, cut, incremental);
            }
            script.info(&payload[..cut], size);
        }
        // VP8GetInfo against chunk sizes around the first partition length.
        for chunk_size in [0, 1, 9, 10, size / 2, size, size + 1] {
            script.info(payload, chunk_size);
        }
        // Mutations of the frame and picture headers, of the first partition
        // and of the partition sizes, and insertions of the signature.
        for m in 0..24 {
            let mut data = payload[..payload.len().min(400)].to_vec();
            match m % 3 {
                0 => {
                    let pos = rng.below(data.len().min(12) as u64) as usize;
                    data[pos] ^= 1 << rng.below(8);
                }
                1 => {
                    for _ in 0..rng.range(1, 4) {
                        let pos = rng.below(data.len() as u64) as usize;
                        data[pos] = rng.byte();
                    }
                }
                _ => {
                    let pos = rng.below(data.len().min(24) as u64) as usize;
                    let end = (pos + 3).min(data.len());
                    data[pos..end].copy_from_slice(&SIGNATURE[..end - pos]);
                }
            }
            script.headers(&data, m % 2 == 1);
        }
    }
    // Signatures of every length up to four bytes.
    for len in 0..=4 {
        for _ in 0..4 {
            let mut data = rng.bytes(len);
            if rng.chance(1, 2) {
                let n = len.min(3);
                data[..n].copy_from_slice(&SIGNATURE[..n]);
            }
            script.signature(&data);
        }
    }

    let output = probe.run::<&str>(&[], script.commands.as_bytes());
    output.success();
    compare("headers", &output.stdout, &script.expected);
    eprintln!(
        "headers: {} calls over {} vectors, statuses {:?}",
        script.calls,
        names.len(),
        script.statuses
    );
}
