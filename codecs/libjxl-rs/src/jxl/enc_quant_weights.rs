// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! DC serialization, custom-DC round trip, and scaling from
//! lib/jxl/enc_quant_weights.cc. AC quantization tables remain unported.
//! Native owners replace the memory-manager parameter. The private reader is
//! closed on every Rust path while preserving the source's primary status.

use super::base::status::Status;
use super::dec_bit_reader::BitReader;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_fields::f16_coder;
use super::quant_weights::{DC_QUANT, DequantMatrices, QuantMode};

/// DequantMatricesEncode for matrices whose encodings are all the default
/// library table, the only matrices the admitted encoder profile produces
/// (FindBestDequantMatrices resets them). EncodeQuant for custom encodings
/// is not translated; such matrices return an error before writing.
pub fn dequant_matrices_encode_default(
    matrices: &DequantMatrices,
    writer: &mut BitWriter,
    layer: LayerType,
    aux: Option<&mut AuxOut>,
) -> Status {
    let mut all_default = true;
    for encoding in matrices.encodings() {
        if encoding.mode() != QuantMode::Library || encoding.predefined != 0 {
            all_default = false;
        }
    }
    if !all_default {
        return Err(super::base::status::StatusCode::Unsupported);
    }
    // TODO(janwas): better bound
    writer.with_max_bits(
        512 * 1024,
        layer,
        aux,
        |writer, _| {
            writer.write(1, u64::from(all_default));
            Ok(())
        },
        false,
    )
}

pub fn dequant_matrices_encode_dc(
    matrices: &DequantMatrices,
    writer: &mut BitWriter,
    layer: LayerType,
    aux: Option<&mut AuxOut>,
) -> Status {
    let mut all_default = true;
    let dc_quant = matrices.dc_quants();
    for c in 0..3 {
        if dc_quant[c] != DC_QUANT[c] {
            all_default = false;
        }
    }
    writer.with_max_bits(
        1 + size_of::<f32>() * 8 * 3,
        layer,
        aux,
        |writer, _| {
            writer.write(1, u64::from(all_default));
            if !all_default {
                for &value in dc_quant {
                    f16_coder::write(value * 128.0, writer)?;
                }
            }
            Ok(())
        },
        false,
    )
}

pub fn dequant_matrices_set_custom_dc(matrices: &mut DequantMatrices, dc: &[f32; 3]) -> Status {
    matrices.set_dc_quant(dc);
    let mut writer = BitWriter::new();
    dequant_matrices_encode_dc(matrices, &mut writer, LayerType::Header, None)?;
    writer.zero_pad_to_byte();
    let mut reader = BitReader::new(writer.get_span());
    let decoded = matrices.decode_dc(&mut reader);
    let closed = reader.close();
    decoded?;
    closed
}

pub fn dequant_matrices_scale_dc(matrices: &mut DequantMatrices, scale: f32) -> Status {
    let mut dc = [0.0f32; 3];
    for (c, value) in dc.iter_mut().enumerate() {
        *value = matrices.inv_dc_quant(c) * (1.0 / scale);
    }
    dequant_matrices_set_custom_dc(matrices, &dc)?;
    Ok(())
}
