// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only native/parity observation of the actual Rust frame and writer.
use super::{hex, jpeg_data_protocol, number, unhex};
use jxl::jxl::base::status::{StatusCode, status_code};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::jpeg::jpeg_data::JpegData;
use std::fmt::Write as _;
fn data_observation(j: &JpegData, out: &mut String) {
    write!(out, " {} {} {}", j.width, j.height, j.components.len()).unwrap();
    for c in &j.components {
        write!(
            out,
            " {} {} {} {} {} {} {}",
            c.id,
            c.quant_idx,
            c.h_samp_factor,
            c.v_samp_factor,
            c.width_in_blocks,
            c.height_in_blocks,
            c.coeffs.len()
        )
        .unwrap();
        for v in &c.coeffs {
            write!(out, " {v}").unwrap();
        }
    }
    write!(out, " Q {}", j.quant.len()).unwrap();
    for q in &j.quant {
        write!(out, " {} {}", q.index, q.precision).unwrap();
        for v in q.values {
            write!(out, " {v}").unwrap();
        }
    }
}
pub fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mode = if kind == "jr-frame-mut" {
        number::<u32>(args)
    } else {
        0
    };
    let bytes = unhex(args.next().unwrap());
    let mut jpeg = jpeg_data_protocol::read_data(args);
    if mode != 9 {
        for c in &mut jpeg.components {
            c.coeffs.clear();
        }
    }
    match mode {
        1 => jpeg.components.clear(),
        2 => jpeg.components.resize_with(2, Default::default),
        4 => {
            let count = jpeg.quant.len();
            for c in &mut jpeg.components {
                c.quant_idx = count as u32;
            }
        }
        5 => {
            jpeg.quant.insert(0, jpeg.quant[0].clone());
            for c in &mut jpeg.components {
                c.quant_idx += 1;
            }
        }
        6 => jpeg.quant.push(jpeg.quant.last().unwrap().clone()),
        7 => {
            for c in &mut jpeg.components {
                c.quant_idx = 0;
            }
        }
        10 => {
            jpeg.quant.clear();
            // Missing-table control reaches the original qpos == size guard.
            // A nonzero index into an empty vector violates its precondition.
            for c in &mut jpeg.components {
                c.quant_idx = 0;
            }
        }
        _ => (),
    }
    let mut metadata = jxl::jxl::image_metadata::CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    let status = (|| {
        if reader.read_fixed_bits::<16>() != 0x0aff {
            return Err(StatusCode::GenericError);
        }
        jxl::jxl::headers::read_size_header(&mut reader, &mut metadata.size)?;
        jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut metadata.m)?;
        metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
        jxl::jxl::fields::bundle::read(&mut reader, &mut metadata.transform_data)?;
        assert!(
            !metadata.m.color_encoding.want_icc(),
            "fixture needs original ICC observer"
        );
        reader.jump_to_byte_boundary()
    })();
    let offset = reader.total_bits_consumed() / 8;
    let close = reader.close();
    let mut out = format!("{} {offset} {}", status_code(&status), status_code(&close));
    if status.is_err() || close.is_err() {
        return out;
    }
    let decoded =
        jxl::jxl::dec_frame::decode_jpeg_frame(&metadata, &bytes[offset..], Box::new(jpeg));
    write!(
        out,
        " F {} {} {}",
        status_code(&decoded.status),
        decoded.image.decoded_bytes(),
        decoded.image.color_transform.0
    )
    .unwrap();
    for c in 0..3 {
        write!(
            out,
            " {} {}",
            decoded.image.chroma_subsampling.h_shift(c),
            decoded.image.chroma_subsampling.v_shift(c)
        )
        .unwrap();
    }
    let jpeg = decoded.image.jpeg_data.as_deref().unwrap();
    data_observation(jpeg, &mut out);
    if decoded.status.is_ok() {
        let mut bytes = Vec::new();
        let status = jxl::jxl::jpeg::dec_jpeg_data_writer::write_jpeg(jpeg, |data| {
            let data = data.unwrap_or(&[]);
            bytes.extend_from_slice(data);
            data.len()
        });
        write!(out, " W {} {}", status_code(&status), hex(&bytes)).unwrap();
    }
    out
}
