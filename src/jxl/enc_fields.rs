// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_fields.cc`: primitive coders and Bundle::Write.
//! Codestream/image metadata and frame header writes are also translated.
//! Quantizer writing remains unported.

use super::base::status::{Status, StatusCode};
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::field_encodings::{Fields, U32Enc};
use super::fields::{Visitor, VisitorBase, visitor_base};

pub mod bits_coder {
    use super::*;

    pub fn write(bits: usize, value: u32, writer: &mut BitWriter) -> Status {
        assert!(bits <= 32);
        if u64::from(value) >= 1u64 << bits {
            return Err(StatusCode::GenericError);
        }
        writer.write(bits, u64::from(value));
        Ok(())
    }
}

pub mod u32_coder {
    use super::*;

    pub fn write(enc: U32Enc, value: u32, writer: &mut BitWriter) -> Status {
        let (selector, total_bits) = crate::jxl::fields::u32_coder::choose_selector(enc, value)?;
        writer.write(2, u64::from(selector));
        let d = enc.get_distr(selector);
        if !d.is_direct() {
            let offset = d.offset();
            if value < offset {
                return Err(StatusCode::GenericError);
            }
            writer.write(total_bits - 2, u64::from(value - offset));
        }
        Ok(())
    }
}

pub mod u64_coder {
    use super::*;

    pub fn write(mut value: u64, writer: &mut BitWriter) -> Status {
        if value == 0 {
            writer.write(2, 0);
        } else if value <= 16 {
            writer.write(2, 1);
            writer.write(4, value - 1);
        } else if value <= 272 {
            writer.write(2, 2);
            writer.write(8, value - 17);
        } else {
            writer.write(2, 3);
            writer.write(12, value & 4095);
            value >>= 12;
            let mut shift = 12;
            while value > 0 && shift < 60 {
                writer.write(1, 1);
                writer.write(8, value & 255);
                value >>= 8;
                shift += 8;
            }
            if value > 0 {
                writer.write(1, 1);
                writer.write(4, value & 15);
            } else {
                writer.write(1, 0);
            }
        }
        Ok(())
    }
}

pub mod f16_coder {
    use super::*;

    #[expect(
        clippy::needless_late_init,
        reason = "Keep the upstream F16 conversion branches easy to compare"
    )]
    pub fn write(value: f32, writer: &mut BitWriter) -> Status {
        let bits32 = value.to_bits();
        let sign = bits32 >> 31;
        let biased_exp32 = (bits32 >> 23) & 0xFF;
        let mantissa32 = bits32 & 0x7F_FFFF;
        let exp = biased_exp32 as i32 - 127;
        if exp > 15 {
            return Err(StatusCode::GenericError);
        }
        if exp < -24 {
            writer.write(16, 0);
            return Ok(());
        }
        let biased_exp16;
        let mantissa16;
        if exp < -14 {
            biased_exp16 = 0;
            let sub_exp = (-14 - exp) as u32;
            debug_assert!((1..11).contains(&sub_exp));
            mantissa16 = (1 << (10 - sub_exp)) + (mantissa32 >> (13 + sub_exp));
        } else {
            biased_exp16 = (exp + 15) as u32;
            debug_assert!((1..31).contains(&biased_exp16));
            mantissa16 = mantissa32 >> 13;
        }
        debug_assert!(mantissa16 < 1024);
        let bits16 = (sign << 15) | (biased_exp16 << 10) | mantissa16;
        debug_assert!(bits16 < 0x1_0000);
        writer.write(16, u64::from(bits16));
        Ok(())
    }
}

struct WriteVisitor<'w> {
    base: VisitorBase,
    extension_bits: usize,
    writer: &'w mut BitWriter,
    ok: bool,
}

impl Visitor for WriteVisitor<'_> {
    visitor_base!();

    fn bits(&mut self, bits: usize, _: u32, value: &mut u32) -> Status {
        self.ok &= bits_coder::write(bits, *value, self.writer).is_ok();
        Ok(())
    }

    fn u32(&mut self, enc: U32Enc, _: u32, value: &mut u32) -> Status {
        self.ok &= u32_coder::write(enc, *value, self.writer).is_ok();
        Ok(())
    }

    fn u64(&mut self, _: u64, value: &mut u64) -> Status {
        self.ok &= u64_coder::write(*value, self.writer).is_ok();
        Ok(())
    }

    fn f16(&mut self, _: f32, value: &mut f32) -> Status {
        self.ok &= f16_coder::write(*value, self.writer).is_ok();
        Ok(())
    }

    fn begin_extensions(&mut self, extensions: &mut u64) -> Status {
        self.u64(0, extensions)?;
        self.base().extension_states_begin();
        if *extensions == 0 {
            if self.extension_bits != 0 {
                return Err(StatusCode::GenericError);
            }
            return Ok(());
        }
        self.ok &= u64_coder::write(self.extension_bits as u64, self.writer).is_ok();
        let mut remaining_extensions = *extensions & (*extensions - 1);
        while remaining_extensions != 0 {
            self.ok &= u64_coder::write(0, self.writer).is_ok();
            remaining_extensions &= remaining_extensions - 1;
        }
        Ok(())
    }
}

pub mod bundle {
    use super::*;

    pub fn write(
        fields: &mut dyn Fields,
        writer: &mut BitWriter,
        layer: LayerType,
        aux_out: Option<&mut AuxOut>,
    ) -> Status {
        let (extension_bits, total_bits) = crate::jxl::fields::bundle::can_encode(fields)?;
        writer.with_max_bits(
            total_bits,
            layer,
            aux_out,
            |writer, _| {
                let mut visitor = WriteVisitor {
                    base: VisitorBase::default(),
                    extension_bits,
                    writer,
                    ok: true,
                };
                visitor.visit(fields)?;
                if visitor.ok {
                    Ok(())
                } else {
                    Err(StatusCode::GenericError)
                }
            },
            false,
        )
    }
}

pub fn write_size_header(
    size: &mut super::headers::SizeHeader,
    writer: &mut BitWriter,
    layer: LayerType,
    aux_out: Option<&mut AuxOut>,
) -> Status {
    bundle::write(size, writer, layer, aux_out)
}

pub fn write_image_metadata(
    metadata: &mut super::image_metadata::ImageMetadata,
    writer: &mut BitWriter,
    layer: LayerType,
    aux_out: Option<&mut AuxOut>,
) -> Status {
    bundle::write(metadata, writer, layer, aux_out)
}

pub fn write_codestream_headers(
    metadata: &mut super::image_metadata::CodecMetadata,
    writer: &mut BitWriter,
    mut aux_out: Option<&mut AuxOut>,
) -> Status {
    writer.with_max_bits(
        16,
        LayerType::Header,
        aux_out.as_deref_mut(),
        |w, _| {
            w.write(8, 0xff);
            w.write(8, u64::from(super::headers::CODESTREAM_MARKER));
            Ok(())
        },
        false,
    )?;
    write_size_header(
        &mut metadata.size,
        writer,
        LayerType::Header,
        aux_out.as_deref_mut(),
    )?;
    write_image_metadata(
        &mut metadata.m,
        writer,
        LayerType::Header,
        aux_out.as_deref_mut(),
    )?;
    metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
    bundle::write(
        &mut metadata.transform_data,
        writer,
        LayerType::Header,
        aux_out,
    )
}

pub fn write_frame_header(
    frame: &mut super::frame_header::FrameHeader<'_>,
    writer: &mut BitWriter,
    aux_out: Option<&mut AuxOut>,
) -> Status {
    bundle::write(frame, writer, LayerType::Header, aux_out)
}
