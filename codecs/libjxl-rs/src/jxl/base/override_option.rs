// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Tri-state options from lib/jxl/base/override.h. The module name avoids the
//! reserved Rust keyword while retaining the original type and behavior.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Override(pub i8);
impl Override {
    pub const ON: Self = Self(1);
    pub const OFF: Self = Self(0);
    pub const DEFAULT: Self = Self(-1);
}
pub fn override_from_bool(flag: bool) -> Override {
    if flag { Override::ON } else { Override::OFF }
}
pub fn apply_override(value: Override, default_condition: bool) -> bool {
    if value == Override::ON {
        return true;
    }
    if value == Override::OFF {
        return false;
    }
    default_condition
}
