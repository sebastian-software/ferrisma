// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// This observer calls only the registered Rust crate; it contains no duplicated
// decoder module or codec algorithm.
use jxl::jxl::box_content_decoder::JxlBoxContentDecoder;
use std::fmt::Write;

fn bytes(h: &str) -> Vec<u8> {
    if h == "-" {
        return Vec::new();
    }
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
        .collect()
}
fn hex(b: &[u8]) -> String {
    if b.is_empty() {
        return "-".into();
    }
    b.iter().map(|b| format!("{b:02x}")).collect()
}
fn state(decoder: &JxlBoxContentDecoder, out: &mut String) {
    // SAFETY: This driver observes source fields only after its StartBox call,
    // with no cursor/field loan or concurrent operation in progress.
    let s = unsafe { decoder.observation() };
    write!(
        out,
        " S{}:{}:{}:{}:{}:{}",
        s.header_done as u8,
        s.brob_decode as u8,
        s.box_until_eof as u8,
        s.remaining,
        s.pos,
        s.brotli.is_some() as u8
    )
    .unwrap();
    if let Some((error, used, finished, pending)) = s.brotli {
        write!(
            out,
            ":{}:{}:{}:{}",
            error, used as u8, finished as u8, pending as u8
        )
        .unwrap();
    }
}
fn number<T: std::str::FromStr>(words: &mut std::str::SplitWhitespace<'_>) -> T {
    words.next().unwrap().parse().ok().unwrap()
}
pub fn observe(request: &str) -> String {
    let mut words = request.split_whitespace();
    assert_eq!(words.next(), Some("box"));
    let mut input = bytes(words.next().unwrap());
    let input_size = input.len();
    if input.is_empty() {
        input.push(0xcc);
    }
    let count: usize = number(&mut words);
    let mut decoder = JxlBoxContentDecoder::new();
    let mut started = false;
    let mut out = String::new();
    for _ in 0..count {
        match words.next().unwrap() {
            "S" => {
                let brob: u32 = number(&mut words);
                let eof: u32 = number(&mut words);
                let n = number(&mut words);
                assert!(brob <= 1 && eof <= 1);
                decoder.start_box(brob != 0, eof != 0, n);
                started = true;
                out.push_str(" B");
                state(&decoder, &mut out);
            }
            "P" => {
                let base: usize = number(&mut words);
                let available = number(&mut words);
                let pos = number(&mut words);
                let output_size: usize = number(&mut words);
                assert!(started && base <= input_size && available <= input_size - base);
                let mut output = vec![0xcc; output_size.max(1)];
                let mut next_out = output.as_mut_ptr();
                let mut available_out = output_size;
                // SAFETY: StartBox initialized source fields; the original
                // initialized input and non-NULL zero-sized backing are live.
                // All cursor/count cells and input/output extents are disjoint,
                // aligned and remain live through the same retained decoder.
                let status = unsafe {
                    decoder.process_raw(
                        input.as_ptr().add(base),
                        available,
                        pos,
                        &mut next_out,
                        &mut available_out,
                    )
                };
                // SAFETY: The source pointer remains within this actual output
                // allocation; the producer pointer checkpoint determines length.
                let produced = unsafe { next_out.offset_from(output.as_ptr()) } as usize;
                write!(
                    out,
                    " R{}:{}:{}:{}:{}",
                    status.0,
                    available_out,
                    produced,
                    hex(&output[..produced]),
                    hex(&output[produced..])
                )
                .unwrap();
                state(&decoder, &mut out);
            }
            _ => panic!("unknown fixture action"),
        }
    }
    assert!(words.next().is_none());
    out
}
#[cfg(not(test))]
fn main() {
    use std::io::{BufRead, Write};
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for request in std::io::stdin().lock().lines() {
        writeln!(out, "{}", observe(&request.unwrap())).unwrap();
    }
}
