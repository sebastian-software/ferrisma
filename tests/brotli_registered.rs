// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Public crate wiring must execute the same source captures as the independently
// compiled streaming packet. No duplicated decoder modules occur in this test.
use jxl::brotli::compound_dictionary::CompoundDictionaryBindings;
use jxl::brotli::instance::DecoderInstance;

fn bytes(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn registered_decoder_matches_original_stream_cursors_statuses_and_pixels() {
    let mut cases = 0;
    for row in include_str!("../tools/captures/brotli_stream.tsv").lines() {
        let mut fields = row.splitn(3, '\t');
        let _group = fields.next().unwrap();
        let request = fields.next().unwrap();
        let expected = fields.next().unwrap();
        if !request.starts_with("state ") {
            continue;
        }
        let mut args = request.split_whitespace().skip(1);
        let input_chunk: usize = args.next().unwrap().parse().unwrap();
        let output_chunk: usize = args.next().unwrap().parse().unwrap();
        let disable = args.next().unwrap().parse().unwrap();
        let large = args.next().unwrap().parse().unwrap();
        let input = bytes(args.next().unwrap());
        let mut decoder = DecoderInstance::new().unwrap();
        let bindings = CompoundDictionaryBindings::default();
        let mut records = expected.split_whitespace();
        assert_eq!(records.next(), Some("P11"));
        assert!(decoder.set_parameter(0, disable));
        assert!(decoder.set_parameter(1, large));
        let mut offset = 0;
        let mut total = 999;
        let mut terminal = false;
        for record in records.filter(|r| r.starts_with('R')) {
            assert!(!terminal, "unexpected record after terminal source status");
            let values: Vec<_> = record[1..].split(':').collect();
            assert_eq!(values.len(), 10);
            let wanted: Vec<usize> = values[..9].iter().map(|v| v.parse().unwrap()).collect();
            let mut available_in = input_chunk.min(input.len() - offset);
            let mut available_out = output_chunk;
            // SAFETY: offset is the retained source-consumed position within
            // this same live initialized caller allocation, including one-past.
            let start = unsafe { input.as_ptr().add(offset) };
            let mut next_in = start;
            let mut output = vec![0xcc; output_chunk];
            let mut next_out = output.as_mut_ptr();
            // SAFETY: All cursor cells are initialized, live and disjoint;
            // caller input/output extents are valid, no dictionaries/callbacks
            // are installed, and the same decoder allocation is retained.
            let result = unsafe {
                decoder.decompress_stream_raw(
                    &bindings,
                    &mut available_in,
                    &mut next_in,
                    &mut available_out,
                    &mut next_out,
                    &mut total,
                )
            };
            // SAFETY: Source consumption retains both pointers in this exact
            // initialized input allocation; no replacement occurs in this case.
            let consumed = unsafe { next_in.offset_from(start) } as usize;
            offset += consumed;
            assert_eq!(
                [
                    result as usize,
                    available_in,
                    available_out,
                    total,
                    consumed,
                    decoder.get_error_code() as usize,
                    decoder.is_used() as usize,
                    decoder.is_finished() as usize,
                    decoder.has_more_output() as usize,
                ],
                wanted.as_slice(),
                "{request}: {record}"
            );
            assert_eq!(&output[..output_chunk - available_out], bytes(values[9]));
            assert!(
                output[output_chunk - available_out..]
                    .iter()
                    .all(|&b| b == 0xcc)
            );
            terminal = result <= 1 || (result == 2 && offset == input.len());
        }
        assert!(terminal, "missing final source status: {request}");
        cases += 1;
    }
    assert_eq!(cases, 7);
}
