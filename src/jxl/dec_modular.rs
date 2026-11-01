// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ModularStreamId storage, constructors, and index mapping from
//! lib/jxl/dec_modular.h. The frame decoder and debug descriptions are unported.
//! Kind retains raw fixed-underlying-type enum values: the source switch leaves
//! an unknown kind's initialized index at zero. All size_t arithmetic wraps.

use super::base::status::{StatusCode, StatusOr};
use super::frame_dimensions::FrameDimensions;
use super::quant_weights::NUM_QUANT_TABLES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ModularStreamKind(pub i32);
impl ModularStreamKind {
    pub const GLOBAL_DATA: Self = Self(0);
    pub const VAR_DCT_DC: Self = Self(1);
    pub const MODULAR_DC: Self = Self(2);
    pub const AC_METADATA: Self = Self(3);
    pub const QUANT_TABLE: Self = Self(4);
    pub const MODULAR_AC: Self = Self(5);
}
#[derive(Clone, Copy, Debug)]
pub struct ModularStreamId {
    pub kind: ModularStreamKind,
    pub quant_table_id: usize,
    pub group_id: usize,
    pub pass_id: usize,
}
impl ModularStreamId {
    pub fn id(&self, frame_dim: &FrameDimensions) -> usize {
        match self.kind {
            ModularStreamKind::GLOBAL_DATA => 0,
            ModularStreamKind::VAR_DCT_DC => 1usize.wrapping_add(self.group_id),
            ModularStreamKind::MODULAR_DC => 1usize
                .wrapping_add(frame_dim.num_dc_groups)
                .wrapping_add(self.group_id),
            ModularStreamKind::AC_METADATA => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(2))
                .wrapping_add(self.group_id),
            ModularStreamKind::QUANT_TABLE => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(self.quant_table_id),
            ModularStreamKind::MODULAR_AC => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(NUM_QUANT_TABLES as usize)
                .wrapping_add(frame_dim.num_groups.wrapping_mul(self.pass_id))
                .wrapping_add(self.group_id),
            _ => 0,
        }
    }
    pub fn global() -> Self {
        Self {
            kind: ModularStreamKind::GLOBAL_DATA,
            quant_table_id: 0,
            group_id: 0,
            pass_id: 0,
        }
    }
    pub fn var_dct_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::VAR_DCT_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn modular_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn ac_metadata(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::AC_METADATA,
            group_id,
            ..Self::global()
        }
    }
    pub fn quant_table(quant_table_id: usize) -> StatusOr<Self> {
        if quant_table_id >= NUM_QUANT_TABLES as usize {
            return Err(StatusCode::GenericError);
        }
        Ok(Self {
            kind: ModularStreamKind::QUANT_TABLE,
            quant_table_id,
            ..Self::global()
        })
    }
    pub fn modular_ac(group_id: usize, pass_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_AC,
            group_id,
            pass_id,
            ..Self::global()
        }
    }
    pub fn num(frame_dim: &FrameDimensions, passes: usize) -> usize {
        Self::modular_ac(0, passes).id(frame_dim)
    }
}
