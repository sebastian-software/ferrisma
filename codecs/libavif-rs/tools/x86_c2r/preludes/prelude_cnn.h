typedef int int8_t; typedef int int16_t; typedef int int32_t; typedef int int64_t;
typedef int uint8_t; typedef int uint16_t; typedef int uint32_t; typedef int uint64_t;
typedef int size_t; typedef int intptr_t; typedef int ptrdiff_t;
typedef int __m128i; typedef int __m256i; typedef int __m128; typedef int __m256; typedef int __m128d; typedef int __m256d;
typedef struct CNN_LAYER_CONFIG CNN_LAYER_CONFIG;
void av1_cnn_convolve_no_maxpool_padding_valid_c(const float **input, int in_width, int in_height, int in_stride, const CNN_LAYER_CONFIG *layer_config, float **output, int out_stride, int start_idx, int cstep, int channel_step);
