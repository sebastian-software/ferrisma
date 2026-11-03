// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Observation adapter using the actual Rust library. No fitting algorithm.
use jxl::skcms::{curve::Curve, fitting, transfer_function::TransferFunction as Tf};
use std::fmt::Write as _;
use std::ptr;

fn number<T: std::str::FromStr>(a: &mut std::str::SplitWhitespace<'_>) -> T {
    a.next()
        .expect("protocol argument")
        .parse()
        .ok()
        .expect("number")
}
fn float(a: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(a))
}
fn tf(a: &mut std::str::SplitWhitespace<'_>) -> Tf {
    Tf {
        g: float(a),
        a: float(a),
        b: float(a),
        c: float(a),
        d: float(a),
        e: float(a),
        f: float(a),
    }
}
fn field(t: *mut Tf, i: i32) -> *mut f32 {
    // SAFETY: only the pointer is projected; callers supply a live TF.
    unsafe {
        match i {
            0 => ptr::addr_of_mut!((*t).g),
            1 => ptr::addr_of_mut!((*t).a),
            2 => ptr::addr_of_mut!((*t).b),
            3 => ptr::addr_of_mut!((*t).c),
            4 => ptr::addr_of_mut!((*t).d),
            5 => ptr::addr_of_mut!((*t).e),
            6 => ptr::addr_of_mut!((*t).f),
            _ => panic!("field"),
        }
    }
}
pub fn observe(line: &str) -> String {
    let mut a = line.split_whitespace();
    let command = a.next().unwrap();
    let mode: u32 = number(&mut a);
    let count: u32 = number(&mut a);
    let physical: usize = number(&mut a);
    let flags: u32 = number(&mut a);
    let alias: i32 = number(&mut a);
    let x0 = float(&mut a);
    let dx = float(&mut a);
    let l: i32 = number(&mut a);
    let n: i32 = number(&mut a);
    let mut error = float(&mut a);
    let mut t = tf(&mut a);
    let mut curve = Curve::zeroed();
    let mut table = Vec::new();
    if mode == 0 {
        curve = Curve::from_parametric(tf(&mut a));
    } else {
        for _ in 0..physical {
            let v: u16 = number(&mut a);
            if mode == 8 {
                table.push(v as u8);
            } else {
                table.extend_from_slice(&v.to_be_bytes());
            }
        }
        // SAFETY: these are fields of the live initialized union backing.
        // The vector stays live and unmoved through every reached read.
        unsafe {
            ptr::addr_of_mut!(curve.table.table_8).write(if mode == 8 {
                table.as_ptr()
            } else {
                ptr::null()
            });
            ptr::addr_of_mut!(curve.table.table_16).write(if mode == 16 {
                table.as_ptr()
            } else {
                ptr::null()
            });
        }
    }
    // SAFETY: prefix is initialized in both canonical source branches.
    unsafe { (&raw mut curve).cast::<u32>().write(count) };
    let error_ptr = if alias < 0 {
        &raw mut error
    } else {
        field(&raw mut t, alias)
    };
    let mut gradient = [error, x0, dx];
    // SAFETY: the external corpus establishes source defined table/math
    // domains. Deliberate nulls reach only original early-return guards;
    // aliased max_error points to exactly one actual live TF field.
    let (status, result) = unsafe {
        match command {
            "skcms-fit-approx" => (
                u8::from(fitting::approximate_curve(
                    if flags & 1 != 0 {
                        ptr::null()
                    } else {
                        &raw const curve
                    },
                    if flags & 2 != 0 {
                        ptr::null_mut()
                    } else {
                        &raw mut t
                    },
                    if flags & 4 != 0 {
                        ptr::null_mut()
                    } else {
                        error_ptr
                    },
                ))
                .to_string(),
                error_ptr.read(),
            ),
            "skcms-fit-roundtrip" => (
                u8::from(fitting::are_approximate_inverses(
                    &raw const curve,
                    &raw const t,
                ))
                .to_string(),
                fitting::max_roundtrip_error(&raw const curve, &raw const t),
            ),
            "skcms-fit-gradient" => (
                "V".to_owned(),
                fitting::rg_nonlinear(x0, &raw const curve, &raw const t, gradient.as_mut_ptr()),
            ),
            "skcms-fit-step" => (
                u8::from(fitting::gauss_newton_step(
                    &raw const curve,
                    &raw mut t,
                    x0,
                    dx,
                    n,
                ))
                .to_string(),
                error,
            ),
            "skcms-fit-nonlinear" => (
                u8::from(fitting::fit_nonlinear(&raw const curve, l, n, &raw mut t)).to_string(),
                error,
            ),
            "skcms-fit-checked" => (
                "V".to_owned(),
                fitting::max_roundtrip_error_checked(&raw const curve, &raw const t),
            ),
            _ => panic!("fitting command"),
        }
    };
    let mut output = format!("{} {}", status, result.to_bits());
    for x in [t.g, t.a, t.b, t.c, t.d, t.e, t.f] {
        write!(output, " {}", x.to_bits()).unwrap();
    }
    if command == "skcms-fit-gradient" {
        for x in gradient {
            write!(output, " {}", x.to_bits()).unwrap();
        }
    }
    assert!(a.next().is_none());
    output
}
