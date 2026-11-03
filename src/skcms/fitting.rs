// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Original skcms.cc:328-342,2110-2415 curve approximation and fitting closure.
//! Borrowed table storage, source tolerances/iteration count and partial output
//! checkpoints are retained. No alternative optimizer or curve model is used.

use super::curve::{Curve, eval_curve, fit_linear};
use super::transfer_function::{TfType, TransferFunction as Tf, log2f, logf, powf};
use super::types::Matrix3x3;
use std::ptr;

fn finite(x: f32) -> bool {
    0.0 == x * 0.0
}
fn fabs(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}
fn max(x: f32, y: f32) -> f32 {
    if x > y { x } else { y }
}
fn madd(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}
fn subtract_product(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, -c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b - c
    }
}

/// Original maximum sampled round-trip error, including source NaN selection.
///
/// # Safety
/// Curve count/selected branch and reached inverse TF fields are initialized; borrowed
/// tables must remain readable for every reached sample. eval_curve/TF math
/// must satisfy the source's index, bounds and float-to-integer preconditions.
pub unsafe fn max_roundtrip_error(curve: *const Curve, inv_tf: *const Tf) -> f32 {
    // SAFETY: the source union count is the initialized common prefix.
    let entries = unsafe { curve.cast::<u32>().read() };
    let n = if entries > 256 { entries } else { 256 };
    let dx = 1.0 / (n - 1) as f32;
    let mut error = 0.0;
    for i in 0..n {
        let x = i as f32 * dx;
        // SAFETY: the caller establishes all reached source sampling reads.
        let y = unsafe { eval_curve(curve, x) };
        // SAFETY: shared inverse storage remains valid through this call.
        error = max(
            error,
            fabs(x - unsafe { super::transfer_function::eval_raw(inv_tf, y) }),
        );
    }
    error
}

/// # Safety
/// Same source sampling contract as max_roundtrip_error.
pub unsafe fn are_approximate_inverses(curve: *const Curve, inv_tf: *const Tf) -> bool {
    // SAFETY: the caller supplies the identical source sampling preconditions.
    unsafe { max_roundtrip_error(curve, inv_tf) < 1.0 / 512.0 }
}

/// Original residual and three gradient writes, in their original order.
///
/// # Safety
/// Curve and reached TF words must be initialized and source math defined.
/// Gradient points to three writable floats. Reached aliases must remain valid;
/// no Rust references are retained across the three external output writes.
pub unsafe fn rg_nonlinear(x: f32, curve: *const Curve, tf: *const Tf, gradient: *mut f32) -> f32 {
    // SAFETY: the caller supplies readable source curve storage for x.
    let y = unsafe { eval_curve(curve, x) };
    // SAFETY: only the six fields loaded by the original helper are read.
    // In particular, the unused source e word is not inspected.
    let (g, a, b, c, d, f) = unsafe {
        (
            ptr::addr_of!((*tf).g).read(),
            ptr::addr_of!((*tf).a).read(),
            ptr::addr_of!((*tf).b).read(),
            ptr::addr_of!((*tf).c).read(),
            ptr::addr_of!((*tf).d).read(),
            ptr::addr_of!((*tf).f).read(),
        )
    };
    let yy = max(madd(a, y, b), 0.0);
    let dd = madd(a, d, b);
    let first = subtract_product(logf(yy), powf(yy, g), logf(dd) * powf(dd, g));
    // SAFETY: gradient's first float is writable. All original TF inputs have
    // already been loaded, so reached source aliases preserve their ordering.
    unsafe { gradient.write(first) };
    let second = subtract_product(y * g, powf(yy, g - 1.0), d * g * powf(dd, g - 1.0));
    // SAFETY: gradient's second float is writable under the same contract.
    unsafe { gradient.add(1).write(second) };
    let third = subtract_product(g, powf(yy, g - 1.0), g * powf(dd, g - 1.0));
    // SAFETY: gradient's third float is writable under the same contract.
    unsafe { gradient.add(2).write(third) };
    let inverse = madd(c, d, powf(yy, g) - powf(dd, g)) + f;
    x - inverse
}

/// One original Gauss-Newton step; singular/unusable matrices retain TF state.
///
/// # Safety
/// Every reached curve/TF sample satisfies rg_nonlinear's contract, and TF's
/// g/a/b fields are initialized and writable. Count/step arithmetic must remain
/// source-defined. Curve storage need not be reached when N is nonpositive.
pub unsafe fn gauss_newton_step(
    curve: *const Curve,
    tf: *mut Tf,
    x0: f32,
    dx: f32,
    n: i32,
) -> bool {
    let mut lhs = Matrix3x3::default();
    let mut rhs = [0.0; 3];
    for i in 0..n {
        let x = madd(i as f32, dx, x0);
        let mut gradient = [0.0; 3];
        // SAFETY: caller's source sample contract applies at each reached x;
        // the three gradient outputs are initialized distinct local floats.
        let residual = unsafe { rg_nonlinear(x, curve, tf, gradient.as_mut_ptr()) };
        for r in 0..3 {
            for c in 0..3 {
                lhs.vals[r][c] = madd(gradient[r], gradient[c], lhs.vals[r][c]);
            }
            rhs[r] = madd(gradient[r], residual, rhs[r]);
        }
    }
    for k in 0..3 {
        if lhs.vals[0][k] == 0.0
            && lhs.vals[1][k] == 0.0
            && lhs.vals[2][k] == 0.0
            && lhs.vals[k][0] == 0.0
            && lhs.vals[k][1] == 0.0
            && lhs.vals[k][2] == 0.0
        {
            lhs.vals[k][k] = 1.0;
        }
    }
    let mut inverse = Matrix3x3::default();
    if !lhs.invert_into(&mut inverse) {
        return false;
    }
    let delta = inverse.mul_vector(rhs);
    // SAFETY: the caller provides the original initialized, writable fields.
    // Read/add/write each field in source order; no other TF word is read.
    unsafe {
        let g = ptr::addr_of_mut!((*tf).g);
        g.write(g.read() + delta[0]);
        let a = ptr::addr_of_mut!((*tf).a);
        a.write(a.read() + delta[1]);
        let b = ptr::addr_of_mut!((*tf).b);
        b.write(b.read() + delta[2]);
        finite(g.read()) && finite(a.read()) && finite(b.read())
    }
}

/// # Safety
/// The curve sampling contract applies if inversion succeeds; only inverse TF
/// fields reached by classification/inversion must be initialized.
pub unsafe fn max_roundtrip_error_checked(curve: *const Curve, tf_inv: *const Tf) -> f32 {
    let mut tf = Tf::default();
    // SAFETY: reached classifier/inversion fields remain readable; local output
    // is initialized and disjoint from the possibly partial external function.
    if !unsafe { super::transfer_function::invert_raw(tf_inv, &mut tf) }
        || tf.get_type() != TfType::Srgbish
    {
        return f32::INFINITY;
    }
    let mut again = Tf::default();
    if !tf.invert_into(&mut again) {
        return f32::INFINITY;
    }
    // SAFETY: source curve samples and this initialized local inverse are valid.
    unsafe { max_roundtrip_error(curve, &raw const again) }
}

unsafe fn fixup_tf(tf: *mut Tf) -> bool {
    // SAFETY: fitting supplies initialized original TF fields and writable b/e.
    unsafe {
        let a = ptr::addr_of!((*tf).a).read();
        if a < 0.0 {
            return false;
        }
        let d = ptr::addr_of!((*tf).d).read();
        let b = ptr::addr_of_mut!((*tf).b);
        if madd(a, d, b.read()) < 0.0 {
            b.write(-a * d);
        }
        let e = ptr::addr_of_mut!((*tf).e);
        e.write(
            madd(
                ptr::addr_of!((*tf).c).read(),
                d,
                ptr::addr_of!((*tf).f).read(),
            ) - powf(madd(a, d, b.read()), ptr::addr_of!((*tf).g).read()),
        );
        finite(e.read())
    }
}

/// Original eight fitting iterations, retaining best-TF restoration checkpoints.
///
/// # Safety
/// TF is initialized and writable; curve/table storage and all reached math
/// satisfy the original fitting call contract. N-1, N-L and sample arithmetic
/// must be source-defined and borrowed tables must remain live throughout.
pub unsafe fn fit_nonlinear(curve: *const Curve, l: i32, n: i32, tf: *mut Tf) -> bool {
    // SAFETY: caller provides initialized source fields and writable e/b.
    if !unsafe { fixup_tf(tf) } {
        return false;
    }
    let dx = 1.0 / (n - 1) as f32;
    // SAFETY: fixup has established all initialized TF words before the copy.
    let mut best_tf = unsafe { tf.read() };
    let mut best_error = f32::INFINITY;
    // SAFETY: the same source sampling preconditions hold for this inverse.
    let initial = unsafe { max_roundtrip_error_checked(curve, tf) };
    if initial < best_error {
        best_error = initial;
        best_tf = unsafe { tf.read() };
    }
    for _ in 0..8 {
        // SAFETY: each source iteration operates on the same live TF and curve.
        if !unsafe { gauss_newton_step(curve, tf, l as f32 * dx, dx, n - l) }
            || !unsafe { fixup_tf(tf) }
        {
            // SAFETY: the original failure path restores the fully initialized
            // saved TF before returning the finite-best-error predicate.
            unsafe { tf.write(best_tf) };
            return finite(best_error);
        }
        // SAFETY: source curve storage and initialized TF remain live here.
        let error = unsafe { max_roundtrip_error_checked(curve, tf) };
        if error < best_error {
            best_error = error;
            best_tf = unsafe { tf.read() };
        }
    }
    // SAFETY: exact final source restoration to the saved initialized TF.
    unsafe { tf.write(best_tf) };
    finite(best_error)
}

/// Original public ApproximateCurve. Outputs retain source partial-write order.
///
/// # Safety
/// Non-null curve has an initialized count and, when admitted, valid borrowed
/// table storage. Non-null output pointers are writable at each reached source
/// write. Reached aliases must remain source-defined, including subsequent
/// curve reads. No output initialization is required before its first write.
pub unsafe fn approximate_curve(curve: *const Curve, approx: *mut Tf, max_error: *mut f32) -> bool {
    if curve.is_null() || approx.is_null() || max_error.is_null() {
        return false;
    }
    // SAFETY: source reads only the initialized count before these guards.
    let entries = unsafe { curve.cast::<u32>().read() };
    if entries == 0 || entries == 1 || entries > i32::MAX as u32 {
        return false;
    }
    let n = entries as i32;
    let dx = 1.0 / (n - 1) as f32;
    // SAFETY: the original admits the writable max_error output here.
    unsafe { max_error.write(f32::INFINITY) };
    for tolerance in [1.5 / 65535.0, 1.0 / 512.0] {
        // Private initialization adds no external writes. fit_linear writes c/d
        // for these valid tables; each following branch initializes the rest.
        let mut tf = Tf::default();
        // SAFETY: source table has at least two readable entries, and these
        // local c/d outputs are distinct writable fields. Null F forces zero.
        let l = unsafe {
            fit_linear(
                curve,
                n,
                tolerance,
                &raw mut tf.c,
                &raw mut tf.d,
                ptr::null_mut(),
            )
        };
        let mut inverse = Tf::default();
        if l == n {
            tf.g = 1.0;
            tf.a = tf.c;
            tf.b = tf.f;
            tf.c = 0.0;
            tf.d = 0.0;
            tf.e = 0.0;
            tf.f = 0.0;
        } else if l == n - 1 {
            tf.g = 1.0;
            // SAFETY: both final source table samples are within its extent.
            tf.a = unsafe {
                (eval_curve(curve, (n - 1) as f32 * dx) - eval_curve(curve, (n - 2) as f32 * dx))
                    / dx
            };
            // SAFETY: the same reached source sample is readable.
            let at_previous = unsafe { eval_curve(curve, (n - 2) as f32 * dx) };
            #[cfg(target_arch = "aarch64")]
            {
                // The original archive's degenerate branch rounds a*index,
                // then contracts its final scaled subtraction with FMADD.
                // Keep that checkpoint rather than rounding both products.
                tf.b = (-tf.a * (n - 2) as f32).mul_add(dx, at_previous);
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                tf.b = at_previous - tf.a * (n - 2) as f32 * dx;
            }
            tf.e = 0.0;
        } else {
            let middle = (l + n) / 2;
            let middle_x = middle as f32 / (n - 1) as f32;
            // SAFETY: reached midpoint lies within the source table.
            let middle_y = unsafe { eval_curve(curve, middle_x) };
            tf.g = log2f(middle_y) / log2f(middle_x);
            tf.a = 1.0;
            tf.b = 0.0;
            tf.e = madd(tf.c, tf.d, tf.f) - powf(madd(tf.a, tf.d, tf.b), tf.g);
            // SAFETY: curve and initialized inverse satisfy the original fit.
            if !tf.invert_into(&mut inverse)
                || !unsafe { fit_nonlinear(curve, l, n, &raw mut inverse) }
            {
                continue;
            }
            if !inverse.invert_into(&mut tf) {
                continue;
            }
        }
        if tf.get_type() != TfType::Srgbish || !tf.invert_into(&mut inverse) {
            continue;
        }
        // SAFETY: table and initialized inverse satisfy the source sampling contract.
        let error = unsafe { max_roundtrip_error(curve, &raw const inverse) };
        // SAFETY: reread the actual caller slot after possible prior approx
        // alias writes, then preserve max_error-before-approx assignment order.
        if unsafe { max_error.read() } > error {
            unsafe {
                max_error.write(error);
                approx.write(tf);
            }
        }
    }
    // SAFETY: the source initialized max_error on the admitted path.
    finite(unsafe { max_error.read() })
}
