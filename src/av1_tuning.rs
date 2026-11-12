//! Still-image tuning of `rav1e`: the quality-to-quantizer curve and the
//! per-speed overrides of `rav1e`'s presets.
//!
//! Both are adapted from `ravif` 0.13.0 (`src/av1encoder.rs`,
//! `quality_to_quantizer` and `SpeedTweaks`), which Ferrisma used before it
//! drove `rav1e` directly. Keeping them unchanged means `quality` and
//! `speed` keep their meaning, and the encoder experiment
//! (`docs/research/encoder-experiment`) shows the direct path produces the
//! same bytes as `ravif` with them, while `rav1e`'s plain presets are
//! 1.6–2× slower at equal SSIMULACRA2. That code is distributed under the
//! following license, whose notice must be retained:
//!
//! ```text
//! BSD 3-Clause License
//!
//! Copyright (c) 2020, Kornel
//! All rights reserved.
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//! ```

use rav1e::prelude::{
    BlockSize, PartitionRange, PredictionModesSetting, SGRComplexityLevel, SceneDetectionSpeed,
    SpeedSettings,
};

/// Map `quality` in `1.0..=100.0` to an AV1 quantizer in `0..=255`.
pub(crate) fn quality_to_quantizer(quality: f32) -> u8 {
    let q = quality / 100.0;
    let x = if q >= 0.82 {
        (1.0 - q) * 2.6
    } else if q > 0.25 {
        q.mul_add(-0.5, 1.0 - 0.125)
    } else {
        1.0 - q
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let quantizer = (x * 255.0).round().clamp(0.0, 255.0) as u8;
    quantizer
}

/// `rav1e`'s preset for `speed` with the still-image overrides.
pub(crate) fn speed_settings(speed: u8, quantizer: u8) -> SpeedSettings {
    let low_quality = quantizer < quality_to_quantizer(55.0);
    let high_quality = quantizer > quality_to_quantizer(80.0);
    let max_block = if high_quality { 16 } else { 64 };
    let block = |size: u8| match size {
        4 => BlockSize::BLOCK_4X4,
        8 => BlockSize::BLOCK_8X8,
        16 => BlockSize::BLOCK_16X16,
        32 => BlockSize::BLOCK_32X32,
        _ => BlockSize::BLOCK_64X64,
    };
    let (smallest, largest) = match speed {
        0 => (4, 64.min(max_block)),
        1 if low_quality => (4, 64.min(max_block)),
        2 if low_quality => (4, 32.min(max_block)),
        1..=4 => (4, 16),
        5..=8 => (8, 16),
        _ => (16, 16),
    };

    let mut settings = SpeedSettings::from_preset(speed);
    // A still image has no other frames to reference or detect cuts in.
    settings.multiref = false;
    settings.rdo_lookahead_frames = 1;
    settings.scene_detection_mode = SceneDetectionSpeed::None;
    settings.motion.include_near_mvs = false;
    settings.motion.use_satd_subpel = false;

    settings.partition.partition_range = PartitionRange::new(block(smallest), block(largest));
    settings.prediction.prediction_modes = if speed <= 1 {
        PredictionModesSetting::ComplexAll
    } else {
        PredictionModesSetting::Simple
    };
    settings.sgr_complexity = if speed <= 2 {
        SGRComplexityLevel::Full
    } else {
        SGRComplexityLevel::Reduced
    };
    settings.partition.encode_bottomup = speed <= 2;
    settings.transform.rdo_tx_decision = speed <= 4 && !high_quality;
    settings.transform.reduced_tx_set = speed == 4 || speed >= 9;
    settings.prediction.fine_directional_intra = speed <= 6;
    settings.fast_deblock = speed >= 7 && !high_quality;
    settings.lrf = low_quality && speed <= 8;
    settings.cdef = low_quality && speed <= 9;
    settings.transform.enable_inter_tx_split = speed >= 9;
    settings.transform.tx_domain_rate = speed >= 10;
    settings
}
