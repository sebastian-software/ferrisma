// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Section table output from lib/jxl/enc_toc.cc. Nested allotments preserve
//! separate TOC and permutation-entropy layer accounting.

use super::base::status::Status;
use super::coeff_order_fwd::CoeffOrder;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_coeff_order::encode_permutation;
use super::enc_fields::u32_coder;
use super::toc::{TOC_DIST, max_bits};

pub fn write_toc_permutation(
    permutation: &[CoeffOrder],
    writer: &mut BitWriter,
    aux: Option<&mut AuxOut>,
) -> Status {
    writer.with_max_bits(
        max_bits(0),
        LayerType::Toc,
        aux,
        |writer, aux| {
            if !permutation.is_empty() {
                writer.write(1, 1);
                encode_permutation(
                    permutation,
                    0,
                    permutation.len(),
                    writer,
                    LayerType::Header,
                    aux,
                )?;
            } else {
                writer.write(1, 0);
            }
            writer.zero_pad_to_byte();
            Ok(())
        },
        false,
    )
}
pub fn write_toc_sizes(
    group_sizes: &[usize],
    writer: &mut BitWriter,
    aux: Option<&mut AuxOut>,
) -> Status {
    writer.with_max_bits(
        max_bits(group_sizes.len()),
        LayerType::Toc,
        aux,
        |writer, _| {
            for &size in group_sizes {
                u32_coder::write(TOC_DIST, size as u32, writer)?;
            }
            writer.zero_pad_to_byte();
            Ok(())
        },
        false,
    )
}
