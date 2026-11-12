# prof/jpeg24-avif-down-t1.json.gz: total CPU weight 2264438 (thread CPU delta units, us)

## Self (top 40)
| % | function |
| ---: | --- |
| 12.8 | `av1_optimize_txb` |
| 8.5 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 6.7 | `av1_cnn_convolve_no_maxpool_padding_valid_c` |
| 5.9 | `av1_highbd_quantize_fp_facade` |
| 4.4 | `jpeg::jdhuff::decode_mcu` |
| 4.4 | `_platform_memset` |
| 2.5 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 2.4 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 1.9 | `aom::av1__encoder__encodeframe::highbd_compute_gradient_info_sb` |
| 1.8 | `<u8 as c2rust_bitfields::FieldType>::get_field` |
| 1.7 | `avifImageRGBToYUV` |
| 1.3 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct32x32_neon` |
| 1.3 | `aom::av1__encoder__tx_search::search_tx_type` |
| 1.3 | `_platform_memmove` |
| 1.3 | `ferrisma::fast_execute::quantize_row::<3>` |
| 1.3 | `aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::highbd_fdct32_x4_neon` |
| 1.2 | `av1_write_coeffs_txb` |
| 1.2 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 1.1 | `ferrisma_kernels::dispatch::neon::axpy` |
| 1.1 | `cfl_predict_hbd_8x8_c` |
| 1.1 | `aom_highbd_quantize_b_helper_c` |
| 1.0 | `av1_predict_intra_block` |
| 1.0 | `av1_fwd_txfm2d_8x8_neon` |
| 0.9 | `av1_get_nz_map_contexts_c` |
| 0.9 | `libsystem_malloc.dylib+?` |
| 0.9 | `aom::av1__encoder__pickcdef::compute_cdef_dist_highbd` |
| 0.9 | `aom::aom_dsp__entenc::od_ec_enc_normalize` |
| 0.9 | `aom::av1__encoder__tx_search::dist_block_tx_domain` |
| 0.8 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.8 | `av1_cost_coeffs_txb` |
| 0.8 | `__bzero` |
| 0.8 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::inv_txfm2d_add_universe_neon` |
| 0.7 | `cdef_filter_16_0_neon` |
| 0.6 | `av1_nn_predict_c` |
| 0.6 | `av1_update_and_record_txb_context` |
| 0.6 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.6 | `libsystem_m.dylib+?` |
| 0.6 | `av1_highbd_quantize_fp_neon` |
| 0.5 | `aom::simd::av1__common__arm__highbd_inv_txfm_neon::idct8x8_neon` |
| 0.5 | `aom_highbd_subtract_block_neon` |

## Inclusive (top 40)
| % | function |
| ---: | --- |
| 100.0 | `ferrisma::cli::run` |
| 100.0 | `ferrisma::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 100.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.6 | `ferrisma::pipeline::run` |
| 74.9 | `<ferrisma_avif_write::Frame>::finish` |
| 74.9 | `<ferrisma::own_avif::Frame as ferrisma::encoder_backend::RowEncoder>::finish` |
| 74.9 | `<ferrisma::pipeline::Sink>::finish` |
| 72.8 | `aom::av1__av1_cx_iface::encoder_encode` |
| 72.8 | `aom_codec_encode` |
| 72.8 | `avif::codec_aom::aomCodecEncodeImage` |
| 72.8 | `avif::write::avifEncoderAddImageInternal` |
| 72.8 | `avifEncoderWrite` |
| 72.6 | `aom::jump::value_scope::<u32, aom::av1__av1_cx_iface::encoder_encode::{closure#0}>` |
| 72.4 | `av1_encode_strategy` |
| 72.4 | `aom::jump::value_scope::<i32, aom::av1__encoder__encoder::av1_get_compressed_data::{closure#0}>` |
| 71.9 | `av1_encode` |
| 71.7 | `aom::av1__encoder__encoder::encode_with_recode_loop_and_filter` |
| 64.2 | `aom::av1__encoder__encodeframe::encode_frame_internal` |
| 64.2 | `av1_encode_frame` |
| 64.1 | `av1_encode_sb_row` |
| 64.1 | `av1_encode_tile` |
| 61.5 | `av1_rd_pick_partition` |
| 41.0 | `aom::av1__encoder__partition_search::pick_sb_modes` |
| 40.9 | `av1_rd_pick_intra_mode_sb` |
| 32.2 | `av1_txfm_rd_in_plane` |
| 31.9 | `aom::av1__encoder__tx_search::block_rd_txfm` |
| 30.2 | `aom::av1__encoder__tx_search::search_tx_type` |
| 24.9 | `av1_rd_pick_intra_sby_mode` |
| 24.7 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 24.4 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 22.7 | `av1_pick_uniform_tx_size_type_yrd` |
| 17.1 | `aom::av1__encoder__intra_mode_search::intra_block_yrd` |
| 15.9 | `av1_rd_pick_intra_sbuv_mode` |
| 15.1 | `av1_optimize_b` |
| 15.0 | `av1_optimize_txb` |
| 13.6 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| aom-encode | 72.8 | `av1_optimize_txb` 12.8; `av1_cnn_convolve_no_maxpool_padding_valid_c` 6.7; `av1_highbd_quantize_fp_facade` 5.9; `_platform_memset` 4.4 |
| jpeg-decode | 10.8 | `jpeg::jdhuff::decode_mcu` 4.4; `jpeg::jdhuff::HUFF_DECODE_FAST` 2.5; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 1.2; `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` 0.8 |
| resize | 9.6 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 8.5; `ferrisma_kernels::dispatch::neon::axpy` 1.1 |
| linearize | 2.4 | `ferrisma::fast_execute::source_row_to_linear::<3>` 2.4 |
| rgb2yuv | 2.1 | `avifImageRGBToYUV` 1.7; `libsystem_m.dylib+?` 0.4; `ferrisma-base+?` 0.1 |
| quantize | 1.3 | `ferrisma::fast_execute::quantize_row::<3>` 1.3 |
| engine-pipeline | 1.0 | `read` 0.3; `_platform_memmove` 0.2; `ferrisma::quality::taps` 0.1; `libsystem_m.dylib+?` 0.1 |
