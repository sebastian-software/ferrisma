// Copyright 2023 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/colrconvert.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifColorPrimariesGetValues(
        acp: avifColorPrimaries,
        outPrimaries: *mut core::ffi::c_float,
    );
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabsf(__x: core::ffi::c_float) -> core::ffi::c_float;
}
pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type avifBool = core::ffi::c_int;
pub type avifColorPrimaries = uint16_t;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
static mut epsilon: core::ffi::c_double = 1e-12f64;
unsafe extern "C" fn avifXyToXYZ(
    mut xy: *const core::ffi::c_float,
    mut XYZ: *mut core::ffi::c_double,
) -> avifBool {
    if (fabsf(*xy.offset(1 as core::ffi::c_int as isize)) as core::ffi::c_double)
        < epsilon
    {
        return AVIF_FALSE;
    }
    let factor: core::ffi::c_double = 1.0f64
        / *xy.offset(1 as core::ffi::c_int as isize) as core::ffi::c_double;
    *XYZ.offset(0 as core::ffi::c_int as isize) = *xy
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_double * factor;
    *XYZ.offset(1 as core::ffi::c_int as isize) = 1 as core::ffi::c_int
        as core::ffi::c_double;
    *XYZ.offset(2 as core::ffi::c_int as isize) = (1 as core::ffi::c_int
        as core::ffi::c_float - *xy.offset(0 as core::ffi::c_int as isize)
        - *xy.offset(1 as core::ffi::c_int as isize)) as core::ffi::c_double * factor;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifMatInv(
    mut M: *mut [core::ffi::c_double; 3],
    mut I: *mut [core::ffi::c_double; 3],
) -> avifBool {
    let mut det: core::ffi::c_double = (*M
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * ((*M.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*M.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            - (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
                * (*M
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[2 as core::ffi::c_int as usize])
        - (*M.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * ((*M
                .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                * (*M
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[2 as core::ffi::c_int as usize]
                - (*M
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[2 as core::ffi::c_int as usize]
                    * (*M
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize])
        + (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * ((*M
                .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                * (*M
                    .offset(
                        2 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize]
                - (*M
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize]
                    * (*M
                        .offset(
                            2 as core::ffi::c_int as isize,
                        ))[0 as core::ffi::c_int as usize]);
    if fabs(det) < epsilon {
        return AVIF_FALSE;
    }
    det = 1.0f64 / det;
    (*I.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ((*M
        .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        * (*M.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        - (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*M
                .offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize])
        * det;
    (*I.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ((*M
        .offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        * (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        - (*M.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*M
                .offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize])
        * det;
    (*I.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = ((*M
        .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        * (*M.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        - (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*M
                .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize])
        * det;
    (*I.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ((*M
        .offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        * (*M.offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        - (*M.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            * (*M
                .offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize])
        * det;
    (*I.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ((*M
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*M.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        - (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*M
                .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize])
        * det;
    (*I.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = ((*M
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        - (*M.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            * (*M
                .offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize])
        * det;
    (*I.offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = ((*M
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        - (*M.offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            * (*M
                .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize])
        * det;
    (*I.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = ((*M
        .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*M.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        - (*M.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            * (*M
                .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize])
        * det;
    (*I.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = ((*M
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*M.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        - (*M.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            * (*M
                .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize])
        * det;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifMatMul(
    mut A: *mut [core::ffi::c_double; 3],
    mut B: *mut [core::ffi::c_double; 3],
    mut C: *mut [core::ffi::c_double; 3],
) {
    (*C.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = (*A
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize];
    (*C.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = (*A
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
    (*C.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = (*A
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize];
    (*C.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = (*A
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize];
    (*C.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = (*A
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
    (*C.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = (*A
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize];
    (*C.offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = (*A
        .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize];
    (*C.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = (*A
        .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
    (*C.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = (*A
        .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * (*B.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * (*B.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
        + (*A.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * (*B
                .offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize];
}
unsafe extern "C" fn avifMatDiag(
    mut d: *const core::ffi::c_double,
    mut M: *mut [core::ffi::c_double; 3],
) {
    (*M.offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = *d
        .offset(0 as core::ffi::c_int as isize);
    (*M.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = *d
        .offset(1 as core::ffi::c_int as isize);
    (*M.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = 0
        as core::ffi::c_int as core::ffi::c_double;
    (*M.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = *d
        .offset(2 as core::ffi::c_int as isize);
}
unsafe extern "C" fn avifVecMul(
    mut M: *mut [core::ffi::c_double; 3],
    mut x: *const core::ffi::c_double,
    mut y: *mut core::ffi::c_double,
) {
    *y.offset(0 as core::ffi::c_int as isize) = (*M
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * *x.offset(0 as core::ffi::c_int as isize)
        + (*M.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * *x.offset(1 as core::ffi::c_int as isize)
        + (*M.offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * *x.offset(2 as core::ffi::c_int as isize);
    *y.offset(1 as core::ffi::c_int as isize) = (*M
        .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * *x.offset(0 as core::ffi::c_int as isize)
        + (*M.offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * *x.offset(1 as core::ffi::c_int as isize)
        + (*M.offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * *x.offset(2 as core::ffi::c_int as isize);
    *y.offset(2 as core::ffi::c_int as isize) = (*M
        .offset(2 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
        * *x.offset(0 as core::ffi::c_int as isize)
        + (*M.offset(2 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            * *x.offset(1 as core::ffi::c_int as isize)
        + (*M.offset(2 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
            * *x.offset(2 as core::ffi::c_int as isize);
}
static mut avifBradford: [[core::ffi::c_double; 3]; 3] = [
    [0.8951f64, 0.2664f64, -0.1614f64],
    [-0.7502f64, 1.7135f64, 0.0367f64],
    [0.0389f64, -0.0685f64, 1.0296f64],
];
static mut avifLmsD50: [core::ffi::c_double; 3] = [0.996284f64, 1.02043f64, 0.818644f64];
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesComputeRGBToXYZD50Matrix(
    mut colorPrimaries: avifColorPrimaries,
    mut coeffs: *mut [core::ffi::c_double; 3],
) -> avifBool {
    let mut primaries: [core::ffi::c_float; 8] = [0.; 8];
    avifColorPrimariesGetValues(colorPrimaries, primaries.as_mut_ptr());
    let mut whitePointXYZ: [core::ffi::c_double; 3] = [0.; 3];
    if avifXyToXYZ(
        &mut *primaries.as_mut_ptr().offset(6 as core::ffi::c_int as isize)
            as *mut core::ffi::c_float as *const core::ffi::c_float,
        whitePointXYZ.as_mut_ptr(),
    ) == 0
    {
        return AVIF_FALSE;
    }
    let mut rgbPrimaries: [[core::ffi::c_double; 3]; 3] = [
        [
            primaries[0 as core::ffi::c_int as usize] as core::ffi::c_double,
            primaries[2 as core::ffi::c_int as usize] as core::ffi::c_double,
            primaries[4 as core::ffi::c_int as usize] as core::ffi::c_double,
        ],
        [
            primaries[1 as core::ffi::c_int as usize] as core::ffi::c_double,
            primaries[3 as core::ffi::c_int as usize] as core::ffi::c_double,
            primaries[5 as core::ffi::c_int as usize] as core::ffi::c_double,
        ],
        [
            1.0f64 - primaries[0 as core::ffi::c_int as usize] as core::ffi::c_double
                - primaries[1 as core::ffi::c_int as usize] as core::ffi::c_double,
            1.0f64 - primaries[2 as core::ffi::c_int as usize] as core::ffi::c_double
                - primaries[3 as core::ffi::c_int as usize] as core::ffi::c_double,
            1.0f64 - primaries[4 as core::ffi::c_int as usize] as core::ffi::c_double
                - primaries[5 as core::ffi::c_int as usize] as core::ffi::c_double,
        ],
    ];
    let mut rgbPrimariesInv: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifMatInv(rgbPrimaries.as_mut_ptr(), rgbPrimariesInv.as_mut_ptr()) == 0 {
        return AVIF_FALSE;
    }
    let mut rgbCoefficients: [core::ffi::c_double; 3] = [0.; 3];
    avifVecMul(
        rgbPrimariesInv.as_mut_ptr(),
        whitePointXYZ.as_mut_ptr() as *const core::ffi::c_double,
        rgbCoefficients.as_mut_ptr(),
    );
    let mut rgbCoefficientsMat: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    avifMatDiag(
        rgbCoefficients.as_mut_ptr() as *const core::ffi::c_double,
        rgbCoefficientsMat.as_mut_ptr(),
    );
    let mut rgbXYZ: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    avifMatMul(
        rgbPrimaries.as_mut_ptr(),
        rgbCoefficientsMat.as_mut_ptr(),
        rgbXYZ.as_mut_ptr(),
    );
    let mut lms: [core::ffi::c_double; 3] = [0.; 3];
    avifVecMul(
        core::ptr::addr_of_mut!(avifBradford).cast::<[core::ffi::c_double; 3]>(),
        whitePointXYZ.as_mut_ptr() as *const core::ffi::c_double,
        lms.as_mut_ptr(),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        if fabs(lms[i as usize]) < epsilon {
            return AVIF_FALSE;
        }
        lms[i as usize] = avifLmsD50[i as usize] / lms[i as usize];
        i += 1;
    }
    let mut adaptation: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    avifMatDiag(lms.as_mut_ptr() as *const core::ffi::c_double, adaptation.as_mut_ptr());
    let mut tmp: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    avifMatMul(adaptation.as_mut_ptr(), core::ptr::addr_of_mut!(avifBradford).cast::<[core::ffi::c_double; 3]>(), tmp.as_mut_ptr());
    let mut bradfordInv: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifMatInv(core::ptr::addr_of_mut!(avifBradford).cast::<[core::ffi::c_double; 3]>(), bradfordInv.as_mut_ptr()) == 0 {
        return AVIF_FALSE;
    }
    avifMatMul(bradfordInv.as_mut_ptr(), tmp.as_mut_ptr(), adaptation.as_mut_ptr());
    avifMatMul(adaptation.as_mut_ptr(), rgbXYZ.as_mut_ptr(), coeffs);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesComputeXYZD50ToRGBMatrix(
    mut colorPrimaries: avifColorPrimaries,
    mut coeffs: *mut [core::ffi::c_double; 3],
) -> avifBool {
    let mut rgbToXyz: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifColorPrimariesComputeRGBToXYZD50Matrix(colorPrimaries, rgbToXyz.as_mut_ptr())
        == 0
    {
        return AVIF_FALSE;
    }
    if avifMatInv(rgbToXyz.as_mut_ptr(), coeffs) == 0 {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesComputeRGBToRGBMatrix(
    mut srcColorPrimaries: avifColorPrimaries,
    mut dstColorPrimaries: avifColorPrimaries,
    mut coeffs: *mut [core::ffi::c_double; 3],
) -> avifBool {
    let mut srcRGBToXYZ: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifColorPrimariesComputeRGBToXYZD50Matrix(
        srcColorPrimaries,
        srcRGBToXYZ.as_mut_ptr(),
    ) == 0
    {
        return AVIF_FALSE;
    }
    let mut xyzToDstRGB: [[core::ffi::c_double; 3]; 3] = [[0.; 3]; 3];
    if avifColorPrimariesComputeXYZD50ToRGBMatrix(
        dstColorPrimaries,
        xyzToDstRGB.as_mut_ptr(),
    ) == 0
    {
        return AVIF_FALSE;
    }
    avifMatMul(xyzToDstRGB.as_mut_ptr(), srcRGBToXYZ.as_mut_ptr(), coeffs);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifLinearRGBConvertColorSpace(
    mut rgb: *mut core::ffi::c_float,
    mut coeffs: *mut [core::ffi::c_double; 3],
) {
    let rgbDouble: [core::ffi::c_double; 3] = [
        *rgb.offset(0 as core::ffi::c_int as isize) as core::ffi::c_double,
        *rgb.offset(1 as core::ffi::c_int as isize) as core::ffi::c_double,
        *rgb.offset(2 as core::ffi::c_int as isize) as core::ffi::c_double,
    ];
    let mut converted: [core::ffi::c_double; 3] = [0.; 3];
    avifVecMul(coeffs, rgbDouble.as_ptr(), converted.as_mut_ptr());
    *rgb.offset(0 as core::ffi::c_int as isize) = converted[0 as core::ffi::c_int
        as usize] as core::ffi::c_float;
    *rgb.offset(1 as core::ffi::c_int as isize) = converted[1 as core::ffi::c_int
        as usize] as core::ffi::c_float;
    *rgb.offset(2 as core::ffi::c_int as isize) = converted[2 as core::ffi::c_int
        as usize] as core::ffi::c_float;
}
