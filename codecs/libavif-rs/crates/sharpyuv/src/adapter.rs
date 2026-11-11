// SharpYUV 0.4.2 from libwebp 1.6.0. Copyright 2022 Google Inc.
// BSD-3-Clause and patent grant: LICENSES/.
// Unsafe C ABI and allocator adapter for the shared safe conversion core.

use libsharpyuv_core::sharpyuv::{self as shared, Dsp, SampleAccess, Scratch};

extern "C" {
    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_malloc")]
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    #[cfg_attr(sharpyuv_test_allocator, link_name = "sharp_test_free")]
    fn free(pointer: *mut core::ffi::c_void);
    fn pthread_mutex_lock(lock: *mut libc::pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(lock: *mut libc::pthread_mutex_t) -> core::ffi::c_int;
    fn SharpYuvInitDsp();
    fn SharpYuvInitGammaTables();
    static mut SharpYuvGetCPUInfo: VP8CPUInfo;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvConversionMatrix {
    pub rgb_to_y: [core::ffi::c_int; 4],
    pub rgb_to_u: [core::ffi::c_int; 4],
    pub rgb_to_v: [core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvOptions {
    pub yuv_matrix: *const SharpYuvConversionMatrix,
    pub transfer_type: SharpYuvTransferFunctionType,
}
pub type SharpYuvTransferFunctionType = core::ffi::c_uint;
pub const kSharpYuvTransferFunctionNum: SharpYuvTransferFunctionType = 19;
pub const kSharpYuvTransferFunctionHlg: SharpYuvTransferFunctionType = 18;
pub const kSharpYuvTransferFunctionSmpte428: SharpYuvTransferFunctionType = 17;
pub const kSharpYuvTransferFunctionSmpte2084: SharpYuvTransferFunctionType = 16;
pub const kSharpYuvTransferFunctionBt2020_12Bit: SharpYuvTransferFunctionType = 15;
pub const kSharpYuvTransferFunctionBt2020_10Bit: SharpYuvTransferFunctionType = 14;
pub const kSharpYuvTransferFunctionSrgb: SharpYuvTransferFunctionType = 13;
pub const kSharpYuvTransferFunctionBt1361: SharpYuvTransferFunctionType = 12;
pub const kSharpYuvTransferFunctionIec61966: SharpYuvTransferFunctionType = 11;
pub const kSharpYuvTransferFunctionLog100_Sqrt10: SharpYuvTransferFunctionType = 10;
pub const kSharpYuvTransferFunctionLog100: SharpYuvTransferFunctionType = 9;
pub const kSharpYuvTransferFunctionLinear: SharpYuvTransferFunctionType = 8;
pub const kSharpYuvTransferFunctionSmpte240: SharpYuvTransferFunctionType = 7;
pub const kSharpYuvTransferFunctionBt601: SharpYuvTransferFunctionType = 6;
pub const kSharpYuvTransferFunctionBt470Bg: SharpYuvTransferFunctionType = 5;
pub const kSharpYuvTransferFunctionBt470M: SharpYuvTransferFunctionType = 4;
pub const kSharpYuvTransferFunctionBt709: SharpYuvTransferFunctionType = 1;
pub type uint8_t = u8;
pub type fixed_y_t = uint16_t;
pub type uint16_t = u16;
pub type uint64_t = u64;
pub type size_t = usize;
pub type fixed_t = int16_t;
pub type int16_t = i16;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type VP8CPUInfo = *const core::ffi::c_void;
pub type CPUFeature = core::ffi::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type pthread_mutex_t = libc::pthread_mutex_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub const SHARPYUV_VERSION_MAJOR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SHARPYUV_VERSION_MINOR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SHARPYUV_VERSION: core::ffi::c_int = (0 as core::ffi::c_int)
    << 24 as core::ffi::c_int | (4 as core::ffi::c_int) << 16 as core::ffi::c_int
    | 2 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: core::ffi::c_int = 2_147_483_647;
pub const _PTHREAD_MUTEX_SIG_init: core::ffi::c_int = 0x32aaaba7 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGetVersion() -> core::ffi::c_int {
    return SHARPYUV_VERSION;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvInit(mut cpu_info_func: VP8CPUInfo) {
    static mut sharpyuv_last_cpuinfo_used: VP8CPUInfo = unsafe {
        ::core::mem::transmute::<
            *mut VP8CPUInfo,
            VP8CPUInfo,
        >(core::ptr::addr_of!(sharpyuv_last_cpuinfo_used) as *const VP8CPUInfo as *mut VP8CPUInfo)
    };
    static mut sharpyuv_lock: pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;
    if pthread_mutex_lock(core::ptr::addr_of_mut!(sharpyuv_lock)) != 0 {
        return;
    }
    if cpu_info_func
        != ::core::mem::transmute::<
            *mut VP8CPUInfo,
            VP8CPUInfo,
        >(core::ptr::addr_of_mut!(SharpYuvGetCPUInfo) as *mut VP8CPUInfo)
    {
        SharpYuvGetCPUInfo = cpu_info_func;
    }
    if sharpyuv_last_cpuinfo_used == SharpYuvGetCPUInfo {
        pthread_mutex_unlock(core::ptr::addr_of_mut!(sharpyuv_lock));
        return;
    }
    SharpYuvInitDsp();
    SharpYuvInitGammaTables();
    ::core::ptr::write_volatile(
        core::ptr::addr_of_mut!(sharpyuv_last_cpuinfo_used) as *mut VP8CPUInfo,
        SharpYuvGetCPUInfo,
    );
    pthread_mutex_unlock(core::ptr::addr_of_mut!(sharpyuv_lock));
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvConvert(
    mut r_ptr: *const core::ffi::c_void,
    mut g_ptr: *const core::ffi::c_void,
    mut b_ptr: *const core::ffi::c_void,
    mut rgb_step: core::ffi::c_int,
    mut rgb_stride: core::ffi::c_int,
    mut rgb_bit_depth: core::ffi::c_int,
    mut y_ptr: *mut core::ffi::c_void,
    mut y_stride: core::ffi::c_int,
    mut u_ptr: *mut core::ffi::c_void,
    mut u_stride: core::ffi::c_int,
    mut v_ptr: *mut core::ffi::c_void,
    mut v_stride: core::ffi::c_int,
    mut yuv_bit_depth: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut yuv_matrix: *const SharpYuvConversionMatrix,
) -> core::ffi::c_int {
    let mut options: SharpYuvOptions = SharpYuvOptions {
        yuv_matrix: 0 as *const SharpYuvConversionMatrix,
        transfer_type: 0 as SharpYuvTransferFunctionType,
    };
    options.yuv_matrix = yuv_matrix;
    options.transfer_type = kSharpYuvTransferFunctionSrgb;
    return SharpYuvConvertWithOptions(
        r_ptr,
        g_ptr,
        b_ptr,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_stride,
        u_ptr,
        u_stride,
        v_ptr,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &mut options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvOptionsInitInternal(
    mut yuv_matrix: *const SharpYuvConversionMatrix,
    mut options: *mut SharpYuvOptions,
    mut version: core::ffi::c_int,
) -> core::ffi::c_int {
    let major: core::ffi::c_int = version >> 24 as core::ffi::c_int;
    let minor: core::ffi::c_int = version >> 16 as core::ffi::c_int
        & 0xff as core::ffi::c_int;
    if options.is_null() || yuv_matrix.is_null()
        || major == SHARPYUV_VERSION_MAJOR && major == 0 as core::ffi::c_int
            && minor != SHARPYUV_VERSION_MINOR || major != SHARPYUV_VERSION_MAJOR
    {
        return 0 as core::ffi::c_int;
    }
    (*options).yuv_matrix = yuv_matrix;
    (*options).transfer_type = kSharpYuvTransferFunctionSrgb;
    return 1 as core::ffi::c_int;
}

/// The C caller supplies live aligned storage for every addressed sample.
/// No Rust slice is forged from a pointer whose allocation length is unknown.
struct RawAccess {
    input: [*const u8; 3],
    output: [*mut u8; 3],
    rgb_step: isize,
    rgb_stride: isize,
    output_stride: [isize; 3],
}

impl SampleAccess for RawAccess {
    fn read(&self, channel: usize, row: usize, column: usize, bit_depth: i32) -> u16 {
        let offset = row as isize * self.rgb_stride + column as isize * self.rgb_step;
        // SAFETY: The C caller promises this sample is live, aligned for its
        // depth, and reachable under the declared signed row and pixel steps.
        unsafe {
            let pointer = self.input[channel].offset(offset);
            if bit_depth == 8 { u16::from(*pointer) } else { *(pointer as *const u16) }
        }
    }

    fn write(&mut self, plane: usize, row: usize, column: usize, value: u16, bit_depth: i32) {
        let unit = if bit_depth <= 8 { 1 } else { 2 };
        let offset = row as isize * self.output_stride[plane] + column as isize * unit;
        // SAFETY: The C caller promises this output sample is live, writable,
        // aligned for its depth, and its planes satisfy the C aliasing contract.
        unsafe {
            let pointer = self.output[plane].offset(offset);
            if bit_depth <= 8 { *pointer = value as u8; } else { *(pointer as *mut u16) = value; }
        }
    }
}

struct RawScratch([*mut core::ffi::c_void; 7]);

impl RawScratch {
    unsafe fn allocate(lengths: [usize; 7]) -> Self {
        let mut pointers = [core::ptr::null_mut(); 7];
        for (index, length) in lengths.iter().enumerate() {
            // SAFETY: The C allocation hook has the same signature as malloc.
            pointers[index] = unsafe { malloc(length * 2) };
        }
        Self(pointers)
    }

    fn complete(&self) -> bool {
        self.0.iter().all(|pointer| !pointer.is_null())
    }

    unsafe fn initialized(&mut self, lengths: [usize; 7]) -> Scratch<'_> {
        // SAFETY: Every successful malloc is disjoint, aligned to at least
        // u16, and spans its requested element count. Zeroing first makes
        // each typed Rust slice fully initialized before it is created.
        for (pointer, length) in self.0.iter().zip(lengths) {
            unsafe { core::ptr::write_bytes(*pointer as *mut u8, 0, length * 2); }
        }
        let [a, b, c, d, e, f, g] = self.0;
        Scratch {
            tmp_buffer: unsafe { core::slice::from_raw_parts_mut(a as *mut u16, lengths[0]) },
            best_y_base: unsafe { core::slice::from_raw_parts_mut(b as *mut u16, lengths[1]) },
            target_y_base: unsafe { core::slice::from_raw_parts_mut(c as *mut u16, lengths[2]) },
            best_rgb_y: unsafe { core::slice::from_raw_parts_mut(d as *mut u16, lengths[3]) },
            best_uv_base: unsafe { core::slice::from_raw_parts_mut(e as *mut i16, lengths[4]) },
            target_uv_base: unsafe { core::slice::from_raw_parts_mut(f as *mut i16, lengths[5]) },
            best_rgb_uv: unsafe { core::slice::from_raw_parts_mut(g as *mut i16, lengths[6]) },
        }
    }
}

impl Drop for RawScratch {
    fn drop(&mut self) {
        // Preserve the translated C cleanup order, including free(NULL).
        for index in [1, 4, 2, 5, 3, 6, 0] {
            // SAFETY: Each pointer came from malloc and is freed exactly once.
            unsafe { free(self.0[index]); }
        }
    }
}

#[no_mangle]
#[expect(clippy::too_many_arguments, reason = "preserved SharpYUV C ABI")]
pub unsafe extern "C" fn SharpYuvConvertWithOptions(
    r_ptr: *const core::ffi::c_void,
    g_ptr: *const core::ffi::c_void,
    b_ptr: *const core::ffi::c_void,
    rgb_step: core::ffi::c_int,
    rgb_stride: core::ffi::c_int,
    rgb_bit_depth: core::ffi::c_int,
    y_ptr: *mut core::ffi::c_void,
    y_stride: core::ffi::c_int,
    u_ptr: *mut core::ffi::c_void,
    u_stride: core::ffi::c_int,
    v_ptr: *mut core::ffi::c_void,
    v_stride: core::ffi::c_int,
    yuv_bit_depth: core::ffi::c_int,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    options: *const SharpYuvOptions,
) -> core::ffi::c_int {
    // The original C API requires a live options pointer. Null options are
    // outside its defined rejection domain; retain that caller contract.
    let options = unsafe { &*options };
    if r_ptr.is_null() || g_ptr.is_null() || b_ptr.is_null()
        || y_ptr.is_null() || u_ptr.is_null() || v_ptr.is_null()
        || !shared::valid_arguments(rgb_step, rgb_stride, rgb_bit_depth,
            y_stride, u_stride, v_stride, yuv_bit_depth, width, height)
    {
        return 0;
    }
    // SAFETY: The caller supplied the original C matrix contract.
    let matrix = unsafe { &*options.yuv_matrix };
    // SAFETY: This is the original independent callback sentinel; its state
    // and mutex are kept in the AVIF package, never routed through WebP.
    unsafe {
        SharpYuvInit(core::mem::transmute::<*mut VP8CPUInfo, VP8CPUInfo>(
            core::ptr::addr_of_mut!(SharpYuvGetCPUInfo)))
    };
    let lengths = shared::scratch_lengths(width as usize, height as usize);
    // SAFETY: All seven calls use the original malloc and test allocator hook.
    let mut owned = unsafe { RawScratch::allocate(lengths) };
    if !owned.complete() { return 0; }
    // SAFETY: `complete` established all allocations; initialized zeroes
    // every element before creating mutable slices.
    let scratch = unsafe { owned.initialized(lengths) };
    let mut access = RawAccess {
        input: [r_ptr.cast(), g_ptr.cast(), b_ptr.cast()],
        output: [y_ptr.cast(), u_ptr.cast(), v_ptr.cast()],
        rgb_step: rgb_step as isize,
        rgb_stride: rgb_stride as isize,
        output_stride: [y_stride as isize, u_stride as isize, v_stride as isize],
    };
    let matrix = shared::SharpYuvConversionMatrix {
        rgb_to_y: matrix.rgb_to_y, rgb_to_u: matrix.rgb_to_u,
        rgb_to_v: matrix.rgb_to_v,
    };
    i32::from(shared::convert_with_access(
        &mut access, scratch, Dsp::SCALAR, rgb_bit_depth,
        yuv_bit_depth, width as usize, height as usize, &matrix,
        shared::SharpYuvTransferFunctionType(options.transfer_type),
    ))
}
