/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/flow_estimation/ransac.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static kIdentityParams: [core::ffi::c_double; 6];
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_void,
        *const core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type TransformationType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Correspondence {
    pub x: core::ffi::c_double,
    pub y: core::ffi::c_double,
    pub rx: core::ffi::c_double,
    pub ry: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RansacModelInfo {
    pub find_transformation: FindTransformationFunc,
    pub score_model: ScoreModelFunc,
    pub minpts: core::ffi::c_int,
}
pub type ScoreModelFunc = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_double,
        *const Correspondence,
        core::ffi::c_int,
        *mut RANSAC_MOTION,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RANSAC_MOTION {
    pub num_inliers: core::ffi::c_int,
    pub sse: core::ffi::c_double,
    pub inlier_indices: *mut core::ffi::c_int,
}
pub type FindTransformationFunc = Option<
    unsafe extern "C-unwind" fn(
        *const Correspondence,
        *const core::ffi::c_int,
        core::ffi::c_int,
        *mut core::ffi::c_double,
    ) -> bool,
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_PARAMDIM: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MINPTS_MULTIPLIER: core::ffi::c_int = 5 as core::ffi::c_int;
pub const INLIER_THRESHOLD: core::ffi::c_double = 1.25f64;
pub const INLIER_THRESHOLD_SQUARED: core::ffi::c_double = INLIER_THRESHOLD
    * INLIER_THRESHOLD;
pub const NUM_TRIALS: core::ffi::c_int = 20 as core::ffi::c_int;
pub const NUM_REFINES: core::ffi::c_int = 5 as core::ffi::c_int;
unsafe extern "C-unwind" fn score_affine(
    mut mat: *const core::ffi::c_double,
    mut points: *const Correspondence,
    mut num_points: core::ffi::c_int,
    mut model: *mut RANSAC_MOTION,
) {
    (*model).num_inliers = 0 as core::ffi::c_int;
    (*model).sse = 0.0f64;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_points {
        let x1: core::ffi::c_double = (*points.offset(i as isize)).x;
        let y1: core::ffi::c_double = (*points.offset(i as isize)).y;
        let x2: core::ffi::c_double = (*points.offset(i as isize)).rx;
        let y2: core::ffi::c_double = (*points.offset(i as isize)).ry;
        let proj_x: core::ffi::c_double = *mat.offset(2 as core::ffi::c_int as isize)
            * x1 + *mat.offset(3 as core::ffi::c_int as isize) * y1
            + *mat.offset(0 as core::ffi::c_int as isize);
        let proj_y: core::ffi::c_double = *mat.offset(4 as core::ffi::c_int as isize)
            * x1 + *mat.offset(5 as core::ffi::c_int as isize) * y1
            + *mat.offset(1 as core::ffi::c_int as isize);
        let dx: core::ffi::c_double = proj_x - x2;
        let dy: core::ffi::c_double = proj_y - y2;
        let sse: core::ffi::c_double = dx * dx + dy * dy;
        if sse < INLIER_THRESHOLD_SQUARED {
            let fresh0 = (*model).num_inliers;
            (*model).num_inliers = (*model).num_inliers + 1;
            *((*model).inlier_indices).offset(fresh0 as isize) = i;
            (*model).sse += sse;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn find_rotzoom(
    mut points: *const Correspondence,
    mut indices: *const core::ffi::c_int,
    mut num_indices: core::ffi::c_int,
    mut params: *mut core::ffi::c_double,
) -> bool {
    let n: core::ffi::c_int = 4 as core::ffi::c_int;
    let mut mat: [core::ffi::c_double; 16] = [0.; 16];
    let mut y: [core::ffi::c_double; 4] = [0.; 4];
    let mut a: [core::ffi::c_double; 4] = [0.; 4];
    let mut b: core::ffi::c_double = 0.;
    least_squares_init(mat.as_mut_ptr(), y.as_mut_ptr(), n);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_indices {
        let mut index: core::ffi::c_int = *indices.offset(i as isize);
        let sx: core::ffi::c_double = (*points.offset(index as isize)).x;
        let sy: core::ffi::c_double = (*points.offset(index as isize)).y;
        let dx: core::ffi::c_double = (*points.offset(index as isize)).rx;
        let dy: core::ffi::c_double = (*points.offset(index as isize)).ry;
        a[0 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as core::ffi::c_double;
        a[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as core::ffi::c_double;
        a[2 as core::ffi::c_int as usize] = sx;
        a[3 as core::ffi::c_int as usize] = sy;
        b = dx;
        least_squares_accumulate(mat.as_mut_ptr(), y.as_mut_ptr(), a.as_mut_ptr(), b, n);
        a[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as core::ffi::c_double;
        a[1 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as core::ffi::c_double;
        a[2 as core::ffi::c_int as usize] = sy;
        a[3 as core::ffi::c_int as usize] = -sx;
        b = dy;
        least_squares_accumulate(mat.as_mut_ptr(), y.as_mut_ptr(), a.as_mut_ptr(), b, n);
        i += 1;
    }
    if least_squares_solve(mat.as_mut_ptr(), y.as_mut_ptr(), params, n) == 0 {
        return false_0 != 0;
    }
    *params.offset(4 as core::ffi::c_int as isize) = -*params
        .offset(3 as core::ffi::c_int as isize);
    *params.offset(5 as core::ffi::c_int as isize) = *params
        .offset(2 as core::ffi::c_int as isize);
    return true_0 != 0;
}
unsafe extern "C-unwind" fn find_affine(
    mut points: *const Correspondence,
    mut indices: *const core::ffi::c_int,
    mut num_indices: core::ffi::c_int,
    mut params: *mut core::ffi::c_double,
) -> bool {
    let n: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut mat: [[core::ffi::c_double; 9]; 2] = [[0.; 9]; 2];
    let mut y: [[core::ffi::c_double; 3]; 2] = [[0.; 3]; 2];
    let mut x: [[core::ffi::c_double; 3]; 2] = [[0.; 3]; 2];
    let mut a: [[core::ffi::c_double; 3]; 2] = [[0.; 3]; 2];
    let mut b: [core::ffi::c_double; 2] = [0.; 2];
    least_squares_init(
        (mat[0 as core::ffi::c_int as usize]).as_mut_ptr(),
        (y[0 as core::ffi::c_int as usize]).as_mut_ptr(),
        n,
    );
    least_squares_init(
        (mat[1 as core::ffi::c_int as usize]).as_mut_ptr(),
        (y[1 as core::ffi::c_int as usize]).as_mut_ptr(),
        n,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_indices {
        let mut index: core::ffi::c_int = *indices.offset(i as isize);
        let sx: core::ffi::c_double = (*points.offset(index as isize)).x;
        let sy: core::ffi::c_double = (*points.offset(index as isize)).y;
        let dx: core::ffi::c_double = (*points.offset(index as isize)).rx;
        let dy: core::ffi::c_double = (*points.offset(index as isize)).ry;
        a[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 1
            as core::ffi::c_int as core::ffi::c_double;
        a[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = sx;
        a[0 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = sy;
        b[0 as core::ffi::c_int as usize] = dx;
        least_squares_accumulate(
            (mat[0 as core::ffi::c_int as usize]).as_mut_ptr(),
            (y[0 as core::ffi::c_int as usize]).as_mut_ptr(),
            (a[0 as core::ffi::c_int as usize]).as_mut_ptr(),
            b[0 as core::ffi::c_int as usize],
            n,
        );
        a[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 1
            as core::ffi::c_int as core::ffi::c_double;
        a[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = sx;
        a[1 as core::ffi::c_int as usize][2 as core::ffi::c_int as usize] = sy;
        b[1 as core::ffi::c_int as usize] = dy;
        least_squares_accumulate(
            (mat[1 as core::ffi::c_int as usize]).as_mut_ptr(),
            (y[1 as core::ffi::c_int as usize]).as_mut_ptr(),
            (a[1 as core::ffi::c_int as usize]).as_mut_ptr(),
            b[1 as core::ffi::c_int as usize],
            n,
        );
        i += 1;
    }
    if least_squares_solve(
        (mat[0 as core::ffi::c_int as usize]).as_mut_ptr(),
        (y[0 as core::ffi::c_int as usize]).as_mut_ptr(),
        (x[0 as core::ffi::c_int as usize]).as_mut_ptr(),
        n,
    ) == 0
    {
        return false_0 != 0;
    }
    if least_squares_solve(
        (mat[1 as core::ffi::c_int as usize]).as_mut_ptr(),
        (y[1 as core::ffi::c_int as usize]).as_mut_ptr(),
        (x[1 as core::ffi::c_int as usize]).as_mut_ptr(),
        n,
    ) == 0
    {
        return false_0 != 0;
    }
    *params.offset(0 as core::ffi::c_int as isize) = x[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize];
    *params.offset(1 as core::ffi::c_int as isize) = x[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize];
    *params.offset(2 as core::ffi::c_int as isize) = x[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize];
    *params.offset(3 as core::ffi::c_int as isize) = x[0 as core::ffi::c_int
        as usize][2 as core::ffi::c_int as usize];
    *params.offset(4 as core::ffi::c_int as isize) = x[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize];
    *params.offset(5 as core::ffi::c_int as isize) = x[1 as core::ffi::c_int
        as usize][2 as core::ffi::c_int as usize];
    return true_0 != 0;
}
unsafe extern "C-unwind" fn compare_motions(
    mut arg_a: *const core::ffi::c_void,
    mut arg_b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let mut motion_a: *const RANSAC_MOTION = arg_a as *mut RANSAC_MOTION;
    let mut motion_b: *const RANSAC_MOTION = arg_b as *mut RANSAC_MOTION;
    if (*motion_a).num_inliers > (*motion_b).num_inliers {
        return -(1 as core::ffi::c_int);
    }
    if (*motion_a).num_inliers < (*motion_b).num_inliers {
        return 1 as core::ffi::c_int;
    }
    if (*motion_a).sse < (*motion_b).sse {
        return -(1 as core::ffi::c_int);
    }
    if (*motion_a).sse > (*motion_b).sse {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_better_motion(
    mut motion_a: *const RANSAC_MOTION,
    mut motion_b: *const RANSAC_MOTION,
) -> bool {
    return compare_motions(
        motion_a as *const core::ffi::c_void,
        motion_b as *const core::ffi::c_void,
    ) < 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn ransac_internal(
    mut matched_points: *const Correspondence,
    mut npoints: core::ffi::c_int,
    mut motion_models: *mut MotionModel,
    mut num_desired_motions: core::ffi::c_int,
    mut model_info: *const RansacModelInfo,
    mut mem_alloc_failed: *mut bool,
) -> bool {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut minpts: core::ffi::c_int = (*model_info).minpts;
    let mut ret_val: bool = true_0 != 0;
    let mut seed: core::ffi::c_uint = npoints as core::ffi::c_uint;
    let mut indices: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let mut motions: *mut RANSAC_MOTION = 0 as *mut RANSAC_MOTION;
    let mut worst_kept_motion: *mut RANSAC_MOTION = 0 as *mut RANSAC_MOTION;
    let mut current_motion: RANSAC_MOTION = RANSAC_MOTION {
        num_inliers: 0,
        sse: 0.,
        inlier_indices: 0 as *mut core::ffi::c_int,
    };
    let mut params_this_motion: [core::ffi::c_double; 6] = [0.; 6];
    i = 0 as core::ffi::c_int;
    while i < num_desired_motions {
        memcpy(
            ((*motion_models.offset(i as isize)).params).as_mut_ptr()
                as *mut core::ffi::c_void,
            kIdentityParams.as_ptr() as *const core::ffi::c_void,
            (MAX_PARAMDIM as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        );
        (*motion_models.offset(i as isize)).num_inliers = 0 as core::ffi::c_int;
        i += 1;
    }
    if npoints < minpts * MINPTS_MULTIPLIER || npoints == 0 as core::ffi::c_int {
        return false_0 != 0;
    }
    let mut min_inliers: core::ffi::c_int = if (0.1f64 * npoints as core::ffi::c_double)
        as core::ffi::c_int > minpts
    {
        (0.1f64 * npoints as core::ffi::c_double) as core::ffi::c_int
    } else {
        minpts
    };
    motions = aom_calloc(
        num_desired_motions as size_t,
        ::core::mem::size_of::<RANSAC_MOTION>() as size_t,
    ) as *mut RANSAC_MOTION;
    let mut inlier_buffer: *mut core::ffi::c_int = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(npoints as size_t)
            .wrapping_mul((num_desired_motions + 1 as core::ffi::c_int) as size_t),
    ) as *mut core::ffi::c_int;
    if !(!motions.is_null() && !inlier_buffer.is_null()) {
        ret_val = false_0 != 0;
        *mem_alloc_failed = true_0 != 0;
    } else {
        worst_kept_motion = motions;
        i = 0 as core::ffi::c_int;
        while i < num_desired_motions {
            let fresh1 = &raw mut (*motions.offset(i as isize)).inlier_indices;
            *fresh1 = inlier_buffer.offset((i * npoints) as isize);
            i += 1;
        }
        memset(
            &raw mut current_motion as *mut RANSAC_MOTION as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<RANSAC_MOTION>() as size_t,
        );
        current_motion.inlier_indices = inlier_buffer
            .offset((num_desired_motions * npoints) as isize);
        let mut trial_count: core::ffi::c_int = 0 as core::ffi::c_int;
        while trial_count < NUM_TRIALS {
            lcg_pick(npoints, minpts, indices.as_mut_ptr(), &raw mut seed);
            if ((*model_info).find_transformation)
                .expect(
                    "non-null function pointer",
                )(
                matched_points,
                indices.as_mut_ptr(),
                minpts,
                params_this_motion.as_mut_ptr(),
            )
            {
                ((*model_info).score_model)
                    .expect(
                        "non-null function pointer",
                    )(
                    params_this_motion.as_mut_ptr(),
                    matched_points,
                    npoints,
                    &raw mut current_motion,
                );
                if !(current_motion.num_inliers < min_inliers) {
                    if is_better_motion(&raw mut current_motion, worst_kept_motion) {
                        (*worst_kept_motion).num_inliers = current_motion.num_inliers;
                        (*worst_kept_motion).sse = current_motion.sse;
                        let mut tmp: *mut core::ffi::c_int = (*worst_kept_motion)
                            .inlier_indices;
                        (*worst_kept_motion).inlier_indices = current_motion
                            .inlier_indices;
                        current_motion.inlier_indices = tmp;
                        i = 0 as core::ffi::c_int;
                        while i < num_desired_motions {
                            if is_better_motion(
                                worst_kept_motion,
                                &raw mut *motions.offset(i as isize),
                            ) {
                                worst_kept_motion = &raw mut *motions.offset(i as isize)
                                    as *mut RANSAC_MOTION;
                            }
                            i += 1;
                        }
                    }
                }
            }
            trial_count += 1;
        }
        qsort(
            motions as *mut core::ffi::c_void,
            num_desired_motions as size_t,
            ::core::mem::size_of::<RANSAC_MOTION>() as size_t,
            Some(
                compare_motions
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_void,
                        *const core::ffi::c_void,
                    ) -> core::ffi::c_int,
            ),
        );
        i = 0 as core::ffi::c_int;
        while i < num_desired_motions {
            if !((*motions.offset(i as isize)).num_inliers <= 0 as core::ffi::c_int) {
                let mut bad_model: bool = false_0 != 0;
                let mut refine_count: core::ffi::c_int = 0 as core::ffi::c_int;
                while refine_count < NUM_REFINES {
                    let mut num_inliers: core::ffi::c_int = (*motions.offset(i as isize))
                        .num_inliers;
                    if !((*model_info).find_transformation)
                        .expect(
                            "non-null function pointer",
                        )(
                        matched_points,
                        (*motions.offset(i as isize)).inlier_indices,
                        num_inliers,
                        params_this_motion.as_mut_ptr(),
                    )
                    {
                        bad_model = true_0 != 0;
                        break;
                    } else {
                        ((*model_info).score_model)
                            .expect(
                                "non-null function pointer",
                            )(
                            params_this_motion.as_mut_ptr(),
                            matched_points,
                            npoints,
                            &raw mut current_motion,
                        );
                        if !(current_motion.num_inliers
                            > (*motions.offset(i as isize)).num_inliers)
                        {
                            break;
                        }
                        (*motions.offset(i as isize)).num_inliers = current_motion
                            .num_inliers;
                        (*motions.offset(i as isize)).sse = current_motion.sse;
                        let mut tmp_0: *mut core::ffi::c_int = (*motions
                            .offset(i as isize))
                            .inlier_indices;
                        let fresh2 = &raw mut (*motions.offset(i as isize))
                            .inlier_indices;
                        *fresh2 = current_motion.inlier_indices;
                        current_motion.inlier_indices = tmp_0;
                        refine_count += 1;
                    }
                }
                if !bad_model {
                    memcpy(
                        ((*motion_models.offset(i as isize)).params).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        params_this_motion.as_mut_ptr() as *const core::ffi::c_void,
                        (MAX_PARAMDIM as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<core::ffi::c_double>() as size_t,
                            ),
                    );
                    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                    while j < (*motions.offset(i as isize)).num_inliers {
                        let mut index: core::ffi::c_int = *((*motions.offset(i as isize))
                            .inlier_indices)
                            .offset(j as isize);
                        let mut corr: *const Correspondence = &*matched_points
                            .offset(index as isize) as *const Correspondence;
                        *((*motion_models.offset(i as isize)).inliers)
                            .offset(
                                (2 as core::ffi::c_int * j + 0 as core::ffi::c_int) as isize,
                            ) = rint((*corr).x) as core::ffi::c_int;
                        *((*motion_models.offset(i as isize)).inliers)
                            .offset(
                                (2 as core::ffi::c_int * j + 1 as core::ffi::c_int) as isize,
                            ) = rint((*corr).y) as core::ffi::c_int;
                        j += 1;
                    }
                    (*motion_models.offset(i as isize)).num_inliers = (*motions
                        .offset(i as isize))
                        .num_inliers;
                }
            }
            i += 1;
        }
    }
    aom_free(inlier_buffer as *mut core::ffi::c_void);
    aom_free(motions as *mut core::ffi::c_void);
    return ret_val;
}
static mut ransac_model_info: [RansacModelInfo; 4] = unsafe {
    [
        {
            let mut init = RansacModelInfo {
                find_transformation: None,
                score_model: None,
                minpts: 0 as core::ffi::c_int,
            };
            init
        },
        {
            let mut init = RansacModelInfo {
                find_transformation: None,
                score_model: None,
                minpts: 0 as core::ffi::c_int,
            };
            init
        },
        {
            let mut init = RansacModelInfo {
                find_transformation: Some(
                    find_rotzoom
                        as unsafe extern "C-unwind" fn(
                            *const Correspondence,
                            *const core::ffi::c_int,
                            core::ffi::c_int,
                            *mut core::ffi::c_double,
                        ) -> bool,
                ),
                score_model: Some(
                    score_affine
                        as unsafe extern "C-unwind" fn(
                            *const core::ffi::c_double,
                            *const Correspondence,
                            core::ffi::c_int,
                            *mut RANSAC_MOTION,
                        ) -> (),
                ),
                minpts: 2 as core::ffi::c_int,
            };
            init
        },
        {
            let mut init = RansacModelInfo {
                find_transformation: Some(
                    find_affine
                        as unsafe extern "C-unwind" fn(
                            *const Correspondence,
                            *const core::ffi::c_int,
                            core::ffi::c_int,
                            *mut core::ffi::c_double,
                        ) -> bool,
                ),
                score_model: Some(
                    score_affine
                        as unsafe extern "C-unwind" fn(
                            *const core::ffi::c_double,
                            *const Correspondence,
                            core::ffi::c_int,
                            *mut RANSAC_MOTION,
                        ) -> (),
                ),
                minpts: 3 as core::ffi::c_int,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C-unwind" fn ransac(
    mut matched_points: *const Correspondence,
    mut npoints: core::ffi::c_int,
    mut type_0: TransformationType,
    mut motion_models: *mut MotionModel,
    mut num_desired_motions: core::ffi::c_int,
    mut mem_alloc_failed: *mut bool,
) -> bool {
    return ransac_internal(
        matched_points,
        npoints,
        motion_models,
        num_desired_motions,
        &*core::ptr::addr_of!(ransac_model_info).cast::<RansacModelInfo>().offset(type_0 as isize),
        mem_alloc_failed,
    );
}
static mut TINY_NEAR_ZERO: core::ffi::c_double = 1.0E-16f64;
#[inline]
unsafe extern "C-unwind" fn linsolve(
    mut n: core::ffi::c_int,
    mut A: *mut core::ffi::c_double,
    mut stride: core::ffi::c_int,
    mut b: *mut core::ffi::c_double,
    mut x: *mut core::ffi::c_double,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut c: core::ffi::c_double = 0.;
    k = 0 as core::ffi::c_int;
    while k < n - 1 as core::ffi::c_int {
        i = n - 1 as core::ffi::c_int;
        while i > k {
            if fabs(*A.offset(((i - 1 as core::ffi::c_int) * stride + k) as isize))
                < fabs(*A.offset((i * stride + k) as isize))
            {
                j = 0 as core::ffi::c_int;
                while j < n {
                    c = *A.offset((i * stride + j) as isize);
                    *A.offset((i * stride + j) as isize) = *A
                        .offset(((i - 1 as core::ffi::c_int) * stride + j) as isize);
                    *A.offset(((i - 1 as core::ffi::c_int) * stride + j) as isize) = c;
                    j += 1;
                }
                c = *b.offset(i as isize);
                *b.offset(i as isize) = *b.offset((i - 1 as core::ffi::c_int) as isize);
                *b.offset((i - 1 as core::ffi::c_int) as isize) = c;
            }
            i -= 1;
        }
        i = k;
        while i < n - 1 as core::ffi::c_int {
            if fabs(*A.offset((k * stride + k) as isize)) < TINY_NEAR_ZERO {
                return 0 as core::ffi::c_int;
            }
            c = *A.offset(((i + 1 as core::ffi::c_int) * stride + k) as isize)
                / *A.offset((k * stride + k) as isize);
            j = 0 as core::ffi::c_int;
            while j < n {
                *A.offset(((i + 1 as core::ffi::c_int) * stride + j) as isize)
                    -= c * *A.offset((k * stride + j) as isize);
                j += 1;
            }
            *b.offset((i + 1 as core::ffi::c_int) as isize) -= c * *b.offset(k as isize);
            i += 1;
        }
        k += 1;
    }
    i = n - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        if fabs(*A.offset((i * stride + i) as isize)) < TINY_NEAR_ZERO {
            return 0 as core::ffi::c_int;
        }
        c = 0 as core::ffi::c_int as core::ffi::c_double;
        j = i + 1 as core::ffi::c_int;
        while j <= n - 1 as core::ffi::c_int {
            c += *A.offset((i * stride + j) as isize) * *x.offset(j as isize);
            j += 1;
        }
        *x.offset(i as isize) = (*b.offset(i as isize) - c)
            / *A.offset((i * stride + i) as isize);
        i -= 1;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn least_squares_init(
    mut mat: *mut core::ffi::c_double,
    mut y: *mut core::ffi::c_double,
    mut n: core::ffi::c_int,
) {
    memset(
        mat as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((n * n) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    );
    memset(
        y as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    );
}
#[inline]
unsafe extern "C-unwind" fn least_squares_accumulate(
    mut mat: *mut core::ffi::c_double,
    mut y: *mut core::ffi::c_double,
    mut a: *const core::ffi::c_double,
    mut b: core::ffi::c_double,
    mut n: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < n {
            *mat.offset((i * n + j) as isize)
                += *a.offset(i as isize) * *a.offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < n {
        *y.offset(i_0 as isize) += *a.offset(i_0 as isize) * b;
        i_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn least_squares_solve(
    mut mat: *mut core::ffi::c_double,
    mut y: *mut core::ffi::c_double,
    mut x: *mut core::ffi::c_double,
    mut n: core::ffi::c_int,
) -> core::ffi::c_int {
    return linsolve(n, mat, n, y, x);
}
#[inline]
unsafe extern "C-unwind" fn lcg_next(mut state: *mut uint32_t) -> uint32_t {
    *state = (*state as core::ffi::c_ulonglong)
        .wrapping_mul(1103515245 as core::ffi::c_ulonglong)
        .wrapping_add(12345 as core::ffi::c_ulonglong) as uint32_t;
    return *state;
}
#[inline]
unsafe extern "C-unwind" fn lcg_randint(mut state: *mut uint32_t, mut n: uint32_t) -> uint32_t {
    let mut v: uint64_t = (lcg_next(state) as uint64_t).wrapping_mul(n as uint64_t)
        >> 32 as core::ffi::c_int;
    return v as uint32_t;
}
#[inline]
unsafe extern "C-unwind" fn lcg_pick(
    mut n: core::ffi::c_int,
    mut k: core::ffi::c_int,
    mut out: *mut core::ffi::c_int,
    mut seed: *mut core::ffi::c_uint,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < k {
        let mut v: core::ffi::c_int = 0;
        '_resample: loop {
            v = lcg_randint(seed as *mut uint32_t, n as uint32_t) as core::ffi::c_int;
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            loop {
                if !(j < i) {
                    break '_resample;
                }
                if v == *out.offset(j as isize) {
                    break;
                }
                j += 1;
            }
        }
        *out.offset(i as isize) = v;
        i += 1;
    }
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn rint(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
