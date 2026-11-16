typedef int int8_t; typedef int int16_t; typedef int int32_t; typedef int int64_t;
typedef int uint8_t; typedef int uint16_t; typedef int uint32_t; typedef int uint64_t;
typedef int size_t; typedef int intptr_t; typedef int ptrdiff_t;
typedef int __m128i; typedef int __m256i; typedef int __m128; typedef int __m256; typedef int __m128d; typedef int __m256d; typedef int __m64;
__m128i xx_loadu_2x64(const void *hi, const void *lo);
void xx_storeu_128(void *a, __m128i v);
void yy_storeu_256(void *a, __m256i v);
int64_t loadu_int64(const void *src);
float sqrtf(float x);
