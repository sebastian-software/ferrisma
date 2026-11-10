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

// Mechanically translated from crates/aom/reference/macos/debug/generated/config/aom_config.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
static mut cfg: *const core::ffi::c_char = b"cmake ../aom-native-source -G \"Ninja\" -DAOM_TARGET_CPU=generic -DCONFIG_TUNE_VMAF=0 -DENABLE_DOCS=OFF\0"
    as *const u8 as *const core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_build_config() -> *const core::ffi::c_char {
    return cfg;
}
