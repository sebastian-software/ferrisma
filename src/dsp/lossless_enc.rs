// libwebp v1.6.0, src/dsp/lossless_enc.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Image transform methods for lossless encoder.
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)
//          Urvang Joshi (urvang@google.com)

//! Translation of `src/dsp/lossless_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Predictor signatures.** The encoder form of the batch predictors,
//!    `PredictorSubFunc`, is `(data, in, upper, num_pixels, out)`: the input
//!    row and the row above are positions in one buffer (the scratch rows), and
//!    the residuals go to a separate slice (as for the decoder's predictors in
//!    `src/dsp/lossless.rs`). C passes `NULL` as the row above to modes 0 and
//!    1, which do not read it; position 0 stands for it. The C
//!    `assert(upper != NULL)` has no counterpart.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. `VP8LExtraCost` with an odd or short length; see
//!    `src/enc/near_lossless_enc.rs`, `# Excluded undefined behavior`, item 1.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **SIMD variants.** `lossless_enc_sse2.c`, `lossless_enc_sse41.c`,
//!    `lossless_enc_avx2.c`, `lossless_enc_neon.c` and the `_SSE` pointers of
//!    `lossless.h` belong to the SIMD modules (ADR 0008). MIPS
//!    (`lossless_enc_mips32.c`, `lossless_enc_mips_dsp_r2.c`,
//!    `lossless_enc_msa.c`) is excluded in `tools/units.json`.
//! 2. **Floating point.** See `src/enc/histogram_enc.rs`,
//!    `# Checks not applicable`, item 1.

use crate::dsp::lossless::{
    VP8LBitEntropy, VP8LMultipliers, VP8LStreaks, vp8l_predictor2_c, vp8l_predictor3_c,
    vp8l_predictor4_c, vp8l_predictor5_c, vp8l_predictor6_c, vp8l_predictor7_c, vp8l_predictor8_c,
    vp8l_predictor9_c, vp8l_predictor10_c, vp8l_predictor11_c, vp8l_predictor12_c,
    vp8l_predictor13_c,
};
use crate::dsp::lossless_common::{
    APPROX_LOG_MAX, APPROX_LOG_WITH_CORRECTION_MAX, LOG_2_PRECISION_BITS, LOG_2_RECIPROCAL_FIXED,
    LOG_2_RECIPROCAL_FIXED_DOUBLE, LOG_LOOKUP_IDX_MAX, VP8LPrefixCode, div_round, vp8l_fast_s_log2,
    vp8l_sub_pixels,
};
use crate::enc::histogram_enc::VP8L_NON_TRIVIAL_SYM;
use crate::utils::utils::bits_log2_floor;
use crate::webp::format_constants::ARGB_BLACK;

/// Converts a C `int` count that is not negative to a slice length; a negative
/// count runs no C loop iteration and becomes 0.
#[inline]
fn count(n: i32) -> usize {
    usize::try_from(n).unwrap_or(0)
}

// upstream: src/dsp/lossless_enc.c kLog2Table
/// lookup table for small values of log2(int) * (1 << `LOG_2_PRECISION_BITS`).
/// Obtained in Python with:
///
/// ```text
/// a = [ str(round((1<<23)*math.log2(i))) if i else "0" for i in range(256)]
/// print(',\n'.join(['  '+','.join(v)
///       for v in batched([i.rjust(9) for i in a],7)]))
/// ```
pub static LOG2_TABLE: [u32; LOG_LOOKUP_IDX_MAX as usize] = [
    0, 0, 8_388_608, 13_295_629, 16_777_216, 19_477_745, 21_684_237, 23_549_800, 25_165_824,
    26_591_258, 27_866_353, 29_019_816, 30_072_845, 31_041_538, 31_938_408, 32_773_374, 33_554_432,
    34_288_123, 34_979_866, 35_634_199, 36_254_961, 36_845_429, 37_408_424, 37_946_388, 38_461_453,
    38_955_489, 39_430_146, 39_886_887, 40_327_016, 40_751_698, 41_161_982, 41_558_811, 41_943_040,
    42_315_445, 42_676_731, 43_027_545, 43_368_474, 43_700_062, 44_022_807, 44_337_167, 44_643_569,
    44_942_404, 45_234_037, 45_518_808, 45_797_032, 46_069_003, 46_334_996, 46_595_268, 46_850_061,
    47_099_600, 47_344_097, 47_583_753, 47_818_754, 48_049_279, 48_275_495, 48_497_560, 48_715_624,
    48_929_828, 49_140_306, 49_347_187, 49_550_590, 49_750_631, 49_947_419, 50_141_058, 50_331_648,
    50_519_283, 50_704_053, 50_886_044, 51_065_339, 51_242_017, 51_416_153, 51_587_818, 51_757_082,
    51_924_012, 52_088_670, 52_251_118, 52_411_415, 52_569_616, 52_725_775, 52_879_946, 53_032_177,
    53_182_516, 53_331_012, 53_477_707, 53_622_645, 53_765_868, 53_907_416, 54_047_327, 54_185_640,
    54_322_389, 54_457_611, 54_591_338, 54_723_604, 54_854_440, 54_983_876, 55_111_943, 55_238_669,
    55_364_082, 55_488_208, 55_611_074, 55_732_705, 55_853_126, 55_972_361, 56_090_432, 56_207_362,
    56_323_174, 56_437_887, 56_551_524, 56_664_103, 56_775_645, 56_886_168, 56_995_691, 57_104_232,
    57_211_808, 57_318_436, 57_424_133, 57_528_914, 57_632_796, 57_735_795, 57_837_923, 57_939_198,
    58_039_632, 58_139_239, 58_238_033, 58_336_027, 58_433_234, 58_529_666, 58_625_336, 58_720_256,
    58_814_437, 58_907_891, 59_000_628, 59_092_661, 59_183_999, 59_274_652, 59_364_632, 59_453_947,
    59_542_609, 59_630_625, 59_718_006, 59_804_761, 59_890_898, 59_976_426, 60_061_354, 60_145_690,
    60_229_443, 60_312_620, 60_395_229, 60_477_278, 60_558_775, 60_639_726, 60_720_140, 60_800_023,
    60_879_382, 60_958_224, 61_036_555, 61_114_383, 61_191_714, 61_268_554, 61_344_908, 61_420_785,
    61_496_188, 61_571_124, 61_645_600, 61_719_620, 61_793_189, 61_866_315, 61_939_001, 62_011_253,
    62_083_076, 62_154_476, 62_225_457, 62_296_024, 62_366_182, 62_435_935, 62_505_289, 62_574_248,
    62_642_816, 62_710_997, 62_778_797, 62_846_219, 62_913_267, 62_979_946, 63_046_260, 63_112_212,
    63_177_807, 63_243_048, 63_307_939, 63_372_484, 63_436_687, 63_500_551, 63_564_080, 63_627_277,
    63_690_146, 63_752_690, 63_814_912, 63_876_816, 63_938_405, 63_999_682, 64_060_650, 64_121_313,
    64_181_673, 64_241_734, 64_301_498, 64_360_969, 64_420_148, 64_479_040, 64_537_646, 64_595_970,
    64_654_014, 64_711_782, 64_769_274, 64_826_495, 64_883_447, 64_940_132, 64_996_553, 65_052_711,
    65_108_611, 65_164_253, 65_219_641, 65_274_776, 65_329_662, 65_384_299, 65_438_691, 65_492_840,
    65_546_747, 65_600_416, 65_653_847, 65_707_044, 65_760_008, 65_812_741, 65_865_245, 65_917_522,
    65_969_575, 66_021_404, 66_073_013, 66_124_403, 66_175_575, 66_226_531, 66_277_275, 66_327_806,
    66_378_127, 66_428_240, 66_478_146, 66_527_847, 66_577_345, 66_626_641, 66_675_737, 66_724_635,
    66_773_336, 66_821_842, 66_870_154, 66_918_274, 66_966_204, 67_013_944, 67_061_497,
];

// upstream: src/dsp/lossless_enc.c kSLog2Table
/// lookup table for small values of int*log2(int) * (1 << `LOG_2_PRECISION_BITS`).
/// Obtained in Python with:
///
/// ```text
/// a=[ "%d"%i if i<(1<<32) else "%dull"%i
///     for i in [ round((1<<LOG_2_PRECISION_BITS)*math.log2(i)*i) if i
///     else 0 for i in range(256)]]
/// print(',\n '.join([','.join(v) for v in batched([i.rjust(15)
///                      for i in a],4)]))
/// ```
pub static S_LOG2_TABLE: [u64; LOG_LOOKUP_IDX_MAX as usize] = [
    0,
    0,
    16_777_216,
    39_886_887,
    67_108_864,
    97_388_723,
    130_105_423,
    164_848_600,
    201_326_592,
    239_321_324,
    278_663_526,
    319_217_973,
    360_874_141,
    403_539_997,
    447_137_711,
    491_600_606,
    536_870_912,
    582_898_099,
    629_637_592,
    677_049_776,
    725_099_212,
    773_754_010,
    822_985_323,
    872_766_924,
    923_074_875,
    973_887_230,
    1_025_183_802,
    1_076_945_958,
    1_129_156_447,
    1_181_799_249,
    1_234_859_451,
    1_288_323_135,
    1_342_177_280,
    1_396_409_681,
    1_451_008_871,
    1_505_964_059,
    1_561_265_072,
    1_616_902_301,
    1_672_866_655,
    1_729_149_526,
    1_785_742_744,
    1_842_638_548,
    1_899_829_557,
    1_957_308_741,
    2_015_069_397,
    2_073_105_127,
    2_131_409_817,
    2_189_977_618,
    2_248_802_933,
    2_307_880_396,
    2_367_204_859,
    2_426_771_383,
    2_486_575_220,
    2_546_611_805,
    2_606_876_748,
    2_667_365_819,
    2_728_074_942,
    2_789_000_187,
    2_850_137_762,
    2_911_484_006,
    2_973_035_382,
    3_034_788_471,
    3_096_739_966,
    3_158_886_666,
    3_221_225_472,
    3_283_753_383,
    3_346_467_489,
    3_409_364_969,
    3_472_443_085,
    3_535_699_182,
    3_599_130_679,
    3_662_735_070,
    3_726_509_920,
    3_790_452_862,
    3_854_561_593,
    3_918_833_872,
    3_983_267_519,
    4_047_860_410,
    4_112_610_476,
    4_177_515_704,
    4_242_574_127,
    4_307_783_833,
    4_373_142_952,
    4_438_649_662,
    4_504_302_186,
    4_570_098_787,
    4_636_037_770,
    4_702_117_480,
    4_768_336_298,
    4_834_692_645,
    4_901_184_974,
    4_967_811_774,
    5_034_571_569,
    5_101_462_912,
    5_168_484_389,
    5_235_634_615,
    5_302_912_235,
    5_370_315_922,
    5_437_844_376,
    5_505_496_324,
    5_573_270_518,
    5_641_165_737,
    5_709_180_782,
    5_777_314_477,
    5_845_565_671,
    5_913_933_235,
    5_982_416_059,
    6_051_013_057,
    6_119_723_161,
    6_188_545_324,
    6_257_478_518,
    6_326_521_733,
    6_395_673_979,
    6_464_934_282,
    6_534_301_685,
    6_603_775_250,
    6_673_354_052,
    6_743_037_185,
    6_812_823_756,
    6_882_712_890,
    6_952_703_725,
    7_022_795_412,
    7_092_987_118,
    7_163_278_025,
    7_233_667_324,
    7_304_154_222,
    7_374_737_939,
    7_445_417_707,
    7_516_192_768,
    7_587_062_379,
    7_658_025_806,
    7_729_082_328,
    7_800_231_234,
    7_871_471_825,
    7_942_803_410,
    8_014_225_311,
    8_085_736_859,
    8_157_337_394,
    8_229_026_267,
    8_300_802_839,
    8_372_666_477,
    8_444_616_560,
    8_516_652_476,
    8_588_773_618,
    8_660_979_393,
    8_733_269_211,
    8_805_642_493,
    8_878_098_667,
    8_950_637_170,
    9_023_257_446,
    9_095_958_945,
    9_168_741_125,
    9_241_603_454,
    9_314_545_403,
    9_387_566_451,
    9_460_666_086,
    9_533_843_800,
    9_607_099_093,
    9_680_431_471,
    9_753_840_445,
    9_827_325_535,
    9_900_886_263,
    9_974_522_161,
    10_048_232_765,
    10_122_017_615,
    10_195_876_260,
    10_269_808_253,
    10_343_813_150,
    10_417_890_516,
    10_492_039_919,
    10_566_260_934,
    10_640_553_138,
    10_714_916_116,
    10_789_349_456,
    10_863_852_751,
    10_938_425_600,
    11_013_067_604,
    11_087_778_372,
    11_162_557_513,
    11_237_404_645,
    11_312_319_387,
    11_387_301_364,
    11_462_350_205,
    11_537_465_541,
    11_612_647_010,
    11_687_894_253,
    11_763_206_912,
    11_838_584_638,
    11_914_027_082,
    11_989_533_899,
    12_065_104_750,
    12_140_739_296,
    12_216_437_206,
    12_292_198_148,
    12_368_021_795,
    12_443_907_826,
    12_519_855_920,
    12_595_865_759,
    12_671_937_032,
    12_748_069_427,
    12_824_262_637,
    12_900_516_358,
    12_976_830_290,
    13_053_204_134,
    13_129_637_595,
    13_206_130_381,
    13_282_682_202,
    13_359_292_772,
    13_435_961_806,
    13_512_689_025,
    13_589_474_149,
    13_666_316_903,
    13_743_217_014,
    13_820_174_211,
    13_897_188_225,
    13_974_258_793,
    14_051_385_649,
    14_128_568_535,
    14_205_807_192,
    14_283_101_363,
    14_360_450_796,
    14_437_855_239,
    14_515_314_443,
    14_592_828_162,
    14_670_396_151,
    14_748_018_167,
    14_825_693_972,
    14_903_423_326,
    14_981_205_995,
    15_059_041_743,
    15_136_930_339,
    15_214_871_554,
    15_292_865_160,
    15_370_910_930,
    15_449_008_641,
    15_527_158_071,
    15_605_359_001,
    15_683_611_210,
    15_761_914_485,
    15_840_268_608,
    15_918_673_369,
    15_997_128_556,
    16_075_633_960,
    16_154_189_373,
    16_232_794_589,
    16_311_449_405,
    16_390_153_617,
    16_468_907_026,
    16_547_709_431,
    16_626_560_636,
    16_705_460_444,
    16_784_408_661,
    16_863_405_094,
    16_942_449_552,
    17_021_541_845,
    17_100_681_785,
];

/// A `VP8LPrefixCode` entry of the table below.
const fn pc(code: i8, extra_bits: i8) -> VP8LPrefixCode {
    VP8LPrefixCode { code, extra_bits }
}

// upstream: src/dsp/lossless_enc.c kPrefixEncodeCode
/// These tables are derived using `VP8LPrefixEncodeNoLUT`.
pub static PREFIX_ENCODE_CODE: [VP8LPrefixCode; 512] = [
    pc(0, 0),
    pc(0, 0),
    pc(1, 0),
    pc(2, 0),
    pc(3, 0),
    pc(4, 1),
    pc(4, 1),
    pc(5, 1),
    pc(5, 1),
    pc(6, 2),
    pc(6, 2),
    pc(6, 2),
    pc(6, 2),
    pc(7, 2),
    pc(7, 2),
    pc(7, 2),
    pc(7, 2),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(8, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(9, 3),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(10, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(11, 4),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(12, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(13, 5),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(14, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(15, 6),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(16, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
    pc(17, 7),
];

// upstream: src/dsp/lossless_enc.c kPrefixEncodeExtraBitsValue
pub static PREFIX_ENCODE_EXTRA_BITS_VALUE: [u8; 512] = [
    0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6,
    7, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
    58, 59, 60, 61, 62, 63, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
    44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 0, 1, 2, 3, 4,
    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
    54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
    78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
    101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
    120, 121, 122, 123, 124, 125, 126, 127, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
    88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
    109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
];

// upstream: src/dsp/lossless_enc.c FastSLog2Slow_C
/// `v * log2(v)` in fixed point for `v` of at least `LOG_LOOKUP_IDX_MAX`. Only
/// the branch that uses `BitsLog2Floor` is translated: GCC and Clang define the
/// builtin, so `WEBP_HAVE_SLOW_CLZ_CTZ` is unset.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the rounded double to uint64_t"
)]
fn fast_s_log2_slow_c(mut v: u32) -> u64 {
    debug_assert!(v >= LOG_LOOKUP_IDX_MAX.cast_unsigned());
    if v < APPROX_LOG_WITH_CORRECTION_MAX.cast_unsigned() {
        let orig_v = u64::from(v);
        // use clz if available
        let log_cnt = i64::from(bits_log2_floor(v) - 7).cast_unsigned();
        let y: u32 = 1 << log_cnt;
        v >>= log_cnt;
        // vf = (2^log_cnt) * Xf; where y = 2^log_cnt and Xf < 256
        // Xf = floor(Xf) * (1 + (v % y) / v)
        // log2(Xf) = log2(floor(Xf)) + log2(1 + (v % y) / v)
        // The correction factor: log(1 + d) ~ d; for very small d values, so
        // log2(1 + (v % y) / v) ~ LOG_2_RECIPROCAL * (v % y)/v
        let correction = LOG_2_RECIPROCAL_FIXED.wrapping_mul(orig_v & u64::from(y - 1));
        orig_v
            .wrapping_mul(
                u64::from(LOG2_TABLE[v as usize]).wrapping_add(log_cnt << LOG_2_PRECISION_BITS),
            )
            .wrapping_add(correction)
    } else {
        (LOG_2_RECIPROCAL_FIXED_DOUBLE * f64::from(v) * f64::from(v).ln() + 0.5) as u64
    }
}

// upstream: src/dsp/lossless_enc.c FastLog2Slow_C
/// `log2(v)` in fixed point for `v` of at least `LOG_LOOKUP_IDX_MAX`, with the
/// `BitsLog2Floor` branch as in [`fast_s_log2_slow_c`].
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions of the rounded quotient and double to uint32_t"
)]
fn fast_log2_slow_c(mut v: u32) -> u32 {
    debug_assert!(v >= LOG_LOOKUP_IDX_MAX.cast_unsigned());
    if v < APPROX_LOG_WITH_CORRECTION_MAX.cast_unsigned() {
        let orig_v = v;
        // use clz if available
        let log_cnt = (bits_log2_floor(v) - 7).cast_unsigned();
        let y: u32 = 1 << log_cnt;
        v >>= log_cnt;
        let mut log_2 = LOG2_TABLE[v as usize].wrapping_add(log_cnt << LOG_2_PRECISION_BITS);
        if orig_v >= APPROX_LOG_MAX.cast_unsigned() {
            // Since the division is still expensive, add this correction factor only
            // for large values of 'v'.
            let correction = LOG_2_RECIPROCAL_FIXED.wrapping_mul(u64::from(orig_v & (y - 1)));
            log_2 =
                log_2.wrapping_add(div_round(correction.cast_signed(), i64::from(orig_v)) as u32);
        }
        log_2
    } else {
        (LOG_2_RECIPROCAL_FIXED_DOUBLE * f64::from(v).ln() + 0.5) as u32
    }
}

//------------------------------------------------------------------------------
// Methods to calculate Entropy (Shannon).

// upstream: src/dsp/lossless_enc.c CombinedShannonEntropy_C
/// Compute the combined Shanon's entropy for distribution {X} and {X+Y}.
/// `x` and `y` hold at least 256 entries. The unsigned sums wrap, as in C.
fn combined_shannon_entropy_c(x: &[u32], y: &[u32]) -> u64 {
    let mut retval: u64 = 0;
    let mut sum_x: u32 = 0;
    let mut sum_xy: u32 = 0;
    for (&xi, &yi) in x[..256].iter().zip(&y[..256]) {
        if xi != 0 {
            let xy = xi.wrapping_add(yi);
            sum_x = sum_x.wrapping_add(xi);
            retval = retval.wrapping_add(vp8l_fast_s_log2(xi));
            sum_xy = sum_xy.wrapping_add(xy);
            retval = retval.wrapping_add(vp8l_fast_s_log2(xy));
        } else if yi != 0 {
            sum_xy = sum_xy.wrapping_add(yi);
            retval = retval.wrapping_add(vp8l_fast_s_log2(yi));
        }
    }
    vp8l_fast_s_log2(sum_x)
        .wrapping_add(vp8l_fast_s_log2(sum_xy))
        .wrapping_sub(retval)
}

// upstream: src/dsp/lossless_enc.c ShannonEntropy_C
/// The C loop reads each count into an `int` and passes it back as `uint32_t`;
/// the two conversions keep the bits, so the count is used unconverted.
fn shannon_entropy_c(x: &[u32], n: i32) -> u64 {
    let mut retval: u64 = 0;
    let mut sum_x: u32 = 0;
    for &xi in &x[..count(n)] {
        if xi != 0 {
            sum_x = sum_x.wrapping_add(xi);
            retval = retval.wrapping_add(vp8l_fast_s_log2(xi));
        }
    }
    vp8l_fast_s_log2(sum_x).wrapping_sub(retval)
}

// upstream: src/dsp/lossless_enc.c VP8LBitEntropyInit
pub fn vp8l_bit_entropy_init(entropy: &mut VP8LBitEntropy) {
    entropy.entropy = 0;
    entropy.sum = 0;
    entropy.nonzeros = 0;
    entropy.max_val = 0;
    entropy.nonzero_code = u32::from(VP8L_NON_TRIVIAL_SYM);
}

// upstream: src/dsp/lossless_enc.c VP8LBitsEntropyUnrefined
/// The entropy of the first `n` entries of `array`, without refinement.
pub fn vp8l_bits_entropy_unrefined(array: &[u32], n: i32, entropy: &mut VP8LBitEntropy) {
    vp8l_bit_entropy_init(entropy);

    for (i, &value) in (0_u32..).zip(&array[..count(n)]) {
        if value != 0 {
            entropy.sum = entropy.sum.wrapping_add(value);
            entropy.nonzero_code = i;
            entropy.nonzeros += 1;
            entropy.entropy = entropy.entropy.wrapping_add(vp8l_fast_s_log2(value));
            if entropy.max_val < value {
                entropy.max_val = value;
            }
        }
    }
    entropy.entropy = vp8l_fast_s_log2(entropy.sum).wrapping_sub(entropy.entropy);
}

// upstream: src/dsp/lossless_enc.c GetEntropyUnrefinedHelper
#[inline]
fn get_entropy_unrefined_helper(
    val: u32,
    i: i32,
    val_prev: &mut u32,
    i_prev: &mut i32,
    bit_entropy: &mut VP8LBitEntropy,
    stats: &mut VP8LStreaks,
) {
    let streak = i - *i_prev;

    // Gather info for the bit entropy.
    if *val_prev != 0 {
        bit_entropy.sum = bit_entropy
            .sum
            .wrapping_add(val_prev.wrapping_mul(streak.cast_unsigned()));
        bit_entropy.nonzeros += streak;
        bit_entropy.nonzero_code = i_prev.cast_unsigned();
        bit_entropy.entropy = bit_entropy.entropy.wrapping_add(
            vp8l_fast_s_log2(*val_prev).wrapping_mul(i64::from(streak).cast_unsigned()),
        );
        if bit_entropy.max_val < *val_prev {
            bit_entropy.max_val = *val_prev;
        }
    }

    // Gather info for the Huffman cost.
    let nonzero = usize::from(*val_prev != 0);
    stats.counts[nonzero] += i32::from(streak > 3);
    stats.streaks[nonzero][usize::from(streak > 3)] += streak;

    *val_prev = val;
    *i_prev = i;
}

// upstream: src/dsp/lossless_enc.c GetEntropyUnrefined_C
fn get_entropy_unrefined_c(
    x: &[u32],
    length: i32,
    bit_entropy: &mut VP8LBitEntropy,
    stats: &mut VP8LStreaks,
) {
    let mut i_prev: i32 = 0;
    let mut x_prev: u32 = x[0];

    *stats = VP8LStreaks::default();
    vp8l_bit_entropy_init(bit_entropy);

    let mut i: i32 = 1;
    while i < length {
        let xi = x[count(i)];
        if xi != x_prev {
            get_entropy_unrefined_helper(xi, i, &mut x_prev, &mut i_prev, bit_entropy, stats);
        }
        i += 1;
    }
    get_entropy_unrefined_helper(0, i, &mut x_prev, &mut i_prev, bit_entropy, stats);

    bit_entropy.entropy = vp8l_fast_s_log2(bit_entropy.sum).wrapping_sub(bit_entropy.entropy);
}

// upstream: src/dsp/lossless_enc.c GetCombinedEntropyUnrefined_C
fn get_combined_entropy_unrefined_c(
    x: &[u32],
    y: &[u32],
    length: i32,
    bit_entropy: &mut VP8LBitEntropy,
    stats: &mut VP8LStreaks,
) {
    let mut i_prev: i32 = 0;
    let mut xy_prev: u32 = x[0].wrapping_add(y[0]);

    *stats = VP8LStreaks::default();
    vp8l_bit_entropy_init(bit_entropy);

    let mut i: i32 = 1;
    while i < length {
        let xy = x[count(i)].wrapping_add(y[count(i)]);
        if xy != xy_prev {
            get_entropy_unrefined_helper(xy, i, &mut xy_prev, &mut i_prev, bit_entropy, stats);
        }
        i += 1;
    }
    get_entropy_unrefined_helper(0, i, &mut xy_prev, &mut i_prev, bit_entropy, stats);

    bit_entropy.entropy = vp8l_fast_s_log2(bit_entropy.sum).wrapping_sub(bit_entropy.entropy);
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless_enc.c VP8LSubtractGreenFromBlueAndRed_C
/// The C code reads each pixel into an `int`; the masks make the sign
/// irrelevant, so the pixel is used as `uint32_t`.
pub fn vp8l_subtract_green_from_blue_and_red_c(argb_data: &mut [u32], num_pixels: i32) {
    for pixel in &mut argb_data[..count(num_pixels)] {
        let argb = *pixel;
        let green = (argb >> 8) & 0xff;
        let new_r = ((argb >> 16) & 0xff).wrapping_sub(green) & 0xff;
        let new_b = (argb & 0xff).wrapping_sub(green) & 0xff;
        *pixel = (argb & 0xff00_ff00) | (new_r << 16) | new_b;
    }
}

// upstream: src/dsp/lossless_enc.c ColorTransformDelta
#[inline]
fn color_transform_delta(color_pred: i8, color: i8) -> i32 {
    (i32::from(color_pred) * i32::from(color)) >> 5
}

// upstream: src/dsp/lossless_enc.c U32ToS8
#[inline]
fn u32_to_s8(v: u32) -> i8 {
    ((v & 0xff) as u8).cast_signed()
}

// upstream: src/dsp/lossless_enc.c VP8LTransformColor_C
#[expect(
    clippy::cast_sign_loss,
    reason = "the masked channels are not negative"
)]
pub fn vp8l_transform_color_c(m: &VP8LMultipliers, data: &mut [u32], num_pixels: i32) {
    for pixel in &mut data[..count(num_pixels)] {
        let argb = *pixel;
        let green = u32_to_s8(argb >> 8);
        let red = u32_to_s8(argb >> 16);
        let mut new_red: i32 = i32::from(red) & 0xff;
        let mut new_blue: i32 = (argb & 0xff).cast_signed();
        new_red -= color_transform_delta(m.green_to_red.cast_signed(), green);
        new_red &= 0xff;
        new_blue -= color_transform_delta(m.green_to_blue.cast_signed(), green);
        new_blue -= color_transform_delta(m.red_to_blue.cast_signed(), red);
        new_blue &= 0xff;
        *pixel = (argb & 0xff00_ff00) | ((new_red as u32) << 16) | (new_blue as u32);
    }
}

// upstream: src/dsp/lossless_enc.c TransformColorRed
#[inline]
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion of the masked channel to uint8_t"
)]
fn transform_color_red(green_to_red: u8, argb: u32) -> u8 {
    let green = u32_to_s8(argb >> 8);
    let mut new_red: i32 = (argb >> 16).cast_signed();
    new_red -= color_transform_delta(green_to_red.cast_signed(), green);
    (new_red & 0xff) as u8
}

// upstream: src/dsp/lossless_enc.c TransformColorBlue
#[inline]
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion of the masked channel to uint8_t"
)]
fn transform_color_blue(green_to_blue: u8, red_to_blue: u8, argb: u32) -> u8 {
    let green = u32_to_s8(argb >> 8);
    let red = u32_to_s8(argb >> 16);
    let mut new_blue: i32 = (argb & 0xff).cast_signed();
    new_blue -= color_transform_delta(green_to_blue.cast_signed(), green);
    new_blue -= color_transform_delta(red_to_blue.cast_signed(), red);
    (new_blue & 0xff) as u8
}

/// The start of row `row` of a tile whose rows are `stride` pixels apart, as
/// the C pointer advances by `stride` per row.
#[inline]
fn tile_row(row: i32, stride: i32) -> usize {
    usize::try_from(i64::from(row) * i64::from(stride)).expect("a tile row lies in its buffer")
}

// upstream: src/dsp/lossless_enc.c VP8LCollectColorRedTransforms_C
/// `argb` starts at the tile; `histo` holds 256 counts.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C (uint8_t) cast of the multiplier"
)]
pub fn vp8l_collect_color_red_transforms_c(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    for row in 0..tile_height.max(0) {
        let start = tile_row(row, stride);
        for &pixel in &argb[start..start + count(tile_width)] {
            let index = usize::from(transform_color_red(green_to_red as u8, pixel));
            histo[index] = histo[index].wrapping_add(1);
        }
    }
}

// upstream: src/dsp/lossless_enc.c VP8LCollectColorBlueTransforms_C
/// `argb` starts at the tile; `histo` holds 256 counts.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C (uint8_t) casts of the multipliers"
)]
pub fn vp8l_collect_color_blue_transforms_c(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    for row in 0..tile_height.max(0) {
        let start = tile_row(row, stride);
        for &pixel in &argb[start..start + count(tile_width)] {
            let index = usize::from(transform_color_blue(
                green_to_blue as u8,
                red_to_blue as u8,
                pixel,
            ));
            histo[index] = histo[index].wrapping_add(1);
        }
    }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless_enc.c VectorMismatch_C
fn vector_mismatch_c(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    let mut match_len: i32 = 0;

    while match_len < length && array1[count(match_len)] == array2[count(match_len)] {
        match_len += 1;
    }
    match_len
}

// upstream: src/dsp/lossless_enc.c VP8LBundleColorMap_C
/// Bundles multiple (1, 2, 4 or 8) pixels into a single pixel.
#[expect(
    clippy::cast_sign_loss,
    reason = "the shifts are below 32 and the indexes are not negative"
)]
pub fn vp8l_bundle_color_map_c(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    if xbits > 0 {
        let bit_depth = 1 << (3 - xbits);
        let mask = (1 << xbits) - 1;
        let mut code: u32 = 0xff00_0000;
        for x in 0..width {
            let xsub = x & mask;
            if xsub == 0 {
                code = 0xff00_0000;
            }
            code |= u32::from(row[x as usize]) << (8 + bit_depth * xsub);
            dst[(x >> xbits) as usize] = code;
        }
    } else {
        for (out, &index) in dst[..count(width)].iter_mut().zip(row) {
            *out = 0xff00_0000 | (u32::from(index) << 8);
        }
    }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless_enc.c ExtraCost_C
#[expect(
    clippy::cast_sign_loss,
    reason = "the C int index converts to unsigned in the product"
)]
fn extra_cost_c(population: &[u32], length: i32) -> u32 {
    let mut cost = population[4].wrapping_add(population[5]);
    debug_assert_eq!(length % 2, 0);
    let mut i: i32 = 2;
    while i < length / 2 - 1 {
        let pair = population[count(2 * i + 2)].wrapping_add(population[count(2 * i + 3)]);
        cost = cost.wrapping_add((i as u32).wrapping_mul(pair));
        i += 1;
    }
    cost
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless_enc.c AddVector_C
fn add_vector_c(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    let size = count(size);
    for ((o, &x), &y) in out[..size].iter_mut().zip(&a[..size]).zip(&b[..size]) {
        *o = x.wrapping_add(y);
    }
}

// upstream: src/dsp/lossless_enc.c AddVectorEq_C
fn add_vector_eq_c(a: &[u32], out: &mut [u32], size: i32) {
    let size = count(size);
    for (o, &x) in out[..size].iter_mut().zip(&a[..size]) {
        *o = o.wrapping_add(x);
    }
}

//------------------------------------------------------------------------------
// Image transforms.

/// The encoder form of `VP8LPredictorAddSubFunc` (Rust-only; see the
/// `# Deviations from upstream` section of this module, item 1):
/// `(data, in, upper, num_pixels, out)`, where `in` and `upper` are positions
/// in `data`, the buffer of the input rows, and the residuals go to
/// `out[..num_pixels]`. `data[in - 1]` and `data[upper - 1 ..= upper + num_pixels]`
/// are read as the C functions read `in[-1]`, `upper[-1]` and `upper[num_pixels]`.
pub type PredictorSubFunc = fn(&[u32], usize, usize, i32, &mut [u32]);

// upstream: src/dsp/lossless_enc.c PredictorSub0_C
fn predictor_sub0_c(
    data: &[u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let n = count(num_pixels);
    for (o, &pixel) in out[..n].iter_mut().zip(&data[in_pos..in_pos + n]) {
        *o = vp8l_sub_pixels(pixel, ARGB_BLACK);
    }
}

// upstream: src/dsp/lossless_enc.c PredictorSub1_C
fn predictor_sub1_c(
    data: &[u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    for (i, o) in out[..count(num_pixels)].iter_mut().enumerate() {
        *o = vp8l_sub_pixels(data[in_pos + i], data[in_pos + i - 1]);
    }
}

// upstream: src/dsp/lossless_enc.c GENERATE_PREDICTOR_SUB
/// It subtracts the prediction from the input pixel and stores the residual in
/// the output pixel. The C `assert(upper != NULL)` has no counterpart, because
/// positions cannot be `NULL`; the left position is formed with a wrapping
/// subtraction, as the C `&in[x - 1]` is formed, and is only read by the
/// predictors that read it.
macro_rules! generate_predictor_sub {
    ($predictor:path, $predictor_sub:ident) => {
        fn $predictor_sub(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            for (x, o) in out[..count(num_pixels)].iter_mut().enumerate() {
                let pred = $predictor(data, (in_pos + x).wrapping_sub(1), data, upper_pos + x);
                *o = vp8l_sub_pixels(data[in_pos + x], pred);
            }
        }
    };
}

generate_predictor_sub!(vp8l_predictor2_c, predictor_sub2_c);
generate_predictor_sub!(vp8l_predictor3_c, predictor_sub3_c);
generate_predictor_sub!(vp8l_predictor4_c, predictor_sub4_c);
generate_predictor_sub!(vp8l_predictor5_c, predictor_sub5_c);
generate_predictor_sub!(vp8l_predictor6_c, predictor_sub6_c);
generate_predictor_sub!(vp8l_predictor7_c, predictor_sub7_c);
generate_predictor_sub!(vp8l_predictor8_c, predictor_sub8_c);
generate_predictor_sub!(vp8l_predictor9_c, predictor_sub9_c);
generate_predictor_sub!(vp8l_predictor10_c, predictor_sub10_c);
generate_predictor_sub!(vp8l_predictor11_c, predictor_sub11_c);
generate_predictor_sub!(vp8l_predictor12_c, predictor_sub12_c);
generate_predictor_sub!(vp8l_predictor13_c, predictor_sub13_c);

//------------------------------------------------------------------------------
// Dispatch (ADR 0003). `VP8LEncDspInit` only assigns the portable functions
// below to the global pointers (after calling `VP8LDspInit`, which only assigns
// pointers too), so it has no counterpart, and each pointer is a function.

// upstream: src/dsp/lossless_enc.c VP8LPredictorsSub
/// The table that `VP8LEncDspInit` fills, with the sentinels at 14 and 15.
static VP8L_PREDICTORS_SUB: [PredictorSubFunc; 16] = [
    predictor_sub0_c,
    predictor_sub1_c,
    predictor_sub2_c,
    predictor_sub3_c,
    predictor_sub4_c,
    predictor_sub5_c,
    predictor_sub6_c,
    predictor_sub7_c,
    predictor_sub8_c,
    predictor_sub9_c,
    predictor_sub10_c,
    predictor_sub11_c,
    predictor_sub12_c,
    predictor_sub13_c,
    predictor_sub0_c, // <- padding security sentinels
    predictor_sub0_c,
];

// upstream: src/dsp/lossless_enc.c VP8LPredictorsSub_C
/// The exposed plain-C implementations, which `VP8LEncDspInit` fills with the
/// same entries.
static VP8L_PREDICTORS_SUB_C: [PredictorSubFunc; 16] = VP8L_PREDICTORS_SUB;

// upstream: src/dsp/lossless_enc.c VP8LPredictorsSub
/// Calls the batch predictor `VP8LPredictorsSub[mode]` in the form of
/// [`PredictorSubFunc`].
///
/// # Panics
///
/// Panics for a mode outside 0..16.
#[inline]
pub fn vp8l_predictors_sub(
    mode: usize,
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    VP8L_PREDICTORS_SUB[mode](data, in_pos, upper_pos, num_pixels, out);
}

// upstream: src/dsp/lossless_enc.c VP8LPredictorsSub_C
/// Calls the plain-C batch predictor `VP8LPredictorsSub_C[mode]`.
///
/// # Panics
///
/// Panics for a mode outside 0..16.
#[inline]
pub fn vp8l_predictors_sub_c(
    mode: usize,
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    VP8L_PREDICTORS_SUB_C[mode](data, in_pos, upper_pos, num_pixels, out);
}

// upstream: src/dsp/lossless_enc.c VP8LSubtractGreenFromBlueAndRed
#[inline]
pub fn vp8l_subtract_green_from_blue_and_red(argb_data: &mut [u32], num_pixels: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_enc_neon::subtract_green_from_blue_and_red(
            argb_data, num_pixels,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_subtract_green_from_blue_and_red_c(argb_data, num_pixels);
}

// upstream: src/dsp/lossless_enc.c VP8LTransformColor
#[inline]
pub fn vp8l_transform_color(m: &VP8LMultipliers, data: &mut [u32], num_pixels: i32) {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::lossless_enc_neon::transform_color(m, data, num_pixels);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    vp8l_transform_color_c(m, data, num_pixels);
}

// upstream: src/dsp/lossless_enc.c VP8LCollectColorBlueTransforms
#[inline]
pub fn vp8l_collect_color_blue_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    vp8l_collect_color_blue_transforms_c(
        argb,
        stride,
        tile_width,
        tile_height,
        green_to_blue,
        red_to_blue,
        histo,
    );
}

// upstream: src/dsp/lossless_enc.c VP8LCollectColorRedTransforms
#[inline]
pub fn vp8l_collect_color_red_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    vp8l_collect_color_red_transforms_c(argb, stride, tile_width, tile_height, green_to_red, histo);
}

// upstream: src/dsp/lossless_enc.c VP8LFastLog2Slow
#[must_use]
#[inline]
pub fn vp8l_fast_log2_slow(v: u32) -> u32 {
    fast_log2_slow_c(v)
}

// upstream: src/dsp/lossless_enc.c VP8LFastSLog2Slow
#[must_use]
#[inline]
pub fn vp8l_fast_s_log2_slow(v: u32) -> u64 {
    fast_s_log2_slow_c(v)
}

// upstream: src/dsp/lossless_enc.c VP8LExtraCost
#[must_use]
#[inline]
pub fn vp8l_extra_cost(population: &[u32], length: i32) -> u32 {
    extra_cost_c(population, length)
}

// upstream: src/dsp/lossless_enc.c VP8LCombinedShannonEntropy
/// `x` and `y` hold at least 256 entries.
#[must_use]
#[inline]
pub fn vp8l_combined_shannon_entropy(x: &[u32], y: &[u32]) -> u64 {
    combined_shannon_entropy_c(x, y)
}

// upstream: src/dsp/lossless_enc.c VP8LShannonEntropy
#[must_use]
#[inline]
pub fn vp8l_shannon_entropy(x: &[u32], length: i32) -> u64 {
    shannon_entropy_c(x, length)
}

// upstream: src/dsp/lossless_enc.c VP8LGetEntropyUnrefined
/// Get the entropy for the distribution `x`.
#[inline]
pub fn vp8l_get_entropy_unrefined(
    x: &[u32],
    length: i32,
    bit_entropy: &mut VP8LBitEntropy,
    stats: &mut VP8LStreaks,
) {
    get_entropy_unrefined_c(x, length, bit_entropy, stats);
}

// upstream: src/dsp/lossless_enc.c VP8LGetCombinedEntropyUnrefined
/// Get the combined symbol bit entropy and Huffman cost stats for the
/// distributions `x` and `y`. Those results can then be refined according to
/// codec specific heuristics.
#[inline]
pub fn vp8l_get_combined_entropy_unrefined(
    x: &[u32],
    y: &[u32],
    length: i32,
    bit_entropy: &mut VP8LBitEntropy,
    stats: &mut VP8LStreaks,
) {
    get_combined_entropy_unrefined_c(x, y, length, bit_entropy, stats);
}

// upstream: src/dsp/lossless_enc.c VP8LAddVector
#[inline]
pub fn vp8l_add_vector(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    add_vector_c(a, b, out, size);
}

// upstream: src/dsp/lossless_enc.c VP8LAddVectorEq
#[inline]
pub fn vp8l_add_vector_eq(a: &[u32], out: &mut [u32], size: i32) {
    add_vector_eq_c(a, out, size);
}

// upstream: src/dsp/lossless_enc.c VP8LVectorMismatch
/// Returns the first index where `array1` and `array2` are different.
#[must_use]
#[inline]
pub fn vp8l_vector_mismatch(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    vector_mismatch_c(array1, array2, length)
}

// upstream: src/dsp/lossless_enc.c VP8LBundleColorMap
#[inline]
pub fn vp8l_bundle_color_map(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    vp8l_bundle_color_map_c(row, width, xbits, dst);
}
