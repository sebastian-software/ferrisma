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
//
// Multi-threaded worker
//
// Original source:
//  https://chromium.googlesource.com/webm/libwebp

// Enable GNU extensions in glibc so that we can call pthread_setname_np().
// This must be before any #include statements.

// Mechanically translated from upstream/aom/aom_util/aom_thread.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: core::ffi::c_uint,
    pub __high: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: core::ffi::c_int,
    pub __count: core::ffi::c_uint,
    pub __owner: core::ffi::c_int,
    pub __nusers: core::ffi::c_uint,
    pub __kind: core::ffi::c_int,
    pub __spins: core::ffi::c_short,
    pub __elision: core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [core::ffi::c_uint; 2],
    pub __g_size: [core::ffi::c_uint; 2],
    pub __g1_orig_size: core::ffi::c_uint,
    pub __wrefs: core::ffi::c_uint,
    pub __g_signals: [core::ffi::c_uint; 2],
}
pub type pthread_t = libc::pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [core::ffi::c_char; 56],
    pub __align: core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [core::ffi::c_char; 40],
    pub __align: core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [core::ffi::c_char; 48],
    pub __align: core::ffi::c_longlong,
}
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorkerImpl {
    pub mutex_: pthread_mutex_t,
    pub condition_: pthread_cond_t,
    pub thread_: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorker {
    pub impl_: *mut AVxWorkerImpl,
    pub status_: AVxWorkerStatus,
    pub thread_name: *const core::ffi::c_char,
    pub hook: AVxWorkerHook,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
    pub had_error: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorkerInterface {
    pub init: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub reset: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub sync: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub launch: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub execute: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub end: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const THREAD_EXIT_SUCCESS: *mut core::ffi::c_void = NULL;
unsafe extern "C-unwind" fn thread_loop(
    mut ptr: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let worker: *mut AVxWorker = ptr as *mut AVxWorker;
    if !((*worker).thread_name).is_null() {
        let mut thread_name: [core::ffi::c_char; 16] = [0; 16];
        strncpy(
            thread_name.as_mut_ptr(),
            (*worker).thread_name,
            (::core::mem::size_of::<[core::ffi::c_char; 16]>() as size_t)
                .wrapping_sub(1 as size_t),
        );
        thread_name[(::core::mem::size_of::<[core::ffi::c_char; 16]>() as usize)
            .wrapping_sub(1 as usize) as usize] = '\0' as i32 as core::ffi::c_char;
        pthread_setname_np(pthread_self(), thread_name.as_mut_ptr());
    }
    pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
    loop {
        while (*worker).status_ as core::ffi::c_uint
            == AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
        {
            pthread_cond_wait(
                &raw mut (*(*worker).impl_).condition_,
                &raw mut (*(*worker).impl_).mutex_,
            );
        }
        if !((*worker).status_ as core::ffi::c_uint
            == AVX_WORKER_STATUS_WORKING as core::ffi::c_int as core::ffi::c_uint)
        {
            break;
        }
        pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
        execute(worker);
        pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
        (*worker).status_ = AVX_WORKER_STATUS_OK;
        pthread_cond_signal(&raw mut (*(*worker).impl_).condition_);
    }
    pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
    return THREAD_EXIT_SUCCESS;
}
unsafe extern "C-unwind" fn change_state(
    worker: *mut AVxWorker,
    mut new_status: AVxWorkerStatus,
) {
    if ((*worker).impl_).is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
    if (*worker).status_ as core::ffi::c_uint
        >= AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
    {
        while (*worker).status_ as core::ffi::c_uint
            != AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
        {
            pthread_cond_wait(
                &raw mut (*(*worker).impl_).condition_,
                &raw mut (*(*worker).impl_).mutex_,
            );
        }
        if new_status as core::ffi::c_uint
            != AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
        {
            (*worker).status_ = new_status;
            pthread_cond_signal(&raw mut (*(*worker).impl_).condition_);
        }
    }
    pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
}
unsafe extern "C-unwind" fn init(worker: *mut AVxWorker) {
    memset(
        worker as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<AVxWorker>() as size_t,
    );
    (*worker).status_ = AVX_WORKER_STATUS_NOT_OK;
}
unsafe extern "C-unwind" fn sync(worker: *mut AVxWorker) -> core::ffi::c_int {
    change_state(worker, AVX_WORKER_STATUS_OK);
    return ((*worker).had_error == 0) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn reset(worker: *mut AVxWorker) -> core::ffi::c_int {
    let mut kMinStackSize: size_t = 0;
    let mut stacksize: size_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut current_block: u64;
    let mut ok: core::ffi::c_int = 1 as core::ffi::c_int;
    (*worker).had_error = 0 as core::ffi::c_int;
    if ((*worker).status_ as core::ffi::c_uint)
        < AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*worker).impl_ = aom_calloc(
            1 as size_t,
            ::core::mem::size_of::<AVxWorkerImpl>() as size_t,
        ) as *mut AVxWorkerImpl;
        if ((*worker).impl_).is_null() {
            return 0 as core::ffi::c_int;
        }
        if pthread_mutex_init(
            &raw mut (*(*worker).impl_).mutex_,
            0 as *const pthread_mutexattr_t,
        ) != 0
        {
            current_block = 0;
        } else if pthread_cond_init(
            &raw mut (*(*worker).impl_).condition_,
            0 as *const pthread_condattr_t,
        ) != 0
        {
            pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
            current_block = 0;
        } else {
            attr = pthread_attr_t { __size: [0; 56] };
            if pthread_attr_init(&raw mut attr) != 0 {
                current_block = 1;
            } else {
                kMinStackSize = (256 as core::ffi::c_int * 1024 as core::ffi::c_int)
                    as size_t;
                stacksize = 0;
                if pthread_attr_getstacksize(&raw mut attr, &raw mut stacksize) == 0 {
                    if stacksize < kMinStackSize
                        && pthread_attr_setstacksize(&raw mut attr, kMinStackSize) != 0
                    {
                        pthread_attr_destroy(&raw mut attr);
                        current_block = 1;
                    } else {
                        current_block = 2;
                    }
                } else {
                    current_block = 2;
                }
                match current_block {
                    1 => {}
                    _ => {
                        pthread_mutex_lock(&raw mut (*(*worker).impl_).mutex_);
                        ok = (pthread_create(
                            &raw mut (*(*worker).impl_).thread_,
                            &raw mut attr,
                            Some(
                                thread_loop
                                    as unsafe extern "C-unwind" fn(
                                        *mut core::ffi::c_void,
                                    ) -> *mut core::ffi::c_void,
                            ),
                            worker as *mut core::ffi::c_void,
                        ) == 0) as core::ffi::c_int;
                        if ok != 0 {
                            (*worker).status_ = AVX_WORKER_STATUS_OK;
                        }
                        pthread_mutex_unlock(&raw mut (*(*worker).impl_).mutex_);
                        pthread_attr_destroy(&raw mut attr);
                        if ok == 0 {
                            current_block = 1;
                        } else {
                            current_block = 3;
                        }
                    }
                }
            }
            match current_block {
                3 => {}
                _ => {
                    pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
                    pthread_cond_destroy(&raw mut (*(*worker).impl_).condition_);
                    current_block = 0;
                }
            }
        }
        match current_block {
            3 => {}
            _ => {
                aom_free((*worker).impl_ as *mut core::ffi::c_void);
                (*worker).impl_ = 0 as *mut AVxWorkerImpl;
                return 0 as core::ffi::c_int;
            }
        }
    } else if (*worker).status_ as core::ffi::c_uint
        > AVX_WORKER_STATUS_OK as core::ffi::c_int as core::ffi::c_uint
    {
        ok = sync(worker);
    }
    return ok;
}
unsafe extern "C-unwind" fn execute(worker: *mut AVxWorker) {
    if ((*worker).hook).is_some() {
        (*worker).had_error
            |= (((*worker).hook)
                .expect("non-null function pointer")((*worker).data1, (*worker).data2)
                == 0) as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn launch(worker: *mut AVxWorker) {
    change_state(worker, AVX_WORKER_STATUS_WORKING);
}
unsafe extern "C-unwind" fn end(worker: *mut AVxWorker) {
    if !((*worker).impl_).is_null() {
        change_state(worker, AVX_WORKER_STATUS_NOT_OK);
        pthread_join((*(*worker).impl_).thread_, 0 as *mut *mut core::ffi::c_void);
        pthread_mutex_destroy(&raw mut (*(*worker).impl_).mutex_);
        pthread_cond_destroy(&raw mut (*(*worker).impl_).condition_);
        aom_free((*worker).impl_ as *mut core::ffi::c_void);
        (*worker).impl_ = 0 as *mut AVxWorkerImpl;
    }
}
static mut g_worker_interface: AVxWorkerInterface = unsafe {
    {
        let mut init = AVxWorkerInterface {
            init: Some(init as unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()),
            reset: Some(
                reset as unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int,
            ),
            sync: Some(sync as unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int),
            launch: Some(launch as unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()),
            execute: Some(execute as unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()),
            end: Some(end as unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_set_worker_interface(
    winterface: *const AVxWorkerInterface,
) -> core::ffi::c_int {
    if winterface.is_null() || ((*winterface).init).is_none()
        || ((*winterface).reset).is_none() || ((*winterface).sync).is_none()
        || ((*winterface).launch).is_none() || ((*winterface).execute).is_none()
        || ((*winterface).end).is_none()
    {
        return 0 as core::ffi::c_int;
    }
    g_worker_interface = *winterface;
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_worker_interface() -> *const AVxWorkerInterface {
    return &raw mut g_worker_interface;
}

extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn strncpy(
        __dest: *mut core::ffi::c_char,
        __src: *const core::ffi::c_char,
        __n: size_t,
    ) -> *mut core::ffi::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        >,
        __arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> core::ffi::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> core::ffi::c_int;
    fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut size_t,
    ) -> core::ffi::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> core::ffi::c_int;
    fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> core::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
}
