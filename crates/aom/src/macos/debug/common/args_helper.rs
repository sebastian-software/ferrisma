/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/common/args_helper.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg {
    pub argv: *mut *mut core::ffi::c_char,
    pub name: *const core::ffi::c_char,
    pub val: *const core::ffi::c_char,
    pub argv_step: core::ffi::c_uint,
    pub def: *const arg_def,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_def {
    pub short_name: *const core::ffi::c_char,
    pub long_name: *const core::ffi::c_char,
    pub has_val: core::ffi::c_int,
    pub desc: *const core::ffi::c_char,
    pub enums: *const arg_enum_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_enum_list {
    pub name: *const core::ffi::c_char,
    pub val: core::ffi::c_int,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const ARG_ERR_MSG_MAX_LEN: core::ffi::c_int = 200 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: core::ffi::c_int = -__INT_MAX__ - 1 as core::ffi::c_int;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
unsafe extern "C-unwind" fn arg_init(mut argv: *mut *mut core::ffi::c_char) -> arg {
    let mut a: arg = arg {
        argv: 0 as *mut *mut core::ffi::c_char,
        name: 0 as *const core::ffi::c_char,
        val: 0 as *const core::ffi::c_char,
        argv_step: 0,
        def: 0 as *const arg_def,
    };
    a.argv = argv;
    a.argv_step = 1 as core::ffi::c_uint;
    a.name = 0 as *const core::ffi::c_char;
    a.val = 0 as *const core::ffi::c_char;
    a.def = 0 as *const arg_def;
    return a;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_match_helper(
    mut arg_: *mut arg,
    mut def: *const arg_def,
    mut argv: *mut *mut core::ffi::c_char,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut arg: arg = arg {
        argv: 0 as *mut *mut core::ffi::c_char,
        name: 0 as *const core::ffi::c_char,
        val: 0 as *const core::ffi::c_char,
        argv_step: 0,
        def: 0 as *const arg_def,
    };
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    if !((*def).has_val == 0 as core::ffi::c_int
        || (*def).has_val == 1 as core::ffi::c_int
        || (*def).has_val == -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"arg_match_helper\0"))
                .as_ptr(),
            b"args_helper.c\0" as *const u8 as *const core::ffi::c_char,
            39 as core::ffi::c_int,
            b"def->has_val == 0 || def->has_val == 1 || def->has_val == -1\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*argv.offset(0 as core::ffi::c_int as isize)).is_null()
        || *(*argv.offset(0 as core::ffi::c_int as isize))
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != '-' as i32
    {
        return 0 as core::ffi::c_int;
    }
    arg = arg_init(argv);
    if !((*def).short_name).is_null()
        && strcmp(
            (*(arg.argv).offset(0 as core::ffi::c_int as isize))
                .offset(1 as core::ffi::c_int as isize),
            (*def).short_name,
        ) == 0
    {
        arg.name = (*(arg.argv).offset(0 as core::ffi::c_int as isize))
            .offset(1 as core::ffi::c_int as isize);
        arg.val = if (*def).has_val != 0 {
            *(arg.argv).offset(1 as core::ffi::c_int as isize)
        } else {
            0 as *mut core::ffi::c_char
        };
        arg.argv_step = (if (*def).has_val != 0 {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        }) as core::ffi::c_uint;
    } else if !((*def).long_name).is_null() {
        let name_len: size_t = strlen((*def).long_name) as size_t;
        if *(*(arg.argv).offset(0 as core::ffi::c_int as isize))
            .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int == '-' as i32
            && strncmp(
                (*(arg.argv).offset(0 as core::ffi::c_int as isize))
                    .offset(2 as core::ffi::c_int as isize),
                (*def).long_name,
                name_len,
            ) == 0
            && (*(*(arg.argv).offset(0 as core::ffi::c_int as isize))
                .offset(name_len.wrapping_add(2 as size_t) as isize) as core::ffi::c_int
                == '=' as i32
                || *(*(arg.argv).offset(0 as core::ffi::c_int as isize))
                    .offset(name_len.wrapping_add(2 as size_t) as isize)
                    as core::ffi::c_int == '\0' as i32)
        {
            arg.name = (*(arg.argv).offset(0 as core::ffi::c_int as isize))
                .offset(2 as core::ffi::c_int as isize);
            arg.val = if *(arg.name).offset(name_len as isize) as core::ffi::c_int
                == '=' as i32
            {
                (arg.name)
                    .offset(name_len as isize)
                    .offset(1 as core::ffi::c_int as isize)
            } else {
                0 as *const core::ffi::c_char
            };
            arg.argv_step = 1 as core::ffi::c_uint;
        }
    }
    if !(arg.name).is_null() {
        if (*def).has_val == -(1 as core::ffi::c_int) {
            arg.def = def as *const arg_def;
            *arg_ = arg;
            return 1 as core::ffi::c_int;
        }
        if (arg.val).is_null() && (*def).has_val != 0 {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Error: option %s requires argument.\n\0" as *const u8
                        as *const core::ffi::c_char,
                    arg.name,
                );
            }
            return 0 as core::ffi::c_int;
        }
        if !(arg.val).is_null() && (*def).has_val == 0 {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Error: option %s requires no argument.\n\0" as *const u8
                        as *const core::ffi::c_char,
                    arg.name,
                );
            }
            return 0 as core::ffi::c_int;
        }
        arg.def = def as *const arg_def;
        *arg_ = arg;
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_uint_helper(
    mut arg: *const arg,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_uint {
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let rawval: core::ffi::c_ulong = strtoul(
        (*arg).val,
        &raw mut endptr,
        10 as core::ffi::c_int,
    ) as core::ffi::c_ulong;
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    if *((*arg).val).offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '\0' as i32
    {
        if rawval <= UINT_MAX as core::ffi::c_ulong {
            return rawval as core::ffi::c_uint;
        }
        if !err_msg.is_null() {
            snprintf(
                err_msg,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Option %s: Value %lu out of range for unsigned int\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*arg).name,
                rawval,
            );
        }
        return 0 as core::ffi::c_uint;
    }
    if !err_msg.is_null() {
        snprintf(
            err_msg,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Option %s: Invalid character '%c'\n\0" as *const u8
                as *const core::ffi::c_char,
            (*arg).name,
            *endptr as core::ffi::c_int,
        );
    }
    return 0 as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_int_helper(
    mut arg: *const arg,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let rawval: core::ffi::c_long = strtol(
        (*arg).val,
        &raw mut endptr,
        10 as core::ffi::c_int,
    ) as core::ffi::c_long;
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    if *((*arg).val).offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '\0' as i32
    {
        if rawval >= INT_MIN as core::ffi::c_long
            && rawval <= INT_MAX as core::ffi::c_long
        {
            return rawval as core::ffi::c_int;
        }
        if !err_msg.is_null() {
            snprintf(
                err_msg,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Option %s: Value %ld out of range for signed int\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*arg).name,
                rawval,
            );
        }
        return 0 as core::ffi::c_int;
    }
    if !err_msg.is_null() {
        snprintf(
            err_msg,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Option %s: Invalid character '%c'\n\0" as *const u8
                as *const core::ffi::c_char,
            (*arg).name,
            *endptr as core::ffi::c_int,
        );
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_rational_helper(
    mut arg: *const arg,
    mut err_msg: *mut core::ffi::c_char,
) -> aom_rational {
    let mut rawval: core::ffi::c_long = 0;
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut rat: aom_rational = {
        let mut init = aom_rational {
            num: 0 as core::ffi::c_int,
            den: 1 as core::ffi::c_int,
        };
        init
    };
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    rawval = strtol((*arg).val, &raw mut endptr, 10 as core::ffi::c_int);
    if *((*arg).val).offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '/' as i32
    {
        if rawval >= INT_MIN as core::ffi::c_long
            && rawval <= INT_MAX as core::ffi::c_long
        {
            rat.num = rawval as core::ffi::c_int;
        } else {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Option %s: Value %ld out of range for signed int\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*arg).name,
                    rawval,
                );
            }
            return rat;
        }
    } else {
        if !err_msg.is_null() {
            snprintf(
                err_msg,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Option %s: Expected / at '%c'\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*arg).name,
                *endptr as core::ffi::c_int,
            );
        }
        return rat;
    }
    rawval = strtol(
        endptr.offset(1 as core::ffi::c_int as isize),
        &raw mut endptr,
        10 as core::ffi::c_int,
    );
    if *((*arg).val).offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '\0' as i32
    {
        if rawval >= INT_MIN as core::ffi::c_long
            && rawval <= INT_MAX as core::ffi::c_long
        {
            rat.den = rawval as core::ffi::c_int;
        } else {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Option %s: Value %ld out of range for signed int\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*arg).name,
                    rawval,
                );
            }
            return rat;
        }
    } else {
        if !err_msg.is_null() {
            snprintf(
                err_msg,
                ARG_ERR_MSG_MAX_LEN as size_t,
                b"Option %s: Invalid character '%c'\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*arg).name,
                *endptr as core::ffi::c_int,
            );
        }
        return rat;
    }
    return rat;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_enum_helper(
    mut arg: *const arg,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut listptr: *const arg_enum_list = 0 as *const arg_enum_list;
    let mut rawval: core::ffi::c_long = 0;
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    rawval = strtol((*arg).val, &raw mut endptr, 10 as core::ffi::c_int);
    if *((*arg).val).offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        != '\0' as i32
        && *endptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            == '\0' as i32
    {
        listptr = (*(*arg).def).enums;
        while !((*listptr).name).is_null() {
            if (*listptr).val as core::ffi::c_long == rawval {
                return rawval as core::ffi::c_int;
            }
            listptr = listptr.offset(1);
        }
    }
    listptr = (*(*arg).def).enums;
    while !((*listptr).name).is_null() {
        if strcmp((*arg).val, (*listptr).name) == 0 {
            return (*listptr).val;
        }
        listptr = listptr.offset(1);
    }
    if !err_msg.is_null() {
        snprintf(
            err_msg,
            ARG_ERR_MSG_MAX_LEN as size_t,
            b"Option %s: Invalid value '%s'\n\0" as *const u8
                as *const core::ffi::c_char,
            (*arg).name,
            (*arg).val,
        );
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_enum_or_int_helper(
    mut arg: *const arg,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    if !((*(*arg).def).enums).is_null() {
        return arg_parse_enum_helper(arg, err_msg);
    }
    return arg_parse_int_helper(arg, err_msg);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn arg_parse_list_helper(
    mut arg: *const arg,
    mut list: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut err_msg: *mut core::ffi::c_char,
) -> core::ffi::c_int {
    let mut ptr: *const core::ffi::c_char = (*arg).val;
    let mut endptr: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    if !err_msg.is_null() {
        *err_msg.offset(0 as core::ffi::c_int as isize) = '\0' as i32
            as core::ffi::c_char;
    }
    while *ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != '\0' as i32
    {
        let mut rawval: core::ffi::c_long = strtol(
            ptr,
            &raw mut endptr,
            10 as core::ffi::c_int,
        );
        if rawval < INT_MIN as core::ffi::c_long || rawval > INT_MAX as core::ffi::c_long
        {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Option %s: Value %ld out of range for signed int\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*arg).name,
                    rawval,
                );
            }
            return 0 as core::ffi::c_int;
        } else if i >= n {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Option %s: List has more than %d entries\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*arg).name,
                    n,
                );
            }
            return 0 as core::ffi::c_int;
        } else if *endptr as core::ffi::c_int == ',' as i32 {
            endptr = endptr.offset(1);
        } else if *endptr as core::ffi::c_int != '\0' as i32 {
            if !err_msg.is_null() {
                snprintf(
                    err_msg,
                    ARG_ERR_MSG_MAX_LEN as size_t,
                    b"Option %s: Bad list separator '%c'\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*arg).name,
                    *endptr as core::ffi::c_int,
                );
            }
            return 0 as core::ffi::c_int;
        }
        let fresh0 = i;
        i = i + 1;
        *list.offset(fresh0 as isize) = rawval as core::ffi::c_int;
        ptr = endptr;
    }
    return i;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;

extern "C" {
    fn snprintf(
        __str: *mut core::ffi::c_char,
        __size: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn strtol(
        __str: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn strtoul(
        __str: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_ulong;
    fn strcmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
}
