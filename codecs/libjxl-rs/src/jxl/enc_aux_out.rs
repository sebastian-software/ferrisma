// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Accounting, statistics storage, and assimilation from
//! lib/jxl/enc_aux_out.{h,cc}. Formatted diagnostics remain unported.

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
impl LayerTotals {
    pub fn assimilate(&mut self, victim: &Self) {
        self.num_clustered_histograms = self
            .num_clustered_histograms
            .wrapping_add(victim.num_clustered_histograms);
        self.histogram_bits = self.histogram_bits.wrapping_add(victim.histogram_bits);
        self.extra_bits = self.extra_bits.wrapping_add(victim.extra_bits);
        self.total_bits = self.total_bits.wrapping_add(victim.total_bits);
        self.clustered_entropy += victim.clustered_entropy;
    }
}

#[derive(Clone, Default, Debug)]
pub struct AuxOut {
    pub layers: [LayerTotals; NUM_IMAGE_LAYERS],
    pub num_blocks: usize,
    pub num_small_blocks: usize,
    pub num_dct4x8_blocks: usize,
    pub num_afv_blocks: usize,
    pub num_dct8_blocks: usize,
    pub num_dct8x16_blocks: usize,
    pub num_dct8x32_blocks: usize,
    pub num_dct16_blocks: usize,
    pub num_dct16x32_blocks: usize,
    pub num_dct32_blocks: usize,
    pub num_dct32x64_blocks: usize,
    pub num_dct64_blocks: usize,
    pub num_butteraugli_iters: i32,
}

impl AuxOut {
    pub fn assimilate(&mut self, victim: &Self) {
        for i in 0..NUM_IMAGE_LAYERS {
            self.layers[i].assimilate(&victim.layers[i]);
        }
        self.num_blocks = self.num_blocks.wrapping_add(victim.num_blocks);
        self.num_small_blocks = self.num_small_blocks.wrapping_add(victim.num_small_blocks);
        self.num_dct4x8_blocks = self
            .num_dct4x8_blocks
            .wrapping_add(victim.num_dct4x8_blocks);
        self.num_afv_blocks = self.num_afv_blocks.wrapping_add(victim.num_afv_blocks);
        self.num_dct8_blocks = self.num_dct8_blocks.wrapping_add(victim.num_dct8_blocks);
        self.num_dct8x16_blocks = self
            .num_dct8x16_blocks
            .wrapping_add(victim.num_dct8x16_blocks);
        self.num_dct8x32_blocks = self
            .num_dct8x32_blocks
            .wrapping_add(victim.num_dct8x32_blocks);
        self.num_dct16_blocks = self.num_dct16_blocks.wrapping_add(victim.num_dct16_blocks);
        self.num_dct16x32_blocks = self
            .num_dct16x32_blocks
            .wrapping_add(victim.num_dct16x32_blocks);
        self.num_dct32_blocks = self.num_dct32_blocks.wrapping_add(victim.num_dct32_blocks);
        self.num_dct32x64_blocks = self
            .num_dct32x64_blocks
            .wrapping_add(victim.num_dct32x64_blocks);
        self.num_dct64_blocks = self.num_dct64_blocks.wrapping_add(victim.num_dct64_blocks);
        self.num_butteraugli_iters = self
            .num_butteraugli_iters
            .checked_add(victim.num_butteraugli_iters)
            .expect("source signed statistic sum must be representable");
    }
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
