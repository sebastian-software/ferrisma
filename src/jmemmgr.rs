// libjpeg-turbo 3.2.1, src/jmemmgr.c:
//
// /*
//  * jmemmgr.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2016, 2021-2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the JPEG system-independent memory management
//  * routines.  This code is usable across a wide variety of machines; most
//  * of the system dependencies have been isolated in a separate file.
//  * The major functions provided here are:
//  *   * pool-based allocation and freeing of memory;
//  *   * policy decisions about how to divide available memory among the
//  *     virtual arrays;
//  *   * control logic for swapping virtual arrays between main memory and
//  *     backing storage.
//  * The separate system-dependent file provides the actual backing-storage
//  * access code, and it contains the policy decision about how much total
//  * main memory to use.
//  * This file is system-dependent in the sense that some of its functions
//  * are unnecessary in some systems.  For example, if there is enough virtual
//  * memory so that backing storage will never be used, much of the virtual
//  * array control logic could be removed.  (Of course, if you have that much
//  * memory then you shouldn't care about a little bit of unused code...)
//  */
//! Translation of `src/jmemmgr.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The pool bookkeeping is upstream's, with the same arithmetic and order;
//! the storage is not (ADR 0005, "The memory manager"). Each request names
//! the C size of what it allocates, and the storage behind it is a Rust value
//! that the requesting code owns: [`alloc_small_box`], [`alloc_small_vec`]
//! and [`alloc_large_vec`] are the typed forms of the `alloc_small` and
//! `alloc_large` calls, and `alloc_sarray` and `alloc_barray` return a
//! [`SampleArray`] or [`BlockArray`]. A pool is a size record in its list, so
//! that `total_space_allocated` and the slop rules behave as in C.
//!
//! Not translated: the virtual sample arrays (`jvirt_sarray_control`,
//! `request_virt_sarray`, `access_virt_sarray`, `do_sarray_io` and their
//! loops in `realize_virt_arrays` and `free_pool`), which serve only two-pass
//! quantization (ADR 0005); and the backing-store I/O (`do_barray_io`, the
//! `b_s_open` and `b_s_info` fields, the reload in `access_virt_barray` and
//! the close loop in `free_pool`), because `jmemnobs.c` never opens a backing
//! store: its `jpeg_open_backing_store` always fails, so `b_s_open` never
//! becomes `TRUE` and every path that needs it ends in its error exit.
//! `MEM_STATS` and `print_mem_stats` are off in the reference configuration.

use core::ffi::c_long;

use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jmemnobs::{
    jpeg_free_large, jpeg_free_small, jpeg_get_large, jpeg_get_small, jpeg_mem_available,
    jpeg_mem_init, jpeg_mem_term, jpeg_open_backing_store,
};
use crate::jmemsys::MAX_ALLOC_CHUNK;
use crate::jmorecfg::{
    C_BOOLEAN, C_JDIMENSION, C_LONG, C_POINTER, C_SIZE_T, CLayout, JDIMENSION, JSAMPLE,
};
use crate::jpegint::MIN;
use crate::jpeglib::{
    BlockArray, JPOOL_IMAGE, JPOOL_NUMPOOLS, JPOOL_PERMANENT, Reservation, SIZEOF_JBLOCK,
    SIZEOF_JBLOCKROW, SIZEOF_JSAMPROW, SampleArray, j_common_ptr, jpeg_memory_mgr,
    jvirt_barray_ptr,
};
use crate::jutils::jzero_far;

// upstream: src/jmemmgr.c round_up_pow2
/// `a` rounded up to the next multiple of `b`, i.e. `ceil(a/b)*b`; assumes
/// `b` is a power of 2.
fn round_up_pow2(a: usize, b: usize) -> usize {
    a.wrapping_add(b).wrapping_sub(1) & !(b - 1)
}

// upstream: src/jmemmgr.c ALIGN_SIZE
/// `MAX(sizeof(void *), sizeof(double))`, since `WITH_SIMD` is undefined in
/// the reference configuration (ADR 0005).
pub const ALIGN_SIZE: usize = if C_POINTER.size > core::mem::size_of::<f64>() {
    C_POINTER.size
} else {
    core::mem::size_of::<f64>()
};

// upstream: src/jmemmgr.c small_pool_hdr
/// The record of one small pool. `next` is the pool's position in its list.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct small_pool_hdr {
    /// How many bytes already used within pool.
    pub bytes_used: usize,
    /// Bytes still available in this pool.
    pub bytes_left: usize,
}

// upstream: src/jmemmgr.c large_pool_hdr
/// The record of one large pool. `next` is the pool's position in its list.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct large_pool_hdr {
    /// How many bytes already used within pool.
    pub bytes_used: usize,
    /// Bytes still available in this pool.
    pub bytes_left: usize,
}

/// `sizeof(small_pool_hdr)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_small_pool_hdr: usize = CLayout::structure(&[C_POINTER, C_SIZE_T, C_SIZE_T]).size;
/// `sizeof(large_pool_hdr)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_large_pool_hdr: usize = CLayout::structure(&[C_POINTER, C_SIZE_T, C_SIZE_T]).size;

// upstream: src/jmemmgr.c my_memory_mgr
/// The full definition of a memory manager object.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Debug)]
pub struct my_memory_mgr {
    /// Public fields.
    pub pub_: jpeg_memory_mgr,
    /// The small pools of each lifetime class, head of the list first.
    pub small_list: [Vec<small_pool_hdr>; JPOOL_NUMPOOLS as usize],
    /// The large pools of each lifetime class, head of the list first.
    pub large_list: [Vec<large_pool_hdr>; JPOOL_NUMPOOLS as usize],
    /// The virtual block arrays in the order they were requested, so that a
    /// `jvirt_barray_ptr` stays valid; upstream links each new array at the
    /// head of the list, so its list order is the reverse.
    pub virt_barray_list: Vec<jvirt_barray_control>,
    /// Total space obtained from `jpeg_get_small` and `jpeg_get_large`.
    pub total_space_allocated: usize,
    /// `rowsperchunk` of the most recent `alloc_sarray` or `alloc_barray`.
    pub last_rowsperchunk: JDIMENSION,
}

/// `sizeof(my_memory_mgr)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_memory_mgr: usize = CLayout::structure(&[
    // struct jpeg_memory_mgr: eleven method pointers and two longs
    C_POINTER.array(11),
    C_LONG.array(2),
    C_POINTER.array(JPOOL_NUMPOOLS as usize),
    C_POINTER.array(JPOOL_NUMPOOLS as usize),
    C_POINTER,
    C_POINTER,
    C_SIZE_T,
    C_JDIMENSION,
])
.size;

// upstream: src/jmemmgr.c jvirt_barray_control
/// The control block of a virtual coefficient-block array.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct jvirt_barray_control {
    /// The in-memory buffer; `None` until `realize_virt_arrays`.
    pub mem_buffer: Option<BlockArray>,
    /// Total virtual array height.
    pub rows_in_array: JDIMENSION,
    /// Width of array (and of memory buffer).
    pub blocksperrow: JDIMENSION,
    /// Max rows accessed by `access_virt_barray`.
    pub maxaccess: JDIMENSION,
    /// Height of memory buffer.
    pub rows_in_mem: JDIMENSION,
    /// Allocation chunk size in `mem_buffer`.
    pub rowsperchunk: JDIMENSION,
    /// First logical row number in the buffer.
    pub cur_start_row: JDIMENSION,
    /// Row number of first uninitialized row.
    pub first_undef_row: JDIMENSION,
    /// Pre-zero mode requested?
    pub pre_zero: bool,
    /// Do current buffer contents need written?
    pub dirty: bool,
}

/// `sizeof(struct jvirt_barray_control)` on the target, with its
/// `b_s_open`, `next` and `b_s_info` (`backing_store_info` of `jmemsys.h`:
/// three method pointers, a `FILE *` and `char temp_name[64]`).
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_jvirt_barray_control: usize = CLayout::structure(&[
    C_POINTER,
    C_JDIMENSION.array(7),
    C_BOOLEAN.array(3),
    C_POINTER,
    CLayout::structure(&[C_POINTER.array(3), C_POINTER, CLayout::of::<u8>().array(64)]),
])
.size;

// upstream: src/jmemmgr.c out_of_memory
/// Reports an out-of-memory error.
fn out_of_memory(cinfo: j_common_ptr<'_, '_>, which: i32) -> ErrorExit {
    ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, which)
}

// upstream: src/jmemmgr.c first_pool_slop
#[expect(non_upper_case_globals, reason = "C table name")]
static first_pool_slop: [usize; JPOOL_NUMPOOLS as usize] = [
    1600,  /* first PERMANENT pool */
    16000, /* first IMAGE pool */
];

// upstream: src/jmemmgr.c extra_pool_slop
#[expect(non_upper_case_globals, reason = "C table name")]
static extra_pool_slop: [usize; JPOOL_NUMPOOLS as usize] = [
    0,    /* additional PERMANENT pools */
    5000, /* additional IMAGE pools */
];

// upstream: src/jmemmgr.c MIN_SLOP
/// Greater than 0 to avoid futile looping.
const MIN_SLOP: usize = 50;

#[expect(
    clippy::cast_possible_truncation,
    reason = "MAX_ALLOC_CHUNK fits size_t, as jinit_memory_mgr checks"
)]
const MAX_ALLOC_CHUNK_SIZE: usize = MAX_ALLOC_CHUNK as usize;

// upstream: src/jmemmgr.c alloc_small
/// Allocates a "small" object: accounts for it in a small pool of `pool_id`,
/// creating a pool with slop when none has room, and makes the caller's
/// reservation where C obtains the memory.
///
/// # Errors
///
/// `JERR_OUT_OF_MEMORY` (cases 7, 1 and 2) or `JERR_BAD_POOL_ID`.
#[expect(clippy::cast_sign_loss, reason = "pool_id is checked first")]
pub fn alloc_small(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    mut sizeofobject: usize,
    reserve: Reservation<'_>,
) -> Result<(), ErrorExit> {
    /*
     * Round up the requested size to a multiple of ALIGN_SIZE in order
     * to assure alignment for the next object allocated in the same pool
     * and so that algorithms can straddle outside the proper area up
     * to the next alignment.
     */
    if sizeofobject > MAX_ALLOC_CHUNK_SIZE {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        return Err(out_of_memory(cinfo, 7));
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE);

    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if (SIZEOF_small_pool_hdr + sizeofobject + ALIGN_SIZE - 1) > MAX_ALLOC_CHUNK_SIZE {
        return Err(out_of_memory(cinfo, 1)); /* request exceeds malloc's ability */
    }

    /* See if space is available in any existing pool */
    if !(0..JPOOL_NUMPOOLS).contains(&pool_id) {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id)); /* safety check */
    }
    let pool = pool_id as usize;
    let found = cinfo.mem().small_list[pool]
        .iter()
        .position(|hdr| hdr.bytes_left >= sizeofobject); /* found pool with enough space */

    let index = if let Some(index) = found {
        // C's object lies in the pool's memory already; the Rust storage is
        // reserved here, and a refusal is the small-pool failure.
        if !reserve() {
            return Err(out_of_memory(cinfo, 2));
        }
        index
    } else {
        /* Time to make a new pool */
        /* min_request is what we need now, slop is what will be leftover */
        let min_request = SIZEOF_small_pool_hdr + sizeofobject + ALIGN_SIZE - 1;
        let mut slop = if cinfo.mem().small_list[pool].is_empty() {
            /* first pool in class? */
            first_pool_slop[pool]
        } else {
            extra_pool_slop[pool]
        };
        /* Don't ask for more than MAX_ALLOC_CHUNK */
        if slop > MAX_ALLOC_CHUNK_SIZE - min_request {
            slop = MAX_ALLOC_CHUNK_SIZE - min_request;
        }
        /* Try to get space, if fail reduce slop and try again */
        loop {
            if jpeg_get_small(cinfo, min_request + slop, reserve) {
                break;
            }
            slop /= 2;
            if slop < MIN_SLOP {
                /* give up when it gets real small */
                return Err(out_of_memory(cinfo, 2)); /* jpeg_get_small failed */
            }
        }
        let mem = cinfo.mem_mut();
        mem.total_space_allocated = mem.total_space_allocated.wrapping_add(min_request + slop);
        /* Success, initialize the new pool header and add to end of list */
        mem.small_list[pool].push(small_pool_hdr {
            bytes_used: 0,
            bytes_left: sizeofobject + slop,
        });
        mem.small_list[pool].len() - 1
    };

    /* OK, allocate the object from the current pool */
    let hdr = &mut cinfo.mem_mut().small_list[pool][index];
    hdr.bytes_used += sizeofobject;
    hdr.bytes_left -= sizeofobject;
    Ok(())
}

// upstream: src/jmemmgr.c alloc_large
/// Allocates a "large" object: always a new pool of `pool_id`, whose memory
/// is the caller's reservation.
///
/// # Errors
///
/// `JERR_OUT_OF_MEMORY` (cases 8, 3 and 4) or `JERR_BAD_POOL_ID`.
#[expect(clippy::cast_sign_loss, reason = "pool_id is checked first")]
pub fn alloc_large(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    mut sizeofobject: usize,
    reserve: Reservation<'_>,
) -> Result<(), ErrorExit> {
    /*
     * Round up the requested size to a multiple of ALIGN_SIZE so that
     * algorithms can straddle outside the proper area up to the next
     * alignment.
     */
    if sizeofobject > MAX_ALLOC_CHUNK_SIZE {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        return Err(out_of_memory(cinfo, 8));
    }
    sizeofobject = round_up_pow2(sizeofobject, ALIGN_SIZE);

    /* Check for unsatisfiable request (do now to ensure no overflow below) */
    if (SIZEOF_large_pool_hdr + sizeofobject + ALIGN_SIZE - 1) > MAX_ALLOC_CHUNK_SIZE {
        return Err(out_of_memory(cinfo, 3)); /* request exceeds malloc's ability */
    }

    /* Always make a new pool */
    if !(0..JPOOL_NUMPOOLS).contains(&pool_id) {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id)); /* safety check */
    }

    if !jpeg_get_large(
        cinfo,
        sizeofobject + SIZEOF_large_pool_hdr + ALIGN_SIZE - 1,
        reserve,
    ) {
        return Err(out_of_memory(cinfo, 4)); /* jpeg_get_large failed */
    }
    let mem = cinfo.mem_mut();
    mem.total_space_allocated = mem
        .total_space_allocated
        .wrapping_add(sizeofobject + SIZEOF_large_pool_hdr + ALIGN_SIZE - 1);

    /* Success, initialize the new pool header and add to list */
    /* We maintain space counts in each pool header for statistical purposes,
     * even though they are not needed for allocation.
     */
    mem.large_list[pool_id as usize].insert(
        0,
        large_pool_hdr {
            bytes_used: sizeofobject,
            bytes_left: 0,
        },
    );
    Ok(())
}

/// `(T *)(*cinfo->mem->alloc_small) (cinfo, pool_id, sizeofobject)` for a
/// fixed-size object: the request is accounted with its C size
/// `sizeofobject`, and `value` is boxed infallibly (ADR 0005, "Owned
/// storage"). Not an upstream item.
///
/// # Errors
///
/// What `alloc_small` reports.
pub fn alloc_small_box<T>(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    sizeofobject: usize,
    value: T,
) -> Result<Box<T>, ErrorExit> {
    let alloc_small = cinfo.mem().pub_.alloc_small;
    alloc_small(cinfo, pool_id, sizeofobject, &mut || true)?;
    Ok(Box::new(value))
}

/// `(T *)(*cinfo->mem->alloc_small) (cinfo, pool_id, sizeofobject)` for an
/// array of `len` elements, each initialized to `value`: the request is
/// accounted with its C size, and the storage is reserved fallibly where C
/// obtains the memory. Not an upstream item.
///
/// # Errors
///
/// What `alloc_small` reports; a refused reservation is case 2.
pub fn alloc_small_vec<T: Clone>(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    sizeofobject: usize,
    len: usize,
    value: T,
) -> Result<Vec<T>, ErrorExit> {
    let alloc_small = cinfo.mem().pub_.alloc_small;
    let mut storage = Vec::new();
    alloc_small(cinfo, pool_id, sizeofobject, &mut || {
        storage.try_reserve_exact(len).is_ok()
    })?;
    storage.resize(len, value);
    Ok(storage)
}

/// `(T *)(*cinfo->mem->alloc_large) (cinfo, pool_id, sizeofobject)` for an
/// array of `len` elements, each initialized to `value`; see
/// [`alloc_small_vec`]. Not an upstream item.
///
/// # Errors
///
/// What `alloc_large` reports; a refused reservation is case 4.
pub fn alloc_large_vec<T: Clone>(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    sizeofobject: usize,
    len: usize,
    value: T,
) -> Result<Vec<T>, ErrorExit> {
    let alloc_large = cinfo.mem().pub_.alloc_large;
    let mut storage = Vec::new();
    alloc_large(cinfo, pool_id, sizeofobject, &mut || {
        storage.try_reserve_exact(len).is_ok()
    })?;
    storage.resize(len, value);
    Ok(storage)
}

/// `sizeof(J12SAMPLE)` (`short`) and `sizeof(J16SAMPLE)` (`unsigned short`),
/// which `alloc_sarray` still uses to size rows for those precisions before
/// it fails.
const SIZEOF_J12SAMPLE: usize = core::mem::size_of::<i16>();
const SIZEOF_J16SAMPLE: usize = core::mem::size_of::<u16>();

// upstream: src/jmemmgr.c alloc_sarray
/// Allocates a 2-D sample array: the row list as a small object and the rows
/// in chunks of large objects, each chunk at most `MAX_ALLOC_CHUNK` bytes.
/// The rows are one buffer with a stride of `samplesperrow` rounded up to
/// `2 * ALIGN_SIZE` samples; the chunking affects only the accounting (ADR
/// 0005, "Storage and row lists").
///
/// A data precision above 8 selects upstream's 12- and 16-bit branches,
/// which are not translated: both fail with `JERR_BAD_PRECISION`, as the
/// 16-bit branch does upstream without `D_LOSSLESS_SUPPORTED` (ADR 0005).
///
/// # Errors
///
/// `JERR_OUT_OF_MEMORY` (cases 5 and 9 here, and those of `alloc_small` and
/// `alloc_large`), `JERR_WIDTH_OVERFLOW`, `JERR_BAD_POOL_ID` or
/// `JERR_BAD_PRECISION`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between JDIMENSION, long and size_t"
)]
pub fn alloc_sarray(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    mut samplesperrow: JDIMENSION,
    numrows: JDIMENSION,
) -> Result<SampleArray, ErrorExit> {
    let data_precision = cinfo.data_precision;
    let sample_size = if data_precision > 12 {
        SIZEOF_J16SAMPLE
    } else if data_precision > 8 {
        SIZEOF_J12SAMPLE
    } else {
        core::mem::size_of::<JSAMPLE>()
    };

    /* Make sure each row is properly aligned */
    #[expect(clippy::manual_is_multiple_of, reason = "the C remainder test")]
    if (ALIGN_SIZE % sample_size) != 0 {
        return Err(out_of_memory(cinfo, 5)); /* safety check */
    }

    if samplesperrow == 0 || c_long::from(samplesperrow) > MAX_ALLOC_CHUNK {
        /* This prevents overflow/wrap-around in round_up_pow2() if sizeofobject
        is close to SIZE_MAX. */
        return Err(out_of_memory(cinfo, 9));
    }
    samplesperrow =
        round_up_pow2(samplesperrow as usize, (2 * ALIGN_SIZE) / sample_size) as JDIMENSION;

    /* Calculate max # of rows allowed in one allocation chunk */
    let ltemp = ((MAX_ALLOC_CHUNK_SIZE - SIZEOF_large_pool_hdr)
        / ((c_long::from(samplesperrow) * sample_size as c_long) as usize))
        as c_long;
    if ltemp <= 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW));
    }
    let mut rowsperchunk = if ltemp < c_long::from(numrows) {
        ltemp as JDIMENSION
    } else {
        numrows
    };
    cinfo.mem_mut().last_rowsperchunk = rowsperchunk;

    if data_precision <= 8 {
        /* Get space for row pointers (small object) */
        let mut rows: Vec<usize> = Vec::new();
        alloc_small(
            cinfo,
            pool_id,
            (numrows as usize).wrapping_mul(SIZEOF_JSAMPROW),
            &mut || rows.try_reserve_exact(numrows as usize).is_ok(),
        )?;

        /* Get the rows themselves (large objects) */
        let mut buffer: Vec<JSAMPLE> = Vec::new();
        let mut currow: JDIMENSION = 0;
        while currow < numrows {
            rowsperchunk = MIN(rowsperchunk, numrows - currow);
            let chunk = rowsperchunk as usize * samplesperrow as usize;
            alloc_large(
                cinfo,
                pool_id,
                (rowsperchunk as usize)
                    .wrapping_mul(samplesperrow as usize)
                    .wrapping_mul(sample_size),
                &mut || buffer.try_reserve_exact(chunk).is_ok(),
            )?;
            let mut workspace = buffer.len();
            buffer.resize(workspace + chunk, 0);
            for _ in 0..rowsperchunk {
                rows.push(workspace);
                currow += 1;
                workspace += samplesperrow as usize;
            }
        }

        Ok(SampleArray { buffer, rows })
    } else if data_precision <= 12 {
        // Upstream allocates a J12SAMPARRAY here; 12-bit samples are not
        // translated (ADR 0005).
        Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ))
    } else {
        Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ))
    }
}

// upstream: src/jmemmgr.c alloc_barray
/// Allocates a 2-D coefficient-block array; see [`alloc_sarray`].
///
/// # Errors
///
/// `JERR_OUT_OF_MEMORY` (case 6 here, and those of `alloc_small` and
/// `alloc_large`), `JERR_WIDTH_OVERFLOW` or `JERR_BAD_POOL_ID`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between JDIMENSION, long and size_t"
)]
pub fn alloc_barray(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    blocksperrow: JDIMENSION,
    numrows: JDIMENSION,
) -> Result<BlockArray, ErrorExit> {
    /* Make sure each row is properly aligned */
    #[expect(clippy::manual_is_multiple_of, reason = "the C remainder test")]
    if blocksperrow == 0 || (SIZEOF_JBLOCK % ALIGN_SIZE) != 0 {
        return Err(out_of_memory(cinfo, 6)); /* safety check */
    }

    /* Calculate max # of rows allowed in one allocation chunk */
    let ltemp = ((MAX_ALLOC_CHUNK_SIZE - SIZEOF_large_pool_hdr)
        / (c_long::from(blocksperrow) as usize).wrapping_mul(SIZEOF_JBLOCK))
        as c_long;
    if ltemp <= 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_WIDTH_OVERFLOW));
    }
    let mut rowsperchunk = if ltemp < c_long::from(numrows) {
        ltemp as JDIMENSION
    } else {
        numrows
    };
    cinfo.mem_mut().last_rowsperchunk = rowsperchunk;

    /* Get space for row pointers (small object) */
    let mut rows: Vec<usize> = Vec::new();
    alloc_small(
        cinfo,
        pool_id,
        (numrows as usize).wrapping_mul(SIZEOF_JBLOCKROW),
        &mut || rows.try_reserve_exact(numrows as usize).is_ok(),
    )?;

    /* Get the rows themselves (large objects) */
    let mut buffer = Vec::new();
    let mut currow: JDIMENSION = 0;
    while currow < numrows {
        rowsperchunk = MIN(rowsperchunk, numrows - currow);
        let chunk = rowsperchunk as usize * blocksperrow as usize;
        alloc_large(
            cinfo,
            pool_id,
            (rowsperchunk as usize)
                .wrapping_mul(blocksperrow as usize)
                .wrapping_mul(SIZEOF_JBLOCK),
            &mut || buffer.try_reserve_exact(chunk).is_ok(),
        )?;
        let mut workspace = buffer.len();
        buffer.resize(workspace + chunk, [0; 64]);
        for _ in 0..rowsperchunk {
            rows.push(workspace);
            currow += 1;
            workspace += blocksperrow as usize;
        }
    }

    Ok(BlockArray { buffer, rows })
}

// upstream: src/jmemmgr.c request_virt_barray
/// Requests a virtual 2-D coefficient-block array; the in-memory buffer is
/// allocated by `realize_virt_arrays`.
///
/// # Errors
///
/// `JERR_BAD_POOL_ID` unless `pool_id` is `JPOOL_IMAGE`, or what
/// `alloc_small` reports for the control block.
pub fn request_virt_barray(
    cinfo: j_common_ptr<'_, '_>,
    pool_id: i32,
    pre_zero: bool,
    blocksperrow: JDIMENSION,
    numrows: JDIMENSION,
    maxaccess: JDIMENSION,
) -> Result<jvirt_barray_ptr, ErrorExit> {
    /* Only IMAGE-lifetime virtual arrays are currently supported */
    if pool_id != JPOOL_IMAGE {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id)); /* safety check */
    }

    /* get control block */
    alloc_small(cinfo, pool_id, SIZEOF_jvirt_barray_control, &mut || true)?;

    let mem = cinfo.mem_mut();
    mem.virt_barray_list.push(jvirt_barray_control {
        mem_buffer: None, /* marks array not yet realized */
        rows_in_array: numrows,
        blocksperrow,
        maxaccess,
        pre_zero,
        ..jvirt_barray_control::default()
    }); /* add to list of virtual arrays */

    Ok(jvirt_barray_ptr(mem.virt_barray_list.len() - 1))
}

// upstream: src/jmemmgr.c realize_virt_arrays
/// Allocates the in-memory buffers for any unrealized virtual arrays, in
/// upstream's list order (the most recently requested first). An array that
/// does not fit in the memory that `jpeg_mem_available` grants would need a
/// backing store, so the call fails there with `JERR_NO_BACKING_STORE`.
///
/// # Errors
///
/// `JERR_OUT_OF_MEMORY` (case 11, and those of `alloc_barray`),
/// `JERR_NO_BACKING_STORE` or `JERR_WIDTH_OVERFLOW`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between JDIMENSION, long and size_t"
)]
pub fn realize_virt_arrays(cinfo: j_common_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Compute the minimum space needed (maxaccess rows in each buffer)
     * and the maximum space needed (full image height in each buffer).
     * These may be of use to the system-dependent jpeg_mem_available routine.
     */
    let mut space_per_minheight: usize = 0;
    let mut maximum_space: usize = 0;
    let count = cinfo.mem().virt_barray_list.len();
    for index in (0..count).rev() {
        let bptr = &cinfo.mem().virt_barray_list[index];
        if bptr.mem_buffer.is_none() {
            /* if not realized yet */
            let new_space = (bptr.rows_in_array as usize)
                .wrapping_mul(bptr.blocksperrow as usize)
                .wrapping_mul(SIZEOF_JBLOCK);

            space_per_minheight = space_per_minheight.wrapping_add(
                (bptr.maxaccess as usize)
                    .wrapping_mul(bptr.blocksperrow as usize)
                    .wrapping_mul(SIZEOF_JBLOCK),
            );
            if usize::MAX - maximum_space < new_space {
                return Err(out_of_memory(cinfo, 11));
            }
            maximum_space += new_space;
        }
    }

    if space_per_minheight == 0 {
        return Ok(()); /* no unrealized arrays, no work */
    }

    /* Determine amount of memory to actually use; this is system-dependent. */
    let already_allocated = cinfo.mem().total_space_allocated;
    let avail_mem =
        jpeg_mem_available(cinfo, space_per_minheight, maximum_space, already_allocated);

    /* If the maximum space needed is available, make all the buffers full
     * height; otherwise parcel it out with the same number of minheights
     * in each buffer.
     */
    let max_minheights: usize = if avail_mem >= maximum_space {
        1_000_000_000
    } else {
        /* If there doesn't seem to be enough space, try to get the minimum
         * anyway.  This allows a "stub" implementation of jpeg_mem_available().
         */
        match avail_mem / space_per_minheight {
            0 => 1,
            max_minheights => max_minheights,
        }
    };

    /* Allocate the in-memory buffers and initialize backing store as needed. */

    for index in (0..count).rev() {
        let bptr = &cinfo.mem().virt_barray_list[index];
        if bptr.mem_buffer.is_none() {
            /* if not realized yet */
            let rows_in_array = bptr.rows_in_array;
            let blocksperrow = bptr.blocksperrow;
            let maxaccess = bptr.maxaccess;
            let minheights =
                ((c_long::from(rows_in_array) - 1) / c_long::from(maxaccess) + 1) as usize;
            if minheights <= max_minheights {
                /* This buffer fits in memory */
                cinfo.mem_mut().virt_barray_list[index].rows_in_mem = rows_in_array;
            } else {
                /* It doesn't fit in memory, create backing store. */
                cinfo.mem_mut().virt_barray_list[index].rows_in_mem =
                    max_minheights.wrapping_mul(maxaccess as usize) as JDIMENSION;
                return Err(jpeg_open_backing_store(
                    cinfo,
                    (rows_in_array as usize)
                        .wrapping_mul(blocksperrow as usize)
                        .wrapping_mul(SIZEOF_JBLOCK) as c_long,
                ));
            }
            let rows_in_mem = cinfo.mem().virt_barray_list[index].rows_in_mem;
            let mem_buffer = alloc_barray(cinfo, JPOOL_IMAGE, blocksperrow, rows_in_mem)?;
            let mem = cinfo.mem_mut();
            let rowsperchunk = mem.last_rowsperchunk;
            let bptr = &mut mem.virt_barray_list[index];
            bptr.mem_buffer = Some(mem_buffer);
            bptr.rowsperchunk = rowsperchunk;
            bptr.cur_start_row = 0;
            bptr.first_undef_row = 0;
            bptr.dirty = false;
        }
    }
    Ok(())
}

// upstream: src/jmemmgr.c access_virt_barray
/// Accesses the part of a virtual block array starting at `start_row` and
/// extending for `num_rows` rows; `writable` is true if the caller intends to
/// modify the accessed area. Returns the index of `start_row` in the array's
/// in-memory buffer, the offset that C adds to `mem_buffer`.
///
/// Without a backing store the buffer always holds the whole array, so a
/// request outside it is `JERR_VIRTUAL_BUG`, as upstream reports it when
/// `b_s_open` is `FALSE`.
///
/// # Errors
///
/// `JERR_BAD_VIRTUAL_ACCESS` or `JERR_VIRTUAL_BUG`.
///
/// # Panics
///
/// When `ptr` names no array, where C would follow a dangling pointer.
pub fn access_virt_barray(
    cinfo: j_common_ptr<'_, '_>,
    ptr: jvirt_barray_ptr,
    start_row: JDIMENSION,
    num_rows: JDIMENSION,
    writable: bool,
) -> Result<usize, ErrorExit> {
    let mut end_row = start_row.wrapping_add(num_rows);

    /* debugging check */
    let bptr = &cinfo.mem().virt_barray_list[ptr.0];
    if end_row > bptr.rows_in_array || num_rows > bptr.maxaccess || bptr.mem_buffer.is_none() {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS));
    }

    /* Make the desired part of the virtual array accessible */
    if start_row < bptr.cur_start_row || end_row > bptr.cur_start_row.wrapping_add(bptr.rows_in_mem)
    {
        // !ptr->b_s_open always holds without a backing store
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_VIRTUAL_BUG));
    }
    /* Ensure the accessed part of the array is defined; prezero if needed.
     * To improve locality of access, we only prezero the part of the array
     * that the caller is about to access, not the entire in-memory array.
     */
    if bptr.first_undef_row < end_row {
        let mut undef_row = if bptr.first_undef_row < start_row {
            if writable {
                /* writer skipped over a section of array */
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS));
            }
            start_row /* but reader is allowed to read ahead */
        } else {
            bptr.first_undef_row
        };
        if !bptr.pre_zero && !writable {
            /* reader looking at undefined data */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_VIRTUAL_ACCESS));
        }
        let bptr = &mut cinfo.mem_mut().virt_barray_list[ptr.0];
        if writable {
            bptr.first_undef_row = end_row;
        }
        if bptr.pre_zero {
            let bytesperrow = bptr.blocksperrow as usize * SIZEOF_JBLOCK;
            undef_row = undef_row.wrapping_sub(bptr.cur_start_row); /* make indexes relative to buffer */
            end_row = end_row.wrapping_sub(bptr.cur_start_row);
            let mem_buffer = bptr.mem_buffer.as_mut().expect("realized above");
            while undef_row < end_row {
                jzero_far(mem_buffer.row_mut(undef_row as usize), bytesperrow);
                undef_row += 1;
            }
        }
    }
    let bptr = &mut cinfo.mem_mut().virt_barray_list[ptr.0];
    /* Flag the buffer dirty if caller will write in it */
    if writable {
        bptr.dirty = true;
    }
    /* Return address of proper part of the buffer */
    Ok(start_row.wrapping_sub(bptr.cur_start_row) as usize)
}

// upstream: src/jmemmgr.c free_pool
/// Releases all objects belonging to a specified pool: drops its pool
/// records, lowering `total_space_allocated` as C does, and resets the fields
/// of the object that upstream fills from the pool (see
/// [`release_pool_fields`]), which drops their storage.
///
/// # Errors
///
/// `JERR_BAD_POOL_ID`.
#[expect(clippy::cast_sign_loss, reason = "pool_id is checked first")]
pub fn free_pool(cinfo: j_common_ptr<'_, '_>, pool_id: i32) -> Result<(), ErrorExit> {
    if !(0..JPOOL_NUMPOOLS).contains(&pool_id) {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_POOL_ID, pool_id)); /* safety check */
    }
    let pool = pool_id as usize;

    /* If freeing IMAGE pool, close any virtual arrays first */
    if pool_id == JPOOL_IMAGE {
        // No backing store is ever open, so there is nothing to close.
        cinfo.mem_mut().virt_barray_list = Vec::new();
    }

    /* Release large objects */
    let large_list = core::mem::take(&mut cinfo.mem_mut().large_list[pool]);
    for lhdr in large_list {
        let space_freed =
            lhdr.bytes_used + lhdr.bytes_left + SIZEOF_large_pool_hdr + ALIGN_SIZE - 1;
        jpeg_free_large(cinfo, lhdr, space_freed);
        let mem = cinfo.mem_mut();
        mem.total_space_allocated = mem.total_space_allocated.wrapping_sub(space_freed);
    }

    /* Release small objects */
    let small_list = core::mem::take(&mut cinfo.mem_mut().small_list[pool]);
    for shdr in small_list {
        let space_freed =
            shdr.bytes_used + shdr.bytes_left + SIZEOF_small_pool_hdr + ALIGN_SIZE - 1;
        jpeg_free_small(cinfo, shdr, space_freed);
        let mem = cinfo.mem_mut();
        mem.total_space_allocated = mem.total_space_allocated.wrapping_sub(space_freed);
    }

    release_pool_fields(cinfo, pool_id);
    Ok(())
}

/// Resets the fields of the object that upstream fills from pool `pool_id`,
/// so that their storage is dropped with the pool (ADR 0005, "Pools").
/// `tools/pools.json` lists every allocation of the in-scope upstream files
/// with the field it fills; a package adds its fields here when it adds them
/// to the object. Not an upstream item.
pub fn release_pool_fields(cinfo: j_common_ptr<'_, '_>, pool_id: i32) {
    if pool_id == JPOOL_PERMANENT {
        // jpeg_alloc_quant_table and jpeg_alloc_huff_table (jdmarker.c,
        // jstdhuff.c), the source manager of jdatasrc.c, and the modules
        // that jpeg_CreateDecompress creates.
        cinfo.quant_tbl_ptrs = Default::default();
        cinfo.dc_huff_tbl_ptrs = Default::default();
        cinfo.ac_huff_tbl_ptrs = Default::default();
        cinfo.src = None;
        cinfo.master = None;
        cinfo.inputctl = None;
        cinfo.marker = None;
    }
    if pool_id == JPOOL_IMAGE {
        // The virtual block arrays are released by free_pool. A marker being
        // saved belongs to the image pool although the permanent marker
        // reader holds it; reset_marker_reader drops it in C's next use.
        cinfo.comp_info = None;
        cinfo.marker_list = None;
        cinfo.coef_bits = None;
        cinfo.entropy = None;
        cinfo.coef = None;
        if let Some(marker) = cinfo.marker.as_deref_mut() {
            marker.cur_marker = None;
        }
    }
}

// upstream: src/jmemmgr.c self_destruct
/// Closes up shop entirely: releases both pools in reverse order and the
/// memory manager itself, leaving `cinfo.mem` `None`.
///
/// # Errors
///
/// None in practice: `free_pool` fails only for an invalid pool.
pub fn self_destruct(cinfo: j_common_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Close all backing store, release all memory.
     * Releasing pools in reverse order might help avoid fragmentation
     * with some (brain-damaged) malloc libraries.
     */
    for pool in (JPOOL_PERMANENT..JPOOL_NUMPOOLS).rev() {
        free_pool(cinfo, pool)?;
    }

    /* Release the memory manager control block too. */
    let mem = cinfo.mem.take(); /* ensures I will be called only once */
    jpeg_free_small(cinfo, mem, SIZEOF_my_memory_mgr);

    jpeg_mem_term(cinfo); /* system-dependent cleanup */
    Ok(())
}

/// `errno` values of `GETENV_S`; the same on macOS and Linux.
const EINVAL: i32 = 22;
const ERANGE: i32 = 34;

// upstream: src/jinclude.h GETENV_S
/// Copies the environment variable `name` into `buffer` as a NUL-terminated
/// string, like C11's `getenv_s`. `buffer` and `name` are never null here,
/// and `errno` is not set.
#[expect(non_snake_case, reason = "C function name")]
fn GETENV_S(buffer: &mut [u8], name: &str) -> i32 {
    if buffer.is_empty() {
        return EINVAL;
    }

    let Some(env) = std::env::var_os(name) else {
        buffer[0] = 0;
        return 0;
    };
    let env = env.as_encoded_bytes();

    if env.len() + 1 > buffer.len() {
        buffer[0] = 0;
        return ERANGE;
    }

    let length = buffer.len();
    let copied = env.len().min(length);
    buffer[..copied].copy_from_slice(&env[..copied]);
    buffer[copied..].fill(0);

    0
}

/// `sscanf(input, "%ld%c", value, ch)` on the NUL-terminated `input`: the
/// number of conversions, or -1 (`EOF`) when the input ends before the
/// first. A value out of range saturates, as the C libraries' `strtol` does.
fn sscanf_ld_c(input: &[u8], value: &mut c_long, ch: &mut u8) -> i32 {
    let length = input
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(input.len());
    let input = &input[..length];
    let mut index = 0;
    while index < input.len() && matches!(input[index], b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
    {
        index += 1;
    }
    if index == input.len() {
        return -1;
    }
    let negative = input[index] == b'-';
    if matches!(input[index], b'+' | b'-') {
        index += 1;
    }
    let digits = index;
    let mut magnitude: u128 = 0;
    while index < input.len() && input[index].is_ascii_digit() {
        magnitude =
            (magnitude * 10 + u128::from(input[index] - b'0')).min(u128::from(u64::MAX) + 1);
        index += 1;
    }
    if index == digits {
        return 0;
    }
    *value = if negative {
        c_long::try_from(magnitude).map_or(c_long::MIN, |magnitude| -magnitude)
    } else {
        c_long::try_from(magnitude).unwrap_or(c_long::MAX)
    };
    if index < input.len() {
        *ch = input[index];
        2
    } else {
        1
    }
}

// upstream: src/jmemmgr.c jinit_memory_mgr
/// Memory manager initialization. Reads `JPEGMEM` from the environment as
/// upstream does (ADR 0005, decision 2): a number of kilobytes, or of
/// megabytes when followed by `m` or `M`, that overrides `max_memory_to_use`.
///
/// # Errors
///
/// `JERR_BAD_ALIGN_TYPE`, `JERR_BAD_ALLOC_CHUNK` or `JERR_OUT_OF_MEMORY`
/// (case 0); none of them can occur on the CI hosts.
pub fn jinit_memory_mgr(cinfo: j_common_ptr<'_, '_>) -> Result<(), ErrorExit> {
    cinfo.mem = None; /* for safety if init fails */

    /* Check for configuration errors.
     * sizeof(ALIGN_TYPE) should be a power of 2; otherwise, it probably
     * doesn't reflect any real hardware alignment requirement.
     * The test is a little tricky: for X>0, X and X-1 have no one-bits
     * in common if and only if X is a power of 2, ie has only one one-bit.
     * Some compilers may give an "unreachable code" warning here; ignore it.
     */
    if (ALIGN_SIZE & (ALIGN_SIZE - 1)) != 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_ALIGN_TYPE));
    }
    /* MAX_ALLOC_CHUNK must be representable as type size_t, and must be
     * a multiple of ALIGN_SIZE.
     * Again, an "unreachable code" warning may be ignored here.
     * But a "constant too large" warning means you need to fix MAX_ALLOC_CHUNK.
     */
    let test_mac = MAX_ALLOC_CHUNK_SIZE;
    #[expect(clippy::manual_is_multiple_of, reason = "the C remainder test")]
    if c_long::try_from(test_mac) != Ok(MAX_ALLOC_CHUNK) || (MAX_ALLOC_CHUNK_SIZE % ALIGN_SIZE) != 0
    {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_ALLOC_CHUNK));
    }

    let mut max_to_use = jpeg_mem_init(cinfo); /* system-dependent initialization */

    /* Attempt to allocate memory manager's control block */
    if !jpeg_get_small(cinfo, SIZEOF_my_memory_mgr, &mut || true) {
        jpeg_mem_term(cinfo); /* system-dependent cleanup */
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, 0));
    }

    let mem = Box::new(my_memory_mgr {
        /* OK, fill in the method pointers */
        pub_: jpeg_memory_mgr {
            alloc_small,
            alloc_large,
            alloc_sarray,
            alloc_barray,
            request_virt_barray,
            realize_virt_arrays,
            access_virt_barray,
            free_pool,
            self_destruct,
            /* Initialize working state */
            max_memory_to_use: max_to_use,
            /* Make MAX_ALLOC_CHUNK accessible to other modules */
            max_alloc_chunk: MAX_ALLOC_CHUNK,
        },
        small_list: Default::default(),
        large_list: Default::default(),
        virt_barray_list: Vec::new(),
        total_space_allocated: SIZEOF_my_memory_mgr,
        last_rowsperchunk: 0,
    });

    /* Declare ourselves open for business */
    cinfo.mem = Some(mem);

    /* Check for an environment variable JPEGMEM; if found, override the
     * default max_memory setting from jpeg_mem_init.  Note that the
     * surrounding application may again override this value.
     * If your system doesn't support getenv(), define NO_GETENV to disable
     * this feature.
     */
    let mut memenv = [0u8; 30];

    if GETENV_S(&mut memenv, "JPEGMEM") == 0 && memenv[0] != 0 {
        let mut ch = b'x';

        if sscanf_ld_c(&memenv, &mut max_to_use, &mut ch) > 0 {
            if ch == b'm' || ch == b'M' {
                max_to_use *= 1000;
            }
            cinfo.mem_mut().pub_.max_memory_to_use = max_to_use * 1000;
        }
    }

    Ok(())
}
