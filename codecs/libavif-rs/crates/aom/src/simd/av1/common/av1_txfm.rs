/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from the `#if HAVE_NEON` portions of
// upstream/aom/av1/common/av1_txfm.c (the reduced bit-width constant tables)
// and upstream/aom/av1/common/av1_txfm.h (their extern declarations and
// inline accessors) (ADR 0056). The mechanical translation was generated with
// HAVE_NEON=0, so these items do not exist there. Names and values are copied
// exactly; the tables are referenced only by the hand-translated Neon kernels,
// so they are plain Rust statics without exported C symbols.
#![allow(dead_code)]

// From av1/common/av1_txfm.h (shared by every translation unit).
const cos_bit_min: i32 = 10;

// ---- av1/common/av1_txfm.c ----

// The reduced bit-width arrays are only used in the Arm Neon implementations
// in av1_fwd_txfm2d_neon.c for now.
// Constants are stored in groups of four, where symmetrical constants in the
// cospi array are stored adjacent in memory, followed immediately by the same
// constants but negated, i.e.:
//   f(i,j) = (int)round(cos(PI*j/128) * (1<<(cos_bit_min+i))) << (3-i)
// and then in memory we store 4-tuples of constants together as:
//   f4(i,j) = [ f(i,j), f(i,64-j), -f(i,j), -f(i,64-j) ]
//
// Constants are stored in Q2.13 format, see:
// https://en.wikipedia.org/wiki/Q_(number_format)
//
// The order of the constants is such that increasing subdivisions of 64 store
// f4 tuples contiguously:
// av1_cospi_arr_q13_data[i] = {
//   f4(i,32),  // f(i,32) twice
//   f4(i,16),  // f(i,16) and f(i,48), f4(i,32) skipped since present above.
//   f4(i,8), f(i,24), // f4(i,16) and f4(i,32) skipped since present above.
//   f4(i,4), f(i,12), f4(i,20), f4(i,28),
//   f4(i,2), f4(i,6), f4(i,10), f4(i,14), f4(i,18), ...
//   f4(i,1), f4(i,3), f4(i,5), f4(i,7), f4(i,9), f4(i,11), ...
// }
pub static av1_cospi_arr_q13_data: [[i16; 128]; 4] = [
    [
        5792, 5792, -5792, -5792, 7568, 3136, -7568, -3136, 8032, 1600,
        -8032, -1600, 6808, 4552, -6808, -4552, 8152, 800, -8152, -800,
        7840, 2376, -7840, -2376, 7224, 3864, -7224, -3864, 6336, 5200,
        -6336, -5200, 8184, 400, -8184, -400, 8104, 1200, -8104, -1200,
        7944, 1992, -7944, -1992, 7712, 2760, -7712, -2760, 7408, 3504,
        -7408, -3504, 7024, 4208, -7024, -4208, 6576, 4880, -6576, -4880,
        6072, 5504, -6072, -5504, 8192, 200, -8192, -200, 8168, 600,
        -8168, -600, 8128, 1000, -8128, -1000, 8072, 1400, -8072, -1400,
        7992, 1792, -7992, -1792, 7896, 2184, -7896, -2184, 7776, 2568,
        -7776, -2568, 7640, 2952, -7640, -2952, 7488, 3320, -7488, -3320,
        7320, 3680, -7320, -3680, 7128, 4040, -7128, -4040, 6920, 4384,
        -6920, -4384, 6696, 4720, -6696, -4720, 6456, 5040, -6456, -5040,
        6200, 5352, -6200, -5352, 5936, 5648, -5936, -5648,
    ],
    [
        5792, 5792, -5792, -5792, 7568, 3136, -7568, -3136, 8036, 1600,
        -8036, -1600, 6812, 4552, -6812, -4552, 8152, 804, -8152, -804,
        7840, 2380, -7840, -2380, 7224, 3860, -7224, -3860, 6332, 5196,
        -6332, -5196, 8184, 400, -8184, -400, 8104, 1204, -8104, -1204,
        7948, 1992, -7948, -1992, 7712, 2760, -7712, -2760, 7404, 3504,
        -7404, -3504, 7028, 4212, -7028, -4212, 6580, 4880, -6580, -4880,
        6068, 5500, -6068, -5500, 8188, 200, -8188, -200, 8168, 604,
        -8168, -604, 8132, 1004, -8132, -1004, 8072, 1400, -8072, -1400,
        7992, 1796, -7992, -1796, 7896, 2184, -7896, -2184, 7780, 2568,
        -7780, -2568, 7644, 2948, -7644, -2948, 7488, 3320, -7488, -3320,
        7316, 3684, -7316, -3684, 7128, 4036, -7128, -4036, 6920, 4384,
        -6920, -4384, 6696, 4716, -6696, -4716, 6460, 5040, -6460, -5040,
        6204, 5352, -6204, -5352, 5932, 5648, -5932, -5648,
    ],
    [
        5792, 5792, -5792, -5792, 7568, 3134, -7568, -3134, 8034, 1598,
        -8034, -1598, 6812, 4552, -6812, -4552, 8152, 802, -8152, -802,
        7840, 2378, -7840, -2378, 7224, 3862, -7224, -3862, 6332, 5196,
        -6332, -5196, 8182, 402, -8182, -402, 8104, 1202, -8104, -1202,
        7946, 1990, -7946, -1990, 7714, 2760, -7714, -2760, 7406, 3502,
        -7406, -3502, 7026, 4212, -7026, -4212, 6580, 4880, -6580, -4880,
        6070, 5502, -6070, -5502, 8190, 202, -8190, -202, 8170, 602,
        -8170, -602, 8130, 1002, -8130, -1002, 8072, 1400, -8072, -1400,
        7992, 1794, -7992, -1794, 7896, 2184, -7896, -2184, 7778, 2570,
        -7778, -2570, 7644, 2948, -7644, -2948, 7490, 3320, -7490, -3320,
        7318, 3684, -7318, -3684, 7128, 4038, -7128, -4038, 6922, 4382,
        -6922, -4382, 6698, 4718, -6698, -4718, 6458, 5040, -6458, -5040,
        6204, 5350, -6204, -5350, 5934, 5648, -5934, -5648,
    ],
    [
        5793, 5793, -5793, -5793, 7568, 3135, -7568, -3135, 8035, 1598,
        -8035, -1598, 6811, 4551, -6811, -4551, 8153, 803, -8153, -803,
        7839, 2378, -7839, -2378, 7225, 3862, -7225, -3862, 6333, 5197,
        -6333, -5197, 8182, 402, -8182, -402, 8103, 1202, -8103, -1202,
        7946, 1990, -7946, -1990, 7713, 2760, -7713, -2760, 7405, 3503,
        -7405, -3503, 7027, 4212, -7027, -4212, 6580, 4880, -6580, -4880,
        6070, 5501, -6070, -5501, 8190, 201, -8190, -201, 8170, 603,
        -8170, -603, 8130, 1003, -8130, -1003, 8071, 1401, -8071, -1401,
        7993, 1795, -7993, -1795, 7895, 2185, -7895, -2185, 7779, 2570,
        -7779, -2570, 7643, 2948, -7643, -2948, 7489, 3320, -7489, -3320,
        7317, 3683, -7317, -3683, 7128, 4038, -7128, -4038, 6921, 4383,
        -6921, -4383, 6698, 4717, -6698, -4717, 6458, 5040, -6458, -5040,
        6203, 5351, -6203, -5351, 5933, 5649, -5933, -5649,
    ],
];

// av1_sinpi_arr_q13_data[i][j] =
//   round((sqrt2 * sin((j+1)*Pi/9) * 2/3) * (1 << (cos_bit_min + i))) << (3-i)
// modified so that elements j=0,1 sum to element j=3.
// See also: https://en.wikipedia.org/wiki/Q_(number_format)
pub static av1_sinpi_arr_q13_data: [[i16; 4]; 4] = [
    [
        2640, 4968, 6688, 7608,
    ],
    [
        2640, 4964, 6688, 7604,
    ],
    [
        2642, 4964, 6688, 7606,
    ],
    [
        2642, 4964, 6689, 7606,
    ],
];

// Constants are stored in pairs, where symmetrical constants in the
// cospi array are stored adjacent in memory, i.e.:
//   f(i,j) = (int)round(cos(PI*j/128) * (1<<(cos_bit_min+i)))
// and then in memory we store 4-tuples of constants together as:
//   f2(i,j) = [ f(i,j), f(i,64-j) ]
pub static av1_cospi_arr_s32_data: [[i32; 66]; 4] = [
    [
        1024, 0, 1024, 25, 1023, 50, 1021, 75, 1019, 100,
        1016, 125, 1013, 150, 1009, 175, 1004, 200, 999, 224,
        993, 249, 987, 273, 980, 297, 972, 321, 964, 345,
        955, 369, 946, 392, 936, 415, 926, 438, 915, 460,
        903, 483, 891, 505, 878, 526, 865, 548, 851, 569,
        837, 590, 822, 610, 807, 630, 792, 650, 775, 669,
        759, 688, 742, 706, 724, 724,
    ],
    [
        2048, 0, 2047, 50, 2046, 100, 2042, 151, 2038, 201,
        2033, 251, 2026, 301, 2018, 350, 2009, 400, 1998, 449,
        1987, 498, 1974, 546, 1960, 595, 1945, 642, 1928, 690,
        1911, 737, 1892, 784, 1872, 830, 1851, 876, 1829, 921,
        1806, 965, 1782, 1009, 1757, 1053, 1730, 1096, 1703, 1138,
        1674, 1179, 1645, 1220, 1615, 1260, 1583, 1299, 1551, 1338,
        1517, 1375, 1483, 1412, 1448, 1448,
    ],
    [
        4096, 0, 4095, 101, 4091, 201, 4085, 301, 4076, 401,
        4065, 501, 4052, 601, 4036, 700, 4017, 799, 3996, 897,
        3973, 995, 3948, 1092, 3920, 1189, 3889, 1285, 3857, 1380,
        3822, 1474, 3784, 1567, 3745, 1660, 3703, 1751, 3659, 1842,
        3612, 1931, 3564, 2019, 3513, 2106, 3461, 2191, 3406, 2276,
        3349, 2359, 3290, 2440, 3229, 2520, 3166, 2598, 3102, 2675,
        3035, 2751, 2967, 2824, 2896, 2896,
    ],
    [
        8192, 0, 8190, 201, 8182, 402, 8170, 603, 8153, 803,
        8130, 1003, 8103, 1202, 8071, 1401, 8035, 1598, 7993, 1795,
        7946, 1990, 7895, 2185, 7839, 2378, 7779, 2570, 7713, 2760,
        7643, 2948, 7568, 3135, 7489, 3320, 7405, 3503, 7317, 3683,
        7225, 3862, 7128, 4038, 7027, 4212, 6921, 4383, 6811, 4551,
        6698, 4717, 6580, 4880, 6458, 5040, 6333, 5197, 6203, 5351,
        6070, 5501, 5933, 5649, 5793, 5793,
    ],
];

// ---- av1/common/av1_txfm.h ----

// The reduced bit-width and permuted arrays are only used in the Arm Neon
// implementations in av1_fwd_txfm2d_neon.c and highbd_fwd_txfm_neon.c for now.
// Store cospi/sinpi costants in Q2.13 format.
// See: https://en.wikipedia.org/wiki/Q_(number_format)

#[inline(always)]
pub unsafe fn cospi_arr_q13(n: i32) -> *const i16 {
    av1_cospi_arr_q13_data[(n - cos_bit_min) as usize].as_ptr()
}

#[inline(always)]
pub unsafe fn sinpi_arr_q13(n: i32) -> *const i16 {
    av1_sinpi_arr_q13_data[(n - cos_bit_min) as usize].as_ptr()
}

#[inline(always)]
pub unsafe fn cospi_arr_s32(n: i32) -> *const i32 {
    av1_cospi_arr_s32_data[(n - cos_bit_min) as usize].as_ptr()
}
