// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! The canonical skcms_Curve union and directly reached scalar operations from
//! src/skcms_public.h and skcms.cc:302-325,1187-1254 at the pinned revision.
//! Table storage is borrowed. Raw operations require the source's initialized
//! branch, storage lifetime, bounds and arithmetic preconditions.

use super::transfer_function::TransferFunction;
use std::mem::{MaybeUninit, align_of, size_of};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParametricCurve {
    pub alias_of_table_entries: u32,
    pub function: TransferFunction,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TableCurve {
    pub table_entries: u32,
    padding: [MaybeUninit<u8>; TABLE_PADDING],
    pub table_8: *const u8,
    pub table_16: *const u8,
}
const TABLE_PADDING: usize =
    (align_of::<*const u8>() - 4 % align_of::<*const u8>()) % align_of::<*const u8>();
const CURVE_STORAGE_SIZE: usize = if size_of::<ParametricCurve>() > size_of::<TableCurve>() {
    size_of::<ParametricCurve>()
} else {
    size_of::<TableCurve>()
};

/// Exact source union. A zero count selects the function beginning AFTER the
/// count. Table pointer padding and inactive bytes are not implicit defaults.
#[repr(C)]
#[derive(Clone, Copy)]
pub union Curve {
    pub parametric: ParametricCurve,
    pub table: TableCurve,
    storage: [MaybeUninit<u8>; CURVE_STORAGE_SIZE],
}
impl Curve {
    pub const fn zeroed() -> Self {
        Self {
            storage: [MaybeUninit::new(0); CURVE_STORAGE_SIZE],
        }
    }
    pub const fn from_parametric(function: TransferFunction) -> Self {
        Self {
            parametric: ParametricCurve {
                alias_of_table_entries: 0,
                function,
            },
        }
    }
}

/// Copies every source union byte, including initialized inactive bytes.
///
/// # Safety
/// Source and destination must point to disjoint, aligned Curve storage valid
/// for a full object copy. Borrowed table storage is not copied or retained by
/// an owner; its lifetime must cover every later use of the copied pointers.
pub unsafe fn copy_curve(source: *const Curve, destination: *mut Curve) {
    // SAFETY: the caller provides disjoint full objects. MaybeUninit copies
    // representation bytes without reading inactive indeterminate bytes as u8.
    unsafe {
        ptr::copy_nonoverlapping(
            source.cast::<MaybeUninit<u8>>(),
            destination.cast::<MaybeUninit<u8>>(),
            size_of::<Curve>(),
        );
    }
}

fn fmax(x: f32, y: f32) -> f32 {
    if x > y { x } else { y }
}
fn fmin(x: f32, y: f32) -> f32 {
    if x < y { x } else { y }
}
fn minus_1_ulp(x: f32) -> f32 {
    f32::from_bits(((x.to_bits() as i32) - 1) as u32)
}
fn interpolate(l: f32, h: f32, t: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        // The matching native original contracts this source expression.
        (h - l).mul_add(t, l)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        l + (h - l) * t
    }
}

/// Evaluates the actual source's parametric function or borrowed 8/16-bit table.
///
/// # Safety
/// The prefix count and selected source branch must be initialized. A zero
/// count requires all seven function words. A nonzero count requires initialized
/// table pointers and readable storage for all reached indices; the count/index
/// arithmetic and float-to-int conversions must be source-defined. Borrowed
/// bytes must remain live and must not be concurrently mutated during reads.
pub unsafe fn eval_curve(curve: *const Curve, x: f32) -> f32 {
    // SAFETY: the initialized prefix is common to both source union branches.
    let entries = unsafe { curve.cast::<u32>().read() };
    if entries == 0 {
        // SAFETY: the selected source parametric branch has seven valid words;
        // this temporary shared view ends before any caller can mutate them.
        return unsafe { (&*ptr::addr_of!((*curve).parametric.function)).eval(x) };
    }
    let ix = fmax(0.0, fmin(x, 1.0)) * (entries - 1) as f32;
    let lo = ix as i32;
    let hi = minus_1_ulp(ix + 1.0) as i32;
    let t = ix - lo as f32;
    // SAFETY: the active table branch initializes this pointer field. Padding
    // and inactive parametric words are not loaded.
    let table_8 = unsafe { ptr::addr_of!((*curve).table.table_8).read() };
    let (l, h) = if !table_8.is_null() {
        // SAFETY: the caller guarantees readable reached indices in this table.
        unsafe {
            (
                *table_8.add(lo as usize) as f32 * (1.0 / 255.0),
                *table_8.add(hi as usize) as f32 * (1.0 / 255.0),
            )
        }
    } else {
        // SAFETY: the selected branch initializes this pointer, and both source
        // two-byte reads are within live table storage. Unaligned loads match
        // the original memcpy followed by its unconditional byte swap.
        unsafe {
            let table = ptr::addr_of!((*curve).table.table_16).read();
            (
                table
                    .add((2 * lo) as usize)
                    .cast::<u16>()
                    .read_unaligned()
                    .swap_bytes() as f32
                    * (1.0 / 65535.0),
                table
                    .add((2 * hi) as usize)
                    .cast::<u16>()
                    .read_unaligned()
                    .swap_bytes() as f32
                    * (1.0 / 65535.0),
            )
        }
    };
    interpolate(l, h, t)
}

/// Fits the source linear prefix, preserving partial writes and output aliases.
///
/// # Safety
/// Curve must satisfy eval_curve's contract at every reached sample. N and its
/// arithmetic satisfy the original internal caller contract (N > 1). C and D
/// must be writable at their reached writes; non-null F must be readable and
/// writable. Aliases are permitted only when their reached accesses remain
/// source-defined; no Rust references to output slots are retained across reads.
pub unsafe fn fit_linear(
    curve: *const Curve,
    n: i32,
    tol: f32,
    c: *mut f32,
    d: *mut f32,
    mut f: *mut f32,
) -> i32 {
    let dx = 1.0 / (n - 1) as f32;
    let mut lin_points = 1;
    let mut f_zero = 0.0;
    if !f.is_null() {
        // SAFETY: the caller provides initialized curve storage and writable F;
        // the evaluated shared view has ended before the output is mutated.
        unsafe { f.write(eval_curve(curve, 0.0)) };
    } else {
        f = &raw mut f_zero;
    }
    let mut slope_min = f32::NEG_INFINITY;
    let mut slope_max = f32::INFINITY;
    for i in 1..n {
        let x = i as f32 * dx;
        // SAFETY: source curve samples and output-slot reads are valid here.
        let (y, slope_max_i, slope_min_i) = unsafe {
            let y = eval_curve(curve, x);
            (y, (y + tol - f.read()) / x, (y - tol - f.read()) / x)
        };
        if slope_max_i < slope_min || slope_max < slope_min_i {
            break;
        }
        slope_max = fmin(slope_max, slope_max_i);
        slope_min = fmax(slope_min, slope_min_i);
        // SAFETY: F is the live initialized caller slot or local f_zero. It is
        // reread because a previous C write may alias and change this value.
        let cur_slope = (y - unsafe { f.read() }) / x;
        if slope_min <= cur_slope && cur_slope <= slope_max {
            lin_points = i + 1;
            // SAFETY: the caller supplies writable C at every reached write.
            unsafe { c.write(cur_slope) };
        }
    }
    // SAFETY: D is writable at the source's final checkpoint, after all C/F
    // accesses; possible caller aliases retain this exact write order.
    unsafe { d.write((lin_points - 1) as f32 * dx) };
    lin_points
}

/// Rewrites a source identity table into its parametric representation.
///
/// # Safety
/// Curve's count and selected storage satisfy eval_curve/fit_linear, including
/// the reached signed 2*N arithmetic. It is writable through the final rewrite;
/// no external borrowed table is freed, copied, or mutated by this operation.
pub unsafe fn canonicalize_identity(curve: *mut Curve) {
    // SAFETY: count is the initialized common prefix.
    let entries = unsafe { curve.cast::<u32>().read() };
    if entries != 0 && entries <= i32::MAX as u32 {
        let n = entries as i32;
        let (mut c, mut d, mut f) = (0.0, 0.0, 0.0);
        // SAFETY: caller's curve satisfies the source sampling contract; local
        // outputs are initialized, writable and distinct.
        if n == unsafe {
            fit_linear(
                curve,
                n,
                1.0 / (2 * n) as f32,
                &raw mut c,
                &raw mut d,
                &raw mut f,
            )
        } && c == 1.0
            && f == 0.0
        {
            // SAFETY: exact original field writes preserve overlap order and
            // then initialize all seven words of the new parametric branch.
            unsafe {
                curve.cast::<u32>().write(0);
                ptr::addr_of_mut!((*curve).table.table_8).write(ptr::null());
                ptr::addr_of_mut!((*curve).table.table_16).write(ptr::null());
                ptr::addr_of_mut!((*curve).parametric.function).write(TransferFunction {
                    g: 1.0,
                    a: 1.0,
                    b: 0.0,
                    c: 0.0,
                    d: 0.0,
                    e: 0.0,
                    f: 0.0,
                });
            }
        }
    }
}
