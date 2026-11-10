// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Types and coefficient layout helpers from lib/jxl/coeff_order_fwd.h.

pub type CoeffOrder = u32;
pub const NUM_ORDERS: u8 = 13;
pub const fn coefficient_rows(rows: usize, columns: usize) -> usize {
    if rows < columns { rows } else { columns }
}
pub const fn coefficient_columns(rows: usize, columns: usize) -> usize {
    if rows < columns { columns } else { rows }
}
pub fn coefficient_layout(rows: &mut usize, columns: &mut usize) {
    let r = *rows;
    let c = *columns;
    *rows = coefficient_rows(r, c);
    *columns = coefficient_columns(r, c);
}
