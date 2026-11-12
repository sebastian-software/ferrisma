# prof/jpeg24-avif-down-t0.json.gz: total CPU weight 1833718 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 13.5 | `__psynch_cvwait` |
| 10.9 | `av1_optimize_txb` |
| 8.1 | `__psynch_mutexwait` |
| 5.4 | `av1_highbd_quantize_fp_facade` |
| 5.2 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 5.2 | `_platform_memset` |
| 4.0 | `jpeg::jdhuff::decode_mcu` |
| 2.9 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 2.7 | `libsystem_malloc.dylib+?` |
| 2.1 | `av1_cost_coeffs_txb` |
| 2.0 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 1.7 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.6 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.6 | `semaphore_wait_trap` |
| 1.2 | `avifImageRGBToYUV` |
| 1.2 | `av1_predict_intra_block` |
| 1.1 | `_platform_memmove` |
| 1.1 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 1.1 | `av1_update_and_record_txb_context` |
| 1.0 | `av1_write_coeffs_txb` |
| 0.9 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` |
| 0.8 | `av1_setup_xform` |
| 0.8 | `aom_highbd_quantize_b_helper_c` |
| 0.8 | `av1_get_nz_map_contexts_c` |
| 0.8 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct16_x4_neon` |
| 0.7 | `aom::av1__encoder__txb_rdopt::get_coeff_cost_general` |
| 0.7 | `cfl_predict_hbd_16x16_c` |
| 0.6 | `aom_highbd_hadamard_8x8_c` |
| 0.6 | `av1_fwd_txfm2d_8x8_neon` |
| 0.6 | `cdef_filter_16_0_neon` |
| 0.5 | `av1_rd_pick_partition` |
| 0.5 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 0.4 | `av1_highbd_quantize_fp_neon` |
| 0.4 | `av1_rd_pick_intra_sby_mode` |
| 0.4 | `semaphore_signal_trap` |
| 0.4 | `cfl_predict_hbd_8x8_c` |
| 0.4 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 0.4 | `av1_set_offsets_without_segment_id` |
| 0.4 | `av1_fwd_txfm2d_16x16_neon` |
| 0.4 | `__bzero` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 85.0 | `_pthread_start` |
| 75.0 | `aom::aom_util__aom_thread::thread_loop` |
| 70.2 | `av1_encode_sb_row` |
| 70.2 | `aom::jump::value_scope::<i32, aom::av1__encoder__ethread::enc_row_mt_worker_hook::{closure#0}>` |
| 70.2 | `aom::av1__encoder__ethread::enc_row_mt_worker_hook` |
| 56.2 | `av1_rd_pick_partition` |
| 38.2 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 37.9 | `av1_rd_pick_intra_mode_sb` |
| 31.4 | `av1_txfm_rd_in_plane` |
| 31.2 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 28.2 | `aom::av1__encoder__tx_search::search_tx_type` |
| 26.4 | `av1_rd_pick_intra_sby_mode` |
| 24.3 | `av1_pick_uniform_tx_size_type_yrd` |
| 18.1 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 15.0 | `ferrisma::cli::run` |
| 15.0 | `ferrisma::main` |
| 15.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 15.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 15.0 | `std::rt::lang_start_internal` |
| 15.0 | `main` |
| 15.0 | `start` |
| 14.5 | `ferrisma::pipeline::run` |
| 13.8 | `av1_foreach_transformed_block_in_plane` |
| 13.5 | `_pthread_cond_wait` |
| 13.5 | `__psynch_cvwait` |
| 12.9 | `av1_optimize_b` |
| 12.8 | `av1_optimize_txb` |
| 11.4 | `av1_rd_pick_intra_sbuv_mode` |
| 10.8 | `av1_row_mt_sync_read` |
| 10.1 | `ferrisma::parallel_execute::worker::<3>` |
| 10.1 | `std::sys::backtrace::__rust_begin_short_backtrace::<ferrisma::parallel_execute::run<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}::{closure#0}, ()>` |
| 10.1 | `<std::thread::lifecycle::spawn_unchecked<..>::{closure#1} as ops::function::FnOnce<()>>::call_once::{shim:vtable#0}` |
| 10.1 | `<std::sys::thread::unix::Thread>::new::thread_start` |
| 9.6 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 9.2 | `ferrisma::parallel_execute::drive::<3, ferrisma::pipeline::run::{closure#1}>` |
| 9.2 | `std::thread::scoped::scope::<ferrisma::parallel_execute::run<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}, result::Result<(ferrisma::decoder::DecodeReport, time::Duration, time::Duration), fe` |
| 9.2 | `ferrisma::parallel_execute::run::<3, ferrisma::pipeline::run::{closure#1}>` |
| 9.2 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 8.2 | `aom::av1__encoder__partition_search::encode_sb` |
| 8.2 | `aom::av1__encoder__partition_search::encode_b` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 61.1 | `av1_optimize_txb` 10.9; `av1_highbd_quantize_fp_facade` 5.4; `av1_cnn_convolve_no_maxpool_padding_valid_c` 5.2; `_platform_memset` 4.8 |
| sync/wait | 23.8 | `__psynch_cvwait` 13.5; `__psynch_mutexwait` 8.1; `semaphore_wait_trap` 1.6; `semaphore_signal_trap` 0.4 |
| jpeg-decode | 8.0 | `jpeg::jdhuff::decode_mcu` 4.0; `jpeg::jdhuff::HUFF_DECODE_FAST` 2.0; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 0.5; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 0.3 |
| other | 3.6 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` 0.9; `av1_write_coeffs_txb` 0.6; `cdef_filter_16_0_neon` 0.6; `cdef_filter_16_1_neon` 0.2 |
| engine-pipeline | 1.8 | `read` 0.4; `_platform_memmove` 0.3; `_platform_memset` 0.2; `ferrisma::quality::taps` 0.2 |
| rgb2yuv | 1.3 | `avifImageRGBToYUV` 1.2; `libsystem_m.dylib+?` 0.1; `ferrisma-base+?` 0.0; `avifCalcYUVCoefficients` 0.0 |
| resize | 0.4 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 0.3; `ferrisma_kernels::dispatch::neon::axpy` 0.1 |
| linearize | 0.1 | `ferrisma::fast_execute::source_row_to_linear::<3>` 0.1 |
| quantize | 0.0 | `ferrisma::fast_execute::quantize_row::<3>` 0.0 |
