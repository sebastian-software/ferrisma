
// TRANSPOSE_8X8_AVX2() of av1_fwd_txfm2d_avx2.c: the C macro reads b0..b3 and
// writes c0..c3 of its caller.
macro_rules! TRANSPOSE_8X8_AVX2 {
    ($b0:expr, $b1:expr, $b2:expr, $b3:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr) => {{
        // aa0:    00 10 01 11  02 12 03 13 | 40 50 41 51  42 52 43 53
        // aa1:    04 14 05 15  06 16 07 17 | 44 54 45 55  46 56 47 57
        // aa2:    20 30 21 31  22 32 23 33 | 60 70 61 71  62 72 63 73
        // aa3:    24 34 25 35  26 36 27 37 | 64 74 65 75  66 76 67 77
        let aa0: __m256i = _mm256_unpacklo_epi16($b0, $b1);
        let aa1: __m256i = _mm256_unpackhi_epi16($b0, $b1);
        let aa2: __m256i = _mm256_unpacklo_epi16($b2, $b3);
        let aa3: __m256i = _mm256_unpackhi_epi16($b2, $b3);
        // Unpack 32 bit elements resulting in:
        // bb0: 00 10 20 30  01 11 21 31 | 40 50 60 70  41 51 61 71
        // bb1: 02 12 22 32  03 13 23 33 | 42 52 62 72  43 53 63 73
        // bb2: 04 14 24 34  05 15 25 35 | 44 54 64 74  45 55 65 75
        // bb2: 06 16 26 36  07 17 27 37 | 46 56 66 76  47 57 67 77
        let bb0: __m256i = _mm256_unpacklo_epi32(aa0, aa2);
        let bb1: __m256i = _mm256_unpackhi_epi32(aa0, aa2);
        let bb2: __m256i = _mm256_unpacklo_epi32(aa1, aa3);
        let bb3: __m256i = _mm256_unpackhi_epi32(aa1, aa3);
        // bb0: 00 10 20 30 40 50 60 70| 01 11 21 31 41 51 61 71
        // bb1: 02 12 22 32 42 52 62 72| 03 13 23 33 43 53 63 73
        // bb2: 04 14 24 34 44 54 64 74| 05 15 25 35 45 55 65 75
        // bb2: 06 16 26 36 46 56 66 76| 07 17 27 37 47 57 67 77
        $c0 = _mm256_permute4x64_epi64::<0xd8>(bb0);
        $c1 = _mm256_permute4x64_epi64::<0xd8>(bb1);
        $c2 = _mm256_permute4x64_epi64::<0xd8>(bb2);
        $c3 = _mm256_permute4x64_epi64::<0xd8>(bb3);
    }};
}
