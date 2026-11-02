// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Canonical profile/tag representations and complete ICC parsing from pinned
//! src/skcms_public.h, src/skcms_internals.h and skcms.cc:355-1728.
//! ICC bytes, curve tables and color grids remain borrowed at their original
//! addresses. Raw operations retain source zeroing, partial writes and guards.

use super::curve::{Curve, canonicalize_identity, copy_curve};
use super::transfer_function::TransferFunction;
use super::types::{Matrix3x3, Matrix3x4};
use std::mem::{MaybeUninit, align_of, size_of};
use std::ptr;

const fn padding(offset: usize, alignment: usize) -> usize {
    (alignment - offset % alignment) % alignment
}
const ZERO_MATRIX_3: Matrix3x3 = Matrix3x3 {
    vals: [[0.0; 3]; 3],
};
const ZERO_MATRIX_4: Matrix3x4 = Matrix3x4 {
    vals: [[0.0; 4]; 3],
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct A2B {
    pub input_curves: [Curve; 4],
    pub grid_8: *const u8,
    pub grid_16: *const u8,
    pub input_channels: u32,
    pub grid_points: [u8; 4],
    pub matrix_curves: [Curve; 3],
    pub matrix: Matrix3x4,
    pub matrix_channels: u32,
    pub output_channels: u32,
    pub output_curves: [Curve; 3],
}
impl A2B {
    pub const fn zeroed() -> Self {
        Self {
            input_curves: [Curve::zeroed(); 4],
            grid_8: ptr::null(),
            grid_16: ptr::null(),
            input_channels: 0,
            grid_points: [0; 4],
            matrix_curves: [Curve::zeroed(); 3],
            matrix: ZERO_MATRIX_4,
            matrix_channels: 0,
            output_channels: 0,
            output_curves: [Curve::zeroed(); 3],
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct B2A {
    pub input_curves: [Curve; 3],
    pub input_channels: u32,
    pub matrix_channels: u32,
    pub matrix_curves: [Curve; 3],
    pub matrix: Matrix3x4,
    pub output_curves: [Curve; 4],
    pub grid_8: *const u8,
    pub grid_16: *const u8,
    pub grid_points: [u8; 4],
    pub output_channels: u32,
}
impl B2A {
    pub const fn zeroed() -> Self {
        Self {
            input_curves: [Curve::zeroed(); 3],
            input_channels: 0,
            matrix_channels: 0,
            matrix_curves: [Curve::zeroed(); 3],
            matrix: ZERO_MATRIX_4,
            output_curves: [Curve::zeroed(); 4],
            grid_8: ptr::null(),
            grid_16: ptr::null(),
            grid_points: [0; 4],
            output_channels: 0,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Cicp {
    pub color_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub video_full_range_flag: u8,
}
const PROFILE_HEADER_PADDING: usize = padding(size_of::<*const u8>(), align_of::<u32>());
const PROFILE_MATRIX_END: usize = size_of::<*const u8>()
    + PROFILE_HEADER_PADDING
    + 4 * size_of::<u32>()
    + 3 * size_of::<Curve>()
    + size_of::<Matrix3x3>();
const PROFILE_MATRIX_PADDING: usize = padding(PROFILE_MATRIX_END, align_of::<A2B>());
const PROFILE_FLAGS_END: usize = PROFILE_MATRIX_END
    + PROFILE_MATRIX_PADDING
    + size_of::<A2B>()
    + size_of::<B2A>()
    + size_of::<Cicp>()
    + 5 * size_of::<bool>();
const PROFILE_TAIL_PADDING: usize = padding(PROFILE_FLAGS_END, align_of::<A2B>());

/// Source skcms_ICCProfile. Every pointer borrows caller-owned ICC/table bytes;
/// a bitwise copy transfers no ownership or lifetime extension.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IccProfile {
    pub buffer: *const u8,
    header_padding: [MaybeUninit<u8>; PROFILE_HEADER_PADDING],
    pub size: u32,
    pub data_color_space: u32,
    pub pcs: u32,
    pub tag_count: u32,
    pub trc: [Curve; 3],
    pub to_xyz_d50: Matrix3x3,
    matrix_padding: [MaybeUninit<u8>; PROFILE_MATRIX_PADDING],
    pub a2b: A2B,
    pub b2a: B2A,
    pub cicp: Cicp,
    pub has_trc: bool,
    pub has_to_xyz_d50: bool,
    pub has_a2b: bool,
    pub has_b2a: bool,
    pub has_cicp: bool,
    tail_padding: [MaybeUninit<u8>; PROFILE_TAIL_PADDING],
}
impl IccProfile {
    pub const fn zeroed() -> Self {
        Self {
            buffer: ptr::null(),
            header_padding: [MaybeUninit::new(0); PROFILE_HEADER_PADDING],
            size: 0,
            data_color_space: 0,
            pcs: 0,
            tag_count: 0,
            trc: [Curve::zeroed(); 3],
            to_xyz_d50: ZERO_MATRIX_3,
            matrix_padding: [MaybeUninit::new(0); PROFILE_MATRIX_PADDING],
            a2b: A2B::zeroed(),
            b2a: B2A::zeroed(),
            cicp: Cicp {
                color_primaries: 0,
                transfer_characteristics: 0,
                matrix_coefficients: 0,
                video_full_range_flag: 0,
            },
            has_trc: false,
            has_to_xyz_d50: false,
            has_a2b: false,
            has_b2a: false,
            has_cicp: false,
            tail_padding: [MaybeUninit::new(0); PROFILE_TAIL_PADDING],
        }
    }
}
const TAG_POINTER_PADDING: usize = padding(3 * size_of::<u32>(), align_of::<*const u8>());
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IccTag {
    pub signature: u32,
    pub kind: u32,
    pub size: u32,
    pointer_padding: [MaybeUninit<u8>; TAG_POINTER_PADDING],
    pub buffer: *const u8,
}
impl IccTag {
    pub const fn zeroed() -> Self {
        Self {
            signature: 0,
            kind: 0,
            size: 0,
            pointer_padding: [MaybeUninit::new(0); TAG_POINTER_PADDING],
            buffer: ptr::null(),
        }
    }
}

pub const SIGNATURE_CMYK: u32 = 0x434d594b;
pub const SIGNATURE_GRAY: u32 = 0x47524159;
pub const SIGNATURE_RGB: u32 = 0x52474220;
pub const SIGNATURE_LAB: u32 = 0x4c616220;
pub const SIGNATURE_XYZ: u32 = 0x58595a20;
const SIGNATURE_ACSP: u32 = 0x61637370;
const SIGNATURE_RTRC: u32 = 0x72545243;
const SIGNATURE_GTRC: u32 = 0x67545243;
const SIGNATURE_BTRC: u32 = 0x62545243;
const SIGNATURE_KTRC: u32 = 0x6b545243;
const SIGNATURE_RXYZ: u32 = 0x7258595a;
const SIGNATURE_GXYZ: u32 = 0x6758595a;
const SIGNATURE_BXYZ: u32 = 0x6258595a;
const SIGNATURE_A2B0: u32 = 0x41324230;
const SIGNATURE_B2A0: u32 = 0x42324130;
const SIGNATURE_CHAD: u32 = 0x63686164;
const SIGNATURE_WTPT: u32 = 0x77747074;
const SIGNATURE_CICP: u32 = 0x63696370;
const SIGNATURE_CURV: u32 = 0x63757276;
const SIGNATURE_MFT1: u32 = 0x6d667431;
const SIGNATURE_MFT2: u32 = 0x6d667432;
const SIGNATURE_MAB: u32 = 0x6d414220;
const SIGNATURE_MBA: u32 = 0x6d424120;
const SIGNATURE_PARA: u32 = 0x70617261;
const SIGNATURE_SF32: u32 = 0x73663332;

// Every reader is called only after the source's size guard establishes the
// reached physical bytes. Native memcpy + unconditional swap matches source.
unsafe fn read_big_u16(pointer: *const u8) -> u16 {
    // SAFETY: caller supplies two readable bytes, without alignment promises.
    unsafe { pointer.cast::<u16>().read_unaligned().swap_bytes() }
}
unsafe fn read_big_u32(pointer: *const u8) -> u32 {
    // SAFETY: caller supplies four readable bytes, without alignment promises.
    unsafe { pointer.cast::<u32>().read_unaligned().swap_bytes() }
}
unsafe fn read_big_fixed(pointer: *const u8) -> f32 {
    // SAFETY: caller supplies the source's four-byte signed fixed-point word.
    unsafe { read_big_u32(pointer) as i32 as f32 * (1.0 / 65536.0) }
}

/// Original skcms_Init, including all representation bytes.
///
/// # Safety
/// Profile must point to aligned writable storage for a complete IccProfile.
/// Replacing its borrowed pointers does not release their external owners.
pub unsafe fn init(profile: *mut IccProfile) {
    // SAFETY: caller provides a complete writable object. Zero is valid for
    // every source field; MaybeUninit padding is initialized by this memset.
    unsafe {
        ptr::write_bytes(
            profile.cast::<MaybeUninit<u8>>(),
            0,
            size_of::<IccProfile>(),
        );
        ptr::addr_of_mut!((*profile).data_color_space).write(SIGNATURE_RGB);
        ptr::addr_of_mut!((*profile).pcs).write(SIGNATURE_XYZ);
    }
}

/// Original programmatic transfer-function setter.
///
/// # Safety
/// Profile's reached fields are writable; function points to seven initialized
/// words. Existing borrowed storage remains externally owned. Valid source
/// self-aliases are allowed; each iteration reads the function at its checkpoint.
pub unsafe fn set_transfer_function(profile: *mut IccProfile, function: *const TransferFunction) {
    // SAFETY: caller guarantees writable flags/curves and readable function;
    // no exclusive Rust borrow of the profile is held across an aliased read.
    unsafe {
        ptr::addr_of_mut!((*profile).has_trc).write(true);
        for i in 0..3 {
            let curve = ptr::addr_of_mut!((*profile).trc).cast::<Curve>().add(i);
            curve.cast::<u32>().write(0);
            ptr::addr_of_mut!((*curve).parametric.function).write(function.read());
        }
    }
}

/// Original programmatic gamut-matrix setter.
///
/// # Safety
/// Profile's flag and matrix are writable, and matrix holds nine initialized
/// words. Source-compatible self-aliases are allowed; no payload owner changes.
pub unsafe fn set_xyz_d50(profile: *mut IccProfile, matrix: *const Matrix3x3) {
    // SAFETY: caller guarantees the reached fields; raw read then write also
    // supports the original in-place matrix assignment without aliasing refs.
    unsafe {
        ptr::addr_of_mut!((*profile).has_to_xyz_d50).write(true);
        ptr::addr_of_mut!((*profile).to_xyz_d50).write(matrix.read());
    }
}

/// Original tag lookup, including its literal idx > tag_count guard.
///
/// # Safety
/// Non-null profile must have an initialized buffer and tag_count. All reached
/// tag-table and payload reads must be physically readable, even for the source
/// idx==tag_count case; successful output fields must be writable. Profile/ICC
/// storage and retained pointers must remain live. Null outputs follow source
/// guards; this does not validate an arbitrary failed-parse profile for lookup.
pub unsafe fn get_tag_by_index(profile: *const IccProfile, idx: u32, tag: *mut IccTag) {
    // SAFETY: short-circuit order matches source; reached metadata is initialized.
    if profile.is_null() || unsafe { (*profile).buffer.is_null() } || tag.is_null() {
        return;
    }
    // SAFETY: caller guarantees this metadata read.
    if idx > unsafe { (*profile).tag_count } {
        return;
    }
    // SAFETY: caller supplies the reached physical row/payload, not merely the
    // declared count. Each field access preserves the source's read/write order.
    unsafe {
        let row = (*profile).buffer.add(132 + idx as usize * 12);
        ptr::addr_of_mut!((*tag).signature).write(read_big_u32(row));
        ptr::addr_of_mut!((*tag).size).write(read_big_u32(row.add(8)));
        ptr::addr_of_mut!((*tag).buffer)
            .write((*profile).buffer.add(read_big_u32(row.add(4)) as usize));
        ptr::addr_of_mut!((*tag).kind).write(read_big_u32((*tag).buffer));
    }
}

/// Original first-matching signature lookup; unsuccessful output is untouched.
///
/// # Safety
/// The same reached-storage/lifetime contract as get_tag_by_index applies to all
/// scanned declared rows. Output may be null as guarded by the original.
pub unsafe fn get_tag_by_signature(
    profile: *const IccProfile,
    signature: u32,
    tag: *mut IccTag,
) -> bool {
    // SAFETY: source short-circuit order reads only initialized reached metadata.
    if profile.is_null() || unsafe { (*profile).buffer.is_null() } || tag.is_null() {
        return false;
    }
    // SAFETY: caller supplies every reached declared row and payload. The local
    // table base is retained, while profile fields are reread as in the source.
    unsafe {
        let rows = (*profile).buffer.add(132);
        for i in 0..(*profile).tag_count {
            let row = rows.add(i as usize * 12);
            if read_big_u32(row) == signature {
                ptr::addr_of_mut!((*tag).signature).write(signature);
                ptr::addr_of_mut!((*tag).size).write(read_big_u32(row.add(8)));
                ptr::addr_of_mut!((*tag).buffer)
                    .write((*profile).buffer.add(read_big_u32(row.add(4)) as usize));
                ptr::addr_of_mut!((*tag).kind).write(read_big_u32((*tag).buffer));
                return true;
            }
        }
    }
    false
}

unsafe fn read_tag_xyz(tag: *const IccTag, x: *mut f32, y: *mut f32, z: *mut f32) -> bool {
    // SAFETY: internal callers provide initialized tag fields.
    if unsafe { (*tag).kind != SIGNATURE_XYZ || (*tag).size < 20 } {
        return false;
    }
    // SAFETY: source size guard establishes all three fixed words in borrowed
    // bytes. Outputs are writable; interleaved reads/writes preserve aliases.
    unsafe {
        let buffer = (*tag).buffer;
        x.write(read_big_fixed(buffer.add(8)));
        y.write(read_big_fixed(buffer.add(12)));
        z.write(read_big_fixed(buffer.add(16)));
    }
    true
}

/// Original chromatic-adaptation-tag extraction.
///
/// # Safety
/// Profile satisfies get_tag_by_signature. If a correctly typed/sized CHAD is
/// found, matrix must be writable and all its reached input words readable.
pub unsafe fn get_chad(profile: *const IccProfile, matrix: *mut Matrix3x3) -> bool {
    let mut tag = MaybeUninit::<IccTag>::uninit();
    // SAFETY: caller provides valid lookup storage; success initializes logical
    // tag fields, while its source padding may remain indeterminate.
    if !unsafe { get_tag_by_signature(profile, SIGNATURE_CHAD, tag.as_mut_ptr()) } {
        return false;
    }
    // SAFETY: successful lookup wrote all non-padding tag fields.
    let tag = unsafe { tag.assume_init() };
    if tag.kind != SIGNATURE_SF32 || tag.size < 44 {
        return false;
    }
    // SAFETY: checked tag size covers all nine fixed words; caller supplies the
    // reached matrix slots. No borrowed input is cloned or held as &mut data.
    unsafe {
        let mut values = tag.buffer.add(8);
        for r in 0..3 {
            for c in 0..3 {
                ptr::addr_of_mut!((*matrix).vals[r][c]).write(read_big_fixed(values));
                values = values.add(4);
            }
        }
    }
    true
}

/// Original white-point-tag extraction.
///
/// # Safety
/// Profile satisfies get_tag_by_signature. Whenever lookup succeeds, xyz
/// supports formation of all three element addresses before the later tag-kind
/// and size guard. Correctly typed/sized XYZ tags additionally require three
/// writable floats and live readable tag bytes through the interleaved writes.
pub unsafe fn get_wtpt(profile: *const IccProfile, xyz: *mut f32) -> bool {
    let mut tag = MaybeUninit::<IccTag>::uninit();
    // SAFETY: source lookup initializes tag fields only on success. Output-slot
    // arithmetic and reads are reached only after that source guard succeeds.
    unsafe {
        get_tag_by_signature(profile, SIGNATURE_WTPT, tag.as_mut_ptr())
            && read_tag_xyz(tag.as_ptr(), xyz, xyz.add(1), xyz.add(2))
    }
}

fn data_color_space_channel_count(signature: u32) -> i32 {
    match signature {
        SIGNATURE_CMYK => 4,
        SIGNATURE_GRAY => 1,
        SIGNATURE_RGB | SIGNATURE_LAB | SIGNATURE_XYZ | 0x4c757620 | 0x59436272 | 0x59787920
        | 0x48535620 | 0x484c5320 | 0x434d5920 | 0x33434c52 => 3,
        0x32434c52 => 2,
        0x34434c52 => 4,
        0x35434c52 => 5,
        0x36434c52 => 6,
        0x37434c52 => 7,
        0x38434c52 => 8,
        0x39434c52 => 9,
        0x41434c52 => 10,
        0x42434c52 => 11,
        0x43434c52 => 12,
        0x44434c52 => 13,
        0x45434c52 => 14,
        0x46434c52 => 15,
        _ => -1,
    }
}

/// Original input-channel consistency check, including an optional kTRC lookup.
///
/// # Safety
/// Profile and its reached fields are initialized and physically valid for the
/// source tag lookup. It must not be null; borrowed ICC storage remains live.
pub unsafe fn get_input_channel_count(profile: *const IccProfile) -> i32 {
    let mut a2b_count = 0;
    // SAFETY: caller supplies initialized source flags/channel metadata.
    unsafe {
        if (*profile).has_a2b {
            a2b_count = if (*profile).a2b.input_channels != 0 {
                (*profile).a2b.input_channels as i32
            } else {
                3
            };
        }
    }
    let mut tag = MaybeUninit::<IccTag>::uninit();
    // SAFETY: caller guarantees source lookup's reached input. No uninitialized
    // tag fields are read on failure, and this check observes only the boolean.
    let trc_count = if unsafe { get_tag_by_signature(profile, SIGNATURE_KTRC, tag.as_mut_ptr()) } {
        1
    } else if unsafe { (*profile).has_trc } {
        3
    } else {
        0
    };
    // SAFETY: source data-color-space field is initialized.
    let dcs_count = data_color_space_channel_count(unsafe { (*profile).data_color_space });
    if dcs_count < 0 {
        return -1;
    }
    if a2b_count > 0 && a2b_count != dcs_count {
        return -1;
    }
    if trc_count > 0 && trc_count != dcs_count {
        return -1;
    }
    dcs_count
}

unsafe fn read_to_xyz_d50(
    r: *const IccTag,
    g: *const IccTag,
    b: *const IccTag,
    matrix: *mut Matrix3x3,
) -> bool {
    // SAFETY: callers supply valid tags/matrix; each column retains the original
    // short-circuit order, so an error preserves all earlier reached writes.
    unsafe {
        read_tag_xyz(
            r,
            ptr::addr_of_mut!((*matrix).vals[0][0]),
            ptr::addr_of_mut!((*matrix).vals[1][0]),
            ptr::addr_of_mut!((*matrix).vals[2][0]),
        ) && read_tag_xyz(
            g,
            ptr::addr_of_mut!((*matrix).vals[0][1]),
            ptr::addr_of_mut!((*matrix).vals[1][1]),
            ptr::addr_of_mut!((*matrix).vals[2][1]),
        ) && read_tag_xyz(
            b,
            ptr::addr_of_mut!((*matrix).vals[0][2]),
            ptr::addr_of_mut!((*matrix).vals[1][2]),
            ptr::addr_of_mut!((*matrix).vals[2][2]),
        )
    }
}

unsafe fn read_curve_para(
    buffer: *const u8,
    size: u32,
    curve: *mut Curve,
    curve_size: *mut u32,
) -> bool {
    if (size as u64) < 12 {
        return false;
    }
    // SAFETY: the source fixed-size guard covers the two-byte function type.
    let function_type = unsafe { read_big_u16(buffer.add(8)) };
    if function_type > 4 {
        return false;
    }
    const CURVE_BYTES: [u32; 5] = [4, 12, 16, 20, 28];
    if (size as u64) < 12 + CURVE_BYTES[function_type as usize] as u64 {
        return false;
    }
    // SAFETY: source guards establish every parameter read and curve storage is
    // writable. Individual fields retain original overlap/partial-failure order.
    unsafe {
        if !curve_size.is_null() {
            curve_size.write(12 + CURVE_BYTES[function_type as usize]);
        }
        curve.cast::<u32>().write(0);
        ptr::addr_of_mut!((*curve).parametric.function.a).write(1.0);
        ptr::addr_of_mut!((*curve).parametric.function.b).write(0.0);
        ptr::addr_of_mut!((*curve).parametric.function.c).write(0.0);
        ptr::addr_of_mut!((*curve).parametric.function.d).write(0.0);
        ptr::addr_of_mut!((*curve).parametric.function.e).write(0.0);
        ptr::addr_of_mut!((*curve).parametric.function.f).write(0.0);
        ptr::addr_of_mut!((*curve).parametric.function.g).write(read_big_fixed(buffer.add(12)));
        match function_type {
            1 => {
                ptr::addr_of_mut!((*curve).parametric.function.a)
                    .write(read_big_fixed(buffer.add(16)));
                ptr::addr_of_mut!((*curve).parametric.function.b)
                    .write(read_big_fixed(buffer.add(20)));
                if (*curve).parametric.function.a == 0.0 {
                    return false;
                }
                ptr::addr_of_mut!((*curve).parametric.function.d)
                    .write(-(*curve).parametric.function.b / (*curve).parametric.function.a);
            }
            2 => {
                ptr::addr_of_mut!((*curve).parametric.function.a)
                    .write(read_big_fixed(buffer.add(16)));
                ptr::addr_of_mut!((*curve).parametric.function.b)
                    .write(read_big_fixed(buffer.add(20)));
                ptr::addr_of_mut!((*curve).parametric.function.e)
                    .write(read_big_fixed(buffer.add(24)));
                if (*curve).parametric.function.a == 0.0 {
                    return false;
                }
                ptr::addr_of_mut!((*curve).parametric.function.d)
                    .write(-(*curve).parametric.function.b / (*curve).parametric.function.a);
                ptr::addr_of_mut!((*curve).parametric.function.f)
                    .write((*curve).parametric.function.e);
            }
            3 => {
                ptr::addr_of_mut!((*curve).parametric.function.a)
                    .write(read_big_fixed(buffer.add(16)));
                ptr::addr_of_mut!((*curve).parametric.function.b)
                    .write(read_big_fixed(buffer.add(20)));
                ptr::addr_of_mut!((*curve).parametric.function.c)
                    .write(read_big_fixed(buffer.add(24)));
                ptr::addr_of_mut!((*curve).parametric.function.d)
                    .write(read_big_fixed(buffer.add(28)));
            }
            4 => {
                ptr::addr_of_mut!((*curve).parametric.function.a)
                    .write(read_big_fixed(buffer.add(16)));
                ptr::addr_of_mut!((*curve).parametric.function.b)
                    .write(read_big_fixed(buffer.add(20)));
                ptr::addr_of_mut!((*curve).parametric.function.c)
                    .write(read_big_fixed(buffer.add(24)));
                ptr::addr_of_mut!((*curve).parametric.function.d)
                    .write(read_big_fixed(buffer.add(28)));
                ptr::addr_of_mut!((*curve).parametric.function.e)
                    .write(read_big_fixed(buffer.add(32)));
                ptr::addr_of_mut!((*curve).parametric.function.f)
                    .write(read_big_fixed(buffer.add(36)));
            }
            _ => {}
        }
        (&*ptr::addr_of!((*curve).parametric.function)).is_srgbish()
    }
}

unsafe fn read_curve_curv(
    buffer: *const u8,
    size: u32,
    curve: *mut Curve,
    curve_size: *mut u32,
) -> bool {
    if (size as u64) < 12 {
        return false;
    }
    // SAFETY: source fixed-size guard covers the count.
    let value_count = unsafe { read_big_u32(buffer.add(8)) };
    if (size as u64) < 12 + value_count as u64 * 2 {
        return false;
    }
    // SAFETY: checked source extent covers every reached word. Table bytes stay
    // borrowed, and only the original active fields are overwritten.
    unsafe {
        if !curve_size.is_null() {
            curve_size.write((12 + value_count as u64 * 2) as u32);
        }
        if value_count < 2 {
            curve.cast::<u32>().write(0);
            ptr::addr_of_mut!((*curve).parametric.function.a).write(1.0);
            ptr::addr_of_mut!((*curve).parametric.function.b).write(0.0);
            ptr::addr_of_mut!((*curve).parametric.function.c).write(0.0);
            ptr::addr_of_mut!((*curve).parametric.function.d).write(0.0);
            ptr::addr_of_mut!((*curve).parametric.function.e).write(0.0);
            ptr::addr_of_mut!((*curve).parametric.function.f).write(0.0);
            ptr::addr_of_mut!((*curve).parametric.function.g).write(if value_count == 0 {
                1.0
            } else {
                read_big_u16(buffer.add(12)) as f32 * (1.0 / 256.0)
            });
        } else {
            ptr::addr_of_mut!((*curve).table.table_8).write(ptr::null());
            ptr::addr_of_mut!((*curve).table.table_16).write(buffer.add(12));
            curve.cast::<u32>().write(value_count);
        }
    }
    true
}

/// Original curv/para reader, including its unchanged-output failure prefixes.
///
/// # Safety
/// Non-null buffer must cover all reads permitted by size and the source's
/// guards. Reached curve/curve_size outputs must be writable and any retained
/// input pointers remain borrowed for all later table use. Null inputs follow
/// the original guard; this does not create an owner or clone the table bytes.
pub unsafe fn read_curve(
    buffer: *const u8,
    size: u32,
    curve: *mut Curve,
    curve_size: *mut u32,
) -> bool {
    if buffer.is_null() || size < 4 || curve.is_null() {
        return false;
    }
    // SAFETY: caller supplies the reached input extent; selected readers retain
    // their original size guards before reading/writing any further fields.
    unsafe {
        let kind = read_big_u32(buffer);
        if kind == SIGNATURE_PARA {
            return read_curve_para(buffer, size, curve, curve_size);
        }
        if kind == SIGNATURE_CURV {
            return read_curve_curv(buffer, size, curve, curve_size);
        }
    }
    false
}

unsafe fn read_mft_common_a2b(buffer: *const u8, out: *mut A2B) -> bool {
    // SAFETY: caller establishes the complete 48-byte common layout and writable
    // output fields. Channel guards precede all indexed writes.
    unsafe {
        ptr::addr_of_mut!((*out).matrix_channels).write(0);
        ptr::addr_of_mut!((*out).input_channels).write(*buffer.add(8) as u32);
        ptr::addr_of_mut!((*out).output_channels).write(*buffer.add(9) as u32);
        if (*out).output_channels != 3 {
            return false;
        }
        if (*out).input_channels < 1 || (*out).input_channels > 4 {
            return false;
        }
        for i in 0..(*out).input_channels {
            ptr::addr_of_mut!((*out).grid_points)
                .cast::<u8>()
                .add(i as usize)
                .write(*buffer.add(10));
        }
        if (*out).grid_points[0] < 2 {
            return false;
        }
    }
    true
}

unsafe fn read_mft_common_b2a(buffer: *const u8, out: *mut B2A) -> bool {
    // SAFETY: caller establishes the same complete common layout. Source B2A
    // channel bounds protect its three input and four output slots.
    unsafe {
        ptr::addr_of_mut!((*out).matrix_channels).write(0);
        ptr::addr_of_mut!((*out).input_channels).write(*buffer.add(8) as u32);
        ptr::addr_of_mut!((*out).output_channels).write(*buffer.add(9) as u32);
        if (*out).input_channels != 3 {
            return false;
        }
        if (*out).output_channels < 3 || (*out).output_channels > 4 {
            return false;
        }
        for i in 0..(*out).input_channels {
            ptr::addr_of_mut!((*out).grid_points)
                .cast::<u8>()
                .add(i as usize)
                .write(*buffer.add(10));
        }
        if (*out).grid_points[0] < 2 {
            return false;
        }
    }
    true
}

// Original A2B_or_B2A templates. Macro expansion retains two monomorphized
// field layouts without adding an owning view or borrowing table payloads twice.
macro_rules! mft_readers {
    ($ty:ty, $common:ident, $init:ident, $mft1:ident, $mft2:ident) => {
        unsafe fn $init(
            table_base: *const u8,
            max_tables_len: u64,
            byte_width: u32,
            input_table_entries: u32,
            output_table_entries: u32,
            out: *mut $ty,
        ) -> bool {
            let byte_len_per_input_table = input_table_entries * byte_width;
            let byte_len_per_output_table = output_table_entries * byte_width;
            // SAFETY: common reader has checked all channel bounds; caller
            // supplies initialized metadata and writable curve/grid fields.
            unsafe {
                let byte_len_all_input_tables = (*out).input_channels * byte_len_per_input_table;
                let byte_len_all_output_tables = (*out).output_channels * byte_len_per_output_table;
                let mut grid_size = ((*out).output_channels * byte_width) as u64;
                for axis in 0..(*out).input_channels {
                    grid_size *= (*out).grid_points[axis as usize] as u64;
                }
                if max_tables_len
                    < byte_len_all_input_tables as u64
                        + grid_size
                        + byte_len_all_output_tables as u64
                {
                    return false;
                }
                for i in 0..(*out).input_channels {
                    let curve = ptr::addr_of_mut!((*out).input_curves)
                        .cast::<Curve>()
                        .add(i as usize);
                    curve.cast::<u32>().write(input_table_entries);
                    if byte_width == 1 {
                        ptr::addr_of_mut!((*curve).table.table_8)
                            .write(table_base.add((i * byte_len_per_input_table) as usize));
                        ptr::addr_of_mut!((*curve).table.table_16).write(ptr::null());
                    } else {
                        ptr::addr_of_mut!((*curve).table.table_8).write(ptr::null());
                        ptr::addr_of_mut!((*curve).table.table_16)
                            .write(table_base.add((i * byte_len_per_input_table) as usize));
                    }
                }
                if byte_width == 1 {
                    ptr::addr_of_mut!((*out).grid_8)
                        .write(table_base.add(byte_len_all_input_tables as usize));
                    ptr::addr_of_mut!((*out).grid_16).write(ptr::null());
                } else {
                    ptr::addr_of_mut!((*out).grid_8).write(ptr::null());
                    ptr::addr_of_mut!((*out).grid_16)
                        .write(table_base.add(byte_len_all_input_tables as usize));
                }
                let output_table_base = table_base
                    .add(byte_len_all_input_tables as usize)
                    .add(grid_size as usize);
                for i in 0..(*out).output_channels {
                    let curve = ptr::addr_of_mut!((*out).output_curves)
                        .cast::<Curve>()
                        .add(i as usize);
                    curve.cast::<u32>().write(output_table_entries);
                    if byte_width == 1 {
                        ptr::addr_of_mut!((*curve).table.table_8)
                            .write(output_table_base.add((i * byte_len_per_output_table) as usize));
                        ptr::addr_of_mut!((*curve).table.table_16).write(ptr::null());
                    } else {
                        ptr::addr_of_mut!((*curve).table.table_8).write(ptr::null());
                        ptr::addr_of_mut!((*curve).table.table_16)
                            .write(output_table_base.add((i * byte_len_per_output_table) as usize));
                    }
                }
            }
            true
        }
        unsafe fn $mft1(tag: *const IccTag, out: *mut $ty) -> bool {
            // SAFETY: tag's initialized size covers all common bytes on this
            // branch. The subsequent size guard in init protects table offsets.
            unsafe {
                if ((*tag).size as u64) < 48 {
                    return false;
                }
                let buffer = (*tag).buffer;
                if !$common(buffer, out) {
                    return false;
                }
                let input_table_entries = 256;
                let output_table_entries = 256;
                $init(
                    buffer.add(48),
                    (*tag).size as u64 - 48,
                    1,
                    input_table_entries,
                    output_table_entries,
                    out,
                )
            }
        }
        #[expect(
            clippy::manual_range_contains,
            reason = "Preserve source table-entry guard order."
        )]
        unsafe fn $mft2(tag: *const IccTag, out: *mut $ty) -> bool {
            // SAFETY: fixed-size guard covers count words; common metadata and
            // explicit 2..4096 guards establish the template's integer bounds.
            unsafe {
                if ((*tag).size as u64) < 52 {
                    return false;
                }
                let buffer = (*tag).buffer;
                if !$common(buffer, out) {
                    return false;
                }
                let input_table_entries = read_big_u16(buffer.add(48)) as u32;
                let output_table_entries = read_big_u16(buffer.add(50)) as u32;
                if input_table_entries < 2
                    || input_table_entries > 4096
                    || output_table_entries < 2
                    || output_table_entries > 4096
                {
                    return false;
                }
                $init(
                    buffer.add(52),
                    (*tag).size as u64 - 52,
                    2,
                    input_table_entries,
                    output_table_entries,
                    out,
                )
            }
        }
    };
}
mft_readers!(
    A2B,
    read_mft_common_a2b,
    init_tables_a2b,
    read_tag_mft1_a2b,
    read_tag_mft2_a2b
);
mft_readers!(
    B2A,
    read_mft_common_b2a,
    init_tables_b2a,
    read_tag_mft1_b2a,
    read_tag_mft2_b2a
);

unsafe fn read_curves(
    buffer: *const u8,
    size: u32,
    mut curve_offset: u32,
    num_curves: u32,
    curves: *mut Curve,
) -> bool {
    for i in 0..num_curves {
        if curve_offset > size {
            return false;
        }
        let mut curve_bytes = MaybeUninit::<u32>::uninit();
        // SAFETY: the original offset guard allows one-past formation; read_curve
        // checks remaining size before each byte read and initializes curve_bytes
        // only on success. Caller provides num_curves writable curve slots.
        if !unsafe {
            read_curve(
                buffer.add(curve_offset as usize),
                size - curve_offset,
                curves.add(i as usize),
                curve_bytes.as_mut_ptr(),
            )
        } {
            return false;
        }
        // SAFETY: every successful source reader writes curve_size.
        let mut curve_bytes = unsafe { curve_bytes.assume_init() };
        if curve_bytes > u32::MAX - 3 {
            return false;
        }
        curve_bytes = (curve_bytes + 3) & !3;
        let new_offset_64 = curve_offset as u64 + curve_bytes as u64;
        curve_offset = new_offset_64 as u32;
        if new_offset_64 != curve_offset as u64 {
            return false;
        }
    }
    true
}

unsafe fn read_tag_mab(tag: *const IccTag, out: *mut A2B, pcs_is_xyz: bool) -> bool {
    // SAFETY: caller supplies an initialized tag with physically valid declared
    // bytes and writable output storage. Each source extent/channel guard stays
    // before its reached reads/writes; input/grid/table pointers remain borrowed.
    unsafe {
        if ((*tag).size as u64) < 32 {
            return false;
        }
        let buffer = (*tag).buffer;
        ptr::addr_of_mut!((*out).input_channels).write(*buffer.add(8) as u32);
        ptr::addr_of_mut!((*out).output_channels).write(*buffer.add(9) as u32);
        if (*out).output_channels != 3 {
            return false;
        }
        if (*out).input_channels > 4 {
            return false;
        }
        let b_curve_offset = read_big_u32(buffer.add(12));
        let matrix_offset = read_big_u32(buffer.add(16));
        let m_curve_offset = read_big_u32(buffer.add(20));
        let clut_offset = read_big_u32(buffer.add(24));
        let a_curve_offset = read_big_u32(buffer.add(28));
        if b_curve_offset == 0 {
            return false;
        }
        if !read_curves(
            buffer,
            (*tag).size,
            b_curve_offset,
            (*out).output_channels,
            ptr::addr_of_mut!((*out).output_curves).cast::<Curve>(),
        ) {
            return false;
        }
        if m_curve_offset != 0 {
            if matrix_offset == 0 {
                return false;
            }
            ptr::addr_of_mut!((*out).matrix_channels).write((*out).output_channels);
            if !read_curves(
                buffer,
                (*tag).size,
                m_curve_offset,
                (*out).matrix_channels,
                ptr::addr_of_mut!((*out).matrix_curves).cast::<Curve>(),
            ) {
                return false;
            }
            if ((*tag).size as u64) < matrix_offset as u64 + 12 * 4 {
                return false;
            }
            let encoding_factor = if pcs_is_xyz { 65535.0 / 32768.0 } else { 1.0 };
            let mtx_buf = buffer.add(matrix_offset as usize);
            ptr::addr_of_mut!((*out).matrix.vals[0][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(0)));
            ptr::addr_of_mut!((*out).matrix.vals[0][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(4)));
            ptr::addr_of_mut!((*out).matrix.vals[0][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(8)));
            ptr::addr_of_mut!((*out).matrix.vals[1][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(12)));
            ptr::addr_of_mut!((*out).matrix.vals[1][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(16)));
            ptr::addr_of_mut!((*out).matrix.vals[1][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(20)));
            ptr::addr_of_mut!((*out).matrix.vals[2][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(24)));
            ptr::addr_of_mut!((*out).matrix.vals[2][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(28)));
            ptr::addr_of_mut!((*out).matrix.vals[2][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(32)));
            ptr::addr_of_mut!((*out).matrix.vals[0][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(36)));
            ptr::addr_of_mut!((*out).matrix.vals[1][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(40)));
            ptr::addr_of_mut!((*out).matrix.vals[2][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(44)));
        } else {
            if matrix_offset != 0 {
                return false;
            }
            ptr::addr_of_mut!((*out).matrix_channels).write(0);
        }
        if a_curve_offset != 0 {
            if clut_offset == 0 {
                return false;
            }
            if !read_curves(
                buffer,
                (*tag).size,
                a_curve_offset,
                (*out).input_channels,
                ptr::addr_of_mut!((*out).input_curves).cast::<Curve>(),
            ) {
                return false;
            }
            if ((*tag).size as u64) < clut_offset as u64 + 20 {
                return false;
            }
            let clut = buffer.add(clut_offset as usize);
            if *clut.add(16) == 1 {
                ptr::addr_of_mut!((*out).grid_8).write(clut.add(20));
                ptr::addr_of_mut!((*out).grid_16).write(ptr::null());
            } else if *clut.add(16) == 2 {
                ptr::addr_of_mut!((*out).grid_8).write(ptr::null());
                ptr::addr_of_mut!((*out).grid_16).write(clut.add(20));
            } else {
                return false;
            }
            let mut grid_size = ((*out).output_channels * *clut.add(16) as u32) as u64;
            for i in 0..(*out).input_channels {
                ptr::addr_of_mut!((*out).grid_points)
                    .cast::<u8>()
                    .add(i as usize)
                    .write(*clut.add(i as usize));
                if (*out).grid_points[i as usize] < 2 {
                    return false;
                }
                grid_size *= (*out).grid_points[i as usize] as u64;
            }
            if ((*tag).size as u64) < clut_offset as u64 + 20 + grid_size {
                return false;
            }
        } else {
            if clut_offset != 0 {
                return false;
            }
            if (*out).input_channels != (*out).output_channels {
                return false;
            }
            ptr::addr_of_mut!((*out).input_channels).write(0);
        }
    }
    true
}

unsafe fn read_tag_mba(tag: *const IccTag, out: *mut B2A, pcs_is_xyz: bool) -> bool {
    // SAFETY: caller supplies an initialized tag with physically valid declared
    // bytes and writable output storage. Each source extent/channel guard stays
    // before its reached reads/writes; input/grid/table pointers remain borrowed.
    unsafe {
        if ((*tag).size as u64) < 32 {
            return false;
        }
        let buffer = (*tag).buffer;
        ptr::addr_of_mut!((*out).input_channels).write(*buffer.add(8) as u32);
        ptr::addr_of_mut!((*out).output_channels).write(*buffer.add(9) as u32);
        if (*out).input_channels != 3 {
            return false;
        }
        if (*out).output_channels < 3 || (*out).output_channels > 4 {
            return false;
        }
        let b_curve_offset = read_big_u32(buffer.add(12));
        let matrix_offset = read_big_u32(buffer.add(16));
        let m_curve_offset = read_big_u32(buffer.add(20));
        let clut_offset = read_big_u32(buffer.add(24));
        let a_curve_offset = read_big_u32(buffer.add(28));
        if b_curve_offset == 0 {
            return false;
        }
        if !read_curves(
            buffer,
            (*tag).size,
            b_curve_offset,
            (*out).input_channels,
            ptr::addr_of_mut!((*out).input_curves).cast::<Curve>(),
        ) {
            return false;
        }
        if m_curve_offset != 0 {
            if matrix_offset == 0 {
                return false;
            }
            ptr::addr_of_mut!((*out).matrix_channels).write((*out).input_channels);
            if !read_curves(
                buffer,
                (*tag).size,
                m_curve_offset,
                (*out).matrix_channels,
                ptr::addr_of_mut!((*out).matrix_curves).cast::<Curve>(),
            ) {
                return false;
            }
            if ((*tag).size as u64) < matrix_offset as u64 + 12 * 4 {
                return false;
            }
            let encoding_factor = if pcs_is_xyz { 32768.0 / 65535.0 } else { 1.0 };
            let mtx_buf = buffer.add(matrix_offset as usize);
            ptr::addr_of_mut!((*out).matrix.vals[0][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(0)));
            ptr::addr_of_mut!((*out).matrix.vals[0][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(4)));
            ptr::addr_of_mut!((*out).matrix.vals[0][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(8)));
            ptr::addr_of_mut!((*out).matrix.vals[1][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(12)));
            ptr::addr_of_mut!((*out).matrix.vals[1][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(16)));
            ptr::addr_of_mut!((*out).matrix.vals[1][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(20)));
            ptr::addr_of_mut!((*out).matrix.vals[2][0])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(24)));
            ptr::addr_of_mut!((*out).matrix.vals[2][1])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(28)));
            ptr::addr_of_mut!((*out).matrix.vals[2][2])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(32)));
            ptr::addr_of_mut!((*out).matrix.vals[0][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(36)));
            ptr::addr_of_mut!((*out).matrix.vals[1][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(40)));
            ptr::addr_of_mut!((*out).matrix.vals[2][3])
                .write(encoding_factor * read_big_fixed(mtx_buf.add(44)));
        } else {
            if matrix_offset != 0 {
                return false;
            }
            ptr::addr_of_mut!((*out).matrix_channels).write(0);
        }
        if a_curve_offset != 0 {
            if clut_offset == 0 {
                return false;
            }
            if !read_curves(
                buffer,
                (*tag).size,
                a_curve_offset,
                (*out).output_channels,
                ptr::addr_of_mut!((*out).output_curves).cast::<Curve>(),
            ) {
                return false;
            }
            if ((*tag).size as u64) < clut_offset as u64 + 20 {
                return false;
            }
            let clut = buffer.add(clut_offset as usize);
            if *clut.add(16) == 1 {
                ptr::addr_of_mut!((*out).grid_8).write(clut.add(20));
                ptr::addr_of_mut!((*out).grid_16).write(ptr::null());
            } else if *clut.add(16) == 2 {
                ptr::addr_of_mut!((*out).grid_8).write(ptr::null());
                ptr::addr_of_mut!((*out).grid_16).write(clut.add(20));
            } else {
                return false;
            }
            let mut grid_size = ((*out).output_channels * *clut.add(16) as u32) as u64;
            for i in 0..(*out).input_channels {
                ptr::addr_of_mut!((*out).grid_points)
                    .cast::<u8>()
                    .add(i as usize)
                    .write(*clut.add(i as usize));
                if (*out).grid_points[i as usize] < 2 {
                    return false;
                }
                grid_size *= (*out).grid_points[i as usize] as u64;
            }
            if ((*tag).size as u64) < clut_offset as u64 + 20 + grid_size {
                return false;
            }
        } else {
            if clut_offset != 0 {
                return false;
            }
            if (*out).input_channels != (*out).output_channels {
                return false;
            }
            ptr::addr_of_mut!((*out).output_channels).write(0);
        }
    }
    true
}

unsafe fn read_a2b(tag: *const IccTag, out: *mut A2B, pcs_is_xyz: bool) -> bool {
    let mut ok = false;
    // SAFETY: tag extent and writable output satisfy the selected reader. Its
    // success establishes initialized active curves before canonicalization.
    unsafe {
        if (*tag).kind == SIGNATURE_MFT1 {
            ok = read_tag_mft1_a2b(tag, out);
        }
        if (*tag).kind == SIGNATURE_MFT2 {
            ok = read_tag_mft2_a2b(tag, out);
        }
        if (*tag).kind == SIGNATURE_MAB {
            ok = read_tag_mab(tag, out, pcs_is_xyz);
        }
        if !ok {
            return false;
        }
        if (*out).input_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[0]));
        }
        if (*out).input_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[1]));
        }
        if (*out).input_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[2]));
        }
        if (*out).input_channels > 3 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[3]));
        }
        if (*out).matrix_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[0]));
        }
        if (*out).matrix_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[1]));
        }
        if (*out).matrix_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[2]));
        }
        if (*out).output_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[0]));
        }
        if (*out).output_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[1]));
        }
        if (*out).output_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[2]));
        }
    }
    true
}
unsafe fn read_b2a(tag: *const IccTag, out: *mut B2A, pcs_is_xyz: bool) -> bool {
    let mut ok = false;
    // SAFETY: source readers guard all channel/extent bounds and initialize the
    // active branch of each curve before its reached identity test.
    unsafe {
        if (*tag).kind == SIGNATURE_MFT1 {
            ok = read_tag_mft1_b2a(tag, out);
        }
        if (*tag).kind == SIGNATURE_MFT2 {
            ok = read_tag_mft2_b2a(tag, out);
        }
        if (*tag).kind == SIGNATURE_MBA {
            ok = read_tag_mba(tag, out, pcs_is_xyz);
        }
        if !ok {
            return false;
        }
        if (*out).input_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[0]));
        }
        if (*out).input_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[1]));
        }
        if (*out).input_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).input_curves[2]));
        }
        if (*out).matrix_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[0]));
        }
        if (*out).matrix_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[1]));
        }
        if (*out).matrix_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).matrix_curves[2]));
        }
        if (*out).output_channels > 0 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[0]));
        }
        if (*out).output_channels > 1 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[1]));
        }
        if (*out).output_channels > 2 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[2]));
        }
        if (*out).output_channels > 3 {
            canonicalize_identity(ptr::addr_of_mut!((*out).output_curves[3]));
        }
    }
    true
}
unsafe fn read_cicp(tag: *const IccTag, out: *mut Cicp) -> bool {
    // SAFETY: initialized tag fields precede source type/extent checks; all four
    // byte accesses and individual output fields are then valid.
    unsafe {
        if (*tag).kind != SIGNATURE_CICP || ((*tag).size as u64) < 12 {
            return false;
        }
        ptr::addr_of_mut!((*out).color_primaries).write(*(*tag).buffer.add(8));
        ptr::addr_of_mut!((*out).transfer_characteristics).write(*(*tag).buffer.add(9));
        ptr::addr_of_mut!((*out).matrix_coefficients).write(*(*tag).buffer.add(10));
        ptr::addr_of_mut!((*out).video_full_range_flag).write(*(*tag).buffer.add(11));
    }
    true
}

/// Complete original skcms_ParseWithA2BPriority, preserving zeroing and every
/// source failure checkpoint. Payload/table pointers borrow the caller's bytes.
///
/// # Safety
/// A non-null profile points to writable aligned storage for a complete object.
/// For len >= 132, buffer covers len live readable bytes (including every source
/// declared tag extent). A reached priority index points to an initialized i32.
/// Storage must remain live while any returned profile table/grid pointer is
/// used. Source-defined aliases are allowed; no Rust references are retained
/// across raw writes. Callers must not concurrently access mutated storage.
#[expect(
    clippy::manual_range_contains,
    reason = "Preserve source priority validation predicates."
)]
pub unsafe fn parse_with_a2b_priority(
    buffer: *const u8,
    len: usize,
    priority: *const i32,
    priorities: i32,
    profile: *mut IccProfile,
) -> bool {
    if profile.is_null() {
        return false;
    }
    // SAFETY: caller provides full writable profile storage. This is the exact
    // source memset, including bytes of inactive union branches and padding.
    unsafe {
        ptr::write_bytes(
            profile.cast::<MaybeUninit<u8>>(),
            0,
            size_of::<IccProfile>(),
        );
    }
    if len < 132 {
        return false;
    }
    // SAFETY: caller supplies the header extent. Subsequent source checks guard
    // all reached tag readers before their accesses. Raw field writes preserve
    // partial profiles without manufacturing references to inactive branches.
    unsafe {
        ptr::addr_of_mut!((*profile).buffer).write(buffer);
        ptr::addr_of_mut!((*profile).size).write(read_big_u32(buffer));
        let version = read_big_u32(buffer.add(8));
        ptr::addr_of_mut!((*profile).data_color_space).write(read_big_u32(buffer.add(16)));
        ptr::addr_of_mut!((*profile).pcs).write(read_big_u32(buffer.add(20)));
        let signature = read_big_u32(buffer.add(36));
        let illuminant_x = read_big_fixed(buffer.add(68));
        let illuminant_y = read_big_fixed(buffer.add(72));
        let illuminant_z = read_big_fixed(buffer.add(76));
        ptr::addr_of_mut!((*profile).tag_count).write(read_big_u32(buffer.add(128)));
        let tag_table_size = (*profile).tag_count as u64 * 12;
        if signature != SIGNATURE_ACSP
            || (*profile).size as usize > len
            || ((*profile).size as u64) < 132 + tag_table_size
            || (version >> 24) > 4
        {
            return false;
        }
        if (illuminant_x - 0.9642).abs() > 0.0100
            || (illuminant_y - 1.0000).abs() > 0.0100
            || (illuminant_z - 0.8249).abs() > 0.0100
        {
            return false;
        }
        let tags = buffer.add(132);
        for i in 0..(*profile).tag_count {
            let entry = tags.add(i as usize * 12);
            let tag_offset = read_big_u32(entry.add(4));
            let tag_size = read_big_u32(entry.add(8));
            let tag_end = tag_offset as u64 + tag_size as u64;
            if tag_size < 4 || tag_end > (*profile).size as u64 {
                return false;
            }
        }
        if (*profile).pcs != SIGNATURE_XYZ && (*profile).pcs != SIGNATURE_LAB {
            return false;
        }
        let pcs_is_xyz = (*profile).pcs == SIGNATURE_XYZ;
        let mut k_trc = MaybeUninit::<IccTag>::uninit();
        if (*profile).data_color_space == SIGNATURE_GRAY
            && get_tag_by_signature(profile, SIGNATURE_KTRC, k_trc.as_mut_ptr())
        {
            let tag = k_trc.as_ptr();
            if !read_curve(
                (*tag).buffer,
                (*tag).size,
                ptr::addr_of_mut!((*profile).trc[0]),
                ptr::null_mut(),
            ) {
                return false;
            }
            copy_curve(
                ptr::addr_of!((*profile).trc[0]),
                ptr::addr_of_mut!((*profile).trc[1]),
            );
            copy_curve(
                ptr::addr_of!((*profile).trc[0]),
                ptr::addr_of_mut!((*profile).trc[2]),
            );
            ptr::addr_of_mut!((*profile).has_trc).write(true);
            if pcs_is_xyz {
                ptr::addr_of_mut!((*profile).to_xyz_d50.vals[0][0]).write(illuminant_x);
                ptr::addr_of_mut!((*profile).to_xyz_d50.vals[1][1]).write(illuminant_y);
                ptr::addr_of_mut!((*profile).to_xyz_d50.vals[2][2]).write(illuminant_z);
                ptr::addr_of_mut!((*profile).has_to_xyz_d50).write(true);
            }
        } else {
            let mut r_trc = MaybeUninit::<IccTag>::uninit();
            let mut g_trc = MaybeUninit::<IccTag>::uninit();
            let mut b_trc = MaybeUninit::<IccTag>::uninit();
            if get_tag_by_signature(profile, SIGNATURE_RTRC, r_trc.as_mut_ptr())
                && get_tag_by_signature(profile, SIGNATURE_GTRC, g_trc.as_mut_ptr())
                && get_tag_by_signature(profile, SIGNATURE_BTRC, b_trc.as_mut_ptr())
            {
                let r = r_trc.as_ptr();
                let g = g_trc.as_ptr();
                let b = b_trc.as_ptr();
                if !read_curve(
                    (*r).buffer,
                    (*r).size,
                    ptr::addr_of_mut!((*profile).trc[0]),
                    ptr::null_mut(),
                ) || !read_curve(
                    (*g).buffer,
                    (*g).size,
                    ptr::addr_of_mut!((*profile).trc[1]),
                    ptr::null_mut(),
                ) || !read_curve(
                    (*b).buffer,
                    (*b).size,
                    ptr::addr_of_mut!((*profile).trc[2]),
                    ptr::null_mut(),
                ) {
                    return false;
                }
                ptr::addr_of_mut!((*profile).has_trc).write(true);
            }
            let mut r_xyz = MaybeUninit::<IccTag>::uninit();
            let mut g_xyz = MaybeUninit::<IccTag>::uninit();
            let mut b_xyz = MaybeUninit::<IccTag>::uninit();
            if get_tag_by_signature(profile, SIGNATURE_RXYZ, r_xyz.as_mut_ptr())
                && get_tag_by_signature(profile, SIGNATURE_GXYZ, g_xyz.as_mut_ptr())
                && get_tag_by_signature(profile, SIGNATURE_BXYZ, b_xyz.as_mut_ptr())
            {
                if !read_to_xyz_d50(
                    r_xyz.as_ptr(),
                    g_xyz.as_ptr(),
                    b_xyz.as_ptr(),
                    ptr::addr_of_mut!((*profile).to_xyz_d50),
                ) {
                    return false;
                }
                ptr::addr_of_mut!((*profile).has_to_xyz_d50).write(true);
            }
        }
        for i in 0..priorities {
            let value = *priority.add(i as usize);
            if value < 0 || value > 2 {
                return false;
            }
            let sig = SIGNATURE_A2B0 + value as u32;
            let mut tag = MaybeUninit::<IccTag>::uninit();
            if get_tag_by_signature(profile, sig, tag.as_mut_ptr()) {
                if !read_a2b(tag.as_ptr(), ptr::addr_of_mut!((*profile).a2b), pcs_is_xyz) {
                    return false;
                }
                ptr::addr_of_mut!((*profile).has_a2b).write(true);
                break;
            }
        }
        for i in 0..priorities {
            let value = *priority.add(i as usize);
            if value < 0 || value > 2 {
                return false;
            }
            let sig = SIGNATURE_B2A0 + value as u32;
            let mut tag = MaybeUninit::<IccTag>::uninit();
            if get_tag_by_signature(profile, sig, tag.as_mut_ptr()) {
                if !read_b2a(tag.as_ptr(), ptr::addr_of_mut!((*profile).b2a), pcs_is_xyz) {
                    return false;
                }
                ptr::addr_of_mut!((*profile).has_b2a).write(true);
                break;
            }
        }
        let mut cicp = MaybeUninit::<IccTag>::uninit();
        if get_tag_by_signature(profile, SIGNATURE_CICP, cicp.as_mut_ptr()) {
            if !read_cicp(cicp.as_ptr(), ptr::addr_of_mut!((*profile).cicp)) {
                return false;
            }
            ptr::addr_of_mut!((*profile).has_cicp).write(true);
        }
        (*profile).has_a2b || ((*profile).has_trc && (*profile).has_to_xyz_d50)
    }
}

/// Original skcms_Parse default A2B/B2A priority order.
///
/// # Safety
/// The arguments satisfy parse_with_a2b_priority's buffer/profile contract.
pub unsafe fn parse(buffer: *const u8, len: usize, profile: *mut IccProfile) -> bool {
    const PRIORITY: [i32; 2] = [0, 1];
    // SAFETY: the static priority array covers every reached index.
    unsafe { parse_with_a2b_priority(buffer, len, PRIORITY.as_ptr(), 2, profile) }
}

// Source static profiles never borrow external storage: every curve is the
// initialized parametric branch and every grid/buffer pointer is null. Keep the
// Sync proof on these immutable constants rather than on arbitrary profiles.
struct ImmutableProfile(IccProfile);
// SAFETY: the only instances below contain no non-null pointers, are completely
// initialized, have no interior mutability, and are never exposed mutably.
unsafe impl Sync for ImmutableProfile {}

const SRGB_FUNCTION: TransferFunction = TransferFunction {
    g: 2.4,
    a: (1.0_f64 / 1.055_f64) as f32,
    b: (0.055_f64 / 1.055_f64) as f32,
    c: (1.0_f64 / 12.92_f64) as f32,
    d: 0.04045,
    e: 0.0,
    f: 0.0,
};
const IDENTITY_FUNCTION: TransferFunction = TransferFunction {
    g: 1.0,
    a: 1.0,
    b: 0.0,
    c: 0.0,
    d: 0.0,
    e: 0.0,
    f: 0.0,
};
#[expect(
    clippy::excessive_precision,
    reason = "Preserve pristine sRGB matrix decimal literals."
)]
const fn srgb_profile_value() -> IccProfile {
    let mut profile = IccProfile::zeroed();
    profile.data_color_space = SIGNATURE_RGB;
    profile.pcs = SIGNATURE_XYZ;
    profile.trc = [Curve::from_parametric(SRGB_FUNCTION); 3];
    profile.to_xyz_d50 = Matrix3x3 {
        vals: [
            [0.436065674, 0.385147095, 0.143066406],
            [0.222488403, 0.716873169, 0.060607910],
            [0.013916016, 0.097076416, 0.714096069],
        ],
    };
    profile.has_trc = true;
    profile.has_to_xyz_d50 = true;
    profile
}
const fn xyz_d50_profile_value() -> IccProfile {
    let mut profile = IccProfile::zeroed();
    profile.data_color_space = SIGNATURE_RGB;
    profile.pcs = SIGNATURE_XYZ;
    profile.trc = [Curve::from_parametric(IDENTITY_FUNCTION); 3];
    profile.to_xyz_d50 = Matrix3x3 {
        vals: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    profile.has_trc = true;
    profile.has_to_xyz_d50 = true;
    profile
}
static SRGB_PROFILE: ImmutableProfile = ImmutableProfile(srgb_profile_value());
static XYZ_D50_PROFILE: ImmutableProfile = ImmutableProfile(xyz_d50_profile_value());
#[expect(
    clippy::excessive_precision,
    reason = "Preserve pristine inverse sRGB decimal literals."
)]
static SRGB_INVERSE_FUNCTION: TransferFunction = TransferFunction {
    g: 0.416666657,
    a: 1.137283325,
    b: -0.0,
    c: 12.920000076,
    d: 0.003130805,
    e: -0.054969788,
    f: -0.0,
};
static IDENTITY_TRANSFER_FUNCTION: TransferFunction = IDENTITY_FUNCTION;

/// Original immutable sRGB profile; the returned static storage must not be
/// written through a cast. Repeated calls retain pointer identity.
pub fn srgb_profile() -> *const IccProfile {
    &raw const SRGB_PROFILE.0
}
/// Original immutable XYZ D50 profile (whose data color space is source RGB).
pub fn xyz_d50_profile() -> *const IccProfile {
    &raw const XYZ_D50_PROFILE.0
}
/// Original sRGB transfer-function accessor, aliasing the first profile curve.
pub fn srgb_transfer_function() -> *const TransferFunction {
    // SAFETY: this static's three parametric curves are fully initialized and
    // immutable. Address formation does not read an inactive table branch.
    unsafe { ptr::addr_of!((*srgb_profile()).trc[0].parametric.function) }
}
/// Original immutable inverse sRGB transfer-function constant.
pub fn srgb_inverse_transfer_function() -> *const TransferFunction {
    &raw const SRGB_INVERSE_FUNCTION
}
/// Original immutable identity transfer-function constant.
pub fn identity_transfer_function() -> *const TransferFunction {
    &raw const IDENTITY_TRANSFER_FUNCTION
}
