# prof/jpeg24-avif-none-t0.json.gz: total CPU weight 15287266 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 13.3 | `av1_optimize_txb` |
| 9.4 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 9.3 | `__psynch_cvwait` |
| 6.4 | `av1_highbd_quantize_fp_facade` |
| 6.1 | `_platform_memset` |
| 3.0 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 2.3 | `_platform_memmove` |
| 2.1 | `avifImageRGBToYUV` |
| 2.0 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.9 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.9 | `__psynch_mutexwait` |
| 1.9 | `libsystem_malloc.dylib+?` |
| 1.6 | `av1_write_coeffs_txb` |
| 1.4 | `aom_highbd_quantize_b_helper_c` |
| 1.2 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct16x16_neon` |
| 1.2 | `av1_cost_coeffs_txb` |
| 1.2 | `av1_fwd_txfm2d_8x8_neon` |
| 1.1 | `av1_predict_intra_block` |
| 1.0 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 1.0 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` |
| 0.9 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.9 | `cfl_predict_hbd_8x8_c` |
| 0.8 | `cdef_filter_16_0_neon` |
| 0.8 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 0.7 | `av1_get_nz_map_contexts_c` |
| 0.7 | `av1_update_and_record_txb_context` |
| 0.7 | `aom::av1__encoder__txb_rdopt::get_coeff_cost_general` |
| 0.7 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 0.7 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 0.6 | `av1_nn_predict_c` |
| 0.6 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.6 | `av1_fwd_txfm2d_16x16_neon` |
| 0.6 | `av1_highbd_quantize_fp_neon` |
| 0.6 | `aom::aom_dsp__entenc::od_ec_enc_normalize` |
| 0.6 | `semaphore_wait_trap` |
| 0.5 | `av1_rd_pick_partition` |
| 0.5 | `aom::av1__encoder__txb_rdopt::get_tx_type_cost` |
| 0.5 | `av1_avg_cdf_symbols` |
| 0.5 | `ferrisma-base+?` |
| 0.5 | `__bzero` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 89.9 | `_pthread_start` |
| 87.4 | `aom::aom_util__aom_thread::thread_loop` |
| 84.2 | `av1_encode_sb_row` |
| 84.2 | `aom::jump::value_scope::<i32, aom::av1__encoder__ethread::enc_row_mt_worker_hook::{closure#0}>` |
| 84.2 | `aom::av1__encoder__ethread::enc_row_mt_worker_hook` |
| 72.2 | `av1_rd_pick_partition` |
| 45.5 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 45.3 | `av1_rd_pick_intra_mode_sb` |
| 36.2 | `av1_txfm_rd_in_plane` |
| 35.8 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 34.0 | `aom::av1__encoder__tx_search::search_tx_type` |
| 28.8 | `av1_rd_pick_intra_sby_mode` |
| 26.6 | `av1_pick_uniform_tx_size_type_yrd` |
| 19.8 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 16.3 | `av1_rd_pick_intra_sbuv_mode` |
| 16.1 | `av1_optimize_b` |
| 16.0 | `av1_optimize_txb` |
| 12.3 | `av1_foreach_transformed_block_in_plane` |
| 12.0 | `aom::av1__encoder__partition_search::encode_sb` |
| 11.9 | `aom::av1__encoder__partition_search::encode_b` |
| 11.7 | `av1_encode_intra_block_plane` |
| 11.5 | `aom::av1__encoder__partition_search::encode_superblock` |
| 11.5 | `aom::av1__encoder__encodemb::encode_block_intra_and_set_context` |
| 10.3 | `av1_prune_partitions_before_search` |
| 10.1 | `ferrisma::cli::run` |
| 10.1 | `ferrisma::main` |
| 10.1 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 10.1 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 10.1 | `std::rt::lang_start_internal` |
| 10.1 | `main` |
| 10.1 | `start` |
| 10.0 | `ferrisma::pipeline::run` |
| 9.7 | `av1_cnn_predict_img_multi_out_highbd` |
| 9.6 | `av1_cnn_predict_c` |
| 9.4 | `aom::av1__encoder__cnn::av1_cnn_convolve` |
| 9.4 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 9.3 | `av1_quant` |
| 9.3 | `_pthread_cond_wait` |
| 9.3 | `__psynch_cvwait` |
| 8.6 | `<ferrisma_avif_write::Frame>::finish` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 78.4 | `av1_optimize_txb` 13.3; `av1_cnn_convolve_no_maxpool_padding_valid_c` 9.4; `av1_highbd_quantize_fp_facade` 6.4; `_platform_memset` 5.9 |
| sync/wait | 12.3 | `__psynch_cvwait` 9.3; `__psynch_mutexwait` 1.9; `semaphore_wait_trap` 0.6; `__ulock_wait` 0.3 |
| other | 5.1 | `av1_write_coeffs_txb` 1.3; `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` 0.8; `cdef_filter_16_0_neon` 0.7; `aom::aom_dsp__entenc::od_ec_enc_normalize` 0.5 |
| rgb2yuv | 2.6 | `avifImageRGBToYUV` 2.1; `libsystem_m.dylib+?` 0.3; `ferrisma-base+?` 0.2 |
| jpeg-decode | 1.0 | `jpeg::jdhuff::decode_mcu` 0.4; `jpeg::jdhuff::HUFF_DECODE_FAST` 0.2; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.1; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 0.1 |
| engine-pipeline | 0.4 | `_platform_memmove` 0.2; `libsystem_m.dylib+?` 0.0; `read` 0.0; `libsystem_malloc.dylib+?` 0.0 |
| quantize | 0.2 | `ferrisma::fast_execute::quantize_row::<3>` 0.2 |
| resize | 0.0 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 0.0; `ferrisma_kernels::dispatch::neon::axpy` 0.0; `__sysctl` 0.0; `<ferrisma_kernels::Kernels>::horizontal_rgba` 0.0 |
| linearize | 0.0 | `ferrisma::fast_execute::source_row_to_linear::<3>` 0.0 |
