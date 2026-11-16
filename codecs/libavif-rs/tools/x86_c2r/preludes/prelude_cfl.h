typedef int int8_t; typedef int int16_t; typedef int int32_t; typedef int int64_t;
typedef int uint8_t; typedef int uint16_t; typedef int uint32_t; typedef int uint64_t;
typedef int size_t; typedef int intptr_t; typedef int ptrdiff_t;
typedef int __m128i; typedef int __m256i; typedef int __m128; typedef int __m256; typedef int __m128d; typedef int __m256d;
typedef int TX_SIZE; typedef int TX_TYPE; typedef int tran_low_t; typedef int ITX_TYPE_1D;
typedef struct TxfmParam TxfmParam;
typedef void (*transform_1d_ssse3)(const __m128i *input, __m128i *output);
typedef void (*transform_1d_avx2)(const __m256i *input, __m256i *output);
const int32_t *cospi_arr(int n);
const int32_t *sinpi_arr(int n);
void get_flip_cfg(TX_TYPE tx_type, int *ud_flip, int *lr_flip);
int get_rect_tx_log_ratio(int col, int row);
int get_txw_idx(TX_SIZE tx_size);
int get_txh_idx(TX_SIZE tx_size);
int AOMMIN(int x, int y);
void av1_inv_txfm_add_c(const tran_low_t *dqcoeff, uint8_t *dst, int stride, const TxfmParam *txfm_param);
void get_eobx_eoby_scan_default(int *eobx, int *eoby, TX_SIZE tx_size, int eob);
void get_eobx_eoby_scan_h_identity(int *eobx, int *eoby, TX_SIZE tx_size, int eob);
void get_eobx_eoby_scan_v_identity(int *eobx, int *eoby, TX_SIZE tx_size, int eob);
__m128i pair_set_epi16(int a, int b);
void transpose_16bit_4x4(const __m128i *const in, __m128i *const out);
void transpose_16bit_4x8(const __m128i *const in, __m128i *const out);
void transpose_16bit_8x4(const __m128i *const in, __m128i *const out);
void transpose_16bit_8x8(const __m128i *const in, __m128i *const out);
void flip_buf_sse2(__m128i *in, __m128i *out, int size);
typedef int qm_val_t;
__m128i xx_loadl_32(const void *a);
__m128i xx_loadl_64(const void *a);
__m128i xx_load_128(const void *a);
__m128i xx_loadu_128(const void *a);
void xx_storel_32(void *a, __m128i v);
void xx_storel_64(void *a, __m128i v);
void xx_store_128(void *a, __m128i v);
void xx_storeu_128(void *a, __m128i v);
__m256i yy_load_256(const void *a);
__m256i yy_loadu_256(const void *a);
void yy_store_256(void *a, __m256i v);
void yy_storeu_256(void *a, __m256i v);
void aom_quantize_b_helper_c(const tran_low_t *coeff_ptr, intptr_t n_coeffs, const int16_t *zbin_ptr, const int16_t *round_ptr, const int16_t *quant_ptr, const int16_t *quant_shift_ptr, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const int16_t *scan, const int16_t *iscan, const qm_val_t *qm_ptr, const qm_val_t *iqm_ptr, const int log_scale);
void load_b_values(const int16_t *zbin_ptr, __m128i *zbin, const int16_t *round_ptr, __m128i *round, const int16_t *quant_ptr, __m128i *quant, const int16_t *dequant_ptr, __m128i *dequant, const int16_t *shift_ptr, __m128i *shift);
__m128i invert_sign_sse2(__m128i a, __m128i sign);
__m128i invert_sign_32_sse2(__m128i a, __m128i sign);
void calculate_qcoeff(__m128i *coeff, const __m128i round, const __m128i quant, const __m128i shift);
void calculate_qcoeff_log_scale(__m128i *coeff, const __m128i round, const __m128i quant, const __m128i *shift, const int *log_scale);
__m128i calculate_dqcoeff(__m128i qcoeff, __m128i dequant);
void calculate_dqcoeff_and_store_log_scale(__m128i qcoeff, __m128i dequant, const __m128i zero, tran_low_t *dqcoeff, const int *log_scale);
__m128i scan_for_eob(__m128i *coeff0, __m128i *coeff1, const __m128i zbin_mask0, const __m128i zbin_mask1, const int16_t *scan_ptr, const int index, const __m128i zero);
int16_t accumulate_eob(__m128i eob);
__m128i load_coefficients(const tran_low_t *coeff_ptr);
void store_coefficients(__m128i coeff_vals, tran_low_t *coeff_ptr);
void update_mask1(__m128i *cmp_mask0, __m128i *cmp_mask1, const int16_t *iscan_ptr, int *is_found, __m128i *mask);
void update_mask0(__m128i *qcoeff0, __m128i *qcoeff1, __m128i *threshold, const int16_t *iscan_ptr, int *is_found, __m128i *mask);
int calculate_non_zero_count(__m128i mask);
typedef void (*cfl_subsample_lbd_fn)(const uint8_t *input, int input_stride, uint16_t *output_q3);
typedef void (*cfl_subsample_hbd_fn)(const uint16_t *input, int input_stride, uint16_t *output_q3);
typedef void (*cfl_subtract_average_fn)(const uint16_t *src, int16_t *dst);
typedef void (*cfl_predict_lbd_fn)(const int16_t *src, uint8_t *dst, int dst_stride, int alpha_q3);
typedef void (*cfl_predict_hbd_fn)(const int16_t *src, uint16_t *dst, int dst_stride, int alpha_q3, int bd);
void cfl_subsample_lbd_420_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_420_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_420_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_420_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_422_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_422_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_422_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_422_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_444_4x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_4x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_4x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_8x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_8x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_lbd_444_8x16_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_8x32_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_16x4_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_16x8_ssse3(const uint8_t *cfl_type, int input_stride,
                                      uint16_t *output_q3);
void cfl_subsample_lbd_444_16x16_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_lbd_444_16x32_ssse3(const uint8_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_420_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_420_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_422_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_422_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_4x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_4x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_4x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_8x4_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_8x8_ssse3(const uint16_t *cfl_type, int input_stride,
                                     uint16_t *output_q3);
void cfl_subsample_hbd_444_8x16_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_8x32_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x4_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x8_ssse3(const uint16_t *cfl_type,
                                      int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x16_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subsample_hbd_444_16x32_ssse3(const uint16_t *cfl_type,
                                       int input_stride, uint16_t *output_q3);
void cfl_subtract_average_4x4_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_4x8_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_4x16_sse2(const uint16_t *src, int16_t *dst);


void cfl_subtract_average_8x4_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x8_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x16_sse2(const uint16_t *src, int16_t *dst);
void cfl_subtract_average_8x32_sse2(const uint16_t *src, int16_t *dst);

void cfl_predict_lbd_4x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_4x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_4x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                               int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_8x32_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x4_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x8_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x16_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                 int dst_stride, int alpha_q3);
void cfl_predict_lbd_16x32_ssse3(const int16_t *pred_buf_q3, uint8_t *dst,
                                 int dst_stride, int alpha_q3);
void cfl_predict_hbd_4x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_4x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_4x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                               int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_8x32_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x4_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x8_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x16_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                 int dst_stride, int alpha_q3, int bd);
void cfl_predict_hbd_16x32_ssse3(const int16_t *pred_buf_q3, uint16_t *dst,
                                 int dst_stride, int alpha_q3, int bd);
