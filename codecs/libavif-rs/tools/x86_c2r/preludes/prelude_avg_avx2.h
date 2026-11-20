typedef int int8_t; typedef int int16_t; typedef int int32_t; typedef int int64_t;
typedef int uint8_t; typedef int uint16_t; typedef int uint32_t; typedef int uint64_t;
typedef int size_t; typedef int intptr_t; typedef int ptrdiff_t;
typedef int __m128i; typedef int __m256i; typedef int __m128; typedef int __m256; typedef int __m128d; typedef int __m256d;
typedef int tran_low_t;
void store_tran_low(__m256i a, tran_low_t *b);
