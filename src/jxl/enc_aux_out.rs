// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from the layer-accounting portion of `lib/jxl/enc_aux_out.{h,cc}`.
//! Image block statistics and formatted diagnostic output remain unported.

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum LayerType {
    Header = 0,
    Toc,
    Dictionary,
    Splines,
    Noise,
    Quant,
    ModularTree,
    ModularGlobal,
    Dc,
    ModularDcGroup,
    ControlFields,
    Order,
    Ac,
    AcTokens,
    ModularAcGroup,
}

pub const NUM_IMAGE_LAYERS: usize = LayerType::ModularAcGroup as usize + 1;

#[derive(Clone, Copy, Default, Debug)]
pub struct LayerTotals {
    pub num_clustered_histograms: usize,
    pub extra_bits: usize,
    pub histogram_bits: usize,
    pub total_bits: usize,
    pub clustered_entropy: f64,
}

#[derive(Clone, Default, Debug)]
pub struct AuxOut {
    pub layers: [LayerTotals; NUM_IMAGE_LAYERS],
}

impl AuxOut {
    pub fn total_bits(&self) -> usize {
        let mut total: usize = 0;
        for layer in &self.layers {
            total = total.wrapping_add(layer.total_bits);
        }
        total
    }

    pub fn layer(&self, idx: LayerType) -> &LayerTotals {
        &self.layers[idx as usize]
    }

    pub fn layer_mut(&mut self, idx: LayerType) -> &mut LayerTotals {
        &mut self.layers[idx as usize]
    }
}
